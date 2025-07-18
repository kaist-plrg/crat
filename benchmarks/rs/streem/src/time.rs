use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_ns_new(_: *mut strm_state, _: *const libc::c_char) -> *mut strm_state;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
    fn strm_float_value(_: libc::c_double) -> strm_value;
    fn strm_nil_value() -> strm_value;
    fn strm_value_int(_: strm_value) -> strm_int;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_value_ptr(_: strm_value, _: strm_ptr_type) -> *mut libc::c_void;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_strp_ptr(_: *mut strm_string) -> *const libc::c_char;
    fn strm_str_len(_: strm_string) -> strm_int;
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn strptime(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
    ) -> *mut libc::c_char;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __intptr_t = libc::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
pub type intptr_t = __intptr_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value_tag = libc::c_ulong;
pub const STRM_TAG_FOREIGN: strm_value_tag = 18446462598732840960;
pub const STRM_TAG_PTR: strm_value_tag = 18445899648779419648;
pub const STRM_TAG_CFUNC: strm_value_tag = 18445336698825998336;
pub const STRM_TAG_STRING_F: strm_value_tag = 18445055223849287680;
pub const STRM_TAG_STRING_O: strm_value_tag = 18444773748872577024;
pub const STRM_TAG_STRING_6: strm_value_tag = 18444492273895866368;
pub const STRM_TAG_STRING_I: strm_value_tag = 18444210798919155712;
pub const STRM_TAG_STRUCT: strm_value_tag = 18443647848965734400;
pub const STRM_TAG_ARRAY: strm_value_tag = 18443366373989023744;
pub const STRM_TAG_LIST: strm_value_tag = 18443084899012313088;
pub const STRM_TAG_INT: strm_value_tag = 18442803424035602432;
pub const STRM_TAG_BOOL: strm_value_tag = 18442521949058891776;
pub const STRM_TAG_NAN: strm_value_tag = 18442240474082181120;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_cfunc = Option::<
    unsafe extern "C" fn(
        *mut strm_stream,
        libc::c_int,
        *mut strm_value,
        *mut strm_value,
    ) -> libc::c_int,
