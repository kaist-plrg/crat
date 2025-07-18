use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _argp_fmtstream_update(__fs: argp_fmtstream_t);
    fn _argp_fmtstream_ensure(__fs: argp_fmtstream_t, __amount: size_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
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
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_point(mut __fs: argp_fmtstream_t) -> size_t {
    if ((*__fs).p).offset_from((*__fs).buf) as libc::c_long as size_t
        > (*__fs).point_offs
    {
        _argp_fmtstream_update(__fs);
    }
    return (if (*__fs).point_col >= 0 as libc::c_int as libc::c_long {
        (*__fs).point_col
    } else {
        0 as libc::c_int as libc::c_long
    }) as size_t;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_set_wmargin(
    mut __fs: argp_fmtstream_t,
    mut __wmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as libc::c_long as size_t
        > (*__fs).point_offs
    {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).wmargin as size_t;
    (*__fs).wmargin = __wmargin as ssize_t;
    return __old;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_set_rmargin(
    mut __fs: argp_fmtstream_t,
    mut __rmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as libc::c_long as size_t
        > (*__fs).point_offs
    {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).rmargin;
    (*__fs).rmargin = __rmargin;
    return __old;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_write(
    mut __fs: argp_fmtstream_t,
    mut __str: *const libc::c_char,
    mut __len: size_t,
) -> size_t {
    if ((*__fs).p).offset(__len as isize) <= (*__fs).end
        || _argp_fmtstream_ensure(__fs, __len) != 0
    {
        memcpy((*__fs).p as *mut libc::c_void, __str as *const libc::c_void, __len);
        (*__fs).p = ((*__fs).p).offset(__len as isize);
        return __len;
    } else {
        return 0 as libc::c_int as size_t
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_puts(
    mut __fs: argp_fmtstream_t,
    mut __str: *const libc::c_char,
) -> libc::c_int {
    let mut __len: size_t = strlen(__str);
    if __len != 0 {
        let mut __wrote: size_t = argp_fmtstream_write(__fs, __str, __len);
        return if __wrote == __len { 0 as libc::c_int } else { -(1 as libc::c_int) };
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_putc(
    mut __fs: argp_fmtstream_t,
    mut __ch: libc::c_int,
) -> libc::c_int {
    if (*__fs).p < (*__fs).end
        || _argp_fmtstream_ensure(__fs, 1 as libc::c_int as size_t) != 0
    {
        let fresh0 = (*__fs).p;
        (*__fs).p = ((*__fs).p).offset(1);
        *fresh0 = __ch as libc::c_char;
        return *fresh0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_set_lmargin(
    mut __fs: argp_fmtstream_t,
    mut __lmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as libc::c_long as size_t
        > (*__fs).point_offs
    {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).lmargin;
    (*__fs).lmargin = __lmargin;
    return __old;
}
