use ::libc;
extern "C" {
    fn mktime(__tp: *mut tm) -> time_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn tzset();
    fn timegm(__tp: *mut tm) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
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
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm_zone {
    pub next: *mut tm_zone,
    pub tz_is_set: libc::c_char,
    pub abbrs: [libc::c_char; 0],
}
pub type timezone_t = *mut tm_zone;
pub const ABBR_SIZE_MIN: C2RustUnnamed = 119;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed = libc::c_uint;
static mut local_tz: timezone_t = unsafe { 1 as libc::c_int as timezone_t };
unsafe extern "C" fn extend_abbrs(
    mut abbrs: *mut libc::c_char,
    mut abbr: *const libc::c_char,
    mut abbr_size: size_t,
) {
    memcpy(abbrs as *mut libc::c_void, abbr as *const libc::c_void, abbr_size);
    *abbrs.offset(abbr_size as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn tzalloc(mut name: *const libc::c_char) -> timezone_t {
    let mut name_size: size_t = if !name.is_null() {
        (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut abbr_size: size_t = if name_size
        < ABBR_SIZE_MIN as libc::c_int as libc::c_ulong
    {
        ABBR_SIZE_MIN as libc::c_int as libc::c_ulong
    } else {
        name_size.wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
    let mut tz: timezone_t = malloc(
        (9 as libc::c_ulong)
            .wrapping_add(::std::mem::align_of::<tm_zone>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(abbr_size)
            & !(::std::mem::align_of::<tm_zone>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as timezone_t;
    if !tz.is_null() {
        (*tz).next = 0 as *mut tm_zone;
        (*tz).tz_is_set = !name.is_null() as libc::c_int as libc::c_char;
        *((*tz).abbrs)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        if !name.is_null() {
            extend_abbrs(((*tz).abbrs).as_mut_ptr(), name, name_size);
        }
    }
    return tz;
}
unsafe extern "C" fn save_abbr(mut tz: timezone_t, mut tm: *mut tm) -> bool {
    let mut zone: *const libc::c_char = 0 as *const libc::c_char;
    let mut zone_copy: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    zone = (*tm).tm_zone;
    if zone.is_null()
        || tm as *mut libc::c_char <= zone as *mut libc::c_char
            && zone
                < tm.offset(1 as libc::c_int as isize) as *mut libc::c_char
                    as *const libc::c_char
    {
        return 1 as libc::c_int != 0;
    }
    if *zone != 0 {
        zone_copy = ((*tz).abbrs).as_mut_ptr();
        while strcmp(zone_copy, zone) != 0 as libc::c_int {
            if !(*zone_copy as libc::c_int != 0
                || zone_copy == ((*tz).abbrs).as_mut_ptr()
                    && (*tz).tz_is_set as libc::c_int != 0)
            {
                let mut zone_size: idx_t = (strlen(zone))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as idx_t;
                if zone_size
                    < ((*tz).abbrs)
                        .as_mut_ptr()
                        .offset(ABBR_SIZE_MIN as libc::c_int as isize)
                        .offset_from(zone_copy) as libc::c_long
                {
                    extend_abbrs(zone_copy, zone, zone_size as size_t);
                } else {
                    (*tz).next = tzalloc(zone);
                    tz = (*tz).next;
                    if tz.is_null() {
                        return 0 as libc::c_int != 0;
                    }
                    (*tz).tz_is_set = 0 as libc::c_int as libc::c_char;
                    zone_copy = ((*tz).abbrs).as_mut_ptr();
                }
                break;
            } else {
                zone_copy = zone_copy
                    .offset(
                        (strlen(zone_copy))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                if *zone_copy == 0 && !((*tz).next).is_null() {
                    tz = (*tz).next;
                    zone_copy = ((*tz).abbrs).as_mut_ptr();
                }
            }
        }
    }
    (*tm).tm_zone = zone_copy;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn tzfree(mut tz: timezone_t) {
    if tz != local_tz {
        while !tz.is_null() {
            let mut next: timezone_t = (*tz).next;
            rpl_free(tz as *mut libc::c_void);
            tz = next;
        }
    }
}
unsafe extern "C" fn getenv_TZ() -> *mut libc::c_char {
    return getenv(b"TZ\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn setenv_TZ(mut tz: *const libc::c_char) -> libc::c_int {
    return if !tz.is_null() {
        setenv(b"TZ\0" as *const u8 as *const libc::c_char, tz, 1 as libc::c_int)
    } else {
        unsetenv(b"TZ\0" as *const u8 as *const libc::c_char)
    };
}
unsafe extern "C" fn change_env(mut tz: timezone_t) -> bool {
    if setenv_TZ(
        if (*tz).tz_is_set as libc::c_int != 0 {
            ((*tz).abbrs).as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        },
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    tzset();
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn set_tz(mut tz: timezone_t) -> timezone_t {
    let mut env_tz: *mut libc::c_char = getenv_TZ();
    if if !env_tz.is_null() {
        ((*tz).tz_is_set as libc::c_int != 0
            && strcmp(((*tz).abbrs).as_mut_ptr(), env_tz) == 0 as libc::c_int)
            as libc::c_int
    } else {
        ((*tz).tz_is_set == 0) as libc::c_int
    } != 0
    {
        return local_tz
    } else {
        let mut old_tz: timezone_t = tzalloc(env_tz);
        if old_tz.is_null() {
            return old_tz;
        }
        if !change_env(tz) {
            let mut saved_errno: libc::c_int = *__errno_location();
            tzfree(old_tz);
            *__errno_location() = saved_errno;
            return 0 as timezone_t;
        }
        return old_tz;
    };
}
unsafe extern "C" fn revert_tz(mut tz: timezone_t) -> bool {
    if tz == local_tz {
        return 1 as libc::c_int != 0
    } else {
        let mut saved_errno: libc::c_int = *__errno_location();
        let mut ok: bool = change_env(tz);
        if !ok {
            saved_errno = *__errno_location();
        }
        tzfree(tz);
        *__errno_location() = saved_errno;
        return ok;
    };
}
pub unsafe extern "C" fn localtime_rz(
    mut tz: timezone_t,
    mut t: *const time_t,
    mut tm: *mut tm,
) -> *mut tm {
    if tz.is_null() {
        return gmtime_r(t, tm)
    } else {
        let mut old_tz: timezone_t = set_tz(tz);
        if !old_tz.is_null() {
            let mut abbr_saved: bool = !(localtime_r(t, tm)).is_null()
                && save_abbr(tz, tm) as libc::c_int != 0;
            if revert_tz(old_tz) as libc::c_int != 0 && abbr_saved as libc::c_int != 0 {
                return tm;
            }
        }
        return 0 as *mut tm;
    };
}
pub unsafe extern "C" fn mktime_z(mut tz: timezone_t, mut tm: *mut tm) -> time_t {
    if tz.is_null() {
        return timegm(tm)
    } else {
        let mut old_tz: timezone_t = set_tz(tz);
        if !old_tz.is_null() {
            let mut tm_1: tm = tm {
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
            tm_1.tm_sec = (*tm).tm_sec;
            tm_1.tm_min = (*tm).tm_min;
            tm_1.tm_hour = (*tm).tm_hour;
            tm_1.tm_mday = (*tm).tm_mday;
            tm_1.tm_mon = (*tm).tm_mon;
            tm_1.tm_year = (*tm).tm_year;
            tm_1.tm_yday = -(1 as libc::c_int);
            tm_1.tm_isdst = (*tm).tm_isdst;
            let mut t: time_t = mktime(&mut tm_1);
            let mut ok: bool = 0 as libc::c_int <= tm_1.tm_yday;
            ok = ok as libc::c_int != 0 && save_abbr(tz, &mut tm_1) as libc::c_int != 0;
            if revert_tz(old_tz) as libc::c_int != 0 && ok as libc::c_int != 0 {
                *tm = tm_1;
                return t;
            }
        }
        return -(1 as libc::c_int) as time_t;
    };
}
