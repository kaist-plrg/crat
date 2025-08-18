use std::collections::HashMap;

use rustc_ast::{LitIntType, LitKind, UnOp};
use rustc_hir::{
    Expr, ExprKind, Item, ItemKind, Node, Path, QPath, Ty, TyKind,
    def::{DefKind, Res},
    def_id::LocalDefId,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::{Span, source_map::Spanned};
use rustc_type_ir::{EarlyBinder, UintTy};

use crate::finder::enum_finder::{EnumDefinition, EnumTy, EnumVariant};

struct EnumDefChainItem {
    type_alias_def_id: LocalDefId,
    type_alias_span: Span,
    integer_const_bindings: Vec<(LocalDefId, Span, i32)>,
    chain_byte: u32,
}

pub(crate) fn find_enum_tys<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<EnumTy> {
    let free_items: &'tcx [Item<'tcx>] = find_free_items(tcx);
    let type_alias_items: &'tcx [Item<'tcx>] = find_u32_ty_alias_items(tcx, free_items);
    let enum_defs: Vec<EnumDefinition> = find_enum_def(tcx, type_alias_items, free_items);
    let points_to_defs: Vec<EnumTy> = find_points_to(&enum_defs, free_items);

    let mut enum_tys = Vec::with_capacity(enum_defs.len() + points_to_defs.len());
    enum_tys.extend(enum_defs.into_iter().map(EnumTy::Definition));
    enum_tys.extend(points_to_defs);

    enum_tys
}

fn find_points_to(enum_defs: &[EnumDefinition], free_items: &[Item]) -> Vec<EnumTy> {
    let id_to_def_map = enum_defs
        .iter()
        .map(|def| (def.def_id, def))
        .collect::<HashMap<_, _>>();

    free_items
        .iter()
        .flat_map(|item| match item.kind {
            ItemKind::TyAlias(
                _,
                _,
                Ty {
                    kind:
                        TyKind::Path(QPath::Resolved(
                            None,
                            Path {
                                res: Res::Def(DefKind::TyAlias, def_id),
                                ..
                            },
                        )),
                    ..
                },
            ) => def_id.as_local().and_then(|def_id| {
                id_to_def_map
                    .get(&def_id)
                    .map(|enum_def| EnumTy::PointsTo(def_id, item.span, (*enum_def).clone()))
            }),
            _ => None,
        })
        .collect()
}

