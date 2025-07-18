#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(rustc_private)]


extern crate libc;
pub mod examples {
pub mod camtest;
pub mod chunkview;
pub mod glpclview;
pub mod glview;
pub mod hiview;
pub mod micview;
pub mod regtest;
pub mod regview;
pub mod tiltdemo;
pub mod wavrecord;
} // mod examples
pub mod fakenect {
pub mod fakenect;
pub mod parson;
pub mod record;
} // mod fakenect
pub mod src {
pub mod audio;
pub mod cameras;
pub mod core;
pub mod flags;
pub mod loader;
pub mod registration;
pub mod tilt;
pub mod usb_libusb10;
} // mod src
pub mod wrappers {
pub mod c_sync {
pub mod libfreenect_sync;
} // mod c_sync
} // mod wrappers
