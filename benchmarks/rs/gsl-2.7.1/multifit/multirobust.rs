use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_multifit_linear_alloc(
        n: size_t,
        p: size_t,
    ) -> *mut gsl_multifit_linear_workspace;
    fn gsl_multifit_linear_free(w: *mut gsl_multifit_linear_workspace);
    fn gsl_multifit_linear(
        X: *const gsl_matrix,
        y: *const gsl_vector,
        c: *mut gsl_vector,
        cov: *mut gsl_matrix,
        chisq: *mut libc::c_double,
        work: *mut gsl_multifit_linear_workspace,
    ) -> libc::c_int;
    fn gsl_multifit_wlinear(
        X: *const gsl_matrix,
        w: *const gsl_vector,
        y: *const gsl_vector,
        c: *mut gsl_vector,
        cov: *mut gsl_matrix,
        chisq: *mut libc::c_double,
        work: *mut gsl_multifit_linear_workspace,
    ) -> libc::c_int;
    fn gsl_multifit_linear_est(
        x: *const gsl_vector,
        c: *const gsl_vector,
        cov: *const gsl_matrix,
        y: *mut libc::c_double,
        y_err: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_multifit_linear_residuals(
        X: *const gsl_matrix,
        y: *const gsl_vector,
        c: *const gsl_vector,
        r: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_stats_sd(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_median_from_sorted_data(
        sorted_data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_tss(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_mean(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_sort_vector(v: *mut gsl_vector);
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_linalg_SV_leverage(U: *const gsl_matrix, h: *mut gsl_vector) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_robust_type {
    pub name: *const libc::c_char,
    pub wfun: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut gsl_vector) -> libc::c_int,
    >,
    pub psi_deriv: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut gsl_vector) -> libc::c_int,
    >,
    pub tuning_default: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_robust_stats {
    pub sigma_ols: libc::c_double,
    pub sigma_mad: libc::c_double,
    pub sigma_rob: libc::c_double,
    pub sigma: libc::c_double,
    pub Rsq: libc::c_double,
    pub adj_Rsq: libc::c_double,
    pub rmse: libc::c_double,
    pub sse: libc::c_double,
    pub dof: size_t,
    pub numit: size_t,
    pub weights: *mut gsl_vector,
    pub r: *mut gsl_vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_robust_workspace {
    pub n: size_t,
    pub p: size_t,
    pub numit: size_t,
    pub maxiter: size_t,
    pub type_0: *const gsl_multifit_robust_type,
    pub tune: libc::c_double,
    pub r: *mut gsl_vector,
    pub weights: *mut gsl_vector,
    pub c_prev: *mut gsl_vector,
    pub resfac: *mut gsl_vector,
    pub psi: *mut gsl_vector,
    pub dpsi: *mut gsl_vector,
    pub QSI: *mut gsl_matrix,
    pub D: *mut gsl_vector,
    pub workn: *mut gsl_vector,
    pub stats: gsl_multifit_robust_stats,
    pub multifit_p: *mut gsl_multifit_linear_workspace,
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
pub unsafe extern "C" fn gsl_multifit_robust_alloc(
    mut T: *const gsl_multifit_robust_type,
    n: size_t,
    p: size_t,
) -> *mut gsl_multifit_robust_workspace {
    let mut w: *mut gsl_multifit_robust_workspace = 0
        as *mut gsl_multifit_robust_workspace;
    if n < p {
        gsl_error(
            b"observations n must be >= p\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_multifit_robust_workspace>() as libc::c_ulong,
    ) as *mut gsl_multifit_robust_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for multifit_robust struct\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).n = n;
    (*w).p = p;
    (*w).type_0 = T;
    (*w).maxiter = 100 as libc::c_int as size_t;
    (*w).tune = (*(*w).type_0).tuning_default;
    (*w).multifit_p = gsl_multifit_linear_alloc(n, p);
    if ((*w).multifit_p).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for multifit_linear struct\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).r = gsl_vector_alloc(n);
    if ((*w).r).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for residuals\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).weights = gsl_vector_alloc(n);
    if ((*w).weights).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for weights\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).c_prev = gsl_vector_alloc(p);
    if ((*w).c_prev).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for c_prev\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).resfac = gsl_vector_alloc(n);
    if ((*w).resfac).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for residual factors\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).psi = gsl_vector_alloc(n);
    if ((*w).psi).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for psi\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).dpsi = gsl_vector_alloc(n);
    if ((*w).dpsi).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for dpsi\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).QSI = gsl_matrix_alloc(p, p);
    if ((*w).QSI).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for QSI\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).D = gsl_vector_alloc(p);
    if ((*w).D).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for D\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).workn = gsl_vector_alloc(n);
    if ((*w).workn).is_null() {
        gsl_multifit_robust_free(w);
        gsl_error(
            b"failed to allocate space for workn\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_robust_workspace;
    }
    (*w).stats.sigma_ols = 0.0f64;
    (*w).stats.sigma_mad = 0.0f64;
    (*w).stats.sigma_rob = 0.0f64;
    (*w).stats.sigma = 0.0f64;
    (*w).stats.Rsq = 0.0f64;
    (*w).stats.adj_Rsq = 0.0f64;
    (*w).stats.rmse = 0.0f64;
    (*w).stats.sse = 0.0f64;
    (*w).stats.dof = n.wrapping_sub(p);
    (*w).stats.weights = (*w).weights;
    (*w).stats.r = (*w).r;
    return w;
}
pub unsafe extern "C" fn gsl_multifit_robust_free(
    mut w: *mut gsl_multifit_robust_workspace,
) {
    if !((*w).multifit_p).is_null() {
        gsl_multifit_linear_free((*w).multifit_p);
    }
    if !((*w).r).is_null() {
        gsl_vector_free((*w).r);
    }
    if !((*w).weights).is_null() {
        gsl_vector_free((*w).weights);
    }
    if !((*w).c_prev).is_null() {
        gsl_vector_free((*w).c_prev);
    }
    if !((*w).resfac).is_null() {
        gsl_vector_free((*w).resfac);
    }
    if !((*w).psi).is_null() {
        gsl_vector_free((*w).psi);
    }
    if !((*w).dpsi).is_null() {
        gsl_vector_free((*w).dpsi);
    }
    if !((*w).QSI).is_null() {
        gsl_matrix_free((*w).QSI);
    }
    if !((*w).D).is_null() {
        gsl_vector_free((*w).D);
    }
    if !((*w).workn).is_null() {
        gsl_vector_free((*w).workn);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_multifit_robust_tune(
    tune: libc::c_double,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_int {
    (*w).tune = tune;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_multifit_robust_maxiter(
    maxiter: size_t,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_int {
    if (*w).maxiter == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"maxiter must be greater than 0\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        (*w).maxiter = maxiter;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_multifit_robust_name(
    mut w: *const gsl_multifit_robust_workspace,
) -> *const libc::c_char {
    return (*(*w).type_0).name;
}
pub unsafe extern "C" fn gsl_multifit_robust_statistics(
    mut w: *const gsl_multifit_robust_workspace,
) -> gsl_multifit_robust_stats {
    return (*w).stats;
}
pub unsafe extern "C" fn gsl_multifit_robust_weights(
    mut r: *const gsl_vector,
    mut wts: *mut gsl_vector,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_int {
    if (*r).size != (*wts).size {
        gsl_error(
            b"residual vector does not match weight vector size\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        let mut sigma: libc::c_double = 0.;
        sigma = robust_madsigma(r, (*w).p, wts);
        gsl_vector_memcpy(wts, r);
        if sigma > 0.0f64 {
            gsl_vector_scale(wts, 1.0f64 / (sigma * (*w).tune));
        }
        s = ((*(*w).type_0).wfun).unwrap()(wts, wts);
        return s;
    };
}
pub unsafe extern "C" fn gsl_multifit_robust(
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut c: *mut gsl_vector,
    mut cov: *mut gsl_matrix,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_int {
    if (*X).size1 != (*y).size {
        gsl_error(
            b"number of observations in y does not match rows of matrix X\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*X).size2 != (*c).size {
        gsl_error(
            b"number of parameters c does not match columns of matrix X\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            326 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*cov).size1 != (*cov).size2 {
        gsl_error(
            b"covariance matrix is not square\0" as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*c).size != (*cov).size1 {
        gsl_error(
            b"number of parameters does not match size of covariance matrix\0"
                as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*X).size1 != (*w).n || (*X).size2 != (*w).p {
        gsl_error(
            b"size of workspace does not match size of observation matrix\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            342 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        let mut chisq: libc::c_double = 0.;
        let tol: libc::c_double = 1.4901161193847656e-08f64;
        let mut converged: libc::c_int = 0 as libc::c_int;
        let mut numit: size_t = 0 as libc::c_int as size_t;
        let n: size_t = (*y).size;
        let mut sigy: libc::c_double = gsl_stats_sd(
            (*y).data as *const libc::c_double,
            (*y).stride,
            n,
        );
        let mut sig_lower: libc::c_double = 0.;
        let mut i: size_t = 0;
        sig_lower = 1.0e-6f64 * sigy;
        if sig_lower == 0.0f64 {
            sig_lower = 1.0f64;
        }
        s = gsl_multifit_linear(X, y, c, cov, &mut chisq, (*w).multifit_p);
        if s != 0 {
            return s;
        }
        gsl_matrix_memcpy((*w).QSI, (*(*w).multifit_p).QSI);
        gsl_vector_memcpy((*w).D, (*(*w).multifit_p).D);
        s = gsl_linalg_SV_leverage((*(*w).multifit_p).A, (*w).resfac);
        if s != 0 {
            return s;
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut h: libc::c_double = gsl_vector_get((*w).resfac, i);
            if h > 0.9999f64 {
                h = 0.9999f64;
            }
            gsl_vector_set((*w).resfac, i, 1.0f64 / sqrt(1.0f64 - h));
            i = i.wrapping_add(1);
            i;
        }
        s = gsl_multifit_linear_residuals(X, y, c, (*w).r);
        if s != 0 {
            return s;
        }
        (*w)
            .stats
            .sigma_ols = gsl_blas_dnrm2((*w).r) / sqrt((*w).stats.dof as libc::c_double);
        while converged == 0
            && {
                numit = numit.wrapping_add(1);
                numit <= (*w).maxiter
            }
        {
            let mut sig: libc::c_double = 0.;
            s = gsl_vector_mul((*w).r, (*w).resfac);
            if s != 0 {
                return s;
            }
            sig = robust_madsigma((*w).r, (*w).p, (*w).workn);
            gsl_vector_scale(
                (*w).r,
                1.0f64 / ((if sig > sig_lower { sig } else { sig_lower }) * (*w).tune),
            );
            s = ((*(*w).type_0).wfun).unwrap()((*w).r, (*w).weights);
            if s != 0 {
                return s;
            }
            gsl_vector_memcpy((*w).c_prev, c);
            s = gsl_multifit_wlinear(
                X,
                (*w).weights,
                y,
                c,
                cov,
                &mut chisq,
                (*w).multifit_p,
            );
            if s != 0 {
                return s;
            }
            s = gsl_multifit_linear_residuals(X, y, c, (*w).r);
            if s != 0 {
                return s;
            }
            converged = robust_test_convergence((*w).c_prev, c, tol);
        }
        (*w).stats.sigma_mad = robust_madsigma((*w).r, (*w).p, (*w).workn);
        (*w)
            .stats
            .sigma_rob = robust_robsigma((*w).r, (*w).stats.sigma_mad, (*w).tune, w);
        (*w).stats.sigma = robust_sigma((*w).stats.sigma_ols, (*w).stats.sigma_rob, w);
        (*w).stats.numit = numit;
        let mut dof: libc::c_double = (*w).stats.dof as libc::c_double;
        let mut rnorm: libc::c_double = (*w).stats.sigma * sqrt(dof);
        let mut ss_err: libc::c_double = rnorm * rnorm;
        let mut ss_tot: libc::c_double = gsl_stats_tss(
            (*y).data as *const libc::c_double,
            (*y).stride,
            n,
        );
        (*w).stats.Rsq = 1.0f64 - ss_err / ss_tot;
        (*w)
            .stats
            .adj_Rsq = 1.0f64
            - (1.0f64 - (*w).stats.Rsq) * (n as libc::c_double - 1.0f64) / dof;
        (*w).stats.rmse = sqrt(ss_err / dof);
        (*w).stats.sse = ss_err;
        s = robust_covariance((*w).stats.sigma, cov, w);
        if s != 0 {
            return s;
        }
        if numit > (*w).maxiter {
            gsl_error(
                b"maximum iterations exceeded\0" as *const u8 as *const libc::c_char,
                b"multirobust.c\0" as *const u8 as *const libc::c_char,
                473 as libc::c_int,
                GSL_EMAXITER as libc::c_int,
            );
            return GSL_EMAXITER as libc::c_int;
        }
        return s;
    };
}
pub unsafe extern "C" fn gsl_multifit_robust_est(
    mut x: *const gsl_vector,
    mut c: *const gsl_vector,
    mut cov: *const gsl_matrix,
    mut y: *mut libc::c_double,
    mut y_err: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = gsl_multifit_linear_est(x, c, cov, y, y_err);
    return s;
}
pub unsafe extern "C" fn gsl_multifit_robust_residuals(
    mut X: *const gsl_matrix,
    mut y: *const gsl_vector,
    mut c: *const gsl_vector,
    mut r: *mut gsl_vector,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_int {
    if (*X).size1 != (*y).size {
        gsl_error(
            b"number of observations in y does not match rows of matrix X\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*X).size2 != (*c).size {
        gsl_error(
            b"number of parameters c does not match columns of matrix X\0" as *const u8
                as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            521 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*y).size != (*r).size {
        gsl_error(
            b"number of observations in y does not match number of residuals\0"
                as *const u8 as *const libc::c_char,
            b"multirobust.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let sigma: libc::c_double = (*w).stats.sigma;
        let mut s: libc::c_int = 0;
        let mut i: size_t = 0;
        s = gsl_multifit_linear_residuals(X, y, c, r);
        if s != 0 {
            return s;
        }
        i = 0 as libc::c_int as size_t;
        while i < (*r).size {
            let mut hfac: libc::c_double = gsl_vector_get((*w).resfac, i);
            let mut ri: *mut libc::c_double = gsl_vector_ptr(r, i);
            *ri *= hfac / sigma;
            i = i.wrapping_add(1);
            i;
        }
        return s;
    };
}
unsafe extern "C" fn robust_test_convergence(
    mut c_prev: *const gsl_vector,
    mut c: *const gsl_vector,
    tol: libc::c_double,
) -> libc::c_int {
    let mut p: size_t = (*c).size;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < p {
        let mut ai: libc::c_double = gsl_vector_get(c_prev, i);
        let mut bi: libc::c_double = gsl_vector_get(c, i);
        if fabs(bi - ai) > tol * (if fabs(ai) > fabs(bi) { fabs(ai) } else { fabs(bi) })
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn robust_madsigma(
    mut r: *const gsl_vector,
    p: size_t,
    mut workn: *mut gsl_vector,
) -> libc::c_double {
    let mut n: size_t = (*r).size;
    let mut sigma: libc::c_double = 0.;
    let mut i: size_t = 0;
    let mut v1: gsl_vector_view = gsl_vector_subvector(
        workn,
        0 as libc::c_int as size_t,
        n,
    );
    let mut v2: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(&mut v1.vector, i, fabs(gsl_vector_get(r, i)));
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector(&mut v1.vector);
    v2 = gsl_vector_subvector(
        &mut v1.vector,
        p.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        n.wrapping_sub(p).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sigma = gsl_stats_median_from_sorted_data(
        v2.vector.data as *const libc::c_double,
        v2.vector.stride,
        v2.vector.size,
    ) / 0.6745f64;
    return sigma;
}
unsafe extern "C" fn robust_robsigma(
    mut r: *const gsl_vector,
    s: libc::c_double,
    tune: libc::c_double,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_double {
    let mut sigma: libc::c_double = 0.;
    let mut i: size_t = 0;
    let n: size_t = (*w).n;
    let p: size_t = (*w).p;
    let st: libc::c_double = s * tune;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut lambda: libc::c_double = 0.;
    gsl_vector_memcpy((*w).workn, r);
    gsl_vector_mul((*w).workn, (*w).resfac);
    gsl_vector_scale((*w).workn, 1.0f64 / st);
    ((*(*w).type_0).wfun).unwrap()((*w).workn, (*w).psi);
    ((*(*w).type_0).psi_deriv).unwrap()((*w).workn, (*w).dpsi);
    gsl_vector_mul((*w).psi, (*w).workn);
    a = gsl_stats_mean(
        (*(*w).dpsi).data as *const libc::c_double,
        (*(*w).dpsi).stride,
        n,
    );
    b = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut psi_i: libc::c_double = gsl_vector_get((*w).psi, i);
        let mut resfac: libc::c_double = gsl_vector_get((*w).resfac, i);
        let mut fac: libc::c_double = 1.0f64 / (resfac * resfac);
        b += fac * psi_i * psi_i;
        i = i.wrapping_add(1);
        i;
    }
    b /= n.wrapping_sub(p) as libc::c_double;
    lambda = 1.0f64 + p as libc::c_double / n as libc::c_double * (1.0f64 - a) / a;
    sigma = lambda * sqrt(b) * st / a;
    return sigma;
}
unsafe extern "C" fn robust_sigma(
    s_ols: libc::c_double,
    s_rob: libc::c_double,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_double {
    let mut sigma: libc::c_double = 0.;
    let p: libc::c_double = (*w).p as libc::c_double;
    let n: libc::c_double = (*w).n as libc::c_double;
    sigma = if s_rob > sqrt((s_ols * s_ols * p * p + s_rob * s_rob * n) / (p * p + n)) {
        s_rob
    } else {
        sqrt((s_ols * s_ols * p * p + s_rob * s_rob * n) / (p * p + n))
    };
    return sigma;
}
unsafe extern "C" fn robust_covariance(
    sigma: libc::c_double,
    mut cov: *mut gsl_matrix,
    mut w: *mut gsl_multifit_robust_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let p: size_t = (*w).p;
    let s2: libc::c_double = sigma * sigma;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut QSI: *mut gsl_matrix = (*w).QSI;
    let mut D: *mut gsl_vector = (*w).D;
    i = 0 as libc::c_int as size_t;
    while i < p {
        let mut row_i: gsl_vector_view = gsl_matrix_row(QSI, i);
        let mut d_i: libc::c_double = gsl_vector_get(D, i);
        j = i;
        while j < p {
            let mut row_j: gsl_vector_view = gsl_matrix_row(QSI, j);
            let mut d_j: libc::c_double = gsl_vector_get(D, j);
            let mut s: libc::c_double = 0.;
            gsl_blas_ddot(&mut row_i.vector, &mut row_j.vector, &mut s);
            gsl_matrix_set(cov, i, j, s * s2 / (d_i * d_j));
            gsl_matrix_set(cov, j, i, s * s2 / (d_i * d_j));
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return status;
}
