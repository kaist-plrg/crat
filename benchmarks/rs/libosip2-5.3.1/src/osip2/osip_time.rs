use ::libc;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
static mut offset: timeval = {
    let mut init = timeval {
        tv_sec: 0 as libc::c_int as __time_t,
        tv_usec: 0 as libc::c_int as __suseconds_t,
    };
    init
};
pub unsafe extern "C" fn add_gettimeofday(mut atv: *mut timeval, mut ms: libc::c_int) {
    let mut m: libc::c_int = 0;
    if ms >= 1000000 as libc::c_int {
        (*atv).tv_usec = 0 as libc::c_int as __suseconds_t;
        m = ms / 1000 as libc::c_int;
    } else {
        (*atv).tv_usec += (ms * 1000 as libc::c_int) as libc::c_long;
        m = ((*atv).tv_usec / 1000000 as libc::c_int as libc::c_long) as libc::c_int;
        (*atv).tv_usec = (*atv).tv_usec % 1000000 as libc::c_int as libc::c_long;
    }
    (*atv).tv_sec += m as libc::c_long;
}
pub unsafe extern "C" fn min_timercmp(mut tv1: *mut timeval, mut tv2: *mut timeval) {
    if (*tv2).tv_sec == -(1 as libc::c_int) as libc::c_long {
        return;
    }
    if if (*tv1).tv_sec == (*tv2).tv_sec {
        ((*tv1).tv_usec > (*tv2).tv_usec) as libc::c_int
    } else {
        ((*tv1).tv_sec > (*tv2).tv_sec) as libc::c_int
    } != 0
    {
        (*tv1).tv_sec = (*tv2).tv_sec;
        (*tv1).tv_usec = (*tv2).tv_usec;
    }
}
pub unsafe extern "C" fn osip_gettimeofday(
    mut tp: *mut timeval,
    mut tz: *mut libc::c_void,
) -> libc::c_int {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if clock_gettime(1 as libc::c_int, &mut ts) < 0 as libc::c_int {
        gettimeofday(tp, tz);
        return 0 as libc::c_int;
    }
    (*tp).tv_sec = ts.tv_sec + offset.tv_sec;
    (*tp).tv_usec = ts.tv_nsec / 1000 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_compensatetime() {}
pub unsafe extern "C" fn osip_getsystemtime(mut t: *mut time_t) -> time_t {
    let mut now_monotonic: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    osip_gettimeofday(&mut now_monotonic, 0 as *mut libc::c_void);
    if !t.is_null() {
        *t = now_monotonic.tv_sec;
    }
    return now_monotonic.tv_sec;
}
