use rustc_abi::{FieldIdx, Size, VariantIdx};
use rustc_apfloat::ieee::{Double, Half, Quad, Single};
use rustc_ast as ast;
use rustc_const_eval::interpret::{Allocation, Scalar};
use rustc_hash::FxHashMap;
use rustc_hir::{ItemLocalId, LangItem, def_id::LocalDefId};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        AggregateKind, BasicBlock, BasicBlockData, CallSource, CastKind, CoercionSource, Const,
        ConstOperand, ConstValue, Local, LocalDecl, NullOp, OUTERMOST_SOURCE_SCOPE, Operand, Place,
        PlaceElem, Rvalue, SourceInfo, Statement, StatementKind, SwitchTargets, Terminator,
        TerminatorKind, UnevaluatedConst, UnwindAction,
    },
    thir::{
        AdtExpr, AdtExprBase, Expr, ExprId, ExprKind, FruInfo, InlineAsmExpr, LocalVarId,
        LogicalOp, StmtKind, Thir,
    },
    ty::{AdtDef, ScalarInt, Ty, TyCtxt, TyKind},
};
use rustc_span::{DUMMY_SP, Span, Symbol, source_map::Spanned};
use rustc_type_ir::{FloatTy, UintTy};

use crate::rustc_apfloat::Float as _;

macro_rules! unpack {
    ($x:ident = $c:expr) => {{
        let BlockAnd(b, v) = $c;
        $x = b;
        v
    }};
}

struct Builder<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    thir: &'a Thir<'tcx>,
    local_decls: IndexVec<Local, LocalDecl<'tcx>>,
    var_indices: FxHashMap<LocalVarId, Local>,
    dummy_local: Local,
    cfg: CFG<'tcx>,
    scopes: Vec<ItemLocalId>,
    breakable_scopes: FxHashMap<ItemLocalId, BreakableScope>,
}

#[derive(Clone, Copy)]
struct BreakableScope {
    dest: Local,
    target: BasicBlock,
    head: Option<BasicBlock>,
}

#[derive(Clone, Copy)]
enum PlaceBase {
    Local(Local),
    Upvar {
        /// HirId of the upvar
        var_hir_id: LocalVarId,
        /// DefId of the closure
        closure_def_id: LocalDefId,
    },
}

struct PlaceBuilder<'tcx> {
    base: PlaceBase,
    projection: Vec<PlaceElem<'tcx>>,
}

impl<'tcx> PlaceBuilder<'tcx> {
    fn to_place(&self, cx: &Builder<'_, 'tcx>) -> Place<'tcx> {
        let resolved = self.resolve_upvar(cx);
        let builder = resolved.as_ref().unwrap_or(self);
        let PlaceBase::Local(local) = builder.base else { panic!() };
        let projection = cx.tcx.mk_place_elems(&builder.projection);
        Place { local, projection }
    }

    fn resolve_upvar(&self, cx: &Builder<'_, 'tcx>) -> Option<PlaceBuilder<'tcx>> {
        let PlaceBase::Upvar {
            var_hir_id,
            closure_def_id,
        } = self.base
        else {
            return None;
        };
        to_upvars_resolved_place_builder(cx, var_hir_id, closure_def_id, &self.projection)
    }

    fn field(self, f: FieldIdx, ty: Ty<'tcx>) -> Self {
        self.project(PlaceElem::Field(f, ty))
    }

    fn deref(self) -> Self {
        self.project(PlaceElem::Deref)
    }

    fn downcast(self, adt_def: AdtDef<'tcx>, variant_index: VariantIdx) -> Self {
        self.project(PlaceElem::Downcast(
            Some(adt_def.variant(variant_index).name),
            variant_index,
        ))
    }

    fn index(self, index: Local) -> Self {
        self.project(PlaceElem::Index(index))
    }

    fn project(mut self, elem: PlaceElem<'tcx>) -> Self {
        self.projection.push(elem);
        self
    }
}

impl<'tcx> From<Local> for PlaceBuilder<'tcx> {
    fn from(local: Local) -> Self {
        Self {
            base: PlaceBase::Local(local),
            projection: vec![],
        }
    }
}

