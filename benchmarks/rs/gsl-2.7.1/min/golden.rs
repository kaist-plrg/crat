use ::libc;
extern "C" {
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
pub struct goldensection_state_t {
    pub dummy: libc::c_double,
}
unsafe extern "C" fn goldensection_init(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut x_minimum: libc::c_double,
    mut f_minimum: libc::c_double,
    mut x_lower: libc::c_double,
    mut f_lower: libc::c_double,
    mut x_upper: libc::c_double,
    mut f_upper: libc::c_double,
) -> libc::c_int {
    let mut state: *mut goldensection_state_t = vstate as *mut goldensection_state_t;
    state = 0 as *mut goldensection_state_t;
    f = 0 as *mut gsl_function;
    x_minimum = 0 as libc::c_int as libc::c_double;
    f_minimum = 0 as libc::c_int as libc::c_double;
    x_lower = 0 as libc::c_int as libc::c_double;
    f_lower = 0 as libc::c_int as libc::c_double;
    x_upper = 0 as libc::c_int as libc::c_double;
    f_upper = 0 as libc::c_int as libc::c_double;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn goldensection_iterate(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut x_minimum: *mut libc::c_double,
    mut f_minimum: *mut libc::c_double,
    mut x_lower: *mut libc::c_double,
    mut f_lower: *mut libc::c_double,
    mut x_upper: *mut libc::c_double,
    mut f_upper: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut goldensection_state_t = vstate as *mut goldensection_state_t;
    let x_center: libc::c_double = *x_minimum;
    let x_left: libc::c_double = *x_lower;
    let x_right: libc::c_double = *x_upper;
    let f_min: libc::c_double = *f_minimum;
    let golden: libc::c_double = 0.3819660f64;
    let w_lower: libc::c_double = x_center - x_left;
    let w_upper: libc::c_double = x_right - x_center;
    let mut x_new: libc::c_double = 0.;
    let mut f_new: libc::c_double = 0.;
    state = 0 as *mut goldensection_state_t;
    x_new = x_center + golden * (if w_upper > w_lower { w_upper } else { -w_lower });
    f_new = (Some(((*f).function).unwrap())).unwrap()(x_new, (*f).params);
    if gsl_finite(f_new) == 0 {
        gsl_error(
            b"computed function value is infinite or NaN\0" as *const u8
                as *const libc::c_char,
            b"golden.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    if f_new < f_min {
        *x_minimum = x_new;
        *f_minimum = f_new;
        return GSL_SUCCESS as libc::c_int;
    } else if x_new < x_center && f_new > f_min {
        *x_lower = x_new;
        *f_lower = f_new;
        return GSL_SUCCESS as libc::c_int;
    } else if x_new > x_center && f_new > f_min {
        *x_upper = x_new;
        *f_upper = f_new;
        return GSL_SUCCESS as libc::c_int;
    } else {
        return GSL_FAILURE as libc::c_int
    };
}
static mut goldensection_type: gsl_min_fminimizer_type = {
    let mut init = gsl_min_fminimizer_type {
        name: b"goldensection\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<goldensection_state_t>() as libc::c_ulong,
        set: Some(
            goldensection_init
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
            goldensection_iterate
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
pub static mut gsl_min_fminimizer_goldensection: *const gsl_min_fminimizer_type = unsafe {
    &goldensection_type as *const gsl_min_fminimizer_type
};
