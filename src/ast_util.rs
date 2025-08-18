use std::{cmp::Ordering, path::PathBuf};

use etrace::some_or;
use rustc_ast::{node_id::NodeMap, ptr::P, *};
use rustc_ast_pretty::pprust;
use rustc_hir::{self as hir, HirId};
use rustc_middle::ty::TyCtxt;
use rustc_parse::parser::{AttemptLocalParseRecovery, ForceCollect, Parser};
use rustc_session::parse::ParseSess;
use rustc_span::{FileName, Ident, RealFileName, def_id::LocalDefId};
use thin_vec::ThinVec;

#[derive(Debug)]
pub struct TransformationResult(pub Vec<(PathBuf, String)>);

impl TransformationResult {
    pub fn apply(&self) {
        for (path, content) in &self.0 {
            std::fs::write(path, content).unwrap();
        }
    }
}

pub fn transform_ast<F: std::ops::FnMut(&mut Crate) -> bool>(
    mut f: F,
    tcx: TyCtxt<'_>,
) -> TransformationResult {
    tcx.resolver_for_lowering();

    let source_map = tcx.sess.source_map();
    let parse_sess = new_parse_sess();

    let mut v = vec![];
    for file in source_map.files().iter() {
        let p = match &file.name {
            FileName::Real(RealFileName::LocalPath(p)) => p.clone(),
            FileName::Custom(p) => PathBuf::from(p),
            _ => continue,
        };
        let src = some_or!(file.src.as_ref(), continue);
        let mut parser = rustc_parse::new_parser_from_source_str(
            &parse_sess,
            file.name.clone(),
            src.to_string(),
        )
        .unwrap();
        let mut krate = parser.parse_crate_mod().unwrap();
        if f(&mut krate) {
            let s = pprust::crate_to_string_for_macros(&krate);
            v.push((p, s));
        }
    }
    TransformationResult(v)
}

#[inline]
pub fn new_parse_sess() -> ParseSess {
    ParseSess::with_fatal_emitter(vec![], "".to_string())
}

#[inline]
pub fn new_parser_from_str(parse_sess: &ParseSess, s: String) -> Parser<'_> {
    let file_name = FileName::Custom("main.rs".to_string());
    rustc_parse::new_parser_from_source_str(parse_sess, file_name, s).unwrap()
}

#[inline]
pub fn parse_crate(krate: String) -> Crate {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, krate);
    parser.parse_crate_mod().unwrap()
}

#[macro_export]
macro_rules! krate {
    ($($arg:tt)*) => {{
        parse_crate(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_item(item: String) -> Item {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, item);
    *parser.parse_item(ForceCollect::No).unwrap().unwrap()
}

#[macro_export]
macro_rules! item {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_item(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_items(items: String) -> ThinVec<P<Item>> {
    let krate = parse_crate(items);
    krate.items
}

#[macro_export]
macro_rules! items {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_items(format!($($arg)*))
    }};
}

pub fn parse_ty_param(param: String) -> GenericParam {
    let item = item!("fn f<{}>() {{}}", param);
    let ItemKind::Fn(box mut f) = item.kind else { panic!() };
    f.generics.params.pop().unwrap()
}

#[macro_export]
macro_rules! ty_param {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_ty_param(format!($($arg)*))
    }};
}

pub fn parse_param(param: String) -> Param {
    let item = item!("fn f({}) {{}}", param);
    let ItemKind::Fn(box mut f) = item.kind else { panic!() };
    f.sig.decl.inputs.pop().unwrap()
}

#[macro_export]
macro_rules! param {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_param(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_stmt(stmt: String) -> Stmt {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, stmt);
    parser
        .parse_full_stmt(AttemptLocalParseRecovery::No)
        .unwrap()
        .unwrap()
}

#[macro_export]
macro_rules! stmt {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_stmt(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_expr(expr: String) -> Expr {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, expr);
    *parser.parse_expr().unwrap()
}

#[macro_export]
macro_rules! expr {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_expr(format!($($arg)*))
    }};
}

pub fn parse_path(path: String) -> Path {
    let ExprKind::Path(_, path) = parse_expr(path).kind else { panic!() };
    path
}

#[macro_export]
macro_rules! path {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_path(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_pat(pat: String) -> Pat {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, pat);
    *parser
        .parse_pat_allow_top_guard(
            None,
            rustc_parse::parser::RecoverComma::No,
            rustc_parse::parser::RecoverColon::No,
            rustc_parse::parser::CommaRecoveryMode::LikelyTuple,
        )
        .unwrap()
}

#[macro_export]
macro_rules! pat {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_pat(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_ty(ty: String) -> Ty {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, ty);
    *parser.parse_ty().unwrap()
}

#[macro_export]
macro_rules! ty {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_ty(format!($($arg)*))
    }};
}

pub fn construct_maps<'tcx>(krate: &mut Crate, hmod: &hir::Mod<'tcx>, tcx: TyCtxt<'tcx>) {
    let mut ctx = Ctx {
        tcx,
        next_node_id: NodeId::ZERO,
        global_map: NodeMap::default(),
        local_map: NodeMap::default(),
    };
    ctx.map_crate_to_mod(krate, hmod);
}

struct Ctx<'tcx> {
    tcx: TyCtxt<'tcx>,
    next_node_id: NodeId,
    global_map: NodeMap<LocalDefId>,
    local_map: NodeMap<HirId>,
}

impl<'tcx> Ctx<'tcx> {
    fn add_global(&mut self, node_id: &mut NodeId, def_id: LocalDefId) {
        if *node_id == DUMMY_NODE_ID {
            *node_id = self.next_node_id;
            self.next_node_id += 1;
        }
        self.global_map.insert(*node_id, def_id);
    }

    fn add_local(&mut self, node_id: &mut NodeId, hir_id: HirId) {
        if *node_id == DUMMY_NODE_ID {
            *node_id = self.next_node_id;
            self.next_node_id += 1;
        }
        self.local_map.insert(*node_id, hir_id);
    }

    fn map_crate_to_mod(&mut self, krate: &mut Crate, hmod: &hir::Mod<'tcx>) {
        let mut i = 0;
        for hitem in hmod.item_ids {
            if self
                .tcx
                .is_automatically_derived(hitem.owner_id.to_def_id())
            {
                continue;
            }
            let item = &mut krate.items[i];
            let hitem = self.tcx.hir_item(*hitem);
            self.map_item_to_item(item, hitem);
            i += 1;
        }
    }

