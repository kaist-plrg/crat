use ::libc;
extern "C" {
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> libc::c_int;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub unsafe extern "C" fn _glp_xgmtime(mut timer: *const time_t) -> *mut tm {
    #[thread_local]
    static mut result: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    gmtime_r(timer, &mut result);
    return &mut result;
}
pub unsafe extern "C" fn _glp_xstrerr(mut errnum: libc::c_int) -> *mut libc::c_char {
    #[thread_local]
    static mut s: [libc::c_char; 1024] = [0; 1024];
    strerror_r(
        errnum,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    return s.as_mut_ptr();
}
pub unsafe extern "C" fn _glp_xstrtok(
    mut s1: *mut libc::c_char,
    mut s2: *const libc::c_char,
) -> *mut libc::c_char {
    #[thread_local]
    static mut ptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    return strtok_r(s1, s2, &mut ptr);
}
