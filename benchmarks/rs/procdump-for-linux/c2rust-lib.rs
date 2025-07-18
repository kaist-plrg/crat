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
pub mod src {
pub mod CoreDumpWriter;
pub mod Events;
pub mod Handle;
pub mod Logging;
pub mod ProcDumpConfiguration;
pub mod Procdump;
pub mod Process;
pub mod TriggerThreadProcs;
} // mod src
