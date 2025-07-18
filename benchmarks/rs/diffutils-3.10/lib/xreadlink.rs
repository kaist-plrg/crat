use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn areadlink(filename: *const libc::c_char) -> *mut libc::c_char;
    fn xalloc_die();
}
pub unsafe extern "C" fn xreadlink(
    mut filename: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = areadlink(filename);
    if result.is_null() && *__errno_location() == 12 as libc::c_int {
        xalloc_die();
    }
    return result;
}
