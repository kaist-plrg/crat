use ::libc;
pub static mut gsl_version: *const libc::c_char = b"2.7.1\0" as *const u8
    as *const libc::c_char;
