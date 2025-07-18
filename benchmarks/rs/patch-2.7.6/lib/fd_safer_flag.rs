use ::libc;
extern "C" {
    fn dup_safer_flag(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub unsafe extern "C" fn fd_safer_flag(
    mut fd: libc::c_int,
    mut flag: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int <= fd && fd <= 2 as libc::c_int {
        let mut f: libc::c_int = dup_safer_flag(fd, flag);
        let mut e: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = e;
        fd = f;
    }
    return fd;
}
