#![allow(dead_code)]

pub mod analyze;
pub mod hir;
pub mod mir;

use rustc_hir::{Ty, def_id::LocalDefId};
use rustc_middle::ty::TyCtxt;
use rustc_span::{Ident, Span};

use crate::finder::enum_finder::{
    hir::{definition::find_enum_tys, usage::find_enum_usage},
    mir::{get_optimized_mirs, process_mirs},
};

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariant {
    def_id: LocalDefId,
    span: Span,
    value: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDefinition {
    def_id: LocalDefId,
    span: Span,
    variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnumTy {
    Definition(EnumDefinition),
    // For the pattern `typedef enum Name_t { ... } Name;`
    PointsTo(LocalDefId, Span, EnumDefinition),
}

#[derive(Debug, Clone)]
pub enum EnumTyAnnotation<'tcx> {
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
    // for enum_ty in &enum_tys {
    //     match enum_ty {
    //         EnumTy::Definition(def) => dbg!(&def.span),
    //         EnumTy::PointsTo(_, span, _) => dbg!(span),
    //     };
    // }

    let enum_usages = find_enum_usage(tcx, enum_tys.clone());
    // for enum_ty_ann in &enum_usages {
    //     match enum_ty_ann {
    //         EnumTyAnnotation::Let(_, span, _)
    //         | EnumTyAnnotation::Struct(_, _, span, _)
    //         | EnumTyAnnotation::Fn(_, _, span, _, _) => dbg!(span),
    //     };
    // }

    let mir_basic_blocks = get_optimized_mirs(tcx);
    process_mirs(&mir_basic_blocks, tcx, &enum_tys, &enum_usages);
}
