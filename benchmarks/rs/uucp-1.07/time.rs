use ::libc;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type boolean = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_timespan {
    pub uuconf_qnext: *mut uuconf_timespan,
    pub uuconf_istart: libc::c_int,
    pub uuconf_iend: libc::c_int,
    pub uuconf_ival: libc::c_long,
    pub uuconf_cretry: libc::c_int,
}
pub static mut time_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: time.c,v 1.22 2002/03/05 19:10:41 ian Rel $\0")
};
pub unsafe extern "C" fn ftimespan_match(
    mut qspan: *const uuconf_timespan,
    mut pival: *mut libc::c_long,
    mut pcretry: *mut libc::c_int,
) -> boolean {
    let mut inow: time_t = 0;
    let mut qtm: *mut tm = 0 as *mut tm;
    let mut itm: libc::c_int = 0;
    let mut q: *const uuconf_timespan = 0 as *const uuconf_timespan;
    if qspan.is_null() {
        return 0 as libc::c_int;
    }
    time(&mut inow);
    qtm = localtime(&mut inow);
    itm = (*qtm).tm_wday * 24 as libc::c_int * 60 as libc::c_int
        + (*qtm).tm_hour * 60 as libc::c_int + (*qtm).tm_min;
    q = qspan;
    while !q.is_null() {
        if (*q).uuconf_istart <= itm && itm <= (*q).uuconf_iend {
            if !pival.is_null() {
                *pival = (*q).uuconf_ival;
            }
            if !pcretry.is_null() {
                *pcretry = (*q).uuconf_cretry;
            }
            return 1 as libc::c_int;
        }
        q = (*q).uuconf_qnext;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cmax_size_ever(
    mut qtimesize: *const uuconf_timespan,
) -> libc::c_long {
    let mut imax: libc::c_long = 0;
    let mut q: *const uuconf_timespan = 0 as *const uuconf_timespan;
    if qtimesize.is_null() {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*qtimesize).uuconf_istart >= 60 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    imax = -(1 as libc::c_int) as libc::c_long;
    q = qtimesize;
    while !q.is_null() {
        if ((*q).uuconf_qnext).is_null() {
            if (*q).uuconf_iend
                <= 6 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                    + 23 as libc::c_int * 60 as libc::c_int
            {
                return -(1 as libc::c_int) as libc::c_long;
            }
        } else if (*q).uuconf_iend + 60 as libc::c_int
            <= (*(*q).uuconf_qnext).uuconf_istart
        {
            return -(1 as libc::c_int) as libc::c_long
        }
        if imax < (*q).uuconf_ival {
            imax = (*q).uuconf_ival;
        }
        q = (*q).uuconf_qnext;
    }
    return imax;
}
