use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct longhelp_entry {
    pub options: *mut libc::c_char,
    pub description: *mut libc::c_char,
}
unsafe extern "C" fn remove_quotes(mut buff: *mut libc::c_char) {
    let mut k: libc::c_int = 0;
    while *buff != 0 {
        k = 0 as libc::c_int;
        while *buff.offset(k as isize) as libc::c_int == '"' as i32 {
            k += 1;
            k;
        }
        if k != 0 {
            memmove(
                buff as *mut libc::c_void,
                buff.offset(k as isize) as *const libc::c_void,
                (strlen(buff))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(k as libc::c_ulong),
            );
        } else {
            buff = buff.offset(1);
            buff;
        }
    }
}
unsafe extern "C" fn char_is_space(mut c: libc::c_int) -> libc::c_int {
    if c == 32 as libc::c_int || c == 13 as libc::c_int || c == 10 as libc::c_int
        || c == 9 as libc::c_int
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn tokenize_spaces_dyn(
    mut buff: *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut intoken: libc::c_int = 0;
    let mut inquota: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nm: libc::c_int = 0;
    let mut rtokens: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    nm = 16 as libc::c_int;
    rtokens = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nm as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if rtokens.is_null() && 1 as libc::c_int > 0 as libc::c_int {
        fprintf(
            stderr,
            b"tokenize.c: %s.\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    if rtokens.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    intoken = 0 as libc::c_int;
    inquota = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while *buff != 0 {
        if char_is_space(*buff as libc::c_int) == 0 && intoken == 0 {
            let ref mut fresh0 = *rtokens.offset(n as isize);
            *fresh0 = buff;
            intoken = (0 as libc::c_int == 0) as libc::c_int;
            inquota = 0 as libc::c_int;
            n += 1;
            n;
            if *buff as libc::c_int == '"' as i32 {
                inquota = (0 as libc::c_int == 0) as libc::c_int;
            }
            buff = buff.offset(1);
            buff;
            if n >= nm - 1 as libc::c_int {
                nm += 16 as libc::c_int;
                rtokens = realloc(
                    rtokens as *mut libc::c_void,
                    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(nm as libc::c_ulong),
                ) as *mut *mut libc::c_char;
                if rtokens.is_null()
                    && (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(nm as libc::c_ulong)
                        > 0 as libc::c_int as libc::c_ulong
                {
                    fprintf(
                        stderr,
                        b"tokenize.c: %s.\n\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"memory exhausted\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    abort();
                }
            }
        } else if intoken != 0
            && (char_is_space(*buff as libc::c_int) != 0 && inquota != 0
                || char_is_space(*buff as libc::c_int) == 0)
        {
            if *buff as libc::c_int == '"' as i32 {
                inquota = (inquota == 0) as libc::c_int;
            }
            buff = buff.offset(1);
            buff;
        } else if intoken != 0 && inquota == 0
            && char_is_space(*buff as libc::c_int) != 0
        {
            *buff = 0 as libc::c_int as libc::c_char;
            buff = buff.offset(1);
            buff;
            intoken = 0 as libc::c_int;
        } else {
            buff = buff.offset(1);
            buff;
        }
    }
    let ref mut fresh1 = *rtokens.offset(n as isize);
    *fresh1 = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < n {
        remove_quotes(*rtokens.offset(i as isize));
        i += 1;
        i;
    }
    return rtokens;
}
unsafe extern "C" fn longhelp_fprint_description(
    mut fw: *mut FILE,
    mut width: libc::c_int,
    mut w: libc::c_int,
    mut fpad: libc::c_int,
    mut pad: libc::c_int,
    mut desc: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = w;
    while i < pad {
        fprintf(fw, b" \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    if width <= 0 as libc::c_int {
        fprintf(fw, b"%s\n\0" as *const u8 as *const libc::c_char, desc);
    } else {
        let mut dd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cmd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut n: libc::c_int = 0;
        let mut w_0: libc::c_int = 0;
        let mut p: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        dd = strdup(desc);
        cmd = tokenize_spaces_dyn(dd);
        p = pad;
        w_0 = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while !cmd.is_null() && !(*cmd.offset(n as isize)).is_null() {
            l = strlen(*cmd.offset(n as isize)) as libc::c_int;
            if w_0 > 0 as libc::c_int {
                l += 1;
                l;
            }
            if l > width - p {
                fprintf(fw, b"\n\0" as *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                while i < fpad {
                    fprintf(fw, b" \0" as *const u8 as *const libc::c_char);
                    i += 1;
                    i;
                }
                if w_0 > 0 as libc::c_int {
                    l -= 1;
                    l;
                }
                w_0 = 0 as libc::c_int;
                p = fpad;
            }
            if w_0 > 0 as libc::c_int {
                fprintf(
                    fw,
                    b" %s\0" as *const u8 as *const libc::c_char,
                    *cmd.offset(n as isize),
                );
            } else {
                fprintf(
                    fw,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    *cmd.offset(n as isize),
                );
            }
            p += l;
            w_0 += 1;
            w_0;
            n += 1;
            n;
        }
        fprintf(fw, b"\n\0" as *const u8 as *const libc::c_char);
        if !cmd.is_null() {
            free(cmd as *mut libc::c_void);
        }
        free(dd as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn longhelp_fprint_entry(
    mut fw: *mut FILE,
    mut entry: *mut longhelp_entry,
    mut flags: libc::c_int,
    mut width: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut fpad: libc::c_int = 0;
    let mut pad: libc::c_int = 0;
    w = fprintf(fw, b" %s\0" as *const u8 as *const libc::c_char, (*entry).options);
    pad = w + 4 as libc::c_int + 7 as libc::c_int & !(7 as libc::c_int);
    fpad = 16 as libc::c_int;
    if width > 0 as libc::c_int {
        longhelp_fprint_description(fw, width, w, fpad, pad, (*entry).description);
    } else {
        while w < pad {
            fprintf(fw, b" \0" as *const u8 as *const libc::c_char);
            w += 1;
            w;
        }
        fprintf(fw, b"%s\n\0" as *const u8 as *const libc::c_char, (*entry).description);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn longhelp_fprint(
    mut fw: *mut FILE,
    mut entry: *mut longhelp_entry,
    mut flags: libc::c_int,
    mut width: libc::c_int,
) -> libc::c_int {
    let mut lcnt: libc::c_int = 0;
    if width < 0 as libc::c_int && isatty(fileno(fw)) != 0 {
        let mut ws: winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        if ioctl(
            fileno(fw),
            0x5413 as libc::c_int as libc::c_ulong,
            &mut ws as *mut winsize,
        ) == 0
        {
            width = ws.ws_col as libc::c_int - 1 as libc::c_int;
        } else {
            width = 0 as libc::c_int;
        }
    }
    lcnt = 0 as libc::c_int;
    while !entry.is_null() && !((*entry).options).is_null() {
        if ((*entry).description).is_null() {
            if lcnt > 0 as libc::c_int {
                fprintf(fw, b"\n\0" as *const u8 as *const libc::c_char);
            }
            fprintf(fw, b"%s\n\0" as *const u8 as *const libc::c_char, (*entry).options);
            lcnt = 0 as libc::c_int;
        } else {
            longhelp_fprint_entry(fw, entry, flags, width);
            lcnt += 1;
            lcnt;
        }
        entry = entry.offset(1);
        entry;
    }
    return 0 as libc::c_int;
}
