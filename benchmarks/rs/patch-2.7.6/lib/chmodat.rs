use ::libc;
extern "C" {
    fn fchmodat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __mode: __mode_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lchmodat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return fchmodat(fd, file, mode, 0x100 as libc::c_int);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn chmodat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return fchmodat(fd, file, mode, 0 as libc::c_int);
}
