use ::libc;
extern "C" {
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
    fn gsl_blas_dger(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *const gsl_vector,
        A: *mut gsl_matrix,
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
pub unsafe extern "C" fn gsl_linalg_householder_transform(
    mut v: *mut gsl_vector,
) -> libc::c_double {
    let n: size_t = (*v).size;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut alpha: libc::c_double = 0.;
        let mut beta: libc::c_double = 0.;
        let mut tau: libc::c_double = 0.;
        let mut x: gsl_vector_view = gsl_vector_subvector(
            v,
            1 as libc::c_int as size_t,
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut xnorm: libc::c_double = gsl_blas_dnrm2(&mut x.vector);
        if xnorm == 0 as libc::c_int as libc::c_double {
            return 0.0f64;
        }
        alpha = gsl_vector_get(v, 0 as libc::c_int as size_t);
        beta = -(if alpha >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
            as libc::c_double * hypot(alpha, xnorm);
        tau = (beta - alpha) / beta;
        let mut s: libc::c_double = alpha - beta;
        if fabs(s) > 2.2250738585072014e-308f64 {
            gsl_blas_dscal(1.0f64 / s, &mut x.vector);
            gsl_vector_set(v, 0 as libc::c_int as size_t, beta);
        } else {
            gsl_blas_dscal(2.2204460492503131e-16f64 / s, &mut x.vector);
            gsl_blas_dscal(1.0f64 / 2.2204460492503131e-16f64, &mut x.vector);
            gsl_vector_set(v, 0 as libc::c_int as size_t, beta);
        }
        return tau;
    };
}
pub unsafe extern "C" fn gsl_linalg_householder_transform2(
    mut alpha: *mut libc::c_double,
    mut v: *mut gsl_vector,
) -> libc::c_double {
    let n: size_t = (*v).size;
    if n == 1 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut beta: libc::c_double = 0.;
        let mut tau: libc::c_double = 0.;
        let mut x: gsl_vector_view = gsl_vector_subvector(
            v,
            0 as libc::c_int as size_t,
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut xnorm: libc::c_double = gsl_blas_dnrm2(&mut x.vector);
        if xnorm == 0 as libc::c_int as libc::c_double {
            return 0.0f64;
        }
        beta = -(if *alpha >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
            as libc::c_double * hypot(*alpha, xnorm);
        tau = (beta - *alpha) / beta;
        let mut s: libc::c_double = *alpha - beta;
        if fabs(s) > 2.2250738585072014e-308f64 {
            gsl_blas_dscal(1.0f64 / s, &mut x.vector);
            *alpha = beta;
        } else {
            gsl_blas_dscal(2.2204460492503131e-16f64 / s, &mut x.vector);
            gsl_blas_dscal(1.0f64 / 2.2204460492503131e-16f64, &mut x.vector);
            *alpha = beta;
        }
        return tau;
    };
}
pub unsafe extern "C" fn gsl_linalg_householder_hm(
    mut tau: libc::c_double,
    mut v: *const gsl_vector,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    if tau == 0.0f64 {
        return GSL_SUCCESS as libc::c_int;
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < (*A).size2 {
        let mut wj: libc::c_double = gsl_matrix_get(A, 0 as libc::c_int as size_t, j);
        i = 1 as libc::c_int as size_t;
        while i < (*A).size1 {
            wj += gsl_matrix_get(A, i, j) * gsl_vector_get(v, i);
            i = i.wrapping_add(1);
            i;
        }
        let mut A0j: libc::c_double = gsl_matrix_get(A, 0 as libc::c_int as size_t, j);
        gsl_matrix_set(A, 0 as libc::c_int as size_t, j, A0j - tau * wj);
        i = 1 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut Aij: libc::c_double = gsl_matrix_get(A, i, j);
            let mut vi: libc::c_double = gsl_vector_get(v, i);
            gsl_matrix_set(A, i, j, Aij - tau * vi * wj);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_householder_mh(
    mut tau: libc::c_double,
    mut v: *const gsl_vector,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    if tau == 0 as libc::c_int as libc::c_double {
        return GSL_SUCCESS as libc::c_int;
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*A).size1 {
        let mut wi: libc::c_double = gsl_matrix_get(A, i, 0 as libc::c_int as size_t);
        j = 1 as libc::c_int as size_t;
        while j < (*A).size2 {
            wi += gsl_matrix_get(A, i, j) * gsl_vector_get(v, j);
            j = j.wrapping_add(1);
            j;
        }
        let mut Ai0: libc::c_double = gsl_matrix_get(A, i, 0 as libc::c_int as size_t);
        gsl_matrix_set(A, i, 0 as libc::c_int as size_t, Ai0 - tau * wi);
        j = 1 as libc::c_int as size_t;
        while j < (*A).size2 {
            let mut vj: libc::c_double = gsl_vector_get(v, j);
            let mut Aij: libc::c_double = gsl_matrix_get(A, i, j);
            gsl_matrix_set(A, i, j, Aij - tau * wi * vj);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_householder_hv(
    mut tau: libc::c_double,
    mut v: *const gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*v).size;
    if tau == 0 as libc::c_int as libc::c_double {
        return GSL_SUCCESS as libc::c_int;
    }
    let mut w0: libc::c_double = gsl_vector_get(w, 0 as libc::c_int as size_t);
    let mut d1: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let v1: gsl_vector_const_view = gsl_vector_const_subvector(
        v,
        1 as libc::c_int as size_t,
        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    let mut w1: gsl_vector_view = gsl_vector_subvector(
        w,
        1 as libc::c_int as size_t,
        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    gsl_blas_ddot(&v1.vector, &mut w1.vector, &mut d1);
    d = w0 + d1;
    gsl_vector_set(w, 0 as libc::c_int as size_t, w0 - tau * d);
    gsl_blas_daxpy(-tau * d, &v1.vector, &mut w1.vector);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_householder_left(
    tau: libc::c_double,
    mut v: *const gsl_vector,
    mut A: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*v).size != M {
        gsl_error(
            b"matrix must match Householder vector dimensions\0" as *const u8
                as *const libc::c_char,
            b"householder.c\0" as *const u8 as *const libc::c_char,
            355 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != N {
        gsl_error(
            b"workspace must match matrix\0" as *const u8 as *const libc::c_char,
            b"householder.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        if tau == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        gsl_blas_dgemv(CblasTrans, 1.0f64, A, v, 0.0f64, work);
        gsl_blas_dger(-tau, v, work, A);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_householder_right(
    tau: libc::c_double,
    mut v: *const gsl_vector,
    mut A: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*v).size != N {
        gsl_error(
            b"matrix must match Householder vector dimensions\0" as *const u8
                as *const libc::c_char,
            b"householder.c\0" as *const u8 as *const libc::c_char,
            404 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != M {
        gsl_error(
            b"workspace must match matrix\0" as *const u8 as *const libc::c_char,
            b"householder.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut v0: libc::c_double = 0.;
        if tau == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        v0 = gsl_vector_get(v, 0 as libc::c_int as size_t);
        *((*v).data).offset(0 as libc::c_int as isize) = 1.0f64;
        gsl_blas_dgemv(CblasNoTrans, 1.0f64, A, v, 0.0f64, work);
        gsl_blas_dger(-tau, work, v, A);
        *((*v).data).offset(0 as libc::c_int as isize) = v0;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_householder_hm1(
    mut tau: libc::c_double,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    if tau == 0 as libc::c_int as libc::c_double {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_matrix_set(
            A,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            1.0f64,
        );
        j = 1 as libc::c_int as size_t;
        while j < (*A).size2 {
            gsl_matrix_set(A, 0 as libc::c_int as size_t, j, 0.0f64);
            j = j.wrapping_add(1);
            j;
        }
        i = 1 as libc::c_int as size_t;
        while i < (*A).size1 {
            gsl_matrix_set(A, i, 0 as libc::c_int as size_t, 0.0f64);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    }
    let mut i_0: size_t = 0;
    let mut j_0: size_t = 0;
    j_0 = 1 as libc::c_int as size_t;
    while j_0 < (*A).size2 {
        let mut wj: libc::c_double = 0.0f64;
        i_0 = 1 as libc::c_int as size_t;
        while i_0 < (*A).size1 {
            let mut vi: libc::c_double = gsl_matrix_get(
                A,
                i_0,
                0 as libc::c_int as size_t,
            );
            wj += gsl_matrix_get(A, i_0, j_0) * vi;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        gsl_matrix_set(A, 0 as libc::c_int as size_t, j_0, -tau * wj);
        i_0 = 1 as libc::c_int as size_t;
        while i_0 < (*A).size1 {
            let mut vi_0: libc::c_double = gsl_matrix_get(
                A,
                i_0,
                0 as libc::c_int as size_t,
            );
            let mut Aij: libc::c_double = gsl_matrix_get(A, i_0, j_0);
            gsl_matrix_set(A, i_0, j_0, Aij - tau * vi_0 * wj);
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        j_0 = j_0.wrapping_add(1);
        j_0;
    }
    i_0 = 1 as libc::c_int as size_t;
    while i_0 < (*A).size1 {
        let mut vi_1: libc::c_double = gsl_matrix_get(
            A,
            i_0,
            0 as libc::c_int as size_t,
        );
        gsl_matrix_set(A, i_0, 0 as libc::c_int as size_t, -tau * vi_1);
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    gsl_matrix_set(
        A,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        1.0f64 - tau,
    );
    return GSL_SUCCESS as libc::c_int;
}
