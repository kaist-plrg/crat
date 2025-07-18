use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_equal(u: *const gsl_vector, v: *const gsl_vector) -> libc::c_int;
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
pub struct gsl_multimin_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut libc::c_void) -> libc::c_double,
    >,
    pub df: Option::<
        unsafe extern "C" fn(*const gsl_vector, *mut libc::c_void, *mut gsl_vector) -> (),
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut libc::c_double,
            *mut gsl_vector,
        ) -> (),
    >,
    pub n: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_multimin_function_fdf = gsl_multimin_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multimin_fdfminimizer_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int>,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function_fdf,
            *const gsl_vector,
            *mut libc::c_double,
            *mut gsl_vector,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_multimin_function_fdf,
            *mut gsl_vector,
            *mut libc::c_double,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub restart: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conjugate_pr_state_t {
    pub iter: libc::c_int,
    pub step: libc::c_double,
    pub max_step: libc::c_double,
    pub tol: libc::c_double,
    pub x1: *mut gsl_vector,
    pub dx1: *mut gsl_vector,
    pub x2: *mut gsl_vector,
    pub pnorm: libc::c_double,
    pub p: *mut gsl_vector,
    pub g0norm: libc::c_double,
    pub g0: *mut gsl_vector,
}
unsafe extern "C" fn take_step(
    mut x: *const gsl_vector,
    mut p: *const gsl_vector,
    mut step: libc::c_double,
    mut lambda: libc::c_double,
    mut x1: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) {
    gsl_vector_set_zero(dx);
    gsl_blas_daxpy(-step * lambda, p, dx);
    gsl_vector_memcpy(x1, x);
    gsl_blas_daxpy(1.0f64, dx, x1);
}
unsafe extern "C" fn intermediate_point(
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *const gsl_vector,
    mut p: *const gsl_vector,
    mut lambda: libc::c_double,
    mut pg: libc::c_double,
    mut stepa: libc::c_double,
    mut stepc: libc::c_double,
    mut fa: libc::c_double,
    mut fc: libc::c_double,
    mut x1: *mut gsl_vector,
    mut dx: *mut gsl_vector,
    mut gradient: *mut gsl_vector,
    mut step: *mut libc::c_double,
    mut f: *mut libc::c_double,
) {
    let mut stepb: libc::c_double = 0.;
    let mut fb: libc::c_double = 0.;
    loop {
        let mut u: libc::c_double = fabs(pg * lambda * stepc);
        stepb = 0.5f64 * stepc * u / (fc - fa + u);
        take_step(x, p, stepb, lambda, x1, dx);
        if gsl_vector_equal(x, x1) != 0 {
            *step = 0 as libc::c_int as libc::c_double;
            *f = fa;
            (Some(((*fdf).df).unwrap())).unwrap()(x1, (*fdf).params, gradient);
            return;
        }
        fb = (Some(((*fdf).f).unwrap())).unwrap()(x1, (*fdf).params);
        if !(fb >= fa && stepb > 0.0f64) {
            break;
        }
        fc = fb;
        stepc = stepb;
    }
    *step = stepb;
    *f = fb;
    (Some(((*fdf).df).unwrap())).unwrap()(x1, (*fdf).params, gradient);
}
unsafe extern "C" fn minimize(
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *const gsl_vector,
    mut p: *const gsl_vector,
    mut lambda: libc::c_double,
    mut stepa: libc::c_double,
    mut stepb: libc::c_double,
    mut stepc: libc::c_double,
    mut fa: libc::c_double,
    mut fb: libc::c_double,
    mut fc: libc::c_double,
    mut tol: libc::c_double,
    mut x1: *mut gsl_vector,
    mut dx1: *mut gsl_vector,
    mut x2: *mut gsl_vector,
    mut dx2: *mut gsl_vector,
    mut gradient: *mut gsl_vector,
    mut step: *mut libc::c_double,
    mut f: *mut libc::c_double,
    mut gnorm: *mut libc::c_double,
) {
    let mut u: libc::c_double = stepb;
    let mut v: libc::c_double = stepa;
    let mut w: libc::c_double = stepc;
    let mut fu: libc::c_double = fb;
    let mut fv: libc::c_double = fa;
    let mut fw: libc::c_double = fc;
    let mut old2: libc::c_double = fabs(w - v);
    let mut old1: libc::c_double = fabs(v - u);
    let mut stepm: libc::c_double = 0.;
    let mut fm: libc::c_double = 0.;
    let mut pg: libc::c_double = 0.;
    let mut gnorm1: libc::c_double = 0.;
    let mut iter: libc::c_int = 0 as libc::c_int;
    gsl_vector_memcpy(x2, x1);
    gsl_vector_memcpy(dx2, dx1);
    *f = fb;
    *step = stepb;
    *gnorm = gsl_blas_dnrm2(gradient);
    loop {
        iter += 1;
        iter;
        if iter > 10 as libc::c_int {
            return;
        }
        let mut dw: libc::c_double = w - u;
        let mut dv: libc::c_double = v - u;
        let mut du: libc::c_double = 0.0f64;
        let mut e1: libc::c_double = (fv - fu) * dw * dw + (fu - fw) * dv * dv;
        let mut e2: libc::c_double = 2.0f64 * ((fv - fu) * dw + (fu - fw) * dv);
        if e2 != 0.0f64 {
            du = e1 / e2;
        }
        if du > 0.0f64 && du < stepc - stepb && fabs(du) < 0.5f64 * old2 {
            stepm = u + du;
        } else if du < 0.0f64 && du > stepa - stepb && fabs(du) < 0.5f64 * old2 {
            stepm = u + du;
        } else if stepc - stepb > stepb - stepa {
            stepm = 0.38f64 * (stepc - stepb) + stepb;
        } else {
            stepm = stepb - 0.38f64 * (stepb - stepa);
        }
        take_step(x, p, stepm, lambda, x1, dx1);
        fm = (Some(((*fdf).f).unwrap())).unwrap()(x1, (*fdf).params);
        if fm > fb {
            if fm < fv {
                w = v;
                v = stepm;
                fw = fv;
                fv = fm;
            } else if fm < fw {
                w = stepm;
                fw = fm;
            }
            if stepm < stepb {
                stepa = stepm;
                fa = fm;
            } else {
                stepc = stepm;
                fc = fm;
            }
        } else {
            if !(fm <= fb) {
                break;
            }
            old2 = old1;
            old1 = fabs(u - stepm);
            w = v;
            v = u;
            u = stepm;
            fw = fv;
            fv = fu;
            fu = fm;
            gsl_vector_memcpy(x2, x1);
            gsl_vector_memcpy(dx2, dx1);
            (Some(((*fdf).df).unwrap())).unwrap()(x1, (*fdf).params, gradient);
            gsl_blas_ddot(p, gradient, &mut pg);
            gnorm1 = gsl_blas_dnrm2(gradient);
            *f = fm;
            *step = stepm;
            *gnorm = gnorm1;
            if fabs(pg * lambda / gnorm1) < tol {
                return;
            }
            if stepm < stepb {
                stepc = stepb;
                fc = fb;
                stepb = stepm;
                fb = fm;
            } else {
                stepa = stepb;
                fa = fb;
                stepb = stepm;
                fb = fm;
            }
        }
    };
}
unsafe extern "C" fn conjugate_pr_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut conjugate_pr_state_t = vstate as *mut conjugate_pr_state_t;
    (*state).x1 = gsl_vector_calloc(n);
    if ((*state).x1).is_null() {
        gsl_error(
            b"failed to allocate space for x1\0" as *const u8 as *const libc::c_char,
            b"conjugate_pr.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).dx1 = gsl_vector_calloc(n);
    if ((*state).dx1).is_null() {
        gsl_vector_free((*state).x1);
        gsl_error(
            b"failed to allocate space for dx1\0" as *const u8 as *const libc::c_char,
            b"conjugate_pr.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x2 = gsl_vector_calloc(n);
    if ((*state).x2).is_null() {
        gsl_vector_free((*state).dx1);
        gsl_vector_free((*state).x1);
        gsl_error(
            b"failed to allocate space for x2\0" as *const u8 as *const libc::c_char,
            b"conjugate_pr.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).p = gsl_vector_calloc(n);
    if ((*state).p).is_null() {
        gsl_vector_free((*state).x2);
        gsl_vector_free((*state).dx1);
        gsl_vector_free((*state).x1);
        gsl_error(
            b"failed to allocate space for p\0" as *const u8 as *const libc::c_char,
            b"conjugate_pr.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).g0 = gsl_vector_calloc(n);
    if ((*state).g0).is_null() {
        gsl_vector_free((*state).p);
        gsl_vector_free((*state).x2);
        gsl_vector_free((*state).dx1);
        gsl_vector_free((*state).x1);
        gsl_error(
            b"failed to allocate space for g0\0" as *const u8 as *const libc::c_char,
            b"conjugate_pr.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn conjugate_pr_set(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *const gsl_vector,
    mut f: *mut libc::c_double,
    mut gradient: *mut gsl_vector,
    mut step_size: libc::c_double,
    mut tol: libc::c_double,
) -> libc::c_int {
    let mut state: *mut conjugate_pr_state_t = vstate as *mut conjugate_pr_state_t;
    (*state).iter = 0 as libc::c_int;
    (*state).step = step_size;
    (*state).max_step = step_size;
    (*state).tol = tol;
    (Some(((*fdf).fdf).unwrap())).unwrap()(x, (*fdf).params, f, gradient);
    gsl_vector_memcpy((*state).p, gradient);
    gsl_vector_memcpy((*state).g0, gradient);
    let mut gnorm: libc::c_double = gsl_blas_dnrm2(gradient);
    (*state).pnorm = gnorm;
    (*state).g0norm = gnorm;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn conjugate_pr_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut conjugate_pr_state_t = vstate as *mut conjugate_pr_state_t;
    gsl_vector_free((*state).g0);
    gsl_vector_free((*state).p);
    gsl_vector_free((*state).x2);
    gsl_vector_free((*state).dx1);
    gsl_vector_free((*state).x1);
}
unsafe extern "C" fn conjugate_pr_restart(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut conjugate_pr_state_t = vstate as *mut conjugate_pr_state_t;
    (*state).iter = 0 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn conjugate_pr_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut libc::c_double,
    mut gradient: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut conjugate_pr_state_t = vstate as *mut conjugate_pr_state_t;
    let mut x1: *mut gsl_vector = (*state).x1;
    let mut dx1: *mut gsl_vector = (*state).dx1;
    let mut x2: *mut gsl_vector = (*state).x2;
    let mut p: *mut gsl_vector = (*state).p;
    let mut g0: *mut gsl_vector = (*state).g0;
    let mut pnorm: libc::c_double = (*state).pnorm;
    let mut g0norm: libc::c_double = (*state).g0norm;
    let mut fa: libc::c_double = *f;
    let mut fb: libc::c_double = 0.;
    let mut fc: libc::c_double = 0.;
    let mut dir: libc::c_double = 0.;
    let mut stepa: libc::c_double = 0.0f64;
    let mut stepb: libc::c_double = 0.;
    let mut stepc: libc::c_double = (*state).step;
    let mut tol: libc::c_double = (*state).tol;
    let mut g1norm: libc::c_double = 0.;
    let mut pg: libc::c_double = 0.;
    if pnorm == 0.0f64 || g0norm == 0.0f64 {
        gsl_vector_set_zero(dx);
        return GSL_ENOPROG as libc::c_int;
    }
    gsl_blas_ddot(p, gradient, &mut pg);
    dir = if pg >= 0.0f64 { 1.0f64 } else { -1.0f64 };
    take_step(x, p, stepc, dir / pnorm, x1, dx);
    fc = (Some(((*fdf).f).unwrap())).unwrap()(x1, (*fdf).params);
    if fc < fa {
        (*state).step = stepc * 2.0f64;
        *f = fc;
        gsl_vector_memcpy(x, x1);
        (Some(((*fdf).df).unwrap())).unwrap()(x1, (*fdf).params, gradient);
        return GSL_SUCCESS as libc::c_int;
    }
    intermediate_point(
        fdf,
        x,
        p,
        dir / pnorm,
        pg,
        stepa,
        stepc,
        fa,
        fc,
        x1,
        dx1,
        gradient,
        &mut stepb,
        &mut fb,
    );
    if stepb == 0.0f64 {
        return GSL_ENOPROG as libc::c_int;
    }
    minimize(
        fdf,
        x,
        p,
        dir / pnorm,
        stepa,
        stepb,
        stepc,
        fa,
        fb,
        fc,
        tol,
        x1,
        dx1,
        x2,
        dx,
        gradient,
        &mut (*state).step,
        f,
        &mut g1norm,
    );
    gsl_vector_memcpy(x, x2);
    (*state)
        .iter = (((*state).iter + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_rem((*x).size) as libc::c_int;
    if (*state).iter == 0 as libc::c_int {
        gsl_vector_memcpy(p, gradient);
        (*state).pnorm = g1norm;
    } else {
        let mut g0g1: libc::c_double = 0.;
        let mut beta: libc::c_double = 0.;
        gsl_blas_daxpy(-1.0f64, gradient, g0);
        gsl_blas_ddot(g0, gradient, &mut g0g1);
        beta = g0g1 / (g0norm * g0norm);
        gsl_blas_dscal(-beta, p);
        gsl_blas_daxpy(1.0f64, gradient, p);
        (*state).pnorm = gsl_blas_dnrm2(p);
    }
    (*state).g0norm = g1norm;
    gsl_vector_memcpy(g0, gradient);
    return GSL_SUCCESS as libc::c_int;
}
static mut conjugate_pr_type: gsl_multimin_fdfminimizer_type = {
    let mut init = gsl_multimin_fdfminimizer_type {
        name: b"conjugate_pr\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<conjugate_pr_state_t>() as libc::c_ulong,
        alloc: Some(
            conjugate_pr_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            conjugate_pr_set
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multimin_function_fdf,
                    *const gsl_vector,
                    *mut libc::c_double,
                    *mut gsl_vector,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_int,
        ),
        iterate: Some(
            conjugate_pr_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_multimin_function_fdf,
                    *mut gsl_vector,
                    *mut libc::c_double,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        restart: Some(
            conjugate_pr_restart
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        ),
        free: Some(conjugate_pr_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multimin_fdfminimizer_conjugate_pr: *const gsl_multimin_fdfminimizer_type = unsafe {
    &conjugate_pr_type as *const gsl_multimin_fdfminimizer_type
};
