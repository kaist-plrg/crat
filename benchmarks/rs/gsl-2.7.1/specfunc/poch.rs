use ::libc;
extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_expm1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_log_1plusx_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
    fn gsl_sf_psi_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_sgn_e(
        x: libc::c_double,
        result_lg: *mut gsl_sf_result,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_gammainv_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
static mut bern: [libc::c_double; 21] = [
    0.0f64,
    0.833333333333333333333333333333333e-01f64,
    -0.138888888888888888888888888888888e-02f64,
    0.330687830687830687830687830687830e-04f64,
    -0.826719576719576719576719576719576e-06f64,
    0.208767569878680989792100903212014e-07f64,
    -0.528419013868749318484768220217955e-09f64,
    0.133825365306846788328269809751291e-10f64,
    -0.338968029632258286683019539124944e-12f64,
    0.858606205627784456413590545042562e-14f64,
    -0.217486869855806187304151642386591e-15f64,
    0.550900282836022951520265260890225e-17f64,
    -0.139544646858125233407076862640635e-18f64,
    0.353470703962946747169322997780379e-20f64,
    -0.895351742703754685040261131811274e-22f64,
    0.226795245233768306031095073886816e-23f64,
    -0.574472439520264523834847971943400e-24f64,
    0.145517247561486490186626486727132e-26f64,
    -0.368599494066531017818178247990866e-28f64,
    0.933673425709504467203255515278562e-30f64,
    -0.236502241570062993455963519636983e-31f64,
];
unsafe extern "C" fn pochrel_smallx(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let SQTBIG: libc::c_double = 1.0f64
        / (2.0f64 * 1.41421356237309504880f64 * 1.73205080756887729352744634151f64
            * 1.4916681462400413e-154f64);
    let ALNEPS: libc::c_double = -3.6043653389117154e+01f64 - 0.69314718055994530942f64;
    if x == 0.0f64 {
        return gsl_sf_psi_e(a, result)
    } else {
        let bp: libc::c_double = if a < -0.5f64 { 1.0f64 - a - x } else { a };
        let incr: libc::c_int = (if bp < 10.0f64 {
            11.0f64 - bp
        } else {
            0 as libc::c_int as libc::c_double
        }) as libc::c_int;
        let b: libc::c_double = bp + incr as libc::c_double;
        let mut dpoch1: libc::c_double = 0.;
        let mut dexprl: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_dexprl: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut var: libc::c_double = b + 0.5f64 * (x - 1.0f64);
        let mut alnvar: libc::c_double = log(var);
        let mut q: libc::c_double = x * alnvar;
        let mut poly1: libc::c_double = 0.0f64;
        if var < SQTBIG {
            let nterms: libc::c_int = (-0.5f64 * ALNEPS / alnvar + 1.0f64)
                as libc::c_int;
            let var2: libc::c_double = 1.0f64 / var / var;
            let rho: libc::c_double = 0.5f64 * (x + 1.0f64);
            let mut term: libc::c_double = var2;
            let mut gbern: [libc::c_double; 24] = [0.; 24];
            let mut k: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            gbern[1 as libc::c_int as usize] = 1.0f64;
            gbern[2 as libc::c_int as usize] = -rho / 12.0f64;
            poly1 = gbern[2 as libc::c_int as usize] * term;
            if nterms > 20 as libc::c_int {
                (*result).val = 0.0f64;
                (*result).err = 0.0f64;
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"poch.c\0" as *const u8 as *const libc::c_char,
                    128 as libc::c_int,
                    GSL_ESANITY as libc::c_int,
                );
                return GSL_ESANITY as libc::c_int;
            }
            k = 2 as libc::c_int;
            while k <= nterms {
                let mut gbk: libc::c_double = 0.0f64;
                j = 1 as libc::c_int;
                while j <= k {
                    gbk += bern[(k - j + 1 as libc::c_int) as usize] * gbern[j as usize];
                    j += 1;
                    j;
                }
                gbern[(k + 1 as libc::c_int)
                    as usize] = -rho * gbk / k as libc::c_double;
                term
                    *= ((2 as libc::c_int * k - 2 as libc::c_int) as libc::c_double - x)
                        * ((2 as libc::c_int * k - 1 as libc::c_int) as libc::c_double
                            - x) * var2;
                poly1 += gbern[(k + 1 as libc::c_int) as usize] * term;
                k += 1;
                k;
            }
        }
        stat_dexprl = gsl_sf_expm1_e(q, &mut dexprl);
        if stat_dexprl != GSL_SUCCESS as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_dexprl;
        }
        dexprl.val = dexprl.val / q;
        poly1 *= x - 1.0f64;
        dpoch1 = dexprl.val * (alnvar + q * poly1) + poly1;
        i = incr - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let mut binv: libc::c_double = 1.0f64 / (bp + i as libc::c_double);
            dpoch1 = (dpoch1 - binv) / (1.0f64 + x * binv);
            i -= 1;
            i;
        }
        if bp == a {
            (*result).val = dpoch1;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(incr as libc::c_double) + 1.0f64) * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut sinpxx: libc::c_double = sin(3.14159265358979323846f64 * x) / x;
            let mut sinpx2: libc::c_double = sin(0.5f64 * 3.14159265358979323846f64 * x);
            let mut t1: libc::c_double = sinpxx / tan(3.14159265358979323846f64 * b);
            let mut t2: libc::c_double = 2.0f64 * sinpx2 * (sinpx2 / x);
            let mut trig: libc::c_double = t1 - t2;
            (*result).val = dpoch1 * (1.0f64 + x * trig) + trig;
            (*result)
                .err = (fabs(dpoch1 * x) + 1.0f64) * 2.2204460492503131e-16f64
                * (fabs(t1) + fabs(t2));
            (*result).err
                += 2.0f64 * 2.2204460492503131e-16f64
                    * (fabs(incr as libc::c_double) + 1.0f64) * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
