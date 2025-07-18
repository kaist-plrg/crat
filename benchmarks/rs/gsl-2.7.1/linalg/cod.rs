use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
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
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
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
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_matrix_const_diagonal(m: *const gsl_matrix) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subrow(
        m: *const gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subcolumn(
        m: *const gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_matrix_tricpy(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_matrix_transpose_tricpy(
        Uplo_src: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_permute_vector_inverse(
        p: *const gsl_permutation,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
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
    fn gsl_blas_dger(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_left(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QR_QTvec(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QR_Qvec(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QRPT_decomp(
        A: *mut gsl_matrix,
        tau: *mut gsl_vector,
        p: *mut gsl_permutation,
        signum: *mut libc::c_int,
        norm: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QRPT_rank(QR: *const gsl_matrix, tol: libc::c_double) -> size_t;
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
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
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
pub unsafe extern "C" fn gsl_linalg_COD_decomp_e(
    mut A: *mut gsl_matrix,
    mut tau_Q: *mut gsl_vector,
    mut tau_Z: *mut gsl_vector,
    mut p: *mut gsl_permutation,
    mut tol: libc::c_double,
    mut rank: *mut size_t,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*tau_Q).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau_Q must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*tau_Z).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau_Z must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*p).size != N {
        gsl_error(
            b"permutation size must be N\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != N {
        gsl_error(
            b"work size must be N\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut signum: libc::c_int = 0;
        let mut r: size_t = 0;
        status = gsl_linalg_QRPT_decomp(A, tau_Q, p, &mut signum, work);
        if status != 0 {
            return status;
        }
        r = gsl_linalg_QRPT_rank(A, tol);
        if r < N {
            let mut R_upper: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                r,
                N,
            );
            let mut t: gsl_vector_view = gsl_vector_subvector(
                tau_Z,
                0 as libc::c_int as size_t,
                r,
            );
            cod_RZ(&mut R_upper.matrix, &mut t.vector);
        }
        *rank = r;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_COD_decomp(
    mut A: *mut gsl_matrix,
    mut tau_Q: *mut gsl_vector,
    mut tau_Z: *mut gsl_vector,
    mut p: *mut gsl_permutation,
    mut rank: *mut size_t,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    return gsl_linalg_COD_decomp_e(A, tau_Q, tau_Z, p, -1.0f64, rank, work);
}
pub unsafe extern "C" fn gsl_linalg_COD_lssolve(
    mut QRZT: *const gsl_matrix,
    mut tau_Q: *const gsl_vector,
    mut tau_Z: *const gsl_vector,
    mut perm: *const gsl_permutation,
    rank: size_t,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut residual: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*QRZT).size1;
    let N: size_t = (*QRZT).size2;
    if M < N {
        gsl_error(
            b"QRZT matrix must have M>=N\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if rank > (if M < N { M } else { N }) {
        gsl_error(
            b"rank must be <= MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*residual).size {
        gsl_error(
            b"matrix size must match residual size\0" as *const u8
                as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let R11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QRZT,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            rank,
            rank,
        );
        let mut QTb1: gsl_vector_view = gsl_vector_subvector(
            residual,
            0 as libc::c_int as size_t,
            rank,
        );
        let mut x1: gsl_vector_view = gsl_vector_subvector(
            x,
            0 as libc::c_int as size_t,
            rank,
        );
        gsl_vector_set_zero(x);
        gsl_vector_memcpy(residual, b);
        gsl_linalg_QR_QTvec(QRZT, tau_Q, residual);
        gsl_vector_memcpy(&mut x1.vector, &mut QTb1.vector);
        gsl_blas_dtrsv(
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            &R11.matrix,
            &mut x1.vector,
        );
        cod_householder_Zvec(QRZT, tau_Z, rank, x);
        gsl_permute_vector_inverse(perm, x);
        gsl_vector_set_zero(&mut QTb1.vector);
        gsl_linalg_QR_Qvec(QRZT, tau_Q, residual);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_COD_lssolve2(
    lambda: libc::c_double,
    mut QRZT: *const gsl_matrix,
    mut tau_Q: *const gsl_vector,
    mut tau_Z: *const gsl_vector,
    mut perm: *const gsl_permutation,
    rank: size_t,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut residual: *mut gsl_vector,
    mut S: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*QRZT).size1;
    let N: size_t = (*QRZT).size2;
    if M < N {
        gsl_error(
            b"QRZT matrix must have M>=N\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if rank > (if M < N { M } else { N }) {
        gsl_error(
            b"rank must be <= MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            245 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*residual).size {
        gsl_error(
            b"matrix size must match residual size\0" as *const u8
                as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*S).size1 != rank || (*S).size2 != rank {
        gsl_error(
            b"S must be rank-by-rank\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != rank {
        gsl_error(
            b"work must be length rank\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let R11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QRZT,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            rank,
            rank,
        );
        let mut c1: gsl_vector_view = gsl_vector_subvector(
            residual,
            0 as libc::c_int as size_t,
            rank,
        );
        let mut y1: gsl_vector_view = gsl_vector_subvector(
            x,
            0 as libc::c_int as size_t,
            rank,
        );
        gsl_vector_set_zero(x);
        gsl_vector_memcpy(residual, b);
        gsl_linalg_QR_QTvec(QRZT, tau_Q, residual);
        cod_trireg_solve(&R11.matrix, lambda, &mut c1.vector, S, &mut y1.vector, work);
        gsl_vector_memcpy(work, &mut y1.vector);
        cod_householder_Zvec(QRZT, tau_Z, rank, x);
        gsl_permute_vector_inverse(perm, x);
        gsl_blas_dtrmv(CblasUpper, CblasNoTrans, CblasNonUnit, &R11.matrix, work);
        gsl_vector_sub(&mut c1.vector, work);
        gsl_linalg_QR_Qvec(QRZT, tau_Q, residual);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_COD_unpack(
    mut QRZT: *const gsl_matrix,
    mut tau_Q: *const gsl_vector,
    mut tau_Z: *const gsl_vector,
    rank: size_t,
    mut Q: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut Z: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*QRZT).size1;
    let N: size_t = (*QRZT).size2;
    if (*tau_Q).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau_Q must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*tau_Z).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau_Z must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if rank > (if M < N { M } else { N }) {
        gsl_error(
            b"rank must be <= MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            326 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Q).size1 != M || (*Q).size2 != M {
        gsl_error(
            b"Q must by M-by-M\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*R).size1 != M || (*R).size2 != N {
        gsl_error(
            b"R must by M-by-N\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Z).size1 != N || (*Z).size2 != N {
        gsl_error(
            b"Z must by N-by-N\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut R11: gsl_matrix_view = gsl_matrix_submatrix(
            R,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            rank,
            rank,
        );
        let QRZT11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QRZT,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            rank,
            rank,
        );
        gsl_matrix_set_identity(Q);
        i = if M < N { M } else { N };
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let h: gsl_vector_const_view = gsl_matrix_const_subcolumn(
                QRZT,
                i,
                i,
                M.wrapping_sub(i),
            );
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                Q,
                i,
                i,
                M.wrapping_sub(i),
                M.wrapping_sub(i),
            );
            let mut work: gsl_vector_view = gsl_matrix_subcolumn(
                R,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                M.wrapping_sub(i),
            );
            let mut ti: libc::c_double = gsl_vector_get(tau_Q, i);
            let mut ptr: *mut libc::c_double = gsl_vector_ptr(
                &h.vector as *const gsl_vector as *mut gsl_vector,
                0 as libc::c_int as size_t,
            );
            let mut tmp: libc::c_double = *ptr;
            *ptr = 1.0f64;
            gsl_linalg_householder_left(ti, &h.vector, &mut m.matrix, &mut work.vector);
            *ptr = tmp;
        }
        gsl_matrix_set_identity(Z);
        if rank < N {
            let mut work_0: gsl_vector_view = gsl_matrix_row(
                R,
                0 as libc::c_int as size_t,
            );
            gsl_linalg_COD_matZ(QRZT, tau_Z, rank, Z, &mut work_0.vector);
        }
        gsl_matrix_set_zero(R);
        gsl_matrix_tricpy(CblasUpper, CblasNonUnit, &mut R11.matrix, &QRZT11.matrix);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_COD_matZ(
    mut QRZT: *const gsl_matrix,
    mut tau_Z: *const gsl_vector,
    rank: size_t,
    mut A: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*tau_Z).size
        != (if (*QRZT).size1 < (*QRZT).size2 { (*QRZT).size1 } else { (*QRZT).size2 })
    {
        gsl_error(
            b"tau_Z must be GSL_MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            404 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*QRZT).size2 != N {
        gsl_error(
            b"QRZT must have N columns\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != M {
        gsl_error(
            b"workspace must be length M\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        if rank < N {
            let mut i: size_t = 0;
            i = rank;
            while i > 0 as libc::c_int as libc::c_ulong
                && {
                    let fresh1 = i;
                    i = i.wrapping_sub(1);
                    fresh1 != 0
                }
            {
                let h: gsl_vector_const_view = gsl_matrix_const_subrow(
                    QRZT,
                    i,
                    rank,
                    N.wrapping_sub(rank),
                );
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    0 as libc::c_int as size_t,
                    i,
                    M,
                    N.wrapping_sub(i),
                );
                let mut ti: libc::c_double = gsl_vector_get(tau_Z, i);
                cod_householder_mh(ti, &h.vector, &mut m.matrix, work);
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cod_RZ(
    mut A: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*tau).size != M {
        gsl_error(
            b"tau has wrong size\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            466 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N < M {
        gsl_error(
            b"N must be >= M\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            470 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if M == N {
        gsl_vector_set_all(tau, 0.0f64);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut k: size_t = 0;
        k = M;
        while k > 0 as libc::c_int as libc::c_ulong
            && {
                let fresh2 = k;
                k = k.wrapping_sub(1);
                fresh2 != 0
            }
        {
            let mut alpha: *mut libc::c_double = gsl_matrix_ptr(A, k, k);
            let mut z: gsl_vector_view = gsl_matrix_subrow(A, k, M, N.wrapping_sub(M));
            let mut tauk: libc::c_double = 0.;
            tauk = cod_householder_transform(alpha, &mut z.vector);
            gsl_vector_set(tau, k, tauk);
            if tauk != 0 as libc::c_int as libc::c_double
                && k > 0 as libc::c_int as libc::c_ulong
            {
                let mut w: gsl_vector_view = gsl_vector_subvector(
                    tau,
                    0 as libc::c_int as size_t,
                    k,
                );
                let mut B: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    0 as libc::c_int as size_t,
                    k,
                    k,
                    N.wrapping_sub(k),
                );
                cod_householder_mh(tauk, &mut z.vector, &mut B.matrix, &mut w.vector);
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cod_householder_transform(
    mut alpha: *mut libc::c_double,
    mut v: *mut gsl_vector,
) -> libc::c_double {
    let mut beta: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut xnorm: libc::c_double = gsl_blas_dnrm2(v);
    if xnorm == 0 as libc::c_int as libc::c_double {
        return 0.0f64;
    }
    beta = -(if *alpha >= 0.0f64 { 1.0f64 } else { -1.0f64 }) * gsl_hypot(*alpha, xnorm);
    tau = (beta - *alpha) / beta;
    let mut s: libc::c_double = *alpha - beta;
    if fabs(s) > 2.2250738585072014e-308f64 {
        gsl_blas_dscal(1.0f64 / s, v);
    } else {
        gsl_blas_dscal(2.2204460492503131e-16f64 / s, v);
        gsl_blas_dscal(1.0f64 / 2.2204460492503131e-16f64, v);
    }
    *alpha = beta;
    return tau;
}
unsafe extern "C" fn cod_householder_hv(
    tau: libc::c_double,
    mut v: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    if tau == 0 as libc::c_int as libc::c_double {
        return GSL_SUCCESS as libc::c_int
    } else {
        let M: size_t = (*w).size;
        let L: size_t = (*v).size;
        let mut w0: libc::c_double = gsl_vector_get(w, 0 as libc::c_int as size_t);
        let mut w1: gsl_vector_view = gsl_vector_subvector(w, M.wrapping_sub(L), L);
        let mut d1: libc::c_double = 0.;
        let mut d: libc::c_double = 0.;
        gsl_blas_ddot(v, &mut w1.vector, &mut d1);
        d = w0 + d1;
        gsl_vector_set(w, 0 as libc::c_int as size_t, w0 - tau * d);
        gsl_blas_daxpy(-tau * d, v, &mut w1.vector);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cod_householder_mh(
    tau: libc::c_double,
    mut v: *const gsl_vector,
    mut A: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    if tau == 0 as libc::c_int as libc::c_double {
        return GSL_SUCCESS as libc::c_int
    } else {
        let M: size_t = (*A).size1;
        let N: size_t = (*A).size2;
        let L: size_t = (*v).size;
        let mut A1: gsl_vector_view = gsl_matrix_subcolumn(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
        );
        let mut C: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as libc::c_int as size_t,
            N.wrapping_sub(L),
            M,
            L,
        );
        gsl_vector_memcpy(work, &mut A1.vector);
        gsl_blas_dgemv(CblasNoTrans, 1.0f64, &mut C.matrix, v, 1.0f64, work);
        gsl_blas_daxpy(-tau, work, &mut A1.vector);
        gsl_blas_dger(-tau, work, v, &mut C.matrix);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cod_householder_Zvec(
    mut QRZT: *const gsl_matrix,
    mut tau_Z: *const gsl_vector,
    rank: size_t,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*QRZT).size1;
    let N: size_t = (*QRZT).size2;
    if (*tau_Z).size != (if M < N { M } else { N }) {
        gsl_error(
            b"tau_Z must be GSL_MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            649 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*v).size != N {
        gsl_error(
            b"v must be length N\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            653 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        if rank < N {
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < rank {
                let h: gsl_vector_const_view = gsl_matrix_const_subrow(
                    QRZT,
                    i,
                    rank,
                    N.wrapping_sub(rank),
                );
                let mut w: gsl_vector_view = gsl_vector_subvector(
                    v,
                    i,
                    N.wrapping_sub(i),
                );
                let mut ti: libc::c_double = gsl_vector_get(tau_Z, i);
                cod_householder_hv(ti, &h.vector, &mut w.vector);
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn cod_trireg_solve(
    mut R: *const gsl_matrix,
    lambda: libc::c_double,
    mut b: *const gsl_vector,
    mut S: *mut gsl_matrix,
    mut x: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*R).size2;
    let diag: gsl_vector_const_view = gsl_matrix_const_diagonal(R);
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if lambda <= 0.0f64 {
        gsl_error(
            b"lambda must be positive\0" as *const u8 as *const libc::c_char,
            b"cod.c\0" as *const u8 as *const libc::c_char,
            712 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    gsl_matrix_transpose_tricpy(CblasUpper, CblasUnit, S, R);
    gsl_vector_memcpy(work, &diag.vector);
    gsl_vector_memcpy(x, b);
    j = 0 as libc::c_int as size_t;
    while j < N {
        let mut bj: libc::c_double = 0.0f64;
        gsl_matrix_set(S, j, j, lambda);
        k = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while k < N {
            gsl_matrix_set(S, k, k, 0.0f64);
            k = k.wrapping_add(1);
            k;
        }
        k = j;
        while k < N {
            let mut sine: libc::c_double = 0.;
            let mut cosine: libc::c_double = 0.;
            let mut xk: libc::c_double = gsl_vector_get(x, k);
            let mut rkk: libc::c_double = gsl_vector_get(work, k);
            let mut skk: libc::c_double = gsl_matrix_get(S, k, k);
            if !(skk == 0 as libc::c_int as libc::c_double) {
                if fabs(rkk) < fabs(skk) {
                    let mut cotangent: libc::c_double = rkk / skk;
                    sine = 0.5f64 / sqrt(0.25f64 + 0.25f64 * cotangent * cotangent);
                    cosine = sine * cotangent;
                } else {
                    let mut tangent: libc::c_double = skk / rkk;
                    cosine = 0.5f64 / sqrt(0.25f64 + 0.25f64 * tangent * tangent);
                    sine = cosine * tangent;
                }
                let mut new_rkk: libc::c_double = cosine * rkk + sine * skk;
                let mut new_xk: libc::c_double = cosine * xk + sine * bj;
                bj = -sine * xk + cosine * bj;
                gsl_vector_set(work, k, new_rkk);
                gsl_matrix_set(S, k, k, new_rkk);
                gsl_vector_set(x, k, new_xk);
                i = k.wrapping_add(1 as libc::c_int as libc::c_ulong);
                while i < N {
                    let mut sik: libc::c_double = gsl_matrix_get(S, i, k);
                    let mut sii: libc::c_double = gsl_matrix_get(S, i, i);
                    let mut new_sik: libc::c_double = cosine * sik + sine * sii;
                    let mut new_sii: libc::c_double = -sine * sik + cosine * sii;
                    gsl_matrix_set(S, i, k, new_sik);
                    gsl_matrix_set(S, i, i, new_sii);
                    i = i.wrapping_add(1);
                    i;
                }
            }
            k = k.wrapping_add(1);
            k;
        }
        j = j.wrapping_add(1);
        j;
    }
    gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, S, x);
    return GSL_SUCCESS as libc::c_int;
}
