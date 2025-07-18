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
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_mul(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
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
pub type gsl_multifit_nlinear_fdtype = libc::c_uint;
pub const GSL_MULTIFIT_NLINEAR_CTRDIFF: gsl_multifit_nlinear_fdtype = 1;
pub const GSL_MULTIFIT_NLINEAR_FWDIFF: gsl_multifit_nlinear_fdtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_fdf {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub fvv: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
    pub nevalfvv: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trs {
    pub name: *const libc::c_char,
    pub alloc: Option::<
        unsafe extern "C" fn(*const libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub preloop: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub step: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            libc::c_double,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub preduction: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const gsl_vector,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_scale {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
    pub update: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_solver {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub presolve: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub solve: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut gsl_vector,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_parameters {
    pub trs: *const gsl_multifit_nlinear_trs,
    pub scale: *const gsl_multifit_nlinear_scale,
    pub solver: *const gsl_multifit_nlinear_solver,
    pub fdtype: gsl_multifit_nlinear_fdtype,
    pub factor_up: libc::c_double,
    pub factor_down: libc::c_double,
    pub avmax: libc::c_double,
    pub h_df: libc::c_double,
    pub h_fvv: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trust_state {
    pub x: *const gsl_vector,
    pub f: *const gsl_vector,
    pub g: *const gsl_vector,
    pub J: *const gsl_matrix,
    pub diag: *const gsl_vector,
    pub sqrt_wts: *const gsl_vector,
    pub mu: *const libc::c_double,
    pub params: *const gsl_multifit_nlinear_parameters,
    pub solver_state: *mut libc::c_void,
    pub fdf: *mut gsl_multifit_nlinear_fdf,
    pub avratio: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dogleg_state_t {
    pub n: size_t,
    pub p: size_t,
    pub dx_gn: *mut gsl_vector,
    pub dx_sd: *mut gsl_vector,
    pub norm_Dgn: libc::c_double,
    pub norm_Dsd: libc::c_double,
    pub norm_Dinvg: libc::c_double,
    pub norm_JDinv2g: libc::c_double,
    pub workp: *mut gsl_vector,
    pub workn: *mut gsl_vector,
    pub params: gsl_multifit_nlinear_parameters,
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
unsafe extern "C" fn scaled_enorm(
    mut d: *const gsl_vector,
    mut f: *const gsl_vector,
) -> libc::c_double {
    let mut e2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    let mut n: size_t = (*f).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut di: libc::c_double = gsl_vector_get(d, i);
        let mut u: libc::c_double = di * fi;
        e2 += u * u;
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(e2);
}
unsafe extern "C" fn scaled_addition(
    alpha: libc::c_double,
    mut x: *const gsl_vector,
    beta: libc::c_double,
    mut y: *const gsl_vector,
    mut z: *mut gsl_vector,
) {
    let N: size_t = (*z).size;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        let mut yi: libc::c_double = gsl_vector_get(y, i);
        gsl_vector_set(z, i, alpha * xi + beta * yi);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn quadratic_preduction(
    mut f: *const gsl_vector,
    mut J: *const gsl_matrix,
    mut dx: *const gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_double {
    let n: size_t = (*f).size;
    let normf: libc::c_double = gsl_blas_dnrm2(f);
    let mut pred_reduction: libc::c_double = 0.;
    let mut norm_beta: libc::c_double = 0.;
    let mut i: size_t = 0;
    gsl_blas_dgemv(CblasNoTrans, 1.0f64 / normf, J, dx, 0.0f64, work);
    norm_beta = gsl_blas_dnrm2(work);
    pred_reduction = -norm_beta * norm_beta;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut betai: libc::c_double = gsl_vector_get(work, i);
        pred_reduction -= 2.0f64 * (fi / normf) * betai;
        i = i.wrapping_add(1);
        i;
    }
    return pred_reduction;
}
unsafe extern "C" fn dogleg_alloc(
    mut params: *const libc::c_void,
    n: size_t,
    p: size_t,
) -> *mut libc::c_void {
    let mut mparams: *const gsl_multifit_nlinear_parameters = params
        as *const gsl_multifit_nlinear_parameters;
    let mut state: *mut dogleg_state_t = 0 as *mut dogleg_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<dogleg_state_t>() as libc::c_ulong,
    ) as *mut dogleg_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate dogleg state\0" as *const u8 as *const libc::c_char,
            b"dogleg.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dx_gn = gsl_vector_alloc(p);
    if ((*state).dx_gn).is_null() {
        gsl_error(
            b"failed to allocate space for dx_gn\0" as *const u8 as *const libc::c_char,
            b"dogleg.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).dx_sd = gsl_vector_alloc(p);
    if ((*state).dx_sd).is_null() {
        gsl_error(
            b"failed to allocate space for dx_sd\0" as *const u8 as *const libc::c_char,
            b"dogleg.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workp = gsl_vector_alloc(p);
    if ((*state).workp).is_null() {
        gsl_error(
            b"failed to allocate space for workp\0" as *const u8 as *const libc::c_char,
            b"dogleg.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workn = gsl_vector_alloc(n);
    if ((*state).workn).is_null() {
        gsl_error(
            b"failed to allocate space for workn\0" as *const u8 as *const libc::c_char,
            b"dogleg.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).n = n;
    (*state).p = p;
    (*state).params = *mparams;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn dogleg_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut dogleg_state_t = vstate as *mut dogleg_state_t;
    if !((*state).dx_gn).is_null() {
        gsl_vector_free((*state).dx_gn);
    }
    if !((*state).dx_sd).is_null() {
        gsl_vector_free((*state).dx_sd);
    }
    if !((*state).workp).is_null() {
        gsl_vector_free((*state).workp);
    }
    if !((*state).workn).is_null() {
        gsl_vector_free((*state).workn);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn dogleg_init(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dogleg_preloop(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut dogleg_state_t = vstate as *mut dogleg_state_t;
    let mut u: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    gsl_vector_memcpy((*state).workp, (*trust_state).g);
    gsl_vector_div((*state).workp, (*trust_state).diag);
    (*state).norm_Dinvg = gsl_blas_dnrm2((*state).workp);
    gsl_vector_div((*state).workp, (*trust_state).diag);
    gsl_blas_dgemv(
        CblasNoTrans,
        1.0f64,
        (*trust_state).J,
        (*state).workp,
        0.0f64,
        (*state).workn,
    );
    (*state).norm_JDinv2g = gsl_blas_dnrm2((*state).workn);
    u = (*state).norm_Dinvg / (*state).norm_JDinv2g;
    alpha = u * u;
    gsl_vector_memcpy((*state).dx_sd, (*state).workp);
    gsl_vector_scale((*state).dx_sd, -alpha);
    (*state).norm_Dsd = scaled_enorm((*trust_state).diag, (*state).dx_sd);
    (*state).norm_Dgn = -1.0f64;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dogleg_step(
    mut vtrust_state: *const libc::c_void,
    delta: libc::c_double,
    mut dx: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut dogleg_state_t = vstate as *mut dogleg_state_t;
    if (*state).norm_Dsd >= delta {
        gsl_vector_memcpy(dx, (*state).dx_sd);
        gsl_vector_scale(dx, delta / (*state).norm_Dsd);
    } else {
        if (*state).norm_Dgn < 0.0f64 {
            let mut status: libc::c_int = dogleg_calc_gn(trust_state, (*state).dx_gn);
            if status != 0 {
                return status;
            }
            (*state).norm_Dgn = scaled_enorm((*trust_state).diag, (*state).dx_gn);
        }
        if (*state).norm_Dgn <= delta {
            gsl_vector_memcpy(dx, (*state).dx_gn);
        } else {
            let mut beta: libc::c_double = dogleg_beta(
                1.0f64,
                delta,
                (*trust_state).diag,
                state,
            );
            scaled_addition(
                1.0f64,
                (*state).dx_gn,
                -1.0f64,
                (*state).dx_sd,
                (*state).workp,
            );
            scaled_addition(beta, (*state).workp, 1.0f64, (*state).dx_sd, dx);
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dogleg_double_step(
    mut vtrust_state: *const libc::c_void,
    delta: libc::c_double,
    mut dx: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let alpha_fac: libc::c_double = 0.8f64;
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut dogleg_state_t = vstate as *mut dogleg_state_t;
    if (*state).norm_Dsd >= delta {
        gsl_vector_memcpy(dx, (*state).dx_sd);
        gsl_vector_scale(dx, delta / (*state).norm_Dsd);
    } else {
        if (*state).norm_Dgn < 0.0f64 {
            let mut status: libc::c_int = dogleg_calc_gn(trust_state, (*state).dx_gn);
            if status != 0 {
                return status;
            }
            (*state).norm_Dgn = scaled_enorm((*trust_state).diag, (*state).dx_gn);
        }
        if (*state).norm_Dgn <= delta {
            gsl_vector_memcpy(dx, (*state).dx_gn);
        } else {
            let mut t: libc::c_double = 0.;
            let mut u: libc::c_double = 0.;
            let mut v: libc::c_double = 0.;
            let mut c: libc::c_double = 0.;
            v = (*state).norm_Dinvg / (*state).norm_JDinv2g;
            u = v * v;
            gsl_blas_ddot((*trust_state).g, (*state).dx_gn, &mut v);
            c = u * ((*state).norm_Dinvg / fabs(v)) * (*state).norm_Dinvg;
            t = 1.0f64 - alpha_fac * (1.0f64 - c);
            if t * (*state).norm_Dgn <= delta {
                gsl_vector_memcpy(dx, (*state).dx_gn);
                gsl_vector_scale(dx, delta / (*state).norm_Dgn);
            } else {
                let mut beta: libc::c_double = dogleg_beta(
                    t,
                    delta,
                    (*trust_state).diag,
                    state,
                );
                scaled_addition(
                    t,
                    (*state).dx_gn,
                    -1.0f64,
                    (*state).dx_sd,
                    (*state).workp,
                );
                scaled_addition(beta, (*state).workp, 1.0f64, (*state).dx_sd, dx);
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dogleg_preduction(
    mut vtrust_state: *const libc::c_void,
    mut dx: *const gsl_vector,
    mut pred: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut dogleg_state_t = vstate as *mut dogleg_state_t;
    *pred = quadratic_preduction((*trust_state).f, (*trust_state).J, dx, (*state).workn);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dogleg_calc_gn(
    mut trust_state: *const gsl_multifit_nlinear_trust_state,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut params: *const gsl_multifit_nlinear_parameters = (*trust_state).params;
    status = ((*(*params).solver).init)
        .unwrap()(trust_state as *const libc::c_void, (*trust_state).solver_state);
    if status != 0 {
        return status;
    }
    status = ((*(*params).solver).presolve)
        .unwrap()(
        0.0f64,
        trust_state as *const libc::c_void,
        (*trust_state).solver_state,
    );
    if status != 0 {
        return status;
    }
    status = ((*(*params).solver).solve)
        .unwrap()(
        (*trust_state).f,
        dx,
        trust_state as *const libc::c_void,
        (*trust_state).solver_state,
    );
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dogleg_beta(
    t: libc::c_double,
    delta: libc::c_double,
    mut diag: *const gsl_vector,
    mut state: *mut dogleg_state_t,
) -> libc::c_double {
    let mut beta: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    scaled_addition(t, (*state).dx_gn, -1.0f64, (*state).dx_sd, (*state).workp);
    a = scaled_enorm(diag, (*state).workp);
    a *= a;
    gsl_vector_mul((*state).workp, diag);
    gsl_vector_mul((*state).workp, diag);
    gsl_blas_ddot((*state).dx_sd, (*state).workp, &mut b);
    b *= 2.0f64;
    c = ((*state).norm_Dsd + delta) * ((*state).norm_Dsd - delta);
    if b > 0.0f64 {
        beta = -2.0f64 * c / (b + sqrt(b * b - 4.0f64 * a * c));
    } else {
        beta = (-b + sqrt(b * b - 4.0f64 * a * c)) / (2.0f64 * a);
    }
    return beta;
}
static mut dogleg_type: gsl_multifit_nlinear_trs = unsafe {
    {
        let mut init = gsl_multifit_nlinear_trs {
            name: b"dogleg\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                dogleg_alloc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            init: Some(
                dogleg_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preloop: Some(
                dogleg_preloop
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            step: Some(
                dogleg_step
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        libc::c_double,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preduction: Some(
                dogleg_preduction
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const gsl_vector,
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(dogleg_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multifit_nlinear_trs_dogleg: *const gsl_multifit_nlinear_trs = unsafe {
    &dogleg_type as *const gsl_multifit_nlinear_trs
};
static mut ddogleg_type: gsl_multifit_nlinear_trs = unsafe {
    {
        let mut init = gsl_multifit_nlinear_trs {
            name: b"double-dogleg\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                dogleg_alloc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            init: Some(
                dogleg_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preloop: Some(
                dogleg_preloop
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            step: Some(
                dogleg_double_step
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        libc::c_double,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preduction: Some(
                dogleg_preduction
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const gsl_vector,
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(dogleg_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multifit_nlinear_trs_ddogleg: *const gsl_multifit_nlinear_trs = unsafe {
    &ddogleg_type as *const gsl_multifit_nlinear_trs
};
