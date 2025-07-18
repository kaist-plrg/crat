use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
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
pub unsafe extern "C" fn remove_newlines_and_comments(mut buff: *mut libc::c_char) {
    let mut k: libc::c_int = 0;
    while *buff != 0 {
        if *buff as libc::c_int == '#' as i32 {
            *buff = 0 as libc::c_int as libc::c_char;
        } else {
            k = 0 as libc::c_int;
            while *buff.offset(k as isize) as libc::c_int == 10 as libc::c_int
                || *buff.offset(k as isize) as libc::c_int == 13 as libc::c_int
            {
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
}
pub unsafe extern "C" fn remove_spaces_and_comments(mut buff: *mut libc::c_char) {
    let mut k: libc::c_int = 0;
    while *buff != 0 {
        if *buff as libc::c_int == '#' as i32 {
            *buff = 0 as libc::c_int as libc::c_char;
        } else {
            k = 0 as libc::c_int;
            while *buff.offset(k as isize) as libc::c_int == 9 as libc::c_int
                || *buff.offset(k as isize) as libc::c_int == 32 as libc::c_int
                || *buff.offset(k as isize) as libc::c_int == 10 as libc::c_int
                || *buff.offset(k as isize) as libc::c_int == 13 as libc::c_int
            {
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
}
pub unsafe extern "C" fn remove_spaces(mut buff: *mut libc::c_char) {
    let mut k: libc::c_int = 0;
    while *buff != 0 {
        k = 0 as libc::c_int;
        while *buff.offset(k as isize) as libc::c_int == 9 as libc::c_int
            || *buff.offset(k as isize) as libc::c_int == 32 as libc::c_int
            || *buff.offset(k as isize) as libc::c_int == 10 as libc::c_int
            || *buff.offset(k as isize) as libc::c_int == 13 as libc::c_int
        {
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
pub unsafe extern "C" fn remove_quotes(mut buff: *mut libc::c_char) {
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
pub unsafe extern "C" fn char_is_space(mut c: libc::c_int) -> libc::c_int {
    if c == 32 as libc::c_int || c == 13 as libc::c_int || c == 10 as libc::c_int
        || c == 9 as libc::c_int
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn tokenize_spaces(
    mut buff: *mut libc::c_char,
    mut tokens: *mut *mut libc::c_char,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut intoken: libc::c_int = 0;
    let mut inquota: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tsave: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    tsave = tokens;
    intoken = 0 as libc::c_int;
    inquota = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while *buff as libc::c_int != 0 && n < max {
        if char_is_space(*buff as libc::c_int) == 0 && intoken == 0 {
            *tokens = buff;
            intoken = (0 as libc::c_int == 0) as libc::c_int;
            inquota = 0 as libc::c_int;
            n += 1;
            n;
            if *buff as libc::c_int == '"' as i32 {
                inquota = (0 as libc::c_int == 0) as libc::c_int;
            }
            tokens = tokens.offset(1);
            tokens;
            buff = buff.offset(1);
            buff;
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
    *tokens = 0 as *mut libc::c_char;
    while !(*tsave).is_null() {
        remove_quotes(*tsave);
        tsave = tsave.offset(1);
        tsave;
    }
    return n;
}
pub unsafe extern "C" fn tokenize_spaces_dyn(
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
pub unsafe extern "C" fn tokenize_char(
    mut buff: *mut libc::c_char,
    mut tokens: *mut *mut libc::c_char,
    mut tchar: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if *buff as libc::c_int == 0 as libc::c_int {
        *tokens = 0 as *mut libc::c_char;
        return 0 as libc::c_int;
    }
    n = 1 as libc::c_int;
    *tokens = buff;
    tokens = tokens.offset(1);
    tokens;
    while *buff as libc::c_int != 0 && n < max {
        if *buff as libc::c_int != tchar {
            buff = buff.offset(1);
            buff;
        } else {
            *buff = 0 as libc::c_int as libc::c_char;
            buff = buff.offset(1);
            buff;
            *tokens = buff;
            tokens = tokens.offset(1);
            tokens;
            n += 1;
            n;
        }
    }
    *tokens = 0 as *mut libc::c_char;
    return n;
}
pub unsafe extern "C" fn tokenize_char_dyn_wwt(
    mut buff: *mut libc::c_char,
    mut tchar: libc::c_int,
    mut is_terminate: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut tokens: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if buff.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    tokens = malloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as *mut *mut libc::c_char;
    if tokens.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    if *buff as libc::c_int == 0 as libc::c_int {
        *tokens = 0 as *mut libc::c_char;
        return tokens;
    }
    n = 0 as libc::c_int;
    let ref mut fresh2 = *tokens.offset(n as isize);
    *fresh2 = buff;
    n += 1;
    n;
    while *buff != 0 {
        if *buff as libc::c_int != tchar {
            buff = buff.offset(1);
            buff;
        } else {
            if is_terminate != 0 {
                *buff = 0 as libc::c_int as libc::c_char;
            }
            buff = buff.offset(1);
            buff;
            tokens = realloc(
                tokens as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((n + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            if tokens.is_null() && 1 as libc::c_int > 0 as libc::c_int {
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
            let ref mut fresh3 = *tokens.offset(n as isize);
            *fresh3 = buff;
            n += 1;
            n;
        }
    }
    tokens = realloc(
        tokens as *mut libc::c_void,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((n + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if tokens.is_null() && 1 as libc::c_int > 0 as libc::c_int {
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
    let ref mut fresh4 = *tokens.offset(n as isize);
    *fresh4 = 0 as *mut libc::c_char;
    return tokens;
}
pub unsafe extern "C" fn tokenize_char_dyn(
    mut buff: *mut libc::c_char,
    mut tchar: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut ret: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    ret = tokenize_char_dyn_wwt(buff, tchar, 1 as libc::c_int);
    return ret;
}