>;
pub type strm_string = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_misc {
    pub type_0: strm_ptr_type,
    pub ns: *mut strm_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_time {
    pub type_0: strm_ptr_type,
    pub ns: *mut strm_state,
    pub tv: timeval,
    pub utc_offset: libc::c_int,
}
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
pub const TZ_FAIL: time_tz = 60000;
pub const TZ_NONE: time_tz = 50000;
pub type time_tz = libc::c_uint;
pub const TZ_UTC: time_tz = 0;
static mut ns_time: *mut strm_state = 0 as *const strm_state as *mut strm_state;
pub unsafe extern "C" fn strm_time_p(mut val: strm_value) -> libc::c_int {
    if val
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        != STRM_TAG_PTR as libc::c_ulong
    {
        return 0 as libc::c_int
    } else {
        let mut p: *mut strm_misc = (val
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int)) as intptr_t as *mut libc::c_void
            as *mut strm_misc;
        if p.is_null() {
            return 0 as libc::c_int;
        }
        if (*p).type_0 as libc::c_uint != STRM_PTR_AUX as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        if (*p).ns != ns_time {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn num_to_timeval(mut d: libc::c_double, mut tv: *mut timeval) {
    let mut sec: libc::c_double = floor(d);
    (*tv).tv_sec = sec as __time_t;
    (*tv)
        .tv_usec = ((d - sec) * 1000000 as libc::c_int as libc::c_double)
        as __suseconds_t;
}
pub unsafe extern "C" fn timeval_to_num(mut tv: *mut timeval) -> libc::c_double {
    let mut d: libc::c_double = (*tv).tv_sec as libc::c_double;
    return d + (*tv).tv_usec as libc::c_double / 1000000.0f64;
}
unsafe extern "C" fn time_localoffset(mut force: libc::c_int) -> libc::c_int {
    static mut localoffset: libc::c_int = 1 as libc::c_int;
    if force != 0 || localoffset == 1 as libc::c_int {
        let mut now: time_t = 0;
        let mut gm: tm = tm {
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
        now = time(0 as *mut time_t);
        gmtime_r(&mut now, &mut gm);
        localoffset = difftime(now, mktime(&mut gm)) as libc::c_int;
    }
    return localoffset;
}
unsafe extern "C" fn time_alloc(
    mut tv: *mut timeval,
    mut utc_offset: libc::c_int,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t: *mut strm_time = malloc(
        ::std::mem::size_of::<strm_time>() as libc::c_ulong,
    ) as *mut strm_time;
    if t.is_null() {
        return 1 as libc::c_int;
    }
    (*t).type_0 = STRM_PTR_AUX;
    (*t).ns = ns_time;
    (*t).tv = *tv;
    (*t).utc_offset = utc_offset;
    *ret = strm_ptr_value(t as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn scan_digit(c: libc::c_char) -> libc::c_int {
    if '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32 {
        return c as libc::c_int - '0' as i32
    } else {
        return TZ_FAIL as libc::c_int
    };
}
unsafe extern "C" fn scan_num(
    mut sp: *mut *const libc::c_char,
    mut send: *const libc::c_char,
) -> libc::c_int {
    let mut s: *const libc::c_char = *sp;
    let mut n: libc::c_int = 0 as libc::c_int;
    while s < send {
        let mut i: libc::c_int = scan_digit(*s);
        if i > 9 as libc::c_int {
            if s == *sp {
                return -(1 as libc::c_int);
            }
            *sp = s;
            return n;
        }
        s = s.offset(1);
        s;
        n = 10 as libc::c_int * n + i;
    }
    *sp = s;
    return n;
}
unsafe extern "C" fn parse_tz(
    mut s: *const libc::c_char,
    mut len: strm_int,
) -> libc::c_int {
    let mut h: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut send: *const libc::c_char = s.offset(len as isize);
    match *s.offset(0 as libc::c_int as isize) as libc::c_int {
        90 => return 0 as libc::c_int,
        43 | 45 => {
            let fresh0 = s;
            s = s.offset(1);
            c = *fresh0;
            h = scan_num(&mut s, send);
            if h < 0 as libc::c_int {
                return TZ_FAIL as libc::c_int;
            }
            if *s as libc::c_int == ':' as i32 {
                s = s.offset(1);
                s;
                m = scan_num(&mut s, send);
            } else if h >= 100 as libc::c_int {
                let mut i: libc::c_int = h;
                h = i / 100 as libc::c_int;
                m = i % 100 as libc::c_int;
            } else {
                m = 0 as libc::c_int;
            }
            if h > 14 as libc::c_int {
                return TZ_FAIL as libc::c_int;
            }
            if m > 59 as libc::c_int {
                return TZ_FAIL as libc::c_int;
            }
            return (h * 60 as libc::c_int + m)
                * (if c as libc::c_int == '-' as i32 {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                }) * 60 as libc::c_int;
        }
        _ => return TZ_FAIL as libc::c_int,
    };
}
pub unsafe extern "C" fn strm_time_parse_time(
    mut p: *const libc::c_char,
    mut len: strm_int,
    mut sec: *mut libc::c_long,
    mut usec: *mut libc::c_long,
    mut offset: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut s: *const libc::c_char = p;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut t2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tend: *const libc::c_char = 0 as *const libc::c_char;
    let mut tm: tm = {
        let mut init = tm {
            tm_sec: 0 as libc::c_int,
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
        init
    };
    let mut localoffset: libc::c_int = time_localoffset(1 as libc::c_int);
    let mut tt: time_t = 0;
    if *s.offset(len as isize) as libc::c_int != '\0' as i32 {
        let mut pp: *mut libc::c_char = malloc((len + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        memcpy(pp as *mut libc::c_void, p as *const libc::c_void, len as libc::c_ulong);
        *pp.offset(len as isize) = '\0' as i32 as libc::c_char;
        s = pp as *const libc::c_char;
    }
    tend = s.offset(len as isize);
    *usec = 0 as libc::c_int as libc::c_long;
    t = strptime(s, b"%Y.%m.%d\0" as *const u8 as *const libc::c_char, &mut tm);
    if t.is_null() {
        t = strptime(s, b"%Y-%m-%d\0" as *const u8 as *const libc::c_char, &mut tm);
    }
    if t.is_null() {
        t = strptime(s, b"%Y/%m/%d\0" as *const u8 as *const libc::c_char, &mut tm);
    }
    if t.is_null() {
        t = strptime(s, b"%Y%m%d\0" as *const u8 as *const libc::c_char, &mut tm);
        if !t.is_null()
            && !(*t.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32
                || *t.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32)
        {
            t = 0 as *const libc::c_char;
        }
    }
    if t.is_null() {
        t = strptime(s, b"%b %d %Y\0" as *const u8 as *const libc::c_char, &mut tm);
        if !t.is_null()
            && *t.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
        {
            t = 0 as *const libc::c_char;
        }
    }
    if t.is_null() {
        let mut tm2: tm = tm {
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
        tt = time(0 as *mut time_t);
        localtime_r(&mut tt, &mut tm2);
        tm.tm_year = tm2.tm_year;
        t = strptime(s, b"%b %d\0" as *const u8 as *const libc::c_char, &mut tm);
    }
    if !t.is_null() {
        if t == tend {
            tt = mktime(&mut tm);
            tt += localoffset as libc::c_long;
            *sec = tt;
            *usec = 0 as libc::c_int as libc::c_long;
            *offset = TZ_NONE as libc::c_int;
            current_block = 13754873135133134502;
        } else {
            let fresh1 = t;
            t = t.offset(1);
            match *fresh1 as libc::c_int {
                84 => {
                    current_block = 15597372965620363352;
                    match current_block {
                        14718330381335561043 => {
                            while *t as libc::c_int == ' ' as i32 {
                                t = t.offset(1);
                                t;
                            }
                        }
                        _ => {}
                    }
                    t2 = strptime(
                        t,
                        b"%H:%M:%S\0" as *const u8 as *const libc::c_char,
                        &mut tm,
                    );
                    if t2.is_null() {
                        t2 = strptime(
                            t,
                            b"%H%M%S\0" as *const u8 as *const libc::c_char,
                            &mut tm,
                        );
                    }
                    if t2.is_null() {
                        t2 = strptime(
                            t,
                            b"%H:%M\0" as *const u8 as *const libc::c_char,
                            &mut tm,
                        );
                        if t2.is_null() {
                            current_block = 5105096235594990183;
                        } else {
                            current_block = 790185930182612747;
                        }
                    } else {
                        current_block = 790185930182612747;
                    }
                    match current_block {
                        5105096235594990183 => {}
                        _ => {
                            t = t2;
                            tt = mktime(&mut tm);
                            if *t.offset(0 as libc::c_int as isize) as libc::c_int
                                == '.' as i32
                            {
                                t = t.offset(1);
                                t;
                                let mut frac: libc::c_long = scan_num(&mut t, tend)
                                    as libc::c_long;
                                if frac < 0 as libc::c_int as libc::c_long {
                                    current_block = 5105096235594990183;
                                } else {
                                    if frac > 0 as libc::c_int as libc::c_long {
                                        let mut d: libc::c_double = ceil(
                                            log10(frac as libc::c_double),
                                        );
                                        d = frac as libc::c_double / pow(10.0f64, d);
                                        *usec = (d * 1000000 as libc::c_int as libc::c_double)
                                            as libc::c_long;
                                    }
                                    current_block = 10095721787123848864;
                                }
                            } else {
                                current_block = 10095721787123848864;
                            }
                            match current_block {
                                5105096235594990183 => {}
                                _ => {
                                    if *t.offset(0 as libc::c_int as isize) as libc::c_int
                                        == 'Z' as i32
                                    {
                                        tt += localoffset as libc::c_long;
                                        *offset = 0 as libc::c_int;
                                        current_block = 13125627826496529465;
                                    } else if t == tend {
                                        *offset = localoffset;
                                        current_block = 13125627826496529465;
                                    } else {
                                        let mut n: libc::c_int = 0;
                                        match *t.offset(0 as libc::c_int as isize) as libc::c_int {
                                            43 | 45 => {
                                                n = parse_tz(
                                                    t,
                                                    tend.offset_from(t) as libc::c_long as strm_int,
                                                );
                                                if n == TZ_FAIL as libc::c_int {
                                                    current_block = 5105096235594990183;
                                                } else {
                                                    tt += localoffset as libc::c_long;
                                                    tt -= n as libc::c_long;
                                                    *offset = n;
                                                    current_block = 13125627826496529465;
                                                }
                                            }
                                            _ => {
                                                current_block = 5105096235594990183;
                                            }
                                        }
                                    }
                                    match current_block {
                                        5105096235594990183 => {}
                                        _ => {
                                            *sec = tt;
                                            current_block = 13754873135133134502;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                32 => {
                    current_block = 14718330381335561043;
                    match current_block {
                        14718330381335561043 => {
                            while *t as libc::c_int == ' ' as i32 {
                                t = t.offset(1);
                                t;
                            }
                        }
                        _ => {}
                    }
                    t2 = strptime(
                        t,
                        b"%H:%M:%S\0" as *const u8 as *const libc::c_char,
                        &mut tm,
                    );
                    if t2.is_null() {
                        t2 = strptime(
                            t,
                            b"%H%M%S\0" as *const u8 as *const libc::c_char,
                            &mut tm,
                        );
                    }
                    if t2.is_null() {
                        t2 = strptime(
                            t,
                            b"%H:%M\0" as *const u8 as *const libc::c_char,
                            &mut tm,
                        );
                        if t2.is_null() {
                            current_block = 5105096235594990183;
                        } else {
                            current_block = 790185930182612747;
                        }
                    } else {
                        current_block = 790185930182612747;
                    }
                    match current_block {
                        5105096235594990183 => {}
                        _ => {
                            t = t2;
                            tt = mktime(&mut tm);
                            if *t.offset(0 as libc::c_int as isize) as libc::c_int
                                == '.' as i32
                            {
                                t = t.offset(1);
                                t;
                                let mut frac: libc::c_long = scan_num(&mut t, tend)
                                    as libc::c_long;
                                if frac < 0 as libc::c_int as libc::c_long {
                                    current_block = 5105096235594990183;
                                } else {
                                    if frac > 0 as libc::c_int as libc::c_long {
                                        let mut d: libc::c_double = ceil(
                                            log10(frac as libc::c_double),
                                        );
                                        d = frac as libc::c_double / pow(10.0f64, d);
                                        *usec = (d * 1000000 as libc::c_int as libc::c_double)
                                            as libc::c_long;
                                    }
                                    current_block = 10095721787123848864;
                                }
                            } else {
                                current_block = 10095721787123848864;
                            }
                            match current_block {
                                5105096235594990183 => {}
                                _ => {
                                    if *t.offset(0 as libc::c_int as isize) as libc::c_int
                                        == 'Z' as i32
                                    {
                                        tt += localoffset as libc::c_long;
                                        *offset = 0 as libc::c_int;
                                        current_block = 13125627826496529465;
                                    } else if t == tend {
                                        *offset = localoffset;
                                        current_block = 13125627826496529465;
                                    } else {
                                        let mut n: libc::c_int = 0;
                                        match *t.offset(0 as libc::c_int as isize) as libc::c_int {
                                            43 | 45 => {
                                                n = parse_tz(
                                                    t,
                                                    tend.offset_from(t) as libc::c_long as strm_int,
                                                );
                                                if n == TZ_FAIL as libc::c_int {
                                                    current_block = 5105096235594990183;
                                                } else {
                                                    tt += localoffset as libc::c_long;
                                                    tt -= n as libc::c_long;
                                                    *offset = n;
                                                    current_block = 13125627826496529465;
                                                }
                                            }
                                            _ => {
                                                current_block = 5105096235594990183;
                                            }
                                        }
                                    }
                                    match current_block {
                                        5105096235594990183 => {}
                                        _ => {
                                            *sec = tt;
                                            current_block = 13754873135133134502;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    current_block = 5105096235594990183;
                }
            }
        }
        match current_block {
            5105096235594990183 => {}
            _ => {
                if s != p {
                    free(s as *mut libc::c_char as *mut libc::c_void);
                }
                return 0 as libc::c_int;
            }
        }
    }
    if s != p {
        free(s as *mut libc::c_char as *mut libc::c_void);
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn strm_time_new(
    mut sec: libc::c_long,
    mut usec: libc::c_long,
    mut offset: libc::c_int,
) -> strm_value {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut v: strm_value = 0;
    tv.tv_sec = sec;
    tv.tv_usec = usec;
    if time_alloc(&mut tv, offset, &mut v) == 1 as libc::c_int {
        return strm_nil_value();
    }
    return v;
}
unsafe extern "C" fn time_time(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    let mut tm: tm = {
        let mut init = tm {
            tm_sec: 0 as libc::c_int,
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
        init
    };
    let mut t: time_t = 0;
    let mut utc_offset: libc::c_int = 0 as libc::c_int;
    let mut current_block_42: u64;
    match argc {
        1 => {
            let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut slen: libc::c_int = 0;
            let mut sec: libc::c_long = 0;
            let mut usec: libc::c_long = 0;
            if strm_parse_args(
                strm,
                argc,
                args,
                b"s\0" as *const u8 as *const libc::c_char,
                &mut s as *mut *mut libc::c_char,
                &mut slen as *mut libc::c_int,
            ) == 1 as libc::c_int
            {
                return 1 as libc::c_int;
            }
            if strm_time_parse_time(s, slen, &mut sec, &mut usec, &mut utc_offset)
                < 0 as libc::c_int
            {
                strm_raise(
                    strm,
                    b"bad time format\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            tv.tv_sec = sec;
            tv.tv_usec = usec;
            return time_alloc(&mut tv, utc_offset, ret);
        }
        3 => {
            tm.tm_year = strm_value_int(*args.offset(0 as libc::c_int as isize));
            tm
                .tm_mon = strm_value_int(*args.offset(1 as libc::c_int as isize))
                - 1 as libc::c_int;
            tm.tm_mday = strm_value_int(*args.offset(2 as libc::c_int as isize));
            tv.tv_sec = mktime(&mut tm);
            tv.tv_sec += time_localoffset(1 as libc::c_int) as libc::c_long;
            utc_offset = TZ_NONE as libc::c_int;
            return time_alloc(&mut tv, utc_offset, ret);
        }
        8 => {
            let mut str: strm_string = *args.offset(7 as libc::c_int as isize);
            utc_offset = parse_tz(strm_strp_ptr(&mut str), strm_str_len(str));
            if utc_offset == TZ_FAIL as libc::c_int {
                strm_raise(
                    strm,
                    b"wrong timezeone\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            current_block_42 = 5425707239587141579;
        }
        7 => {
            current_block_42 = 5425707239587141579;
        }
        6 => {
            current_block_42 = 7557068031655775931;
        }
        5 => {
            current_block_42 = 1584198394369818069;
        }
        4 => {
            current_block_42 = 3850642056257311267;
        }
        _ => {
            strm_raise(
                strm,
                b"wrong # of arguments\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    match current_block_42 {
        5425707239587141579 => {
            tv
                .tv_usec = (strm_value_int(*args.offset(6 as libc::c_int as isize))
                / 1000 as libc::c_int) as __suseconds_t;
            current_block_42 = 7557068031655775931;
        }
        _ => {}
    }
    match current_block_42 {
        7557068031655775931 => {
            tm.tm_sec = strm_value_int(*args.offset(5 as libc::c_int as isize));
            current_block_42 = 1584198394369818069;
        }
        _ => {}
    }
    match current_block_42 {
        1584198394369818069 => {
            tm.tm_min = strm_value_int(*args.offset(4 as libc::c_int as isize));
        }
        _ => {}
    }
    tm.tm_year = strm_value_int(*args.offset(0 as libc::c_int as isize));
    tm.tm_mon = strm_value_int(*args.offset(1 as libc::c_int as isize));
    tm.tm_mday = strm_value_int(*args.offset(2 as libc::c_int as isize));
    tm.tm_hour = strm_value_int(*args.offset(3 as libc::c_int as isize));
    t = mktime(&mut tm);
    tv.tv_sec = t;
    if argc == 8 as libc::c_int {
        tv.tv_sec += time_localoffset(1 as libc::c_int) as libc::c_long;
        tv.tv_sec -= utc_offset as libc::c_long;
    } else {
        utc_offset = time_localoffset(0 as libc::c_int);
    }
    return time_alloc(&mut tv, utc_offset, ret);
}
unsafe extern "C" fn time_now(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut utc_offset: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        utc_offset = time_localoffset(0 as libc::c_int);
    } else {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut slen: strm_int = 0;
        if strm_parse_args(
            strm,
            argc,
            args,
            b"s\0" as *const u8 as *const libc::c_char,
            &mut s as *mut *mut libc::c_char,
            &mut slen as *mut strm_int,
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        utc_offset = parse_tz(s, slen);
        if utc_offset == TZ_FAIL as libc::c_int {
            strm_raise(strm, b"wrong timezeone\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        }
    }
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    return time_alloc(&mut tv, utc_offset, ret);
}
unsafe extern "C" fn get_time(mut val: strm_value) -> *mut strm_time {
    let mut t: *mut strm_time = strm_value_ptr(val, STRM_PTR_AUX) as *mut strm_time;
    if (*t).ns != ns_time {
        return 0 as *mut strm_time;
    }
    return t;
}
unsafe extern "C" fn time_plus(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t1: *mut strm_time = 0 as *mut strm_time;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tv2: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if argc != 2 as libc::c_int {
        strm_raise(strm, b"wrong # of arguments\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    t1 = get_time(*args.offset(0 as libc::c_int as isize));
    if strm_number_p(*args.offset(1 as libc::c_int as isize)) == 0 {
        strm_raise(strm, b"number required\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    num_to_timeval(strm_value_float(*args.offset(1 as libc::c_int as isize)), &mut tv);
    tv2.tv_sec = (*t1).tv.tv_sec + tv.tv_sec;
    tv2.tv_usec = (*t1).tv.tv_usec + tv.tv_usec;
    while tv2.tv_usec >= 1000000 as libc::c_int as libc::c_long {
        tv2.tv_sec += 1;
        tv2.tv_sec;
        tv2.tv_usec -= 1000000 as libc::c_int as libc::c_long;
    }
    return time_alloc(&mut tv2, (*t1).utc_offset, ret);
}
unsafe extern "C" fn time_minus(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t1: *mut strm_time = 0 as *mut strm_time;
    let mut t2: *mut strm_time = 0 as *mut strm_time;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut d: libc::c_double = 0.;
    if argc != 2 as libc::c_int {
        strm_raise(strm, b"wrong # of arguments\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if strm_number_p(*args.offset(1 as libc::c_int as isize)) != 0 {
        d = strm_value_float(*args.offset(1 as libc::c_int as isize));
        *args.offset(1 as libc::c_int as isize) = strm_float_value(-d);
        return time_plus(strm, argc, args, ret);
    }
    t1 = get_time(*args.offset(0 as libc::c_int as isize));
    t2 = get_time(*args.offset(1 as libc::c_int as isize));
    tv.tv_sec = (*t1).tv.tv_sec - (*t2).tv.tv_sec;
    tv.tv_usec = (*t1).tv.tv_usec - (*t2).tv.tv_usec;
    while tv.tv_usec < 0 as libc::c_int as libc::c_long {
        tv.tv_sec -= 1;
        tv.tv_sec;
        tv.tv_usec += 1000000 as libc::c_int as libc::c_long;
    }
    d = timeval_to_num(&mut tv);
    *ret = strm_float_value(d);
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_tm(
    mut t: time_t,
    mut utc_offset: libc::c_int,
    mut tm: *mut tm,
) {
    t += utc_offset as libc::c_long;
    gmtime_r(&mut t, tm);
}
unsafe extern "C" fn time_str(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut utc_offset: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    if argc == 1 as libc::c_int {
        t = get_time(*args.offset(0 as libc::c_int as isize));
        utc_offset = (*t).utc_offset;
    } else {
        let mut time_0: strm_value = 0;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut slen: strm_int = 0;
        if strm_parse_args(
            strm,
            argc,
            args,
            b"v|s\0" as *const u8 as *const libc::c_char,
            &mut time_0 as *mut strm_value,
            &mut s as *mut *mut libc::c_char,
            &mut slen as *mut strm_int,
        ) == 1 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        t = get_time(time_0);
        utc_offset = parse_tz(s, slen);
        if utc_offset == TZ_FAIL as libc::c_int {
            strm_raise(strm, b"wrong timezeone\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        }
    }
    if utc_offset == TZ_NONE as libc::c_int {
        get_tm((*t).tv.tv_sec, 0 as libc::c_int, &mut tm);
        if tm.tm_hour == 0 as libc::c_int && tm.tm_min == 0 as libc::c_int
            && tm.tm_sec == 0 as libc::c_int
        {
            n = strftime(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%Y.%m.%d\0" as *const u8 as *const libc::c_char,
                &mut tm,
            );
            *ret = strm_str_new(buf.as_mut_ptr(), n as strm_int);
            return 0 as libc::c_int;
        }
        utc_offset = 0 as libc::c_int;
    }
    get_tm((*t).tv.tv_sec, utc_offset, &mut tm);
    n = strftime(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%Y.%m.%dT%H:%M:%S\0" as *const u8 as *const libc::c_char,
        &mut tm,
    );
    p = buf.as_mut_ptr().offset(n as isize);
    if (*t).tv.tv_usec != 0 as libc::c_int as libc::c_long {
        let mut sbuf: [libc::c_char; 20] = [0; 20];
        let mut d: libc::c_double = (*t).tv.tv_usec as libc::c_double / 1000000.0f64;
        let mut t_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        snprintf(
            sbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            b"%.3f\0" as *const u8 as *const libc::c_char,
            d,
        );
        t_0 = sbuf.as_mut_ptr().offset(1 as libc::c_int as isize);
        len = strlen(t_0);
        strncpy(p, t_0, len);
        p = p.offset(len as isize);
    }
    if utc_offset == 0 as libc::c_int {
        *p.offset(0 as libc::c_int as isize) = 'Z' as i32 as libc::c_char;
        *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        let mut off: libc::c_int = utc_offset / 60 as libc::c_int;
        let mut sign: libc::c_char = (if off > 0 as libc::c_int {
            '+' as i32
        } else {
            '-' as i32
        }) as libc::c_char;
        if off < 0 as libc::c_int {
            off = -off;
        }
        snprintf(
            p,
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(
                    p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong,
                ),
            b"%c%02d:%02d\0" as *const u8 as *const libc::c_char,
            sign as libc::c_int,
            off / 60 as libc::c_int,
            off % 60 as libc::c_int,
        );
    }
    *ret = strm_str_new(buf.as_mut_ptr(), strlen(buf.as_mut_ptr()) as strm_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_num(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    if (*t).tv.tv_usec == 0 as libc::c_int as libc::c_long {
        *ret = strm_int_value((*t).tv.tv_sec as strm_int);
    } else {
        *ret = strm_float_value(timeval_to_num(&mut (*t).tv));
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_year(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    get_tm((*t).tv.tv_sec, (*t).utc_offset, &mut tm);
    *ret = strm_int_value(tm.tm_year + 1900 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_month(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    get_tm((*t).tv.tv_sec, (*t).utc_offset, &mut tm);
    *ret = strm_int_value(tm.tm_mon + 1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_day(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    get_tm((*t).tv.tv_sec, (*t).utc_offset, &mut tm);
    *ret = strm_int_value(tm.tm_mday);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_hour(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    get_tm((*t).tv.tv_sec, (*t).utc_offset, &mut tm);
    *ret = strm_int_value(tm.tm_hour);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_min(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    get_tm((*t).tv.tv_sec, (*t).utc_offset, &mut tm);
    *ret = strm_int_value(tm.tm_min);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_sec(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    get_tm((*t).tv.tv_sec, (*t).utc_offset, &mut tm);
    *ret = strm_int_value(tm.tm_sec);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_weekday(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    get_tm((*t).tv.tv_sec, (*t).utc_offset, &mut tm);
    *ret = strm_int_value(tm.tm_wday);
    return 0 as libc::c_int;
}
unsafe extern "C" fn time_nanosec(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut time_0: strm_value = 0;
    let mut t: *mut strm_time = 0 as *mut strm_time;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut time_0 as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    t = get_time(time_0);
    *ret = strm_int_value(
        ((*t).tv.tv_usec * 1000 as libc::c_int as libc::c_long) as strm_int,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_time_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"now\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_now
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"time\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_time
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    ns_time = strm_ns_new(
        0 as *mut strm_state,
        b"time\0" as *const u8 as *const libc::c_char,
    );
    strm_var_def(
        ns_time,
        b"+\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_plus
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"-\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_minus
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"string\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_str
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"number\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_num
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"year\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_year
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"month\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_month
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"day\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_day
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"hour\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_hour
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"minute\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_min
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"second\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_sec
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"nanosecond\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_nanosec
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        ns_time,
        b"weekday\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                time_weekday
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
}
