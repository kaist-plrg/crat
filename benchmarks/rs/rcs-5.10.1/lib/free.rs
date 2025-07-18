use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
}
pub unsafe extern "C" fn rpl_free(mut p: *mut libc::c_void) {
    let mut err: libc::c_int = *__errno_location();
    free(p);
    *__errno_location() = err;
}
