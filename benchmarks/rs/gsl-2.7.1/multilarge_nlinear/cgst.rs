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
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dsymv(
        Uplo: CBLAS_UPLO_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_multilarge_nlinear_eval_df(
        TransJ: CBLAS_TRANSPOSE_t,
        x: *const gsl_vector,
        f: *const gsl_vector,
        u: *const gsl_vector,
        swts: *const gsl_vector,
        h: libc::c_double,
        fdtype: gsl_multilarge_nlinear_fdtype,
        fdf: *mut gsl_multilarge_nlinear_fdf,
        v: *mut gsl_vector,
        JTJ: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    static mut gsl_multilarge_nlinear_solver_cholesky: *const gsl_multilarge_nlinear_solver;
    static mut gsl_multilarge_nlinear_solver_mcholesky: *const gsl_multilarge_nlinear_solver;
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
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
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
pub type gsl_multilarge_nlinear_fdtype = libc::c_uint;
pub const GSL_MULTILARGE_NLINEAR_CTRDIFF: gsl_multilarge_nlinear_fdtype = 1;
pub const GSL_MULTILARGE_NLINEAR_FWDIFF: gsl_multilarge_nlinear_fdtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_fdf {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            CBLAS_TRANSPOSE_t,
            *const gsl_vector,
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
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
    pub nevaldfu: size_t,
    pub nevaldf2: size_t,
    pub nevalfvv: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_trs {
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
pub struct gsl_multilarge_nlinear_scale {
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
pub struct gsl_multilarge_nlinear_solver {
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
        unsafe extern "C" fn(
            *mut libc::c_double,
            *const gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub covar: Option::<
        unsafe extern "C" fn(
            *const gsl_matrix,
            *mut gsl_matrix,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_parameters {
    pub trs: *const gsl_multilarge_nlinear_trs,
    pub scale: *const gsl_multilarge_nlinear_scale,
    pub solver: *const gsl_multilarge_nlinear_solver,
    pub fdtype: gsl_multilarge_nlinear_fdtype,
    pub factor_up: libc::c_double,
    pub factor_down: libc::c_double,
    pub avmax: libc::c_double,
    pub h_df: libc::c_double,
    pub h_fvv: libc::c_double,
    pub max_iter: size_t,
    pub tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multilarge_nlinear_trust_state {
    pub x: *const gsl_vector,
    pub f: *const gsl_vector,
    pub g: *const gsl_vector,
    pub JTJ: *const gsl_matrix,
    pub diag: *const gsl_vector,
    pub sqrt_wts: *const gsl_vector,
    pub mu: *const libc::c_double,
    pub params: *const gsl_multilarge_nlinear_parameters,
    pub solver_state: *mut libc::c_void,
    pub fdf: *mut gsl_multilarge_nlinear_fdf,
    pub avratio: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cgst_state_t {
    pub n: size_t,
    pub p: size_t,
    pub z: *mut gsl_vector,
    pub r: *mut gsl_vector,
    pub d: *mut gsl_vector,
    pub workp: *mut gsl_vector,
    pub workn: *mut gsl_vector,
    pub norm_g: libc::c_double,
    pub cgtol: libc::c_double,
    pub cgmaxit: size_t,
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
    mut trust_state: *const gsl_multilarge_nlinear_trust_state,
    mut dx: *const gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_double {
    let mut f: *const gsl_vector = (*trust_state).f;
    let mut params: *const gsl_multilarge_nlinear_parameters = (*trust_state).params;
    let normf: libc::c_double = gsl_blas_dnrm2(f);
    let mut gTdx: libc::c_double = 0.;
    let mut fdf: *mut gsl_multilarge_nlinear_fdf = (*trust_state).fdf;
    let mut pred_reduction: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    gsl_blas_ddot((*trust_state).g, dx, &mut gTdx);
    pred_reduction = -2.0f64 * gTdx / (normf * normf);
    if (*params).solver == gsl_multilarge_nlinear_solver_cholesky
        || (*params).solver == gsl_multilarge_nlinear_solver_mcholesky
    {
        let p: size_t = (*fdf).p;
        let mut workp: gsl_vector_view = gsl_vector_subvector(
            work,
            0 as libc::c_int as size_t,
            p,
        );
        gsl_blas_dsymv(
            CblasLower,
            1.0f64,
            (*trust_state).JTJ,
            dx,
            0.0f64,
            &mut workp.vector,
        );
        gsl_blas_ddot(&mut workp.vector, dx, &mut u);
        pred_reduction -= u / (normf * normf);
    } else {
        let mut status: libc::c_int = 0;
        let mut x: *const gsl_vector = (*trust_state).x;
        let mut swts: *const gsl_vector = (*trust_state).sqrt_wts;
        status = gsl_multilarge_nlinear_eval_df(
            CblasNoTrans,
            x,
            f,
            dx,
            swts,
            (*params).h_df,
            (*params).fdtype,
            fdf,
            work,
            0 as *mut gsl_matrix,
            0 as *mut gsl_vector,
        );
        if status != 0 {
            gsl_error(
                b"error computing preduction\0" as *const u8 as *const libc::c_char,
                b"./common.c\0" as *const u8 as *const libc::c_char,
                122 as libc::c_int,
                status,
            );
            return 0.0f64;
        }
        u = gsl_blas_dnrm2(work) / normf;
        pred_reduction -= u * u;
    }
    return pred_reduction;
}
unsafe extern "C" fn cgst_alloc(
    mut params: *const libc::c_void,
    n: size_t,
    p: size_t,
) -> *mut libc::c_void {
    let mut par: *const gsl_multilarge_nlinear_parameters = params
        as *const gsl_multilarge_nlinear_parameters;
    let mut state: *mut cgst_state_t = 0 as *mut cgst_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cgst_state_t>() as libc::c_ulong,
    ) as *mut cgst_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate st state\0" as *const u8 as *const libc::c_char,
            b"cgst.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).z = gsl_vector_alloc(p);
    if ((*state).z).is_null() {
        gsl_error(
            b"failed to allocate space for z\0" as *const u8 as *const libc::c_char,
            b"cgst.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).r = gsl_vector_alloc(p);
    if ((*state).r).is_null() {
        gsl_error(
            b"failed to allocate space for r\0" as *const u8 as *const libc::c_char,
            b"cgst.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).d = gsl_vector_alloc(p);
    if ((*state).d).is_null() {
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"cgst.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workp = gsl_vector_alloc(p);
    if ((*state).workp).is_null() {
        gsl_error(
            b"failed to allocate space for workp\0" as *const u8 as *const libc::c_char,
            b"cgst.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workn = gsl_vector_alloc(n);
    if ((*state).workn).is_null() {
        gsl_error(
            b"failed to allocate space for workn\0" as *const u8 as *const libc::c_char,
            b"cgst.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).n = n;
    (*state).p = p;
    (*state).cgmaxit = (*par).max_iter;
    if (*state).cgmaxit == 0 as libc::c_int as libc::c_ulong {
        (*state).cgmaxit = n;
    }
    (*state).cgtol = (*par).tol;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn cgst_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut cgst_state_t = vstate as *mut cgst_state_t;
    if !((*state).z).is_null() {
        gsl_vector_free((*state).z);
    }
    if !((*state).r).is_null() {
        gsl_vector_free((*state).r);
    }
    if !((*state).d).is_null() {
        gsl_vector_free((*state).d);
    }
    if !((*state).workp).is_null() {
        gsl_vector_free((*state).workp);
    }
    if !((*state).workn).is_null() {
        gsl_vector_free((*state).workn);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn cgst_init(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn cgst_preloop(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn cgst_step(
    mut vtrust_state: *const libc::c_void,
    delta: libc::c_double,
    mut dx: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut trust_state: *const gsl_multilarge_nlinear_trust_state = vtrust_state
        as *const gsl_multilarge_nlinear_trust_state;
    let mut state: *mut cgst_state_t = vstate as *mut cgst_state_t;
    let mut x: *const gsl_vector = (*trust_state).x;
    let mut f: *const gsl_vector = (*trust_state).f;
    let mut swts: *const gsl_vector = (*trust_state).sqrt_wts;
    let mut diag: *const gsl_vector = (*trust_state).diag;
    let mut params: *const gsl_multilarge_nlinear_parameters = (*trust_state).params;
    let mut fdf: *mut gsl_multilarge_nlinear_fdf = (*trust_state).fdf;
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut norm_Jd: libc::c_double = 0.;
    let mut norm_r: libc::c_double = 0.;
    let mut norm_rp1: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*state).p {
        let mut gi: libc::c_double = gsl_vector_get((*trust_state).g, i);
        let mut di: libc::c_double = gsl_vector_get((*trust_state).diag, i);
        gsl_vector_set((*state).z, i, 0.0f64);
        gsl_vector_set((*state).r, i, -gi / di);
        gsl_vector_set((*state).d, i, -gi / di);
        gsl_vector_set((*state).workp, i, gi / di);
        i = i.wrapping_add(1);
        i;
    }
    (*state).norm_g = gsl_blas_dnrm2((*state).workp);
    i = 0 as libc::c_int as size_t;
    while i < (*state).cgmaxit {
        gsl_vector_memcpy((*state).workp, (*state).d);
        gsl_vector_div((*state).workp, (*trust_state).diag);
        status = gsl_multilarge_nlinear_eval_df(
            CblasNoTrans,
            x,
            f,
            (*state).workp,
            swts,
            (*params).h_df,
            (*params).fdtype,
            fdf,
            (*state).workn,
            0 as *mut gsl_matrix,
            0 as *mut gsl_vector,
        );
        if status != 0 {
            return status;
        }
        norm_Jd = gsl_blas_dnrm2((*state).workn);
        if norm_Jd == 0.0f64 {
            let mut tau: libc::c_double = cgst_calc_tau((*state).z, (*state).d, delta);
            scaled_addition(1.0f64, (*state).z, tau, (*state).d, dx);
            gsl_vector_div(dx, diag);
            return GSL_SUCCESS as libc::c_int;
        }
        norm_r = gsl_blas_dnrm2((*state).r);
        u = norm_r / norm_Jd;
        alpha = u * u;
        scaled_addition(1.0f64, (*state).z, alpha, (*state).d, (*state).workp);
        u = gsl_blas_dnrm2((*state).workp);
        if u >= delta {
            let mut tau_0: libc::c_double = cgst_calc_tau((*state).z, (*state).d, delta);
            scaled_addition(1.0f64, (*state).z, tau_0, (*state).d, dx);
            gsl_vector_div(dx, diag);
            return GSL_SUCCESS as libc::c_int;
        }
        gsl_vector_memcpy((*state).z, (*state).workp);
        status = gsl_multilarge_nlinear_eval_df(
            CblasTrans,
            x,
            f,
            (*state).workn,
            swts,
            (*params).h_df,
            (*params).fdtype,
            fdf,
            (*state).workp,
            0 as *mut gsl_matrix,
            0 as *mut gsl_vector,
        );
        if status != 0 {
            return status;
        }
        gsl_vector_div((*state).workp, (*trust_state).diag);
        gsl_vector_scale((*state).workp, alpha);
        gsl_vector_sub((*state).r, (*state).workp);
        norm_rp1 = gsl_blas_dnrm2((*state).r);
        u = norm_rp1 / (*state).norm_g;
        if u < (*state).cgtol {
            gsl_vector_memcpy(dx, (*state).z);
            gsl_vector_div(dx, diag);
            return GSL_SUCCESS as libc::c_int;
        }
        u = norm_rp1 / norm_r;
        beta = u * u;
        scaled_addition(1.0f64, (*state).r, beta, (*state).d, (*state).d);
        i = i.wrapping_add(1);
        i;
    }
    gsl_vector_memcpy(dx, (*state).z);
    gsl_vector_div(dx, diag);
    return GSL_EMAXITER as libc::c_int;
}
unsafe extern "C" fn cgst_preduction(
    mut vtrust_state: *const libc::c_void,
    mut dx: *const gsl_vector,
    mut pred: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multilarge_nlinear_trust_state = vtrust_state
        as *const gsl_multilarge_nlinear_trust_state;
    let mut state: *mut cgst_state_t = vstate as *mut cgst_state_t;
    *pred = quadratic_preduction(trust_state, dx, (*state).workn);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn cgst_calc_tau(
    mut p: *const gsl_vector,
    mut d: *const gsl_vector,
    delta: libc::c_double,
) -> libc::c_double {
    let mut norm_p: libc::c_double = 0.;
    let mut norm_d: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    norm_p = gsl_blas_dnrm2(p);
    norm_d = gsl_blas_dnrm2(d);
    gsl_blas_ddot(p, d, &mut u);
    t1 = u / (norm_d * norm_d);
    t2 = t1 * u + (delta + norm_p) * (delta - norm_p);
    tau = -t1 + sqrt(t2) / norm_d;
    return tau;
}
static mut cgst_type: gsl_multilarge_nlinear_trs = unsafe {
    {
        let mut init = gsl_multilarge_nlinear_trs {
            name: b"steihaug-toint\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                cgst_alloc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            init: Some(
                cgst_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preloop: Some(
                cgst_preloop
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            step: Some(
                cgst_step
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        libc::c_double,
                        *mut gsl_vector,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            preduction: Some(
                cgst_preduction
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const gsl_vector,
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(cgst_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multilarge_nlinear_trs_cgst: *const gsl_multilarge_nlinear_trs = unsafe {
    &cgst_type as *const gsl_multilarge_nlinear_trs
};
