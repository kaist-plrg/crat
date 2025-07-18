use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_expint_E1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_expint_Ei_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
}
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
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
pub type cheb_series = cheb_series_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: libc::c_int,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: libc::c_int,
}
#[inline]
unsafe extern "C" fn cheb_eval_e(
    mut cs: *const cheb_series,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut d: libc::c_double = 0.0f64;
    let mut dd: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    let mut e: libc::c_double = 0.0f64;
    j = (*cs).order;
    while j >= 1 as libc::c_int {
        let mut temp: libc::c_double = d;
        d = y2 * d - dd + *((*cs).c).offset(j as isize);
        e += fabs(y2 * temp) + fabs(dd) + fabs(*((*cs).c).offset(j as isize));
        dd = temp;
        j -= 1;
        j;
    }
    let mut temp_0: libc::c_double = d;
    d = y * d - dd + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
    e
        += fabs(y * temp_0) + fabs(dd)
            + 0.5f64 * fabs(*((*cs).c).offset(0 as libc::c_int as isize));
    (*result).val = d;
    (*result)
        .err = 2.2204460492503131e-16f64 * e
        + fabs(*((*cs).c).offset((*cs).order as isize));
    return GSL_SUCCESS as libc::c_int;
}
static mut shi_data: [libc::c_double; 7] = [
    0.0078372685688900950695f64,
    0.0039227664934234563973f64,
    0.0000041346787887617267f64,
    0.0000000024707480372883f64,
    0.0000000000009379295591f64,
    0.0000000000000002451817f64,
    0.0000000000000000000467f64,
];
static mut shi_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: shi_data.as_ptr() as *mut _,
            order: 6 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 6 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_Shi_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let xsml: libc::c_double = 1.4901161193847656e-08f64;
    let ax: libc::c_double = fabs(x);
    if ax < xsml {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax <= 0.375f64 {
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut shi_cs, 128.0f64 * x * x / 9.0f64 - 1.0f64, &mut result_c);
        (*result).val = x * (1.0f64 + result_c.val);
        (*result).err = x * result_c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut result_Ei: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_E1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_Ei: libc::c_int = gsl_sf_expint_Ei_e(x, &mut result_Ei);
        let mut status_E1: libc::c_int = gsl_sf_expint_E1_e(x, &mut result_E1);
        (*result).val = 0.5f64 * (result_Ei.val + result_E1.val);
        (*result).err = 0.5f64 * (result_Ei.err + result_E1.err);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if status_Ei == GSL_EUNDRFLW as libc::c_int
            && status_E1 == GSL_EUNDRFLW as libc::c_int
        {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"shint.c\0" as *const u8 as *const libc::c_char,
                88 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else if status_Ei == GSL_EOVRFLW as libc::c_int
            || status_E1 == GSL_EOVRFLW as libc::c_int
        {
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"shint.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else {
            return GSL_SUCCESS as libc::c_int
        }
    };
}
pub unsafe extern "C" fn gsl_sf_Chi_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut result_Ei: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut result_E1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status_Ei: libc::c_int = gsl_sf_expint_Ei_e(x, &mut result_Ei);
    let mut status_E1: libc::c_int = gsl_sf_expint_E1_e(x, &mut result_E1);
    if status_Ei == GSL_EDOM as libc::c_int || status_E1 == GSL_EDOM as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"shint.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if status_Ei == GSL_EUNDRFLW as libc::c_int
        && status_E1 == GSL_EUNDRFLW as libc::c_int
    {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"shint.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if status_Ei == GSL_EOVRFLW as libc::c_int
        || status_E1 == GSL_EOVRFLW as libc::c_int
    {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"shint.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        (*result).val = 0.5f64 * (result_Ei.val - result_E1.val);
        (*result).err = 0.5f64 * (result_Ei.err + result_E1.err);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_Shi(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_Shi_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_Shi_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"shint.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_Chi(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_Chi_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_Chi_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"shint.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
