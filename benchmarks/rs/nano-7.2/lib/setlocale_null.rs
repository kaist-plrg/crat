use ::libc;
extern "C" {
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn setlocale_null_androidfix(
    mut category: libc::c_int,
) -> *const libc::c_char {
    let mut result: *const libc::c_char = setlocale(category, 0 as *const libc::c_char);
    return result;
}
unsafe extern "C" fn setlocale_null_unlocked(
    mut category: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> libc::c_int {
    let mut result: *const libc::c_char = setlocale_null_androidfix(category);
    if result.is_null() {
        if bufsize > 0 as libc::c_int as libc::c_ulong {
            *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        return 22 as libc::c_int;
    } else {
        let mut length: size_t = strlen(result);
        if length < bufsize {
            memcpy(
                buf as *mut libc::c_void,
                result as *const libc::c_void,
                length.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            return 0 as libc::c_int;
        } else {
            if bufsize > 0 as libc::c_int as libc::c_ulong {
                memcpy(
                    buf as *mut libc::c_void,
                    result as *const libc::c_void,
                    bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                *buf
                    .offset(
                        bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\0' as i32 as libc::c_char;
            }
            return 34 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn setlocale_null_r(
    mut category: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> libc::c_int {
    return setlocale_null_unlocked(category, buf, bufsize);
}
pub unsafe extern "C" fn setlocale_null(
    mut category: libc::c_int,
) -> *const libc::c_char {
    return setlocale_null_androidfix(category);
}
