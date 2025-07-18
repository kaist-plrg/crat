use ::libc;
#[thread_local]
static mut tls: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
pub unsafe extern "C" fn _glp_tls_set_ptr(mut ptr: *mut libc::c_void) {
    tls = ptr;
}
pub unsafe extern "C" fn _glp_tls_get_ptr() -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = tls;
    return ptr;
}
