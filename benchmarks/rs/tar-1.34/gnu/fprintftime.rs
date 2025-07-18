use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn __errno_location() -> *mut libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn mktime_z(__tz: timezone_t, __result: *mut tm) -> time_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm_zone {
    pub next: *mut tm_zone,
    pub tz_is_set: libc::c_char,
    pub abbrs: [libc::c_char; 0],
}
pub type timezone_t = *mut tm_zone;
pub unsafe extern "C" fn fprintftime(
    mut s: *mut FILE,
    mut format: *const libc::c_char,
    mut tp: *const tm,
    mut tz: timezone_t,
    mut ns: libc::c_int,
) -> size_t {
    let mut tzset_called: bool = 0 as libc::c_int != 0;
    return __strftime_internal(
        s,
        format,
        tp,
        0 as libc::c_int != 0,
        0 as libc::c_int,
        -(1 as libc::c_int),
        &mut tzset_called,
        tz,
        ns,
    );
}
unsafe extern "C" fn fwrite_uppcase(
    mut fp: *mut FILE,
    mut src: *const libc::c_char,
    mut len: size_t,
) {
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        fputc(
            {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *src as libc::c_uchar as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(*src as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*src as libc::c_uchar as libc::c_int as isize);
                }
                __res
            },
            fp,
        );
        src = src.offset(1);
        src;
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn fwrite_lowcase(
    mut fp: *mut FILE,
    mut src: *const libc::c_char,
    mut len: size_t,
) {
    loop {
        let fresh1 = len;
        len = len.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        fputc(
            {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *src as libc::c_uchar as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(*src as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*src as libc::c_uchar as libc::c_int as isize);
                }
                __res
            },
            fp,
        );
        src = src.offset(1);
        src;
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn iso_week_days(
    mut yday: libc::c_int,
    mut wday: libc::c_int,
) -> libc::c_int {
    let mut big_enough_multiple_of_7: libc::c_int = (--(366 as libc::c_int)
        / 7 as libc::c_int + 2 as libc::c_int) * 7 as libc::c_int;
    return yday
        - (yday - wday + 4 as libc::c_int + big_enough_multiple_of_7) % 7 as libc::c_int
        + 4 as libc::c_int - 1 as libc::c_int;
}
unsafe extern "C" fn __strftime_internal(
    mut s: *mut FILE,
    mut format: *const libc::c_char,
    mut tp: *const tm,
    mut upcase: bool,
    mut yr_spec: libc::c_int,
    mut width: libc::c_int,
    mut tzset_called: *mut bool,
    mut tz: timezone_t,
    mut ns: libc::c_int,
) -> size_t {
    let mut current_block: u64;
    let mut maxsize: size_t = -(1 as libc::c_int) as size_t;
    let mut saved_errno: libc::c_int = *__errno_location();
    let mut hour12: libc::c_int = (*tp).tm_hour;
    let mut zone: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut FILE = s;
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    zone = 0 as *const libc::c_char;
    zone = (*tp).tm_zone;
    if zone.is_null() {
        zone = b"\0" as *const u8 as *const libc::c_char;
    }
    if hour12 > 12 as libc::c_int {
        hour12 -= 12 as libc::c_int;
    } else if hour12 == 0 as libc::c_int {
        hour12 = 12 as libc::c_int;
    }
    f = format;
    while *f as libc::c_int != '\0' as i32 {
        let mut pad: libc::c_int = 0 as libc::c_int;
        let mut modifier: libc::c_int = 0;
        let mut digits: libc::c_int = 0 as libc::c_int;
        let mut number_value: libc::c_int = 0;
        let mut u_number_value: libc::c_uint = 0;
        let mut negative_number: bool = false;
        let mut always_output_a_sign: bool = false;
        let mut tz_colon_mask: libc::c_int = 0;
        let mut subfmt: *const libc::c_char = 0 as *const libc::c_char;
        let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: [libc::c_char; 23] = [0; 23];
        let mut to_lowcase: bool = 0 as libc::c_int != 0;
        let mut to_uppcase: bool = upcase;
        let mut colons: size_t = 0;
        let mut change_case: bool = 0 as libc::c_int != 0;
        let mut format_char: libc::c_int = 0;
        let mut subwidth: libc::c_int = 0;
        if *f as libc::c_int != '%' as i32 {
            let mut _n: size_t = 1 as libc::c_int as size_t;
            let mut _w: size_t = (if pad == '-' as i32 || width < 0 as libc::c_int {
                0 as libc::c_int
            } else {
                width
            }) as size_t;
            let mut _incr: size_t = if _n < _w { _w } else { _n };
            if _incr >= maxsize.wrapping_sub(i) {
                *__errno_location() = 34 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
            if !p.is_null() {
                if _n < _w {
                    let mut _delta: size_t = _w.wrapping_sub(_n);
                    if pad == '0' as i32 || pad == '+' as i32 {
                        let mut _i: size_t = 0;
                        _i = 0 as libc::c_int as size_t;
                        while _i < _delta {
                            fputc('0' as i32, p);
                            _i = _i.wrapping_add(1);
                            _i;
                        }
                    } else {
                        let mut _i_0: size_t = 0;
                        _i_0 = 0 as libc::c_int as size_t;
                        while _i_0 < _delta {
                            fputc(' ' as i32, p);
                            _i_0 = _i_0.wrapping_add(1);
                            _i_0;
                        }
                    }
                }
                fputc(*f as libc::c_int, p);
            }
            i = (i as libc::c_ulong).wrapping_add(_incr) as size_t as size_t;
        } else {
            loop {
                f = f.offset(1);
                match *f as libc::c_int {
                    95 | 45 | 43 | 48 => {
                        pad = *f as libc::c_int;
                    }
                    94 => {
                        to_uppcase = 1 as libc::c_int != 0;
                    }
                    35 => {
                        change_case = 1 as libc::c_int != 0;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if (*f as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                width = 0 as libc::c_int;
                loop {
                    if (if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
                    {
                        if !((0 as libc::c_int) < -(1 as libc::c_int)) {
                            if (if (10 as libc::c_int) < 0 as libc::c_int {
                                if width < 0 as libc::c_int {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            127 as libc::c_int
                                        }) + 10 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        (width < 127 as libc::c_int / 10 as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        ((if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            ((10 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                        }) != 0
                                        {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 127 as libc::c_int
                                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            127 as libc::c_int / -(10 as libc::c_int)
                                        }) <= -(1 as libc::c_int) - width) as libc::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + (-(127 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_int
                                    }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            width
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int)
                                                < width + (-(127 as libc::c_int) - 1 as libc::c_int))
                                                as libc::c_int
                                        } else {
                                            ((0 as libc::c_int) < width
                                                && -(1 as libc::c_int)
                                                    - (-(127 as libc::c_int) - 1 as libc::c_int)
                                                    < width - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        (((-(127 as libc::c_int) - 1 as libc::c_int)
                                            / 10 as libc::c_int) < width) as libc::c_int
                                    }
                                }
                            } else {
                                if 10 as libc::c_int == 0 as libc::c_int {
                                    0 as libc::c_int
                                } else {
                                    if width < 0 as libc::c_int {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + (-(127 as libc::c_int) - 1 as libc::c_int))
                                                as libc::c_int
                                        }) != 0 && width == -(1 as libc::c_int)
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((0 as libc::c_int)
                                                    < 10 as libc::c_int
                                                        + (-(127 as libc::c_int) - 1 as libc::c_int)) as libc::c_int
                                            } else {
                                                (-(1 as libc::c_int)
                                                    - (-(127 as libc::c_int) - 1 as libc::c_int)
                                                    < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                            }
                                        } else {
                                            ((-(127 as libc::c_int) - 1 as libc::c_int) / width
                                                < 10 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        ((127 as libc::c_int / 10 as libc::c_int) < width)
                                            as libc::c_int
                                    }
                                }
                            }) != 0
                            {
                                width = (width as libc::c_uint)
                                    .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                    as libc::c_schar as libc::c_int;
                                1 as libc::c_int
                            } else {
                                width = (width as libc::c_uint)
                                    .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                    as libc::c_schar as libc::c_int;
                                0 as libc::c_int
                            }
                        } else {
                            if (if (10 as libc::c_int) < 0 as libc::c_int {
                                if width < 0 as libc::c_int {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                        }) + 10 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        (width
                                            < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                / 10 as libc::c_int) as libc::c_int
                                    } else {
                                        ((if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            ((10 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                        }) != 0
                                        {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            })
                                                + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                / -(10 as libc::c_int)
                                        }) <= -(1 as libc::c_int) - width) as libc::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + 0 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int) as libc::c_int
                                    }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            width
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int) < width + 0 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((0 as libc::c_int) < width
                                                && (-(1 as libc::c_int) - 0 as libc::c_int)
                                                    < width - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        ((0 as libc::c_int / 10 as libc::c_int) < width)
                                            as libc::c_int
                                    }
                                }
                            } else {
                                if 10 as libc::c_int == 0 as libc::c_int {
                                    0 as libc::c_int
                                } else {
                                    if width < 0 as libc::c_int {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + 0 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + 0 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) + 0 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + 0 as libc::c_int) as libc::c_int
                                        }) != 0 && width == -(1 as libc::c_int)
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((0 as libc::c_int) < 10 as libc::c_int + 0 as libc::c_int)
                                                    as libc::c_int
                                            } else {
                                                ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                    < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                            }
                                        } else {
                                            (0 as libc::c_int / width < 10 as libc::c_int)
                                                as libc::c_int
                                        }
                                    } else {
                                        (((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                            / 10 as libc::c_int) < width) as libc::c_int
                                    }
                                }
                            }) != 0
                            {
                                width = (width as libc::c_uint)
                                    .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                    as libc::c_uchar as libc::c_int;
                                1 as libc::c_int
                            } else {
                                width = (width as libc::c_uint)
                                    .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                    as libc::c_uchar as libc::c_int;
                                0 as libc::c_int
                            }
                        }
                    } else {
                        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                        {
                            if !((0 as libc::c_int) < -(1 as libc::c_int)) {
                                if (if (10 as libc::c_int) < 0 as libc::c_int {
                                    if width < 0 as libc::c_int {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                32767 as libc::c_int
                                            }) + 10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            (width < 32767 as libc::c_int / 10 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                ((10 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 32767 as libc::c_int
                                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                32767 as libc::c_int / -(10 as libc::c_int)
                                            }) <= -(1 as libc::c_int) - width) as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                as libc::c_int
                                        }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((0 as libc::c_int)
                                                    < width + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                    as libc::c_int
                                            } else {
                                                ((0 as libc::c_int) < width
                                                    && -(1 as libc::c_int)
                                                        - (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        < width - 1 as libc::c_int) as libc::c_int
                                            }
                                        } else {
                                            (((-(32767 as libc::c_int) - 1 as libc::c_int)
                                                / 10 as libc::c_int) < width) as libc::c_int
                                        }
                                    }
                                } else {
                                    if 10 as libc::c_int == 0 as libc::c_int {
                                        0 as libc::c_int
                                    } else {
                                        if width < 0 as libc::c_int {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                    as libc::c_int
                                            }) != 0 && width == -(1 as libc::c_int)
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((0 as libc::c_int)
                                                        < 10 as libc::c_int
                                                            + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                        as libc::c_int
                                                } else {
                                                    (-(1 as libc::c_int)
                                                        - (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                }
                                            } else {
                                                ((-(32767 as libc::c_int) - 1 as libc::c_int) / width
                                                    < 10 as libc::c_int) as libc::c_int
                                            }
                                        } else {
                                            ((32767 as libc::c_int / 10 as libc::c_int) < width)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    width = (width as libc::c_uint)
                                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                        as libc::c_short as libc::c_int;
                                    1 as libc::c_int
                                } else {
                                    width = (width as libc::c_uint)
                                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                        as libc::c_short as libc::c_int;
                                    0 as libc::c_int
                                }
                            } else {
                                if (if (10 as libc::c_int) < 0 as libc::c_int {
                                    if width < 0 as libc::c_int {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                            }) + 10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            (width
                                                < (32767 as libc::c_int * 2 as libc::c_int
                                                    + 1 as libc::c_int) / 10 as libc::c_int) as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                ((10 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                })
                                                    + (32767 as libc::c_int * 2 as libc::c_int
                                                        + 1 as libc::c_int)
                                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                    / -(10 as libc::c_int)
                                            }) <= -(1 as libc::c_int) - width) as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int) as libc::c_int
                                        }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((0 as libc::c_int) < width + 0 as libc::c_int)
                                                    as libc::c_int
                                            } else {
                                                ((0 as libc::c_int) < width
                                                    && (-(1 as libc::c_int) - 0 as libc::c_int)
                                                        < width - 1 as libc::c_int) as libc::c_int
                                            }
                                        } else {
                                            ((0 as libc::c_int / 10 as libc::c_int) < width)
                                                as libc::c_int
                                        }
                                    }
                                } else {
                                    if 10 as libc::c_int == 0 as libc::c_int {
                                        0 as libc::c_int
                                    } else {
                                        if width < 0 as libc::c_int {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + 0 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int) as libc::c_int
                                            }) != 0 && width == -(1 as libc::c_int)
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((0 as libc::c_int) < 10 as libc::c_int + 0 as libc::c_int)
                                                        as libc::c_int
                                                } else {
                                                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                        < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                }
                                            } else {
                                                (0 as libc::c_int / width < 10 as libc::c_int)
                                                    as libc::c_int
                                            }
                                        } else {
                                            (((32767 as libc::c_int * 2 as libc::c_int
                                                + 1 as libc::c_int) / 10 as libc::c_int) < width)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    width = (width as libc::c_uint)
                                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                        as libc::c_ushort as libc::c_int;
                                    1 as libc::c_int
                                } else {
                                    width = (width as libc::c_uint)
                                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                        as libc::c_ushort as libc::c_int;
                                    0 as libc::c_int
                                }
                            }
                        } else {
                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    width
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    if (if (10 as libc::c_int) < 0 as libc::c_int {
                                        if width < 0 as libc::c_int {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    2147483647 as libc::c_int
                                                }) + 10 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                (width < 2147483647 as libc::c_int / 10 as libc::c_int)
                                                    as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((10 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                                }) != 0
                                                {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 2147483647 as libc::c_int
                                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    2147483647 as libc::c_int / -(10 as libc::c_int)
                                                }) <= -(1 as libc::c_int) - width) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                    as libc::c_int
                                            }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((0 as libc::c_int)
                                                        < width + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                        as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < width
                                                        && -(1 as libc::c_int)
                                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            < width - 1 as libc::c_int) as libc::c_int
                                                }
                                            } else {
                                                (((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    / 10 as libc::c_int) < width) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if 10 as libc::c_int == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            if width < 0 as libc::c_int {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                        as libc::c_int
                                                }) != 0 && width == -(1 as libc::c_int)
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int)
                                                            < 10 as libc::c_int
                                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                            as libc::c_int
                                                    } else {
                                                        (-(1 as libc::c_int)
                                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                    }
                                                } else {
                                                    ((-(2147483647 as libc::c_int) - 1 as libc::c_int) / width
                                                        < 10 as libc::c_int) as libc::c_int
                                                }
                                            } else {
                                                ((2147483647 as libc::c_int / 10 as libc::c_int) < width)
                                                    as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        width = (width as libc::c_uint)
                                            .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                            as libc::c_int;
                                        1 as libc::c_int
                                    } else {
                                        width = (width as libc::c_uint)
                                            .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                            as libc::c_int;
                                        0 as libc::c_int
                                    }
                                } else {
                                    if (if (10 as libc::c_int) < 0 as libc::c_int {
                                        if width < 0 as libc::c_int {
                                            if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_uint
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_uint
                                                } else {
                                                    (2147483647 as libc::c_int as libc::c_uint)
                                                        .wrapping_mul(2 as libc::c_uint)
                                                        .wrapping_add(1 as libc::c_uint)
                                                })
                                                    .wrapping_add(10 as libc::c_int as libc::c_uint)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                < 0 as libc::c_int as libc::c_uint
                                            {
                                                ((width as libc::c_uint)
                                                    < (2147483647 as libc::c_int as libc::c_uint)
                                                        .wrapping_mul(2 as libc::c_uint)
                                                        .wrapping_add(1 as libc::c_uint)
                                                        .wrapping_div(10 as libc::c_int as libc::c_uint))
                                                    as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((10 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                                }) != 0
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) as libc::c_uint)
                                                        .wrapping_add(
                                                            (2147483647 as libc::c_int as libc::c_uint)
                                                                .wrapping_mul(2 as libc::c_uint)
                                                                .wrapping_add(1 as libc::c_uint),
                                                        )
                                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (2147483647 as libc::c_int as libc::c_uint)
                                                        .wrapping_mul(2 as libc::c_uint)
                                                        .wrapping_add(1 as libc::c_uint)
                                                        .wrapping_div(-(10 as libc::c_int) as libc::c_uint)
                                                }) <= (-(1 as libc::c_int) - width) as libc::c_uint)
                                                    as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int) as libc::c_int
                                            }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((0 as libc::c_int) < width + 0 as libc::c_int)
                                                        as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < width
                                                        && (-(1 as libc::c_int) - 0 as libc::c_int)
                                                            < width - 1 as libc::c_int) as libc::c_int
                                                }
                                            } else {
                                                ((0 as libc::c_int / 10 as libc::c_int) < width)
                                                    as libc::c_int
                                            }
                                        }
                                    } else {
                                        if 10 as libc::c_int == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            if width < 0 as libc::c_int {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    (((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) + 0 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) + 0 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int) as libc::c_int
                                                }) != 0 && width == -(1 as libc::c_int)
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int) < 10 as libc::c_int + 0 as libc::c_int)
                                                            as libc::c_int
                                                    } else {
                                                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                            < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                    }
                                                } else {
                                                    (0 as libc::c_int / width < 10 as libc::c_int)
                                                        as libc::c_int
                                                }
                                            } else {
                                                ((2147483647 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(2 as libc::c_uint)
                                                    .wrapping_add(1 as libc::c_uint)
                                                    .wrapping_div(10 as libc::c_int as libc::c_uint)
                                                    < width as libc::c_uint) as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        width = (width as libc::c_uint)
                                            .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                            as libc::c_int;
                                        1 as libc::c_int
                                    } else {
                                        width = (width as libc::c_uint)
                                            .wrapping_mul(10 as libc::c_int as libc::c_uint)
                                            as libc::c_int;
                                        0 as libc::c_int
                                    }
                                }
                            } else {
                                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        width
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        if (if (10 as libc::c_int) < 0 as libc::c_int {
                                            if width < 0 as libc::c_int {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        9223372036854775807 as libc::c_long
                                                    }) + 10 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((width as libc::c_long)
                                                        < 9223372036854775807 as libc::c_long
                                                            / 10 as libc::c_int as libc::c_long) as libc::c_int
                                                } else {
                                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) < 0 as libc::c_int
                                                    {
                                                        ((10 as libc::c_int)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 1 as libc::c_int)
                                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) - 1 as libc::c_int
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                                    }) != 0
                                                    {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_long + 9223372036854775807 as libc::c_long
                                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    } else {
                                                        9223372036854775807 as libc::c_long
                                                            / -(10 as libc::c_int) as libc::c_long
                                                    }) <= (-(1 as libc::c_int) - width) as libc::c_long)
                                                        as libc::c_int
                                                }
                                            } else {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) as libc::c_long
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_long
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_long
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) as libc::c_long
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) as libc::c_long
                                                                    + (-(9223372036854775807 as libc::c_long)
                                                                        - 1 as libc::c_long)
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) as libc::c_long
                                                                    + (-(9223372036854775807 as libc::c_long)
                                                                        - 1 as libc::c_long)
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_long
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)) as libc::c_int
                                                }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int as libc::c_long)
                                                            < width as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < width
                                                            && -(1 as libc::c_int) as libc::c_long
                                                                - (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                                < (width - 1 as libc::c_int) as libc::c_long) as libc::c_int
                                                    }
                                                } else {
                                                    (((-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long) / 10 as libc::c_int as libc::c_long)
                                                        < width as libc::c_long) as libc::c_int
                                                }
                                            }
                                        } else {
                                            if 10 as libc::c_int == 0 as libc::c_int {
                                                0 as libc::c_int
                                            } else {
                                                if width < 0 as libc::c_int {
                                                    if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) as libc::c_long
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) as libc::c_long
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) as libc::c_long
                                                                    + (-(9223372036854775807 as libc::c_long)
                                                                        - 1 as libc::c_long)
                                                            }) - 1 as libc::c_int as libc::c_long)
                                                                < 0 as libc::c_int as libc::c_long
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) as libc::c_long
                                                                        + (-(9223372036854775807 as libc::c_long)
                                                                            - 1 as libc::c_long)
                                                                }) + 1 as libc::c_int as libc::c_long)
                                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int as libc::c_long)
                                                                    * 2 as libc::c_int as libc::c_long
                                                                    + 1 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) as libc::c_long
                                                                        + (-(9223372036854775807 as libc::c_long)
                                                                            - 1 as libc::c_long)
                                                                }) - 1 as libc::c_int as libc::c_long
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int as libc::c_long)
                                                            < (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)) as libc::c_int
                                                    }) != 0 && width == -(1 as libc::c_int)
                                                    {
                                                        if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((0 as libc::c_int as libc::c_long)
                                                                < 10 as libc::c_int as libc::c_long
                                                                    + (-(9223372036854775807 as libc::c_long)
                                                                        - 1 as libc::c_long)) as libc::c_int
                                                        } else {
                                                            (-(1 as libc::c_int) as libc::c_long
                                                                - (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                                < (10 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                                                as libc::c_int
                                                        }
                                                    } else {
                                                        (((-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long) / width as libc::c_long)
                                                            < 10 as libc::c_int as libc::c_long) as libc::c_int
                                                    }
                                                } else {
                                                    ((9223372036854775807 as libc::c_long
                                                        / 10 as libc::c_int as libc::c_long)
                                                        < width as libc::c_long) as libc::c_int
                                                }
                                            }
                                        }) != 0
                                        {
                                            width = (width as libc::c_ulong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                                as libc::c_long as libc::c_int;
                                            1 as libc::c_int
                                        } else {
                                            width = (width as libc::c_ulong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                                as libc::c_long as libc::c_int;
                                            0 as libc::c_int
                                        }
                                    } else {
                                        if (if (10 as libc::c_int) < 0 as libc::c_int {
                                            if width < 0 as libc::c_int {
                                                if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_ulong)
                                                            .wrapping_add(1 as libc::c_ulong)
                                                    })
                                                        .wrapping_add(10 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((width as libc::c_ulong)
                                                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_ulong)
                                                            .wrapping_add(1 as libc::c_ulong)
                                                            .wrapping_div(10 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                } else {
                                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) < 0 as libc::c_int
                                                    {
                                                        ((10 as libc::c_int)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 1 as libc::c_int)
                                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) - 1 as libc::c_int
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                                    }) != 0
                                                    {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_ulong)
                                                            .wrapping_add(
                                                                (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                                    .wrapping_mul(2 as libc::c_ulong)
                                                                    .wrapping_add(1 as libc::c_ulong),
                                                            )
                                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    } else {
                                                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_ulong)
                                                            .wrapping_add(1 as libc::c_ulong)
                                                            .wrapping_div(-(10 as libc::c_int) as libc::c_ulong)
                                                    }) <= (-(1 as libc::c_int) - width) as libc::c_ulong)
                                                        as libc::c_int
                                                }
                                            } else {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    (((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 0 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 0 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int) as libc::c_int
                                                }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int) < width + 0 as libc::c_int)
                                                            as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < width
                                                            && (-(1 as libc::c_int) - 0 as libc::c_int)
                                                                < width - 1 as libc::c_int) as libc::c_int
                                                    }
                                                } else {
                                                    ((0 as libc::c_int / 10 as libc::c_int) < width)
                                                        as libc::c_int
                                                }
                                            }
                                        } else {
                                            if 10 as libc::c_int == 0 as libc::c_int {
                                                0 as libc::c_int
                                            } else {
                                                if width < 0 as libc::c_int {
                                                    if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) < 0 as libc::c_int
                                                    {
                                                        (((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) + 0 as libc::c_int
                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) + 0 as libc::c_int
                                                                }) + 1 as libc::c_int)
                                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) + 0 as libc::c_int
                                                                }) - 1 as libc::c_int
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int)
                                                            < (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int) as libc::c_int
                                                    }) != 0 && width == -(1 as libc::c_int)
                                                    {
                                                        if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((0 as libc::c_int) < 10 as libc::c_int + 0 as libc::c_int)
                                                                as libc::c_int
                                                        } else {
                                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                                < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                        }
                                                    } else {
                                                        (0 as libc::c_int / width < 10 as libc::c_int)
                                                            as libc::c_int
                                                    }
                                                } else {
                                                    ((9223372036854775807 as libc::c_long as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_ulong)
                                                        .wrapping_div(10 as libc::c_int as libc::c_ulong)
                                                        < width as libc::c_ulong) as libc::c_int
                                                }
                                            }
                                        }) != 0
                                        {
                                            width = (width as libc::c_ulong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            1 as libc::c_int
                                        } else {
                                            width = (width as libc::c_ulong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            0 as libc::c_int
                                        }
                                    }
                                } else {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        width
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        if (if (10 as libc::c_int) < 0 as libc::c_int {
                                            if width < 0 as libc::c_int {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        9223372036854775807 as libc::c_longlong
                                                    }) + 10 as libc::c_int as libc::c_longlong
                                                }) - 1 as libc::c_int as libc::c_longlong)
                                                    < 0 as libc::c_int as libc::c_longlong
                                                {
                                                    ((width as libc::c_longlong)
                                                        < 9223372036854775807 as libc::c_longlong
                                                            / 10 as libc::c_int as libc::c_longlong) as libc::c_int
                                                } else {
                                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) < 0 as libc::c_int
                                                    {
                                                        ((10 as libc::c_int)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 1 as libc::c_int)
                                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) - 1 as libc::c_int
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                                    }) != 0
                                                    {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_longlong
                                                            + 9223372036854775807 as libc::c_longlong
                                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    } else {
                                                        9223372036854775807 as libc::c_longlong
                                                            / -(10 as libc::c_int) as libc::c_longlong
                                                    }) <= (-(1 as libc::c_int) - width) as libc::c_longlong)
                                                        as libc::c_int
                                                }
                                            } else {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) - 1 as libc::c_int as libc::c_longlong)
                                                    < 0 as libc::c_int as libc::c_longlong
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) + 1 as libc::c_int as libc::c_longlong)
                                                        << (::std::mem::size_of::<libc::c_longlong>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_longlong)
                                                        * 2 as libc::c_int as libc::c_longlong
                                                        + 1 as libc::c_int as libc::c_longlong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) + 0 as libc::c_int as libc::c_longlong
                                                }) < 0 as libc::c_int as libc::c_longlong
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) - 1 as libc::c_int as libc::c_longlong)
                                                            < 0 as libc::c_int as libc::c_longlong
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_longlong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) + 1 as libc::c_int as libc::c_longlong)
                                                                << (::std::mem::size_of::<libc::c_longlong>()
                                                                    as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_longlong)
                                                                * 2 as libc::c_int as libc::c_longlong
                                                                + 1 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_longlong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) - 1 as libc::c_int as libc::c_longlong
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_longlong)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)) as libc::c_int
                                                }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int as libc::c_longlong)
                                                            < width as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < width
                                                            && -(1 as libc::c_int) as libc::c_longlong
                                                                - (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                                < (width - 1 as libc::c_int) as libc::c_longlong)
                                                            as libc::c_int
                                                    }
                                                } else {
                                                    (((-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                                        / 10 as libc::c_int as libc::c_longlong)
                                                        < width as libc::c_longlong) as libc::c_int
                                                }
                                            }
                                        } else {
                                            if 10 as libc::c_int == 0 as libc::c_int {
                                                0 as libc::c_int
                                            } else {
                                                if width < 0 as libc::c_int {
                                                    if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) - 1 as libc::c_int as libc::c_longlong)
                                                        < 0 as libc::c_int as libc::c_longlong
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) + 1 as libc::c_int as libc::c_longlong)
                                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                                as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_longlong)
                                                            * 2 as libc::c_int as libc::c_longlong
                                                            + 1 as libc::c_int as libc::c_longlong)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) + 0 as libc::c_int as libc::c_longlong
                                                    }) < 0 as libc::c_int as libc::c_longlong
                                                    {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_longlong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) - 1 as libc::c_int as libc::c_longlong)
                                                                < 0 as libc::c_int as libc::c_longlong
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_longlong
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) as libc::c_longlong
                                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                                            - 1 as libc::c_longlong)
                                                                }) + 1 as libc::c_int as libc::c_longlong)
                                                                    << (::std::mem::size_of::<libc::c_longlong>()
                                                                        as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int as libc::c_longlong)
                                                                    * 2 as libc::c_int as libc::c_longlong
                                                                    + 1 as libc::c_int as libc::c_longlong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_longlong
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) as libc::c_longlong
                                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                                            - 1 as libc::c_longlong)
                                                                }) - 1 as libc::c_int as libc::c_longlong
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int as libc::c_longlong)
                                                            < (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)) as libc::c_int
                                                    }) != 0 && width == -(1 as libc::c_int)
                                                    {
                                                        if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((0 as libc::c_int as libc::c_longlong)
                                                                < 10 as libc::c_int as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)) as libc::c_int
                                                        } else {
                                                            (-(1 as libc::c_int) as libc::c_longlong
                                                                - (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                                < (10 as libc::c_int - 1 as libc::c_int)
                                                                    as libc::c_longlong) as libc::c_int
                                                        }
                                                    } else {
                                                        (((-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong) / width as libc::c_longlong)
                                                            < 10 as libc::c_int as libc::c_longlong) as libc::c_int
                                                    }
                                                } else {
                                                    ((9223372036854775807 as libc::c_longlong
                                                        / 10 as libc::c_int as libc::c_longlong)
                                                        < width as libc::c_longlong) as libc::c_int
                                                }
                                            }
                                        }) != 0
                                        {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulonglong)
                                                as libc::c_longlong as libc::c_int;
                                            1 as libc::c_int
                                        } else {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulonglong)
                                                as libc::c_longlong as libc::c_int;
                                            0 as libc::c_int
                                        }
                                    } else {
                                        if (if (10 as libc::c_int) < 0 as libc::c_int {
                                            if width < 0 as libc::c_int {
                                                if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulonglong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulonglong
                                                    } else {
                                                        (9223372036854775807 as libc::c_longlong
                                                            as libc::c_ulonglong)
                                                            .wrapping_mul(2 as libc::c_ulonglong)
                                                            .wrapping_add(1 as libc::c_ulonglong)
                                                    })
                                                        .wrapping_add(10 as libc::c_int as libc::c_ulonglong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                    < 0 as libc::c_int as libc::c_ulonglong
                                                {
                                                    ((width as libc::c_ulonglong)
                                                        < (9223372036854775807 as libc::c_longlong
                                                            as libc::c_ulonglong)
                                                            .wrapping_mul(2 as libc::c_ulonglong)
                                                            .wrapping_add(1 as libc::c_ulonglong)
                                                            .wrapping_div(10 as libc::c_int as libc::c_ulonglong))
                                                        as libc::c_int
                                                } else {
                                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) < 0 as libc::c_int
                                                    {
                                                        ((10 as libc::c_int)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 1 as libc::c_int)
                                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) - 1 as libc::c_int
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                                    }) != 0
                                                    {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) as libc::c_ulonglong)
                                                            .wrapping_add(
                                                                (9223372036854775807 as libc::c_longlong
                                                                    as libc::c_ulonglong)
                                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                                    .wrapping_add(1 as libc::c_ulonglong),
                                                            )
                                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    } else {
                                                        (9223372036854775807 as libc::c_longlong
                                                            as libc::c_ulonglong)
                                                            .wrapping_mul(2 as libc::c_ulonglong)
                                                            .wrapping_add(1 as libc::c_ulonglong)
                                                            .wrapping_div(-(10 as libc::c_int) as libc::c_ulonglong)
                                                    }) <= (-(1 as libc::c_int) - width) as libc::c_ulonglong)
                                                        as libc::c_int
                                                }
                                            } else {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    (((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        10 as libc::c_int
                                                    }) + 0 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                10 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 0 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    10 as libc::c_int
                                                                }) + 0 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) + 0 as libc::c_int) as libc::c_int
                                                }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int) < width + 0 as libc::c_int)
                                                            as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int) < width
                                                            && (-(1 as libc::c_int) - 0 as libc::c_int)
                                                                < width - 1 as libc::c_int) as libc::c_int
                                                    }
                                                } else {
                                                    ((0 as libc::c_int / 10 as libc::c_int) < width)
                                                        as libc::c_int
                                                }
                                            }
                                        } else {
                                            if 10 as libc::c_int == 0 as libc::c_int {
                                                0 as libc::c_int
                                            } else {
                                                if width < 0 as libc::c_int {
                                                    if (if (if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        !(((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) < 0 as libc::c_int
                                                    {
                                                        (((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            width
                                                        }) + 0 as libc::c_int)
                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    width
                                                                }) + 0 as libc::c_int
                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                            {
                                                                ((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) + 0 as libc::c_int
                                                                }) + 1 as libc::c_int)
                                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        width
                                                                    }) + 0 as libc::c_int
                                                                }) - 1 as libc::c_int
                                                            })) as libc::c_int
                                                    } else {
                                                        ((0 as libc::c_int)
                                                            < (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                width
                                                            }) + 0 as libc::c_int) as libc::c_int
                                                    }) != 0 && width == -(1 as libc::c_int)
                                                    {
                                                        if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            10 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((0 as libc::c_int) < 10 as libc::c_int + 0 as libc::c_int)
                                                                as libc::c_int
                                                        } else {
                                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                                < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                        }
                                                    } else {
                                                        (0 as libc::c_int / width < 10 as libc::c_int)
                                                            as libc::c_int
                                                    }
                                                } else {
                                                    ((9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(10 as libc::c_int as libc::c_ulonglong)
                                                        < width as libc::c_ulonglong) as libc::c_int
                                                }
                                            }
                                        }) != 0
                                        {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulonglong)
                                                as libc::c_int;
                                            1 as libc::c_int
                                        } else {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulonglong)
                                                as libc::c_int;
                                            0 as libc::c_int
                                        }
                                    }
                                }
                            }
                        }
                    }) != 0
                        || {
                            let (fresh2, fresh3) = width
                                .overflowing_add(*f as libc::c_int - '0' as i32);
                            *(&mut width as *mut libc::c_int) = fresh2;
                            fresh3 as libc::c_int != 0
                        }
                    {
                        width = 2147483647 as libc::c_int;
                    }
                    f = f.offset(1);
                    f;
                    if !((*f as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint)
                    {
                        break;
                    }
                }
            }
            match *f as libc::c_int {
                69 | 79 => {
                    let fresh4 = f;
                    f = f.offset(1);
                    modifier = *fresh4 as libc::c_int;
                }
                _ => {
                    modifier = 0 as libc::c_int;
                }
            }
            format_char = *f as libc::c_int;
            match format_char {
                37 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 4780816269647783540;
                    } else {
                        let mut _n_0: size_t = 1 as libc::c_int as size_t;
                        let mut _w_0: size_t = (if pad == '-' as i32
                            || width < 0 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            width
                        }) as size_t;
                        let mut _incr_0: size_t = if _n_0 < _w_0 { _w_0 } else { _n_0 };
                        if _incr_0 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as libc::c_int;
                            return 0 as libc::c_int as size_t;
                        }
                        if !p.is_null() {
                            if _n_0 < _w_0 {
                                let mut _delta_0: size_t = _w_0.wrapping_sub(_n_0);
                                if pad == '0' as i32 || pad == '+' as i32 {
                                    let mut _i_1: size_t = 0;
                                    _i_1 = 0 as libc::c_int as size_t;
                                    while _i_1 < _delta_0 {
                                        fputc('0' as i32, p);
                                        _i_1 = _i_1.wrapping_add(1);
                                        _i_1;
                                    }
                                } else {
                                    let mut _i_2: size_t = 0;
                                    _i_2 = 0 as libc::c_int as size_t;
                                    while _i_2 < _delta_0 {
                                        fputc(' ' as i32, p);
                                        _i_2 = _i_2.wrapping_add(1);
                                        _i_2;
                                    }
                                }
                            }
                            fputc(*f as libc::c_int, p);
                        }
                        i = (i as libc::c_ulong).wrapping_add(_incr_0) as size_t
                            as size_t;
                        current_block = 1394248824506584008;
                    }
                }
                97 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 4780816269647783540;
                    } else {
                        if change_case {
                            to_uppcase = 1 as libc::c_int != 0;
                            to_lowcase = 0 as libc::c_int != 0;
                        }
                        current_block = 11524224095985028372;
                    }
                }
                65 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 4780816269647783540;
                    } else {
                        if change_case {
                            to_uppcase = 1 as libc::c_int != 0;
                            to_lowcase = 0 as libc::c_int != 0;
                        }
                        current_block = 11524224095985028372;
                    }
                }
                98 | 104 => {
                    if change_case {
                        to_uppcase = 1 as libc::c_int != 0;
                        to_lowcase = 0 as libc::c_int != 0;
                    }
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        current_block = 11524224095985028372;
                    }
                }
                66 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        if change_case {
                            to_uppcase = 1 as libc::c_int != 0;
                            to_lowcase = 0 as libc::c_int != 0;
                        }
                        current_block = 11524224095985028372;
                    }
                }
                99 => {
                    if modifier == 'O' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        current_block = 11524224095985028372;
                    }
                }
                67 => {
                    if modifier == 'E' as i32 {
                        current_block = 11524224095985028372;
                    } else {
                        let mut negative_year: bool = (*tp).tm_year
                            < -(1900 as libc::c_int);
                        let mut zero_thru_1899: bool = !negative_year as libc::c_int
                            & ((*tp).tm_year < 0 as libc::c_int) as libc::c_int != 0;
                        let mut century: libc::c_int = ((*tp).tm_year
                            - 99 as libc::c_int * zero_thru_1899 as libc::c_int)
                            / 100 as libc::c_int
                            + 1900 as libc::c_int / 100 as libc::c_int;
                        digits = 2 as libc::c_int;
                        negative_number = negative_year;
                        u_number_value = century as libc::c_uint;
                        current_block = 658192006876172990;
                    }
                }
                120 => {
                    if modifier == 'O' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        current_block = 11524224095985028372;
                    }
                }
                68 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 4780816269647783540;
                    } else {
                        subfmt = b"%m/%d/%y\0" as *const u8 as *const libc::c_char;
                        current_block = 12246015723122908320;
                    }
                }
                100 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_mday;
                        current_block = 1180518703837991265;
                    }
                }
                101 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_mday;
                        current_block = 2746649332913740001;
                    }
                }
                70 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 4780816269647783540;
                    } else {
                        if pad == 0 as libc::c_int && width < 0 as libc::c_int {
                            pad = '+' as i32;
                            subwidth = 4 as libc::c_int;
                        } else {
                            subwidth = width - 6 as libc::c_int;
                            if subwidth < 0 as libc::c_int {
                                subwidth = 0 as libc::c_int;
                            }
                        }
                        subfmt = b"%Y-%m-%d\0" as *const u8 as *const libc::c_char;
                        current_block = 14207246619195705495;
                    }
                }
                72 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_hour;
                        current_block = 1180518703837991265;
                    }
                }
                73 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = hour12;
                        current_block = 1180518703837991265;
                    }
                }
                107 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_hour;
                        current_block = 2746649332913740001;
                    }
                }
                108 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = hour12;
                        current_block = 2746649332913740001;
                    }
                }
                106 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 3 as libc::c_int;
                        negative_number = (*tp).tm_yday < -(1 as libc::c_int);
                        u_number_value = ((*tp).tm_yday as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint);
                        current_block = 7056221854519279815;
                    }
                }
                77 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_min;
                        current_block = 1180518703837991265;
                    }
                }
                109 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        negative_number = (*tp).tm_mon < -(1 as libc::c_int);
                        u_number_value = ((*tp).tm_mon as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint);
                        current_block = 7056221854519279815;
                    }
                }
                78 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        let mut n: libc::c_int = ns;
                        let mut ns_digits: libc::c_int = 9 as libc::c_int;
                        if width <= 0 as libc::c_int {
                            width = ns_digits;
                        }
                        let mut ndigs: libc::c_int = ns_digits;
                        while width < ndigs
                            || (1 as libc::c_int) < ndigs
                                && n % 10 as libc::c_int == 0 as libc::c_int
                        {
                            ndigs -= 1;
                            ndigs;
                            n /= 10 as libc::c_int;
                        }
                        let mut j: libc::c_int = ndigs;
                        while (0 as libc::c_int) < j {
                            buf[(j - 1 as libc::c_int)
                                as usize] = (n % 10 as libc::c_int + '0' as i32)
                                as libc::c_char;
                            n /= 10 as libc::c_int;
                            j -= 1;
                            j;
                        }
                        if pad == 0 {
                            pad = '0' as i32;
                        }
                        let mut _n_5: size_t = ndigs as size_t;
                        let mut _w_5: size_t = (if pad == '-' as i32
                            || (0 as libc::c_int) < 0 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as size_t;
                        let mut _incr_5: size_t = if _n_5 < _w_5 { _w_5 } else { _n_5 };
                        if _incr_5 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as libc::c_int;
                            return 0 as libc::c_int as size_t;
                        }
                        if !p.is_null() {
                            if _n_5 < _w_5 {
                                let mut _delta_5: size_t = _w_5.wrapping_sub(_n_5);
                                if pad == '0' as i32 || pad == '+' as i32 {
                                    let mut _i_12: size_t = 0;
                                    _i_12 = 0 as libc::c_int as size_t;
                                    while _i_12 < _delta_5 {
                                        fputc('0' as i32, p);
                                        _i_12 = _i_12.wrapping_add(1);
                                        _i_12;
                                    }
                                } else {
                                    let mut _i_13: size_t = 0;
                                    _i_13 = 0 as libc::c_int as size_t;
                                    while _i_13 < _delta_5 {
                                        fputc(' ' as i32, p);
                                        _i_13 = _i_13.wrapping_add(1);
                                        _i_13;
                                    }
                                }
                            }
                            if to_lowcase {
                                fwrite_lowcase(p, buf.as_mut_ptr(), _n_5);
                            } else if to_uppcase {
                                fwrite_uppcase(p, buf.as_mut_ptr(), _n_5);
                            } else {
                                fwrite(
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    _n_5,
                                    1 as libc::c_int as libc::c_ulong,
                                    p,
                                );
                            }
                        }
                        i = (i as libc::c_ulong).wrapping_add(_incr_5) as size_t
                            as size_t;
                        let mut _n_6: size_t = 0 as libc::c_int as size_t;
                        let mut _w_6: size_t = (if pad == '-' as i32
                            || width - ndigs < 0 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            width - ndigs
                        }) as size_t;
                        let mut _incr_6: size_t = if _n_6 < _w_6 { _w_6 } else { _n_6 };
                        if _incr_6 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as libc::c_int;
                            return 0 as libc::c_int as size_t;
                        }
                        if !p.is_null() {
                            if _n_6 < _w_6 {
                                let mut _delta_6: size_t = _w_6.wrapping_sub(_n_6);
                                if pad == '0' as i32 || pad == '+' as i32 {
                                    let mut _i_14: size_t = 0;
                                    _i_14 = 0 as libc::c_int as size_t;
                                    while _i_14 < _delta_6 {
                                        fputc('0' as i32, p);
                                        _i_14 = _i_14.wrapping_add(1);
                                        _i_14;
                                    }
                                } else {
                                    let mut _i_15: size_t = 0;
                                    _i_15 = 0 as libc::c_int as size_t;
                                    while _i_15 < _delta_6 {
                                        fputc(' ' as i32, p);
                                        _i_15 = _i_15.wrapping_add(1);
                                        _i_15;
                                    }
                                }
                            }
                        }
                        i = (i as libc::c_ulong).wrapping_add(_incr_6) as size_t
                            as size_t;
                        current_block = 1394248824506584008;
                    }
                }
                110 => {
                    let mut _n_7: size_t = 1 as libc::c_int as size_t;
                    let mut _w_7: size_t = (if pad == '-' as i32
                        || width < 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_7: size_t = if _n_7 < _w_7 { _w_7 } else { _n_7 };
                    if _incr_7 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    if !p.is_null() {
                        if _n_7 < _w_7 {
                            let mut _delta_7: size_t = _w_7.wrapping_sub(_n_7);
                            if pad == '0' as i32 || pad == '+' as i32 {
                                let mut _i_16: size_t = 0;
                                _i_16 = 0 as libc::c_int as size_t;
                                while _i_16 < _delta_7 {
                                    fputc('0' as i32, p);
                                    _i_16 = _i_16.wrapping_add(1);
                                    _i_16;
                                }
                            } else {
                                let mut _i_17: size_t = 0;
                                _i_17 = 0 as libc::c_int as size_t;
                                while _i_17 < _delta_7 {
                                    fputc(' ' as i32, p);
                                    _i_17 = _i_17.wrapping_add(1);
                                    _i_17;
                                }
                            }
                        }
                        fputc('\n' as i32, p);
                    }
                    i = (i as libc::c_ulong).wrapping_add(_incr_7) as size_t as size_t;
                    current_block = 1394248824506584008;
                }
                80 => {
                    to_lowcase = 1 as libc::c_int != 0;
                    format_char = 'p' as i32;
                    current_block = 12711609573701009411;
                }
                112 => {
                    current_block = 12711609573701009411;
                }
                113 => {
                    digits = 1 as libc::c_int;
                    negative_number = 0 as libc::c_int != 0;
                    u_number_value = (((*tp).tm_mon * 11 as libc::c_int
                        >> 5 as libc::c_int) + 1 as libc::c_int) as libc::c_uint;
                    current_block = 7056221854519279815;
                }
                82 => {
                    subfmt = b"%H:%M\0" as *const u8 as *const libc::c_char;
                    current_block = 12246015723122908320;
                }
                114 => {
                    current_block = 11524224095985028372;
                }
                83 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_sec;
                        current_block = 1180518703837991265;
                    }
                }
                115 => {
                    let mut ltm: tm = tm {
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
                    let mut t: time_t = 0;
                    ltm = *tp;
                    ltm.tm_yday = -(1 as libc::c_int);
                    t = mktime_z(tz, &mut ltm);
                    if ltm.tm_yday < 0 as libc::c_int {
                        *__errno_location() = 75 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    bufp = buf
                        .as_mut_ptr()
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 23]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ) as isize,
                        );
                    negative_number = t < 0 as libc::c_int as libc::c_long;
                    loop {
                        let mut d: libc::c_int = (t % 10 as libc::c_int as libc::c_long)
                            as libc::c_int;
                        t /= 10 as libc::c_int as libc::c_long;
                        bufp = bufp.offset(-1);
                        *bufp = ((if negative_number as libc::c_int != 0 {
                            -d
                        } else {
                            d
                        }) + '0' as i32) as libc::c_char;
                        if !(t != 0 as libc::c_int as libc::c_long) {
                            break;
                        }
                    }
                    digits = 1 as libc::c_int;
                    always_output_a_sign = 0 as libc::c_int != 0;
                    current_block = 1997968267199582064;
                }
                88 => {
                    if modifier == 'O' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        current_block = 11524224095985028372;
                    }
                }
                84 => {
                    subfmt = b"%H:%M:%S\0" as *const u8 as *const libc::c_char;
                    current_block = 12246015723122908320;
                }
                116 => {
                    let mut _n_8: size_t = 1 as libc::c_int as size_t;
                    let mut _w_8: size_t = (if pad == '-' as i32
                        || width < 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_8: size_t = if _n_8 < _w_8 { _w_8 } else { _n_8 };
                    if _incr_8 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    if !p.is_null() {
                        if _n_8 < _w_8 {
                            let mut _delta_8: size_t = _w_8.wrapping_sub(_n_8);
                            if pad == '0' as i32 || pad == '+' as i32 {
                                let mut _i_18: size_t = 0;
                                _i_18 = 0 as libc::c_int as size_t;
                                while _i_18 < _delta_8 {
                                    fputc('0' as i32, p);
                                    _i_18 = _i_18.wrapping_add(1);
                                    _i_18;
                                }
                            } else {
                                let mut _i_19: size_t = 0;
                                _i_19 = 0 as libc::c_int as size_t;
                                while _i_19 < _delta_8 {
                                    fputc(' ' as i32, p);
                                    _i_19 = _i_19.wrapping_add(1);
                                    _i_19;
                                }
                            }
                        }
                        fputc('\t' as i32, p);
                    }
                    i = (i as libc::c_ulong).wrapping_add(_incr_8) as size_t as size_t;
                    current_block = 1394248824506584008;
                }
                117 => {
                    digits = 1 as libc::c_int;
                    number_value = ((*tp).tm_wday - 1 as libc::c_int + 7 as libc::c_int)
                        % 7 as libc::c_int + 1 as libc::c_int;
                    current_block = 1180518703837991265;
                }
                85 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = ((*tp).tm_yday - (*tp).tm_wday + 7 as libc::c_int)
                            / 7 as libc::c_int;
                        current_block = 1180518703837991265;
                    }
                }
                86 | 103 | 71 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        let mut year: libc::c_int = (*tp).tm_year
                            + (if (*tp).tm_year < 0 as libc::c_int {
                                1900 as libc::c_int % 400 as libc::c_int
                            } else {
                                1900 as libc::c_int % 400 as libc::c_int
                                    - 400 as libc::c_int
                            });
                        let mut year_adjust: libc::c_int = 0 as libc::c_int;
                        let mut days: libc::c_int = iso_week_days(
                            (*tp).tm_yday,
                            (*tp).tm_wday,
                        );
                        if days < 0 as libc::c_int {
                            year_adjust = -(1 as libc::c_int);
                            days = iso_week_days(
                                (*tp).tm_yday
                                    + (365 as libc::c_int
                                        + ((year - 1 as libc::c_int) % 4 as libc::c_int
                                            == 0 as libc::c_int
                                            && ((year - 1 as libc::c_int) % 100 as libc::c_int
                                                != 0 as libc::c_int
                                                || (year - 1 as libc::c_int) % 400 as libc::c_int
                                                    == 0 as libc::c_int)) as libc::c_int),
                                (*tp).tm_wday,
                            );
                        } else {
                            let mut d_0: libc::c_int = iso_week_days(
                                (*tp).tm_yday
                                    - (365 as libc::c_int
                                        + (year % 4 as libc::c_int == 0 as libc::c_int
                                            && (year % 100 as libc::c_int != 0 as libc::c_int
                                                || year % 400 as libc::c_int == 0 as libc::c_int))
                                            as libc::c_int),
                                (*tp).tm_wday,
                            );
                            if 0 as libc::c_int <= d_0 {
                                year_adjust = 1 as libc::c_int;
                                days = d_0;
                            }
                        }
                        match *f as libc::c_int {
                            103 => {
                                let mut yy: libc::c_int = ((*tp).tm_year
                                    % 100 as libc::c_int + year_adjust) % 100 as libc::c_int;
                                digits = 2 as libc::c_int;
                                negative_number = 0 as libc::c_int != 0;
                                u_number_value = (if 0 as libc::c_int <= yy {
                                    yy
                                } else if (*tp).tm_year
                                    < -(1900 as libc::c_int) - year_adjust
                                {
                                    -yy
                                } else {
                                    yy + 100 as libc::c_int
                                }) as libc::c_uint;
                                current_block = 658192006876172990;
                            }
                            71 => {
                                digits = 4 as libc::c_int;
                                negative_number = (*tp).tm_year
                                    < -(1900 as libc::c_int) - year_adjust;
                                u_number_value = ((*tp).tm_year as libc::c_uint)
                                    .wrapping_add(1900 as libc::c_int as libc::c_uint)
                                    .wrapping_add(year_adjust as libc::c_uint);
                                current_block = 658192006876172990;
                            }
                            _ => {
                                digits = 2 as libc::c_int;
                                number_value = days / 7 as libc::c_int + 1 as libc::c_int;
                                current_block = 1180518703837991265;
                            }
                        }
                    }
                }
                87 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = ((*tp).tm_yday
                            - ((*tp).tm_wday - 1 as libc::c_int + 7 as libc::c_int)
                                % 7 as libc::c_int + 7 as libc::c_int) / 7 as libc::c_int;
                        current_block = 1180518703837991265;
                    }
                }
                119 => {
                    if modifier == 'E' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 1 as libc::c_int;
                        number_value = (*tp).tm_wday;
                        current_block = 1180518703837991265;
                    }
                }
                89 => {
                    if modifier == 'E' as i32 {
                        current_block = 11524224095985028372;
                    } else if modifier == 'O' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        digits = 4 as libc::c_int;
                        negative_number = (*tp).tm_year < -(1900 as libc::c_int);
                        u_number_value = ((*tp).tm_year as libc::c_uint)
                            .wrapping_add(1900 as libc::c_int as libc::c_uint);
                        current_block = 658192006876172990;
                    }
                }
                121 => {
                    if modifier == 'E' as i32 {
                        current_block = 11524224095985028372;
                    } else {
                        let mut yy_0: libc::c_int = (*tp).tm_year % 100 as libc::c_int;
                        if yy_0 < 0 as libc::c_int {
                            yy_0 = if (*tp).tm_year < -(1900 as libc::c_int) {
                                -yy_0
                            } else {
                                yy_0 + 100 as libc::c_int
                            };
                        }
                        digits = 2 as libc::c_int;
                        negative_number = 0 as libc::c_int != 0;
                        u_number_value = yy_0 as libc::c_uint;
                        current_block = 658192006876172990;
                    }
                }
                90 => {
                    if change_case {
                        to_uppcase = 0 as libc::c_int != 0;
                        to_lowcase = 1 as libc::c_int != 0;
                    }
                    let mut _n_9: size_t = strlen(zone);
                    let mut _w_9: size_t = (if pad == '-' as i32
                        || width < 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_9: size_t = if _n_9 < _w_9 { _w_9 } else { _n_9 };
                    if _incr_9 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    if !p.is_null() {
                        if _n_9 < _w_9 {
                            let mut _delta_9: size_t = _w_9.wrapping_sub(_n_9);
                            if pad == '0' as i32 || pad == '+' as i32 {
                                let mut _i_20: size_t = 0;
                                _i_20 = 0 as libc::c_int as size_t;
                                while _i_20 < _delta_9 {
                                    fputc('0' as i32, p);
                                    _i_20 = _i_20.wrapping_add(1);
                                    _i_20;
                                }
                            } else {
                                let mut _i_21: size_t = 0;
                                _i_21 = 0 as libc::c_int as size_t;
                                while _i_21 < _delta_9 {
                                    fputc(' ' as i32, p);
                                    _i_21 = _i_21.wrapping_add(1);
                                    _i_21;
                                }
                            }
                        }
                        if to_lowcase {
                            fwrite_lowcase(p, zone, _n_9);
                        } else if to_uppcase {
                            fwrite_uppcase(p, zone, _n_9);
                        } else {
                            fwrite(
                                zone as *const libc::c_void,
                                _n_9,
                                1 as libc::c_int as libc::c_ulong,
                                p,
                            );
                        }
                    }
                    i = (i as libc::c_ulong).wrapping_add(_incr_9) as size_t as size_t;
                    current_block = 1394248824506584008;
                }
                58 => {
                    colons = 1 as libc::c_int as size_t;
                    while *f.offset(colons as isize) as libc::c_int == ':' as i32 {
                        colons = colons.wrapping_add(1);
                        colons;
                    }
                    if *f.offset(colons as isize) as libc::c_int != 'z' as i32 {
                        current_block = 4780816269647783540;
                    } else {
                        f = f.offset(colons as isize);
                        current_block = 13770406105293266558;
                    }
                }
                122 => {
                    colons = 0 as libc::c_int as size_t;
                    current_block = 13770406105293266558;
                }
                0 => {
                    f = f.offset(-1);
                    f;
                    current_block = 4780816269647783540;
                }
                _ => {
                    current_block = 4780816269647783540;
                }
            }
            match current_block {
                1394248824506584008 => {}
                _ => {
                    match current_block {
                        658192006876172990 => {
                            if pad == 0 as libc::c_int {
                                pad = yr_spec;
                            }
                            always_output_a_sign = pad == '+' as i32
                                && (((if digits == 2 as libc::c_int {
                                    99 as libc::c_int
                                } else {
                                    9999 as libc::c_int
                                }) as libc::c_uint) < u_number_value || digits < width);
                            current_block = 13125679426215309247;
                        }
                        13770406105293266558 => {
                            if (*tp).tm_isdst < 0 as libc::c_int {
                                current_block = 1394248824506584008;
                            } else {
                                let mut diff: libc::c_int = 0;
                                let mut hour_diff: libc::c_int = 0;
                                let mut min_diff: libc::c_int = 0;
                                let mut sec_diff: libc::c_int = 0;
                                diff = (*tp).tm_gmtoff as libc::c_int;
                                negative_number = diff < 0 as libc::c_int
                                    || diff == 0 as libc::c_int
                                        && *zone as libc::c_int == '-' as i32;
                                hour_diff = diff / 60 as libc::c_int / 60 as libc::c_int;
                                min_diff = diff / 60 as libc::c_int % 60 as libc::c_int;
                                sec_diff = diff % 60 as libc::c_int;
                                match colons {
                                    0 => {
                                        current_block = 11256224404817644654;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 17854291963920166136;
                                            }
                                            7697959209126204083 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 17854291963920166136;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 12497294037349282726;
                                    }
                                    1 => {
                                        current_block = 9136364834649731412;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 17854291963920166136;
                                            }
                                            7697959209126204083 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 17854291963920166136;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 12497294037349282726;
                                    }
                                    2 => {
                                        current_block = 13364777937596457114;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 17854291963920166136;
                                            }
                                            7697959209126204083 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 17854291963920166136;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 12497294037349282726;
                                    }
                                    3 => {
                                        current_block = 7697959209126204083;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 17854291963920166136;
                                            }
                                            7697959209126204083 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 17854291963920166136;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 12497294037349282726;
                                    }
                                    _ => {
                                        current_block = 4780816269647783540;
                                    }
                                }
                            }
                        }
                        12711609573701009411 => {
                            if change_case {
                                to_uppcase = 0 as libc::c_int != 0;
                                to_lowcase = 1 as libc::c_int != 0;
                            }
                            current_block = 11524224095985028372;
                        }
                        2746649332913740001 => {
                            if pad == 0 as libc::c_int {
                                pad = '_' as i32;
                            }
                            current_block = 1180518703837991265;
                        }
                        12246015723122908320 => {
                            subwidth = -(1 as libc::c_int);
                            current_block = 14207246619195705495;
                        }
                        _ => {}
                    }
                    match current_block {
                        1394248824506584008 => {}
                        _ => {
                            match current_block {
                                1180518703837991265 => {
                                    negative_number = number_value < 0 as libc::c_int;
                                    u_number_value = number_value as libc::c_uint;
                                    current_block = 7056221854519279815;
                                }
                                4780816269647783540 => {
                                    let mut flen: libc::c_int = 0;
                                    flen = 1 as libc::c_int;
                                    while *f.offset((1 as libc::c_int - flen) as isize)
                                        as libc::c_int != '%' as i32
                                    {
                                        flen += 1;
                                        flen;
                                    }
                                    let mut _n_10: size_t = flen as size_t;
                                    let mut _w_10: size_t = (if pad == '-' as i32
                                        || width < 0 as libc::c_int
                                    {
                                        0 as libc::c_int
                                    } else {
                                        width
                                    }) as size_t;
                                    let mut _incr_10: size_t = if _n_10 < _w_10 {
                                        _w_10
                                    } else {
                                        _n_10
                                    };
                                    if _incr_10 >= maxsize.wrapping_sub(i) {
                                        *__errno_location() = 34 as libc::c_int;
                                        return 0 as libc::c_int as size_t;
                                    }
                                    if !p.is_null() {
                                        if _n_10 < _w_10 {
                                            let mut _delta_10: size_t = _w_10.wrapping_sub(_n_10);
                                            if pad == '0' as i32 || pad == '+' as i32 {
                                                let mut _i_22: size_t = 0;
                                                _i_22 = 0 as libc::c_int as size_t;
                                                while _i_22 < _delta_10 {
                                                    fputc('0' as i32, p);
                                                    _i_22 = _i_22.wrapping_add(1);
                                                    _i_22;
                                                }
                                            } else {
                                                let mut _i_23: size_t = 0;
                                                _i_23 = 0 as libc::c_int as size_t;
                                                while _i_23 < _delta_10 {
                                                    fputc(' ' as i32, p);
                                                    _i_23 = _i_23.wrapping_add(1);
                                                    _i_23;
                                                }
                                            }
                                        }
                                        if to_lowcase {
                                            fwrite_lowcase(
                                                p,
                                                &*f.offset((1 as libc::c_int - flen) as isize),
                                                _n_10,
                                            );
                                        } else if to_uppcase {
                                            fwrite_uppcase(
                                                p,
                                                &*f.offset((1 as libc::c_int - flen) as isize),
                                                _n_10,
                                            );
                                        } else {
                                            fwrite(
                                                &*f.offset((1 as libc::c_int - flen) as isize)
                                                    as *const libc::c_char as *const libc::c_void,
                                                _n_10,
                                                1 as libc::c_int as libc::c_ulong,
                                                p,
                                            );
                                        }
                                    }
                                    i = (i as libc::c_ulong).wrapping_add(_incr_10) as size_t
                                        as size_t;
                                    current_block = 1394248824506584008;
                                }
                                14207246619195705495 => {
                                    let mut len: size_t = __strftime_internal(
                                        0 as *mut FILE,
                                        subfmt,
                                        tp,
                                        to_uppcase,
                                        pad,
                                        subwidth,
                                        tzset_called,
                                        tz,
                                        ns,
                                    );
                                    let mut _n_1: size_t = len;
                                    let mut _w_1: size_t = (if pad == '-' as i32
                                        || width < 0 as libc::c_int
                                    {
                                        0 as libc::c_int
                                    } else {
                                        width
                                    }) as size_t;
                                    let mut _incr_1: size_t = if _n_1 < _w_1 {
                                        _w_1
                                    } else {
                                        _n_1
                                    };
                                    if _incr_1 >= maxsize.wrapping_sub(i) {
                                        *__errno_location() = 34 as libc::c_int;
                                        return 0 as libc::c_int as size_t;
                                    }
                                    if !p.is_null() {
                                        if _n_1 < _w_1 {
                                            let mut _delta_1: size_t = _w_1.wrapping_sub(_n_1);
                                            if pad == '0' as i32 || pad == '+' as i32 {
                                                let mut _i_3: size_t = 0;
                                                _i_3 = 0 as libc::c_int as size_t;
                                                while _i_3 < _delta_1 {
                                                    fputc('0' as i32, p);
                                                    _i_3 = _i_3.wrapping_add(1);
                                                    _i_3;
                                                }
                                            } else {
                                                let mut _i_4: size_t = 0;
                                                _i_4 = 0 as libc::c_int as size_t;
                                                while _i_4 < _delta_1 {
                                                    fputc(' ' as i32, p);
                                                    _i_4 = _i_4.wrapping_add(1);
                                                    _i_4;
                                                }
                                            }
                                        }
                                        __strftime_internal(
                                            p,
                                            subfmt,
                                            tp,
                                            to_uppcase,
                                            pad,
                                            subwidth,
                                            tzset_called,
                                            tz,
                                            ns,
                                        );
                                    }
                                    i = (i as libc::c_ulong).wrapping_add(_incr_1) as size_t
                                        as size_t;
                                    current_block = 1394248824506584008;
                                }
                                _ => {}
                            }
                            match current_block {
                                1394248824506584008 => {}
                                _ => {
                                    match current_block {
                                        7056221854519279815 => {
                                            always_output_a_sign = 0 as libc::c_int != 0;
                                            current_block = 13125679426215309247;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        13125679426215309247 => {
                                            tz_colon_mask = 0 as libc::c_int;
                                            current_block = 12497294037349282726;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        12497294037349282726 => {
                                            if modifier == 'O' as i32 && !negative_number {
                                                current_block = 11524224095985028372;
                                            } else {
                                                bufp = buf
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (::std::mem::size_of::<[libc::c_char; 23]>()
                                                            as libc::c_ulong)
                                                            .wrapping_div(
                                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                            ) as isize,
                                                    );
                                                if negative_number {
                                                    u_number_value = u_number_value.wrapping_neg();
                                                }
                                                loop {
                                                    if tz_colon_mask & 1 as libc::c_int != 0 {
                                                        bufp = bufp.offset(-1);
                                                        *bufp = ':' as i32 as libc::c_char;
                                                    }
                                                    tz_colon_mask >>= 1 as libc::c_int;
                                                    bufp = bufp.offset(-1);
                                                    *bufp = u_number_value
                                                        .wrapping_rem(10 as libc::c_int as libc::c_uint)
                                                        .wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
                                                    u_number_value = u_number_value
                                                        .wrapping_div(10 as libc::c_int as libc::c_uint);
                                                    if !(u_number_value != 0 as libc::c_int as libc::c_uint
                                                        || tz_colon_mask != 0 as libc::c_int)
                                                    {
                                                        break;
                                                    }
                                                }
                                                current_block = 1997968267199582064;
                                            }
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        11524224095985028372 => {
                                            let mut ufmt: [libc::c_char; 5] = [0; 5];
                                            let mut u: *mut libc::c_char = ufmt.as_mut_ptr();
                                            let mut ubuf: [libc::c_char; 1024] = [0; 1024];
                                            let mut len_0: size_t = 0;
                                            let fresh5 = u;
                                            u = u.offset(1);
                                            *fresh5 = ' ' as i32 as libc::c_char;
                                            let fresh6 = u;
                                            u = u.offset(1);
                                            *fresh6 = '%' as i32 as libc::c_char;
                                            if modifier != 0 as libc::c_int {
                                                let fresh7 = u;
                                                u = u.offset(1);
                                                *fresh7 = modifier as libc::c_char;
                                            }
                                            let fresh8 = u;
                                            u = u.offset(1);
                                            *fresh8 = format_char as libc::c_char;
                                            *u = '\0' as i32 as libc::c_char;
                                            len_0 = strftime(
                                                ubuf.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                    as libc::c_ulong,
                                                ufmt.as_mut_ptr(),
                                                tp,
                                            );
                                            if len_0 != 0 as libc::c_int as libc::c_ulong {
                                                let mut _n_2: size_t = len_0
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                                                let mut _w_2: size_t = (if pad == '-' as i32
                                                    || width < 0 as libc::c_int
                                                {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) as size_t;
                                                let mut _incr_2: size_t = if _n_2 < _w_2 {
                                                    _w_2
                                                } else {
                                                    _n_2
                                                };
                                                if _incr_2 >= maxsize.wrapping_sub(i) {
                                                    *__errno_location() = 34 as libc::c_int;
                                                    return 0 as libc::c_int as size_t;
                                                }
                                                if !p.is_null() {
                                                    if _n_2 < _w_2 {
                                                        let mut _delta_2: size_t = _w_2.wrapping_sub(_n_2);
                                                        if pad == '0' as i32 || pad == '+' as i32 {
                                                            let mut _i_5: size_t = 0;
                                                            _i_5 = 0 as libc::c_int as size_t;
                                                            while _i_5 < _delta_2 {
                                                                fputc('0' as i32, p);
                                                                _i_5 = _i_5.wrapping_add(1);
                                                                _i_5;
                                                            }
                                                        } else {
                                                            let mut _i_6: size_t = 0;
                                                            _i_6 = 0 as libc::c_int as size_t;
                                                            while _i_6 < _delta_2 {
                                                                fputc(' ' as i32, p);
                                                                _i_6 = _i_6.wrapping_add(1);
                                                                _i_6;
                                                            }
                                                        }
                                                    }
                                                    if to_lowcase {
                                                        fwrite_lowcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as libc::c_int as isize),
                                                            _n_2,
                                                        );
                                                    } else if to_uppcase {
                                                        fwrite_uppcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as libc::c_int as isize),
                                                            _n_2,
                                                        );
                                                    } else {
                                                        fwrite(
                                                            ubuf.as_mut_ptr().offset(1 as libc::c_int as isize)
                                                                as *const libc::c_void,
                                                            _n_2,
                                                            1 as libc::c_int as libc::c_ulong,
                                                            p,
                                                        );
                                                    }
                                                }
                                                i = (i as libc::c_ulong).wrapping_add(_incr_2) as size_t
                                                    as size_t;
                                            }
                                        }
                                        _ => {
                                            if pad == 0 as libc::c_int {
                                                pad = '0' as i32;
                                            }
                                            if width < 0 as libc::c_int {
                                                width = digits;
                                            }
                                            let mut sign_char: libc::c_char = (if negative_number
                                                as libc::c_int != 0
                                            {
                                                '-' as i32
                                            } else if always_output_a_sign as libc::c_int != 0 {
                                                '+' as i32
                                            } else {
                                                0 as libc::c_int
                                            }) as libc::c_char;
                                            let mut numlen: libc::c_int = buf
                                                .as_mut_ptr()
                                                .offset(
                                                    (::std::mem::size_of::<[libc::c_char; 23]>()
                                                        as libc::c_ulong)
                                                        .wrapping_div(
                                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                        ) as isize,
                                                )
                                                .offset_from(bufp) as libc::c_long as libc::c_int;
                                            let mut shortage: libc::c_int = width
                                                - (sign_char != 0) as libc::c_int - numlen;
                                            let mut padding: libc::c_int = if pad == '-' as i32
                                                || shortage <= 0 as libc::c_int
                                            {
                                                0 as libc::c_int
                                            } else {
                                                shortage
                                            };
                                            if sign_char != 0 {
                                                if pad == '_' as i32 {
                                                    if !p.is_null() {
                                                        let mut _i_7: size_t = 0;
                                                        _i_7 = 0 as libc::c_int as size_t;
                                                        while _i_7 < padding as libc::c_ulong {
                                                            fputc(' ' as i32, p);
                                                            _i_7 = _i_7.wrapping_add(1);
                                                            _i_7;
                                                        }
                                                    }
                                                    i = (i as libc::c_ulong)
                                                        .wrapping_add(padding as libc::c_ulong) as size_t as size_t;
                                                    width -= padding;
                                                }
                                                let mut _n_3: size_t = 1 as libc::c_int as size_t;
                                                let mut _w_3: size_t = (if pad == '-' as i32
                                                    || (0 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    0 as libc::c_int
                                                } else {
                                                    0 as libc::c_int
                                                }) as size_t;
                                                let mut _incr_3: size_t = if _n_3 < _w_3 {
                                                    _w_3
                                                } else {
                                                    _n_3
                                                };
                                                if _incr_3 >= maxsize.wrapping_sub(i) {
                                                    *__errno_location() = 34 as libc::c_int;
                                                    return 0 as libc::c_int as size_t;
                                                }
                                                if !p.is_null() {
                                                    if _n_3 < _w_3 {
                                                        let mut _delta_3: size_t = _w_3.wrapping_sub(_n_3);
                                                        if pad == '0' as i32 || pad == '+' as i32 {
                                                            let mut _i_8: size_t = 0;
                                                            _i_8 = 0 as libc::c_int as size_t;
                                                            while _i_8 < _delta_3 {
                                                                fputc('0' as i32, p);
                                                                _i_8 = _i_8.wrapping_add(1);
                                                                _i_8;
                                                            }
                                                        } else {
                                                            let mut _i_9: size_t = 0;
                                                            _i_9 = 0 as libc::c_int as size_t;
                                                            while _i_9 < _delta_3 {
                                                                fputc(' ' as i32, p);
                                                                _i_9 = _i_9.wrapping_add(1);
                                                                _i_9;
                                                            }
                                                        }
                                                    }
                                                    fputc(sign_char as libc::c_int, p);
                                                }
                                                i = (i as libc::c_ulong).wrapping_add(_incr_3) as size_t
                                                    as size_t;
                                                width -= 1;
                                                width;
                                            }
                                            let mut _n_4: size_t = numlen as size_t;
                                            let mut _w_4: size_t = (if pad == '-' as i32
                                                || width < 0 as libc::c_int
                                            {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) as size_t;
                                            let mut _incr_4: size_t = if _n_4 < _w_4 {
                                                _w_4
                                            } else {
                                                _n_4
                                            };
                                            if _incr_4 >= maxsize.wrapping_sub(i) {
                                                *__errno_location() = 34 as libc::c_int;
                                                return 0 as libc::c_int as size_t;
                                            }
                                            if !p.is_null() {
                                                if _n_4 < _w_4 {
                                                    let mut _delta_4: size_t = _w_4.wrapping_sub(_n_4);
                                                    if pad == '0' as i32 || pad == '+' as i32 {
                                                        let mut _i_10: size_t = 0;
                                                        _i_10 = 0 as libc::c_int as size_t;
                                                        while _i_10 < _delta_4 {
                                                            fputc('0' as i32, p);
                                                            _i_10 = _i_10.wrapping_add(1);
                                                            _i_10;
                                                        }
                                                    } else {
                                                        let mut _i_11: size_t = 0;
                                                        _i_11 = 0 as libc::c_int as size_t;
                                                        while _i_11 < _delta_4 {
                                                            fputc(' ' as i32, p);
                                                            _i_11 = _i_11.wrapping_add(1);
                                                            _i_11;
                                                        }
                                                    }
                                                }
                                                if to_lowcase {
                                                    fwrite_lowcase(p, bufp, _n_4);
                                                } else if to_uppcase {
                                                    fwrite_uppcase(p, bufp, _n_4);
                                                } else {
                                                    fwrite(
                                                        bufp as *const libc::c_void,
                                                        _n_4,
                                                        1 as libc::c_int as libc::c_ulong,
                                                        p,
                                                    );
                                                }
                                            }
                                            i = (i as libc::c_ulong).wrapping_add(_incr_4) as size_t
                                                as size_t;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        width = -(1 as libc::c_int);
        f = f.offset(1);
        f;
    }
    *__errno_location() = saved_errno;
    return i;
}
