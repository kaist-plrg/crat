use rustc_hir::{
    BareFnTy, FnDecl, FnRetTy, MutTy, Path, QPath, Ty, TyKind::*, UnsafeBinderTy, def::Res,
};

use crate::finder::enum_finder::EnumTys;

pub(super) fn is_enum_ty(ty: &Ty<'_>, enum_tys: &[EnumTys]) -> bool {
    match ty.kind {
        InferDelegation(_, _) => false,
        Slice(ty) | Array(ty, _) => is_enum_ty(ty, enum_tys),
        Ptr(mut_ty) | Ref(_, mut_ty) => is_enum_mut_ty(&mut_ty, enum_tys),
        BareFn(BareFnTy {
            decl:
                FnDecl {
                    inputs,
                    output: FnRetTy::Return(ret_ty),
                    ..
                },
            ..
        }) => {
            inputs.iter().any(|input| is_enum_ty(input, enum_tys)) || is_enum_ty(ret_ty, enum_tys)
        }
        UnsafeBinder(UnsafeBinderTy { inner_ty, .. }) => is_enum_ty(inner_ty, enum_tys),
        Never => false,
        Tup(items) => items.iter().any(|item| is_enum_ty(item, enum_tys)),
        Path(QPath::Resolved(
            None,
            Path {
                res: Res::Def(_, def_id),
                ..
            },
        )) => {
            def_id.is_local()
                && enum_tys
                    .iter()
                    .any(|def| Some(def.get_def_id()) == def_id.as_local())
        }
        OpaqueDef(_) | TraitAscription(_) | TraitObject(_, _) => false,
        Typeof(_) => unreachable!(),
        Err(_) | Pat(_, _) => false,
        Infer(_) => false,
        _ => false,
    }
}

fn is_enum_mut_ty<'tcx>(mut_ty: &MutTy<'tcx>, enum_tys: &'tcx [EnumTys]) -> bool {
    let MutTy { ty, mutbl: _ } = mut_ty;
    is_enum_ty(ty, enum_tys)
}
