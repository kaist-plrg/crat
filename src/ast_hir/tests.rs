use rustc_ast::{
    visit::{self, Visitor},
    *,
};
use rustc_hir::{self as hir, def_id::LocalModDefId};
use rustc_span::FileName;

use super::AstToHir;
use crate::{ast_util, compile_util};

fn run_test(code: &str) {
    let code = format!(
        "
        #![feature(associated_type_defaults)]
        #![feature(extern_types)]
        #![feature(trait_alias)]
        #![feature(c_variadic)]
        #![feature(default_field_values)]
        #![feature(closure_lifetime_binder)]
        #![feature(more_qualified_paths)]
        #![feature(box_patterns)]
        #![feature(guard_patterns)]
        #![feature(unsafe_binders)]
        mod a {{ {code} }}"
    );
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
        self.ast_to_hir.get_foreign_item(item.id).unwrap();
        visit::walk_item(self, item);
    }

    fn visit_item(&mut self, item: &'a Item) {
        self.ast_to_hir.get_item(item.id).unwrap();
        visit::walk_item(self, item);
    }

    fn visit_local(&mut self, local: &'a Local) {
        self.ast_to_hir.get_let_stmt(local.id).unwrap();
        visit::walk_local(self, local);
    }

    fn visit_block(&mut self, block: &'a Block) {
        self.ast_to_hir.get_block(block.id).unwrap();
        visit::walk_block(self, block);
    }

    fn visit_param(&mut self, param: &'a Param) {
        if matches!(param.pat.kind, PatKind::Missing) {
            // vararg
            return;
        }
        self.ast_to_hir.get_param(param.id).unwrap();
        visit::walk_param(self, param);
    }

    fn visit_arm(&mut self, arm: &'a Arm) {
        self.ast_to_hir.get_arm(arm.id).unwrap();
        visit::walk_arm(self, arm);
    }

    fn visit_pat(&mut self, pat: &'a Pat) {
        if matches!(pat.kind, PatKind::Rest) {
            // .. pattern
            return;
        }
        self.ast_to_hir.get_pat(pat.id).unwrap();
        visit::walk_pat(self, pat);
    }

    fn visit_anon_const(&mut self, c: &'a AnonConst) {
        let node = self.ast_to_hir.get_local_node(c.id).unwrap();
        assert!(matches!(
            node,
            hir::Node::AnonConst(..) | hir::Node::ConstBlock(..) | hir::Node::ConstArg(..)
        ));
        visit::walk_anon_const(self, c);
    }

    fn visit_expr(&mut self, expr: &'a Expr) {
        let node = self.ast_to_hir.get_local_node(expr.id).unwrap();
        assert!(matches!(node, hir::Node::Expr(..) | hir::Node::PatExpr(..)));
        visit::walk_expr(self, expr);
    }

    fn visit_ty(&mut self, ty: &'a Ty) {
        self.ast_to_hir.get_ty(ty.id).unwrap();
        visit::walk_ty(self, ty);
    }

    fn visit_generic_param(&mut self, param: &'a GenericParam) {
        self.ast_to_hir.get_generic_param(param.id).unwrap();
        visit::walk_generic_param(self, param);
    }

    fn visit_where_predicate(&mut self, pred: &'a WherePredicate) {
        self.ast_to_hir.get_where_predicate(pred.id).unwrap();
        visit::walk_where_predicate(self, pred);
    }

    fn visit_assoc_item(&mut self, item: &'a AssocItem, ctxt: visit::AssocCtxt) {
        let node = self.ast_to_hir.get_global_node(item.id).unwrap();
        assert!(matches!(
            node,
            hir::Node::ImplItem(..) | hir::Node::TraitItem(..)
        ));
        visit::walk_assoc_item(self, item, ctxt);
    }

    fn visit_trait_ref(&mut self, tref: &'a TraitRef) {
        self.ast_to_hir.get_trait_ref(tref.ref_id).unwrap();
        visit::walk_trait_ref(self, tref);
    }

    fn visit_field_def(&mut self, fd: &'a FieldDef) {
        self.ast_to_hir.get_field_def(fd.id).unwrap();
        visit::walk_field_def(self, fd);
    }

    fn visit_variant(&mut self, variant: &'a Variant) {
        self.ast_to_hir.get_variant(variant.id).unwrap();
        visit::walk_variant(self, variant);
    }

    fn visit_lifetime(&mut self, lifetime: &'a Lifetime, _: visit::LifetimeCtxt) {
        self.ast_to_hir.get_lifetime(lifetime.id).unwrap();
        visit::walk_lifetime(self, lifetime);
    }

    fn visit_path_segment(&mut self, seg: &'a PathSegment) {
        self.ast_to_hir.get_path_segment(seg.id).unwrap();
        visit::walk_path_segment(self, seg);
    }

    fn visit_assoc_item_constraint(&mut self, constraint: &'a AssocItemConstraint) {
        self.ast_to_hir
            .get_assoc_item_constraint(constraint.id)
            .unwrap();
        visit::walk_assoc_item_constraint(self, constraint);
    }

    fn visit_expr_field(&mut self, field: &'a ExprField) {
        self.ast_to_hir.get_expr_field(field.id).unwrap();
        visit::walk_expr_field(self, field);
    }

    fn visit_pat_field(&mut self, field: &'a PatField) {
        self.ast_to_hir.get_pat_field(field.id).unwrap();
        visit::walk_pat_field(self, field);
    }

    fn visit_stmt(&mut self, stmt: &'a Stmt) {
        let node = self.ast_to_hir.get_local_node(stmt.id).unwrap();
        assert!(matches!(node, hir::Node::Stmt(..) | hir::Node::Expr(..)));
        visit::walk_stmt(self, stmt);
    }
}

