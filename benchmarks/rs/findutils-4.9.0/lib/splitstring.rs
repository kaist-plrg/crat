use ::libc;
extern "C" {
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn field_length(
    mut str: *const libc::c_char,
    mut separators: *const libc::c_char,
) -> size_t {
    if *separators != 0 {
        let mut end: *const libc::c_char = strpbrk(str, separators);
        if !end.is_null() {
            return end.offset_from(str) as libc::c_long as size_t;
        }
    }
    return strlen(str);
}
pub unsafe extern "C" fn splitstring(
    mut s: *const libc::c_char,
    mut separators: *const libc::c_char,
    mut first: bool,
    mut pos: *mut size_t,
    mut len: *mut size_t,
) -> bool {
    if first {
        *pos = 0 as libc::c_uint as size_t;
        *len = 0 as libc::c_uint as size_t;
    } else {
        *pos = (*pos as libc::c_ulong).wrapping_add(*len) as size_t as size_t;
        if *s.offset(*pos as isize) != 0 {
            *pos = (*pos).wrapping_add(1);
            *pos;
        } else {
            return 0 as libc::c_int != 0
        }
    }
    *len = field_length(&*s.offset(*pos as isize), separators);
    return 1 as libc::c_int != 0;
}
