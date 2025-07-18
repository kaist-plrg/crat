use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> size_t {
    let mut len: size_t = strlen(src);
    let mut ret: size_t = len;
    if len >= size {
        len = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    memcpy(dst as *mut libc::c_void, src as *const libc::c_void, len);
    *dst.offset(len as isize) = '\0' as i32 as libc::c_char;
    return ret;
}
pub unsafe extern "C" fn strlcat(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> size_t {
    let mut len1: size_t = strlen(dst);
    let mut len2: size_t = strlen(src);
    let mut ret: size_t = len1.wrapping_add(len2);
    if len1.wrapping_add(len2) >= size {
        len2 = size.wrapping_sub(len1).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    if len2 > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            dst.offset(len1 as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            len2,
        );
        *dst.offset(len1.wrapping_add(len2) as isize) = '\0' as i32 as libc::c_char;
    }
    return ret;
}
pub unsafe extern "C" fn chomp(
    mut buffer: *mut libc::c_char,
    mut length: size_t,
) -> ssize_t {
    let mut chars: size_t = 0;
    if buffer.is_null() {
        return -(14 as libc::c_int) as ssize_t;
    }
    if length < 1 as libc::c_int as libc::c_ulong {
        return -(34 as libc::c_int) as ssize_t;
    }
    chars = 0 as libc::c_int as size_t;
    length = length.wrapping_sub(1);
    length;
    while *buffer.offset(length as isize) as libc::c_int == '\r' as i32
        || *buffer.offset(length as isize) as libc::c_int == '\n' as i32
    {
        *buffer.offset(length as isize) = '\0' as i32 as libc::c_char;
        chars = chars.wrapping_add(1);
        chars;
        let fresh0 = length;
        length = length.wrapping_sub(1);
        if fresh0 == 0 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    return chars as ssize_t;
}
