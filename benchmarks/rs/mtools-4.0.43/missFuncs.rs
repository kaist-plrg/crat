use ::libc;
extern "C" {
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
static mut PATH_SEP: libc::c_char = '/' as i32 as libc::c_char;
pub unsafe extern "C" fn _basename(
    mut filename: *const libc::c_char,
) -> *const libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = strrchr(filename, PATH_SEP as libc::c_int);
    if !ptr.is_null() {
        filename = ptr.offset(1 as libc::c_int as isize);
    }
    return filename;
}
pub unsafe extern "C" fn _stripexe(mut filename: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = strrchr(filename, '.' as i32);
    if !ptr.is_null()
        && strcasecmp(ptr, b".exe\0" as *const u8 as *const libc::c_char) == 0
    {
        *ptr = '\0' as i32 as libc::c_char;
    }
}
