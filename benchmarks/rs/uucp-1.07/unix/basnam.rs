use ::libc;
extern "C" {
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
}
pub unsafe extern "C" fn zsysdep_base_name(
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    z = strrchr(zfile, '/' as i32);
    if !z.is_null() {
        return zbufcpy(z.offset(1 as libc::c_int as isize));
    }
    return zbufcpy(zfile);
}
