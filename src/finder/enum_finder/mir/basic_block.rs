use rustc_middle::{
    mir::{BasicBlocks, PlaceElem, StatementKind},
    ty::{List, TyCtxt},
};

use crate::finder::enum_finder::{
    EnumTy, EnumTyAnnotation,
    mir::{BodyIndex, IdentifierDetail, IdentifierKey},
    utils::pair_index_vec::PairIndexVec,
};

#[derive(Debug)]
pub struct PlaceAnalyze<'tcx> {
    pub identifier: IdentifierKey,
    pub projection: &'tcx List<PlaceElem<'tcx>>,
}

pub fn process_basic_blocks(
    body_idx: BodyIndex,
    basic_blocks: &BasicBlocks<'_>,
    variables: &PairIndexVec<IdentifierKey, IdentifierDetail<'_>>,
    tcx: TyCtxt<'_>,
    enum_tys: &[EnumTy],
    enum_usages: &[EnumTyAnnotation<'_>],
) {
    let assigns = basic_blocks
        .iter_enumerated()
        .flat_map(|(i, block_data)| {
            block_data.statements.iter().filter_map(move |stmt| {
                if let StatementKind::Assign(box (place, rvalue)) = &stmt.kind {
                    Some((
                        PlaceAnalyze {
                            identifier: IdentifierKey(body_idx, place.local),
                            projection: place.projection,
                        },
                        rvalue,
                    ))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    for assign in &assigns {
        let place_analyze = &assign.0;
        let rvalue = &assign.1;

        if let Some(var_detail) = variables.get(place_analyze.identifier) {
            println!(
                "BodyIndex: {:?}\tLocal: {:?}\tProjection: {:?}\tRvalue: {:?}\tSource Info: {:?}",
                place_analyze.identifier.0,
                place_analyze.identifier.1,
                place_analyze.projection,
                rvalue,
                var_detail.source_info.span
            );
        }
    }
}
