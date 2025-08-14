mod definition;
mod usage;

use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::finder::enum_finder::definition::find_enum_def;

#[derive(Clone, Debug, PartialEq)]
struct EnumVariant {
    def_id: DefId,
    span: Span,
    value: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct EnumDefinition {
    def_id: DefId,
    span: Span,
    variants: Vec<EnumVariant>,
}

pub fn find_enum(tcx: TyCtxt<'_>) {
    find_enum_def(tcx);
}
