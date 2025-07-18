use ::libc;
extern "C" {
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type rstatus_t = libc::c_int;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;
pub unsafe extern "C" fn string_init(mut str: *mut string) {
    (*str).len = 0 as libc::c_int as uint32_t;
    (*str).data = 0 as *mut uint8_t;
}
pub unsafe extern "C" fn string_deinit(mut str: *mut string) {
    if !((*str).data).is_null() {
        _nc_free(
            (*str).data as *mut libc::c_void,
            b"nc_string.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
        );
        (*str).data = 0 as *mut uint8_t;
        string_init(str);
    }
}
pub unsafe extern "C" fn string_empty(mut str: *const string) -> bool {
    return (*str).len == 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn string_duplicate(
    mut dst: *mut string,
    mut src: *const string,
) -> rstatus_t {
    (*dst)
        .data = strndup(
        (*src).data as *mut libc::c_char,
        ((*src).len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut uint8_t;
    if ((*dst).data).is_null() {
        return -(3 as libc::c_int);
    }
    (*dst).len = (*src).len;
    *((*dst).data).offset((*dst).len as isize) = '\0' as i32 as uint8_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn string_copy(
    mut dst: *mut string,
    mut src: *const uint8_t,
    mut srclen: uint32_t,
) -> rstatus_t {
    (*dst)
        .data = strndup(
        src as *mut libc::c_char,
        srclen.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut uint8_t;
    if ((*dst).data).is_null() {
        return -(3 as libc::c_int);
    }
    (*dst).len = srclen;
    *((*dst).data).offset((*dst).len as isize) = '\0' as i32 as uint8_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn string_compare(
    mut s1: *const string,
    mut s2: *const string,
) -> libc::c_int {
    if (*s1).len != (*s2).len {
        return if (*s1).len > (*s2).len {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return strncmp(
        (*s1).data as *mut libc::c_char,
        (*s2).data as *mut libc::c_char,
        (*s1).len as size_t,
    );
}
static mut hex: *const libc::c_char = b"0123456789abcdef\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn _safe_utoa(
    mut _base: libc::c_int,
    mut val: uint64_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut base: uint32_t = _base as uint32_t;
    let fresh0 = buf;
    buf = buf.offset(-1);
    *fresh0 = 0 as libc::c_int as libc::c_char;
    loop {
        let fresh1 = buf;
        buf = buf.offset(-1);
        *fresh1 = *hex.offset(val.wrapping_rem(base as libc::c_ulong) as isize);
        val = (val as libc::c_ulong).wrapping_div(base as libc::c_ulong) as uint64_t
            as uint64_t;
        if !(val != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    return buf.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn _safe_itoa(
    mut base: libc::c_int,
    mut val: int64_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut orig_buf: *mut libc::c_char = buf;
    let is_neg: int32_t = (val < 0 as libc::c_int as libc::c_long) as libc::c_int;
    let fresh2 = buf;
    buf = buf.offset(-1);
    *fresh2 = 0 as libc::c_int as libc::c_char;
    if is_neg != 0 {
        val = -val;
    }
    if is_neg != 0 && base == 16 as libc::c_int {
        let mut ix: libc::c_int = 0;
        val -= 1 as libc::c_int as libc::c_long;
        ix = 0 as libc::c_int;
        while ix < 16 as libc::c_int {
            *buf.offset(-ix as isize) = '0' as i32 as libc::c_char;
            ix += 1;
            ix;
        }
    }
    loop {
        let fresh3 = buf;
        buf = buf.offset(-1);
        *fresh3 = *hex.offset((val % base as libc::c_long) as isize);
        val /= base as libc::c_long;
        if !(val != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    if is_neg != 0 && base == 10 as libc::c_int {
        let fresh4 = buf;
        buf = buf.offset(-1);
        *fresh4 = '-' as i32 as libc::c_char;
    }
    if is_neg != 0 && base == 16 as libc::c_int {
        let mut ix_0: libc::c_int = 0;
        buf = orig_buf.offset(-(1 as libc::c_int as isize));
        ix_0 = 0 as libc::c_int;
        while ix_0 < 16 as libc::c_int {
            match *buf as libc::c_int {
                48 => {
                    *buf = 'f' as i32 as libc::c_char;
                }
                49 => {
                    *buf = 'e' as i32 as libc::c_char;
                }
                50 => {
                    *buf = 'd' as i32 as libc::c_char;
                }
                51 => {
                    *buf = 'c' as i32 as libc::c_char;
                }
                52 => {
                    *buf = 'b' as i32 as libc::c_char;
                }
                53 => {
                    *buf = 'a' as i32 as libc::c_char;
                }
                54 => {
                    *buf = '9' as i32 as libc::c_char;
                }
                55 => {
                    *buf = '8' as i32 as libc::c_char;
                }
                56 => {
                    *buf = '7' as i32 as libc::c_char;
                }
                57 => {
                    *buf = '6' as i32 as libc::c_char;
                }
                97 => {
                    *buf = '5' as i32 as libc::c_char;
                }
                98 => {
                    *buf = '4' as i32 as libc::c_char;
                }
                99 => {
                    *buf = '3' as i32 as libc::c_char;
                }
                100 => {
                    *buf = '2' as i32 as libc::c_char;
                }
                101 => {
                    *buf = '1' as i32 as libc::c_char;
                }
                102 => {
                    *buf = '0' as i32 as libc::c_char;
                }
                _ => {}
            }
            ix_0 += 1;
            ix_0;
            buf = buf.offset(-1);
            buf;
        }
    }
    return buf.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn _safe_check_longlong(
    mut fmt: *const libc::c_char,
    mut have_longlong: *mut int32_t,
) -> *const libc::c_char {
    *have_longlong = 0 as libc::c_int;
    if *fmt as libc::c_int == 'l' as i32 {
        fmt = fmt.offset(1);
        fmt;
        if *fmt as libc::c_int != 'l' as i32 {
            *have_longlong = (::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                == ::std::mem::size_of::<int64_t>() as libc::c_ulong) as libc::c_int;
        } else {
            fmt = fmt.offset(1);
            fmt;
            *have_longlong = 1 as libc::c_int;
        }
    }
    return fmt;
}
pub unsafe extern "C" fn _safe_vsnprintf(
    mut to: *mut libc::c_char,
    mut size: size_t,
    mut format: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut start: *mut libc::c_char = to;
    let mut end: *mut libc::c_char = start
        .offset(size as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut current_block_29: u64;
    while *format != 0 {
        let mut have_longlong: int32_t = 0 as libc::c_int;
        if *format as libc::c_int != '%' as i32 {
            if to == end {
                break;
            }
            let fresh5 = to;
            to = to.offset(1);
            *fresh5 = *format;
        } else {
            format = format.offset(1);
            format;
            format = _safe_check_longlong(format, &mut have_longlong);
            match *format as libc::c_int {
                100 | 105 | 117 | 120 | 112 => {
                    current_block_29 = 7815301370352969686;
                    match current_block_29 {
                        4775909272756257391 => {
                            let mut val: *const libc::c_char = ap
                                .arg::<*mut libc::c_char>();
                            if val.is_null() {
                                val = b"(null)\0" as *const u8 as *const libc::c_char;
                            }
                            while *val as libc::c_int != 0 && to < end {
                                let fresh8 = val;
                                val = val.offset(1);
                                let fresh9 = to;
                                to = to.offset(1);
                                *fresh9 = *fresh8;
                            }
                        }
                        _ => {
                            let mut ival: int64_t = 0 as libc::c_int as int64_t;
                            let mut uval: uint64_t = 0 as libc::c_int as uint64_t;
                            if *format as libc::c_int == 'p' as i32 {
                                have_longlong = (::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                                    == ::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                                    as libc::c_int;
                            }
                            if have_longlong != 0 {
                                if *format as libc::c_int == 'u' as i32 {
                                    uval = ap.arg::<uint64_t>();
                                } else {
                                    ival = ap.arg::<int64_t>();
                                }
                            } else if *format as libc::c_int == 'u' as i32 {
                                uval = ap.arg::<uint32_t>() as uint64_t;
                            } else {
                                ival = ap.arg::<int32_t>() as int64_t;
                            }
                            let mut buff: [libc::c_char; 22] = [0; 22];
                            let base: libc::c_int = if *format as libc::c_int
                                == 'x' as i32 || *format as libc::c_int == 'p' as i32
                            {
                                16 as libc::c_int
                            } else {
                                10 as libc::c_int
                            };
                            let mut val_as_str: *mut libc::c_char = if *format
                                as libc::c_int == 'u' as i32
                            {
                                _safe_utoa(
                                    base,
                                    uval,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::std::mem::size_of::<[libc::c_char; 22]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ),
                                )
                            } else {
                                _safe_itoa(
                                    base,
                                    ival,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::std::mem::size_of::<[libc::c_char; 22]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ),
                                )
                            };
                            if *format as libc::c_int == 'x' as i32 && have_longlong == 0
                                && ival < 0 as libc::c_int as libc::c_long
                            {
                                val_as_str = val_as_str.offset(8 as libc::c_int as isize);
                            }
                            while *val_as_str as libc::c_int != 0 && to < end {
                                let fresh6 = val_as_str;
                                val_as_str = val_as_str.offset(1);
                                let fresh7 = to;
                                to = to.offset(1);
                                *fresh7 = *fresh6;
                            }
                        }
                    }
                }
                115 => {
                    current_block_29 = 4775909272756257391;
                    match current_block_29 {
                        4775909272756257391 => {
                            let mut val: *const libc::c_char = ap
                                .arg::<*mut libc::c_char>();
                            if val.is_null() {
                                val = b"(null)\0" as *const u8 as *const libc::c_char;
                            }
                            while *val as libc::c_int != 0 && to < end {
                                let fresh8 = val;
                                val = val.offset(1);
                                let fresh9 = to;
                                to = to.offset(1);
                                *fresh9 = *fresh8;
                            }
                        }
                        _ => {
                            let mut ival: int64_t = 0 as libc::c_int as int64_t;
                            let mut uval: uint64_t = 0 as libc::c_int as uint64_t;
                            if *format as libc::c_int == 'p' as i32 {
                                have_longlong = (::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                                    == ::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                                    as libc::c_int;
                            }
                            if have_longlong != 0 {
                                if *format as libc::c_int == 'u' as i32 {
                                    uval = ap.arg::<uint64_t>();
                                } else {
                                    ival = ap.arg::<int64_t>();
                                }
                            } else if *format as libc::c_int == 'u' as i32 {
                                uval = ap.arg::<uint32_t>() as uint64_t;
                            } else {
                                ival = ap.arg::<int32_t>() as int64_t;
                            }
                            let mut buff: [libc::c_char; 22] = [0; 22];
                            let base: libc::c_int = if *format as libc::c_int
                                == 'x' as i32 || *format as libc::c_int == 'p' as i32
                            {
                                16 as libc::c_int
                            } else {
                                10 as libc::c_int
                            };
                            let mut val_as_str: *mut libc::c_char = if *format
                                as libc::c_int == 'u' as i32
                            {
                                _safe_utoa(
                                    base,
                                    uval,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::std::mem::size_of::<[libc::c_char; 22]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ),
                                )
                            } else {
                                _safe_itoa(
                                    base,
                                    ival,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::std::mem::size_of::<[libc::c_char; 22]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ),
                                )
                            };
                            if *format as libc::c_int == 'x' as i32 && have_longlong == 0
                                && ival < 0 as libc::c_int as libc::c_long
                            {
                                val_as_str = val_as_str.offset(8 as libc::c_int as isize);
                            }
                            while *val_as_str as libc::c_int != 0 && to < end {
                                let fresh6 = val_as_str;
                                val_as_str = val_as_str.offset(1);
                                let fresh7 = to;
                                to = to.offset(1);
                                *fresh7 = *fresh6;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        format = format.offset(1);
        format;
    }
    *to = 0 as libc::c_int as libc::c_char;
    return to.offset_from(start) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn _safe_snprintf(
    mut to: *mut libc::c_char,
    mut n: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    result = _safe_vsnprintf(to, n, fmt, args_0.as_va_list());
    return result;
}
