use ::libc;
extern "C" {
    fn xalloc_die();
    fn mmalloca(n: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn xmmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = mmalloca(n);
    if p.is_null() {
        xalloc_die();
    }
    return p;
}
