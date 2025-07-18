use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut libc::c_ulong,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut libc::c_long,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut libc::c_uchar,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type gsl_block_char = gsl_block_char_struct;
pub unsafe extern "C" fn gsl_block_long_double_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_long_double,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_complex,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_float,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_uint,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uint = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_short,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_short = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_uchar,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uchar = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_complex_float,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_long,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_long = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_ushort,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ushort = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_complex_long_double,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_int,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_int = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_char = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_fread(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_ulong,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ulong = (*b).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_uchar,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uchar = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_ulong,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ulong = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_int,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_int = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_float,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_complex,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_complex_long_double,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_ushort,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ushort = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_char = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_complex_float,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_uint,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uint = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_long,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_long = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_double_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_long_double,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block_short,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_short = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_fwrite(
    mut stream: *mut FILE,
    mut b: *const gsl_block,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        n,
        stream,
    );
    if items != n {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_uint,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_long,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_double_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut f128::f128,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_ushort,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_float,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_short,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_double,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_double,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_uchar,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_ulong,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_int,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut f128::f128,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_float,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_raw_fread(
    mut stream: *mut FILE,
    mut data: *mut libc::c_char,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fread(
            data as *mut libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fread failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fread(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *mut libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fread failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_long,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_uint,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_ulong,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_short,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_char,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_double_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const f128::f128,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_int,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_double,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_float,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_ushort,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_float,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_uchar,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const libc::c_double,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_raw_fwrite(
    mut stream: *mut FILE,
    mut data: *const f128::f128,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    if stride == 1 as libc::c_int as libc::c_ulong {
        let mut items: size_t = fwrite(
            data as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
            n,
            stream,
        );
        if items != n {
            gsl_error(
                b"fwrite failed\0" as *const u8 as *const libc::c_char,
                b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut item: size_t = fwrite(
                data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride) as isize,
                    ) as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
                1 as libc::c_int as libc::c_ulong,
                stream,
            );
            if item != 1 as libc::c_int as libc::c_ulong {
                gsl_error(
                    b"fwrite failed\0" as *const u8 as *const libc::c_char,
                    b"./fwrite_source.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_int,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_int = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_char,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_char = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_ushort,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ushort = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_complex,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_complex_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_double,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_long,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_long = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_uchar,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uchar = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_uint,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uint = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_short,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_short = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_complex_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_double,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_double_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_fprintf(
    mut stream: *mut FILE,
    mut b: *const gsl_block_ulong,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ulong = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        44 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_long,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_long = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_long = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%ld\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_long,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_double = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_double,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_double_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_long_double,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: f128::f128 = f128::f128::ZERO;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%Lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut f128::f128,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_float,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_float = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%g\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_float,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_int,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_int = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_int = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_int,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_uchar,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uchar = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_uint = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%u\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_uint,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp as libc::c_uchar;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_ulong,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ulong = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_ulong = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%lu\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_ulong,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_complex,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_double = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            let mut tmp: libc::c_double = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_double,
            );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_complex_long_double,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut f128::f128 = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            let mut tmp: f128::f128 = f128::f128::ZERO;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%Lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut f128::f128,
            );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_ushort,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_ushort = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_ushort = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%hu\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_ushort,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_complex_float,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_float = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            let mut tmp: libc::c_float = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%g\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_float,
            );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_uint,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_uint = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_uint = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%u\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_uint,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_short,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_short = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_short = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%hd\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_short,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_fscanf(
    mut stream: *mut FILE,
    mut b: *mut gsl_block_char,
) -> libc::c_int {
    let mut n: size_t = (*b).size;
    let mut data: *mut libc::c_char = (*b).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_int = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_int,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp as libc::c_char;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    90 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_double,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_char,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_uchar,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_double_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const f128::f128,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_int,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_float,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_double,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_long,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_float,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_double,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_uint,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_ushort,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const f128::f128,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_ulong,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_double,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ),
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_raw_fprintf(
    mut stream: *mut FILE,
    mut data: *const libc::c_short,
    n: size_t,
    stride: size_t,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            if k > 0 as libc::c_int {
                status = putc(' ' as i32, stream);
                if status == -(1 as libc::c_int) {
                    gsl_error(
                        b"putc failed\0" as *const u8 as *const libc::c_char,
                        b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                        121 as libc::c_int,
                        GSL_EFAILED as libc::c_int,
                    );
                    return GSL_EFAILED as libc::c_int;
                }
            }
            status = fprintf(
                stream,
                format,
                *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride)
                            .wrapping_add(k as libc::c_ulong) as isize,
                    ) as libc::c_int,
            );
            if status < 0 as libc::c_int {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    129 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        status = putc('\n' as i32, stream);
        if status == -(1 as libc::c_int) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const libc::c_char,
                b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uint_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_uint,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_uint = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%u\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_uint,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_double,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_double = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_double,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut f128::f128,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            let mut tmp: f128::f128 = f128::f128::ZERO;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%Lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut f128::f128,
            );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_float_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_float,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_float = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%g\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_float,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_long,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_long = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%ld\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_long,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_int_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_int,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_int = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_int,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ushort_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_ushort,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_ushort = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%hu\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_ushort,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_ulong_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_ulong,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_ulong = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%lu\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_ulong,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_long_double_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut f128::f128,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: f128::f128 = f128::f128::ZERO;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%Lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut f128::f128,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_char_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_char,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_int = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_int,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp as libc::c_char;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_double,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            let mut tmp: libc::c_double = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_double,
            );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_uchar_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_uchar,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_uint = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%u\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_uint,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp as libc::c_uchar;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_short_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_short,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 1 as libc::c_int {
            let mut tmp: libc::c_short = 0;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%hd\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_short,
            );
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_block_complex_float_raw_fscanf(
    mut stream: *mut FILE,
    mut data: *mut libc::c_float,
    n: size_t,
    stride: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            let mut tmp: libc::c_float = 0.;
            let mut status: libc::c_int = fscanf(
                stream,
                b"%g\0" as *const u8 as *const libc::c_char,
                &mut tmp as *mut libc::c_float,
            );
            *data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride)
                        .wrapping_add(k as libc::c_ulong) as isize,
                ) = tmp;
            if status != 1 as libc::c_int {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const libc::c_char,
                    b"./fprintf_source.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            k += 1;
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
