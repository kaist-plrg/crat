use ::libc;
pub static mut version_string: *const libc::c_char = b"4.1.1\0" as *const u8
    as *const libc::c_char;
pub static mut program_name: *const libc::c_char = b"siege\0" as *const u8
    as *const libc::c_char;
pub static mut author_name: *const libc::c_char = b"Jeffrey Fulmer, et al.\0"
    as *const u8 as *const libc::c_char;
pub static mut email_address: *const libc::c_char = b"jeff@joedog.org\0" as *const u8
    as *const libc::c_char;
pub static mut years: *const libc::c_char = b"1999-2021\0" as *const u8
    as *const libc::c_char;
pub static mut copyright: *const libc::c_char = b"Copyright (C) 2021 by Jeffrey Fulmer, et al.\nThis is free software; see the source for copying conditions.\nThere is NO warranty; not even for MERCHANTABILITY or FITNESS\nFOR A PARTICULAR PURPOSE.\n\0"
    as *const u8 as *const libc::c_char;
