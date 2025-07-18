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
pub mod bc {
pub mod bc;
pub mod execute;
pub mod global;
pub mod load;
pub mod main;
pub mod scan;
pub mod storage;
pub mod util;
pub mod warranty;
} // mod bc
pub mod dc {
pub mod array;
pub mod dc;
pub mod eval;
pub mod misc;
pub mod numeric;
pub mod stack;
pub mod string;
} // mod dc
pub mod lib {
pub mod getopt;
pub mod getopt1;
pub mod number;
pub mod vfprintf;
} // mod lib
