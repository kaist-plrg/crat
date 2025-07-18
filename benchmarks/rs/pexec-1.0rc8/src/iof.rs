use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub unsafe extern "C" fn freadline(mut fr: *mut FILE) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buff: [libc::c_char; 256] = [0; 256];
    ret = 0 as *mut libc::c_char;
    if feof(fr) != 0 {
        return 0 as *mut libc::c_char;
    }
    while !(fgets(buff.as_mut_ptr(), 255 as libc::c_int, fr)).is_null() {
        if ret.is_null() {
            ret = malloc(
                (strlen(buff.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            *ret.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        } else {
            ret = realloc(
                ret as *mut libc::c_void,
                (strlen(ret))
                    .wrapping_add(strlen(buff.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
        }
        if ret.is_null() && 1 as libc::c_int > 0 as libc::c_int {
            fprintf(
                stderr,
                b"iof.c: %s.\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
        strcat(ret, buff.as_mut_ptr());
        if *ret
            .offset(
                (strlen(ret)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == 10 as libc::c_int
        {
            break;
        }
    }
    return ret;
}
pub unsafe extern "C" fn freadline_bs(mut fr: *mut FILE) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    ret = 0 as *mut libc::c_char;
    loop {
        wr = freadline(fr);
        if wr.is_null() {
            return ret
        } else {
            if ret.is_null() {
                ret = wr;
            } else {
                ret = realloc(
                    ret as *mut libc::c_void,
                    (strlen(ret))
                        .wrapping_add(strlen(wr))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if ret.is_null() && 1 as libc::c_int > 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"iof.c: %s.\n\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"memory exhausted\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    abort();
                }
                strcat(ret, wr);
                free(wr as *mut libc::c_void);
            }
            n = strlen(ret) as libc::c_int;
            if n >= 2 as libc::c_int {
                if *ret.offset((n - 1 as libc::c_int) as isize) as libc::c_int
                    == 0xa as libc::c_int
                    && *ret.offset((n - 2 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32
                {
                    *ret
                        .offset(
                            (n - 2 as libc::c_int) as isize,
                        ) = 0 as libc::c_int as libc::c_char;
                } else {
                    return ret
                }
            } else {
                return ret
            }
        }
    };
}
pub unsafe extern "C" fn fopenread(mut name: *mut libc::c_char) -> *mut FILE {
    let mut fr: *mut FILE = 0 as *mut FILE;
    if name.is_null() {
        fr = stdin;
    } else if strcmp(name, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        fr = stdin;
    } else {
        fr = fopen(name, b"rb\0" as *const u8 as *const libc::c_char);
    }
    return fr;
}
pub unsafe extern "C" fn fopenwrite(mut name: *mut libc::c_char) -> *mut FILE {
    let mut fw: *mut FILE = 0 as *mut FILE;
    if name.is_null() {
        fw = stdout;
    } else if strcmp(name, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        fw = stdout;
    } else {
        fw = fopen(name, b"wb\0" as *const u8 as *const libc::c_char);
    }
    return fw;
}
pub unsafe extern "C" fn fcloseread(mut fr: *mut FILE) -> libc::c_int {
    if fileno(fr) != fileno(stdin) {
        fclose(fr);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fclosewrite(mut fw: *mut FILE) -> libc::c_int {
    if fileno(fw) != fileno(stdout) {
        fclose(fw);
    }
    return 0 as libc::c_int;
}
