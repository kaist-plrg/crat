//! Preprocessor
//!
//! # Deduplicate asserts
//!
//! C2Rust may generate code like below:
//!
//! ```rust,ignore
//! if cond {
//! } else {
//!     __assert_fail();
//! }
//! 'c: {
//!     if cond {
//!     } else {
//!         __assert_fail();
//!     }
//! }
//! ```
//!
//! We deduplicate such asserts as follows:
//!
//! ```rust,ignore
//! if cond {
//! } else {
//!     __assert_fail();
//! }
//! 'c: {}
//! ```
//!
//! # Remove dead code
//!
//! Some code contains dead code like below:
//!
//! ```rust,ignore
//! if 0 != 0 {
//!     foo();
//! }
//! ```
//!
//! We remove such dead code as follows:
//!
//! ```rust,ignore
//! {}
//! ```
//!
//! # Simplify `Some`-`unwrap`
//!
//! C2Rust may generate code like below:
//!
//! ```rust,ignore
//! (Some(p.unwrap())).unwrap()();
//! ```
//!
//! We simplify such code as follows:
//!
//! ```rust,ignore
//! p.unwrap()();
//! ```
//!
//! # Remove parameter-assigned variables
//!
//! Some functions assign parameters to variables and never use the parameters again, like below:
//!
//! ```rust,ignore
//! fn foo(x: i32) {
//!     let y = x;
//!     bar(y);
//! }
//! ```
//!
//! We remove such variables as follows:
//!
//! ```rust,ignore
//! fn foo(x: i32) {
//!     {}
//!     bar(x);
//! }
//! ```
//!
//! # Hoist pointer arguments
//!
//! Some function calls use the same pointer arguments multiple times, like below:
//!
//! ```rust,ignore
//! foo(p, bar(p, 0));
//! ```
//!
//! We hoist such pointer arguments as follows:
//!
//! ```rust,ignore
//! {
//!     let __arg_1 = bar(p, 0);
//!     foo(p, __arg_1)
//! };
//! ```
//!
//! Some I/O API function calls use conditional arguments, like below:
//!
//! ```rust,ignore
//! fgetc(if cond { p } else { q });
//! ```
//!
//! We hoist such arguments as follows:
//!
//! ```rust,ignore
//! {
//!     let __arg_1 = if cond { p } else { q };
//!     fgetc(__arg_1);
//! };
//! ```
//!
//! # Replace file function pointer type aliases
//!
//! Some type aliases contain function pointers types with `FILE *`, like below:
//!
//! ```rust,ignore
//! type func = Option::<fn(*mut FILE)>;
//! fn foo(x: func) {}
//! ```
//!
//! We replace such type aliases with corresponding types as follows:
//!
//! ```rust,ignore
//! fn foo(x: Option::<fn(*mut FILE)>) {}
//! ```

use std::fmt::Write as _;

use etrace::some_or;
use rustc_ast::*;
use rustc_ast_pretty::pprust;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir as hir;
use rustc_hir::{
    HirId, QPath,
    def::{DefKind, Res},
    def_id::LocalDefId,
    intravisit,
};
use rustc_middle::{hir::nested_filter, ty::TyCtxt};
use rustc_span::{Span, Symbol};

use crate::{
    ast_util, ast_util::TransformationResult, expr, io_replacer,
    rustc_ast::mut_visit::MutVisitor as _, stmt, ty,
};

pub fn preprocess(tcx: TyCtxt<'_>) {
    transform(tcx).apply();
}

fn transform(tcx: TyCtxt<'_>) -> TransformationResult {
    let mut visitor = HirVisitor {
        tcx,
        ctx: HirCtx::default(),
    };
    tcx.hir_visit_all_item_likes_in_crate(&mut visitor);

    let mut lets_to_remove = FxHashSet::default();
    let mut vars_to_replace = FxHashMap::default();
    let mut params_to_be_mut = FxHashSet::default();
    for (rhs, lhs) in &visitor.ctx.rhs_to_lhs {
        if lhs.len() > 1 || visitor.ctx.used_vars.contains(rhs) {
            continue;
        }
        let (name, param_span) = some_or!(visitor.ctx.params.get(rhs), continue);
        let (lhs, let_span) = lhs[0];
        lets_to_remove.insert(let_span);
        params_to_be_mut.insert(*param_span);
        let bounds = some_or!(visitor.ctx.bound_occurrences.get(&lhs), continue);
        for span in bounds {
            vars_to_replace.insert(*span, *name);
        }
    }

    let mut visitor = AstVisitor {
        hir: &visitor.ctx,
        lets_to_remove: &lets_to_remove,
        vars_to_replace: &vars_to_replace,
        params_to_be_mut: &params_to_be_mut,
        updated: false,
    };
    ast_util::transform_ast(
        |krate| {
            visitor.updated = false;
            visitor.visit_crate(krate);
            visitor.updated
        },
        tcx,
    )
}

