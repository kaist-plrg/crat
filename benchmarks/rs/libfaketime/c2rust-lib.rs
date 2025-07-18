#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(rustc_private)]
#![feature(thread_local)]


extern crate libc;
pub mod src {
pub mod faketime;
pub mod libfaketime;
} // mod src
