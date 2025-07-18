use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __time_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __suseconds_t = libc::c_long;
pub unsafe extern "C" fn snoopy_datasource_timestamp(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut retVal: libc::c_int = 0;
    retVal = gettimeofday(&mut tv, 0 as *mut libc::c_void);
    if 0 as libc::c_int == retVal {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            tv.tv_sec as libc::c_int,
        )
    } else {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error: %d)\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
        )
    };
}