#[test]
fn test_item_use() {
    run_test("use std::path::Path;")
}

#[test]
fn test_item_static() {
    run_test("static X: i32 = 0;")
}

#[test]
fn test_item_const() {
    run_test("const X: i32 = 0;")
}

#[test]
fn test_item_fn() {
    run_test("fn f<T>(x: T) -> T { x }")
}

#[test]
fn test_item_fn_variadic() {
    run_test("unsafe extern \"C\" fn f(x: i32, ...) -> i32 { x }")
}

#[test]
fn test_item_foreign_mod_static() {
    run_test("extern \"C\" { static mut X: i32; }")
}

#[test]
fn test_item_foreign_fn() {
    run_test("extern \"C\" { fn f() -> i32; }")
}

#[test]
fn test_item_foreign_fn_vararg() {
    run_test("extern \"C\" { fn f(...) -> i32; }")
}

#[test]
fn test_item_foreign_mod_ty_alias() {
    run_test("extern \"C\" { type T; }")
}

#[test]
fn test_item_ty_alias() {
    run_test("type X<T> = T;")
}

#[test]
fn test_item_enum() {
    run_test("enum X<T: Clone> { A = 0, B(T), C { x: T } }")
}

#[test]
fn test_item_struct() {
    run_test("struct X<T: Clone> { x: T, y: i32, z: i32 = 0 }")
}

#[test]
fn test_item_struct_tuple() {
    run_test("struct X<T: Clone>(T, i32);")
}

#[test]
fn test_item_union() {
    run_test("union X<T: Clone> { x: T, y: i32 }")
}

#[test]
fn test_item_trait() {
    run_test(
        "
        trait X<T: Clone>: Clone {
            type Y: Clone;
            type Z = i32;
            const N: i32;
            fn new() -> Self;
            fn f<A>(self, x: A) -> A { x }
        }",
    )
}

#[test]
fn test_item_trait_alias() {
    run_test("trait X<T: Clone> = Clone + Send;")
}

#[test]
fn test_item_impl() {
    run_test(
        "
        struct S<T>;
        trait X<T: Clone>: Clone {
            type Y: Clone;
            type Z = i32;
            const N: i32;
            fn new() -> Self;
            fn f<A>(self, x: A) -> A { x }
        }
        impl<T: Clone> X<T> for S<T> {
            type Y = i32;
            const N: i32 = 0;
            fn new() -> Self { Self }
            fn f<A>(self, x: A) -> A { x }
        }",
    )
}

#[test]
fn test_expr_array() {
    run_test(
        "
        fn f() {
            [1, 2, 3][0];
        }",
    )
}

#[test]
fn test_expr_const_block() {
    run_test(
        "
        fn f() {
            const { 1 + 2 };
        }",
    )
}

#[test]
fn test_expr_call() {
    run_test(
        "
        fn g(x: i32, y: i32) -> i32 {
            x + y
        }
        fn f() {
            g(1, 2);
        }",
    )
}

#[test]
fn test_expr_method_call() {
    run_test(
        "
        struct A;
        impl A {
            fn f(self, x: i32) -> i32 {
                -x + 1
            }
        }
        fn f() {
            A.f(1);
        }",
    )
}

#[test]
fn test_expr_tup() {
    run_test(
        "
        fn f() {
            (1, 2, 3);
        }",
    )
}

