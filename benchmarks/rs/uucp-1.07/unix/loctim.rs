use ::libc;
extern "C" {
    fn localtime(__timer: *const time_t) -> *mut tm;
}
pub type __time_t = libc::c_long;
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
pub unsafe extern "C" fn usysdep_localtime(mut itime: libc::c_long, mut q: *mut tm) {
    let mut i: time_t = 0;
    i = itime;
    *q = *localtime(&mut i);
}
