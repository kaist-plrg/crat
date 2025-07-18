use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> *mut libc::c_char;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __xpg_strerror_r(
        errnum: libc::c_int,
        buf: *mut libc::c_char,
        buflen: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn safe_copy(
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut len: size_t = strlen(msg);
    let mut moved: size_t = if len < buflen {
        len
    } else {
        buflen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    };
    memmove(buf as *mut libc::c_void, msg as *const libc::c_void, moved);
    *buf.offset(moved as isize) = '\0' as i32 as libc::c_char;
    return if len < buflen { 0 as libc::c_int } else { 34 as libc::c_int };
}
pub unsafe extern "C" fn rpl_strerror_r(
    mut errnum: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> libc::c_int {
    if buflen <= 1 as libc::c_int as libc::c_ulong {
        if buflen != 0 {
            *buf = '\0' as i32 as libc::c_char;
        }
        return 34 as libc::c_int;
    }
    *buf = '\0' as i32 as libc::c_char;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    if !msg.is_null() {
        return safe_copy(buf, buflen, msg);
    }
    let mut ret: libc::c_int = 0;
    let mut saved_errno: libc::c_int = *__errno_location();
    ret = 0 as libc::c_int;
    ret = __xpg_strerror_r(errnum, buf, buflen);
    if ret < 0 as libc::c_int {
        ret = *__errno_location();
    }
    if *buf == 0 {
        let mut errstring: *mut libc::c_char = strerror_r(errnum, buf, buflen);
        ret = if !errstring.is_null() {
            safe_copy(buf, buflen, errstring)
        } else {
            *__errno_location()
        };
    }
    if ret == 22 as libc::c_int && *buf == 0 {
        snprintf(
            buf,
            buflen,
            b"Unknown error %d\0" as *const u8 as *const libc::c_char,
            errnum,
        );
    }
    *__errno_location() = saved_errno;
    return ret;
}
