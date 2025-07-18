use ::libc;
extern "C" {
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn xstrndup(
    mut string: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = strndup(string, n);
    if s.is_null() {
        xalloc_die();
    }
    return s;
}
