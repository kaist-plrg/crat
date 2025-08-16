use rustc_ast::{LitIntType, LitKind, UnOp};
use rustc_hir::{
    def::{DefKind, Res}, def_id::LocalDefId, Expr, ExprKind, Item, ItemKind, Node, Path, QPath, TyKind
};
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_span::{Span, source_map::Spanned};
use rustc_type_ir::{EarlyBinder, UintTy};

use crate::finder::enum_finder::{EnumDefinition, EnumVariant};

struct EnumDefChainItem {
    type_alias_def_id: LocalDefId,
    type_alias_span: Span,
    integer_const_bindings: Vec<(LocalDefId, Span, i32)>,
    chain_byte: u32,
}

pub(crate) fn find_enum_def(tcx: TyCtxt<'_>) -> Vec<EnumDefinition> {
    let free_item_ids = tcx.hir_free_items();
    let free_items: Vec<Item> = free_item_ids
        .map(|item_id| tcx.hir_item(item_id))
        .cloned()
        .collect();

    let type_alias_items = free_items.iter().filter(|item| match item.kind {
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
        ) => tcx.type_of(*def_id) == EarlyBinder::bind(Ty::new_uint(tcx, UintTy::U32)),
        _ => false,
    });

    let mut enum_def_chain = type_alias_items
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
                if tcx.type_of(*def_id) != EarlyBinder::bind(Ty::new_uint(tcx, UintTy::U32)) {
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
"#,
            |tcx| {
                let result = find_enum_def(tcx);
                assert_eq!(result.len(), 2);

                assert_eq!(result[0].variants[0].value, 1);
                assert_eq!(result[0].variants[1].value, 0);

                assert_eq!(result[1].variants[0].value, -1);
                assert_eq!(result[1].variants[1].value, 0);
            },
        )
        .unwrap()
    }
}
