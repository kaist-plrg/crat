use ::libc;
extern "C" {
    fn abort() -> !;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type time_t = __time_t;
pub unsafe extern "C" fn rpl_time(mut tp: *mut time_t) -> time_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tt: time_t = 0;
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) < 0 as libc::c_int {
        abort();
    }
    tt = tv.tv_sec;
    if !tp.is_null() {
        *tp = tt;
    }
    return tt;
}
