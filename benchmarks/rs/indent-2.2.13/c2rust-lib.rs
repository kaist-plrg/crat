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


extern crate libc;
pub mod man {
pub mod texinfo2man;
} // mod man
pub mod src {
pub mod args;
pub mod backup;
pub mod code_io;
pub mod comments;
pub mod globs;
pub mod handletoken;
pub mod indent;
pub mod lexi;
pub mod output;
pub mod parse;
pub mod utils;
} // mod src
