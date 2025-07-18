use ::libc;
extern "C" {
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn strnlen1(
    mut string: *const libc::c_char,
    mut maxlen: size_t,
) -> size_t {
    let mut end: *const libc::c_char = memchr(
        string as *const libc::c_void,
        '\0' as i32,
        maxlen,
    ) as *const libc::c_char;
    if !end.is_null() {
        return (end.offset_from(string) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as size_t
    } else {
        return maxlen
    };
}
