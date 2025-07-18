use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_complex_row(
        m: *mut gsl_matrix_complex,
        i: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_diagonal(
        m: *mut gsl_matrix_complex,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_subrow(
        m: *mut gsl_matrix_complex,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_const_row(
        m: *const gsl_matrix_complex,
        i: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_const_diagonal(
        m: *const gsl_matrix_complex,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_const_subrow(
        m: *const gsl_matrix_complex,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_matrix_complex_float_row(
        m: *mut gsl_matrix_complex_float,
        i: size_t,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_complex_float_diagonal(
        m: *mut gsl_matrix_complex_float,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_complex_float_subrow(
        m: *mut gsl_matrix_complex_float,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_complex_float_const_row(
        m: *const gsl_matrix_complex_float,
        i: size_t,
    ) -> _gsl_vector_complex_float_const_view;
    fn gsl_matrix_complex_float_const_diagonal(
        m: *const gsl_matrix_complex_float,
    ) -> _gsl_vector_complex_float_const_view;
    fn gsl_matrix_complex_float_const_subrow(
        m: *const gsl_matrix_complex_float,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_float_const_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
    fn gsl_matrix_subrow(
        m: *mut gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_const_row(m: *const gsl_matrix, i: size_t) -> _gsl_vector_const_view;
    fn gsl_matrix_const_diagonal(m: *const gsl_matrix) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subrow(
        m: *const gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_float_row(
        m: *mut gsl_matrix_float,
        i: size_t,
    ) -> _gsl_vector_float_view;
    fn gsl_matrix_float_diagonal(m: *mut gsl_matrix_float) -> _gsl_vector_float_view;
    fn gsl_matrix_float_subrow(
        m: *mut gsl_matrix_float,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_float_view;
    fn gsl_matrix_float_const_row(
        m: *const gsl_matrix_float,
        i: size_t,
    ) -> _gsl_vector_float_const_view;
    fn gsl_matrix_float_const_diagonal(
        m: *const gsl_matrix_float,
    ) -> _gsl_vector_float_const_view;
    fn gsl_matrix_float_const_subrow(
        m: *const gsl_matrix_float,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_float_const_view;
    fn gsl_blas_sswap(X: *mut gsl_vector_float, Y: *mut gsl_vector_float) -> libc::c_int;
    fn gsl_blas_scopy(
        X: *const gsl_vector_float,
        Y: *mut gsl_vector_float,
    ) -> libc::c_int;
    fn gsl_blas_dswap(X: *mut gsl_vector, Y: *mut gsl_vector) -> libc::c_int;
    fn gsl_blas_dcopy(X: *const gsl_vector, Y: *mut gsl_vector) -> libc::c_int;
    fn gsl_blas_cswap(
        X: *mut gsl_vector_complex_float,
        Y: *mut gsl_vector_complex_float,
    ) -> libc::c_int;
    fn gsl_blas_ccopy(
        X: *const gsl_vector_complex_float,
        Y: *mut gsl_vector_complex_float,
    ) -> libc::c_int;
    fn gsl_blas_zswap(
        X: *mut gsl_vector_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zcopy(
        X: *const gsl_vector_complex,
        Y: *mut gsl_vector_complex,
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
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
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
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
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
pub struct _gsl_vector_complex_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_view = _gsl_vector_complex_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_const_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_const_view = _gsl_vector_complex_const_view;
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
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_float_view {
    pub vector: gsl_vector_float,
}
pub type gsl_vector_float_view = _gsl_vector_float_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_float_const_view {
    pub vector: gsl_vector_float,
}
pub type gsl_vector_float_const_view = _gsl_vector_float_const_view;
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
pub struct _gsl_vector_complex_float_view {
    pub vector: gsl_vector_complex_float,
}
pub type gsl_vector_complex_float_view = _gsl_vector_complex_float_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_float_const_view {
    pub vector: gsl_vector_complex_float,
}
pub type gsl_vector_complex_float_const_view = _gsl_vector_complex_float_const_view;
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
pub unsafe extern "C" fn gsl_matrix_ulong_memcpy(
    mut dest: *mut gsl_matrix_ulong,
    mut src: *const gsl_matrix_ulong,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_memcpy(
    mut dest: *mut gsl_matrix_float,
    mut src: *const gsl_matrix_float,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let sv: gsl_vector_float_const_view = gsl_matrix_float_const_row(src, i);
        let mut dv: gsl_vector_float_view = gsl_matrix_float_row(dest, i);
        gsl_blas_scopy(&sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_memcpy(
    mut dest: *mut gsl_matrix_complex_long_double,
    mut src: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (2 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_memcpy(
    mut dest: *mut gsl_matrix_int,
    mut src: *const gsl_matrix_int,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_memcpy(
    mut dest: *mut gsl_matrix_complex,
    mut src: *const gsl_matrix_complex,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let sv: gsl_vector_complex_const_view = gsl_matrix_complex_const_row(src, i);
        let mut dv: gsl_vector_complex_view = gsl_matrix_complex_row(dest, i);
        gsl_blas_zcopy(&sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_memcpy(
    mut dest: *mut gsl_matrix_char,
    mut src: *const gsl_matrix_char,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_memcpy(
    mut dest: *mut gsl_matrix_uint,
    mut src: *const gsl_matrix_uint,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_memcpy(
    mut dest: *mut gsl_matrix_complex_float,
    mut src: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let sv: gsl_vector_complex_float_const_view = gsl_matrix_complex_float_const_row(
            src,
            i,
        );
        let mut dv: gsl_vector_complex_float_view = gsl_matrix_complex_float_row(
            dest,
            i,
        );
        gsl_blas_ccopy(&sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_memcpy(
    mut dest: *mut gsl_matrix_long_double,
    mut src: *const gsl_matrix_long_double,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_memcpy(
    mut dest: *mut gsl_matrix_long,
    mut src: *const gsl_matrix_long,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_memcpy(
    mut dest: *mut gsl_matrix,
    mut src: *const gsl_matrix,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let sv: gsl_vector_const_view = gsl_matrix_const_row(src, i);
        let mut dv: gsl_vector_view = gsl_matrix_row(dest, i);
        gsl_blas_dcopy(&sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_memcpy(
    mut dest: *mut gsl_matrix_ushort,
    mut src: *const gsl_matrix_ushort,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_memcpy(
    mut dest: *mut gsl_matrix_uchar,
    mut src: *const gsl_matrix_uchar,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_memcpy(
    mut dest: *mut gsl_matrix_short,
    mut src: *const gsl_matrix_short,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_swap(
    mut dest: *mut gsl_matrix,
    mut src: *mut gsl_matrix,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let mut sv: gsl_vector_view = gsl_matrix_row(src, i);
        let mut dv: gsl_vector_view = gsl_matrix_row(dest, i);
        gsl_blas_dswap(&mut sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_swap(
    mut dest: *mut gsl_matrix_uchar,
    mut src: *mut gsl_matrix_uchar,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_uchar = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_swap(
    mut dest: *mut gsl_matrix_int,
    mut src: *mut gsl_matrix_int,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_int = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_swap(
    mut dest: *mut gsl_matrix_short,
    mut src: *mut gsl_matrix_short,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_short = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_swap(
    mut dest: *mut gsl_matrix_float,
    mut src: *mut gsl_matrix_float,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let mut sv: gsl_vector_float_view = gsl_matrix_float_row(src, i);
        let mut dv: gsl_vector_float_view = gsl_matrix_float_row(dest, i);
        gsl_blas_sswap(&mut sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_swap(
    mut dest: *mut gsl_matrix_complex_long_double,
    mut src: *mut gsl_matrix_complex_long_double,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (2 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: f128::f128 = *((*src).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_swap(
    mut dest: *mut gsl_matrix_uint,
    mut src: *mut gsl_matrix_uint,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_uint = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_swap(
    mut dest: *mut gsl_matrix_ushort,
    mut src: *mut gsl_matrix_ushort,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_ushort = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_swap(
    mut dest: *mut gsl_matrix_complex,
    mut src: *mut gsl_matrix_complex,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let mut sv: gsl_vector_complex_view = gsl_matrix_complex_row(src, i);
        let mut dv: gsl_vector_complex_view = gsl_matrix_complex_row(dest, i);
        gsl_blas_zswap(&mut sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_swap(
    mut dest: *mut gsl_matrix_ulong,
    mut src: *mut gsl_matrix_ulong,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_ulong = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_swap(
    mut dest: *mut gsl_matrix_long,
    mut src: *mut gsl_matrix_long,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_long = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_swap(
    mut dest: *mut gsl_matrix_long_double,
    mut src: *mut gsl_matrix_long_double,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: f128::f128 = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_swap(
    mut dest: *mut gsl_matrix_complex_float,
    mut src: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let mut sv: gsl_vector_complex_float_view = gsl_matrix_complex_float_row(src, i);
        let mut dv: gsl_vector_complex_float_view = gsl_matrix_complex_float_row(
            dest,
            i,
        );
        gsl_blas_cswap(&mut sv.vector, &mut dv.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_swap(
    mut dest: *mut gsl_matrix_char,
    mut src: *mut gsl_matrix_char,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if src_size1 != dest_size1 || src_size2 != dest_size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        j = 0 as libc::c_int as size_t;
        while j < (1 as libc::c_int as libc::c_ulong).wrapping_mul(src_size2) {
            let mut tmp: libc::c_char = *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*src).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(src_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                );
            *((*dest).data)
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(dest_tda)
                        .wrapping_mul(i)
                        .wrapping_add(j) as isize,
                ) = tmp;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_ulong,
    mut src: *const gsl_matrix_ulong,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_complex_float,
    mut src: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            let mut k: size_t = if i < N { i } else { N };
            let a: gsl_vector_complex_float_const_view = gsl_matrix_complex_float_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            let mut b: gsl_vector_complex_float_view = gsl_matrix_complex_float_subrow(
                dest,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            gsl_blas_ccopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i
            < (if M < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                M
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
        {
            let a_0: gsl_vector_complex_float_const_view = gsl_matrix_complex_float_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_complex_float_view = gsl_matrix_complex_float_subrow(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_ccopy(&a_0.vector, &mut b_0.vector);
            i = i.wrapping_add(1);
            i;
        }
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        let a_1: gsl_vector_complex_float_const_view = gsl_matrix_complex_float_const_diagonal(
            src,
        );
        let mut b_1: gsl_vector_complex_float_view = gsl_matrix_complex_float_diagonal(
            dest,
        );
        gsl_blas_ccopy(&a_1.vector, &mut b_1.vector);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_float,
    mut src: *const gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            let mut k: size_t = if i < N { i } else { N };
            let a: gsl_vector_float_const_view = gsl_matrix_float_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            let mut b: gsl_vector_float_view = gsl_matrix_float_subrow(
                dest,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            gsl_blas_scopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i
            < (if M < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                M
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
        {
            let a_0: gsl_vector_float_const_view = gsl_matrix_float_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_float_view = gsl_matrix_float_subrow(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_scopy(&a_0.vector, &mut b_0.vector);
            i = i.wrapping_add(1);
            i;
        }
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        let a_1: gsl_vector_float_const_view = gsl_matrix_float_const_diagonal(src);
        let mut b_1: gsl_vector_float_view = gsl_matrix_float_diagonal(dest);
        gsl_blas_scopy(&a_1.vector, &mut b_1.vector);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_uchar,
    mut src: *const gsl_matrix_uchar,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_int,
    mut src: *const gsl_matrix_int,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_complex,
    mut src: *const gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            let mut k: size_t = if i < N { i } else { N };
            let a: gsl_vector_complex_const_view = gsl_matrix_complex_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            let mut b: gsl_vector_complex_view = gsl_matrix_complex_subrow(
                dest,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            gsl_blas_zcopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i
            < (if M < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                M
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
        {
            let a_0: gsl_vector_complex_const_view = gsl_matrix_complex_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_complex_view = gsl_matrix_complex_subrow(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_zcopy(&a_0.vector, &mut b_0.vector);
            i = i.wrapping_add(1);
            i;
        }
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        let a_1: gsl_vector_complex_const_view = gsl_matrix_complex_const_diagonal(src);
        let mut b_1: gsl_vector_complex_view = gsl_matrix_complex_diagonal(dest);
        gsl_blas_zcopy(&a_1.vector, &mut b_1.vector);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_long,
    mut src: *const gsl_matrix_long,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_long_double,
    mut src: *const gsl_matrix_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_char,
    mut src: *const gsl_matrix_char,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_ushort,
    mut src: *const gsl_matrix_ushort,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_complex_long_double,
    mut src: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 2 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 2 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix,
    mut src: *const gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            let mut k: size_t = if i < N { i } else { N };
            let a: gsl_vector_const_view = gsl_matrix_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            let mut b: gsl_vector_view = gsl_matrix_subrow(
                dest,
                i,
                0 as libc::c_int as size_t,
                k,
            );
            gsl_blas_dcopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i
            < (if M < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                M
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
        {
            let a_0: gsl_vector_const_view = gsl_matrix_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_view = gsl_matrix_subrow(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_dcopy(&a_0.vector, &mut b_0.vector);
            i = i.wrapping_add(1);
            i;
        }
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        let a_1: gsl_vector_const_view = gsl_matrix_const_diagonal(src);
        let mut b_1: gsl_vector_view = gsl_matrix_diagonal(dest);
        gsl_blas_dcopy(&a_1.vector, &mut b_1.vector);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_short,
    mut src: *const gsl_matrix_short,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_tricpy(
    mut Uplo: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_uint,
    mut src: *const gsl_matrix_uint,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let mut i: size_t = 0;
    if M != (*dest).size1 || N != (*dest).size2 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < M {
            j = 0 as libc::c_int as size_t;
            while j < (if i < N { i } else { N }) {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < M {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = i
                        .wrapping_mul(dest_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    let mut e2_0: size_t = i
                        .wrapping_mul(src_tda)
                        .wrapping_add(j)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(k);
                    *((*dest).data)
                        .offset(e1_0 as isize) = *((*src).data).offset(e2_0 as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        gsl_error(
            b"invalid Uplo parameter\0" as *const u8 as *const libc::c_char,
            b"./copy_source.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1_1: size_t = i
                    .wrapping_mul(dest_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2_1: size_t = i
                    .wrapping_mul(src_tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data)
                    .offset(e1_1 as isize) = *((*src).data).offset(e2_1 as isize);
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
