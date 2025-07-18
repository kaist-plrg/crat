use ::libc;
extern "C" {
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub unsafe extern "C" fn pthread_sigmask(
    mut how: libc::c_int,
    mut new_mask: *const sigset_t,
    mut old_mask: *mut sigset_t,
) -> libc::c_int {
    let mut ret: libc::c_int = sigprocmask(how, new_mask, old_mask);
    return if ret < 0 as libc::c_int { *__errno_location() } else { 0 as libc::c_int };
}
