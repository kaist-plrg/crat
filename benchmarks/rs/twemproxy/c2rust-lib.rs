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
pub mod contrib {
pub mod yaml_0_2_5 {
pub mod src {
pub mod api;
pub mod dumper;
pub mod emitter;
pub mod loader;
pub mod parser;
pub mod reader;
pub mod scanner;
pub mod writer;
} // mod src
pub mod tests {
pub mod example_deconstructor;
pub mod example_deconstructor_alt;
pub mod example_reformatter;
pub mod example_reformatter_alt;
pub mod run_dumper;
pub mod run_emitter;
pub mod run_emitter_test_suite;
pub mod run_loader;
pub mod run_parser;
pub mod run_parser_test_suite;
pub mod run_scanner;
} // mod tests
} // mod yaml_0_2_5
} // mod contrib
pub mod src {
pub mod event {
pub mod nc_epoll;
pub mod nc_evport;
pub mod nc_kqueue;
} // mod event
pub mod hashkit {
pub mod nc_crc16;
pub mod nc_crc32;
pub mod nc_fnv;
pub mod nc_hsieh;
pub mod nc_jenkins;
pub mod nc_ketama;
pub mod nc_md5;
pub mod nc_modula;
pub mod nc_murmur;
pub mod nc_one_at_a_time;
pub mod nc_random;
} // mod hashkit
pub mod nc;
pub mod nc_array;
pub mod nc_client;
pub mod nc_conf;
pub mod nc_connection;
pub mod nc_core;
pub mod nc_log;
pub mod nc_mbuf;
pub mod nc_message;
pub mod nc_proxy;
pub mod nc_rbtree;
pub mod nc_request;
pub mod nc_response;
pub mod nc_server;
pub mod nc_signal;
pub mod nc_stats;
pub mod nc_string;
pub mod nc_util;
pub mod proto {
pub mod nc_memcache;
pub mod nc_redis;
} // mod proto
pub mod test_all;
} // mod src
