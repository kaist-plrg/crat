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
pub mod deps {
pub mod theft {
pub mod theft;
pub mod theft_bloom;
pub mod theft_hash;
pub mod theft_mt;
} // mod theft
} // mod deps
pub mod src {
pub mod choices;
pub mod options;
pub mod r#match;
} // mod src
pub mod test {
pub mod fzytest;
pub mod test_choices;
pub mod test_match;
pub mod test_properties;
} // mod test
