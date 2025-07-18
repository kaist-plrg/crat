#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(rustc_private)]


extern crate libc;
pub mod tests {
pub mod src {
pub mod api;
pub mod conc_increment;
pub mod no_work;
pub mod nonzero_heap_stack;
pub mod pause_resume;
pub mod wait;
} // mod src
} // mod tests
pub mod thpool;