struct AstVisitor<'a> {
    hir: &'a HirCtx,

    lets_to_remove: &'a FxHashSet<Span>,
    vars_to_replace: &'a FxHashMap<Span, Symbol>,
    params_to_be_mut: &'a FxHashSet<Span>,

    updated: bool,
}

impl mut_visit::MutVisitor for AstVisitor<'_> {
    fn visit_ty(&mut self, ty: &mut Ty) {
        mut_visit::walk_ty(self, ty);

        if let Some(def_id) = self.hir.bound_file_ty_aliases.get(&ty.span) {
            self.updated = true;
            *ty = ty!("{}", self.hir.ty_aliases[def_id]);
        }
    }

    fn visit_block(&mut self, b: &mut Block) {
        let mut assert = false;
        for stmt in &mut b.stmts {
            if assert {
                assert = false;
                let StmtKind::Semi(e) = &mut stmt.kind else { continue };
                let ExprKind::Block(b, Some(_)) = &mut e.kind else { continue };
                let [stmt] = &b.stmts[..] else { continue };
                if is_assert_stmt(stmt) {
                    self.updated = true;
                    b.stmts.clear();
                }
            } else {
                assert = is_assert_stmt(stmt);
                if self.lets_to_remove.contains(&stmt.span) {
                    self.updated = true;
                    *stmt = stmt!("{{}}");
                }
            }
        }
        mut_visit::walk_block(self, b);
    }

    fn visit_param(&mut self, param: &mut Param) {
        if let PatKind::Ident(mode, ident, _) = &mut param.pat.kind
            && self.params_to_be_mut.contains(&ident.span)
        {
            mode.1 = Mutability::Mut;
        }

        mut_visit::walk_param(self, param);
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        match &mut expr.kind {
            ExprKind::Path(_, _) => {
                if let Some(name) = self.vars_to_replace.get(&expr.span) {
                    self.updated = true;
                    *expr = expr!("{name}");
                }
            }
            ExprKind::If(c, t, f) => {
                if let Some(Value::Bool(b)) = eval_expr(c) {
                    self.updated = true;
                    if b {
                        let e = Expr {
                            id: DUMMY_NODE_ID,
                            kind: ExprKind::Block(t.clone(), None),
                            span: expr.span,
                            attrs: expr.attrs.clone(),
                            tokens: expr.tokens.clone(),
                        };
                        *expr = e;
                    } else if let Some(f) = f {
                        *expr = *f.clone();
                    } else {
                        *expr = expr!("{{}}");
                    }
                }
            }
            _ => {}
        }

        mut_visit::walk_expr(self, expr);

        let expr_span = expr.span;
        match &mut expr.kind {
            ExprKind::Call(_, args) => {
                let mut indices: Vec<ArgIdx> = vec![];
                if let Some(if_args) = self.hir.call_span_to_if_args.get(&expr_span) {
                    indices.extend(if_args);
                }
                if let Some(nested_args) = self.hir.call_span_to_nested_args.get(&expr_span) {
                    indices.extend(nested_args);
                }
                if !indices.is_empty() {
                    self.updated = true;
                    indices.sort();
                    indices.dedup();
                    let mut new_expr = "{".to_string();
                    for i in indices {
                        let i = i.0;
                        ref_to_ptr_in_if(&mut args[i]);
                        let a = pprust::expr_to_string(&args[i]);
                        write!(new_expr, "let __arg_{i} = {a};").unwrap();
                        *args[i] = expr!("__arg_{i}");
                    }
                    new_expr.push_str(&pprust::expr_to_string(expr));
                    new_expr.push('}');
                    *expr = expr!("{new_expr}");
                }
            }
            ExprKind::MethodCall(box call) => {
                if call.seg.ident.name.as_str() != "unwrap" {
                    return;
                }
                let ExprKind::Paren(e) = &call.receiver.kind else { return };
                let ExprKind::Call(callee, e) = &e.kind else { return };
                let ExprKind::Path(_, path) = &callee.kind else { return };
                if path.segments.last().unwrap().ident.name.as_str() != "Some" {
                    return;
                }
                let [arg] = &e[..] else { return };
                let ExprKind::MethodCall(box call) = &arg.kind else { return };
                if call.seg.ident.name.as_str() != "unwrap" {
                    return;
                }
                self.updated = true;
                let arg = pprust::expr_to_string(arg);
                *expr = expr!("{arg}");
            }
            _ => {}
        }
    }
}

