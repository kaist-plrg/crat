use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_swap_elements(v: *mut gsl_vector, i: size_t, j: size_t) -> libc::c_int;
    fn gsl_vector_minmax(
        v: *const gsl_vector,
        min_out: *mut libc::c_double,
        max_out: *mut libc::c_double,
    );
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_const_submatrix(
        m: *const gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_const_view;
    fn gsl_matrix_const_diagonal(m: *const gsl_matrix) -> _gsl_vector_const_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_swap_columns(m: *mut gsl_matrix, i: size_t, j: size_t) -> libc::c_int;
    fn gsl_permutation_init(p: *mut gsl_permutation);
    fn gsl_permutation_swap(
        p: *mut gsl_permutation,
        i: size_t,
        j: size_t,
    ) -> libc::c_int;
    fn gsl_permute_vector_inverse(
        p: *const gsl_permutation,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
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
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
    fn gsl_linalg_householder_hm(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
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
    fn gsl_linalg_QR_unpack(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        Q: *mut gsl_matrix,
        R: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_tri_rcond(
        Uplo: CBLAS_UPLO_t,
        A: *const gsl_matrix,
        rcond: *mut libc::c_double,
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
unsafe extern "C" fn gsl_permutation_get(
    mut p: *const gsl_permutation,
    i: size_t,
) -> size_t {
    return *((*p).data).offset(i as isize);
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
unsafe extern "C" fn apply_givens_qr(
    mut M: size_t,
    mut N: size_t,
    mut Q: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut i: size_t,
    mut j: size_t,
    mut c: libc::c_double,
    mut s: libc::c_double,
) {
    let mut k: size_t = 0;
    k = 0 as libc::c_int as size_t;
    while k < M {
        let mut qki: libc::c_double = gsl_matrix_get(Q, k, i);
        let mut qkj: libc::c_double = gsl_matrix_get(Q, k, j);
        gsl_matrix_set(Q, k, i, qki * c - qkj * s);
        gsl_matrix_set(Q, k, j, qki * s + qkj * c);
        k = k.wrapping_add(1);
        k;
    }
    k = if i < j { i } else { j };
    while k < N {
        let mut rik: libc::c_double = gsl_matrix_get(R, i, k);
        let mut rjk: libc::c_double = gsl_matrix_get(R, j, k);
        gsl_matrix_set(R, i, k, c * rik - s * rjk);
        gsl_matrix_set(R, j, k, s * rik + c * rjk);
        k = k.wrapping_add(1);
        k;
    }
}
pub unsafe extern "C" fn gsl_linalg_QRPT_decomp(
    mut A: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
    mut p: *mut gsl_permutation,
    mut signum: *mut libc::c_int,
    mut norm: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*p).size != N {
        gsl_error(
            b"permutation size must be N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*norm).size != N {
        gsl_error(
            b"norm size must be N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        *signum = 1 as libc::c_int;
        gsl_permutation_init(p);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut c: gsl_vector_view = gsl_matrix_column(A, i);
            let mut x: libc::c_double = gsl_blas_dnrm2(&mut c.vector);
            gsl_vector_set(norm, i, x);
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < (if M < N { M } else { N }) {
            let mut max_norm: libc::c_double = gsl_vector_get(norm, i);
            let mut j: size_t = 0;
            let mut kmax: size_t = i;
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                let mut x_0: libc::c_double = gsl_vector_get(norm, j);
                if x_0 > max_norm {
                    max_norm = x_0;
                    kmax = j;
                }
                j = j.wrapping_add(1);
                j;
            }
            if kmax != i {
                gsl_matrix_swap_columns(A, i, kmax);
                gsl_permutation_swap(p, i, kmax);
                gsl_vector_swap_elements(norm, i, kmax);
                *signum = -*signum;
            }
            let mut c_full: gsl_vector_view = gsl_matrix_column(A, i);
            let mut c_0: gsl_vector_view = gsl_vector_subvector(
                &mut c_full.vector,
                i,
                M.wrapping_sub(i),
            );
            let mut tau_i: libc::c_double = gsl_linalg_householder_transform(
                &mut c_0.vector,
            );
            gsl_vector_set(tau, i, tau_i);
            if i.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
                let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    i,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    M.wrapping_sub(i),
                    N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                );
                gsl_linalg_householder_hm(tau_i, &mut c_0.vector, &mut m.matrix);
            }
            if i.wrapping_add(1 as libc::c_int as libc::c_ulong) < M {
                j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
                while j < N {
                    let mut x_1: libc::c_double = gsl_vector_get(norm, j);
                    if x_1 > 0.0f64 {
                        let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
                        let mut temp: libc::c_double = gsl_matrix_get(A, i, j) / x_1;
                        if fabs(temp) >= 1 as libc::c_int as libc::c_double {
                            y = 0.0f64;
                        } else {
                            y = x_1
                                * sqrt(1 as libc::c_int as libc::c_double - temp * temp);
                        }
                        if fabs(y / x_1) < sqrt(20.0f64) * 1.4901161193847656e-08f64 {
                            let mut c_full_0: gsl_vector_view = gsl_matrix_column(A, j);
                            let mut c_1: gsl_vector_view = gsl_vector_subvector(
                                &mut c_full_0.vector,
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                M
                                    .wrapping_sub(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ),
                            );
                            y = gsl_blas_dnrm2(&mut c_1.vector);
                        }
                        gsl_vector_set(norm, j, y);
                    }
                    j = j.wrapping_add(1);
                    j;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_decomp2(
    mut A: *const gsl_matrix,
    mut q: *mut gsl_matrix,
    mut r: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
    mut p: *mut gsl_permutation,
    mut signum: *mut libc::c_int,
    mut norm: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*q).size1 != M || (*q).size2 != M {
        gsl_error(
            b"q must be M x M\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*r).size1 != M || (*r).size2 != N {
        gsl_error(
            b"r must be M x N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*tau).size != (if M < N { M } else { N }) {
        gsl_error(
            b"size of tau must be MIN(M,N)\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*p).size != N {
        gsl_error(
            b"permutation size must be N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*norm).size != N {
        gsl_error(
            b"norm size must be N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_matrix_memcpy(r, A);
    gsl_linalg_QRPT_decomp(r, tau, p, signum, norm);
    gsl_linalg_QR_unpack(r, tau, q, r);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_QRPT_solve(
    mut QR: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*QR).size1 != (*QR).size2 {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            245 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*QR).size1 != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*QR).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*QR).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_memcpy(x, b);
        gsl_linalg_QRPT_svx(QR, tau, p, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_svx(
    mut QR: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut p: *const gsl_permutation,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*QR).size1 != (*QR).size2 {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*QR).size1 != (*p).size {
        gsl_error(
            b"matrix size must match permutation size\0" as *const u8
                as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*QR).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_linalg_QR_QTvec(QR, tau, x);
        gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, QR, x);
        gsl_permute_vector_inverse(p, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_lssolve(
    mut QR: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut residual: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*QR).size2;
    let mut status: libc::c_int = gsl_linalg_QRPT_lssolve2(
        QR,
        tau,
        p,
        b,
        N,
        x,
        residual,
    );
    return status;
}
pub unsafe extern "C" fn gsl_linalg_QRPT_lssolve2(
    mut QR: *const gsl_matrix,
    mut tau: *const gsl_vector,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    rank: size_t,
    mut x: *mut gsl_vector,
    mut residual: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"QR matrix must have M>=N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            337 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if rank == 0 as libc::c_int as libc::c_ulong || rank > N {
        gsl_error(
            b"rank must have 0 < rank <= N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8
                as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*residual).size {
        gsl_error(
            b"matrix size must match residual size\0" as *const u8
                as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            353 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let R11: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            QR,
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
        let mut i: size_t = 0;
        gsl_vector_memcpy(residual, b);
        gsl_linalg_QR_QTvec(QR, tau, residual);
        gsl_vector_memcpy(&mut x1.vector, &mut QTb1.vector);
        gsl_blas_dtrsv(
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            &R11.matrix,
            &mut x1.vector,
        );
        i = rank;
        while i < N {
            gsl_vector_set(x, i, 0.0f64);
            i = i.wrapping_add(1);
            i;
        }
        gsl_permute_vector_inverse(p, x);
        gsl_vector_set_zero(&mut QTb1.vector);
        gsl_linalg_QR_Qvec(QR, tau, residual);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_QRsolve(
    mut Q: *const gsl_matrix,
    mut R: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*Q).size1 != (*Q).size2 || (*R).size1 != (*R).size2 {
        return GSL_ENOTSQR as libc::c_int
    } else if (*Q).size1 != (*p).size || (*Q).size1 != (*R).size1
        || (*Q).size1 != (*b).size
    {
        return GSL_EBADLEN as libc::c_int
    } else {
        gsl_blas_dgemv(CblasTrans, 1.0f64, Q, b, 0.0f64, x);
        gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, R, x);
        gsl_permute_vector_inverse(p, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_Rsolve(
    mut QR: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*QR).size1 != (*QR).size2 {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            426 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*QR).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*QR).size2 != (*x).size {
        gsl_error(
            b"matrix size must match x size\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*p).size != (*x).size {
        gsl_error(
            b"permutation size must match x size\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_memcpy(x, b);
        gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, QR, x);
        gsl_permute_vector_inverse(p, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_Rsvx(
    mut QR: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*QR).size1 != (*QR).size2 {
        gsl_error(
            b"QR matrix must be square\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*QR).size2 != (*x).size {
        gsl_error(
            b"matrix size must match x size\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            468 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*p).size != (*x).size {
        gsl_error(
            b"permutation size must match x size\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            472 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, QR, x);
        gsl_permute_vector_inverse(p, x);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_update(
    mut Q: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut p: *const gsl_permutation,
    mut w: *mut gsl_vector,
    mut v: *const gsl_vector,
) -> libc::c_int {
    let M: size_t = (*R).size1;
    let N: size_t = (*R).size2;
    if (*Q).size1 != M || (*Q).size2 != M {
        gsl_error(
            b"Q matrix must be M x M if R is M x N\0" as *const u8
                as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*w).size != M {
        gsl_error(
            b"w must be length M if R is M x N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*v).size != N {
        gsl_error(
            b"v must be length N if R is M x N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            517 as libc::c_int,
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
            apply_givens_qr(
                M,
                N,
                Q,
                R,
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
            let mut r0j: libc::c_double = gsl_matrix_get(
                R,
                0 as libc::c_int as size_t,
                j,
            );
            let mut p_j: size_t = gsl_permutation_get(p, j);
            let mut vj: libc::c_double = gsl_vector_get(v, p_j);
            gsl_matrix_set(R, 0 as libc::c_int as size_t, j, r0j + w0 * vj);
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
                R,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut offdiag: libc::c_double = gsl_matrix_get(
                R,
                k,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_linalg_givens(diag, offdiag, &mut c_0, &mut s_0);
            apply_givens_qr(
                M,
                N,
                Q,
                R,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                k,
                c_0,
                s_0,
            );
            gsl_matrix_set(
                R,
                k,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0.0f64,
            );
            k = k.wrapping_add(1);
            k;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_QRPT_rank(
    mut QR: *const gsl_matrix,
    tol: libc::c_double,
) -> size_t {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    let diag: gsl_vector_const_view = gsl_matrix_const_diagonal(QR);
    let mut eps: libc::c_double = 0.;
    let mut i: size_t = 0;
    let mut r: size_t = 0 as libc::c_int as size_t;
    if tol < 0.0f64 {
        let mut min: libc::c_double = 0.;
        let mut max: libc::c_double = 0.;
        let mut absmax: libc::c_double = 0.;
        let mut ee: libc::c_int = 0;
        gsl_vector_minmax(&diag.vector, &mut min, &mut max);
        absmax = if fabs(min) > fabs(max) { fabs(min) } else { fabs(max) };
        ee = (log(absmax) / 0.69314718055994530942f64) as libc::c_int;
        eps = 20.0f64 * M.wrapping_add(N) as libc::c_double
            * pow(2.0f64, ee as libc::c_double) * 2.2204460492503131e-16f64;
    } else {
        eps = tol;
    }
    i = 0 as libc::c_int as size_t;
    while i < (if M < N { M } else { N }) {
        let mut di: libc::c_double = gsl_vector_get(&diag.vector, i);
        if fabs(di) > eps {
            r = r.wrapping_add(1);
            r;
        }
        i = i.wrapping_add(1);
        i;
    }
    return r;
}
pub unsafe extern "C" fn gsl_linalg_QRPT_rcond(
    mut QR: *const gsl_matrix,
    mut rcond: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*QR).size1;
    let N: size_t = (*QR).size2;
    if M < N {
        gsl_error(
            b"M must be >= N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            628 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != (3 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
        gsl_error(
            b"work vector must have length 3*N\0" as *const u8 as *const libc::c_char,
            b"qrpt.c\0" as *const u8 as *const libc::c_char,
            632 as libc::c_int,
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
        let mut status: libc::c_int = 0;
        status = gsl_linalg_tri_rcond(CblasUpper, &R.matrix, rcond, work);
        return status;
    };
}
