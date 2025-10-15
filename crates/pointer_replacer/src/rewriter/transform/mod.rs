use itertools::izip;
use regex::Regex;
use rustc_ast::{
    HasNodeId, ItemKind,
    ast::*,
    mut_visit::{self, MutVisitor},
    ptr::P,
};
use rustc_ast_pretty::pprust;
use rustc_hash::FxHashMap;
use rustc_hir::{
    Expr as HirExpr, ExprKind as HirExprKind, HirId, Node as HirNode, PatKind as HirPatKind, QPath,
    StmtKind as HirStmtKind, def::Res, def_id::DefId,
};
use rustc_middle::ty::{Ty as MirTy, TyCtxt};
use rustc_span::symbol::Ident;
use smallvec::SmallVec;
use utils::ir::AstToHir;

use super::{
    Analysis,
    collector::collect_diffs,
    decision::{PtrKind, PtrKindDiff, SigDecisions},
};
use crate::utils::rustc::RustProgram;

pub mod post;

#[derive(Clone, Copy, Debug, Default)]
pub struct RewriteStats {
    pub params: usize, // number of function parameters changed
    pub defs: usize,   // number of variable definitions changed
    pub writes: usize, // number of variable writes changed
    pub usages: usize, // number of variable usages changed
}

impl std::ops::Add for RewriteStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RewriteStats {
            params: self.params + other.params,
            defs: self.defs + other.defs,
            writes: self.writes + other.writes,
            usages: self.usages + other.usages,
        }
    }
}

pub(crate) struct TransformVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    sig_decs: SigDecisions,
    ptr_diffs: FxHashMap<HirId, PtrKindDiff>,
    ast_to_hir: AstToHir,
    pub stats: RewriteStats,
}

