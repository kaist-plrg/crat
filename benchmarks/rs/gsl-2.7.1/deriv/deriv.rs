use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
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
unsafe extern "C" fn central_deriv(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut h: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr_round: *mut libc::c_double,
    mut abserr_trunc: *mut libc::c_double,
) {
    let mut fm1: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x - h, (*f).params);
    let mut fp1: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x + h, (*f).params);
    let mut fmh: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x - h / 2 as libc::c_int as libc::c_double, (*f).params);
    let mut fph: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x + h / 2 as libc::c_int as libc::c_double, (*f).params);
    let mut r3: libc::c_double = 0.5f64 * (fp1 - fm1);
    let mut r5: libc::c_double = 4.0f64 / 3.0f64 * (fph - fmh) - 1.0f64 / 3.0f64 * r3;
    let mut e3: libc::c_double = (fabs(fp1) + fabs(fm1)) * 2.2204460492503131e-16f64;
    let mut e5: libc::c_double = 2.0f64 * (fabs(fph) + fabs(fmh))
        * 2.2204460492503131e-16f64 + e3;
    let mut dy: libc::c_double = (if fabs(r3 / h) > fabs(r5 / h) {
        fabs(r3 / h)
    } else {
        fabs(r5 / h)
    }) * (fabs(x) / h) * 2.2204460492503131e-16f64;
    *result = r5 / h;
    *abserr_trunc = fabs((r5 - r3) / h);
    *abserr_round = fabs(e5 / h) + dy;
}
pub unsafe extern "C" fn gsl_deriv_central(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut h: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut r_0: libc::c_double = 0.;
    let mut round: libc::c_double = 0.;
    let mut trunc: libc::c_double = 0.;
    let mut error: libc::c_double = 0.;
    central_deriv(f, x, h, &mut r_0, &mut round, &mut trunc);
    error = round + trunc;
    if round < trunc
        && (round > 0 as libc::c_int as libc::c_double
            && trunc > 0 as libc::c_int as libc::c_double)
    {
        let mut r_opt: libc::c_double = 0.;
        let mut round_opt: libc::c_double = 0.;
        let mut trunc_opt: libc::c_double = 0.;
        let mut error_opt: libc::c_double = 0.;
        let mut h_opt: libc::c_double = h
            * pow(round / (2.0f64 * trunc), 1.0f64 / 3.0f64);
        central_deriv(f, x, h_opt, &mut r_opt, &mut round_opt, &mut trunc_opt);
        error_opt = round_opt + trunc_opt;
        if error_opt < error && fabs(r_opt - r_0) < 4.0f64 * error {
            r_0 = r_opt;
            error = error_opt;
        }
    }
    *result = r_0;
    *abserr = error;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn forward_deriv(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut h: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr_round: *mut libc::c_double,
    mut abserr_trunc: *mut libc::c_double,
) {
    let mut f1: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x + h / 4.0f64, (*f).params);
    let mut f2: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x + h / 2.0f64, (*f).params);
    let mut f3: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x + 3.0f64 / 4.0f64 * h, (*f).params);
    let mut f4: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(x + h, (*f).params);
    let mut r2: libc::c_double = 2.0f64 * (f4 - f2);
    let mut r4: libc::c_double = 22.0f64 / 3.0f64 * (f4 - f3)
        - 62.0f64 / 3.0f64 * (f3 - f2) + 52.0f64 / 3.0f64 * (f2 - f1);
    let mut e4: libc::c_double = 2 as libc::c_int as libc::c_double * 20.67f64
        * (fabs(f4) + fabs(f3) + fabs(f2) + fabs(f1)) * 2.2204460492503131e-16f64;
    let mut dy: libc::c_double = (if fabs(r2 / h) > fabs(r4 / h) {
        fabs(r2 / h)
    } else {
        fabs(r4 / h)
    }) * fabs(x / h) * 2.2204460492503131e-16f64;
    *result = r4 / h;
    *abserr_trunc = fabs((r4 - r2) / h);
    *abserr_round = fabs(e4 / h) + dy;
}
pub unsafe extern "C" fn gsl_deriv_forward(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut h: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut r_0: libc::c_double = 0.;
    let mut round: libc::c_double = 0.;
    let mut trunc: libc::c_double = 0.;
    let mut error: libc::c_double = 0.;
    forward_deriv(f, x, h, &mut r_0, &mut round, &mut trunc);
    error = round + trunc;
    if round < trunc
        && (round > 0 as libc::c_int as libc::c_double
            && trunc > 0 as libc::c_int as libc::c_double)
    {
        let mut r_opt: libc::c_double = 0.;
        let mut round_opt: libc::c_double = 0.;
        let mut trunc_opt: libc::c_double = 0.;
        let mut error_opt: libc::c_double = 0.;
        let mut h_opt: libc::c_double = h * pow(round / trunc, 1.0f64 / 2.0f64);
        forward_deriv(f, x, h_opt, &mut r_opt, &mut round_opt, &mut trunc_opt);
        error_opt = round_opt + trunc_opt;
        if error_opt < error && fabs(r_opt - r_0) < 4.0f64 * error {
            r_0 = r_opt;
            error = error_opt;
        }
    }
    *result = r_0;
    *abserr = error;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_deriv_backward(
    mut f: *const gsl_function,
    mut x: libc::c_double,
    mut h: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    return gsl_deriv_forward(f, x, -h, result, abserr);
}
