use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_matrix_const_subcolumn(
        m: *const gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_eigen_symm_alloc(n: size_t) -> *mut gsl_eigen_symm_workspace;
    fn gsl_eigen_symm_free(w: *mut gsl_eigen_symm_workspace);
    fn gsl_eigen_symm(
        A: *mut gsl_matrix,
        eval: *mut gsl_vector,
        w: *mut gsl_eigen_symm_workspace,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_decomp1(A: *mut gsl_matrix) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dsyr2k(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dsyr2(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dsymm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *const gsl_matrix,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_symm_workspace {
    pub size: size_t,
    pub d: *mut libc::c_double,
    pub sd: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_gensymm_workspace {
    pub size: size_t,
    pub symm_workspace_p: *mut gsl_eigen_symm_workspace,
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
pub unsafe extern "C" fn gsl_eigen_gensymm_alloc(
    n: size_t,
) -> *mut gsl_eigen_gensymm_workspace {
    let mut w: *mut gsl_eigen_gensymm_workspace = 0 as *mut gsl_eigen_gensymm_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"gensymm.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_eigen_gensymm_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_eigen_gensymm_workspace>() as libc::c_ulong,
    ) as *mut gsl_eigen_gensymm_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"gensymm.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_gensymm_workspace;
    }
    (*w).size = n;
    (*w).symm_workspace_p = gsl_eigen_symm_alloc(n);
    if ((*w).symm_workspace_p).is_null() {
        gsl_eigen_gensymm_free(w);
        gsl_error(
            b"failed to allocate space for symm workspace\0" as *const u8
                as *const libc::c_char,
            b"gensymm.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_gensymm_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_eigen_gensymm_free(
    mut w: *mut gsl_eigen_gensymm_workspace,
) {
    if w.is_null() {
        return;
    }
    if !((*w).symm_workspace_p).is_null() {
        gsl_eigen_symm_free((*w).symm_workspace_p);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_eigen_gensymm(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut eval: *mut gsl_vector,
    mut w: *mut gsl_eigen_gensymm_workspace,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8
                as *const libc::c_char,
            b"gensymm.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*B).size1 || N != (*B).size2 {
        gsl_error(
            b"B matrix dimensions must match A\0" as *const u8 as *const libc::c_char,
            b"gensymm.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*eval).size != N {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"gensymm.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*w).size != N {
        gsl_error(
            b"matrix size does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"gensymm.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        s = gsl_linalg_cholesky_decomp1(B);
        if s != GSL_SUCCESS as libc::c_int {
            return s;
        }
        gsl_eigen_gensymm_standardize(A, B);
        s = gsl_eigen_symm(A, eval, (*w).symm_workspace_p);
        return s;
    };
}
pub unsafe extern "C" fn gsl_eigen_gensymm_standardize(
    mut A: *mut gsl_matrix,
    mut B: *const gsl_matrix,
) -> libc::c_int {
    return gensymm_standardize_L3(A, B);
}
unsafe extern "C" fn gensymm_standardize_L2(
    mut A: *mut gsl_matrix,
    mut B: *const gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let mut i: size_t = 0;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < N {
        a = gsl_matrix_get(A, i, i);
        b = gsl_matrix_get(B, i, i);
        a /= b * b;
        gsl_matrix_set(A, i, i, a);
        if i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut ai: gsl_vector_view = gsl_matrix_subcolumn(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut ma: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let bi: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                B,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mb: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                B,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_dscal(1.0f64 / b, &mut ai.vector);
            c = -0.5f64 * a;
            gsl_blas_daxpy(c, &bi.vector, &mut ai.vector);
            gsl_blas_dsyr2(
                CblasLower,
                -1.0f64,
                &mut ai.vector,
                &bi.vector,
                &mut ma.matrix,
            );
            gsl_blas_daxpy(c, &bi.vector, &mut ai.vector);
            gsl_blas_dtrsv(
                CblasLower,
                CblasNoTrans,
                CblasNonUnit,
                &mb.matrix,
                &mut ai.vector,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gensymm_standardize_L3(
    mut A: *mut gsl_matrix,
    mut B: *const gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N <= 24 as libc::c_int as libc::c_ulong {
        return gensymm_standardize_L2(A, B)
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
        let B11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N1,
            N1,
        );
        let B21: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            B,
            N1,
            0 as libc::c_int as size_t,
            N2,
            N1,
        );
        let B22: gsl_matrix_const_view = gsl_matrix_const_submatrix(B, N1, N1, N2, N2);
        status = gensymm_standardize_L3(&mut A11.matrix, &B11.matrix);
        if status != 0 {
            return status;
        }
        gsl_blas_dtrsm(
            CblasRight,
            CblasLower,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &B11.matrix,
            &mut A21.matrix,
        );
        gsl_blas_dsymm(
            CblasRight,
            CblasLower,
            -0.5f64,
            &mut A11.matrix,
            &B21.matrix,
            1.0f64,
            &mut A21.matrix,
        );
        gsl_blas_dsyr2k(
            CblasLower,
            CblasNoTrans,
            -1.0f64,
            &mut A21.matrix,
            &B21.matrix,
            1.0f64,
            &mut A22.matrix,
        );
        gsl_blas_dsymm(
            CblasRight,
            CblasLower,
            -0.5f64,
            &mut A11.matrix,
            &B21.matrix,
            1.0f64,
            &mut A21.matrix,
        );
        gsl_blas_dtrsm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasNonUnit,
            1.0f64,
            &B22.matrix,
            &mut A21.matrix,
        );
        status = gensymm_standardize_L3(&mut A22.matrix, &B22.matrix);
        if status != 0 {
            return status;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
