use ::libc;
pub static mut VERSION: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
