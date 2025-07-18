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
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod src {
pub mod debug;
pub mod libpiano {
pub mod crypt;
pub mod list;
pub mod piano;
pub mod request;
pub mod response;
} // mod libpiano
pub mod main;
pub mod player;
pub mod settings;
pub mod terminal;
pub mod ui;
pub mod ui_act;
pub mod ui_dispatch;
pub mod ui_readline;
} // mod src
