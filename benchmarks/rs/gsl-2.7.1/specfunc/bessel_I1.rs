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
static mut bi1_data: [libc::c_double; 11] = [
    -0.001971713261099859f64,
    0.407348876675464810f64,
    0.034838994299959456f64,
    0.001545394556300123f64,
    0.000041888521098377f64,
    0.000000764902676483f64,
    0.000000010042493924f64,
    0.000000000099322077f64,
    0.000000000000766380f64,
    0.000000000000004741f64,
    0.000000000000000024f64,
];
static mut bi1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bi1_data.as_ptr() as *mut _,
            order: 10 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut ai1_data: [libc::c_double; 21] = [
    -0.02846744181881479f64,
    -0.01922953231443221f64,
    -0.00061151858579437f64,
    -0.00002069971253350f64,
    0.00000858561914581f64,
    0.00000104949824671f64,
    -0.00000029183389184f64,
    -0.00000001559378146f64,
    0.00000001318012367f64,
    -0.00000000144842341f64,
    -0.00000000029085122f64,
    0.00000000012663889f64,
    -0.00000000001664947f64,
    -0.00000000000166665f64,
    0.00000000000124260f64,
    -0.00000000000027315f64,
    0.00000000000002023f64,
    0.00000000000000730f64,
    -0.00000000000000333f64,
    0.00000000000000071f64,
    -0.00000000000000006f64,
];
static mut ai1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ai1_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
static mut ai12_data: [libc::c_double; 22] = [
    0.02857623501828014f64,
    -0.00976109749136147f64,
    -0.00011058893876263f64,
    -0.00000388256480887f64,
    -0.00000025122362377f64,
    -0.00000002631468847f64,
    -0.00000000383538039f64,
    -0.00000000055897433f64,
    -0.00000000001897495f64,
    0.00000000003252602f64,
    0.00000000001412580f64,
    0.00000000000203564f64,
    -0.00000000000071985f64,
    -0.00000000000040836f64,
    -0.00000000000002101f64,
    0.00000000000004273f64,
    0.00000000000001041f64,
    -0.00000000000000382f64,
    -0.00000000000000186f64,
    0.00000000000000033f64,
    0.00000000000000028f64,
    -0.00000000000000003f64,
];
static mut ai12_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ai12_data.as_ptr() as *mut _,
            order: 21 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_bessel_I1_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let xmin: libc::c_double = 2.0f64 * 2.2250738585072014e-308f64;
    let x_small: libc::c_double = 2.0f64 * 1.41421356237309504880f64
        * 1.4901161193847656e-08f64;
    let y: libc::c_double = fabs(x);
    if y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if y < xmin {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"bessel_I1.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if y < x_small {
        (*result).val = 0.5f64 * x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if y <= 3.0f64 {
        let ey: libc::c_double = exp(-y);
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut bi1_cs, y * y / 4.5f64 - 1.0f64, &mut c);
        (*result).val = x * ey * (0.875f64 + c.val);
        (*result).err = ey * c.err + y * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if y <= 8.0f64 {
        let sy: libc::c_double = sqrt(y);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut b: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        cheb_eval_e(&mut ai1_cs, (48.0f64 / y - 11.0f64) / 5.0f64, &mut c_0);
        b = (0.375f64 + c_0.val) / sy;
        s = if x > 0.0f64 { 1.0f64 } else { -1.0f64 };
        (*result).val = s * b;
        (*result).err = c_0.err / sy;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let sy_0: libc::c_double = sqrt(y);
        let mut c_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut b_0: libc::c_double = 0.;
        let mut s_0: libc::c_double = 0.;
        cheb_eval_e(&mut ai12_cs, 16.0f64 / y - 1.0f64, &mut c_1);
        b_0 = (0.375f64 + c_1.val) / sy_0;
        s_0 = if x > 0.0f64 { 1.0f64 } else { -1.0f64 };
        (*result).val = s_0 * b_0;
        (*result).err = c_1.err / sy_0;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_I1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let xmin: libc::c_double = 2.0f64 * 2.2250738585072014e-308f64;
    let x_small: libc::c_double = 2.0f64 * 1.41421356237309504880f64
        * 1.4901161193847656e-08f64;
    let y: libc::c_double = fabs(x);
    if y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if y < xmin {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"bessel_I1.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if y < x_small {
        (*result).val = 0.5f64 * x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if y <= 3.0f64 {
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut bi1_cs, y * y / 4.5f64 - 1.0f64, &mut c);
        (*result).val = x * (0.875f64 + c.val);
        (*result).err = y * c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if y < 7.0978271289338397e+02f64 {
        let ey: libc::c_double = exp(y);
        let mut I1_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_bessel_I1_scaled_e(x, &mut I1_scaled);
        (*result).val = ey * I1_scaled.val;
        (*result)
            .err = ey * I1_scaled.err
            + y * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_I1.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_I1_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_I1_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_I1_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_I1.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_I1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_I1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_I1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_I1.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
