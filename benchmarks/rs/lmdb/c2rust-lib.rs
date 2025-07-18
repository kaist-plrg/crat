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


extern crate libc;
pub mod libraries {
pub mod liblmdb {
pub mod mdb;
pub mod mdb_copy;
pub mod mdb_drop;
pub mod mdb_dump;
pub mod mdb_load;
pub mod mdb_stat;
pub mod midl;
pub mod mtest;
pub mod mtest2;
pub mod mtest3;
pub mod mtest4;
pub mod mtest5;
} // mod liblmdb
} // mod libraries
