#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(if_let_guard)]
#![feature(iter_intersperse)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_literal_escaper;
extern crate rustc_middle;
extern crate rustc_mir_build;
extern crate rustc_mir_dataflow;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_type_ir;
extern crate smallvec;
extern crate thin_vec;

pub mod preprocessor;
#[macro_use]
pub mod ast_util;
pub mod bin_file_adder;
pub mod bit_set;
pub mod check_unsafety;
pub mod compile_util;
pub mod disjoint_set;
pub mod equiv_classes;
pub mod extern_resolver;
pub mod finder;
pub mod formatter;
pub mod graph_util;
pub mod io_replacer;
pub mod ir_util;
pub mod points_to;
pub mod ty_shape;
pub mod type_checker;
pub mod union_replacer;
pub mod unsafe_resolver;

#[inline(never)]
pub fn example_function(x: u32) -> (u32, i32) {
    if x > 10 { (x * 2, -1) } else { (x + 5, 1) }
}

#[inline(never)]
pub fn example_function2(x: u32) {
    let y = example_function(x);
    y.0.checked_add(u32::MAX).unwrap();
}
