use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub const BUGGY_READ_MAXIMUM: C2RustUnnamed = 2147475456;
pub type C2RustUnnamed = libc::c_uint;
pub unsafe extern "C" fn safe_write(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> size_t {
    loop {
        let mut result: ssize_t = write(fd, buf, count);
        if 0 as libc::c_int as libc::c_long <= result {
            return result as size_t
        } else {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if *__errno_location() == 22 as libc::c_int
                && (BUGGY_READ_MAXIMUM as libc::c_int as libc::c_ulong) < count
            {
                count = BUGGY_READ_MAXIMUM as libc::c_int as size_t;
            } else {
                return result as size_t
            }
        }
    };
}
