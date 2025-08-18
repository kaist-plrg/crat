#![allow(dead_code)]

mod definition;
pub mod usage;

use rustc_hir::{Ty, def_id::LocalDefId};
use rustc_middle::ty::TyCtxt;
use rustc_span::{Ident, Span};

use crate::finder::enum_finder::{definition::find_enum_tys, usage::find_enum_usage};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EnumVariant {
    def_id: LocalDefId,
    span: Span,
    value: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EnumDefinition {
    def_id: LocalDefId,
    span: Span,
    variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EnumTy {
    Definition(EnumDefinition),
    // For the pattern `typedef enum Name_t { ... } Name;`
    PointsTo(LocalDefId, Span, EnumDefinition),
}

#[derive(Debug, Clone)]
pub(crate) enum EnumTyAnnotation<'tcx> {
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

impl EnumTy {
    pub fn get_def_id(&self) -> LocalDefId {
        match self {
            EnumTy::Definition(def) => def.def_id,
            EnumTy::PointsTo(def_id, _, _) => *def_id,
        }
    }
}

pub fn find_enum<'tcx>(tcx: TyCtxt<'tcx>) {
    let enum_tys = find_enum_tys(tcx);
    let enum_usages = find_enum_usage(tcx, enum_tys);

    dbg!(enum_usages);
}
