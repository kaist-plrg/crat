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
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod lib {
pub mod concat_filename;
pub mod findprog_in;
} // mod lib
pub mod src {
pub mod ar;
pub mod arscan;
pub mod commands;
pub mod default;
pub mod dir;
pub mod expand;
pub mod file;
pub mod function;
pub mod getopt;
pub mod getopt1;
pub mod guile;
pub mod hash;
pub mod implicit;
pub mod job;
pub mod load;
pub mod loadapi;
pub mod main;
pub mod misc;
pub mod output;
pub mod posixos;
pub mod read;
pub mod remake;
pub mod remote_stub;
pub mod rule;
pub mod shuffle;
pub mod signame;
pub mod strcache;
pub mod variable;
pub mod version;
pub mod vpath;
} // mod src