fn is_assert_stmt(stmt: &Stmt) -> bool {
    let StmtKind::Expr(e) = &stmt.kind else { return false };
    let ExprKind::If(_, t, f) = &e.kind else { return false };
    if !t.stmts.is_empty() {
        return false;
    }
    let f = some_or!(f.as_ref(), return false);
    let ExprKind::Block(b, None) = &f.kind else { return false };
    let [s] = &b.stmts[..] else { return false };
    let StmtKind::Semi(e) = &s.kind else { return false };
    let ExprKind::Call(e, _) = &e.kind else { return false };
    let ExprKind::Path(_, path) = &e.kind else { return false };
    let [segment] = &path.segments[..] else { return false };
    segment.ident.name.as_str() == "__assert_fail"
}

fn ref_to_ptr_in_if(expr: &mut Expr) {
    let ExprKind::If(_, t, Some(f)) = &mut expr.kind else { return };
    ref_to_ptr(t);
    match &mut f.kind {
        ExprKind::If(_, _, _) => ref_to_ptr_in_if(f),
        ExprKind::Block(f, _) => ref_to_ptr(f),
        _ => panic!(),
    }
}

fn ref_to_ptr(block: &mut Block) {
    if let Some(s) = block.stmts.last_mut()
        && let StmtKind::Expr(e) = &mut s.kind
        && let ExprKind::AddrOf(BorrowKind::Ref, m, _) = &e.kind
    {
        let e_str = pprust::expr_to_string(e);
        let m = if m.is_mut() { "mut" } else { "const" };
        **e = expr!("({e_str}) as *{m} _");
    }
}

#[allow(variant_size_differences)]
enum Value {
    Bool(bool),
    Int(usize),
}

