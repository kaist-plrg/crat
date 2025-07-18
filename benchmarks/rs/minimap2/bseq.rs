use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn gzdopen(fd: libc::c_int, mode: *const libc::c_char) -> gzFile;
    fn gzread(file: gzFile, buf: voidp, len: libc::c_uint) -> libc::c_int;
    fn gzclose(file: gzFile) -> libc::c_int;
    fn gzopen(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn krealloc(
        km: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type voidp = *mut libc::c_void;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
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
pub struct mm_bseq_file_s {
    pub fp: gzFile,
    pub ks: *mut kseq_t,
    pub s: mm_bseq1_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_bseq1_t {
    pub l_seq: libc::c_int,
    pub rid: libc::c_int,
    pub name: *mut libc::c_char,
    pub seq: *mut libc::c_char,
    pub qual: *mut libc::c_char,
    pub comment: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kseq_t {
    pub name: kstring_t,
    pub comment: kstring_t,
    pub seq: kstring_t,
    pub qual: kstring_t,
    pub last_char: libc::c_int,
    pub f: *mut kstream_t,
}
pub type kstream_t = __kstream_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __kstream_t {
    pub begin: libc::c_int,
    pub end: libc::c_int,
    #[bitfield(name = "is_eof", ty = "libc::c_int", bits = "0..=1")]
    #[bitfield(name = "bufsize", ty = "libc::c_int", bits = "2..=31")]
    pub is_eof_bufsize: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub f: gzFile,
    pub buf: *mut libc::c_uchar,
}
pub type kstring_t = __kstring_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __kstring_t {
    pub l: size_t,
    pub m: size_t,
    pub s: *mut libc::c_char,
}
pub type mm_bseq_file_t = mm_bseq_file_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm_bseq1_t,
}
pub const _ISspace: C2RustUnnamed_1 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm_bseq1_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
#[inline]
unsafe extern "C" fn mm_qname_len(mut s: *const libc::c_char) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = strlen(s) as libc::c_int;
    return if l >= 3 as libc::c_int
        && *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int >= '0' as i32
        && *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int <= '9' as i32
        && *s.offset((l - 2 as libc::c_int) as isize) as libc::c_int == '/' as i32
    {
        l - 2 as libc::c_int
    } else {
        l
    };
}
#[inline]
unsafe extern "C" fn mm_qname_same(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = mm_qname_len(s1);
    l2 = mm_qname_len(s2);
    return (l1 == l2 && strncmp(s1, s2, l1 as libc::c_ulong) == 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn ks_getuntil2(
    mut ks: *mut kstream_t,
    mut delimiter: libc::c_int,
    mut str: *mut kstring_t,
    mut dret: *mut libc::c_int,
    mut append: libc::c_int,
) -> libc::c_int {
    if !dret.is_null() {
        *dret = 0 as libc::c_int;
    }
    (*str).l = if append != 0 { (*str).l } else { 0 as libc::c_int as libc::c_ulong };
    if (*ks).begin >= (*ks).end && (*ks).is_eof() != 0 {
        return -(1 as libc::c_int);
    }
    loop {
        let mut i: libc::c_int = 0;
        if (*ks).begin >= (*ks).end {
            if !((*ks).is_eof() == 0) {
                break;
            }
            (*ks).begin = 0 as libc::c_int;
            (*ks)
                .end = gzread(
                (*ks).f,
                (*ks).buf as voidp,
                (*ks).bufsize() as libc::c_uint,
            );
            if (*ks).end < (*ks).bufsize() {
                (*ks).set_is_eof(1 as libc::c_int);
            }
            if (*ks).end == 0 as libc::c_int {
                break;
            }
        }
        if delimiter == 2 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                if *((*ks).buf).offset(i as isize) as libc::c_int == '\n' as i32 {
                    break;
                }
                i += 1;
                i;
            }
        } else if delimiter > 2 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                if *((*ks).buf).offset(i as isize) as libc::c_int == delimiter {
                    break;
                }
                i += 1;
                i;
            }
        } else if delimiter == 0 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                if *(*__ctype_b_loc())
                    .offset(*((*ks).buf).offset(i as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    break;
                }
                i += 1;
                i;
            }
        } else if delimiter == 1 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                if *(*__ctype_b_loc())
                    .offset(*((*ks).buf).offset(i as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && *((*ks).buf).offset(i as isize) as libc::c_int != ' ' as i32
                {
                    break;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
        }
        if ((*str).m).wrapping_sub((*str).l)
            < (i - (*ks).begin + 1 as libc::c_int) as size_t
        {
            (*str)
                .m = ((*str).l)
                .wrapping_add((i - (*ks).begin) as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            (*str).m = ((*str).m).wrapping_sub(1);
            (*str).m;
            (*str).m |= (*str).m >> 1 as libc::c_int;
            (*str).m |= (*str).m >> 2 as libc::c_int;
            (*str).m |= (*str).m >> 4 as libc::c_int;
            (*str).m |= (*str).m >> 8 as libc::c_int;
            (*str).m |= (*str).m >> 16 as libc::c_int;
            (*str).m = ((*str).m).wrapping_add(1);
            (*str).m;
            (*str)
                .s = realloc((*str).s as *mut libc::c_void, (*str).m)
                as *mut libc::c_char;
        }
        memcpy(
            ((*str).s).offset((*str).l as isize) as *mut libc::c_void,
            ((*ks).buf).offset((*ks).begin as isize) as *const libc::c_void,
            (i - (*ks).begin) as libc::c_ulong,
        );
        (*str).l = ((*str).l).wrapping_add((i - (*ks).begin) as libc::c_ulong);
        (*ks).begin = i + 1 as libc::c_int;
        if !(i < (*ks).end) {
            continue;
        }
        if !dret.is_null() {
            *dret = *((*ks).buf).offset(i as isize) as libc::c_int;
        }
        break;
    }
    if ((*str).s).is_null() {
        (*str).m = 1 as libc::c_int as size_t;
        (*str)
            .s = calloc(
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
    } else if delimiter == 2 as libc::c_int
        && (*str).l > 1 as libc::c_int as libc::c_ulong
        && *((*str).s)
            .offset(((*str).l).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\r' as i32
    {
        (*str).l = ((*str).l).wrapping_sub(1);
        (*str).l;
    }
    *((*str).s).offset((*str).l as isize) = '\0' as i32 as libc::c_char;
    return (*str).l as libc::c_int;
}
#[inline]
unsafe extern "C" fn ks_getc(mut ks: *mut kstream_t) -> libc::c_int {
    if (*ks).is_eof() != 0 && (*ks).begin >= (*ks).end {
        return -(1 as libc::c_int);
    }
    if (*ks).begin >= (*ks).end {
        (*ks).begin = 0 as libc::c_int;
        (*ks).end = gzread((*ks).f, (*ks).buf as voidp, (*ks).bufsize() as libc::c_uint);
        if (*ks).end < (*ks).bufsize() {
            (*ks).set_is_eof(1 as libc::c_int);
        }
        if (*ks).end == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    let fresh0 = (*ks).begin;
    (*ks).begin = (*ks).begin + 1;
    return *((*ks).buf).offset(fresh0 as isize) as libc::c_int;
}
pub unsafe extern "C" fn ks_init(mut f: gzFile) -> *mut kstream_t {
    let mut ks: *mut kstream_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<kstream_t>() as libc::c_ulong,
    ) as *mut kstream_t;
    (*ks).f = f;
    (*ks).set_bufsize(16384 as libc::c_int);
    (*ks).buf = malloc(16384 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    return ks;
}
pub unsafe extern "C" fn kseq_read(mut seq: *mut kseq_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut ks: *mut kstream_t = (*seq).f;
    if (*seq).last_char == 0 as libc::c_int {
        loop {
            c = ks_getc(ks);
            if !(c != -(1 as libc::c_int) && c != '>' as i32 && c != '@' as i32) {
                break;
            }
        }
        if c == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        (*seq).last_char = c;
    }
    (*seq).qual.l = 0 as libc::c_int as size_t;
    (*seq).seq.l = (*seq).qual.l;
    (*seq).comment.l = (*seq).seq.l;
    if ks_getuntil(ks, 0 as libc::c_int, &mut (*seq).name, &mut c) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if c != '\n' as i32 {
        ks_getuntil(ks, 2 as libc::c_int, &mut (*seq).comment, 0 as *mut libc::c_int);
    }
    if ((*seq).seq.s).is_null() {
        (*seq).seq.m = 256 as libc::c_int as size_t;
        (*seq).seq.s = malloc((*seq).seq.m) as *mut libc::c_char;
    }
    loop {
        c = ks_getc(ks);
        if !(c != -(1 as libc::c_int) && c != '>' as i32 && c != '+' as i32
            && c != '@' as i32)
        {
            break;
        }
        if c == '\n' as i32 {
            continue;
        }
        let fresh1 = (*seq).seq.l;
        (*seq).seq.l = ((*seq).seq.l).wrapping_add(1);
        *((*seq).seq.s).offset(fresh1 as isize) = c as libc::c_char;
        ks_getuntil2(
            ks,
            2 as libc::c_int,
            &mut (*seq).seq,
            0 as *mut libc::c_int,
            1 as libc::c_int,
        );
    }
    if c == '>' as i32 || c == '@' as i32 {
        (*seq).last_char = c;
    }
    if ((*seq).seq.l).wrapping_add(1 as libc::c_int as libc::c_ulong) >= (*seq).seq.m {
        (*seq).seq.m = ((*seq).seq.l).wrapping_add(2 as libc::c_int as libc::c_ulong);
        (*seq).seq.m = ((*seq).seq.m).wrapping_sub(1);
        (*seq).seq.m;
        (*seq).seq.m |= (*seq).seq.m >> 1 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 2 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 4 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 8 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 16 as libc::c_int;
        (*seq).seq.m = ((*seq).seq.m).wrapping_add(1);
        (*seq).seq.m;
        (*seq)
            .seq
            .s = realloc((*seq).seq.s as *mut libc::c_void, (*seq).seq.m)
            as *mut libc::c_char;
    }
    *((*seq).seq.s).offset((*seq).seq.l as isize) = 0 as libc::c_int as libc::c_char;
    if c != '+' as i32 {
        return (*seq).seq.l as libc::c_int;
    }
    if (*seq).qual.m < (*seq).seq.m {
        (*seq).qual.m = (*seq).seq.m;
        (*seq)
            .qual
            .s = realloc((*seq).qual.s as *mut libc::c_void, (*seq).qual.m)
            as *mut libc::c_char;
    }
    loop {
        c = ks_getc(ks);
        if !(c != -(1 as libc::c_int) && c != '\n' as i32) {
            break;
        }
    }
    if c == -(1 as libc::c_int) {
        return -(2 as libc::c_int);
    }
    while ks_getuntil2(
        ks,
        2 as libc::c_int,
        &mut (*seq).qual,
        0 as *mut libc::c_int,
        1 as libc::c_int,
    ) >= 0 as libc::c_int && (*seq).qual.l < (*seq).seq.l
    {}
    (*seq).last_char = 0 as libc::c_int;
    if (*seq).seq.l != (*seq).qual.l {
        return -(2 as libc::c_int);
    }
    return (*seq).seq.l as libc::c_int;
}
#[inline]
unsafe extern "C" fn ks_getuntil(
    mut ks: *mut kstream_t,
    mut delimiter: libc::c_int,
    mut str: *mut kstring_t,
    mut dret: *mut libc::c_int,
) -> libc::c_int {
    return ks_getuntil2(ks, delimiter, str, dret, 0 as libc::c_int);
}
pub unsafe extern "C" fn kseq_destroy(mut ks: *mut kseq_t) {
    if ks.is_null() {
        return;
    }
    free((*ks).name.s as *mut libc::c_void);
    free((*ks).comment.s as *mut libc::c_void);
    free((*ks).seq.s as *mut libc::c_void);
    free((*ks).qual.s as *mut libc::c_void);
    ks_destroy((*ks).f);
    free(ks as *mut libc::c_void);
}
pub unsafe extern "C" fn ks_destroy(mut ks: *mut kstream_t) {
    if ks.is_null() {
        return;
    }
    free((*ks).buf as *mut libc::c_void);
    free(ks as *mut libc::c_void);
}
pub unsafe extern "C" fn kseq_init(mut fd: gzFile) -> *mut kseq_t {
    let mut s: *mut kseq_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<kseq_t>() as libc::c_ulong,
    ) as *mut kseq_t;
    (*s).f = ks_init(fd);
    return s;
}
pub static mut seq_comp_table: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    'T' as i32 as libc::c_uchar,
    'V' as i32 as libc::c_uchar,
    'G' as i32 as libc::c_uchar,
    'H' as i32 as libc::c_uchar,
    'E' as i32 as libc::c_uchar,
    'F' as i32 as libc::c_uchar,
    'C' as i32 as libc::c_uchar,
    'D' as i32 as libc::c_uchar,
    'I' as i32 as libc::c_uchar,
    'J' as i32 as libc::c_uchar,
    'M' as i32 as libc::c_uchar,
    'L' as i32 as libc::c_uchar,
    'K' as i32 as libc::c_uchar,
    'N' as i32 as libc::c_uchar,
    'O' as i32 as libc::c_uchar,
    'P' as i32 as libc::c_uchar,
    'Q' as i32 as libc::c_uchar,
    'Y' as i32 as libc::c_uchar,
    'S' as i32 as libc::c_uchar,
    'A' as i32 as libc::c_uchar,
    'A' as i32 as libc::c_uchar,
    'B' as i32 as libc::c_uchar,
    'W' as i32 as libc::c_uchar,
    'X' as i32 as libc::c_uchar,
    'R' as i32 as libc::c_uchar,
    'Z' as i32 as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    92 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    94 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    't' as i32 as libc::c_uchar,
    'v' as i32 as libc::c_uchar,
    'g' as i32 as libc::c_uchar,
    'h' as i32 as libc::c_uchar,
    'e' as i32 as libc::c_uchar,
    'f' as i32 as libc::c_uchar,
    'c' as i32 as libc::c_uchar,
    'd' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'j' as i32 as libc::c_uchar,
    'm' as i32 as libc::c_uchar,
    'l' as i32 as libc::c_uchar,
    'k' as i32 as libc::c_uchar,
    'n' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    'p' as i32 as libc::c_uchar,
    'q' as i32 as libc::c_uchar,
    'y' as i32 as libc::c_uchar,
    's' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'b' as i32 as libc::c_uchar,
    'w' as i32 as libc::c_uchar,
    'x' as i32 as libc::c_uchar,
    'r' as i32 as libc::c_uchar,
    'z' as i32 as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    135 as libc::c_int as libc::c_uchar,
    136 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    139 as libc::c_int as libc::c_uchar,
    140 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    146 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    149 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    151 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    156 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    164 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    169 as libc::c_int as libc::c_uchar,
    170 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    172 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    175 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    178 as libc::c_int as libc::c_uchar,
    179 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    184 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    186 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    190 as libc::c_int as libc::c_uchar,
    191 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    194 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    196 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    202 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    204 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    207 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    210 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    213 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    215 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    217 as libc::c_int as libc::c_uchar,
    218 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    220 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    222 as libc::c_int as libc::c_uchar,
    223 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    226 as libc::c_int as libc::c_uchar,
    227 as libc::c_int as libc::c_uchar,
    228 as libc::c_int as libc::c_uchar,
    229 as libc::c_int as libc::c_uchar,
    230 as libc::c_int as libc::c_uchar,
    231 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    233 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    237 as libc::c_int as libc::c_uchar,
    238 as libc::c_int as libc::c_uchar,
    239 as libc::c_int as libc::c_uchar,
    240 as libc::c_int as libc::c_uchar,
    241 as libc::c_int as libc::c_uchar,
    242 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    247 as libc::c_int as libc::c_uchar,
    248 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    251 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    254 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn mm_bseq_open(
    mut fn_0: *const libc::c_char,
) -> *mut mm_bseq_file_t {
    let mut fp: *mut mm_bseq_file_t = 0 as *mut mm_bseq_file_t;
    let mut f: gzFile = 0 as *mut gzFile_s;
    f = if !fn_0.is_null()
        && strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char) != 0
    {
        gzopen(fn_0, b"r\0" as *const u8 as *const libc::c_char)
    } else {
        gzdopen(0 as libc::c_int, b"r\0" as *const u8 as *const libc::c_char)
    };
    if f.is_null() {
        return 0 as *mut mm_bseq_file_t;
    }
    fp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mm_bseq_file_t>() as libc::c_ulong,
    ) as *mut mm_bseq_file_t;
    (*fp).fp = f;
    (*fp).ks = kseq_init((*fp).fp);
    return fp;
}
pub unsafe extern "C" fn mm_bseq_close(mut fp: *mut mm_bseq_file_t) {
    kseq_destroy((*fp).ks);
    gzclose((*fp).fp);
    free(fp as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn kstrdup(mut s: *const kstring_t) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = malloc(((*s).l).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memcpy(
        t as *mut libc::c_void,
        (*s).s as *const libc::c_void,
        ((*s).l).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return t;
}
#[inline]
unsafe extern "C" fn kseq2bseq(
    mut ks: *mut kseq_t,
    mut s: *mut mm_bseq1_t,
    mut with_qual: libc::c_int,
    mut with_comment: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if (*ks).name.l == 0 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"[WARNING]\x1B[1;31m empty sequence name in the input.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*s).name = kstrdup(&mut (*ks).name);
    (*s).seq = kstrdup(&mut (*ks).seq);
    i = 0 as libc::c_int;
    while i < (*ks).seq.l as libc::c_int {
        if *((*s).seq).offset(i as isize) as libc::c_int == 'u' as i32
            || *((*s).seq).offset(i as isize) as libc::c_int == 'U' as i32
        {
            let ref mut fresh2 = *((*s).seq).offset(i as isize);
            *fresh2 -= 1;
            *fresh2;
        }
        i += 1;
        i;
    }
    (*s)
        .qual = if with_qual != 0 && (*ks).qual.l != 0 {
        kstrdup(&mut (*ks).qual)
    } else {
        0 as *mut libc::c_char
    };
    (*s)
        .comment = if with_comment != 0 && (*ks).comment.l != 0 {
        kstrdup(&mut (*ks).comment)
    } else {
        0 as *mut libc::c_char
    };
    (*s).l_seq = (*ks).seq.l as libc::c_int;
}
pub unsafe extern "C" fn mm_bseq_read3(
    mut fp: *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut with_comment: libc::c_int,
    mut frag_mode: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    let mut size: int64_t = 0 as libc::c_int as int64_t;
    let mut ret: libc::c_int = 0;
    let mut a: C2RustUnnamed = {
        let mut init = C2RustUnnamed {
            n: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            a: 0 as *mut mm_bseq1_t,
        };
        init
    };
    let mut ks: *mut kseq_t = (*fp).ks;
    *n_ = 0 as libc::c_int;
    if !((*fp).s.seq).is_null() {
        if a.m < 256 as libc::c_int as libc::c_ulong {
            a.m = 256 as libc::c_int as size_t;
            a.m = (a.m).wrapping_sub(1);
            a.m;
            a.m |= a.m >> 1 as libc::c_int;
            a.m |= a.m >> 2 as libc::c_int;
            a.m |= a.m >> 4 as libc::c_int;
            a.m |= a.m >> 8 as libc::c_int;
            a.m |= a.m >> 16 as libc::c_int;
            a.m = (a.m).wrapping_add(1);
            a.m;
            a
                .a = krealloc(
                0 as *mut libc::c_void,
                a.a as *mut libc::c_void,
                (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong).wrapping_mul(a.m),
            ) as *mut mm_bseq1_t;
        }
        if a.n == a.m {
            a
                .m = if a.m != 0 {
                a.m << 1 as libc::c_int
            } else {
                2 as libc::c_int as libc::c_ulong
            };
            a
                .a = krealloc(
                0 as *mut libc::c_void,
                a.a as *mut libc::c_void,
                (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong).wrapping_mul(a.m),
            ) as *mut mm_bseq1_t;
        }
        let fresh3 = a.n;
        a.n = (a.n).wrapping_add(1);
        *(a.a).offset(fresh3 as isize) = (*fp).s;
        size = (*fp).s.l_seq as int64_t;
        memset(
            &mut (*fp).s as *mut mm_bseq1_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong,
        );
    }
    loop {
        ret = kseq_read(ks);
        if !(ret >= 0 as libc::c_int) {
            break;
        }
        let mut s: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
        if (*ks).seq.l <= 2147483647 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ks->seq.l <= INT32_MAX\0" as *const u8 as *const libc::c_char,
                b"bseq.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"mm_bseq1_t *mm_bseq_read3(mm_bseq_file_t *, int64_t, int, int, int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5721: {
            if (*ks).seq.l <= 2147483647 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"ks->seq.l <= INT32_MAX\0" as *const u8 as *const libc::c_char,
                    b"bseq.c\0" as *const u8 as *const libc::c_char,
                    95 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"mm_bseq1_t *mm_bseq_read3(mm_bseq_file_t *, int64_t, int, int, int, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if a.m == 0 as libc::c_int as libc::c_ulong {
            if a.m < 256 as libc::c_int as libc::c_ulong {
                a.m = 256 as libc::c_int as size_t;
                a.m = (a.m).wrapping_sub(1);
                a.m;
                a.m |= a.m >> 1 as libc::c_int;
                a.m |= a.m >> 2 as libc::c_int;
                a.m |= a.m >> 4 as libc::c_int;
                a.m |= a.m >> 8 as libc::c_int;
                a.m |= a.m >> 16 as libc::c_int;
                a.m = (a.m).wrapping_add(1);
                a.m;
                a
                    .a = krealloc(
                    0 as *mut libc::c_void,
                    a.a as *mut libc::c_void,
                    (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                        .wrapping_mul(a.m),
                ) as *mut mm_bseq1_t;
            }
        }
        if a.n == a.m {
            a
                .m = if a.m != 0 {
                a.m << 1 as libc::c_int
            } else {
                2 as libc::c_int as libc::c_ulong
            };
            a
                .a = krealloc(
                0 as *mut libc::c_void,
                a.a as *mut libc::c_void,
                (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong).wrapping_mul(a.m),
            ) as *mut mm_bseq1_t;
        }
        let fresh4 = a.n;
        a.n = (a.n).wrapping_add(1);
        s = &mut *(a.a).offset(fresh4 as isize) as *mut mm_bseq1_t;
        kseq2bseq(ks, s, with_qual, with_comment);
        size += (*s).l_seq as libc::c_long;
        if !(size >= chunk_size) {
            continue;
        }
        if frag_mode != 0
            && (*(a.a)
                .offset((a.n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .l_seq < 1000000 as libc::c_int
        {
            loop {
                ret = kseq_read(ks);
                if !(ret >= 0 as libc::c_int) {
                    break;
                }
                kseq2bseq(ks, &mut (*fp).s, with_qual, with_comment);
                if !(mm_qname_same(
                    (*fp).s.name,
                    (*(a.a)
                        .offset(
                            (a.n).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ))
                        .name,
                ) != 0)
                {
                    break;
                }
                if a.n == a.m {
                    a
                        .m = if a.m != 0 {
                        a.m << 1 as libc::c_int
                    } else {
                        2 as libc::c_int as libc::c_ulong
                    };
                    a
                        .a = krealloc(
                        0 as *mut libc::c_void,
                        a.a as *mut libc::c_void,
                        (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                            .wrapping_mul(a.m),
                    ) as *mut mm_bseq1_t;
                }
                let fresh5 = a.n;
                a.n = (a.n).wrapping_add(1);
                *(a.a).offset(fresh5 as isize) = (*fp).s;
                memset(
                    &mut (*fp).s as *mut mm_bseq1_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong,
                );
            }
        }
        break;
    }
    if ret < -(1 as libc::c_int) {
        if a.n != 0 {
            fprintf(
                stderr,
                b"[WARNING]\x1B[1;31m failed to parse the FASTA/FASTQ record next to '%s'. Continue anyway.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
                (*(a.a)
                    .offset(
                        (a.n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .name,
            );
        } else {
            fprintf(
                stderr,
                b"[WARNING]\x1B[1;31m failed to parse the first FASTA/FASTQ record. Continue anyway.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    *n_ = a.n as libc::c_int;
    return a.a;
}
pub unsafe extern "C" fn mm_bseq_read2(
    mut fp: *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut frag_mode: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    return mm_bseq_read3(fp, chunk_size, with_qual, 0 as libc::c_int, frag_mode, n_);
}
pub unsafe extern "C" fn mm_bseq_read(
    mut fp: *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    return mm_bseq_read2(fp, chunk_size, with_qual, 0 as libc::c_int, n_);
}
pub unsafe extern "C" fn mm_bseq_read_frag2(
    mut n_fp: libc::c_int,
    mut fp: *mut *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut with_comment: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    let mut i: libc::c_int = 0;
    let mut size: int64_t = 0 as libc::c_int as int64_t;
    let mut a: C2RustUnnamed_0 = {
        let mut init = C2RustUnnamed_0 {
            n: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            a: 0 as *mut mm_bseq1_t,
        };
        init
    };
    *n_ = 0 as libc::c_int;
    if n_fp < 1 as libc::c_int {
        return 0 as *mut mm_bseq1_t;
    }
    loop {
        let mut n_read: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < n_fp {
            if kseq_read((**fp.offset(i as isize)).ks) >= 0 as libc::c_int {
                n_read += 1;
                n_read;
            }
            i += 1;
            i;
        }
        if n_read < n_fp {
            if n_read > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"[W::%s]\x1B[1;31m query files have different number of records; extra records skipped.\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 19],
                        &[libc::c_char; 19],
                    >(b"mm_bseq_read_frag2\0"))
                        .as_ptr(),
                );
            }
            break;
        } else {
            if a.m == 0 as libc::c_int as libc::c_ulong {
                if a.m < 256 as libc::c_int as libc::c_ulong {
                    a.m = 256 as libc::c_int as size_t;
                    a.m = (a.m).wrapping_sub(1);
                    a.m;
                    a.m |= a.m >> 1 as libc::c_int;
                    a.m |= a.m >> 2 as libc::c_int;
                    a.m |= a.m >> 4 as libc::c_int;
                    a.m |= a.m >> 8 as libc::c_int;
                    a.m |= a.m >> 16 as libc::c_int;
                    a.m = (a.m).wrapping_add(1);
                    a.m;
                    a
                        .a = krealloc(
                        0 as *mut libc::c_void,
                        a.a as *mut libc::c_void,
                        (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                            .wrapping_mul(a.m),
                    ) as *mut mm_bseq1_t;
                }
            }
            i = 0 as libc::c_int;
            while i < n_fp {
                let mut s: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
                if a.n == a.m {
                    a
                        .m = if a.m != 0 {
                        a.m << 1 as libc::c_int
                    } else {
                        2 as libc::c_int as libc::c_ulong
                    };
                    a
                        .a = krealloc(
                        0 as *mut libc::c_void,
                        a.a as *mut libc::c_void,
                        (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                            .wrapping_mul(a.m),
                    ) as *mut mm_bseq1_t;
                }
                let fresh6 = a.n;
                a.n = (a.n).wrapping_add(1);
                s = &mut *(a.a).offset(fresh6 as isize) as *mut mm_bseq1_t;
                kseq2bseq((**fp.offset(i as isize)).ks, s, with_qual, with_comment);
                size += (*s).l_seq as libc::c_long;
                i += 1;
                i;
            }
            if size >= chunk_size {
                break;
            }
        }
    }
    *n_ = a.n as libc::c_int;
    return a.a;
}
pub unsafe extern "C" fn mm_bseq_read_frag(
    mut n_fp: libc::c_int,
    mut fp: *mut *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    return mm_bseq_read_frag2(n_fp, fp, chunk_size, with_qual, 0 as libc::c_int, n_);
}
pub unsafe extern "C" fn mm_bseq_eof(mut fp: *mut mm_bseq_file_t) -> libc::c_int {
    return ((*(*(*fp).ks).f).is_eof() != 0
        && (*(*(*fp).ks).f).begin >= (*(*(*fp).ks).f).end && ((*fp).s.seq).is_null())
        as libc::c_int;
}
