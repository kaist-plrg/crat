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
pub mod array;
pub mod auth;
pub mod base64;
pub mod browser;
pub mod cache;
pub mod cfg;
pub mod cookie;
pub mod cookies;
pub mod creds;
pub mod crew;
pub mod data;
pub mod date;
pub mod eval;
pub mod ftp;
pub mod getopt;
pub mod getopt1;
pub mod handler;
pub mod hash;
pub mod http;
pub mod init;
pub mod load;
pub mod log;
pub mod main;
pub mod md5;
pub mod memory;
pub mod notify;
pub mod page;
pub mod parser;
pub mod perl;
pub mod response;
pub mod sock;
pub mod ssl;
pub mod stralloc;
pub mod timer;
pub mod url;
pub mod util;
pub mod version;
} // mod src
