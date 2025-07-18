use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
}
pub type pointer = *mut libc::c_void;
pub unsafe extern "C" fn xfree(mut p: pointer) {
    if !p.is_null() {
        free(p);
    }
}
