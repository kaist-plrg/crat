use ::libc;
pub static mut Version: *const libc::c_char = b"4.9\0" as *const u8
    as *const libc::c_char;
