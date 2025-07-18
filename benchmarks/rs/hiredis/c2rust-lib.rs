#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(rustc_private)]


extern crate libc;
pub mod alloc;
pub mod hiredis;
pub mod net;
pub mod r#async;
pub mod read;
pub mod sds;
pub mod sockcompat;
pub mod test;