    fn map_item_to_item(&mut self, item: &mut Item, hitem: &hir::Item<'tcx>) {
        self.add_global(&mut item.id, hitem.owner_id.def_id);
        match &mut item.kind {
            ItemKind::ExternCrate(symbol, ident) => {
                let hir::ItemKind::ExternCrate(hsymbol, hident) = hitem.kind else { panic!() };
                assert_eq!(*symbol, hsymbol);
                assert_eq!(*ident, hident);
            }
            ItemKind::Use(_) => {}
            ItemKind::Static(box StaticItem {
                ident,
                ty,
                mutability,
                expr,
                ..
            }) => {
                let hir::ItemKind::Static(hmutability, hident, hty, body_id) = hitem.kind else {
                    panic!()
                };
                assert_eq!(*ident, hident);
                assert_eq!(*mutability, hmutability);
                self.map_ty_to_ty(ty, hty);
                let body = self.tcx.hir_body(body_id);
                assert_eq!(body.params.len(), 0);
                self.map_expr_to_expr(expr.as_mut().unwrap(), body.value);
            }
            ItemKind::Const(box ConstItem {
                ident,
                generics,
                ty,
                expr,
                ..
            }) => {
                let hir::ItemKind::Const(hident, hgenerics, hty, body_id) = hitem.kind else {
                    panic!()
                };
                assert_eq!(*ident, hident);
                self.map_ty_to_ty(ty, hty);
                self.map_generics_to_generics(generics, hgenerics);
                let body = self.tcx.hir_body(body_id);
                assert_eq!(body.params.len(), 0);
                self.map_expr_to_expr(expr.as_mut().unwrap(), body.value);
            }
            ItemKind::Fn(box Fn {
                ident,
                generics,
                sig,
                body,
                ..
            }) => {
                let hir::ItemKind::Fn {
                    sig: hsig,
                    ident: hident,
                    generics: hgenerics,
                    body: hbody,
                    ..
                } = hitem.kind
                else {
                    panic!()
                };
                assert_eq!(*ident, hident);
                self.map_generics_to_generics(generics, hgenerics);
                self.map_fn_decl_to_fn_decl(&mut sig.decl, hsig.decl);
                let hbody = self.tcx.hir_body(hbody);
                self.map_fn_decl_block_to_body(&mut sig.decl, body.as_mut().unwrap(), hbody);
            }
            ItemKind::Mod(_, _, _) => panic!(),
            ItemKind::ForeignMod(ForeignMod { items, .. }) => {
                let hir::ItemKind::ForeignMod { items: hitems, .. } = hitem.kind else { panic!() };
                assert_eq!(items.len(), hitems.len());
                for (item, hitem) in items.iter_mut().zip(hitems) {
                    let hitem = self.tcx.hir_foreign_item(hitem.id);
                    self.map_foreign_item_to_foreign_item(item, hitem, hitem.ident);
                }
            }
            ItemKind::GlobalAsm(..) => todo!(),
            ItemKind::TyAlias(box TyAlias {
                ident,
                generics,
                bounds,
                ty,
                ..
            }) => {
                let hir::ItemKind::TyAlias(hident, hgenerics, hty) = hitem.kind else { panic!() };
                assert_eq!(*ident, hident);
                self.map_ty_to_ty(ty.as_mut().unwrap(), hty);
                self.map_generics_to_generics(generics, hgenerics);
                assert_eq!(bounds.len(), 0);
            }
            ItemKind::Enum(ident, generics, ed) => {
                let hir::ItemKind::Enum(hident, hgenerics, hed) = hitem.kind else { panic!() };
                assert_eq!(*ident, hident);
                self.map_generics_to_generics(generics, hgenerics);
                assert_eq!(ed.variants.len(), hed.variants.len());
                for (variant, hvariant) in ed.variants.iter_mut().zip(hed.variants) {
                    self.map_variant_to_variant(variant, hvariant);
                }
            }
            ItemKind::Struct(ident, generics, vd) => {
                let hir::ItemKind::Struct(hident, hgenerics, hvd) = hitem.kind else { panic!() };
                assert_eq!(*ident, hident);
                self.map_generics_to_generics(generics, hgenerics);
                self.map_variant_data_to_variant_data(vd, &hvd);
            }
            ItemKind::Union(ident, generics, vd) => {
                let hir::ItemKind::Union(hident, hgenerics, hvd) = hitem.kind else { panic!() };
                assert_eq!(*ident, hident);
                self.map_generics_to_generics(generics, hgenerics);
                self.map_variant_data_to_variant_data(vd, &hvd);
            }
            ItemKind::Trait(box Trait {
                ident,
                generics,
                bounds,
                items,
                ..
            }) => {
                let hir::ItemKind::Trait(_, _, hident, hgenerics, hbounds, hitems) = hitem.kind
                else {
                    panic!()
                };
                assert_eq!(*ident, hident);
                self.map_generics_to_generics(generics, hgenerics);
                assert_eq!(bounds.len(), hbounds.len());
                for (bound, hbound) in bounds.iter_mut().zip(hbounds) {
                    self.map_generic_bound_to_generic_bound(bound, hbound);
                }
                assert_eq!(items.len(), hitems.len());
                for (item, hitem) in items.iter_mut().zip(hitems) {
                    let hitem = self.tcx.hir_trait_item(hitem.id);
                    self.map_assoc_item_to_trait_item(item, hitem);
                }
            }
            ItemKind::TraitAlias(ident, generics, bounds) => {
                let hir::ItemKind::TraitAlias(hident, hgenerics, hbounds) = hitem.kind else {
                    panic!()
                };
                assert_eq!(*ident, hident);
                self.map_generics_to_generics(generics, hgenerics);
                assert_eq!(bounds.len(), hbounds.len());
                for (bound, hbound) in bounds.iter_mut().zip(hbounds) {
                    self.map_generic_bound_to_generic_bound(bound, hbound);
                }
            }
            ItemKind::Impl(box Impl {
                generics,
                of_trait,
                self_ty,
                items,
                ..
            }) => {
                let hir::ItemKind::Impl(hir::Impl {
                    generics: hgenerics,
                    of_trait: hof_trait,
                    self_ty: hself_ty,
                    items: hitems,
                    ..
                }) = hitem.kind
                else {
                    panic!()
                };
                self.map_generics_to_generics(generics, hgenerics);
                assert_eq!(of_trait.is_some(), hof_trait.is_some());
                if let (Some(of_trait), Some(hof_trait)) = (of_trait, hof_trait) {
                    self.map_trait_ref_to_trait_ref(of_trait, hof_trait);
                }
                self.map_ty_to_ty(self_ty, hself_ty);
                assert_eq!(items.len(), hitems.len());
                for (item, hitem) in items.iter_mut().zip(*hitems) {
                    let hitem = self.tcx.hir_impl_item(hitem.id);
                    self.map_assoc_item_to_impl_item(item, hitem);
                }
            }
            ItemKind::MacCall(..) => todo!(),
            ItemKind::MacroDef(..) => todo!(),
            ItemKind::Delegation(..) => todo!(),
            ItemKind::DelegationMac(..) => todo!(),
        }
    }

    fn map_foreign_item_to_foreign_item(
        &mut self,
        foreign_item: &mut ForeignItem,
        hforeign_item: &hir::ForeignItem<'tcx>,
        hident: Ident,
    ) {
        self.add_global(&mut foreign_item.id, hforeign_item.owner_id.def_id);
        match &mut foreign_item.kind {
            ForeignItemKind::Static(box StaticItem {
                ident,
                ty,
                mutability,
                ..
            }) => {
                assert_eq!(*ident, hident);
                let hir::ForeignItemKind::Static(hty, hmutability, _) = hforeign_item.kind else {
                    panic!()
                };
                assert_eq!(*mutability, hmutability);
                self.map_ty_to_ty(ty, hty);
            }
            ForeignItemKind::Fn(box Fn {
                ident,
                generics,
                sig,
                ..
            }) => {
                assert_eq!(*ident, hident);
                let hir::ForeignItemKind::Fn(hsig, hparams, hgenerics) = hforeign_item.kind else {
                    panic!()
                };
                self.map_generics_to_generics(generics, hgenerics);
                assert_eq!(sig.decl.inputs.len(), hparams.len());
                self.map_fn_decl_to_fn_decl(&mut sig.decl, hsig.decl);
            }
            ForeignItemKind::TyAlias(box TyAlias { ident, .. }) => {
                assert_eq!(*ident, hident);
                let hir::ForeignItemKind::Type = hforeign_item.kind else { panic!() };
            }
            ForeignItemKind::MacCall(..) => todo!(),
        }
    }

    fn map_assoc_item_to_trait_item(
        &mut self,
        assoc_item: &mut AssocItem,
        htrait_item: &hir::TraitItem<'tcx>,
    ) {
        self.add_global(&mut assoc_item.id, htrait_item.owner_id.def_id);
        match &mut assoc_item.kind {
            AssocItemKind::Const(box ConstItem {
                ident,
                generics,
                ty,
                expr,
                ..
            }) => {
                let hir::TraitItemKind::Const(hty, hbody) = htrait_item.kind else { panic!() };
                assert_eq!(*ident, htrait_item.ident);
                self.map_generics_to_generics(generics, htrait_item.generics);
                self.map_ty_to_ty(ty, hty);
                assert_eq!(expr.is_some(), hbody.is_some());
                if let (Some(expr), Some(hbody)) = (expr, hbody) {
                    let hbody = self.tcx.hir_body(hbody);
                    assert_eq!(hbody.params.len(), 0);
                    self.map_expr_to_expr(expr, hbody.value);
                }
            }
            AssocItemKind::Fn(box Fn {
                ident,
                generics,
                sig,
                body,
                ..
            }) => {
                let hir::TraitItemKind::Fn(hsig, hfn) = htrait_item.kind else { panic!() };
                assert_eq!(*ident, htrait_item.ident);
                self.map_generics_to_generics(generics, htrait_item.generics);
                self.map_fn_decl_to_fn_decl(&mut sig.decl, hsig.decl);
                if let Some(body) = body {
                    let hir::TraitFn::Provided(hbody) = hfn else { panic!() };
                    let hbody = self.tcx.hir_body(hbody);
                    self.map_fn_decl_block_to_body(&mut sig.decl, body, hbody);
                } else {
                    assert!(matches!(hfn, hir::TraitFn::Required(_)));
                }
            }
            AssocItemKind::Type(box TyAlias {
                ident,
                generics,
                bounds,
                ty,
                ..
            }) => {
                let hir::TraitItemKind::Type(hbounds, hty) = htrait_item.kind else { panic!() };
                assert_eq!(*ident, htrait_item.ident);
                self.map_generics_to_generics(generics, htrait_item.generics);
                assert_eq!(bounds.len(), hbounds.len());
                for (bound, hbound) in bounds.iter_mut().zip(hbounds) {
                    self.map_generic_bound_to_generic_bound(bound, hbound);
                }
                assert_eq!(ty.is_some(), hty.is_some());
                if let (Some(ty), Some(hty)) = (ty, hty) {
                    self.map_ty_to_ty(ty, hty);
                }
            }
            AssocItemKind::MacCall(..) => todo!(),
            AssocItemKind::Delegation(..) => todo!(),
            AssocItemKind::DelegationMac(..) => todo!(),
        }
    }

