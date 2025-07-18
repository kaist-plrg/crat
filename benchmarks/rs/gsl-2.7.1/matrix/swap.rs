use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_complex_column(
        m: *mut gsl_matrix_complex,
        j: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_diagonal(
        m: *mut gsl_matrix_complex,
    ) -> _gsl_vector_complex_view;
    fn gsl_matrix_complex_subcolumn(
        m: *mut gsl_matrix_complex,
        j: size_t,
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
    fn gsl_matrix_complex_float_column(
        m: *mut gsl_matrix_complex_float,
        j: size_t,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_complex_float_diagonal(
        m: *mut gsl_matrix_complex_float,
    ) -> _gsl_vector_complex_float_view;
    fn gsl_matrix_complex_float_subcolumn(
        m: *mut gsl_matrix_complex_float,
        j: size_t,
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
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
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
    fn gsl_matrix_float_column(
        m: *mut gsl_matrix_float,
        j: size_t,
    ) -> _gsl_vector_float_view;
    fn gsl_matrix_float_diagonal(m: *mut gsl_matrix_float) -> _gsl_vector_float_view;
    fn gsl_matrix_float_subcolumn(
        m: *mut gsl_matrix_float,
        j: size_t,
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
    fn gsl_blas_scopy(
        X: *const gsl_vector_float,
        Y: *mut gsl_vector_float,
    ) -> libc::c_int;
    fn gsl_blas_dcopy(X: *const gsl_vector, Y: *mut gsl_vector) -> libc::c_int;
    fn gsl_blas_ccopy(
        X: *const gsl_vector_complex_float,
        Y: *mut gsl_vector_complex_float,
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
pub unsafe extern "C" fn gsl_matrix_int_swap_rows(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_int = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_int = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_int = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_swap_rows(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_uint = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_uint = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_uint = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_swap_rows(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut f128::f128 = ((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut f128::f128 = ((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (2 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: f128::f128 = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_swap_rows(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut f128::f128 = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut f128::f128 = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: f128::f128 = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_swap_rows(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_short = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_short = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_short = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_swap_rows(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_long = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_long = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_long = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_swap_rows(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_double = ((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_double = ((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (2 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_double = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_swap_rows(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_double = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_double = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_double = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_swap_rows(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_float = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_float = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_float = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_swap_rows(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_uchar = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_uchar = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_uchar = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_swap_rows(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_ushort = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_ushort = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_ushort = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_swap_rows(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_char = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_char = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_char = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_swap_rows(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_float = ((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_float = ((*m).data)
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (2 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_float = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_swap_rows(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size1 {
        gsl_error(
            b"first row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size1 {
        gsl_error(
            b"second row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut row1: *mut libc::c_ulong = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut row2: *mut libc::c_ulong = ((*m).data)
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_mul((*m).tda) as isize,
            );
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < (1 as libc::c_int as libc::c_ulong).wrapping_mul(size2) {
            let mut tmp: libc::c_ulong = *row1.offset(k as isize);
            *row1.offset(k as isize) = *row2.offset(k as isize);
            *row2.offset(k as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_swap_columns(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_float = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_float = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_float = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_swap_columns(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_uint = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_uint = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_uint = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_swap_columns(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_uchar = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_uchar = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_uchar = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_swap_columns(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_char = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_char = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_char = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_swap_columns(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut f128::f128 = ((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut f128::f128 = ((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut tmp: f128::f128 = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_swap_columns(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_float = ((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_float = ((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_float = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_swap_columns(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_int = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_int = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_int = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_swap_columns(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_short = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_short = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_short = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_swap_columns(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_long = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_long = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_long = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_swap_columns(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_double = ((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_double = ((*m).data)
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_double = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_swap_columns(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_double = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_double = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_double = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_swap_columns(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut f128::f128 = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut f128::f128 = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: f128::f128 = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_swap_columns(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_ulong = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_ulong = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_ulong = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_swap_columns(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if i >= size2 {
        gsl_error(
            b"first column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"second column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut col1: *mut libc::c_ushort = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize);
        let mut col2: *mut libc::c_ushort = ((*m).data)
            .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
        let mut p: size_t = 0;
        p = 0 as libc::c_int as size_t;
        while p < size1 {
            let mut k: size_t = 0;
            let mut n: size_t = p
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*m).tda);
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut tmp: libc::c_ushort = *col1.offset(n.wrapping_add(k) as isize);
                *col1
                    .offset(
                        n.wrapping_add(k) as isize,
                    ) = *col2.offset(n.wrapping_add(k) as isize);
                *col2.offset(n.wrapping_add(k) as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            p = p.wrapping_add(1);
            p;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_swap_rowcol(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_double = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_double = ((*m).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_swap_rowcol(
    mut m: *mut gsl_matrix_char,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_char = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_char = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_char = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_swap_rowcol(
    mut m: *mut gsl_matrix_ushort,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_ushort = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_ushort = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ushort = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_swap_rowcol(
    mut m: *mut gsl_matrix_ulong,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_ulong = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_ulong = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_ulong = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_swap_rowcol(
    mut m: *mut gsl_matrix_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut f128::f128 = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut f128::f128 = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_swap_rowcol(
    mut m: *mut gsl_matrix_complex_long_double,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut f128::f128 = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut f128::f128 = ((*m).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: f128::f128 = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_swap_rowcol(
    mut m: *mut gsl_matrix_long,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_long = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_long = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_long = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_swap_rowcol(
    mut m: *mut gsl_matrix_complex_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_float = ((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_float = ((*m).data)
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 2 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_swap_rowcol(
    mut m: *mut gsl_matrix_float,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_float = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_float = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_float = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_swap_rowcol(
    mut m: *mut gsl_matrix_uint,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_uint = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_uint = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uint = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_swap_rowcol(
    mut m: *mut gsl_matrix_uchar,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_uchar = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_uchar = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_uchar = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_swap_rowcol(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_double = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_double = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_double = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_swap_rowcol(
    mut m: *mut gsl_matrix_int,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_int = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_int = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_int = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_swap_rowcol(
    mut m: *mut gsl_matrix_short,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to swap row and column\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if i >= size1 {
        gsl_error(
            b"row index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size2 {
        gsl_error(
            b"column index is out of range\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    let mut row: *mut libc::c_short = ((*m).data)
        .offset(
            (1 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*m).tda)
                as isize,
        );
    let mut col: *mut libc::c_short = ((*m).data)
        .offset((1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize);
    let mut p: size_t = 0;
    p = 0 as libc::c_int as size_t;
    while p < size1 {
        let mut k: size_t = 0;
        let mut r: size_t = p.wrapping_mul(1 as libc::c_int as libc::c_ulong);
        let mut c: size_t = p
            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*m).tda);
        k = 0 as libc::c_int as size_t;
        while k < 1 as libc::c_int as libc::c_ulong {
            let mut tmp: libc::c_short = *col.offset(c.wrapping_add(k) as isize);
            *col
                .offset(
                    c.wrapping_add(k) as isize,
                ) = *row.offset(r.wrapping_add(k) as isize);
            *row.offset(r.wrapping_add(k) as isize) = tmp;
            k = k.wrapping_add(1);
            k;
        }
        p = p.wrapping_add(1);
        p;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_transpose(mut m: *mut gsl_matrix) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_double = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_transpose(
    mut m: *mut gsl_matrix_char,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_char = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_transpose(
    mut m: *mut gsl_matrix_ushort,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_ushort = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_transpose(
    mut m: *mut gsl_matrix_complex_long_double,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: f128::f128 = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_transpose(
    mut m: *mut gsl_matrix_int,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_int = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_transpose(
    mut m: *mut gsl_matrix_float,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_float = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_transpose(
    mut m: *mut gsl_matrix_complex,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_double = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_transpose(
    mut m: *mut gsl_matrix_uchar,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_uchar = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_transpose(
    mut m: *mut gsl_matrix_long_double,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: f128::f128 = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_transpose(
    mut m: *mut gsl_matrix_uint,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_uint = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_transpose(
    mut m: *mut gsl_matrix_short,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_short = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_transpose(
    mut m: *mut gsl_matrix_ulong,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_ulong = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_transpose(
    mut m: *mut gsl_matrix_long,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_long = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_transpose(
    mut m: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"matrix must be square to take transpose\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size1 {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size2 {
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*m).tda)
                    .wrapping_add(j)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*m).tda)
                    .wrapping_add(i)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut tmp: libc::c_float = *((*m).data).offset(e1 as isize);
                *((*m).data).offset(e1 as isize) = *((*m).data).offset(e2 as isize);
                *((*m).data).offset(e2 as isize) = tmp;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_transpose_memcpy(
    mut dest: *mut gsl_matrix_long,
    mut src: *const gsl_matrix_long,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_float_transpose_memcpy(
    mut dest: *mut gsl_matrix_float,
    mut src: *const gsl_matrix_float,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let a: gsl_vector_float_const_view = gsl_matrix_float_const_row(src, i);
        let mut b: gsl_vector_float_view = gsl_matrix_float_column(dest, i);
        gsl_blas_scopy(&a.vector, &mut b.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ulong_transpose_memcpy(
    mut dest: *mut gsl_matrix_ulong,
    mut src: *const gsl_matrix_ulong,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_transpose_memcpy(
    mut dest: *mut gsl_matrix,
    mut src: *const gsl_matrix,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let a: gsl_vector_const_view = gsl_matrix_const_row(src, i);
        let mut b: gsl_vector_view = gsl_matrix_column(dest, i);
        gsl_blas_dcopy(&a.vector, &mut b.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_int_transpose_memcpy(
    mut dest: *mut gsl_matrix_int,
    mut src: *const gsl_matrix_int,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_transpose_memcpy(
    mut dest: *mut gsl_matrix_complex_float,
    mut src: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let a: gsl_vector_complex_float_const_view = gsl_matrix_complex_float_const_row(
            src,
            i,
        );
        let mut b: gsl_vector_complex_float_view = gsl_matrix_complex_float_column(
            dest,
            i,
        );
        gsl_blas_ccopy(&a.vector, &mut b.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_transpose_memcpy(
    mut dest: *mut gsl_matrix_complex_long_double,
    mut src: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 2 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_short_transpose_memcpy(
    mut dest: *mut gsl_matrix_short,
    mut src: *const gsl_matrix_short,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uint_transpose_memcpy(
    mut dest: *mut gsl_matrix_uint,
    mut src: *const gsl_matrix_uint,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_long_double_transpose_memcpy(
    mut dest: *mut gsl_matrix_long_double,
    mut src: *const gsl_matrix_long_double,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_char_transpose_memcpy(
    mut dest: *mut gsl_matrix_char,
    mut src: *const gsl_matrix_char,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_transpose_memcpy(
    mut dest: *mut gsl_matrix_complex,
    mut src: *const gsl_matrix_complex,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < src_size1 {
        let a: gsl_vector_complex_const_view = gsl_matrix_complex_const_row(src, i);
        let mut b: gsl_vector_complex_view = gsl_matrix_complex_column(dest, i);
        gsl_blas_zcopy(&a.vector, &mut b.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_uchar_transpose_memcpy(
    mut dest: *mut gsl_matrix_uchar,
    mut src: *const gsl_matrix_uchar,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_ushort_transpose_memcpy(
    mut dest: *mut gsl_matrix_ushort,
    mut src: *const gsl_matrix_ushort,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            k = 0 as libc::c_int as size_t;
            while k < 1 as libc::c_int as libc::c_ulong {
                let mut e1: size_t = i
                    .wrapping_mul((*dest).tda)
                    .wrapping_add(j)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                let mut e2: size_t = j
                    .wrapping_mul((*src).tda)
                    .wrapping_add(i)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(k);
                *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_complex,
    mut src: *const gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < K {
            let a: gsl_vector_complex_const_view = gsl_matrix_complex_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            let mut b: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                dest,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            gsl_blas_zcopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let a_0: gsl_vector_complex_const_view = gsl_matrix_complex_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_complex_view = gsl_matrix_complex_subcolumn(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
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
pub unsafe extern "C" fn gsl_matrix_float_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_float,
    mut src: *const gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < K {
            let a: gsl_vector_float_const_view = gsl_matrix_float_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            let mut b: gsl_vector_float_view = gsl_matrix_float_subcolumn(
                dest,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            gsl_blas_scopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let a_0: gsl_vector_float_const_view = gsl_matrix_float_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_float_view = gsl_matrix_float_subcolumn(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
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
pub unsafe extern "C" fn gsl_matrix_long_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_long,
    mut src: *const gsl_matrix_long,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_uchar_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_uchar,
    mut src: *const gsl_matrix_uchar,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_uint_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_uint,
    mut src: *const gsl_matrix_uint,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_char_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_char,
    mut src: *const gsl_matrix_char,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_long_double_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_long_double,
    mut src: *const gsl_matrix_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_complex_long_double_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_complex_long_double,
    mut src: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 2 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 2 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_short_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_short,
    mut src: *const gsl_matrix_short,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix,
    mut src: *const gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < K {
            let a: gsl_vector_const_view = gsl_matrix_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            let mut b: gsl_vector_view = gsl_matrix_subcolumn(
                dest,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            gsl_blas_dcopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let a_0: gsl_vector_const_view = gsl_matrix_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_view = gsl_matrix_subcolumn(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
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
pub unsafe extern "C" fn gsl_matrix_ulong_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_ulong,
    mut src: *const gsl_matrix_ulong,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_int_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_int,
    mut src: *const gsl_matrix_int,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_complex_float_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_complex_float,
    mut src: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int as size_t;
        while i < K {
            let a: gsl_vector_complex_float_const_view = gsl_matrix_complex_float_const_subrow(
                src,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            let mut b: gsl_vector_complex_float_view = gsl_matrix_complex_float_subcolumn(
                dest,
                i,
                0 as libc::c_int as size_t,
                i,
            );
            gsl_blas_ccopy(&a.vector, &mut b.vector);
            i = i.wrapping_add(1);
            i;
        }
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let a_0: gsl_vector_complex_float_const_view = gsl_matrix_complex_float_const_subrow(
                src,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut b_0: gsl_vector_complex_float_view = gsl_matrix_complex_float_subcolumn(
                dest,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                K.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
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
pub unsafe extern "C" fn gsl_matrix_ushort_transpose_tricpy(
    mut Uplo_src: CBLAS_UPLO_t,
    mut Diag: CBLAS_DIAG_t,
    mut dest: *mut gsl_matrix_ushort,
    mut src: *const gsl_matrix_ushort,
) -> libc::c_int {
    let M: size_t = (*src).size1;
    let N: size_t = (*src).size2;
    let K: size_t = if M < N { M } else { N };
    let mut i: size_t = 0;
    if M != (*dest).size2 || N != (*dest).size1 {
        gsl_error(
            b"matrix sizes are different\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let src_tda: size_t = (*src).tda;
    let dest_tda: size_t = (*dest).tda;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if Uplo_src as libc::c_uint == CblasLower as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = 0 as libc::c_int as size_t;
            while j < i {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
    } else if Uplo_src as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < K {
                k = 0 as libc::c_int as size_t;
                while k < 1 as libc::c_int as libc::c_ulong {
                    let mut e1_0: size_t = j
                        .wrapping_mul(dest_tda)
                        .wrapping_add(i)
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
            b"invalid Uplo_src parameter\0" as *const u8 as *const libc::c_char,
            b"./swap_source.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if Diag as libc::c_uint == CblasNonUnit as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as size_t;
        while i < K {
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
pub unsafe extern "C" fn gsl_matrix_complex_float_conjtrans_memcpy(
    mut dest: *mut gsl_matrix_complex_float,
    mut src: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_complex_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            let mut e1: size_t = i
                .wrapping_mul((*dest).tda)
                .wrapping_add(j)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            let mut e2: size_t = j
                .wrapping_mul((*src).tda)
                .wrapping_add(i)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
            *((*dest).data)
                .offset(
                    e1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = -*((*src).data)
                .offset(e2.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_conjtrans_memcpy(
    mut dest: *mut gsl_matrix_complex,
    mut src: *const gsl_matrix_complex,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_complex_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            let mut e1: size_t = i
                .wrapping_mul((*dest).tda)
                .wrapping_add(j)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            let mut e2: size_t = j
                .wrapping_mul((*src).tda)
                .wrapping_add(i)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
            *((*dest).data)
                .offset(
                    e1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = -*((*src).data)
                .offset(e2.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_conjtrans_memcpy(
    mut dest: *mut gsl_matrix_complex_long_double,
    mut src: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let src_size1: size_t = (*src).size1;
    let src_size2: size_t = (*src).size2;
    let dest_size1: size_t = (*dest).size1;
    let dest_size2: size_t = (*dest).size2;
    let mut i: size_t = 0;
    if dest_size2 != src_size1 || dest_size1 != src_size2 {
        gsl_error(
            b"dimensions of dest matrix must be transpose of src matrix\0" as *const u8
                as *const libc::c_char,
            b"./swap_complex_source.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dest_size1 {
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < dest_size2 {
            let mut e1: size_t = i
                .wrapping_mul((*dest).tda)
                .wrapping_add(j)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            let mut e2: size_t = j
                .wrapping_mul((*src).tda)
                .wrapping_add(i)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            *((*dest).data).offset(e1 as isize) = *((*src).data).offset(e2 as isize);
            *((*dest).data)
                .offset(
                    e1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = -*((*src).data)
                .offset(e2.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
