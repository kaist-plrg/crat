#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod acl;
pub mod b64 {
pub mod cencode;
} // mod b64
pub mod client;
pub mod cmd;
pub mod conf;
pub mod formats {
pub mod common;
pub mod custom_type;
pub mod json;
pub mod raw;
} // mod formats
pub mod hiredis {
pub mod alloc;
pub mod dict;
pub mod hiredis;
pub mod net;
pub mod r#async;
pub mod read;
pub mod sds;
pub mod sockcompat;
} // mod hiredis
pub mod http;
pub mod http_parser {
pub mod http_parser;
} // mod http_parser
pub mod jansson {
pub mod src {
pub mod dump;
pub mod error;
pub mod hashtable;
pub mod load;
pub mod strbuffer;
pub mod utf;
pub mod value;
pub mod variadic;
} // mod src
} // mod jansson
pub mod md5 {
pub mod md5;
} // mod md5
pub mod pool;
pub mod server;
pub mod sha1 {
pub mod sha1;
} // mod sha1
pub mod slog;
pub mod webdis;
pub mod websocket;
pub mod worker;
} // mod src
