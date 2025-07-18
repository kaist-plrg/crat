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

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod arp;
pub mod cli;
pub mod dst;
pub mod icmpv4;
pub mod inet;
pub mod ip_input;
pub mod ip_output;
pub mod ipc;
pub mod main;
pub mod netdev;
pub mod route;
pub mod skbuff;
pub mod sock;
pub mod socket;
pub mod tcp;
pub mod tcp_data;
pub mod tcp_input;
pub mod tcp_output;
pub mod timer;
pub mod tuntap_if;
pub mod utils;
} // mod src
