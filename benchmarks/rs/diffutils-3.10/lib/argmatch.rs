use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut exit_failure: libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
unsafe extern "C" fn __argmatch_die() {
    exit(exit_failure);
}
pub static mut argmatch_die: argmatch_exit_fn = unsafe {
    Some(__argmatch_die as unsafe extern "C" fn() -> ())
};
pub unsafe extern "C" fn argmatch(
    mut arg: *const libc::c_char,
    mut arglist: *const *const libc::c_char,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) -> ptrdiff_t {
    let mut i: size_t = 0;
    let mut arglen: size_t = 0;
    let mut matchind: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
    let mut ambiguous: bool = 0 as libc::c_int != 0;
    arglen = strlen(arg);
    i = 0 as libc::c_int as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if strncmp(*arglist.offset(i as isize), arg, arglen) == 0 {
            if strlen(*arglist.offset(i as isize)) == arglen {
                return i as ptrdiff_t
            } else if matchind == -(1 as libc::c_int) as libc::c_long {
                matchind = i as ptrdiff_t;
            } else if vallist.is_null()
                || memcmp(
                    (vallist as *const libc::c_char)
                        .offset(valsize.wrapping_mul(matchind as libc::c_ulong) as isize)
                        as *const libc::c_void,
                    (vallist as *const libc::c_char)
                        .offset(valsize.wrapping_mul(i) as isize) as *const libc::c_void,
                    valsize,
                ) != 0
            {
                ambiguous = 1 as libc::c_int != 0;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if ambiguous { return -(2 as libc::c_int) as ptrdiff_t } else { return matchind };
}
pub unsafe extern "C" fn argmatch_exact(
    mut arg: *const libc::c_char,
    mut arglist: *const *const libc::c_char,
) -> ptrdiff_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if strcmp(*arglist.offset(i as isize), arg) == 0 {
            return i as ptrdiff_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return -(1 as libc::c_int) as ptrdiff_t;
}
pub unsafe extern "C" fn argmatch_invalid(
    mut context: *const libc::c_char,
    mut value: *const libc::c_char,
    mut problem: ptrdiff_t,
) {
    let mut format: *const libc::c_char = if problem
        == -(1 as libc::c_int) as libc::c_long
    {
        dcgettext(
            0 as *const libc::c_char,
            b"invalid argument %s for %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        )
    } else {
        dcgettext(
            0 as *const libc::c_char,
            b"ambiguous argument %s for %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        )
    };
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        format,
        quotearg_n_style(0 as libc::c_int, locale_quoting_style, value),
        quote_n(1 as libc::c_int, context),
    );
}
pub unsafe extern "C" fn argmatch_valid(
    mut arglist: *const *const libc::c_char,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) {
    let mut i: size_t = 0;
    let mut last_val: *const libc::c_char = 0 as *const libc::c_char;
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"Valid arguments are:\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stderr,
    );
    i = 0 as libc::c_int as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if i == 0 as libc::c_int as libc::c_ulong
            || memcmp(
                last_val as *const libc::c_void,
                (vallist as *const libc::c_char).offset(valsize.wrapping_mul(i) as isize)
                    as *const libc::c_void,
                valsize,
            ) != 0
        {
            fprintf(
                stderr,
                b"\n  - %s\0" as *const u8 as *const libc::c_char,
                quote(*arglist.offset(i as isize)),
            );
            last_val = (vallist as *const libc::c_char)
                .offset(valsize.wrapping_mul(i) as isize);
        } else {
            fprintf(
                stderr,
                b", %s\0" as *const u8 as *const libc::c_char,
                quote(*arglist.offset(i as isize)),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    putc_unlocked('\n' as i32, stderr);
}
pub unsafe extern "C" fn __xargmatch_internal(
    mut context: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut arglist: *const *const libc::c_char,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
    mut exit_fn: argmatch_exit_fn,
    mut allow_abbreviation: bool,
) -> ptrdiff_t {
    let mut res: ptrdiff_t = 0;
    if allow_abbreviation {
        res = argmatch(arg, arglist, vallist, valsize);
    } else {
        res = argmatch_exact(arg, arglist);
    }
    if res >= 0 as libc::c_int as libc::c_long {
        return res;
    }
    argmatch_invalid(context, arg, res);
    argmatch_valid(arglist, vallist, valsize);
    (Some(exit_fn.unwrap())).unwrap()();
    return -(1 as libc::c_int) as ptrdiff_t;
}
pub unsafe extern "C" fn argmatch_to_argument(
    mut value: *const libc::c_void,
    mut arglist: *const *const libc::c_char,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) -> *const libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if memcmp(
            value,
            (vallist as *const libc::c_char).offset(valsize.wrapping_mul(i) as isize)
                as *const libc::c_void,
            valsize,
        ) == 0
        {
            return *arglist.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const libc::c_char;
}
