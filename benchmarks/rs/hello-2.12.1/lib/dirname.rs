use ::libc;
extern "C" {
    fn mdir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn xalloc_die();
}
pub unsafe extern "C" fn dir_name(mut file: *const libc::c_char) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = mdir_name(file);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
