use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
pub type size_t = libc::c_ulong;
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
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
pub type time_t = __time_t;
pub type __time_t = libc::c_long;
pub unsafe extern "C" fn snoopy_datasource_datetime(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut curTime: time_t = 0;
    let mut curLocalTimeBuf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut curLocalTime: *const tm = 0 as *const tm;
    let mut formatToUse: *const libc::c_char = 0 as *const libc::c_char;
    let mut timeBuffer: [libc::c_char; 80] = [0; 80];
    if -(1 as libc::c_int) as time_t == time(&mut curTime) {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error @ time(): %d)\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
    }
    curLocalTime = localtime_r(&mut curTime, &mut curLocalTimeBuf);
    if curLocalTime.is_null() {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error @ localtime_r())\0" as *const u8 as *const libc::c_char,
        );
    }
    if *arg.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        formatToUse = arg;
    } else {
        formatToUse = b"%FT%T%z\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int as libc::c_ulong
        == strftime(
            timeBuffer.as_mut_ptr(),
            80 as libc::c_int as size_t,
            formatToUse,
            curLocalTime,
        )
    {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(error @ strftime())\0" as *const u8 as *const libc::c_char,
        );
    }
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        timeBuffer.as_mut_ptr(),
    );
}
