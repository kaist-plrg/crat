use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
static mut atanint_data: [libc::c_double; 21] = [
    1.91040361296235937512f64,
    -0.4176351437656746940e-01f64,
    0.275392550786367434e-02f64,
    -0.25051809526248881e-03f64,
    0.2666981285121171e-04f64,
    -0.311890514107001e-05f64,
    0.38833853132249e-06f64,
    -0.5057274584964e-07f64,
    0.681225282949e-08f64,
    -0.94212561654e-09f64,
    0.13307878816e-09f64,
    -0.1912678075e-10f64,
    0.278912620e-11f64,
    -0.41174820e-12f64,
    0.6142987e-13f64,
    -0.924929e-14f64,
    0.140387e-14f64,
    -0.21460e-15f64,
    0.3301e-16f64,
    -0.511e-17f64,
    0.79e-18f64,
];
static mut atanint_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: atanint_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_atanint_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    let sgn: libc::c_double = (if x >= 0.0f64 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as libc::c_double;
    if ax == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax < 0.5f64 * 1.4901161193847656e-08f64 {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax <= 1.0f64 {
        let t: libc::c_double = 2.0f64 * (x * x - 0.5f64);
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut atanint_cs, t, &mut result_c);
        (*result).val = x * result_c.val;
        (*result).err = x * result_c.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if ax < 1.0f64 / 1.4901161193847656e-08f64 {
        let t_0: libc::c_double = 2.0f64 * (1.0f64 / (x * x) - 0.5f64);
        let mut result_c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut atanint_cs, t_0, &mut result_c_0);
        (*result)
            .val = sgn
            * (0.5f64 * 3.14159265358979323846f64 * log(ax) + result_c_0.val / ax);
        (*result)
            .err = result_c_0.err / ax + fabs((*result).val * 2.2204460492503131e-16f64);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result)
            .val = sgn * (0.5f64 * 3.14159265358979323846f64 * log(ax) + 1.0f64 / ax);
        (*result).err = 2.0f64 * fabs((*result).val * 2.2204460492503131e-16f64);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_atanint(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_atanint_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_atanint_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"atanint.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