    fn map_assoc_item_to_impl_item(
        &mut self,
        assoc_item: &mut AssocItem,
        himpl_item: &hir::ImplItem<'tcx>,
    ) {
        self.add_global(&mut assoc_item.id, himpl_item.owner_id.def_id);
        match &mut assoc_item.kind {
            AssocItemKind::Const(box ConstItem {
                ident,
                generics,
                ty,
                expr,
                ..
            }) => {
                let hir::ImplItemKind::Const(hty, hbody) = himpl_item.kind else { panic!() };
                assert_eq!(*ident, himpl_item.ident);
                self.map_generics_to_generics(generics, himpl_item.generics);
                self.map_ty_to_ty(ty, hty);
                let hbody = self.tcx.hir_body(hbody);
                assert_eq!(hbody.params.len(), 0);
                self.map_expr_to_expr(expr.as_mut().unwrap(), hbody.value);
            }
            AssocItemKind::Fn(box Fn {
                ident,
                generics,
                sig,
                body,
                ..
            }) => {
                let hir::ImplItemKind::Fn(hsig, hbody) = himpl_item.kind else { panic!() };
                assert_eq!(*ident, himpl_item.ident);
                self.map_generics_to_generics(generics, himpl_item.generics);
                self.map_fn_decl_to_fn_decl(&mut sig.decl, hsig.decl);
                let hbody = self.tcx.hir_body(hbody);
                self.map_fn_decl_block_to_body(&mut sig.decl, body.as_mut().unwrap(), hbody);
            }
            AssocItemKind::Type(box TyAlias {
                ident,
                generics,
                bounds,
                ty,
                ..
            }) => {
                let hir::ImplItemKind::Type(hty) = himpl_item.kind else { panic!() };
                assert_eq!(*ident, himpl_item.ident);
                self.map_generics_to_generics(generics, himpl_item.generics);
                assert_eq!(bounds.len(), 0);
                self.map_ty_to_ty(ty.as_mut().unwrap(), hty);
            }
            AssocItemKind::MacCall(..) => todo!(),
            AssocItemKind::Delegation(..) => todo!(),
            AssocItemKind::DelegationMac(..) => todo!(),
        }
    }

    fn map_fn_decl_to_fn_decl(&mut self, decl: &mut FnDecl, hdecl: &hir::FnDecl<'tcx>) {
        assert_eq!(decl.inputs.len(), hdecl.inputs.len());
        for (input, hinput) in decl.inputs.iter_mut().zip(hdecl.inputs) {
            self.map_ty_to_ty(&mut input.ty, hinput);
        }
        match &mut decl.output {
            FnRetTy::Ty(ty) => {
                let hir::FnRetTy::Return(hty) = hdecl.output else { panic!() };
                self.map_ty_to_ty(ty, hty);
            }
            FnRetTy::Default(span) => {
                let hir::FnRetTy::DefaultReturn(hspan) = hdecl.output else { panic!() };
                assert_eq!(*span, hspan);
            }
        }
    }

    fn map_fn_decl_block_to_body(
        &mut self,
        decl: &mut FnDecl,
        body: &mut Block,
        hbody: &hir::Body<'tcx>,
    ) {
        assert_eq!(decl.inputs.len(), hbody.params.len());
        for (input, param) in decl.inputs.iter_mut().zip(hbody.params) {
            self.add_local(&mut input.id, param.hir_id);
            self.map_pat_to_pat(&mut input.pat, param.pat);
        }
        let hir::ExprKind::Block(hbody, None) = hbody.value.kind else { panic!() };
        self.map_block_to_block(body, hbody);
    }

    fn map_variant_to_variant(&mut self, variant: &mut Variant, hvariant: &hir::Variant<'tcx>) {
        self.add_local(&mut variant.id, hvariant.hir_id);
        self.add_global(&mut variant.id, hvariant.def_id);
        assert_eq!(variant.span, hvariant.span);
        assert_eq!(variant.ident, hvariant.ident);
        self.map_variant_data_to_variant_data(&mut variant.data, &hvariant.data);
        assert_eq!(variant.disr_expr.is_some(), hvariant.disr_expr.is_some());
        if let (Some(disr_expr), Some(hdisr_expr)) = (&mut variant.disr_expr, hvariant.disr_expr) {
            self.map_anon_const_to_anon_const(disr_expr, hdisr_expr);
        }
    }

    fn map_variant_data_to_variant_data(
        &mut self,
        vd: &mut VariantData,
        hvd: &hir::VariantData<'tcx>,
    ) {
        match vd {
            VariantData::Struct { fields, .. } => {
                let hir::VariantData::Struct {
                    fields: hfields, ..
                } = *hvd
                else {
                    panic!()
                };
                assert_eq!(fields.len(), hfields.len());
                for (field, hfield) in fields.iter_mut().zip(hfields) {
                    self.map_field_def_to_field_def(field, hfield);
                }
            }
            VariantData::Tuple(fields, node_id) => {
                let hir::VariantData::Tuple(hfields, hir_id, def_id) = *hvd else { panic!() };
                self.add_local(&mut *node_id, hir_id);
                self.add_global(&mut *node_id, def_id);
                assert_eq!(fields.len(), hfields.len());
                for (field, hfield) in fields.iter_mut().zip(hfields) {
                    self.map_field_def_to_field_def(field, hfield);
                }
            }
            VariantData::Unit(node_id) => {
                let hir::VariantData::Unit(hir_id, def_id) = *hvd else { panic!() };
                self.add_local(&mut *node_id, hir_id);
                self.add_global(&mut *node_id, def_id);
            }
        }
    }

    fn map_field_def_to_field_def(
        &mut self,
        field_def: &mut FieldDef,
        hfield_def: &hir::FieldDef<'tcx>,
    ) {
        self.add_local(&mut field_def.id, hfield_def.hir_id);
        self.add_global(&mut field_def.id, hfield_def.def_id);
        assert_eq!(field_def.span, hfield_def.span);
        if let Some(ident) = &field_def.ident {
            assert_eq!(hfield_def.ident.name, ident.name);
        }
        self.map_ty_to_ty(&mut field_def.ty, hfield_def.ty);
        assert_eq!(field_def.default.is_some(), hfield_def.default.is_some());
        if let (Some(default), Some(hdefault)) = (&mut field_def.default, &hfield_def.default) {
            self.map_anon_const_to_anon_const(default, hdefault);
        }
    }