fn eval_expr(expr: &Expr) -> Option<Value> {
    use Value::*;
    match &expr.kind {
        ExprKind::Binary(op, l, r) => match op.node {
            BinOpKind::And => match (eval_expr(l), eval_expr(r)) {
                (Some(Bool(true)), Some(Bool(true))) => Some(Bool(true)),
                (Some(Bool(false)), _) | (_, Some(Bool(false))) => Some(Bool(false)),
                _ => None,
            },
            BinOpKind::Or => match (eval_expr(l), eval_expr(r)) {
                (Some(Bool(true)), _) | (_, Some(Bool(true))) => Some(Bool(true)),
                (Some(Bool(false)), Some(Bool(false))) => Some(Bool(false)),
                _ => None,
            },
            BinOpKind::Eq => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Bool(l == r)),
                _ => None,
            },
            BinOpKind::Ne => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Bool(l != r)),
                _ => None,
            },
            BinOpKind::Gt => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Bool(l > r)),
                _ => None,
            },
            BinOpKind::Ge => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Bool(l >= r)),
                _ => None,
            },
            BinOpKind::Lt => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Bool(l < r)),
                _ => None,
            },
            BinOpKind::Le => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Bool(l <= r)),
                _ => None,
            },
            BinOpKind::Add => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l + r)),
                _ => None,
            },
            BinOpKind::Sub => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l - r)),
                _ => None,
            },
            BinOpKind::Mul => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l * r)),
                _ => None,
            },
            BinOpKind::Div => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l / r)),
                _ => None,
            },
            BinOpKind::Rem => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l % r)),
                _ => None,
            },
            BinOpKind::BitAnd => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l & r)),
                _ => None,
            },
            BinOpKind::BitOr => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l | r)),
                _ => None,
            },
            BinOpKind::BitXor => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l ^ r)),
                _ => None,
            },
            BinOpKind::Shl => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l << r)),
                _ => None,
            },
            BinOpKind::Shr => match (eval_expr(l), eval_expr(r)) {
                (Some(Int(l)), Some(Int(r))) => Some(Int(l >> r)),
                _ => None,
            },
        },
        ExprKind::Cast(e, ty) => {
            let v = eval_expr(e)?;
            let TyKind::Path(_, path) = &ty.kind else {
                return None;
            };
            match path.segments.last()?.ident.name.as_str() {
                "bool" => match v {
                    Bool(b) => Some(Bool(b)),
                    Int(i) => Some(Bool(i != 0)),
                },
                "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "i64" | "isize"
                | "c_char" | "c_int" | "c_long" | "c_longlong" | "c_schar" | "c_short"
                | "c_uchar" | "c_uint" | "c_ulong" | "c_ulonglong" | "c_ushort" => match v {
                    Bool(b) => Some(Int(b as usize)),
                    Int(i) => Some(Int(i)),
                },
                _ => None,
            }
        }
        ExprKind::Lit(l) => {
            if matches!(l.kind, token::LitKind::Integer) {
                l.symbol.as_str().parse().ok().map(Int)
            } else {
                None
            }
        }
        ExprKind::Unary(op, v) => {
            if *op == UnOp::Not {
                if let Some(Bool(b)) = eval_expr(v) {
                    Some(Bool(!b))
                } else {
                    None
                }
            } else {
                None
            }
        }
        ExprKind::Paren(expr) => eval_expr(expr),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
struct BoundOccurrence {
    var_id: HirId,
    expr_id: HirId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ArgIdx(usize);

#[derive(Default)]
struct HirCtx {
    call_id_to_args: FxHashMap<HirId, Vec<(Span, Vec<BoundOccurrence>)>>,
    call_span_to_nested_args: FxHashMap<Span, Vec<ArgIdx>>,
    call_span_to_if_args: FxHashMap<Span, Vec<ArgIdx>>,

    ty_aliases: FxHashMap<LocalDefId, String>,
    bound_file_ty_aliases: FxHashMap<Span, LocalDefId>,

    /// function param hir_id to ident symbol and span
    params: FxHashMap<HirId, (Symbol, Span)>,
    /// let stmt rhs variable hir_id to lhs variable hir_id and let stmt span
    rhs_to_lhs: FxHashMap<HirId, Vec<(HirId, Span)>>,
    /// hir_ids of variables used, excluding let stmt rhs
    used_vars: FxHashSet<HirId>,
    /// variable hir_id to bound occurrence spans
    bound_occurrences: FxHashMap<HirId, Vec<Span>>,
}

struct HirVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    ctx: HirCtx,
}

impl HirVisitor<'_> {
    fn find_call_parent(&self, hir_id: HirId) -> HirId {
        for (hir_id, node) in self.tcx.hir_parent_iter(hir_id) {
            if matches!(
                node,
                hir::Node::Expr(hir::Expr {
                    kind: hir::ExprKind::Call(_, _),
                    ..
                })
            ) {
                return hir_id;
            }
        }
        panic!()
    }
}

impl<'tcx> intravisit::Visitor<'tcx> for HirVisitor<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) {
        intravisit::walk_item(self, item);

