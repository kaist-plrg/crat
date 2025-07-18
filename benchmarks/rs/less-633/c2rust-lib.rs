#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod brac;
pub mod ch;
pub mod charset;
pub mod cmdbuf;
pub mod command;
pub mod cvt;
pub mod decode;
pub mod edit;
pub mod filename;
pub mod forwback;
pub mod help;
pub mod ifile;
pub mod input;
pub mod jump;
pub mod lessecho;
pub mod lesskey;
pub mod lesskey_parse;
pub mod line;
pub mod linenum;
pub mod lsystem;
pub mod main;
pub mod mark;
pub mod optfunc;
pub mod option;
pub mod opttbl;
pub mod os;
pub mod output;
pub mod pattern;
pub mod position;
pub mod prompt;
pub mod screen;
pub mod search;
pub mod signal;
pub mod tags;
pub mod ttyin;
pub mod version;
pub mod xbuf;
