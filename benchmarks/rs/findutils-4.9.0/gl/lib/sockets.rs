use ::libc;
pub unsafe extern "C" fn gl_sockets_startup(mut version: libc::c_int) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gl_sockets_cleanup() -> libc::c_int {
    return 0 as libc::c_int;
}
