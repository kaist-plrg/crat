use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn gsl_matrix_const_column(
        m: *const gsl_matrix,
        j: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_multifit_nlinear_eval_f(
        fdf: *mut gsl_multifit_nlinear_fdf,
        x: *const gsl_vector,
        swts: *const gsl_vector,
        y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_multifit_nlinear_eval_df(
        x: *const gsl_vector,
        f: *const gsl_vector,
        swts: *const gsl_vector,
        h: libc::c_double,
        fdtype: gsl_multifit_nlinear_fdtype,
        fdf: *mut gsl_multifit_nlinear_fdf,
        df: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    static mut gsl_multifit_nlinear_trs_lmaccel: *const gsl_multifit_nlinear_trs;
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
pub struct gsl_multifit_nlinear_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<
        unsafe extern "C" fn(
            *const gsl_multifit_nlinear_parameters,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_nlinear_fdf,
            *const gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_nlinear_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_matrix,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub avratio: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
pub struct trust_state_t {
    pub n: size_t,
    pub p: size_t,
    pub delta: libc::c_double,
    pub mu: libc::c_double,
    pub nu: libc::c_long,
    pub diag: *mut gsl_vector,
    pub x_trial: *mut gsl_vector,
    pub f_trial: *mut gsl_vector,
    pub workp: *mut gsl_vector,
    pub workn: *mut gsl_vector,
    pub trs_state: *mut libc::c_void,
    pub solver_state: *mut libc::c_void,
    pub avratio: libc::c_double,
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
unsafe extern "C" fn nielsen_init(
    mut J: *const gsl_matrix,
    mut diag: *const gsl_vector,
    mut mu: *mut libc::c_double,
    mut nu: *mut libc::c_long,
) -> libc::c_int {
    let mu0: libc::c_double = 1.0e-3f64;
    let p: size_t = (*J).size2;
    let mut j: size_t = 0;
    let mut max: libc::c_double = -1.0f64;
    *nu = 2 as libc::c_int as libc::c_long;
    j = 0 as libc::c_int as size_t;
    while j < p {
        let v: gsl_vector_const_view = gsl_matrix_const_column(J, j);
        let mut dj: libc::c_double = gsl_vector_get(diag, j);
        let mut norm: libc::c_double = gsl_blas_dnrm2(&v.vector) / dj;
        max = if max > norm { max } else { norm };
        j = j.wrapping_add(1);
        j;
    }
    *mu = mu0 * max * max;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn nielsen_accept(
    rho: libc::c_double,
    mut mu: *mut libc::c_double,
    mut nu: *mut libc::c_long,
) -> libc::c_int {
    let mut b: libc::c_double = 0.;
    *nu = 2 as libc::c_int as libc::c_long;
    b = 2.0f64 * rho - 1.0f64;
    b = 1.0f64 - b * b * b;
    *mu *= if 0.333333333333333f64 > b { 0.333333333333333f64 } else { b };
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn nielsen_reject(
    mut mu: *mut libc::c_double,
    mut nu: *mut libc::c_long,
) -> libc::c_int {
    *mu *= *nu as libc::c_double;
    *nu <<= 1 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn trust_alloc(
    mut params: *const gsl_multifit_nlinear_parameters,
    n: size_t,
    p: size_t,
) -> *mut libc::c_void {
    let mut state: *mut trust_state_t = 0 as *mut trust_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<trust_state_t>() as libc::c_ulong,
    ) as *mut trust_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate lm state\0" as *const u8 as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).diag = gsl_vector_alloc(p);
    if ((*state).diag).is_null() {
        gsl_error(
            b"failed to allocate space for diag\0" as *const u8 as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workp = gsl_vector_alloc(p);
    if ((*state).workp).is_null() {
        gsl_error(
            b"failed to allocate space for workp\0" as *const u8 as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workn = gsl_vector_alloc(n);
    if ((*state).workn).is_null() {
        gsl_error(
            b"failed to allocate space for workn\0" as *const u8 as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).x_trial = gsl_vector_alloc(p);
    if ((*state).x_trial).is_null() {
        gsl_error(
            b"failed to allocate space for x_trial\0" as *const u8
                as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).f_trial = gsl_vector_alloc(n);
    if ((*state).f_trial).is_null() {
        gsl_error(
            b"failed to allocate space for f_trial\0" as *const u8
                as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .trs_state = ((*(*params).trs).alloc)
        .unwrap()(params as *const libc::c_void, n, p);
    if ((*state).trs_state).is_null() {
        gsl_error(
            b"failed to allocate space for trs state\0" as *const u8
                as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).solver_state = ((*(*params).solver).alloc).unwrap()(n, p);
    if ((*state).solver_state).is_null() {
        gsl_error(
            b"failed to allocate space for solver state\0" as *const u8
                as *const libc::c_char,
            b"trust.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).n = n;
    (*state).p = p;
    (*state).delta = 0.0f64;
    (*state).params = *params;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn trust_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut trust_state_t = vstate as *mut trust_state_t;
    let mut params: *const gsl_multifit_nlinear_parameters = &mut (*state).params;
    if !((*state).diag).is_null() {
        gsl_vector_free((*state).diag);
    }
    if !((*state).workp).is_null() {
        gsl_vector_free((*state).workp);
    }
    if !((*state).workn).is_null() {
        gsl_vector_free((*state).workn);
    }
    if !((*state).x_trial).is_null() {
        gsl_vector_free((*state).x_trial);
    }
    if !((*state).f_trial).is_null() {
        gsl_vector_free((*state).f_trial);
    }
    if !((*state).trs_state).is_null() {
        ((*(*params).trs).free).unwrap()((*state).trs_state);
    }
    if !((*state).solver_state).is_null() {
        ((*(*params).solver).free).unwrap()((*state).solver_state);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn trust_init(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut x: *const gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut g: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut state: *mut trust_state_t = vstate as *mut trust_state_t;
    let mut params: *const gsl_multifit_nlinear_parameters = &mut (*state).params;
    let mut Dx: libc::c_double = 0.;
    status = gsl_multifit_nlinear_eval_f(fdf, x, swts, f);
    if status != 0 {
        return status;
    }
    status = gsl_multifit_nlinear_eval_df(
        x,
        f,
        swts,
        (*params).h_df,
        (*params).fdtype,
        fdf,
        J,
        (*state).workn,
    );
    if status != 0 {
        return status;
    }
    gsl_blas_dgemv(CblasTrans, 1.0f64, J, f, 0.0f64, g);
    ((*(*params).scale).init).unwrap()(J, (*state).diag);
    Dx = trust_scaled_norm((*state).diag, x);
    (*state).delta = 0.3f64 * (if 1.0f64 > Dx { 1.0f64 } else { Dx });
    status = nielsen_init(J, (*state).diag, &mut (*state).mu, &mut (*state).nu);
    if status != 0 {
        return status;
    }
    let mut trust_state: gsl_multifit_nlinear_trust_state = gsl_multifit_nlinear_trust_state {
        x: 0 as *const gsl_vector,
        f: 0 as *const gsl_vector,
        g: 0 as *const gsl_vector,
        J: 0 as *const gsl_matrix,
        diag: 0 as *const gsl_vector,
        sqrt_wts: 0 as *const gsl_vector,
        mu: 0 as *const libc::c_double,
        params: 0 as *const gsl_multifit_nlinear_parameters,
        solver_state: 0 as *mut libc::c_void,
        fdf: 0 as *mut gsl_multifit_nlinear_fdf,
        avratio: 0 as *mut libc::c_double,
    };
    trust_state.x = x;
    trust_state.f = f;
    trust_state.g = g;
    trust_state.J = J;
    trust_state.diag = (*state).diag;
    trust_state.sqrt_wts = swts;
    trust_state.mu = &mut (*state).mu;
    trust_state.params = params;
    trust_state.solver_state = (*state).solver_state;
    trust_state.fdf = fdf;
    trust_state.avratio = &mut (*state).avratio;
    status = ((*(*params).trs).init)
        .unwrap()(
        &mut trust_state as *mut gsl_multifit_nlinear_trust_state as *const libc::c_void,
        (*state).trs_state,
    );
    if status != 0 {
        return status;
    }
    (*state).avratio = 0.0f64;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn trust_iterate(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_nlinear_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut J: *mut gsl_matrix,
    mut g: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut state: *mut trust_state_t = vstate as *mut trust_state_t;
    let mut params: *const gsl_multifit_nlinear_parameters = &mut (*state).params;
    let mut trs: *const gsl_multifit_nlinear_trs = (*params).trs;
    let mut trust_state: gsl_multifit_nlinear_trust_state = gsl_multifit_nlinear_trust_state {
        x: 0 as *const gsl_vector,
        f: 0 as *const gsl_vector,
        g: 0 as *const gsl_vector,
        J: 0 as *const gsl_matrix,
        diag: 0 as *const gsl_vector,
        sqrt_wts: 0 as *const gsl_vector,
        mu: 0 as *const libc::c_double,
        params: 0 as *const gsl_multifit_nlinear_parameters,
        solver_state: 0 as *mut libc::c_void,
        fdf: 0 as *mut gsl_multifit_nlinear_fdf,
        avratio: 0 as *mut libc::c_double,
    };
    let mut x_trial: *mut gsl_vector = (*state).x_trial;
    let mut f_trial: *mut gsl_vector = (*state).f_trial;
    let mut diag: *mut gsl_vector = (*state).diag;
    let mut rho: libc::c_double = 0.;
    let mut foundstep: libc::c_int = 0 as libc::c_int;
    let mut bad_steps: libc::c_int = 0 as libc::c_int;
    trust_state.x = x;
    trust_state.f = f;
    trust_state.g = g;
    trust_state.J = J;
    trust_state.diag = (*state).diag;
    trust_state.sqrt_wts = swts;
    trust_state.mu = &mut (*state).mu;
    trust_state.params = params;
    trust_state.solver_state = (*state).solver_state;
    trust_state.fdf = fdf;
    trust_state.avratio = &mut (*state).avratio;
    status = ((*trs).preloop)
        .unwrap()(
        &mut trust_state as *mut gsl_multifit_nlinear_trust_state as *const libc::c_void,
        (*state).trs_state,
    );
    if status != 0 {
        return status;
    }
    while foundstep == 0 {
        status = ((*trs).step)
            .unwrap()(
            &mut trust_state as *mut gsl_multifit_nlinear_trust_state
                as *const libc::c_void,
            (*state).delta,
            dx,
            (*state).trs_state,
        );
        if status == GSL_SUCCESS as libc::c_int {
            trust_trial_step(x, dx, x_trial);
            status = gsl_multifit_nlinear_eval_f(fdf, x_trial, swts, f_trial);
            if status != 0 {
                return status;
            }
            status = trust_eval_step(f, f_trial, g, J, dx, &mut rho, state);
            if status == GSL_SUCCESS as libc::c_int {
                foundstep = 1 as libc::c_int;
            }
        } else {
            rho = -1.0f64;
        }
        if rho > 0.75f64 {
            (*state).delta *= (*params).factor_up;
        } else if rho < 0.25f64 {
            (*state).delta /= (*params).factor_down;
        }
        if foundstep != 0 {
            status = gsl_multifit_nlinear_eval_df(
                x_trial,
                f_trial,
                swts,
                (*params).h_df,
                (*params).fdtype,
                fdf,
                J,
                (*state).workn,
            );
            if status != 0 {
                return status;
            }
            gsl_vector_memcpy(x, x_trial);
            gsl_vector_memcpy(f, f_trial);
            gsl_blas_dgemv(CblasTrans, 1.0f64, J, f, 0.0f64, g);
            ((*(*params).scale).update).unwrap()(J, diag);
            status = nielsen_accept(rho, &mut (*state).mu, &mut (*state).nu);
            if status != 0 {
                return status;
            }
            bad_steps = 0 as libc::c_int;
        } else {
            status = nielsen_reject(&mut (*state).mu, &mut (*state).nu);
            if status != 0 {
                return status;
            }
            bad_steps += 1;
            if bad_steps > 15 as libc::c_int {
                return GSL_ENOPROG as libc::c_int;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn trust_rcond(
    mut rcond: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut state: *mut trust_state_t = vstate as *mut trust_state_t;
    let mut params: *const gsl_multifit_nlinear_parameters = &mut (*state).params;
    status = ((*(*params).solver).rcond).unwrap()(rcond, (*state).solver_state);
    return status;
}
unsafe extern "C" fn trust_avratio(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut trust_state_t = vstate as *mut trust_state_t;
    return (*state).avratio;
}
unsafe extern "C" fn trust_trial_step(
    mut x: *const gsl_vector,
    mut dx: *const gsl_vector,
    mut x_trial: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut N: size_t = (*x).size;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut dxi: libc::c_double = gsl_vector_get(dx, i);
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        gsl_vector_set(x_trial, i, xi + dxi);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn trust_calc_rho(
    mut f: *const gsl_vector,
    mut f_trial: *const gsl_vector,
    mut g: *const gsl_vector,
    mut J: *const gsl_matrix,
    mut dx: *const gsl_vector,
    mut state: *mut trust_state_t,
) -> libc::c_double {
    let mut status: libc::c_int = 0;
    let mut params: *const gsl_multifit_nlinear_parameters = &mut (*state).params;
    let mut trs: *const gsl_multifit_nlinear_trs = (*params).trs;
    let normf: libc::c_double = gsl_blas_dnrm2(f);
    let normf_trial: libc::c_double = gsl_blas_dnrm2(f_trial);
    let mut trust_state: gsl_multifit_nlinear_trust_state = gsl_multifit_nlinear_trust_state {
        x: 0 as *const gsl_vector,
        f: 0 as *const gsl_vector,
        g: 0 as *const gsl_vector,
        J: 0 as *const gsl_matrix,
        diag: 0 as *const gsl_vector,
        sqrt_wts: 0 as *const gsl_vector,
        mu: 0 as *const libc::c_double,
        params: 0 as *const gsl_multifit_nlinear_parameters,
        solver_state: 0 as *mut libc::c_void,
        fdf: 0 as *mut gsl_multifit_nlinear_fdf,
        avratio: 0 as *mut libc::c_double,
    };
    let mut rho: libc::c_double = 0.;
    let mut actual_reduction: libc::c_double = 0.;
    let mut pred_reduction: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    if normf_trial >= normf {
        return -1.0f64;
    }
    trust_state.x = 0 as *const gsl_vector;
    trust_state.f = f;
    trust_state.g = g;
    trust_state.J = J;
    trust_state.diag = (*state).diag;
    trust_state.sqrt_wts = 0 as *const gsl_vector;
    trust_state.mu = &mut (*state).mu;
    trust_state.params = params;
    trust_state.solver_state = (*state).solver_state;
    trust_state.fdf = 0 as *mut gsl_multifit_nlinear_fdf;
    trust_state.avratio = &mut (*state).avratio;
    u = normf_trial / normf;
    actual_reduction = 1.0f64 - u * u;
    status = ((*trs).preduction)
        .unwrap()(
        &mut trust_state as *mut gsl_multifit_nlinear_trust_state as *const libc::c_void,
        dx,
        &mut pred_reduction,
        (*state).trs_state,
    );
    if status != 0 {
        return -1.0f64;
    }
    if pred_reduction > 0.0f64 {
        rho = actual_reduction / pred_reduction;
    } else {
        rho = -1.0f64;
    }
    return rho;
}
unsafe extern "C" fn trust_eval_step(
    mut f: *const gsl_vector,
    mut f_trial: *const gsl_vector,
    mut g: *const gsl_vector,
    mut J: *const gsl_matrix,
    mut dx: *const gsl_vector,
    mut rho: *mut libc::c_double,
    mut state: *mut trust_state_t,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut params: *const gsl_multifit_nlinear_parameters = &mut (*state).params;
    if (*params).trs == gsl_multifit_nlinear_trs_lmaccel {
        if (*state).avratio > (*params).avmax {
            status = GSL_FAILURE as libc::c_int;
        }
    }
    *rho = trust_calc_rho(f, f_trial, g, J, dx, state);
    if *rho <= 0.0f64 {
        status = GSL_FAILURE as libc::c_int;
    }
    return status;
}
unsafe extern "C" fn trust_scaled_norm(
    mut D: *const gsl_vector,
    mut a: *const gsl_vector,
) -> libc::c_double {
    let n: size_t = (*a).size;
    let mut e2: libc::c_double = 0.0f64;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut Di: libc::c_double = gsl_vector_get(D, i);
        let mut ai: libc::c_double = gsl_vector_get(a, i);
        let mut u: libc::c_double = Di * ai;
        e2 += u * u;
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(e2);
}
static mut trust_type: gsl_multifit_nlinear_type = unsafe {
    {
        let mut init = gsl_multifit_nlinear_type {
            name: b"trust-region\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                trust_alloc
                    as unsafe extern "C" fn(
                        *const gsl_multifit_nlinear_parameters,
                        size_t,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            init: Some(
                trust_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const gsl_vector,
                        *mut gsl_multifit_nlinear_fdf,
                        *const gsl_vector,
                        *mut gsl_vector,
                        *mut gsl_matrix,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
            iterate: Some(
                trust_iterate
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const gsl_vector,
                        *mut gsl_multifit_nlinear_fdf,
                        *mut gsl_vector,
                        *mut gsl_vector,
                        *mut gsl_matrix,
                        *mut gsl_vector,
                        *mut gsl_vector,
                    ) -> libc::c_int,
            ),
            rcond: Some(
                trust_rcond
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            avratio: Some(
                trust_avratio
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
            ),
            free: Some(trust_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multifit_nlinear_trust: *const gsl_multifit_nlinear_type = unsafe {
    &trust_type as *const gsl_multifit_nlinear_type
};
