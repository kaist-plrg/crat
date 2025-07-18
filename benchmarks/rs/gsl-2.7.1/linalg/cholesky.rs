use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_subrow(
        m: *mut gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_const_subrow(
        m: *const gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_transpose_tricpy(
        Uplo_src: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dtrsm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_invnorm1(
        N: size_t,
        Ainvx: Option::<
            unsafe extern "C" fn(
                CBLAS_TRANSPOSE_t,
                *mut gsl_vector,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        params: *mut libc::c_void,
        Ainvnorm: *mut libc::c_double,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_tri_invert(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        T: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_tri_LTL(L: *mut gsl_matrix) -> libc::c_int;
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
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
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
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_linalg_cholesky_decomp(
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = gsl_linalg_cholesky_decomp1(A);
    if status == GSL_SUCCESS as libc::c_int {
        gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, A, A);
    }
    return status;
}
pub unsafe extern "C" fn gsl_linalg_cholesky_decomp1(
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, A, A);
        return cholesky_decomp_L3(A);
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_solve(
    mut LLT: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LLT).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_cholesky_svx(LLT, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_svx(
    mut LLT: *const gsl_matrix,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasNonUnit, LLT, x);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, LLT, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_solve_mat(
    mut LLT: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut X: *mut gsl_matrix,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LLT).size1 != (*B).size1 {
        gsl_error(
            b"matrix size must match B size\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LLT).size2 != (*X).size1 {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_matrix_memcpy(X, B);
        status = gsl_linalg_cholesky_svx_mat(LLT, X);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_svx_mat(
    mut LLT: *const gsl_matrix,
    mut X: *mut gsl_matrix,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LLT).size2 != (*X).size1 {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_blas_dtrsm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasNonUnit,
            1.0f64,
            LLT,
            X,
        );
        gsl_blas_dtrsm(CblasLeft, CblasLower, CblasTrans, CblasNonUnit, 1.0f64, LLT, X);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_invert(
    mut LLT: *mut gsl_matrix,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        status = gsl_linalg_tri_invert(CblasLower, CblasNonUnit, LLT);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_tri_LTL(LLT);
        if status != 0 {
            return status;
        }
        gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, LLT, LLT);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_decomp_unit(
    mut A: *mut gsl_matrix,
    mut D: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut stat_chol: libc::c_int = gsl_linalg_cholesky_decomp1(A);
    if stat_chol == GSL_SUCCESS as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < N {
            let C_ii: libc::c_double = gsl_matrix_get(A, i, i);
            gsl_vector_set(D, i, C_ii * C_ii);
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < N {
            j = 0 as libc::c_int as size_t;
            while j < N {
                gsl_matrix_set(
                    A,
                    i,
                    j,
                    gsl_matrix_get(A, i, j) / sqrt(gsl_vector_get(D, j)),
                );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < N {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                gsl_matrix_set(A, i, j, gsl_matrix_get(A, j, i));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return stat_chol;
}
pub unsafe extern "C" fn gsl_linalg_cholesky_scale(
    mut A: *const gsl_matrix,
    mut S: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"A is not a square matrix\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            319 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*S).size {
        gsl_error(
            b"S must have length N\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            323 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut Aii: libc::c_double = gsl_matrix_get(A, i, i);
            if Aii <= 0.0f64 {
                gsl_vector_set(S, i, 1.0f64);
            } else {
                gsl_vector_set(S, i, 1.0f64 / sqrt(Aii));
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_scale_apply(
    mut A: *mut gsl_matrix,
    mut S: *const gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"A is not a square matrix\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*S).size {
        gsl_error(
            b"S must have length N\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            368 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut sj: libc::c_double = gsl_vector_get(S, j);
            i = j;
            while i < N {
                let mut si: libc::c_double = gsl_vector_get(S, i);
                let mut Aij: *mut libc::c_double = gsl_matrix_ptr(A, i, j);
                *Aij *= si * sj;
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_decomp2(
    mut A: *mut gsl_matrix,
    mut S: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*S).size {
        gsl_error(
            b"S must have length N\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            403 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        status = gsl_linalg_cholesky_scale(A, S);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_cholesky_scale_apply(A, S);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_cholesky_decomp1(A);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_svx2(
    mut LLT: *const gsl_matrix,
    mut S: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            435 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LLT).size2 != (*S).size {
        gsl_error(
            b"matrix size must match S\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_mul(x, S);
        gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasNonUnit, LLT, x);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, LLT, x);
        gsl_vector_mul(x, S);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_solve2(
    mut LLT: *const gsl_matrix,
    mut S: *const gsl_vector,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LLT).size1 != (*LLT).size2 {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            471 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LLT).size1 != (*S).size {
        gsl_error(
            b"matrix size must match S size\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            475 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LLT).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LLT).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_cholesky_svx2(LLT, S, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_cholesky_rcond(
    mut LLT: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*LLT).size1;
    let N: size_t = (*LLT).size2;
    if M != N {
        gsl_error(
            b"cholesky matrix must be square\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            507 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*work).size != (3 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
        gsl_error(
            b"work vector must have length 3*N\0" as *const u8 as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            511 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut Anorm: libc::c_double = cholesky_norm1(LLT, work);
        let mut Ainvnorm: libc::c_double = 0.;
        *rcond = 0.0f64;
        if Anorm == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        status = gsl_linalg_invnorm1(
            N,
            Some(
                cholesky_Ainv
                    as unsafe extern "C" fn(
                        CBLAS_TRANSPOSE_t,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            LLT as *mut libc::c_void,
            &mut Ainvnorm,
            work,
        );
        if status != 0 {
            return status;
        }
        if Ainvnorm != 0.0f64 {
            *rcond = 1.0f64 / Anorm / Ainvnorm;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cholesky_norm1(
    mut LLT: *const gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_double {
    let N: size_t = (*LLT).size1;
    let mut max: libc::c_double = 0.0f64;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < N {
        let mut sum: libc::c_double = 0.0f64;
        let lj: gsl_vector_const_view = gsl_matrix_const_subrow(
            LLT,
            j,
            0 as libc::c_int as size_t,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        let mut Ajj: libc::c_double = 0.;
        gsl_blas_ddot(&lj.vector, &lj.vector, &mut Ajj);
        i = 0 as libc::c_int as size_t;
        while i < j {
            let mut wi: *mut libc::c_double = gsl_vector_ptr(work, i);
            let mut Aij: libc::c_double = gsl_matrix_get(LLT, i, j);
            let mut absAij: libc::c_double = fabs(Aij);
            sum += absAij;
            *wi += absAij;
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(work, j, sum + fabs(Ajj));
        j = j.wrapping_add(1);
        j;
    }
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut wi_0: libc::c_double = gsl_vector_get(work, i);
        max = if max > wi_0 { max } else { wi_0 };
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
unsafe extern "C" fn cholesky_Ainv(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut x: *mut gsl_vector,
    mut params: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut A: *mut gsl_matrix = params as *mut gsl_matrix;
    status = gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasNonUnit, A, x);
    if status != 0 {
        return status;
    }
    status = gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, A, x);
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn cholesky_decomp_L2(mut A: *mut gsl_matrix) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            622 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut ajj: libc::c_double = 0.;
            let mut v: gsl_vector_view = gsl_matrix_subcolumn(
                A,
                j,
                j,
                N.wrapping_sub(j),
            );
            if j > 0 as libc::c_int as libc::c_ulong {
                let mut w: gsl_vector_view = gsl_matrix_subrow(
                    A,
                    j,
                    0 as libc::c_int as size_t,
                    j,
                );
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    j,
                    0 as libc::c_int as size_t,
                    N.wrapping_sub(j),
                    j,
                );
                gsl_blas_dgemv(
                    CblasNoTrans,
                    -1.0f64,
                    &mut m.matrix,
                    &mut w.vector,
                    1.0f64,
                    &mut v.vector,
                );
            }
            ajj = gsl_matrix_get(A, j, j);
            if ajj <= 0.0f64 {
                gsl_error(
                    b"matrix is not positive definite\0" as *const u8
                        as *const libc::c_char,
                    b"cholesky.c\0" as *const u8 as *const libc::c_char,
                    645 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            }
            ajj = sqrt(ajj);
            gsl_vector_scale(&mut v.vector, 1.0f64 / ajj);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cholesky_decomp_L3(mut A: *mut gsl_matrix) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Cholesky decomposition requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"cholesky.c\0" as *const u8 as *const libc::c_char,
            681 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N <= 24 as libc::c_int as libc::c_ulong {
        return cholesky_decomp_L2(A)
    } else {
        let mut status: libc::c_int = 0;
        let N1: size_t = if N >= 16 as libc::c_int as libc::c_ulong {
            N.wrapping_add(8 as libc::c_int as libc::c_ulong)
                .wrapping_div(16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        } else {
            N.wrapping_div(2 as libc::c_int as libc::c_ulong)
        };
        let N2: size_t = N.wrapping_sub(N1);
        let mut A11: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let mut A21: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let mut A22: gsl_matrix_view = gsl_matrix_submatrix(A, N1, N1, N2, N2);
        status = cholesky_decomp_L3(&mut A11.matrix);
        if status != 0 {
            return status;
        }
        gsl_blas_dtrsm(
            CblasRight,
            CblasLower,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut A11.matrix,
            &mut A21.matrix,
        );
        gsl_blas_dsyrk(
            CblasLower,
            CblasNoTrans,
            -1.0f64,
            &mut A21.matrix,
            1.0f64,
            &mut A22.matrix,
        );
        status = cholesky_decomp_L3(&mut A22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
