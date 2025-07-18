use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn abort() -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
unsafe extern "C" fn format_int_realloc(
    mut out: *mut *mut libc::c_char,
    mut len: *mut libc::c_int,
    mut alen: *mut libc::c_int,
    mut req: libc::c_int,
) -> libc::c_int {
    if *len + req > *alen {
        req += *len;
        *alen = 256 as libc::c_int
            * ((req + 256 as libc::c_int - 1 as libc::c_int) / 256 as libc::c_int);
        *out = realloc(*out as *mut libc::c_void, *alen as libc::c_ulong)
            as *mut libc::c_char;
        if (*out).is_null() && *alen > 0 as libc::c_int {
            fprintf(
                stderr,
                b"format.c: %s.\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn format_get_parameters(
    mut format: *mut libc::c_char,
    mut d1: *mut libc::c_int,
    mut d2: *mut libc::c_int,
    mut fmt: *mut libc::c_int,
    mut next: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut set: libc::c_int = 0;
    let mut fstart: *mut libc::c_char = 0 as *mut libc::c_char;
    fstart = format;
    set = 0 as libc::c_int;
    while *(*__ctype_b_loc()).offset(*format as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        format = format.offset(1);
        format;
    }
    if *format as libc::c_int != 0
        && sscanf(format, b"%d\0" as *const u8 as *const libc::c_char, d1)
            == 1 as libc::c_int
    {
        set |= 1 as libc::c_int;
        while *format as libc::c_int == '-' as i32
            || *(*__ctype_b_loc()).offset(*format as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            format = format.offset(1);
            format;
        }
        while *(*__ctype_b_loc()).offset(*format as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            format = format.offset(1);
            format;
        }
    }
    if *format as libc::c_int == '.' as i32 {
        format = format.offset(1);
        format;
    }
    while *(*__ctype_b_loc()).offset(*format as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        format = format.offset(1);
        format;
    }
    if *format as libc::c_int != 0
        && sscanf(format, b"%d\0" as *const u8 as *const libc::c_char, d2)
            == 1 as libc::c_int
    {
        set |= 2 as libc::c_int;
        while *format as libc::c_int == '-' as i32
            || *(*__ctype_b_loc()).offset(*format as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            format = format.offset(1);
            format;
        }
        while *(*__ctype_b_loc()).offset(*format as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            format = format.offset(1);
            format;
        }
    }
    if *(*__ctype_b_loc()).offset(*format as libc::c_int as isize) as libc::c_int
        & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *fmt = *format as libc::c_int;
        format = format.offset(1);
        format;
        if !next.is_null() {
            *next = format;
        }
        return set;
    } else {
        if !next.is_null() {
            *next = format;
        }
        return -(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn format_check_if_formatted(
    mut format: *mut libc::c_char,
    mut fchars: *mut libc::c_char,
) -> libc::c_int {
    let mut fmt: libc::c_int = 0;
    let mut d1: libc::c_int = 0;
    let mut d2: libc::c_int = 0;
    let mut set: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    while *format != 0 {
        if *format as libc::c_int == '%' as i32
            && *format.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            format = format.offset(1);
            format;
            set = format_get_parameters(format, &mut d1, &mut d2, &mut fmt, &mut next);
            if set >= 0 as libc::c_int {
                format = next;
            } else if *format as libc::c_int == '%' as i32 {
                format = format.offset(1);
                format;
            }
            if !(strchr(fchars, fmt)).is_null() {
                return 1 as libc::c_int;
            }
        } else {
            format = format.offset(1);
            format;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn format_int_add_string(
    mut out: *mut *mut libc::c_char,
    mut len: *mut libc::c_int,
    mut alen: *mut libc::c_int,
    mut set: libc::c_int,
    mut d1: libc::c_int,
    mut vs: *mut libc::c_char,
) -> libc::c_int {
    let mut slen: libc::c_int = 0;
    let mut req: libc::c_int = 0;
    slen = strlen(vs) as libc::c_int;
    if set & 1 as libc::c_int != 0 && abs(d1) > slen {
        req = abs(d1);
    } else {
        req = slen;
    }
    format_int_realloc(out, len, alen, req);
    if set & 1 as libc::c_int != 0 && d1 > slen {
        memset(
            (*out).offset(*len as isize) as *mut libc::c_void,
            32 as libc::c_int,
            (d1 - slen) as libc::c_ulong,
        );
        strcpy((*out).offset(*len as isize).offset((d1 - slen) as isize), vs);
        *len += req;
    } else if set & 1 as libc::c_int != 0 && -d1 > slen {
        strcpy((*out).offset(*len as isize), vs);
        memset(
            (*out).offset(*len as isize).offset(slen as isize) as *mut libc::c_void,
            32 as libc::c_int,
            (-d1 - slen) as libc::c_ulong,
        );
        *(*out).offset((*len + req) as isize) = 0 as libc::c_int as libc::c_char;
        *len += req;
    } else {
        strcpy((*out).offset(*len as isize), vs);
        *len += slen;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn format_replace(
    mut format: *mut libc::c_char,
    mut is_escape: libc::c_int,
    mut args: ...
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fstart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut alen: libc::c_int = 0;
    let mut set: libc::c_int = 0;
    let mut d1: libc::c_int = 0;
    let mut d2: libc::c_int = 0;
    let mut fmt: libc::c_int = 0;
    let mut chr: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    alen = 256 as libc::c_int;
    out = malloc(alen as libc::c_ulong) as *mut libc::c_char;
    if out.is_null() && 1 as libc::c_int > 0 as libc::c_int {
        fprintf(
            stderr,
            b"format.c: %s.\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    *out.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    len = 0 as libc::c_int;
    while *format != 0 {
        if is_escape != 0 && *format as libc::c_int == '\\' as i32 {
            format = format.offset(1);
            format;
            format_int_realloc(&mut out, &mut len, &mut alen, 1 as libc::c_int);
            match *format as libc::c_int {
                110 => {
                    *out.offset(len as isize) = '\n' as i32 as libc::c_char;
                    len += 1;
                    len;
                }
                116 => {
                    *out.offset(len as isize) = '\t' as i32 as libc::c_char;
                    len += 1;
                    len;
                }
                114 => {
                    *out.offset(len as isize) = '\r' as i32 as libc::c_char;
                    len += 1;
                    len;
                }
                0 => {
                    *out.offset(len as isize) = '\\' as i32 as libc::c_char;
                    len += 1;
                    len;
                }
                _ => {
                    *out.offset(len as isize) = *format;
                    len += 1;
                    len;
                }
            }
            if *format != 0 {
                format = format.offset(1);
                format;
            }
        } else if *format as libc::c_int == '%' as i32
            && *format.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            fstart = format;
            format = format.offset(1);
            format;
            set = format_get_parameters(format, &mut d1, &mut d2, &mut fmt, &mut next);
            if set >= 0 as libc::c_int {
                ap = args.clone();
                loop {
                    chr = ap.arg::<libc::c_int>();
                    if !(chr > 0 as libc::c_int) {
                        break;
                    }
                    type_0 = ap.arg::<libc::c_int>();
                    if chr == fmt {
                        break;
                    }
                    match type_0 {
                        1 => {
                            ap.arg::<libc::c_int>();
                        }
                        2 => {
                            ap.arg::<*mut libc::c_char>();
                        }
                        _ => {}
                    }
                }
                if chr > 0 as libc::c_int {
                    let mut vi: libc::c_int = 0;
                    let mut vs: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut buff: [libc::c_char; 256] = [0; 256];
                    let mut sbuff: [libc::c_char; 16] = [0; 16];
                    match type_0 {
                        1 => {
                            vi = ap.arg::<libc::c_int>();
                            if set & 2 as libc::c_int != 0 && d2 > 0 as libc::c_int {
                                if d2 > 128 as libc::c_int {
                                    d2 = 128 as libc::c_int;
                                }
                                sprintf(
                                    sbuff.as_mut_ptr(),
                                    b"%%.%dd\0" as *const u8 as *const libc::c_char,
                                    d2,
                                );
                                sprintf(buff.as_mut_ptr(), sbuff.as_mut_ptr(), vi);
                            } else {
                                sprintf(
                                    buff.as_mut_ptr(),
                                    b"%d\0" as *const u8 as *const libc::c_char,
                                    vi,
                                );
                            }
                            format_int_add_string(
                                &mut out,
                                &mut len,
                                &mut alen,
                                set,
                                d1,
                                buff.as_mut_ptr(),
                            );
                        }
                        2 => {
                            vs = ap.arg::<*mut libc::c_char>();
                            if vs.is_null() {
                                vs = b"(null)\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            format_int_add_string(
                                &mut out,
                                &mut len,
                                &mut alen,
                                set,
                                d1,
                                vs,
                            );
                        }
                        _ => {}
                    }
                    format = next;
                } else {
                    format = format.offset(-1);
                    format;
                    format_int_realloc(
                        &mut out,
                        &mut len,
                        &mut alen,
                        next.offset_from(format) as libc::c_long as libc::c_int,
                    );
                    memcpy(
                        out.offset(len as isize) as *mut libc::c_void,
                        format as *const libc::c_void,
                        next.offset_from(format) as libc::c_long as libc::c_ulong,
                    );
                    len = (len as libc::c_long
                        + next.offset_from(format) as libc::c_long) as libc::c_int;
                    format = next;
                }
            } else if *format as libc::c_int == '%' as i32 {
                format = format.offset(1);
                format;
                format_int_realloc(&mut out, &mut len, &mut alen, 1 as libc::c_int);
                *out.offset(len as isize) = '%' as i32 as libc::c_char;
                len += 1;
                len;
            } else {
                format_int_realloc(&mut out, &mut len, &mut alen, 2 as libc::c_int);
                *out.offset(len as isize) = '%' as i32 as libc::c_char;
                *out.offset((len + 1 as libc::c_int) as isize) = *format;
                format = format.offset(1);
                format;
                len += 2 as libc::c_int;
            }
        } else {
            format_int_realloc(&mut out, &mut len, &mut alen, 1 as libc::c_int);
            *out.offset(len as isize) = *format;
            len += 1;
            len;
            format = format.offset(1);
            format;
        }
    }
    format_int_realloc(&mut out, &mut len, &mut alen, 1 as libc::c_int);
    *out.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return out;
}
