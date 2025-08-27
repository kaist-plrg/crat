use rustc_middle::mir::SourceInfo;

use crate::finder::enum_finder::EnumDefinition;

pub struct Var<'abs> {
    source_info: SourceInfo,
    ty: Type<'abs>,
}

#[derive(Clone, Debug)]
pub enum Type<'abs> {
    Enum(EnumDefinition),
    Int,
    RawPtr(&'abs Type<'abs>),
    // ...
    // TODO
}
