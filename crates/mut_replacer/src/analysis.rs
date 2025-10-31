use rustc_abi::{FieldIdx, VariantIdx};
use rustc_hash::FxHashMap;
use rustc_hir::{ItemKind, def_id::LocalDefId};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{Body, ConstOperand, Local, LocalDecl, Operand, Place, Rvalue, Statement, StatementKind},
    ty::{GenericArgKind, List, Ty, TyCtxt, TyKind},
};

rustc_index::newtype_index! {
    #[debug_format = "M{}"]
    struct MutId {}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Field {
    def_id: LocalDefId,
    variant: VariantIdx,
    field: FieldIdx,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LocalVar {
    def_id: LocalDefId,
    local: Local,
}

pub fn analyze(tcx: TyCtxt<'_>) {
    let mut next_id = MutId::ZERO;
    let mut fields = FxHashMap::default();
    let mut locals = FxHashMap::default();
    for item_id in tcx.hir_free_items() {
        let def_id = item_id.owner_id.def_id;
        let item = tcx.hir_item(item_id);
        match &item.kind {
            ItemKind::Enum(..) | ItemKind::Struct(..) | ItemKind::Union(..) => {
                let adt_def = tcx.adt_def(def_id);
                for (variant, vd) in adt_def.variants().iter_enumerated() {
                    for (field, fd) in vd.fields.iter_enumerated() {
                        let ty = fd.ty(tcx, List::empty());
                        let count = count_muts(ty);
                        if count != 0 {
                            let field = Field {
                                def_id,
                                variant,
                                field,
                            };
                            fields.insert(field, (next_id, count));
                            next_id += count;
                        }
                    }
                }
            }
            ItemKind::Fn { .. } => {
                let body = tcx.mir_drops_elaborated_and_const_checked(def_id);
                let body: &Body<'_> = &body.borrow();
                for (local, ld) in body.local_decls.iter_enumerated() {
                    let count = count_muts(ld.ty);
                    if count != 0 {
                        let local = LocalVar { def_id, local };
                        locals.insert(local, (next_id, count));
                        next_id += count;
                    }
                }
            }
            _ => {}
        }
    }

    let mut analyzer = Analyzer { tcx };
    for def_id in tcx.hir_body_owners() {
        let body = tcx.mir_drops_elaborated_and_const_checked(def_id);
        let body: &Body<'_> = &body.borrow();
        let ctx = Ctx {
            local_decls: &body.local_decls,
            fields: &fields,
            locals: &locals,
        };
        for bbd in body.basic_blocks.iter() {
            for stmt in &bbd.statements {
                analyzer.transfer_statement(stmt, ctx);
            }
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
struct Ctx<'a, 'tcx> {
    local_decls: &'a IndexVec<Local, LocalDecl<'tcx>>,
    fields: &'a FxHashMap<Field, (MutId, usize)>,
    locals: &'a FxHashMap<LocalVar, (MutId, usize)>,
}

struct Analyzer<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> Analyzer<'tcx> {
    fn transfer_statement(&mut self, stmt: &Statement<'tcx>, ctx: Ctx<'_, 'tcx>) {
        let StatementKind::Assign(box (l, r)) = &stmt.kind else { return };
        match r {
            Rvalue::Use(operand) => {
                self.transfer_operand(*l, operand, ctx);
            }
            Rvalue::Repeat(operand, _) => {
                self.transfer_operand(*l, operand, ctx);
            }
            Rvalue::Ref(..) => {}
            Rvalue::ThreadLocalRef(..) => {}
            Rvalue::RawPtr(..) => {}
            Rvalue::Len(..) => {}
            Rvalue::Cast(_, operand, _) => {
                self.transfer_operand(*l, operand, ctx);
            }
            Rvalue::BinaryOp(..) => {}
            Rvalue::NullaryOp(..) => {}
            Rvalue::UnaryOp(..) => {}
            Rvalue::Discriminant(..) => {}
            Rvalue::Aggregate(_kind, _operands) => {
                // TODO
            }
            Rvalue::ShallowInitBox(..) => panic!(),
            Rvalue::CopyForDeref(place) => {
                self.transfer_place(*l, *place, ctx);
            }
            Rvalue::WrapUnsafeBinder(..) => panic!(),
        }
    }

    fn transfer_operand(&mut self, l: Place<'tcx>, r: &Operand<'tcx>, ctx: Ctx<'_, 'tcx>) {
        match r {
            Operand::Copy(place) | Operand::Move(place) => {
                self.transfer_place(l, *place, ctx);
            }
            Operand::Constant(box const_) => {
                self.transfer_constant(l, *const_, ctx);
            }
        }
    }

    fn transfer_place(&mut self, l: Place<'tcx>, r: Place<'tcx>, ctx: Ctx<'_, 'tcx>) {
        let lty = l.ty(ctx.local_decls, self.tcx).ty;
        let rty = r.ty(ctx.local_decls, self.tcx).ty;
        if count_muts(lty) == 0 && count_muts(rty) == 0 {
            return;
        }
        match (
            l.is_indirect_first_projection(),
            r.is_indirect_first_projection(),
        ) {
            (false, false) => {
                // TODO
            }
            (false, true) => {
                // TODO
            }
            (true, false) => {
                // TODO
            }
            (true, true) => panic!(),
        }
    }

    fn transfer_constant(&mut self, _l: Place<'tcx>, _r: ConstOperand<'tcx>, _ctx: Ctx<'_, 'tcx>) {}
}

fn count_muts(ty: Ty<'_>) -> usize {
    match ty.kind() {
        TyKind::RawPtr(ty, _) | TyKind::Ref(_, ty, _) => count_muts(*ty) + 1,
        TyKind::FnPtr(binder, _) => {
            let fn_sig_tys = binder.skip_binder();
            fn_sig_tys
                .inputs()
                .iter()
                .map(|ty| count_muts(*ty))
                .sum::<usize>()
                + count_muts(fn_sig_tys.output())
        }
        TyKind::UnsafeBinder(inner) => count_muts(inner.skip_binder()),
        TyKind::Slice(ty) | TyKind::Array(ty, _) => count_muts(*ty),
        TyKind::Tuple(tys) => tys.iter().map(count_muts).sum(),
        TyKind::Adt(_, generics) => generics
            .iter()
            .map(|arg| {
                if let GenericArgKind::Type(ty) = arg.kind() {
                    count_muts(ty)
                } else {
                    0
                }
            })
            .sum(),
        TyKind::Bool
        | TyKind::Char
        | TyKind::Int(..)
        | TyKind::Uint(..)
        | TyKind::Float(..)
        | TyKind::Foreign(..)
        | TyKind::Str
        | TyKind::Pat(..)
        | TyKind::FnDef(..)
        | TyKind::Never
        | TyKind::Closure(..)
        | TyKind::Dynamic(..)
        | TyKind::Alias(..)
        | TyKind::Param(..)
        | TyKind::Bound(..)
        | TyKind::CoroutineClosure(..)
        | TyKind::Coroutine(..)
        | TyKind::CoroutineWitness(..)
        | TyKind::Placeholder(..)
        | TyKind::Infer(..)
        | TyKind::Error(..) => 0,
    }
}
