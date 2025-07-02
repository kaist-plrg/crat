use etrace::some_or;
use rustc_ast::{ptr::P, *};
use rustc_middle::ty::TyCtxt;

use crate::{ast_util, rustc_ast::mut_visit::MutVisitor as _};

pub fn dedup_assert(tcx: TyCtxt<'_>) {
    let mut visitor = AstVisitor { updated: false };
    let res = ast_util::transform_ast(
        |krate| {
            visitor.updated = false;
            visitor.visit_crate(krate);
            visitor.updated
        },
        tcx,
    );
    res.apply();
}

struct AstVisitor {
    updated: bool,
}

impl mut_visit::MutVisitor for AstVisitor {
    fn visit_block(&mut self, b: &mut P<Block>) {
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
            }
        }
        mut_visit::walk_block(self, b);
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
