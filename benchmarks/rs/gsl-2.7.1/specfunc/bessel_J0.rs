use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_bessel_cos_pi4_e(
        y: libc::c_double,
        eps: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    static _gsl_sf_bessel_amp_phase_bm0_cs: cheb_series;
    static _gsl_sf_bessel_amp_phase_bth0_cs: cheb_series;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: libc::c_int,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: libc::c_int,
}
pub type cheb_series = cheb_series_struct;
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
static mut bj0_data: [libc::c_double; 13] = [
    0.100254161968939137f64,
    -0.665223007764405132f64,
    0.248983703498281314f64,
    -0.0332527231700357697f64,
    0.0023114179304694015f64,
    -0.0000991127741995080f64,
    0.0000028916708643998f64,
    -0.0000000612108586630f64,
    0.0000000009838650793f64,
    -0.0000000000124235515f64,
    0.0000000000001265433f64,
    -0.0000000000000010619f64,
    0.0000000000000000074f64,
];
static mut bj0_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bj0_data.as_ptr() as *mut _,
            order: 12 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_bessel_J0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut y: libc::c_double = fabs(x);
    if y < 2.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64;
        (*result).err = y * y;
        return GSL_SUCCESS as libc::c_int;
    } else if y <= 4.0f64 {
        return cheb_eval_e(&mut bj0_cs, 0.125f64 * y * y - 1.0f64, result)
    } else {
        let z: libc::c_double = 32.0f64 / (y * y) - 1.0f64;
        let mut ca: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ct: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut cp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_ca: libc::c_int = cheb_eval_e(
            &_gsl_sf_bessel_amp_phase_bm0_cs,
            z,
            &mut ca,
        );
        let stat_ct: libc::c_int = cheb_eval_e(
            &_gsl_sf_bessel_amp_phase_bth0_cs,
            z,
            &mut ct,
        );
        let stat_cp: libc::c_int = gsl_sf_bessel_cos_pi4_e(y, ct.val / y, &mut cp);
        let sqrty: libc::c_double = sqrt(y);
        let ampl: libc::c_double = (0.75f64 + ca.val) / sqrty;
        (*result).val = ampl * cp.val;
        (*result).err = fabs(cp.val) * ca.err / sqrty + fabs(ampl) * cp.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_ca != GSL_SUCCESS as libc::c_int {
            stat_ca
        } else if stat_ct != GSL_SUCCESS as libc::c_int {
            stat_ct
        } else if stat_cp != GSL_SUCCESS as libc::c_int {
            stat_cp
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_J0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_J0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_J0_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_J0.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