#[test]
fn test_expr_cast() {
    run_test(
        "
        fn f() {
            1.0 as i32;
        }",
    )
}

#[test]
fn test_expr_let() {
    run_test(
        "
        fn f() {
            if let 0 = 0 {}
        }",
    )
}

#[test]
fn test_expr_if() {
    run_test(
        "
        fn f() {
            if true {
                1;
            } else if true {
                2;
            } else {
                3;
            }
        }",
    )
}

#[test]
fn test_expr_while() {
    run_test(
        "
        fn f() {
            'l: while true {
                1;
            }
        }",
    )
}

#[test]
fn test_expr_for_loop() {
    run_test(
        "
        fn f() {
            'l: for x in 0..10 {
                x;
            }
        }",
    )
}

#[test]
fn test_expr_loop() {
    run_test(
        "
        fn f() {
            'l: loop {
                1;
                break 'l 1;
                continue 'l;
            }
        }",
    )
}

#[test]
fn test_expr_match() {
    run_test(
        "
        fn f() {
            match true {
                true if true => 1,
                true => 2,
                false => 3,
            }
        }",
    )
}

#[test]
fn test_expr_closure() {
    run_test(
        "
        fn f() {
            for<'a> |_: &'a (), x: i32| x;
            for<'a> |_: &'a (), x: i32| {
                x;
            };
        }",
    )
}

#[test]
fn test_expr_assign() {
    run_test(
        "
        fn f() {
            let x;
            x = 0;
        }",
    )
}

#[test]
fn test_expr_assign_op() {
    run_test(
        "
        fn f() {
            let mut x = 0;
            x += 1;
        }",
    )
}

#[test]
fn test_expr_field() {
    run_test(
        "
        struct S { x: i32 }
        fn f() {
            S { x: 1 }.x;
        }",
    )
}

#[test]
fn test_expr_range() {
    run_test(
        "
        fn f() {
            1..;
            ..1;
        }",
    )
}

#[test]
fn test_expr_addr_of() {
    run_test(
        "
        fn f() {
            let x = 0;
            &x;
        }",
    )
}

#[test]
fn test_expr_return() {
    run_test(
        "
        fn f() -> i32 {
            return 0;
        }",
    )
}

#[test]
fn test_expr_struct() {
    run_test(
        "
        struct S { x: i32, y: i32 }
        fn f() {
            let s = S { x: 1, y: 2 };
            S { x: 3, ..s };
        }",
    )
}

#[test]
fn test_expr_repeat() {
    run_test(
        "
        fn f() {
            [1; 10];
        }",
    )
}

#[test]
fn test_expr_paren() {
    run_test(
        "
        fn f() {
            (1);
        }",
    )
}

#[test]
fn test_stmt_let() {
    run_test(
        "
        fn f() {
            let Some(x) = Some(0) else { return };
        }",
    )
}

#[test]
fn test_stmt_item() {
    run_test(
        "
        fn f() {
            static mut X: i32 = 0;
        }",
    )
}

#[test]
fn test_stmt_expr() {
    run_test(
        "
        fn f() {
            if true {} else {}
            0;
        }",
    )
}

#[test]
fn test_qpath() {
    run_test(
        "
        trait T<V> {
            type A;
        }
        struct S;
        impl<V> T<V> for S {
            type A = X<V>;
        }
        struct X<V> { x: V }
        fn f() {
            <S as T<i32>>::A { x: 0 };
            X::<i32> { x: 0 };
        }",
    )
}

#[test]
fn test_pat_binding() {
    run_test(
        "
        fn f() {
            match Some(0) {
                x @ Some(y) => {}
                _ => {}
            }
        }",
    )
}

#[test]
fn test_pat_binding_expr() {
    run_test(
        "
        struct S;
        fn f() {
            match S {
                S => {}
            }
        }",
    )
}

#[test]
fn test_pat_struct() {
    run_test(
        "
        fn f() {
            struct S { x: i32, y: i32, z: i32 }
            match (S { x: 0, y: 1 }) {
                S { x, y: yy, .. } => {}
            }
        }",
    )
}

#[test]
fn test_pat_tuple_struct() {
    run_test(
        "
        fn f() {
            struct S(i32, i32, i32);
            match S(0, 1, 2) {
                S(0, 0, 0) => {}
                S(0, .., 1) => {}
                S(.., 0, 2) => {}
                S(.., 3) => {}
                S(1, 0, ..) => {}
                S(2, ..) => {}
                S(..) => {}
            }
        }",
    )
}

#[test]
fn test_pat_or() {
    run_test(
        "
        fn f() {
            match 0 {
                -1 | 0 | 1 => {}
                _ => {}
            }
        }",
    )
}

