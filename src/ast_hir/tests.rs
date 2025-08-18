use rustc_ast::{
    visit::{self, Visitor},
    *,
};
use rustc_hir::{self as hir, def_id::LocalModDefId};
use rustc_span::FileName;

use super::AstToHir;
use crate::{ast_util, compile_util};

fn run_test(code: &str) {
    let code = format!("mod a {{ {code} }}");
    compile_util::run_compiler_on_str(&code, |tcx| {
        let parse_sess = ast_util::new_parse_sess();
        let mut parser = rustc_parse::new_parser_from_source_str(
            &parse_sess,
            FileName::Custom("test.rs".to_string()),
            code.clone(),
        )
        .unwrap();
        let mut krate = parser.parse_crate_mod().unwrap();
        let items = krate
            .items
            .iter_mut()
            .find_map(|item| {
                if let ItemKind::Mod(_, ident, ModKind::Loaded(items, _, _, _)) = &mut item.kind
                    && ident.as_str() == "a"
                {
                    Some(items)
                } else {
                    None
                }
            })
            .unwrap();
        let (module, _, _) = tcx.hir_get_module(LocalModDefId::CRATE_DEF_ID);
        let hitems = module
            .item_ids
            .iter()
            .find_map(|item_id| {
                let item = tcx.hir_item(*item_id);
                if let hir::ItemKind::Mod(ident, module) = &item.kind
                    && ident.as_str() == "a"
                {
                    Some(module.item_ids)
                } else {
                    None
                }
            })
            .unwrap();
        let mut ast_to_hir = AstToHir::new(tcx);
        ast_to_hir.map_items_to_items(items, hitems);
        let mut checker = MappingChecker { ast_to_hir };
        for item in items {
            checker.visit_item(item);
        }
    })
    .unwrap();
}

struct MappingChecker<'tcx> {
    ast_to_hir: AstToHir<'tcx>,
}

impl<'a> Visitor<'a> for MappingChecker<'_> {
    fn visit_foreign_item(&mut self, item: &'a ForeignItem) {
        self.ast_to_hir.get_foreign_item(item.id);
        visit::walk_item(self, item);
    }

    fn visit_item(&mut self, item: &'a Item) {
        self.ast_to_hir.get_item(item.id);
        visit::walk_item(self, item);
    }

    fn visit_local(&mut self, local: &'a Local) {
        self.ast_to_hir.get_let_stmt(local.id);
        visit::walk_local(self, local);
    }

    fn visit_block(&mut self, block: &'a Block) {
        self.ast_to_hir.get_block(block.id);
        visit::walk_block(self, block);
    }

    fn visit_param(&mut self, param: &'a Param) {
        self.ast_to_hir.get_param(param.id);
        visit::walk_param(self, param);
    }

    fn visit_arm(&mut self, arm: &'a Arm) {
        self.ast_to_hir.get_arm(arm.id);
        visit::walk_arm(self, arm);
    }

    fn visit_pat(&mut self, pat: &'a Pat) {
        self.ast_to_hir.get_pat(pat.id);
        visit::walk_pat(self, pat);
    }

    fn visit_anon_const(&mut self, c: &'a AnonConst) {
        let node = self.ast_to_hir.get_local_node(c.id);
        assert!(matches!(
            node,
            hir::Node::AnonConst(..) | hir::Node::ConstBlock(..) | hir::Node::ConstArg(..)
        ));
        visit::walk_anon_const(self, c);
    }

    fn visit_expr(&mut self, expr: &'a Expr) {
        let node = self.ast_to_hir.get_local_node(expr.id);
        assert!(matches!(node, hir::Node::Expr(..) | hir::Node::PatExpr(..)));
        visit::walk_expr(self, expr);
    }

    fn visit_ty(&mut self, ty: &'a Ty) {
        self.ast_to_hir.get_ty(ty.id);
        visit::walk_ty(self, ty);
    }

    fn visit_generic_param(&mut self, param: &'a GenericParam) {
        self.ast_to_hir.get_generic_param(param.id);
        visit::walk_generic_param(self, param);
    }

    fn visit_where_predicate(&mut self, pred: &'a WherePredicate) {
        self.ast_to_hir.get_where_predicate(pred.id);
        visit::walk_where_predicate(self, pred);
    }

    fn visit_assoc_item(&mut self, item: &'a AssocItem, ctxt: visit::AssocCtxt) {
        let node = self.ast_to_hir.get_global_node(item.id);
        assert!(matches!(
            node,
            hir::Node::ImplItem(..) | hir::Node::TraitItem(..)
        ));
        visit::walk_assoc_item(self, item, ctxt);
    }

    fn visit_trait_ref(&mut self, tref: &'a TraitRef) {
        self.ast_to_hir.get_trait_ref(tref.ref_id);
        visit::walk_trait_ref(self, tref);
    }

    fn visit_field_def(&mut self, fd: &'a FieldDef) {
        self.ast_to_hir.get_field_def(fd.id);
        visit::walk_field_def(self, fd);
    }

    fn visit_variant(&mut self, variant: &'a Variant) {
        self.ast_to_hir.get_variant(variant.id);
        visit::walk_variant(self, variant);
    }

    fn visit_lifetime(&mut self, lifetime: &'a Lifetime, _: visit::LifetimeCtxt) {
        self.ast_to_hir.get_lifetime(lifetime.id);
        visit::walk_lifetime(self, lifetime);
    }

    fn visit_path_segment(&mut self, seg: &'a PathSegment) {
        self.ast_to_hir.get_path_segment(seg.id);
        visit::walk_path_segment(self, seg);
    }

    fn visit_assoc_item_constraint(&mut self, constraint: &'a AssocItemConstraint) {
        self.ast_to_hir.get_assoc_item_constraint(constraint.id);
        visit::walk_assoc_item_constraint(self, constraint);
    }

    fn visit_expr_field(&mut self, field: &'a ExprField) {
        self.ast_to_hir.get_expr_field(field.id);
        visit::walk_expr_field(self, field);
    }

    fn visit_pat_field(&mut self, field: &'a PatField) {
        self.ast_to_hir.get_pat_field(field.id);
        visit::walk_pat_field(self, field);
    }

    fn visit_stmt(&mut self, stmt: &'a Stmt) {
        self.ast_to_hir.get_stmt(stmt.id);
        visit::walk_stmt(self, stmt);
    }
}

#[test]
fn test_static() {
    run_test("static mut X: i32 = 0;")
}
