use ::libc;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn trim(str: *mut libc::c_char) -> *mut libc::c_char;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
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
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COOKIE_T {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub expires: time_t,
    pub expstr: *mut libc::c_char,
    pub none: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub session: BOOLEAN,
    pub secure: BOOLEAN,
}
pub type COOKIE = *mut COOKIE_T;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
pub static mut COOKIESIZE: size_t = ::std::mem::size_of::<COOKIE_T>() as libc::c_ulong;
static mut months: [*mut libc::c_char; 12] = [
    b"Jan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Feb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Mar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Apr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"May\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Jun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Jul\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Aug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Sep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Oct\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Nov\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Dec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn new_cookie(
    mut str: *mut libc::c_char,
    mut host: *mut libc::c_char,
) -> COOKIE {
    let mut this: COOKIE = 0 as *mut COOKIE_T;
    this = calloc(
        ::std::mem::size_of::<COOKIE_T>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as COOKIE;
    (*this).name = 0 as *mut libc::c_char;
    (*this).value = 0 as *mut libc::c_char;
    (*this).domain = 0 as *mut libc::c_char;
    (*this).expires = 0 as libc::c_int as time_t;
    (*this).expstr = 0 as *mut libc::c_char;
    (*this).string = 0 as *mut libc::c_char;
    (*this).session = boolean_true;
    (*this).none = strdup(b"none\0" as *const u8 as *const libc::c_char);
    if __parse_input(this, str, host) as libc::c_uint
        == boolean_false as libc::c_int as libc::c_uint
    {
        return cookie_destroy(this);
    }
    return this;
}
pub unsafe extern "C" fn cookie_destroy(mut this: COOKIE) -> COOKIE {
    if !this.is_null() {
        free((*this).name as *mut libc::c_void);
        free((*this).value as *mut libc::c_void);
        free((*this).domain as *mut libc::c_void);
        free((*this).expstr as *mut libc::c_void);
        free((*this).path as *mut libc::c_void);
        free((*this).none as *mut libc::c_void);
        free((*this).string as *mut libc::c_void);
        free(this as *mut libc::c_void);
    }
    return 0 as COOKIE;
}
pub unsafe extern "C" fn cookie_set_name(mut this: COOKIE, mut str: *mut libc::c_char) {
    let mut len: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*this)
        .name = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(len),
    ) as *mut libc::c_char;
    memset((*this).name as *mut libc::c_void, '\0' as i32, len);
    memcpy((*this).name as *mut libc::c_void, str as *const libc::c_void, len);
}
pub unsafe extern "C" fn cookie_set_value(mut this: COOKIE, mut str: *mut libc::c_char) {
    let mut len: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*this)
        .value = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(len),
    ) as *mut libc::c_char;
    memset((*this).value as *mut libc::c_void, '\0' as i32, len);
    memcpy((*this).value as *mut libc::c_void, str as *const libc::c_void, len);
}
pub unsafe extern "C" fn cookie_set_path(mut this: COOKIE, mut str: *mut libc::c_char) {
    let mut len: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*this)
        .path = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(len),
    ) as *mut libc::c_char;
    memset((*this).path as *mut libc::c_void, '\0' as i32, len);
    memcpy((*this).path as *mut libc::c_void, str as *const libc::c_void, len);
}
pub unsafe extern "C" fn cookie_set_domain(
    mut this: COOKIE,
    mut str: *mut libc::c_char,
) {
    let mut len: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*this)
        .domain = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(len),
    ) as *mut libc::c_char;
    memset((*this).domain as *mut libc::c_void, '\0' as i32, len);
    memcpy((*this).domain as *mut libc::c_void, str as *const libc::c_void, len);
}
pub unsafe extern "C" fn cookie_set_expires(mut this: COOKIE, mut expires: time_t) {
    (*this).expires = expires;
}
pub unsafe extern "C" fn cookie_get_name(mut this: COOKIE) -> *mut libc::c_char {
    if this.is_null() && ((*this).name).is_null() {
        return (*this).none;
    }
    return (*this).name;
}
pub unsafe extern "C" fn cookie_get_value(mut this: COOKIE) -> *mut libc::c_char {
    if this.is_null() && ((*this).value).is_null() {
        return (*this).none;
    }
    return (*this).value;
}
pub unsafe extern "C" fn cookie_get_domain(mut this: COOKIE) -> *mut libc::c_char {
    if this.is_null() && ((*this).domain).is_null() {
        return (*this).none;
    }
    return (*this).domain;
}
pub unsafe extern "C" fn cookie_get_path(mut this: COOKIE) -> *mut libc::c_char {
    if this.is_null() && ((*this).path).is_null() {
        return (*this).none;
    }
    return (*this).path;
}
pub unsafe extern "C" fn cookie_get_expires(mut this: COOKIE) -> time_t {
    if this.is_null() {
        return -(1 as libc::c_int) as time_t;
    }
    return (*this).expires;
}
pub unsafe extern "C" fn cookie_get_session(mut this: COOKIE) -> BOOLEAN {
    if this.is_null() {
        return boolean_true;
    }
    return (*this).session;
}
pub unsafe extern "C" fn cookie_expires_string(mut this: COOKIE) -> *mut libc::c_char {
    (*this)
        .expstr = realloc(
        (*this).expstr as *mut libc::c_void,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(128 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        (*this).expstr as *mut libc::c_void,
        '\0' as i32,
        128 as libc::c_int as libc::c_ulong,
    );
    let mut timeinfo: *mut tm = 0 as *mut tm;
    timeinfo = localtime(&mut (*this).expires);
    strftime(
        (*this).expstr,
        128 as libc::c_int as size_t,
        b"%a, %d %b %Y %H:%M:%S %z\0" as *const u8 as *const libc::c_char,
        timeinfo,
    );
    return (*this).expstr;
}
pub unsafe extern "C" fn cookie_to_string(mut this: COOKIE) -> *mut libc::c_char {
    let mut len: libc::c_int = 4096 as libc::c_int;
    if ((*this).name).is_null() || ((*this).value).is_null()
        || ((*this).domain).is_null()
    {
        return 0 as *mut libc::c_char;
    }
    (*this)
        .string = realloc(
        (*this).string as *mut libc::c_void,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong),
    ) as *mut libc::c_char;
    memset((*this).string as *mut libc::c_void, '\0' as i32, len as libc::c_ulong);
    snprintf(
        (*this).string,
        len as libc::c_ulong,
        b"%s=%s; domain=%s; path=%s; expires=%lld\0" as *const u8 as *const libc::c_char,
        (*this).name,
        (*this).value,
        if !((*this).domain).is_null() {
            (*this).domain as *const libc::c_char
        } else {
            b"none\0" as *const u8 as *const libc::c_char
        },
        if !((*this).path).is_null() {
            (*this).path as *const libc::c_char
        } else {
            b"/\0" as *const u8 as *const libc::c_char
        },
        (*this).expires as libc::c_longlong,
    );
    return (*this).string;
}
pub unsafe extern "C" fn strealloc(
    mut old: *mut libc::c_char,
    mut str: *mut libc::c_char,
) -> *mut libc::c_void {
    let mut num: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut newptr: *mut libc::c_char = realloc(
        old as *mut libc::c_void,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(num),
    ) as *mut libc::c_char;
    if !newptr.is_null() {
        memset(
            newptr as *mut libc::c_void,
            '\0' as i32,
            num.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(newptr as *mut libc::c_void, str as *const libc::c_void, num);
    }
    return newptr as *mut libc::c_void;
}
pub unsafe extern "C" fn cookie_reset_value(
    mut this: COOKIE,
    mut value: *mut libc::c_char,
) {
    (*this).value = strealloc((*this).value, value) as *mut libc::c_char;
}
pub unsafe extern "C" fn cookie_clone(mut this: COOKIE, mut that: COOKIE) -> COOKIE {
    (*this)
        .value = strealloc((*this).value, cookie_get_value(that)) as *mut libc::c_char;
    (*this)
        .domain = strealloc((*this).domain, cookie_get_domain(that))
        as *mut libc::c_char;
    (*this).path = strealloc((*this).path, cookie_get_path(that)) as *mut libc::c_char;
    if (*this).expires > 0 as libc::c_int as libc::c_long {
        (*this).expires = time(cookie_get_expires(that) as *mut time_t);
    }
    if (*this).session as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        (*this).session = cookie_get_session(that);
    }
    return this;
}
unsafe extern "C" fn __parse_input(
    mut this: COOKIE,
    mut str: *mut libc::c_char,
    mut host: *mut libc::c_char,
) -> BOOLEAN {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expires: libc::c_int = 0 as libc::c_int;
    if str.is_null() {
        printf(
            b"Coookie: Unable to parse header string\0" as *const u8
                as *const libc::c_char,
        );
        return boolean_false;
    }
    while *str as libc::c_int != 0 && *str as libc::c_int == ' ' as i32 {
        str = str.offset(1);
        str;
    }
    let mut newline: *mut libc::c_char = str;
    loop {
        tmp = __parse_pair(&mut newline);
        if tmp.is_null() {
            break;
        }
        key = tmp;
        while *tmp as libc::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            && !('=' as i32 == *tmp as libc::c_int || ':' as i32 == *tmp as libc::c_int)
        {
            tmp = tmp.offset(1);
            tmp;
        }
        let fresh0 = tmp;
        tmp = tmp.offset(1);
        *fresh0 = 0 as libc::c_int as libc::c_char;
        while *(*__ctype_b_loc())
            .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || ('=' as i32 == *tmp as libc::c_int || ':' as i32 == *tmp as libc::c_int)
        {
            tmp = tmp.offset(1);
            tmp;
        }
        val = tmp;
        while *tmp != 0 {
            tmp = tmp.offset(1);
            tmp;
        }
        if strncasecmp(
            key,
            b"expires\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            expires = __parse_time(val);
            if expires != -(1 as libc::c_int) {
                (*this).session = boolean_false;
                (*this).expires = expires as time_t;
            }
        } else if strncasecmp(
            key,
            b"max-age\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            let mut gmt: *mut tm = 0 as *mut tm;
            let mut max: libc::c_long = -(1 as libc::c_int) as libc::c_long;
            let mut now: time_t = time(0 as *mut time_t);
            gmt = gmtime(&mut now);
            now = mktime(gmt);
            max = atof(val) as libc::c_long;
            if max != -(1 as libc::c_int) as libc::c_long {
                (*this).session = boolean_false;
            }
        } else if strncasecmp(
            key,
            b"path\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*this).path = strdup(val);
        } else if strncasecmp(
            key,
            b"domain\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            cookie_set_domain(this, val);
        } else if strncasecmp(
            key,
            b"secure\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*this).secure = boolean_true;
        } else {
            (*this).name = strdup(key);
            (*this).value = strdup(val);
        }
    }
    if (*this).expires < 1000 as libc::c_int as libc::c_long {
        (*this).session = boolean_true;
    }
    if ((*this).domain).is_null() {
        pos = strchr(host, '.' as i32);
        if pos.is_null() {
            (*this).domain = xstrdup(b".\0" as *const u8 as *const libc::c_char);
        } else {
            (*this).domain = xstrdup(pos);
        }
    }
    return boolean_true;
}
unsafe extern "C" fn __parse_pair(mut str: *mut *mut libc::c_char) -> *mut libc::c_char {
    let mut okay: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = *str;
    let mut pair: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() || (*str).is_null() {
        return 0 as *mut libc::c_char;
    }
    pair = p;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != ';' as i32 {
        if *p == 0 {
            *str = p;
            return 0 as *mut libc::c_char;
        }
        if *p as libc::c_int == '=' as i32 {
            okay = 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = '\0' as i32 as libc::c_char;
    *str = p;
    trim(pair);
    if okay != 0 { return pair } else { return 0 as *mut libc::c_char };
}
unsafe extern "C" fn __utc_offset() -> libc::c_int {
    let mut hrs: libc::c_int = 0;
    let mut ptr: *mut tm = 0 as *mut tm;
    let mut zip: time_t = (24 as libc::c_int * 60 as libc::c_int) as libc::c_long
        * 60 as libc::c_long;
    ptr = localtime(&mut zip);
    hrs = (*ptr).tm_hour;
    if (*ptr).tm_mday < 2 as libc::c_int {
        hrs -= 24 as libc::c_int;
    }
    return hrs;
}
unsafe extern "C" fn __parse_time(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut rv: time_t = 0;
    let mut now: time_t = 0;
    if str.is_null() {
        return 0 as libc::c_int;
    }
    s = strchr(str, ',' as i32);
    if !s.is_null() {
        s = s.offset(1);
        s;
        while *s as libc::c_int != 0 && *s as libc::c_int == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        if !(strchr(s, '-' as i32)).is_null() {
            if (strlen(s) as libc::c_int) < 18 as libc::c_int {
                return 0 as libc::c_int;
            }
            tm.tm_mday = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_mon = __mkmonth(s, &mut s);
            s = s.offset(1);
            tm
                .tm_year = (strtol(s, &mut s, 10 as libc::c_int)
                - 1900 as libc::c_int as libc::c_long) as libc::c_int;
            s = s.offset(1);
            tm.tm_hour = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_min = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_sec = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
        } else {
            if (strlen(s) as libc::c_int) < 20 as libc::c_int {
                return 0 as libc::c_int;
            }
            tm.tm_mday = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            tm.tm_mon = __mkmonth(s, &mut s);
            tm
                .tm_year = (strtol(s, &mut s, 10 as libc::c_int)
                - 1900 as libc::c_int as libc::c_long) as libc::c_int;
            tm.tm_hour = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_min = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_sec = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
        }
    } else if *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if !(strchr(str, 'T' as i32)).is_null() {
            s = str as *mut libc::c_char;
            while *s as libc::c_int != 0 && *s as libc::c_int == ' ' as i32 {
                s = s.offset(1);
                s;
            }
            if (strlen(s) as libc::c_int) < 21 as libc::c_int {
                return 0 as libc::c_int;
            }
            tm
                .tm_year = (strtol(s, &mut s, 10 as libc::c_int)
                - 1900 as libc::c_int as libc::c_long) as libc::c_int;
            s = s.offset(1);
            tm.tm_mon = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_mday = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_hour = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_min = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
            s = s.offset(1);
            tm.tm_sec = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
        } else {
            return atol(str) as libc::c_int
        }
    } else {
        s = str as *mut libc::c_char;
        while *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32 {
            s = s.offset(1);
            s;
        }
        if (strlen(s) as libc::c_int) < 20 as libc::c_int {
            return 0 as libc::c_int;
        }
        tm.tm_mon = __mkmonth(s, &mut s);
        tm.tm_mday = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
        tm.tm_hour = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
        s = s.offset(1);
        tm.tm_min = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
        s = s.offset(1);
        tm.tm_sec = strtol(s, &mut s, 10 as libc::c_int) as libc::c_int;
        tm
            .tm_year = (strtol(s, &mut s, 10 as libc::c_int)
            - 1900 as libc::c_int as libc::c_long) as libc::c_int;
    }
    if tm.tm_sec < 0 as libc::c_int || tm.tm_sec > 59 as libc::c_int
        || tm.tm_min < 0 as libc::c_int || tm.tm_min > 59 as libc::c_int
        || tm.tm_hour < 0 as libc::c_int || tm.tm_hour > 23 as libc::c_int
        || tm.tm_mday < 1 as libc::c_int || tm.tm_mday > 31 as libc::c_int
        || tm.tm_mon < 0 as libc::c_int || tm.tm_mon > 11 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    tm.tm_isdst = -(1 as libc::c_int);
    rv = mktime(&mut tm);
    if (strstr(str, b" GMT\0" as *const u8 as *const libc::c_char)).is_null()
        && (strstr(str, b" UTC\0" as *const u8 as *const libc::c_char)).is_null()
    {
        rv += (__utc_offset() * 3600 as libc::c_int) as libc::c_long;
    }
    if rv == -(1 as libc::c_int) as libc::c_long {
        return rv as libc::c_int;
    }
    now = time(0 as *mut time_t);
    if rv - now < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    return rv as libc::c_int;
}
unsafe extern "C" fn __mkmonth(
    mut s: *mut libc::c_char,
    mut ends: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ptr: *mut libc::c_char = s;
    while *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    if *ptr != 0 {
        let mut i: libc::c_int = 0;
        *ends = ptr.offset(3 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < 12 as libc::c_int {
            if strncasecmp(months[i as usize], ptr, 3 as libc::c_int as libc::c_ulong)
                == 0
            {
                return i;
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
