use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_const_subvector(
        v: *const gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_sub(a: *mut gsl_matrix, b: *const gsl_matrix) -> libc::c_int;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_dgemm(
        TransA: CBLAS_TRANSPOSE_t,
        TransB: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dtrmm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_UU_lssolve(
        R: *const gsl_matrix,
        Y: *const gsl_matrix,
        T: *const gsl_matrix,
        b: *const gsl_vector,
        x: *mut gsl_vector,
        work: *mut gsl_vector,
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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_SIDE = libc::c_uint;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
pub type CBLAS_SIDE_t = CBLAS_SIDE;
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
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_view = _gsl_matrix_view;
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_linalg_QR_UD_decomp(
    mut U: *mut gsl_matrix,
    mut D: *const gsl_vector,
    mut Y: *mut gsl_matrix,
    mut T: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*U).size1;
    if N != (*U).size2 {
        gsl_error(
            b"U matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*D).size != N {
        gsl_error(
            b"D matrix does not match dimensions of U\0" as *const u8
                as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Y).size1 != (*Y).size2 {
        gsl_error(
            b"Y matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*Y).size1 != N {
        gsl_error(
            b"Y matrix does not match dimensions of U\0" as *const u8
                as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let mut T00: *mut libc::c_double = gsl_matrix_ptr(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut U00: *mut libc::c_double = gsl_matrix_ptr(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut Y00: *mut libc::c_double = gsl_matrix_ptr(
            Y,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        *Y00 = gsl_vector_get(D, 0 as libc::c_int as size_t);
        *T00 = qrtd_householder_transform(U00, Y00);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let mut U11: gsl_matrix_view = gsl_matrix_submatrix(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut U12: gsl_matrix_view = gsl_matrix_submatrix(
            U,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut U22: gsl_matrix_view = gsl_matrix_submatrix(U, N1, N1, N2, N2);
        let D1: gsl_vector_const_view = gsl_vector_const_subvector(
            D,
            0 as libc::c_int as size_t,
            N1,
        );
        let D2: gsl_vector_const_view = gsl_vector_const_subvector(D, N1, N2);
        let mut Y11: gsl_matrix_view = gsl_matrix_submatrix(
            Y,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut Y12: gsl_matrix_view = gsl_matrix_submatrix(
            Y,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut T11: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut T12: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut T22: gsl_matrix_view = gsl_matrix_submatrix(T, N1, N1, N2, N2);
        let mut m: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        status = gsl_linalg_QR_UD_decomp(
            &mut U11.matrix,
            &D1.vector,
            &mut Y11.matrix,
            &mut T11.matrix,
        );
        if status != 0 {
            return status;
        }
        gsl_matrix_memcpy(&mut T12.matrix, &mut U12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_sub(&mut U12.matrix, &mut T12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            &mut Y11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_memcpy(&mut Y12.matrix, &mut T12.matrix);
        m = gsl_matrix_submatrix(Y, 0 as libc::c_int as size_t, N1, N, N2);
        status = aux_QR_TRD_decomp(
            &mut U22.matrix,
            &mut m.matrix,
            &D2.vector,
            &mut T22.matrix,
        );
        if status != 0 {
            return status;
        }
        gsl_matrix_memcpy(&mut T12.matrix, &mut Y12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut Y11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_dtrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            1.0f64,
            &mut T22.matrix,
            &mut T12.matrix,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QR_UD_lssolve(
    mut R: *const gsl_matrix,
    mut Y: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*R).size1;
    let M: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(N);
    if (*R).size2 != N {
        gsl_error(
            b"R matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*Y).size1 != (*Y).size2 {
        gsl_error(
            b"Y matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*Y).size1 != N {
        gsl_error(
            b"Y and R must have same dimensions\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            250 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*work).size {
        gsl_error(
            b"workspace must be length N\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            258 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        return gsl_linalg_QR_UU_lssolve(R, Y, T, b, x, work)
    };
}
unsafe extern "C" fn aux_QR_TRD_decomp(
    mut U: *mut gsl_matrix,
    mut A: *mut gsl_matrix,
    mut D: *const gsl_vector,
    mut T: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*U).size1;
    if N != (*U).size2 {
        gsl_error(
            b"U matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*A).size1 <= N {
        gsl_error(
            b"A matrix must have > N rows\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*D).size != N {
        gsl_error(
            b"D matrix does not match dimensions of U\0" as *const u8
                as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"qr_ud.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let M: size_t = ((*A).size1).wrapping_sub(N);
        let mut T00: *mut libc::c_double = gsl_matrix_ptr(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut U00: *mut libc::c_double = gsl_matrix_ptr(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut v: gsl_vector_view = gsl_matrix_subcolumn(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
        );
        let mut D00: *mut libc::c_double = gsl_matrix_ptr(
            A,
            M,
            0 as libc::c_int as size_t,
        );
        *D00 = gsl_vector_get(D, 0 as libc::c_int as size_t);
        *T00 = qrtrd_householder_transform(U00, &mut v.vector, D00);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let M_0: size_t = ((*A).size1).wrapping_sub(N);
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let mut U11: gsl_matrix_view = gsl_matrix_submatrix(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut U12: gsl_matrix_view = gsl_matrix_submatrix(
            U,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut U22: gsl_matrix_view = gsl_matrix_submatrix(U, N1, N1, N2, N2);
        let mut A1: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M_0,
            N1,
        );
        let mut A2: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            M_0,
            N2,
        );
        let D1: gsl_vector_const_view = gsl_vector_const_subvector(
            D,
            0 as libc::c_int as size_t,
            N1,
        );
        let D2: gsl_vector_const_view = gsl_vector_const_subvector(D, N1, N2);
        let mut T11: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut T12: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut T22: gsl_matrix_view = gsl_matrix_submatrix(T, N1, N1, N2, N2);
        let mut Y41: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            M_0,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut Y42: gsl_matrix_view = gsl_matrix_submatrix(A, M_0, N1, N1, N2);
        let mut m: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        m = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M_0.wrapping_add(N1),
            N1,
        );
        status = aux_QR_TRD_decomp(
            &mut U11.matrix,
            &mut m.matrix,
            &D1.vector,
            &mut T11.matrix,
        );
        if status != 0 {
            return status;
        }
        gsl_matrix_memcpy(&mut T12.matrix, &mut U12.matrix);
        gsl_blas_dgemm(
            CblasTrans,
            CblasNoTrans,
            1.0f64,
            &mut A1.matrix,
            &mut A2.matrix,
            1.0f64,
            &mut T12.matrix,
        );
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_sub(&mut U12.matrix, &mut T12.matrix);
        gsl_blas_dgemm(
            CblasNoTrans,
            CblasNoTrans,
            -1.0f64,
            &mut A1.matrix,
            &mut T12.matrix,
            1.0f64,
            &mut A2.matrix,
        );
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            &mut Y41.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_memcpy(&mut Y42.matrix, &mut T12.matrix);
        m = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            M_0.wrapping_add(N),
            N2,
        );
        status = aux_QR_TRD_decomp(
            &mut U22.matrix,
            &mut m.matrix,
            &D2.vector,
            &mut T22.matrix,
        );
        if status != 0 {
            return status;
        }
        gsl_matrix_memcpy(&mut T12.matrix, &mut Y42.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut Y41.matrix,
            &mut T12.matrix,
        );
        gsl_blas_dgemm(
            CblasTrans,
            CblasNoTrans,
            1.0f64,
            &mut A1.matrix,
            &mut A2.matrix,
            1.0f64,
            &mut T12.matrix,
        );
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_dtrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            1.0f64,
            &mut T22.matrix,
            &mut T12.matrix,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn qrtd_householder_transform(
    mut v0: *mut libc::c_double,
    mut v1: *mut libc::c_double,
) -> libc::c_double {
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut xnorm: libc::c_double = fabs(*v1);
    if xnorm == 0 as libc::c_int as libc::c_double {
        return 0.0f64;
    }
    alpha = *v0;
    beta = -(if alpha >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
        as libc::c_double * hypot(alpha, xnorm);
    tau = (beta - alpha) / beta;
    let mut s: libc::c_double = alpha - beta;
    if fabs(s) > 2.2250738585072014e-308f64 {
        *v1 /= s;
        *v0 = beta;
    } else {
        *v1 *= 2.2204460492503131e-16f64 / s;
        *v1 /= 2.2204460492503131e-16f64;
        *v0 = beta;
    }
    return tau;
}
unsafe extern "C" fn qrtrd_householder_transform(
    mut v0: *mut libc::c_double,
    mut v: *mut gsl_vector,
    mut d: *mut libc::c_double,
) -> libc::c_double {
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut xnorm: libc::c_double = 0.;
    xnorm = gsl_blas_dnrm2(v);
    xnorm = gsl_hypot(xnorm, *d);
    if xnorm == 0 as libc::c_int as libc::c_double {
        return 0.0f64;
    }
    alpha = *v0;
    beta = -(if alpha >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
        as libc::c_double * hypot(alpha, xnorm);
    tau = (beta - alpha) / beta;
    let mut s: libc::c_double = alpha - beta;
    if fabs(s) > 2.2250738585072014e-308f64 {
        gsl_blas_dscal(1.0f64 / s, v);
        *d /= s;
        *v0 = beta;
    } else {
        gsl_blas_dscal(2.2204460492503131e-16f64 / s, v);
        gsl_blas_dscal(1.0f64 / 2.2204460492503131e-16f64, v);
        *d *= 2.2204460492503131e-16f64 / s;
        *d /= 2.2204460492503131e-16f64;
        *v0 = beta;
    }
    return tau;
}
