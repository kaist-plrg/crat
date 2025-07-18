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
pub mod anonymous;
pub mod base64;
pub mod basicauth;
pub mod buffer;
pub mod child;
pub mod conf;
pub mod conf_tokens;
pub mod connect_ports;
pub mod conns;
pub mod daemon;
pub mod filter;
pub mod heap;
pub mod hostspec;
pub mod hsearch;
pub mod html_error;
pub mod http_message;
pub mod log;
pub mod main;
pub mod mypoll;
pub mod network;
pub mod orderedmap;
pub mod r#loop;
pub mod reqs;
pub mod reverse_proxy;
pub mod sblist;
pub mod sock;
pub mod stats;
pub mod text;
pub mod transparent_proxy;
pub mod upstream;
pub mod utils;
} // mod src
