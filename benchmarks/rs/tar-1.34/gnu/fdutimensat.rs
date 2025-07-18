use ::libc;
extern "C" {
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub unsafe extern "C" fn fdutimensat(
    mut fd: libc::c_int,
    mut dir: libc::c_int,
    mut file: *const libc::c_char,
    mut ts: *const timespec,
    mut atflag: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 1 as libc::c_int;
    if 0 as libc::c_int <= fd {
        result = futimens(fd, ts);
    }
    if !file.is_null()
        && (fd < 0 as libc::c_int
            || result == -(1 as libc::c_int) && *__errno_location() == 38 as libc::c_int)
    {
        result = utimensat(dir, file, ts, atflag);
    }
    if result == 1 as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        result = -(1 as libc::c_int);
    }
    return result;
}