    fn map_expr_to_expr(&mut self, expr: &mut Expr, hexpr: &hir::Expr<'tcx>) {
        if let hir::ExprKind::DropTemps(hexpr) = hexpr.kind {
            self.map_expr_to_expr(expr, hexpr);
            return;
        }
        self.add_local(&mut expr.id, hexpr.hir_id);
        match &mut expr.kind {
            ExprKind::Array(exprs) => {
                let hir::ExprKind::Array(hexprs) = hexpr.kind else { panic!() };
                assert_eq!(exprs.len(), hexprs.len());
                for (ae, he) in exprs.iter_mut().zip(hexprs) {
                    self.map_expr_to_expr(ae, he);
                }
            }
            ExprKind::ConstBlock(c) => {
                let hir::ExprKind::ConstBlock(hc) = hexpr.kind else { panic!() };
                self.map_anon_const_to_const_block(c, &hc);
            }
            ExprKind::Call(callee, args) => {
                let hir::ExprKind::Call(hcallee, hargs) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(callee, hcallee);
                assert_eq!(args.len(), hargs.len());
                for (ae, he) in args.iter_mut().zip(hargs) {
                    self.map_expr_to_expr(ae, he);
                }
            }
            ExprKind::MethodCall(box MethodCall {
                seg,
                receiver,
                args,
                span,
            }) => {
                let hir::ExprKind::MethodCall(hseg, hreceiver, hargs, hspan) = hexpr.kind else {
                    panic!()
                };
                assert_eq!(seg.ident.name, hseg.ident.name);
                assert_eq!(*span, hspan);
                self.map_path_segment_to_path_segment(seg, hseg);
                self.map_expr_to_expr(receiver, hreceiver);
                assert_eq!(args.len(), hargs.len());
                for (ae, he) in args.iter_mut().zip(hargs) {
                    self.map_expr_to_expr(ae, he);
                }
            }
            ExprKind::Tup(exprs) => {
                let hir::ExprKind::Tup(hexprs) = hexpr.kind else { panic!() };
                assert_eq!(exprs.len(), hexprs.len());
                for (ae, he) in exprs.iter_mut().zip(hexprs) {
                    self.map_expr_to_expr(ae, he);
                }
            }
            ExprKind::Binary(op, lhs, rhs) => {
                let hir::ExprKind::Binary(hop, hlhs, hrhs) = hexpr.kind else { panic!() };
                assert_eq!(op.node, hop.node);
                self.map_expr_to_expr(lhs, hlhs);
                self.map_expr_to_expr(rhs, hrhs);
            }
            ExprKind::Unary(op, expr) => {
                let hir::ExprKind::Unary(hop, hexpr) = hexpr.kind else { panic!() };
                assert_eq!(*op, hop);
                self.map_expr_to_expr(expr, hexpr);
            }
            ExprKind::Lit(_) => {
                let hir::ExprKind::Lit(_) = hexpr.kind else { panic!() };
            }
            ExprKind::Cast(expr, ty) => {
                let hir::ExprKind::Cast(hexpr, hty) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
                self.map_ty_to_ty(ty, hty);
            }
            ExprKind::Type(expr, ty) => {
                let hir::ExprKind::Type(hexpr, hty) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
                self.map_ty_to_ty(ty, hty);
            }
            ExprKind::Let(pat, expr, span, _) => {
                let hir::ExprKind::Let(hir::LetExpr {
                    span: hspan,
                    pat: hpat,
                    ty: None,
                    init: hexpr,
                    ..
                }) = hexpr.kind
                else {
                    panic!()
                };
                assert_eq!(span, hspan);
                self.map_pat_to_pat(pat, hpat);
                self.map_expr_to_expr(expr, hexpr);
            }
            ExprKind::If(c, t, f) => {
                let hir::ExprKind::If(hc, ht, hf) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(c, hc);
                let hir::ExprKind::Block(ht, None) = ht.kind else { panic!() };
                self.map_block_to_block(t, ht);
                assert_eq!(f.is_some(), hf.is_some());
                if let (Some(f), Some(hf)) = (f, hf) {
                    self.map_expr_to_expr(f, hf);
                }
            }
            ExprKind::While(expr, block, label) => {
                let hir::ExprKind::Loop(hblock, hlabel, source, _) = hexpr.kind else { panic!() };
                assert_eq!(*label, hlabel);
                assert_eq!(source, hir::LoopSource::While);
                assert_eq!(hblock.stmts.len(), 0);
                let hir::ExprKind::If(c, t, Some(_)) = hblock.expr.unwrap().kind else { panic!() };
                self.map_expr_to_expr(expr, c);
                let hir::ExprKind::Block(t, None) = t.kind else { panic!() };
                self.map_block_to_block(block, t);
            }
            ExprKind::ForLoop {
                pat,
                iter,
                body,
                label,
                ..
            } => {
                let hir::ExprKind::Match(hexpr, [harm], _) = hexpr.kind else { panic!() };
                let hir::ExprKind::Call(_, [hiter]) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(iter, hiter);
                let hir::ExprKind::Loop(hblock, hlabel, _, _) = harm.body.kind else { panic!() };
                assert_eq!(*label, hlabel);
                let hir::StmtKind::Expr(hexpr) = hblock.stmts[0].kind else { panic!() };
                let hir::ExprKind::Match(_, [_, harm], _) = hexpr.kind else { panic!() };
                let hir::PatKind::Struct(_, [hpat], _) = harm.pat.kind else { panic!() };
                self.map_pat_to_pat(pat, hpat.pat);
                let hir::ExprKind::Block(hbody, None) = harm.body.kind else { panic!() };
                self.map_block_to_block(body, hbody);
            }
            ExprKind::Loop(block, label, span) => {
                let hir::ExprKind::Loop(hblock, hlabel, source, hspan) = hexpr.kind else {
                    panic!()
                };
                assert_eq!(*label, hlabel);
                assert_eq!(source, hir::LoopSource::Loop);
                assert_eq!(*span, hspan);
                self.map_block_to_block(block, hblock);
            }
            ExprKind::Match(expr, arms, _) => {
                let hir::ExprKind::Match(hexpr, harms, _) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
                assert_eq!(arms.len(), harms.len());
                for (arm, harm) in arms.iter_mut().zip(harms) {
                    self.map_arm_to_arm(arm, harm);
                }
            }
            ExprKind::Closure(box Closure {
                binder,
                fn_decl,
                body,
                fn_decl_span,
                fn_arg_span,
                ..
            }) => {
                let hir::ExprKind::Closure(hir::Closure {
                    def_id,
                    bound_generic_params: hbound_generic_params,
                    fn_decl: hfn_decl,
                    body: hbody,
                    fn_decl_span: hfn_decl_span,
                    fn_arg_span: hfn_arg_span,
                    ..
                }) = hexpr.kind
                else {
                    panic!()
                };
                self.add_global(&mut expr.id, *def_id);
                if let ClosureBinder::For { generic_params, .. } = binder {
                    assert!(generic_params.len() <= hbound_generic_params.len());
                    for (param, hparam) in generic_params.iter_mut().zip(*hbound_generic_params) {
                        self.map_generic_param_to_generic_param(param, hparam);
                    }
                }
                assert_eq!(fn_decl_span, hfn_decl_span);
                assert_eq!(*fn_arg_span, hfn_arg_span.unwrap());
                self.map_fn_decl_to_fn_decl(fn_decl, hfn_decl);
                let hbody = self.tcx.hir_body(*hbody);
                assert_eq!(fn_decl.inputs.len(), hbody.params.len());
                for (input, param) in fn_decl.inputs.iter_mut().zip(hbody.params) {
                    self.add_local(&mut input.id, param.hir_id);
                    self.map_pat_to_pat(&mut input.pat, param.pat);
                }
                self.map_expr_to_expr(body, hbody.value);
            }
            ExprKind::Block(block, label) => {
                let hir::ExprKind::Block(hblock, hlabel) = hexpr.kind else { panic!() };
                assert_eq!(*label, hlabel);
                self.map_block_to_block(block, hblock);
            }
            ExprKind::Gen(..) => todo!(),
            ExprKind::Await(..) => todo!(),
            ExprKind::Use(expr, span) => {
                let hir::ExprKind::Use(hexpr, hspan) = hexpr.kind else { panic!() };
                assert_eq!(*span, hspan);
                self.map_expr_to_expr(expr, hexpr);
            }
            ExprKind::TryBlock(..) => todo!(),
            ExprKind::Assign(lhs, rhs, span) => {
                let hir::ExprKind::Assign(hlhs, hrhs, hspan) = hexpr.kind else { panic!() };
                assert_eq!(*span, hspan);
                self.map_expr_to_expr(lhs, hlhs);
                self.map_expr_to_expr(rhs, hrhs);
            }
            ExprKind::AssignOp(op, lhs, rhs) => {
                let hir::ExprKind::AssignOp(hop, hlhs, hrhs) = hexpr.kind else { panic!() };
                assert_eq!(*op, hop);
                self.map_expr_to_expr(lhs, hlhs);
                self.map_expr_to_expr(rhs, hrhs);
            }
            ExprKind::Field(expr, field) => {
                let hir::ExprKind::Field(hexpr, hfield) = hexpr.kind else { panic!() };
                assert_eq!(*field, hfield);
                self.map_expr_to_expr(expr, hexpr);
            }
            ExprKind::Index(expr, index, span) => {
                let hir::ExprKind::Index(hexpr, hindex, hspan) = hexpr.kind else { panic!() };
                assert_eq!(*span, hspan);
                self.map_expr_to_expr(expr, hexpr);
                self.map_expr_to_expr(index, hindex);
            }
            ExprKind::Range(from, to, _) => {
                let hir::ExprKind::Struct(_, hfields, _) = hexpr.kind else { panic!() };
                if let Some(from) = from {
                    let hfrom = hfields[0].expr;
                    self.map_expr_to_expr(from, hfrom);
                }
                if let Some(to) = to {
                    let i = if from.is_some() { 1 } else { 0 };
                    let hto = hfields[i].expr;
                    self.map_expr_to_expr(to, hto);
                }
            }
            ExprKind::Underscore => panic!(),
            ExprKind::Path(qself, path) => {
                let hir::ExprKind::Path(hqpath) = hexpr.kind else { panic!() };
                self.map_path_to_qpath(qself, path, &hqpath);
            }
            ExprKind::AddrOf(kind, mutability, expr) => {
                let hir::ExprKind::AddrOf(hkind, hmutability, hexpr) = hexpr.kind else { panic!() };
                assert_eq!(*kind, hkind);
                assert_eq!(*mutability, hmutability);
                self.map_expr_to_expr(expr, hexpr);
            }
            ExprKind::Break(label, expr) => {
                let hir::ExprKind::Break(hdest, hexpr) = hexpr.kind else { panic!() };
                assert_eq!(*label, hdest.label);
                assert_eq!(expr.is_some(), hexpr.is_some());
                if let (Some(expr), Some(hexpr)) = (expr, hexpr) {
                    self.map_expr_to_expr(expr, hexpr);
                }
            }
            ExprKind::Continue(label) => {
                let hir::ExprKind::Continue(hdest) = hexpr.kind else { panic!() };
                assert_eq!(*label, hdest.label);
            }
            ExprKind::Ret(expr) => {
                let hir::ExprKind::Ret(hexpr) = hexpr.kind else { panic!() };
                assert_eq!(expr.is_some(), hexpr.is_some());
                if let (Some(expr), Some(hexpr)) = (expr, hexpr) {
                    self.map_expr_to_expr(expr, hexpr);
                }
            }
            ExprKind::InlineAsm(..) => panic!(),
            ExprKind::OffsetOf(ty, idents) => {
                let hir::ExprKind::OffsetOf(hty, hidents) = hexpr.kind else { panic!() };
                self.map_ty_to_ty(ty, hty);
                assert_eq!(idents.len(), hidents.len());
            }
            ExprKind::MacCall(..) => todo!(),
            ExprKind::Struct(box StructExpr {
                qself,
                path,
                fields,
                rest,
            }) => {
                let hir::ExprKind::Struct(hqpath, hfields, hrest) = hexpr.kind else { panic!() };
                self.map_path_to_qpath(qself, path, hqpath);
                assert_eq!(fields.len(), hfields.len());
                for (field, hfield) in fields.iter_mut().zip(hfields) {
                    self.map_expr_field_to_expr_field(field, hfield);
                }
                match rest {
                    StructRest::Base(expr) => {
                        let hir::StructTailExpr::Base(hexpr) = hrest else { panic!() };
                        self.map_expr_to_expr(expr, hexpr);
                    }
                    StructRest::Rest(span) => {
                        let hir::StructTailExpr::DefaultFields(hspan) = hrest else { panic!() };
                        assert_eq!(*span, hspan);
                    }
                    StructRest::None => {
                        assert!(matches!(hrest, hir::StructTailExpr::None));
                    }
                }
            }
            ExprKind::Repeat(expr, len) => {
                let hir::ExprKind::Repeat(hexpr, hlen) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
                self.map_anon_const_to_const_arg(len, hlen);
            }
            ExprKind::Paren(expr) => self.map_expr_to_expr(expr, hexpr),
            ExprKind::Try(expr) => {
                let hir::ExprKind::Match(hexpr, _, _) = hexpr.kind else { panic!() };
                let hir::ExprKind::Call(_, [hexpr]) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
            }
            ExprKind::Yield(..) => todo!(),
            ExprKind::Yeet(..) => todo!(),
            ExprKind::Become(expr) => {
                let hir::ExprKind::Become(hexpr) = hexpr.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
            }
            ExprKind::IncludedBytes(..) => todo!(),
            ExprKind::FormatArgs(..) => todo!(),
            ExprKind::UnsafeBinderCast(cast, expr, ty) => {
                let hir::ExprKind::UnsafeBinderCast(hcast, hexpr, hty) = hexpr.kind else {
                    panic!()
                };
                assert_eq!(*cast, hcast);
                self.map_expr_to_expr(expr, hexpr);
                assert_eq!(ty.is_some(), hty.is_some());
                if let (Some(ty), Some(hty)) = (ty, hty) {
                    self.map_ty_to_ty(ty, hty);
                }
            }
            ExprKind::Err(_) => panic!(),
            ExprKind::Dummy => panic!(),
        }
    }

