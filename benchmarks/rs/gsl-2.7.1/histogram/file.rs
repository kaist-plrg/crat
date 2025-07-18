use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_block_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram_fread(
    mut stream: *mut FILE,
    mut h: *mut gsl_histogram,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_raw_fread(
        stream,
        (*h).range,
        ((*h).n).wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as size_t,
    );
    if status != 0 {
        return status;
    }
    status = gsl_block_raw_fread(stream, (*h).bin, (*h).n, 1 as libc::c_int as size_t);
    return status;
}
pub unsafe extern "C" fn gsl_histogram_fwrite(
    mut stream: *mut FILE,
    mut h: *const gsl_histogram,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_raw_fwrite(
        stream,
        (*h).range,
        ((*h).n).wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as size_t,
    );
    if status != 0 {
        return status;
    }
    status = gsl_block_raw_fwrite(stream, (*h).bin, (*h).n, 1 as libc::c_int as size_t);
    return status;
}
pub unsafe extern "C" fn gsl_histogram_fprintf(
    mut stream: *mut FILE,
    mut h: *const gsl_histogram,
    mut range_format: *const libc::c_char,
    mut bin_format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    let n: size_t = (*h).n;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut status: libc::c_int = fprintf(
            stream,
            range_format,
            *((*h).range).offset(i as isize),
        );
        if status < 0 as libc::c_int {
            gsl_error(
                b"fprintf failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        status = putc(' ' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        status = fprintf(
            stream,
            range_format,
            *((*h).range)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
        );
        if status < 0 as libc::c_int {
            gsl_error(
                b"fprintf failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        status = putc(' ' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        status = fprintf(stream, bin_format, *((*h).bin).offset(i as isize));
        if status < 0 as libc::c_int {
            gsl_error(
                b"fprintf failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_fscanf(
    mut stream: *mut FILE,
    mut h: *mut gsl_histogram,
) -> libc::c_int {
    let mut i: size_t = 0;
    let n: size_t = (*h).n;
    let mut upper: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut status: libc::c_int = fscanf(
            stream,
            b"%lg %lg %lg\0" as *const u8 as *const libc::c_char,
            ((*h).range).offset(i as isize),
            &mut upper as *mut libc::c_double,
            ((*h).bin).offset(i as isize),
        );
        if status != 3 as libc::c_int {
            gsl_error(
                b"fscanf failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    *((*h).range).offset(n as isize) = upper;
    return GSL_SUCCESS as libc::c_int;
}
