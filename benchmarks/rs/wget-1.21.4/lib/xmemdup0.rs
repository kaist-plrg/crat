use ::libc;
extern "C" {
    fn xcharalloc(n: size_t) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn xmemdup0(
    mut p: *const libc::c_void,
    mut s: size_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = xcharalloc(
        s.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if s > 0 as libc::c_int as libc::c_ulong {
        memcpy(result as *mut libc::c_void, p, s);
    }
    *result.offset(s as isize) = 0 as libc::c_int as libc::c_char;
    return result;
}
