#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod align;
pub mod bseq;
pub mod esterr;
pub mod format;
pub mod hit;
pub mod index;
pub mod kalloc;
pub mod ksw2_dispatch;
pub mod kthread;
pub mod lchain;
pub mod main;
pub mod map;
pub mod misc;
pub mod options;
pub mod pe;
pub mod sdust;
pub mod seed;
pub mod sketch;
pub mod splitidx;
