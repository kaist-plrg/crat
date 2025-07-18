use ::libc;
use ::f128;
extern "C" {
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
pub unsafe extern "C" fn gsl_vector_ushort_equal(
    mut u: *const gsl_vector_ushort,
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    ) as libc::c_int
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_equal(
    mut u: *const gsl_vector_uint,
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_equal(
    mut u: *const gsl_vector_long_double,
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_equal(
    mut u: *const gsl_vector_complex_float,
    mut v: *const gsl_vector_complex_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_equal(
    mut u: *const gsl_vector_char,
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    ) as libc::c_int
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_equal(
    mut u: *const gsl_vector_short,
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    ) as libc::c_int
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_equal(
    mut u: *const gsl_vector_ulong,
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_equal(
    mut u: *const gsl_vector_complex,
    mut v: *const gsl_vector_complex,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_equal(
    mut u: *const gsl_vector_long,
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_equal(
    mut u: *const gsl_vector_uchar,
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    ) as libc::c_int
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_equal(
    mut u: *const gsl_vector,
    mut v: *const gsl_vector,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_equal(
    mut u: *const gsl_vector_complex_long_double,
    mut v: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_equal(
    mut u: *const gsl_vector_float,
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_equal(
    mut u: *const gsl_vector_int,
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride_u: size_t = (*u).stride;
    let stride_v: size_t = (*v).stride;
    let mut j: size_t = 0;
    if (*u).size != (*v).size {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./prop_source.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*u).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride_u)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                )
                != *((*v).data)
                    .offset(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride_v)
                            .wrapping_mul(j)
                            .wrapping_add(k) as isize,
                    )
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_isnull(
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_isnull(
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) != f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_isnull(
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_isnull(
    mut v: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) != f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_isnull(
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_isnull(
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_isnull(
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_isnull(
    mut v: *const gsl_vector_complex_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_isnull(
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_isnull(
    mut v: *const gsl_vector_complex,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_isnull(mut v: *const gsl_vector) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_isnull(
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_isnull(
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_isnull(
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double != 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ispos(mut v: *const gsl_vector) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_ispos(
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_ispos(
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_ispos(
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_ispos(
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_ispos(
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_ispos(
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) <= f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_ispos(
    mut v: *const gsl_vector_complex,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_ispos(
    mut v: *const gsl_vector_complex_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_ispos(
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_ispos(
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_ispos(
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_ispos(
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double <= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_ispos(
    mut v: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) <= f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_isneg(
    mut v: *const gsl_vector_complex_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_isneg(
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_isneg(
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_isneg(
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_isneg(
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) >= f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_isneg(
    mut v: *const gsl_vector_complex,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_isneg(
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_isneg(
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_isneg(
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_isneg(
    mut v: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) >= f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_isneg(
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_isneg(mut v: *const gsl_vector) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_isneg(
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_isneg(
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double >= 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_isnonneg(
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_isnonneg(mut v: *const gsl_vector) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_isnonneg(
    mut v: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) < f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_isnonneg(
    mut v: *const gsl_vector_complex_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_isnonneg(
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_isnonneg(
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_isnonneg(
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_isnonneg(
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_isnonneg(
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) < f128::f128::new(0.0f64)
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_isnonneg(
    mut v: *const gsl_vector_complex,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            if *((*v).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_isnonneg(
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_isnonneg(
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_isnonneg(
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_isnonneg(
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    let n: size_t = (*v).size;
    let stride: size_t = (*v).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            if (*((*v).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) as libc::c_int as libc::c_double) < 0.0f64
            {
                return 0 as libc::c_int;
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 1 as libc::c_int;
}
