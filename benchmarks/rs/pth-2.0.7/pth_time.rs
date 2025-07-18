use ::libc;
extern "C" {
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pth_time_t = timeval;
pub static mut __pth_time_zero: pth_time_t = {
    let mut init = timeval {
        tv_sec: 0 as libc::c_long,
        tv_usec: 0 as libc::c_long,
    };
    init
};
pub unsafe extern "C" fn __pth_time_usleep(mut usec: libc::c_ulong) {
    usleep(usec as libc::c_uint);
}
pub unsafe extern "C" fn pth_time(
    mut sec: libc::c_long,
    mut usec: libc::c_long,
) -> pth_time_t {
    let mut tv: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    tv.tv_sec = sec;
    tv.tv_usec = usec;
    return tv;
}
pub unsafe extern "C" fn pth_timeout(
    mut sec: libc::c_long,
    mut usec: libc::c_long,
) -> pth_time_t {
    let mut tv: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tvd: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut tv, 0 as *mut libc::c_void);
    } else {
        tv.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        tv.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    tvd.tv_sec = sec;
    tvd.tv_usec = usec;
    tv.tv_sec += tvd.tv_sec;
    tv.tv_usec += tvd.tv_usec;
    if tv.tv_usec > 1000000 as libc::c_int as libc::c_long {
        tv.tv_sec += 1 as libc::c_int as libc::c_long;
        tv.tv_usec -= 1000000 as libc::c_int as libc::c_long;
    }
    return tv;
}
pub unsafe extern "C" fn __pth_time_cmp(
    mut t1: *mut pth_time_t,
    mut t2: *mut pth_time_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = ((*t1).tv_sec - (*t2).tv_sec) as libc::c_int;
    if rc == 0 as libc::c_int {
        rc = ((*t1).tv_usec - (*t2).tv_usec) as libc::c_int;
    }
    return rc;
}
pub unsafe extern "C" fn __pth_time_div(mut t1: *mut pth_time_t, mut n: libc::c_int) {
    let mut q: libc::c_long = 0;
    let mut r: libc::c_long = 0;
    q = (*t1).tv_sec / n as libc::c_long;
    r = (*t1).tv_sec % n as libc::c_long * 1000000 as libc::c_int as libc::c_long
        / n as libc::c_long + (*t1).tv_usec / n as libc::c_long;
    if r > 1000000 as libc::c_int as libc::c_long {
        q += 1 as libc::c_int as libc::c_long;
        r -= 1000000 as libc::c_int as libc::c_long;
    }
    (*t1).tv_sec = q;
    (*t1).tv_usec = r;
}
pub unsafe extern "C" fn __pth_time_mul(mut t1: *mut pth_time_t, mut n: libc::c_int) {
    (*t1).tv_sec *= n as libc::c_long;
    (*t1).tv_usec *= n as libc::c_long;
    (*t1).tv_sec += (*t1).tv_usec / 1000000 as libc::c_int as libc::c_long;
    (*t1).tv_usec = (*t1).tv_usec % 1000000 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn __pth_time_t2d(mut t: *mut pth_time_t) -> libc::c_double {
    let mut d: libc::c_double = 0.;
    d = ((*t).tv_sec as libc::c_double * 1000000 as libc::c_int as libc::c_double
        + (*t).tv_usec as libc::c_double) / 1000000 as libc::c_int as libc::c_double;
    return d;
}
pub unsafe extern "C" fn __pth_time_t2i(mut t: *mut pth_time_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = (((*t).tv_sec * 1000000 as libc::c_int as libc::c_long + (*t).tv_usec)
        / 1000000 as libc::c_int as libc::c_long) as libc::c_int;
    return i;
}
pub unsafe extern "C" fn __pth_time_pos(mut t: *mut pth_time_t) -> libc::c_int {
    if (*t).tv_sec > 0 as libc::c_int as libc::c_long
        && (*t).tv_usec > 0 as libc::c_int as libc::c_long
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
