#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]


extern crate libc;
pub mod src {
pub mod args;
pub mod array;
pub mod core;
pub mod csv;
pub mod env;
pub mod exec;
pub mod graph;
pub mod init;
pub mod io;
pub mod iter;
pub mod kvs;
pub mod latch;
pub mod main;
pub mod math;
pub mod ncpu;
pub mod node;
pub mod ns;
pub mod number;
pub mod pollfd;
pub mod queue;
pub mod random;
pub mod signal;
pub mod socket;
pub mod sort;
pub mod stat;
pub mod string;
pub mod strptime;
pub mod time;
pub mod value;
pub mod y_tab;
} // mod src
