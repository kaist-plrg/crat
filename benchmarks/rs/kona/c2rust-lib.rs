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
extern crate c2rust_asm_casts;
extern crate libc;
pub mod src {
pub mod zero;
pub mod bswap;
pub mod c;
pub mod getline;
pub mod k;
pub mod kc;
pub mod kg;
pub mod km;
pub mod kn;
pub mod ko;
pub mod ks;
pub mod kx;
pub mod main;
pub mod mt;
pub mod p;
pub mod r;
pub mod tests;
pub mod v;
pub mod va;
pub mod vc;
pub mod vd;
pub mod vf;
pub mod vg;
pub mod vq;
} // mod src
