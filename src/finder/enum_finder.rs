mod definition;
mod usage;

use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::finder::enum_finder::{definition::find_enum_tys, usage::find_enum_usage};

#[derive(Clone, Debug, PartialEq)]
struct EnumVariant {
    def_id: LocalDefId,
    span: Span,
    value: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct EnumDefinition {
    def_id: LocalDefId,
    span: Span,
    variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, PartialEq)]
enum EnumTys {
    Definition(EnumDefinition),
    PointsTo(LocalDefId, Span, EnumDefinition),
}

impl EnumTys {
    pub fn get_def_id(&self) -> LocalDefId {
        match self {
            EnumTys::Definition(def) => def.def_id,
            EnumTys::PointsTo(def_id, _, _) => *def_id,
        }
    }
}

pub fn find_enum<'tcx>(tcx: TyCtxt<'tcx>) {
    let enum_tys = find_enum_tys(tcx);
    for def in enum_tys {
        dbg!(def);
    }
    find_enum_usage(tcx, enum_tys);
}