struct CFG<'tcx> {
    basic_blocks: IndexVec<BasicBlock, BasicBlockData<'tcx>>,
}

impl<'tcx> CFG<'tcx> {
    fn block_data(&self, blk: BasicBlock) -> &BasicBlockData<'tcx> {
        &self.basic_blocks[blk]
    }

    fn block_data_mut(&mut self, blk: BasicBlock) -> &mut BasicBlockData<'tcx> {
        &mut self.basic_blocks[blk]
    }

    #[inline(never)]
    fn start_new_block(&mut self) -> BasicBlock {
        self.basic_blocks.push(BasicBlockData::new(None, false))
    }

    fn push(&mut self, block: BasicBlock, statement: Statement<'tcx>) {
        self.block_data_mut(block).statements.push(statement);
    }

    fn push_assign(&mut self, block: BasicBlock, place: Place<'tcx>, rvalue: Rvalue<'tcx>) {
        let source_info = SourceInfo {
            span: DUMMY_SP,
            scope: OUTERMOST_SOURCE_SCOPE,
        };
        self.push(
            block,
            Statement {
                source_info,
                kind: StatementKind::Assign(Box::new((place, rvalue))),
            },
        );
    }

    fn terminate(&mut self, block: BasicBlock, kind: TerminatorKind<'tcx>) {
        let source_info = SourceInfo {
            span: DUMMY_SP,
            scope: OUTERMOST_SOURCE_SCOPE,
        };
        let terminator = &mut self.block_data_mut(block).terminator;
        assert!(terminator.is_none());
        *terminator = Some(Terminator { source_info, kind });
    }

    fn goto(&mut self, origin: BasicBlock, target: BasicBlock) {
        self.terminate(origin, TerminatorKind::Goto { target })
    }
}

