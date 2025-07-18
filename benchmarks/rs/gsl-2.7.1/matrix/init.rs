use ::libc;
use ::f128;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn gsl_block_float_alloc(n: size_t) -> *mut gsl_block_float;
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
    fn gsl_block_char_alloc(n: size_t) -> *mut gsl_block_char;
    fn gsl_block_char_free(b: *mut gsl_block_char);
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
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
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
pub struct gsl_matrix_ulong {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_matrix_long {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_matrix_uint {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_matrix_int {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_matrix_ushort {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_matrix_short {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_matrix_uchar {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
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
pub struct gsl_matrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
pub unsafe extern "C" fn gsl_matrix_ushort_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_ushort {
    let mut block: *mut gsl_block_ushort = 0 as *mut gsl_block_ushort;
    let mut m: *mut gsl_matrix_ushort = 0 as *mut gsl_matrix_ushort;
    m = malloc(::std::mem::size_of::<gsl_matrix_ushort>() as libc::c_ulong)
        as *mut gsl_matrix_ushort;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    }
    block = gsl_block_ushort_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_long {
    let mut block: *mut gsl_block_long = 0 as *mut gsl_block_long;
    let mut m: *mut gsl_matrix_long = 0 as *mut gsl_matrix_long;
    m = malloc(::std::mem::size_of::<gsl_matrix_long>() as libc::c_ulong)
        as *mut gsl_matrix_long;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    }
    block = gsl_block_long_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex_long_double {
    let mut block: *mut gsl_block_complex_long_double = 0
        as *mut gsl_block_complex_long_double;
    let mut m: *mut gsl_matrix_complex_long_double = 0
        as *mut gsl_matrix_complex_long_double;
    m = malloc(::std::mem::size_of::<gsl_matrix_complex_long_double>() as libc::c_ulong)
        as *mut gsl_matrix_complex_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    }
    block = gsl_block_complex_long_double_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_int_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_int {
    let mut block: *mut gsl_block_int = 0 as *mut gsl_block_int;
    let mut m: *mut gsl_matrix_int = 0 as *mut gsl_matrix_int;
    m = malloc(::std::mem::size_of::<gsl_matrix_int>() as libc::c_ulong)
        as *mut gsl_matrix_int;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    }
    block = gsl_block_int_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_ulong_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_ulong {
    let mut block: *mut gsl_block_ulong = 0 as *mut gsl_block_ulong;
    let mut m: *mut gsl_matrix_ulong = 0 as *mut gsl_matrix_ulong;
    m = malloc(::std::mem::size_of::<gsl_matrix_ulong>() as libc::c_ulong)
        as *mut gsl_matrix_ulong;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    }
    block = gsl_block_ulong_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_double_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_long_double {
    let mut block: *mut gsl_block_long_double = 0 as *mut gsl_block_long_double;
    let mut m: *mut gsl_matrix_long_double = 0 as *mut gsl_matrix_long_double;
    m = malloc(::std::mem::size_of::<gsl_matrix_long_double>() as libc::c_ulong)
        as *mut gsl_matrix_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    }
    block = gsl_block_long_double_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uint_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_uint {
    let mut block: *mut gsl_block_uint = 0 as *mut gsl_block_uint;
    let mut m: *mut gsl_matrix_uint = 0 as *mut gsl_matrix_uint;
    m = malloc(::std::mem::size_of::<gsl_matrix_uint>() as libc::c_ulong)
        as *mut gsl_matrix_uint;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    }
    block = gsl_block_uint_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_float_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_float {
    let mut block: *mut gsl_block_float = 0 as *mut gsl_block_float;
    let mut m: *mut gsl_matrix_float = 0 as *mut gsl_matrix_float;
    m = malloc(::std::mem::size_of::<gsl_matrix_float>() as libc::c_ulong)
        as *mut gsl_matrix_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    }
    block = gsl_block_float_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_char_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_char {
    let mut block: *mut gsl_block_char = 0 as *mut gsl_block_char;
    let mut m: *mut gsl_matrix_char = 0 as *mut gsl_matrix_char;
    m = malloc(::std::mem::size_of::<gsl_matrix_char>() as libc::c_ulong)
        as *mut gsl_matrix_char;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    }
    block = gsl_block_char_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex {
    let mut block: *mut gsl_block_complex = 0 as *mut gsl_block_complex;
    let mut m: *mut gsl_matrix_complex = 0 as *mut gsl_matrix_complex;
    m = malloc(::std::mem::size_of::<gsl_matrix_complex>() as libc::c_ulong)
        as *mut gsl_matrix_complex;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    }
    block = gsl_block_complex_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex_float {
    let mut block: *mut gsl_block_complex_float = 0 as *mut gsl_block_complex_float;
    let mut m: *mut gsl_matrix_complex_float = 0 as *mut gsl_matrix_complex_float;
    m = malloc(::std::mem::size_of::<gsl_matrix_complex_float>() as libc::c_ulong)
        as *mut gsl_matrix_complex_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    }
    block = gsl_block_complex_float_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_short_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_short {
    let mut block: *mut gsl_block_short = 0 as *mut gsl_block_short;
    let mut m: *mut gsl_matrix_short = 0 as *mut gsl_matrix_short;
    m = malloc(::std::mem::size_of::<gsl_matrix_short>() as libc::c_ulong)
        as *mut gsl_matrix_short;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    }
    block = gsl_block_short_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uchar_alloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_uchar {
    let mut block: *mut gsl_block_uchar = 0 as *mut gsl_block_uchar;
    let mut m: *mut gsl_matrix_uchar = 0 as *mut gsl_matrix_uchar;
    m = malloc(::std::mem::size_of::<gsl_matrix_uchar>() as libc::c_ulong)
        as *mut gsl_matrix_uchar;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    }
    block = gsl_block_uchar_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix {
    let mut block: *mut gsl_block = 0 as *mut gsl_block;
    let mut m: *mut gsl_matrix = 0 as *mut gsl_matrix;
    m = malloc(::std::mem::size_of::<gsl_matrix>() as libc::c_ulong) as *mut gsl_matrix;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    }
    block = gsl_block_alloc(n1.wrapping_mul(n2));
    if block.is_null() {
        gsl_error(
            b"failed to allocate space for block\0" as *const u8 as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    }
    (*m).data = (*block).data;
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = n2;
    (*m).block = block;
    (*m).owner = 1 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_ulong_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_ulong {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_ulong = gsl_matrix_ulong_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_ulong;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex_float {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_complex_float = gsl_matrix_complex_float_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_complex_float;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_int_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_int {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_int = gsl_matrix_int_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_int;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_double_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_long_double {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_long_double = gsl_matrix_long_double_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_long_double;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = f128::f128::new(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uchar_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_uchar {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_uchar = gsl_matrix_uchar_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_uchar;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_char_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_char {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_char = gsl_matrix_char_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_char;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_complex = gsl_matrix_complex_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_complex;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_long {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_long = gsl_matrix_long_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_long;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_long;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uint_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_uint {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_uint = gsl_matrix_uint_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_uint;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_short_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_short {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_short = gsl_matrix_short_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_short;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex_long_double {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_complex_long_double = gsl_matrix_complex_long_double_alloc(
        n1,
        n2,
    );
    if m.is_null() {
        return 0 as *mut gsl_matrix_complex_long_double;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = f128::f128::new(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_float_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_float {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_float = gsl_matrix_float_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_float;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_ushort_calloc(
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_ushort {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix_ushort = gsl_matrix_ushort_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix_ushort;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *mut gsl_matrix {
    let mut i: size_t = 0;
    let mut m: *mut gsl_matrix = gsl_matrix_alloc(n1, n2);
    if m.is_null() {
        return 0 as *mut gsl_matrix;
    }
    memset(
        (*m).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n1)
            .wrapping_mul(n2)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n1).wrapping_mul(n2) {
        *((*m).data).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_alloc_from_block(
    mut block: *mut gsl_block_long,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_long {
    let mut m: *mut gsl_matrix_long = 0 as *mut gsl_matrix_long;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_long>() as libc::c_ulong)
        as *mut gsl_matrix_long;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_float_alloc_from_block(
    mut block: *mut gsl_block_float,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_float {
    let mut m: *mut gsl_matrix_float = 0 as *mut gsl_matrix_float;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_float>() as libc::c_ulong)
        as *mut gsl_matrix_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_alloc_from_block(
    mut block: *mut gsl_block_complex_float,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_complex_float {
    let mut m: *mut gsl_matrix_complex_float = 0 as *mut gsl_matrix_complex_float;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_complex_float>() as libc::c_ulong)
        as *mut gsl_matrix_complex_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    }
    (*m)
        .data = ((*block).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_alloc_from_block(
    mut block: *mut gsl_block_complex,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_complex {
    let mut m: *mut gsl_matrix_complex = 0 as *mut gsl_matrix_complex;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_complex>() as libc::c_ulong)
        as *mut gsl_matrix_complex;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    }
    (*m)
        .data = ((*block).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_ushort_alloc_from_block(
    mut block: *mut gsl_block_ushort,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_ushort {
    let mut m: *mut gsl_matrix_ushort = 0 as *mut gsl_matrix_ushort;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_ushort>() as libc::c_ulong)
        as *mut gsl_matrix_ushort;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_ulong_alloc_from_block(
    mut block: *mut gsl_block_ulong,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_ulong {
    let mut m: *mut gsl_matrix_ulong = 0 as *mut gsl_matrix_ulong;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_ulong>() as libc::c_ulong)
        as *mut gsl_matrix_ulong;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_alloc_from_block(
    mut block: *mut gsl_block_complex_long_double,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_complex_long_double {
    let mut m: *mut gsl_matrix_complex_long_double = 0
        as *mut gsl_matrix_complex_long_double;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_complex_long_double>() as libc::c_ulong)
        as *mut gsl_matrix_complex_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    }
    (*m)
        .data = ((*block).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_char_alloc_from_block(
    mut block: *mut gsl_block_char,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_char {
    let mut m: *mut gsl_matrix_char = 0 as *mut gsl_matrix_char;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_char>() as libc::c_ulong)
        as *mut gsl_matrix_char;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uint_alloc_from_block(
    mut block: *mut gsl_block_uint,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_uint {
    let mut m: *mut gsl_matrix_uint = 0 as *mut gsl_matrix_uint;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_uint>() as libc::c_ulong)
        as *mut gsl_matrix_uint;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_short_alloc_from_block(
    mut block: *mut gsl_block_short,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_short {
    let mut m: *mut gsl_matrix_short = 0 as *mut gsl_matrix_short;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_short>() as libc::c_ulong)
        as *mut gsl_matrix_short;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_int_alloc_from_block(
    mut block: *mut gsl_block_int,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_int {
    let mut m: *mut gsl_matrix_int = 0 as *mut gsl_matrix_int;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_int>() as libc::c_ulong)
        as *mut gsl_matrix_int;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_double_alloc_from_block(
    mut block: *mut gsl_block_long_double,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_long_double {
    let mut m: *mut gsl_matrix_long_double = 0 as *mut gsl_matrix_long_double;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_long_double>() as libc::c_ulong)
        as *mut gsl_matrix_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uchar_alloc_from_block(
    mut block: *mut gsl_block_uchar,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix_uchar {
    let mut m: *mut gsl_matrix_uchar = 0 as *mut gsl_matrix_uchar;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_uchar>() as libc::c_ulong)
        as *mut gsl_matrix_uchar;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_alloc_from_block(
    mut block: *mut gsl_block,
    offset: size_t,
    n1: size_t,
    n2: size_t,
    d2: size_t,
) -> *mut gsl_matrix {
    let mut m: *mut gsl_matrix = 0 as *mut gsl_matrix;
    if d2 < n2 {
        gsl_error(
            b"matrix dimension d2 must be greater than n2\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    } else if (*block).size < offset.wrapping_add(n1.wrapping_mul(d2)) {
        gsl_error(
            b"matrix size exceeds available block size\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix>() as libc::c_ulong) as *mut gsl_matrix;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    }
    (*m)
        .data = ((*block).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(offset) as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = d2;
    (*m).block = block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_float_alloc_from_matrix(
    mut mm: *mut gsl_matrix_float,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_float {
    let mut m: *mut gsl_matrix_float = 0 as *mut gsl_matrix_float;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_float>() as libc::c_ulong)
        as *mut gsl_matrix_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_float;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_ushort_alloc_from_matrix(
    mut mm: *mut gsl_matrix_ushort,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_ushort {
    let mut m: *mut gsl_matrix_ushort = 0 as *mut gsl_matrix_ushort;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_ushort>() as libc::c_ulong)
        as *mut gsl_matrix_ushort;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ushort;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uint_alloc_from_matrix(
    mut mm: *mut gsl_matrix_uint,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_uint {
    let mut m: *mut gsl_matrix_uint = 0 as *mut gsl_matrix_uint;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_uint>() as libc::c_ulong)
        as *mut gsl_matrix_uint;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uint;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_alloc_from_matrix(
    mut mm: *mut gsl_matrix_complex_long_double,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex_long_double {
    let mut m: *mut gsl_matrix_complex_long_double = 0
        as *mut gsl_matrix_complex_long_double;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_complex_long_double>() as libc::c_ulong)
        as *mut gsl_matrix_complex_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_long_double;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_alloc_from_matrix(
    mut mm: *mut gsl_matrix_long,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_long {
    let mut m: *mut gsl_matrix_long = 0 as *mut gsl_matrix_long;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_long>() as libc::c_ulong)
        as *mut gsl_matrix_long;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_alloc_from_matrix(
    mut mm: *mut gsl_matrix_complex,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex {
    let mut m: *mut gsl_matrix_complex = 0 as *mut gsl_matrix_complex;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_complex>() as libc::c_ulong)
        as *mut gsl_matrix_complex;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_uchar_alloc_from_matrix(
    mut mm: *mut gsl_matrix_uchar,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_uchar {
    let mut m: *mut gsl_matrix_uchar = 0 as *mut gsl_matrix_uchar;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_uchar>() as libc::c_ulong)
        as *mut gsl_matrix_uchar;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_uchar;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_ulong_alloc_from_matrix(
    mut mm: *mut gsl_matrix_ulong,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_ulong {
    let mut m: *mut gsl_matrix_ulong = 0 as *mut gsl_matrix_ulong;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_ulong>() as libc::c_ulong)
        as *mut gsl_matrix_ulong;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_ulong;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_alloc_from_matrix(
    mut mm: *mut gsl_matrix_complex_float,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_complex_float {
    let mut m: *mut gsl_matrix_complex_float = 0 as *mut gsl_matrix_complex_float;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_complex_float>() as libc::c_ulong)
        as *mut gsl_matrix_complex_float;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_complex_float;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_char_alloc_from_matrix(
    mut mm: *mut gsl_matrix_char,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_char {
    let mut m: *mut gsl_matrix_char = 0 as *mut gsl_matrix_char;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_char>() as libc::c_ulong)
        as *mut gsl_matrix_char;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_char;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_long_double_alloc_from_matrix(
    mut mm: *mut gsl_matrix_long_double,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_long_double {
    let mut m: *mut gsl_matrix_long_double = 0 as *mut gsl_matrix_long_double;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_long_double>() as libc::c_ulong)
        as *mut gsl_matrix_long_double;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_long_double;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_int_alloc_from_matrix(
    mut mm: *mut gsl_matrix_int,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_int {
    let mut m: *mut gsl_matrix_int = 0 as *mut gsl_matrix_int;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_int>() as libc::c_ulong)
        as *mut gsl_matrix_int;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_int;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_short_alloc_from_matrix(
    mut mm: *mut gsl_matrix_short,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix_short {
    let mut m: *mut gsl_matrix_short = 0 as *mut gsl_matrix_short;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix_short>() as libc::c_ulong)
        as *mut gsl_matrix_short;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix_short;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_alloc_from_matrix(
    mut mm: *mut gsl_matrix,
    k1: size_t,
    k2: size_t,
    n1: size_t,
    n2: size_t,
) -> *mut gsl_matrix {
    let mut m: *mut gsl_matrix = 0 as *mut gsl_matrix;
    if k1.wrapping_add(n1) > (*mm).size1 {
        gsl_error(
            b"submatrix dimension 1 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    } else if k2.wrapping_add(n2) > (*mm).size2 {
        gsl_error(
            b"submatrix dimension 2 exceeds size of original\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    }
    m = malloc(::std::mem::size_of::<gsl_matrix>() as libc::c_ulong) as *mut gsl_matrix;
    if m.is_null() {
        gsl_error(
            b"failed to allocate space for matrix struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_matrix;
    }
    (*m)
        .data = ((*mm).data)
        .offset(k1.wrapping_mul((*mm).tda) as isize)
        .offset(k2 as isize);
    (*m).size1 = n1;
    (*m).size2 = n2;
    (*m).tda = (*mm).tda;
    (*m).block = (*mm).block;
    (*m).owner = 0 as libc::c_int;
    return m;
}
pub unsafe extern "C" fn gsl_matrix_short_free(mut m: *mut gsl_matrix_short) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_short_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_char_free(mut m: *mut gsl_matrix_char) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_char_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_long_double_free(
    mut m: *mut gsl_matrix_long_double,
) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_long_double_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_free(
    mut m: *mut gsl_matrix_complex_long_double,
) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_complex_long_double_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_int_free(mut m: *mut gsl_matrix_int) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_int_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_long_free(mut m: *mut gsl_matrix_long) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_long_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_free(mut m: *mut gsl_matrix) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_complex_free(mut m: *mut gsl_matrix_complex) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_complex_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_complex_float_free(
    mut m: *mut gsl_matrix_complex_float,
) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_complex_float_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_uint_free(mut m: *mut gsl_matrix_uint) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_uint_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_uchar_free(mut m: *mut gsl_matrix_uchar) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_uchar_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_ulong_free(mut m: *mut gsl_matrix_ulong) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_ulong_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_float_free(mut m: *mut gsl_matrix_float) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_float_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_ushort_free(mut m: *mut gsl_matrix_ushort) {
    if m.is_null() {
        return;
    }
    if (*m).owner != 0 {
        gsl_block_ushort_free((*m).block);
    }
    free(m as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_matrix_ulong_set_identity(mut m: *mut gsl_matrix_ulong) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_ulong = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_ulong = 0 as libc::c_ulong;
    let one: libc::c_ulong = 1 as libc::c_ulong;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_uint_set_identity(mut m: *mut gsl_matrix_uint) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_uint = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_uint = 0 as libc::c_uint;
    let one: libc::c_uint = 1 as libc::c_uint;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_set_identity(mut m: *mut gsl_matrix) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_double = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_double = 0.0f64;
    let one: libc::c_double = 1.0f64;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_int_set_identity(mut m: *mut gsl_matrix_int) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_int = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_int = 0 as libc::c_int;
    let one: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_uchar_set_identity(mut m: *mut gsl_matrix_uchar) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_uchar = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_uchar = 0 as libc::c_uint as libc::c_uchar;
    let one: libc::c_uchar = 1 as libc::c_uint as libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = (if i == j { one as libc::c_int } else { zero as libc::c_int })
                as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_ushort_set_identity(mut m: *mut gsl_matrix_ushort) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_ushort = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_ushort = 0 as libc::c_uint as libc::c_ushort;
    let one: libc::c_ushort = 1 as libc::c_uint as libc::c_ushort;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = (if i == j { one as libc::c_int } else { zero as libc::c_int })
                as libc::c_ushort;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_long_set_identity(mut m: *mut gsl_matrix_long) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_long = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_long = 0 as libc::c_long;
    let one: libc::c_long = 1 as libc::c_long;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_float_set_identity(mut m: *mut gsl_matrix_float) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_float = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_float = 0.0f32;
    let one: libc::c_float = 1.0f32;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_set_identity(
    mut m: *mut gsl_matrix_complex,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_double = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
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
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_set_identity(
    mut m: *mut gsl_matrix_complex_long_double,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut f128::f128 = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
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
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex_long_double) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_short_set_identity(mut m: *mut gsl_matrix_short) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_short = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_short = 0 as libc::c_int as libc::c_short;
    let one: libc::c_short = 1 as libc::c_int as libc::c_short;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = (if i == j { one as libc::c_int } else { zero as libc::c_int })
                as libc::c_short;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_long_double_set_identity(
    mut m: *mut gsl_matrix_long_double,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut f128::f128 = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: f128::f128 = f128::f128::new(0.0);
    let one: f128::f128 = f128::f128::new(1.0);
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_float_set_identity(
    mut m: *mut gsl_matrix_complex_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_float = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
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
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex_float) = if i == j { one } else { zero };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_char_set_identity(mut m: *mut gsl_matrix_char) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_char = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_char = 0 as libc::c_int as libc::c_char;
    let one: libc::c_char = 1 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = (if i == j { one as libc::c_int } else { zero as libc::c_int })
                as libc::c_char;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_long_set_zero(mut m: *mut gsl_matrix_long) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_long = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_long = 0 as libc::c_long;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_set_zero(
    mut m: *mut gsl_matrix_complex_long_double,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut f128::f128 = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: gsl_complex_long_double = {
        let mut init = gsl_complex_long_double {
            dat: [f128::f128::new(0.0), f128::f128::new(0.0)],
        };
        init
    };
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex_long_double) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_char_set_zero(mut m: *mut gsl_matrix_char) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_char = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_char = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_ulong_set_zero(mut m: *mut gsl_matrix_ulong) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_ulong = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_ulong = 0 as libc::c_ulong;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_uint_set_zero(mut m: *mut gsl_matrix_uint) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_uint = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_uint = 0 as libc::c_uint;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_int_set_zero(mut m: *mut gsl_matrix_int) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_int = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_short_set_zero(mut m: *mut gsl_matrix_short) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_short = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_short = 0 as libc::c_int as libc::c_short;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_uchar_set_zero(mut m: *mut gsl_matrix_uchar) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_uchar = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_uchar = 0 as libc::c_uint as libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_set_zero(mut m: *mut gsl_matrix) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_double = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_ushort_set_zero(mut m: *mut gsl_matrix_ushort) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_ushort = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_ushort = 0 as libc::c_uint as libc::c_ushort;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_set_zero(mut m: *mut gsl_matrix_complex) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_double = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: gsl_complex = {
        let mut init = gsl_complex {
            dat: [0.0f64, 0.0f64],
        };
        init
    };
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_float_set_zero(mut m: *mut gsl_matrix_float) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_float = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: libc::c_float = 0.0f32;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_long_double_set_zero(
    mut m: *mut gsl_matrix_long_double,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut f128::f128 = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: f128::f128 = f128::f128::new(0.0);
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_float_set_zero(
    mut m: *mut gsl_matrix_complex_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_float = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    let zero: gsl_complex_float = {
        let mut init = gsl_complex_float {
            dat: [0.0f32, 0.0f32],
        };
        init
    };
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex_float) = zero;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_short_set_all(
    mut m: *mut gsl_matrix_short,
    mut x: libc::c_short,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_short = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_ulong_set_all(
    mut m: *mut gsl_matrix_ulong,
    mut x: libc::c_ulong,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_ulong = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_long_double_set_all(
    mut m: *mut gsl_matrix_long_double,
    mut x: f128::f128,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut f128::f128 = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_set_all(
    mut m: *mut gsl_matrix_complex_long_double,
    mut x: gsl_complex_long_double,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut f128::f128 = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex_long_double) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_uchar_set_all(
    mut m: *mut gsl_matrix_uchar,
    mut x: libc::c_uchar,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_uchar = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_int_set_all(
    mut m: *mut gsl_matrix_int,
    mut x: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_int = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_set_all(
    mut m: *mut gsl_matrix_complex,
    mut x: gsl_complex,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_double = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_char_set_all(
    mut m: *mut gsl_matrix_char,
    mut x: libc::c_char,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_char = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_complex_float_set_all(
    mut m: *mut gsl_matrix_complex_float,
    mut x: gsl_complex_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_float = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *(data
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) as *mut gsl_complex_float) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_uint_set_all(
    mut m: *mut gsl_matrix_uint,
    mut x: libc::c_uint,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_uint = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_set_all(
    mut m: *mut gsl_matrix,
    mut x: libc::c_double,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_double = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_long_set_all(
    mut m: *mut gsl_matrix_long,
    mut x: libc::c_long,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_long = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_float_set_all(
    mut m: *mut gsl_matrix_float,
    mut x: libc::c_float,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_float = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_matrix_ushort_set_all(
    mut m: *mut gsl_matrix_ushort,
    mut x: libc::c_ushort,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let data: *mut libc::c_ushort = (*m).data;
    let p: size_t = (*m).size1;
    let q: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    i = 0 as libc::c_int as size_t;
    while i < p {
        j = 0 as libc::c_int as size_t;
        while j < q {
            *data
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i.wrapping_mul(tda).wrapping_add(j)) as isize,
                ) = x;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
