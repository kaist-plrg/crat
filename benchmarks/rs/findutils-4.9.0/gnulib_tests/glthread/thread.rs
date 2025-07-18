use ::libc;
extern "C" {
    fn abort() -> !;
}
pub type gl_thread_t = libc::c_int;
pub unsafe extern "C" fn gl_thread_create(
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) -> gl_thread_t {
    let mut thread: gl_thread_t = 0;
    let mut ret: libc::c_int = 0;
    ret = 38 as libc::c_int;
    if ret != 0 as libc::c_int {
        abort();
    }
    return thread;
}
