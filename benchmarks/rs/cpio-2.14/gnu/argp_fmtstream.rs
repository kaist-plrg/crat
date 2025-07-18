use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rpl_free(ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
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
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type va_list = __builtin_va_list;
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
pub struct argp_fmtstream {
    pub stream: *mut FILE,
    pub lmargin: size_t,
    pub rmargin: size_t,
    pub wmargin: ssize_t,
    pub point_offs: size_t,
    pub point_col: ssize_t,
    pub buf: *mut libc::c_char,
    pub p: *mut libc::c_char,
    pub end: *mut libc::c_char,
}
pub type argp_fmtstream_t = *mut argp_fmtstream;
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
pub unsafe extern "C" fn argp_make_fmtstream(
    mut stream: *mut FILE,
    mut lmargin: size_t,
    mut rmargin: size_t,
    mut wmargin: ssize_t,
) -> argp_fmtstream_t {
    let mut fs: argp_fmtstream_t = 0 as *mut argp_fmtstream;
    fs = malloc(::std::mem::size_of::<argp_fmtstream>() as libc::c_ulong)
        as *mut argp_fmtstream;
    if !fs.is_null() {
        (*fs).stream = stream;
        (*fs).lmargin = lmargin;
        (*fs).rmargin = rmargin;
        (*fs).wmargin = wmargin;
        (*fs).point_col = 0 as libc::c_int as ssize_t;
        (*fs).point_offs = 0 as libc::c_int as size_t;
        (*fs).buf = malloc(200 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        if ((*fs).buf).is_null() {
            rpl_free(fs as *mut libc::c_void);
            fs = 0 as argp_fmtstream_t;
        } else {
            (*fs).p = (*fs).buf;
            (*fs).end = ((*fs).buf).offset(200 as libc::c_int as isize);
        }
    }
    return fs;
}
pub unsafe extern "C" fn argp_fmtstream_free(mut fs: argp_fmtstream_t) {
    _argp_fmtstream_update(fs);
    if (*fs).p > (*fs).buf {
        if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t)
                .wrapping_mul(((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = (*fs).buf as *const libc::c_char;
                let mut __stream: *mut FILE = (*fs).stream;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t)
                    .wrapping_mul(
                        ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t,
                    );
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh1 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh1 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                0u8
            });
        } else {
            if 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t
                        == 0 as libc::c_int as libc::c_ulong
            {} else {
                fwrite_unlocked(
                    (*fs).buf as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t,
                    (*fs).stream,
                );
            };
        };
        0u8;
    }
    rpl_free((*fs).buf as *mut libc::c_void);
    rpl_free(fs as *mut libc::c_void);
}
pub unsafe extern "C" fn _argp_fmtstream_update(mut fs: argp_fmtstream_t) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    buf = ((*fs).buf).offset((*fs).point_offs as isize);
    while buf < (*fs).p {
        let mut r: size_t = 0;
        if (*fs).point_col == 0 as libc::c_int as libc::c_long
            && (*fs).lmargin != 0 as libc::c_int as libc::c_ulong
        {
            let pad: size_t = (*fs).lmargin;
            if ((*fs).p).offset(pad as isize) < (*fs).end {
                memmove(
                    buf.offset(pad as isize) as *mut libc::c_void,
                    buf as *const libc::c_void,
                    ((*fs).p).offset_from(buf) as libc::c_long as libc::c_ulong,
                );
                (*fs).p = ((*fs).p).offset(pad as isize);
                memset(buf as *mut libc::c_void, ' ' as i32, pad);
                buf = buf.offset(pad as isize);
            } else {
                let mut i: size_t = 0;
                i = 0 as libc::c_int as size_t;
                while i < pad {
                    putc_unlocked(' ' as i32, (*fs).stream);
                    i = i.wrapping_add(1);
                    i;
                }
            }
            (*fs).point_col = pad as ssize_t;
        }
        len = ((*fs).p).offset_from(buf) as libc::c_long as size_t;
        nl = memchr(buf as *const libc::c_void, '\n' as i32, len) as *mut libc::c_char;
        if (*fs).point_col < 0 as libc::c_int as libc::c_long {
            (*fs).point_col = 0 as libc::c_int as ssize_t;
        }
        if nl.is_null() {
            if ((*fs).point_col as libc::c_ulong).wrapping_add(len) < (*fs).rmargin {
                (*fs)
                    .point_col = ((*fs).point_col as libc::c_ulong).wrapping_add(len)
                    as ssize_t as ssize_t;
                break;
            } else {
                nl = (*fs).p;
            }
        } else if ((*fs).point_col + nl.offset_from(buf) as libc::c_long)
            < (*fs).rmargin as ssize_t
        {
            (*fs).point_col = 0 as libc::c_int as ssize_t;
            buf = nl.offset(1 as libc::c_int as isize);
            continue;
        }
        r = ((*fs).rmargin).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if (*fs).wmargin < 0 as libc::c_int as libc::c_long {
            if nl < (*fs).p {
                memmove(
                    buf.offset(r.wrapping_sub((*fs).point_col as libc::c_ulong) as isize)
                        as *mut libc::c_void,
                    nl as *const libc::c_void,
                    ((*fs).p).offset_from(nl) as libc::c_long as libc::c_ulong,
                );
                (*fs)
                    .p = ((*fs).p)
                    .offset(
                        -(buf
                            .offset(
                                r.wrapping_sub((*fs).point_col as libc::c_ulong) as isize,
                            )
                            .offset_from(nl) as libc::c_long as isize),
                    );
                (*fs).point_col = 0 as libc::c_int as ssize_t;
                buf = buf
                    .offset(r.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            } else {
                (*fs)
                    .point_col = ((*fs).point_col as libc::c_ulong).wrapping_add(len)
                    as ssize_t as ssize_t;
                (*fs)
                    .p = ((*fs).p)
                    .offset(
                        -(((*fs).point_col as libc::c_ulong).wrapping_sub(r) as isize),
                    );
                break;
            }
        } else {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut nextline: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut i_0: libc::c_int = 0;
            p = buf
                .offset(
                    r
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub((*fs).point_col as libc::c_ulong) as isize,
                );
            while p >= buf
                && *(*__ctype_b_loc())
                    .offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                p = p.offset(-1);
                p;
            }
            nextline = p.offset(1 as libc::c_int as isize);
            if nextline > buf {
                if p >= buf {
                    loop {
                        p = p.offset(-1);
                        p;
                        if !(p >= buf
                            && *(*__ctype_b_loc())
                                .offset(*p as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                        {
                            break;
                        }
                    }
                }
                nl = p.offset(1 as libc::c_int as isize);
            } else {
                p = buf
                    .offset(
                        r
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_sub((*fs).point_col as libc::c_ulong) as isize,
                    );
                if p < nl {
                    loop {
                        p = p.offset(1);
                        p;
                        if !(p < nl
                            && *(*__ctype_b_loc())
                                .offset(*p as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                                == 0)
                        {
                            break;
                        }
                    }
                }
                if p == nl {
                    (*fs).point_col = 0 as libc::c_int as ssize_t;
                    buf = nl.offset(1 as libc::c_int as isize);
                    continue;
                } else {
                    nl = p;
                    loop {
                        p = p.offset(1);
                        p;
                        if !(*(*__ctype_b_loc())
                            .offset(*p as libc::c_uchar as libc::c_int as isize)
                            as libc::c_int
                            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
                        {
                            break;
                        }
                    }
                    nextline = p;
                }
            }
            if (if nextline == buf.offset(len as isize).offset(1 as libc::c_int as isize)
            {
                ((((*fs).end).offset_from(nl) as libc::c_long)
                    < (*fs).wmargin + 1 as libc::c_int as libc::c_long) as libc::c_int
            } else {
                ((nextline.offset_from(nl.offset(1 as libc::c_int as isize))
                    as libc::c_long) < (*fs).wmargin) as libc::c_int
            }) != 0 && (*fs).p > nextline
            {
                if ((*fs).end).offset_from((*fs).p) as libc::c_long
                    > (*fs).wmargin + 1 as libc::c_int as libc::c_long
                {
                    let mut mv: size_t = ((*fs).p).offset_from(nextline) as libc::c_long
                        as size_t;
                    memmove(
                        nl
                            .offset(1 as libc::c_int as isize)
                            .offset((*fs).wmargin as isize) as *mut libc::c_void,
                        nextline as *const libc::c_void,
                        mv,
                    );
                    nextline = nl
                        .offset(1 as libc::c_int as isize)
                        .offset((*fs).wmargin as isize);
                    len = nextline.offset(mv as isize).offset_from(buf) as libc::c_long
                        as size_t;
                    let fresh2 = nl;
                    nl = nl.offset(1);
                    *fresh2 = '\n' as i32 as libc::c_char;
                } else {
                    if nl > (*fs).buf {
                        if 0 != 0 && 0 != 0
                            && (1 as libc::c_int as size_t)
                                .wrapping_mul(
                                    nl.offset_from((*fs).buf) as libc::c_long as size_t,
                                ) <= 8 as libc::c_int as libc::c_ulong
                            && 1 as libc::c_int as size_t
                                != 0 as libc::c_int as libc::c_ulong
                        {
                            ({
                                let mut __ptr: *const libc::c_char = (*fs).buf
                                    as *const libc::c_char;
                                let mut __stream: *mut FILE = (*fs).stream;
                                let mut __cnt: size_t = 0;
                                __cnt = (1 as libc::c_int as size_t)
                                    .wrapping_mul(
                                        nl.offset_from((*fs).buf) as libc::c_long as size_t,
                                    );
                                while __cnt > 0 as libc::c_int as libc::c_ulong {
                                    let fresh3 = __ptr;
                                    __ptr = __ptr.offset(1);
                                    if putc_unlocked(*fresh3 as libc::c_int, __stream)
                                        == -(1 as libc::c_int)
                                    {
                                        break;
                                    }
                                    __cnt = __cnt.wrapping_sub(1);
                                    __cnt;
                                }
                                0u8
                            });
                        } else {
                            if 0 != 0
                                && 1 as libc::c_int as size_t
                                    == 0 as libc::c_int as libc::c_ulong
                                || 0 != 0
                                    && nl.offset_from((*fs).buf) as libc::c_long as size_t
                                        == 0 as libc::c_int as libc::c_ulong
                            {} else {
                                fwrite_unlocked(
                                    (*fs).buf as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                    nl.offset_from((*fs).buf) as libc::c_long as size_t,
                                    (*fs).stream,
                                );
                            };
                        };
                        0u8;
                    }
                    putc_unlocked('\n' as i32, (*fs).stream);
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            buf.offset_from((*fs).buf) as libc::c_long as libc::c_ulong,
                        ) as size_t as size_t;
                    buf = (*fs).buf;
                    nl = buf;
                }
            } else {
                let fresh4 = nl;
                nl = nl.offset(1);
                *fresh4 = '\n' as i32 as libc::c_char;
            }
            if nextline.offset_from(nl) as libc::c_long >= (*fs).wmargin
                || nextline == buf.offset(len as isize).offset(1 as libc::c_int as isize)
                    && ((*fs).end).offset_from(nextline) as libc::c_long >= (*fs).wmargin
            {
                i_0 = 0 as libc::c_int;
                while (i_0 as libc::c_long) < (*fs).wmargin {
                    let fresh5 = nl;
                    nl = nl.offset(1);
                    *fresh5 = ' ' as i32 as libc::c_char;
                    i_0 += 1;
                    i_0;
                }
            } else {
                i_0 = 0 as libc::c_int;
                while (i_0 as libc::c_long) < (*fs).wmargin {
                    putc_unlocked(' ' as i32, (*fs).stream);
                    i_0 += 1;
                    i_0;
                }
            }
            if nl < nextline {
                memmove(
                    nl as *mut libc::c_void,
                    nextline as *const libc::c_void,
                    buf.offset(len as isize).offset_from(nextline) as libc::c_long
                        as libc::c_ulong,
                );
            }
            len = (len as libc::c_ulong)
                .wrapping_sub(nextline.offset_from(buf) as libc::c_long as libc::c_ulong)
                as size_t as size_t;
            buf = nl;
            (*fs).p = nl.offset(len as isize);
            (*fs)
                .point_col = if (*fs).wmargin != 0 {
                (*fs).wmargin
            } else {
                -(1 as libc::c_int) as libc::c_long
            };
        }
    }
    (*fs).point_offs = ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t;
}
pub unsafe extern "C" fn _argp_fmtstream_ensure(
    mut fs: *mut argp_fmtstream,
    mut amount: size_t,
) -> libc::c_int {
    if (((*fs).end).offset_from((*fs).p) as libc::c_long as size_t) < amount {
        let mut wrote: ssize_t = 0;
        _argp_fmtstream_update(fs);
        wrote = (if 0 != 0 && 0 != 0
            && (1 as libc::c_int as size_t)
                .wrapping_mul(((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            {
                let mut __ptr: *const libc::c_char = (*fs).buf as *const libc::c_char;
                let mut __stream: *mut FILE = (*fs).stream;
                let mut __cnt: size_t = 0;
                __cnt = (1 as libc::c_int as size_t)
                    .wrapping_mul(
                        ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t,
                    );
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh6 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh6 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                (1 as libc::c_int as size_t)
                    .wrapping_mul(
                        ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t,
                    )
                    .wrapping_sub(__cnt)
                    .wrapping_div(1 as libc::c_int as size_t)
            }
        } else if 0 != 0
            && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            || 0 != 0
                && ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t
                    == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int as size_t
        } else {
            fwrite_unlocked(
                (*fs).buf as *const libc::c_void,
                1 as libc::c_int as size_t,
                ((*fs).p).offset_from((*fs).buf) as libc::c_long as size_t,
                (*fs).stream,
            )
        }) as ssize_t;
        if wrote == ((*fs).p).offset_from((*fs).buf) as libc::c_long {
            (*fs).p = (*fs).buf;
            (*fs).point_offs = 0 as libc::c_int as size_t;
        } else {
            (*fs).p = ((*fs).p).offset(-(wrote as isize));
            (*fs)
                .point_offs = ((*fs).point_offs as libc::c_ulong)
                .wrapping_sub(wrote as libc::c_ulong) as size_t as size_t;
            memmove(
                (*fs).buf as *mut libc::c_void,
                ((*fs).buf).offset(wrote as isize) as *const libc::c_void,
                ((*fs).p).offset_from((*fs).buf) as libc::c_long as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
        if (((*fs).end).offset_from((*fs).buf) as libc::c_long as size_t) < amount {
            let mut old_size: size_t = ((*fs).end).offset_from((*fs).buf) as libc::c_long
                as size_t;
            let mut new_size: size_t = old_size.wrapping_add(amount);
            let mut new_buf: *mut libc::c_char = 0 as *mut libc::c_char;
            if new_size < old_size
                || {
                    new_buf = realloc((*fs).buf as *mut libc::c_void, new_size)
                        as *mut libc::c_char;
                    new_buf.is_null()
                }
            {
                *__errno_location() = 12 as libc::c_int;
                return 0 as libc::c_int;
            }
            (*fs).buf = new_buf;
            (*fs).end = new_buf.offset(new_size as isize);
            (*fs).p = (*fs).buf;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn argp_fmtstream_printf(
    mut fs: *mut argp_fmtstream,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ssize_t {
    let mut out: libc::c_int = 0;
    let mut avail: size_t = 0;
    let mut size_guess: size_t = 150 as libc::c_int as size_t;
    loop {
        let mut args_0: ::std::ffi::VaListImpl;
        if _argp_fmtstream_ensure(fs, size_guess) == 0 {
            return -(1 as libc::c_int) as ssize_t;
        }
        args_0 = args.clone();
        avail = ((*fs).end).offset_from((*fs).p) as libc::c_long as size_t;
        out = vsnprintf((*fs).p, avail, fmt, args_0.as_va_list());
        if out as size_t >= avail {
            size_guess = (out + 1 as libc::c_int) as size_t;
        }
        if !(out as size_t >= avail) {
            break;
        }
    }
    (*fs).p = ((*fs).p).offset(out as isize);
    return out as ssize_t;
}
