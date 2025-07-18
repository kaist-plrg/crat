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
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod src {
pub mod fifo;
pub mod format;
pub mod iof;
pub mod linebuffer;
pub mod longhelp;
pub mod numhash;
pub mod pexec;
pub mod r#str;
pub mod tokenize;
} // mod src