fn find_enum_def<'tcx>(
    tcx: TyCtxt<'tcx>,
    type_alias_items: &'tcx [Item<'tcx>],
    free_items: &'tcx [Item<'tcx>],
) -> Vec<EnumDefinition> {
    let mut enum_def_chain = type_alias_items
        .iter()
        .map(|item| EnumDefChainItem {
            type_alias_def_id: item.owner_id.def_id,
            type_alias_span: item.span,
            integer_const_bindings: Vec::new(),
            chain_byte: item.span.hi().0 + 1,
        })
        .collect::<Vec<_>>();

    enum_def_chain.sort_by_key(|chain_item| chain_item.chain_byte);

    let integer_const_binding_items = free_items
        .iter()
        .filter_map(|item| match item.kind {
            ItemKind::Const(
                _,
                _,
                rustc_hir::Ty {
                    hir_id: _,
                    kind:
                        TyKind::Path(QPath::Resolved(
                            None,
                            Path {
                                span: _,
                                res: Res::Def(DefKind::TyAlias, def_id),
                                segments: _,
                            },
                        )),
                    span: _,
                },
                body_id,
            ) => {
                if tcx.type_of(*def_id)
                    != EarlyBinder::bind(rustc_middle::ty::Ty::new_uint(tcx, UintTy::U32))
                {
                    return None;
                }

                if let Node::Expr(expr) = tcx.hir_node(body_id.hir_id) {
                    let val = match &expr.kind {
                        ExprKind::Lit(Spanned {
                            node: LitKind::Int(pu128, LitIntType::Unsuffixed),
                            ..
                        }) => pu128.0 as i32,

                        ExprKind::Unary(
                            UnOp::Neg,
                            Expr {
                                kind:
                                    ExprKind::Lit(Spanned {
                                        node: LitKind::Int(pu128, LitIntType::Unsuffixed),
                                        ..
                                    }),
                                ..
                            },
                        ) => -(pu128.0 as i32),

                        _ => return None,
                    };
                    Some((item, val))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut last_found_index = 0usize;
    for (item, value) in integer_const_binding_items {
        while last_found_index < enum_def_chain.len()
            && enum_def_chain[last_found_index].chain_byte < item.span.lo().0
        {
            last_found_index += 1;
        }

        if last_found_index == enum_def_chain.len() {
            break;
        }

        if enum_def_chain[last_found_index].chain_byte == item.span.lo().0 {
            enum_def_chain[last_found_index].chain_byte = item.span.hi().0 + 1;
            enum_def_chain[last_found_index]
                .integer_const_bindings
                .push((item.owner_id.def_id, item.span, value));
        }
    }

    enum_def_chain
        .iter()
        .filter(|chain_item| !chain_item.integer_const_bindings.is_empty())
        .map(|chain_item| EnumDefinition {
            def_id: chain_item.type_alias_def_id,
            span: chain_item.type_alias_span,
            variants: chain_item
                .integer_const_bindings
                .iter()
                .map(|(def_id, span, value)| EnumVariant {
                    def_id: *def_id,
                    span: *span,
                    value: *value,
                })
                .collect(),
        })
        .collect()
}

fn find_free_items<'tcx>(tcx: TyCtxt<'tcx>) -> &'tcx [Item<'tcx>] {
    let free_item_ids = tcx.hir_free_items();
    let free_items = free_item_ids.map(|item_id| tcx.hir_item(item_id)).cloned();
    tcx.arena.alloc_from_iter(free_items)
}

fn find_u32_ty_alias_items<'tcx>(
    tcx: TyCtxt<'tcx>,
    free_items: &'tcx [Item<'tcx>],
) -> &'tcx [Item<'tcx>] {
    let ty_alias_items = free_items
        .iter()
        .filter(move |item| match item.kind {
            ItemKind::TyAlias(
                _,
                _,
                rustc_hir::Ty {
                    hir_id: _,
                    kind:
                        TyKind::Path(QPath::Resolved(
                            None,
                            Path {
                                span: _,
                                res: Res::Def(DefKind::TyAlias, def_id),
                                segments: _,
                            },
                        )),
                    span: _,
                },
            ) => {
                tcx.type_of(*def_id)
                    == EarlyBinder::bind(rustc_middle::ty::Ty::new_uint(tcx, UintTy::U32))
            }
            _ => false,
        })
        .cloned();
    tcx.arena.alloc_from_iter(ty_alias_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile_util;

    #[test]
    fn test_enum_definitions() {
        compile_util::run_compiler_on_str(
            r#"
mod libc {
    pub type c_uint = u32;
}

pub type BrotliDecoderParameter = libc::c_uint;
pub const BROTLI_DECODER_PARAM_LARGE_WINDOW: BrotliDecoderParameter = 1;
pub const BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: BrotliDecoderParameter = 0;

fn f() -> i32 { 0 }
fn g(x: i32) -> i32 { x }
fn h(x: i32) -> i32 { x + x }

pub type json_parse_flags_e = libc::c_uint;
pub const json_parse_flags_allow_trailing_comma: json_parse_flags_e = -1;
pub const json_parse_flags_default: json_parse_flags_e = 0;
pub type json_parse_flags = json_parse_flags_e;
"#,
            |tcx| {
                let result = find_enum_tys(tcx);
                assert_eq!(result.len(), 3);

                assert!(matches!(result[0], EnumTy::Definition(_)));
                assert!(matches!(result[1], EnumTy::Definition(_)));
                assert!(matches!(result[2], EnumTy::PointsTo(_, _, _)));

                if let EnumTy::Definition(def) = &result[0] {
                    assert_eq!(def.variants.len(), 2);
                    assert_eq!(def.variants[0].value, 1);
                    assert_eq!(def.variants[1].value, 0);
                } else {
                    unreachable!();
                }

                if let EnumTy::Definition(def) = &result[1] {
                    assert_eq!(def.variants.len(), 2);
                    assert_eq!(def.variants[0].value, -1);
                    assert_eq!(def.variants[1].value, 0);
                } else {
                    unreachable!();
                }

                assert_eq!(result[1].get_def_id(), result[2].get_def_id());
            },
        )
        .unwrap()
    }
}
