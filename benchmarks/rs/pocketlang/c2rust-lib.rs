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
pub mod cli {
pub mod main;
} // mod cli
pub mod src {
pub mod core {
pub mod compiler;
pub mod core;
pub mod debug;
pub mod public;
pub mod utils;
pub mod value;
pub mod vm;
} // mod core
pub mod libs {
pub mod libs;
pub mod std_dummy;
pub mod std_io;
pub mod std_json;
pub mod std_math;
pub mod std_os;
pub mod std_path;
pub mod std_term;
pub mod std_time;
pub mod std_types;
} // mod libs
} // mod src