#[test]
fn test_pat_path() {
    run_test(
        "
        trait T<V> {
            type A;
        }
        struct S;
        impl<V> T<V> for S {
            type A = X<V>;
        }
        struct X<V> { x: V }
        fn f() {
            match (X::<i32> { x: 0 }) {
                <S as T<i32>>::A { x: 0 } => {}
            }
        }",
    )
}

#[test]
fn test_pat_tuple() {
    run_test(
        "
        fn f() {
            match (0, 1, 2) {
                (0, 0, 0) => {}
                (0, .., 1) => {}
                (.., 0, 2) => {}
                (.., 3) => {}
                (1, 0, ..) => {}
                (2, ..) => {}
                (..) => {}
            }
        }",
    )
}

#[test]
fn test_pat_box() {
    run_test(
        "
        fn f() {
            match Box::new(0) {
                box x => {}
            }
        }",
    )
}

#[test]
fn test_pat_ref() {
    run_test(
        "
        fn f() {
            match 0 {
                ref x => {}
            }
        }",
    )
}

#[test]
fn test_pat_range() {
    run_test(
        "
        fn f() {
            match 0 {
                0..10 => {}
                10.. => {}
                ..0 => {}
            }
        }",
    )
}

#[test]
fn test_pat_slice() {
    run_test(
        "
        fn f() {
            match [1, 2, 3].as_slice() {
                [0, 0, 0] => {}
                [0, .., 1] => {}
                [0, x @ .., 2] => {}
                [.., 3] => {}
                [x @ .., 4] => {}
                [1, ..] => {}
                [2, x @ ..] => {}
                [..] => {}
                [x @ ..] => {}
            }
        }",
    )
}

#[test]
fn test_pat_guard() {
    run_test(
        "
        struct S(i32);
        fn f() {
            match S(0) {
                S(x if true) => {}
            }
        }",
    )
}

#[test]
fn test_pat_paren() {
    run_test(
        "
        fn f() {
            match 0 {
                (x) => {}
            }
        }",
    )
}

#[test]
fn test_ty_slice() {
    run_test(
        "
        fn f<'a>(x: &'a [i32]) {}
        ",
    )
}

#[test]
fn test_ty_array() {
    run_test(
        "
        fn f(x: [i32; 3]) {}
        ",
    )
}

#[test]
fn test_ty_ptr() {
    run_test(
        "
        fn f(x: *mut i32, y: *const i32) {}
        ",
    )
}

#[test]
fn test_ty_ref() {
    run_test(
        "
        fn f<'a>(x: &'a mut i32, y: &'a i32) {}
        ",
    )
}

#[test]
fn test_ty_bare_fn() {
    run_test(
        "
        fn f<'a>(x: fn (i32, i32) -> i32) {}
        ",
    )
}

#[test]
fn test_ty_unsafe_binder() {
    run_test(
        "
        fn f(x: unsafe<'a> &'a ()) {}
        ",
    )
}

#[test]
fn test_ty_never() {
    run_test(
        "
        fn f() -> ! { loop {} }
        ",
    )
}

#[test]
fn test_ty_tup() {
    run_test(
        "
        fn f(x: (i32, i32, i32)) {}
        ",
    )
}

#[test]
fn test_ty_path() {
    run_test(
        "
        struct S<T>(T);
        fn f(x: S<i32>) {}
        ",
    )
}

#[test]
fn test_ty_trait_object() {
    run_test(
        "
        trait A {}
        fn f(x: Box<dyn A + Send + 'static>) {}
        ",
    )
}

#[test]
fn test_ty_impl_trait() {
    run_test(
        "
        fn f() -> impl Send + 'static { 1 }
        ",
    )
}

#[test]
fn test_ty_impl_trait_param() {
    run_test(
        "
        trait A {}
        fn f(x: impl A + Send + 'static) {}
        ",
    )
}

#[test]
fn test_generics() {
    run_test(
        "
        fn f<'a, 'b: 'a, T: Copy, const A: bool>(x: &'a &'b T) where T: Send, 'b: 'static {}
        ",
    )
}

#[test]
fn test_generic_args_parenthesized() {
    run_test(
        "
        fn f<F: Fn(), G: Fn(u32), H: Fn(u32) -> u32>(f: F, g: G, h: H) {}
        ",
    )
}

#[test]
fn test_assoc_item_constraint_kind_equality() {
    run_test(
        "
        trait T<A> {
            type X;
        }
        fn f<A: T<i32, X = i32>>(a: A) {}
        ",
    )
}
