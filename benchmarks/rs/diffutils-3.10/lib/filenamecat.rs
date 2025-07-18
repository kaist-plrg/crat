use ::libc;
extern "C" {
    fn mfile_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn xalloc_die();
}
pub unsafe extern "C" fn file_name_concat(
    mut dir: *const libc::c_char,
    mut base: *const libc::c_char,
    mut base_in_result: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = mfile_name_concat(dir, base, base_in_result);
    if p.is_null() {
        xalloc_die();
    }
    return p;
}
