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
pub mod lib {
pub mod basename_lgpl;
pub mod dirname_lgpl;
pub mod getprogname;
pub mod localtime_buffer;
pub mod progname;
pub mod stripslash;
pub mod unistd;
pub mod version_etc;
pub mod version_etc_fsf;
} // mod lib
pub mod src {
pub mod resuse;
pub mod rusage_kb;
pub mod time;
} // mod src
