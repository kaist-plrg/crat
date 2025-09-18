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
    _variables: &PairIndexVec<IdentifierKey, IdentifierDetail<'_>>,
    _tcx: TyCtxt<'_>,
    _enum_tys: &[EnumTy],
    _enum_usages: &[EnumTyAnnotation<'_>],
) {
    let _assigns = basic_blocks
        .iter()
        .flat_map(|block_data| {
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

    // for assign in &assigns {
    //     let place_analyze = &assign.0;
    //     let rvalue = &assign.1;

    //     if let Some(var_detail) = variables.get(place_analyze.identifier) {
    //         println!(
    //             "BodyIndex: {:?}\tLocal: {:?}\tProjection: {:?}\tRvalue: {:?}\tSource Info: {:?}",
    //             place_analyze.identifier.0,
    //             place_analyze.identifier.1,
    //             place_analyze.projection,
    //             rvalue,
    //             var_detail.source_info.span
    //         );
    //     }
    // }

    let _returns = basic_blocks
        .iter()
        .filter_map(|block_data| {
            block_data.terminator.as_ref().filter(|terminator| {
                matches!(terminator.kind, rustc_middle::mir::TerminatorKind::Return)
            })
        })
        .collect::<Vec<_>>();

    // for ret in &returns {
    //     println!("BodyIndex: {body_idx:?}\tReturn Terminator: {ret:?}");
    // }

    let calls = basic_blocks
        .iter()
        .filter_map(|block_data| {
            block_data.terminator.as_ref().filter(|terminator| {
                matches!(
                    terminator.kind,
                    rustc_middle::mir::TerminatorKind::Call { .. }
                        | rustc_middle::mir::TerminatorKind::TailCall { .. }
                )
            })
        })
        .collect::<Vec<_>>();

    let mut tail_call_cnt = 0;
    for call in &calls {
        if matches!(
            call.kind,
            rustc_middle::mir::TerminatorKind::TailCall { .. }
        ) {
            tail_call_cnt += 1;
        }
        println!("BodyIndex: {body_idx:?}\tCall Terminator: {call:?}");
    }
    println!("Tail Call Count: {tail_call_cnt}"); // prints 0
}
