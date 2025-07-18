#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod src {
pub mod allocator_thread;
pub mod common;
pub mod core;
pub mod debug;
pub mod hash;
pub mod hostsreader;
pub mod libproxychains;
pub mod rdns;
pub mod version;
} // mod src
