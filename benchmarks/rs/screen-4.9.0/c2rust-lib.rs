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
pub mod acls;
pub mod ansi;
pub mod attacher;
pub mod braille;
pub mod braille_tsi;
pub mod canvas;
pub mod comm;
pub mod display;
pub mod encoding;
pub mod fileio;
pub mod help;
pub mod input;
pub mod kmapdef;
pub mod layer;
pub mod layout;
pub mod list_display;
pub mod list_generic;
pub mod list_window;
pub mod loadav;
pub mod logfile;
pub mod mark;
pub mod misc;
pub mod nethack;
pub mod process;
pub mod pty;
pub mod putenv;
pub mod resize;
pub mod sched;
pub mod screen;
pub mod search;
pub mod socket;
pub mod teln;
pub mod term;
pub mod termcap;
pub mod tty;
pub mod utmp;
pub mod viewport;
pub mod window;
