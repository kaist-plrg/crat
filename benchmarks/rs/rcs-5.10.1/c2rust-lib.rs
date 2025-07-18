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
#![feature(linkage)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod lib {
pub mod asnprintf;
pub mod basename_lgpl;
pub mod concat_filename;
pub mod dup_safer;
pub mod exitfail;
pub mod fcntl;
pub mod fd_safer;
pub mod findprog;
pub mod free;
pub mod getprogname;
pub mod gettime;
pub mod hash_pjw;
pub mod ialloc;
pub mod math;
pub mod obstack;
pub mod pipe_safer;
pub mod printf_args;
pub mod printf_frexp;
pub mod printf_frexpl;
pub mod printf_parse;
pub mod progname;
pub mod sig_handler;
pub mod sprintf;
pub mod stat_time;
pub mod timespec;
pub mod unistd;
pub mod utimens;
pub mod vasnprintf;
pub mod xalloc_die;
pub mod xconcat_filename;
pub mod xmalloc;
pub mod xsize;
} // mod lib
pub mod src {
pub mod b_anchor;
pub mod b_complain;
pub mod b_divvy;
pub mod b_esds;
pub mod b_excwho;
pub mod b_fb;
pub mod b_feph;
pub mod b_fro;
pub mod b_grok;
pub mod b_isr;
pub mod b_kwxout;
pub mod b_peer;
pub mod ci;
pub mod co;
pub mod gnu_h_v;
pub mod ident;
pub mod maketime;
pub mod merge;
pub mod merger;
pub mod partime;
pub mod supr;
pub mod rcs;
pub mod rcsclean;
pub mod rcsdiff;
pub mod rcsedit;
pub mod rcsfcmp;
pub mod rcsfnms;
pub mod rcsgen;
pub mod rcskeep;
pub mod rcsmap;
pub mod rcsmerge;
pub mod rcsrev;
pub mod rcstime;
pub mod rcsutil;
pub mod rlog;
} // mod src
