#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(rustc_private)]


extern crate libc;
pub mod src {
pub mod backend;
pub mod backends {
pub mod carbon;
pub mod kafka;
} // mod backends
pub mod bloom;
pub mod city;
pub mod histogram;
pub mod ht;
pub mod http;
pub mod internal_sampler;
pub mod log;
pub mod metric;
pub mod sampler;
pub mod samplers {
pub mod statsd;
} // mod samplers
pub mod server;
pub mod setproctitle;
pub mod slab;
pub mod tags;
pub mod utils;
} // mod src
pub mod tests {
pub mod atomic;
pub mod ftoa;
pub mod histogram;
pub mod main;
pub mod mstore;
pub mod statsd_msg;
pub mod tags;
} // mod tests
