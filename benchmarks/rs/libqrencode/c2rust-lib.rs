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
pub mod bitstream;
pub mod mask;
pub mod mmask;
pub mod mqrspec;
pub mod qrenc;
pub mod qrencode;
pub mod qrinput;
pub mod qrspec;
pub mod rsecc;
pub mod split;
pub mod tests {
pub mod common;
pub mod datachunk;
pub mod decoder;
pub mod prof_qrencode;
pub mod pthread_qrencode;
pub mod rscode;
pub mod rsecc_decoder;
pub mod test_bitstream;
pub mod test_estimatebit;
pub mod test_mask;
pub mod test_mmask;
pub mod test_monkey;
pub mod test_mqrspec;
pub mod test_qrencode;
pub mod test_qrinput;
pub mod test_qrspec;
pub mod test_rs;
pub mod test_split;
pub mod test_split_urls;
} // mod tests
