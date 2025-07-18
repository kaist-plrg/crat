use ::libc;
extern "C" {
    pub type __dirstream;
    fn __errno_location() -> *mut libc::c_int;
}
pub type DIR = __dirstream;
pub unsafe extern "C" fn rpl_dirfd(mut dir_p: *mut DIR) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        *__errno_location() = 95 as libc::c_int;
    }
    return fd;
}
