use ::libc;
extern "C" {
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub const SYS_BUFSIZE_MAX: C2RustUnnamed = 2146435072;
pub type C2RustUnnamed = libc::c_uint;
pub unsafe extern "C" fn safe_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> size_t {
    loop {
        let mut result: ssize_t = read(fd, buf, count);
        if 0 as libc::c_int as libc::c_long <= result {
            return result as size_t
        } else {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if *__errno_location() == 22 as libc::c_int
                && (SYS_BUFSIZE_MAX as libc::c_int as libc::c_ulong) < count
            {
                count = SYS_BUFSIZE_MAX as libc::c_int as size_t;
            } else {
                return result as size_t
            }
        }
    };
}
