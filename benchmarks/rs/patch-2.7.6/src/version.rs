use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
static mut copyright_string: [libc::c_char; 91] = unsafe {
    *::std::mem::transmute::<
        &[u8; 91],
        &[libc::c_char; 91],
    >(
        b"Copyright (C) 2003, 2009-2012 Free Software Foundation, Inc.\nCopyright (C) 1988 Larry Wall\0",
    )
};
static mut free_software_msgid: [libc::c_char; 200] = unsafe {
    *::std::mem::transmute::<
        &[u8; 200],
        &[libc::c_char; 200],
    >(
        b"License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\0",
    )
};
static mut authorship_msgid: [libc::c_char; 38] = unsafe {
    *::std::mem::transmute::<
        &[u8; 38],
        &[libc::c_char; 38],
    >(b"Written by Larry Wall and Paul Eggert\0")
};
pub unsafe extern "C" fn version() {
    printf(
        b"%s %s\n%s\n\n%s\n\n%s\n\0" as *const u8 as *const libc::c_char,
        b"GNU patch\0" as *const u8 as *const libc::c_char,
        b"2.7.6\0" as *const u8 as *const libc::c_char,
        copyright_string.as_ptr(),
        free_software_msgid.as_ptr(),
        authorship_msgid.as_ptr(),
    );
}
