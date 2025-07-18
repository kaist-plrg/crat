use ::libc;
pub static mut version_string: *const libc::c_char = b"GNU Awk 5.2.2\0" as *const u8
    as *const libc::c_char;
