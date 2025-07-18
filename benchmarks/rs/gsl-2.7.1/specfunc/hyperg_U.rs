use ::libc;
extern "C" {
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exp_e10_e(
        x: libc::c_double,
        result: *mut gsl_sf_result_e10,
    ) -> libc::c_int;
    fn gsl_sf_exprel_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_err_e10_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result_e10,
    ) -> libc::c_int;
    fn gsl_sf_result_smash_e(
        re: *const gsl_sf_result_e10,
        r: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_mult_err_e10_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result_e10,
    ) -> libc::c_int;
    fn gsl_sf_gamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_gammainv_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_gammainv(x: libc::c_double) -> libc::c_double;
    fn gsl_sf_lnfact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_poch_e(
        a: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_poch(a: libc::c_double, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_pochrel_e(
        a: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_lnKnu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_laguerre_n_e(
        n: libc::c_int,
        a: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_pow_int_e(
        x: libc::c_double,
        n: libc::c_int,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_1F1_e(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_U_large_b_e(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
        ln_multiplier: *mut libc::c_double,
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
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn hyperg_lnU_beq2a(
    a: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let lx: libc::c_double = log(x);
    let nu: libc::c_double = a - 0.5f64;
    let lnpre: libc::c_double = 0.5f64 * (x - 1.14472988584940017414342735135f64)
        - nu * lx;
    let mut lnK: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    gsl_sf_bessel_lnKnu_e(nu, 0.5f64 * x, &mut lnK);
    (*result).val = lnpre + lnK.val;
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64
        * (fabs(0.5f64 * x) + 0.5f64 * 1.14472988584940017414342735135f64
            + fabs(nu * lx));
    (*result).err += lnK.err;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hyperg_U_CF1(
    a: libc::c_double,
    b: libc::c_double,
    N: libc::c_int,
    x: libc::c_double,
    mut result: *mut libc::c_double,
    mut count: *mut libc::c_int,
) -> libc::c_int {
    let RECUR_BIG: libc::c_double = 1.3407807929942596e+154f64;
    let maxiter: libc::c_int = 20000 as libc::c_int;
    let mut n: libc::c_int = 1 as libc::c_int;
    let mut Anm2: libc::c_double = 1.0f64;
    let mut Bnm2: libc::c_double = 0.0f64;
    let mut Anm1: libc::c_double = 0.0f64;
    let mut Bnm1: libc::c_double = 1.0f64;
    let mut a1: libc::c_double = -(a + N as libc::c_double);
    let mut b1: libc::c_double = b - 2.0f64 * a - x
        - 2.0f64 * (N + 1 as libc::c_int) as libc::c_double;
    let mut An: libc::c_double = b1 * Anm1 + a1 * Anm2;
    let mut Bn: libc::c_double = b1 * Bnm1 + a1 * Bnm2;
    let mut an: libc::c_double = 0.;
    let mut bn: libc::c_double = 0.;
    let mut fn_0: libc::c_double = An / Bn;
    while n < maxiter {
        let mut old_fn: libc::c_double = 0.;
        let mut del: libc::c_double = 0.;
        n += 1;
        n;
        Anm2 = Anm1;
        Bnm2 = Bnm1;
        Anm1 = An;
        Bnm1 = Bn;
        an = -(a + N as libc::c_double + n as libc::c_double - b)
            * (a + N as libc::c_double + n as libc::c_double - 1.0f64);
        bn = b - 2.0f64 * a - x - 2.0f64 * (N + n) as libc::c_double;
        An = bn * Anm1 + an * Anm2;
        Bn = bn * Bnm1 + an * Bnm2;
        if fabs(An) > RECUR_BIG || fabs(Bn) > RECUR_BIG {
            An /= RECUR_BIG;
            Bn /= RECUR_BIG;
            Anm1 /= RECUR_BIG;
            Bnm1 /= RECUR_BIG;
            Anm2 /= RECUR_BIG;
            Bnm2 /= RECUR_BIG;
        }
        old_fn = fn_0;
        fn_0 = An / Bn;
        del = old_fn / fn_0;
        if fabs(del - 1.0f64) < 10.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
    }
    *result = fn_0;
    *count = n;
    if n == maxiter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn d9chu(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let EPS: libc::c_double = 8.0f64 * 2.2204460492503131e-16f64;
    let maxiter: libc::c_int = 500 as libc::c_int;
    let mut aa: [libc::c_double; 4] = [0.; 4];
    let mut bb: [libc::c_double; 4] = [0.; 4];
    let mut i: libc::c_int = 0;
    let mut bp: libc::c_double = 1.0f64 + a - b;
    let mut ab: libc::c_double = a * bp;
    let mut ct2: libc::c_double = 2.0f64 * (x - ab);
    let mut sab: libc::c_double = a + bp;
    let mut ct3: libc::c_double = sab + 1.0f64 + ab;
    let mut anbn: libc::c_double = ct3 + sab + 3.0f64;
    let mut ct1: libc::c_double = 1.0f64 + 2.0f64 * x / anbn;
    bb[0 as libc::c_int as usize] = 1.0f64;
    aa[0 as libc::c_int as usize] = 1.0f64;
    bb[1 as libc::c_int as usize] = 1.0f64 + 2.0f64 * x / ct3;
    aa[1 as libc::c_int as usize] = 1.0f64 + ct2 / ct3;
    bb[2 as libc::c_int as usize] = 1.0f64 + 6.0f64 * ct1 * x / ct3;
    aa[2 as libc::c_int
        as usize] = 1.0f64 + 6.0f64 * ab / anbn + 3.0f64 * ct1 * ct2 / ct3;
    i = 4 as libc::c_int;
    while i < maxiter {
        let mut j: libc::c_int = 0;
        let mut c2: libc::c_double = 0.;
        let mut d1z: libc::c_double = 0.;
        let mut g1: libc::c_double = 0.;
        let mut g2: libc::c_double = 0.;
        let mut g3: libc::c_double = 0.;
        let mut x2i1: libc::c_double = (2 as libc::c_int * i - 3 as libc::c_int)
            as libc::c_double;
        ct1 = x2i1 / (x2i1 - 2.0f64);
        anbn += x2i1 + sab;
        ct2 = (x2i1 - 1.0f64) / anbn;
        c2 = x2i1 * ct2 - 1.0f64;
        d1z = 2.0f64 * x2i1 * x / anbn;
        ct3 = sab * ct2;
        g1 = d1z + ct1 * (c2 + ct3);
        g2 = d1z - c2;
        g3 = ct1 * (1.0f64 - ct3 - 2.0f64 * ct2);
        bb[3 as libc::c_int
            as usize] = g1 * bb[2 as libc::c_int as usize]
            + g2 * bb[1 as libc::c_int as usize] + g3 * bb[0 as libc::c_int as usize];
        aa[3 as libc::c_int
            as usize] = g1 * aa[2 as libc::c_int as usize]
            + g2 * aa[1 as libc::c_int as usize] + g3 * aa[0 as libc::c_int as usize];
        if fabs(
            aa[3 as libc::c_int as usize] * bb[0 as libc::c_int as usize]
                - aa[0 as libc::c_int as usize] * bb[3 as libc::c_int as usize],
        ) < EPS * fabs(bb[3 as libc::c_int as usize] * bb[0 as libc::c_int as usize])
        {
            break;
        }
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            aa[j as usize] = aa[(j + 1 as libc::c_int) as usize];
            bb[j as usize] = bb[(j + 1 as libc::c_int) as usize];
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*result).val = aa[3 as libc::c_int as usize] / bb[3 as libc::c_int as usize];
    (*result).err = 8.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    if i == maxiter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn hyperg_zaU_asymp(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ap: libc::c_double = a;
    let bp: libc::c_double = 1.0f64 + a - b;
    let rintap: libc::c_double = floor(ap + 0.5f64);
    let rintbp: libc::c_double = floor(bp + 0.5f64);
    let ap_neg_int: libc::c_int = (ap < 0.0f64
        && fabs(ap - rintap) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    let bp_neg_int: libc::c_int = (bp < 0.0f64
        && fabs(bp - rintbp) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if ap_neg_int != 0 || bp_neg_int != 0 {
        let mut mxi: libc::c_double = -1.0f64 / x;
        let mut nmax: libc::c_double = -(((if ap < bp { ap } else { bp }) - 0.1f64)
            as libc::c_int) as libc::c_double;
        let mut tn: libc::c_double = 1.0f64;
        let mut sum: libc::c_double = 1.0f64;
        let mut n: libc::c_double = 1.0f64;
        let mut sum_err: libc::c_double = 0.0f64;
        while n <= nmax {
            let mut apn: libc::c_double = ap + n - 1.0f64;
            let mut bpn: libc::c_double = bp + n - 1.0f64;
            tn *= apn / n * mxi * bpn;
            sum += tn;
            sum_err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(tn);
            n += 1.0f64;
        }
        (*result).val = sum;
        (*result).err = sum_err;
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(nmax) + 1.0f64) * fabs(sum);
        return GSL_SUCCESS as libc::c_int;
    } else {
        return d9chu(a, b, x, result)
    };
}
unsafe extern "C" fn hyperg_U_finite_sum(
    mut N: libc::c_int,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut xeps: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sum_val: libc::c_double = 0.;
    let mut sum_err: libc::c_double = 0.;
    if N <= 0 as libc::c_int {
        let mut t_val: libc::c_double = 1.0f64;
        let mut t_err: libc::c_double = 0.0f64;
        let mut poch: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_poch: libc::c_int = 0;
        sum_val = 1.0f64;
        sum_err = 0.0f64;
        i = 1 as libc::c_int;
        while i <= -N {
            let xi1: libc::c_double = (i - 1 as libc::c_int) as libc::c_double;
            let mult: libc::c_double = (a + xi1) * x / ((b + xi1) * (xi1 + 1.0f64));
            t_val *= mult;
            t_err
                += fabs(mult) * t_err
                    + fabs(t_val) * 8.0f64 * 2.0f64 * 2.2204460492503131e-16f64;
            sum_val += t_val;
            sum_err += t_err;
            i += 1;
            i;
        }
        stat_poch = gsl_sf_poch_e(1.0f64 + a - b, -a, &mut poch);
        (*result).val = sum_val * poch.val;
        (*result).err = fabs(sum_val) * poch.err + sum_err * fabs(poch.val);
        (*result).err
            += fabs(poch.val) * (fabs(N as libc::c_double) + 2.0f64)
                * 2.2204460492503131e-16f64 * fabs(sum_val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).err *= 2.0f64;
        return stat_poch;
    } else {
        let M: libc::c_int = N - 2 as libc::c_int;
        if M < 0 as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut gbm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut gamr: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_gbm1: libc::c_int = 0;
            let mut stat_gamr: libc::c_int = 0;
            let mut t_val_0: libc::c_double = 1.0f64;
            let mut t_err_0: libc::c_double = 0.0f64;
            sum_val = 1.0f64;
            sum_err = 0.0f64;
            i = 1 as libc::c_int;
            while i <= M {
                let mult_0: libc::c_double = (a - b + i as libc::c_double) * x
                    / ((1.0f64 - b + i as libc::c_double) * i as libc::c_double);
                t_val_0 *= mult_0;
                t_err_0
                    += t_err_0 * fabs(mult_0)
                        + fabs(t_val_0) * 8.0f64 * 2.0f64 * 2.2204460492503131e-16f64;
                sum_val += t_val_0;
                sum_err += t_err_0;
                i += 1;
                i;
            }
            stat_gbm1 = gsl_sf_gamma_e(b - 1.0f64, &mut gbm1);
            stat_gamr = gsl_sf_gammainv_e(a, &mut gamr);
            if stat_gbm1 == GSL_SUCCESS as libc::c_int {
                let mut powx1N: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
                let mut stat_p: libc::c_int = gsl_sf_pow_int_e(
                    x,
                    1 as libc::c_int - N,
                    &mut powx1N,
                );
                let mut pe_val: libc::c_double = powx1N.val * xeps;
                let mut pe_err: libc::c_double = powx1N.err * fabs(xeps)
                    + 2.0f64 * 2.2204460492503131e-16f64 * fabs(pe_val);
                let mut coeff_val: libc::c_double = gbm1.val * gamr.val * pe_val;
                let mut coeff_err: libc::c_double = gbm1.err * fabs(gamr.val * pe_val)
                    + gamr.err * fabs(gbm1.val * pe_val)
                    + fabs(gbm1.val * gamr.val) * pe_err
                    + 2.0f64 * 2.2204460492503131e-16f64 * fabs(coeff_val);
                (*result).val = sum_val * coeff_val;
                (*result).err = fabs(sum_val) * coeff_err + sum_err * fabs(coeff_val);
                (*result).err
                    += 2.0f64 * 2.2204460492503131e-16f64
                        * (M as libc::c_double + 2.0f64) * fabs((*result).val);
                (*result).err *= 2.0f64;
                return stat_p;
            } else {
                (*result).val = 0.0f64;
                (*result).err = 0.0f64;
                return stat_gbm1;
            }
        }
    };
}
unsafe extern "C" fn hyperg_U_infinite_sum_stable(
    mut N: libc::c_int,
    mut a: libc::c_double,
    mut bint: libc::c_double,
    mut b: libc::c_double,
    mut beps: libc::c_double,
    mut x: libc::c_double,
    mut xeps: libc::c_double,
    mut sum: gsl_sf_result,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let EPS: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64;
    let mut istrt: libc::c_int = if N < 1 as libc::c_int {
        1 as libc::c_int - N
    } else {
        0 as libc::c_int
    };
    let mut xi: libc::c_double = istrt as libc::c_double;
    let mut gamr: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut powx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_gamr: libc::c_int = gsl_sf_gammainv_e(1.0f64 + a - b, &mut gamr);
    let mut stat_powx: libc::c_int = gsl_sf_pow_int_e(x, istrt, &mut powx);
    let mut sarg: libc::c_double = beps * 3.14159265358979323846f64;
    let mut sfact: libc::c_double = if sarg != 0.0f64 {
        sarg / sin(sarg)
    } else {
        1.0f64
    };
    let mut factor_val: libc::c_double = sfact
        * (if N & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 }) * gamr.val
        * powx.val;
    let mut factor_err: libc::c_double = fabs(gamr.val) * powx.err
        + fabs(powx.val) * gamr.err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(factor_val);
    let mut pochai: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamri1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamrni: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_pochai: libc::c_int = gsl_sf_poch_e(a, xi, &mut pochai);
    let mut stat_gamri1: libc::c_int = gsl_sf_gammainv_e(xi + 1.0f64, &mut gamri1);
    let mut stat_gamrni: libc::c_int = gsl_sf_gammainv_e(bint + xi, &mut gamrni);
    let mut stat_gam123: libc::c_int = if stat_gamr != GSL_SUCCESS as libc::c_int {
        stat_gamr
    } else if stat_gamri1 != GSL_SUCCESS as libc::c_int {
        stat_gamri1
    } else if stat_gamrni != GSL_SUCCESS as libc::c_int {
        stat_gamrni
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut stat_gamall: libc::c_int = if stat_gam123 != GSL_SUCCESS as libc::c_int {
        stat_gam123
    } else if stat_pochai != GSL_SUCCESS as libc::c_int {
        stat_pochai
    } else if stat_powx != GSL_SUCCESS as libc::c_int {
        stat_powx
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut pochaxibeps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamrxi1beps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_pochaxibeps: libc::c_int = gsl_sf_poch_e(
        a,
        xi - beps,
        &mut pochaxibeps,
    );
    let mut stat_gamrxi1beps: libc::c_int = gsl_sf_gammainv_e(
        xi + 1.0f64 - beps,
        &mut gamrxi1beps,
    );
    let mut stat_all: libc::c_int = if stat_gamall != GSL_SUCCESS as libc::c_int {
        stat_gamall
    } else if stat_pochaxibeps != GSL_SUCCESS as libc::c_int {
        stat_pochaxibeps
    } else if stat_gamrxi1beps != GSL_SUCCESS as libc::c_int {
        stat_gamrxi1beps
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut b0_val: libc::c_double = factor_val * pochaxibeps.val * gamrni.val
        * gamrxi1beps.val;
    let mut b0_err: libc::c_double = fabs(factor_val * pochaxibeps.val * gamrni.val)
        * gamrxi1beps.err
        + fabs(factor_val * pochaxibeps.val * gamrxi1beps.val) * gamrni.err
        + fabs(factor_val * gamrni.val * gamrxi1beps.val) * pochaxibeps.err
        + fabs(pochaxibeps.val * gamrni.val * gamrxi1beps.val) * factor_err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(b0_val);
    let mut i: libc::c_int = 0;
    let mut dchu_val: libc::c_double = 0.;
    let mut dchu_err: libc::c_double = 0.;
    let mut t_val: libc::c_double = 0.;
    let mut t_err: libc::c_double = 0.;
    let mut dgamrbxi: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_dgamrbxi: libc::c_int = gsl_sf_gammainv_e(b + xi, &mut dgamrbxi);
    let mut a0_val: libc::c_double = factor_val * pochai.val * dgamrbxi.val * gamri1.val
        / beps;
    let mut a0_err: libc::c_double = fabs(factor_val * pochai.val * dgamrbxi.val / beps)
        * gamri1.err + fabs(factor_val * pochai.val * gamri1.val / beps) * dgamrbxi.err
        + fabs(factor_val * dgamrbxi.val * gamri1.val / beps) * pochai.err
        + fabs(pochai.val * dgamrbxi.val * gamri1.val / beps) * factor_err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(a0_val);
    stat_all = if stat_all != GSL_SUCCESS as libc::c_int {
        stat_all
    } else if stat_dgamrbxi != GSL_SUCCESS as libc::c_int {
        stat_dgamrbxi
    } else {
        GSL_SUCCESS as libc::c_int
    };
    b0_val = xeps * b0_val / beps;
    b0_err = fabs(xeps / beps) * b0_err
        + 4.0f64 * 2.2204460492503131e-16f64 * fabs(b0_val);
    dchu_val = sum.val + a0_val - b0_val;
    dchu_err = sum.err + a0_err + b0_err
        + 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(sum.val) + fabs(a0_val) + fabs(b0_val));
    i = 1 as libc::c_int;
    while i < 2000 as libc::c_int {
        let mut xi_0: libc::c_double = (istrt + i) as libc::c_double;
        let mut xi1: libc::c_double = (istrt + i - 1 as libc::c_int) as libc::c_double;
        let mut a0_multiplier: libc::c_double = (a + xi1) * x / ((b + xi1) * xi_0);
        let mut b0_multiplier: libc::c_double = (a + xi1 - beps) * x
            / ((bint + xi1) * (xi_0 - beps));
        a0_val *= a0_multiplier;
        a0_err += fabs(a0_multiplier) * a0_err;
        b0_val *= b0_multiplier;
        b0_err += fabs(b0_multiplier) * b0_err;
        t_val = a0_val - b0_val;
        t_err = a0_err + b0_err;
        dchu_val += t_val;
        dchu_err += t_err;
        if fabs(t_val) < EPS * fabs(dchu_val) {
            break;
        }
        i += 1;
        i;
    }
    (*result).val = dchu_val;
    (*result).err = 2.0f64 * dchu_err;
    (*result).err += 2.0f64 * fabs(t_val);
    (*result).err
        += 4.0f64 * 2.2204460492503131e-16f64 * (i as libc::c_double + 2.0f64)
            * fabs(dchu_val);
    (*result).err *= 2.0f64;
    if i >= 2000 as libc::c_int {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return stat_all
    };
}
unsafe extern "C" fn hyperg_U_infinite_sum_simple(
    mut N: libc::c_int,
    mut a: libc::c_double,
    mut bint: libc::c_double,
    mut b: libc::c_double,
    mut beps: libc::c_double,
    mut x: libc::c_double,
    mut xeps: libc::c_double,
    mut sum: gsl_sf_result,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let EPS: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64;
    let mut istrt: libc::c_int = if N < 1 as libc::c_int {
        1 as libc::c_int - N
    } else {
        0 as libc::c_int
    };
    let mut xi: libc::c_double = istrt as libc::c_double;
    let mut powx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_powx: libc::c_int = gsl_sf_pow_int_e(x, istrt, &mut powx);
    let mut sarg: libc::c_double = beps * 3.14159265358979323846f64;
    let mut sfact: libc::c_double = if sarg != 0.0f64 {
        sarg / sin(sarg)
    } else {
        1.0f64
    };
    let mut factor_val: libc::c_double = sfact
        * (if N & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 }) * powx.val;
    let mut factor_err: libc::c_double = fabs(powx.err)
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(factor_val);
    let mut pochai: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamri1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamrni: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_pochai: libc::c_int = gsl_sf_poch_e(a, xi, &mut pochai);
    let mut stat_gamri1: libc::c_int = gsl_sf_gammainv_e(xi + 1.0f64, &mut gamri1);
    let mut stat_gamrni: libc::c_int = gsl_sf_gammainv_e(bint + xi, &mut gamrni);
    let mut stat_gam123: libc::c_int = if stat_gamri1 != GSL_SUCCESS as libc::c_int {
        stat_gamri1
    } else if stat_gamrni != GSL_SUCCESS as libc::c_int {
        stat_gamrni
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut stat_gamall: libc::c_int = if stat_gam123 != GSL_SUCCESS as libc::c_int {
        stat_gam123
    } else if stat_pochai != GSL_SUCCESS as libc::c_int {
        stat_pochai
    } else if stat_powx != GSL_SUCCESS as libc::c_int {
        stat_powx
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut pochaxibeps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamrxi1beps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_pochaxibeps: libc::c_int = gsl_sf_poch_e(
        a,
        xi - beps,
        &mut pochaxibeps,
    );
    let mut stat_gamrxi1beps: libc::c_int = gsl_sf_gammainv_e(
        xi + 1.0f64 - beps,
        &mut gamrxi1beps,
    );
    let mut stat_all: libc::c_int = if stat_gamall != GSL_SUCCESS as libc::c_int {
        stat_gamall
    } else if stat_pochaxibeps != GSL_SUCCESS as libc::c_int {
        stat_pochaxibeps
    } else if stat_gamrxi1beps != GSL_SUCCESS as libc::c_int {
        stat_gamrxi1beps
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut X: libc::c_double = sfact
        * (if N & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 }) * powx.val
        * gsl_sf_poch(
            1 as libc::c_int as libc::c_double + a - b,
            xi - 1 as libc::c_int as libc::c_double + b - beps,
        ) * gsl_sf_gammainv(a);
    let mut b0_val: libc::c_double = X * gamrni.val * gamrxi1beps.val;
    let mut b0_err: libc::c_double = fabs(factor_val * pochaxibeps.val * gamrni.val)
        * gamrxi1beps.err
        + fabs(factor_val * pochaxibeps.val * gamrxi1beps.val) * gamrni.err
        + fabs(factor_val * gamrni.val * gamrxi1beps.val) * pochaxibeps.err
        + fabs(pochaxibeps.val * gamrni.val * gamrxi1beps.val) * factor_err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(b0_val);
    let mut i: libc::c_int = 0;
    let mut dchu_val: libc::c_double = 0.;
    let mut dchu_err: libc::c_double = 0.;
    let mut t_val: libc::c_double = 0.;
    let mut t_err: libc::c_double = 0.;
    let mut gamr: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut dgamrbxi: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_gamr: libc::c_int = gsl_sf_gammainv_e(1.0f64 + a - b, &mut gamr);
    let mut stat_dgamrbxi: libc::c_int = gsl_sf_gammainv_e(b + xi, &mut dgamrbxi);
    let mut a0_val: libc::c_double = factor_val * gamr.val * pochai.val * dgamrbxi.val
        * gamri1.val / beps;
    let mut a0_err: libc::c_double = fabs(
        factor_val * pochai.val * dgamrbxi.val * gamri1.val / beps,
    ) * gamr.err
        + fabs(factor_val * gamr.val * dgamrbxi.val * gamri1.val / beps) * pochai.err
        + fabs(factor_val * gamr.val * pochai.val * gamri1.val / beps) * dgamrbxi.err
        + fabs(factor_val * gamr.val * pochai.val * dgamrbxi.val / beps) * gamri1.err
        + fabs(pochai.val * gamr.val * dgamrbxi.val * gamri1.val / beps) * factor_err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(a0_val);
    stat_all = if stat_all != GSL_SUCCESS as libc::c_int {
        stat_all
    } else if stat_gamr != GSL_SUCCESS as libc::c_int {
        stat_gamr
    } else if stat_dgamrbxi != GSL_SUCCESS as libc::c_int {
        stat_dgamrbxi
    } else {
        GSL_SUCCESS as libc::c_int
    };
    b0_val = xeps * b0_val / beps;
    b0_err = fabs(xeps / beps) * b0_err
        + 4.0f64 * 2.2204460492503131e-16f64 * fabs(b0_val);
    dchu_val = sum.val + a0_val - b0_val;
    dchu_err = sum.err + a0_err + b0_err
        + 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(sum.val) + fabs(a0_val) + fabs(b0_val));
    i = 1 as libc::c_int;
    while i < 2000 as libc::c_int {
        let mut xi_0: libc::c_double = (istrt + i) as libc::c_double;
        let mut xi1: libc::c_double = (istrt + i - 1 as libc::c_int) as libc::c_double;
        let mut a0_multiplier: libc::c_double = (a + xi1) * x / ((b + xi1) * xi_0);
        let mut b0_multiplier: libc::c_double = (a + xi1 - beps) * x
            / ((bint + xi1) * (xi_0 - beps));
        a0_val *= a0_multiplier;
        a0_err += fabs(a0_multiplier) * a0_err;
        b0_val *= b0_multiplier;
        b0_err += fabs(b0_multiplier) * b0_err;
        t_val = a0_val - b0_val;
        t_err = a0_err + b0_err;
        dchu_val += t_val;
        dchu_err += t_err;
        if gsl_finite(t_val) == 0 || fabs(t_val) < EPS * fabs(dchu_val) {
            break;
        }
        i += 1;
        i;
    }
    (*result).val = dchu_val;
    (*result).err = 2.0f64 * dchu_err;
    (*result).err += 2.0f64 * fabs(t_val);
    (*result).err
        += 4.0f64 * 2.2204460492503131e-16f64 * (i as libc::c_double + 2.0f64)
            * fabs(dchu_val);
    (*result).err *= 2.0f64;
    if i >= 2000 as libc::c_int {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            537 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return stat_all
    };
}
unsafe extern "C" fn hyperg_U_infinite_sum_improved(
    mut N: libc::c_int,
    mut a: libc::c_double,
    mut bint: libc::c_double,
    mut b: libc::c_double,
    mut beps: libc::c_double,
    mut x: libc::c_double,
    mut xeps: libc::c_double,
    mut sum: gsl_sf_result,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let EPS: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64;
    let lnx: libc::c_double = log(x);
    let mut istrt: libc::c_int = if N < 1 as libc::c_int {
        1 as libc::c_int - N
    } else {
        0 as libc::c_int
    };
    let mut xi: libc::c_double = istrt as libc::c_double;
    let mut gamr: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut powx: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_gamr: libc::c_int = gsl_sf_gammainv_e(1.0f64 + a - b, &mut gamr);
    let mut stat_powx: libc::c_int = gsl_sf_pow_int_e(x, istrt, &mut powx);
    let mut sarg: libc::c_double = beps * 3.14159265358979323846f64;
    let mut sfact: libc::c_double = if sarg != 0.0f64 {
        sarg / sin(sarg)
    } else {
        1.0f64
    };
    let mut factor_val: libc::c_double = sfact
        * (if N & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 }) * gamr.val
        * powx.val;
    let mut factor_err: libc::c_double = fabs(gamr.val) * powx.err
        + fabs(powx.val) * gamr.err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(factor_val);
    let mut pochai: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamri1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamrni: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_pochai: libc::c_int = gsl_sf_poch_e(a, xi, &mut pochai);
    let mut stat_gamri1: libc::c_int = gsl_sf_gammainv_e(xi + 1.0f64, &mut gamri1);
    let mut stat_gamrni: libc::c_int = gsl_sf_gammainv_e(bint + xi, &mut gamrni);
    let mut stat_gam123: libc::c_int = if stat_gamr != GSL_SUCCESS as libc::c_int {
        stat_gamr
    } else if stat_gamri1 != GSL_SUCCESS as libc::c_int {
        stat_gamri1
    } else if stat_gamrni != GSL_SUCCESS as libc::c_int {
        stat_gamrni
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut stat_gamall: libc::c_int = if stat_gam123 != GSL_SUCCESS as libc::c_int {
        stat_gam123
    } else if stat_pochai != GSL_SUCCESS as libc::c_int {
        stat_pochai
    } else if stat_powx != GSL_SUCCESS as libc::c_int {
        stat_powx
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut pochaxibeps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut gamrxi1beps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_pochaxibeps: libc::c_int = gsl_sf_poch_e(
        a,
        xi - beps,
        &mut pochaxibeps,
    );
    let mut stat_gamrxi1beps: libc::c_int = gsl_sf_gammainv_e(
        xi + 1.0f64 - beps,
        &mut gamrxi1beps,
    );
    let mut stat_all: libc::c_int = if stat_gamall != GSL_SUCCESS as libc::c_int {
        stat_gamall
    } else if stat_pochaxibeps != GSL_SUCCESS as libc::c_int {
        stat_pochaxibeps
    } else if stat_gamrxi1beps != GSL_SUCCESS as libc::c_int {
        stat_gamrxi1beps
    } else {
        GSL_SUCCESS as libc::c_int
    };
    let mut b0_val: libc::c_double = factor_val * pochaxibeps.val * gamrni.val
        * gamrxi1beps.val;
    let mut b0_err: libc::c_double = fabs(factor_val * pochaxibeps.val * gamrni.val)
        * gamrxi1beps.err
        + fabs(factor_val * pochaxibeps.val * gamrxi1beps.val) * gamrni.err
        + fabs(factor_val * gamrni.val * gamrxi1beps.val) * pochaxibeps.err
        + fabs(pochaxibeps.val * gamrni.val * gamrxi1beps.val) * factor_err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(b0_val);
    let mut i: libc::c_int = 0;
    let mut pch1ai: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut pch1i: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut poch1bxibeps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_pch1ai: libc::c_int = gsl_sf_pochrel_e(a + xi, -beps, &mut pch1ai);
    let mut stat_pch1i: libc::c_int = gsl_sf_pochrel_e(
        xi + 1.0f64 - beps,
        beps,
        &mut pch1i,
    );
    let mut stat_poch1bxibeps: libc::c_int = gsl_sf_pochrel_e(
        b + xi,
        -beps,
        &mut poch1bxibeps,
    );
    let mut c0_t1_val: libc::c_double = beps * pch1ai.val * pch1i.val;
    let mut c0_t1_err: libc::c_double = fabs(beps) * fabs(pch1ai.val) * pch1i.err
        + fabs(beps) * fabs(pch1i.val) * pch1ai.err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(c0_t1_val);
    let mut c0_t2_val: libc::c_double = -poch1bxibeps.val + pch1ai.val - pch1i.val
        + c0_t1_val;
    let mut c0_t2_err: libc::c_double = poch1bxibeps.err + pch1ai.err + pch1i.err
        + c0_t1_err + 2.0f64 * 2.2204460492503131e-16f64 * fabs(c0_t2_val);
    let mut c0_val: libc::c_double = factor_val * pochai.val * gamrni.val * gamri1.val
        * c0_t2_val;
    let mut c0_err: libc::c_double = fabs(
        factor_val * pochai.val * gamrni.val * gamri1.val,
    ) * c0_t2_err + fabs(factor_val * pochai.val * gamrni.val * c0_t2_val) * gamri1.err
        + fabs(factor_val * pochai.val * gamri1.val * c0_t2_val) * gamrni.err
        + fabs(factor_val * gamrni.val * gamri1.val * c0_t2_val) * pochai.err
        + fabs(pochai.val * gamrni.val * gamri1.val * c0_t2_val) * factor_err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(c0_val);
    let mut dexprl: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_dexprl: libc::c_int = gsl_sf_exprel_e(-beps * lnx, &mut dexprl);
    let mut xeps1_val: libc::c_double = lnx * dexprl.val;
    let mut xeps1_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
        * (1.0f64 + fabs(beps * lnx)) * fabs(dexprl.val) + fabs(lnx) * dexprl.err
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(xeps1_val);
    let mut dchu_val: libc::c_double = sum.val + c0_val + xeps1_val * b0_val;
    let mut dchu_err: libc::c_double = sum.err + c0_err + fabs(xeps1_val) * b0_err
        + xeps1_err * fabs(b0_val) + fabs(b0_val * lnx) * dexprl.err
        + 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(sum.val) + fabs(c0_val) + fabs(xeps1_val * b0_val));
    let mut xn: libc::c_double = N as libc::c_double;
    let mut t_val: libc::c_double = 0.;
    let mut t_err: libc::c_double = 0.;
    stat_all = if stat_all != GSL_SUCCESS as libc::c_int {
        stat_all
    } else if stat_dexprl != GSL_SUCCESS as libc::c_int {
        stat_dexprl
    } else if stat_poch1bxibeps != GSL_SUCCESS as libc::c_int {
        stat_poch1bxibeps
    } else if stat_pch1i != GSL_SUCCESS as libc::c_int {
        stat_pch1i
    } else if stat_pch1ai != GSL_SUCCESS as libc::c_int {
        stat_pch1ai
    } else {
        GSL_SUCCESS as libc::c_int
    };
    i = 1 as libc::c_int;
    while i < 2000 as libc::c_int {
        let xi_0: libc::c_double = (istrt + i) as libc::c_double;
        let xi1: libc::c_double = (istrt + i - 1 as libc::c_int) as libc::c_double;
        let tmp: libc::c_double = (a - 1.0f64) * (xn + 2.0f64 * xi_0 - 1.0f64)
            + xi_0 * (xi_0 - beps);
        let b0_multiplier: libc::c_double = (a + xi1 - beps) * x
            / ((xn + xi1) * (xi_0 - beps));
        let c0_multiplier_1: libc::c_double = (a + xi1) * x / ((b + xi1) * xi_0);
        let c0_multiplier_2: libc::c_double = tmp
            / (xi_0 * (b + xi1) * (a + xi1 - beps));
        b0_val *= b0_multiplier;
        b0_err
            += fabs(b0_multiplier) * b0_err
                + fabs(b0_val) * 8.0f64 * 2.0f64 * 2.2204460492503131e-16f64;
        c0_val = c0_multiplier_1 * c0_val - c0_multiplier_2 * b0_val;
        c0_err = fabs(c0_multiplier_1) * c0_err + fabs(c0_multiplier_2) * b0_err
            + fabs(c0_val) * 8.0f64 * 2.0f64 * 2.2204460492503131e-16f64
            + fabs(b0_val * c0_multiplier_2) * 16.0f64 * 2.0f64
                * 2.2204460492503131e-16f64;
        t_val = c0_val + xeps1_val * b0_val;
        t_err = c0_err + fabs(xeps1_val) * b0_err;
        t_err += fabs(b0_val * lnx) * dexprl.err;
        t_err += fabs(b0_val) * xeps1_err;
        dchu_val += t_val;
        dchu_err += t_err;
        if fabs(t_val) < EPS * fabs(dchu_val) {
            break;
        }
        i += 1;
        i;
    }
    (*result).val = dchu_val;
    (*result).err = 2.0f64 * dchu_err;
    (*result).err += 2.0f64 * fabs(t_val);
    (*result).err
        += 4.0f64 * 2.2204460492503131e-16f64 * (i as libc::c_double + 2.0f64)
            * fabs(dchu_val);
    (*result).err *= 2.0f64;
    if i >= 2000 as libc::c_int {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            664 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return stat_all
    };
}
unsafe extern "C" fn hyperg_U_series(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let SQRT_EPS: libc::c_double = 1.41421356237309504880f64 * 1.4901161193847656e-08f64;
    let mut bint: libc::c_double = if b < 0.0f64 {
        ceil(b - 0.5f64)
    } else {
        floor(b + 0.5f64)
    };
    let mut beps: libc::c_double = b - bint;
    let mut a_beps: libc::c_double = a - beps;
    let mut r_a_beps: libc::c_double = floor(a_beps + 0.5f64);
    let mut a_beps_int: libc::c_double = (fabs(a_beps - r_a_beps)
        < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int as libc::c_double;
    if a_beps_int != 0. && a_beps <= 0 as libc::c_int as libc::c_double {
        beps = beps - 1 as libc::c_int as libc::c_double + floor(a_beps);
        bint = bint + 1 as libc::c_int as libc::c_double - floor(a_beps);
    }
    if fabs(1.0f64 + a - b) < SQRT_EPS {
        let mut lnr: libc::c_double = -a * log(x);
        let mut stat_e: libc::c_int = gsl_sf_exp_e(lnr, result);
        (*result).err += 2.0f64 * SQRT_EPS * fabs((*result).val);
        return stat_e;
    } else {
        let mut N: libc::c_int = bint as libc::c_int;
        let mut lnx: libc::c_double = log(x);
        let mut xeps: libc::c_double = exp(-beps * lnx);
        let mut sum: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_sum: libc::c_int = hyperg_U_finite_sum(N, a, b, x, xeps, &mut sum);
        let mut stat_inf: libc::c_int = 0;
        if fabs(xeps - 1.0f64) > 0.5f64 {
            stat_inf = hyperg_U_infinite_sum_stable(
                N,
                a,
                bint,
                b,
                beps,
                x,
                xeps,
                sum,
                result,
            );
        } else if 1 as libc::c_int as libc::c_double + a - b
            < 0 as libc::c_int as libc::c_double
            && 1 as libc::c_int as libc::c_double + a - b
                == floor(1 as libc::c_int as libc::c_double + a - b)
            && beps != 0 as libc::c_int as libc::c_double
        {
            stat_inf = hyperg_U_infinite_sum_simple(
                N,
                a,
                bint,
                b,
                beps,
                x,
                xeps,
                sum,
                result,
            );
        } else {
            stat_inf = hyperg_U_infinite_sum_improved(
                N,
                a,
                bint,
                b,
                beps,
                x,
                xeps,
                sum,
                result,
            );
        }
        return if stat_sum != GSL_SUCCESS as libc::c_int {
            stat_sum
        } else if stat_inf != GSL_SUCCESS as libc::c_int {
            stat_inf
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
unsafe extern "C" fn hyperg_U_small_ab(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if a == -1.0f64 {
        (*result).val = -b + x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(b) + fabs(x));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if a == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if GSL_MAX_DBL(fabs(a), 1.0f64) * GSL_MAX_DBL(fabs(1.0f64 + a - b), 1.0f64)
        < 0.99f64 * fabs(x)
    {
        let mut p: libc::c_double = pow(x, -a);
        let mut asymp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_asymp: libc::c_int = hyperg_zaU_asymp(a, b, x, &mut asymp);
        (*result).val = asymp.val * p;
        (*result).err = asymp.err * p;
        (*result).err += fabs(asymp.val) * 2.2204460492503131e-16f64 * fabs(a) * p;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_asymp;
    } else {
        return hyperg_U_series(a, b, x, result)
    };
}
unsafe extern "C" fn hyperg_U_small_a_bgt0(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
    mut ln_multiplier: *mut libc::c_double,
) -> libc::c_int {
    if a == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        *ln_multiplier = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if b > 5000.0f64 && x < 0.90f64 * fabs(b)
        || b > 500.0f64 && x < 0.50f64 * fabs(b)
    {
        let mut stat: libc::c_int = gsl_sf_hyperg_U_large_b_e(
            a,
            b,
            x,
            result,
            ln_multiplier,
        );
        if stat == GSL_EOVRFLW as libc::c_int {
            return GSL_SUCCESS as libc::c_int
        } else {
            return stat
        }
    } else if b > 15.0f64 {
        let mut eps: libc::c_double = b - floor(b);
        let mut b0: libc::c_double = 1.0f64 + eps;
        let mut r_Ubm1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Ub: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = hyperg_U_small_ab(a, b0, x, &mut r_Ubm1);
        let mut stat_1: libc::c_int = hyperg_U_small_ab(a, b0 + 1.0f64, x, &mut r_Ub);
        let mut Ubm1: libc::c_double = r_Ubm1.val;
        let mut Ub: libc::c_double = r_Ub.val;
        let mut Ubp1: libc::c_double = 0.;
        let mut bp: libc::c_double = 0.;
        bp = b0 + 1.0f64;
        while bp < b - 0.1f64 {
            Ubp1 = ((1.0f64 + a - bp) * Ubm1 + (bp + x - 1.0f64) * Ub) / x;
            Ubm1 = Ub;
            Ub = Ubp1;
            bp += 1.0f64;
        }
        (*result).val = Ub;
        (*result)
            .err = (fabs(r_Ubm1.err / r_Ubm1.val) + fabs(r_Ub.err / r_Ub.val))
            * fabs(Ub);
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(b - b0) + 1.0f64) * fabs(Ub);
        *ln_multiplier = 0.0f64;
        return if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        *ln_multiplier = 0.0f64;
        return hyperg_U_small_ab(a, b, x, result);
    };
}
unsafe extern "C" fn hyperg_U_int_bge1(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    if a == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else if a == -(1 as libc::c_int) {
        (*result).val = -b as libc::c_double + x;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(b as libc::c_double) + fabs(x));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else if b == a + 1 as libc::c_int {
        return gsl_sf_exp_e10_e(-a as libc::c_double * log(x), result)
    } else if GSL_MAX_DBL(fabs(a as libc::c_double), 1.0f64)
        * GSL_MAX_DBL(fabs(1.0f64 + a as libc::c_double - b as libc::c_double), 1.0f64)
        < 0.99f64 * fabs(x)
    {
        let ln_pre_val: libc::c_double = -a as libc::c_double * log(x);
        let ln_pre_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
            * fabs(ln_pre_val);
        let mut asymp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_asymp: libc::c_int = hyperg_zaU_asymp(
            a as libc::c_double,
            b as libc::c_double,
            x,
            &mut asymp,
        );
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e10_e(
            ln_pre_val,
            ln_pre_err,
            asymp.val,
            asymp.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_asymp != GSL_SUCCESS as libc::c_int {
            stat_asymp
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if (fabs(a as libc::c_double) < 5 as libc::c_int as libc::c_double
        && b < 5 as libc::c_int && x < 2.0f64
        || fabs(a as libc::c_double) < 10 as libc::c_int as libc::c_double
            && b < 10 as libc::c_int && x < 1.0f64)
        && 1 as libc::c_int + a - b > 0 as libc::c_int
    {
        let mut ser: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_ser: libc::c_int = hyperg_U_series(
            a as libc::c_double,
            b as libc::c_double,
            x,
            &mut ser,
        );
        (*result).val = ser.val;
        (*result).err = ser.err;
        (*result).e10 = 0 as libc::c_int;
        return stat_ser;
    } else if a < 0 as libc::c_int {
        let mut scale_count: libc::c_int = 0 as libc::c_int;
        let scale_factor: libc::c_double = 1.3407807929942596e+154f64;
        let mut lnm: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut y: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnscale: libc::c_double = 0.;
        let mut Uap1: libc::c_double = 1.0f64;
        let mut Ua: libc::c_double = -b as libc::c_double + x;
        let mut Uam1: libc::c_double = 0.;
        let mut ap: libc::c_int = 0;
        ap = -(1 as libc::c_int);
        while ap > a {
            Uam1 = ap as libc::c_double * ((b - ap) as libc::c_double - 1.0f64) * Uap1
                + (x + 2.0f64 * ap as libc::c_double - b as libc::c_double) * Ua;
            Uap1 = Ua;
            Ua = Uam1;
            let mut au0: libc::c_double = fabs(Ua);
            if au0 > scale_factor {
                Ua /= scale_factor;
                Uap1 /= scale_factor;
                scale_count += 1;
                scale_count;
            } else if au0 < 1.0f64 / scale_factor {
                Ua *= scale_factor;
                Uap1 *= scale_factor;
                scale_count -= 1;
                scale_count;
            }
            ap -= 1;
            ap;
        }
        lnscale = log(scale_factor);
        lnm.val = scale_count as libc::c_double * lnscale;
        lnm.err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(lnm.val);
        y.val = Ua;
        y
            .err = 4.0f64 * 2.2204460492503131e-16f64
            * (fabs(a as libc::c_double) + 1.0f64) * fabs(Ua);
        return gsl_sf_exp_mult_err_e10_e(lnm.val, lnm.err, y.val, y.err, result);
    } else if b as libc::c_double >= 2.0f64 * a as libc::c_double + x {
        let mut scale_count_0: libc::c_int = 0 as libc::c_int;
        let scale_factor_0: libc::c_double = 1.3407807929942596e+154f64;
        let mut r_Ua: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnm_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut y_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnscale_0: libc::c_double = 0.;
        let mut lm: libc::c_double = 0.;
        let mut stat_1: libc::c_int = hyperg_U_small_a_bgt0(
            1.0f64,
            b as libc::c_double,
            x,
            &mut r_Ua,
            &mut lm,
        );
        let mut stat_e_0: libc::c_int = 0;
        let mut Uam1_0: libc::c_double = 1.0f64;
        let mut Ua_0: libc::c_double = r_Ua.val;
        let mut Uap1_0: libc::c_double = 0.;
        let mut ap_0: libc::c_int = 0;
        Uam1_0 *= exp(-lm);
        ap_0 = 1 as libc::c_int;
        while ap_0 < a {
            Uap1_0 = -(Uam1_0
                + (b as libc::c_double - 2.0f64 * ap_0 as libc::c_double - x) * Ua_0)
                / (ap_0 as libc::c_double
                    * (1.0f64 + ap_0 as libc::c_double - b as libc::c_double));
            Uam1_0 = Ua_0;
            Ua_0 = Uap1_0;
            let mut au0_0: libc::c_double = fabs(Ua_0);
            if au0_0 > scale_factor_0 {
                Ua_0 /= scale_factor_0;
                Uam1_0 /= scale_factor_0;
                scale_count_0 += 1;
                scale_count_0;
            } else if au0_0 < 1.0f64 / scale_factor_0 {
                Ua_0 *= scale_factor_0;
                Uam1_0 *= scale_factor_0;
                scale_count_0 -= 1;
                scale_count_0;
            }
            ap_0 += 1;
            ap_0;
        }
        lnscale_0 = log(scale_factor_0);
        lnm_0.val = lm + scale_count_0 as libc::c_double * lnscale_0;
        lnm_0
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(lm) + fabs(scale_count_0 as libc::c_double * lnscale_0));
        y_0.val = Ua_0;
        y_0.err = fabs(r_Ua.err / r_Ua.val) * fabs(Ua_0);
        y_0.err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(a as libc::c_double) + 1.0f64)
                * fabs(Ua_0);
        stat_e_0 = gsl_sf_exp_mult_err_e10_e(
            lnm_0.val,
            lnm_0.err,
            y_0.val,
            y_0.err,
            result,
        );
        return if stat_e_0 != GSL_SUCCESS as libc::c_int {
            stat_e_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if b as libc::c_double <= x {
        let scale_factor_1: libc::c_double = 1.3407807929942596e+154f64;
        let mut scale_count_1: libc::c_int = 0 as libc::c_int;
        let mut stat_CF1: libc::c_int = 0;
        let mut ru: libc::c_double = 0.;
        let mut CF1_count: libc::c_int = 0;
        let mut a_target: libc::c_int = 0;
        let mut lnU_target: libc::c_double = 0.;
        let mut Ua_1: libc::c_double = 0.;
        let mut Uap1_1: libc::c_double = 0.;
        let mut Uam1_1: libc::c_double = 0.;
        let mut ap_1: libc::c_int = 0;
        if b < a + 1 as libc::c_int {
            a_target = b - 1 as libc::c_int;
            lnU_target = -a_target as libc::c_double * log(x);
        } else {
            a_target = 0 as libc::c_int;
            lnU_target = 0.0f64;
        }
        stat_CF1 = hyperg_U_CF1(
            a as libc::c_double,
            b as libc::c_double,
            0 as libc::c_int,
            x,
            &mut ru,
            &mut CF1_count,
        );
        Ua_1 = 1.0f64;
        Uap1_1 = ru / a as libc::c_double * Ua_1;
        ap_1 = a;
        while ap_1 > a_target {
            Uam1_1 = -((b as libc::c_double - 2.0f64 * ap_1 as libc::c_double - x) * Ua_1
                + ap_1 as libc::c_double
                    * (1.0f64 + ap_1 as libc::c_double - b as libc::c_double) * Uap1_1);
            Uap1_1 = Ua_1;
            Ua_1 = Uam1_1;
            let mut au0_1: libc::c_double = fabs(Ua_1);
            if au0_1 > scale_factor_1 {
                Ua_1 /= scale_factor_1;
                Uap1_1 /= scale_factor_1;
                scale_count_1 += 1;
                scale_count_1;
            } else if au0_1 < 1.0f64 / scale_factor_1 {
                Ua_1 *= scale_factor_1;
                Uap1_1 *= scale_factor_1;
                scale_count_1 -= 1;
                scale_count_1;
            }
            ap_1 -= 1;
            ap_1;
        }
        if Ua_1 == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            (*result).e10 = 0 as libc::c_int;
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
                1002 as libc::c_int,
                GSL_EZERODIV as libc::c_int,
            );
            return GSL_EZERODIV as libc::c_int;
        } else {
            let mut lnscl: libc::c_double = -scale_count_1 as libc::c_double
                * log(scale_factor_1);
            let mut lnpre_val: libc::c_double = lnU_target + lnscl;
            let mut lnpre_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(lnU_target) + fabs(lnscl));
            let mut oUa_err: libc::c_double = 2.0f64
                * (fabs((a_target - a) as libc::c_double) + CF1_count as libc::c_double
                    + 1.0f64) * 2.2204460492503131e-16f64 * fabs(1.0f64 / Ua_1);
            let mut stat_e_1: libc::c_int = gsl_sf_exp_mult_err_e10_e(
                lnpre_val,
                lnpre_err,
                1.0f64 / Ua_1,
                oUa_err,
                result,
            );
            return if stat_e_1 != GSL_SUCCESS as libc::c_int {
                stat_e_1
            } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
                stat_CF1
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    } else {
        let scale_factor_2: libc::c_double = 1.3407807929942596e+154f64;
        let mut scale_count_for: libc::c_int = 0 as libc::c_int;
        let mut scale_count_bck: libc::c_int = 0 as libc::c_int;
        let mut a0: libc::c_int = 1 as libc::c_int;
        let mut a1: libc::c_int = (a0 as libc::c_double
            + ceil(0.5f64 * (b as libc::c_double - x) - a0 as libc::c_double))
            as libc::c_int;
        let mut Ua1_bck_val: libc::c_double = 0.;
        let mut Ua1_bck_err: libc::c_double = 0.;
        let mut Ua1_for_val: libc::c_double = 0.;
        let mut Ua1_for_err: libc::c_double = 0.;
        let mut stat_for: libc::c_int = 0;
        let mut stat_bck: libc::c_int = 0;
        let mut lm_for: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ru_0: libc::c_double = 0.;
        let mut CF1_count_0: libc::c_int = 0;
        let mut stat_CF1_0: libc::c_int = hyperg_U_CF1(
            a as libc::c_double,
            b as libc::c_double,
            0 as libc::c_int,
            x,
            &mut ru_0,
            &mut CF1_count_0,
        );
        let mut Ua_2: libc::c_double = 1.0f64;
        let mut Uap1_2: libc::c_double = ru_0 / a as libc::c_double * Ua_2;
        let mut Uam1_2: libc::c_double = 0.;
        let mut ap_2: libc::c_int = 0;
        ap_2 = a;
        while ap_2 > a1 {
            Uam1_2 = -((b as libc::c_double - 2.0f64 * ap_2 as libc::c_double - x) * Ua_2
                + ap_2 as libc::c_double
                    * (1.0f64 + ap_2 as libc::c_double - b as libc::c_double) * Uap1_2);
            Uap1_2 = Ua_2;
            Ua_2 = Uam1_2;
            let mut au0_2: libc::c_double = fabs(Ua_2);
            if au0_2 > scale_factor_2 {
                Ua_2 /= scale_factor_2;
                Uap1_2 /= scale_factor_2;
                scale_count_bck += 1;
                scale_count_bck;
            } else if au0_2 < 1.0f64 / scale_factor_2 {
                Ua_2 *= scale_factor_2;
                Uap1_2 *= scale_factor_2;
                scale_count_bck -= 1;
                scale_count_bck;
            }
            ap_2 -= 1;
            ap_2;
        }
        Ua1_bck_val = Ua_2;
        Ua1_bck_err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs((a1 - a) as libc::c_double) + CF1_count_0 as libc::c_double + 1.0f64)
            * fabs(Ua_2);
        stat_bck = stat_CF1_0;
        if b == 2 as libc::c_int * a1 && a1 > 1 as libc::c_int {
            hyperg_lnU_beq2a(a1 as libc::c_double, x, &mut lm_for);
            Ua1_for_val = 1.0f64;
            Ua1_for_err = 0.0f64;
            stat_for = GSL_SUCCESS as libc::c_int;
        } else if b == 2 as libc::c_int * a1 - 1 as libc::c_int && a1 > 1 as libc::c_int
        {
            let mut lnU00: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lnU12: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut U00: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut U12: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            hyperg_lnU_beq2a(a1 as libc::c_double - 1.0f64, x, &mut lnU00);
            hyperg_lnU_beq2a(a1 as libc::c_double, x, &mut lnU12);
            if lnU00.val > lnU12.val {
                lm_for.val = lnU00.val;
                lm_for.err = lnU00.err;
                U00.val = 1.0f64;
                U00.err = 0.0f64;
                gsl_sf_exp_err_e(
                    lnU12.val - lm_for.val,
                    lnU12.err + lm_for.err,
                    &mut U12,
                );
            } else {
                lm_for.val = lnU12.val;
                lm_for.err = lnU12.err;
                U12.val = 1.0f64;
                U12.err = 0.0f64;
                gsl_sf_exp_err_e(
                    lnU00.val - lm_for.val,
                    lnU00.err + lm_for.err,
                    &mut U00,
                );
            }
            Ua1_for_val = (x * U12.val - U00.val)
                / (2.0f64 * a1 as libc::c_double - 2.0f64);
            Ua1_for_err = (fabs(x) * U12.err + U00.err)
                / fabs(2.0f64 * a1 as libc::c_double - 2.0f64);
            Ua1_for_err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(Ua1_for_val);
            stat_for = GSL_SUCCESS as libc::c_int;
        } else {
            let mut r_Ua_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut Uam1_3: libc::c_double = 1.0f64;
            let mut Ua_3: libc::c_double = 0.;
            let mut Uap1_3: libc::c_double = 0.;
            let mut ap_3: libc::c_int = 0;
            let mut lm_for_local: libc::c_double = 0.;
            stat_for = hyperg_U_small_a_bgt0(
                a0 as libc::c_double,
                b as libc::c_double,
                x,
                &mut r_Ua_0,
                &mut lm_for_local,
            );
            Ua_3 = r_Ua_0.val;
            Uam1_3 *= exp(-lm_for_local);
            lm_for.val = lm_for_local;
            lm_for.err = 0.0f64;
            ap_3 = a0;
            while ap_3 < a1 {
                Uap1_3 = -(Uam1_3
                    + (b as libc::c_double - 2.0f64 * ap_3 as libc::c_double - x) * Ua_3)
                    / (ap_3 as libc::c_double
                        * (1.0f64 + ap_3 as libc::c_double - b as libc::c_double));
                Uam1_3 = Ua_3;
                Ua_3 = Uap1_3;
                let mut au0_3: libc::c_double = fabs(Ua_3);
                if au0_3 > scale_factor_2 {
                    Ua_3 /= scale_factor_2;
                    Uam1_3 /= scale_factor_2;
                    scale_count_for += 1;
                    scale_count_for;
                } else if au0_3 < 1.0f64 / scale_factor_2 {
                    Ua_3 *= scale_factor_2;
                    Uam1_3 *= scale_factor_2;
                    scale_count_for -= 1;
                    scale_count_for;
                }
                ap_3 += 1;
                ap_3;
            }
            Ua1_for_val = Ua_3;
            Ua1_for_err = fabs(Ua_3) * fabs(r_Ua_0.err / r_Ua_0.val);
            Ua1_for_err
                += 2.0f64 * 2.2204460492503131e-16f64
                    * (fabs((a1 - a0) as libc::c_double) + 1.0f64) * fabs(Ua1_for_val);
        }
        if Ua1_bck_val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            (*result).e10 = 0 as libc::c_int;
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
                1127 as libc::c_int,
                GSL_EZERODIV as libc::c_int,
            );
            return GSL_EZERODIV as libc::c_int;
        } else if Ua1_for_val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            (*result).e10 = 0 as libc::c_int;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
                1131 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            let mut lns: libc::c_double = (scale_count_for - scale_count_bck)
                as libc::c_double * log(scale_factor_2);
            let mut ln_for_val: libc::c_double = log(fabs(Ua1_for_val));
            let mut ln_for_err: libc::c_double = 2.2204460492503131e-16f64
                + fabs(Ua1_for_err / Ua1_for_val);
            let mut ln_bck_val: libc::c_double = log(fabs(Ua1_bck_val));
            let mut ln_bck_err: libc::c_double = 2.2204460492503131e-16f64
                + fabs(Ua1_bck_err / Ua1_bck_val);
            let mut lnr_val: libc::c_double = lm_for.val + ln_for_val - ln_bck_val + lns;
            let mut lnr_err: libc::c_double = lm_for.err + ln_for_err + ln_bck_err
                + 2.0f64 * 2.2204460492503131e-16f64
                    * (fabs(lm_for.val) + fabs(ln_for_val) + fabs(ln_bck_val)
                        + fabs(lns));
            let mut sgn: libc::c_double = ((if Ua1_for_val >= 0.0f64 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            })
                * (if Ua1_bck_val >= 0.0f64 {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                })) as libc::c_double;
            let mut stat_e_2: libc::c_int = gsl_sf_exp_err_e10_e(
                lnr_val,
                lnr_err,
                result,
            );
            (*result).val *= sgn;
            return if stat_e_2 != GSL_SUCCESS as libc::c_int {
                stat_e_2
            } else if stat_bck != GSL_SUCCESS as libc::c_int {
                stat_bck
            } else if stat_for != GSL_SUCCESS as libc::c_int {
                stat_for
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
unsafe extern "C" fn hyperg_U_bge1(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    let rinta: libc::c_double = floor(a + 0.5f64);
    let a_neg_integer: libc::c_int = (a < 0.0f64
        && fabs(a - rinta) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if a == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else if a_neg_integer != 0
        && fabs(rinta) < 2147483647 as libc::c_int as libc::c_double
    {
        let n: libc::c_int = -(rinta as libc::c_int);
        let sgn: libc::c_double = if n & 1 as libc::c_int != 0 {
            -1.0f64
        } else {
            1.0f64
        };
        let mut lnfact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut L: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_L: libc::c_int = gsl_sf_laguerre_n_e(n, b - 1.0f64, x, &mut L);
        gsl_sf_lnfact_e(n as libc::c_uint, &mut lnfact);
        let stat_e: libc::c_int = gsl_sf_exp_mult_err_e10_e(
            lnfact.val,
            lnfact.err,
            sgn * L.val,
            L.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_L != GSL_SUCCESS as libc::c_int {
            stat_L
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if GSL_MAX_DBL(fabs(a), 1.0f64) * GSL_MAX_DBL(fabs(1.0f64 + a - b), 1.0f64)
        < 0.99f64 * fabs(x)
    {
        let ln_pre_val: libc::c_double = -a * log(x);
        let ln_pre_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
            * fabs(ln_pre_val);
        let mut asymp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_asymp: libc::c_int = hyperg_zaU_asymp(a, b, x, &mut asymp);
        let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e10_e(
            ln_pre_val,
            ln_pre_err,
            asymp.val,
            asymp.err,
            result,
        );
        return if stat_e_0 != GSL_SUCCESS as libc::c_int {
            stat_e_0
        } else if stat_asymp != GSL_SUCCESS as libc::c_int {
            stat_asymp
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if fabs(a) <= 1.0f64 {
        let mut rU: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_multiplier: libc::c_double = 0.;
        let mut stat_U: libc::c_int = hyperg_U_small_a_bgt0(
            a,
            b,
            x,
            &mut rU,
            &mut ln_multiplier,
        );
        let mut stat_e_1: libc::c_int = gsl_sf_exp_mult_err_e10_e(
            ln_multiplier,
            2.0f64 * 2.2204460492503131e-16f64 * fabs(ln_multiplier),
            rU.val,
            rU.err,
            result,
        );
        return if stat_U != GSL_SUCCESS as libc::c_int {
            stat_U
        } else if stat_e_1 != GSL_SUCCESS as libc::c_int {
            stat_e_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if fabs(a) < 5 as libc::c_int as libc::c_double
        && b < 5 as libc::c_int as libc::c_double && x < 2.0f64
        || fabs(a) < 10 as libc::c_int as libc::c_double
            && b < 10 as libc::c_int as libc::c_double && x < 1.0f64
    {
        let mut ser: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_ser: libc::c_int = hyperg_U_series(a, b, x, &mut ser);
        (*result).val = ser.val;
        (*result).err = ser.err;
        (*result).e10 = 0 as libc::c_int;
        return stat_ser;
    } else if a < 0.0f64 {
        let scale_factor: libc::c_double = 1.3407807929942596e+154f64;
        let a0: libc::c_double = a - floor(a) - 1.0f64;
        let b0: libc::c_double = b - floor(b) + 1.0f64;
        let mut scale_count: libc::c_int = 0 as libc::c_int;
        let mut lm_0: libc::c_double = 0.;
        let mut lm_1: libc::c_double = 0.;
        let mut lm_max: libc::c_double = 0.;
        let mut r_Uap1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Ua: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0: libc::c_int = hyperg_U_small_a_bgt0(
            a0 + 1.0f64,
            b0,
            x,
            &mut r_Uap1,
            &mut lm_0,
        );
        let mut stat_1: libc::c_int = hyperg_U_small_a_bgt0(
            a0,
            b0,
            x,
            &mut r_Ua,
            &mut lm_1,
        );
        let mut stat_e_2: libc::c_int = 0;
        let mut Uap1: libc::c_double = r_Uap1.val;
        let mut Ua: libc::c_double = r_Ua.val;
        let mut Uam1: libc::c_double = 0.;
        let mut ap: libc::c_double = 0.;
        lm_max = if lm_0 > lm_1 { lm_0 } else { lm_1 };
        Uap1 *= exp(lm_0 - lm_max);
        Ua *= exp(lm_1 - lm_max);
        ap = a0;
        while ap > a + 0.1f64 {
            Uam1 = ap * (b0 - ap - 1.0f64) * Uap1 + (x + 2.0f64 * ap - b0) * Ua;
            Uap1 = Ua;
            Ua = Uam1;
            let mut au0: libc::c_double = fabs(Ua);
            if au0 > scale_factor {
                Ua /= scale_factor;
                Uap1 /= scale_factor;
                scale_count += 1;
                scale_count;
            } else if au0 < 1.0f64 / scale_factor {
                Ua *= scale_factor;
                Uap1 *= scale_factor;
                scale_count -= 1;
                scale_count;
            }
            ap -= 1.0f64;
        }
        if b < 2.0f64 {
            let lnscale: libc::c_double = log(scale_factor);
            let mut lnm: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut y: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            lnm.val = lm_max + scale_count as libc::c_double * lnscale;
            lnm
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(lm_max) + scale_count as libc::c_double * fabs(lnscale));
            y.val = Ua;
            y.err = fabs(r_Uap1.err / r_Uap1.val) * fabs(Ua);
            y.err += fabs(r_Ua.err / r_Ua.val) * fabs(Ua);
            y.err
                += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(a - a0) + 1.0f64)
                    * fabs(Ua);
            y.err *= fabs(lm_0 - lm_max) + 1.0f64;
            y.err *= fabs(lm_1 - lm_max) + 1.0f64;
            stat_e_2 = gsl_sf_exp_mult_err_e10_e(lnm.val, lnm.err, y.val, y.err, result);
        } else {
            let err_mult: libc::c_double = fabs(b - b0) + fabs(a - a0) + 1.0f64;
            let lnscale_0: libc::c_double = log(scale_factor);
            let mut lnm_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut y_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut Ubm1: libc::c_double = Ua;
            let mut Ub: libc::c_double = (a * (b0 - a - 1.0f64) * Uap1 + (a + x) * Ua)
                / x;
            let mut Ubp1: libc::c_double = 0.;
            let mut bp: libc::c_double = 0.;
            bp = b0 + 1.0f64;
            while bp < b - 0.1f64 {
                Ubp1 = ((1.0f64 + a - bp) * Ubm1 + (bp + x - 1.0f64) * Ub) / x;
                Ubm1 = Ub;
                Ub = Ubp1;
                let mut au0_0: libc::c_double = fabs(Ub);
                if au0_0 > scale_factor {
                    Ub /= scale_factor;
                    Ubm1 /= scale_factor;
                    scale_count += 1;
                    scale_count;
                } else if au0_0 < 1.0f64 / scale_factor {
                    Ub *= scale_factor;
                    Ubm1 *= scale_factor;
                    scale_count -= 1;
                    scale_count;
                }
                bp += 1.0f64;
            }
            lnm_0.val = lm_max + scale_count as libc::c_double * lnscale_0;
            lnm_0
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(lm_max) + fabs(scale_count as libc::c_double * lnscale_0));
            y_0.val = Ub;
            y_0.err = 2.0f64 * err_mult * fabs(r_Uap1.err / r_Uap1.val) * fabs(Ub);
            y_0.err += 2.0f64 * err_mult * fabs(r_Ua.err / r_Ua.val) * fabs(Ub);
            y_0.err += 2.0f64 * 2.2204460492503131e-16f64 * err_mult * fabs(Ub);
            y_0.err *= fabs(lm_0 - lm_max) + 1.0f64;
            y_0.err *= fabs(lm_1 - lm_max) + 1.0f64;
            stat_e_2 = gsl_sf_exp_mult_err_e10_e(
                lnm_0.val,
                lnm_0.err,
                y_0.val,
                y_0.err,
                result,
            );
        }
        return if stat_e_2 != GSL_SUCCESS as libc::c_int {
            stat_e_2
        } else if stat_0 != GSL_SUCCESS as libc::c_int {
            stat_0
        } else if stat_1 != GSL_SUCCESS as libc::c_int {
            stat_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if b >= 2 as libc::c_int as libc::c_double * a + x {
        let mut scale_count_0: libc::c_int = 0 as libc::c_int;
        let a0_0: libc::c_double = a - floor(a);
        let scale_factor_0: libc::c_double = 1.3407807929942596e+154f64;
        let mut lnscale_1: libc::c_double = 0.;
        let mut lm_0_0: libc::c_double = 0.;
        let mut lm_1_0: libc::c_double = 0.;
        let mut lm_max_0: libc::c_double = 0.;
        let mut r_Uam1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Ua_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_0_0: libc::c_int = hyperg_U_small_a_bgt0(
            a0_0 - 1.0f64,
            b,
            x,
            &mut r_Uam1,
            &mut lm_0_0,
        );
        let mut stat_1_0: libc::c_int = hyperg_U_small_a_bgt0(
            a0_0,
            b,
            x,
            &mut r_Ua_0,
            &mut lm_1_0,
        );
        let mut stat_e_3: libc::c_int = 0;
        let mut lnm_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut y_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Uam1_0: libc::c_double = r_Uam1.val;
        let mut Ua_0: libc::c_double = r_Ua_0.val;
        let mut Uap1_0: libc::c_double = 0.;
        let mut ap_0: libc::c_double = 0.;
        lm_max_0 = if lm_0_0 > lm_1_0 { lm_0_0 } else { lm_1_0 };
        Uam1_0 *= exp(lm_0_0 - lm_max_0);
        Ua_0 *= exp(lm_1_0 - lm_max_0);
        ap_0 = a0_0;
        while ap_0 < a - 0.1f64 {
            Uap1_0 = -(Uam1_0 + (b - 2.0f64 * ap_0 - x) * Ua_0)
                / (ap_0 * (1.0f64 + ap_0 - b));
            Uam1_0 = Ua_0;
            Ua_0 = Uap1_0;
            let mut au0_1: libc::c_double = fabs(Ua_0);
            if au0_1 > scale_factor_0 {
                Ua_0 /= scale_factor_0;
                Uam1_0 /= scale_factor_0;
                scale_count_0 += 1;
                scale_count_0;
            } else if au0_1 < 1.0f64 / scale_factor_0 {
                Ua_0 *= scale_factor_0;
                Uam1_0 *= scale_factor_0;
                scale_count_0 -= 1;
                scale_count_0;
            }
            ap_0 += 1.0f64;
        }
        lnscale_1 = log(scale_factor_0);
        lnm_1.val = lm_max_0 + scale_count_0 as libc::c_double * lnscale_1;
        lnm_1
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(lm_max_0) + fabs(scale_count_0 as libc::c_double * lnscale_1));
        y_1.val = Ua_0;
        y_1.err = fabs(r_Uam1.err / r_Uam1.val) * fabs(Ua_0);
        y_1.err += fabs(r_Ua_0.err / r_Ua_0.val) * fabs(Ua_0);
        y_1.err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(a - a0_0) + 1.0f64)
                * fabs(Ua_0);
        y_1.err *= fabs(lm_0_0 - lm_max_0) + 1.0f64;
        y_1.err *= fabs(lm_1_0 - lm_max_0) + 1.0f64;
        stat_e_3 = gsl_sf_exp_mult_err_e10_e(
            lnm_1.val,
            lnm_1.err,
            y_1.val,
            y_1.err,
            result,
        );
        return if stat_e_3 != GSL_SUCCESS as libc::c_int {
            stat_e_3
        } else if stat_0_0 != GSL_SUCCESS as libc::c_int {
            stat_0_0
        } else if stat_1_0 != GSL_SUCCESS as libc::c_int {
            stat_1_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if b <= x {
        let a0_1: libc::c_double = a - floor(a);
        let scale_factor_1: libc::c_double = 1.3407807929942596e+154f64;
        let mut scale_count_1: libc::c_int = 0 as libc::c_int;
        let mut lnm_2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut y_2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnscale_2: libc::c_double = 0.;
        let mut lm_0_1: libc::c_double = 0.;
        let mut Uap1_1: libc::c_double = 0.;
        let mut Ua_1: libc::c_double = 0.;
        let mut Uam1_1: libc::c_double = 0.;
        let mut U0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ap_1: libc::c_double = 0.;
        let mut ru: libc::c_double = 0.;
        let mut r: libc::c_double = 0.;
        let mut CF1_count: libc::c_int = 0;
        let mut stat_CF1: libc::c_int = hyperg_U_CF1(
            a,
            b,
            0 as libc::c_int,
            x,
            &mut ru,
            &mut CF1_count,
        );
        let mut stat_U0: libc::c_int = 0;
        let mut stat_e_4: libc::c_int = 0;
        r = ru / a;
        Ua_1 = 1.4916681462400413e-154f64;
        Uap1_1 = r * Ua_1;
        ap_1 = a;
        while ap_1 > a0_1 + 0.1f64 {
            Uam1_1 = -((b - 2.0f64 * ap_1 - x) * Ua_1
                + ap_1 * (1.0f64 + ap_1 - b) * Uap1_1);
            Uap1_1 = Ua_1;
            Ua_1 = Uam1_1;
            let mut au0_2: libc::c_double = fabs(Ua_1);
            if au0_2 > scale_factor_1 {
                Ua_1 /= scale_factor_1;
                Uap1_1 /= scale_factor_1;
                scale_count_1 += 1;
                scale_count_1;
            } else if au0_2 < 1.0f64 / scale_factor_1 {
                Ua_1 *= scale_factor_1;
                Uap1_1 *= scale_factor_1;
                scale_count_1 -= 1;
                scale_count_1;
            }
            ap_1 -= 1.0f64;
        }
        stat_U0 = hyperg_U_small_a_bgt0(a0_1, b, x, &mut U0, &mut lm_0_1);
        lnscale_2 = log(scale_factor_1);
        lnm_2.val = lm_0_1 - scale_count_1 as libc::c_double * lnscale_2;
        lnm_2
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(lm_0_1) + fabs(scale_count_1 as libc::c_double * lnscale_2));
        y_2.val = 1.4916681462400413e-154f64 * (U0.val / Ua_1);
        y_2.err = 1.4916681462400413e-154f64 * (U0.err / fabs(Ua_1));
        y_2.err
            += 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(a0_1 - a) + CF1_count as libc::c_double + 1.0f64)
                * fabs(y_2.val);
        stat_e_4 = gsl_sf_exp_mult_err_e10_e(
            lnm_2.val,
            lnm_2.err,
            y_2.val,
            y_2.err,
            result,
        );
        return if stat_e_4 != GSL_SUCCESS as libc::c_int {
            stat_e_4
        } else if stat_U0 != GSL_SUCCESS as libc::c_int {
            stat_U0
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        let mut scale_count_for: libc::c_int = 0 as libc::c_int;
        let mut scale_count_bck: libc::c_int = 0 as libc::c_int;
        let scale_factor_2: libc::c_double = 1.3407807929942596e+154f64;
        let eps: libc::c_double = a - floor(a);
        let a0_2: libc::c_double = if eps == 0.0f64 { 1.0f64 } else { eps };
        let a1: libc::c_double = a0_2 + ceil(0.5f64 * (b - x) - a0_2);
        let mut lnm_3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut y_3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lm_for: libc::c_double = 0.;
        let mut lnscale_3: libc::c_double = 0.;
        let mut Ua1_bck: libc::c_double = 0.;
        let mut Ua1_for: libc::c_double = 0.;
        let mut stat_for: libc::c_int = 0;
        let mut stat_bck: libc::c_int = 0;
        let mut stat_e_5: libc::c_int = 0;
        let mut CF1_count_0: libc::c_int = 0;
        let mut Uap1_2: libc::c_double = 0.;
        let mut Ua_2: libc::c_double = 0.;
        let mut Uam1_2: libc::c_double = 0.;
        let mut ap_2: libc::c_double = 0.;
        let mut ru_0: libc::c_double = 0.;
        let mut r_0: libc::c_double = 0.;
        let mut stat_CF1_0: libc::c_int = hyperg_U_CF1(
            a,
            b,
            0 as libc::c_int,
            x,
            &mut ru_0,
            &mut CF1_count_0,
        );
        r_0 = ru_0 / a;
        Ua_2 = 1.4916681462400413e-154f64;
        Uap1_2 = r_0 * Ua_2;
        ap_2 = a;
        while ap_2 > a1 + 0.1f64 {
            Uam1_2 = -((b - 2.0f64 * ap_2 - x) * Ua_2
                + ap_2 * (1.0f64 + ap_2 - b) * Uap1_2);
            Uap1_2 = Ua_2;
            Ua_2 = Uam1_2;
            let mut au0_3: libc::c_double = fabs(Ua_2);
            if au0_3 > scale_factor_2 {
                Ua_2 /= scale_factor_2;
                Uap1_2 /= scale_factor_2;
                scale_count_bck += 1;
                scale_count_bck;
            } else if au0_3 < 1.0f64 / scale_factor_2 {
                Ua_2 *= scale_factor_2;
                Uap1_2 *= scale_factor_2;
                scale_count_bck -= 1;
                scale_count_bck;
            }
            ap_2 -= 1.0f64;
        }
        Ua1_bck = Ua_2;
        stat_bck = stat_CF1_0;
        let mut r_Uam1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut r_Ua_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lm_0_2: libc::c_double = 0.;
        let mut lm_1_1: libc::c_double = 0.;
        let mut stat_0_1: libc::c_int = hyperg_U_small_a_bgt0(
            a0_2 - 1.0f64,
            b,
            x,
            &mut r_Uam1_0,
            &mut lm_0_2,
        );
        let mut stat_1_1: libc::c_int = hyperg_U_small_a_bgt0(
            a0_2,
            b,
            x,
            &mut r_Ua_1,
            &mut lm_1_1,
        );
        let mut Uam1_3: libc::c_double = r_Uam1_0.val;
        let mut Ua_3: libc::c_double = r_Ua_1.val;
        let mut Uap1_3: libc::c_double = 0.;
        let mut ap_3: libc::c_double = 0.;
        lm_for = if lm_0_2 > lm_1_1 { lm_0_2 } else { lm_1_1 };
        Uam1_3 *= exp(lm_0_2 - lm_for);
        Ua_3 *= exp(lm_1_1 - lm_for);
        ap_3 = a0_2;
        while ap_3 < a1 - 0.1f64 {
            Uap1_3 = -(Uam1_3 + (b - 2.0f64 * ap_3 - x) * Ua_3)
                / (ap_3 * (1.0f64 + ap_3 - b));
            Uam1_3 = Ua_3;
            Ua_3 = Uap1_3;
            let mut au0_4: libc::c_double = fabs(Ua_3);
            if au0_4 > scale_factor_2 {
                Ua_3 /= scale_factor_2;
                Uam1_3 /= scale_factor_2;
                scale_count_for += 1;
                scale_count_for;
            } else if au0_4 < 1.0f64 / scale_factor_2 {
                Ua_3 *= scale_factor_2;
                Uam1_3 *= scale_factor_2;
                scale_count_for -= 1;
                scale_count_for;
            }
            ap_3 += 1.0f64;
        }
        Ua1_for = Ua_3;
        stat_for = if stat_0_1 != GSL_SUCCESS as libc::c_int {
            stat_0_1
        } else if stat_1_1 != GSL_SUCCESS as libc::c_int {
            stat_1_1
        } else {
            GSL_SUCCESS as libc::c_int
        };
        lnscale_3 = log(scale_factor_2);
        lnm_3
            .val = lm_for
            + (scale_count_for - scale_count_bck) as libc::c_double * lnscale_3;
        lnm_3
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(lm_for)
                + fabs((scale_count_for - scale_count_bck) as libc::c_double)
                    * fabs(lnscale_3));
        y_3.val = 1.4916681462400413e-154f64 * Ua1_for / Ua1_bck;
        y_3
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(a - a0_2) + CF1_count_0 as libc::c_double + 1.0f64) * fabs(y_3.val);
        stat_e_5 = gsl_sf_exp_mult_err_e10_e(
            lnm_3.val,
            lnm_3.err,
            y_3.val,
            y_3.err,
            result,
        );
        return if stat_e_5 != GSL_SUCCESS as libc::c_int {
            stat_e_5
        } else if stat_bck != GSL_SUCCESS as libc::c_int {
            stat_bck
        } else if stat_for != GSL_SUCCESS as libc::c_int {
            stat_for
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
unsafe extern "C" fn hyperg_U_origin(
    a: libc::c_double,
    b: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    let mut r1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut r2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_1: libc::c_int = gsl_sf_gammainv_e(
        1 as libc::c_int as libc::c_double + a - b,
        &mut r1,
    );
    let mut stat_2: libc::c_int = gsl_sf_gammainv_e(b, &mut r2);
    let mut factor: libc::c_double = 3.14159265358979323846f64
        / sin(3.14159265358979323846f64 * b);
    (*result).val = factor * r1.val * r2.val;
    (*result).err = fabs(factor) * (r1.err + r2.err);
    (*result).e10 = 0 as libc::c_int;
    return if stat_1 != GSL_SUCCESS as libc::c_int {
        stat_1
    } else if stat_2 != GSL_SUCCESS as libc::c_int {
        stat_2
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn hyperg_U_int_origin(
    a: libc::c_int,
    b: libc::c_int,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    return hyperg_U_origin(a as libc::c_double, b as libc::c_double, result);
}
unsafe extern "C" fn hyperg_U_negx(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    let mut r1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut r2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_1: libc::c_int = 0;
    let mut stat_2: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut a_int: libc::c_int = (a == floor(a)) as libc::c_int;
    let mut b_int: libc::c_int = (b == floor(b)) as libc::c_int;
    let mut T1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut T1_err: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut T2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut T2_err: libc::c_double = 0 as libc::c_int as libc::c_double;
    if b_int != 0 && b <= 0 as libc::c_int as libc::c_double
        && !(a_int != 0 && a <= 0 as libc::c_int as libc::c_double && a >= b)
    {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"limit case integer b <= 0 unimplemented\0" as *const u8
                as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            1548 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    } else {
        stat_1 = gsl_sf_poch_e(1 as libc::c_int as libc::c_double + a - b, -a, &mut r1);
        status = stat_1;
        if r1.val != 0.0f64 {
            let mut Mr1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_Mr1: libc::c_int = gsl_sf_hyperg_1F1_e(a, b, x, &mut Mr1);
            status = if status != GSL_SUCCESS as libc::c_int {
                status
            } else if stat_Mr1 != GSL_SUCCESS as libc::c_int {
                stat_Mr1
            } else {
                GSL_SUCCESS as libc::c_int
            };
            T1 = Mr1.val * r1.val;
            T1_err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(T1)
                + fabs(Mr1.err * r1.val) + fabs(Mr1.val * r1.err);
        }
    }
    if b_int != 0 && b >= 2 as libc::c_int as libc::c_double
        && !(a_int != 0 && a <= b - 2 as libc::c_int as libc::c_double)
    {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"limit case integer b >= 2 unimplemented\0" as *const u8
                as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            1581 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    } else {
        if a_int != 0 && a <= 0 as libc::c_int as libc::c_double
            && b >= 1 as libc::c_int as libc::c_double
        {
            r2.val = 0 as libc::c_int as libc::c_double;
            r2.err = 0 as libc::c_int as libc::c_double;
        } else {
            stat_2 = gsl_sf_poch_e(
                a,
                -(1 as libc::c_int as libc::c_double + a - b),
                &mut r2,
            );
            status = if status != GSL_SUCCESS as libc::c_int {
                status
            } else if stat_2 != GSL_SUCCESS as libc::c_int {
                stat_2
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
        if r2.val != 0.0f64 {
            let mut Mr2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_Mr2: libc::c_int = gsl_sf_hyperg_1F1_e(
                1 as libc::c_int as libc::c_double + a - b,
                2 as libc::c_int as libc::c_double - b,
                x,
                &mut Mr2,
            );
            T2 = Mr2.val * r2.val;
            T2_err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(T2)
                + fabs(Mr2.err * r2.val) + fabs(Mr2.val * r2.err);
            status = if status != GSL_SUCCESS as libc::c_int {
                status
            } else if stat_Mr2 != GSL_SUCCESS as libc::c_int {
                stat_Mr2
            } else {
                GSL_SUCCESS as libc::c_int
            };
            if T2 != 0.0f64 {
                let mut x1mb: libc::c_double = pow(
                    x,
                    1 as libc::c_int as libc::c_double - b,
                );
                T2 = x1mb * T2;
                T2_err = fabs(x1mb) * T2_err;
            }
        }
    }
    (*result).val = T1 + T2;
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
        + (T1_err + T2_err);
    (*result).e10 = 0 as libc::c_int;
    return status;
}
unsafe extern "C" fn hyperg_U_int_negx(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    if a < b && b <= 0 as libc::c_int {
        let mut r1: gsl_sf_result_e10 = gsl_sf_result_e10 {
            val: 0.,
            err: 0.,
            e10: 0,
        };
        let mut z21_z: libc::c_double = pow(x, (1 as libc::c_int - b) as libc::c_double);
        let mut status: libc::c_int = hyperg_U_negx(
            (1 as libc::c_int + a - b) as libc::c_double,
            (2 as libc::c_int - b) as libc::c_double,
            x,
            &mut r1,
        );
        let mut res_tem: libc::c_double = z21_z * r1.val;
        let mut res_tem_err: libc::c_double = fabs(z21_z) * r1.err;
        (*result).val = res_tem;
        (*result).err = res_tem_err;
        return status;
    } else {
        return hyperg_U_negx(a as libc::c_double, b as libc::c_double, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_U_int_e10_e(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    if x == 0.0f64 && b >= 1 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        (*result).e10 = 0 as libc::c_int;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            1656 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        return hyperg_U_int_origin(a, b, result)
    } else if x < 0.0f64 {
        return hyperg_U_int_negx(a, b, x, result)
    } else if b >= 1 as libc::c_int {
        return hyperg_U_int_bge1(a, b, x, result)
    } else {
        let mut U: gsl_sf_result_e10 = gsl_sf_result_e10 {
            val: 0.,
            err: 0.,
            e10: 0,
        };
        let mut ln_x: libc::c_double = log(x);
        let mut ap: libc::c_int = 1 as libc::c_int + a - b;
        let mut bp: libc::c_int = 2 as libc::c_int - b;
        let mut stat_e: libc::c_int = 0;
        let mut stat_U: libc::c_int = hyperg_U_int_bge1(ap, bp, x, &mut U);
        let mut ln_pre_val: libc::c_double = (1.0f64 - b as libc::c_double) * ln_x;
        let mut ln_pre_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(b as libc::c_double) + 1.0f64) * fabs(ln_x);
        ln_pre_err
            += 2.0f64 * 2.2204460492503131e-16f64 * fabs(1.0f64 - b as libc::c_double);
        stat_e = gsl_sf_exp_mult_err_e10_e(
            ln_pre_val + U.e10 as libc::c_double * 2.30258509299404568402f64,
            ln_pre_err,
            U.val,
            U.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_U != GSL_SUCCESS as libc::c_int {
            stat_U
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_U_e10_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result_e10,
) -> libc::c_int {
    let rinta: libc::c_double = floor(a + 0.5f64);
    let rintb: libc::c_double = floor(b + 0.5f64);
    let a_integer: libc::c_int = (fabs(a - rinta)
        < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    let b_integer: libc::c_int = (fabs(b - rintb)
        < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if x == 0.0f64 && b >= 1 as libc::c_int as libc::c_double {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        (*result).e10 = 0 as libc::c_int;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            1701 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if a == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else if x == 0.0f64 {
        return hyperg_U_origin(a, b, result)
    } else if a_integer != 0 && b == a + 1 as libc::c_int as libc::c_double {
        let mut powx1N_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_pow_int_e(x, -a as libc::c_int, &mut powx1N_1);
        (*result).val = powx1N_1.val;
        (*result).err = powx1N_1.err;
        (*result).e10 = 0 as libc::c_int;
        return GSL_SUCCESS as libc::c_int;
    } else if a_integer != 0 && b_integer != 0 {
        return gsl_sf_hyperg_U_int_e10_e(
            rinta as libc::c_int,
            rintb as libc::c_int,
            x,
            result,
        )
    } else if x < 0.0f64 {
        return hyperg_U_negx(a, b, x, result)
    } else if b >= 1.0f64 {
        return hyperg_U_bge1(a, b, x, result)
    } else {
        let lnx: libc::c_double = log(x);
        let ln_pre_val: libc::c_double = (1.0f64 - b) * lnx;
        let ln_pre_err: libc::c_double = fabs(lnx) * 2.0f64 * 2.2204460492503131e-16f64
            * (1.0f64 + fabs(b));
        let ap: libc::c_double = 1.0f64 + a - b;
        let bp: libc::c_double = 2.0f64 - b;
        let mut U: gsl_sf_result_e10 = gsl_sf_result_e10 {
            val: 0.,
            err: 0.,
            e10: 0,
        };
        let mut stat_U: libc::c_int = hyperg_U_bge1(ap, bp, x, &mut U);
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e10_e(
            ln_pre_val + U.e10 as libc::c_double * 2.30258509299404568402f64,
            ln_pre_err,
            U.val,
            U.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_U != GSL_SUCCESS as libc::c_int {
            stat_U
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_U_int_e(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut re: gsl_sf_result_e10 = {
        let mut init = gsl_sf_result_e10_struct {
            val: 0 as libc::c_int as libc::c_double,
            err: 0 as libc::c_int as libc::c_double,
            e10: 0 as libc::c_int,
        };
        init
    };
    let mut stat_U: libc::c_int = gsl_sf_hyperg_U_int_e10_e(a, b, x, &mut re);
    let mut stat_c: libc::c_int = gsl_sf_result_smash_e(&mut re, result);
    return if stat_c != GSL_SUCCESS as libc::c_int {
        stat_c
    } else if stat_U != GSL_SUCCESS as libc::c_int {
        stat_U
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_U_e(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut re: gsl_sf_result_e10 = {
        let mut init = gsl_sf_result_e10_struct {
            val: 0 as libc::c_int as libc::c_double,
            err: 0 as libc::c_int as libc::c_double,
            e10: 0 as libc::c_int,
        };
        init
    };
    let mut stat_U: libc::c_int = gsl_sf_hyperg_U_e10_e(a, b, x, &mut re);
    let mut stat_c: libc::c_int = gsl_sf_result_smash_e(&mut re, result);
    return if stat_c != GSL_SUCCESS as libc::c_int {
        stat_c
    } else if stat_U != GSL_SUCCESS as libc::c_int {
        stat_U
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_U_int(
    a: libc::c_int,
    b: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_U_int_e(a, b, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_U_int_e(a, b, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            1781 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hyperg_U(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_U_e(a, b, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_U_e(a, b, x, &result)\0" as *const u8 as *const libc::c_char,
            b"hyperg_U.c\0" as *const u8 as *const libc::c_char,
            1786 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
