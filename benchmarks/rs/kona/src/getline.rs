use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn kerr(s: cS) -> K;
    fn strdupn(s: S, k: I) -> S;
    fn strlenn(s: S, k: I) -> I;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type I = libc::c_longlong;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k0 {
    pub _c: I,
    pub t: I,
    pub n: I,
    pub k: [*mut k0; 1],
}
pub type K = *mut k0;
pub unsafe extern "C" fn expander(mut s: *mut S, mut n: I) -> I {
    let mut t: S = 0 as *mut C;
    let mut q: I = 0;
    q = n;
    t = realloc(
        *s as *mut libc::c_void,
        (if 1 as libc::c_int as libc::c_longlong > q {
            1 as libc::c_int as libc::c_longlong
        } else {
            q
        }) as libc::c_ulong,
    ) as S;
    if t.is_null() {
        kerr(b"wsfull\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int) as I;
    }
    *s = t;
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn appender(
    mut s: *mut S,
    mut n: *mut I,
    mut t: S,
    mut k: I,
) -> I {
    if expander(s, *n + k + 1 as libc::c_int as libc::c_longlong) != 0 {
        return -(1 as libc::c_int) as I;
    }
    memcpy(
        (*s).offset(*n as isize) as *mut libc::c_void,
        t as *const libc::c_void,
        k as libc::c_ulong,
    );
    *n += k;
    *(*s).offset(*n as isize) = '\0' as i32 as C;
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn getline_(mut s: *mut S, mut n: *mut I, mut f: *mut FILE) -> I {
    return getdelim_(s, n, '\n' as i32 as I, f);
}
pub unsafe extern "C" fn getdelim_(
    mut s: *mut S,
    mut n: *mut I,
    mut d: I,
    mut f: *mut FILE,
) -> I {
    let mut m: I = 0;
    let mut z: S = 0 as *mut C;
    let mut o: size_t = *n as size_t;
    if getdelim(s, &mut o, d as libc::c_int, f) == -(1 as libc::c_int) as libc::c_long {
        *n = 0 as libc::c_int as I;
        return -(1 as libc::c_int) as I;
    }
    *n = o as I;
    m = strlenn(*s, *n);
    if (1 as libc::c_int as libc::c_longlong) < m
        && '\n' as i32
            == *(*s).offset((m - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
        && '\r' as i32
            == *(*s).offset((m - 2 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
    {
        m -= 1;
        *(*s).offset(m as isize) = '\0' as i32 as C;
        *(*s)
            .offset(
                (m - 1 as libc::c_int as libc::c_longlong) as isize,
            ) = '\n' as i32 as C;
    }
    z = strdupn(*s, m);
    free(*s as *mut libc::c_void);
    *s = z;
    *n = m;
    return *n;
}
