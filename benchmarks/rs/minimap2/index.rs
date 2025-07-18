use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_bseq_file_s;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
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
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn kt_for(
        n_threads: libc::c_int,
        func: Option::<
            unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
        >,
        data: *mut libc::c_void,
        n: libc::c_long,
    );
    fn kt_pipeline(
        n_threads: libc::c_int,
        func: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_void,
            ) -> *mut libc::c_void,
        >,
        shared_data: *mut libc::c_void,
        n_steps: libc::c_int,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mm_bseq_close(fp: *mut mm_bseq_file_t);
    fn mm_bseq_open(fn_0: *const libc::c_char) -> *mut mm_bseq_file_t;
    fn mm_bseq_read(
        fp: *mut mm_bseq_file_t,
        chunk_size: int64_t,
        with_qual: libc::c_int,
        n_: *mut libc::c_int,
    ) -> *mut mm_bseq1_t;
    fn mm_bseq_eof(fp: *mut mm_bseq_file_t) -> libc::c_int;
    static mut seq_nt4_table: [libc::c_uchar; 256];
    static mut mm_verbose: libc::c_int;
    static mut mm_dbg_flag: libc::c_int;
    static mut mm_realtime0: libc::c_double;
    fn cputime() -> libc::c_double;
    fn realtime() -> libc::c_double;
    fn radix_sort_128x(beg: *mut mm128_t, end: *mut mm128_t);
    fn radix_sort_64(beg: *mut uint64_t, end: *mut uint64_t);
    fn ks_ksmall_uint32_t(n: size_t, arr: *mut uint32_t, kk: size_t) -> uint32_t;
    fn mm_sketch(
        km: *mut libc::c_void,
        str: *const libc::c_char,
        len: libc::c_int,
        w: libc::c_int,
        k: libc::c_int,
        rid: uint32_t,
        is_hpc: libc::c_int,
        p: *mut mm128_v,
    );
    fn mm_idxopt_init(opt: *mut mm_idxopt_t);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn krealloc(
        km: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn kcalloc(km: *mut libc::c_void, count: size_t, size: size_t) -> *mut libc::c_void;
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    fn km_init() -> *mut libc::c_void;
    fn km_destroy(km: *mut libc::c_void);
    fn gzdopen(fd: libc::c_int, mode: *const libc::c_char) -> gzFile;
    fn gzclose(file: gzFile) -> libc::c_int;
    fn gzopen(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
    fn ks_getuntil2(
        ks: *mut kstream_t,
        delimiter: libc::c_int,
        str: *mut kstring_t,
        dret: *mut libc::c_int,
        append: libc::c_int,
    ) -> libc::c_int;
    fn ks_destroy(ks: *mut kstream_t);
    fn ks_init(f: gzFile) -> *mut kstream_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type mm_bseq_file_t = mm_bseq_file_s;
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
pub struct mm128_t {
    pub x: uint64_t,
    pub y: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_v {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm128_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_seq_t {
    pub name: *mut libc::c_char,
    pub offset: uint64_t,
    pub len: uint32_t,
    pub is_alt: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_t {
    pub b: int32_t,
    pub w: int32_t,
    pub k: int32_t,
    pub flag: int32_t,
    pub n_seq: uint32_t,
    pub index: int32_t,
    pub n_alt: int32_t,
    pub seq: *mut mm_idx_seq_t,
    pub S: *mut uint32_t,
    pub B: *mut mm_idx_bucket_s,
    pub I: *mut mm_idx_intv_s,
    pub km: *mut libc::c_void,
    pub h: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_intv_s {
    pub n: int32_t,
    pub m: int32_t,
    pub a: *mut mm_idx_intv1_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mm_idx_intv1_t {
    pub st: int32_t,
    pub en: int32_t,
    pub max: int32_t,
    #[bitfield(name = "score", ty = "int32_t", bits = "0..=29")]
    #[bitfield(name = "strand", ty = "int32_t", bits = "30..=31")]
    pub score_strand: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_bucket_s {
    pub a: mm128_v,
    pub n: int32_t,
    pub p: *mut uint64_t,
    pub h: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idxopt_t {
    pub k: libc::c_short,
    pub w: libc::c_short,
    pub flag: libc::c_short,
    pub bucket_bits: libc::c_short,
    pub mini_batch_size: int64_t,
    pub batch_size: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_reader_t {
    pub is_idx: libc::c_int,
    pub n_parts: libc::c_int,
    pub idx_size: int64_t,
    pub opt: mm_idxopt_t,
    pub fp_out: *mut FILE,
    pub fp: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub seq: *mut mm_bseq_file_s,
    pub idx: *mut FILE,
}
pub type khint_t = khint32_t;
pub type khint32_t = libc::c_uint;
pub type idxhash_t = kh_idx_t;
pub type kh_idx_t = kh_idx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_idx_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut uint64_t,
    pub vals: *mut uint64_t,
}
pub type mm_idx_bucket_t = mm_idx_bucket_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pipeline_t {
    pub mini_batch_size: libc::c_int,
    pub batch_size: uint64_t,
    pub sum_len: uint64_t,
    pub fp: *mut mm_bseq_file_t,
    pub mi: *mut mm_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct step_t {
    pub n_seq: libc::c_int,
    pub seq: *mut mm_bseq1_t,
    pub a: mm128_v,
}
pub type kh_cstr_t = *const libc::c_char;
pub type kh_str_t = kh_str_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_str_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut kh_cstr_t,
    pub vals: *mut uint32_t,
}
pub type kstring_t = __kstring_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __kstring_t {
    pub l: size_t,
    pub m: size_t,
    pub s: *mut libc::c_char,
}
pub const _ISspace: C2RustUnnamed_0 = 8192;
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
pub type gzFile = *mut gzFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsbucket_bed_t {
    pub b: *mut mm_idx_intv1_t,
    pub e: *mut mm_idx_intv1_t,
}
pub type mm_idx_intv_t = mm_idx_intv_s;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
static mut __ac_HASH_UPPER: libc::c_double = 0.77f64;
#[inline]
unsafe extern "C" fn __ac_X31_hash_string(mut s: *const libc::c_char) -> khint_t {
    let mut h: khint_t = *s as khint_t;
    if h != 0 {
        s = s.offset(1);
        s;
        while *s != 0 {
            h = (h << 5 as libc::c_int).wrapping_sub(h).wrapping_add(*s as khint_t);
            s = s.offset(1);
            s;
        }
    }
    return h;
}
#[inline]
unsafe extern "C" fn kh_put_idx(
    mut h: *mut kh_idx_t,
    mut key: uint64_t,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            if kh_resize_idx(
                h,
                ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else if kh_resize_idx(
            h,
            ((*h).n_buckets).wrapping_add(1 as libc::c_int as libc::c_uint),
        ) < 0 as libc::c_int
        {
            *ret = -(1 as libc::c_int);
            return (*h).n_buckets;
        }
    }
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = ((*h).n_buckets)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut step: khint_t = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = (key >> 1 as libc::c_int) as khint_t;
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) >> 1 as libc::c_int
                    == key >> 1 as libc::c_int))
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 2 as libc::c_int as libc::c_uint != 0 && site != (*h).n_buckets
            {
                x = site;
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh0 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh0 = (*fresh0 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        (*h).n_occupied;
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh1 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh1 = (*fresh1 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn kh_destroy_idx(mut h: *mut kh_idx_t) {
    if !h.is_null() {
        kfree(0 as *mut libc::c_void, (*h).keys as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).vals as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, h as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn kh_init_idx() -> *mut kh_idx_t {
    return kcalloc(
        0 as *mut libc::c_void,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kh_idx_t>() as libc::c_ulong,
    ) as *mut kh_idx_t;
}
#[inline]
unsafe extern "C" fn kh_resize_idx(
    mut h: *mut kh_idx_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets;
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    new_n_buckets;
    if new_n_buckets < 4 as libc::c_int as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        new_flags = kmalloc(
            0 as *mut libc::c_void,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        ) as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        memset(
            new_flags as *mut libc::c_void,
            0xaa as libc::c_int,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            let mut new_keys: *mut uint64_t = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            ) as *mut uint64_t;
            if new_keys.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            let mut new_vals: *mut uint64_t = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            ) as *mut uint64_t;
            if new_vals.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let mut key: uint64_t = *((*h).keys).offset(j as isize);
                let mut val: uint64_t = 0;
                let mut new_mask: khint_t = 0;
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_int as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                let ref mut fresh2 = *((*h).flags)
                    .offset((j >> 4 as libc::c_int) as isize);
                *fresh2 = (*fresh2 as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 0xf as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut step: khint_t = 0 as libc::c_int as khint_t;
                    k = (key >> 1 as libc::c_int) as khint_t;
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_int as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    let ref mut fresh3 = *new_flags
                        .offset((i >> 4 as libc::c_int) as isize);
                    *fresh3 = (*fresh3 as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 0xf as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets
                        && *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        let mut tmp: uint64_t = *((*h).keys).offset(i as isize);
                        *((*h).keys).offset(i as isize) = key;
                        key = tmp;
                        let mut tmp_0: uint64_t = *((*h).vals).offset(i as isize);
                        *((*h).vals).offset(i as isize) = val;
                        val = tmp_0;
                        let ref mut fresh4 = *((*h).flags)
                            .offset((i >> 4 as libc::c_int) as isize);
                        *fresh4 = (*fresh4 as libc::c_ulong
                            | (1 as libc::c_ulong)
                                << ((i & 0xf as libc::c_uint) << 1 as libc::c_int))
                            as khint32_t;
                    } else {
                        *((*h).keys).offset(i as isize) = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if (*h).n_buckets > new_n_buckets {
            (*h)
                .keys = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            ) as *mut uint64_t;
            (*h)
                .vals = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            ) as *mut uint64_t;
        }
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64)
            as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_get_idx(mut h: *const kh_idx_t, mut key: uint64_t) -> khint_t {
    if (*h).n_buckets != 0 {
        let mut k: khint_t = 0;
        let mut i: khint_t = 0;
        let mut last: khint_t = 0;
        let mut mask: khint_t = 0;
        let mut step: khint_t = 0 as libc::c_int as khint_t;
        mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint);
        k = (key >> 1 as libc::c_int) as khint_t;
        i = k & mask;
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) >> 1 as libc::c_int
                    == key >> 1 as libc::c_int))
        {
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if i == last {
                return (*h).n_buckets;
            }
        }
        return if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 3 as libc::c_int as libc::c_uint != 0
        {
            (*h).n_buckets
        } else {
            i
        };
    } else {
        return 0 as libc::c_int as khint_t
    };
}
#[inline]
unsafe extern "C" fn kh_get_str(mut h: *const kh_str_t, mut key: kh_cstr_t) -> khint_t {
    if (*h).n_buckets != 0 {
        let mut k: khint_t = 0;
        let mut i: khint_t = 0;
        let mut last: khint_t = 0;
        let mut mask: khint_t = 0;
        let mut step: khint_t = 0 as libc::c_int as khint_t;
        mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint);
        k = __ac_X31_hash_string(key);
        i = k & mask;
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(strcmp(*((*h).keys).offset(i as isize), key) == 0 as libc::c_int))
        {
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if i == last {
                return (*h).n_buckets;
            }
        }
        return if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 3 as libc::c_int as libc::c_uint != 0
        {
            (*h).n_buckets
        } else {
            i
        };
    } else {
        return 0 as libc::c_int as khint_t
    };
}
#[inline]
unsafe extern "C" fn kh_put_str(
    mut h: *mut kh_str_t,
    mut key: kh_cstr_t,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            if kh_resize_str(
                h,
                ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else if kh_resize_str(
            h,
            ((*h).n_buckets).wrapping_add(1 as libc::c_int as libc::c_uint),
        ) < 0 as libc::c_int
        {
            *ret = -(1 as libc::c_int);
            return (*h).n_buckets;
        }
    }
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = ((*h).n_buckets)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut step: khint_t = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = __ac_X31_hash_string(key);
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(strcmp(*((*h).keys).offset(i as isize), key) == 0 as libc::c_int))
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 2 as libc::c_int as libc::c_uint != 0 && site != (*h).n_buckets
            {
                x = site;
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        let ref mut fresh5 = *((*h).keys).offset(x as isize);
        *fresh5 = key;
        let ref mut fresh6 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh6 = (*fresh6 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        (*h).n_occupied;
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0
    {
        let ref mut fresh7 = *((*h).keys).offset(x as isize);
        *fresh7 = key;
        let ref mut fresh8 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh8 = (*fresh8 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn kh_resize_str(
    mut h: *mut kh_str_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets;
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    new_n_buckets;
    if new_n_buckets < 4 as libc::c_int as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        new_flags = kmalloc(
            0 as *mut libc::c_void,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        ) as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        memset(
            new_flags as *mut libc::c_void,
            0xaa as libc::c_int,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            let mut new_keys: *mut kh_cstr_t = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<kh_cstr_t>() as libc::c_ulong),
            ) as *mut kh_cstr_t;
            if new_keys.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            let mut new_vals: *mut uint32_t = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            ) as *mut uint32_t;
            if new_vals.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let mut key: kh_cstr_t = *((*h).keys).offset(j as isize);
                let mut val: uint32_t = 0;
                let mut new_mask: khint_t = 0;
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_int as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                let ref mut fresh9 = *((*h).flags)
                    .offset((j >> 4 as libc::c_int) as isize);
                *fresh9 = (*fresh9 as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 0xf as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut step: khint_t = 0 as libc::c_int as khint_t;
                    k = __ac_X31_hash_string(key);
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_int as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    let ref mut fresh10 = *new_flags
                        .offset((i >> 4 as libc::c_int) as isize);
                    *fresh10 = (*fresh10 as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 0xf as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets
                        && *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        let mut tmp: kh_cstr_t = *((*h).keys).offset(i as isize);
                        let ref mut fresh11 = *((*h).keys).offset(i as isize);
                        *fresh11 = key;
                        key = tmp;
                        let mut tmp_0: uint32_t = *((*h).vals).offset(i as isize);
                        *((*h).vals).offset(i as isize) = val;
                        val = tmp_0;
                        let ref mut fresh12 = *((*h).flags)
                            .offset((i >> 4 as libc::c_int) as isize);
                        *fresh12 = (*fresh12 as libc::c_ulong
                            | (1 as libc::c_ulong)
                                << ((i & 0xf as libc::c_uint) << 1 as libc::c_int))
                            as khint32_t;
                    } else {
                        let ref mut fresh13 = *((*h).keys).offset(i as isize);
                        *fresh13 = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if (*h).n_buckets > new_n_buckets {
            (*h)
                .keys = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<kh_cstr_t>() as libc::c_ulong),
            ) as *mut kh_cstr_t;
            (*h)
                .vals = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            ) as *mut uint32_t;
        }
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64)
            as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_init_str() -> *mut kh_str_t {
    return kcalloc(
        0 as *mut libc::c_void,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kh_str_t>() as libc::c_ulong,
    ) as *mut kh_str_t;
}
#[inline]
unsafe extern "C" fn kh_destroy_str(mut h: *mut kh_str_t) {
    if !h.is_null() {
        kfree(0 as *mut libc::c_void, (*h).keys as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).vals as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, h as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mm_idx_init(
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut b: libc::c_int,
    mut flag: libc::c_int,
) -> *mut mm_idx_t {
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    if (k * 2 as libc::c_int) < b {
        b = k * 2 as libc::c_int;
    }
    if w < 1 as libc::c_int {
        w = 1 as libc::c_int;
    }
    mi = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mm_idx_t>() as libc::c_ulong,
    ) as *mut mm_idx_t;
    (*mi).w = w;
    (*mi).k = k;
    (*mi).b = b;
    (*mi).flag = flag;
    (*mi)
        .B = calloc(
        ((1 as libc::c_int) << b) as libc::c_ulong,
        ::std::mem::size_of::<mm_idx_bucket_t>() as libc::c_ulong,
    ) as *mut mm_idx_bucket_t;
    if mm_dbg_flag & 1 as libc::c_int == 0 {
        (*mi).km = km_init();
    }
    return mi;
}
pub unsafe extern "C" fn mm_idx_destroy(mut mi: *mut mm_idx_t) {
    let mut i: uint32_t = 0;
    if mi.is_null() {
        return;
    }
    if !((*mi).h).is_null() {
        kh_destroy_str((*mi).h as *mut kh_str_t);
    }
    if !((*mi).B).is_null() {
        i = 0 as libc::c_int as uint32_t;
        while i < (1 as libc::c_uint) << (*mi).b {
            free((*((*mi).B).offset(i as isize)).p as *mut libc::c_void);
            free((*((*mi).B).offset(i as isize)).a.a as *mut libc::c_void);
            kh_destroy_idx((*((*mi).B).offset(i as isize)).h as *mut idxhash_t);
            i = i.wrapping_add(1);
            i;
        }
    }
    if !((*mi).I).is_null() {
        i = 0 as libc::c_int as uint32_t;
        while i < (*mi).n_seq {
            free((*((*mi).I).offset(i as isize)).a as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free((*mi).I as *mut libc::c_void);
    }
    if ((*mi).km).is_null() {
        i = 0 as libc::c_int as uint32_t;
        while i < (*mi).n_seq {
            free((*((*mi).seq).offset(i as isize)).name as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free((*mi).seq as *mut libc::c_void);
    } else {
        km_destroy((*mi).km);
    }
    free((*mi).B as *mut libc::c_void);
    free((*mi).S as *mut libc::c_void);
    free(mi as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_idx_get(
    mut mi: *const mm_idx_t,
    mut minier: uint64_t,
    mut n: *mut libc::c_int,
) -> *const uint64_t {
    let mut mask: libc::c_int = ((1 as libc::c_int) << (*mi).b) - 1 as libc::c_int;
    let mut k: khint_t = 0;
    let mut b: *mut mm_idx_bucket_t = &mut *((*mi).B)
        .offset((minier & mask as libc::c_ulong) as isize) as *mut mm_idx_bucket_s;
    let mut h: *mut idxhash_t = (*b).h as *mut idxhash_t;
    *n = 0 as libc::c_int;
    if h.is_null() {
        return 0 as *const uint64_t;
    }
    k = kh_get_idx(h, minier >> (*mi).b << 1 as libc::c_int);
    if k == (*h).n_buckets {
        return 0 as *const uint64_t;
    }
    if *((*h).keys).offset(k as isize) & 1 as libc::c_int as libc::c_ulong != 0 {
        *n = 1 as libc::c_int;
        return &mut *((*h).vals).offset(k as isize) as *mut uint64_t;
    } else {
        *n = *((*h).vals).offset(k as isize) as uint32_t as libc::c_int;
        return &mut *((*b).p)
            .offset((*((*h).vals).offset(k as isize) >> 32 as libc::c_int) as isize)
            as *mut uint64_t;
    };
}
pub unsafe extern "C" fn mm_idx_stat(mut mi: *const mm_idx_t) {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut n1: libc::c_int = 0 as libc::c_int;
    let mut i: uint32_t = 0;
    let mut sum: uint64_t = 0 as libc::c_int as uint64_t;
    let mut len: uint64_t = 0 as libc::c_int as uint64_t;
    fprintf(
        stderr,
        b"[M::%s] kmer size: %d; skip: %d; is_hpc: %d; #seq: %d\n\0" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"mm_idx_stat\0"))
            .as_ptr(),
        (*mi).k,
        (*mi).w,
        (*mi).flag & 0x1 as libc::c_int,
        (*mi).n_seq,
    );
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        len = (len as libc::c_ulong)
            .wrapping_add((*((*mi).seq).offset(i as isize)).len as libc::c_ulong)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (1 as libc::c_uint) << (*mi).b {
        if !((*((*mi).B).offset(i as isize)).h).is_null() {
            n = (n as libc::c_uint)
                .wrapping_add(
                    (*((*((*mi).B).offset(i as isize)).h as *mut idxhash_t)).size,
                ) as libc::c_int as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (1 as libc::c_uint) << (*mi).b {
        let mut h: *mut idxhash_t = (*((*mi).B).offset(i as isize)).h as *mut idxhash_t;
        let mut k: khint_t = 0;
        if !h.is_null() {
            k = 0 as libc::c_int as khint_t;
            while k < (*h).n_buckets {
                if *((*h).flags).offset((k >> 4 as libc::c_int) as isize)
                    >> ((k & 0xf as libc::c_uint) << 1 as libc::c_int)
                    & 3 as libc::c_int as libc::c_uint == 0
                {
                    sum = (sum as libc::c_ulong)
                        .wrapping_add(
                            (if *((*h).keys).offset(k as isize)
                                & 1 as libc::c_int as libc::c_ulong != 0
                            {
                                1 as libc::c_int as libc::c_uint
                            } else {
                                *((*h).vals).offset(k as isize) as uint32_t
                            }) as libc::c_ulong,
                        ) as uint64_t as uint64_t;
                    if *((*h).keys).offset(k as isize)
                        & 1 as libc::c_int as libc::c_ulong != 0
                    {
                        n1 += 1;
                        n1;
                    }
                }
                k = k.wrapping_add(1);
                k;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(
        stderr,
        b"[M::%s::%.3f*%.2f] distinct minimizers: %d (%.2f%% are singletons); average occurrences: %.3lf; average spacing: %.3lf; total length: %ld\n\0"
            as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"mm_idx_stat\0"))
            .as_ptr(),
        realtime() - mm_realtime0,
        cputime() / (realtime() - mm_realtime0),
        n,
        100.0f64 * n1 as libc::c_double / n as libc::c_double,
        sum as libc::c_double / n as libc::c_double,
        len as libc::c_double / sum as libc::c_double,
        len as libc::c_long,
    );
}
pub unsafe extern "C" fn mm_idx_index_name(mut mi: *mut mm_idx_t) -> libc::c_int {
    let mut h: *mut kh_str_t = 0 as *mut kh_str_t;
    let mut i: uint32_t = 0;
    let mut has_dup: libc::c_int = 0 as libc::c_int;
    let mut absent: libc::c_int = 0;
    if !((*mi).h).is_null() {
        return 0 as libc::c_int;
    }
    h = kh_init_str();
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        let mut k: khint_t = 0;
        k = kh_put_str(
            h,
            (*((*mi).seq).offset(i as isize)).name as kh_cstr_t,
            &mut absent,
        );
        if absent != 0 {
            *((*h).vals).offset(k as isize) = i;
        } else {
            has_dup = 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*mi).h = h as *mut libc::c_void;
    if has_dup != 0 && mm_verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"[WARNING] some database sequences have identical sequence names\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return has_dup;
}
pub unsafe extern "C" fn mm_idx_name2id(
    mut mi: *const mm_idx_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut h: *mut kh_str_t = (*mi).h as *mut kh_str_t;
    let mut k: khint_t = 0;
    if h.is_null() {
        return -(2 as libc::c_int);
    }
    k = kh_get_str(h, name);
    return (if k == (*h).n_buckets {
        -(1 as libc::c_int) as libc::c_uint
    } else {
        *((*h).vals).offset(k as isize)
    }) as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_getseq(
    mut mi: *const mm_idx_t,
    mut rid: uint32_t,
    mut st: uint32_t,
    mut en: uint32_t,
    mut seq: *mut uint8_t,
) -> libc::c_int {
    let mut i: uint64_t = 0;
    let mut st1: uint64_t = 0;
    let mut en1: uint64_t = 0;
    if rid >= (*mi).n_seq || st >= (*((*mi).seq).offset(rid as isize)).len {
        return -(1 as libc::c_int);
    }
    if en > (*((*mi).seq).offset(rid as isize)).len {
        en = (*((*mi).seq).offset(rid as isize)).len;
    }
    st1 = ((*((*mi).seq).offset(rid as isize)).offset).wrapping_add(st as libc::c_ulong);
    en1 = ((*((*mi).seq).offset(rid as isize)).offset).wrapping_add(en as libc::c_ulong);
    i = st1;
    while i < en1 {
        *seq
            .offset(
                i.wrapping_sub(st1) as isize,
            ) = (*((*mi).S).offset((i >> 3 as libc::c_int) as isize)
            >> ((i & 7 as libc::c_int as libc::c_ulong) << 2 as libc::c_int)
            & 0xf as libc::c_int as libc::c_uint) as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
    return en.wrapping_sub(st) as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_getseq_rev(
    mut mi: *const mm_idx_t,
    mut rid: uint32_t,
    mut st: uint32_t,
    mut en: uint32_t,
    mut seq: *mut uint8_t,
) -> libc::c_int {
    let mut i: uint64_t = 0;
    let mut st1: uint64_t = 0;
    let mut en1: uint64_t = 0;
    let mut s: *const mm_idx_seq_t = 0 as *const mm_idx_seq_t;
    if rid >= (*mi).n_seq || st >= (*((*mi).seq).offset(rid as isize)).len {
        return -(1 as libc::c_int);
    }
    s = &mut *((*mi).seq).offset(rid as isize) as *mut mm_idx_seq_t;
    if en > (*s).len {
        en = (*s).len;
    }
    st1 = ((*s).offset).wrapping_add(((*s).len).wrapping_sub(en) as libc::c_ulong);
    en1 = ((*s).offset).wrapping_add(((*s).len).wrapping_sub(st) as libc::c_ulong);
    i = st1;
    while i < en1 {
        let mut c: uint8_t = (*((*mi).S).offset((i >> 3 as libc::c_int) as isize)
            >> ((i & 7 as libc::c_int as libc::c_ulong) << 2 as libc::c_int)
            & 0xf as libc::c_int as libc::c_uint) as uint8_t;
        *seq
            .offset(
                en1.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) = (if (c as libc::c_int) < 4 as libc::c_int {
            3 as libc::c_int - c as libc::c_int
        } else {
            c as libc::c_int
        }) as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
    return en.wrapping_sub(st) as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_getseq2(
    mut mi: *const mm_idx_t,
    mut is_rev: libc::c_int,
    mut rid: uint32_t,
    mut st: uint32_t,
    mut en: uint32_t,
    mut seq: *mut uint8_t,
) -> libc::c_int {
    if is_rev != 0 {
        return mm_idx_getseq_rev(mi, rid, st, en, seq)
    } else {
        return mm_idx_getseq(mi, rid, st, en, seq)
    };
}
pub unsafe extern "C" fn mm_idx_cal_max_occ(
    mut mi: *const mm_idx_t,
    mut f: libc::c_float,
) -> int32_t {
    let mut i: libc::c_int = 0;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut thres: uint32_t = 0;
    let mut a: *mut khint_t = 0 as *mut khint_t;
    let mut k: khint_t = 0;
    if f as libc::c_double <= 0.0f64 {
        return 2147483647 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << (*mi).b {
        if !((*((*mi).B).offset(i as isize)).h).is_null() {
            n = (n as libc::c_ulong)
                .wrapping_add(
                    (*((*((*mi).B).offset(i as isize)).h as *mut idxhash_t)).size
                        as libc::c_ulong,
                ) as size_t as size_t;
        }
        i += 1;
        i;
    }
    a = malloc(n.wrapping_mul(4 as libc::c_int as libc::c_ulong)) as *mut uint32_t;
    n = 0 as libc::c_int as size_t;
    i = n as libc::c_int;
    while i < (1 as libc::c_int) << (*mi).b {
        let mut h: *mut idxhash_t = (*((*mi).B).offset(i as isize)).h as *mut idxhash_t;
        if !h.is_null() {
            k = 0 as libc::c_int as khint_t;
            while k < (*h).n_buckets {
                if !(*((*h).flags).offset((k >> 4 as libc::c_int) as isize)
                    >> ((k & 0xf as libc::c_uint) << 1 as libc::c_int)
                    & 3 as libc::c_int as libc::c_uint != 0)
                {
                    let fresh14 = n;
                    n = n.wrapping_add(1);
                    *a
                        .offset(
                            fresh14 as isize,
                        ) = if *((*h).keys).offset(k as isize)
                        & 1 as libc::c_int as libc::c_ulong != 0
                    {
                        1 as libc::c_int as libc::c_uint
                    } else {
                        *((*h).vals).offset(k as isize) as uint32_t
                    };
                }
                k = k.wrapping_add(1);
                k;
            }
        }
        i += 1;
        i;
    }
    thres = (ks_ksmall_uint32_t(
        n,
        a,
        ((1.0f64 - f as libc::c_double) * n as libc::c_double) as uint32_t as size_t,
    ))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    free(a as *mut libc::c_void);
    return thres as int32_t;
}
unsafe extern "C" fn worker_post(
    mut g: *mut libc::c_void,
    mut i: libc::c_long,
    mut tid: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut n_keys: libc::c_int = 0;
    let mut j: size_t = 0;
    let mut start_a: size_t = 0;
    let mut start_p: size_t = 0;
    let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
    let mut mi: *mut mm_idx_t = g as *mut mm_idx_t;
    let mut b: *mut mm_idx_bucket_t = &mut *((*mi).B).offset(i as isize)
        as *mut mm_idx_bucket_s;
    if (*b).a.n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    radix_sort_128x((*b).a.a, ((*b).a.a).offset((*b).a.n as isize));
    j = 1 as libc::c_int as size_t;
    n = 1 as libc::c_int;
    n_keys = 0 as libc::c_int;
    (*b).n = 0 as libc::c_int;
    while j <= (*b).a.n {
        if j == (*b).a.n
            || (*((*b).a.a).offset(j as isize)).x >> 8 as libc::c_int
                != (*((*b).a.a)
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .x >> 8 as libc::c_int
        {
            n_keys += 1;
            n_keys;
            if n > 1 as libc::c_int {
                (*b).n += n;
            }
            n = 1 as libc::c_int;
        } else {
            n += 1;
            n;
        }
        j = j.wrapping_add(1);
        j;
    }
    h = kh_init_idx();
    kh_resize_idx(h, n_keys as khint_t);
    (*b)
        .p = calloc((*b).n as libc::c_ulong, 8 as libc::c_int as libc::c_ulong)
        as *mut uint64_t;
    j = 1 as libc::c_int as size_t;
    n = 1 as libc::c_int;
    start_p = 0 as libc::c_int as size_t;
    start_a = start_p;
    while j <= (*b).a.n {
        if j == (*b).a.n
            || (*((*b).a.a).offset(j as isize)).x >> 8 as libc::c_int
                != (*((*b).a.a)
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .x >> 8 as libc::c_int
        {
            let mut itr: khint_t = 0;
            let mut absent: libc::c_int = 0;
            let mut p: *mut mm128_t = &mut *((*b).a.a)
                .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as *mut mm128_t;
            itr = kh_put_idx(
                h,
                (*p).x >> 8 as libc::c_int >> (*mi).b << 1 as libc::c_int,
                &mut absent,
            );
            if absent != 0 && j == start_a.wrapping_add(n as libc::c_ulong) {} else {
                __assert_fail(
                    b"absent && j == start_a + n\0" as *const u8 as *const libc::c_char,
                    b"index.c\0" as *const u8 as *const libc::c_char,
                    244 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"void worker_post(void *, long, int)\0"))
                        .as_ptr(),
                );
            }
            'c_4815: {
                if absent != 0 && j == start_a.wrapping_add(n as libc::c_ulong) {} else {
                    __assert_fail(
                        b"absent && j == start_a + n\0" as *const u8
                            as *const libc::c_char,
                        b"index.c\0" as *const u8 as *const libc::c_char,
                        244 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 36],
                            &[libc::c_char; 36],
                        >(b"void worker_post(void *, long, int)\0"))
                            .as_ptr(),
                    );
                }
            };
            if n == 1 as libc::c_int {
                let ref mut fresh15 = *((*h).keys).offset(itr as isize);
                *fresh15 |= 1 as libc::c_int as libc::c_ulong;
                *((*h).vals).offset(itr as isize) = (*p).y;
            } else {
                let mut k: libc::c_int = 0;
                k = 0 as libc::c_int;
                while k < n {
                    *((*b).p)
                        .offset(
                            start_p.wrapping_add(k as libc::c_ulong) as isize,
                        ) = (*((*b).a.a)
                        .offset(start_a.wrapping_add(k as libc::c_ulong) as isize))
                        .y;
                    k += 1;
                    k;
                }
                radix_sort_64(
                    &mut *((*b).p).offset(start_p as isize),
                    &mut *((*b).p)
                        .offset(start_p.wrapping_add(n as libc::c_ulong) as isize),
                );
                *((*h).vals)
                    .offset(
                        itr as isize,
                    ) = start_p << 32 as libc::c_int | n as libc::c_ulong;
                start_p = (start_p as libc::c_ulong).wrapping_add(n as libc::c_ulong)
                    as size_t as size_t;
            }
            start_a = j;
            n = 1 as libc::c_int;
        } else {
            n += 1;
            n;
        }
        j = j.wrapping_add(1);
        j;
    }
    (*b).h = h as *mut libc::c_void;
    if (*b).n == start_p as int32_t {} else {
        __assert_fail(
            b"b->n == (int32_t)start_p\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void worker_post(void *, long, int)\0"))
                .as_ptr(),
        );
    }
    'c_4585: {
        if (*b).n == start_p as int32_t {} else {
            __assert_fail(
                b"b->n == (int32_t)start_p\0" as *const u8 as *const libc::c_char,
                b"index.c\0" as *const u8 as *const libc::c_char,
                260 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void worker_post(void *, long, int)\0"))
                    .as_ptr(),
            );
        }
    };
    kfree(0 as *mut libc::c_void, (*b).a.a as *mut libc::c_void);
    (*b).a.m = 0 as libc::c_int as size_t;
    (*b).a.n = (*b).a.m;
    (*b).a.a = 0 as *mut mm128_t;
}
unsafe extern "C" fn mm_idx_post(mut mi: *mut mm_idx_t, mut n_threads: libc::c_int) {
    kt_for(
        n_threads,
        Some(
            worker_post
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_long,
                    libc::c_int,
                ) -> (),
        ),
        mi as *mut libc::c_void,
        ((1 as libc::c_int) << (*mi).b) as libc::c_long,
    );
}
unsafe extern "C" fn mm_idx_add(
    mut mi: *mut mm_idx_t,
    mut n: libc::c_int,
    mut a: *const mm128_t,
) {
    let mut i: libc::c_int = 0;
    let mut mask: libc::c_int = ((1 as libc::c_int) << (*mi).b) - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let mut p: *mut mm128_v = &mut (*((*mi).B)
            .offset(
                ((*a.offset(i as isize)).x >> 8 as libc::c_int & mask as libc::c_ulong)
                    as isize,
            ))
            .a;
        if (*p).n == (*p).m {
            (*p)
                .m = if (*p).m != 0 {
                (*p).m << 1 as libc::c_int
            } else {
                2 as libc::c_int as libc::c_ulong
            };
            (*p)
                .a = krealloc(
                0 as *mut libc::c_void,
                (*p).a as *mut libc::c_void,
                (::std::mem::size_of::<mm128_t>() as libc::c_ulong).wrapping_mul((*p).m),
            ) as *mut mm128_t;
        }
        let fresh16 = (*p).n;
        (*p).n = ((*p).n).wrapping_add(1);
        *((*p).a).offset(fresh16 as isize) = *a.offset(i as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn worker_pipeline(
    mut shared: *mut libc::c_void,
    mut step: libc::c_int,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut p: *mut pipeline_t = shared as *mut pipeline_t;
    if step == 0 as libc::c_int {
        let mut s: *mut step_t = 0 as *mut step_t;
        if (*p).sum_len > (*p).batch_size {
            return 0 as *mut libc::c_void;
        }
        s = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<step_t>() as libc::c_ulong,
        ) as *mut step_t;
        (*s)
            .seq = mm_bseq_read(
            (*p).fp,
            (*p).mini_batch_size as int64_t,
            0 as libc::c_int,
            &mut (*s).n_seq,
        );
        if !((*s).seq).is_null() {
            let mut old_m: uint32_t = 0;
            let mut m: uint32_t = 0;
            if ((*(*p).mi).n_seq as uint64_t).wrapping_add((*s).n_seq as libc::c_ulong)
                <= 4294967295 as libc::c_uint as libc::c_ulong
            {} else {
                __assert_fail(
                    b"(uint64_t)p->mi->n_seq + s->n_seq <= UINT32_MAX\0" as *const u8
                        as *const libc::c_char,
                    b"index.c\0" as *const u8 as *const libc::c_char,
                    313 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"void *worker_pipeline(void *, int, void *)\0"))
                        .as_ptr(),
                );
            }
            'c_7479: {
                if ((*(*p).mi).n_seq as uint64_t)
                    .wrapping_add((*s).n_seq as libc::c_ulong)
                    <= 4294967295 as libc::c_uint as libc::c_ulong
                {} else {
                    __assert_fail(
                        b"(uint64_t)p->mi->n_seq + s->n_seq <= UINT32_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"index.c\0" as *const u8 as *const libc::c_char,
                        313 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 43],
                            &[libc::c_char; 43],
                        >(b"void *worker_pipeline(void *, int, void *)\0"))
                            .as_ptr(),
                    );
                }
            };
            old_m = (*(*p).mi).n_seq;
            m = ((*(*p).mi).n_seq).wrapping_add((*s).n_seq as libc::c_uint);
            m = m.wrapping_sub(1);
            m;
            m |= m >> 1 as libc::c_int;
            m |= m >> 2 as libc::c_int;
            m |= m >> 4 as libc::c_int;
            m |= m >> 8 as libc::c_int;
            m |= m >> 16 as libc::c_int;
            m = m.wrapping_add(1);
            m;
            old_m = old_m.wrapping_sub(1);
            old_m;
            old_m |= old_m >> 1 as libc::c_int;
            old_m |= old_m >> 2 as libc::c_int;
            old_m |= old_m >> 4 as libc::c_int;
            old_m |= old_m >> 8 as libc::c_int;
            old_m |= old_m >> 16 as libc::c_int;
            old_m = old_m.wrapping_add(1);
            old_m;
            if old_m != m {
                (*(*p).mi)
                    .seq = krealloc(
                    (*(*p).mi).km,
                    (*(*p).mi).seq as *mut libc::c_void,
                    (m as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
                        ),
                ) as *mut mm_idx_seq_t;
            }
            if (*(*p).mi).flag & 0x2 as libc::c_int == 0 {
                let mut sum_len: uint64_t = 0;
                let mut old_max_len: uint64_t = 0;
                let mut max_len: uint64_t = 0;
                i = 0 as libc::c_int;
                sum_len = 0 as libc::c_int as uint64_t;
                while i < (*s).n_seq {
                    sum_len = (sum_len as libc::c_ulong)
                        .wrapping_add(
                            (*((*s).seq).offset(i as isize)).l_seq as libc::c_ulong,
                        ) as uint64_t as uint64_t;
                    i += 1;
                    i;
                }
                old_max_len = ((*p).sum_len)
                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                    .wrapping_div(8 as libc::c_int as libc::c_ulong);
                max_len = ((*p).sum_len)
                    .wrapping_add(sum_len)
                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                    .wrapping_div(8 as libc::c_int as libc::c_ulong);
                old_max_len = old_max_len.wrapping_sub(1);
                old_max_len;
                old_max_len |= old_max_len >> 1 as libc::c_int;
                old_max_len |= old_max_len >> 2 as libc::c_int;
                old_max_len |= old_max_len >> 4 as libc::c_int;
                old_max_len |= old_max_len >> 8 as libc::c_int;
                old_max_len |= old_max_len >> 16 as libc::c_int;
                old_max_len |= old_max_len >> 32 as libc::c_int;
                old_max_len = old_max_len.wrapping_add(1);
                old_max_len;
                max_len = max_len.wrapping_sub(1);
                max_len;
                max_len |= max_len >> 1 as libc::c_int;
                max_len |= max_len >> 2 as libc::c_int;
                max_len |= max_len >> 4 as libc::c_int;
                max_len |= max_len >> 8 as libc::c_int;
                max_len |= max_len >> 16 as libc::c_int;
                max_len |= max_len >> 32 as libc::c_int;
                max_len = max_len.wrapping_add(1);
                max_len;
                if old_max_len != max_len {
                    (*(*p).mi)
                        .S = realloc(
                        (*(*p).mi).S as *mut libc::c_void,
                        max_len.wrapping_mul(4 as libc::c_int as libc::c_ulong),
                    ) as *mut uint32_t;
                    memset(
                        &mut *((*(*p).mi).S).offset(old_max_len as isize)
                            as *mut uint32_t as *mut libc::c_void,
                        0 as libc::c_int,
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(max_len.wrapping_sub(old_max_len)),
                    );
                }
            }
            i = 0 as libc::c_int;
            while i < (*s).n_seq {
                let mut seq: *mut mm_idx_seq_t = &mut *((*(*p).mi).seq)
                    .offset((*(*p).mi).n_seq as isize) as *mut mm_idx_seq_t;
                let mut j: uint32_t = 0;
                if (*(*p).mi).flag & 0x4 as libc::c_int == 0 {
                    (*seq)
                        .name = kmalloc(
                        (*(*p).mi).km,
                        (strlen((*((*s).seq).offset(i as isize)).name))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    strcpy((*seq).name, (*((*s).seq).offset(i as isize)).name);
                } else {
                    (*seq).name = 0 as *mut libc::c_char;
                }
                (*seq).len = (*((*s).seq).offset(i as isize)).l_seq as uint32_t;
                (*seq).offset = (*p).sum_len;
                (*seq).is_alt = 0 as libc::c_int as uint32_t;
                if (*(*p).mi).flag & 0x2 as libc::c_int == 0 {
                    j = 0 as libc::c_int as uint32_t;
                    while j < (*seq).len {
                        let mut o: uint64_t = ((*p).sum_len)
                            .wrapping_add(j as libc::c_ulong);
                        let mut c: libc::c_int = seq_nt4_table[*((*((*s).seq)
                            .offset(i as isize))
                            .seq)
                            .offset(j as isize) as uint8_t as usize] as libc::c_int;
                        let ref mut fresh17 = *((*(*p).mi).S)
                            .offset((o >> 3 as libc::c_int) as isize);
                        *fresh17
                            |= (c as uint32_t)
                                << ((o & 7 as libc::c_int as libc::c_ulong)
                                    << 2 as libc::c_int);
                        j = j.wrapping_add(1);
                        j;
                    }
                }
                (*p)
                    .sum_len = ((*p).sum_len as libc::c_ulong)
                    .wrapping_add((*seq).len as libc::c_ulong) as uint64_t as uint64_t;
                let fresh18 = (*(*p).mi).n_seq;
                (*(*p).mi).n_seq = ((*(*p).mi).n_seq).wrapping_add(1);
                (*((*s).seq).offset(i as isize)).rid = fresh18 as libc::c_int;
                i += 1;
                i;
            }
            return s as *mut libc::c_void;
        } else {
            free(s as *mut libc::c_void);
        }
    } else if step == 1 as libc::c_int {
        let mut s_0: *mut step_t = in_0 as *mut step_t;
        i = 0 as libc::c_int;
        while i < (*s_0).n_seq {
            let mut t: *mut mm_bseq1_t = &mut *((*s_0).seq).offset(i as isize)
                as *mut mm_bseq1_t;
            if (*t).l_seq > 0 as libc::c_int {
                mm_sketch(
                    0 as *mut libc::c_void,
                    (*t).seq,
                    (*t).l_seq,
                    (*(*p).mi).w,
                    (*(*p).mi).k,
                    (*t).rid as uint32_t,
                    (*(*p).mi).flag & 0x1 as libc::c_int,
                    &mut (*s_0).a,
                );
            } else if mm_verbose >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"[WARNING] the length database sequence '%s' is 0\n\0" as *const u8
                        as *const libc::c_char,
                    (*t).name,
                );
            }
            free((*t).seq as *mut libc::c_void);
            free((*t).name as *mut libc::c_void);
            i += 1;
            i;
        }
        free((*s_0).seq as *mut libc::c_void);
        (*s_0).seq = 0 as *mut mm_bseq1_t;
        return s_0 as *mut libc::c_void;
    } else if step == 2 as libc::c_int {
        let mut s_1: *mut step_t = in_0 as *mut step_t;
        mm_idx_add((*p).mi, (*s_1).a.n as libc::c_int, (*s_1).a.a);
        kfree(0 as *mut libc::c_void, (*s_1).a.a as *mut libc::c_void);
        free(s_1 as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn mm_idx_gen(
    mut fp: *mut mm_bseq_file_t,
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut b: libc::c_int,
    mut flag: libc::c_int,
    mut mini_batch_size: libc::c_int,
    mut n_threads: libc::c_int,
    mut batch_size: uint64_t,
) -> *mut mm_idx_t {
    let mut pl: pipeline_t = pipeline_t {
        mini_batch_size: 0,
        batch_size: 0,
        sum_len: 0,
        fp: 0 as *mut mm_bseq_file_t,
        mi: 0 as *mut mm_idx_t,
    };
    if fp.is_null() || mm_bseq_eof(fp) != 0 {
        return 0 as *mut mm_idx_t;
    }
    memset(
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pipeline_t>() as libc::c_ulong,
    );
    pl
        .mini_batch_size = (if (mini_batch_size as uint64_t) < batch_size {
        mini_batch_size as libc::c_ulong
    } else {
        batch_size
    }) as libc::c_int;
    pl.batch_size = batch_size;
    pl.fp = fp;
    pl.mi = mm_idx_init(w, k, b, flag);
    kt_pipeline(
        if n_threads < 3 as libc::c_int { n_threads } else { 3 as libc::c_int },
        Some(
            worker_pipeline
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        3 as libc::c_int,
    );
    if mm_verbose >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"[M::%s::%.3f*%.2f] collected minimizers\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mm_idx_gen\0"))
                .as_ptr(),
            realtime() - mm_realtime0,
            cputime() / (realtime() - mm_realtime0),
        );
    }
    mm_idx_post(pl.mi, n_threads);
    if mm_verbose >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"[M::%s::%.3f*%.2f] sorted minimizers\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"mm_idx_gen\0"))
                .as_ptr(),
            realtime() - mm_realtime0,
            cputime() / (realtime() - mm_realtime0),
        );
    }
    return pl.mi;
}
pub unsafe extern "C" fn mm_idx_build(
    mut fn_0: *const libc::c_char,
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut flag: libc::c_int,
    mut n_threads: libc::c_int,
) -> *mut mm_idx_t {
    let mut fp: *mut mm_bseq_file_t = 0 as *mut mm_bseq_file_t;
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    fp = mm_bseq_open(fn_0);
    if fp.is_null() {
        return 0 as *mut mm_idx_t;
    }
    mi = mm_idx_gen(
        fp,
        w,
        k,
        14 as libc::c_int,
        flag,
        (1 as libc::c_int) << 18 as libc::c_int,
        n_threads,
        18446744073709551615 as libc::c_ulong,
    );
    mm_bseq_close(fp);
    return mi;
}
pub unsafe extern "C" fn mm_idx_str(
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut is_hpc: libc::c_int,
    mut bucket_bits: libc::c_int,
    mut n: libc::c_int,
    mut seq: *mut *const libc::c_char,
    mut name: *mut *const libc::c_char,
) -> *mut mm_idx_t {
    let mut sum_len: uint64_t = 0 as libc::c_int as uint64_t;
    let mut a: mm128_v = {
        let mut init = mm128_v {
            n: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            a: 0 as *mut mm128_t,
        };
        init
    };
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut h: *mut kh_str_t = 0 as *mut kh_str_t;
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    if n <= 0 as libc::c_int {
        return 0 as *mut mm_idx_t;
    }
    i = 0 as libc::c_int;
    while i < n {
        sum_len = (sum_len as libc::c_ulong)
            .wrapping_add(strlen(*seq.offset(i as isize))) as uint64_t as uint64_t;
        i += 1;
        i;
    }
    if is_hpc != 0 {
        flag |= 0x1 as libc::c_int;
    }
    if name.is_null() {
        flag |= 0x4 as libc::c_int;
    }
    if bucket_bits < 0 as libc::c_int {
        bucket_bits = 14 as libc::c_int;
    }
    mi = mm_idx_init(w, k, bucket_bits, flag);
    (*mi).n_seq = n as uint32_t;
    (*mi)
        .seq = kcalloc(
        (*mi).km,
        n as size_t,
        ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
    ) as *mut mm_idx_seq_t;
    (*mi)
        .S = calloc(
        sum_len
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            .wrapping_div(8 as libc::c_int as libc::c_ulong),
        4 as libc::c_int as libc::c_ulong,
    ) as *mut uint32_t;
    h = kh_init_str();
    (*mi).h = h as *mut libc::c_void;
    i = 0 as libc::c_int;
    sum_len = 0 as libc::c_int as uint64_t;
    while i < n {
        let mut s: *const libc::c_char = *seq.offset(i as isize);
        let mut p: *mut mm_idx_seq_t = &mut *((*mi).seq).offset(i as isize)
            as *mut mm_idx_seq_t;
        let mut j: uint32_t = 0;
        if !name.is_null() && !(*name.offset(i as isize)).is_null() {
            let mut absent: libc::c_int = 0;
            (*p)
                .name = kmalloc(
                (*mi).km,
                (strlen(*name.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcpy((*p).name, *name.offset(i as isize));
            kh_put_str(h, (*p).name as kh_cstr_t, &mut absent);
            if absent != 0 {} else {
                __assert_fail(
                    b"absent\0" as *const u8 as *const libc::c_char,
                    b"index.c\0" as *const u8 as *const libc::c_char,
                    436 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 76],
                        &[libc::c_char; 76],
                    >(
                        b"mm_idx_t *mm_idx_str(int, int, int, int, int, const char **, const char **)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_8717: {
                if absent != 0 {} else {
                    __assert_fail(
                        b"absent\0" as *const u8 as *const libc::c_char,
                        b"index.c\0" as *const u8 as *const libc::c_char,
                        436 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 76],
                            &[libc::c_char; 76],
                        >(
                            b"mm_idx_t *mm_idx_str(int, int, int, int, int, const char **, const char **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
        (*p).offset = sum_len;
        (*p).len = strlen(s) as uint32_t;
        (*p).is_alt = 0 as libc::c_int as uint32_t;
        j = 0 as libc::c_int as uint32_t;
        while j < (*p).len {
            let mut c: libc::c_int = seq_nt4_table[*s.offset(j as isize) as uint8_t
                as usize] as libc::c_int;
            let mut o: uint64_t = sum_len.wrapping_add(j as libc::c_ulong);
            let ref mut fresh19 = *((*mi).S).offset((o >> 3 as libc::c_int) as isize);
            *fresh19
                |= (c as uint32_t)
                    << ((o & 7 as libc::c_int as libc::c_ulong) << 2 as libc::c_int);
            j = j.wrapping_add(1);
            j;
        }
        sum_len = (sum_len as libc::c_ulong).wrapping_add((*p).len as libc::c_ulong)
            as uint64_t as uint64_t;
        if (*p).len > 0 as libc::c_int as libc::c_uint {
            a.n = 0 as libc::c_int as size_t;
            mm_sketch(
                0 as *mut libc::c_void,
                s,
                (*p).len as libc::c_int,
                w,
                k,
                i as uint32_t,
                is_hpc,
                &mut a,
            );
            mm_idx_add(mi, a.n as libc::c_int, a.a);
        }
        i += 1;
        i;
    }
    free(a.a as *mut libc::c_void);
    mm_idx_post(mi, 1 as libc::c_int);
    return mi;
}
pub unsafe extern "C" fn mm_idx_dump(mut fp: *mut FILE, mut mi: *const mm_idx_t) {
    let mut sum_len: uint64_t = 0 as libc::c_int as uint64_t;
    let mut x: [uint32_t; 5] = [0; 5];
    let mut i: uint32_t = 0;
    x[0 as libc::c_int as usize] = (*mi).w as uint32_t;
    x[1 as libc::c_int as usize] = (*mi).k as uint32_t;
    x[2 as libc::c_int as usize] = (*mi).b as uint32_t;
    x[3 as libc::c_int as usize] = (*mi).n_seq;
    x[4 as libc::c_int as usize] = (*mi).flag as uint32_t;
    fwrite(
        b"MMI\x02\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        4 as libc::c_int as libc::c_ulong,
        fp,
    );
    fwrite(
        x.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        5 as libc::c_int as libc::c_ulong,
        fp,
    );
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        if !((*((*mi).seq).offset(i as isize)).name).is_null() {
            let mut l: uint8_t = strlen((*((*mi).seq).offset(i as isize)).name)
                as uint8_t;
            fwrite(
                &mut l as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                fp,
            );
            fwrite(
                (*((*mi).seq).offset(i as isize)).name as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                l as libc::c_ulong,
                fp,
            );
        } else {
            let mut l_0: uint8_t = 0 as libc::c_int as uint8_t;
            fwrite(
                &mut l_0 as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                fp,
            );
        }
        fwrite(
            &mut (*((*mi).seq).offset(i as isize)).len as *mut uint32_t
                as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
        sum_len = (sum_len as libc::c_ulong)
            .wrapping_add((*((*mi).seq).offset(i as isize)).len as libc::c_ulong)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < ((1 as libc::c_int) << (*mi).b) as libc::c_uint {
        let mut b: *mut mm_idx_bucket_t = &mut *((*mi).B).offset(i as isize)
            as *mut mm_idx_bucket_s;
        let mut k: khint_t = 0;
        let mut h: *mut idxhash_t = (*b).h as *mut idxhash_t;
        let mut size: uint32_t = if !h.is_null() {
            (*h).size
        } else {
            0 as libc::c_int as libc::c_uint
        };
        fwrite(
            &mut (*b).n as *mut int32_t as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
        fwrite(
            (*b).p as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
            (*b).n as libc::c_ulong,
            fp,
        );
        fwrite(
            &mut size as *mut uint32_t as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
        if !(size == 0 as libc::c_int as libc::c_uint) {
            k = 0 as libc::c_int as khint_t;
            while k < (*h).n_buckets {
                let mut x_0: [uint64_t; 2] = [0; 2];
                if !(*((*h).flags).offset((k >> 4 as libc::c_int) as isize)
                    >> ((k & 0xf as libc::c_uint) << 1 as libc::c_int)
                    & 3 as libc::c_int as libc::c_uint != 0)
                {
                    x_0[0 as libc::c_int as usize] = *((*h).keys).offset(k as isize);
                    x_0[1 as libc::c_int as usize] = *((*h).vals).offset(k as isize);
                    fwrite(
                        x_0.as_mut_ptr() as *const libc::c_void,
                        8 as libc::c_int as libc::c_ulong,
                        2 as libc::c_int as libc::c_ulong,
                        fp,
                    );
                }
                k = k.wrapping_add(1);
                k;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*mi).flag & 0x2 as libc::c_int == 0 {
        fwrite(
            (*mi).S as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            sum_len
                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                .wrapping_div(8 as libc::c_int as libc::c_ulong),
            fp,
        );
    }
    fflush(fp);
}
pub unsafe extern "C" fn mm_idx_load(mut fp: *mut FILE) -> *mut mm_idx_t {
    let mut magic: [libc::c_char; 4] = [0; 4];
    let mut x: [uint32_t; 5] = [0; 5];
    let mut i: uint32_t = 0;
    let mut sum_len: uint64_t = 0 as libc::c_int as uint64_t;
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    if fread(
        magic.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        4 as libc::c_int as libc::c_ulong,
        fp,
    ) != 4 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut mm_idx_t;
    }
    if strncmp(
        magic.as_mut_ptr(),
        b"MMI\x02\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return 0 as *mut mm_idx_t;
    }
    if fread(
        x.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        5 as libc::c_int as libc::c_ulong,
        fp,
    ) != 5 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut mm_idx_t;
    }
    mi = mm_idx_init(
        x[0 as libc::c_int as usize] as libc::c_int,
        x[1 as libc::c_int as usize] as libc::c_int,
        x[2 as libc::c_int as usize] as libc::c_int,
        x[4 as libc::c_int as usize] as libc::c_int,
    );
    (*mi).n_seq = x[3 as libc::c_int as usize];
    (*mi)
        .seq = kcalloc(
        (*mi).km,
        (*mi).n_seq as size_t,
        ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
    ) as *mut mm_idx_seq_t;
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        let mut l: uint8_t = 0;
        let mut s: *mut mm_idx_seq_t = &mut *((*mi).seq).offset(i as isize)
            as *mut mm_idx_seq_t;
        fread(
            &mut l as *mut uint8_t as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
        if l != 0 {
            (*s)
                .name = kmalloc(
                (*mi).km,
                (l as libc::c_int + 1 as libc::c_int) as size_t,
            ) as *mut libc::c_char;
            fread(
                (*s).name as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                l as libc::c_ulong,
                fp,
            );
            *((*s).name).offset(l as isize) = 0 as libc::c_int as libc::c_char;
        }
        fread(
            &mut (*s).len as *mut uint32_t as *mut libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
        (*s).offset = sum_len;
        (*s).is_alt = 0 as libc::c_int as uint32_t;
        sum_len = (sum_len as libc::c_ulong).wrapping_add((*s).len as libc::c_ulong)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < ((1 as libc::c_int) << (*mi).b) as libc::c_uint {
        let mut b: *mut mm_idx_bucket_t = &mut *((*mi).B).offset(i as isize)
            as *mut mm_idx_bucket_s;
        let mut j: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut k: khint_t = 0;
        let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
        fread(
            &mut (*b).n as *mut int32_t as *mut libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
        (*b).p = malloc(((*b).n * 8 as libc::c_int) as libc::c_ulong) as *mut uint64_t;
        fread(
            (*b).p as *mut libc::c_void,
            8 as libc::c_int as libc::c_ulong,
            (*b).n as libc::c_ulong,
            fp,
        );
        fread(
            &mut size as *mut uint32_t as *mut libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
        if !(size == 0 as libc::c_int as libc::c_uint) {
            h = kh_init_idx();
            (*b).h = h as *mut libc::c_void;
            kh_resize_idx(h, size);
            j = 0 as libc::c_int as uint32_t;
            while j < size {
                let mut x_0: [uint64_t; 2] = [0; 2];
                let mut absent: libc::c_int = 0;
                fread(
                    x_0.as_mut_ptr() as *mut libc::c_void,
                    8 as libc::c_int as libc::c_ulong,
                    2 as libc::c_int as libc::c_ulong,
                    fp,
                );
                k = kh_put_idx(h, x_0[0 as libc::c_int as usize], &mut absent);
                if absent != 0 {} else {
                    __assert_fail(
                        b"absent\0" as *const u8 as *const libc::c_char,
                        b"index.c\0" as *const u8 as *const libc::c_char,
                        547 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 30],
                            &[libc::c_char; 30],
                        >(b"mm_idx_t *mm_idx_load(FILE *)\0"))
                            .as_ptr(),
                    );
                }
                'c_7966: {
                    if absent != 0 {} else {
                        __assert_fail(
                            b"absent\0" as *const u8 as *const libc::c_char,
                            b"index.c\0" as *const u8 as *const libc::c_char,
                            547 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 30],
                                &[libc::c_char; 30],
                            >(b"mm_idx_t *mm_idx_load(FILE *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                *((*h).vals).offset(k as isize) = x_0[1 as libc::c_int as usize];
                j = j.wrapping_add(1);
                j;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*mi).flag & 0x2 as libc::c_int == 0 {
        (*mi)
            .S = malloc(
            sum_len
                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
        ) as *mut uint32_t;
        fread(
            (*mi).S as *mut libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            sum_len
                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                .wrapping_div(8 as libc::c_int as libc::c_ulong),
            fp,
        );
    }
    return mi;
}
pub unsafe extern "C" fn mm_idx_is_idx(mut fn_0: *const libc::c_char) -> int64_t {
    let mut fd: libc::c_int = 0;
    let mut is_idx: libc::c_int = 0 as libc::c_int;
    let mut ret: int64_t = 0;
    let mut off_end: int64_t = 0;
    let mut magic: [libc::c_char; 4] = [0; 4];
    if strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0 as libc::c_int as int64_t;
    }
    fd = open(fn_0, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int) as int64_t;
    }
    off_end = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    if off_end >= 4 as libc::c_int as libc::c_long {
        lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
        ret = read(
            fd,
            magic.as_mut_ptr() as *mut libc::c_void,
            4 as libc::c_int as size_t,
        );
        if ret == 4 as libc::c_int as libc::c_long
            && strncmp(
                magic.as_mut_ptr(),
                b"MMI\x02\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            is_idx = 1 as libc::c_int;
        }
    }
    close(fd);
    return if is_idx != 0 { off_end } else { 0 as libc::c_int as libc::c_long };
}
pub unsafe extern "C" fn mm_idx_reader_open(
    mut fn_0: *const libc::c_char,
    mut opt: *const mm_idxopt_t,
    mut fn_out: *const libc::c_char,
) -> *mut mm_idx_reader_t {
    let mut is_idx: int64_t = 0;
    let mut r: *mut mm_idx_reader_t = 0 as *mut mm_idx_reader_t;
    is_idx = mm_idx_is_idx(fn_0);
    if is_idx < 0 as libc::c_int as libc::c_long {
        return 0 as *mut mm_idx_reader_t;
    }
    r = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mm_idx_reader_t>() as libc::c_ulong,
    ) as *mut mm_idx_reader_t;
    (*r).is_idx = is_idx as libc::c_int;
    if !opt.is_null() {
        (*r).opt = *opt;
    } else {
        mm_idxopt_init(&mut (*r).opt);
    }
    if (*r).is_idx != 0 {
        (*r).fp.idx = fopen(fn_0, b"rb\0" as *const u8 as *const libc::c_char);
        (*r).idx_size = is_idx;
    } else {
        (*r).fp.seq = mm_bseq_open(fn_0);
    }
    if !fn_out.is_null() {
        (*r).fp_out = fopen(fn_out, b"wb\0" as *const u8 as *const libc::c_char);
    }
    return r;
}
pub unsafe extern "C" fn mm_idx_reader_close(mut r: *mut mm_idx_reader_t) {
    if (*r).is_idx != 0 {
        fclose((*r).fp.idx);
    } else {
        mm_bseq_close((*r).fp.seq);
    }
    if !((*r).fp_out).is_null() {
        fclose((*r).fp_out);
    }
    free(r as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_idx_reader_read(
    mut r: *mut mm_idx_reader_t,
    mut n_threads: libc::c_int,
) -> *mut mm_idx_t {
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    if (*r).is_idx != 0 {
        mi = mm_idx_load((*r).fp.idx);
        if !mi.is_null() && mm_verbose >= 2 as libc::c_int
            && ((*mi).k != (*r).opt.k as libc::c_int
                || (*mi).w != (*r).opt.w as libc::c_int
                || (*mi).flag & 0x1 as libc::c_int
                    != (*r).opt.flag as libc::c_int & 0x1 as libc::c_int)
        {
            fprintf(
                stderr,
                b"[WARNING]\x1B[1;31m Indexing parameters (-k, -w or -H) overridden by parameters used in the prebuilt index.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else {
        mi = mm_idx_gen(
            (*r).fp.seq,
            (*r).opt.w as libc::c_int,
            (*r).opt.k as libc::c_int,
            (*r).opt.bucket_bits as libc::c_int,
            (*r).opt.flag as libc::c_int,
            (*r).opt.mini_batch_size as libc::c_int,
            n_threads,
            (*r).opt.batch_size,
        );
    }
    if !mi.is_null() {
        if !((*r).fp_out).is_null() {
            mm_idx_dump((*r).fp_out, mi);
        }
        let fresh20 = (*r).n_parts;
        (*r).n_parts = (*r).n_parts + 1;
        (*mi).index = fresh20;
    }
    return mi;
}
pub unsafe extern "C" fn mm_idx_reader_eof(
    mut r: *const mm_idx_reader_t,
) -> libc::c_int {
    return if (*r).is_idx != 0 {
        (feof((*r).fp.idx) != 0 || ftell((*r).fp.idx) == (*r).idx_size) as libc::c_int
    } else {
        mm_bseq_eof((*r).fp.seq)
    };
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
pub unsafe extern "C" fn mm_idx_alt_read(
    mut mi: *mut mm_idx_t,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut n_alt: libc::c_int = 0 as libc::c_int;
    let mut fp: gzFile = 0 as *mut gzFile_s;
    let mut ks: *mut kstream_t = 0 as *mut kstream_t;
    let mut str: kstring_t = {
        let mut init = __kstring_t {
            l: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            s: 0 as *mut libc::c_char,
        };
        init
    };
    fp = if !fn_0.is_null()
        && strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char) != 0
    {
        gzopen(fn_0, b"r\0" as *const u8 as *const libc::c_char)
    } else {
        gzdopen(fileno(stdin), b"r\0" as *const u8 as *const libc::c_char)
    };
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    ks = ks_init(fp);
    if ((*mi).h).is_null() {
        mm_idx_index_name(mi);
    }
    while ks_getuntil(ks, 2 as libc::c_int, &mut str, 0 as *mut libc::c_int)
        >= 0 as libc::c_int
    {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut id: libc::c_int = 0;
        p = str.s;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            p = p.offset(1);
            p;
        }
        *p = 0 as libc::c_int as libc::c_char;
        id = mm_idx_name2id(mi, str.s);
        if id >= 0 as libc::c_int {
            (*((*mi).seq).offset(id as isize)).is_alt = 1 as libc::c_int as uint32_t;
            n_alt += 1;
            n_alt;
        }
    }
    (*mi).n_alt = n_alt;
    if mm_verbose >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"[M::%s] found %d ALT contigs\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mm_idx_alt_read\0"))
                .as_ptr(),
            n_alt,
        );
    }
    return n_alt;
}
pub unsafe extern "C" fn rs_insertsort_bed(
    mut beg: *mut mm_idx_intv1_t,
    mut end: *mut mm_idx_intv1_t,
) {
    let mut i: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    i = beg.offset(1 as libc::c_int as isize);
    while i < end {
        if (*i).st < (*i.offset(-(1 as libc::c_int as isize))).st {
            let mut j: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
            let mut tmp: mm_idx_intv1_t = *i;
            j = i;
            while j > beg && tmp.st < (*j.offset(-(1 as libc::c_int as isize))).st {
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
                j;
            }
            *j = tmp;
        }
        i = i.offset(1);
        i;
    }
}
pub unsafe extern "C" fn radix_sort_bed(
    mut beg: *mut mm_idx_intv1_t,
    mut end: *mut mm_idx_intv1_t,
) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_int as libc::c_long {
        rs_insertsort_bed(beg, end);
    } else {
        rs_sort_bed(
            beg,
            end,
            8 as libc::c_int,
            (4 as libc::c_int - 1 as libc::c_int) * 8 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn rs_sort_bed(
    mut beg: *mut mm_idx_intv1_t,
    mut end: *mut mm_idx_intv1_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    let mut size: libc::c_int = (1 as libc::c_int) << n_bits;
    let mut m: libc::c_int = size - 1 as libc::c_int;
    let mut k: *mut rsbucket_bed_t = 0 as *mut rsbucket_bed_t;
    let mut b: [rsbucket_bed_t; 256] = [rsbucket_bed_t {
        b: 0 as *mut mm_idx_intv1_t,
        e: 0 as *mut mm_idx_intv1_t,
    }; 256];
    let mut be: *mut rsbucket_bed_t = b.as_mut_ptr().offset(size as isize);
    if n_bits <= 8 as libc::c_int {} else {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            660 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void rs_sort_bed(mm_idx_intv1_t *, mm_idx_intv1_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_12270: {
        if n_bits <= 8 as libc::c_int {} else {
            __assert_fail(
                b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
                b"index.c\0" as *const u8 as *const libc::c_char,
                660 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"void rs_sort_bed(mm_idx_intv1_t *, mm_idx_intv1_t *, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    k = b.as_mut_ptr();
    while k != be {
        (*k).e = beg;
        (*k).b = (*k).e;
        k = k.offset(1);
        k;
    }
    i = beg;
    while i != end {
        b[((*i).st >> s & m) as usize].e = (b[((*i).st >> s & m) as usize].e).offset(1);
        b[((*i).st >> s & m) as usize].e;
        i = i.offset(1);
        i;
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    k = b.as_mut_ptr();
    while k != be {
        if (*k).b != (*k).e {
            let mut l: *mut rsbucket_bed_t = 0 as *mut rsbucket_bed_t;
            l = b.as_mut_ptr().offset(((*(*k).b).st >> s & m) as isize);
            if l != k {
                let mut tmp: mm_idx_intv1_t = *(*k).b;
                let mut swap: mm_idx_intv1_t = mm_idx_intv1_t {
                    st: 0,
                    en: 0,
                    max: 0,
                    score_strand: [0; 4],
                };
                loop {
                    swap = tmp;
                    tmp = *(*l).b;
                    let fresh21 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *fresh21 = swap;
                    l = b.as_mut_ptr().offset((tmp.st >> s & m) as isize);
                    if !(l != k) {
                        break;
                    }
                }
                let fresh22 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *fresh22 = tmp;
            } else {
                (*k).b = ((*k).b).offset(1);
                (*k).b;
            }
        } else {
            k = k.offset(1);
            k;
        }
    }
    let ref mut fresh23 = (*b.as_mut_ptr()).b;
    *fresh23 = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    if s != 0 {
        s = if s > n_bits { s - n_bits } else { 0 as libc::c_int };
        k = b.as_mut_ptr();
        while k != be {
            if ((*k).e).offset_from((*k).b) as libc::c_long
                > 64 as libc::c_int as libc::c_long
            {
                rs_sort_bed((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long
                > 1 as libc::c_int as libc::c_long
            {
                rs_insertsort_bed((*k).b, (*k).e);
            }
            k = k.offset(1);
            k;
        }
    }
}
pub unsafe extern "C" fn mm_idx_read_bed(
    mut mi: *const mm_idx_t,
    mut fn_0: *const libc::c_char,
    mut read_junc: libc::c_int,
) -> *mut mm_idx_intv_t {
    let mut fp: gzFile = 0 as *mut gzFile_s;
    let mut ks: *mut kstream_t = 0 as *mut kstream_t;
    let mut str: kstring_t = {
        let mut init = __kstring_t {
            l: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            s: 0 as *mut libc::c_char,
        };
        init
    };
    let mut I: *mut mm_idx_intv_t = 0 as *mut mm_idx_intv_t;
    fp = if !fn_0.is_null()
        && strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char) != 0
    {
        gzopen(fn_0, b"r\0" as *const u8 as *const libc::c_char)
    } else {
        gzdopen(fileno(stdin), b"r\0" as *const u8 as *const libc::c_char)
    };
    if fp.is_null() {
        return 0 as *mut mm_idx_intv_t;
    }
    I = calloc(
        (*mi).n_seq as libc::c_ulong,
        ::std::mem::size_of::<mm_idx_intv_t>() as libc::c_ulong,
    ) as *mut mm_idx_intv_t;
    ks = ks_init(fp);
    while ks_getuntil(ks, 2 as libc::c_int, &mut str, 0 as *mut libc::c_int)
        >= 0 as libc::c_int
    {
        let mut r: *mut mm_idx_intv_t = 0 as *mut mm_idx_intv_t;
        let mut t: mm_idx_intv1_t = {
            let mut init = mm_idx_intv1_t {
                score_strand: [0; 4],
                st: -(1 as libc::c_int),
                en: -(1 as libc::c_int),
                max: -(1 as libc::c_int),
            };
            init.set_score(-(1 as libc::c_int));
            init.set_strand(0 as libc::c_int);
            init
        };
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut bl: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut bs: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: int32_t = 0;
        let mut id: int32_t = -(1 as libc::c_int);
        let mut n_blk: int32_t = 0 as libc::c_int;
        q = str.s;
        p = q;
        i = 0 as libc::c_int;
        loop {
            if *p as libc::c_int == 0 as libc::c_int || *p as libc::c_int == '\t' as i32
            {
                let mut c: int32_t = *p as int32_t;
                *p = 0 as libc::c_int as libc::c_char;
                if i == 0 as libc::c_int {
                    id = mm_idx_name2id(mi, q);
                    if id < 0 as libc::c_int {
                        break;
                    }
                } else if i == 1 as libc::c_int {
                    t.st = atol(q) as int32_t;
                    if t.st < 0 as libc::c_int {
                        break;
                    }
                } else if i == 2 as libc::c_int {
                    t.en = atol(q) as int32_t;
                    if t.en < 0 as libc::c_int {
                        break;
                    }
                } else if i == 4 as libc::c_int {
                    t.set_score(atol(q) as int32_t);
                } else if i == 5 as libc::c_int {
                    t.set_strand(
                        if *q as libc::c_int == '+' as i32 {
                            1 as libc::c_int
                        } else if *q as libc::c_int == '-' as i32 {
                            -(1 as libc::c_int)
                        } else {
                            0 as libc::c_int
                        },
                    );
                } else if i == 9 as libc::c_int {
                    if *(*__ctype_b_loc()).offset(*q as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        break;
                    }
                    n_blk = atol(q) as int32_t;
                } else if i == 10 as libc::c_int {
                    bl = q;
                } else if i == 11 as libc::c_int {
                    bs = q;
                    break;
                }
                if c == 0 as libc::c_int {
                    break;
                }
                i += 1;
                i;
                q = p.offset(1 as libc::c_int as isize);
            }
            p = p.offset(1);
            p;
        }
        if id < 0 as libc::c_int || t.st < 0 as libc::c_int || t.st >= t.en {
            continue;
        }
        r = &mut *I.offset(id as isize) as *mut mm_idx_intv_t;
        if i >= 11 as libc::c_int && read_junc != 0 {
            let mut st: int32_t = 0;
            let mut sz: int32_t = 0;
            let mut en: int32_t = 0;
            st = strtol(bs, &mut bs, 10 as libc::c_int) as int32_t;
            bs = bs.offset(1);
            bs;
            sz = strtol(bl, &mut bl, 10 as libc::c_int) as int32_t;
            bl = bl.offset(1);
            bl;
            en = t.st + st + sz;
            i = 1 as libc::c_int;
            while i < n_blk {
                let mut s: mm_idx_intv1_t = t;
                if (*r).n == (*r).m {
                    (*r)
                        .m = if (*r).m != 0 {
                        (*r).m + ((*r).m >> 1 as libc::c_int)
                    } else {
                        16 as libc::c_int
                    };
                    (*r)
                        .a = realloc(
                        (*r).a as *mut libc::c_void,
                        (::std::mem::size_of::<mm_idx_intv1_t>() as libc::c_ulong)
                            .wrapping_mul((*r).m as libc::c_ulong),
                    ) as *mut mm_idx_intv1_t;
                }
                st = strtol(bs, &mut bs, 10 as libc::c_int) as int32_t;
                bs = bs.offset(1);
                bs;
                sz = strtol(bl, &mut bl, 10 as libc::c_int) as int32_t;
                bl = bl.offset(1);
                bl;
                s.st = en;
                s.en = t.st + st;
                en = t.st + st + sz;
                if s.en > s.st {
                    let fresh24 = (*r).n;
                    (*r).n = (*r).n + 1;
                    *((*r).a).offset(fresh24 as isize) = s;
                }
                i += 1;
                i;
            }
        } else {
            if (*r).n == (*r).m {
                (*r)
                    .m = if (*r).m != 0 {
                    (*r).m + ((*r).m >> 1 as libc::c_int)
                } else {
                    16 as libc::c_int
                };
                (*r)
                    .a = realloc(
                    (*r).a as *mut libc::c_void,
                    (::std::mem::size_of::<mm_idx_intv1_t>() as libc::c_ulong)
                        .wrapping_mul((*r).m as libc::c_ulong),
                ) as *mut mm_idx_intv1_t;
            }
            let fresh25 = (*r).n;
            (*r).n = (*r).n + 1;
            *((*r).a).offset(fresh25 as isize) = t;
        }
    }
    free(str.s as *mut libc::c_void);
    ks_destroy(ks);
    gzclose(fp);
    return I;
}
pub unsafe extern "C" fn mm_idx_bed_read(
    mut mi: *mut mm_idx_t,
    mut fn_0: *const libc::c_char,
    mut read_junc: libc::c_int,
) -> libc::c_int {
    let mut i: int32_t = 0;
    if ((*mi).h).is_null() {
        mm_idx_index_name(mi);
    }
    (*mi).I = mm_idx_read_bed(mi, fn_0, read_junc);
    if ((*mi).I).is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*mi).n_seq {
        radix_sort_bed(
            (*((*mi).I).offset(i as isize)).a,
            ((*((*mi).I).offset(i as isize)).a)
                .offset((*((*mi).I).offset(i as isize)).n as isize),
        );
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_bed_junc(
    mut mi: *const mm_idx_t,
    mut ctg: int32_t,
    mut st: int32_t,
    mut en: int32_t,
    mut s: *mut uint8_t,
) -> libc::c_int {
    let mut i: int32_t = 0;
    let mut left: int32_t = 0;
    let mut right: int32_t = 0;
    let mut r: *mut mm_idx_intv_t = 0 as *mut mm_idx_intv_t;
    memset(s as *mut libc::c_void, 0 as libc::c_int, (en - st) as libc::c_ulong);
    if ((*mi).I).is_null() || ctg < 0 as libc::c_int
        || ctg as libc::c_uint >= (*mi).n_seq
    {
        return -(1 as libc::c_int);
    }
    r = &mut *((*mi).I).offset(ctg as isize) as *mut mm_idx_intv_s;
    left = 0 as libc::c_int;
    right = (*r).n;
    while right > left {
        let mut mid: int32_t = left + (right - left >> 1 as libc::c_int);
        if (*((*r).a).offset(mid as isize)).st >= st {
            right = mid;
        } else {
            left = mid + 1 as libc::c_int;
        }
    }
    i = left;
    while i < (*r).n {
        if st <= (*((*r).a).offset(i as isize)).st
            && en >= (*((*r).a).offset(i as isize)).en
            && (*((*r).a).offset(i as isize)).strand() != 0 as libc::c_int
        {
            if (*((*r).a).offset(i as isize)).strand() > 0 as libc::c_int {
                let ref mut fresh26 = *s
                    .offset(((*((*r).a).offset(i as isize)).st - st) as isize);
                *fresh26 = (*fresh26 as libc::c_int | 1 as libc::c_int) as uint8_t;
                let ref mut fresh27 = *s
                    .offset(
                        ((*((*r).a).offset(i as isize)).en - 1 as libc::c_int - st)
                            as isize,
                    );
                *fresh27 = (*fresh27 as libc::c_int | 2 as libc::c_int) as uint8_t;
            } else {
                let ref mut fresh28 = *s
                    .offset(((*((*r).a).offset(i as isize)).st - st) as isize);
                *fresh28 = (*fresh28 as libc::c_int | 8 as libc::c_int) as uint8_t;
                let ref mut fresh29 = *s
                    .offset(
                        ((*((*r).a).offset(i as isize)).en - 1 as libc::c_int - st)
                            as isize,
                    );
                *fresh29 = (*fresh29 as libc::c_int | 4 as libc::c_int) as uint8_t;
            }
        }
        i += 1;
        i;
    }
    return left;
}