    fn map_block_to_block(&mut self, block: &mut Block, hblock: &hir::Block<'tcx>) {
        self.add_local(&mut block.id, hblock.hir_id);
        assert_eq!(block.span, hblock.span);
        let mut i = 0;
        for stmt in &mut block.stmts {
            match stmt.kind {
                StmtKind::Empty => {}
                StmtKind::MacCall(_) => todo!(),
                _ => match i.cmp(&hblock.stmts.len()) {
                    Ordering::Less => {
                        let hstmt = &hblock.stmts[i];
                        self.map_stmt_to_stmt(stmt, hstmt);
                        i += 1;
                    }
                    Ordering::Equal => {
                        let StmtKind::Expr(expr) = &mut stmt.kind else { panic!() };
                        let hexpr = hblock.expr.unwrap();
                        self.map_expr_to_expr(expr, hexpr);
                        i += 1;
                    }
                    Ordering::Greater => panic!(),
                },
            }
        }
        assert!(i >= hblock.stmts.len());
    }

    fn map_stmt_to_stmt(&mut self, stmt: &mut Stmt, hstmt: &hir::Stmt<'tcx>) {
        self.add_local(&mut stmt.id, hstmt.hir_id);
        match &mut stmt.kind {
            StmtKind::Let(local) => {
                let hir::StmtKind::Let(hlocal) = hstmt.kind else { panic!() };
                self.map_local_to_let_stmt(local, hlocal);
            }
            StmtKind::Item(item) => {
                let hir::StmtKind::Item(item_id) = hstmt.kind else { panic!() };
                let hitem = self.tcx.hir_item(item_id);
                self.map_item_to_item(item, hitem);
            }
            StmtKind::Expr(expr) => {
                let hir::StmtKind::Expr(hexpr) = hstmt.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
            }
            StmtKind::Semi(expr) => {
                let hir::StmtKind::Semi(hexpr) = hstmt.kind else { panic!() };
                self.map_expr_to_expr(expr, hexpr);
            }
            StmtKind::Empty => panic!(),
            StmtKind::MacCall(_) => todo!(),
        }
    }

    fn map_local_to_let_stmt(&mut self, local: &mut Local, hlocal: &hir::LetStmt<'tcx>) {
        self.add_local(&mut local.id, hlocal.hir_id);
        assert_eq!(local.super_, hlocal.super_);
        self.map_pat_to_pat(&mut local.pat, hlocal.pat);
        assert_eq!(local.ty.is_some(), hlocal.ty.is_some());
        if let (Some(ty), Some(hty)) = (&mut local.ty, &hlocal.ty) {
            self.map_ty_to_ty(ty, hty);
        }
        assert_eq!(local.span, hlocal.span);
        assert!(matches!(hlocal.source, hir::LocalSource::Normal));
        match &mut local.kind {
            LocalKind::Decl => {
                assert!(hlocal.init.is_none());
                assert!(hlocal.els.is_none());
            }
            LocalKind::Init(expr) => {
                assert!(hlocal.els.is_none());
                let hexpr = hlocal.init.unwrap();
                self.map_expr_to_expr(expr, hexpr);
            }
            LocalKind::InitElse(expr, block) => {
                let hexpr = hlocal.init.unwrap();
                self.map_expr_to_expr(expr, hexpr);
                let hblock = hlocal.els.unwrap();
                self.map_block_to_block(block, hblock);
            }
        }
    }

    fn map_expr_field_to_expr_field(
        &mut self,
        expr_field: &mut ExprField,
        hexpr_field: &hir::ExprField<'tcx>,
    ) {
        self.add_local(&mut expr_field.id, hexpr_field.hir_id);
        assert_eq!(expr_field.span, hexpr_field.span);
        assert_eq!(expr_field.ident, hexpr_field.ident);
        assert_eq!(expr_field.is_shorthand, hexpr_field.is_shorthand);
        self.map_expr_to_expr(&mut expr_field.expr, hexpr_field.expr);
    }

    fn map_arm_to_arm(&mut self, arm: &mut Arm, harm: &hir::Arm<'tcx>) {
        self.add_local(&mut arm.id, harm.hir_id);
        self.map_pat_to_pat(&mut arm.pat, harm.pat);
        assert_eq!(arm.span, harm.span);
        assert_eq!(arm.guard.is_some(), harm.guard.is_some());
        if let (Some(guard), Some(hguard)) = (&mut arm.guard, &harm.guard) {
            self.map_expr_to_expr(guard, hguard);
        }
        if let Some(body) = &mut arm.body {
            self.map_expr_to_expr(body, harm.body);
        }
    }

    fn map_path_to_qpath<Q: AsMut<QSelf>>(
        &mut self,
        qself: &mut Option<Q>,
        path: &mut Path,
        hqpath: &hir::QPath<'tcx>,
    ) {
        let hir::QPath::Resolved(hqself, hpath) = hqpath else { panic!() };
        assert_eq!(qself.is_some(), hqself.is_some());
        assert_eq!(path.segments.len(), hpath.segments.len());
        if let (Some(qself), Some(hqself)) = (qself, hqself) {
            self.map_ty_to_ty(&mut qself.as_mut().ty, hqself);
        }
        for (seg, hseg) in path.segments.iter_mut().zip(hpath.segments) {
            self.map_path_segment_to_path_segment(seg, hseg);
        }
    }

    fn map_path_segment_to_path_segment(
        &mut self,
        seg: &mut PathSegment,
        hseg: &hir::PathSegment<'tcx>,
    ) {
        self.add_local(&mut seg.id, hseg.hir_id);
        assert_eq!(seg.ident, hseg.ident);
        assert_eq!(seg.args.is_some(), hseg.args.is_some());
        if let (Some(args), Some(hargs)) = (&mut seg.args, &hseg.args) {
            self.map_generic_args_to_generic_args(args, hargs);
        }
    }

    fn map_anon_const_to_anon_const(
        &mut self,
        anon_const: &mut AnonConst,
        hconst_arg: &hir::AnonConst,
    ) {
        self.add_local(&mut anon_const.id, hconst_arg.hir_id);
        self.add_global(&mut anon_const.id, hconst_arg.def_id);
        let body = self.tcx.hir_body(hconst_arg.body);
        assert_eq!(body.params.len(), 0);
        self.map_expr_to_expr(&mut anon_const.value, body.value);
    }

