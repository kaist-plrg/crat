use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub type gsl_mode_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: size_t,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: size_t,
    pub f: *mut libc::c_double,
}
pub type gsl_cheb_series = gsl_cheb_series_struct;
#[inline]
unsafe extern "C" fn GSL_MODE_PREC(mut mt: gsl_mode_t) -> libc::c_uint {
    return mt & 7 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn gsl_cheb_eval(
    mut cs: *const gsl_cheb_series,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut d1: libc::c_double = 0.0f64;
    let mut d2: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    i = (*cs).order;
    while i >= 1 as libc::c_int as libc::c_ulong {
        let mut temp: libc::c_double = d1;
        d1 = y2 * d1 - d2 + *((*cs).c).offset(i as isize);
        d2 = temp;
        i = i.wrapping_sub(1);
        i;
    }
    return y * d1 - d2 + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
}
pub unsafe extern "C" fn gsl_cheb_eval_n(
    mut cs: *const gsl_cheb_series,
    n: size_t,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut d1: libc::c_double = 0.0f64;
    let mut d2: libc::c_double = 0.0f64;
    let mut eval_order: size_t = if n < (*cs).order { n } else { (*cs).order };
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    i = eval_order;
    while i >= 1 as libc::c_int as libc::c_ulong {
        let mut temp: libc::c_double = d1;
        d1 = y2 * d1 - d2 + *((*cs).c).offset(i as isize);
        d2 = temp;
        i = i.wrapping_sub(1);
        i;
    }
    return y * d1 - d2 + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
}
pub unsafe extern "C" fn gsl_cheb_eval_err(
    mut cs: *const gsl_cheb_series,
    x: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut d1: libc::c_double = 0.0f64;
    let mut d2: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    let mut absc: libc::c_double = 0.0f64;
    i = (*cs).order;
    while i >= 1 as libc::c_int as libc::c_ulong {
        let mut temp: libc::c_double = d1;
        d1 = y2 * d1 - d2 + *((*cs).c).offset(i as isize);
        d2 = temp;
        i = i.wrapping_sub(1);
        i;
    }
    *result = y * d1 - d2 + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i <= (*cs).order {
        absc += fabs(*((*cs).c).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    *abserr = fabs(*((*cs).c).offset((*cs).order as isize))
        + absc * 2.2204460492503131e-16f64;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_cheb_eval_n_err(
    mut cs: *const gsl_cheb_series,
    n: size_t,
    x: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut d1: libc::c_double = 0.0f64;
    let mut d2: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    let mut absc: libc::c_double = 0.0f64;
    let mut eval_order: size_t = if n < (*cs).order { n } else { (*cs).order };
    i = eval_order;
    while i >= 1 as libc::c_int as libc::c_ulong {
        let mut temp: libc::c_double = d1;
        d1 = y2 * d1 - d2 + *((*cs).c).offset(i as isize);
        d2 = temp;
        i = i.wrapping_sub(1);
        i;
    }
    *result = y * d1 - d2 + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i <= eval_order {
        absc += fabs(*((*cs).c).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    *abserr = fabs(*((*cs).c).offset(eval_order as isize))
        + absc * 2.2204460492503131e-16f64;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_cheb_eval_mode_e(
    mut cs: *const gsl_cheb_series,
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut d1: libc::c_double = 0.0f64;
    let mut d2: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    let mut absc: libc::c_double = 0.0f64;
    let mut eval_order: size_t = 0;
    if GSL_MODE_PREC(mode) == 0 as libc::c_int as libc::c_uint {
        eval_order = (*cs).order;
    } else {
        eval_order = (*cs).order_sp;
    }
    i = eval_order;
    while i >= 1 as libc::c_int as libc::c_ulong {
        let mut temp: libc::c_double = d1;
        d1 = y2 * d1 - d2 + *((*cs).c).offset(i as isize);
        d2 = temp;
        i = i.wrapping_sub(1);
        i;
    }
    *result = y * d1 - d2 + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i <= eval_order {
        absc += fabs(*((*cs).c).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    *abserr = fabs(*((*cs).c).offset(eval_order as isize))
        + absc * 2.2204460492503131e-16f64;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_cheb_eval_mode(
    mut cs: *const gsl_cheb_series,
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut abserr: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_cheb_eval_mode_e(
        cs,
        x,
        mode,
        &mut result,
        &mut abserr,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_cheb_eval_mode\0" as *const u8 as *const libc::c_char,
            b"eval.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            status,
        );
        return result;
    }
    return result;
}
