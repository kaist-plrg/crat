pub mod basic_block;

use std::collections::HashSet;

use rustc_middle::{
    mir::{Body, Local, SourceInfo},
    ty::{Ty, TyCtxt},
};
use rustc_span::Span;
use toml::de;

use crate::finder::enum_finder::{
    EnumTy, EnumTyAnnotation,
    utils::pair_index_vec::{PairIndex, PairIndexVec},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BodyIndex(usize);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct IdentifierKey(BodyIndex, Local);

impl PairIndex for IdentifierKey {
    fn pair_index(&self) -> (usize, usize) {
        (self.0.0, self.1.as_usize())
    }

    fn from_pair_index(pair: (usize, usize)) -> Self {
        Self(BodyIndex(pair.0), Local::from_usize(pair.1))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct IdentifierDetail<'mir> {
    key: IdentifierKey,
    kind: IdentifierKind,
    ty: Ty<'mir>,
    source_info: SourceInfo,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum IdentifierKind {
    Local,
    Return,
    Argument,
}

pub(crate) fn get_optimized_mirs(tcx: TyCtxt<'_>) -> Vec<Body<'_>> {
    tcx.hir_body_owners()
        .map(|def_id| {
            if tcx.def_kind(def_id).is_fn_like() {
                tcx.optimized_mir(def_id)
            } else {
                tcx.mir_for_ctfe(def_id)
            }
        })
        .cloned()
        .collect()
}

pub(crate) fn process_mirs(
    bodies: &[Body<'_>],
    tcx: TyCtxt<'_>,
    enum_tys: &[EnumTy],
    enum_usages: &[EnumTyAnnotation<'_>],
) {
    let variables = bodies
        .iter()
        .enumerate()
        .flat_map(|(i, body)| {
            body.local_decls
                .iter_enumerated()
                .map(move |(local, decl)| {
                    let key = IdentifierKey(BodyIndex(i), local);
                    let detail = IdentifierDetail {
                        key,
                        kind: match local.as_usize() {
                            0 => IdentifierKind::Return,
                            n if n <= body.arg_count => IdentifierKind::Argument,
                            _ => IdentifierKind::Local,
                        },
                        ty: decl.ty,
                        source_info: decl.source_info,
                    };
                    (key, detail)
                })
        })
        .collect::<PairIndexVec<_, _>>();

    bodies.iter().enumerate().for_each(|(i, body)| {
        basic_block::process_basic_blocks(
            BodyIndex(i),
            &body.basic_blocks,
            &variables,
            tcx,
            enum_tys,
            enum_usages,
        );
    });

    let enum_locals = variables
        .iter()
        .filter_map(|(key, detail)| {
            enum_usages.iter().find_map(|enum_usage| {
                if let EnumTyAnnotation::Let(_, span, ty) = enum_usage
                    && detail.source_info.span == *span
                {
                    Some((key, (detail, *ty)))
                } else {
                    None
                }
            })
        })
        .collect::<PairIndexVec<_, _>>();

    let mut arg_spans = HashSet::new();
    let mut recent_fn_span = None;
    let mut recent_fn_arg_count = -1;

    let enum_params = variables
        .iter()
        .filter_map(|(key, detail)| {
            enum_usages.iter().find_map(|enum_usage| {
                if let EnumTyAnnotation::Fn(_, _, span, arg_tys, _) = enum_usage
                    && span.lo().0 <= detail.source_info.span.lo().0
                    && detail.source_info.span.hi().0 <= span.hi().0
                    && (detail.kind == IdentifierKind::Argument
                        || detail.kind == IdentifierKind::Local)
                {
                    if recent_fn_span.map(|span: Span| span.lo().0) == Some(span.lo().0) {
                        if !arg_spans.insert(detail.source_info.span.lo().0) {
                            return None;
                        }
                        recent_fn_arg_count += 1;
                    } else {
                        arg_spans.clear();
                        arg_spans.insert(detail.source_info.span.lo().0);
                        recent_fn_span = Some(*span);
                        recent_fn_arg_count = 0;
                    }
                    if arg_tys[recent_fn_arg_count as usize].is_some() {
                        Some((
                            key,
                            (detail, arg_tys[recent_fn_arg_count as usize].unwrap()),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect::<PairIndexVec<_, _>>();

    for (_, (detail, _)) in enum_params.iter() {
        dbg!(detail.source_info.span);
    }
}
