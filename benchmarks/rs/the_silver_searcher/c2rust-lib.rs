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
#![feature(linkage)]
#![feature(rustc_private)]
#![feature(thread_local)]


extern crate libc;
pub mod src {
pub mod decompress;
pub mod ignore;
pub mod lang;
pub mod log;
pub mod main;
pub mod options;
pub mod print;
pub mod print_w32;
pub mod scandir;
pub mod search;
pub mod util;
pub mod zfile;
} // mod src
