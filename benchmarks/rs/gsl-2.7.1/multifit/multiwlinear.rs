use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_multifit_linear_applyW(
        X: *const gsl_matrix,
        w: *const gsl_vector,
        y: *const gsl_vector,
        WX: *mut gsl_matrix,
        Wy: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_multifit_linear_bsvd(
        X: *const gsl_matrix,
        work: *mut gsl_multifit_linear_workspace,
    ) -> libc::c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
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
pub unsafe extern "C" fn gsl_multifit_wlinear(
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut c: *mut gsl_vector,
    mut cov: *mut gsl_matrix,
    mut chisq: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let mut rank: size_t = 0;
    let mut status: libc::c_int = gsl_multifit_wlinear_tsvd(
        X,
        w,
        y,
        2.2204460492503131e-16f64,
        c,
        cov,
        chisq,
        &mut rank,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_multifit_wlinear_tsvd(
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    tol: libc::c_double,
    mut c: *mut gsl_vector,
    mut cov: *mut gsl_matrix,
    mut chisq: *mut libc::c_double,
    mut rank: *mut size_t,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let n: size_t = (*X).size1;
    let p: size_t = (*X).size2;
    if (*y).size != n {
        gsl_error(
            b"number of observations in y does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"multiwlinear.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*w).size != n {
        gsl_error(
            b"number of weights in w does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"multiwlinear.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if p != (*c).size {
        gsl_error(
            b"number of parameters c does not match matrix\0" as *const u8
                as *const libc::c_char,
            b"multiwlinear.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if tol <= 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"tolerance must be positive\0" as *const u8 as *const libc::c_char,
            b"multiwlinear.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut rnorm: libc::c_double = 0.;
        let mut snorm: libc::c_double = 0.;
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
        status = gsl_multifit_linear_applyW(X, w, y, &mut A.matrix, &mut b.vector);
        if status != 0 {
            return status;
        }
        status = gsl_multifit_linear_bsvd(&mut A.matrix, work);
        if status != 0 {
            return status;
        }
        status = multifit_linear_solve(
            X,
            &mut b.vector,
            tol,
            0.0f64,
            rank,
            c,
            &mut rnorm,
            &mut snorm,
            work,
        );
        if status != 0 {
            return status;
        }
        *chisq = rnorm * rnorm;
        let p_0: size_t = (*X).size2;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut QSI: gsl_matrix_view = gsl_matrix_submatrix(
            (*work).QSI,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            p_0,
            p_0,
        );
        let mut D: gsl_vector_view = gsl_vector_subvector(
            (*work).D,
            0 as libc::c_int as size_t,
            p_0,
        );
        i = 0 as libc::c_int as size_t;
        while i < p_0 {
            let mut row_i: gsl_vector_view = gsl_matrix_row(&mut QSI.matrix, i);
            let mut d_i: libc::c_double = gsl_vector_get(&mut D.vector, i);
            j = i;
            while j < p_0 {
                let mut row_j: gsl_vector_view = gsl_matrix_row(&mut QSI.matrix, j);
                let mut d_j: libc::c_double = gsl_vector_get(&mut D.vector, j);
                let mut s: libc::c_double = 0.;
                gsl_blas_ddot(&mut row_i.vector, &mut row_j.vector, &mut s);
                gsl_matrix_set(cov, i, j, s / (d_i * d_j));
                gsl_matrix_set(cov, j, i, s / (d_i * d_j));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_multifit_wlinear_svd(
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut tol: libc::c_double,
    mut rank: *mut size_t,
    mut c: *mut gsl_vector,
    mut cov: *mut gsl_matrix,
    mut chisq: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_multifit_wlinear_tsvd(
        X,
        w,
        y,
        tol,
        c,
        cov,
        chisq,
        rank,
        work,
    );
    return status;
}
pub unsafe extern "C" fn gsl_multifit_wlinear_usvd(
    mut X: *const gsl_matrix,
    mut w: *const gsl_vector,
    mut y: *const gsl_vector,
    mut tol: libc::c_double,
    mut rank: *mut size_t,
    mut c: *mut gsl_vector,
    mut cov: *mut gsl_matrix,
    mut chisq: *mut libc::c_double,
    mut work: *mut gsl_multifit_linear_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_multifit_wlinear_tsvd(
        X,
        w,
        y,
        tol,
        c,
        cov,
        chisq,
        rank,
        work,
    );
    return status;
}
