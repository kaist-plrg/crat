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


extern crate libc;
pub mod bits;
pub mod deflate;
pub mod gzip;
pub mod inflate;
pub mod lib {
pub mod asnprintf;
pub mod basename_lgpl;
pub mod chdir_long;
pub mod cloexec;
pub mod creat_safer;
pub mod dirname_lgpl;
pub mod dup_safer;
pub mod dup_safer_flag;
pub mod exitfail;
pub mod fclose;
pub mod fcntl;
pub mod fd_hook;
pub mod fd_safer;
pub mod fd_safer_flag;
pub mod fflush;
pub mod filenamecat_lgpl;
pub mod fprintf;
pub mod fpurge;
pub mod freading;
pub mod free;
pub mod fseek;
pub mod fseeko;
pub mod fseterr;
pub mod getprogname;
pub mod gettime;
pub mod glthread {
pub mod lock;
pub mod threadlib;
} // mod glthread
pub mod ialloc;
pub mod malloca;
pub mod math;
pub mod open_safer;
pub mod openat_die;
pub mod openat_proc;
pub mod openat_safer;
pub mod opendir_safer;
pub mod pipe_safer;
pub mod printf;
pub mod printf_args;
pub mod printf_frexp;
pub mod printf_frexpl;
pub mod printf_parse;
pub mod save_cwd;
pub mod savedir;
pub mod sig_handler;
pub mod stat_time;
pub mod strerror_r;
pub mod stripslash;
pub mod timespec;
pub mod unistd;
pub mod utimens;
pub mod vasnprintf;
pub mod vfprintf;
pub mod xalloc_die;
pub mod xmalloc;
pub mod xsize;
pub mod yesno;
} // mod lib
pub mod trees;
pub mod unlzh;
pub mod unlzw;
pub mod unpack;
pub mod unzip;
pub mod util;
pub mod version;
pub mod zip;
