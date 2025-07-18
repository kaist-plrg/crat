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
#![feature(label_break_value)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod array;
pub mod awkgram;
pub mod awklib {
pub mod eg {
pub mod lib {
pub mod grcat;
pub mod pwcat;
} // mod lib
} // mod eg
} // mod awklib
pub mod builtin;
pub mod cint_array;
pub mod command;
pub mod debug;
pub mod eval;
pub mod ext;
pub mod extension {
pub mod filefuncs;
pub mod fnmatch;
pub mod fork;
pub mod gawkfts;
pub mod inplace;
pub mod intdiv;
pub mod ordchr;
pub mod readdir;
pub mod readdir_test;
pub mod readfile;
pub mod revoutput;
pub mod revtwoway;
pub mod rwarray;
pub mod stack;
pub mod testext;
pub mod time;
} // mod extension
pub mod field;
pub mod floatcomp;
pub mod gawkapi;
pub mod gawkmisc;
pub mod int_array;
pub mod io;
pub mod main;
pub mod mpfr;
pub mod msg;
pub mod node;
pub mod profile;
pub mod re;
pub mod replace;
pub mod str_array;
pub mod support {
pub mod dfa;
pub mod getopt;
pub mod getopt1;
pub mod localeinfo;
pub mod malloc {
pub mod dynarray_at_failure;
pub mod dynarray_emplace_enlarge;
pub mod dynarray_finalize;
pub mod dynarray_resize;
pub mod dynarray_resize_clear;
} // mod malloc
pub mod random;
pub mod regex;
} // mod support
pub mod symbol;
pub mod version;
