#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]


extern crate libc;
pub mod pigz;
pub mod r#try;
pub mod yarn;
pub mod zopfli {
pub mod src {
pub mod zopfli {
pub mod blocksplitter;
pub mod cache;
pub mod deflate;
pub mod hash;
pub mod katajainen;
pub mod lz77;
pub mod squeeze;
pub mod symbols;
pub mod tree;
pub mod util;
} // mod zopfli
} // mod src
} // mod zopfli
