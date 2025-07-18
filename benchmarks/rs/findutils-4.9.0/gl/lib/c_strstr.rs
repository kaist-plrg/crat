use ::libc;
extern "C" {
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub unsafe extern "C" fn c_strstr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    return strstr(haystack, needle);
}
