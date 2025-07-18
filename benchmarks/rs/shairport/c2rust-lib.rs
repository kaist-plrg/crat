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
pub mod alac;
pub mod audio;
pub mod audio_alsa;
pub mod audio_ao;
pub mod audio_dummy;
pub mod audio_pipe;
pub mod audio_pulse;
pub mod common;
pub mod daemon;
pub mod mdns;
pub mod mdns_external;
pub mod mdns_tinysvcmdns;
pub mod metadata;
pub mod player;
pub mod rtp;
pub mod rtsp;
pub mod shairport;
pub mod tinysvcmdns;
