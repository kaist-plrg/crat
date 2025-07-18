use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_histogram_increment(h: *mut gsl_histogram, x: libc::c_double) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ntuple {
    pub file: *mut FILE,
    pub ntuple_data: *mut libc::c_void,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ntuple_select_fn {
    pub function: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ntuple_value_fn {
    pub function: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub unsafe extern "C" fn gsl_ntuple_create(
    mut filename: *mut libc::c_char,
    mut ntuple_data: *mut libc::c_void,
    mut size: size_t,
) -> *mut gsl_ntuple {
    let mut ntuple: *mut gsl_ntuple = malloc(
        ::std::mem::size_of::<gsl_ntuple>() as libc::c_ulong,
    ) as *mut gsl_ntuple;
    if ntuple.is_null() {
        gsl_error(
            b"failed to allocate space for ntuple struct\0" as *const u8
                as *const libc::c_char,
            b"ntuple.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_ntuple;
    }
    (*ntuple).ntuple_data = ntuple_data;
    (*ntuple).size = size;
    (*ntuple).file = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    if ((*ntuple).file).is_null() {
        free(ntuple as *mut libc::c_void);
        gsl_error(
            b"unable to create ntuple file\0" as *const u8 as *const libc::c_char,
            b"ntuple.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_ntuple;
    }
    return ntuple;
}
pub unsafe extern "C" fn gsl_ntuple_open(
    mut filename: *mut libc::c_char,
    mut ntuple_data: *mut libc::c_void,
    mut size: size_t,
) -> *mut gsl_ntuple {
    let mut ntuple: *mut gsl_ntuple = malloc(
        ::std::mem::size_of::<gsl_ntuple>() as libc::c_ulong,
    ) as *mut gsl_ntuple;
    if ntuple.is_null() {
        gsl_error(
            b"failed to allocate space for ntuple struct\0" as *const u8
                as *const libc::c_char,
            b"ntuple.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_ntuple;
    }
    (*ntuple).ntuple_data = ntuple_data;
    (*ntuple).size = size;
    (*ntuple).file = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if ((*ntuple).file).is_null() {
        free(ntuple as *mut libc::c_void);
        gsl_error(
            b"unable to open ntuple file for reading\0" as *const u8
                as *const libc::c_char,
            b"ntuple.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return 0 as *mut gsl_ntuple;
    }
    return ntuple;
}
pub unsafe extern "C" fn gsl_ntuple_write(mut ntuple: *mut gsl_ntuple) -> libc::c_int {
    let mut nwrite: size_t = 0;
    nwrite = fwrite(
        (*ntuple).ntuple_data,
        (*ntuple).size,
        1 as libc::c_int as libc::c_ulong,
        (*ntuple).file,
    );
    if nwrite != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"failed to write ntuple entry to file\0" as *const u8
                as *const libc::c_char,
            b"ntuple.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_ntuple_bookdata(
    mut ntuple: *mut gsl_ntuple,
) -> libc::c_int {
    return gsl_ntuple_write(ntuple);
}
pub unsafe extern "C" fn gsl_ntuple_read(mut ntuple: *mut gsl_ntuple) -> libc::c_int {
    let mut nread: size_t = 0;
    nread = fread(
        (*ntuple).ntuple_data,
        (*ntuple).size,
        1 as libc::c_int as libc::c_ulong,
        (*ntuple).file,
    );
    if nread == 0 as libc::c_int as libc::c_ulong && feof((*ntuple).file) != 0 {
        return GSL_EOF as libc::c_int;
    }
    if nread != 1 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"failed to read ntuple entry from file\0" as *const u8
                as *const libc::c_char,
            b"ntuple.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_ntuple_project(
    mut h: *mut gsl_histogram,
    mut ntuple: *mut gsl_ntuple,
    mut value_func: *mut gsl_ntuple_value_fn,
    mut select_func: *mut gsl_ntuple_select_fn,
) -> libc::c_int {
    let mut nread: size_t = 0;
    loop {
        nread = fread(
            (*ntuple).ntuple_data,
            (*ntuple).size,
            1 as libc::c_int as libc::c_ulong,
            (*ntuple).file,
        );
        if nread == 0 as libc::c_int as libc::c_ulong && feof((*ntuple).file) != 0 {
            break;
        }
        if nread != 1 as libc::c_int as libc::c_ulong {
            gsl_error(
                b"failed to read ntuple for projection\0" as *const u8
                    as *const libc::c_char,
                b"ntuple.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        if (Some(((*select_func).function).unwrap()))
            .unwrap()((*ntuple).ntuple_data, (*select_func).params) != 0
        {
            gsl_histogram_increment(
                h,
                (Some(((*value_func).function).unwrap()))
                    .unwrap()((*ntuple).ntuple_data, (*value_func).params),
            );
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_ntuple_close(mut ntuple: *mut gsl_ntuple) -> libc::c_int {
    let mut status: libc::c_int = fclose((*ntuple).file);
    if status != 0 {
        gsl_error(
            b"failed to close ntuple file\0" as *const u8 as *const libc::c_char,
            b"ntuple.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    free(ntuple as *mut libc::c_void);
    return GSL_SUCCESS as libc::c_int;
}
