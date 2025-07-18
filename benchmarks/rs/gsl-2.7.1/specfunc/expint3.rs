use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
static mut expint3_data: [libc::c_double; 24] = [
    1.269198414221126014f64,
    -0.248846446384140982f64,
    0.80526220717231041e-01f64,
    -0.25772733251968330e-01f64,
    0.7599878873073774e-02f64,
    -0.2030695581940405e-02f64,
    0.490834586699330e-03f64,
    -0.107682239142021e-03f64,
    0.21551726264290e-04f64,
    -0.3956705137384e-05f64,
    0.6699240933896e-06f64,
    -0.105132180807e-06f64,
    0.15362580199e-07f64,
    -0.20990960364e-08f64,
    0.2692109538e-09f64,
    -0.325195242e-10f64,
    0.37114816e-11f64,
    -0.4013652e-12f64,
    0.412334e-13f64,
    -0.40338e-14f64,
    0.3766e-15f64,
    -0.336e-16f64,
    0.29e-17f64,
    -0.2e-18f64,
];
static mut expint3_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: expint3_data.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut expint3a_data: [libc::c_double; 23] = [
    1.9270464955068273729f64,
    -0.349293565204813805e-01f64,
    0.14503383718983009e-02f64,
    -0.8925336718327903e-04f64,
    0.70542392191184e-05f64,
    -0.6671727454761e-06f64,
    0.724267589982e-07f64,
    -0.87825825606e-08f64,
    0.11672234428e-08f64,
    -0.1676631281e-09f64,
    0.257550158e-10f64,
    -0.41957888e-11f64,
    0.7201041e-12f64,
    -0.1294906e-12f64,
    0.24287e-13f64,
    -0.47331e-14f64,
    0.95531e-15f64,
    -0.1991e-15f64,
    0.428e-16f64,
    -0.94e-17f64,
    0.21e-17f64,
    -0.5e-18f64,
    0.1e-18f64,
];
static mut expint3a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: expint3a_data.as_ptr() as *mut _,
            order: 22 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_expint_3_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 0.892979511569249211f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"expint3.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.6f64 * 6.0554544523933429e-06f64 {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 2.0f64 {
        let t: libc::c_double = x * x * x / 4.0f64 - 1.0f64;
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut expint3_cs, t, &mut result_c);
        (*result).val = x * result_c.val;
        (*result).err = x * result_c.err;
        return GSL_SUCCESS as libc::c_int;
    } else if x < pow(--3.6043653389117154e+01f64, 1.0f64 / 3.0f64) {
        let t_0: libc::c_double = 16.0f64 / (x * x * x) - 1.0f64;
        let s: libc::c_double = exp(-x * x * x) / (3.0f64 * x * x);
        let mut result_c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut expint3a_cs, t_0, &mut result_c_0);
        (*result).val = val_infinity - result_c_0.val * s;
        (*result).err = val_infinity * 2.2204460492503131e-16f64 + s * result_c_0.err;
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = val_infinity;
        (*result).err = val_infinity * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_expint_3(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_3_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_3_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"expint3.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
