mod enum_ty;

use rustc_hir::{
    BindingMode, ByRef, PatKind, Ty,
    intravisit::{Visitor, walk_fn, walk_local},
};
use rustc_middle::{hir::nested_filter::OnlyBodies, ty::TyCtxt};
use rustc_span::{Ident, Span};

use crate::finder::enum_finder::{EnumTy, usage::enum_ty::is_enum_ty};

#[derive(Debug, Clone)]
enum EnumTyBindings<'tcx> {
    Let(Ident, Span, &'tcx Ty<'tcx>),
    Field(Ident, Span, &'tcx Ty<'tcx>),
}

struct CollectEnumTyBindings<'tcx> {
    tcx: TyCtxt<'tcx>,
    ty_annotations: Vec<EnumTyBindings<'tcx>>,
    enum_tys: Vec<EnumTy>,
}

impl<'tcx> Visitor<'tcx> for CollectEnumTyBindings<'tcx> {
    type NestedFilter = OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_local(&mut self, l: &'tcx rustc_hir::LetStmt<'tcx>) -> Self::Result {
        if let PatKind::Binding(BindingMode(ByRef::No, _), hir_id, ident, None) = l.pat.kind
            && let Some(ty) = l.ty
            && is_enum_ty(ty, &self.enum_tys)
        {
            self.ty_annotations
                .push(EnumTyBindings::Let(ident, l.pat.span, ty));
        }
        walk_local(self, l);
    }

    fn visit_fn(
        &mut self,
        fk: rustc_hir::intravisit::FnKind<'tcx>,
        fd: &'tcx rustc_hir::FnDecl<'tcx>,
        body: rustc_hir::BodyId,
        _: rustc_span::Span,
        id: rustc_hir::def_id::LocalDefId,
    ) -> Self::Result {
        let input_tys = fd.inputs;
        let ret_ty = fd.output;

        let input_params = self.tcx.hir_body(body).params;

        // for param in input_params {
        //     dbg!(ret_ty);
        //     dbg!(param.span);
        //     dbg!(param.pat.kind);
        // }

        walk_fn(self, fk, fd, body, id);
    }

    // fn visit_field_def(&mut self, s: &'tcx rustc_hir::FieldDef<'tcx>) -> Self::Result {}
}

pub(super) fn find_enum_usage<'tcx>(tcx: TyCtxt<'tcx>, enum_tys: Vec<EnumTy>) {
    let visitor = &mut CollectEnumTyBindings {
        tcx,
        ty_annotations: vec![],
        enum_tys,
    };

    tcx.hir_visit_all_item_likes_in_crate(visitor);

    visitor.ty_annotations.iter().for_each(|binding| {
        // dbg!(binding);
    });

    todo!();
}

#[cfg(test)]
mod tests {
    use crate::compile_util;

    #[test]
    fn test_enum_usages() {
        compile_util::run_compiler_on_str(
            r#"
mod libc {
    pub type c_uint = u32;
}

pub type BrotliDecoderParameter = libc::c_uint;
pub const BROTLI_DECODER_PARAM_LARGE_WINDOW: BrotliDecoderParameter = 1;
pub const BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: BrotliDecoderParameter = 0;

fn f() -> BrotliDecoderParameter {
    let x: libc::c_uint = 0 as libc::c_uint;
    let y: BrotliDecoderParameter = x as BrotliDecoderParameter;
    0 as BrotliDecoderParameter
}
"#,
            |_tcx| {
                // let enum_definitions = find_enum_def(tcx);
                // assert_eq!(enum_definitions.len(), 1);

                // assert_eq!(enum_definitions[0].variants[0].value, 1);
                // assert_eq!(enum_definitions[0].variants[1].value, 0);
            },
        )
        .unwrap()
    }
}
