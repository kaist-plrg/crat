use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DATE_T {
    pub date: *mut libc::c_char,
    pub etag: *mut libc::c_char,
    pub head: *mut libc::c_char,
    pub tm: *mut tm,
    pub safe: tm,
}
pub type DATE = *mut DATE_T;
pub type assume = libc::c_uint;
pub const DATE_TIME: assume = 2;
pub const DATE_YEAR: assume = 1;
pub const DATE_MDAY: assume = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tzinfo {
    pub name: *const libc::c_char,
    pub offset: libc::c_int,
}
pub static mut wday: [*const libc::c_char; 7] = [
    b"Mon\0" as *const u8 as *const libc::c_char,
    b"Tue\0" as *const u8 as *const libc::c_char,
    b"Wed\0" as *const u8 as *const libc::c_char,
    b"Thu\0" as *const u8 as *const libc::c_char,
    b"Fri\0" as *const u8 as *const libc::c_char,
    b"Sat\0" as *const u8 as *const libc::c_char,
    b"Sun\0" as *const u8 as *const libc::c_char,
];
pub static mut weekday: [*const libc::c_char; 7] = [
    b"Sunday\0" as *const u8 as *const libc::c_char,
    b"Monday\0" as *const u8 as *const libc::c_char,
    b"Tuesday\0" as *const u8 as *const libc::c_char,
    b"Wednesday\0" as *const u8 as *const libc::c_char,
    b"Thursday\0" as *const u8 as *const libc::c_char,
    b"Friday\0" as *const u8 as *const libc::c_char,
    b"Saturday\0" as *const u8 as *const libc::c_char,
];
pub static mut month: [*const libc::c_char; 12] = [
    b"Jan\0" as *const u8 as *const libc::c_char,
    b"Feb\0" as *const u8 as *const libc::c_char,
    b"Mar\0" as *const u8 as *const libc::c_char,
    b"Apr\0" as *const u8 as *const libc::c_char,
    b"May\0" as *const u8 as *const libc::c_char,
    b"Jun\0" as *const u8 as *const libc::c_char,
    b"Jul\0" as *const u8 as *const libc::c_char,
    b"Aug\0" as *const u8 as *const libc::c_char,
    b"Sep\0" as *const u8 as *const libc::c_char,
    b"Oct\0" as *const u8 as *const libc::c_char,
    b"Nov\0" as *const u8 as *const libc::c_char,
    b"Dec\0" as *const u8 as *const libc::c_char,
];
static mut tz: [tzinfo; 43] = [
    {
        let mut init = tzinfo {
            name: b"GMT\0" as *const u8 as *const libc::c_char,
            offset: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"UTC\0" as *const u8 as *const libc::c_char,
            offset: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"WET\0" as *const u8 as *const libc::c_char,
            offset: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"BST\0" as *const u8 as *const libc::c_char,
            offset: 0 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"WAT\0" as *const u8 as *const libc::c_char,
            offset: 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"AST\0" as *const u8 as *const libc::c_char,
            offset: 240 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"ADT\0" as *const u8 as *const libc::c_char,
            offset: 240 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"EST\0" as *const u8 as *const libc::c_char,
            offset: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"EDT\0" as *const u8 as *const libc::c_char,
            offset: 300 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"CST\0" as *const u8 as *const libc::c_char,
            offset: 360 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"CDT\0" as *const u8 as *const libc::c_char,
            offset: 360 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"MST\0" as *const u8 as *const libc::c_char,
            offset: 420 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"MDT\0" as *const u8 as *const libc::c_char,
            offset: 420 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"PST\0" as *const u8 as *const libc::c_char,
            offset: 480 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"PDT\0" as *const u8 as *const libc::c_char,
            offset: 480 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"YST\0" as *const u8 as *const libc::c_char,
            offset: 540 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"YDT\0" as *const u8 as *const libc::c_char,
            offset: 540 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"HST\0" as *const u8 as *const libc::c_char,
            offset: 600 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"HDT\0" as *const u8 as *const libc::c_char,
            offset: 600 as libc::c_int - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"CAT\0" as *const u8 as *const libc::c_char,
            offset: 600 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"AHST\0" as *const u8 as *const libc::c_char,
            offset: 600 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"NT\0" as *const u8 as *const libc::c_char,
            offset: 660 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"IDLW\0" as *const u8 as *const libc::c_char,
            offset: 720 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"CET\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"MET\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"MEWT\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"MEST\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int) - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"CEST\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int) - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"MESZ\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int) - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"FWT\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"FST\0" as *const u8 as *const libc::c_char,
            offset: -(60 as libc::c_int) - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"EET\0" as *const u8 as *const libc::c_char,
            offset: -(120 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"WAST\0" as *const u8 as *const libc::c_char,
            offset: -(420 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"WADT\0" as *const u8 as *const libc::c_char,
            offset: -(420 as libc::c_int) - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"CCT\0" as *const u8 as *const libc::c_char,
            offset: -(480 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"JST\0" as *const u8 as *const libc::c_char,
            offset: -(540 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"EAST\0" as *const u8 as *const libc::c_char,
            offset: -(600 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"EADT\0" as *const u8 as *const libc::c_char,
            offset: -(600 as libc::c_int) - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"GST\0" as *const u8 as *const libc::c_char,
            offset: -(600 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"NZT\0" as *const u8 as *const libc::c_char,
            offset: -(720 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"NZST\0" as *const u8 as *const libc::c_char,
            offset: -(720 as libc::c_int),
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"NZDT\0" as *const u8 as *const libc::c_char,
            offset: -(720 as libc::c_int) - 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = tzinfo {
            name: b"IDLE\0" as *const u8 as *const libc::c_char,
            offset: -(720 as libc::c_int),
        };
        init
    },
];
pub static mut DATESIZE: size_t = ::std::mem::size_of::<DATE_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_date(mut date: *mut libc::c_char) -> DATE {
    let mut now: time_t = 0;
    let mut this: DATE = calloc(DATESIZE, 1 as libc::c_int as libc::c_ulong) as DATE;
    (*this).tm = 0 as *mut tm;
    (*this).etag = 0 as *mut libc::c_char;
    (*this).date = malloc(64 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    (*this).head = malloc(64 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    memset(
        (*this).date as *mut libc::c_void,
        '\0' as i32,
        64 as libc::c_int as libc::c_ulong,
    );
    memset(
        (*this).head as *mut libc::c_void,
        '\0' as i32,
        64 as libc::c_int as libc::c_ulong,
    );
    if date.is_null() {
        now = time(0 as *mut time_t);
        (*this).tm = gmtime_r(&mut now, &mut (*this).safe);
    } else {
        now = __strtotime(date);
        (*this).tm = gmtime_r(&mut now, &mut (*this).safe);
    }
    return this;
}
pub unsafe extern "C" fn new_etag(mut etag: *mut libc::c_char) -> DATE {
    let mut this: DATE = calloc(DATESIZE, 1 as libc::c_int as libc::c_ulong) as DATE;
    (*this).tm = 0 as *mut tm;
    (*this).date = 0 as *mut libc::c_char;
    (*this).etag = 0 as *mut libc::c_char;
    if !etag.is_null() {
        (*this).etag = xstrdup(etag);
    }
    return this;
}
pub unsafe extern "C" fn date_destroy(mut this: DATE) -> DATE {
    if !this.is_null() {
        free((*this).date as *mut libc::c_void);
        free((*this).head as *mut libc::c_void);
        free((*this).etag as *mut libc::c_void);
        free(this as *mut libc::c_void);
        this = 0 as DATE;
    }
    return this;
}
pub unsafe extern "C" fn date_get_etag(mut this: DATE) -> *mut libc::c_char {
    return (if ((*this).etag).is_null() {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        (*this).etag as *const libc::c_char
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn date_get_rfc850(mut this: DATE) -> *mut libc::c_char {
    memset(
        (*this).date as *mut libc::c_void,
        '\0' as i32,
        64 as libc::c_int as libc::c_ulong,
    );
    if ((*this).tm).is_null() || (*(*this).tm).tm_year == 0 as libc::c_int {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    snprintf(
        (*this).date,
        64 as libc::c_int as libc::c_ulong,
        b"%s, %d %s %d %d:%d:%d GMT\0" as *const u8 as *const libc::c_char,
        wday[(*(*this).tm).tm_wday as usize],
        (*(*this).tm).tm_mday,
        month[(*(*this).tm).tm_mon as usize],
        (*(*this).tm).tm_year,
        (*(*this).tm).tm_hour,
        (*(*this).tm).tm_min,
        (*(*this).tm).tm_sec,
    );
    return (*this).date;
}
pub unsafe extern "C" fn date_expired(mut this: DATE) -> BOOLEAN {
    let mut res: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut now: time_t = 0;
    let mut gmt: *mut tm = 0 as *mut tm;
    let mut then: time_t = 0;
    if this.is_null() || ((*this).tm).is_null() {
        return boolean_true;
    }
    now = time(0 as *mut time_t);
    gmt = gmtime(&mut now);
    then = mktime((*this).tm);
    now = mktime(gmt);
    res = difftime(then, now) as libc::c_long;
    return (if res < 0 as libc::c_int as libc::c_long {
        boolean_true as libc::c_int
    } else {
        boolean_false as libc::c_int
    }) as BOOLEAN;
}
pub unsafe extern "C" fn date_to_string(mut this: DATE) -> *mut libc::c_char {
    if !((*this).etag).is_null() {
        return (*this).etag;
    }
    memset(
        (*this).date as *mut libc::c_void,
        '\0' as i32,
        64 as libc::c_int as libc::c_ulong,
    );
    setlocale(2 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    strftime(
        (*this).date,
        64 as libc::c_int as size_t,
        b"%a, %F %T \0" as *const u8 as *const libc::c_char,
        (*this).tm,
    );
    return (*this).date;
}
pub unsafe extern "C" fn mylocaltime(mut tm: *mut tm) -> time_t {
    let mut epoch: time_t = 0 as libc::c_int as time_t;
    let mut offset: time_t = mktime(localtime(&mut epoch));
    let mut local: time_t = mktime(tm);
    return difftime(local, offset) as time_t;
}
pub unsafe extern "C" fn date_stamp(mut this: DATE) -> *mut libc::c_char {
    let mut tmp: *mut tm = 0 as *mut tm;
    let mut time_0: time_t = 0;
    time_0 = mylocaltime((*this).tm);
    tmp = localtime(&mut time_0);
    memset(
        (*this).date as *mut libc::c_void,
        '\0' as i32,
        64 as libc::c_int as libc::c_ulong,
    );
    strftime(
        (*this).date,
        64 as libc::c_int as size_t,
        b"[%a, %F %T] \0" as *const u8 as *const libc::c_char,
        tmp,
    );
    return (*this).date;
}
pub unsafe extern "C" fn date_adjust(
    mut tvalue: time_t,
    mut secs: libc::c_int,
) -> time_t {
    let mut tp: *mut tm = 0 as *mut tm;
    let mut ret: time_t = 0;
    ret = (tvalue != -(1 as libc::c_int) as time_t) as libc::c_int as time_t;
    if ret != 0 {
        tp = localtime(&mut tvalue);
        if secs > 2147483647 as libc::c_int - (*tp).tm_sec {
            ret = -(1 as libc::c_int) as time_t;
        } else {
            (*tp).tm_sec += secs;
            ret = mktime(tp);
        }
    }
    return ret;
}
unsafe extern "C" fn __checkday(
    mut check: *mut libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut what: *const *const libc::c_char = 0 as *const *const libc::c_char;
    let mut found: BOOLEAN = boolean_false;
    if len > 3 as libc::c_int as libc::c_ulong {
        what = &*weekday.as_ptr().offset(0 as libc::c_int as isize)
            as *const *const libc::c_char;
    } else {
        what = &*wday.as_ptr().offset(0 as libc::c_int as isize)
            as *const *const libc::c_char;
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if strmatch(check, *what.offset(0 as libc::c_int as isize) as *mut libc::c_char)
            as u64 != 0
        {
            found = boolean_true;
            break;
        } else {
            what = what.offset(1);
            what;
            i += 1;
            i;
        }
    }
    return if found as libc::c_uint != 0 { i } else { -(1 as libc::c_int) };
}
unsafe extern "C" fn __checkmonth(mut check: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut what: *const *const libc::c_char = 0 as *const *const libc::c_char;
    let mut found: BOOLEAN = boolean_false;
    what = &*month.as_ptr().offset(0 as libc::c_int as isize)
        as *const *const libc::c_char;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        if strmatch(check, *what.offset(0 as libc::c_int as isize) as *mut libc::c_char)
            as u64 != 0
        {
            found = boolean_true;
            break;
        } else {
            what = what.offset(1);
            what;
            i += 1;
            i;
        }
    }
    return if found as libc::c_uint != 0 { i } else { -(1 as libc::c_int) };
}
unsafe extern "C" fn __checktz(mut check: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut what: *const tzinfo = 0 as *const tzinfo;
    let mut found: BOOLEAN = boolean_false;
    what = tz.as_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[tzinfo; 43]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<tzinfo>() as libc::c_ulong)
    {
        if strmatch(check, (*what).name as *mut libc::c_char) as u64 != 0 {
            found = boolean_true;
            break;
        } else {
            what = what.offset(1);
            what;
            i = i.wrapping_add(1);
            i;
        }
    }
    return if found as libc::c_uint != 0 {
        (*what).offset * 60 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn skip(mut date: *mut *const libc::c_char) {
    while **date as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(**date as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        *date = (*date).offset(1);
        *date;
    }
}
unsafe extern "C" fn __strtotime(mut string: *const libc::c_char) -> time_t {
    let mut sec: libc::c_int = -(1 as libc::c_int);
    let mut min: libc::c_int = -(1 as libc::c_int);
    let mut hour: libc::c_int = -(1 as libc::c_int);
    let mut mday: libc::c_int = -(1 as libc::c_int);
    let mut mon: libc::c_int = -(1 as libc::c_int);
    let mut year: libc::c_int = -(1 as libc::c_int);
    let mut wday_0: libc::c_int = -(1 as libc::c_int);
    let mut tzoff: libc::c_int = -(1 as libc::c_int);
    let mut part: libc::c_int = 0 as libc::c_int;
    let mut t: time_t = 0 as libc::c_int as time_t;
    let mut tm: tm = tm {
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
    let mut date: *const libc::c_char = 0 as *const libc::c_char;
    let mut indate: *const libc::c_char = string;
    let mut dignext: assume = DATE_MDAY;
    let mut found: BOOLEAN = boolean_false;
    if !(!string.is_null() && *string as libc::c_int != 0) {
        return 0 as libc::c_int as time_t;
    }
    date = string;
    while *date as libc::c_int != 0 && part < 6 as libc::c_int {
        found = boolean_false;
        skip(&mut date);
        if *(*__ctype_b_loc()).offset(*date as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let mut buf: [libc::c_char; 32] = *::std::mem::transmute::<
                &[u8; 32],
                &mut [libc::c_char; 32],
            >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
            let mut len: size_t = 0;
            sscanf(
                date,
                b"%31[A-Za-z]\0" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
            );
            len = strlen(buf.as_mut_ptr());
            if wday_0 == -(1 as libc::c_int) {
                wday_0 = __checkday(buf.as_mut_ptr(), len);
                if wday_0 != -(1 as libc::c_int) {
                    found = boolean_true;
                }
            }
            if found as u64 == 0 && mon == -(1 as libc::c_int) {
                mon = __checkmonth(buf.as_mut_ptr());
                if mon != -(1 as libc::c_int) {
                    found = boolean_true;
                }
            }
            if found as u64 == 0 && tzoff == -(1 as libc::c_int) {
                tzoff = __checktz(buf.as_mut_ptr());
                if tzoff != -(1 as libc::c_int) {
                    found = boolean_true;
                }
            }
            if found as u64 == 0 {
                return -(1 as libc::c_int) as time_t;
            }
            date = date.offset(len as isize);
        } else if *(*__ctype_b_loc())
            .offset(*date as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut val: libc::c_int = 0;
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            if sec == -(1 as libc::c_int)
                && 3 as libc::c_int
                    == sscanf(
                        date,
                        b"%02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
                        &mut hour as *mut libc::c_int,
                        &mut min as *mut libc::c_int,
                        &mut sec as *mut libc::c_int,
                    )
            {
                date = date.offset(8 as libc::c_int as isize);
                found = boolean_true;
            } else {
                val = strtol(date, &mut end, 10 as libc::c_int) as libc::c_int;
                if tzoff == -(1 as libc::c_int)
                    && end.offset_from(date) as libc::c_long
                        == 4 as libc::c_int as libc::c_long && val < 1300 as libc::c_int
                    && indate < date
                    && (*date.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '+' as i32
                        || *date.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '-' as i32)
                {
                    found = boolean_true;
                    tzoff = (val / 100 as libc::c_int * 60 as libc::c_int
                        + val % 100 as libc::c_int) * 60 as libc::c_int;
                    tzoff = if *date.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '+' as i32
                    {
                        -tzoff
                    } else {
                        tzoff
                    };
                }
                if end.offset_from(date) as libc::c_long
                    == 8 as libc::c_int as libc::c_long && year == -(1 as libc::c_int)
                    && mon == -(1 as libc::c_int) && mday == -(1 as libc::c_int)
                {
                    found = boolean_true;
                    year = val / 10000 as libc::c_int;
                    mon = val % 10000 as libc::c_int / 100 as libc::c_int
                        - 1 as libc::c_int;
                    mday = val % 100 as libc::c_int;
                }
                if found as u64 == 0
                    && dignext as libc::c_uint
                        == DATE_MDAY as libc::c_int as libc::c_uint
                    && mday == -(1 as libc::c_int)
                {
                    if val > 0 as libc::c_int && val < 32 as libc::c_int {
                        mday = val;
                        found = boolean_true;
                    }
                    dignext = DATE_YEAR;
                }
                if found as u64 == 0
                    && dignext as libc::c_uint
                        == DATE_YEAR as libc::c_int as libc::c_uint
                    && year == -(1 as libc::c_int)
                {
                    year = val;
                    found = boolean_true;
                    if year > 1970 as libc::c_int {
                        year -= 1900 as libc::c_int;
                    }
                    if mday == -(1 as libc::c_int) {
                        dignext = DATE_MDAY;
                    }
                }
                if found as u64 == 0 {
                    return -(1 as libc::c_int) as time_t;
                }
                date = end;
            }
        }
        part += 1;
        part;
    }
    if -(1 as libc::c_int) == sec {
        hour = 0 as libc::c_int;
        min = hour;
        sec = min;
    }
    if -(1 as libc::c_int) == mday || -(1 as libc::c_int) == mon
        || -(1 as libc::c_int) == year
    {
        return -(1 as libc::c_int) as time_t;
    }
    if year > 2037 as libc::c_int {
        return 0x7fffffff as libc::c_int as time_t;
    }
    tm.tm_sec = sec;
    tm.tm_min = min;
    tm.tm_hour = hour;
    tm.tm_mday = mday;
    tm.tm_mon = mon;
    tm.tm_year = year;
    tm.tm_wday = 0 as libc::c_int;
    tm.tm_yday = 0 as libc::c_int;
    tm.tm_isdst = 0 as libc::c_int;
    t = mktime(&mut tm);
    if -(1 as libc::c_int) != t as libc::c_int {
        let mut gmt: *mut tm = 0 as *mut tm;
        let mut delta: libc::c_long = 0;
        let mut t2: time_t = 0;
        let mut keeptime2: tm = tm {
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
        gmt = gmtime_r(&mut t, &mut keeptime2);
        if gmt.is_null() {
            return -(1 as libc::c_int) as time_t;
        }
        t2 = mktime(gmt);
        delta = (if tzoff != -(1 as libc::c_int) { tzoff } else { 0 as libc::c_int })
            as libc::c_long + (t - t2);
        if delta > 0 as libc::c_int as libc::c_long && t + delta < t {
            return -(1 as libc::c_int) as time_t;
        }
        t += delta;
    }
    return t;
}
