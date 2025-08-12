mod definition;
mod usage;

use rustc_middle::ty::TyCtxt;

#[derive(Clone, Debug, PartialEq)]
struct EnumVariant {
    name: String,
    value: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct EnumDefinition {
    name: String,
    variants: Vec<EnumVariant>,
}

pub fn find_enum(tcx: TyCtxt<'_>) {
    todo!()
}
