use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn xgetcwd() -> *mut libc::c_char {
    let mut cwd: *mut libc::c_char = getcwd(
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
    );
    if cwd.is_null() && *__errno_location() == 12 as libc::c_int {
        xalloc_die();
    }
    return cwd;
}
