use ::libc;
pub static mut version_string: *mut libc::c_char = b"4.4.1\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut make_host: *mut libc::c_char = b"x86_64-pc-linux-gnu\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
