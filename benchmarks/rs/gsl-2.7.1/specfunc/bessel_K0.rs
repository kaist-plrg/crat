use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
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
unsafe extern "C" fn gsl_poly_eval(
    mut c: *const libc::c_double,
    len: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut ans: libc::c_double = *c.offset((len - 1 as libc::c_int) as isize);
    i = len - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        ans = *c.offset((i - 1 as libc::c_int) as isize) + x * ans;
        i -= 1;
        i;
    }
    return ans;
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
static mut k0_poly: [libc::c_double; 8] = [
    1.1593151565841244842077226e-01f64,
    2.7898287891460317300886539e-01f64,
    2.5248929932161220559969776e-02f64,
    8.4603509072136578707676406e-04f64,
    1.4914719243067801775856150e-05f64,
    1.6271068931224552553548933e-07f64,
    1.2082660336282566759313543e-09f64,
    6.6117104672254184399933971e-12f64,
];
static mut i0_poly: [libc::c_double; 7] = [
    1.0000000000000000044974165e+00f64,
    2.4999999999999822316775454e-01f64,
    2.7777777777892149148858521e-02f64,
    1.7361111083544590676709592e-03f64,
    6.9444476047072424198677755e-05f64,
    1.9288265756466775034067979e-06f64,
    3.9908220583262192851839992e-08f64,
];
static mut ak0_data: [libc::c_double; 24] = [
    -3.28737867094650101e-02f64,
    -4.49369057710236880e-02f64,
    2.98149992004308095e-03f64,
    -3.03693649396187920e-04f64,
    3.91085569307646836e-05f64,
    -5.86872422399215952e-06f64,
    9.82873709937322009e-07f64,
    -1.78978645055651171e-07f64,
    3.48332306845240957e-08f64,
    -7.15909210462546599e-09f64,
    1.54019930048919494e-09f64,
    -3.44555485579194210e-10f64,
    7.97356101783753023e-11f64,
    -1.90090968913069735e-11f64,
    4.65295609304114621e-12f64,
    -1.16614287433470780e-12f64,
    2.98554375218596891e-13f64,
    -7.79276979512292169e-14f64,
    2.07027467168948402e-14f64,
    -5.58987860393825313e-15f64,
    1.53202965950646914e-15f64,
    -4.25737536712188186e-16f64,
    1.19840238501357389e-16f64,
    -3.41407346762502397e-17f64,
];
static mut ak0_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ak0_data.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut ak02_data: [libc::c_double; 14] = [
    -0.1201869826307592240E-1f64,
    -0.9174852691025695311E-2f64,
    0.1444550931775005821E-3f64,
    -0.4013614175435709729E-5f64,
    0.1567831810852310673E-6f64,
    -0.7770110438521737710E-8f64,
    0.4611182576179717883E-9f64,
    -0.3158592997860565771E-10f64,
    0.2435018039365041128E-11f64,
    -0.2074331387398347898E-12f64,
    0.1925787280589917085E-13f64,
    -0.1927554805838956104E-14f64,
    0.2062198029197818278E-15f64,
    -0.2341685117579242403E-16f64,
];
static mut ak02_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ak02_data.as_ptr() as *mut _,
            order: 13 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_bessel_K0_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_K0.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.0f64 {
        let lx: libc::c_double = log(x);
        let ex: libc::c_double = exp(x);
        let x2: libc::c_double = x * x;
        (*result)
            .val = ex
            * (gsl_poly_eval(
                k0_poly.as_mut_ptr() as *const libc::c_double,
                8 as libc::c_int,
                x2,
            )
                - lx
                    * (1.0f64
                        + 0.25f64 * x2
                            * gsl_poly_eval(
                                i0_poly.as_mut_ptr() as *const libc::c_double,
                                7 as libc::c_int,
                                0.25f64 * x2,
                            )));
        (*result).err = ex * (1.6f64 + fabs(lx) * 0.6f64) * 2.2204460492503131e-16f64;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 8.0f64 {
        let sx: libc::c_double = sqrt(x);
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut ak0_cs, (16.0f64 / x - 9.0f64) / 7.0f64, &mut c);
        (*result).val = (1.203125f64 + c.val) / sx;
        (*result).err = c.err / sx;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let sx_0: libc::c_double = sqrt(x);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut ak02_cs, 16.0f64 / x - 1.0f64, &mut c_0);
        (*result).val = (1.25f64 + c_0.val) / sx_0;
        (*result).err = (c_0.err + 2.2204460492503131e-16f64) / sx_0;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_K0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_K0.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.0f64 {
        let lx: libc::c_double = log(x);
        let x2: libc::c_double = x * x;
        (*result)
            .val = gsl_poly_eval(
            k0_poly.as_mut_ptr() as *const libc::c_double,
            8 as libc::c_int,
            x2,
        )
            - lx
                * (1.0f64
                    + 0.25f64 * x2
                        * gsl_poly_eval(
                            i0_poly.as_mut_ptr() as *const libc::c_double,
                            7 as libc::c_int,
                            0.25f64 * x2,
                        ));
        (*result).err = (1.6f64 + fabs(lx) * 0.6f64) * 2.2204460492503131e-16f64;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut K0_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K0: libc::c_int = gsl_sf_bessel_K0_scaled_e(x, &mut K0_scaled);
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            -x,
            2.2204460492503131e-16f64 * fabs(x),
            K0_scaled.val,
            K0_scaled.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_K0 != GSL_SUCCESS as libc::c_int {
            stat_K0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_K0_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_K0_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_K0_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_K0.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_K0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_K0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_K0_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_K0.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
