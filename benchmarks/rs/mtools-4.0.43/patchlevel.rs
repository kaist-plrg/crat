use ::libc;
pub static mut mversion: *const libc::c_char = b"4.0.43\0" as *const u8
    as *const libc::c_char;
pub static mut mdate: *const libc::c_char = b"March 21st, 2023\0" as *const u8
    as *const libc::c_char;
pub static mut mformat_banner: *const libc::c_char = b"MTOO4043\0" as *const u8
    as *const libc::c_char;
