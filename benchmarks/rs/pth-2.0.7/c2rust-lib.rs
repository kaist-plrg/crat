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
extern crate f128;#[macro_use]
extern crate num_traits;
extern crate libc;
pub mod pth_attr;
pub mod pth_cancel;
pub mod pth_clean;
pub mod pth_compat;
pub mod pth_data;
pub mod pth_debug;
pub mod pth_errno;
pub mod pth_event;
pub mod pth_ext;
pub mod pth_fork;
pub mod pth_high;
pub mod pth_lib;
pub mod pth_mctx;
pub mod pth_msg;
pub mod pth_pqueue;
pub mod pth_ring;
pub mod pth_sched;
pub mod pth_string;
pub mod pth_sync;
pub mod pth_syscall;
pub mod pth_tcb;
pub mod pth_time;
pub mod pth_uctx;
pub mod pth_util;
pub mod test_common;
pub mod test_httpd;
pub mod test_misc;
pub mod test_mp;
pub mod test_philo;
pub mod test_select;
pub mod test_sfio;
pub mod test_sig;
pub mod test_std;
pub mod test_uctx;
