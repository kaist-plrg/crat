use ::libc;
pub unsafe extern "C" fn nettle_version_major() -> libc::c_int {
    return 3 as libc::c_int;
}
pub unsafe extern "C" fn nettle_version_minor() -> libc::c_int {
    return 9 as libc::c_int;
}
