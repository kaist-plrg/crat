use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut mm_verbose: libc::c_int;
    fn mm_err_fread(p: *mut libc::c_void, size: size_t, nitems: size_t, fp: *mut FILE);
    fn mm_err_fwrite(
        p: *const libc::c_void,
        size: size_t,
        nitems: size_t,
        fp: *mut FILE,
    );
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub unsafe extern "C" fn mm_split_init(
    mut prefix: *const libc::c_char,
    mut mi: *const mm_idx_t,
) -> *mut FILE {
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: uint32_t = 0;
    let mut k: uint32_t = (*mi).k as uint32_t;
    fn_0 = calloc(
        (strlen(prefix)).wrapping_add(10 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    sprintf(
        fn_0,
        b"%s.%.4d.tmp\0" as *const u8 as *const libc::c_char,
        prefix,
        (*mi).index,
    );
    fp = fopen(fn_0, b"wb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m failed to write to temporary file '%s'\x1B[0m: %s\n\0"
                    as *const u8 as *const libc::c_char,
                fn_0,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    mm_err_fwrite(
        &mut k as *mut uint32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fp,
    );
    mm_err_fwrite(
        &(*mi).n_seq as *const uint32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fp,
    );
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        let mut l: uint32_t = 0;
        l = strlen((*((*mi).seq).offset(i as isize)).name) as uint32_t;
        mm_err_fwrite(
            &mut l as *mut uint32_t as *const libc::c_void,
            1 as libc::c_int as size_t,
            4 as libc::c_int as size_t,
            fp,
        );
        mm_err_fwrite(
            (*((*mi).seq).offset(i as isize)).name as *const libc::c_void,
            1 as libc::c_int as size_t,
            l as size_t,
            fp,
        );
        mm_err_fwrite(
            &mut (*((*mi).seq).offset(i as isize)).len as *mut uint32_t
                as *const libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        i = i.wrapping_add(1);
        i;
    }
    free(fn_0 as *mut libc::c_void);
    return fp;
}
pub unsafe extern "C" fn mm_split_merge_prep(
    mut prefix: *const libc::c_char,
    mut n_splits: libc::c_int,
    mut fp: *mut *mut FILE,
    mut n_seq_part: *mut uint32_t,
) -> *mut mm_idx_t {
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if n_splits < 1 as libc::c_int {
        return 0 as *mut mm_idx_t;
    }
    fn_0 = calloc(
        (strlen(prefix)).wrapping_add(10 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < n_splits {
        sprintf(fn_0, b"%s.%.4d.tmp\0" as *const u8 as *const libc::c_char, prefix, i);
        let ref mut fresh0 = *fp.offset(i as isize);
        *fresh0 = fopen(fn_0, b"rb\0" as *const u8 as *const libc::c_char);
        if (*fresh0).is_null() {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"ERROR: failed to open temporary file '%s': %s\n\0" as *const u8
                        as *const libc::c_char,
                    fn_0,
                    strerror(*__errno_location()),
                );
            }
            j = 0 as libc::c_int;
            while j < i {
                fclose(*fp.offset(j as isize));
                j += 1;
                j;
            }
            free(fn_0 as *mut libc::c_void);
            return 0 as *mut mm_idx_t;
        }
        i += 1;
        i;
    }
    free(fn_0 as *mut libc::c_void);
    mi = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mm_idx_t>() as libc::c_ulong,
    ) as *mut mm_idx_t;
    i = 0 as libc::c_int;
    while i < n_splits {
        mm_err_fread(
            &mut (*mi).k as *mut int32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            *fp.offset(i as isize),
        );
        mm_err_fread(
            &mut *n_seq_part.offset(i as isize) as *mut uint32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            *fp.offset(i as isize),
        );
        (*mi)
            .n_seq = ((*mi).n_seq as libc::c_uint)
            .wrapping_add(*n_seq_part.offset(i as isize)) as uint32_t as uint32_t;
        i += 1;
        i;
    }
    (*mi)
        .seq = calloc(
        (*mi).n_seq as libc::c_ulong,
        ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
    ) as *mut mm_idx_seq_t;
    j = 0 as libc::c_int;
    i = j;
    while i < n_splits {
        let mut k: uint32_t = 0;
        k = 0 as libc::c_int as uint32_t;
        while k < *n_seq_part.offset(i as isize) {
            let mut l: uint32_t = 0;
            mm_err_fread(
                &mut l as *mut uint32_t as *mut libc::c_void,
                1 as libc::c_int as size_t,
                4 as libc::c_int as size_t,
                *fp.offset(i as isize),
            );
            let ref mut fresh1 = (*((*mi).seq).offset(j as isize)).name;
            *fresh1 = calloc(
                l.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
            ) as *mut libc::c_char;
            mm_err_fread(
                (*((*mi).seq).offset(j as isize)).name as *mut libc::c_void,
                1 as libc::c_int as size_t,
                l as size_t,
                *fp.offset(i as isize),
            );
            mm_err_fread(
                &mut (*((*mi).seq).offset(j as isize)).len as *mut uint32_t
                    as *mut libc::c_void,
                4 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                *fp.offset(i as isize),
            );
            k = k.wrapping_add(1);
            k;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return mi;
}
pub unsafe extern "C" fn mm_split_rm_tmp(
    mut prefix: *const libc::c_char,
    mut n_splits: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    fn_0 = calloc(
        (strlen(prefix)).wrapping_add(10 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < n_splits {
        sprintf(fn_0, b"%s.%.4d.tmp\0" as *const u8 as *const libc::c_char, prefix, i);
        remove(fn_0);
        i += 1;
        i;
    }
    free(fn_0 as *mut libc::c_void);
}
