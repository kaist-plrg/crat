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


extern crate libc;
pub mod src {
pub mod author;
pub mod control;
pub mod datas;
pub mod dns;
pub mod event;
pub mod init;
pub mod io;
pub mod memory;
pub mod net;
pub mod storage;
pub mod update;
pub mod utils;
} // mod src
