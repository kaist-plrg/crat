mod enum_ty;

use rustc_hir::{
    BindingMode, ByRef, FnRetTy, ForeignItem, ForeignItemKind, Item, ItemKind, Node, PatKind, Ty,
    VariantData,
    def_id::LocalDefId,
    intravisit::{Visitor, walk_fn_decl, walk_local},
};
use rustc_middle::{hir::nested_filter::OnlyBodies, ty::TyCtxt};
use rustc_span::{Ident, Span};

use crate::finder::enum_finder::{EnumTy, definition::find_free_items, usage::enum_ty::is_enum_ty};

#[derive(Debug, Clone)]
pub enum TyAnnotation<'tcx> {
    Let(Ident, Span, &'tcx Ty<'tcx>),
    Struct(
        LocalDefId,
        Ident,
        Span,
        Vec<(LocalDefId, Ident, Span, &'tcx Ty<'tcx>)>,
    ),
    Fn(
        LocalDefId,
        Ident,
        Span,
        Vec<Option<&'tcx Ty<'tcx>>>, // Argument: `Some` only if `is_enum_ty`
        Option<&'tcx Ty<'tcx>>,      // Return: `Some` only if `is_enum_ty`
    ),
}

struct CollectEnumTyBindings<'tcx> {
    tcx: TyCtxt<'tcx>,
    ty_annotations: Vec<TyAnnotation<'tcx>>,
    enum_tys: Vec<EnumTy>,
}

impl<'tcx> Visitor<'tcx> for CollectEnumTyBindings<'tcx> {
    type NestedFilter = OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_local(&mut self, l: &'tcx rustc_hir::LetStmt<'tcx>) -> Self::Result {
        if let PatKind::Binding(BindingMode(ByRef::No, _), _, ident, None) = l.pat.kind
            && let Some(ty) = l.ty
            && is_enum_ty(ty, &self.enum_tys)
        {
            self.ty_annotations
                .push(TyAnnotation::Let(ident, l.pat.span, ty));
        }
        walk_local(self, l);
    }

    fn visit_fn_decl(&mut self, fd: &'tcx rustc_hir::FnDecl<'tcx>) -> Self::Result {
        'b: {
            let reference_ty = if fd.inputs.is_empty() {
                if let FnRetTy::Return(ty) = fd.output {
                    ty
                } else {
                    break 'b;
                }
            } else {
                &fd.inputs[0]
            };

            let arguments = fd
                .inputs
                .iter()
                .map(|ty| {
                    if is_enum_ty(ty, &self.enum_tys) {
                        Some(ty)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let return_ty = if let FnRetTy::Return(ty) = fd.output {
                if is_enum_ty(ty, &self.enum_tys) {
                    Some(ty)
                } else {
                    None
                }
            } else {
                None
            };

            if arguments.iter().all(|arg| arg.is_none()) && return_ty.is_none() {
                break 'b;
            }

            let parent_node = self.tcx.parent_hir_node(reference_ty.hir_id);

            match parent_node {
                Node::ForeignItem(ForeignItem {
                    kind: ForeignItemKind::Fn(sig, _, _),
                    owner_id,
                    ident,
                    ..
                })
                | Node::Item(Item {
                    kind: ItemKind::Fn { sig, ident, .. },
                    owner_id,
                    ..
                }) => {
                    self.ty_annotations.push(TyAnnotation::Fn(
                        owner_id.def_id,
                        *ident,
                        sig.span,
                        arguments,
                        return_ty,
                    ));
                }
                _ => break 'b,
            }
        }

        walk_fn_decl(self, fd);
    }

    // fn visit_field_def(&mut self, s: &'tcx rustc_hir::FieldDef<'tcx>) -> Self::Result {}
}

pub(super) fn find_enum_usage<'tcx>(
    tcx: TyCtxt<'tcx>,
    enum_tys: Vec<EnumTy>,
) -> Vec<TyAnnotation<'tcx>> {
    let local_fn_visitor = &mut CollectEnumTyBindings {
        tcx,
        ty_annotations: vec![],
        enum_tys: enum_tys.clone(),
    };

    tcx.hir_visit_all_item_likes_in_crate(local_fn_visitor);

    let local_fn_bindings = &local_fn_visitor.ty_annotations;

    let struct_bindings = find_free_items(tcx)
        .iter()
        .filter_map(|item| match item.kind {
            ItemKind::Struct(ident, _, VariantData::Struct { fields, .. }) => {
                let fields = fields
                    .iter()
                    .filter_map(|field| {
                        if is_enum_ty(field.ty, &enum_tys) {
                            Some((field.def_id, field.ident, field.span, field.ty))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                if fields.is_empty() {
                    None
                } else {
                    Some(TyAnnotation::Struct(
                        item.owner_id.def_id,
                        ident,
                        item.span,
                        fields,
                    ))
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    struct_bindings
        .into_iter()
        .chain(local_fn_bindings.iter().cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        compile_util,
        finder::enum_finder::{definition::find_enum_tys, find_enum_usage, usage::TyAnnotation},
    };

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

struct BrotliDecoder {
    just: u32,
    params: BrotliDecoderParameter,
}

fn f(a1: u32, a2: u32, a3: BrotliDecoderParameter) -> BrotliDecoderParameter {
    let x: libc::c_uint = 0 as libc::c_uint;
    let y: BrotliDecoderParameter = x as BrotliDecoderParameter;
    0 as BrotliDecoderParameter
}
"#,
            |tcx| {
                let enum_tys = find_enum_tys(tcx);

                let enum_usages = find_enum_usage(tcx, enum_tys);

                assert_eq!(enum_usages.len(), 3);
                assert!(matches!(enum_usages[0], TyAnnotation::Struct(_, _, _, _)));
                assert!(matches!(enum_usages[1], TyAnnotation::Fn(_, _, _, _, _)));
                assert!(matches!(enum_usages[2], TyAnnotation::Let(_, _, _)));

                if let TyAnnotation::Struct(_, _, _, fields) = &enum_usages[0] {
                    assert_eq!(fields.len(), 1);
                } else {
                    unreachable!();
                }

                if let TyAnnotation::Fn(_, _, _, args, return_ty) = &enum_usages[1] {
                    assert_eq!(args.len(), 3);
                    assert!(args[0].is_none());
                    assert!(args[1].is_none());
                    assert!(args[2].is_some());
                    assert!(return_ty.is_some());
                } else {
                    unreachable!();
                }
            },
        )
        .unwrap()
    }
}
