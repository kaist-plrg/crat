use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_multiply_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_expm1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exprel_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exprel_2_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exprel_n_e(
        n: libc::c_int,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_mult_err_e10_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result_e10,
    ) -> libc::c_int;
    fn gsl_sf_bessel_J1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_bessel_I1_scaled_e(
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_In_scaled(n: libc::c_int, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_bessel_Inu_scaled_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_sgn_e(
        x: libc::c_double,
        result_lg: *mut gsl_sf_result,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_lnfact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lnbeta_e(
        a: libc::c_double,
        b: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_beta_e(
        a: libc::c_double,
        b: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_laguerre_n_e(
        n: libc::c_int,
        a: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_U_e10_e(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result_e10,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_1F1_series_e(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_2F0_series_e(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
        n_trunc: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_e10_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
    pub e10: libc::c_int,
}
pub type gsl_sf_result_e10 = gsl_sf_result_e10_struct;
#[inline]
unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn hyperg_1F1_asymp_negx(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut lg_b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lg_bma: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut sgn_b: libc::c_double = 0.;
    let mut sgn_bma: libc::c_double = 0.;
    let mut stat_b: libc::c_int = gsl_sf_lngamma_sgn_e(b, &mut lg_b, &mut sgn_b);
    let mut stat_bma: libc::c_int = gsl_sf_lngamma_sgn_e(
        b - a,
        &mut lg_bma,
        &mut sgn_bma,
    );
    if stat_b == GSL_SUCCESS as libc::c_int && stat_bma == GSL_SUCCESS as libc::c_int {
        let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_F: libc::c_int = gsl_sf_hyperg_2F0_series_e(
            a,
            1.0f64 + a - b,
            -1.0f64 / x,
            -(1 as libc::c_int),
            &mut F,
        );
        if F.val != 0 as libc::c_int as libc::c_double {
            let mut ln_term_val: libc::c_double = a * log(-x);
            let mut ln_term_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(a) + fabs(ln_term_val));
            let mut ln_pre_val: libc::c_double = lg_b.val - lg_bma.val - ln_term_val;
            let mut ln_pre_err: libc::c_double = lg_b.err + lg_bma.err + ln_term_err;
            let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
                ln_pre_val,
                ln_pre_err,
                sgn_bma * sgn_b * F.val,
                F.err,
                result,
            );
            return if stat_e != GSL_SUCCESS as libc::c_int {
                stat_e
            } else if stat_F != GSL_SUCCESS as libc::c_int {
                stat_F
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_F;
        }
    } else {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
unsafe extern "C" fn hyperg_1F1_asymp_posx(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut lg_b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lg_a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut sgn_b: libc::c_double = 0.;
    let mut sgn_a: libc::c_double = 0.;
    let mut stat_b: libc::c_int = gsl_sf_lngamma_sgn_e(b, &mut lg_b, &mut sgn_b);
    let mut stat_a: libc::c_int = gsl_sf_lngamma_sgn_e(a, &mut lg_a, &mut sgn_a);
    if stat_a == GSL_SUCCESS as libc::c_int && stat_b == GSL_SUCCESS as libc::c_int {
        let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_F: libc::c_int = gsl_sf_hyperg_2F0_series_e(
            b - a,
            1.0f64 - a,
            1.0f64 / x,
            -(1 as libc::c_int),
            &mut F,
        );
        if stat_F == GSL_SUCCESS as libc::c_int
            && F.val != 0 as libc::c_int as libc::c_double
        {
            let mut lnx: libc::c_double = log(x);
            let mut ln_term_val: libc::c_double = (a - b) * lnx;
            let mut ln_term_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(a) + fabs(b)) * fabs(lnx)
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(a - b);
            let mut ln_pre_val: libc::c_double = lg_b.val - lg_a.val + ln_term_val + x;
            let mut ln_pre_err: libc::c_double = lg_b.err + lg_a.err + ln_term_err
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(x);
            let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
                ln_pre_val,
                ln_pre_err,
                sgn_a * sgn_b * F.val,
                F.err,
                result,
            );
            return if stat_e != GSL_SUCCESS as libc::c_int {
                stat_e
            } else if stat_F != GSL_SUCCESS as libc::c_int {
                stat_F
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_F;
        }
    } else {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
unsafe extern "C" fn hyperg_1F1_largebx(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut y: libc::c_double = x / b;
    let mut f: libc::c_double = exp(-a * log1p(-y));
    let mut t1: libc::c_double = -(a * (a + 1.0f64)
        / (2 as libc::c_int as libc::c_double * b)) * pow(y / (1.0f64 - y), 2.0f64);
    let mut t2: libc::c_double = 1 as libc::c_int as libc::c_double
        / (24 as libc::c_int as libc::c_double * b * b)
        * (a * (a + 1 as libc::c_int as libc::c_double) * y * y
            / pow(
                1 as libc::c_int as libc::c_double - y,
                4 as libc::c_int as libc::c_double,
            ))
        * (12 as libc::c_int as libc::c_double
            + 8 as libc::c_int as libc::c_double
                * (2 as libc::c_int as libc::c_double * a
                    + 1 as libc::c_int as libc::c_double) * y
            + (3 as libc::c_int as libc::c_double * a * a - a
                - 2 as libc::c_int as libc::c_double) * y * y);
    let mut t3: libc::c_double = -(1 as libc::c_int) as libc::c_double
        / (48 as libc::c_int as libc::c_double * b * b * b
            * pow(
                1 as libc::c_int as libc::c_double - y,
                6 as libc::c_int as libc::c_double,
            )) * a
        * ((a + 1 as libc::c_int as libc::c_double)
            * ((y
                * ((a + 1 as libc::c_int as libc::c_double)
                    * (a
                        * (y
                            * (y
                                * ((y * (a - 2 as libc::c_int as libc::c_double)
                                    + 16 as libc::c_int as libc::c_double)
                                    * (a - 1 as libc::c_int as libc::c_double))
                                + 72 as libc::c_int as libc::c_double))
                        + 96 as libc::c_int as libc::c_double))
                + 24 as libc::c_int as libc::c_double)
                * pow(y, 2 as libc::c_int as libc::c_double)));
    (*result).val = f * (1 as libc::c_int as libc::c_double + t1 + t2 + t3);
    (*result)
        .err = 2 as libc::c_int as libc::c_double * fabs(f * t3)
        + 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
            * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hyperg_1F1_large2bm4a(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut eta: libc::c_double = 2.0f64 * b - 4.0f64 * a;
    let mut cos2th: libc::c_double = x / eta;
    let mut sin2th: libc::c_double = 1.0f64 - cos2th;
    let mut th: libc::c_double = acos(sqrt(cos2th));
    let mut pre_h: libc::c_double = 0.25f64 * 3.14159265358979323846f64
        * 3.14159265358979323846f64 * eta * eta * cos2th * sin2th;
    let mut lg_b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_lg: libc::c_int = gsl_sf_lngamma_e(b, &mut lg_b);
    let mut t1: libc::c_double = 0.5f64 * (1.0f64 - b) * log(0.25f64 * x * eta);
    let mut t2: libc::c_double = 0.25f64 * log(pre_h);
    let mut lnpre_val: libc::c_double = lg_b.val + 0.5f64 * x + t1 - t2;
    let mut lnpre_err: libc::c_double = lg_b.err
        + 2.0f64 * 2.2204460492503131e-16f64 * (fabs(0.5f64 * x) + fabs(t1) + fabs(t2));
    let mut s1: libc::c_double = sin(a * 3.14159265358979323846f64);
    let mut s2: libc::c_double = sin(
        0.25f64 * eta * (2.0f64 * th - sin(2.0f64 * th))
            + 0.25f64 * 3.14159265358979323846f64,
    );
    let mut ser_val: libc::c_double = s1 + s2;
    let mut ser_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
        * (fabs(s1) + fabs(s2));
    let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
        lnpre_val,
        lnpre_err,
        ser_val,
        ser_err,
        result,
    );
    return if stat_e != GSL_SUCCESS as libc::c_int {
        stat_e
    } else if stat_lg != GSL_SUCCESS as libc::c_int {
        stat_lg
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn hyperg_1F1_luke(
    a: libc::c_double,
    c: libc::c_double,
    xin: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let RECUR_BIG: libc::c_double = 1.0e+50f64;
    let nmax: libc::c_int = 5000 as libc::c_int;
    let mut n: libc::c_int = 3 as libc::c_int;
    let x: libc::c_double = -xin;
    let x3: libc::c_double = x * x * x;
    let t0: libc::c_double = a / c;
    let t1: libc::c_double = (a + 1.0f64) / (2.0f64 * c);
    let t2: libc::c_double = (a + 2.0f64) / (2.0f64 * (c + 1.0f64));
    let mut F: libc::c_double = 1.0f64;
    let mut prec: libc::c_double = 0.;
    let mut Bnm3: libc::c_double = 1.0f64;
    let mut Bnm2: libc::c_double = 1.0f64 + t1 * x;
    let mut Bnm1: libc::c_double = 1.0f64 + t2 * x * (1.0f64 + t1 / 3.0f64 * x);
    let mut Anm3: libc::c_double = 1.0f64;
    let mut Anm2: libc::c_double = Bnm2 - t0 * x;
    let mut Anm1: libc::c_double = Bnm1 - t0 * (1.0f64 + t2 * x) * x
        + t0 * t1 * (c / (c + 1.0f64)) * x * x;
    loop {
        let mut npam1: libc::c_double = n as libc::c_double + a
            - 1 as libc::c_int as libc::c_double;
        let mut npcm1: libc::c_double = n as libc::c_double + c
            - 1 as libc::c_int as libc::c_double;
        let mut npam2: libc::c_double = n as libc::c_double + a
            - 2 as libc::c_int as libc::c_double;
        let mut npcm2: libc::c_double = n as libc::c_double + c
            - 2 as libc::c_int as libc::c_double;
        let mut tnm1: libc::c_double = (2 as libc::c_int * n - 1 as libc::c_int)
            as libc::c_double;
        let mut tnm3: libc::c_double = (2 as libc::c_int * n - 3 as libc::c_int)
            as libc::c_double;
        let mut tnm5: libc::c_double = (2 as libc::c_int * n - 5 as libc::c_int)
            as libc::c_double;
        let mut F1: libc::c_double = (n as libc::c_double - a
            - 2 as libc::c_int as libc::c_double)
            / (2 as libc::c_int as libc::c_double * tnm3 * npcm1);
        let mut F2: libc::c_double = (n as libc::c_double + a) * npam1
            / (4 as libc::c_int as libc::c_double * tnm1 * tnm3 * npcm2 * npcm1);
        let mut F3: libc::c_double = -npam2 * npam1
            * (n as libc::c_double - a - 2 as libc::c_int as libc::c_double)
            / (8 as libc::c_int as libc::c_double * tnm3 * tnm3 * tnm5
                * (n as libc::c_double + c - 3 as libc::c_int as libc::c_double) * npcm2
                * npcm1);
        let mut E: libc::c_double = -npam1
            * (n as libc::c_double - c - 1 as libc::c_int as libc::c_double)
            / (2 as libc::c_int as libc::c_double * tnm3 * npcm2 * npcm1);
        let mut An: libc::c_double = (1.0f64 + F1 * x) * Anm1 + (E + F2 * x) * x * Anm2
            + F3 * x3 * Anm3;
        let mut Bn: libc::c_double = (1.0f64 + F1 * x) * Bnm1 + (E + F2 * x) * x * Bnm2
            + F3 * x3 * Bnm3;
        let mut r: libc::c_double = An / Bn;
        prec = fabs((F - r) / F);
        F = r;
        if prec < 2.2204460492503131e-16f64 || n > nmax {
            break;
        }
        if fabs(An) > RECUR_BIG || fabs(Bn) > RECUR_BIG {
            An /= RECUR_BIG;
            Bn /= RECUR_BIG;
            Anm1 /= RECUR_BIG;
            Bnm1 /= RECUR_BIG;
            Anm2 /= RECUR_BIG;
            Bnm2 /= RECUR_BIG;
            Anm3 /= RECUR_BIG;
            Bnm3 /= RECUR_BIG;
        } else if fabs(An) < 1.0f64 / RECUR_BIG || fabs(Bn) < 1.0f64 / RECUR_BIG {
            An *= RECUR_BIG;
            Bn *= RECUR_BIG;
            Anm1 *= RECUR_BIG;
            Bnm1 *= RECUR_BIG;
            Anm2 *= RECUR_BIG;
            Bnm2 *= RECUR_BIG;
            Anm3 *= RECUR_BIG;
            Bnm3 *= RECUR_BIG;
        }
        n += 1;
        n;
        Bnm3 = Bnm2;
        Bnm2 = Bnm1;
        Bnm1 = Bn;
        Anm3 = Anm2;
        Anm2 = Anm1;
        Anm1 = An;
    }
    (*result).val = F;
    (*result).err = 2.0f64 * fabs(F * prec);
    (*result).err
        += 2.0f64 * 2.2204460492503131e-16f64 * (n as libc::c_double - 1.0f64) * fabs(F);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hyperg_1F1_1_series(
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut sum_val: libc::c_double = 1.0f64;
    let mut sum_err: libc::c_double = 0.0f64;
    let mut term: libc::c_double = 1.0f64;
    let mut n: libc::c_double = 1.0f64;
    while fabs(term / sum_val) > 0.25f64 * 2.2204460492503131e-16f64 {
        term *= x / (b + n - 1 as libc::c_int as libc::c_double);
        sum_val += term;
        sum_err
            += 8.0f64 * 2.2204460492503131e-16f64 * fabs(term)
                + 2.2204460492503131e-16f64 * fabs(sum_val);
        n += 1.0f64;
    }
    (*result).val = sum_val;
    (*result).err = sum_err;
    (*result).err += 2.0f64 * fabs(term);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hyperg_1F1_1_int(
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if b < 1 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if b == 1 as libc::c_int {
        return gsl_sf_exp_e(x, result)
    } else if b == 2 as libc::c_int {
        return gsl_sf_exprel_e(x, result)
    } else if b == 3 as libc::c_int {
        return gsl_sf_exprel_2_e(x, result)
    } else {
        return gsl_sf_exprel_n_e(b - 1 as libc::c_int, x, result)
    };
}
unsafe extern "C" fn hyperg_1F1_1(
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    let mut ib: libc::c_double = floor(b + 0.1f64);
    if b < 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            352 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if b == 1.0f64 {
        return gsl_sf_exp_e(x, result)
    } else if b >= 1.4f64 * ax {
        return hyperg_1F1_1_series(b, x, result)
    } else if fabs(b - ib) < 100.0f64 * 2.2204460492503131e-16f64
        && ib < 2147483647 as libc::c_int as libc::c_double
    {
        return hyperg_1F1_1_int(ib as libc::c_int, x, result)
    } else if x > 0.0f64 {
        if x > 100.0f64 && b < 0.75f64 * x {
            return hyperg_1F1_asymp_posx(1.0f64, b, x, result)
        } else if b < 1.0e+05f64 {
            let off: libc::c_double = ceil(1.4f64 * x - b) + 1.0f64;
            let mut bp: libc::c_double = b + off;
            let mut M: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_s: libc::c_int = hyperg_1F1_1_series(bp, x, &mut M);
            let err_rat: libc::c_double = M.err / fabs(M.val);
            while bp > b + 0.1f64 {
                bp -= 1.0f64;
                M.val = 1.0f64 + x / bp * M.val;
            }
            (*result).val = M.val;
            (*result).err = err_rat * fabs(M.val);
            (*result).err
                += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(off) + 1.0f64)
                    * fabs(M.val);
            return stat_s;
        } else if fabs(x) < fabs(b) && fabs(x) < sqrt(fabs(b)) * fabs(b - x) {
            return hyperg_1F1_largebx(1.0f64, b, x, result)
        } else if fabs(x) > fabs(b) {
            return hyperg_1F1_1_series(b, x, result)
        } else {
            return hyperg_1F1_large2bm4a(1.0f64, b, x, result)
        }
    } else if ax < 10.0f64 && b < 10.0f64 {
        return hyperg_1F1_1_series(b, x, result)
    } else if ax >= 100.0f64 && GSL_MAX_DBL(fabs(2.0f64 - b), 1.0f64) < 0.99f64 * ax {
        return hyperg_1F1_asymp_negx(1.0f64, b, x, result)
    } else {
        return hyperg_1F1_luke(1.0f64, b, x, result)
    };
}
unsafe extern "C" fn hyperg_1F1_renorm_b0(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut eta: libc::c_double = a * x;
    if eta > 0.0f64 {
        let mut root_eta: libc::c_double = sqrt(eta);
        let mut I1_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_I: libc::c_int = gsl_sf_bessel_I1_scaled_e(
            2.0f64 * root_eta,
            &mut I1_scaled,
        );
        if I1_scaled.val <= 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return if stat_I != GSL_SUCCESS as libc::c_int {
                stat_I
            } else if GSL_EDOM as libc::c_int != GSL_SUCCESS as libc::c_int {
                GSL_EDOM as libc::c_int
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let corr1: libc::c_double = 2.0f64 / 3.0f64 * a
                * pow(x / (4.0f64 * a), 1.5f64)
                * gsl_sf_bessel_In_scaled(2 as libc::c_int, 2.0f64 * root_eta);
            let lnr_val: libc::c_double = 0.5f64 * x + 0.5f64 * log(eta)
                + fabs(2.0f64 * root_eta) + log(I1_scaled.val + corr1);
            let lnr_err: libc::c_double = 2.2204460492503131e-16f64
                * (1.5f64 * fabs(x) + 1.0f64)
                + fabs((I1_scaled.err + corr1) / I1_scaled.val);
            return gsl_sf_exp_err_e(lnr_val, lnr_err, result);
        }
    } else if eta == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut root_eta_0: libc::c_double = sqrt(-eta);
        let mut J1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_J: libc::c_int = gsl_sf_bessel_J1_e(2.0f64 * root_eta_0, &mut J1);
        if J1.val <= 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return if stat_J != GSL_SUCCESS as libc::c_int {
                stat_J
            } else if GSL_EDOM as libc::c_int != GSL_SUCCESS as libc::c_int {
                GSL_EDOM as libc::c_int
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let t1: libc::c_double = 0.5f64 * x;
            let t2: libc::c_double = 0.5f64 * log(-eta);
            let t3: libc::c_double = fabs(x);
            let t4: libc::c_double = log(J1.val);
            let lnr_val_0: libc::c_double = t1 + t2 + t3 + t4;
            let lnr_err_0: libc::c_double = 2.2204460492503131e-16f64
                * (1.5f64 * fabs(x) + 1.0f64) + fabs(J1.err / J1.val);
            let mut ex: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_e: libc::c_int = gsl_sf_exp_err_e(
                lnr_val_0,
                lnr_err_0,
                &mut ex,
            );
            (*result).val = -ex.val;
            (*result).err = ex.err;
            return stat_e;
        }
    };
}
unsafe extern "C" fn hyperg_1F1_CF1_p_ser(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    if a == 0.0f64 {
        *result = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let maxiter: libc::c_int = 5000 as libc::c_int;
        let mut sum: libc::c_double = 1.0f64;
        let mut pk: libc::c_double = 1.0f64;
        let mut rhok: libc::c_double = 0.0f64;
        let mut k: libc::c_int = 0;
        k = 1 as libc::c_int;
        while k < maxiter {
            let mut ak: libc::c_double = (a + k as libc::c_double) * x
                / ((b - x + k as libc::c_double - 1.0f64)
                    * (b - x + k as libc::c_double));
            rhok = -ak * (1.0f64 + rhok) / (1.0f64 + ak * (1.0f64 + rhok));
            pk *= rhok;
            sum += pk;
            if fabs(pk / sum) < 2.0f64 * 2.2204460492503131e-16f64 {
                break;
            }
            k += 1;
            k;
        }
        *result = a / (b - x) * sum;
        if k == maxiter {
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
                580 as libc::c_int,
                GSL_EMAXITER as libc::c_int,
            );
            return GSL_EMAXITER as libc::c_int;
        } else {
            return GSL_SUCCESS as libc::c_int
        }
    };
}
unsafe extern "C" fn hyperg_1F1_small_a_bgt0(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let bma: libc::c_double = b - a;
    let oma: libc::c_double = 1.0f64 - a;
    let ap1mb: libc::c_double = 1.0f64 + a - b;
    let abs_bma: libc::c_double = fabs(bma);
    let abs_oma: libc::c_double = fabs(oma);
    let abs_ap1mb: libc::c_double = fabs(ap1mb);
    let ax: libc::c_double = fabs(x);
    if a == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if a == 1.0f64 && b >= 1.0f64 {
        return hyperg_1F1_1(b, x, result)
    } else if a == -1.0f64 {
        (*result).val = 1.0f64 + a / b * x;
        (*result).err = 2.2204460492503131e-16f64 * (1.0f64 + fabs(a / b * x));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if b >= 1.4f64 * ax {
        return gsl_sf_hyperg_1F1_series_e(a, b, x, result)
    } else if x > 0.0f64 {
        if x > 100.0f64 && abs_bma * abs_oma < 0.5f64 * x {
            return hyperg_1F1_asymp_posx(a, b, x, result)
        } else if b < 5.0e+06f64 {
            let b_del: libc::c_double = ceil(1.4f64 * x - b) + 1.0f64;
            let mut bp: libc::c_double = b + b_del;
            let mut r_Mbp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut r_Mb: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut Mbp1: libc::c_double = 0.;
            let mut Mb: libc::c_double = 0.;
            let mut Mbm1: libc::c_double = 0.;
            let mut stat_0: libc::c_int = gsl_sf_hyperg_1F1_series_e(
                a,
                bp + 1.0f64,
                x,
                &mut r_Mbp1,
            );
            let mut stat_1: libc::c_int = gsl_sf_hyperg_1F1_series_e(
                a,
                bp,
                x,
                &mut r_Mb,
            );
            let err_rat: libc::c_double = fabs(r_Mbp1.err / r_Mbp1.val)
                + fabs(r_Mb.err / r_Mb.val);
            Mbp1 = r_Mbp1.val;
            Mb = r_Mb.val;
            while bp > b + 0.1f64 {
                Mbm1 = ((x + bp - 1.0f64) * Mb - x * (bp - a) / bp * Mbp1)
                    / (bp - 1.0f64);
                bp -= 1.0f64;
                Mbp1 = Mb;
                Mb = Mbm1;
            }
            (*result).val = Mb;
            (*result).err = err_rat * (fabs(b_del) + 1.0f64) * fabs(Mb);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(Mb);
            return if stat_0 != GSL_SUCCESS as libc::c_int {
                stat_0
            } else if stat_1 != GSL_SUCCESS as libc::c_int {
                stat_1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else if fabs(x) < fabs(b) && fabs(a * x) < sqrt(fabs(b)) * fabs(b - x) {
            return hyperg_1F1_largebx(a, b, x, result)
        } else {
            return hyperg_1F1_large2bm4a(a, b, x, result)
        }
    } else if ax < 10.0f64 && b < 10.0f64 {
        return gsl_sf_hyperg_1F1_series_e(a, b, x, result)
    } else if ax >= 100.0f64
        && (if abs_ap1mb > 1.0f64 { abs_ap1mb } else { 1.0f64 }) < 0.99f64 * ax
    {
        return hyperg_1F1_asymp_negx(a, b, x, result)
    } else {
        return hyperg_1F1_luke(a, b, x, result)
    };
}
unsafe extern "C" fn hyperg_1F1_beps_bgt0(
    eps: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if b > fabs(x) && fabs(eps) < 1.4901161193847656e-08f64 {
        let mut a: libc::c_double = b + eps;
        let mut exab: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_e: libc::c_int = gsl_sf_exp_e(a * x / b, &mut exab);
        let mut v2: libc::c_double = a / (2.0f64 * b * b * (b + 1.0f64));
        let mut v3: libc::c_double = a * (b - 2.0f64 * a)
            / (3.0f64 * b * b * b * (b + 1.0f64) * (b + 2.0f64));
        let mut v: libc::c_double = v2 + v3 * x;
        let mut f: libc::c_double = 1.0f64 - eps * x * x * v;
        (*result).val = exab.val * f;
        (*result).err = exab.err * fabs(f);
        (*result).err
            += fabs(exab.val) * 2.2204460492503131e-16f64
                * (1.0f64 + fabs(eps * x * x * v));
        (*result).err += 4.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_e;
    } else {
        let mut Kummer_1F1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K: libc::c_int = hyperg_1F1_small_a_bgt0(
            -eps,
            b,
            -x,
            &mut Kummer_1F1,
        );
        if Kummer_1F1.val != 0.0f64 {
            let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e(
                x,
                2.0f64 * 2.2204460492503131e-16f64 * fabs(x),
                Kummer_1F1.val,
                Kummer_1F1.err,
                result,
            );
            return if stat_e_0 != GSL_SUCCESS as libc::c_int {
                stat_e_0
            } else if stat_K != GSL_SUCCESS as libc::c_int {
                stat_K
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_K;
        }
    };
}
unsafe extern "C" fn hyperg_1F1_beq2a_pos(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut I: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_I: libc::c_int = gsl_sf_bessel_Inu_scaled_e(
            a - 0.5f64,
            0.5f64 * fabs(x),
            &mut I,
        );
        let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_g: libc::c_int = gsl_sf_lngamma_e(a + 0.5f64, &mut lg);
        let mut ln_term: libc::c_double = (0.5f64 - a) * log(0.25f64 * fabs(x));
        let mut lnpre_val: libc::c_double = lg.val + GSL_MAX_DBL(x, 0.0f64) + ln_term;
        let mut lnpre_err: libc::c_double = lg.err
            + 2.2204460492503131e-16f64 * (fabs(ln_term) + fabs(x));
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            lnpre_val,
            lnpre_err,
            I.val,
            I.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_g != GSL_SUCCESS as libc::c_int {
            stat_g
        } else if stat_I != GSL_SUCCESS as libc::c_int {
            stat_I
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
unsafe extern "C" fn hyperg_1F1_ab_posint(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if a == b {
        return gsl_sf_exp_e(x, result)
    } else if a == 1 as libc::c_int {
        return gsl_sf_exprel_n_e(b - 1 as libc::c_int, x, result)
    } else if b == a + 1 as libc::c_int {
        let mut K: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K: libc::c_int = gsl_sf_exprel_n_e(a, -x, &mut K);
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            x,
            2.0f64 * 2.2204460492503131e-16f64 * fabs(x),
            K.val,
            K.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_K != GSL_SUCCESS as libc::c_int {
            stat_K
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if a == b + 1 as libc::c_int {
        let mut ex: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_e_0: libc::c_int = gsl_sf_exp_e(x, &mut ex);
        (*result).val = ex.val * (1.0f64 + x / b as libc::c_double);
        (*result).err = ex.err * (1.0f64 + x / b as libc::c_double);
        (*result).err
            += ex.val * 2.2204460492503131e-16f64
                * (1.0f64 + fabs(x / b as libc::c_double));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_e_0;
    } else if a == b + 2 as libc::c_int {
        let mut ex_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_e_1: libc::c_int = gsl_sf_exp_e(x, &mut ex_0);
        let mut poly: libc::c_double = 1.0f64
            + x / b as libc::c_double * (2.0f64 + x / (b as libc::c_double + 1.0f64));
        (*result).val = ex_0.val * poly;
        (*result).err = ex_0.err * fabs(poly);
        (*result).err
            += ex_0.val * 2.2204460492503131e-16f64
                * (1.0f64
                    + fabs(x / b as libc::c_double)
                        * (2.0f64 + fabs(x / (b as libc::c_double + 1.0f64))));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_e_1;
    } else if b == 2 as libc::c_int * a {
        return hyperg_1F1_beq2a_pos(a as libc::c_double, x, result)
    } else if b < 10 as libc::c_int && a < 10 as libc::c_int && ax < 5.0f64
        || b as libc::c_double > a as libc::c_double * ax || b > a && ax < 5.0f64
    {
        return gsl_sf_hyperg_1F1_series_e(
            a as libc::c_double,
            b as libc::c_double,
            x,
            result,
        )
    } else if b > a
        && b as libc::c_double >= (2 as libc::c_int * a) as libc::c_double + x
    {
        let mut rap: libc::c_double = 0.;
        let mut stat_CF1: libc::c_int = hyperg_1F1_CF1_p_ser(
            a as libc::c_double,
            b as libc::c_double,
            x,
            &mut rap,
        );
        let mut ra: libc::c_double = 1.0f64 + x / a as libc::c_double * rap;
        let mut Ma: libc::c_double = 1.4916681462400413e-154f64;
        let mut Map1: libc::c_double = ra * Ma;
        let mut Mnp1: libc::c_double = Map1;
        let mut Mn: libc::c_double = Ma;
        let mut Mnm1: libc::c_double = 0.;
        let mut n: libc::c_int = 0;
        n = a;
        while n > 0 as libc::c_int {
            Mnm1 = (n as libc::c_double * Mnp1
                - ((2 as libc::c_int * n - b) as libc::c_double + x) * Mn)
                / (b - n) as libc::c_double;
            Mnp1 = Mn;
            Mn = Mnm1;
            n -= 1;
            n;
        }
        (*result).val = Ma / Mn;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(a as libc::c_double) + 1.0f64) * fabs(Ma / Mn);
        return stat_CF1;
    } else if b > a
        && (b as libc::c_double) < (2 as libc::c_int * a) as libc::c_double + x
        && b as libc::c_double > x
    {
        let mut rap_0: libc::c_double = 0.;
        let mut stat_CF1_0: libc::c_int = hyperg_1F1_CF1_p_ser(
            a as libc::c_double,
            b as libc::c_double,
            x,
            &mut rap_0,
        );
        let mut ra_0: libc::c_double = 1.0f64 + x / a as libc::c_double * rap_0;
        let mut ex_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_ex: libc::c_int = 0;
        let mut Ma_0: libc::c_double = 1.4916681462400413e-154f64;
        let mut Map1_0: libc::c_double = ra_0 * Ma_0;
        let mut Mnm1_0: libc::c_double = Ma_0;
        let mut Mn_0: libc::c_double = Map1_0;
        let mut Mnp1_0: libc::c_double = 0.;
        let mut n_0: libc::c_int = 0;
        n_0 = a + 1 as libc::c_int;
        while n_0 < b {
            Mnp1_0 = ((b - n_0) as libc::c_double * Mnm1_0
                + ((2 as libc::c_int * n_0 - b) as libc::c_double + x) * Mn_0)
                / n_0 as libc::c_double;
            Mnm1_0 = Mn_0;
            Mn_0 = Mnp1_0;
            n_0 += 1;
            n_0;
        }
        stat_ex = gsl_sf_exp_e(x, &mut ex_1);
        (*result).val = ex_1.val * Ma_0 / Mn_0;
        (*result).err = ex_1.err * fabs(Ma_0 / Mn_0);
        (*result).err
            += 4.0f64 * 2.2204460492503131e-16f64
                * (fabs((b - a) as libc::c_double) + 1.0f64) * fabs((*result).val);
        return if stat_ex != GSL_SUCCESS as libc::c_int {
            stat_ex
        } else if stat_CF1_0 != GSL_SUCCESS as libc::c_int {
            stat_CF1_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x >= 0.0f64 {
        if b < a {
            if x + log(fabs(x / b as libc::c_double))
                < 7.0978271289338397e+02f64 - 2.0f64
            {
                let mut ex_2: libc::c_double = exp(x);
                let mut n_1: libc::c_int = 0;
                let mut Mnm1_1: libc::c_double = ex_2;
                let mut Mn_1: libc::c_double = ex_2 * (1.0f64 + x / b as libc::c_double);
                let mut Mnp1_1: libc::c_double = 0.;
                n_1 = b + 1 as libc::c_int;
                while n_1 < a {
                    Mnp1_1 = ((b - n_1) as libc::c_double * Mnm1_1
                        + ((2 as libc::c_int * n_1 - b) as libc::c_double + x) * Mn_1)
                        / n_1 as libc::c_double;
                    Mnm1_1 = Mn_1;
                    Mn_1 = Mnp1_1;
                    n_1 += 1;
                    n_1;
                }
                (*result).val = Mn_1;
                (*result).err = (x + 1.0f64) * 2.2204460492503131e-16f64 * fabs(Mn_1);
                (*result).err *= fabs((a - b) as libc::c_double) + 1.0f64;
                return GSL_SUCCESS as libc::c_int;
            } else {
                (*result).val = ::std::f32::INFINITY as libc::c_double;
                (*result).err = ::std::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const libc::c_char,
                    b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
                    1062 as libc::c_int,
                    GSL_EOVRFLW as libc::c_int,
                );
                return GSL_EOVRFLW as libc::c_int;
            }
        } else {
            let mut r_Mn: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut Mnm1_2: libc::c_double = 1.0f64;
            let mut Mn_2: libc::c_double = 0.;
            let mut Mnp1_2: libc::c_double = 0.;
            let mut n_2: libc::c_int = 0;
            gsl_sf_exprel_n_e(b - 1 as libc::c_int, x, &mut r_Mn);
            Mn_2 = r_Mn.val;
            n_2 = 1 as libc::c_int;
            while n_2 < a {
                Mnp1_2 = ((b - n_2) as libc::c_double * Mnm1_2
                    + ((2 as libc::c_int * n_2 - b) as libc::c_double + x) * Mn_2)
                    / n_2 as libc::c_double;
                Mnm1_2 = Mn_2;
                Mn_2 = Mnp1_2;
                n_2 += 1;
                n_2;
            }
            (*result).val = Mn_2;
            (*result)
                .err = fabs(Mn_2) * (1.0f64 + fabs(a as libc::c_double))
                * fabs(r_Mn.err / r_Mn.val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(Mn_2);
            return GSL_SUCCESS as libc::c_int;
        }
    } else if a as libc::c_double <= 0.5f64 * (b as libc::c_double - x)
        || a as libc::c_double >= -x
    {
        let mut ex_3: libc::c_double = exp(x);
        let mut Manp1: libc::c_double = ex_3;
        let mut Man: libc::c_double = ex_3
            * (1.0f64 + x / (a as libc::c_double - 1.0f64));
        let mut Manm1: libc::c_double = 0.;
        let mut n_3: libc::c_int = 0;
        n_3 = a - 1 as libc::c_int;
        while n_3 > b {
            Manm1 = (-n_3 as libc::c_double
                * ((1 as libc::c_int - n_3) as libc::c_double - x) * Man
                - x * (n_3 - a) as libc::c_double * Manp1)
                / (n_3 as libc::c_double * (n_3 as libc::c_double - 1.0f64));
            Manp1 = Man;
            Man = Manm1;
            n_3 -= 1;
            n_3;
        }
        (*result).val = Man;
        (*result).err = (fabs(x) + 1.0f64) * 2.2204460492503131e-16f64 * fabs(Man);
        (*result).err *= fabs((b - a) as libc::c_double) + 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut a0: libc::c_int = ceil(0.5f64 * (b as libc::c_double - x))
            as libc::c_int;
        let mut Ma0b: libc::c_double = 0.;
        let mut Ma0bp1: libc::c_double = 0.;
        let mut Ma0p1b: libc::c_double = 0.;
        let mut Mnm1_3: libc::c_double = 0.;
        let mut Mn_3: libc::c_double = 0.;
        let mut Mnp1_3: libc::c_double = 0.;
        let mut n_4: libc::c_int = 0;
        let mut ex_4: libc::c_double = exp(x);
        let mut Ma0np1: libc::c_double = ex_4;
        let mut Ma0n: libc::c_double = ex_4
            * (1.0f64 + x / (a0 as libc::c_double - 1.0f64));
        let mut Ma0nm1: libc::c_double = 0.;
        n_4 = a0 - 1 as libc::c_int;
        while n_4 > b {
            Ma0nm1 = (-n_4 as libc::c_double
                * ((1 as libc::c_int - n_4) as libc::c_double - x) * Ma0n
                - x * (n_4 - a0) as libc::c_double * Ma0np1)
                / (n_4 as libc::c_double * (n_4 as libc::c_double - 1.0f64));
            Ma0np1 = Ma0n;
            Ma0n = Ma0nm1;
            n_4 -= 1;
            n_4;
        }
        Ma0bp1 = Ma0np1;
        Ma0b = Ma0n;
        Ma0p1b = (b as libc::c_double * (a0 as libc::c_double + x) * Ma0b
            + x * (a0 - b) as libc::c_double * Ma0bp1) / (a0 * b) as libc::c_double;
        if a0 >= a {
            Mn_3 = Ma0b;
        } else if a0 + 1 as libc::c_int >= a {
            Mn_3 = Ma0p1b;
        } else {
            Mnm1_3 = Ma0b;
            Mn_3 = Ma0p1b;
            n_4 = a0 + 1 as libc::c_int;
            while n_4 < a {
                Mnp1_3 = ((b - n_4) as libc::c_double * Mnm1_3
                    + ((2 as libc::c_int * n_4 - b) as libc::c_double + x) * Mn_3)
                    / n_4 as libc::c_double;
                Mnm1_3 = Mn_3;
                Mn_3 = Mnp1_3;
                n_4 += 1;
                n_4;
            }
        }
        (*result).val = Mn_3;
        (*result).err = (fabs(x) + 1.0f64) * 2.2204460492503131e-16f64 * fabs(Mn_3);
        (*result).err *= fabs((b - a) as libc::c_double) + 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn hyperg_1F1_a_negint_poly(
    a: libc::c_int,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut N: libc::c_int = -a;
        let mut poly: libc::c_double = 1.0f64;
        let mut k: libc::c_int = 0;
        k = N - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            let mut t: libc::c_double = (a + k) as libc::c_double
                / (b + k as libc::c_double)
                * (x / (k + 1 as libc::c_int) as libc::c_double);
            let mut r: libc::c_double = t + 1.0f64 / poly;
            if r > 0.9f64 * 1.7976931348623157e+308f64 / poly {
                (*result).val = ::std::f32::INFINITY as libc::c_double;
                (*result).err = ::std::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const libc::c_char,
                    b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
                    1198 as libc::c_int,
                    GSL_EOVRFLW as libc::c_int,
                );
                return GSL_EOVRFLW as libc::c_int;
            } else {
                poly *= r;
            }
            k -= 1;
            k;
        }
        (*result).val = poly;
        (*result)
            .err = 2.0f64 * (sqrt(N as libc::c_double) + 1.0f64)
            * 2.2204460492503131e-16f64 * fabs(poly);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn hyperg_1F1_a_negint_lag(
    a: libc::c_int,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let n: libc::c_int = -a;
    let mut lag: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let stat_l: libc::c_int = gsl_sf_laguerre_n_e(n, b - 1.0f64, x, &mut lag);
    if b < 0.0f64 {
        let mut lnfact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lng1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lng2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut s1: libc::c_double = 0.;
        let mut s2: libc::c_double = 0.;
        let stat_f: libc::c_int = gsl_sf_lnfact_e(n as libc::c_uint, &mut lnfact);
        let stat_g1: libc::c_int = gsl_sf_lngamma_sgn_e(
            b + n as libc::c_double,
            &mut lng1,
            &mut s1,
        );
        let stat_g2: libc::c_int = gsl_sf_lngamma_sgn_e(b, &mut lng2, &mut s2);
        let lnpre_val: libc::c_double = lnfact.val - (lng1.val - lng2.val);
        let lnpre_err: libc::c_double = lnfact.err + lng1.err + lng2.err
            + 2.0f64 * 2.2204460492503131e-16f64 * fabs(lnpre_val);
        let stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            lnpre_val,
            lnpre_err,
            s1 * s2 * lag.val,
            lag.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_l != GSL_SUCCESS as libc::c_int {
            stat_l
        } else if stat_g1 != GSL_SUCCESS as libc::c_int {
            stat_g1
        } else if stat_g2 != GSL_SUCCESS as libc::c_int {
            stat_g2
        } else if stat_f != GSL_SUCCESS as libc::c_int {
            stat_f
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        let mut lnbeta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lnbeta_e(b, n as libc::c_double, &mut lnbeta);
        if fabs(lnbeta.val) < 0.1f64 {
            let ln_term_val: libc::c_double = log(1.25f64 * n as libc::c_double);
            let ln_term_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
                * ln_term_val;
            let mut beta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_b: libc::c_int = gsl_sf_beta_e(
                b,
                n as libc::c_double,
                &mut beta,
            );
            let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e(
                ln_term_val,
                ln_term_err,
                lag.val,
                lag.err,
                result,
            );
            (*result).val *= beta.val / 1.25f64;
            (*result).err *= beta.val / 1.25f64;
            return if stat_e_0 != GSL_SUCCESS as libc::c_int {
                stat_e_0
            } else if stat_l != GSL_SUCCESS as libc::c_int {
                stat_l
            } else if stat_b != GSL_SUCCESS as libc::c_int {
                stat_b
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let ln_n: libc::c_double = log(n as libc::c_double);
            let ln_term_val_0: libc::c_double = lnbeta.val + ln_n;
            let ln_term_err_0: libc::c_double = lnbeta.err
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs(ln_n);
            let mut stat_e_1: libc::c_int = gsl_sf_exp_mult_err_e(
                ln_term_val_0,
                ln_term_err_0,
                lag.val,
                lag.err,
                result,
            );
            return if stat_e_1 != GSL_SUCCESS as libc::c_int {
                stat_e_1
            } else if stat_l != GSL_SUCCESS as libc::c_int {
                stat_l
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
unsafe extern "C" fn hyperg_1F1_ab_negint(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x > 0.0f64 {
        return hyperg_1F1_a_negint_poly(a, b as libc::c_double, x, result)
    } else {
        let mut K: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K: libc::c_int = hyperg_1F1_a_negint_poly(
            b - a,
            b as libc::c_double,
            -x,
            &mut K,
        );
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            x,
            2.0f64 * 2.2204460492503131e-16f64 * fabs(x),
            K.val,
            K.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_K != GSL_SUCCESS as libc::c_int {
            stat_K
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
unsafe extern "C" fn hyperg_1F1_U(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let bp: libc::c_double = 2.0f64 - b;
    let ap: libc::c_double = a - b + 1.0f64;
    let mut lg_ap: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lg_bp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut sg_ap: libc::c_double = 0.;
    let mut stat_lg0: libc::c_int = gsl_sf_lngamma_sgn_e(ap, &mut lg_ap, &mut sg_ap);
    let mut stat_lg1: libc::c_int = gsl_sf_lngamma_e(bp, &mut lg_bp);
    let mut stat_lg2: libc::c_int = if stat_lg0 != GSL_SUCCESS as libc::c_int {
        stat_lg0
    } else if stat_lg1 != GSL_SUCCESS as libc::c_int {
        stat_lg1
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut t1: libc::c_double = (bp - 1.0f64) * log(x);
    let mut lnpre_val: libc::c_double = lg_ap.val - lg_bp.val + t1;
    let mut lnpre_err: libc::c_double = lg_ap.err + lg_bp.err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(t1);
    let mut lg_2mbp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut lg_1papmbp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut sg_2mbp: libc::c_double = 0.;
    let mut sg_1papmbp: libc::c_double = 0.;
    let mut stat_lg3: libc::c_int = gsl_sf_lngamma_sgn_e(
        2.0f64 - bp,
        &mut lg_2mbp,
        &mut sg_2mbp,
    );
    let mut stat_lg4: libc::c_int = gsl_sf_lngamma_sgn_e(
        1.0f64 + ap - bp,
        &mut lg_1papmbp,
        &mut sg_1papmbp,
    );
    let mut stat_lg5: libc::c_int = if stat_lg3 != GSL_SUCCESS as libc::c_int {
        stat_lg3
    } else if stat_lg4 != GSL_SUCCESS as libc::c_int {
        stat_lg4
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut lnc1_val: libc::c_double = lg_2mbp.val - lg_1papmbp.val;
    let mut lnc1_err: libc::c_double = lg_2mbp.err + lg_1papmbp.err
        + 2.2204460492503131e-16f64 * (fabs(lg_2mbp.val) + fabs(lg_1papmbp.val));
    let mut M: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut U: gsl_sf_result_e10 = gsl_sf_result_e10 {
        val: 0.,
        err: 0.,
        e10: 0,
    };
    let mut stat_F: libc::c_int = gsl_sf_hyperg_1F1_e(ap, bp, x, &mut M);
    let mut stat_U: libc::c_int = gsl_sf_hyperg_U_e10_e(ap, bp, x, &mut U);
    let mut stat_FU: libc::c_int = if stat_F != GSL_SUCCESS as libc::c_int {
        stat_F
    } else if stat_U != GSL_SUCCESS as libc::c_int {
        stat_U
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut term_M: gsl_sf_result_e10 = gsl_sf_result_e10 {
        val: 0.,
        err: 0.,
        e10: 0,
    };
    let mut stat_e0: libc::c_int = gsl_sf_exp_mult_err_e10_e(
        lnc1_val,
        lnc1_err,
        sg_2mbp * sg_1papmbp * M.val,
        M.err,
        &mut term_M,
    );
    let ombp: libc::c_double = 1.0f64 - bp;
    let Uee_val: libc::c_double = U.e10 as libc::c_double * 2.30258509299404568402f64;
    let Uee_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64 * fabs(Uee_val);
    let Mee_val: libc::c_double = term_M.e10 as libc::c_double
        * 2.30258509299404568402f64;
    let Mee_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64 * fabs(Mee_val);
    let mut stat_e1: libc::c_int = 0;
    if Uee_val > Mee_val {
        let factorM_val: libc::c_double = exp(Mee_val - Uee_val);
        let factorM_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(Mee_val - Uee_val) + 1.0f64) * factorM_val;
        let inner_val: libc::c_double = term_M.val * factorM_val - ombp * U.val;
        let inner_err: libc::c_double = term_M.err * factorM_val + fabs(ombp) * U.err
            + fabs(term_M.val) * factorM_err
            + 2.2204460492503131e-16f64
                * (fabs(term_M.val * factorM_val) + fabs(ombp * U.val));
        stat_e1 = gsl_sf_exp_mult_err_e(
            lnpre_val + Uee_val,
            lnpre_err + Uee_err,
            sg_ap * inner_val,
            inner_err,
            result,
        );
    } else {
        let factorU_val: libc::c_double = exp(Uee_val - Mee_val);
        let factorU_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(Mee_val - Uee_val) + 1.0f64) * factorU_val;
        let inner_val_0: libc::c_double = term_M.val - ombp * factorU_val * U.val;
        let inner_err_0: libc::c_double = term_M.err + fabs(ombp * factorU_val * U.err)
            + fabs(ombp * factorU_err * U.val)
            + 2.2204460492503131e-16f64
                * (fabs(term_M.val) + fabs(ombp * factorU_val * U.val));
        stat_e1 = gsl_sf_exp_mult_err_e(
            lnpre_val + Mee_val,
            lnpre_err + Mee_err,
            sg_ap * inner_val_0,
            inner_err_0,
            result,
        );
    }
    return if stat_e1 != GSL_SUCCESS as libc::c_int {
        stat_e1
    } else if stat_e0 != GSL_SUCCESS as libc::c_int {
        stat_e0
    } else if stat_FU != GSL_SUCCESS as libc::c_int {
        stat_FU
    } else if stat_lg5 != GSL_SUCCESS as libc::c_int {
        stat_lg5
    } else if stat_lg2 != GSL_SUCCESS as libc::c_int {
        stat_lg2
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn hyperg_1F1_ab_pos(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    if b < 10.0f64 && a < 10.0f64 && ax < 5.0f64 || b > a * ax || b > a && ax < 5.0f64 {
        return gsl_sf_hyperg_1F1_series_e(a, b, x, result)
    } else if x < -100.0f64
        && GSL_MAX_DBL(fabs(a), 1.0f64) * GSL_MAX_DBL(fabs(1.0f64 + a - b), 1.0f64)
            < 0.7f64 * fabs(x)
    {
        return hyperg_1F1_asymp_negx(a, b, x, result)
    } else if x > 100.0f64
        && GSL_MAX_DBL(fabs(b - a), 1.0f64) * GSL_MAX_DBL(fabs(1.0f64 - a), 1.0f64)
            < 0.7f64 * fabs(x)
    {
        return hyperg_1F1_asymp_posx(a, b, x, result)
    } else if fabs(b - a) <= 1.0f64 {
        return hyperg_1F1_beps_bgt0(a - b, b, x, result)
    } else if b > a && b >= 2 as libc::c_int as libc::c_double * a + x {
        let mut rap: libc::c_double = 0.;
        let mut stat_CF1: libc::c_int = hyperg_1F1_CF1_p_ser(a, b, x, &mut rap);
        let mut ra: libc::c_double = 1.0f64 + x / a * rap;
        let mut Ma: libc::c_double = 1.4916681462400413e-154f64;
        let mut Map1: libc::c_double = ra * Ma;
        let mut Mnp1: libc::c_double = Map1;
        let mut Mn: libc::c_double = Ma;
        let mut Mnm1: libc::c_double = 0.;
        let mut Mn_true: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_Mt: libc::c_int = 0;
        let mut n: libc::c_double = 0.;
        n = a;
        while n > 0.5f64 {
            Mnm1 = (n * Mnp1 - (2.0f64 * n - b + x) * Mn) / (b - n);
            Mnp1 = Mn;
            Mn = Mnm1;
            n -= 1.0f64;
        }
        stat_Mt = hyperg_1F1_small_a_bgt0(n, b, x, &mut Mn_true);
        (*result).val = Ma / Mn * Mn_true.val;
        (*result).err = fabs(Ma / Mn) * Mn_true.err;
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(a) + 1.0f64)
                * fabs((*result).val);
        return if stat_Mt != GSL_SUCCESS as libc::c_int {
            stat_Mt
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if b > a && b < 2 as libc::c_int as libc::c_double * a + x && b > x {
        let mut Mn_true_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_Mt_0: libc::c_int = 0;
        let mut rap_0: libc::c_double = 0.;
        let mut stat_CF1_0: libc::c_int = hyperg_1F1_CF1_p_ser(a, b, x, &mut rap_0);
        let mut ra_0: libc::c_double = 1.0f64 + x / a * rap_0;
        let mut Ma_0: libc::c_double = 1.4916681462400413e-154f64;
        let mut Mnm1_0: libc::c_double = Ma_0;
        let mut Mn_0: libc::c_double = ra_0 * Mnm1_0;
        let mut Mnp1_0: libc::c_double = 0.;
        let mut n_0: libc::c_double = 0.;
        n_0 = a + 1.0f64;
        while n_0 < b - 0.5f64 {
            Mnp1_0 = ((b - n_0) * Mnm1_0
                + (2 as libc::c_int as libc::c_double * n_0 - b + x) * Mn_0) / n_0;
            Mnm1_0 = Mn_0;
            Mn_0 = Mnp1_0;
            n_0 += 1.0f64;
        }
        stat_Mt_0 = hyperg_1F1_beps_bgt0(n_0 - b, b, x, &mut Mn_true_0);
        (*result).val = Ma_0 / Mn_0 * Mn_true_0.val;
        (*result).err = fabs(Ma_0 / Mn_0) * Mn_true_0.err;
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(b - a) + 1.0f64)
                * fabs((*result).val);
        return if stat_Mt_0 != GSL_SUCCESS as libc::c_int {
            stat_Mt_0
        } else if stat_CF1_0 != GSL_SUCCESS as libc::c_int {
            stat_CF1_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x >= 0.0f64 {
        if b < a {
            let mut N: libc::c_double = floor(a - b);
            let mut eps: libc::c_double = a - b - N;
            let mut r_M0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut r_M1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_0: libc::c_int = hyperg_1F1_beps_bgt0(
                eps - 1.0f64,
                b,
                x,
                &mut r_M0,
            );
            let mut stat_1: libc::c_int = hyperg_1F1_beps_bgt0(eps, b, x, &mut r_M1);
            let mut M0: libc::c_double = r_M0.val;
            let mut M1: libc::c_double = r_M1.val;
            let mut Mam1: libc::c_double = M0;
            let mut Ma_1: libc::c_double = M1;
            let mut Map1_0: libc::c_double = 0.;
            let mut ap: libc::c_double = 0.;
            let mut start_pair: libc::c_double = fabs(M0) + fabs(M1);
            let mut minim_pair: libc::c_double = 1.7976931348623157e+308f64;
            let mut pair_ratio: libc::c_double = 0.;
            let mut rat_0: libc::c_double = fabs(r_M0.err / r_M0.val);
            let mut rat_1: libc::c_double = fabs(r_M1.err / r_M1.val);
            ap = b + eps;
            while ap < a - 0.1f64 {
                Map1_0 = ((b - ap) * Mam1 + (2.0f64 * ap - b + x) * Ma_1) / ap;
                Mam1 = Ma_1;
                Ma_1 = Map1_0;
                minim_pair = GSL_MIN_DBL(fabs(Mam1) + fabs(Ma_1), minim_pair);
                ap += 1.0f64;
            }
            pair_ratio = start_pair / minim_pair;
            (*result).val = Ma_1;
            (*result)
                .err = 2.0f64 * (rat_0 + rat_1 + 2.2204460492503131e-16f64)
                * (fabs(b - a) + 1.0f64) * fabs(Ma_1);
            (*result).err
                += 2.0f64 * (rat_0 + rat_1) * pair_ratio * pair_ratio * fabs(Ma_1);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(Ma_1);
            return if stat_0 != GSL_SUCCESS as libc::c_int {
                stat_0
            } else if stat_1 != GSL_SUCCESS as libc::c_int {
                stat_1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            let mut eps_0: libc::c_double = a - floor(a);
            let mut r_Mnm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut r_Mn: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_0_0: libc::c_int = hyperg_1F1_small_a_bgt0(
                eps_0,
                b,
                x,
                &mut r_Mnm1,
            );
            let mut stat_1_0: libc::c_int = hyperg_1F1_small_a_bgt0(
                eps_0 + 1.0f64,
                b,
                x,
                &mut r_Mn,
            );
            let mut Mnm1_1: libc::c_double = r_Mnm1.val;
            let mut Mn_1: libc::c_double = r_Mn.val;
            let mut Mnp1_1: libc::c_double = 0.;
            let mut n_1: libc::c_double = 0.;
            let mut start_pair_0: libc::c_double = fabs(Mn_1) + fabs(Mnm1_1);
            let mut minim_pair_0: libc::c_double = 1.7976931348623157e+308f64;
            let mut pair_ratio_0: libc::c_double = 0.;
            let mut rat_0_0: libc::c_double = fabs(r_Mnm1.err / r_Mnm1.val);
            let mut rat_1_0: libc::c_double = fabs(r_Mn.err / r_Mn.val);
            n_1 = eps_0 + 1.0f64;
            while n_1 < a - 0.1f64 {
                Mnp1_1 = ((b - n_1) * Mnm1_1
                    + (2 as libc::c_int as libc::c_double * n_1 - b + x) * Mn_1) / n_1;
                Mnm1_1 = Mn_1;
                Mn_1 = Mnp1_1;
                minim_pair_0 = GSL_MIN_DBL(fabs(Mn_1) + fabs(Mnm1_1), minim_pair_0);
                n_1 += 1.;
                n_1;
            }
            pair_ratio_0 = start_pair_0 / minim_pair_0;
            (*result).val = Mn_1;
            (*result)
                .err = 2.0f64 * (rat_0_0 + rat_1_0 + 2.2204460492503131e-16f64)
                * (fabs(a) + 1.0f64) * fabs(Mn_1);
            (*result).err
                += 2.0f64 * (rat_0_0 + rat_1_0) * pair_ratio_0 * pair_ratio_0
                    * fabs(Mn_1);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(Mn_1);
            return if stat_0_0 != GSL_SUCCESS as libc::c_int {
                stat_0_0
            } else if stat_1_0 != GSL_SUCCESS as libc::c_int {
                stat_1_0
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    } else if a <= 0.5f64 * (b - x) || a >= -x {
        let mut N_0: libc::c_double = floor(a - b);
        let mut eps_1: libc::c_double = 1.0f64 + N_0 - a + b;
        let mut r_Manp1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Man: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0_1: libc::c_int = hyperg_1F1_beps_bgt0(
            -eps_1,
            a + eps_1,
            x,
            &mut r_Manp1,
        );
        let mut stat_1_1: libc::c_int = hyperg_1F1_beps_bgt0(
            1.0f64 - eps_1,
            a + eps_1 - 1.0f64,
            x,
            &mut r_Man,
        );
        let mut Manp1: libc::c_double = r_Manp1.val;
        let mut Man: libc::c_double = r_Man.val;
        let mut Manm1: libc::c_double = 0.;
        let mut n_2: libc::c_double = 0.;
        let mut start_pair_1: libc::c_double = fabs(Manp1) + fabs(Man);
        let mut minim_pair_1: libc::c_double = 1.7976931348623157e+308f64;
        let mut pair_ratio_1: libc::c_double = 0.;
        let mut rat_0_1: libc::c_double = fabs(r_Manp1.err / r_Manp1.val);
        let mut rat_1_1: libc::c_double = fabs(r_Man.err / r_Man.val);
        n_2 = a + eps_1 - 1.0f64;
        while n_2 > b + 0.1f64 {
            Manm1 = (-n_2 * (1 as libc::c_int as libc::c_double - n_2 - x) * Man
                - x * (n_2 - a) * Manp1) / (n_2 * (n_2 - 1.0f64));
            Manp1 = Man;
            Man = Manm1;
            minim_pair_1 = GSL_MIN_DBL(fabs(Manp1) + fabs(Man), minim_pair_1);
            n_2 -= 1.0f64;
        }
        pair_ratio_1 = start_pair_1 / minim_pair_1;
        (*result).val = Man;
        (*result)
            .err = 2.0f64 * (rat_0_1 + rat_1_1 + 2.2204460492503131e-16f64)
            * (fabs(b - a) + 1.0f64) * fabs(Man);
        (*result).err *= pair_ratio_1 * pair_ratio_1 + 1.0f64;
        return if stat_0_1 != GSL_SUCCESS as libc::c_int {
            stat_0_1
        } else if stat_1_1 != GSL_SUCCESS as libc::c_int {
            stat_1_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        let mut epsa: libc::c_double = a - floor(a);
        let mut a0: libc::c_double = floor(0.5f64 * (b - x)) + epsa;
        let mut N_1: libc::c_double = floor(a0 - b);
        let mut epsb: libc::c_double = 1.0f64 + N_1 - a0 + b;
        let mut Ma0b: libc::c_double = 0.;
        let mut Ma0bp1: libc::c_double = 0.;
        let mut Ma0p1b: libc::c_double = 0.;
        let mut stat_a0: libc::c_int = 0;
        let mut Mnm1_2: libc::c_double = 0.;
        let mut Mn_2: libc::c_double = 0.;
        let mut Mnp1_2: libc::c_double = 0.;
        let mut n_3: libc::c_double = 0.;
        let mut err_rat: libc::c_double = 0.;
        let mut r_Ma0np1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Ma0n: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0_2: libc::c_int = hyperg_1F1_beps_bgt0(
            -epsb,
            a0 + epsb,
            x,
            &mut r_Ma0np1,
        );
        let mut stat_1_2: libc::c_int = hyperg_1F1_beps_bgt0(
            1.0f64 - epsb,
            a0 + epsb - 1.0f64,
            x,
            &mut r_Ma0n,
        );
        let mut Ma0np1: libc::c_double = r_Ma0np1.val;
        let mut Ma0n: libc::c_double = r_Ma0n.val;
        let mut Ma0nm1: libc::c_double = 0.;
        err_rat = fabs(r_Ma0np1.err / r_Ma0np1.val) + fabs(r_Ma0n.err / r_Ma0n.val);
        n_3 = a0 + epsb - 1.0f64;
        while n_3 > b + 0.1f64 {
            Ma0nm1 = (-n_3 * (1 as libc::c_int as libc::c_double - n_3 - x) * Ma0n
                - x * (n_3 - a0) * Ma0np1) / (n_3 * (n_3 - 1.0f64));
            Ma0np1 = Ma0n;
            Ma0n = Ma0nm1;
            n_3 -= 1.0f64;
        }
        Ma0bp1 = Ma0np1;
        Ma0b = Ma0n;
        Ma0p1b = (b * (a0 + x) * Ma0b + x * (a0 - b) * Ma0bp1) / (a0 * b);
        stat_a0 = if stat_0_2 != GSL_SUCCESS as libc::c_int {
            stat_0_2
        } else if stat_1_2 != GSL_SUCCESS as libc::c_int {
            stat_1_2
        } else {
            GSL_SUCCESS as libc::c_int
        };
        if a0 >= a - 0.1f64 {
            Mn_2 = Ma0b;
        } else if a0 + 1 as libc::c_int as libc::c_double >= a - 0.1f64 {
            Mn_2 = Ma0p1b;
        } else {
            Mnm1_2 = Ma0b;
            Mn_2 = Ma0p1b;
            n_3 = a0 + 1.0f64;
            while n_3 < a - 0.1f64 {
                Mnp1_2 = ((b - n_3) * Mnm1_2
                    + (2 as libc::c_int as libc::c_double * n_3 - b + x) * Mn_2) / n_3;
                Mnm1_2 = Mn_2;
                Mn_2 = Mnp1_2;
                n_3 += 1.0f64;
            }
        }
        (*result).val = Mn_2;
        (*result)
            .err = (err_rat + 2.2204460492503131e-16f64) * (fabs(b - a) + 1.0f64)
            * fabs(Mn_2);
        return stat_a0;
    };
}
unsafe extern "C" fn hyperg_1F1_ab_neg(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let bma: libc::c_double = b - a;
    let abs_x: libc::c_double = fabs(x);
    let abs_a: libc::c_double = fabs(a);
    let abs_b: libc::c_double = fabs(b);
    let size_a: libc::c_double = if abs_a > 1.0f64 { abs_a } else { 1.0f64 };
    let size_b: libc::c_double = if abs_b > 1.0f64 { abs_b } else { 1.0f64 };
    let bma_integer: libc::c_int = (bma - floor(bma + 0.5f64)
        < 100.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if abs_a < 10.0f64 && abs_b < 10.0f64 && abs_x < 5.0f64
        || b > 0.8f64 * (if fabs(a) > 1.0f64 { fabs(a) } else { 1.0f64 }) * fabs(x)
    {
        return gsl_sf_hyperg_1F1_series_e(a, b, x, result)
    } else if x > 0.0f64 && size_b > size_a
        && size_a * log(2.7182818284590452354f64 * x / size_b)
            < -3.6043653389117154e+01f64 + 7.0f64
    {
        return gsl_sf_hyperg_1F1_series_e(a, b, x, result)
    } else if abs_x < 5.0f64 && fabs(bma) < 10.0f64 && abs_b < 10.0f64
        || b > 0.8f64 * GSL_MAX_DBL(fabs(bma), 1.0f64) * abs_x
    {
        let mut Kummer_1F1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K: libc::c_int = gsl_sf_hyperg_1F1_series_e(
            bma,
            b,
            -x,
            &mut Kummer_1F1,
        );
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            x,
            2.2204460492503131e-16f64 * fabs(x),
            Kummer_1F1.val,
            Kummer_1F1.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_K != GSL_SUCCESS as libc::c_int {
            stat_K
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x < -30.0f64
        && GSL_MAX_DBL(fabs(a), 1.0f64) * GSL_MAX_DBL(fabs(1.0f64 + a - b), 1.0f64)
            < 0.99f64 * fabs(x)
    {
        return hyperg_1F1_asymp_negx(a, b, x, result)
    } else if x > 100.0f64
        && GSL_MAX_DBL(fabs(bma), 1.0f64) * GSL_MAX_DBL(fabs(1.0f64 - a), 1.0f64)
            < 0.99f64 * fabs(x)
    {
        return hyperg_1F1_asymp_posx(a, b, x, result)
    } else if x > 0.0f64 && !(bma_integer != 0 && bma > 0.0f64) {
        return hyperg_1F1_U(a, b, x, result)
    } else if x < 0.0f64 {
        let mut status: libc::c_int = gsl_sf_hyperg_1F1_series_e(b - a, b, -x, result);
        let mut K_factor: libc::c_double = exp(x);
        (*result).val *= K_factor;
        (*result).err *= K_factor;
        return status;
    } else {
        let mut status_0: libc::c_int = gsl_sf_hyperg_1F1_series_e(a, b, x, result);
        return status_0;
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_1F1_int_e(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if a == b {
        return gsl_sf_exp_e(x, result)
    } else if b == 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            1803 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if a == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if b < 0 as libc::c_int && (a < b || a > 0 as libc::c_int) {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            1812 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x > 100.0f64
        && GSL_MAX_DBL(1.0f64, fabs((b - a) as libc::c_double))
            * GSL_MAX_DBL(1.0f64, fabs((1 as libc::c_int - a) as libc::c_double))
            < 0.5f64 * x
    {
        return hyperg_1F1_asymp_posx(a as libc::c_double, b as libc::c_double, x, result)
    } else if x < -100.0f64
        && GSL_MAX_DBL(1.0f64, fabs(a as libc::c_double))
            * GSL_MAX_DBL(1.0f64, fabs((1 as libc::c_int + a - b) as libc::c_double))
            < 0.5f64 * fabs(x)
    {
        return hyperg_1F1_asymp_negx(a as libc::c_double, b as libc::c_double, x, result)
    } else if a < 0 as libc::c_int && b < 0 as libc::c_int {
        return hyperg_1F1_ab_negint(a, b, x, result)
    } else if a < 0 as libc::c_int && b > 0 as libc::c_int {
        let mut Kummer_1F1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K: libc::c_int = hyperg_1F1_ab_posint(
            b - a,
            b,
            -x,
            &mut Kummer_1F1,
        );
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            x,
            2.2204460492503131e-16f64 * fabs(x),
            Kummer_1F1.val,
            Kummer_1F1.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_K != GSL_SUCCESS as libc::c_int {
            stat_K
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        return hyperg_1F1_ab_posint(a, b, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_1F1_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let bma: libc::c_double = b - a;
    let rinta: libc::c_double = floor(a + 0.5f64);
    let rintb: libc::c_double = floor(b + 0.5f64);
    let rintbma: libc::c_double = floor(bma + 0.5f64);
    let a_integer: libc::c_int = (fabs(a - rinta) < 100.0f64 * 2.2204460492503131e-16f64
        && rinta > (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        && rinta < 2147483647 as libc::c_int as libc::c_double) as libc::c_int;
    let b_integer: libc::c_int = (fabs(b - rintb) < 100.0f64 * 2.2204460492503131e-16f64
        && rintb > (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        && rintb < 2147483647 as libc::c_int as libc::c_double) as libc::c_int;
    let bma_integer: libc::c_int = (fabs(bma - rintbma)
        < 100.0f64 * 2.2204460492503131e-16f64
        && rintbma > (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        && rintbma < 2147483647 as libc::c_int as libc::c_double) as libc::c_int;
    let b_neg_integer: libc::c_int = (b < -0.1f64 && b_integer != 0) as libc::c_int;
    let a_neg_integer: libc::c_int = (a < -0.1f64 && a_integer != 0) as libc::c_int;
    let bma_neg_integer: libc::c_int = (bma < -0.1f64 && bma_integer != 0)
        as libc::c_int;
    if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if b == 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            1871 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if a == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if a == b {
        return gsl_sf_exp_e(x, result)
    } else if fabs(b) < 100.0f64 * 2.2204460492503131e-16f64
        && fabs(a) < 100.0f64 * 2.2204460492503131e-16f64
    {
        let mut exm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_e: libc::c_int = gsl_sf_expm1_e(x, &mut exm1);
        let mut sa: libc::c_double = if a > 0.0f64 { 1.0f64 } else { -1.0f64 };
        let mut sb: libc::c_double = if b > 0.0f64 { 1.0f64 } else { -1.0f64 };
        let mut lnab: libc::c_double = log(fabs(a / b));
        let mut hx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_hx: libc::c_int = gsl_sf_exp_mult_err_e(
            lnab,
            2.2204460492503131e-16f64 * fabs(lnab),
            sa * sb * exm1.val,
            exm1.err,
            &mut hx,
        );
        (*result)
            .val = if hx.val == 1.7976931348623157e+308f64 {
            hx.val
        } else {
            1.0f64 + hx.val
        };
        (*result).err = hx.err;
        return if stat_hx != GSL_SUCCESS as libc::c_int {
            stat_hx
        } else if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if fabs(b) < 100.0f64 * 2.2204460492503131e-16f64
        && fabs(x * a) < 1 as libc::c_int as libc::c_double
    {
        let m_arg: libc::c_double = 1.0f64 / (0.5f64 * b);
        let mut F_renorm: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_F: libc::c_int = hyperg_1F1_renorm_b0(a, x, &mut F_renorm);
        let mut stat_m: libc::c_int = gsl_sf_multiply_err_e(
            m_arg,
            2.0f64 * 2.2204460492503131e-16f64 * m_arg,
            0.5f64 * F_renorm.val,
            0.5f64 * F_renorm.err,
            result,
        );
        return if stat_m != GSL_SUCCESS as libc::c_int {
            stat_m
        } else if stat_F != GSL_SUCCESS as libc::c_int {
            stat_F
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if a_integer != 0 && b_integer != 0 {
        return gsl_sf_hyperg_1F1_int_e(
            rinta as libc::c_int,
            rintb as libc::c_int,
            x,
            result,
        )
    } else if b_neg_integer != 0 && !(a_neg_integer != 0 && a > b) {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            1925 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if a_neg_integer != 0 {
        return hyperg_1F1_a_negint_lag(rinta as libc::c_int, b, x, result)
    } else if b > 0.0f64 {
        if -1.0f64 <= a && a <= 1.0f64 {
            return hyperg_1F1_small_a_bgt0(a, b, x, result)
        } else if bma_neg_integer != 0 {
            let mut Kummer_1F1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_K: libc::c_int = hyperg_1F1_a_negint_lag(
                rintbma as libc::c_int,
                b,
                -x,
                &mut Kummer_1F1,
            );
            let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e(
                x,
                2.2204460492503131e-16f64 * fabs(x),
                Kummer_1F1.val,
                Kummer_1F1.err,
                result,
            );
            return if stat_e_0 != GSL_SUCCESS as libc::c_int {
                stat_e_0
            } else if stat_K != GSL_SUCCESS as libc::c_int {
                stat_K
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else if a < 0.0f64
            && fabs(x) < 2 as libc::c_int as libc::c_double * 7.0978271289338397e+02f64
        {
            let mut Kummer_1F1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_K_0: libc::c_int = hyperg_1F1_ab_pos(
                b - a,
                b,
                -x,
                &mut Kummer_1F1_0,
            );
            let mut stat_e_1: libc::c_int = gsl_sf_exp_mult_err_e(
                x,
                2.2204460492503131e-16f64 * fabs(x),
                Kummer_1F1_0.val,
                Kummer_1F1_0.err,
                result,
            );
            return if stat_e_1 != GSL_SUCCESS as libc::c_int {
                stat_e_1
            } else if stat_K_0 != GSL_SUCCESS as libc::c_int {
                stat_K_0
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else if a > 0 as libc::c_int as libc::c_double {
            return hyperg_1F1_ab_pos(a, b, x, result)
        } else {
            return gsl_sf_hyperg_1F1_series_e(a, b, x, result)
        }
    } else if bma_neg_integer != 0 && x < 0.0f64 {
        let mut K: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K_1: libc::c_int = 0;
        let mut stat_e_2: libc::c_int = 0;
        if a < 0.0f64 {
            stat_K_1 = hyperg_1F1_a_negint_poly(rintbma as libc::c_int, b, -x, &mut K);
        } else {
            stat_K_1 = hyperg_1F1_a_negint_lag(rintbma as libc::c_int, b, -x, &mut K);
        }
        stat_e_2 = gsl_sf_exp_mult_err_e(
            x,
            2.2204460492503131e-16f64 * fabs(x),
            K.val,
            K.err,
            result,
        );
        return if stat_e_2 != GSL_SUCCESS as libc::c_int {
            stat_e_2
        } else if stat_K_1 != GSL_SUCCESS as libc::c_int {
            stat_K_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if a > 0.0f64 {
        let mut K_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K_2: libc::c_int = hyperg_1F1_ab_neg(b - a, b, -x, &mut K_0);
        let mut stat_e_3: libc::c_int = gsl_sf_exp_mult_err_e(
            x,
            2.2204460492503131e-16f64 * fabs(x),
            K_0.val,
            K_0.err,
            result,
        );
        return if stat_e_3 != GSL_SUCCESS as libc::c_int {
            stat_e_3
        } else if stat_K_2 != GSL_SUCCESS as libc::c_int {
            stat_K_2
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        return hyperg_1F1_ab_neg(a, b, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_1F1_int(
    m: libc::c_int,
    n: libc::c_int,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_1F1_int_e(m, n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_1F1_int_e(m, n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            2059 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hyperg_1F1(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_1F1_e(a, b, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_1F1_e(a, b, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hyperg_1F1.c\0" as *const u8 as *const libc::c_char,
            2064 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
