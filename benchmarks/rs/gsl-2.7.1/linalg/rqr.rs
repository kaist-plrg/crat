use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_add_constant(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_matrix_const_column(
        m: *const gsl_matrix,
        j: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subrow(
        m: *const gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_tricpy(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_matrix_transpose_memcpy(
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_matrix_sub(a: *mut gsl_matrix, b: *const gsl_matrix) -> libc::c_int;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrmv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
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
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_const_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_const_view = _gsl_matrix_const_view;
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_const_ptr(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> *const libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize)
        as *const libc::c_double;
}
pub unsafe extern "C" fn gsl_linalg_QR_decomp_r(
    mut A: *mut gsl_matrix,
    mut T: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != (*T).size2 {
        gsl_error(
            b"T matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*T).size1 != N {
        gsl_error(
            b"T matrix does not match dimensions of A\0" as *const u8
                as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let mut T00: *mut libc::c_double = gsl_matrix_ptr(
            T,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut v: gsl_vector_view = gsl_matrix_column(A, 0 as libc::c_int as size_t);
        *T00 = gsl_linalg_householder_transform(&mut v.vector);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let M2: size_t = M.wrapping_sub(N1);
        let mut A11: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut A12: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut A21: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            N1,
            0 as libc::c_int as size_t,
            M2,
            N1,
        );
        let mut A22: gsl_matrix_view = gsl_matrix_submatrix(A, N1, N1, M2, N2);
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
        m = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
            N1,
        );
        status = gsl_linalg_QR_decomp_r(&mut m.matrix, &mut T11.matrix);
        if status != 0 {
            return status;
        }
        gsl_matrix_memcpy(&mut T12.matrix, &mut A12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasLower,
            CblasTrans,
            CblasUnit,
            1.0f64,
            &mut A11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_dgemm(
            CblasTrans,
            CblasNoTrans,
            1.0f64,
            &mut A21.matrix,
            &mut A22.matrix,
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
        gsl_blas_dgemm(
            CblasNoTrans,
            CblasNoTrans,
            -1.0f64,
            &mut A21.matrix,
            &mut T12.matrix,
            1.0f64,
            &mut A22.matrix,
        );
        gsl_blas_dtrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            1.0f64,
            &mut A11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_sub(&mut A12.matrix, &mut T12.matrix);
        status = gsl_linalg_QR_decomp_r(&mut A22.matrix, &mut T22.matrix);
        if status != 0 {
            return status;
        }
        m = gsl_matrix_submatrix(
            &mut A21.matrix,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        gsl_matrix_transpose_memcpy(&mut T12.matrix, &mut m.matrix);
        m = gsl_matrix_submatrix(A, N1, N1, N2, N2);
        gsl_blas_dtrmm(
            CblasRight,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            1.0f64,
            &mut m.matrix,
            &mut T12.matrix,
        );
        if M > N {
            let mut V31: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N1,
            );
            let mut V32: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                N,
                N1,
                M.wrapping_sub(N),
                N2,
            );
            gsl_blas_dgemm(
                CblasTrans,
                CblasNoTrans,
                1.0f64,
                &mut V31.matrix,
                &mut V32.matrix,
                1.0f64,
                &mut T12.matrix,
            );
        }
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
pub unsafe extern "C" fn gsl_linalg_QR_solve_r(
    mut QR: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*QR).size2;
    if (*QR).size1 != N {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*T).size1 != (*QR).size1 || (*T).size2 != (*QR).size2 {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        gsl_vector_memcpy(x, b);
        gsl_blas_dtrmv(CblasLower, CblasTrans, CblasUnit, QR, x);
        gsl_blas_dtrmv(CblasUpper, CblasTrans, CblasNonUnit, T, x);
        gsl_blas_dtrmv(CblasLower, CblasNoTrans, CblasUnit, QR, x);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut xi: *mut libc::c_double = gsl_vector_ptr(x, i);
            let mut bi: libc::c_double = gsl_vector_get(b, i);
            *xi = bi - *xi;
            i = i.wrapping_add(1);
            i;
        }
        gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, QR, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QR_lssolve_r(
    mut QR: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"QR matrix must have M >= N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*work).size {
        gsl_error(
            b"matrix size must match work size\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let R: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut x1: gsl_vector_view = gsl_vector_subvector(
            x,
            0 as libc::c_int as size_t,
            N,
        );
        gsl_vector_memcpy(x, b);
        gsl_linalg_QR_QTvec_r(QR, T, x, work);
        gsl_blas_dtrsv(
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            &R.matrix,
            &mut x1.vector,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QR_unpack_r(
    mut QR: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut Q: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Q).size1 != M || (*Q).size2 != M {
        gsl_error(
            b"Q matrix must be M-by-M\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*R).size1 != N || (*R).size2 != N {
        gsl_error(
            b"R matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let RV: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut Q1: gsl_matrix_view = gsl_matrix_submatrix(
            Q,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
            N,
        );
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
            Q,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        gsl_matrix_tricpy(CblasUpper, CblasNonUnit, &mut m.matrix, T);
        gsl_matrix_tricpy(CblasLower, CblasUnit, &mut m.matrix, &RV.matrix);
        if M > N {
            let tmp: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            m = gsl_matrix_submatrix(
                Q,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            gsl_matrix_memcpy(&mut m.matrix, &tmp.matrix);
        }
        unpack_Q1(&mut Q1.matrix);
        if M > N {
            let mut Q2: gsl_matrix_view = gsl_matrix_submatrix(
                Q,
                0 as libc::c_int as size_t,
                N,
                M,
                M.wrapping_sub(N),
            );
            unpack_Q2(QR, T, &mut Q2.matrix);
        }
        gsl_matrix_tricpy(CblasUpper, CblasNonUnit, R, &RV.matrix);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QR_QTvec_r(
    mut QR: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut b: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*b).size != M {
        gsl_error(
            b"b vector must have length M\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != N {
        gsl_error(
            b"workspace must be length N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            425 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let V1: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut b1: gsl_vector_view = gsl_vector_subvector(
            b,
            0 as libc::c_int as size_t,
            N,
        );
        let mut b2: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        gsl_vector_memcpy(work, &mut b1.vector);
        gsl_blas_dtrmv(CblasLower, CblasTrans, CblasUnit, &V1.matrix, work);
        if M > N {
            let V2: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            b2 = gsl_vector_subvector(b, N, M.wrapping_sub(N));
            gsl_blas_dgemv(CblasTrans, 1.0f64, &V2.matrix, &mut b2.vector, 1.0f64, work);
        }
        gsl_blas_dtrmv(CblasUpper, CblasTrans, CblasNonUnit, T, work);
        if M > N {
            let V2_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            gsl_blas_dgemv(
                CblasNoTrans,
                -1.0f64,
                &V2_0.matrix,
                work,
                1.0f64,
                &mut b2.vector,
            );
        }
        gsl_blas_dtrmv(CblasLower, CblasNoTrans, CblasUnit, &V1.matrix, work);
        gsl_vector_sub(&mut b1.vector, work);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QR_QTmat_r(
    mut QR: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut B: *mut gsl_matrix,
    mut work: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    let K: size_t = (*B).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            490 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*B).size1 != M {
        gsl_error(
            b"B matrix must have M rows\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            494 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size1 != N || (*work).size2 != K {
        gsl_error(
            b"workspace must be N-by-K\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            498 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let V1: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut B1: gsl_matrix_view = gsl_matrix_submatrix(
            B,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            K,
        );
        let mut B2: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        gsl_matrix_memcpy(work, &mut B1.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasLower,
            CblasTrans,
            CblasUnit,
            1.0f64,
            &V1.matrix,
            work,
        );
        if M > N {
            let V2: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            B2 = gsl_matrix_submatrix(
                B,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                K,
            );
            gsl_blas_dgemm(
                CblasTrans,
                CblasNoTrans,
                1.0f64,
                &V2.matrix,
                &mut B2.matrix,
                1.0f64,
                work,
            );
        }
        gsl_blas_dtrmm(CblasLeft, CblasUpper, CblasTrans, CblasNonUnit, 1.0f64, T, work);
        if M > N {
            let V2_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                QR,
                N,
                0 as libc::c_int as size_t,
                M.wrapping_sub(N),
                N,
            );
            gsl_blas_dgemm(
                CblasNoTrans,
                CblasNoTrans,
                -1.0f64,
                &V2_0.matrix,
                work,
                1.0f64,
                &mut B2.matrix,
            );
        }
        gsl_blas_dtrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            1.0f64,
            &V1.matrix,
            work,
        );
        gsl_matrix_sub(&mut B1.matrix, work);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn unpack_Q1(mut Q: *mut gsl_matrix) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let M: size_t = (*Q).size1;
    let N: size_t = (*Q).size2;
    let mut Q1: gsl_matrix_view = gsl_matrix_submatrix(
        Q,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        N,
        N,
    );
    let mut diag: gsl_vector_view = gsl_matrix_diagonal(&mut Q1.matrix);
    status = aux_ULT(&mut Q1.matrix, &mut Q1.matrix);
    if status != 0 {
        return status;
    }
    if M > N {
        let mut V2: gsl_matrix_view = gsl_matrix_submatrix(
            Q,
            N,
            0 as libc::c_int as size_t,
            M.wrapping_sub(N),
            N,
        );
        gsl_blas_dtrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            &mut Q1.matrix,
            &mut V2.matrix,
        );
    }
    status = aux_mLU(&mut Q1.matrix);
    if status != 0 {
        return status;
    }
    gsl_vector_add_constant(&mut diag.vector, 1.0f64);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn unpack_Q2(
    mut QR: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut Q: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M <= N {
        gsl_error(
            b"M must be > N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            614 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            618 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Q).size1 != M || (*Q).size2 != M.wrapping_sub(N) {
        gsl_error(
            b"Q matrix must be M-by-(M-N)\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            622 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let V1: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QR,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let V2: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QR,
            N,
            0 as libc::c_int as size_t,
            M.wrapping_sub(N),
            N,
        );
        let mut Q1: gsl_matrix_view = gsl_matrix_submatrix(
            Q,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            M.wrapping_sub(N),
        );
        let mut Q2: gsl_matrix_view = gsl_matrix_submatrix(
            Q,
            N,
            0 as libc::c_int as size_t,
            M.wrapping_sub(N),
            M.wrapping_sub(N),
        );
        let mut diag: gsl_vector_view = gsl_matrix_diagonal(&mut Q2.matrix);
        gsl_matrix_transpose_memcpy(&mut Q1.matrix, &V2.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            T,
            &mut Q1.matrix,
        );
        gsl_blas_dgemm(
            CblasNoTrans,
            CblasNoTrans,
            1.0f64,
            &V2.matrix,
            &mut Q1.matrix,
            0.0f64,
            &mut Q2.matrix,
        );
        gsl_vector_add_constant(&mut diag.vector, 1.0f64);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            1.0f64,
            &V1.matrix,
            &mut Q1.matrix,
        );
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn aux_ULT(
    mut L: *const gsl_matrix,
    mut U: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*L).size1;
    if N != (*L).size2 {
        gsl_error(
            b"L matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            659 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*U).size1 != N || (*U).size2 != N {
        gsl_error(
            b"U matrix must be same size as L\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            663 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let L11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            L,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let L21: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            L,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let L22: gsl_matrix_const_view = gsl_matrix_const_submatrix(L, N1, N1, N2, N2);
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
        gsl_blas_dtrmm(
            CblasRight,
            CblasLower,
            CblasTrans,
            CblasUnit,
            1.0f64,
            &L22.matrix,
            &mut U12.matrix,
        );
        status = aux_ApUBT(&mut U11.matrix, &L21.matrix, &mut U12.matrix);
        if status != 0 {
            return status;
        }
        status = aux_ULT(&L11.matrix, &mut U11.matrix);
        if status != 0 {
            return status;
        }
        status = aux_ULT(&L22.matrix, &mut U22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn aux_mLU(mut A: *mut gsl_matrix) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            712 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let mut A00: *mut libc::c_double = gsl_matrix_ptr(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        *A00 = -*A00;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let mut A11: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut A12: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            N1,
            N2,
        );
        let mut A21: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let mut A22: gsl_matrix_view = gsl_matrix_submatrix(A, N1, N1, N2, N2);
        status = aux_mLU(&mut A22.matrix);
        if status != 0 {
            return status;
        }
        gsl_blas_dgemm(
            CblasNoTrans,
            CblasNoTrans,
            -1.0f64,
            &mut A21.matrix,
            &mut A12.matrix,
            1.0f64,
            &mut A22.matrix,
        );
        gsl_blas_dtrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            -1.0f64,
            &mut A11.matrix,
            &mut A12.matrix,
        );
        gsl_blas_dtrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            &mut A11.matrix,
            &mut A21.matrix,
        );
        status = aux_mLU(&mut A11.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn aux_ApUBT(
    mut U: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*U).size1 != M || (*U).size2 != M {
        gsl_error(
            b"U matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            763 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*B).size1 != N || (*B).size2 != M {
        gsl_error(
            b"B matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"rqr.c\0" as *const u8 as *const libc::c_char,
            767 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M == 1 as libc::c_int as libc::c_ulong
        && N == 1 as libc::c_int as libc::c_ulong
    {
        let mut aptr: *mut libc::c_double = gsl_matrix_ptr(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut uptr: *const libc::c_double = gsl_matrix_const_ptr(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut bptr: *const libc::c_double = gsl_matrix_const_ptr(
            B,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        *aptr += *uptr * *bptr;
        return GSL_SUCCESS as libc::c_int;
    } else if M == 1 as libc::c_int as libc::c_ulong {
        let mut U00: libc::c_double = gsl_matrix_get(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        let mut v: gsl_vector_view = gsl_matrix_row(A, 0 as libc::c_int as size_t);
        let w: gsl_vector_const_view = gsl_matrix_const_column(
            B,
            0 as libc::c_int as size_t,
        );
        gsl_blas_daxpy(U00, &w.vector, &mut v.vector);
        return GSL_SUCCESS as libc::c_int;
    } else if N == 1 as libc::c_int as libc::c_ulong {
        let mut status: libc::c_int = 0;
        let M1: size_t = M.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let M2: size_t = M.wrapping_sub(M1);
        let mut A11: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1,
            1 as libc::c_int as size_t,
        );
        let mut A21: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            M1,
            0 as libc::c_int as size_t,
            M2,
            1 as libc::c_int as size_t,
        );
        let mut a1: gsl_vector_view = gsl_matrix_subcolumn(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1,
        );
        let U11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1,
            M1,
        );
        let U12: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            M1,
            M1,
            M2,
        );
        let U22: gsl_matrix_const_view = gsl_matrix_const_submatrix(U, M1, M1, M2, M2);
        let B11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            M1,
        );
        let B12: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            M1,
            1 as libc::c_int as size_t,
            M2,
        );
        let b2: gsl_vector_const_view = gsl_matrix_const_subrow(
            B,
            0 as libc::c_int as size_t,
            M1,
            M2,
        );
        gsl_blas_dgemv(
            CblasNoTrans,
            1.0f64,
            &U12.matrix,
            &b2.vector,
            1.0f64,
            &mut a1.vector,
        );
        status = aux_ApUBT(&U11.matrix, &B11.matrix, &mut A11.matrix);
        if status != 0 {
            return status;
        }
        status = aux_ApUBT(&U22.matrix, &B12.matrix, &mut A21.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut status_0: libc::c_int = 0;
        let M1_0: size_t = M.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let M2_0: size_t = M.wrapping_sub(M1_0);
        let N1: size_t = N.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let N2: size_t = N.wrapping_sub(N1);
        let mut A11_0: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1_0,
            N1,
        );
        let mut A12: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            N1,
            M1_0,
            N2,
        );
        let mut A21_0: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            M1_0,
            0 as libc::c_int as size_t,
            M2_0,
            N1,
        );
        let mut A22: gsl_matrix_view = gsl_matrix_submatrix(A, M1_0, N1, M2_0, N2);
        let U11_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M1_0,
            M1_0,
        );
        let U12_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            U,
            0 as libc::c_int as size_t,
            M1_0,
            M1_0,
            M2_0,
        );
        let U22_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            U,
            M1_0,
            M1_0,
            M2_0,
            M2_0,
        );
        let B11_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            M1_0,
        );
        let B12_0: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            M1_0,
            N1,
            M2_0,
        );
        let B21: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            N1,
            0 as libc::c_int as size_t,
            N2,
            M1_0,
        );
        let B22: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            N1,
            M1_0,
            N2,
            M2_0,
        );
        status_0 = aux_ApUBT(&U11_0.matrix, &B11_0.matrix, &mut A11_0.matrix);
        if status_0 != 0 {
            return status_0;
        }
        gsl_blas_dgemm(
            CblasNoTrans,
            CblasTrans,
            1.0f64,
            &U12_0.matrix,
            &B12_0.matrix,
            1.0f64,
            &mut A11_0.matrix,
        );
        status_0 = aux_ApUBT(&U11_0.matrix, &B21.matrix, &mut A12.matrix);
        if status_0 != 0 {
            return status_0;
        }
        gsl_blas_dgemm(
            CblasNoTrans,
            CblasTrans,
            1.0f64,
            &U12_0.matrix,
            &B22.matrix,
            1.0f64,
            &mut A12.matrix,
        );
        status_0 = aux_ApUBT(&U22_0.matrix, &B12_0.matrix, &mut A21_0.matrix);
        if status_0 != 0 {
            return status_0;
        }
        status_0 = aux_ApUBT(&U22_0.matrix, &B22.matrix, &mut A22.matrix);
        if status_0 != 0 {
            return status_0;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
