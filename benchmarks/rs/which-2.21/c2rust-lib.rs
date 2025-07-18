#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(rustc_private)]


extern crate libc;
pub mod bash;
pub mod getopt;
pub mod getopt1;
pub mod tilde {
pub mod shell;
pub mod tilde;
} // mod tilde
pub mod which;
