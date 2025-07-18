use ::libc;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: libc::c_long) -> timespec {
    return {
        let mut init = timespec { tv_sec: s, tv_nsec: ns };
        init
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn timespec_sign(mut a: timespec) -> libc::c_int {
    return (a.tv_sec > 0 as libc::c_int as libc::c_long) as libc::c_int
        - (a.tv_sec < 0 as libc::c_int as libc::c_long) as libc::c_int
        + ((a.tv_sec == 0) as libc::c_int & (a.tv_nsec != 0) as libc::c_int);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn timespectod(mut a: timespec) -> libc::c_double {
    return a.tv_sec as libc::c_double + a.tv_nsec as libc::c_double / 1e9f64;
}
