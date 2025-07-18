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
pub struct gsl_root_fsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut gsl_function,
            *mut libc::c_double,
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
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bisection_state_t {
    pub f_lower: libc::c_double,
    pub f_upper: libc::c_double,
}
unsafe extern "C" fn bisection_init(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut root: *mut libc::c_double,
    mut x_lower: libc::c_double,
    mut x_upper: libc::c_double,
) -> libc::c_int {
    let mut state: *mut bisection_state_t = vstate as *mut bisection_state_t;
    let mut f_lower: libc::c_double = 0.;
    let mut f_upper: libc::c_double = 0.;
    *root = 0.5f64 * (x_lower + x_upper);
    f_lower = (Some(((*f).function).unwrap())).unwrap()(x_lower, (*f).params);
    if gsl_finite(f_lower) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const libc::c_char,
            b"bisection.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    f_upper = (Some(((*f).function).unwrap())).unwrap()(x_upper, (*f).params);
    if gsl_finite(f_upper) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const libc::c_char,
            b"bisection.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    (*state).f_lower = f_lower;
    (*state).f_upper = f_upper;
    if f_lower < 0.0f64 && f_upper < 0.0f64 || f_lower > 0.0f64 && f_upper > 0.0f64 {
        gsl_error(
            b"endpoints do not straddle y=0\0" as *const u8 as *const libc::c_char,
            b"bisection.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bisection_iterate(
    mut vstate: *mut libc::c_void,
    mut f: *mut gsl_function,
    mut root: *mut libc::c_double,
    mut x_lower: *mut libc::c_double,
    mut x_upper: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *mut bisection_state_t = vstate as *mut bisection_state_t;
    let mut x_bisect: libc::c_double = 0.;
    let mut f_bisect: libc::c_double = 0.;
    let x_left: libc::c_double = *x_lower;
    let x_right: libc::c_double = *x_upper;
    let f_lower: libc::c_double = (*state).f_lower;
    let f_upper: libc::c_double = (*state).f_upper;
    if f_lower == 0.0f64 {
        *root = x_left;
        *x_upper = x_left;
        return GSL_SUCCESS as libc::c_int;
    }
    if f_upper == 0.0f64 {
        *root = x_right;
        *x_lower = x_right;
        return GSL_SUCCESS as libc::c_int;
    }
    x_bisect = (x_left + x_right) / 2.0f64;
    f_bisect = (Some(((*f).function).unwrap())).unwrap()(x_bisect, (*f).params);
    if gsl_finite(f_bisect) == 0 {
        gsl_error(
            b"function value is not finite\0" as *const u8 as *const libc::c_char,
            b"bisection.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            GSL_EBADFUNC as libc::c_int,
        );
        return GSL_EBADFUNC as libc::c_int;
    }
    if f_bisect == 0.0f64 {
        *root = x_bisect;
        *x_lower = x_bisect;
        *x_upper = x_bisect;
        return GSL_SUCCESS as libc::c_int;
    }
    if f_lower > 0.0f64 && f_bisect < 0.0f64 || f_lower < 0.0f64 && f_bisect > 0.0f64 {
        *root = 0.5f64 * (x_left + x_bisect);
        *x_upper = x_bisect;
        (*state).f_upper = f_bisect;
    } else {
        *root = 0.5f64 * (x_bisect + x_right);
        *x_lower = x_bisect;
        (*state).f_lower = f_bisect;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut bisection_type: gsl_root_fsolver_type = {
    let mut init = gsl_root_fsolver_type {
        name: b"bisection\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<bisection_state_t>() as libc::c_ulong,
        set: Some(
            bisection_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function,
                    *mut libc::c_double,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_int,
        ),
        iterate: Some(
            bisection_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_function,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
    };
    init
};
pub static mut gsl_root_fsolver_bisection: *const gsl_root_fsolver_type = unsafe {
    &bisection_type as *const gsl_root_fsolver_type
};
