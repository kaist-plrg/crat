use rustc_middle::ty::TyCtxt;

use crate::finder::enum_finder::EnumDefinition;

pub(super) fn find_enum_def(tcx: TyCtxt<'_>) -> Vec<EnumDefinition> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{compile_util, finder::enum_finder::EnumVariant};

    #[test]
    fn test_enum_definitions() {
        compile_util::run_compiler_on_str(
            r#"
pub type BrotliDecoderParameter = libc::c_uint;
pub const BROTLI_DECODER_PARAM_LARGE_WINDOW: BrotliDecoderParameter = 1;
pub const BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: BrotliDecoderParameter = 0;

fn f() -> i32 { 0 }
fn g(x: i32) -> i32 { x }
fn h(x: i32) -> i32 { x + x }

pub type json_parse_flags_e = libc::c_uint;
pub const json_parse_flags_allow_trailing_comma: json_parse_flags_e = 1;
pub const json_parse_flags_default: json_parse_flags_e = 0;
"#,
            |tcx| {
                let enum_defs = find_enum_def(tcx);
                assert_eq!(
                    enum_defs,
                    vec![
                        EnumDefinition {
                            name: String::from("BrotliDecoderParameter"),
                            variants: vec![
                                EnumVariant {
                                    name: String::from("BROTLI_DECODER_PARAM_LARGE_WINDOW"),
                                    value: 1
                                },
                                EnumVariant {
                                    name: String::from(
                                        "BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION"
                                    ),
                                    value: 0
                                }
                            ]
                        },
                        EnumDefinition {
                            name: String::from("json_parse_flags_e"),
                            variants: vec![
                                EnumVariant {
                                    name: String::from("json_parse_flags_allow_trailing_comma"),
                                    value: 1
                                },
                                EnumVariant {
                                    name: String::from("json_parse_flags_default"),
                                    value: 0
                                }
                            ]
                        }
                    ]
                )
            },
        )
        .unwrap()
    }
}
