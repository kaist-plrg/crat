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
    fn gsl_sf_angle_restrict_pos_e(theta: *mut libc::c_double) -> libc::c_int;
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
static mut aclaus_data: [libc::c_double; 15] = [
    2.142694363766688447e+00f64,
    0.723324281221257925e-01f64,
    0.101642475021151164e-02f64,
    0.3245250328531645e-04f64,
    0.133315187571472e-05f64,
    0.6213240591653e-07f64,
    0.313004135337e-08f64,
    0.16635723056e-09f64,
    0.919659293e-11f64,
    0.52400462e-12f64,
    0.3058040e-13f64,
    0.18197e-14f64,
    0.1100e-15f64,
    0.68e-17f64,
    0.4e-18f64,
];
static mut aclaus_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aclaus_data.as_ptr() as *mut _,
            order: 14 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_clausen_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let x_cut: libc::c_double = 3.14159265358979323846f64 * 1.4901161193847656e-08f64;
    let mut sgn: libc::c_double = 1.0f64;
    let mut status_red: libc::c_int = 0;
    if x < 0.0f64 {
        x = -x;
        sgn = -1.0f64;
    }
    status_red = gsl_sf_angle_restrict_pos_e(&mut x);
    if x > 3.14159265358979323846f64 {
        let p0: libc::c_double = 6.28125f64;
        let p1: libc::c_double = 0.19353071795864769253e-02f64;
        x = p0 - x + p1;
        sgn = -sgn;
    }
    if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
    } else if x < x_cut {
        (*result).val = x * (1.0f64 - log(x));
        (*result).err = x * 2.2204460492503131e-16f64;
    } else {
        let t: libc::c_double = 2.0f64
            * (x * x / (3.14159265358979323846f64 * 3.14159265358979323846f64) - 0.5f64);
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut aclaus_cs, t, &mut result_c);
        (*result).val = x * (result_c.val - log(x));
        (*result).err = x * (result_c.err + 2.2204460492503131e-16f64);
    }
    (*result).val *= sgn;
    return status_red;
}
pub unsafe extern "C" fn gsl_sf_clausen(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_clausen_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_clausen_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"clausen.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
