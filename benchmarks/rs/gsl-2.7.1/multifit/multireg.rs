use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
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
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_add(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_vector_add_constant(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
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
    fn gsl_matrix_superdiagonal(m: *mut gsl_matrix, k: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_view_array(
        base: *mut libc::c_double,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
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
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_transpose_memcpy(
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_matrix_transpose_tricpy(
        Uplo_src: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_matrix_scale(a: *mut gsl_matrix, x: libc::c_double) -> libc::c_int;
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
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_decomp(A: *mut gsl_matrix, tau: *mut gsl_vector) -> libc::c_int;
    fn gsl_linalg_QR_QTvec(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QR_QTmat(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_matQ(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_Qvec(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_decomp(A: *mut gsl_matrix) -> libc::c_int;
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
pub struct gsl_multifit_linear_workspace {
    pub nmax: size_t,
    pub pmax: size_t,
    pub n: size_t,
    pub p: size_t,
    pub A: *mut gsl_matrix,
    pub Q: *mut gsl_matrix,
    pub QSI: *mut gsl_matrix,
    pub S: *mut gsl_vector,
    pub t: *mut gsl_vector,
    pub xt: *mut gsl_vector,
    pub D: *mut gsl_vector,
    pub rcond: libc::c_double,
}
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
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
unsafe extern "C" fn multifit_linear_solve(
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    tol: libc::c_double,
    lambda: libc::c_double,
    mut rank: *mut size_t,
    mut c: *mut gsl_vector,
    mut rnorm: *mut libc::c_double,
    mut snorm: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if n != (*work).n || p != (*work).p {
        gsl_error(
            b"observation matrix does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"./linear_common.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*y).size {
        gsl_error(
            b"number of observations in y does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./linear_common.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*c).size {
        gsl_error(
            b"number of parameters c does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"./linear_common.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if tol <= 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"tolerance must be positive\0" as *const u8 as *const libc::c_char,
            b"./linear_common.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let lambda_sq: libc::c_double = lambda * lambda;
        let mut rho_ls: libc::c_double = 0.0f64;
        let mut j: size_t = 0;
        let mut p_eff: size_t = 0;
        let mut A: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            n,
            p,
        );
        let mut Q: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).Q,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p,
            p,
        );
        let mut S: gsl_vector_view = gsl_vector_subvector(
            (*work).S,
            0 as libc::c_int as size_t,
            p,
        );
        let mut QSI: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).QSI,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p,
            p,
        );
        let mut xt: gsl_vector_view = gsl_vector_subvector(
            (*work).xt,
            0 as libc::c_int as size_t,
            p,
        );
        let mut D: gsl_vector_view = gsl_vector_subvector(
            (*work).D,
            0 as libc::c_int as size_t,
            p,
        );
        let mut t: gsl_vector_view = gsl_vector_subvector(
            (*work).t,
            0 as libc::c_int as size_t,
            n,
        );
        gsl_blas_dgemv(CblasTrans, 1.0f64, &mut A.matrix, y, 0.0f64, &mut xt.vector);
        if n > p {
            gsl_vector_memcpy(&mut t.vector, y);
            gsl_blas_dgemv(
                CblasNoTrans,
                -1.0f64,
                &mut A.matrix,
                &mut xt.vector,
                1.0f64,
                &mut t.vector,
            );
            rho_ls = gsl_blas_dnrm2(&mut t.vector);
        }
        if lambda > 0.0f64 {
            j = 0 as libc::c_int as size_t;
            while j < p {
                let mut sj: libc::c_double = gsl_vector_get(&mut S.vector, j);
                let mut f: libc::c_double = sj * sj / (sj * sj + lambda_sq);
                let mut ptr: *mut libc::c_double = gsl_vector_ptr(&mut xt.vector, j);
                gsl_vector_set(&mut D.vector, j, (1.0f64 - f) * *ptr);
                *ptr *= sj / (sj * sj + lambda_sq);
                j = j.wrapping_add(1);
                j;
            }
            gsl_blas_dgemv(
                CblasNoTrans,
                1.0f64,
                &mut Q.matrix,
                &mut xt.vector,
                0.0f64,
                c,
            );
            *snorm = gsl_blas_dnrm2(c);
            *rnorm = gsl_blas_dnrm2(&mut D.vector);
            if n > p {
                *rnorm = sqrt(*rnorm * *rnorm + rho_ls * rho_ls);
            }
            gsl_vector_set_all(&mut D.vector, 1.0f64);
        } else {
            gsl_matrix_memcpy(&mut QSI.matrix, &mut Q.matrix);
            let mut s0: libc::c_double = gsl_vector_get(
                &mut S.vector,
                0 as libc::c_int as size_t,
            );
            p_eff = 0 as libc::c_int as size_t;
            j = 0 as libc::c_int as size_t;
            while j < p {
                let mut column: gsl_vector_view = gsl_matrix_column(&mut QSI.matrix, j);
                let mut sj_0: libc::c_double = gsl_vector_get(&mut S.vector, j);
                let mut alpha: libc::c_double = 0.;
                if sj_0 <= tol * s0 {
                    alpha = 0.0f64;
                } else {
                    alpha = 1.0f64 / sj_0;
                    p_eff = p_eff.wrapping_add(1);
                    p_eff;
                }
                gsl_vector_scale(&mut column.vector, alpha);
                j = j.wrapping_add(1);
                j;
            }
            *rank = p_eff;
            gsl_blas_dgemv(
                CblasNoTrans,
                1.0f64,
                &mut QSI.matrix,
                &mut xt.vector,
                0.0f64,
                c,
            );
            gsl_vector_div(c, &mut D.vector);
            *snorm = gsl_blas_dnrm2(c);
            *rnorm = rho_ls;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_solve(
    lambda: libc::c_double,
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut c: *mut gsl_vector,
    mut rnorm: *mut libc::c_double,
    mut snorm: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let mut rank: size_t = 0;
    let mut status: libc::c_int = 0;
    status = multifit_linear_solve(
        X,
        y,
        2.2204460492503131e-16f64,
        lambda,
        &mut rank,
        c,
        rnorm,
        snorm,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_multifit_linear_applyW(
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut WX: *mut gsl_matrix,
    mut Wy: *mut gsl_vector,
) -> libc::c_int {
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if n != (*y).size {
        gsl_error(
            b"y vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !w.is_null() && n != (*w).size {
        gsl_error(
            b"weight vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*WX).size1 || p != (*WX).size2 {
        gsl_error(
            b"WX matrix dimensions do not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*Wy).size {
        gsl_error(
            b"Wy vector must be length n\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        if WX != X as *mut gsl_matrix {
            gsl_matrix_memcpy(WX, X);
        }
        if Wy != y as *mut gsl_vector {
            gsl_vector_memcpy(Wy, y);
        }
        if !w.is_null() {
            i = 0 as libc::c_int as size_t;
            while i < n {
                let mut wi: libc::c_double = gsl_vector_get(w, i);
                let mut swi: libc::c_double = 0.;
                let mut row: gsl_vector_view = gsl_matrix_row(WX, i);
                let mut yi: *mut libc::c_double = gsl_vector_ptr(Wy, i);
                if wi < 0.0f64 {
                    wi = 0.0f64;
                }
                swi = sqrt(wi);
                gsl_vector_scale(&mut row.vector, swi);
                *yi *= swi;
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_wstdform1(
    mut L: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if n > (*work).nmax || p > (*work).pmax {
        gsl_error(
            b"observation matrix larger than workspace\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !L.is_null() && p != (*L).size {
        gsl_error(
            b"L vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*y).size {
        gsl_error(
            b"y vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !w.is_null() && n != (*w).size {
        gsl_error(
            b"weight vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*Xs).size1 || p != (*Xs).size2 {
        gsl_error(
            b"Xs matrix dimensions do not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*ys).size {
        gsl_error(
            b"ys vector must be length n\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        status = gsl_multifit_linear_applyW(X, w, y, Xs, ys);
        if status != 0 {
            return status;
        }
        if !L.is_null() {
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < p {
                let mut Xj: gsl_vector_view = gsl_matrix_column(Xs, j);
                let mut lj: libc::c_double = gsl_vector_get(L, j);
                if lj == 0.0f64 {
                    gsl_error(
                        b"L matrix is singular\0" as *const u8 as *const libc::c_char,
                        b"multireg.c\0" as *const u8 as *const libc::c_char,
                        214 as libc::c_int,
                        GSL_EDOM as libc::c_int,
                    );
                    return GSL_EDOM as libc::c_int;
                }
                gsl_vector_scale(&mut Xj.vector, 1.0f64 / lj);
                j = j.wrapping_add(1);
                j;
            }
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_stdform1(
    mut L: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = gsl_multifit_linear_wstdform1(
        L,
        X,
        0 as *const gsl_vector,
        y,
        Xs,
        ys,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_multifit_linear_L_decomp(
    mut L: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    let m: size_t = (*L).size1;
    let p: size_t = (*L).size2;
    let mut status: libc::c_int = 0;
    if (*tau).size != (if m < p { m } else { p }) {
        gsl_error(
            b"tau vector must be min(m,p)\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if m >= p {
        status = gsl_linalg_QR_decomp(L, tau);
        return status;
    } else {
        let mut LTQR: gsl_matrix_view = gsl_matrix_view_array((*L).data, p, m);
        let mut LT: *mut gsl_matrix = gsl_matrix_alloc(p, m);
        gsl_matrix_transpose_memcpy(LT, L);
        gsl_matrix_memcpy(&mut LTQR.matrix, LT);
        gsl_matrix_free(LT);
        status = gsl_linalg_QR_decomp(&mut LTQR.matrix, tau);
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_wstdform2(
    mut LQR: *const gsl_matrix,
    mut Ltau: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut M: *mut gsl_matrix,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let m: size_t = (*LQR).size1;
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if n > (*work).nmax || p > (*work).pmax {
        gsl_error(
            b"observation matrix larger than workspace\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*LQR).size2 {
        gsl_error(
            b"LQR and X matrices have different numbers of columns\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*y).size {
        gsl_error(
            b"y vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            370 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !w.is_null() && n != (*w).size {
        gsl_error(
            b"weights vector must be length n\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            374 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if m >= p {
        if n != (*Xs).size1 || p != (*Xs).size2 {
            gsl_error(
                b"Xs matrix must be n-by-p\0" as *const u8 as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                381 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return GSL_EBADLEN as libc::c_int;
        } else if n != (*ys).size {
            gsl_error(
                b"ys vector must have length n\0" as *const u8 as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                385 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return GSL_EBADLEN as libc::c_int;
        } else {
            let mut status: libc::c_int = 0;
            let mut i: size_t = 0;
            let R: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                LQR,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                p,
                p,
            );
            status = gsl_multifit_linear_applyW(X, w, y, Xs, ys);
            if status != 0 {
                return status;
            }
            i = 0 as libc::c_int as size_t;
            while i < n {
                let mut v: gsl_vector_view = gsl_matrix_row(Xs, i);
                gsl_blas_dtrsv(
                    CblasUpper,
                    CblasTrans,
                    CblasNonUnit,
                    &R.matrix,
                    &mut v.vector,
                );
                i = i.wrapping_add(1);
                i;
            }
            return GSL_SUCCESS as libc::c_int;
        }
    } else {
        let pm: size_t = p.wrapping_sub(m);
        let npm: size_t = n.wrapping_sub(pm);
        if npm != (*Xs).size1 || m != (*Xs).size2 {
            gsl_error(
                b"Xs matrix must be (n-p+m)-by-m\0" as *const u8 as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                422 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return GSL_EBADLEN as libc::c_int;
        } else if npm != (*ys).size {
            gsl_error(
                b"ys vector must be of length (n-p+m)\0" as *const u8
                    as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                426 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return GSL_EBADLEN as libc::c_int;
        } else if n != (*M).size1 || p != (*M).size2 {
            gsl_error(
                b"M matrix must be n-by-p\0" as *const u8 as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                430 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return GSL_EBADLEN as libc::c_int;
        } else {
            let mut status_0: libc::c_int = 0;
            let mut A: gsl_matrix_view = gsl_matrix_submatrix(
                (*work).A,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                n,
                p,
            );
            let mut b: gsl_vector_view = gsl_vector_subvector(
                (*work).t,
                0 as libc::c_int as size_t,
                n,
            );
            let mut LTQR: gsl_matrix_view = gsl_matrix_view_array((*LQR).data, p, m);
            let mut Rp: gsl_matrix_view = gsl_matrix_view_array((*LQR).data, m, m);
            let LTtau: gsl_vector_const_view = gsl_vector_const_subvector(
                Ltau,
                0 as libc::c_int as size_t,
                m,
            );
            let mut MQR: gsl_matrix_view = gsl_matrix_submatrix(
                M,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                n,
                pm,
            );
            let mut Mtau: gsl_vector_view = gsl_matrix_subcolumn(
                M,
                p.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                if n < pm { n } else { pm },
            );
            let mut AKo: gsl_matrix_view = gsl_matrix_view {
                matrix: gsl_matrix {
                    size1: 0,
                    size2: 0,
                    tda: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut AKp: gsl_matrix_view = gsl_matrix_view {
                matrix: gsl_matrix {
                    size1: 0,
                    size2: 0,
                    tda: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut HqTAKp: gsl_matrix_view = gsl_matrix_view {
                matrix: gsl_matrix {
                    size1: 0,
                    size2: 0,
                    tda: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut v_0: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut i_0: size_t = 0;
            status_0 = gsl_multifit_linear_applyW(X, w, y, &mut A.matrix, &mut b.vector);
            if status_0 != 0 {
                return status_0;
            }
            gsl_linalg_QR_matQ(&mut LTQR.matrix, &LTtau.vector, &mut A.matrix);
            AKp = gsl_matrix_submatrix(
                &mut A.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                n,
                m,
            );
            AKo = gsl_matrix_submatrix(
                &mut A.matrix,
                0 as libc::c_int as size_t,
                m,
                n,
                pm,
            );
            gsl_matrix_memcpy(&mut MQR.matrix, &mut AKo.matrix);
            gsl_linalg_QR_decomp(&mut MQR.matrix, &mut Mtau.vector);
            gsl_linalg_QR_QTmat(&mut MQR.matrix, &mut Mtau.vector, &mut AKp.matrix);
            HqTAKp = gsl_matrix_submatrix(
                &mut AKp.matrix,
                pm,
                0 as libc::c_int as size_t,
                npm,
                m,
            );
            gsl_matrix_memcpy(Xs, &mut HqTAKp.matrix);
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < npm {
                let mut x: gsl_vector_view = gsl_matrix_row(Xs, i_0);
                gsl_blas_dtrsv(
                    CblasUpper,
                    CblasNoTrans,
                    CblasNonUnit,
                    &mut Rp.matrix,
                    &mut x.vector,
                );
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            v_0 = gsl_vector_subvector(&mut b.vector, pm, npm);
            gsl_linalg_QR_QTvec(&mut MQR.matrix, &mut Mtau.vector, &mut b.vector);
            gsl_vector_memcpy(ys, &mut v_0.vector);
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_stdform2(
    mut LQR: *const gsl_matrix,
    mut Ltau: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut Xs: *mut gsl_matrix,
    mut ys: *mut gsl_vector,
    mut M: *mut gsl_matrix,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = gsl_multifit_linear_wstdform2(
        LQR,
        Ltau,
        X,
        0 as *const gsl_vector,
        y,
        Xs,
        ys,
        M,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_multifit_linear_genform1(
    mut L: *const gsl_vector,
    mut cs: *const gsl_vector,
    mut c: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    if (*L).size > (*work).pmax {
        gsl_error(
            b"L vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            525 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L).size != (*cs).size {
        gsl_error(
            b"cs vector does not match L\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            529 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L).size != (*c).size {
        gsl_error(
            b"c vector does not match L\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            533 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        gsl_vector_memcpy(c, cs);
        gsl_vector_div(c, L);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_wgenform2(
    mut LQR: *const gsl_matrix,
    mut Ltau: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut cs: *const gsl_vector,
    mut M: *const gsl_matrix,
    mut c: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let m: size_t = (*LQR).size1;
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if n > (*work).nmax || p > (*work).pmax {
        gsl_error(
            b"X matrix does not match workspace\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            578 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*LQR).size2 {
        gsl_error(
            b"LQR matrix does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            582 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*c).size {
        gsl_error(
            b"c vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            586 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !w.is_null() && n != (*w).size {
        gsl_error(
            b"w vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            590 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*y).size {
        gsl_error(
            b"y vector does not match X\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            594 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if m >= p {
        if p != (*cs).size {
            gsl_error(
                b"cs vector must be length p\0" as *const u8 as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                600 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return GSL_EBADLEN as libc::c_int;
        } else {
            let mut s: libc::c_int = 0;
            let R: gsl_matrix_const_view = gsl_matrix_const_submatrix(
                LQR,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                p,
                p,
            );
            gsl_vector_memcpy(c, cs);
            s = gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, &R.matrix, c);
            return s;
        }
    } else if m != (*cs).size {
        gsl_error(
            b"cs vector must be length m\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            618 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*M).size1 || p != (*M).size2 {
        gsl_error(
            b"M matrix must be size n-by-p\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            622 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let pm: size_t = p.wrapping_sub(m);
        let mut A: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            n,
            p,
        );
        let mut b: gsl_vector_view = gsl_vector_subvector(
            (*work).t,
            0 as libc::c_int as size_t,
            n,
        );
        let mut Rp: gsl_matrix_view = gsl_matrix_view_array((*LQR).data, m, m);
        let mut LTQR: gsl_matrix_view = gsl_matrix_view_array((*LQR).data, p, m);
        let LTtau: gsl_vector_const_view = gsl_vector_const_subvector(
            Ltau,
            0 as libc::c_int as size_t,
            m,
        );
        let MQR: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            M,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            n,
            pm,
        );
        let Mtau: gsl_vector_const_view = gsl_matrix_const_subcolumn(
            M,
            p.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            if n < pm { n } else { pm },
        );
        let To: gsl_matrix_const_view = gsl_matrix_const_submatrix(
            &MQR.matrix,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            pm,
            pm,
        );
        let mut workp: gsl_vector_view = gsl_vector_subvector(
            (*work).xt,
            0 as libc::c_int as size_t,
            p,
        );
        let mut v1: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut v2: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        status = gsl_multifit_linear_applyW(X, w, y, &mut A.matrix, &mut b.vector);
        if status != 0 {
            return status;
        }
        gsl_vector_set_zero(c);
        v1 = gsl_vector_subvector(c, 0 as libc::c_int as size_t, m);
        gsl_vector_memcpy(&mut v1.vector, cs);
        gsl_blas_dtrsv(
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            &mut Rp.matrix,
            &mut v1.vector,
        );
        gsl_linalg_QR_Qvec(&mut LTQR.matrix, &LTtau.vector, c);
        gsl_blas_dgemv(CblasNoTrans, -1.0f64, &mut A.matrix, c, 1.0f64, &mut b.vector);
        gsl_linalg_QR_QTvec(&MQR.matrix, &Mtau.vector, &mut b.vector);
        v1 = gsl_vector_subvector(&mut b.vector, 0 as libc::c_int as size_t, pm);
        gsl_blas_dtrsv(
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            &To.matrix,
            &mut v1.vector,
        );
        gsl_vector_set_zero(&mut workp.vector);
        v2 = gsl_vector_subvector(&mut workp.vector, m, pm);
        gsl_vector_memcpy(&mut v2.vector, &mut v1.vector);
        gsl_linalg_QR_Qvec(&mut LTQR.matrix, &LTtau.vector, &mut workp.vector);
        gsl_vector_add(c, &mut workp.vector);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_genform2(
    mut LQR: *const gsl_matrix,
    mut Ltau: *const gsl_vector,
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut cs: *const gsl_vector,
    mut M: *const gsl_matrix,
    mut c: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = gsl_multifit_linear_wgenform2(
        LQR,
        Ltau,
        X,
        0 as *const gsl_vector,
        y,
        cs,
        M,
        c,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_multifit_linear_lreg(
    smin: libc::c_double,
    smax: libc::c_double,
    mut reg_param: *mut gsl_vector,
) -> libc::c_int {
    if smax <= 0.0f64 {
        gsl_error(
            b"smax must be positive\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            717 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let N: size_t = (*reg_param).size;
        let smin_ratio: libc::c_double = 16.0f64 * 2.2204460492503131e-16f64;
        let new_smin: libc::c_double = if smin > smax * smin_ratio {
            smin
        } else {
            smax * smin_ratio
        };
        let mut ratio: libc::c_double = 0.;
        let mut i: size_t = 0;
        gsl_vector_set(
            reg_param,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            new_smin,
        );
        ratio = pow(smax / new_smin, 1.0f64 / (N as libc::c_double - 1.0f64));
        i = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        while i > 0 as libc::c_int as libc::c_ulong
            && {
                let fresh0 = i;
                i = i.wrapping_sub(1);
                fresh0 != 0
            }
        {
            let mut rp1: libc::c_double = gsl_vector_get(
                reg_param,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_set(reg_param, i, ratio * rp1);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_lcurve(
    mut y: *const gsl_vector,
    mut reg_param: *mut gsl_vector,
    mut rho: *mut gsl_vector,
    mut eta: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*y).size;
    let N: size_t = (*rho).size;
    if n != (*work).n {
        gsl_error(
            b"y vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            775 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N < 3 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"at least 3 points are needed for L-curve analysis\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            780 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*eta).size {
        gsl_error(
            b"size of rho and eta vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            785 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*reg_param).size != (*eta).size {
        gsl_error(
            b"size of reg_param and eta vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            790 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let p: size_t = (*work).p;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut A: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            n,
            p,
        );
        let mut S: gsl_vector_view = gsl_vector_subvector(
            (*work).S,
            0 as libc::c_int as size_t,
            p,
        );
        let mut xt: gsl_vector_view = gsl_vector_subvector(
            (*work).xt,
            0 as libc::c_int as size_t,
            p,
        );
        let mut workp: gsl_vector_view = gsl_matrix_subcolumn(
            (*work).QSI,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p,
        );
        let mut workp2: gsl_vector_view = gsl_vector_subvector(
            (*work).D,
            0 as libc::c_int as size_t,
            p,
        );
        let smax: libc::c_double = gsl_vector_get(
            &mut S.vector,
            0 as libc::c_int as size_t,
        );
        let smin: libc::c_double = gsl_vector_get(
            &mut S.vector,
            p.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut dr: libc::c_double = 0.;
        let mut normy: libc::c_double = gsl_blas_dnrm2(y);
        let mut normUTy: libc::c_double = 0.;
        gsl_blas_dgemv(CblasTrans, 1.0f64, &mut A.matrix, y, 0.0f64, &mut xt.vector);
        normUTy = gsl_blas_dnrm2(&mut xt.vector);
        dr = normy * normy - normUTy * normUTy;
        gsl_multifit_linear_lreg(smin, smax, reg_param);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut lambda: libc::c_double = gsl_vector_get(reg_param, i);
            let mut lambda_sq: libc::c_double = lambda * lambda;
            j = 0 as libc::c_int as size_t;
            while j < p {
                let mut sj: libc::c_double = gsl_vector_get(&mut S.vector, j);
                let mut xtj: libc::c_double = gsl_vector_get(&mut xt.vector, j);
                let mut f: libc::c_double = sj / (sj * sj + lambda_sq);
                gsl_vector_set(&mut workp.vector, j, f * xtj);
                gsl_vector_set(&mut workp2.vector, j, (1.0f64 - sj * f) * xtj);
                j = j.wrapping_add(1);
                j;
            }
            gsl_vector_set(eta, i, gsl_blas_dnrm2(&mut workp.vector));
            gsl_vector_set(rho, i, gsl_blas_dnrm2(&mut workp2.vector));
            i = i.wrapping_add(1);
            i;
        }
        if n > p && dr > 0.0f64 {
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut rhoi: libc::c_double = gsl_vector_get(rho, i);
                let mut ptr: *mut libc::c_double = gsl_vector_ptr(rho, i);
                *ptr = sqrt(rhoi * rhoi + dr);
                i = i.wrapping_add(1);
                i;
            }
        }
        gsl_vector_set_all((*work).D, 1.0f64);
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_lcurvature(
    mut y: *const gsl_vector,
    mut reg_param: *const gsl_vector,
    mut rho: *const gsl_vector,
    mut eta: *const gsl_vector,
    mut kappa: *mut gsl_vector,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*y).size;
    let N: size_t = (*rho).size;
    if n != (*work).n {
        gsl_error(
            b"y vector does not match workspace\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            893 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*eta).size {
        gsl_error(
            b"size of rho and eta vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            898 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*reg_param).size != N {
        gsl_error(
            b"size of reg_param and rho vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            903 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*kappa).size != N {
        gsl_error(
            b"size of reg_param and rho vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            908 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let p: size_t = (*work).p;
        let mut U: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            n,
            p,
        );
        let mut S: gsl_vector_view = gsl_vector_subvector(
            (*work).S,
            0 as libc::c_int as size_t,
            p,
        );
        let mut beta: gsl_vector_view = gsl_vector_subvector(
            (*work).xt,
            0 as libc::c_int as size_t,
            p,
        );
        let mut i: size_t = 0;
        gsl_blas_dgemv(CblasTrans, 1.0f64, &mut U.matrix, y, 0.0f64, &mut beta.vector);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut lambda: libc::c_double = gsl_vector_get(reg_param, i);
            let mut lambda_sq: libc::c_double = lambda * lambda;
            let mut eta_i: libc::c_double = gsl_vector_get(eta, i);
            let mut rho_i: libc::c_double = gsl_vector_get(rho, i);
            let mut phi_i: libc::c_double = 0.0f64;
            let mut dphi_i: libc::c_double = 0.0f64;
            let mut psi_i: libc::c_double = 0.0f64;
            let mut dpsi_i: libc::c_double = 0.0f64;
            let mut deta_i: libc::c_double = 0.;
            let mut ddeta_i: libc::c_double = 0.;
            let mut drho_i: libc::c_double = 0.;
            let mut ddrho_i: libc::c_double = 0.;
            let mut dlogeta_i: libc::c_double = 0.;
            let mut ddlogeta_i: libc::c_double = 0.;
            let mut dlogrho_i: libc::c_double = 0.;
            let mut ddlogrho_i: libc::c_double = 0.;
            let mut kappa_i: libc::c_double = 0.;
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < p {
                let mut beta_j: libc::c_double = gsl_vector_get(&mut beta.vector, j);
                let mut s_j: libc::c_double = gsl_vector_get(&mut S.vector, j);
                let mut sj_sq: libc::c_double = s_j * s_j;
                let mut f_j: libc::c_double = sj_sq / (sj_sq + lambda_sq);
                let mut onemf_j: libc::c_double = 1.0f64 - f_j;
                let mut f1_j: libc::c_double = -2.0f64 * f_j * onemf_j / lambda;
                let mut f2_j: libc::c_double = -f1_j * (3.0f64 - 4.0f64 * f_j) / lambda;
                let mut xi_j: libc::c_double = beta_j / s_j;
                phi_i += f_j * f1_j * xi_j * xi_j;
                psi_i += onemf_j * f1_j * beta_j * beta_j;
                dphi_i += (f1_j * f1_j + f_j * f2_j) * xi_j * xi_j;
                dpsi_i += (-f1_j * f1_j + onemf_j * f2_j) * beta_j * beta_j;
                j = j.wrapping_add(1);
                j;
            }
            deta_i = phi_i / eta_i;
            drho_i = -psi_i / rho_i;
            ddeta_i = dphi_i / eta_i - deta_i * (deta_i / eta_i);
            ddrho_i = -dpsi_i / rho_i - drho_i * (drho_i / rho_i);
            dlogeta_i = deta_i / eta_i;
            dlogrho_i = drho_i / rho_i;
            ddlogeta_i = ddeta_i / eta_i - dlogeta_i * dlogeta_i;
            ddlogrho_i = ddrho_i / rho_i - dlogrho_i * dlogrho_i;
            kappa_i = (dlogrho_i * ddlogeta_i - ddlogrho_i * dlogeta_i)
                / pow(dlogrho_i * dlogrho_i + dlogeta_i * dlogeta_i, 1.5f64);
            gsl_vector_set(kappa, i, kappa_i);
            i = i.wrapping_add(1);
            i;
        }
        return status;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_lcorner(
    mut rho: *const gsl_vector,
    mut eta: *const gsl_vector,
    mut idx: *mut size_t,
) -> libc::c_int {
    let n: size_t = (*rho).size;
    if n < 3 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"at least 3 points are needed for L-curve analysis\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            999 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*eta).size {
        gsl_error(
            b"size of rho and eta vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1004 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut i: size_t = 0;
        let mut x1: libc::c_double = 0.;
        let mut y1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut y2: libc::c_double = 0.;
        let mut rmin: libc::c_double = -1.0f64;
        x1 = log(gsl_vector_get(rho, 0 as libc::c_int as size_t));
        y1 = log(gsl_vector_get(eta, 0 as libc::c_int as size_t));
        x2 = log(gsl_vector_get(rho, 1 as libc::c_int as size_t));
        y2 = log(gsl_vector_get(eta, 1 as libc::c_int as size_t));
        i = 1 as libc::c_int as size_t;
        while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut x3: libc::c_double = log(
                gsl_vector_get(rho, i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            let mut y3: libc::c_double = log(
                gsl_vector_get(eta, i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            let mut x21: libc::c_double = x2 - x1;
            let mut y21: libc::c_double = y2 - y1;
            let mut x31: libc::c_double = x3 - x1;
            let mut y31: libc::c_double = y3 - y1;
            let mut h21: libc::c_double = x21 * x21 + y21 * y21;
            let mut h31: libc::c_double = x31 * x31 + y31 * y31;
            let mut d: libc::c_double = fabs(2.0f64 * (x21 * y31 - x31 * y21));
            let mut r: libc::c_double = sqrt(
                h21 * h31 * ((x3 - x2) * (x3 - x2) + (y3 - y2) * (y3 - y2)),
            ) / d;
            if gsl_finite(r) != 0 {
                if r < rmin || rmin < 0.0f64 {
                    rmin = r;
                    *idx = i;
                }
            }
            x1 = x2;
            y1 = y2;
            x2 = x3;
            y2 = y3;
            i = i.wrapping_add(1);
            i;
        }
        if rmin < 0.0f64 {
            gsl_error(
                b"failed to find minimum radius\0" as *const u8 as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                1063 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return s;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_lcorner2(
    mut reg_param: *const gsl_vector,
    mut eta: *const gsl_vector,
    mut idx: *mut size_t,
) -> libc::c_int {
    let n: size_t = (*reg_param).size;
    if n < 3 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"at least 3 points are needed for L-curve analysis\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1102 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if n != (*eta).size {
        gsl_error(
            b"size of reg_param and eta vectors do not match\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1107 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut i: size_t = 0;
        let mut x1: libc::c_double = 0.;
        let mut y1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut y2: libc::c_double = 0.;
        let mut rmin: libc::c_double = -1.0f64;
        x1 = gsl_vector_get(reg_param, 0 as libc::c_int as size_t);
        x1 *= x1;
        y1 = gsl_vector_get(eta, 0 as libc::c_int as size_t);
        y1 *= y1;
        x2 = gsl_vector_get(reg_param, 1 as libc::c_int as size_t);
        x2 *= x2;
        y2 = gsl_vector_get(eta, 1 as libc::c_int as size_t);
        y2 *= y2;
        i = 1 as libc::c_int as size_t;
        while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut lamip1: libc::c_double = gsl_vector_get(
                reg_param,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            let mut etaip1: libc::c_double = gsl_vector_get(
                eta,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            let mut x3: libc::c_double = lamip1 * lamip1;
            let mut y3: libc::c_double = etaip1 * etaip1;
            let mut x21: libc::c_double = x2 - x1;
            let mut y21: libc::c_double = y2 - y1;
            let mut x31: libc::c_double = x3 - x1;
            let mut y31: libc::c_double = y3 - y1;
            let mut h21: libc::c_double = x21 * x21 + y21 * y21;
            let mut h31: libc::c_double = x31 * x31 + y31 * y31;
            let mut d: libc::c_double = fabs(2.0f64 * (x21 * y31 - x31 * y21));
            let mut r: libc::c_double = sqrt(
                h21 * h31 * ((x3 - x2) * (x3 - x2) + (y3 - y2) * (y3 - y2)),
            ) / d;
            if gsl_finite(r) != 0 {
                if r < rmin || rmin < 0.0f64 {
                    rmin = r;
                    *idx = i;
                }
            }
            x1 = x2;
            y1 = y2;
            x2 = x3;
            y2 = y3;
            i = i.wrapping_add(1);
            i;
        }
        if rmin < 0.0f64 {
            gsl_error(
                b"failed to find minimum radius\0" as *const u8 as *const libc::c_char,
                b"multireg.c\0" as *const u8 as *const libc::c_char,
                1172 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return s;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_Lk(
    p: size_t,
    k: size_t,
    mut L: *mut gsl_matrix,
) -> libc::c_int {
    if p <= k {
        gsl_error(
            b"p must be larger than derivative order\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1192 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if k >= (100 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        gsl_error(
            b"derivative order k too large\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1196 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p.wrapping_sub(k) != (*L).size1 || p != (*L).size2 {
        gsl_error(
            b"L matrix must be (p-k)-by-p\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1200 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut c_data: [libc::c_double; 100] = [0.; 100];
        let mut cv: gsl_vector_view = gsl_vector_view_array(
            c_data.as_mut_ptr(),
            k.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        if k == 0 as libc::c_int as libc::c_ulong {
            gsl_matrix_set_identity(L);
            return GSL_SUCCESS as libc::c_int;
        }
        gsl_matrix_set_zero(L);
        gsl_vector_set_zero(&mut cv.vector);
        gsl_vector_set(&mut cv.vector, 0 as libc::c_int as size_t, -1.0f64);
        gsl_vector_set(&mut cv.vector, 1 as libc::c_int as size_t, 1.0f64);
        i = 1 as libc::c_int as size_t;
        while i < k {
            let mut cjm1: libc::c_double = 0.0f64;
            j = 0 as libc::c_int as size_t;
            while j < k.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                let mut cj: libc::c_double = gsl_vector_get(&mut cv.vector, j);
                gsl_vector_set(&mut cv.vector, j, cjm1 - cj);
                cjm1 = cj;
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < k.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            let mut v: gsl_vector_view = gsl_matrix_superdiagonal(L, i);
            let mut ci: libc::c_double = gsl_vector_get(&mut cv.vector, i);
            gsl_vector_set_all(&mut v.vector, ci);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_linear_Lsobolev(
    p: size_t,
    kmax: size_t,
    mut alpha: *const gsl_vector,
    mut L: *mut gsl_matrix,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    if p > (*work).pmax {
        gsl_error(
            b"p is larger than workspace\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1273 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p <= kmax {
        gsl_error(
            b"p must be larger than derivative order\0" as *const u8
                as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1277 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if kmax.wrapping_add(1 as libc::c_int as libc::c_ulong) != (*alpha).size {
        gsl_error(
            b"alpha must be size kmax + 1\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1281 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*L).size1 {
        gsl_error(
            b"L matrix is wrong size\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1285 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L).size1 != (*L).size2 {
        gsl_error(
            b"L matrix is not square\0" as *const u8 as *const libc::c_char,
            b"multireg.c\0" as *const u8 as *const libc::c_char,
            1289 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        let mut d: gsl_vector_view = gsl_matrix_diagonal(L);
        let alpha0: libc::c_double = gsl_vector_get(alpha, 0 as libc::c_int as size_t);
        gsl_matrix_set_zero(L);
        gsl_vector_add_constant(&mut d.vector, alpha0 * alpha0);
        k = 1 as libc::c_int as size_t;
        while k <= kmax {
            let mut Lk: gsl_matrix_view = gsl_matrix_submatrix(
                (*work).Q,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                p.wrapping_sub(k),
                p,
            );
            let mut ak: libc::c_double = gsl_vector_get(alpha, k);
            s = gsl_multifit_linear_Lk(p, k, &mut Lk.matrix);
            if s != 0 {
                return s;
            }
            gsl_matrix_scale(&mut Lk.matrix, ak);
            gsl_blas_dsyrk(CblasLower, CblasTrans, 1.0f64, &mut Lk.matrix, 1.0f64, L);
            k = k.wrapping_add(1);
            k;
        }
        s = gsl_linalg_cholesky_decomp(L);
        if s != 0 {
            return s;
        }
        gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, L, L);
        j = 0 as libc::c_int as size_t;
        while j < p {
            k = 0 as libc::c_int as size_t;
            while k < j {
                gsl_matrix_set(L, j, k, 0.0f64);
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