        let hir::ItemKind::TyAlias(_, _, ty) = item.kind else { return };
        let ty = self.tcx.sess.source_map().span_to_snippet(ty.span).unwrap();
        self.ctx.ty_aliases.insert(item.owner_id.def_id, ty);
    }

    fn visit_ty(&mut self, ty: &'tcx hir::Ty<'tcx, hir::AmbigArg>) {
        intravisit::walk_ty(self, ty);

        let hir::TyKind::Path(QPath::Resolved(_, path)) = ty.kind else { return };
        let Res::Def(DefKind::TyAlias, def_id) = path.res else { return };
        let def_id = some_or!(def_id.as_local(), return);
        let mir_ty = self.tcx.type_of(def_id).skip_binder();
        if io_replacer::util::file_param_index(mir_ty, self.tcx).is_some() {
            self.ctx.bound_file_ty_aliases.insert(ty.span, def_id);
        }
    }

    fn visit_local(&mut self, let_stmt: &'tcx hir::LetStmt<'tcx>) {
        intravisit::walk_local(self, let_stmt);

        if let_stmt.ty.is_none() {
            // ignore C2Rust-introduced variables, which may be used in `asm!` macro calls
            return;
        }
        let hir::PatKind::Binding(_, lhs_id, _, _) = let_stmt.pat.kind else { return };
        let init = some_or!(let_stmt.init, return);
        let hir::ExprKind::Path(QPath::Resolved(_, path)) = init.kind else { return };
        let Res::Local(rhs_id) = path.res else { return };
        self.ctx
            .rhs_to_lhs
            .entry(rhs_id)
            .or_default()
            .push((lhs_id, let_stmt.span));
    }

    fn visit_param(&mut self, param: &'tcx hir::Param<'tcx>) {
        intravisit::walk_param(self, param);

        let hir::PatKind::Binding(_, id, ident, _) = param.pat.kind else { return };
        self.ctx.params.insert(id, (ident.name, ident.span));
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        match expr.kind {
            hir::ExprKind::Call(callee, args) => {
                if let hir::ExprKind::Path(QPath::Resolved(_, path)) = callee.kind
                    && let Res::Def(DefKind::Fn, def_id) = path.res
                    && io_replacer::api_list::is_def_id_api(def_id, self.tcx)
                {
                    let mut if_args = vec![];
                    for (i, arg) in args.iter().enumerate() {
                        if !matches!(arg.kind, hir::ExprKind::If(_, _, _)) {
                            continue;
                        }
                        let typeck = self.tcx.typeck(expr.hir_id.owner.def_id);
                        let ty = typeck.expr_ty(arg);
                        if io_replacer::util::contains_file_ty(ty, self.tcx) {
                            if_args.push(ArgIdx(i));
                        }
                    }
                    if !if_args.is_empty() {
                        self.ctx.call_span_to_if_args.insert(expr.span, if_args);
                    }
                }

                let args = args.iter().map(|arg| (arg.span, vec![])).collect();
                self.ctx.call_id_to_args.insert(expr.hir_id, args);
            }
            hir::ExprKind::Path(QPath::Resolved(_, path)) => {
                if let Res::Local(hir_id) = path.res {
                    let typeck = self.tcx.typeck(expr.hir_id.owner.def_id);
                    let ty = typeck.expr_ty(expr);
                    if ty.is_raw_ptr() {
                        for v in self.ctx.call_id_to_args.values_mut() {
                            for (span, v) in v {
                                if span.contains(expr.span) {
                                    v.push(BoundOccurrence {
                                        var_id: hir_id,
                                        expr_id: expr.hir_id,
                                    });
                                }
                            }
                        }
                    }

                    self.ctx
                        .bound_occurrences
                        .entry(hir_id)
                        .or_default()
                        .push(expr.span);

                    let (_, parent) = self.tcx.hir_parent_iter(expr.hir_id).next().unwrap();
                    if !matches!(parent, hir::Node::LetStmt(_)) {
                        self.ctx.used_vars.insert(hir_id);
                    }
                }
            }
            _ => {}
        }

        intravisit::walk_expr(self, expr);

        if let hir::ExprKind::Call(_, args) = expr.kind {
            let arg_bound_ids = self.ctx.call_id_to_args.remove(&expr.hir_id).unwrap();
            let nested_args: Vec<_> = arg_bound_ids
                .iter()
                .enumerate()
                .filter_map(|(i, (_, ids))| {
                    for boi in ids {
                        if self.find_call_parent(boi.expr_id) == expr.hir_id {
                            continue;
                        }
                        for ((_, ids), arg) in arg_bound_ids.iter().zip(args) {
                            if !matches!(arg.kind, hir::ExprKind::Path(QPath::Resolved(_, _))) {
                                continue;
                            }
                            if ids.is_empty() {
                                continue;
                            }
                            let [boj] = &ids[..] else { panic!() };
                            if boi.var_id == boj.var_id {
                                return Some(ArgIdx(i));
                            }
                        }
                    }
                    None
                })
                .collect();
            if !nested_args.is_empty() {
                self.ctx
                    .call_span_to_nested_args
                    .insert(expr.span, nested_args);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn run_test(code: &str, includes: &[&str], excludes: &[&str]) {
        let res = crate::compile_util::run_compiler_on_str(code, super::transform).unwrap();
        let [(_, s)] = &res.0[..] else { panic!() };
        crate::compile_util::run_compiler_on_str(&s, crate::type_checker::type_check).unwrap();
        for include in includes {
            assert!(s.contains(include), "Expected to find `{include}` in:\n{s}");
        }
        for exclude in excludes {
            assert!(
                !s.contains(exclude),
                "Expected not to find `{exclude}` in:\n{s}"
            );
        }
    }

    #[test]
    fn test_assert() {
        run_test(
            r#"
use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn g() -> libc::c_int {
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn f() {
    if g() != 0 {} else {
        __assert_fail(
            b"0 == 0\0" as *const u8 as *const libc::c_char,
            b"a.c\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"void foo()\0"))
                .as_ptr(),
        );
    }
    'c_561: {
        if g() != 0 {} else {
            __assert_fail(
                b"0 == 0\0" as *const u8 as *const libc::c_char,
                b"a.c\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"void foo()\0"))
                    .as_ptr(),
            );
        }
    };
}
            "#,
            &["g()", "'c_561: {}"],
            &[],
        )
    }

    #[test]
    fn test_dead() {
        run_test(
            r#"
use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub unsafe extern "C" fn f() {
    if 0 as libc::c_int == 1 as libc::c_int {
        printf(b"\0" as *const u8 as *const libc::c_char);
    }
}
            "#,
            &["{}"],
            &["printf(b"],
        );
    }

    #[test]
    fn test_some_unwrap() {
        run_test(
            r#"
use ::libc;
pub unsafe extern "C" fn f() {
    let mut p: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    ::std::mem::transmute::<_, fn() -> libc::c_int>((Some(p.unwrap())).unwrap())();
}
            "#,
            &["(p.unwrap())()"],
            &["(Some(p.unwrap())).unwrap()"],
        );
    }

    #[test]
    fn test_param() {
        run_test(
            r#"
use ::libc;
pub unsafe extern "C" fn f(x: libc::c_int) {
    let mut y: libc::c_int = x;
    let mut z: libc::c_int = y + y;
}
            "#,
            &["mut x", "x + x"],
            &["let mut y: libc::c_int = x;"],
        );
    }

    #[test]
    fn test_nested_arg() {
        run_test(
            r#"
pub unsafe extern "C" fn g(mut x: *mut libc::c_int, mut y: libc::c_int) -> libc::c_int {
    return *x + y;
}
pub unsafe extern "C" fn f(mut x: libc::c_int, mut p: *mut libc::c_int) {
    g(p, g(p, 0 as libc::c_int));
}
            "#,
            &[" = g(p, 0 as libc::c_int);"],
            &["p, g(p, 0 as libc::c_int)"],
        );
    }

    #[test]
    fn test_cond_arg() {
        run_test(
            r#"
#![feature(extern_types)]
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub unsafe extern "C" fn f(mut c: libc::c_int) {
    let mut p: *mut FILE = 0 as *mut FILE;
    let mut q: *mut FILE = 0 as *mut FILE;
    fgetc(if c != 0 { p } else { q });
}
            "#,
            &[" = if c != 0 { p } else { q };"],
            &["(if c != 0 { p } else { q })"],
        );
    }

    #[test]
    fn test_file_ty_alias() {
        run_test(
            r#"
#![feature(extern_types)]
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int_func = Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>;
pub unsafe extern "C" fn g(mut x: *mut FILE) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn f() -> libc::c_int {
    let mut h: int_func = Some(g as unsafe extern "C" fn(*mut FILE) -> libc::c_int);
    return h.unwrap()(0 as *mut FILE);
}
            "#,
            &["h: Option"],
            &["h: int_func"],
        );
    }
}