impl MutVisitor for TransformVisitor<'_> {
    fn visit_item(&mut self, item: &mut Item) {
        let node_id = item.node_id();
        let ItemKind::Fn(box fn_item) = &mut item.kind else {
            return mut_visit::walk_item(self, item);
        };
        let hir_item = self
            .ast_to_hir
            .get_item(node_id, self.tcx)
            .unwrap_or_else(|| panic!("Failed to find HIR item for Item {:?}", item.span));
        let local_def_id = hir_item.owner_id.def_id;
        let def_id = local_def_id.to_def_id();
        let mir_body = self
            .tcx
            .mir_drops_elaborated_and_const_checked(local_def_id)
            .borrow();

        // Rewrite function signature
        let local_input_decls = mir_body
            .args_iter()
            .map(|local| mir_body.local_decls[local].clone())
            .collect::<Vec<_>>();

        let input_len = self.sig_input_len(def_id); // exclude variadic arguments
        let sig_dec = self.sig_decs.expect(&def_id);

        // Currently intra-procedural borrow inference: skip return type, only consider parameters
        for (idx, (local_decl, param)) in
            izip!(local_input_decls, fn_item.sig.decl.inputs.iter_mut())
                .take(input_len)
                .enumerate()
        {
            let ty_res = mir_ty_to_ty(&local_decl.ty); // resolved type (no type alias)
            self.rewrite_ty(&mut param.ty, ty_res, &sig_dec.input_decs[idx]);
            self.stats.params += 1;
            if let PatKind::Ident(binding_mode, ..) = &mut param.pat.kind {
                *binding_mode = BindingMode::MUT;
            }
        }
        mut_visit::walk_item(self, item);
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Self::Result {
        let _expr = expr.clone();
        // visit children first; we can assume that the function arguments have been rewritten
        mut_visit::walk_expr(self, expr);
        let hir_expr_opt = self.get_hir_expr(expr);
        match &mut expr.kind {
            ExprKind::Assign(box lhs, box rhs, _) => {
                // assignment to dereferenced pointer
                if let HirExprKind::Assign(hir_lhs, _, _) = hir_expr_opt.unwrap().kind
                    && let HirExprKind::Unary(UnOp::Deref, hir_lhs_deref) = hir_lhs.kind
                    && let HirExprKind::Path(qpath) = &hir_lhs_deref.kind  // TODO: support multiple deref
                    && let QPath::Resolved(_, path) = qpath
                    && let Res::Local(local_id) = path.res
                    && let Some(ptr_diff) = self.ptr_diffs.get(&local_id)
                {
                    match ptr_diff {
                        PtrKindDiff {
                            before: PtrKind::Raw(_),
                            after: PtrKind::OptRef(true), // Option<&mut T>
                        } => {
                            let ExprKind::Unary(UnOp::Deref, lhs_deref) = &mut lhs.kind else {
                                unreachable!("Expected deref expression on LHS: {:?}", expr.span);
                            };
                            **lhs_deref = utils::expr!(
                                "{}.as_deref_mut().unwrap_or_else(|| panic!(\"attempted to deref a null pointer\"))",
                                pprust::expr_to_string(&*lhs_deref),
                            );
                            self.stats.writes += 1;
                        }
                        PtrKindDiff {
                            before: PtrKind::Raw(_),
                            after: PtrKind::OptRef(false), // Option<&T>
                        } => {
                            unreachable!(
                                "Immutable references cannot be assigned to: {:?}",
                                expr.span
                            );
                        }
                        _ => (),
                    }
                }
                // direct assignment
                else if let HirExprKind::Assign(hir_lhs, _rhs, _) = hir_expr_opt.unwrap().kind
                    && let HirExprKind::Path(qpath) = &hir_lhs.kind
                    && let QPath::Resolved(_, path) = qpath
                    && let Res::Local(local_id) = path.res
                    && let Some(ptr_diff) = self.ptr_diffs.get(&local_id)
                    && let PtrKindDiff {
                        before: PtrKind::Raw(mutability),
                        after: PtrKind::OptRef(_mutability),
                    } = ptr_diff
                {
                    assert!(mutability == _mutability);
                    *rhs = utils::expr!(
                        "Some(&{}*({}))",
                        if *mutability { "mut " } else { "" },
                        pprust::expr_to_string(&*rhs)
                    );
                    self.stats.writes += 1;
                }
            }
            ExprKind::Path(..) => {
                // usage site
                let hir_expr = hir_expr_opt.unwrap();
                if let HirExprKind::Path(qpath) = &hir_expr.kind
                    && let QPath::Resolved(_, path) = qpath
                    && let Res::Local(local_id) = path.res
                    && let Some(ptr_diff) = self.ptr_diffs.get(&local_id).cloned()
                {
                    let parent_node = self.expect_parent_node(hir_expr.hir_id);
                    if let HirNode::Expr(parent_expr) = parent_node {
                        match parent_expr.kind {
                            // HirExprKind::Call(..) => return, // handled in ExprKind::Call below
                            HirExprKind::Assign(lhs, _, _) => {
                                if lhs.hir_id == hir_expr.hir_id {
                                    // assignment to this variable, handled in ExprKind::Assign above
                                    return;
                                }
                            }
                            _ => {
                                let grandparent_node = self.expect_parent_node(parent_expr.hir_id);
                                if let HirNode::Expr(grandparent_expr) = grandparent_node
                                    && let HirExprKind::Assign(lhs, _, _) = grandparent_expr.kind
                                    && let HirExprKind::Unary(UnOp::Deref, lhs_deref) = &lhs.kind
                                    && lhs_deref.hir_id == hir_expr.hir_id
                                {
                                    // assignment to dereferenced pointer, handled in ExprKind::Assign above
                                    return;
                                }
                            }
                        }
                    }
                    match ptr_diff {
                        PtrKindDiff {
                            before: PtrKind::Raw(mutability),
                            after: PtrKind::OptRef(_mutability),
                        } => {
                            let ptr_ty = self.expr_ptr_ty(&*expr);
                            *expr = utils::expr!(
                                "({}).as_deref{}().map(|__r| __r as *{} _).unwrap_or(std::ptr::null{}::<{}>())",
                                pprust::expr_to_string(&*expr),
                                if mutability { "_mut" } else { "" },
                                if mutability { "mut" } else { "const" },
                                if mutability { "_mut" } else { "" },
                                pprust::ty_to_string(&ptr_ty)
                            );
                            self.stats.usages += 1;
                        }
                        PtrKindDiff {
                            before: PtrKind::OptRef(_),
                            after: PtrKind::Raw(_),
                        } => {
                            unreachable!("Raw pointers adapting to references: {:?}", expr.span);
                        }
                        _ => (),
                    }
                }
            }
            ExprKind::Call(box _func_expr, args) => {
                // function call
                let hir_expr = hir_expr_opt.unwrap();
                let typeck_res = self.tcx.typeck(hir_expr.hir_id.owner); // ensure typeck is available
                if let HirExprKind::Call(hir_func_expr, hir_args) = hir_expr.kind
                    && let HirExprKind::Path(func_qpath) = &hir_func_expr.kind
                    && let Res::Def(_, func_did) =
                        typeck_res.qpath_res(func_qpath, hir_func_expr.hir_id)
                    && let Some(sig_dec) = self.sig_decs.get(&func_did)
                {
                    let input_len = self.sig_input_len(func_did); // exclude variadic arguments
                    for (i, (arg, _hir_arg)) in izip!(args.iter_mut(), hir_args.iter())
                        .take(input_len)
                        .enumerate()
                    {
                        // Note: the arguments have been visited and rewritten to *mut T
                        // Hir arguments stays the same, so may not match the AST arguments
                        if let Some(PtrKind::OptRef(mutability)) = &sig_dec.input_decs.get(i).unwrap_or_else(|| {
                                panic!(
                                    "Function call argument index out of bounds: {} in {:?}, function: {:?}",
                                    i, expr.span, self.tcx.def_path_str(func_did)
                                )
                            }) {
                            if let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, box inner) = &arg.kind {
                                // `Some(&mut inner)` when arg is `&raw mut inner`
                                **arg = utils::expr!(
                                    "Some(&mut ({}))",
                                    pprust::expr_to_string(inner)
                                );
                            } else if let ExprKind::AddrOf(BorrowKind::Ref, _, box inner) = &arg.kind
                                   && let ExprKind::Unary(UnOp::Deref, _) = &inner.kind {
                                // `Some(arg)` when arg is `&mut inner` or `&inner`
                                // (c2rust is using automatic casting for &mut T to *mut T)
                                **arg = utils::expr!(
                                    "Some({})",
                                    pprust::expr_to_string(&*arg)
                                );
                            } else {
                                // arg.as_mut()
                                **arg = utils::expr!(
                                    "({}).as_{}()",
                                    pprust::expr_to_string(&*arg),
                                    if *mutability { "mut" } else { "ref" }
                                );
                            }
                            self.stats.usages += 1;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn flat_map_stmt(&mut self, stmt: Stmt) -> SmallVec<[Stmt; 1]> {
        let mut stmts = mut_visit::walk_flat_map_stmt(self, stmt);
        for stmt in &mut stmts {
            let hir_stmt_opt = self.get_hir_stmt(stmt);
            if let StmtKind::Let(box local) = &mut stmt.kind
                && let HirStmtKind::Let(hir_let) = hir_stmt_opt.unwrap().kind
                && let HirPatKind::Binding(_, binding_hir_id, _, _) = hir_let.pat.kind
                && let Some(ptr_diff) = self.ptr_diffs.get(&binding_hir_id)
                && let PtrKindDiff {
                    before: PtrKind::Raw(mutability),
                    after: PtrKind::OptRef(_mutability),
                } = ptr_diff
            {
                assert!(mutability == _mutability);
                let mutability = *mutability;
                if let Some(ty) = &mut local.ty {
                    let hir_ty = hir_let.ty.unwrap();
                    let typeck = self.tcx.typeck(hir_ty.hir_id.owner);
                    let hir_ty_res = typeck.node_type(hir_ty.hir_id);
                    let ty_res = mir_ty_to_ty(&hir_ty_res);
                    self.rewrite_ty(ty, ty_res, &Some(PtrKind::OptRef(mutability)));
                }
                match &mut local.kind {
                    LocalKind::Init(box rhs) | LocalKind::InitElse(box rhs, _) => {
                        *rhs = utils::expr!(
                            "({}).as_{}()",
                            pprust::expr_to_string(&*rhs),
                            if mutability { "mut" } else { "ref" }
                        );
                        self.stats.defs += 1;
                    }
                    LocalKind::Decl => {
                        // No initializer, do nothing
                    }
                }
            }
        }
        stmts
    }
}

impl<'tcx> TransformVisitor<'tcx> {
    pub fn new(
        rust_program: &RustProgram<'tcx>,
        analysis: &Analysis,
        ast_to_hir: AstToHir,
    ) -> TransformVisitor<'tcx> {
        let sig_decs = SigDecisions::new(rust_program, analysis); // TODO: Move outside
        let ptr_diffs = collect_diffs(rust_program, analysis); // TODO: Move outside
        TransformVisitor {
            tcx: rust_program.tcx,
            sig_decs,
            ptr_diffs,
            ast_to_hir,
            stats: RewriteStats::default(),
        }
    }

    #[allow(unused)]
    pub fn updated(&self) -> bool {
        self.stats.usages + self.stats.writes + self.stats.defs + self.stats.params > 0
    }

    fn expect_parent_node(&self, hir_id: HirId) -> HirNode<'tcx> {
        let (_, parent_node) = self.tcx.hir_parent_iter(hir_id).next().unwrap();
        parent_node
    }

    #[allow(unused)]
    fn is_function_arg(&self, hir_id: HirId) -> bool {
        if let Some(outer_expr) = self.get_outer_expr(hir_id)
            && let HirNode::Expr(parent_expr) = self.expect_parent_node(outer_expr.hir_id)
            && let HirExprKind::Call(_func, args) = parent_expr.kind
            && let Some(_) = args.iter().position(|arg| arg.hir_id == outer_expr.hir_id)
        {
            true
        } else {
            false
        }
    }

    /// Get the outermost expression that contains casting and dereferencing
    #[allow(unused)]
    fn get_outer_expr(&self, hir_id: HirId) -> Option<&HirExpr<'tcx>> {
        let mut out_expr = None;
        for (_, parent_node) in self.tcx.hir_parent_iter(hir_id) {
            match parent_node {
                HirNode::Expr(parent_expr) => match parent_expr.kind {
                    HirExprKind::Cast(..)
                    | HirExprKind::AddrOf(..)
                    | HirExprKind::Unary(UnOp::Deref, _) => out_expr = Some(parent_expr),
                    _ => break,
                },
                _ => break,
            }
        }
        out_expr
    }

    fn sig_input_len(&self, def_id: DefId) -> usize {
        self.tcx
            .fn_sig(def_id)
            .skip_binder()
            .inputs()
            .skip_binder()
            .len()
    }

    fn get_hir_expr(&self, expr: &Expr) -> Option<rustc_hir::Expr<'tcx>> {
        self.ast_to_hir.get_expr(expr.node_id(), self.tcx).cloned()
    }

    fn get_hir_stmt(&self, stmt: &Stmt) -> Option<&rustc_hir::Stmt<'tcx>> {
        self.ast_to_hir.get_stmt(stmt.node_id(), self.tcx)
    }

    fn expr_ty(&self, expr: &Expr) -> Ty {
        let hir_expr = self
            .get_hir_expr(expr)
            .unwrap_or_else(|| panic!("Failed to find HIR expr for Expr {:?}", expr.span));
        let typeck = self.tcx.typeck(hir_expr.hir_id.owner);
        let mir_ty = typeck.expr_ty(&hir_expr);

        mir_ty_to_ty(&mir_ty)
    }

    // Get the inner type if the expr is a pointer
    fn expr_ptr_ty(&self, expr: &Expr) -> Ty {
        let ty = self.expr_ty(expr);
        match &ty.kind {
            TyKind::Ptr(mut_ty) => (*mut_ty.ty).clone(),
            _ => panic!("Expected pointer type for type {ty:#?}"),
        }
    }

    fn rewrite_ty(&mut self, ty: &mut Ty, ty_res: Ty, dec: &Option<PtrKind>) {
        if let Some(PtrKind::OptRef(mutability)) = dec {
            // Create Option<&T> or Option<&mut T>
            let ptr_mut_ty = expect_ptr(ty, ty_res);
            assert!(
                ptr_mut_ty.mutbl
                    == if *mutability {
                        rustc_ast::Mutability::Mut
                    } else {
                        rustc_ast::Mutability::Not
                    }
            );

            let inner_ref = P(Ty {
                id: rustc_ast::DUMMY_NODE_ID,
                kind: TyKind::Ref(None, ptr_mut_ty),
                span: rustc_span::DUMMY_SP,
                tokens: None,
            });

            // Create the Option path
            let option_path = Path {
                span: rustc_span::DUMMY_SP,
                segments: vec![PathSegment {
                    ident: Ident::from_str("Option"),
                    id: rustc_ast::DUMMY_NODE_ID,
                    args: Some(P(GenericArgs::AngleBracketed(AngleBracketedArgs {
                        span: rustc_span::DUMMY_SP,
                        args: [AngleBracketedArg::Arg(GenericArg::Type(inner_ref))].into(),
                    }))),
                }]
                .into(),
                tokens: None,
            };

            ty.kind = TyKind::Path(None, option_path);
        }
    }
}

fn expect_ptr(ty: &mut Ty, ty_res: Ty) -> MutTy {
    match std::mem::replace(&mut ty.kind, TyKind::Infer) {
        TyKind::Ptr(ptr) => ptr,
        _ => match ty_res.kind {
            TyKind::Ptr(ptr) => ptr,
            _ => panic!("Expected pointer type for type {ty:#?}"),
        },
    }
}

fn mir_ty_to_ty(mir_ty: &MirTy) -> Ty {
    let mut ty_str = mir_ty.to_string();
    let re = Regex::new(r"(?P<prefix>(^|<|\s|,\s))(src|bin)::").unwrap();
    ty_str = re
        .replace_all(&ty_str, "${prefix}crate::${3}::")
        .to_string();
    utils::ty!("{}", ty_str)
}
