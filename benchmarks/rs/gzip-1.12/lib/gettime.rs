use ::libc;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub unsafe extern "C" fn gettime(mut ts: *mut timespec) {
    clock_gettime(0 as libc::c_int, ts);
}
pub unsafe extern "C" fn current_timespec() -> timespec {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut ts);
    return ts;
}