unsafe extern "C" fn lnpoch_pos(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut absx: libc::c_double = fabs(x);
    if absx > 0.1f64 * a || absx * log(GSL_MAX_DBL(a, 2.0f64)) > 0.1f64 {
        if a < 171.0f64 && a + x < 171.0f64 {
            let mut g1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut g2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            gsl_sf_gammainv_e(a, &mut g1);
            gsl_sf_gammainv_e(a + x, &mut g2);
            (*result).val = -log(g2.val / g1.val);
            (*result).err = g1.err / fabs(g1.val) + g2.err / fabs(g2.val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut lg1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lg2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_1: libc::c_int = gsl_sf_lngamma_e(a, &mut lg1);
            let mut stat_2: libc::c_int = gsl_sf_lngamma_e(a + x, &mut lg2);
            (*result).val = lg2.val - lg1.val;
            (*result).err = lg2.err + lg1.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_1 != GSL_SUCCESS as libc::c_int {
                stat_1
            } else if stat_2 != GSL_SUCCESS as libc::c_int {
                stat_2
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    } else if absx < 0.1f64 * a && a > 15.0f64 {
        let eps: libc::c_double = x / a;
        let den: libc::c_double = 1.0f64 + eps;
        let d3: libc::c_double = den * den * den;
        let d5: libc::c_double = d3 * den * den;
        let d7: libc::c_double = d5 * den * den;
        let c1: libc::c_double = -eps / den;
        let c3: libc::c_double = -eps * (3.0f64 + eps * (3.0f64 + eps)) / d3;
        let c5: libc::c_double = -eps
            * (5.0f64 + eps * (10.0f64 + eps * (10.0f64 + eps * (5.0f64 + eps)))) / d5;
        let c7: libc::c_double = -eps
            * (7.0f64
                + eps
                    * (21.0f64
                        + eps
                            * (35.0f64
                                + eps
                                    * (35.0f64 + eps * (21.0f64 + eps * (7.0f64 + eps))))))
            / d7;
        let p8: libc::c_double = gsl_sf_pow_int(1.0f64 + eps, 8 as libc::c_int);
        let c8: libc::c_double = 1.0f64 / p8 - 1.0f64;
        let c9: libc::c_double = 1.0f64 / (p8 * (1.0f64 + eps)) - 1.0f64;
        let a4: libc::c_double = a * a * a * a;
        let a6: libc::c_double = a4 * a * a;
        let ser_1: libc::c_double = c1 + c3 / (30.0f64 * a * a) + c5 / (105.0f64 * a4)
            + c7 / (140.0f64 * a6);
        let ser_2: libc::c_double = c8 / (99.0f64 * a6 * a * a)
            - 691.0f64 / 360360.0f64 * c9 / (a6 * a4);
        let ser: libc::c_double = (ser_1 + ser_2) / (12.0f64 * a);
        let mut term1: libc::c_double = x * log(a / 2.7182818284590452354f64);
        let mut term2: libc::c_double = 0.;
        let mut ln_1peps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_log_1plusx_e(eps, &mut ln_1peps);
        term2 = (x + a - 0.5f64) * ln_1peps.val;
        (*result).val = term1 + term2 + ser;
        (*result).err = 2.2204460492503131e-16f64 * fabs(term1);
        (*result).err += fabs((x + a - 0.5f64) * ln_1peps.err);
        (*result).err
            += fabs(ln_1peps.val) * 2.2204460492503131e-16f64
                * (fabs(x) + fabs(a) + 0.5f64);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut poch_rel: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_p: libc::c_int = pochrel_smallx(a, x, &mut poch_rel);
        let mut eps_0: libc::c_double = x * poch_rel.val;
        let mut stat_e: libc::c_int = gsl_sf_log_1plusx_e(eps_0, result);
        (*result).err = 2.0f64 * fabs(x * poch_rel.err / (1.0f64 + eps_0));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_p != GSL_SUCCESS as libc::c_int {
            stat_p
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_lnpoch_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a <= 0.0f64 || a + x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"poch.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        return lnpoch_pos(a, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_lnpoch_sgn_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut sgn: *mut libc::c_double,
) -> libc::c_int {
    if x == 0.0f64 {
        *sgn = 1.0f64;
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if a > 0.0f64 && a + x > 0.0f64 {
        *sgn = 1.0f64;
        return lnpoch_pos(a, x, result);
    } else if a <= 0 as libc::c_int as libc::c_double && a == floor(a) {
        if a + x < 0 as libc::c_int as libc::c_double && x == floor(x) {
            let mut result_pos: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat: libc::c_int = lnpoch_pos(-a, -x, &mut result_pos);
            let mut f: libc::c_double = log(a / (a + x));
            let mut s: libc::c_double = (if fmod(x, 2 as libc::c_int as libc::c_double)
                == 0 as libc::c_int as libc::c_double
            {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            (*result).val = f - result_pos.val;
            (*result).err = result_pos.err + 2.0f64 * 2.2204460492503131e-16f64 * f;
            *sgn = s;
            return stat;
        } else if a + x == 0 as libc::c_int as libc::c_double {
            let mut stat_0: libc::c_int = gsl_sf_lngamma_sgn_e(
                -a + 1 as libc::c_int as libc::c_double,
                result,
                sgn,
            );
            let mut s_0: libc::c_double = (if fmod(
                -a,
                2 as libc::c_int as libc::c_double,
            ) == 0 as libc::c_int as libc::c_double
            {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            *sgn *= s_0;
            return stat_0;
        } else {
            (*result).val = -::std::f32::INFINITY as libc::c_double;
            (*result).err = 0.0f64;
            *sgn = 1 as libc::c_int as libc::c_double;
            return GSL_SUCCESS as libc::c_int;
        }
    } else if a < 0.0f64 && a + x < 0.0f64 {
        let mut sin_1: libc::c_double = sin(3.14159265358979323846f64 * (1.0f64 - a));
        let mut sin_2: libc::c_double = sin(
            3.14159265358979323846f64 * (1.0f64 - a - x),
        );
        if sin_1 == 0.0f64 || sin_2 == 0.0f64 {
            *sgn = 0.0f64;
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"poch.c\0" as *const u8 as *const libc::c_char,
                349 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        } else {
            let mut lnp_pos: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_pp: libc::c_int = lnpoch_pos(1.0f64 - a, -x, &mut lnp_pos);
            let mut lnterm: libc::c_double = log(fabs(sin_1 / sin_2));
            (*result).val = lnterm - lnp_pos.val;
            (*result).err = lnp_pos.err;
            (*result).err
                += 2.0f64 * 2.2204460492503131e-16f64
                    * (fabs(1.0f64 - a) + fabs(1.0f64 - a - x)) * fabs(lnterm);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            *sgn = (if sin_1 * sin_2 >= 0.0f64 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            return stat_pp;
        }
    } else {
        let mut lg_apn: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lg_a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut s_apn: libc::c_double = 0.;
        let mut s_a: libc::c_double = 0.;
        let mut stat_apn: libc::c_int = gsl_sf_lngamma_sgn_e(
            a + x,
            &mut lg_apn,
            &mut s_apn,
        );
        let mut stat_a: libc::c_int = gsl_sf_lngamma_sgn_e(a, &mut lg_a, &mut s_a);
        if stat_apn == GSL_SUCCESS as libc::c_int && stat_a == GSL_SUCCESS as libc::c_int
        {
            (*result).val = lg_apn.val - lg_a.val;
            (*result).err = lg_apn.err + lg_a.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            *sgn = s_a * s_apn;
            return GSL_SUCCESS as libc::c_int;
        } else if stat_apn == GSL_EDOM as libc::c_int
            || stat_a == GSL_EDOM as libc::c_int
        {
            *sgn = 0.0f64;
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"poch.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        } else {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            *sgn = 0.0f64;
            return GSL_FAILURE as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_poch_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut lnpoch: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn: libc::c_double = 0.;
        let mut stat_lnpoch: libc::c_int = gsl_sf_lnpoch_sgn_e(
            a,
            x,
            &mut lnpoch,
            &mut sgn,
        );
        if lnpoch.val == -::std::f32::INFINITY as libc::c_double {
            (*result).val = 0 as libc::c_int as libc::c_double;
            (*result).err = 0 as libc::c_int as libc::c_double;
            return stat_lnpoch;
        } else {
            let mut stat_exp: libc::c_int = gsl_sf_exp_err_e(
                lnpoch.val,
                lnpoch.err,
                result,
            );
            (*result).val *= sgn;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return if stat_exp != GSL_SUCCESS as libc::c_int {
                stat_exp
            } else if stat_lnpoch != GSL_SUCCESS as libc::c_int {
                stat_lnpoch
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_pochrel_e(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let absx: libc::c_double = fabs(x);
    let absa: libc::c_double = fabs(a);
    if absx > 0.1f64 * absa
        || absx * log((if absa > 2.0f64 { absa } else { 2.0f64 })) > 0.1f64
    {
        let mut lnpoch: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn: libc::c_double = 0.;
        let mut stat_poch: libc::c_int = gsl_sf_lnpoch_sgn_e(
            a,
            x,
            &mut lnpoch,
            &mut sgn,
        );
        if lnpoch.val > 7.0978271289338397e+02f64 {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"poch.c\0" as *const u8 as *const libc::c_char,
                432 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else {
            let el: libc::c_double = exp(lnpoch.val);
            (*result).val = (sgn * el - 1.0f64) / x;
            (*result)
                .err = fabs((*result).val)
                * (lnpoch.err + 2.0f64 * 2.2204460492503131e-16f64);
            (*result).err
                += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(sgn * el) + 1.0f64)
                    / fabs(x);
            return stat_poch;
        }
    } else {
        return pochrel_smallx(a, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_lnpoch(
    a: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lnpoch_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lnpoch_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"poch.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_poch(
    a: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_poch_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_poch_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"poch.c\0" as *const u8 as *const libc::c_char,
            459 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_pochrel(
    a: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_pochrel_e(a, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_pochrel_e(a, x, &result)\0" as *const u8 as *const libc::c_char,
            b"poch.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
