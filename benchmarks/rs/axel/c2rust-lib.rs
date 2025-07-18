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
pub mod lib {
pub mod strlcat;
pub mod strlcpy;
} // mod lib
pub mod src {
pub mod abuf;
pub mod axel;
pub mod conf;
pub mod conn;
pub mod ftp;
pub mod http;
pub mod search;
pub mod sleep;
pub mod tcp;
pub mod text;
} // mod src
