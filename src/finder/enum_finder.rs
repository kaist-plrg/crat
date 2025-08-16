mod definition;
mod usage;

use rustc_hir::def_id::{LocalDefId};
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::finder::enum_finder::{definition::find_enum_def, usage::find_enum_usage};

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

pub fn find_enum(tcx: TyCtxt<'_>) {
    let enum_definitions = find_enum_def(tcx);
    // for def in enum_definitions {
    //     dbg!(def.span);
    // }
    find_enum_usage(tcx, enum_definitions);
}
