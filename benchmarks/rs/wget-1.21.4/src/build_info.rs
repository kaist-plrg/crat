use ::libc;
pub static mut compiled_features: [*const libc::c_char; 14] = [
    b"-cares\0" as *const u8 as *const libc::c_char,
    b"+digest\0" as *const u8 as *const libc::c_char,
    b"-gpgme\0" as *const u8 as *const libc::c_char,
    b"-https\0" as *const u8 as *const libc::c_char,
    b"+ipv6\0" as *const u8 as *const libc::c_char,
    b"+iri\0" as *const u8 as *const libc::c_char,
    b"+large-file\0" as *const u8 as *const libc::c_char,
    b"-metalink\0" as *const u8 as *const libc::c_char,
    b"+nls\0" as *const u8 as *const libc::c_char,
    b"+ntlm\0" as *const u8 as *const libc::c_char,
    b"+opie\0" as *const u8 as *const libc::c_char,
    b"-psl\0" as *const u8 as *const libc::c_char,
    b"-ssl\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
