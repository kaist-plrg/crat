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
pub mod cava;
pub mod cavacore;
pub mod config;
pub mod input {
pub mod alsa;
pub mod common;
pub mod fifo;
pub mod portaudio;
pub mod pulse;
pub mod shmem;
} // mod input
pub mod output {
pub mod noritake;
pub mod raw;
pub mod terminal_ncurses;
pub mod terminal_noncurses;
} // mod output
