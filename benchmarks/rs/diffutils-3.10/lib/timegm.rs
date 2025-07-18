use ::libc;
extern "C" {
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn mktime_internal(
        tp: *mut tm,
        func: Option::<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
        offset: *mut mktime_offset_t,
    ) -> time_t;
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
pub type mktime_offset_t = time_t;
pub unsafe extern "C" fn rpl_timegm(mut tmp: *mut tm) -> time_t {
    static mut gmtime_offset: mktime_offset_t = 0;
    (*tmp).tm_isdst = 0 as libc::c_int;
    return mktime_internal(
        tmp,
        Some(gmtime_r as unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm),
        &mut gmtime_offset,
    );
}
