#![feature(rustc_private)]
#![feature(box_patterns)]

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
extern crate rustc_middle;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_type_ir;
extern crate smallvec;
extern crate thin_vec;

#[macro_use]
pub mod ast_util;
pub mod compile_util;
pub mod disjoint_set;
pub mod equiv_classes;
pub mod extern_resolver;
pub mod graph_util;
pub mod ir_util;
pub mod type_checker;
pub mod unsafe_resolver;