    fn map_anon_const_to_const_arg<U>(
        &mut self,
        anon_const: &mut AnonConst,
        hconst_arg: &hir::ConstArg<'tcx, U>,
    ) {
        self.add_local(&mut anon_const.id, hconst_arg.hir_id);
        match hconst_arg.kind {
            hir::ConstArgKind::Path(..) => todo!(),
            hir::ConstArgKind::Anon(hanon_const) => {
                self.map_anon_const_to_anon_const(anon_const, hanon_const);
            }
            hir::ConstArgKind::Infer(_, _) => {}
        }
    }

    fn map_anon_const_to_const_block(
        &mut self,
        anon_const: &mut AnonConst,
        hconst_block: &hir::ConstBlock,
    ) {
        self.add_local(&mut anon_const.id, hconst_block.hir_id);
        self.add_global(&mut anon_const.id, hconst_block.def_id);
        let body = self.tcx.hir_body(hconst_block.body);
        assert_eq!(body.params.len(), 0);
        self.map_expr_to_expr(&mut anon_const.value, body.value);
    }

    fn map_pat_to_pat(&mut self, pat: &mut Pat, hpat: &hir::Pat<'tcx>) {
        self.add_local(&mut pat.id, hpat.hir_id);
        match &mut pat.kind {
            PatKind::Missing => {
                assert!(matches!(hpat.kind, hir::PatKind::Missing));
            }
            PatKind::Wild => {
                assert!(matches!(hpat.kind, hir::PatKind::Wild));
            }
            PatKind::Ident(mode, ident, pat) => {
                let hir::PatKind::Binding(hmode, _, hident, hpat) = hpat.kind else { panic!() };
                assert_eq!(*mode, hmode);
                assert_eq!(*ident, hident);
                assert_eq!(pat.is_some(), hpat.is_some());
                if let (Some(pat), Some(hpat)) = (pat, hpat) {
                    self.map_pat_to_pat(pat, hpat);
                }
            }
            PatKind::Struct(qself, path, fields, rest) => {
                let hir::PatKind::Struct(hqpath, hfields, hrest) = hpat.kind else { panic!() };
                self.map_path_to_qpath(qself, path, &hqpath);
                assert_eq!(fields.len(), hfields.len());
                for (field, hfield) in fields.iter_mut().zip(hfields) {
                    self.map_pat_field_to_pat_field(field, hfield);
                }
                match rest {
                    PatFieldsRest::Rest => assert!(hrest),
                    PatFieldsRest::Recovered(_) => panic!(),
                    PatFieldsRest::None => assert!(!hrest),
                }
            }
            PatKind::TupleStruct(qself, path, pats) => {
                let hir::PatKind::TupleStruct(hqpath, hpats, pos) = hpat.kind else { panic!() };
                self.map_path_to_qpath(qself, path, &hqpath);
                self.map_pats_to_pats_with_pos(pats, hpats, pos);
            }
            PatKind::Or(pats) => {
                let hir::PatKind::Or(hpats) = hpat.kind else { panic!() };
                assert_eq!(pats.len(), hpats.len());
                for (pat, hpat) in pats.iter_mut().zip(hpats) {
                    self.map_pat_to_pat(pat, hpat);
                }
            }
            PatKind::Path(qself, path) => {
                let hir::PatKind::Struct(hqpath, hfields, hrest) = hpat.kind else { panic!() };
                self.map_path_to_qpath(qself, path, &hqpath);
                assert_eq!(hfields.len(), 0);
                assert!(!hrest);
            }
            PatKind::Tuple(pats) => {
                let hir::PatKind::Tuple(hpats, pos) = hpat.kind else { panic!() };
                self.map_pats_to_pats_with_pos(pats, hpats, pos);
            }
            PatKind::Box(pat) => {
                let hir::PatKind::Box(hpat) = hpat.kind else { panic!() };
                self.map_pat_to_pat(pat, hpat);
            }
            PatKind::Deref(pat) => {
                let hir::PatKind::Deref(hpat) = hpat.kind else { panic!() };
                self.map_pat_to_pat(pat, hpat);
            }
            PatKind::Ref(pat, mutability) => {
                let hir::PatKind::Ref(hpat, hmutability) = hpat.kind else { panic!() };
                assert_eq!(*mutability, hmutability);
                self.map_pat_to_pat(pat, hpat);
            }
            PatKind::Expr(expr) => {
                let hir::PatKind::Expr(hexpr) = hpat.kind else { panic!() };
                self.map_expr_to_pat_expr(expr, hexpr);
            }
            PatKind::Range(from, to, _) => {
                let hir::PatKind::Range(hfrom, hto, _) = hpat.kind else { panic!() };
                assert_eq!(from.is_some(), hfrom.is_some());
                if let (Some(from), Some(hfrom)) = (from, hfrom) {
                    self.map_expr_to_pat_expr(from, hfrom);
                }
                assert_eq!(to.is_some(), hto.is_some());
                if let (Some(to), Some(hto)) = (to, hto) {
                    self.map_expr_to_pat_expr(to, hto);
                }
            }
            PatKind::Slice(pats) => {
                let hir::PatKind::Slice(hpats1, hpat, hpats2) = hpat.kind else { panic!() };
                for (pat, hpat) in pats.iter_mut().zip(hpats1) {
                    self.map_pat_to_pat(pat, hpat);
                }
                if let Some(hpat) = hpat {
                    assert_eq!(pats.len(), hpats1.len() + 1 + hpats2.len());
                    let pat = &mut pats[hpats1.len()];
                    self.add_local(&mut pat.id, hpat.hir_id);
                    match &mut pat.kind {
                        PatKind::Rest => {
                            assert!(matches!(hpat.kind, hir::PatKind::Wild));
                        }
                        PatKind::Ident(mode, ident, Some(box pat)) => {
                            assert!(matches!(pat.kind, PatKind::Rest));
                            let hir::PatKind::Binding(hmode, _, hident, Some(hpat)) = hpat.kind
                            else {
                                panic!()
                            };
                            assert_eq!(*mode, hmode);
                            assert_eq!(*ident, hident);
                            assert!(matches!(hpat.kind, hir::PatKind::Wild));
                            self.add_local(&mut pat.id, hpat.hir_id);
                        }
                        _ => panic!(),
                    }
                    for (pat, hpat) in pats.iter_mut().skip(hpats1.len() + 1).zip(hpats2) {
                        self.map_pat_to_pat(pat, hpat);
                    }
                } else {
                    assert_eq!(pats.len(), hpats1.len());
                }
            }
            PatKind::Rest => panic!(),
            PatKind::Never => {
                assert!(matches!(hpat.kind, hir::PatKind::Never));
            }
            PatKind::Guard(pat, expr) => {
                let hir::PatKind::Guard(hpat, hexpr) = hpat.kind else { panic!() };
                self.map_pat_to_pat(pat, hpat);
                self.map_expr_to_expr(expr, hexpr);
            }
            PatKind::Paren(pat) => {
                self.map_pat_to_pat(pat, hpat);
            }
            PatKind::MacCall(..) => todo!(),
            PatKind::Err(_) => panic!(),
        }
    }

