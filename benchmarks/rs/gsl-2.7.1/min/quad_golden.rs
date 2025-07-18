use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
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
pub struct gsl_min_fminimizer_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quad_golden_state_t {
    pub step_size: libc::c_double,
    pub stored_step: libc::c_double,
    pub prev_stored_step: libc::c_double,
    pub x_prev_small: libc::c_double,
    pub f_prev_small: libc::c_double,
    pub x_small: libc::c_double,
    pub f_small: libc::c_double,
    pub num_iter: libc::c_uint,
}
unsafe extern "C" fn quad_golden_init(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut x_minimum: libc::c_double,
    mut f_minimum: libc::c_double,
    mut x_lower: libc::c_double,
    mut f_lower: libc::c_double,
    mut x_upper: libc::c_double,
    mut f_upper: libc::c_double,
) -> libc::c_int {
    let mut state: *mut quad_golden_state_t = vstate as *mut quad_golden_state_t;
    (*state).x_prev_small = x_minimum;
    (*state).x_small = x_minimum;
    (*state).f_prev_small = f_minimum;
    (*state).f_small = f_minimum;
    (*state).step_size = 0.0f64;
    (*state).stored_step = 0.0f64;
    (*state).prev_stored_step = 0.0f64;
    (*state).num_iter = 0 as libc::c_int as libc::c_uint;
    x_lower = 0 as libc::c_int as libc::c_double;
    x_upper = 0 as libc::c_int as libc::c_double;
    f_lower = 0 as libc::c_int as libc::c_double;
    f_upper = 0 as libc::c_int as libc::c_double;
    f = 0 as *mut gsl_function;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn quad_golden_iterate(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut x_minimum: *mut libc::c_double,
    mut f_minimum: *mut libc::c_double,
    mut x_lower: *mut libc::c_double,
    mut f_lower: *mut libc::c_double,
    mut x_upper: *mut libc::c_double,
    mut f_upper: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut quad_golden_state_t = vstate as *mut quad_golden_state_t;
    let x_m: libc::c_double = *x_minimum;
    let f_m: libc::c_double = *f_minimum;
    let x_l: libc::c_double = *x_lower;
    let x_u: libc::c_double = *x_upper;
    let x_small: libc::c_double = (*state).x_small;
    let f_small: libc::c_double = (*state).f_small;
    let x_prev_small: libc::c_double = (*state).x_prev_small;
    let f_prev_small: libc::c_double = (*state).f_prev_small;
    let mut stored_step: libc::c_double = (*state).stored_step;
    let mut prev_stored_step: libc::c_double = (*state).prev_stored_step;
    let mut step_size: libc::c_double = (*state).step_size;
    let mut quad_step_size: libc::c_double = prev_stored_step;
    let mut x_trial: libc::c_double = 0.;
    let mut x_eval: libc::c_double = 0.;
    let mut f_eval: libc::c_double = 0.;
    let mut x_midpoint: libc::c_double = 0.5f64 * (x_l + x_u);
    let mut tol: libc::c_double = 1.0e-06f64 * fabs(x_m) + 1.0e-10f64;
    if fabs(stored_step) - tol > -2.0f64 * 2.2204460492503131e-16f64 {
        let mut c3: libc::c_double = (x_m - x_small) * (f_m - f_prev_small);
        let mut c2: libc::c_double = (x_m - x_prev_small) * (f_m - f_small);
        let mut c1: libc::c_double = (x_m - x_prev_small) * c2 - (x_m - x_small) * c3;
        c2 = 2.0f64 * (c2 - c3);
        if fabs(c2) > 2.2204460492503131e-16f64 {
            if c2 > 0.0f64 {
                c1 = -c1;
            }
            c2 = fabs(c2);
            quad_step_size = c1 / c2;
        } else {
            quad_step_size = stored_step;
        }
        prev_stored_step = stored_step;
        stored_step = step_size;
    }
    x_trial = x_m + quad_step_size;
    if fabs(quad_step_size) < fabs(0.5f64 * prev_stored_step) && x_trial > x_l
        && x_trial < x_u
    {
        step_size = quad_step_size;
        if x_trial - x_l < 2.0f64 * tol || x_u - x_trial < 2.0f64 * tol {
            step_size = (if x_midpoint >= x_m { 1.0f64 } else { -1.0f64 }) * fabs(tol);
        }
    } else if x_small != x_prev_small && x_small < x_m && x_prev_small < x_m
        || x_small != x_prev_small && x_small > x_m && x_prev_small > x_m
    {
        let mut outside_interval: libc::c_double = 0.;
        let mut inside_interval: libc::c_double = 0.;
        if x_small < x_m {
            outside_interval = x_l - x_m;
            inside_interval = x_u - x_m;
        } else {
            outside_interval = x_u - x_m;
            inside_interval = x_l - x_m;
        }
        if fabs(inside_interval) <= tol {
            let mut tmp: libc::c_double = outside_interval;
            outside_interval = inside_interval;
            inside_interval = tmp;
        }
        let mut step: libc::c_double = inside_interval;
        let mut scale_factor: libc::c_double = 0.;
        if fabs(outside_interval) < fabs(inside_interval) {
            scale_factor = 0.5f64 * sqrt(-outside_interval / inside_interval);
        } else {
            scale_factor = 5.0f64 / 11.0f64
                * (0.1f64 - inside_interval / outside_interval);
        }
        (*state).stored_step = step;
        step_size = scale_factor * step;
    } else {
        let mut step_0: libc::c_double = 0.;
        if x_m < x_midpoint {
            step_0 = x_u - x_m;
        } else {
            step_0 = x_l - x_m;
        }
        (*state).stored_step = step_0;
        step_size = 0.3819660112501052f64 * step_0;
    }
    if fabs(step_size) > tol {
        x_eval = x_m + step_size;
    } else {
        x_eval = x_m
            + (if step_size >= 0 as libc::c_int as libc::c_double {
                1.0f64
            } else {
                -1.0f64
            }) * fabs(tol);
    }
    f_eval = (Some(((*f).function).unwrap())).unwrap()(x_eval, (*f).params);
    if gsl_finite(f_eval) == 0 {
        gsl_error(
            b"computed function value is infinite or NaN\0" as *const u8
                as *const libc::c_char,
            b"quad_golden.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    if f_eval <= f_m {
        if x_eval < x_m {
            *x_upper = x_m;
            *f_upper = f_m;
        } else {
            *x_lower = x_m;
            *f_upper = f_m;
        }
        (*state).x_prev_small = x_small;
        (*state).f_prev_small = f_small;
        (*state).x_small = x_m;
        (*state).f_small = f_m;
        *x_minimum = x_eval;
        *f_minimum = f_eval;
    } else {
        if x_eval < x_m {
            *x_lower = x_eval;
            *f_lower = f_eval;
        } else {
            *x_upper = x_eval;
            *f_upper = f_eval;
        }
        if f_eval <= f_small || fabs(x_small - x_m) < 2.0f64 * 2.2204460492503131e-16f64
        {
            (*state).x_prev_small = x_small;
            (*state).f_prev_small = f_small;
            (*state).x_small = x_eval;
            (*state).f_small = f_eval;
        } else if f_eval <= f_prev_small
            || fabs(x_prev_small - x_m) < 2.0f64 * 2.2204460492503131e-16f64
            || fabs(x_prev_small - x_small) < 2.0f64 * 2.2204460492503131e-16f64
        {
            (*state).x_prev_small = x_eval;
            (*state).f_prev_small = f_eval;
        }
    }
    (*state).stored_step = stored_step;
    (*state).prev_stored_step = prev_stored_step;
    (*state).step_size = step_size;
    (*state).num_iter = ((*state).num_iter).wrapping_add(1);
    (*state).num_iter;
    return GSL_SUCCESS as libc::c_int;
}
static mut quad_golden_type: gsl_min_fminimizer_type = {
    let mut init = gsl_min_fminimizer_type {
        name: b"quad-golden\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<quad_golden_state_t>() as libc::c_ulong,
        set: Some(
            quad_golden_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_int,
        ),
        iterate: Some(
            quad_golden_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
    };
    init
};
pub static mut gsl_min_fminimizer_quad_golden: *const gsl_min_fminimizer_type = unsafe {
    &quad_golden_type as *const gsl_min_fminimizer_type
};
