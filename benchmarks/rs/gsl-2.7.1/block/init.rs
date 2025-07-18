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
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub unsafe extern "C" fn gsl_block_int_alloc(n: size_t) -> *mut gsl_block_int {
    let mut b: *mut gsl_block_int = 0 as *mut gsl_block_int;
    b = malloc(::std::mem::size_of::<gsl_block_int>() as libc::c_ulong)
        as *mut gsl_block_int;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_int;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_int;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_uint_alloc(n: size_t) -> *mut gsl_block_uint {
    let mut b: *mut gsl_block_uint = 0 as *mut gsl_block_uint;
    b = malloc(::std::mem::size_of::<gsl_block_uint>() as libc::c_ulong)
        as *mut gsl_block_uint;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_uint;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_uint;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_alloc(
    n: size_t,
) -> *mut gsl_block_complex_long_double {
    let mut b: *mut gsl_block_complex_long_double = 0
        as *mut gsl_block_complex_long_double;
    b = malloc(::std::mem::size_of::<gsl_block_complex_long_double>() as libc::c_ulong)
        as *mut gsl_block_complex_long_double;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_complex_long_double;
    }
    (*b)
        .data = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    ) as *mut f128::f128;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_complex_long_double;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_ushort_alloc(n: size_t) -> *mut gsl_block_ushort {
    let mut b: *mut gsl_block_ushort = 0 as *mut gsl_block_ushort;
    b = malloc(::std::mem::size_of::<gsl_block_ushort>() as libc::c_ulong)
        as *mut gsl_block_ushort;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_ushort;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    ) as *mut libc::c_ushort;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_ushort;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_long_double_alloc(
    n: size_t,
) -> *mut gsl_block_long_double {
    let mut b: *mut gsl_block_long_double = 0 as *mut gsl_block_long_double;
    b = malloc(::std::mem::size_of::<gsl_block_long_double>() as libc::c_ulong)
        as *mut gsl_block_long_double;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_long_double;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    ) as *mut f128::f128;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_long_double;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_long_alloc(n: size_t) -> *mut gsl_block_long {
    let mut b: *mut gsl_block_long = 0 as *mut gsl_block_long;
    b = malloc(::std::mem::size_of::<gsl_block_long>() as libc::c_ulong)
        as *mut gsl_block_long;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_long;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    ) as *mut libc::c_long;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_long;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_complex_float_alloc(
    n: size_t,
) -> *mut gsl_block_complex_float {
    let mut b: *mut gsl_block_complex_float = 0 as *mut gsl_block_complex_float;
    b = malloc(::std::mem::size_of::<gsl_block_complex_float>() as libc::c_ulong)
        as *mut gsl_block_complex_float;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_complex_float;
    }
    (*b)
        .data = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_complex_float;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_char_alloc(n: size_t) -> *mut gsl_block_char {
    let mut b: *mut gsl_block_char = 0 as *mut gsl_block_char;
    b = malloc(::std::mem::size_of::<gsl_block_char>() as libc::c_ulong)
        as *mut gsl_block_char;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_char;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_char;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_short_alloc(n: size_t) -> *mut gsl_block_short {
    let mut b: *mut gsl_block_short = 0 as *mut gsl_block_short;
    b = malloc(::std::mem::size_of::<gsl_block_short>() as libc::c_ulong)
        as *mut gsl_block_short;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_short;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_short;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_short;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_ulong_alloc(n: size_t) -> *mut gsl_block_ulong {
    let mut b: *mut gsl_block_ulong = 0 as *mut gsl_block_ulong;
    b = malloc(::std::mem::size_of::<gsl_block_ulong>() as libc::c_ulong)
        as *mut gsl_block_ulong;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_ulong;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_ulong;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_complex_alloc(n: size_t) -> *mut gsl_block_complex {
    let mut b: *mut gsl_block_complex = 0 as *mut gsl_block_complex;
    b = malloc(::std::mem::size_of::<gsl_block_complex>() as libc::c_ulong)
        as *mut gsl_block_complex;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_complex;
    }
    (*b)
        .data = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_complex;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_float_alloc(n: size_t) -> *mut gsl_block_float {
    let mut b: *mut gsl_block_float = 0 as *mut gsl_block_float;
    b = malloc(::std::mem::size_of::<gsl_block_float>() as libc::c_ulong)
        as *mut gsl_block_float;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_float;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_float;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_uchar_alloc(n: size_t) -> *mut gsl_block_uchar {
    let mut b: *mut gsl_block_uchar = 0 as *mut gsl_block_uchar;
    b = malloc(::std::mem::size_of::<gsl_block_uchar>() as libc::c_ulong)
        as *mut gsl_block_uchar;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_uchar;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block_uchar;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_alloc(n: size_t) -> *mut gsl_block {
    let mut b: *mut gsl_block = 0 as *mut gsl_block;
    b = malloc(::std::mem::size_of::<gsl_block>() as libc::c_ulong) as *mut gsl_block;
    if b.is_null() {
        gsl_error(
            b"failed to allocate space for block struct\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block;
    }
    (*b)
        .data = malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*b).data).is_null() && n > 0 as libc::c_int as libc::c_ulong {
        free(b as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for block data\0" as *const u8
                as *const libc::c_char,
            b"./init_source.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_block;
    }
    (*b).size = n;
    return b;
}
pub unsafe extern "C" fn gsl_block_float_calloc(n: size_t) -> *mut gsl_block_float {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_float = gsl_block_float_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_float;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_int_calloc(n: size_t) -> *mut gsl_block_int {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_int = gsl_block_int_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_int;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_short_calloc(n: size_t) -> *mut gsl_block_short {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_short = gsl_block_short_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_short;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_calloc(
    n: size_t,
) -> *mut gsl_block_complex_long_double {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_complex_long_double = gsl_block_complex_long_double_alloc(
        n,
    );
    if b.is_null() {
        return 0 as *mut gsl_block_complex_long_double;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = f128::f128::new(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_char_calloc(n: size_t) -> *mut gsl_block_char {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_char = gsl_block_char_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_char;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_uchar_calloc(n: size_t) -> *mut gsl_block_uchar {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_uchar = gsl_block_uchar_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_uchar;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_ulong_calloc(n: size_t) -> *mut gsl_block_ulong {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_ulong = gsl_block_ulong_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_ulong;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_long_calloc(n: size_t) -> *mut gsl_block_long {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_long = gsl_block_long_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_long;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_long;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_complex_calloc(n: size_t) -> *mut gsl_block_complex {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_complex = gsl_block_complex_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_complex;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_uint_calloc(n: size_t) -> *mut gsl_block_uint {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_uint = gsl_block_uint_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_uint;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_ushort_calloc(n: size_t) -> *mut gsl_block_ushort {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_ushort = gsl_block_ushort_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_ushort;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_complex_float_calloc(
    n: size_t,
) -> *mut gsl_block_complex_float {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_complex_float = gsl_block_complex_float_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_complex_float;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_long_double_calloc(
    n: size_t,
) -> *mut gsl_block_long_double {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block_long_double = gsl_block_long_double_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block_long_double;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = f128::f128::new(0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_calloc(n: size_t) -> *mut gsl_block {
    let mut i: size_t = 0;
    let mut b: *mut gsl_block = gsl_block_alloc(n);
    if b.is_null() {
        return 0 as *mut gsl_block;
    }
    memset(
        (*b).data as *mut libc::c_void,
        0 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as size_t;
    while i < (1 as libc::c_int as libc::c_ulong).wrapping_mul(n) {
        *((*b).data).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return b;
}
pub unsafe extern "C" fn gsl_block_ulong_free(mut b: *mut gsl_block_ulong) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_int_free(mut b: *mut gsl_block_int) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_complex_float_free(
    mut b: *mut gsl_block_complex_float,
) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_float_free(mut b: *mut gsl_block_float) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_long_double_free(mut b: *mut gsl_block_long_double) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_ushort_free(mut b: *mut gsl_block_ushort) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_complex_free(mut b: *mut gsl_block_complex) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_uint_free(mut b: *mut gsl_block_uint) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_char_free(mut b: *mut gsl_block_char) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_long_free(mut b: *mut gsl_block_long) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_complex_long_double_free(
    mut b: *mut gsl_block_complex_long_double,
) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_free(mut b: *mut gsl_block) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_uchar_free(mut b: *mut gsl_block_uchar) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_block_short_free(mut b: *mut gsl_block_short) {
    if b.is_null() {
        return;
    }
    free((*b).data as *mut libc::c_void);
    free(b as *mut libc::c_void);
}
