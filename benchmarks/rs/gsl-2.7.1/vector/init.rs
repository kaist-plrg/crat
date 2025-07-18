use ::libc;
use ::f128;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gsl_block_float_alloc(n: size_t) -> *mut gsl_block_float;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_block_long_double_alloc(n: size_t) -> *mut gsl_block_long_double;
    fn gsl_block_long_double_free(b: *mut gsl_block_long_double);
    fn gsl_block_complex_long_double_alloc(
        n: size_t,
    ) -> *mut gsl_block_complex_long_double;
    fn gsl_block_complex_long_double_free(b: *mut gsl_block_complex_long_double);
    fn gsl_block_alloc(n: size_t) -> *mut gsl_block;
    fn gsl_block_free(b: *mut gsl_block);
    fn gsl_block_complex_alloc(n: size_t) -> *mut gsl_block_complex;
    fn gsl_block_complex_free(b: *mut gsl_block_complex);
    fn gsl_block_float_free(b: *mut gsl_block_float);
    fn gsl_block_complex_float_alloc(n: size_t) -> *mut gsl_block_complex_float;
    fn gsl_block_complex_float_free(b: *mut gsl_block_complex_float);
    fn gsl_block_ulong_alloc(n: size_t) -> *mut gsl_block_ulong;
    fn gsl_block_ulong_free(b: *mut gsl_block_ulong);
    fn gsl_block_long_alloc(n: size_t) -> *mut gsl_block_long;
    fn gsl_block_long_free(b: *mut gsl_block_long);
    fn gsl_block_uint_alloc(n: size_t) -> *mut gsl_block_uint;
    fn gsl_block_uint_free(b: *mut gsl_block_uint);
    fn gsl_block_int_alloc(n: size_t) -> *mut gsl_block_int;
    fn gsl_block_int_free(b: *mut gsl_block_int);
    fn gsl_block_ushort_alloc(n: size_t) -> *mut gsl_block_ushort;
    fn gsl_block_ushort_free(b: *mut gsl_block_ushort);
    fn gsl_block_short_alloc(n: size_t) -> *mut gsl_block_short;
    fn gsl_block_short_free(b: *mut gsl_block_short);
    fn gsl_block_uchar_alloc(n: size_t) -> *mut gsl_block_uchar;
    fn gsl_block_uchar_free(b: *mut gsl_block_uchar);
    fn gsl_block_char_free(b: *mut gsl_block_char);
    fn gsl_block_char_alloc(n: size_t) -> *mut gsl_block_char;
}
pub type size_t = libc::c_ulong;
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
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_long_double {
    pub dat: [f128::f128; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_float {
    pub dat: [libc::c_float; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut libc::c_ulong,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ulong {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut libc::c_long,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_long,
    pub block: *mut gsl_block_long,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_int {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ushort {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_short {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut libc::c_uchar,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uchar {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type gsl_block_char = gsl_block_char_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_char {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
pub unsafe extern "C" fn gsl_vector_ushort_alloc(n: size_t) -> *mut gsl_vector_ushort {
    let mut block: *mut gsl_block_ushort = 0 as *mut gsl_block_ushort;
    let mut v: *mut gsl_vector_ushort = 0 as *mut gsl_vector_ushort;
    v = malloc(::std::mem::size_of::<gsl_vector_ushort>() as libc::c_ulong)
        as *mut gsl_vector_ushort;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    block = gsl_block_ushort_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_float_alloc(
    n: size_t,
) -> *mut gsl_vector_complex_float {
    let mut block: *mut gsl_block_complex_float = 0 as *mut gsl_block_complex_float;
    let mut v: *mut gsl_vector_complex_float = 0 as *mut gsl_vector_complex_float;
    v = malloc(::std::mem::size_of::<gsl_vector_complex_float>() as libc::c_ulong)
        as *mut gsl_vector_complex_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    block = gsl_block_complex_float_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_alloc(
    n: size_t,
) -> *mut gsl_vector_complex_long_double {
    let mut block: *mut gsl_block_complex_long_double = 0
        as *mut gsl_block_complex_long_double;
    let mut v: *mut gsl_vector_complex_long_double = 0
        as *mut gsl_vector_complex_long_double;
    v = malloc(::std::mem::size_of::<gsl_vector_complex_long_double>() as libc::c_ulong)
        as *mut gsl_vector_complex_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    block = gsl_block_complex_long_double_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_char_alloc(n: size_t) -> *mut gsl_vector_char {
    let mut block: *mut gsl_block_char = 0 as *mut gsl_block_char;
    let mut v: *mut gsl_vector_char = 0 as *mut gsl_vector_char;
    v = malloc(::std::mem::size_of::<gsl_vector_char>() as libc::c_ulong)
        as *mut gsl_vector_char;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    block = gsl_block_char_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_alloc(n: size_t) -> *mut gsl_vector_long {
    let mut block: *mut gsl_block_long = 0 as *mut gsl_block_long;
    let mut v: *mut gsl_vector_long = 0 as *mut gsl_vector_long;
    v = malloc(::std::mem::size_of::<gsl_vector_long>() as libc::c_ulong)
        as *mut gsl_vector_long;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    block = gsl_block_long_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_float_alloc(n: size_t) -> *mut gsl_vector_float {
    let mut block: *mut gsl_block_float = 0 as *mut gsl_block_float;
    let mut v: *mut gsl_vector_float = 0 as *mut gsl_vector_float;
    v = malloc(::std::mem::size_of::<gsl_vector_float>() as libc::c_ulong)
        as *mut gsl_vector_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    block = gsl_block_float_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_uchar_alloc(n: size_t) -> *mut gsl_vector_uchar {
    let mut block: *mut gsl_block_uchar = 0 as *mut gsl_block_uchar;
    let mut v: *mut gsl_vector_uchar = 0 as *mut gsl_vector_uchar;
    v = malloc(::std::mem::size_of::<gsl_vector_uchar>() as libc::c_ulong)
        as *mut gsl_vector_uchar;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    block = gsl_block_uchar_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_ulong_alloc(n: size_t) -> *mut gsl_vector_ulong {
    let mut block: *mut gsl_block_ulong = 0 as *mut gsl_block_ulong;
    let mut v: *mut gsl_vector_ulong = 0 as *mut gsl_vector_ulong;
    v = malloc(::std::mem::size_of::<gsl_vector_ulong>() as libc::c_ulong)
        as *mut gsl_vector_ulong;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    block = gsl_block_ulong_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_double_alloc(
    n: size_t,
) -> *mut gsl_vector_long_double {
    let mut block: *mut gsl_block_long_double = 0 as *mut gsl_block_long_double;
    let mut v: *mut gsl_vector_long_double = 0 as *mut gsl_vector_long_double;
    v = malloc(::std::mem::size_of::<gsl_vector_long_double>() as libc::c_ulong)
        as *mut gsl_vector_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    block = gsl_block_long_double_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_uint_alloc(n: size_t) -> *mut gsl_vector_uint {
    let mut block: *mut gsl_block_uint = 0 as *mut gsl_block_uint;
    let mut v: *mut gsl_vector_uint = 0 as *mut gsl_vector_uint;
    v = malloc(::std::mem::size_of::<gsl_vector_uint>() as libc::c_ulong)
        as *mut gsl_vector_uint;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    block = gsl_block_uint_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_short_alloc(n: size_t) -> *mut gsl_vector_short {
    let mut block: *mut gsl_block_short = 0 as *mut gsl_block_short;
    let mut v: *mut gsl_vector_short = 0 as *mut gsl_vector_short;
    v = malloc(::std::mem::size_of::<gsl_vector_short>() as libc::c_ulong)
        as *mut gsl_vector_short;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    block = gsl_block_short_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_int_alloc(n: size_t) -> *mut gsl_vector_int {
    let mut block: *mut gsl_block_int = 0 as *mut gsl_block_int;
    let mut v: *mut gsl_vector_int = 0 as *mut gsl_vector_int;
    v = malloc(::std::mem::size_of::<gsl_vector_int>() as libc::c_ulong)
        as *mut gsl_vector_int;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    block = gsl_block_int_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_alloc(n: size_t) -> *mut gsl_vector_complex {
    let mut block: *mut gsl_block_complex = 0 as *mut gsl_block_complex;
    let mut v: *mut gsl_vector_complex = 0 as *mut gsl_vector_complex;
    v = malloc(::std::mem::size_of::<gsl_vector_complex>() as libc::c_ulong)
        as *mut gsl_vector_complex;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    block = gsl_block_complex_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector {
    let mut block: *mut gsl_block = 0 as *mut gsl_block;
    let mut v: *mut gsl_vector = 0 as *mut gsl_vector;
    v = malloc(::std::mem::size_of::<gsl_vector>() as libc::c_ulong) as *mut gsl_vector;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    block = gsl_block_alloc(n);
    if block.is_null() {
        free(v as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    (*v).data = (*block).data;
    (*v).size = n;
    (*v).stride = 1 as libc::c_int as size_t;
    (*v).block = block;
    (*v).owner = 1 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_ushort_calloc(n: size_t) -> *mut gsl_vector_ushort {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_ushort = gsl_vector_ushort_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_ushort;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_ulong_calloc(n: size_t) -> *mut gsl_vector_ulong {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_ulong = gsl_vector_ulong_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_ulong;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_int_calloc(n: size_t) -> *mut gsl_vector_int {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_int = gsl_vector_int_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_int;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_calloc(
    n: size_t,
) -> *mut gsl_vector_complex {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_complex = gsl_vector_complex_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_complex;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_float_calloc(
    n: size_t,
) -> *mut gsl_vector_complex_float {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_complex_float = gsl_vector_complex_float_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_complex_float;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_char_calloc(n: size_t) -> *mut gsl_vector_char {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_char = gsl_vector_char_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_char;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_uchar_calloc(n: size_t) -> *mut gsl_vector_uchar {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_uchar = gsl_vector_uchar_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_uchar;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_short_calloc(n: size_t) -> *mut gsl_vector_short {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_short = gsl_vector_short_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_short;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_uint_calloc(n: size_t) -> *mut gsl_vector_uint {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_uint = gsl_vector_uint_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_uint;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_calloc(
    n: size_t,
) -> *mut gsl_vector_complex_long_double {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_complex_long_double = gsl_vector_complex_long_double_alloc(
        n,
    );
    if v.is_null() {
        return 0 as *mut gsl_vector_complex_long_double;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = f128::f128::new(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_float_calloc(n: size_t) -> *mut gsl_vector_float {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_float = gsl_vector_float_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_float;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_calloc(n: size_t) -> *mut gsl_vector_long {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_long = gsl_vector_long_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_long;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_long;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_double_calloc(
    n: size_t,
) -> *mut gsl_vector_long_double {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector_long_double = gsl_vector_long_double_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector_long_double;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = f128::f128::new(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector {
    let mut i: size_t = 0;
    let mut v: *mut gsl_vector = gsl_vector_alloc(n);
    if v.is_null() {
        return 0 as *mut gsl_vector;
    }
    memset(
        (*v).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*v).data).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_alloc_from_block(
    mut block: *mut gsl_block_long,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_long {
    let mut v: *mut gsl_vector_long = 0 as *mut gsl_vector_long;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_long>() as libc::c_ulong)
        as *mut gsl_vector_long;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_short_alloc_from_block(
    mut block: *mut gsl_block_short,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_short {
    let mut v: *mut gsl_vector_short = 0 as *mut gsl_vector_short;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_short>() as libc::c_ulong)
        as *mut gsl_vector_short;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_alloc_from_block(
    mut block: *mut gsl_block_complex_long_double,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_complex_long_double {
    let mut v: *mut gsl_vector_complex_long_double = 0
        as *mut gsl_vector_complex_long_double;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_complex_long_double>() as libc::c_ulong)
        as *mut gsl_vector_complex_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    (*v)
        .data = ((*block).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_char_alloc_from_block(
    mut block: *mut gsl_block_char,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_char {
    let mut v: *mut gsl_vector_char = 0 as *mut gsl_vector_char;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_char>() as libc::c_ulong)
        as *mut gsl_vector_char;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_uint_alloc_from_block(
    mut block: *mut gsl_block_uint,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_uint {
    let mut v: *mut gsl_vector_uint = 0 as *mut gsl_vector_uint;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_uint>() as libc::c_ulong)
        as *mut gsl_vector_uint;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_ushort_alloc_from_block(
    mut block: *mut gsl_block_ushort,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_ushort {
    let mut v: *mut gsl_vector_ushort = 0 as *mut gsl_vector_ushort;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_ushort>() as libc::c_ulong)
        as *mut gsl_vector_ushort;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_uchar_alloc_from_block(
    mut block: *mut gsl_block_uchar,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_uchar {
    let mut v: *mut gsl_vector_uchar = 0 as *mut gsl_vector_uchar;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_uchar>() as libc::c_ulong)
        as *mut gsl_vector_uchar;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_double_alloc_from_block(
    mut block: *mut gsl_block_long_double,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_long_double {
    let mut v: *mut gsl_vector_long_double = 0 as *mut gsl_vector_long_double;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_long_double>() as libc::c_ulong)
        as *mut gsl_vector_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_ulong_alloc_from_block(
    mut block: *mut gsl_block_ulong,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_ulong {
    let mut v: *mut gsl_vector_ulong = 0 as *mut gsl_vector_ulong;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_ulong>() as libc::c_ulong)
        as *mut gsl_vector_ulong;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_alloc_from_block(
    mut block: *mut gsl_block_complex,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_complex {
    let mut v: *mut gsl_vector_complex = 0 as *mut gsl_vector_complex;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_complex>() as libc::c_ulong)
        as *mut gsl_vector_complex;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    (*v)
        .data = ((*block).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_int_alloc_from_block(
    mut block: *mut gsl_block_int,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_int {
    let mut v: *mut gsl_vector_int = 0 as *mut gsl_vector_int;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_int>() as libc::c_ulong)
        as *mut gsl_vector_int;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_float_alloc_from_block(
    mut block: *mut gsl_block_float,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_float {
    let mut v: *mut gsl_vector_float = 0 as *mut gsl_vector_float;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_float>() as libc::c_ulong)
        as *mut gsl_vector_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_float_alloc_from_block(
    mut block: *mut gsl_block_complex_float,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_complex_float {
    let mut v: *mut gsl_vector_complex_float = 0 as *mut gsl_vector_complex_float;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_complex_float>() as libc::c_ulong)
        as *mut gsl_vector_complex_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    (*v)
        .data = ((*block).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_alloc_from_block(
    mut block: *mut gsl_block,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector {
    let mut v: *mut gsl_vector = 0 as *mut gsl_vector;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    if (*block).size
        <= offset
            .wrapping_add(
                (if n > 0 as libc::c_int as libc::c_ulong {
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_mul(stride),
            )
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    v = malloc(::std::mem::size_of::<gsl_vector>() as libc::c_ulong) as *mut gsl_vector;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    (*v)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*v).size = n;
    (*v).stride = stride;
    (*v).block = block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_float_alloc_from_vector(
    mut w: *mut gsl_vector_float,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_float {
    let mut v: *mut gsl_vector_float = 0 as *mut gsl_vector_float;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_float>() as libc::c_ulong)
        as *mut gsl_vector_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_float;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_uint_alloc_from_vector(
    mut w: *mut gsl_vector_uint,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_uint {
    let mut v: *mut gsl_vector_uint = 0 as *mut gsl_vector_uint;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_uint>() as libc::c_ulong)
        as *mut gsl_vector_uint;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uint;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_ushort_alloc_from_vector(
    mut w: *mut gsl_vector_ushort,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_ushort {
    let mut v: *mut gsl_vector_ushort = 0 as *mut gsl_vector_ushort;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_ushort>() as libc::c_ulong)
        as *mut gsl_vector_ushort;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ushort;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_alloc_from_vector(
    mut w: *mut gsl_vector_complex,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_complex {
    let mut v: *mut gsl_vector_complex = 0 as *mut gsl_vector_complex;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_complex>() as libc::c_ulong)
        as *mut gsl_vector_complex;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_ulong_alloc_from_vector(
    mut w: *mut gsl_vector_ulong,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_ulong {
    let mut v: *mut gsl_vector_ulong = 0 as *mut gsl_vector_ulong;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_ulong>() as libc::c_ulong)
        as *mut gsl_vector_ulong;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_ulong;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_alloc_from_vector(
    mut w: *mut gsl_vector_complex_long_double,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_complex_long_double {
    let mut v: *mut gsl_vector_complex_long_double = 0
        as *mut gsl_vector_complex_long_double;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_complex_long_double>() as libc::c_ulong)
        as *mut gsl_vector_complex_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_long_double;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_int_alloc_from_vector(
    mut w: *mut gsl_vector_int,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_int {
    let mut v: *mut gsl_vector_int = 0 as *mut gsl_vector_int;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_int>() as libc::c_ulong)
        as *mut gsl_vector_int;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_int;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_char_alloc_from_vector(
    mut w: *mut gsl_vector_char,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_char {
    let mut v: *mut gsl_vector_char = 0 as *mut gsl_vector_char;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_char>() as libc::c_ulong)
        as *mut gsl_vector_char;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_char;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_uchar_alloc_from_vector(
    mut w: *mut gsl_vector_uchar,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_uchar {
    let mut v: *mut gsl_vector_uchar = 0 as *mut gsl_vector_uchar;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_uchar>() as libc::c_ulong)
        as *mut gsl_vector_uchar;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_uchar;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_short_alloc_from_vector(
    mut w: *mut gsl_vector_short,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_short {
    let mut v: *mut gsl_vector_short = 0 as *mut gsl_vector_short;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_short>() as libc::c_ulong)
        as *mut gsl_vector_short;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_short;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_complex_float_alloc_from_vector(
    mut w: *mut gsl_vector_complex_float,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_complex_float {
    let mut v: *mut gsl_vector_complex_float = 0 as *mut gsl_vector_complex_float;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_complex_float>() as libc::c_ulong)
        as *mut gsl_vector_complex_float;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_complex_float;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_double_alloc_from_vector(
    mut w: *mut gsl_vector_long_double,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_long_double {
    let mut v: *mut gsl_vector_long_double = 0 as *mut gsl_vector_long_double;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_long_double>() as libc::c_ulong)
        as *mut gsl_vector_long_double;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long_double;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_long_alloc_from_vector(
    mut w: *mut gsl_vector_long,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector_long {
    let mut v: *mut gsl_vector_long = 0 as *mut gsl_vector_long;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    v = malloc(::std::mem::size_of::<gsl_vector_long>() as libc::c_ulong)
        as *mut gsl_vector_long;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector_long;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_alloc_from_vector(
    mut w: *mut gsl_vector,
    offset: size_t,
    n: size_t,
    stride: size_t,
) -> *mut gsl_vector {
    let mut v: *mut gsl_vector = 0 as *mut gsl_vector;
    if stride == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"stride must be positive integer\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    if offset
        .wrapping_add(
            (if n > 0 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_mul(stride),
        ) >= (*w).size
    {
        gsl_error(
            b"vector would extend past end of block\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    v = malloc(::std::mem::size_of::<gsl_vector>() as libc::c_ulong) as *mut gsl_vector;
    if v.is_null() {
        gsl_error(
            b"failed to allocate space for vector struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_vector;
    }
    (*v)
        .data = ((*w).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*w).stride)
                .wrapping_mul(offset) as isize,
        );
    (*v).size = n;
    (*v).stride = stride.wrapping_mul((*w).stride);
    (*v).block = (*w).block;
    (*v).owner = 0 as libc::c_int;
    return v;
}
pub unsafe extern "C" fn gsl_vector_uchar_free(mut v: *mut gsl_vector_uchar) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_uchar_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_int_free(mut v: *mut gsl_vector_int) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_int_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_ushort_free(mut v: *mut gsl_vector_ushort) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_ushort_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_free(mut v: *mut gsl_vector) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_char_free(mut v: *mut gsl_vector_char) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_char_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_short_free(mut v: *mut gsl_vector_short) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_short_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_float_free(mut v: *mut gsl_vector_float) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_float_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_ulong_free(mut v: *mut gsl_vector_ulong) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_ulong_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_uint_free(mut v: *mut gsl_vector_uint) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_uint_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_complex_float_free(
    mut v: *mut gsl_vector_complex_float,
) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_complex_float_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_long_free(mut v: *mut gsl_vector_long) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_long_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_complex_free(mut v: *mut gsl_vector_complex) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_complex_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_long_double_free(
    mut v: *mut gsl_vector_long_double,
) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_long_double_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_free(
    mut v: *mut gsl_vector_complex_long_double,
) {
    if v.is_null() {
        return;
    }
    if (*v).owner != 0 {
        gsl_block_complex_long_double_free((*v).block);
    }
    free(v as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_vector_ushort_set_all(
    mut v: *mut gsl_vector_ushort,
    mut x: libc::c_ushort,
) {
    let data: *mut libc::c_ushort = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_long_double_set_all(
    mut v: *mut gsl_vector_long_double,
    mut x: f128::f128,
) {
    let data: *mut f128::f128 = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_long_set_all(
    mut v: *mut gsl_vector_long,
    mut x: libc::c_long,
) {
    let data: *mut libc::c_long = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_uint_set_all(
    mut v: *mut gsl_vector_uint,
    mut x: libc::c_uint,
) {
    let data: *mut libc::c_uint = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_ulong_set_all(
    mut v: *mut gsl_vector_ulong,
    mut x: libc::c_ulong,
) {
    let data: *mut libc::c_ulong = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_complex_set_all(
    mut v: *mut gsl_vector_complex,
    mut x: gsl_complex,
) {
    let data: *mut libc::c_double = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_int_set_all(
    mut v: *mut gsl_vector_int,
    mut x: libc::c_int,
) {
    let data: *mut libc::c_int = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_set_all(
    mut v: *mut gsl_vector_complex_long_double,
    mut x: gsl_complex_long_double,
) {
    let data: *mut f128::f128 = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex_long_double) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_char_set_all(
    mut v: *mut gsl_vector_char,
    mut x: libc::c_char,
) {
    let data: *mut libc::c_char = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_set_all(
    mut v: *mut gsl_vector,
    mut x: libc::c_double,
) {
    let data: *mut libc::c_double = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_short_set_all(
    mut v: *mut gsl_vector_short,
    mut x: libc::c_short,
) {
    let data: *mut libc::c_short = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_complex_float_set_all(
    mut v: *mut gsl_vector_complex_float,
    mut x: gsl_complex_float,
) {
    let data: *mut libc::c_float = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex_float) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_uchar_set_all(
    mut v: *mut gsl_vector_uchar,
    mut x: libc::c_uchar,
) {
    let data: *mut libc::c_uchar = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_float_set_all(
    mut v: *mut gsl_vector_float,
    mut x: libc::c_float,
) {
    let data: *mut libc::c_float = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = x;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_float_set_zero(mut v: *mut gsl_vector_float) {
    let data: *mut libc::c_float = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_float = 0.0f32;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_uint_set_zero(mut v: *mut gsl_vector_uint) {
    let data: *mut libc::c_uint = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_uint = 0 as libc::c_uint;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_char_set_zero(mut v: *mut gsl_vector_char) {
    let data: *mut libc::c_char = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_set_zero(mut v: *mut gsl_vector) {
    let data: *mut libc::c_double = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_double = 0.0f64;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_ushort_set_zero(mut v: *mut gsl_vector_ushort) {
    let data: *mut libc::c_ushort = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_ushort = 0 as libc::c_uint as libc::c_ushort;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_uchar_set_zero(mut v: *mut gsl_vector_uchar) {
    let data: *mut libc::c_uchar = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_uchar = 0 as libc::c_uint as libc::c_uchar;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_complex_float_set_zero(
    mut v: *mut gsl_vector_complex_float,
) {
    let data: *mut libc::c_float = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: gsl_complex_float = {
        let mut init = gsl_complex_float {
            dat: [0.0f32, 0.0f32],
        };
        init
    };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex_float) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_long_set_zero(mut v: *mut gsl_vector_long) {
    let data: *mut libc::c_long = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_long = 0 as libc::c_long;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_ulong_set_zero(mut v: *mut gsl_vector_ulong) {
    let data: *mut libc::c_ulong = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_ulong = 0 as libc::c_ulong;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_long_double_set_zero(
    mut v: *mut gsl_vector_long_double,
) {
    let data: *mut f128::f128 = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: f128::f128 = f128::f128::new(0.0);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_set_zero(
    mut v: *mut gsl_vector_complex_long_double,
) {
    let data: *mut f128::f128 = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: gsl_complex_long_double = {
        let mut init = gsl_complex_long_double {
            dat: [f128::f128::new(0.0), f128::f128::new(0.0)],
        };
        init
    };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex_long_double) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_int_set_zero(mut v: *mut gsl_vector_int) {
    let data: *mut libc::c_int = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_complex_set_zero(mut v: *mut gsl_vector_complex) {
    let data: *mut libc::c_double = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: gsl_complex = {
        let mut init = gsl_complex {
            dat: [0.0f64, 0.0f64],
        };
        init
    };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_short_set_zero(mut v: *mut gsl_vector_short) {
    let data: *mut libc::c_short = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = zero;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_vector_complex_set_basis(
    mut v: *mut gsl_vector_complex,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_double = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: gsl_complex = {
        let mut init = gsl_complex {
            dat: [0.0f64, 0.0f64],
        };
        init
    };
    let one: gsl_complex = {
        let mut init = gsl_complex {
            dat: [1.0f64, 0.0f64],
        };
        init
    };
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *(data
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) as *mut gsl_complex) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_set_basis(
    mut v: *mut gsl_vector_uint,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_uint = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_uint = 0 as libc::c_uint;
    let one: libc::c_uint = 1 as libc::c_uint;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_set_basis(
    mut v: *mut gsl_vector,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_double = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_double = 0.0f64;
    let one: libc::c_double = 1.0f64;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_set_basis(
    mut v: *mut gsl_vector_ulong,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_ulong = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_ulong = 0 as libc::c_ulong;
    let one: libc::c_ulong = 1 as libc::c_ulong;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_set_basis(
    mut v: *mut gsl_vector_long_double,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut f128::f128 = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: f128::f128 = f128::f128::new(0.0);
    let one: f128::f128 = f128::f128::new(1.0);
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_set_basis(
    mut v: *mut gsl_vector_long,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_long = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_long = 0 as libc::c_long;
    let one: libc::c_long = 1 as libc::c_long;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_set_basis(
    mut v: *mut gsl_vector_short,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_short = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_short = 0 as libc::c_int as libc::c_short;
    let one: libc::c_short = 1 as libc::c_int as libc::c_short;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_set_basis(
    mut v: *mut gsl_vector_char,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_char = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_char = 0 as libc::c_int as libc::c_char;
    let one: libc::c_char = 1 as libc::c_int as libc::c_char;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_set_basis(
    mut v: *mut gsl_vector_float,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_float = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_float = 0.0f32;
    let one: libc::c_float = 1.0f32;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_set_basis(
    mut v: *mut gsl_vector_int,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_int = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_int = 0 as libc::c_int;
    let one: libc::c_int = 1 as libc::c_int;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_set_basis(
    mut v: *mut gsl_vector_uchar,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_uchar = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_uchar = 0 as libc::c_uint as libc::c_uchar;
    let one: libc::c_uchar = 1 as libc::c_uint as libc::c_uchar;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_set_basis(
    mut v: *mut gsl_vector_complex_long_double,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut f128::f128 = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: gsl_complex_long_double = {
        let mut init = gsl_complex_long_double {
            dat: [f128::f128::new(0.0), f128::f128::new(0.0)],
        };
        init
    };
    let one: gsl_complex_long_double = {
        let mut init = gsl_complex_long_double {
            dat: [f128::f128::new(1.0), f128::f128::new(0.0)],
        };
        init
    };
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex_long_double) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *(data
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) as *mut gsl_complex_long_double) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_set_basis(
    mut v: *mut gsl_vector_ushort,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_ushort = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: libc::c_ushort = 0 as libc::c_uint as libc::c_ushort;
    let one: libc::c_ushort = 1 as libc::c_uint as libc::c_ushort;
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *data
            .offset(
                (1 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *data
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) = one;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_set_basis(
    mut v: *mut gsl_vector_complex_float,
    mut i: size_t,
) -> libc::c_int {
    let data: *mut libc::c_float = (*v).data;
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let zero: gsl_complex_float = {
        let mut init = gsl_complex_float {
            dat: [0.0f32, 0.0f32],
        };
        init
    };
    let one: gsl_complex_float = {
        let mut init = gsl_complex_float {
            dat: [1.0f32, 0.0f32],
        };
        init
    };
    let mut k: size_t = 0;
    if i >= n {
        gsl_error(
            b"index out of range\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int as size_t;
    while k < n {
        *(data
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(k).wrapping_mul(stride)
                    as isize,
            ) as *mut gsl_complex_float) = zero;
        k = k.wrapping_add(1);
        k;
    }
    *(data
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                as isize,
        ) as *mut gsl_complex_float) = one;
    return GSL_SUCCESS as libc::c_int;
}