    fn map_pats_to_pats_with_pos<P: AsMut<Pat>>(
        &mut self,
        pats: &mut [P],
        hpats: &[hir::Pat<'tcx>],
        pos: hir::DotDotPos,
    ) {
        if let Some(pos) = pos.as_opt_usize() {
            assert_eq!(pats.len(), hpats.len() + 1);
            for (i, pat) in pats.iter_mut().enumerate() {
                match i.cmp(&pos) {
                    Ordering::Less => {
                        let hpat = &hpats[i];
                        self.map_pat_to_pat(pat.as_mut(), hpat);
                    }
                    Ordering::Equal => {
                        assert!(matches!(pat.as_mut().kind, PatKind::Rest));
                    }
                    Ordering::Greater => {
                        let hpat = &hpats[i - 1];
                        self.map_pat_to_pat(pat.as_mut(), hpat);
                    }
                }
            }
        } else {
            assert_eq!(pats.len(), hpats.len());
            for (pat, hpat) in pats.iter_mut().zip(hpats) {
                self.map_pat_to_pat(pat.as_mut(), hpat);
            }
        }
    }

    fn map_pat_field_to_pat_field(
        &mut self,
        pat_field: &mut PatField,
        hpat_field: &hir::PatField<'tcx>,
    ) {
        self.add_local(&mut pat_field.id, hpat_field.hir_id);
        assert_eq!(pat_field.ident, hpat_field.ident);
        assert_eq!(pat_field.is_shorthand, hpat_field.is_shorthand);
        assert_eq!(pat_field.span, hpat_field.span);
        self.map_pat_to_pat(&mut pat_field.pat, hpat_field.pat);
    }

    fn map_expr_to_pat_expr(&mut self, expr: &mut Expr, hpat_expr: &hir::PatExpr<'tcx>) {
        self.add_local(&mut expr.id, hpat_expr.hir_id);
        match &mut expr.kind {
            ExprKind::Lit(_) => {
                let hir::PatExprKind::Lit { negated: false, .. } = hpat_expr.kind else { panic!() };
            }
            ExprKind::ConstBlock(c) => {
                let hir::PatExprKind::ConstBlock(hc) = hpat_expr.kind else { panic!() };
                self.map_anon_const_to_const_block(c, &hc);
            }
            ExprKind::IncludedBytes(_) => todo!(),
            ExprKind::Path(_, _) => todo!(),
            ExprKind::Unary(UnOp::Neg, expr) if let ExprKind::Lit(_) = &expr.kind => {
                let hir::PatExprKind::Lit { negated: true, .. } = hpat_expr.kind else { panic!() };
            }
            _ => panic!(),
        }
    }

    fn map_ty_to_ty<U>(&mut self, ty: &mut Ty, hty: &hir::Ty<'tcx, U>) {
        self.add_local(&mut ty.id, hty.hir_id);
        match &mut ty.kind {
            TyKind::Slice(ty) => {
                let hir::TyKind::Slice(hty) = hty.kind else { panic!() };
                self.map_ty_to_ty(ty, hty);
            }
            TyKind::Array(ty, len) => {
                let hir::TyKind::Array(hty, hlen) = hty.kind else { panic!() };
                self.map_ty_to_ty(ty, hty);
                self.map_anon_const_to_const_arg(len, hlen);
            }
            TyKind::Ptr(mut_ty) => {
                let hir::TyKind::Ptr(hmut_ty) = hty.kind else { panic!() };
                self.map_mut_ty_to_mut_ty(mut_ty, &hmut_ty);
            }
            TyKind::Ref(lifetime, mut_ty) => {
                let hir::TyKind::Ref(hlifetime, hmut_ty) = hty.kind else { panic!() };
                if let Some(lifetime) = lifetime {
                    self.map_lifetime_to_lifetime(lifetime, hlifetime);
                }
                self.map_mut_ty_to_mut_ty(mut_ty, &hmut_ty);
            }
            TyKind::PinnedRef(..) => todo!(),
            TyKind::BareFn(box BareFnTy {
                generic_params,
                decl,
                ..
            }) => {
                let hir::TyKind::BareFn(hir::BareFnTy {
                    generic_params: hgeneric_params,
                    decl: hdecl,
                    ..
                }) = hty.kind
                else {
                    panic!()
                };
                assert_eq!(generic_params.len(), hgeneric_params.len());
                for (param, hparam) in generic_params.iter_mut().zip(*hgeneric_params) {
                    self.map_generic_param_to_generic_param(param, hparam);
                }
                self.map_fn_decl_to_fn_decl(decl, hdecl);
            }
            TyKind::UnsafeBinder(box UnsafeBinderTy {
                generic_params,
                inner_ty,
            }) => {
                let hir::TyKind::UnsafeBinder(hir::UnsafeBinderTy {
                    generic_params: hgeneric_params,
                    inner_ty: hinner_ty,
                }) = hty.kind
                else {
                    panic!()
                };
                assert_eq!(generic_params.len(), hgeneric_params.len());
                for (param, hparam) in generic_params.iter_mut().zip(*hgeneric_params) {
                    self.map_generic_param_to_generic_param(param, hparam);
                }
                self.map_ty_to_ty(inner_ty, hinner_ty);
            }
            TyKind::Never => {
                assert!(matches!(hty.kind, hir::TyKind::Never));
            }
            TyKind::Tup(tys) => {
                let hir::TyKind::Tup(htys) = hty.kind else { panic!() };
                assert_eq!(tys.len(), htys.len());
                for (ty, hty) in tys.iter_mut().zip(htys) {
                    self.map_ty_to_ty(ty, hty);
                }
            }
            TyKind::Path(qself, path) => {
                let hir::TyKind::Path(hqpath) = hty.kind else { panic!() };
                self.map_path_to_qpath(qself, path, &hqpath);
            }
            TyKind::TraitObject(bounds, _) => {
                let hir::TyKind::TraitObject(htrefs, _) = hty.kind else { panic!() };
                let mut i = 0;
                for bound in bounds {
                    let GenericBound::Trait(tref) = bound else { continue };
                    let htref = &htrefs[i];
                    self.map_poly_trait_ref_to_poly_trait_ref(tref, htref);
                    i += 1;
                }
            }
            TyKind::ImplTrait(node_id, bounds) => {
                let hir::TyKind::OpaqueDef(hopaque_ty) = hty.kind else { panic!() };
                self.add_local(node_id, hopaque_ty.hir_id);
                self.add_global(node_id, hopaque_ty.def_id);
                assert_eq!(bounds.len(), hopaque_ty.bounds.len());
                for (bound, hbound) in bounds.iter_mut().zip(hopaque_ty.bounds) {
                    self.map_generic_bound_to_generic_bound(bound, hbound);
                }
            }
            TyKind::Paren(ty) => {
                self.map_ty_to_ty(ty, hty);
            }
            TyKind::Typeof(_) => panic!(),
            TyKind::Infer => {}
            TyKind::ImplicitSelf => {}
            TyKind::MacCall(..) => todo!(),
            TyKind::CVarArgs => {}
            TyKind::Pat(..) => todo!(),
            TyKind::Dummy => panic!(),
            TyKind::Err(_) => panic!(),
        }
    }

    fn map_mut_ty_to_mut_ty(&mut self, mut_ty: &mut MutTy, hmut_ty: &hir::MutTy<'tcx>) {
        assert_eq!(mut_ty.mutbl, hmut_ty.mutbl);
        self.map_ty_to_ty(&mut mut_ty.ty, hmut_ty.ty);
    }

    fn map_lifetime_to_lifetime(&mut self, lifetime: &mut Lifetime, hlifetime: &hir::Lifetime) {
        self.add_local(&mut lifetime.id, hlifetime.hir_id);
        assert_eq!(lifetime.ident, hlifetime.ident);
    }

    fn map_generics_to_generics(
        &mut self,
        generics: &mut Generics,
        hgenerics: &hir::Generics<'tcx>,
    ) {
        assert_eq!(generics.span, hgenerics.span);
        assert_eq!(generics.params.len(), hgenerics.params.len());
        for (param, hparam) in generics.params.iter_mut().zip(hgenerics.params) {
            self.map_generic_param_to_generic_param(param, hparam);
        }
        let mut i = 0;
        for param in &mut generics.params {
            if param.bounds.is_empty() {
                continue;
            }
            let hpredicate = &hgenerics.predicates[i];
            match &mut param.kind {
                GenericParamKind::Lifetime => {
                    let hir::WherePredicateKind::RegionPredicate(hpred) = hpredicate.kind else {
                        panic!()
                    };
                    assert_eq!(param.bounds.len(), hpred.bounds.len());
                    for (bound, hbound) in param.bounds.iter_mut().zip(hpred.bounds) {
                        self.map_generic_bound_to_generic_bound(bound, hbound);
                    }
                }
                GenericParamKind::Type { .. } => {
                    let hir::WherePredicateKind::BoundPredicate(hpred) = &hpredicate.kind else {
                        panic!()
                    };
                    assert_eq!(param.bounds.len(), hpred.bounds.len());
                    for (bound, hbound) in param.bounds.iter_mut().zip(hpred.bounds) {
                        self.map_generic_bound_to_generic_bound(bound, hbound);
                    }
                }
                GenericParamKind::Const { .. } => continue,
            }
            i += 1;
        }
        assert_eq!(
            generics.where_clause.predicates.len() + i,
            hgenerics.predicates.len()
        );
        for (predicate, hpredicate) in generics
            .where_clause
            .predicates
            .iter_mut()
            .zip(hgenerics.predicates.iter().skip(i))
        {
            self.map_where_predicate_to_where_predicate(predicate, hpredicate);
        }
    }

    fn map_generic_param_to_generic_param(
        &mut self,
        param: &mut GenericParam,
        hparam: &hir::GenericParam<'tcx>,
    ) {
        self.add_local(&mut param.id, hparam.hir_id);
        self.add_global(&mut param.id, hparam.def_id);
        let hir::ParamName::Plain(hident) = hparam.name else { panic!() };
        assert_eq!(param.ident, hident);
        assert_eq!(param.colon_span, hparam.colon_span);
        match &mut param.kind {
            GenericParamKind::Lifetime => {
                let hir::GenericParamKind::Lifetime { .. } = hparam.kind else { panic!() };
            }
            GenericParamKind::Type { default } => {
                let hir::GenericParamKind::Type {
                    default: hdefault, ..
                } = hparam.kind
                else {
                    panic!()
                };
                assert_eq!(default.is_some(), hdefault.is_some());
                if let (Some(default), Some(hdefault)) = (default, hdefault) {
                    self.map_ty_to_ty(default, hdefault);
                }
            }
            GenericParamKind::Const { ty, default, .. } => {
                let hir::GenericParamKind::Const {
                    ty: hty,
                    default: hdefault,
                    ..
                } = hparam.kind
                else {
                    panic!()
                };
                self.map_ty_to_ty(ty, hty);
                assert_eq!(default.is_some(), hdefault.is_some());
                if let (Some(default), Some(hdefault)) = (default, hdefault) {
                    self.map_anon_const_to_const_arg(default, hdefault);
                }
            }
        }
    }

    fn map_where_predicate_to_where_predicate(
        &mut self,
        predicate: &mut WherePredicate,
        hpredicate: &hir::WherePredicate<'tcx>,
    ) {
        self.add_local(&mut predicate.id, hpredicate.hir_id);
        assert_eq!(predicate.span, hpredicate.span);
        match &mut predicate.kind {
            WherePredicateKind::BoundPredicate(pred) => {
                let hir::WherePredicateKind::BoundPredicate(hpred) = &hpredicate.kind else {
                    panic!()
                };
                assert_eq!(
                    pred.bound_generic_params.len(),
                    hpred.bound_generic_params.len()
                );
                for (param, hparam) in pred
                    .bound_generic_params
                    .iter_mut()
                    .zip(hpred.bound_generic_params)
                {
                    self.map_generic_param_to_generic_param(param, hparam);
                }
                self.map_ty_to_ty(&mut pred.bounded_ty, hpred.bounded_ty);
                assert_eq!(pred.bounds.len(), hpred.bounds.len());
                for (bound, hbound) in pred.bounds.iter_mut().zip(hpred.bounds) {
                    self.map_generic_bound_to_generic_bound(bound, hbound);
                }
            }
            WherePredicateKind::RegionPredicate(pred) => {
                let hir::WherePredicateKind::RegionPredicate(hpred) = &hpredicate.kind else {
                    panic!()
                };
                self.map_lifetime_to_lifetime(&mut pred.lifetime, hpred.lifetime);
                assert_eq!(pred.bounds.len(), hpred.bounds.len());
                for (bound, hbound) in pred.bounds.iter_mut().zip(hpred.bounds) {
                    self.map_generic_bound_to_generic_bound(bound, hbound);
                }
            }
            WherePredicateKind::EqPredicate(_) => panic!(),
        }
    }

    fn map_generic_bound_to_generic_bound(
        &mut self,
        bound: &mut GenericBound,
        hbound: &hir::GenericBound<'tcx>,
    ) {
        match bound {
            GenericBound::Trait(tref) => {
                let hir::GenericBound::Trait(htref) = hbound else { panic!() };
                self.map_poly_trait_ref_to_poly_trait_ref(tref, htref);
            }
            GenericBound::Outlives(lifetime) => {
                let hir::GenericBound::Outlives(hlifetime) = hbound else { panic!() };
                self.map_lifetime_to_lifetime(lifetime, hlifetime);
            }
            GenericBound::Use(args, span) => {
                let hir::GenericBound::Use(hargs, hspan) = hbound else { panic!() };
                assert_eq!(span, hspan);
                assert_eq!(args.len(), hargs.len());
                for (arg, harg) in args.iter_mut().zip(*hargs) {
                    match arg {
                        PreciseCapturingArg::Lifetime(lifetime) => {
                            let hir::PreciseCapturingArg::Lifetime(hlifetime) = harg else {
                                panic!()
                            };
                            self.map_lifetime_to_lifetime(lifetime, hlifetime);
                        }
                        PreciseCapturingArg::Arg(path, node_id) => {
                            let [segment] = path.segments.as_slice() else {
                                panic!();
                            };
                            let hir::PreciseCapturingArg::Param(param) = harg else { panic!() };
                            assert_eq!(segment.ident, param.ident);
                            self.add_local(&mut *node_id, param.hir_id);
                        }
                    }
                }
            }
        }
    }

    fn map_poly_trait_ref_to_poly_trait_ref(
        &mut self,
        tref: &mut PolyTraitRef,
        htref: &hir::PolyTraitRef<'tcx>,
    ) {
        assert_eq!(
            tref.bound_generic_params.len(),
            htref.bound_generic_params.len()
        );
        for (param, hparam) in tref
            .bound_generic_params
            .iter_mut()
            .zip(htref.bound_generic_params)
        {
            self.map_generic_param_to_generic_param(param, hparam);
        }
        assert_eq!(tref.span, htref.span);
        self.map_trait_ref_to_trait_ref(&mut tref.trait_ref, &htref.trait_ref);
    }

    fn map_trait_ref_to_trait_ref(
        &mut self,
        trait_ref: &mut TraitRef,
        htrait_ref: &hir::TraitRef<'tcx>,
    ) {
        self.add_local(&mut trait_ref.ref_id, htrait_ref.hir_ref_id);
        self.map_path_to_path(&mut trait_ref.path, htrait_ref.path);
    }

    fn map_path_to_path(&mut self, path: &mut Path, hpath: &hir::Path<'tcx>) {
        assert_eq!(path.span, hpath.span);
        assert_eq!(path.segments.len(), hpath.segments.len());
        for (seg, hseg) in path.segments.iter_mut().zip(hpath.segments) {
            self.map_path_segment_to_path_segment(seg, hseg);
        }
    }

    fn map_generic_args_to_generic_args(
        &mut self,
        args: &mut GenericArgs,
        hargs: &hir::GenericArgs<'tcx>,
    ) {
        match args {
            GenericArgs::AngleBracketed(args) => {
                let mut arg_i = 0;
                let mut constraint_i = 0;
                for arg in &mut args.args {
                    match arg {
                        AngleBracketedArg::Arg(arg) => {
                            let harg = &hargs.args[arg_i];
                            self.map_generic_arg_to_generic_arg(arg, harg);
                            arg_i += 1;
                        }
                        AngleBracketedArg::Constraint(constraint) => {
                            let hconstraint = &hargs.constraints[constraint_i];
                            self.map_assoc_item_constraint_to_assoc_item_constraint(
                                constraint,
                                hconstraint,
                            );
                            constraint_i += 1;
                        }
                    }
                }
            }
            GenericArgs::Parenthesized(args) => {
                assert_eq!(hargs.args.len(), 1);
                let hir::GenericArg::Type(hty) = hargs.args[0] else { panic!() };
                let hir::TyKind::Tup(hinputs) = hty.kind else { panic!() };
                for (input, hinput) in args.inputs.iter_mut().zip(hinputs) {
                    self.map_ty_to_ty(input, hinput);
                }
                assert_eq!(hargs.constraints.len(), 1);
                let hir::AssocItemConstraintKind::Equality { term } = hargs.constraints[0].kind
                else {
                    panic!()
                };
                let hir::Term::Ty(hty) = term else { panic!() };
                match &mut args.output {
                    FnRetTy::Ty(output) => {
                        self.map_ty_to_ty(output, hty);
                    }
                    FnRetTy::Default(_) => {
                        let hir::TyKind::Tup(houtput) = hty.kind else { panic!() };
                        assert_eq!(houtput.len(), 0);
                    }
                }
            }
            GenericArgs::ParenthesizedElided(_) => {}
        }
    }

    fn map_generic_arg_to_generic_arg(
        &mut self,
        arg: &mut GenericArg,
        harg: &hir::GenericArg<'tcx>,
    ) {
        match arg {
            GenericArg::Lifetime(lifetime) => {
                let hir::GenericArg::Lifetime(hlifetime) = harg else { panic!() };
                self.map_lifetime_to_lifetime(lifetime, hlifetime);
            }
            GenericArg::Type(ty) => {
                let hir::GenericArg::Type(hty) = harg else { panic!() };
                self.map_ty_to_ty(ty, hty);
            }
            GenericArg::Const(anon_const) => {
                let hir::GenericArg::Const(hconst_arg) = harg else { panic!() };
                self.map_anon_const_to_const_arg(anon_const, hconst_arg);
            }
        }
    }

    fn map_assoc_item_constraint_to_assoc_item_constraint(
        &mut self,
        constraint: &mut AssocItemConstraint,
        hconstraint: &hir::AssocItemConstraint<'tcx>,
    ) {
        self.add_local(&mut constraint.id, hconstraint.hir_id);
        assert_eq!(constraint.ident, hconstraint.ident);
        assert_eq!(constraint.span, hconstraint.span);
        if let Some(args) = constraint.gen_args.as_mut() {
            self.map_generic_args_to_generic_args(args, hconstraint.gen_args);
        }
        match &mut constraint.kind {
            AssocItemConstraintKind::Equality { term } => {
                let hir::AssocItemConstraintKind::Equality { term: hterm } = hconstraint.kind
                else {
                    panic!()
                };
                match term {
                    Term::Ty(ty) => {
                        let hir::Term::Ty(hty) = hterm else { panic!() };
                        self.map_ty_to_ty(ty, hty);
                    }
                    Term::Const(anon_const) => {
                        let hir::Term::Const(hconst_arg) = hterm else { panic!() };
                        self.map_anon_const_to_const_arg(anon_const, hconst_arg);
                    }
                }
            }
            AssocItemConstraintKind::Bound { bounds } => {
                let hir::AssocItemConstraintKind::Bound { bounds: hbounds } = hconstraint.kind
                else {
                    panic!()
                };
                assert_eq!(bounds.len(), hbounds.len());
                for (bound, hbound) in bounds.iter_mut().zip(hbounds) {
                    self.map_generic_bound_to_generic_bound(bound, hbound);
                }
            }
        }
    }
}
