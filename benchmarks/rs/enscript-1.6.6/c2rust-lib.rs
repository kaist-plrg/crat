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

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod afmlib {
pub mod afm;
pub mod afmparse;
pub mod afmtest;
pub mod deffont;
pub mod e_88591;
pub mod e_885910;
pub mod e_88592;
pub mod e_88593;
pub mod e_88594;
pub mod e_88595;
pub mod e_88597;
pub mod e_88599;
pub mod e_hp8;
pub mod e_koi8;
pub mod e_mac;
pub mod e_pc;
pub mod e_vms;
pub mod strhash;
} // mod afmlib
pub mod compat {
pub mod getopt;
pub mod getopt1;
pub mod regex;
pub mod xalloc;
} // mod compat
pub mod src {
pub mod main;
pub mod mkafmmap;
pub mod prt_lpr;
pub mod psgen;
pub mod util;
} // mod src
pub mod states {
pub mod gram;
pub mod lex;
pub mod main;
pub mod prims;
pub mod process;
pub mod utils;
} // mod states
