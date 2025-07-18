use ::libc;
extern "C" {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub unsafe extern "C" fn ixsysdep_process_time(
    mut pimicros: *mut libc::c_long,
) -> libc::c_long {
    let mut stime: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut stz: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    gettimeofday(&mut stime, &mut stz as *mut timezone as *mut libc::c_void);
    if !pimicros.is_null() {
        *pimicros = stime.tv_usec;
    }
    return stime.tv_sec;
}
