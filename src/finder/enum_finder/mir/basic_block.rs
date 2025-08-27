use std::collections::HashMap;

use rustc_middle::{mir::BasicBlocks, ty::TyCtxt};

use crate::finder::enum_finder::{
    EnumTy, EnumTyAnnotation,
    mir::{BodyIndex, IdentifierDetail, IdentifierKey},
};

pub fn process_basic_blocks(
    body_idx: BodyIndex,
    basic_blocks: &BasicBlocks<'_>,
    local_decls: &HashMap<IdentifierKey, IdentifierDetail<'_>>,
    tcx: TyCtxt<'_>,
    enum_tys: &[EnumTy],
    enum_usages: &[EnumTyAnnotation<'_>],
) {
}
