#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod libltdl {
pub mod loaders {
pub mod dlopen;
pub mod preopen;
} // mod loaders
pub mod lt__alloc;
pub mod lt__strl;
pub mod lt_dlloader;
pub mod lt_error;
pub mod ltdl;
pub mod slist;
} // mod libltdl
