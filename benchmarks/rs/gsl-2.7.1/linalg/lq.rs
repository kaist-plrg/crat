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
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_const_subvector(
        v: *const gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
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
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_matrix_const_row(m: *const gsl_matrix, i: size_t) -> _gsl_vector_const_view;
    fn gsl_matrix_const_subrow(
        m: *const gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
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
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
    fn gsl_linalg_householder_mh(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_hv(
        tau: libc::c_double,
        v: *const gsl_vector,
        w: *mut gsl_vector,
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
unsafe extern "C" fn gsl_linalg_givens(
    a: libc::c_double,
    b: libc::c_double,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) {
    if b == 0 as libc::c_int as libc::c_double {
        *c = 1 as libc::c_int as libc::c_double;
        *s = 0 as libc::c_int as libc::c_double;
    } else if fabs(b) > fabs(a) {
        let mut t: libc::c_double = -a / b;
        let mut s1: libc::c_double = 1.0f64
            / sqrt(1 as libc::c_int as libc::c_double + t * t);
        *s = s1;
        *c = s1 * t;
    } else {
        let mut t_0: libc::c_double = -b / a;
        let mut c1: libc::c_double = 1.0f64
            / sqrt(1 as libc::c_int as libc::c_double + t_0 * t_0);
        *c = c1;
        *s = c1 * t_0;
    };
}
#[inline]
unsafe extern "C" fn gsl_linalg_givens_gv(
    mut v: *mut gsl_vector,
    i: size_t,
    j: size_t,
    c: libc::c_double,
    s: libc::c_double,
) {
    let mut vi: libc::c_double = gsl_vector_get(v, i);
    let mut vj: libc::c_double = gsl_vector_get(v, j);
    gsl_vector_set(v, i, c * vi - s * vj);
    gsl_vector_set(v, j, s * vi + c * vj);
}
#[inline]
unsafe extern "C" fn apply_givens_lq(
    mut M: size_t,
    mut N: size_t,
    mut Q: *mut gsl_matrix,
    mut L: *mut gsl_matrix,
    mut i: size_t,
    mut j: size_t,
    mut c: libc::c_double,
    mut s: libc::c_double,
) {
    let mut k: size_t = 0;
    k = 0 as libc::c_int as size_t;
    while k < M {
        let mut qik: libc::c_double = gsl_matrix_get(Q, i, k);
        let mut qjk: libc::c_double = gsl_matrix_get(Q, j, k);
        gsl_matrix_set(Q, i, k, qik * c - qjk * s);
        gsl_matrix_set(Q, j, k, qik * s + qjk * c);
        k = k.wrapping_add(1);
        k;
    }
    k = if i < j { i } else { j };
    while k < N {
        let mut lki: libc::c_double = gsl_matrix_get(L, k, i);
        let mut lkj: libc::c_double = gsl_matrix_get(L, k, j);
        gsl_matrix_set(L, k, i, c * lki - s * lkj);
        gsl_matrix_set(L, k, j, s * lki + c * lkj);
        k = k.wrapping_add(1);
        k;
    }
}
pub unsafe extern "C" fn gsl_linalg_LQ_decomp(
    mut A: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let M: size_t = (*A).size2;
    if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            let mut c: gsl_vector_view = gsl_matrix_subrow(A, i, i, M.wrapping_sub(i));
            let mut tau_i: libc::c_double = gsl_linalg_householder_transform(
                &mut c.vector,
            );
            gsl_vector_set(tau, i, tau_i);
            if i.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    i,
                    N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                    M.wrapping_sub(i),
                );
                gsl_linalg_householder_mh(tau_i, &mut c.vector, &mut m.matrix);
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_solve_T(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LQ).size1 != (*LQ).size2 {
        gsl_error(
            b"LQ matrix must be square\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LQ).size2 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LQ).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_memcpy(x, b);
        gsl_linalg_LQ_svx_T(LQ, tau, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_svx_T(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LQ).size1 != (*LQ).size2 {
        gsl_error(
            b"LQ matrix must be square\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LQ).size1 != (*x).size {
        gsl_error(
            b"matrix size must match x/rhs size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_linalg_LQ_vecQT(LQ, tau, x);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, LQ, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_lssolve_T(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut residual: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*LQ).size1;
    let M: size_t = (*LQ).size2;
    if M < N {
        gsl_error(
            b"LQ matrix must have M>=N\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*residual).size {
        gsl_error(
            b"matrix size must match residual size\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let L: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            LQ,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            N,
            N,
        );
        let mut c: gsl_vector_view = gsl_vector_subvector(
            residual,
            0 as libc::c_int as size_t,
            N,
        );
        gsl_vector_memcpy(residual, b);
        gsl_linalg_LQ_vecQT(LQ, tau, residual);
        gsl_vector_memcpy(x, &mut c.vector);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, &L.matrix, x);
        gsl_vector_set_zero(&mut c.vector);
        gsl_linalg_LQ_vecQ(LQ, tau, residual);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_Lsolve_T(
    mut LQ: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LQ).size1 != (*LQ).size2 {
        gsl_error(
            b"LQ matrix must be square\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LQ).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*LQ).size1 != (*x).size {
        gsl_error(
            b"matrix size must match x size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_memcpy(x, b);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, LQ, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_Lsvx_T(
    mut LQ: *const gsl_matrix,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*LQ).size1 != (*LQ).size2 {
        gsl_error(
            b"LQ matrix must be square\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*LQ).size2 != (*x).size {
        gsl_error(
            b"matrix size must match rhs size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, LQ, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_L_solve_T(
    mut L: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*L).size1 != (*L).size2 {
        gsl_error(
            b"R matrix must be square\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*L).size2 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_memcpy(x, b);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, L, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_vecQT(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*LQ).size1;
    let M: size_t = (*LQ).size2;
    if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*v).size != M {
        gsl_error(
            b"vector size must be M\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            let c: gsl_vector_const_view = gsl_matrix_const_row(LQ, i);
            let h: gsl_vector_const_view = gsl_vector_const_subvector(
                &c.vector,
                i,
                M.wrapping_sub(i),
            );
            let mut w: gsl_vector_view = gsl_vector_subvector(v, i, M.wrapping_sub(i));
            let mut ti: libc::c_double = gsl_vector_get(tau, i);
            gsl_linalg_householder_hv(ti, &h.vector, &mut w.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_vecQ(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*LQ).size1;
    let M: size_t = (*LQ).size2;
    if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            369 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*v).size != M {
        gsl_error(
            b"vector size must be M\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            373 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = if M < N { M } else { N };
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let c: gsl_vector_const_view = gsl_matrix_const_row(LQ, i);
            let h: gsl_vector_const_view = gsl_vector_const_subvector(
                &c.vector,
                i,
                M.wrapping_sub(i),
            );
            let mut w: gsl_vector_view = gsl_vector_subvector(v, i, M.wrapping_sub(i));
            let mut ti: libc::c_double = gsl_vector_get(tau, i);
            gsl_linalg_householder_hv(ti, &h.vector, &mut w.vector);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_unpack(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut Q: *mut gsl_matrix,
    mut L: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*LQ).size1;
    let N: size_t = (*LQ).size2;
    if (*Q).size1 != N || (*Q).size2 != N {
        gsl_error(
            b"Q matrix must be N x N\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*L).size1 != M || (*L).size2 != N {
        gsl_error(
            b"L matrix must be N x M\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut l_border: size_t = 0;
        gsl_matrix_set_identity(Q);
        i = if M < N { M } else { N };
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let h: gsl_vector_const_view = gsl_matrix_const_subrow(
                LQ,
                i,
                i,
                N.wrapping_sub(i),
            );
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                Q,
                i,
                i,
                N.wrapping_sub(i),
                N.wrapping_sub(i),
            );
            let mut ti: libc::c_double = gsl_vector_get(tau, i);
            gsl_linalg_householder_mh(ti, &h.vector, &mut m.matrix);
        }
        i = 0 as libc::c_int as size_t;
        while i < M {
            l_border = if i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                i
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            j = 0 as libc::c_int as size_t;
            while j <= l_border {
                gsl_matrix_set(L, i, j, gsl_matrix_get(LQ, i, j));
                j = j.wrapping_add(1);
                j;
            }
            j = l_border.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                gsl_matrix_set(L, i, j, 0.0f64);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_update(
    mut Q: *mut gsl_matrix,
    mut L: *mut gsl_matrix,
    mut v: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*L).size1;
    let M: size_t = (*L).size2;
    if (*Q).size1 != M || (*Q).size2 != M {
        gsl_error(
            b"Q matrix must be N x N if L is M x N\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            469 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*w).size != M {
        gsl_error(
            b"w must be length N if L is M x N\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*v).size != N {
        gsl_error(
            b"v must be length M if L is M x N\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            477 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        let mut w0: libc::c_double = 0.;
        k = M.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        while k > 0 as libc::c_int as libc::c_ulong {
            let mut c: libc::c_double = 0.;
            let mut s: libc::c_double = 0.;
            let mut wk: libc::c_double = gsl_vector_get(w, k);
            let mut wkm1: libc::c_double = gsl_vector_get(
                w,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_linalg_givens(wkm1, wk, &mut c, &mut s);
            gsl_linalg_givens_gv(
                w,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k,
                c,
                s,
            );
            apply_givens_lq(
                M,
                N,
                Q,
                L,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k,
                c,
                s,
            );
            k = k.wrapping_sub(1);
            k;
        }
        w0 = gsl_vector_get(w, 0 as libc::c_int as size_t);
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut lj0: libc::c_double = gsl_matrix_get(
                L,
                j,
                0 as libc::c_int as size_t,
            );
            let mut vj: libc::c_double = gsl_vector_get(v, j);
            gsl_matrix_set(L, j, 0 as libc::c_int as size_t, lj0 + w0 * vj);
            j = j.wrapping_add(1);
            j;
        }
        k = 1 as libc::c_int as size_t;
        while k
            < (if M < N.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                M
            } else {
                N.wrapping_add(1 as libc::c_int as libc::c_ulong)
            })
        {
            let mut c_0: libc::c_double = 0.;
            let mut s_0: libc::c_double = 0.;
            let mut diag: libc::c_double = gsl_matrix_get(
                L,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut offdiag: libc::c_double = gsl_matrix_get(
                L,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k,
            );
            gsl_linalg_givens(diag, offdiag, &mut c_0, &mut s_0);
            apply_givens_lq(
                M,
                N,
                Q,
                L,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k,
                c_0,
                s_0,
            );
            gsl_matrix_set(
                L,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k,
                0.0f64,
            );
            k = k.wrapping_add(1);
            k;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_LQsolve(
    mut Q: *mut gsl_matrix,
    mut L: *mut gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*L).size1;
    let M: size_t = (*L).size2;
    if M != N {
        return GSL_ENOTSQR as libc::c_int
    } else if (*Q).size1 != M || (*b).size != M || (*x).size != M {
        return GSL_EBADLEN as libc::c_int
    } else {
        gsl_blas_dgemv(CblasNoTrans, 1.0f64, Q, b, 0.0f64, x);
        gsl_blas_dtrsv(CblasLower, CblasTrans, CblasNonUnit, L, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_lssolve(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut residual: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*LQ).size1;
    let N: size_t = (*LQ).size2;
    if M > N {
        gsl_error(
            b"LQ matrix must have M<=N\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            577 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            585 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*residual).size {
        gsl_error(
            b"matrix size must match residual size\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            589 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let L1: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            LQ,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            M,
            M,
        );
        let mut x1: gsl_vector_view = gsl_vector_subvector(
            x,
            0 as libc::c_int as size_t,
            M,
        );
        let mut i: size_t = 0;
        gsl_vector_memcpy(&mut x1.vector, b);
        i = M;
        while i < N {
            gsl_vector_set(x, i, 0.0f64);
            i = i.wrapping_add(1);
            i;
        }
        gsl_blas_dtrsv(
            CblasLower,
            CblasNoTrans,
            CblasNonUnit,
            &L1.matrix,
            &mut x1.vector,
        );
        gsl_vector_memcpy(residual, &mut x1.vector);
        gsl_blas_dtrmv(CblasLower, CblasNoTrans, CblasNonUnit, &L1.matrix, residual);
        gsl_vector_sub(residual, b);
        gsl_vector_scale(residual, -1.0f64);
        gsl_linalg_LQ_QTvec(LQ, tau, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_LQ_QTvec(
    mut LQ: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*LQ).size1;
    let N: size_t = (*LQ).size2;
    let v_size: size_t = (*v).size;
    if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            637 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if v_size < (if M < N { M } else { N }) {
        gsl_error(
            b"vector size must be at least MIN(M,N)\0" as *const u8
                as *const libc::c_char,
            b"lq.c\0" as *const u8 as *const libc::c_char,
            641 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = if M < N { M } else { N };
        loop {
            let fresh2 = i;
            i = i.wrapping_sub(1);
            if !(fresh2 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let h: gsl_vector_const_view = gsl_matrix_const_subrow(
                LQ,
                i,
                i,
                v_size.wrapping_sub(i),
            );
            let mut w: gsl_vector_view = gsl_vector_subvector(
                v,
                i,
                v_size.wrapping_sub(i),
            );
            let mut ti: libc::c_double = gsl_vector_get(tau, i);
            gsl_linalg_householder_hv(ti, &h.vector, &mut w.vector);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
