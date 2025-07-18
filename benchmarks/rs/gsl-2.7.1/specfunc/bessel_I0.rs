use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
static mut bi0_data: [libc::c_double; 12] = [
    -0.07660547252839144951f64,
    1.92733795399380827000f64,
    0.22826445869203013390f64,
    0.01304891466707290428f64,
    0.00043442709008164874f64,
    0.00000942265768600193f64,
    0.00000014340062895106f64,
    0.00000000161384906966f64,
    0.00000000001396650044f64,
    0.00000000000009579451f64,
    0.00000000000000053339f64,
    0.00000000000000000245f64,
];
static mut bi0_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bi0_data.as_ptr() as *mut _,
            order: 11 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
static mut ai0_data: [libc::c_double; 21] = [
    0.07575994494023796f64,
    0.00759138081082334f64,
    0.00041531313389237f64,
    0.00001070076463439f64,
    -0.00000790117997921f64,
    -0.00000078261435014f64,
    0.00000027838499429f64,
    0.00000000825247260f64,
    -0.00000001204463945f64,
    0.00000000155964859f64,
    0.00000000022925563f64,
    -0.00000000011916228f64,
    0.00000000001757854f64,
    0.00000000000112822f64,
    -0.00000000000114684f64,
    0.00000000000027155f64,
    -0.00000000000002415f64,
    -0.00000000000000608f64,
    0.00000000000000314f64,
    -0.00000000000000071f64,
    0.00000000000000007f64,
];
static mut ai0_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ai0_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
static mut ai02_data: [libc::c_double; 22] = [
    0.05449041101410882f64,
    0.00336911647825569f64,
    0.00006889758346918f64,
    0.00000289137052082f64,
    0.00000020489185893f64,
    0.00000002266668991f64,
    0.00000000339623203f64,
    0.00000000049406022f64,
    0.00000000001188914f64,
    -0.00000000003149915f64,
    -0.00000000001321580f64,
    -0.00000000000179419f64,
    0.00000000000071801f64,
    0.00000000000038529f64,
    0.00000000000001539f64,
    -0.00000000000004151f64,
    -0.00000000000000954f64,
    0.00000000000000382f64,
    0.00000000000000176f64,
    -0.00000000000000034f64,
    -0.00000000000000027f64,
    0.00000000000000003f64,
];
static mut ai02_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ai02_data.as_ptr() as *mut _,
            order: 21 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_bessel_I0_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut y: libc::c_double = fabs(x);
    if y < 2.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64 - y;
        (*result).err = 0.5f64 * y * y;
        return GSL_SUCCESS as libc::c_int;
    } else if y <= 3.0f64 {
        let ey: libc::c_double = exp(-y);
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut bi0_cs, y * y / 4.5f64 - 1.0f64, &mut c);
        (*result).val = ey * (2.75f64 + c.val);
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val) + ey * c.err;
        return GSL_SUCCESS as libc::c_int;
    } else if y <= 8.0f64 {
        let sy: libc::c_double = sqrt(y);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut ai0_cs, (48.0f64 / y - 11.0f64) / 5.0f64, &mut c_0);
        (*result).val = (0.375f64 + c_0.val) / sy;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (0.375f64 + fabs(c_0.val)) / sy;
        (*result).err += c_0.err / sy;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let sy_0: libc::c_double = sqrt(y);
        let mut c_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut ai02_cs, 16.0f64 / y - 1.0f64, &mut c_1);
        (*result).val = (0.375f64 + c_1.val) / sy_0;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (0.375f64 + fabs(c_1.val))
            / sy_0;
        (*result).err += c_1.err / sy_0;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_I0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut y: libc::c_double = fabs(x);
    if y < 2.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.5f64 * y * y;
        return GSL_SUCCESS as libc::c_int;
    } else if y <= 3.0f64 {
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut bi0_cs, y * y / 4.5f64 - 1.0f64, &mut c);
        (*result).val = 2.75f64 + c.val;
        (*result).err = 2.2204460492503131e-16f64 * (2.75f64 + fabs(c.val));
        (*result).err += c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if y < 7.0978271289338397e+02f64 - 1.0f64 {
        let ey: libc::c_double = exp(y);
        let mut b_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_bessel_I0_scaled_e(x, &mut b_scaled);
        (*result).val = ey * b_scaled.val;
        (*result)
            .err = ey * b_scaled.err
            + y * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_I0.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_I0_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_I0_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_I0_scaled_e(x, &result);\0" as *const u8
                as *const libc::c_char,
            b"bessel_I0.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_I0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_I0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_I0_e(x, &result);\0" as *const u8 as *const libc::c_char,
            b"bessel_I0.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
