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
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate f128;#[macro_use]
extern crate num_traits;
extern crate libc;
pub mod obj {
pub mod bytecode;
} // mod obj
pub mod src {
pub mod ae;
pub mod aprintf;
pub mod http_parser;
pub mod net;
pub mod script;
pub mod ssl;
pub mod stats;
pub mod units;
pub mod version;
pub mod wrk;
pub mod zmalloc;
} // mod src
