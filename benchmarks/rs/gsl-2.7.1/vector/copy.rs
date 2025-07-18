use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_blas_dcopy(X: *const gsl_vector, Y: *mut gsl_vector) -> libc::c_int;
    fn gsl_blas_zcopy(
        X: *const gsl_vector_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_scopy(
        X: *const gsl_vector_float,
        Y: *mut gsl_vector_float,
    ) -> libc::c_int;
    fn gsl_blas_ccopy(
        X: *const gsl_vector_complex_float,
        Y: *mut gsl_vector_complex_float,
    ) -> libc::c_int;
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
pub unsafe extern "C" fn gsl_vector_memcpy(
    mut dest: *mut gsl_vector,
    mut src: *const gsl_vector,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_blas_dcopy(src, dest);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_int_memcpy(
    mut dest: *mut gsl_vector_int,
    mut src: *const gsl_vector_int,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_memcpy(
    mut dest: *mut gsl_vector_complex_long_double,
    mut src: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_short_memcpy(
    mut dest: *mut gsl_vector_short,
    mut src: *const gsl_vector_short,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_memcpy(
    mut dest: *mut gsl_vector_long,
    mut src: *const gsl_vector_long,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_memcpy(
    mut dest: *mut gsl_vector_complex,
    mut src: *const gsl_vector_complex,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_blas_zcopy(src, dest);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_float_memcpy(
    mut dest: *mut gsl_vector_float,
    mut src: *const gsl_vector_float,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_blas_scopy(src, dest);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_complex_float_memcpy(
    mut dest: *mut gsl_vector_complex_float,
    mut src: *const gsl_vector_complex_float,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_blas_ccopy(src, dest);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ulong_memcpy(
    mut dest: *mut gsl_vector_ulong,
    mut src: *const gsl_vector_ulong,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_char_memcpy(
    mut dest: *mut gsl_vector_char,
    mut src: *const gsl_vector_char,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_long_double_memcpy(
    mut dest: *mut gsl_vector_long_double,
    mut src: *const gsl_vector_long_double,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_ushort_memcpy(
    mut dest: *mut gsl_vector_ushort,
    mut src: *const gsl_vector_ushort,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uint_memcpy(
    mut dest: *mut gsl_vector_uint,
    mut src: *const gsl_vector_uint,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_vector_uchar_memcpy(
    mut dest: *mut gsl_vector_uchar,
    mut src: *const gsl_vector_uchar,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"vector lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_stride: size_t = (*src).stride;
    let dest_stride: size_t = (*dest).stride;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_stride)
                        .wrapping_mul(j)
                        .wrapping_add(k) as isize,
                );
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