impl<'a, 'tcx> Builder<'a, 'tcx> {
    fn as_local(&mut self, mut block: BasicBlock, expr_id: ExprId) -> BlockAnd<Local> {
        let expr = &self.thir[expr_id];
        match expr.kind {
            ExprKind::Scope {
                region_scope,
                value,
                ..
            } => {
                self.scopes.push(region_scope.local_id);
                let res = self.as_local(block, value);
                self.scopes.pop();
                res
            }
            ExprKind::Box { .. } => panic!(),
            ExprKind::If {
                cond,
                then,
                else_opt,
                ..
            } => {
                let local_decl = LocalDecl::new(expr.ty, expr.span);
                let local = self.local_decls.push(local_decl);
                let target = self.cfg.start_new_block();

                let true_block = self.cfg.start_new_block();
                let then_block;
                let then = unpack!(then_block = self.as_local(true_block, then));
                self.cfg
                    .push_assign(then_block, Place::from(local), Rvalue::Use(operand(then)));
                self.cfg.goto(then_block, target);

                let false_block = if let Some(els) = else_opt {
                    let false_block = self.cfg.start_new_block();
                    let else_block;
                    let els = unpack!(else_block = self.as_local(false_block, els));
                    self.cfg
                        .push_assign(else_block, Place::from(local), Rvalue::Use(operand(els)));
                    self.cfg.goto(else_block, target);
                    false_block
                } else {
                    target
                };

                let cond = unpack!(block = self.as_local(block, cond));
                self.cfg.terminate(
                    block,
                    TerminatorKind::SwitchInt {
                        discr: operand(cond),
                        targets: SwitchTargets::new([(0, false_block)].into_iter(), true_block),
                    },
                );

                target.and(local)
            }
            ExprKind::Call { fun, ref args, .. } => {
                let fun_expr = &self.thir[fun];
                let func = if let ExprKind::ZstLiteral { .. } = fun_expr.kind {
                    let const_ = Const::Val(ConstValue::ZeroSized, fun_expr.ty);
                    const_operand(const_, fun_expr.span)
                } else {
                    let fun = unpack!(block = self.as_local(block, fun));
                    Operand::Move(fun.into())
                };
                let args: Box<[_]> = args
                    .into_iter()
                    .copied()
                    .map(|arg| {
                        let local = unpack!(block = self.as_local(block, arg));
                        Spanned {
                            node: operand(local),
                            span: self.thir[arg].span,
                        }
                    })
                    .collect();
                let local_decl = LocalDecl::new(expr.ty, expr.span);
                let local = self.local_decls.push(local_decl);
                let success = self.cfg.start_new_block();
                self.cfg.terminate(
                    block,
                    TerminatorKind::Call {
                        func,
                        args,
                        destination: local.into(),
                        target: if expr.ty.is_never() {
                            None
                        } else {
                            Some(success)
                        },
                        unwind: UnwindAction::Continue,
                        call_source: CallSource::Normal,
                        fn_span: DUMMY_SP,
                    },
                );
                success.and(local)
            }
            ExprKind::ByUse { .. } => panic!(),
            ExprKind::Deref { arg } => {
                let arg = unpack!(block = self.as_local(block, arg));
                let arg = Place {
                    local: arg,
                    projection: self.tcx.mk_place_elems(&[PlaceElem::Deref]),
                };
                let rvalue = Rvalue::Use(Operand::Move(arg));
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Binary { op, lhs, rhs } => {
                let lhs = unpack!(block = self.as_local(block, lhs));
                let rhs = unpack!(block = self.as_local(block, rhs));
                let rvalue = Rvalue::BinaryOp(op, Box::new((operand(lhs), operand(rhs))));
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::LogicalOp { op, lhs, rhs } => {
                let local_decl = LocalDecl::new(expr.ty, expr.span);
                let local = self.local_decls.push(local_decl);

                let true_block = self.cfg.start_new_block();
                let false_block = self.cfg.start_new_block();

                let lhs = unpack!(block = self.as_local(block, lhs));
                self.cfg.terminate(
                    block,
                    TerminatorKind::SwitchInt {
                        discr: operand(lhs),
                        targets: SwitchTargets::new([(0, false_block)].into_iter(), true_block),
                    },
                );

                let (short_circuit, continuation, constant) = match op {
                    LogicalOp::And => (false_block, true_block, false),
                    LogicalOp::Or => (true_block, false_block, true),
                };

                let constant = const_operand(Const::from_bool(self.tcx, constant), expr.span);
                self.cfg
                    .push_assign(short_circuit, Place::from(local), Rvalue::Use(constant));

                let rhs_block;
                let rhs = unpack!(rhs_block = self.as_local(continuation, rhs));
                self.cfg
                    .push_assign(rhs_block, Place::from(local), Rvalue::Use(operand(rhs)));

                let target = self.cfg.start_new_block();
                self.cfg.goto(short_circuit, target);
                self.cfg.goto(rhs_block, target);
                target.and(local)
            }
            ExprKind::Unary { op, arg } => {
                let arg = unpack!(block = self.as_local(block, arg));
                let rvalue = Rvalue::UnaryOp(op, operand(arg));
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Cast { source } => {
                let source_expr = &self.thir[source];
                let source = unpack!(block = self.as_local(block, source));
                let ty = if let TyKind::Adt(adt_def, ..) = source_expr.ty.kind()
                    && adt_def.is_enum()
                {
                    todo!()
                } else {
                    source_expr.ty
                };
                let cast_kind = rustc_middle::ty::cast::mir_cast_kind(ty, expr.ty);
                let rvalue = Rvalue::Cast(cast_kind, operand(source), expr.ty);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Use { source } => self.as_local(block, source),
            ExprKind::NeverToAny { source } => {
                let source_expr = &self.thir[source];
                let is_call = matches!(
                    source_expr.kind,
                    ExprKind::Call { .. } | ExprKind::InlineAsm { .. }
                );
                let local = unpack!(block = self.as_local(block, source));
                if is_call {
                    block.and(local)
                } else {
                    self.cfg.terminate(block, TerminatorKind::Unreachable);
                    let end_block = self.cfg.start_new_block();
                    end_block.and(local)
                }
            }
            ExprKind::PointerCoercion {
                cast,
                source,
                is_from_as_cast,
            } => {
                let source = unpack!(block = self.as_local(block, source));
                let origin = if is_from_as_cast {
                    CoercionSource::AsCast
                } else {
                    CoercionSource::Implicit
                };
                let rvalue = Rvalue::Cast(
                    CastKind::PointerCoercion(cast, origin),
                    operand(source),
                    expr.ty,
                );
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Loop { body } => {
                let loop_block = self.cfg.start_new_block();
                self.cfg.goto(block, loop_block);

                let local_decl = LocalDecl::new(expr.ty, expr.span);
                let local = self.local_decls.push(local_decl);

                let target = self.cfg.start_new_block();
                let scope = *self.scopes.last().unwrap();
                let breakable_scope = BreakableScope {
                    dest: local,
                    target,
                    head: Some(loop_block),
                };
                self.breakable_scopes.insert(scope, breakable_scope);

                unpack!(block = self.as_local(loop_block, body));
                self.cfg.goto(block, loop_block);

                self.breakable_scopes.remove(&scope);
                target.and(local)
            }
            ExprKind::Let { .. } => {
                todo!();
            }
            ExprKind::Match { .. } => {
                todo!();
            }
            ExprKind::Block { block: thir_block } => {
                let local_decl = LocalDecl::new(expr.ty, expr.span);
                let local = self.local_decls.push(local_decl);

                let thir_block = &self.thir[thir_block];
                let target = if thir_block.targeted_by_break {
                    let target = self.cfg.start_new_block();
                    let braekable_scope = BreakableScope {
                        dest: local,
                        target,
                        head: None,
                    };
                    self.breakable_scopes
                        .insert(thir_block.region_scope.local_id, braekable_scope);
                    Some(target)
                } else {
                    None
                };

                for stmt in &thir_block.stmts {
                    let stmt = &self.thir[*stmt];
                    match stmt.kind {
                        StmtKind::Expr { expr, .. } => {
                            unpack!(block = self.as_local(block, expr));
                        }
                        StmtKind::Let { .. } => {
                            todo!()
                        }
                    }
                }

                if let Some(expr) = &thir_block.expr {
                    let res = unpack!(block = self.as_local(block, *expr));
                    self.cfg
                        .push_assign(block, Place::from(local), Rvalue::Use(operand(res)));
                } else {
                    let unit = Rvalue::Aggregate(Box::new(AggregateKind::Tuple), IndexVec::new());
                    self.cfg.push_assign(block, Place::from(local), unit);
                }

                self.breakable_scopes
                    .remove(&thir_block.region_scope.local_id);

                if let Some(target) = target {
                    self.cfg.goto(block, target);
                    target.and(local)
                } else {
                    block.and(local)
                }
            }
            ExprKind::Assign { lhs, rhs } => {
                let rhs = unpack!(block = self.as_local(block, rhs));
                let lhs = unpack!(block = self.as_lvalue(block, lhs)).to_place(self);
                self.cfg.push_assign(block, lhs, Rvalue::Use(operand(rhs)));
                block.and(self.dummy_local)
            }
            ExprKind::AssignOp { op, lhs, rhs } => {
                let rhs = unpack!(block = self.as_local(block, rhs));
                let lhs = unpack!(block = self.as_lvalue(block, lhs)).to_place(self);
                let rvalue =
                    Rvalue::BinaryOp(op.into(), Box::new((Operand::Copy(lhs), operand(rhs))));
                self.cfg.push_assign(block, lhs, rvalue);
                block.and(self.dummy_local)
            }
            ExprKind::Field {
                lhs,
                variant_index,
                name,
            } => {
                let lhs_expr = &self.thir[lhs];
                let mut projections = vec![];
                if let TyKind::Adt(adt_def, _) = lhs_expr.ty.kind() {
                    if adt_def.is_enum() {
                        projections.push(PlaceElem::Downcast(
                            Some(adt_def.variant(variant_index).name),
                            variant_index,
                        ));
                    }
                }
                projections.push(PlaceElem::Field(name, expr.ty));
                let lhs = unpack!(block = self.as_local(block, lhs));
                let place = Place {
                    local: lhs,
                    projection: self.tcx.mk_place_elems(&projections),
                };
                let rvalue = Rvalue::Use(Operand::Move(place));
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Index { lhs, index } => {
                let lhs = unpack!(block = self.as_local(block, lhs));
                let index = unpack!(block = self.as_local(block, index));
                let place = Place {
                    local: lhs,
                    projection: self.tcx.mk_place_elems(&[PlaceElem::Index(index)]),
                };
                let rvalue = Rvalue::Use(Operand::Move(place));
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::VarRef { id } => block.and(self.var_indices[&id]),
            ExprKind::UpvarRef { .. } => {
                todo!();
            }
            ExprKind::Borrow { borrow_kind, arg } => {
                let arg = unpack!(block = self.as_lvalue(block, arg)).to_place(self);
                let rvalue = Rvalue::Ref(self.tcx.lifetimes.re_erased, borrow_kind, arg);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::RawBorrow { mutability, arg } => {
                let arg = unpack!(block = self.as_lvalue(block, arg)).to_place(self);
                let rvalue = Rvalue::RawPtr(mutability.into(), arg);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Break { label, value } => {
                let breakable_scope = self.breakable_scopes[&label.local_id];
                if let Some(value) = value {
                    let value = unpack!(block = self.as_local(block, value));
                    self.cfg.push_assign(
                        block,
                        Place::from(breakable_scope.dest),
                        Rvalue::Use(operand(value)),
                    );
                }
                self.cfg.goto(block, breakable_scope.target);
                self.cfg.start_new_block().and(self.dummy_local)
            }
            ExprKind::Continue { label } => {
                let breakable_scope = self.breakable_scopes[&label.local_id];
                self.cfg.goto(block, breakable_scope.head.unwrap());
                self.cfg.start_new_block().and(self.dummy_local)
            }
            ExprKind::Return { value } => {
                if let Some(value) = value {
                    let value = unpack!(block = self.as_local(block, value));
                    let rvalue = Rvalue::Use(operand(value));
                    self.cfg
                        .push_assign(block, Place::from(Local::ZERO), rvalue);
                }
                self.cfg.terminate(block, TerminatorKind::Return);
                self.cfg.start_new_block().and(self.dummy_local)
            }
            ExprKind::Become { .. } => panic!(),
            ExprKind::ConstBlock { did, args } => {
                let uneval = UnevaluatedConst::new(did, args);
                let const_ = Const::Unevaluated(uneval, expr.ty);
                let operand = const_operand(const_, expr.span);
                let rvalue = Rvalue::Use(operand);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Repeat { value, count } => {
                // TODO? insert drop when repeating non-constant zero times
                let value = unpack!(block = self.as_local(block, value));
                let rvalue = Rvalue::Repeat(operand(value), count);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Array { ref fields } => {
                let el_ty = expr.ty.sequence_element_type(self.tcx);
                let fields: IndexVec<FieldIdx, _> = fields
                    .into_iter()
                    .copied()
                    .map(|f| operand(unpack!(block = self.as_local(block, f))))
                    .collect();
                let rvalue = Rvalue::Aggregate(Box::new(AggregateKind::Array(el_ty)), fields);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Tuple { ref fields } => {
                let fields: IndexVec<FieldIdx, _> = fields
                    .into_iter()
                    .copied()
                    .map(|f| operand(unpack!(block = self.as_local(block, f))))
                    .collect();
                let value = Rvalue::Aggregate(Box::new(AggregateKind::Tuple), fields);
                self.rvalue_to_local(block, expr, value)
            }
            ExprKind::Adt(box AdtExpr {
                adt_def,
                variant_index,
                args,
                ref fields,
                ref base,
                ..
            }) => {
                let is_union = adt_def.is_union();
                let active_field_index = is_union.then(|| fields[0].name);

                let fields_map: FxHashMap<_, _> = fields
                    .into_iter()
                    .map(|f| {
                        let f_local = unpack!(block = self.as_local(block, f.expr));
                        (f.name, operand(f_local))
                    })
                    .collect();

                let variant = adt_def.variant(variant_index);
                let field_names = variant.fields.indices();

                let fields = match base {
                    AdtExprBase::None => field_names
                        .filter_map(|n| fields_map.get(&n).cloned())
                        .collect(),
                    AdtExprBase::Base(FruInfo { .. }) => todo!(),
                    AdtExprBase::DefaultFields(_) => todo!(),
                };

                let adt = Box::new(AggregateKind::Adt(
                    adt_def.did(),
                    variant_index,
                    args,
                    None,
                    active_field_index,
                ));
                let rvalue = Rvalue::Aggregate(adt, fields);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::PlaceTypeAscription { .. } => panic!(),
            ExprKind::ValueTypeAscription { .. } => panic!(),
            ExprKind::PlaceUnwrapUnsafeBinder { .. } => panic!(),
            ExprKind::ValueUnwrapUnsafeBinder { .. } => panic!(),
            ExprKind::WrapUnsafeBinder { .. } => panic!(),
            ExprKind::Closure(_) => {
                todo!();
            }
            ExprKind::Literal { lit, neg } => {
                let trunc = |n| {
                    let width = expr.ty.primitive_size(self.tcx);
                    let result = width.truncate(n);
                    ConstValue::Scalar(Scalar::from_uint(result, width))
                };

                let value = match (&lit.node, expr.ty.kind()) {
                    (ast::LitKind::Str(s, _), TyKind::Ref(_, inner_ty, _)) if inner_ty.is_str() => {
                        let s = s.as_str();
                        let allocation =
                            Allocation::from_bytes_byte_aligned_immutable(s.as_bytes(), ());
                        let allocation = self.tcx.mk_const_alloc(allocation);
                        ConstValue::Slice {
                            data: allocation,
                            meta: allocation.inner().size().bytes(),
                        }
                    }
                    (ast::LitKind::ByteStr(data, _), TyKind::Ref(_, inner_ty, _))
                        if matches!(inner_ty.kind(), TyKind::Slice(_)) =>
                    {
                        let allocation =
                            Allocation::from_bytes_byte_aligned_immutable(data as &[u8], ());
                        let allocation = self.tcx.mk_const_alloc(allocation);
                        ConstValue::Slice {
                            data: allocation,
                            meta: allocation.inner().size().bytes(),
                        }
                    }
                    (ast::LitKind::ByteStr(data, _), TyKind::Ref(_, inner_ty, _))
                        if inner_ty.is_array() =>
                    {
                        let id = self.tcx.allocate_bytes_dedup(
                            data,
                            rustc_const_eval::interpret::CTFE_ALLOC_SALT,
                        );
                        ConstValue::Scalar(Scalar::from_pointer(id.into(), &self.tcx))
                    }
                    (ast::LitKind::CStr(data, _), TyKind::Ref(_, inner_ty, _)) if matches!(inner_ty.kind(), TyKind::Adt(def, _) if self.tcx.is_lang_item(def.did(), LangItem::CStr)) =>
                    {
                        let allocation =
                            Allocation::from_bytes_byte_aligned_immutable(data as &[u8], ());
                        let allocation = self.tcx.mk_const_alloc(allocation);
                        ConstValue::Slice {
                            data: allocation,
                            meta: allocation.inner().size().bytes(),
                        }
                    }
                    (ast::LitKind::Byte(n), TyKind::Uint(UintTy::U8)) => {
                        ConstValue::Scalar(Scalar::from_uint(*n, Size::from_bytes(1)))
                    }
                    (ast::LitKind::Int(n, _), TyKind::Uint(_)) if !neg => trunc(n.get()),
                    (ast::LitKind::Int(n, _), TyKind::Int(_)) => trunc(if neg {
                        (n.get() as i128).overflowing_neg().0 as u128
                    } else {
                        n.get()
                    }),
                    (ast::LitKind::Float(n, _), TyKind::Float(fty)) => {
                        parse_float_into_constval(*n, *fty, neg).unwrap()
                    }
                    (ast::LitKind::Bool(b), TyKind::Bool) => {
                        ConstValue::Scalar(Scalar::from_bool(*b))
                    }
                    (ast::LitKind::Char(c), TyKind::Char) => {
                        ConstValue::Scalar(Scalar::from_char(*c))
                    }
                    _ => panic!(),
                };
                let const_ = Const::Val(value, expr.ty);
                let operand = const_operand(const_, expr.span);
                let rvalue = Rvalue::Use(operand);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::NonHirLiteral { .. } => panic!(),
            ExprKind::ZstLiteral { .. } => {
                let const_ = Const::Val(ConstValue::ZeroSized, expr.ty);
                let operand = const_operand(const_, expr.span);
                let rvalue = Rvalue::Use(operand);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::NamedConst { .. } => panic!(),
            ExprKind::ConstParam { .. } => panic!(),
            ExprKind::StaticRef { alloc_id, ty, .. } => {
                let const_val =
                    ConstValue::Scalar(Scalar::from_pointer(alloc_id.into(), &self.tcx));
                let const_ = Const::Val(const_val, ty);
                let operand = const_operand(const_, expr.span);
                let rvalue = Rvalue::Use(operand);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::InlineAsm(box InlineAsmExpr { .. }) => {
                todo!();
            }
            ExprKind::OffsetOf { container, fields } => {
                let rvalue = Rvalue::NullaryOp(NullOp::OffsetOf(fields), container);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::ThreadLocalRef(did) => {
                let rvalue = Rvalue::ThreadLocalRef(did);
                self.rvalue_to_local(block, expr, rvalue)
            }
            ExprKind::Yield { .. } => {
                todo!()
            }
        }
    }

    fn as_lvalue(
        &mut self,
        mut block: BasicBlock,
        expr_id: ExprId,
    ) -> BlockAnd<PlaceBuilder<'tcx>> {
        let expr = &self.thir[expr_id];
        match expr.kind {
            ExprKind::Scope {
                region_scope,
                value,
                ..
            } => {
                self.scopes.push(region_scope.local_id);
                let res = self.as_lvalue(block, value);
                self.scopes.pop();
                res
            }
            ExprKind::Field {
                lhs,
                variant_index,
                name,
            } => {
                let mut place_builder = unpack!(block = self.as_lvalue(block, lhs));
                let lhs_expr = &self.thir[lhs];
                if let TyKind::Adt(adt_def, _) = lhs_expr.ty.kind() {
                    if adt_def.is_enum() {
                        place_builder = place_builder.downcast(*adt_def, variant_index);
                    }
                }
                block.and(place_builder.field(name, expr.ty))
            }
            ExprKind::Deref { arg } => {
                let local = unpack!(block = self.as_local(block, arg));
                let place_builder = PlaceBuilder::from(local);
                block.and(place_builder.deref())
            }
            ExprKind::Index { lhs, index } => {
                let place_builder = unpack!(block = self.as_lvalue(block, lhs));
                let index = unpack!(block = self.as_local(block, index));
                block.and(place_builder.index(index))
            }
            ExprKind::UpvarRef { .. } => {
                todo!()
            }
            ExprKind::VarRef { id } => {
                let local = self.var_indices[&id];
                block.and(PlaceBuilder::from(local))
            }
            ExprKind::PlaceTypeAscription { .. } => panic!(),
            ExprKind::ValueTypeAscription { .. } => panic!(),
            ExprKind::PlaceUnwrapUnsafeBinder { .. } => panic!(),
            ExprKind::ValueUnwrapUnsafeBinder { .. } => panic!(),

            ExprKind::Array { .. }
            | ExprKind::Tuple { .. }
            | ExprKind::Adt { .. }
            | ExprKind::Closure { .. }
            | ExprKind::Unary { .. }
            | ExprKind::Binary { .. }
            | ExprKind::LogicalOp { .. }
            | ExprKind::Box { .. }
            | ExprKind::Cast { .. }
            | ExprKind::Use { .. }
            | ExprKind::NeverToAny { .. }
            | ExprKind::PointerCoercion { .. }
            | ExprKind::Repeat { .. }
            | ExprKind::Borrow { .. }
            | ExprKind::RawBorrow { .. }
            | ExprKind::Match { .. }
            | ExprKind::If { .. }
            | ExprKind::Loop { .. }
            | ExprKind::Block { .. }
            | ExprKind::Let { .. }
            | ExprKind::Assign { .. }
            | ExprKind::AssignOp { .. }
            | ExprKind::Break { .. }
            | ExprKind::Continue { .. }
            | ExprKind::Return { .. }
            | ExprKind::Become { .. }
            | ExprKind::Literal { .. }
            | ExprKind::NamedConst { .. }
            | ExprKind::NonHirLiteral { .. }
            | ExprKind::ZstLiteral { .. }
            | ExprKind::ConstParam { .. }
            | ExprKind::ConstBlock { .. }
            | ExprKind::StaticRef { .. }
            | ExprKind::InlineAsm { .. }
            | ExprKind::OffsetOf { .. }
            | ExprKind::Yield { .. }
            | ExprKind::ThreadLocalRef(_)
            | ExprKind::Call { .. }
            | ExprKind::ByUse { .. }
            | ExprKind::WrapUnsafeBinder { .. } => {
                let local = unpack!(block = self.as_local(block, expr_id));
                block.and(PlaceBuilder::from(local))
            }
        }
    }

    fn rvalue_to_local(
        &mut self,
        block: BasicBlock,
        expr: &'a Expr<'tcx>,
        rvalue: Rvalue<'tcx>,
    ) -> BlockAnd<Local> {
        let local_decl = LocalDecl::new(expr.ty, expr.span);
        let local = self.local_decls.push(local_decl);
        self.cfg.push_assign(block, Place::from(local), rvalue);
        block.and(local)
    }
}

fn operand<'tcx>(local: Local) -> Operand<'tcx> {
    Operand::Move(local.into())
}

fn const_operand<'tcx>(const_: Const<'tcx>, span: Span) -> Operand<'tcx> {
    let operand = ConstOperand {
        span,
        user_ty: None,
        const_,
    };
    Operand::Constant(Box::new(operand))
}

struct BlockAnd<T>(BasicBlock, T);

trait BlockAndExtension {
    fn and<T>(self, v: T) -> BlockAnd<T>;
}

impl BlockAndExtension for BasicBlock {
    fn and<T>(self, v: T) -> BlockAnd<T> {
        BlockAnd(self, v)
    }
}

fn parse_float_into_constval<'tcx>(
    num: Symbol,
    float_ty: FloatTy,
    neg: bool,
) -> Option<ConstValue<'tcx>> {
    parse_float_into_scalar(num, float_ty, neg).map(|s| ConstValue::Scalar(s.into()))
}

fn parse_float_into_scalar(num: Symbol, float_ty: FloatTy, neg: bool) -> Option<ScalarInt> {
    let num = num.as_str();
    match float_ty {
        FloatTy::F16 => {
            let mut f = num.parse::<Half>().ok()?;
            if neg {
                f = -f;
            }
            Some(ScalarInt::from(f))
        }
        FloatTy::F32 => {
            let Ok(rust_f) = num.parse::<f32>() else { return None };
            let mut f = num
                .parse::<Single>()
                .unwrap_or_else(|e| panic!("apfloat::ieee::Single failed to parse `{num}`: {e:?}"));

            assert!(
                u128::from(rust_f.to_bits()) == f.to_bits(),
                "apfloat::ieee::Single gave different result for `{}`: \
                 {}({:#x}) vs Rust's {}({:#x})",
                rust_f,
                f,
                f.to_bits(),
                Single::from_bits(rust_f.to_bits().into()),
                rust_f.to_bits()
            );

            if neg {
                f = -f;
            }

            Some(ScalarInt::from(f))
        }
        FloatTy::F64 => {
            let Ok(rust_f) = num.parse::<f64>() else { return None };
            let mut f = num
                .parse::<Double>()
                .unwrap_or_else(|e| panic!("apfloat::ieee::Double failed to parse `{num}`: {e:?}"));

            assert!(
                u128::from(rust_f.to_bits()) == f.to_bits(),
                "apfloat::ieee::Double gave different result for `{}`: \
                 {}({:#x}) vs Rust's {}({:#x})",
                rust_f,
                f,
                f.to_bits(),
                Double::from_bits(rust_f.to_bits().into()),
                rust_f.to_bits()
            );

            if neg {
                f = -f;
            }

            Some(ScalarInt::from(f))
        }
        FloatTy::F128 => {
            let mut f = num.parse::<Quad>().ok()?;
            if neg {
                f = -f;
            }
            Some(ScalarInt::from(f))
        }
    }
}

fn to_upvars_resolved_place_builder<'tcx>(
    _cx: &Builder<'_, 'tcx>,
    _var_hir_id: LocalVarId,
    _closure_def_id: LocalDefId,
    _projection: &[PlaceElem<'tcx>],
) -> Option<PlaceBuilder<'tcx>> {
    todo!()
}
