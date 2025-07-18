use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type ini_handler = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const libc::c_char,
        *const libc::c_char,
    ) -> libc::c_int,
>;
pub type ini_reader = Option::<
    unsafe extern "C" fn(
        *mut libc::c_char,
        libc::c_int,
        *mut libc::c_void,
    ) -> *mut libc::c_char,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ini_parse_string_ctx {
    pub ptr: *const libc::c_char,
    pub num_left: size_t,
}
unsafe extern "C" fn rstrip(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = s.offset(strlen(s) as isize);
    while p > s
        && {
            p = p.offset(-1);
            *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        }
    {
        *p = '\0' as i32 as libc::c_char;
    }
    return s;
}
unsafe extern "C" fn lskip(mut s: *const libc::c_char) -> *mut libc::c_char {
    while *s as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        s = s.offset(1);
        s;
    }
    return s as *mut libc::c_char;
}
unsafe extern "C" fn find_chars_or_comment(
    mut s: *const libc::c_char,
    mut chars: *const libc::c_char,
) -> *mut libc::c_char {
    let mut was_space: libc::c_int = 0 as libc::c_int;
    while *s as libc::c_int != 0
        && (chars.is_null() || (strchr(chars, *s as libc::c_int)).is_null())
        && !(was_space != 0
            && !(strchr(b";\0" as *const u8 as *const libc::c_char, *s as libc::c_int))
                .is_null())
    {
        was_space = *(*__ctype_b_loc())
            .offset(*s as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int;
        s = s.offset(1);
        s;
    }
    return s as *mut libc::c_char;
}
unsafe extern "C" fn strncpy0(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && *src.offset(i as isize) as libc::c_int != 0
    {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    *dest.offset(i as isize) = '\0' as i32 as libc::c_char;
    return dest;
}
pub unsafe extern "C" fn snoopy_ini_parse_stream(
    mut reader: ini_reader,
    mut stream: *mut libc::c_void,
    mut handler: ini_handler,
    mut user: *mut libc::c_void,
) -> libc::c_int {
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut max_line: size_t = 1024 as libc::c_int as size_t;
    let mut section: [libc::c_char; 50] = *::std::mem::transmute::<
        &[u8; 50],
        &mut [libc::c_char; 50],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut prev_name: [libc::c_char; 50] = *::std::mem::transmute::<
        &[u8; 50],
        &mut [libc::c_char; 50],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lineno: libc::c_int = 0 as libc::c_int;
    let mut error: libc::c_int = 0 as libc::c_int;
    while !(reader.unwrap()(line.as_mut_ptr(), max_line as libc::c_int, stream))
        .is_null()
    {
        lineno += 1;
        lineno;
        start = line.as_mut_ptr();
        if lineno == 1 as libc::c_int
            && *start.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                == 0xef as libc::c_int
            && *start.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                == 0xbb as libc::c_int
            && *start.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                == 0xbf as libc::c_int
        {
            start = start.offset(3 as libc::c_int as isize);
        }
        start = lskip(rstrip(start));
        if (strchr(b";#\0" as *const u8 as *const libc::c_char, *start as libc::c_int))
            .is_null()
        {
            if *prev_name.as_mut_ptr() as libc::c_int != 0 && *start as libc::c_int != 0
                && start > line.as_mut_ptr()
            {
                if handler
                    .unwrap()(user, section.as_mut_ptr(), prev_name.as_mut_ptr(), start)
                    == 0 && error == 0
                {
                    error = lineno;
                }
            } else if *start as libc::c_int == '[' as i32 {
                end = find_chars_or_comment(
                    start.offset(1 as libc::c_int as isize),
                    b"]\0" as *const u8 as *const libc::c_char,
                );
                if *end as libc::c_int == ']' as i32 {
                    *end = '\0' as i32 as libc::c_char;
                    strncpy0(
                        section.as_mut_ptr(),
                        start.offset(1 as libc::c_int as isize),
                        ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
                    );
                    *prev_name.as_mut_ptr() = '\0' as i32 as libc::c_char;
                } else if error == 0 {
                    error = lineno;
                }
            } else if *start != 0 {
                end = find_chars_or_comment(
                    start,
                    b"=:\0" as *const u8 as *const libc::c_char,
                );
                if *end as libc::c_int == '=' as i32 || *end as libc::c_int == ':' as i32
                {
                    *end = '\0' as i32 as libc::c_char;
                    name = rstrip(start);
                    value = end.offset(1 as libc::c_int as isize);
                    end = find_chars_or_comment(value, 0 as *const libc::c_char);
                    if *end != 0 {
                        *end = '\0' as i32 as libc::c_char;
                    }
                    value = lskip(value);
                    rstrip(value);
                    if *value as libc::c_int == '"' as i32
                        && *value
                            .offset(
                                (strlen(value))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '"' as i32
                    {
                        *value
                            .offset(
                                (strlen(value))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) = '\0' as i32 as libc::c_char;
                        value = value.offset(1 as libc::c_int as isize);
                    } else if *value as libc::c_int == '\'' as i32
                        && *value
                            .offset(
                                (strlen(value))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '\'' as i32
                    {
                        *value
                            .offset(
                                (strlen(value))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) = '\0' as i32 as libc::c_char;
                        value = value.offset(1 as libc::c_int as isize);
                    }
                    strncpy0(
                        prev_name.as_mut_ptr(),
                        name,
                        ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
                    );
                    if handler.unwrap()(user, section.as_mut_ptr(), name, value) == 0
                        && error == 0
                    {
                        error = lineno;
                    }
                } else if error == 0 {
                    error = lineno;
                }
            }
        }
    }
    return error;
}
pub unsafe extern "C" fn snoopy_ini_parse_file(
    mut file: *mut FILE,
    mut handler: ini_handler,
    mut user: *mut libc::c_void,
) -> libc::c_int {
    return snoopy_ini_parse_stream(
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut FILE,
                ) -> *mut libc::c_char,
            >,
            ini_reader,
        >(
            Some(
                fgets
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut FILE,
                    ) -> *mut libc::c_char,
            ),
        ),
        file as *mut libc::c_void,
        handler,
        user,
    );
}
pub unsafe extern "C" fn snoopy_ini_parse(
    mut filename: *const libc::c_char,
    mut handler: ini_handler,
    mut user: *mut libc::c_void,
) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut error: libc::c_int = 0;
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    error = snoopy_ini_parse_file(file, handler, user);
    fclose(file);
    return error;
}
unsafe extern "C" fn ini_reader_string(
    mut str: *mut libc::c_char,
    mut num: libc::c_int,
    mut stream: *mut libc::c_void,
) -> *mut libc::c_char {
    let mut ctx: *mut ini_parse_string_ctx = stream as *mut ini_parse_string_ctx;
    let mut ctx_ptr: *const libc::c_char = (*ctx).ptr;
    let mut ctx_num_left: size_t = (*ctx).num_left;
    let mut strp: *mut libc::c_char = str;
    let mut c: libc::c_char = 0;
    if ctx_num_left == 0 as libc::c_int as libc::c_ulong || num < 2 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    while num > 1 as libc::c_int && ctx_num_left != 0 as libc::c_int as libc::c_ulong {
        let fresh0 = ctx_ptr;
        ctx_ptr = ctx_ptr.offset(1);
        c = *fresh0;
        ctx_num_left = ctx_num_left.wrapping_sub(1);
        ctx_num_left;
        let fresh1 = strp;
        strp = strp.offset(1);
        *fresh1 = c;
        if c as libc::c_int == '\n' as i32 {
            break;
        }
        num -= 1;
        num;
    }
    *strp = '\0' as i32 as libc::c_char;
    (*ctx).ptr = ctx_ptr;
    (*ctx).num_left = ctx_num_left;
    return str;
}
pub unsafe extern "C" fn snoopy_ini_parse_string(
    mut string: *const libc::c_char,
    mut handler: ini_handler,
    mut user: *mut libc::c_void,
) -> libc::c_int {
    let mut ctx: ini_parse_string_ctx = ini_parse_string_ctx {
        ptr: 0 as *const libc::c_char,
        num_left: 0,
    };
    ctx.ptr = string;
    ctx.num_left = strlen(string);
    return snoopy_ini_parse_stream(
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_char,
            >,
            ini_reader,
        >(
            Some(
                ini_reader_string
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> *mut libc::c_char,
            ),
        ),
        &mut ctx as *mut ini_parse_string_ctx as *mut libc::c_void,
        handler,
        user,
    );
}
