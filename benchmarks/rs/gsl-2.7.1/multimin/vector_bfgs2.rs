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
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
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
    fn gsl_poly_solve_quadratic(
        a: libc::c_double,
        b: libc::c_double,
        c: libc::c_double,
        x0: *mut libc::c_double,
        x1: *mut libc::c_double,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub df: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *mut libc::c_void,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function_fdf = gsl_function_fdf_struct;
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
pub struct vector_bfgs2_state_t {
    pub iter: libc::c_int,
    pub step: libc::c_double,
    pub g0norm: libc::c_double,
    pub pnorm: libc::c_double,
    pub delta_f: libc::c_double,
    pub fp0: libc::c_double,
    pub x0: *mut gsl_vector,
    pub g0: *mut gsl_vector,
    pub p: *mut gsl_vector,
    pub dx0: *mut gsl_vector,
    pub dg0: *mut gsl_vector,
    pub x_alpha: *mut gsl_vector,
    pub g_alpha: *mut gsl_vector,
    pub wrap: wrapper_t,
    pub rho: libc::c_double,
    pub sigma: libc::c_double,
    pub tau1: libc::c_double,
    pub tau2: libc::c_double,
    pub tau3: libc::c_double,
    pub order: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrapper_t {
    pub fdf_linear: gsl_function_fdf,
    pub fdf: *mut gsl_multimin_function_fdf,
    pub x: *const gsl_vector,
    pub g: *const gsl_vector,
    pub p: *const gsl_vector,
    pub f_alpha: libc::c_double,
    pub df_alpha: libc::c_double,
    pub x_alpha: *mut gsl_vector,
    pub g_alpha: *mut gsl_vector,
    pub f_cache_key: libc::c_double,
    pub df_cache_key: libc::c_double,
    pub x_cache_key: libc::c_double,
    pub g_cache_key: libc::c_double,
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
unsafe extern "C" fn minimize(
    mut fn_0: *mut gsl_function_fdf,
    mut rho: libc::c_double,
    mut sigma: libc::c_double,
    mut tau1: libc::c_double,
    mut tau2: libc::c_double,
    mut tau3: libc::c_double,
    mut order: libc::c_int,
    mut alpha1: libc::c_double,
    mut alpha_new: *mut libc::c_double,
) -> libc::c_int {
    let mut f0: libc::c_double = 0.;
    let mut fp0: libc::c_double = 0.;
    let mut falpha: libc::c_double = 0.;
    let mut falpha_prev: libc::c_double = 0.;
    let mut fpalpha: libc::c_double = 0.;
    let mut fpalpha_prev: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut alpha_next: libc::c_double = 0.;
    let mut alpha: libc::c_double = alpha1;
    let mut alpha_prev: libc::c_double = 0.0f64;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut fa: libc::c_double = 0.;
    let mut fb: libc::c_double = 0.;
    let mut fpa: libc::c_double = 0.;
    let mut fpb: libc::c_double = 0.;
    let bracket_iters: size_t = 100 as libc::c_int as size_t;
    let section_iters: size_t = 100 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    (Some(((*fn_0).fdf).unwrap())).unwrap()(0.0f64, (*fn_0).params, &mut f0, &mut fp0);
    falpha_prev = f0;
    fpalpha_prev = fp0;
    a = 0.0f64;
    b = alpha;
    fa = f0;
    fb = 0.0f64;
    fpa = fp0;
    fpb = 0.0f64;
    loop {
        let fresh0 = i;
        i = i.wrapping_add(1);
        if !(fresh0 < bracket_iters) {
            break;
        }
        falpha = (Some(((*fn_0).f).unwrap())).unwrap()(alpha, (*fn_0).params);
        if falpha > f0 + alpha * rho * fp0 || falpha >= falpha_prev {
            a = alpha_prev;
            fa = falpha_prev;
            fpa = fpalpha_prev;
            b = alpha;
            fb = falpha;
            fpb = ::std::f32::NAN as libc::c_double;
            break;
        } else {
            fpalpha = (Some(((*fn_0).df).unwrap())).unwrap()(alpha, (*fn_0).params);
            if fabs(fpalpha) <= -sigma * fp0 {
                *alpha_new = alpha;
                return GSL_SUCCESS as libc::c_int;
            }
            if fpalpha >= 0 as libc::c_int as libc::c_double {
                a = alpha;
                fa = falpha;
                fpa = fpalpha;
                b = alpha_prev;
                fb = falpha_prev;
                fpb = fpalpha_prev;
                break;
            } else {
                delta = alpha - alpha_prev;
                let mut lower: libc::c_double = alpha + delta;
                let mut upper: libc::c_double = alpha + tau1 * delta;
                alpha_next = interpolate(
                    alpha_prev,
                    falpha_prev,
                    fpalpha_prev,
                    alpha,
                    falpha,
                    fpalpha,
                    lower,
                    upper,
                    order,
                );
                alpha_prev = alpha;
                falpha_prev = falpha;
                fpalpha_prev = fpalpha;
                alpha = alpha_next;
            }
        }
    }
    loop {
        let fresh1 = i;
        i = i.wrapping_add(1);
        if !(fresh1 < section_iters) {
            break;
        }
        delta = b - a;
        let mut lower_0: libc::c_double = a + tau2 * delta;
        let mut upper_0: libc::c_double = b - tau3 * delta;
        alpha = interpolate(a, fa, fpa, b, fb, fpb, lower_0, upper_0, order);
        falpha = (Some(((*fn_0).f).unwrap())).unwrap()(alpha, (*fn_0).params);
        if (a - alpha) * fpa <= 2.2204460492503131e-16f64 {
            return GSL_ENOPROG as libc::c_int;
        }
        if falpha > f0 + rho * alpha * fp0 || falpha >= fa {
            b = alpha;
            fb = falpha;
            fpb = ::std::f32::NAN as libc::c_double;
        } else {
            fpalpha = (Some(((*fn_0).df).unwrap())).unwrap()(alpha, (*fn_0).params);
            if fabs(fpalpha) <= -sigma * fp0 {
                *alpha_new = alpha;
                return GSL_SUCCESS as libc::c_int;
            }
            if b - a >= 0 as libc::c_int as libc::c_double
                && fpalpha >= 0 as libc::c_int as libc::c_double
                || b - a <= 0 as libc::c_int as libc::c_double
                    && fpalpha <= 0 as libc::c_int as libc::c_double
            {
                b = a;
                fb = fa;
                fpb = fpa;
                a = alpha;
                fa = falpha;
                fpa = fpalpha;
            } else {
                a = alpha;
                fa = falpha;
                fpa = fpalpha;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn cubic(
    mut c0: libc::c_double,
    mut c1: libc::c_double,
    mut c2: libc::c_double,
    mut c3: libc::c_double,
    mut z: libc::c_double,
) -> libc::c_double {
    return c0 + z * (c1 + z * (c2 + z * c3));
}
unsafe extern "C" fn interp_cubic(
    mut f0: libc::c_double,
    mut fp0: libc::c_double,
    mut f1: libc::c_double,
    mut fp1: libc::c_double,
    mut zl: libc::c_double,
    mut zh: libc::c_double,
) -> libc::c_double {
    let mut eta: libc::c_double = 3 as libc::c_int as libc::c_double * (f1 - f0)
        - 2 as libc::c_int as libc::c_double * fp0 - fp1;
    let mut xi: libc::c_double = fp0 + fp1
        - 2 as libc::c_int as libc::c_double * (f1 - f0);
    let mut c0: libc::c_double = f0;
    let mut c1: libc::c_double = fp0;
    let mut c2: libc::c_double = eta;
    let mut c3: libc::c_double = xi;
    let mut zmin: libc::c_double = 0.;
    let mut fmin: libc::c_double = 0.;
    let mut z0: libc::c_double = 0.;
    let mut z1: libc::c_double = 0.;
    zmin = zl;
    fmin = cubic(c0, c1, c2, c3, zl);
    check_extremum(c0, c1, c2, c3, zh, &mut zmin, &mut fmin);
    let mut n: libc::c_int = gsl_poly_solve_quadratic(
        3 as libc::c_int as libc::c_double * c3,
        2 as libc::c_int as libc::c_double * c2,
        c1,
        &mut z0,
        &mut z1,
    );
    if n == 2 as libc::c_int {
        if z0 > zl && z0 < zh {
            check_extremum(c0, c1, c2, c3, z0, &mut zmin, &mut fmin);
        }
        if z1 > zl && z1 < zh {
            check_extremum(c0, c1, c2, c3, z1, &mut zmin, &mut fmin);
        }
    } else if n == 1 as libc::c_int {
        if z0 > zl && z0 < zh {
            check_extremum(c0, c1, c2, c3, z0, &mut zmin, &mut fmin);
        }
    }
    return zmin;
}
unsafe extern "C" fn interp_quad(
    mut f0: libc::c_double,
    mut fp0: libc::c_double,
    mut f1: libc::c_double,
    mut zl: libc::c_double,
    mut zh: libc::c_double,
) -> libc::c_double {
    let mut fl: libc::c_double = f0 + zl * (fp0 + zl * (f1 - f0 - fp0));
    let mut fh: libc::c_double = f0 + zh * (fp0 + zh * (f1 - f0 - fp0));
    let mut c: libc::c_double = 2 as libc::c_int as libc::c_double * (f1 - f0 - fp0);
    let mut zmin: libc::c_double = zl;
    let mut fmin: libc::c_double = fl;
    if fh < fmin {
        zmin = zh;
        fmin = fh;
    }
    if c > 0 as libc::c_int as libc::c_double {
        let mut z: libc::c_double = -fp0 / c;
        if z > zl && z < zh {
            let mut f: libc::c_double = f0 + z * (fp0 + z * (f1 - f0 - fp0));
            if f < fmin {
                zmin = z;
                fmin = f;
            }
        }
    }
    return zmin;
}
unsafe extern "C" fn interpolate(
    mut a: libc::c_double,
    mut fa: libc::c_double,
    mut fpa: libc::c_double,
    mut b: libc::c_double,
    mut fb: libc::c_double,
    mut fpb: libc::c_double,
    mut xmin: libc::c_double,
    mut xmax: libc::c_double,
    mut order: libc::c_int,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut zmin: libc::c_double = 0.;
    let mut zmax: libc::c_double = 0.;
    zmin = (xmin - a) / (b - a);
    zmax = (xmax - a) / (b - a);
    if zmin > zmax {
        let mut tmp: libc::c_double = zmin;
        zmin = zmax;
        zmax = tmp;
    }
    if order > 2 as libc::c_int && gsl_finite(fpb) != 0 {
        z = interp_cubic(fa, fpa * (b - a), fb, fpb * (b - a), zmin, zmax);
    } else {
        z = interp_quad(fa, fpa * (b - a), fb, zmin, zmax);
    }
    alpha = a + z * (b - a);
    return alpha;
}
unsafe extern "C" fn check_extremum(
    mut c0: libc::c_double,
    mut c1: libc::c_double,
    mut c2: libc::c_double,
    mut c3: libc::c_double,
    mut z: libc::c_double,
    mut zmin: *mut libc::c_double,
    mut fmin: *mut libc::c_double,
) {
    let mut y: libc::c_double = cubic(c0, c1, c2, c3, z);
    if y < *fmin {
        *zmin = z;
        *fmin = y;
    }
}
unsafe extern "C" fn moveto(mut alpha: libc::c_double, mut w: *mut wrapper_t) {
    if alpha == (*w).x_cache_key {
        return;
    }
    gsl_vector_memcpy((*w).x_alpha, (*w).x);
    gsl_blas_daxpy(alpha, (*w).p, (*w).x_alpha);
    (*w).x_cache_key = alpha;
}
unsafe extern "C" fn slope(mut w: *mut wrapper_t) -> libc::c_double {
    let mut df: libc::c_double = 0.;
    gsl_blas_ddot((*w).g_alpha, (*w).p, &mut df);
    return df;
}
unsafe extern "C" fn wrap_f(
    mut alpha: libc::c_double,
    mut params: *mut libc::c_void,
) -> libc::c_double {
    let mut w: *mut wrapper_t = params as *mut wrapper_t;
    if alpha == (*w).f_cache_key {
        return (*w).f_alpha;
    }
    moveto(alpha, w);
    (*w)
        .f_alpha = (Some(((*(*w).fdf).f).unwrap()))
        .unwrap()((*w).x_alpha, (*(*w).fdf).params);
    (*w).f_cache_key = alpha;
    return (*w).f_alpha;
}
unsafe extern "C" fn wrap_df(
    mut alpha: libc::c_double,
    mut params: *mut libc::c_void,
) -> libc::c_double {
    let mut w: *mut wrapper_t = params as *mut wrapper_t;
    if alpha == (*w).df_cache_key {
        return (*w).df_alpha;
    }
    moveto(alpha, w);
    if alpha != (*w).g_cache_key {
        (Some(((*(*w).fdf).df).unwrap()))
            .unwrap()((*w).x_alpha, (*(*w).fdf).params, (*w).g_alpha);
        (*w).g_cache_key = alpha;
    }
    (*w).df_alpha = slope(w);
    (*w).df_cache_key = alpha;
    return (*w).df_alpha;
}
unsafe extern "C" fn wrap_fdf(
    mut alpha: libc::c_double,
    mut params: *mut libc::c_void,
    mut f: *mut libc::c_double,
    mut df: *mut libc::c_double,
) {
    let mut w: *mut wrapper_t = params as *mut wrapper_t;
    if alpha == (*w).f_cache_key && alpha == (*w).df_cache_key {
        *f = (*w).f_alpha;
        *df = (*w).df_alpha;
        return;
    }
    if alpha == (*w).f_cache_key || alpha == (*w).df_cache_key {
        *f = wrap_f(alpha, params);
        *df = wrap_df(alpha, params);
        return;
    }
    moveto(alpha, w);
    (Some(((*(*w).fdf).fdf).unwrap()))
        .unwrap()((*w).x_alpha, (*(*w).fdf).params, &mut (*w).f_alpha, (*w).g_alpha);
    (*w).f_cache_key = alpha;
    (*w).g_cache_key = alpha;
    (*w).df_alpha = slope(w);
    (*w).df_cache_key = alpha;
    *f = (*w).f_alpha;
    *df = (*w).df_alpha;
}
unsafe extern "C" fn prepare_wrapper(
    mut w: *mut wrapper_t,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *const gsl_vector,
    mut f: libc::c_double,
    mut g: *const gsl_vector,
    mut p: *const gsl_vector,
    mut x_alpha: *mut gsl_vector,
    mut g_alpha: *mut gsl_vector,
) {
    (*w)
        .fdf_linear
        .f = Some(
        wrap_f
            as unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    );
    (*w)
        .fdf_linear
        .df = Some(
        wrap_df
            as unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    );
    (*w)
        .fdf_linear
        .fdf = Some(
        wrap_fdf
            as unsafe extern "C" fn(
                libc::c_double,
                *mut libc::c_void,
                *mut libc::c_double,
                *mut libc::c_double,
            ) -> (),
    );
    (*w).fdf_linear.params = w as *mut libc::c_void;
    (*w).fdf = fdf;
    (*w).x = x;
    (*w).g = g;
    (*w).p = p;
    (*w).x_alpha = x_alpha;
    (*w).g_alpha = g_alpha;
    gsl_vector_memcpy((*w).x_alpha, (*w).x);
    (*w).x_cache_key = 0.0f64;
    (*w).f_alpha = f;
    (*w).f_cache_key = 0.0f64;
    gsl_vector_memcpy((*w).g_alpha, (*w).g);
    (*w).g_cache_key = 0.0f64;
    (*w).df_alpha = slope(w);
    (*w).df_cache_key = 0.0f64;
}
unsafe extern "C" fn update_position(
    mut w: *mut wrapper_t,
    mut alpha: libc::c_double,
    mut x: *mut gsl_vector,
    mut f: *mut libc::c_double,
    mut g: *mut gsl_vector,
) {
    let mut f_alpha: libc::c_double = 0.;
    let mut df_alpha: libc::c_double = 0.;
    wrap_fdf(alpha, w as *mut libc::c_void, &mut f_alpha, &mut df_alpha);
    *f = (*w).f_alpha;
    gsl_vector_memcpy(x, (*w).x_alpha);
    gsl_vector_memcpy(g, (*w).g_alpha);
}
unsafe extern "C" fn change_direction(mut w: *mut wrapper_t) {
    gsl_vector_memcpy((*w).x_alpha, (*w).x);
    (*w).x_cache_key = 0.0f64;
    (*w).f_cache_key = 0.0f64;
    gsl_vector_memcpy((*w).g_alpha, (*w).g);
    (*w).g_cache_key = 0.0f64;
    (*w).df_alpha = slope(w);
    (*w).df_cache_key = 0.0f64;
}
unsafe extern "C" fn vector_bfgs2_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut state: *mut vector_bfgs2_state_t = vstate as *mut vector_bfgs2_state_t;
    (*state).p = gsl_vector_calloc(n);
    if ((*state).p).is_null() {
        gsl_error(
            b"failed to allocate space for p\0" as *const u8 as *const libc::c_char,
            b"vector_bfgs2.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x0 = gsl_vector_calloc(n);
    if ((*state).x0).is_null() {
        gsl_vector_free((*state).p);
        gsl_error(
            b"failed to allocate space for g0\0" as *const u8 as *const libc::c_char,
            b"vector_bfgs2.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).g0 = gsl_vector_calloc(n);
    if ((*state).g0).is_null() {
        gsl_vector_free((*state).x0);
        gsl_vector_free((*state).p);
        gsl_error(
            b"failed to allocate space for g0\0" as *const u8 as *const libc::c_char,
            b"vector_bfgs2.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).dx0 = gsl_vector_calloc(n);
    if ((*state).dx0).is_null() {
        gsl_vector_free((*state).g0);
        gsl_vector_free((*state).x0);
        gsl_vector_free((*state).p);
        gsl_error(
            b"failed to allocate space for g0\0" as *const u8 as *const libc::c_char,
            b"vector_bfgs2.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).dg0 = gsl_vector_calloc(n);
    if ((*state).dg0).is_null() {
        gsl_vector_free((*state).dx0);
        gsl_vector_free((*state).g0);
        gsl_vector_free((*state).x0);
        gsl_vector_free((*state).p);
        gsl_error(
            b"failed to allocate space for g0\0" as *const u8 as *const libc::c_char,
            b"vector_bfgs2.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x_alpha = gsl_vector_calloc(n);
    if ((*state).x_alpha).is_null() {
        gsl_vector_free((*state).dg0);
        gsl_vector_free((*state).dx0);
        gsl_vector_free((*state).g0);
        gsl_vector_free((*state).x0);
        gsl_vector_free((*state).p);
        gsl_error(
            b"failed to allocate space for g0\0" as *const u8 as *const libc::c_char,
            b"vector_bfgs2.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).g_alpha = gsl_vector_calloc(n);
    if ((*state).g_alpha).is_null() {
        gsl_vector_free((*state).x_alpha);
        gsl_vector_free((*state).dg0);
        gsl_vector_free((*state).dx0);
        gsl_vector_free((*state).g0);
        gsl_vector_free((*state).x0);
        gsl_vector_free((*state).p);
        gsl_error(
            b"failed to allocate space for g0\0" as *const u8 as *const libc::c_char,
            b"vector_bfgs2.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn vector_bfgs2_set(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *const gsl_vector,
    mut f: *mut libc::c_double,
    mut gradient: *mut gsl_vector,
    mut step_size: libc::c_double,
    mut tol: libc::c_double,
) -> libc::c_int {
    let mut state: *mut vector_bfgs2_state_t = vstate as *mut vector_bfgs2_state_t;
    (*state).iter = 0 as libc::c_int;
    (*state).step = step_size;
    (*state).delta_f = 0 as libc::c_int as libc::c_double;
    (Some(((*fdf).fdf).unwrap())).unwrap()(x, (*fdf).params, f, gradient);
    gsl_vector_memcpy((*state).x0, x);
    gsl_vector_memcpy((*state).g0, gradient);
    (*state).g0norm = gsl_blas_dnrm2((*state).g0);
    gsl_vector_memcpy((*state).p, gradient);
    gsl_blas_dscal(-(1 as libc::c_int) as libc::c_double / (*state).g0norm, (*state).p);
    (*state).pnorm = gsl_blas_dnrm2((*state).p);
    (*state).fp0 = -(*state).g0norm;
    prepare_wrapper(
        &mut (*state).wrap,
        fdf,
        (*state).x0,
        *f,
        (*state).g0,
        (*state).p,
        (*state).x_alpha,
        (*state).g_alpha,
    );
    (*state).rho = 0.01f64;
    (*state).sigma = tol;
    (*state).tau1 = 9 as libc::c_int as libc::c_double;
    (*state).tau2 = 0.05f64;
    (*state).tau3 = 0.5f64;
    (*state).order = 3 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn vector_bfgs2_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut vector_bfgs2_state_t = vstate as *mut vector_bfgs2_state_t;
    gsl_vector_free((*state).x_alpha);
    gsl_vector_free((*state).g_alpha);
    gsl_vector_free((*state).dg0);
    gsl_vector_free((*state).dx0);
    gsl_vector_free((*state).g0);
    gsl_vector_free((*state).x0);
    gsl_vector_free((*state).p);
}
unsafe extern "C" fn vector_bfgs2_restart(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut vector_bfgs2_state_t = vstate as *mut vector_bfgs2_state_t;
    (*state).iter = 0 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn vector_bfgs2_iterate(
    mut vstate: *mut libc::c_void,
    mut fdf: *mut gsl_multimin_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut libc::c_double,
    mut gradient: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut vector_bfgs2_state_t = vstate as *mut vector_bfgs2_state_t;
    let mut alpha: libc::c_double = 0.0f64;
    let mut alpha1: libc::c_double = 0.;
    let mut x0: *mut gsl_vector = (*state).x0;
    let mut g0: *mut gsl_vector = (*state).g0;
    let mut p: *mut gsl_vector = (*state).p;
    let mut g0norm: libc::c_double = (*state).g0norm;
    let mut pnorm: libc::c_double = (*state).pnorm;
    let mut delta_f: libc::c_double = (*state).delta_f;
    let mut pg: libc::c_double = 0.;
    let mut dir: libc::c_double = 0.;
    let mut status: libc::c_int = 0;
    let mut f0: libc::c_double = *f;
    if pnorm == 0.0f64 || g0norm == 0.0f64
        || (*state).fp0 == 0 as libc::c_int as libc::c_double
    {
        gsl_vector_set_zero(dx);
        return GSL_ENOPROG as libc::c_int;
    }
    if delta_f < 0 as libc::c_int as libc::c_double {
        let mut del: libc::c_double = GSL_MAX_DBL(
            -delta_f,
            10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64 * fabs(f0),
        );
        alpha1 = GSL_MIN_DBL(1.0f64, 2.0f64 * del / -(*state).fp0);
    } else {
        alpha1 = fabs((*state).step);
    }
    status = minimize(
        &mut (*state).wrap.fdf_linear,
        (*state).rho,
        (*state).sigma,
        (*state).tau1,
        (*state).tau2,
        (*state).tau3,
        (*state).order,
        alpha1,
        &mut alpha,
    );
    if status != GSL_SUCCESS as libc::c_int {
        return status;
    }
    update_position(&mut (*state).wrap, alpha, x, f, gradient);
    (*state).delta_f = *f - f0;
    let mut dx0: *mut gsl_vector = (*state).dx0;
    let mut dg0: *mut gsl_vector = (*state).dg0;
    let mut dxg: libc::c_double = 0.;
    let mut dgg: libc::c_double = 0.;
    let mut dxdg: libc::c_double = 0.;
    let mut dgnorm: libc::c_double = 0.;
    let mut A: libc::c_double = 0.;
    let mut B: libc::c_double = 0.;
    gsl_vector_memcpy(dx0, x);
    gsl_blas_daxpy(-1.0f64, x0, dx0);
    gsl_vector_memcpy(dx, dx0);
    gsl_vector_memcpy(dg0, gradient);
    gsl_blas_daxpy(-1.0f64, g0, dg0);
    gsl_blas_ddot(dx0, gradient, &mut dxg);
    gsl_blas_ddot(dg0, gradient, &mut dgg);
    gsl_blas_ddot(dx0, dg0, &mut dxdg);
    dgnorm = gsl_blas_dnrm2(dg0);
    if dxdg != 0 as libc::c_int as libc::c_double {
        B = dxg / dxdg;
        A = -(1.0f64 + dgnorm * dgnorm / dxdg) * B + dgg / dxdg;
    } else {
        B = 0 as libc::c_int as libc::c_double;
        A = 0 as libc::c_int as libc::c_double;
    }
    gsl_vector_memcpy(p, gradient);
    gsl_blas_daxpy(-A, dx0, p);
    gsl_blas_daxpy(-B, dg0, p);
    gsl_vector_memcpy(g0, gradient);
    gsl_vector_memcpy(x0, x);
    (*state).g0norm = gsl_blas_dnrm2(g0);
    (*state).pnorm = gsl_blas_dnrm2(p);
    gsl_blas_ddot(p, gradient, &mut pg);
    dir = if pg >= 0.0f64 { -1.0f64 } else { 1.0f64 };
    gsl_blas_dscal(dir / (*state).pnorm, p);
    (*state).pnorm = gsl_blas_dnrm2(p);
    gsl_blas_ddot(p, g0, &mut (*state).fp0);
    change_direction(&mut (*state).wrap);
    return GSL_SUCCESS as libc::c_int;
}
static mut vector_bfgs2_type: gsl_multimin_fdfminimizer_type = {
    let mut init = gsl_multimin_fdfminimizer_type {
        name: b"vector_bfgs2\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<vector_bfgs2_state_t>() as libc::c_ulong,
        alloc: Some(
            vector_bfgs2_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> libc::c_int,
        ),
        set: Some(
            vector_bfgs2_set
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
            vector_bfgs2_iterate
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
            vector_bfgs2_restart
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        ),
        free: Some(vector_bfgs2_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multimin_fdfminimizer_vector_bfgs2: *const gsl_multimin_fdfminimizer_type = unsafe {
    &vector_bfgs2_type as *const gsl_multimin_fdfminimizer_type
};
