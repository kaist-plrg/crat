use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
pub unsafe extern "C" fn strkcpy(
    mut out: *mut libc::c_char,
    mut in_0: *mut libc::c_char,
    mut size: libc::c_int,
) -> *mut libc::c_char {
    strncpy(out, in_0, size as libc::c_ulong);
    *out.offset((size - 1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
    return out;
}
pub unsafe extern "C" fn strappend(
    mut str: *mut *mut libc::c_char,
    mut cat: *mut libc::c_char,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    if str.is_null() {
        return -(1 as libc::c_int)
    } else if (*str).is_null() {
        *str = strdup(cat);
        if (*str).is_null() && 1 as libc::c_int > 0 as libc::c_int {
            fprintf(
                stderr,
                b"str.c: %s.\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
        return 0 as libc::c_int;
    } else {
        l1 = strlen(*str) as libc::c_int;
        l2 = strlen(cat) as libc::c_int;
        *str = realloc(
            *str as *mut libc::c_void,
            (l1 + l2 + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        if (*str).is_null() && 1 as libc::c_int > 0 as libc::c_int {
            fprintf(
                stderr,
                b"str.c: %s.\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
        strcpy((*str).offset(l1 as isize), cat);
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn vstrappendf(
    mut str: *mut *mut libc::c_char,
    mut fmt: *mut libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if str.is_null() {
        return 0 as libc::c_int;
    }
    if (*str).is_null() {
        l = 0 as libc::c_int;
    } else {
        l = strlen(*str) as libc::c_int;
    }
    size = 128 as libc::c_int;
    *str = realloc(*str as *mut libc::c_void, (l + size) as libc::c_ulong)
        as *mut libc::c_char;
    if (*str).is_null() && l + size > 0 as libc::c_int {
        fprintf(
            stderr,
            b"str.c: %s.\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    if (*str).is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        n = vsnprintf(
            (*str).offset(l as isize),
            size as libc::c_ulong,
            fmt,
            ap.as_va_list(),
        );
        if n > -(1 as libc::c_int) && n < size {
            return 0 as libc::c_int
        } else if n > -(1 as libc::c_int) {
            size = n + 1 as libc::c_int;
        } else {
            size = size * 2 as libc::c_int;
        }
        *str = realloc(*str as *mut libc::c_void, (l + size) as libc::c_ulong)
            as *mut libc::c_char;
        if (*str).is_null() {
            return -(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn strappendf(
    mut str: *mut *mut libc::c_char,
    mut fmt: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    ret = vstrappendf(str, fmt, ap.as_va_list());
    return ret;
}
