use ::libc;
extern "C" {
    fn concatenated_filename(
        directory: *const libc::c_char,
        filename: *const libc::c_char,
        suffix: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xalloc_die();
}
pub unsafe extern "C" fn xconcatenated_filename(
    mut directory: *const libc::c_char,
    mut filename: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    result = concatenated_filename(directory, filename, suffix);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
