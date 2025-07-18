use ::libc;
use ::num_traits;
use ::f128;
use num_traits::ToPrimitive;
extern "C" {
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_zscal(alpha: gsl_complex, X: *mut gsl_vector_complex);
    fn gsl_blas_sscal(alpha: libc::c_float, X: *mut gsl_vector_float);
    fn gsl_blas_cscal(alpha: gsl_complex_float, X: *mut gsl_vector_complex_float);
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
pub unsafe extern "C" fn gsl_vector_complex_long_double_add(
    mut a: *mut gsl_vector_complex_long_double,
    mut b: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                )
                += *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b) as isize,
                    );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                += *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_float_add(
    mut a: *mut gsl_vector_complex_float,
    mut b: *const gsl_vector_complex_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                )
                += *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b) as isize,
                    );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                += *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_add(
    mut a: *mut gsl_vector_complex,
    mut b: *const gsl_vector_complex,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                )
                += *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b) as isize,
                    );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                += *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_float_sub(
    mut a: *mut gsl_vector_complex_float,
    mut b: *const gsl_vector_complex_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                )
                -= *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b) as isize,
                    );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                -= *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_sub(
    mut a: *mut gsl_vector_complex,
    mut b: *const gsl_vector_complex,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                )
                -= *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b) as isize,
                    );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                -= *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_sub(
    mut a: *mut gsl_vector_complex_long_double,
    mut b: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                )
                -= *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b) as isize,
                    );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                -= *((*b).data)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_mul(stride_b)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_float_mul(
    mut a: *mut gsl_vector_complex_float,
    mut b: *const gsl_vector_complex_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut ar: libc::c_float = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                );
            let mut ai: libc::c_float = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut br: libc::c_float = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b) as isize,
                );
            let mut bi: libc::c_float = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                ) = ar * br - ai * bi;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ar * bi + ai * br;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_mul(
    mut a: *mut gsl_vector_complex,
    mut b: *const gsl_vector_complex,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut ar: libc::c_double = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                );
            let mut ai: libc::c_double = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut br: libc::c_double = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b) as isize,
                );
            let mut bi: libc::c_double = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                ) = ar * br - ai * bi;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ar * bi + ai * br;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_mul(
    mut a: *mut gsl_vector_complex_long_double,
    mut b: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut ar: f128::f128 = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                );
            let mut ai: f128::f128 = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut br: f128::f128 = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b) as isize,
                );
            let mut bi: f128::f128 = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                ) = ar * br - ai * bi;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ar * bi + ai * br;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_float_div(
    mut a: *mut gsl_vector_complex_float,
    mut b: *const gsl_vector_complex_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut ar: libc::c_float = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                );
            let mut ai: libc::c_float = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut br: libc::c_float = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b) as isize,
                );
            let mut bi: libc::c_float = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut s: libc::c_float = (1.0f64
                / hypot(br as libc::c_double, bi as libc::c_double)) as libc::c_float;
            let mut sbr: libc::c_float = s * br;
            let mut sbi: libc::c_float = s * bi;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                ) = (ar * sbr + ai * sbi) * s;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = (ai * sbr - ar * sbi) * s;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_div(
    mut a: *mut gsl_vector_complex,
    mut b: *const gsl_vector_complex,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut ar: libc::c_double = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                );
            let mut ai: libc::c_double = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut br: libc::c_double = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b) as isize,
                );
            let mut bi: libc::c_double = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut s: libc::c_double = 1.0f64 / hypot(br, bi);
            let mut sbr: libc::c_double = s * br;
            let mut sbi: libc::c_double = s * bi;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                ) = (ar * sbr + ai * sbi) * s;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = (ai * sbr - ar * sbi) * s;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_div(
    mut a: *mut gsl_vector_complex_long_double,
    mut b: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut ar: f128::f128 = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                );
            let mut ai: f128::f128 = *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut br: f128::f128 = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b) as isize,
                );
            let mut bi: f128::f128 = *((*b).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_b)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut s: f128::f128 = f128::f128::new(
                1.0f64 / hypot(br.to_f64().unwrap(), bi.to_f64().unwrap()),
            );
            let mut sbr: f128::f128 = s * br;
            let mut sbi: f128::f128 = s * bi;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a) as isize,
                ) = (ar * sbr + ai * sbi) * s;
            *((*a).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_mul(stride_a)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = (ai * sbr - ar * sbi) * s;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_float_scale(
    mut a: *mut gsl_vector_complex_float,
    x: gsl_complex_float,
) -> libc::c_int {
    gsl_blas_cscal(x, a);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_scale(
    mut a: *mut gsl_vector_complex,
    x: gsl_complex,
) -> libc::c_int {
    gsl_blas_zscal(x, a);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_scale(
    mut a: *mut gsl_vector_complex_long_double,
    x: gsl_complex_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    let mut xr: f128::f128 = x.dat[0 as libc::c_int as usize];
    let mut xi: f128::f128 = x.dat[1 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut ar: f128::f128 = *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            );
        let mut ai: f128::f128 = *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) = ar * xr - ai * xi;
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = ar * xi + ai * xr;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_add_constant(
    mut a: *mut gsl_vector_complex,
    x: gsl_complex,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    let mut xr: libc::c_double = x.dat[0 as libc::c_int as usize];
    let mut xi: libc::c_double = x.dat[1 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) += xr;
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) += xi;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_add_constant(
    mut a: *mut gsl_vector_complex_float,
    x: gsl_complex_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    let mut xr: libc::c_float = x.dat[0 as libc::c_int as usize];
    let mut xi: libc::c_float = x.dat[1 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) += xr;
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) += xi;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_add_constant(
    mut a: *mut gsl_vector_complex_long_double,
    x: gsl_complex_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    let mut xr: f128::f128 = x.dat[0 as libc::c_int as usize];
    let mut xi: f128::f128 = x.dat[1 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul(stride)
                    as isize,
            ) += xr;
        *((*a).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) += xi;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_axpby(
    alpha: gsl_complex_float,
    mut x: *const gsl_vector_complex_float,
    beta: gsl_complex_float,
    mut y: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta.dat[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float
        && beta.dat[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float
    {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let ar: libc::c_float = alpha.dat[0 as libc::c_int as usize];
        let ai: libc::c_float = alpha.dat[1 as libc::c_int as usize];
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            let mut xr: libc::c_float = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(x_stride) as isize,
                );
            let mut xi: libc::c_float = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(x_stride)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(y_stride) as isize,
                ) = ar * xr - ai * xi;
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(y_stride)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ai * xr + ar * xi;
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let ar_0: libc::c_float = alpha.dat[0 as libc::c_int as usize];
        let ai_0: libc::c_float = alpha.dat[1 as libc::c_int as usize];
        let br: libc::c_float = beta.dat[0 as libc::c_int as usize];
        let bi: libc::c_float = beta.dat[1 as libc::c_int as usize];
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            let mut xr_0: libc::c_float = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(x_stride_0) as isize,
                );
            let mut xi_0: libc::c_float = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(x_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut yr: libc::c_float = *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0) as isize,
                );
            let mut yi: libc::c_float = *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0) as isize,
                ) = ar_0 * xr_0 - ai_0 * xi_0 + br * yr - bi * yi;
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ai_0 * xr_0 + ar_0 * xi_0 + bi * yr + br * yi;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_axpby(
    alpha: gsl_complex_long_double,
    mut x: *const gsl_vector_complex_long_double,
    beta: gsl_complex_long_double,
    mut y: *mut gsl_vector_complex_long_double,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta.dat[0 as libc::c_int as usize] == f128::f128::new(0 as libc::c_int)
        && beta.dat[1 as libc::c_int as usize] == f128::f128::new(0 as libc::c_int)
    {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let ar: f128::f128 = alpha.dat[0 as libc::c_int as usize];
        let ai: f128::f128 = alpha.dat[1 as libc::c_int as usize];
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            let mut xr: f128::f128 = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(x_stride) as isize,
                );
            let mut xi: f128::f128 = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(x_stride)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(y_stride) as isize,
                ) = ar * xr - ai * xi;
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(y_stride)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ai * xr + ar * xi;
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let ar_0: f128::f128 = alpha.dat[0 as libc::c_int as usize];
        let ai_0: f128::f128 = alpha.dat[1 as libc::c_int as usize];
        let br: f128::f128 = beta.dat[0 as libc::c_int as usize];
        let bi: f128::f128 = beta.dat[1 as libc::c_int as usize];
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            let mut xr_0: f128::f128 = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(x_stride_0) as isize,
                );
            let mut xi_0: f128::f128 = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(x_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut yr: f128::f128 = *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0) as isize,
                );
            let mut yi: f128::f128 = *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0) as isize,
                ) = ar_0 * xr_0 - ai_0 * xi_0 + br * yr - bi * yi;
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ai_0 * xr_0 + ar_0 * xi_0 + bi * yr + br * yi;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_complex_axpby(
    alpha: gsl_complex,
    mut x: *const gsl_vector_complex,
    beta: gsl_complex,
    mut y: *mut gsl_vector_complex,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_complex_source.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta.dat[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
        && beta.dat[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
    {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let ar: libc::c_double = alpha.dat[0 as libc::c_int as usize];
        let ai: libc::c_double = alpha.dat[1 as libc::c_int as usize];
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            let mut xr: libc::c_double = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(x_stride) as isize,
                );
            let mut xi: libc::c_double = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(x_stride)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(y_stride) as isize,
                ) = ar * xr - ai * xi;
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_mul(y_stride)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ai * xr + ar * xi;
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let ar_0: libc::c_double = alpha.dat[0 as libc::c_int as usize];
        let ai_0: libc::c_double = alpha.dat[1 as libc::c_int as usize];
        let br: libc::c_double = beta.dat[0 as libc::c_int as usize];
        let bi: libc::c_double = beta.dat[1 as libc::c_int as usize];
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            let mut xr_0: libc::c_double = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(x_stride_0) as isize,
                );
            let mut xi_0: libc::c_double = *((*x).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(x_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            let mut yr: libc::c_double = *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0) as isize,
                );
            let mut yi: libc::c_double = *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0) as isize,
                ) = ar_0 * xr_0 - ai_0 * xi_0 + br * yr - bi * yi;
            *((*y).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j_0)
                        .wrapping_mul(y_stride_0)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ai_0 * xr_0 + ar_0 * xi_0 + bi * yr + br * yi;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_char_add(
    mut a: *mut gsl_vector_char,
    mut b: *const gsl_vector_char,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh0 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh0 = (*fresh0 as libc::c_int
                + *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uchar_add(
    mut a: *mut gsl_vector_uchar,
    mut b: *const gsl_vector_uchar,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh1 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh1 = (*fresh1 as libc::c_int
                + *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_float_add(
    mut a: *mut gsl_vector_float,
    mut b: *const gsl_vector_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                += *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_int_add(
    mut a: *mut gsl_vector_int,
    mut b: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                += *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_double_add(
    mut a: *mut gsl_vector_long_double,
    mut b: *const gsl_vector_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                += *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_short_add(
    mut a: *mut gsl_vector_short,
    mut b: *const gsl_vector_short,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh2 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh2 = (*fresh2 as libc::c_int
                + *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_short;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ulong_add(
    mut a: *mut gsl_vector_ulong,
    mut b: *const gsl_vector_ulong,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh3 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh3 = (*fresh3)
                .wrapping_add(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_add(
    mut a: *mut gsl_vector,
    mut b: *const gsl_vector,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                += *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ushort_add(
    mut a: *mut gsl_vector_ushort,
    mut b: *const gsl_vector_ushort,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh4 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh4 = (*fresh4 as libc::c_int
                + *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_ushort;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uint_add(
    mut a: *mut gsl_vector_uint,
    mut b: *const gsl_vector_uint,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh5 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh5 = (*fresh5)
                .wrapping_add(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_add(
    mut a: *mut gsl_vector_long,
    mut b: *const gsl_vector_long,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                += *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ushort_sub(
    mut a: *mut gsl_vector_ushort,
    mut b: *const gsl_vector_ushort,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh6 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh6 = (*fresh6 as libc::c_int
                - *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_ushort;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uint_sub(
    mut a: *mut gsl_vector_uint,
    mut b: *const gsl_vector_uint,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh7 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh7 = (*fresh7)
                .wrapping_sub(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_char_sub(
    mut a: *mut gsl_vector_char,
    mut b: *const gsl_vector_char,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh8 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh8 = (*fresh8 as libc::c_int
                - *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_double_sub(
    mut a: *mut gsl_vector_long_double,
    mut b: *const gsl_vector_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                -= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_int_sub(
    mut a: *mut gsl_vector_int,
    mut b: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                -= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_sub(
    mut a: *mut gsl_vector_long,
    mut b: *const gsl_vector_long,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                -= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_float_sub(
    mut a: *mut gsl_vector_float,
    mut b: *const gsl_vector_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                -= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_short_sub(
    mut a: *mut gsl_vector_short,
    mut b: *const gsl_vector_short,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh9 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh9 = (*fresh9 as libc::c_int
                - *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_short;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uchar_sub(
    mut a: *mut gsl_vector_uchar,
    mut b: *const gsl_vector_uchar,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh10 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh10 = (*fresh10 as libc::c_int
                - *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_sub(
    mut a: *mut gsl_vector,
    mut b: *const gsl_vector,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                -= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ulong_sub(
    mut a: *mut gsl_vector_ulong,
    mut b: *const gsl_vector_ulong,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh11 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh11 = (*fresh11)
                .wrapping_sub(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_short_mul(
    mut a: *mut gsl_vector_short,
    mut b: *const gsl_vector_short,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh12 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh12 = (*fresh12 as libc::c_int
                * *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_short;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_char_mul(
    mut a: *mut gsl_vector_char,
    mut b: *const gsl_vector_char,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh13 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh13 = (*fresh13 as libc::c_int
                * *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_mul(
    mut a: *mut gsl_vector,
    mut b: *const gsl_vector,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                *= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uint_mul(
    mut a: *mut gsl_vector_uint,
    mut b: *const gsl_vector_uint,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh14 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh14 = (*fresh14)
                .wrapping_mul(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_double_mul(
    mut a: *mut gsl_vector_long_double,
    mut b: *const gsl_vector_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                *= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_float_mul(
    mut a: *mut gsl_vector_float,
    mut b: *const gsl_vector_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                *= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_mul(
    mut a: *mut gsl_vector_long,
    mut b: *const gsl_vector_long,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                *= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ulong_mul(
    mut a: *mut gsl_vector_ulong,
    mut b: *const gsl_vector_ulong,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh15 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh15 = (*fresh15)
                .wrapping_mul(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ushort_mul(
    mut a: *mut gsl_vector_ushort,
    mut b: *const gsl_vector_ushort,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh16 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh16 = (*fresh16 as libc::c_int
                * *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_ushort;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uchar_mul(
    mut a: *mut gsl_vector_uchar,
    mut b: *const gsl_vector_uchar,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh17 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh17 = (*fresh17 as libc::c_int
                * *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_int_mul(
    mut a: *mut gsl_vector_int,
    mut b: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                *= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_div(
    mut a: *mut gsl_vector_long,
    mut b: *const gsl_vector_long,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                /= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ushort_div(
    mut a: *mut gsl_vector_ushort,
    mut b: *const gsl_vector_ushort,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh18 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh18 = (*fresh18 as libc::c_int
                / *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_ushort;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_float_div(
    mut a: *mut gsl_vector_float,
    mut b: *const gsl_vector_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                /= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ulong_div(
    mut a: *mut gsl_vector_ulong,
    mut b: *const gsl_vector_ulong,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh19 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh19 = (*fresh19)
                .wrapping_div(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_div(
    mut a: *mut gsl_vector,
    mut b: *const gsl_vector,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                /= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_int_div(
    mut a: *mut gsl_vector_int,
    mut b: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                /= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_short_div(
    mut a: *mut gsl_vector_short,
    mut b: *const gsl_vector_short,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh20 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh20 = (*fresh20 as libc::c_int
                / *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_short;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uint_div(
    mut a: *mut gsl_vector_uint,
    mut b: *const gsl_vector_uint,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh21 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh21 = (*fresh21)
                .wrapping_div(*((*b).data).offset(i.wrapping_mul(stride_b) as isize));
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_double_div(
    mut a: *mut gsl_vector_long_double,
    mut b: *const gsl_vector_long_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            *((*a).data).offset(i.wrapping_mul(stride_a) as isize)
                /= *((*b).data).offset(i.wrapping_mul(stride_b) as isize);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uchar_div(
    mut a: *mut gsl_vector_uchar,
    mut b: *const gsl_vector_uchar,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh22 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh22 = (*fresh22 as libc::c_int
                / *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_uchar;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_char_div(
    mut a: *mut gsl_vector_char,
    mut b: *const gsl_vector_char,
) -> libc::c_int {
    let N: size_t = (*a).size;
    if (*b).size != N {
        gsl_error(
            b"vectors must have same length\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let stride_a: size_t = (*a).stride;
        let stride_b: size_t = (*b).stride;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let ref mut fresh23 = *((*a).data).offset(i.wrapping_mul(stride_a) as isize);
            *fresh23 = (*fresh23 as libc::c_int
                / *((*b).data).offset(i.wrapping_mul(stride_b) as isize) as libc::c_int)
                as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_float_scale(
    mut a: *mut gsl_vector_float,
    x: libc::c_float,
) -> libc::c_int {
    gsl_blas_sscal(x, a);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_scale(
    mut a: *mut gsl_vector_uint,
    x: libc::c_uint,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh24 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh24 = (*fresh24).wrapping_mul(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_scale(
    mut a: *mut gsl_vector_char,
    x: libc::c_char,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh25 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh25 = (*fresh25 as libc::c_int * x as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_scale(
    mut a: *mut gsl_vector_ulong,
    x: libc::c_ulong,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh26 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh26 = (*fresh26).wrapping_mul(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_scale(
    mut a: *mut gsl_vector_uchar,
    x: libc::c_uchar,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh27 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh27 = (*fresh27 as libc::c_int * x as libc::c_int) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_scale(
    mut a: *mut gsl_vector_long_double,
    x: f128::f128,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_scale(
    mut a: *mut gsl_vector_int,
    x: libc::c_int,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_scale(
    mut a: *mut gsl_vector,
    x: libc::c_double,
) -> libc::c_int {
    gsl_blas_dscal(x, a);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_scale(
    mut a: *mut gsl_vector_ushort,
    x: libc::c_ushort,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh28 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh28 = (*fresh28 as libc::c_int * x as libc::c_int) as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_scale(
    mut a: *mut gsl_vector_long,
    x: libc::c_long,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) *= x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_scale(
    mut a: *mut gsl_vector_short,
    x: libc::c_short,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh29 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh29 = (*fresh29 as libc::c_int * x as libc::c_int) as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_add_constant(
    mut a: *mut gsl_vector_short,
    x: libc::c_short,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh30 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh30 = (*fresh30 as libc::c_int + x as libc::c_int) as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_add_constant(
    mut a: *mut gsl_vector_long,
    x: libc::c_long,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_add_constant(
    mut a: *mut gsl_vector_char,
    x: libc::c_char,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh31 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh31 = (*fresh31 as libc::c_int + x as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_add_constant(
    mut a: *mut gsl_vector_ushort,
    x: libc::c_ushort,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh32 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh32 = (*fresh32 as libc::c_int + x as libc::c_int) as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_add_constant(
    mut a: *mut gsl_vector_ulong,
    x: libc::c_ulong,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh33 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh33 = (*fresh33).wrapping_add(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_add_constant(
    mut a: *mut gsl_vector,
    x: libc::c_double,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_add_constant(
    mut a: *mut gsl_vector_uint,
    x: libc::c_uint,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh34 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh34 = (*fresh34).wrapping_add(x);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_add_constant(
    mut a: *mut gsl_vector_int,
    x: libc::c_int,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_add_constant(
    mut a: *mut gsl_vector_long_double,
    x: f128::f128,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_add_constant(
    mut a: *mut gsl_vector_float,
    x: libc::c_float,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        *((*a).data).offset(i.wrapping_mul(stride) as isize) += x;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_add_constant(
    mut a: *mut gsl_vector_uchar,
    x: libc::c_uchar,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let ref mut fresh35 = *((*a).data).offset(i.wrapping_mul(stride) as isize);
        *fresh35 = (*fresh35 as libc::c_int + x as libc::c_int) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_axpby(
    alpha: libc::c_ushort,
    mut x: *const gsl_vector_ushort,
    beta: libc::c_ushort,
    mut y: *mut gsl_vector_ushort,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta as libc::c_int == 0 as libc::c_int as libc::c_ushort as libc::c_int {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride.wrapping_mul(j) as isize) as libc::c_int)
                as libc::c_ushort;
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                    as libc::c_int
                + beta as libc::c_int
                    * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize)
                        as libc::c_int) as libc::c_ushort;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_double_axpby(
    alpha: f128::f128,
    mut x: *const gsl_vector_long_double,
    beta: f128::f128,
    mut y: *mut gsl_vector_long_double,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta == f128::f128::new(0 as libc::c_int) {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = alpha * *((*x).data).offset(x_stride.wrapping_mul(j) as isize);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = alpha * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                + beta * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize);
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uchar_axpby(
    alpha: libc::c_uchar,
    mut x: *const gsl_vector_uchar,
    beta: libc::c_uchar,
    mut y: *mut gsl_vector_uchar,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta as libc::c_int == 0 as libc::c_int as libc::c_uchar as libc::c_int {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride.wrapping_mul(j) as isize) as libc::c_int)
                as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                    as libc::c_int
                + beta as libc::c_int
                    * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize)
                        as libc::c_int) as libc::c_uchar;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_float_axpby(
    alpha: libc::c_float,
    mut x: *const gsl_vector_float,
    beta: libc::c_float,
    mut y: *mut gsl_vector_float,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta == 0 as libc::c_int as libc::c_float {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = alpha * *((*x).data).offset(x_stride.wrapping_mul(j) as isize);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = alpha * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                + beta * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize);
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_char_axpby(
    alpha: libc::c_char,
    mut x: *const gsl_vector_char,
    beta: libc::c_char,
    mut y: *mut gsl_vector_char,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta as libc::c_int == 0 as libc::c_int as libc::c_char as libc::c_int {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride.wrapping_mul(j) as isize) as libc::c_int)
                as libc::c_char;
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                    as libc::c_int
                + beta as libc::c_int
                    * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize)
                        as libc::c_int) as libc::c_char;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_int_axpby(
    alpha: libc::c_int,
    mut x: *const gsl_vector_int,
    beta: libc::c_int,
    mut y: *mut gsl_vector_int,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta == 0 as libc::c_int {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = alpha * *((*x).data).offset(x_stride.wrapping_mul(j) as isize);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = alpha * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                + beta * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize);
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ulong_axpby(
    alpha: libc::c_ulong,
    mut x: *const gsl_vector_ulong,
    beta: libc::c_ulong,
    mut y: *mut gsl_vector_ulong,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta == 0 as libc::c_int as libc::c_ulong {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = alpha
                .wrapping_mul(*((*x).data).offset(x_stride.wrapping_mul(j) as isize));
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = alpha
                .wrapping_mul(*((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize))
                .wrapping_add(
                    beta
                        .wrapping_mul(
                            *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize),
                        ),
                );
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_uint_axpby(
    alpha: libc::c_uint,
    mut x: *const gsl_vector_uint,
    beta: libc::c_uint,
    mut y: *mut gsl_vector_uint,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta == 0 as libc::c_int as libc::c_uint {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = alpha
                .wrapping_mul(*((*x).data).offset(x_stride.wrapping_mul(j) as isize));
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = alpha
                .wrapping_mul(*((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize))
                .wrapping_add(
                    beta
                        .wrapping_mul(
                            *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize),
                        ),
                );
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_axpby(
    alpha: libc::c_double,
    mut x: *const gsl_vector,
    beta: libc::c_double,
    mut y: *mut gsl_vector,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta == 0 as libc::c_int as libc::c_double {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = alpha * *((*x).data).offset(x_stride.wrapping_mul(j) as isize);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = alpha * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                + beta * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize);
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_short_axpby(
    alpha: libc::c_short,
    mut x: *const gsl_vector_short,
    beta: libc::c_short,
    mut y: *mut gsl_vector_short,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta as libc::c_int == 0 as libc::c_int as libc::c_short as libc::c_int {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride.wrapping_mul(j) as isize) as libc::c_int)
                as libc::c_short;
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = (alpha as libc::c_int
                * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                    as libc::c_int
                + beta as libc::c_int
                    * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize)
                        as libc::c_int) as libc::c_short;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_long_axpby(
    alpha: libc::c_long,
    mut x: *const gsl_vector_long,
    beta: libc::c_long,
    mut y: *mut gsl_vector_long,
) -> libc::c_int {
    let x_size: size_t = (*x).size;
    if x_size != (*y).size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./oper_source.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if beta == 0 as libc::c_int as libc::c_long {
        let x_stride: size_t = (*x).stride;
        let y_stride: size_t = (*y).stride;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < x_size {
            *((*y).data)
                .offset(
                    y_stride.wrapping_mul(j) as isize,
                ) = alpha * *((*x).data).offset(x_stride.wrapping_mul(j) as isize);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_stride_0: size_t = (*x).stride;
        let y_stride_0: size_t = (*y).stride;
        let mut j_0: size_t = 0;
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < x_size {
            *((*y).data)
                .offset(
                    y_stride_0.wrapping_mul(j_0) as isize,
                ) = alpha * *((*x).data).offset(x_stride_0.wrapping_mul(j_0) as isize)
                + beta * *((*y).data).offset(y_stride_0.wrapping_mul(j_0) as isize);
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_vector_ushort_sum(
    mut a: *const gsl_vector_ushort,
) -> libc::c_ushort {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum = (sum as libc::c_int
            + *((*a).data).offset(i.wrapping_mul(stride) as isize) as libc::c_int)
            as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_ulong_sum(
    mut a: *const gsl_vector_ulong,
) -> libc::c_ulong {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum = sum.wrapping_add(*((*a).data).offset(i.wrapping_mul(stride) as isize));
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_long_sum(
    mut a: *const gsl_vector_long,
) -> libc::c_long {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum += *((*a).data).offset(i.wrapping_mul(stride) as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_short_sum(
    mut a: *const gsl_vector_short,
) -> libc::c_short {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum = (sum as libc::c_int
            + *((*a).data).offset(i.wrapping_mul(stride) as isize) as libc::c_int)
            as libc::c_short;
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_long_double_sum(
    mut a: *const gsl_vector_long_double,
) -> f128::f128 {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum += *((*a).data).offset(i.wrapping_mul(stride) as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_char_sum(
    mut a: *const gsl_vector_char,
) -> libc::c_char {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum = (sum as libc::c_int
            + *((*a).data).offset(i.wrapping_mul(stride) as isize) as libc::c_int)
            as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_int_sum(
    mut a: *const gsl_vector_int,
) -> libc::c_int {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum += *((*a).data).offset(i.wrapping_mul(stride) as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_uint_sum(
    mut a: *const gsl_vector_uint,
) -> libc::c_uint {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum = sum.wrapping_add(*((*a).data).offset(i.wrapping_mul(stride) as isize));
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_sum(mut a: *const gsl_vector) -> libc::c_double {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum += *((*a).data).offset(i.wrapping_mul(stride) as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_float_sum(
    mut a: *const gsl_vector_float,
) -> libc::c_float {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum += *((*a).data).offset(i.wrapping_mul(stride) as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub unsafe extern "C" fn gsl_vector_uchar_sum(
    mut a: *const gsl_vector_uchar,
) -> libc::c_uchar {
    let N: size_t = (*a).size;
    let stride: size_t = (*a).stride;
    let mut sum: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        sum = (sum as libc::c_int
            + *((*a).data).offset(i.wrapping_mul(stride) as isize) as libc::c_int)
            as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
