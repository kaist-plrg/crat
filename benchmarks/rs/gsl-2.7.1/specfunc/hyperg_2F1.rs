use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_sgn_e(
        x: libc::c_double,
        result_lg: *mut gsl_sf_result,
        sgn: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_complex_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lnr: *mut gsl_sf_result,
        arg: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_psi_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_hyperg_1F1_e(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
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
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
unsafe extern "C" fn hyperg_2F1_series(
    a: libc::c_double,
    b: libc::c_double,
    c: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut sum_pos: libc::c_double = 1.0f64;
    let mut sum_neg: libc::c_double = 0.0f64;
    let mut del_pos: libc::c_double = 1.0f64;
    let mut del_neg: libc::c_double = 0.0f64;
    let mut del: libc::c_double = 1.0f64;
    let mut del_prev: libc::c_double = 0.;
    let mut k: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    if fabs(c) < 2.2204460492503131e-16f64 {
        (*result).val = 0.0f64;
        (*result).err = 1.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    loop {
        i += 1;
        if i > 30000 as libc::c_int {
            (*result).val = sum_pos - sum_neg;
            (*result).err = del_pos + del_neg;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * (sum_pos + sum_neg);
            (*result).err
                += 2.0f64 * 2.2204460492503131e-16f64 * (2.0f64 * sqrt(k) + 1.0f64)
                    * fabs((*result).val);
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int,
                GSL_EMAXITER as libc::c_int,
            );
            return GSL_EMAXITER as libc::c_int;
        }
        del_prev = del;
        del *= (a + k) * (b + k) * x / ((c + k) * (k + 1.0f64));
        if del > 0.0f64 {
            del_pos = del;
            sum_pos += del;
        } else if del == 0.0f64 {
            del_pos = 0.0f64;
            del_neg = 0.0f64;
            break;
        } else {
            del_neg = -del;
            sum_neg -= del;
        }
        if fabs(del_prev / (sum_pos - sum_neg)) < 2.2204460492503131e-16f64
            && fabs(del / (sum_pos - sum_neg)) < 2.2204460492503131e-16f64
        {
            break;
        }
        k += 1.0f64;
        if !(fabs((del_pos + del_neg) / (sum_pos - sum_neg)) > 2.2204460492503131e-16f64)
        {
            break;
        }
    }
    (*result).val = sum_pos - sum_neg;
    (*result).err = del_pos + del_neg;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * (sum_pos + sum_neg);
    (*result).err
        += 2.0f64 * 2.2204460492503131e-16f64 * (2.0f64 * sqrt(k) + 1.0f64)
            * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn hyperg_2F1_conj_series(
    aR: libc::c_double,
    aI: libc::c_double,
    c: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if c == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut sum_pos: libc::c_double = 1.0f64;
        let mut sum_neg: libc::c_double = 0.0f64;
        let mut del_pos: libc::c_double = 1.0f64;
        let mut del_neg: libc::c_double = 0.0f64;
        let mut del: libc::c_double = 1.0f64;
        let mut k: libc::c_double = 0.0f64;
        loop {
            del *= ((aR + k) * (aR + k) + aI * aI) / ((k + 1.0f64) * (c + k)) * x;
            if del >= 0.0f64 {
                del_pos = del;
                sum_pos += del;
            } else {
                del_neg = -del;
                sum_neg -= del;
            }
            if k > 30000 as libc::c_int as libc::c_double {
                (*result).val = sum_pos - sum_neg;
                (*result).err = del_pos + del_neg;
                (*result).err
                    += 2.0f64 * 2.2204460492503131e-16f64 * (sum_pos + sum_neg);
                (*result).err
                    += 2.0f64 * 2.2204460492503131e-16f64 * (2.0f64 * sqrt(k) + 1.0f64)
                        * fabs((*result).val);
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                    145 as libc::c_int,
                    GSL_EMAXITER as libc::c_int,
                );
                return GSL_EMAXITER as libc::c_int;
            }
            k += 1.0f64;
            if !(fabs((del_pos + del_neg) / (sum_pos - sum_neg))
                > 2.2204460492503131e-16f64)
            {
                break;
            }
        }
        (*result).val = sum_pos - sum_neg;
        (*result).err = del_pos + del_neg;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * (sum_pos + sum_neg);
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (2.0f64 * sqrt(k) + 1.0f64)
                * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn hyperg_2F1_luke(
    a: libc::c_double,
    b: libc::c_double,
    c: libc::c_double,
    xin: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut stat_iter: libc::c_int = 0;
    let RECUR_BIG: libc::c_double = 1.0e+50f64;
    let nmax: libc::c_int = 20000 as libc::c_int;
    let mut n: libc::c_int = 3 as libc::c_int;
    let x: libc::c_double = -xin;
    let x3: libc::c_double = x * x * x;
    let t0: libc::c_double = a * b / c;
    let t1: libc::c_double = (a + 1.0f64) * (b + 1.0f64) / (2.0f64 * c);
    let t2: libc::c_double = (a + 2.0f64) * (b + 2.0f64) / (2.0f64 * (c + 1.0f64));
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
        let mut npbm1: libc::c_double = n as libc::c_double + b
            - 1 as libc::c_int as libc::c_double;
        let mut npcm1: libc::c_double = n as libc::c_double + c
            - 1 as libc::c_int as libc::c_double;
        let mut npam2: libc::c_double = n as libc::c_double + a
            - 2 as libc::c_int as libc::c_double;
        let mut npbm2: libc::c_double = n as libc::c_double + b
            - 2 as libc::c_int as libc::c_double;
        let mut npcm2: libc::c_double = n as libc::c_double + c
            - 2 as libc::c_int as libc::c_double;
        let mut tnm1: libc::c_double = (2 as libc::c_int * n - 1 as libc::c_int)
            as libc::c_double;
        let mut tnm3: libc::c_double = (2 as libc::c_int * n - 3 as libc::c_int)
            as libc::c_double;
        let mut tnm5: libc::c_double = (2 as libc::c_int * n - 5 as libc::c_int)
            as libc::c_double;
        let mut n2: libc::c_double = (n * n) as libc::c_double;
        let mut F1: libc::c_double = (3.0f64 * n2
            + (a + b - 6 as libc::c_int as libc::c_double) * n as libc::c_double
            + 2 as libc::c_int as libc::c_double - a * b
            - 2 as libc::c_int as libc::c_double * (a + b))
            / (2 as libc::c_int as libc::c_double * tnm3 * npcm1);
        let mut F2: libc::c_double = -(3.0f64 * n2
            - (a + b + 6 as libc::c_int as libc::c_double) * n as libc::c_double
            + 2 as libc::c_int as libc::c_double - a * b) * npam1 * npbm1
            / (4 as libc::c_int as libc::c_double * tnm1 * tnm3 * npcm2 * npcm1);
        let mut F3: libc::c_double = npam2 * npam1 * npbm2 * npbm1
            * (n as libc::c_double - a - 2 as libc::c_int as libc::c_double)
            * (n as libc::c_double - b - 2 as libc::c_int as libc::c_double)
            / (8 as libc::c_int as libc::c_double * tnm3 * tnm3 * tnm5
                * (n as libc::c_double + c - 3 as libc::c_int as libc::c_double) * npcm2
                * npcm1);
        let mut E: libc::c_double = -npam1 * npbm1
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
    (*result).err = 2.0f64 * fabs(prec * F);
    (*result).err
        += 2.0f64 * 2.2204460492503131e-16f64 * (n as libc::c_double + 1.0f64) * fabs(F);
    (*result).err *= 8.0f64 * (fabs(a) + fabs(b) + 1.0f64);
    stat_iter = if n >= nmax {
        GSL_EMAXITER as libc::c_int
    } else {
        GSL_SUCCESS as libc::c_int
    };
    return stat_iter;
}
unsafe extern "C" fn hyperg_2F1_conj_luke(
    aR: libc::c_double,
    aI: libc::c_double,
    c: libc::c_double,
    xin: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut stat_iter: libc::c_int = 0;
    let RECUR_BIG: libc::c_double = 1.0e+50f64;
    let nmax: libc::c_int = 10000 as libc::c_int;
    let mut n: libc::c_int = 3 as libc::c_int;
    let x: libc::c_double = -xin;
    let x3: libc::c_double = x * x * x;
    let atimesb: libc::c_double = aR * aR + aI * aI;
    let apb: libc::c_double = 2.0f64 * aR;
    let t0: libc::c_double = atimesb / c;
    let t1: libc::c_double = (atimesb + apb + 1.0f64) / (2.0f64 * c);
    let t2: libc::c_double = (atimesb + 2.0f64 * apb + 4.0f64) / (2.0f64 * (c + 1.0f64));
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
        let mut nm1: libc::c_double = (n - 1 as libc::c_int) as libc::c_double;
        let mut nm2: libc::c_double = (n - 2 as libc::c_int) as libc::c_double;
        let mut npam1_npbm1: libc::c_double = atimesb + nm1 * apb + nm1 * nm1;
        let mut npam2_npbm2: libc::c_double = atimesb + nm2 * apb + nm2 * nm2;
        let mut npcm1: libc::c_double = nm1 + c;
        let mut npcm2: libc::c_double = nm2 + c;
        let mut tnm1: libc::c_double = (2 as libc::c_int * n - 1 as libc::c_int)
            as libc::c_double;
        let mut tnm3: libc::c_double = (2 as libc::c_int * n - 3 as libc::c_int)
            as libc::c_double;
        let mut tnm5: libc::c_double = (2 as libc::c_int * n - 5 as libc::c_int)
            as libc::c_double;
        let mut n2: libc::c_double = (n * n) as libc::c_double;
        let mut F1: libc::c_double = (3.0f64 * n2
            + (apb - 6 as libc::c_int as libc::c_double) * n as libc::c_double
            + 2 as libc::c_int as libc::c_double - atimesb
            - 2 as libc::c_int as libc::c_double * apb)
            / (2 as libc::c_int as libc::c_double * tnm3 * npcm1);
        let mut F2: libc::c_double = -(3.0f64 * n2
            - (apb + 6 as libc::c_int as libc::c_double) * n as libc::c_double
            + 2 as libc::c_int as libc::c_double - atimesb) * npam1_npbm1
            / (4 as libc::c_int as libc::c_double * tnm1 * tnm3 * npcm2 * npcm1);
        let mut F3: libc::c_double = npam2_npbm2 * npam1_npbm1
            * (nm2 * nm2 - nm2 * apb + atimesb)
            / (8 as libc::c_int as libc::c_double * tnm3 * tnm3 * tnm5
                * (n as libc::c_double + c - 3 as libc::c_int as libc::c_double) * npcm2
                * npcm1);
        let mut E: libc::c_double = -npam1_npbm1
            * (n as libc::c_double - c - 1 as libc::c_int as libc::c_double)
            / (2 as libc::c_int as libc::c_double * tnm3 * npcm2 * npcm1);
        let mut An: libc::c_double = (1.0f64 + F1 * x) * Anm1 + (E + F2 * x) * x * Anm2
            + F3 * x3 * Anm3;
        let mut Bn: libc::c_double = (1.0f64 + F1 * x) * Bnm1 + (E + F2 * x) * x * Bnm2
            + F3 * x3 * Bnm3;
        let mut r: libc::c_double = An / Bn;
        prec = fabs(F - r) / fabs(F);
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
    (*result).err = 2.0f64 * fabs(prec * F);
    (*result).err
        += 2.0f64 * 2.2204460492503131e-16f64 * (n as libc::c_double + 1.0f64) * fabs(F);
    (*result).err *= 8.0f64 * (fabs(aR) + fabs(aI) + 1.0f64);
    stat_iter = if n >= nmax {
        GSL_EMAXITER as libc::c_int
    } else {
        GSL_SUCCESS as libc::c_int
    };
    return stat_iter;
}
unsafe extern "C" fn hyperg_2F1_reflect(
    a: libc::c_double,
    b: libc::c_double,
    c: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let d: libc::c_double = c - a - b;
    let intd: libc::c_int = floor(d + 0.5f64) as libc::c_int;
    let d_integer: libc::c_int = (fabs(d - intd as libc::c_double)
        < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if d_integer != 0 {
        let ln_omx: libc::c_double = log(1.0f64 - x);
        let ad: libc::c_double = fabs(d);
        let mut stat_F2: libc::c_int = GSL_SUCCESS as libc::c_int;
        let mut sgn_2: libc::c_double = 0.;
        let mut F1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut F2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d1: libc::c_double = 0.;
        let mut d2: libc::c_double = 0.;
        let mut lng_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lng_ad2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lng_bd2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_c: libc::c_int = 0;
        let mut stat_ad2: libc::c_int = 0;
        let mut stat_bd2: libc::c_int = 0;
        if d >= 0.0f64 {
            d1 = d;
            d2 = 0.0f64;
        } else {
            d1 = 0.0f64;
            d2 = d;
        }
        stat_ad2 = gsl_sf_lngamma_e(a + d2, &mut lng_ad2);
        stat_bd2 = gsl_sf_lngamma_e(b + d2, &mut lng_bd2);
        stat_c = gsl_sf_lngamma_e(c, &mut lng_c);
        if ad < 2.2204460492503131e-16f64 {
            F1.val = 0.0f64;
            F1.err = 0.0f64;
        } else {
            let mut lng_ad: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lng_ad1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut lng_bd1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_ad: libc::c_int = gsl_sf_lngamma_e(ad, &mut lng_ad);
            let mut stat_ad1: libc::c_int = gsl_sf_lngamma_e(a + d1, &mut lng_ad1);
            let mut stat_bd1: libc::c_int = gsl_sf_lngamma_e(b + d1, &mut lng_bd1);
            if stat_ad1 == GSL_SUCCESS as libc::c_int
                && stat_bd1 == GSL_SUCCESS as libc::c_int
                && stat_ad == GSL_SUCCESS as libc::c_int
            {
                let mut i: libc::c_int = 0;
                let mut sum1: libc::c_double = 1.0f64;
                let mut term: libc::c_double = 1.0f64;
                let mut ln_pre1_val: libc::c_double = lng_ad.val + lng_c.val
                    + d2 * ln_omx - lng_ad1.val - lng_bd1.val;
                let mut ln_pre1_err: libc::c_double = lng_ad.err + lng_c.err
                    + lng_ad1.err + lng_bd1.err
                    + 2.2204460492503131e-16f64 * fabs(ln_pre1_val);
                let mut stat_e: libc::c_int = 0;
                i = 1 as libc::c_int;
                while (i as libc::c_double) < ad {
                    let mut j: libc::c_int = i - 1 as libc::c_int;
                    term
                        *= (a + d2 + j as libc::c_double)
                            * (b + d2 + j as libc::c_double)
                            / (1.0f64 + d2 + j as libc::c_double) / i as libc::c_double
                            * (1.0f64 - x);
                    sum1 += term;
                    i += 1;
                    i;
                }
                stat_e = gsl_sf_exp_mult_err_e(
                    ln_pre1_val,
                    ln_pre1_err,
                    sum1,
                    2.2204460492503131e-16f64 * fabs(sum1),
                    &mut F1,
                );
                if stat_e == GSL_EOVRFLW as libc::c_int {
                    (*result).val = ::std::f32::INFINITY as libc::c_double;
                    (*result).err = ::std::f32::INFINITY as libc::c_double;
                    gsl_error(
                        b"overflow\0" as *const u8 as *const libc::c_char,
                        b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                        440 as libc::c_int,
                        GSL_EOVRFLW as libc::c_int,
                    );
                    return GSL_EOVRFLW as libc::c_int;
                }
            } else {
                F1.val = 0.0f64;
                F1.err = 0.0f64;
            }
        }
        if stat_ad2 == GSL_SUCCESS as libc::c_int
            && stat_bd2 == GSL_SUCCESS as libc::c_int
        {
            let maxiter: libc::c_int = 2000 as libc::c_int;
            let mut psi_1: libc::c_double = -0.57721566490153286060651209008f64;
            let mut psi_1pd: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut psi_apd1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut psi_bpd1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_1pd: libc::c_int = gsl_sf_psi_e(1.0f64 + ad, &mut psi_1pd);
            let mut stat_apd1: libc::c_int = gsl_sf_psi_e(a + d1, &mut psi_apd1);
            let mut stat_bpd1: libc::c_int = gsl_sf_psi_e(b + d1, &mut psi_bpd1);
            let mut stat_dall: libc::c_int = if stat_1pd != GSL_SUCCESS as libc::c_int {
                stat_1pd
            } else if stat_apd1 != GSL_SUCCESS as libc::c_int {
                stat_apd1
            } else if stat_bpd1 != GSL_SUCCESS as libc::c_int {
                stat_bpd1
            } else {
                GSL_SUCCESS as libc::c_int
            };
            let mut psi_val: libc::c_double = psi_1 + psi_1pd.val - psi_apd1.val
                - psi_bpd1.val - ln_omx;
            let mut psi_err: libc::c_double = psi_1pd.err + psi_apd1.err + psi_bpd1.err
                + 2.2204460492503131e-16f64 * fabs(psi_val);
            let mut fact: libc::c_double = 1.0f64;
            let mut sum2_val: libc::c_double = psi_val;
            let mut sum2_err: libc::c_double = psi_err;
            let mut ln_pre2_val: libc::c_double = lng_c.val + d1 * ln_omx - lng_ad2.val
                - lng_bd2.val;
            let mut ln_pre2_err: libc::c_double = lng_c.err + lng_ad2.err + lng_bd2.err
                + 2.2204460492503131e-16f64 * fabs(ln_pre2_val);
            let mut stat_e_0: libc::c_int = 0;
            let mut j_0: libc::c_int = 0;
            j_0 = 1 as libc::c_int;
            while j_0 < maxiter {
                let mut term1: libc::c_double = 1.0f64 / j_0 as libc::c_double
                    + 1.0f64 / (ad + j_0 as libc::c_double);
                let mut term2: libc::c_double = 1.0f64
                    / (a + d1 + j_0 as libc::c_double - 1.0f64)
                    + 1.0f64 / (b + d1 + j_0 as libc::c_double - 1.0f64);
                let mut delta: libc::c_double = 0.0f64;
                psi_val += term1 - term2;
                psi_err += 2.2204460492503131e-16f64 * (fabs(term1) + fabs(term2));
                fact
                    *= (a + d1 + j_0 as libc::c_double - 1.0f64)
                        * (b + d1 + j_0 as libc::c_double - 1.0f64)
                        / ((ad + j_0 as libc::c_double) * j_0 as libc::c_double)
                        * (1.0f64 - x);
                delta = fact * psi_val;
                sum2_val += delta;
                sum2_err
                    += fabs(fact * psi_err) + 2.2204460492503131e-16f64 * fabs(delta);
                if fabs(delta) < 2.2204460492503131e-16f64 * fabs(sum2_val) {
                    break;
                }
                j_0 += 1;
                j_0;
            }
            if j_0 == maxiter {
                stat_F2 = GSL_EMAXITER as libc::c_int;
            }
            if sum2_val == 0.0f64 {
                F2.val = 0.0f64;
                F2.err = 0.0f64;
            } else {
                stat_e_0 = gsl_sf_exp_mult_err_e(
                    ln_pre2_val,
                    ln_pre2_err,
                    sum2_val,
                    sum2_err,
                    &mut F2,
                );
                if stat_e_0 == GSL_EOVRFLW as libc::c_int {
                    (*result).val = 0.0f64;
                    (*result).err = 0.0f64;
                    gsl_error(
                        b"error\0" as *const u8 as *const libc::c_char,
                        b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                        509 as libc::c_int,
                        GSL_EOVRFLW as libc::c_int,
                    );
                    return GSL_EOVRFLW as libc::c_int;
                }
            }
            stat_F2 = if stat_F2 != GSL_SUCCESS as libc::c_int {
                stat_F2
            } else if stat_dall != GSL_SUCCESS as libc::c_int {
                stat_dall
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            F2.val = 0.0f64;
            F2.err = 0.0f64;
        }
        sgn_2 = if intd & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 };
        (*result).val = F1.val + sgn_2 * F2.val;
        (*result).err = F1.err + F2.err;
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(F1.val) + fabs(F2.val));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_F2;
    } else {
        let mut pre1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut pre2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn1: libc::c_double = 0.;
        let mut sgn2: libc::c_double = 0.;
        let mut F1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut F2_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_F1: libc::c_int = 0;
        let mut status_F2: libc::c_int = 0;
        let mut ln_g1ca: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_g1cb: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_g2a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_g2b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn_g1ca: libc::c_double = 0.;
        let mut sgn_g1cb: libc::c_double = 0.;
        let mut sgn_g2a: libc::c_double = 0.;
        let mut sgn_g2b: libc::c_double = 0.;
        let mut stat_1ca: libc::c_int = gsl_sf_lngamma_sgn_e(
            c - a,
            &mut ln_g1ca,
            &mut sgn_g1ca,
        );
        let mut stat_1cb: libc::c_int = gsl_sf_lngamma_sgn_e(
            c - b,
            &mut ln_g1cb,
            &mut sgn_g1cb,
        );
        let mut stat_2a: libc::c_int = gsl_sf_lngamma_sgn_e(
            a,
            &mut ln_g2a,
            &mut sgn_g2a,
        );
        let mut stat_2b: libc::c_int = gsl_sf_lngamma_sgn_e(
            b,
            &mut ln_g2b,
            &mut sgn_g2b,
        );
        let mut ok1: libc::c_int = (stat_1ca == GSL_SUCCESS as libc::c_int
            && stat_1cb == GSL_SUCCESS as libc::c_int) as libc::c_int;
        let mut ok2: libc::c_int = (stat_2a == GSL_SUCCESS as libc::c_int
            && stat_2b == GSL_SUCCESS as libc::c_int) as libc::c_int;
        let mut ln_gc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_gd: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_gmd: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn_gc: libc::c_double = 0.;
        let mut sgn_gd: libc::c_double = 0.;
        let mut sgn_gmd: libc::c_double = 0.;
        gsl_sf_lngamma_sgn_e(c, &mut ln_gc, &mut sgn_gc);
        gsl_sf_lngamma_sgn_e(d, &mut ln_gd, &mut sgn_gd);
        gsl_sf_lngamma_sgn_e(-d, &mut ln_gmd, &mut sgn_gmd);
        sgn1 = sgn_gc * sgn_gd * sgn_g1ca * sgn_g1cb;
        sgn2 = sgn_gc * sgn_gmd * sgn_g2a * sgn_g2b;
        if ok1 != 0 && ok2 != 0 {
            let mut ln_pre1_val_0: libc::c_double = ln_gc.val + ln_gd.val - ln_g1ca.val
                - ln_g1cb.val;
            let mut ln_pre2_val_0: libc::c_double = ln_gc.val + ln_gmd.val - ln_g2a.val
                - ln_g2b.val + d * log(1.0f64 - x);
            let mut ln_pre1_err_0: libc::c_double = ln_gc.err + ln_gd.err + ln_g1ca.err
                + ln_g1cb.err;
            let mut ln_pre2_err_0: libc::c_double = ln_gc.err + ln_gmd.err + ln_g2a.err
                + ln_g2b.err;
            if ln_pre1_val_0 < 7.0978271289338397e+02f64
                && ln_pre2_val_0 < 7.0978271289338397e+02f64
            {
                gsl_sf_exp_err_e(ln_pre1_val_0, ln_pre1_err_0, &mut pre1);
                gsl_sf_exp_err_e(ln_pre2_val_0, ln_pre2_err_0, &mut pre2);
                pre1.val *= sgn1;
                pre2.val *= sgn2;
            } else {
                (*result).val = ::std::f32::INFINITY as libc::c_double;
                (*result).err = ::std::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const libc::c_char,
                    b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                    570 as libc::c_int,
                    GSL_EOVRFLW as libc::c_int,
                );
                return GSL_EOVRFLW as libc::c_int;
            }
        } else if ok1 != 0 && ok2 == 0 {
            let mut ln_pre1_val_1: libc::c_double = ln_gc.val + ln_gd.val - ln_g1ca.val
                - ln_g1cb.val;
            let mut ln_pre1_err_1: libc::c_double = ln_gc.err + ln_gd.err + ln_g1ca.err
                + ln_g1cb.err;
            if ln_pre1_val_1 < 7.0978271289338397e+02f64 {
                gsl_sf_exp_err_e(ln_pre1_val_1, ln_pre1_err_1, &mut pre1);
                pre1.val *= sgn1;
                pre2.val = 0.0f64;
                pre2.err = 0.0f64;
            } else {
                (*result).val = ::std::f32::INFINITY as libc::c_double;
                (*result).err = ::std::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const libc::c_char,
                    b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                    583 as libc::c_int,
                    GSL_EOVRFLW as libc::c_int,
                );
                return GSL_EOVRFLW as libc::c_int;
            }
        } else if ok1 == 0 && ok2 != 0 {
            let mut ln_pre2_val_1: libc::c_double = ln_gc.val + ln_gmd.val - ln_g2a.val
                - ln_g2b.val + d * log(1.0f64 - x);
            let mut ln_pre2_err_1: libc::c_double = ln_gc.err + ln_gmd.err + ln_g2a.err
                + ln_g2b.err;
            if ln_pre2_val_1 < 7.0978271289338397e+02f64 {
                pre1.val = 0.0f64;
                pre1.err = 0.0f64;
                gsl_sf_exp_err_e(ln_pre2_val_1, ln_pre2_err_1, &mut pre2);
                pre2.val *= sgn2;
            } else {
                (*result).val = ::std::f32::INFINITY as libc::c_double;
                (*result).err = ::std::f32::INFINITY as libc::c_double;
                gsl_error(
                    b"overflow\0" as *const u8 as *const libc::c_char,
                    b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                    596 as libc::c_int,
                    GSL_EOVRFLW as libc::c_int,
                );
                return GSL_EOVRFLW as libc::c_int;
            }
        } else {
            pre1.val = 0.0f64;
            pre2.val = 0.0f64;
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                602 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        status_F1 = hyperg_2F1_series(a, b, 1.0f64 - d, 1.0f64 - x, &mut F1_0);
        status_F2 = hyperg_2F1_series(c - a, c - b, 1.0f64 + d, 1.0f64 - x, &mut F2_0);
        (*result).val = pre1.val * F1_0.val + pre2.val * F2_0.val;
        (*result).err = fabs(pre1.val * F1_0.err) + fabs(pre2.val * F2_0.err);
        (*result).err += fabs(pre1.err * F1_0.val) + fabs(pre2.err * F2_0.val);
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(pre1.val * F1_0.val) + fabs(pre2.val * F2_0.val));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if status_F1 != 0 {
            return status_F1;
        }
        if status_F2 != 0 {
            return status_F2;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn pow_omx(
    x: libc::c_double,
    p: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ln_omx: libc::c_double = 0.;
    let mut ln_result: libc::c_double = 0.;
    if fabs(x) < 7.4009597974140505e-04f64 {
        ln_omx = -x
            * (1.0f64
                + x
                    * (1.0f64 / 2.0f64
                        + x * (1.0f64 / 3.0f64 + x / 4.0f64 + x * x / 5.0f64)));
    } else {
        ln_omx = log(1.0f64 - x);
    }
    ln_result = p * ln_omx;
    return gsl_sf_exp_err_e(
        ln_result,
        2.2204460492503131e-16f64 * fabs(ln_result),
        result,
    );
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1_e(
    mut a: libc::c_double,
    mut b: libc::c_double,
    c: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let d: libc::c_double = c - a - b;
    let rinta: libc::c_double = floor(a + 0.5f64);
    let rintb: libc::c_double = floor(b + 0.5f64);
    let rintc: libc::c_double = floor(c + 0.5f64);
    let a_neg_integer: libc::c_int = (a < 0.0f64
        && fabs(a - rinta) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    let b_neg_integer: libc::c_int = (b < 0.0f64
        && fabs(b - rintb) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    let c_neg_integer: libc::c_int = (c < 0.0f64
        && fabs(c - rintc) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    (*result).val = 0.0f64;
    (*result).err = 0.0f64;
    if fabs(x - 1.0f64) < 1000.0f64 * 2.2204460492503131e-16f64
        && c - a - b > 0 as libc::c_int as libc::c_double
        && c != 0 as libc::c_int as libc::c_double && c_neg_integer == 0
    {
        let mut lngamc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lngamcab: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lngamca: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lngamcb: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lngamc_sgn: libc::c_double = 0.;
        let mut lngamca_sgn: libc::c_double = 0.;
        let mut lngamcb_sgn: libc::c_double = 0.;
        let mut status: libc::c_int = 0;
        let mut stat1: libc::c_int = gsl_sf_lngamma_sgn_e(
            c,
            &mut lngamc,
            &mut lngamc_sgn,
        );
        let mut stat2: libc::c_int = gsl_sf_lngamma_e(c - a - b, &mut lngamcab);
        let mut stat3: libc::c_int = gsl_sf_lngamma_sgn_e(
            c - a,
            &mut lngamca,
            &mut lngamca_sgn,
        );
        let mut stat4: libc::c_int = gsl_sf_lngamma_sgn_e(
            c - b,
            &mut lngamcb,
            &mut lngamcb_sgn,
        );
        if stat1 != GSL_SUCCESS as libc::c_int || stat2 != GSL_SUCCESS as libc::c_int
            || stat3 != GSL_SUCCESS as libc::c_int || stat4 != GSL_SUCCESS as libc::c_int
        {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                672 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
        status = gsl_sf_exp_err_e(
            lngamc.val + lngamcab.val - lngamca.val - lngamcb.val,
            lngamc.err + lngamcab.err + lngamca.err + lngamcb.err,
            result,
        );
        (*result).val *= lngamc_sgn / (lngamca_sgn * lngamcb_sgn);
        return status;
    }
    if x < -1.0f64 || 1.0f64 <= x {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            685 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if c_neg_integer != 0 {
        if !(a_neg_integer != 0 && a > c + 0.1f64)
            && !(b_neg_integer != 0 && b > c + 0.1f64)
        {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                693 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
    }
    if fabs(c - b) < 1000.0f64 * 2.2204460492503131e-16f64
        || fabs(c - a) < 1000.0f64 * 2.2204460492503131e-16f64
    {
        return pow_omx(x, d, result);
    }
    if a >= 0.0f64 && b >= 0.0f64 && c >= 0.0f64 && x >= 0.0f64 && x < 0.995f64 {
        return hyperg_2F1_series(a, b, c, x, result);
    }
    if fabs(a) < 10.0f64 && fabs(b) < 10.0f64 {
        if a_neg_integer != 0 {
            return hyperg_2F1_series(rinta, b, c, x, result);
        }
        if b_neg_integer != 0 {
            return hyperg_2F1_series(a, rintb, c, x, result);
        }
        if x < -0.25f64 {
            return hyperg_2F1_luke(a, b, c, x, result)
        } else if x < 0.5f64 {
            return hyperg_2F1_series(a, b, c, x, result)
        } else if fabs(c) > 10.0f64 {
            return hyperg_2F1_series(a, b, c, x, result)
        } else {
            return hyperg_2F1_reflect(a, b, c, x, result)
        }
    } else {
        let mut ap: libc::c_double = 0.;
        let mut bp: libc::c_double = 0.;
        if fabs(a) > fabs(b) {
            bp = a;
            ap = b;
        } else {
            bp = b;
            ap = a;
        }
        if x < 0.0f64 {
            return hyperg_2F1_luke(a, b, c, x, result);
        }
        if GSL_MAX_DBL(fabs(ap), 1.0f64) * fabs(bp) * fabs(x) < 2.0f64 * fabs(c) {
            return hyperg_2F1_series(a, b, c, x, result);
        }
        if fabs(bp * bp * x * x) < 0.001f64 * fabs(bp) && fabs(ap) < 10.0f64 {
            let mut stat: libc::c_int = gsl_sf_hyperg_1F1_e(ap, c, bp * x, result);
            (*result).err = 0.001f64 * fabs((*result).val);
            return stat;
        }
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            773 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1_conj_e(
    aR: libc::c_double,
    aI: libc::c_double,
    c: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    let rintc: libc::c_double = floor(c + 0.5f64);
    let c_neg_integer: libc::c_int = (c < 0.0f64
        && fabs(c - rintc) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    (*result).val = 0.0f64;
    (*result).err = 0.0f64;
    if ax >= 1.0f64 || c_neg_integer != 0 || c == 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            791 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    if ax < 0.25f64 && fabs(aR) < 20.0f64 && fabs(aI) < 20.0f64
        || c > 0.0f64 && x > 0.0f64
    {
        return hyperg_2F1_conj_series(aR, aI, c, x, result)
    } else if fabs(aR) < 10.0f64 && fabs(aI) < 10.0f64 {
        if x < -0.25f64 {
            return hyperg_2F1_conj_luke(aR, aI, c, x, result)
        } else {
            return hyperg_2F1_conj_series(aR, aI, c, x, result)
        }
    } else {
        if x < 0.0f64 {
            return hyperg_2F1_conj_luke(aR, aI, c, x, result);
        }
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            817 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1_renorm_e(
    a: libc::c_double,
    b: libc::c_double,
    c: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let rinta: libc::c_double = floor(a + 0.5f64);
    let rintb: libc::c_double = floor(b + 0.5f64);
    let rintc: libc::c_double = floor(c + 0.5f64);
    let a_neg_integer: libc::c_int = (a < 0.0f64
        && fabs(a - rinta) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    let b_neg_integer: libc::c_int = (b < 0.0f64
        && fabs(b - rintb) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    let c_neg_integer: libc::c_int = (c < 0.0f64
        && fabs(c - rintc) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if c_neg_integer != 0 {
        if a_neg_integer != 0 && a > c + 0.1f64 || b_neg_integer != 0 && b > c + 0.1f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut g1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut g2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut g3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut g4: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut g5: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut s1: libc::c_double = 0.;
            let mut s2: libc::c_double = 0.;
            let mut s3: libc::c_double = 0.;
            let mut s4: libc::c_double = 0.;
            let mut s5: libc::c_double = 0.;
            let mut stat: libc::c_int = 0 as libc::c_int;
            stat
                += gsl_sf_lngamma_sgn_e(
                    a - c + 1 as libc::c_int as libc::c_double,
                    &mut g1,
                    &mut s1,
                );
            stat
                += gsl_sf_lngamma_sgn_e(
                    b - c + 1 as libc::c_int as libc::c_double,
                    &mut g2,
                    &mut s2,
                );
            stat += gsl_sf_lngamma_sgn_e(a, &mut g3, &mut s3);
            stat += gsl_sf_lngamma_sgn_e(b, &mut g4, &mut s4);
            stat
                += gsl_sf_lngamma_sgn_e(
                    -c + 2 as libc::c_int as libc::c_double,
                    &mut g5,
                    &mut s5,
                );
            if stat != 0 as libc::c_int {
                (*result).val = ::std::f32::NAN as libc::c_double;
                (*result).err = ::std::f32::NAN as libc::c_double;
                gsl_error(
                    b"domain error\0" as *const u8 as *const libc::c_char,
                    b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                    854 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            } else {
                let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
                let mut stat_F: libc::c_int = gsl_sf_hyperg_2F1_e(
                    a - c + 1 as libc::c_int as libc::c_double,
                    b - c + 1 as libc::c_int as libc::c_double,
                    -c + 2 as libc::c_int as libc::c_double,
                    x,
                    &mut F,
                );
                let mut ln_pre_val: libc::c_double = g1.val + g2.val - g3.val - g4.val
                    - g5.val;
                let mut ln_pre_err: libc::c_double = g1.err + g2.err + g3.err + g4.err
                    + g5.err;
                let mut sg: libc::c_double = s1 * s2 * s3 * s4 * s5;
                let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
                    ln_pre_val,
                    ln_pre_err,
                    sg * F.val,
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
            }
        }
    } else {
        let mut F_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lng: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn: libc::c_double = 0.;
        let mut stat_g: libc::c_int = gsl_sf_lngamma_sgn_e(c, &mut lng, &mut sgn);
        let mut stat_F_0: libc::c_int = gsl_sf_hyperg_2F1_e(a, b, c, x, &mut F_0);
        let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e(
            -lng.val,
            lng.err,
            sgn * F_0.val,
            F_0.err,
            result,
        );
        return if stat_e_0 != GSL_SUCCESS as libc::c_int {
            stat_e_0
        } else if stat_F_0 != GSL_SUCCESS as libc::c_int {
            stat_F_0
        } else if stat_g != GSL_SUCCESS as libc::c_int {
            stat_g
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1_conj_renorm_e(
    aR: libc::c_double,
    aI: libc::c_double,
    c: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let rintc: libc::c_double = floor(c + 0.5f64);
    let rinta: libc::c_double = floor(aR + 0.5f64);
    let a_neg_integer: libc::c_int = (aR < 0.0f64
        && fabs(aR - rinta) < 1000.0f64 * 2.2204460492503131e-16f64 && aI == 0.0f64)
        as libc::c_int;
    let c_neg_integer: libc::c_int = (c < 0.0f64
        && fabs(c - rintc) < 1000.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    if c_neg_integer != 0 {
        if a_neg_integer != 0 && aR > c + 0.1f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut g1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut g2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut g3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut a1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut a2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat: libc::c_int = 0 as libc::c_int;
            stat
                += gsl_sf_lngamma_complex_e(
                    aR - c + 1 as libc::c_int as libc::c_double,
                    aI,
                    &mut g1,
                    &mut a1,
                );
            stat += gsl_sf_lngamma_complex_e(aR, aI, &mut g2, &mut a2);
            stat += gsl_sf_lngamma_e(-c + 2.0f64, &mut g3);
            if stat != 0 as libc::c_int {
                (*result).val = ::std::f32::NAN as libc::c_double;
                (*result).err = ::std::f32::NAN as libc::c_double;
                gsl_error(
                    b"domain error\0" as *const u8 as *const libc::c_char,
                    b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
                    913 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            } else {
                let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
                let mut stat_F: libc::c_int = gsl_sf_hyperg_2F1_conj_e(
                    aR - c + 1 as libc::c_int as libc::c_double,
                    aI,
                    -c + 2 as libc::c_int as libc::c_double,
                    x,
                    &mut F,
                );
                let mut ln_pre_val: libc::c_double = 2.0f64 * (g1.val - g2.val) - g3.val;
                let mut ln_pre_err: libc::c_double = 2.0f64 * (g1.err + g2.err) + g3.err;
                let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
                    ln_pre_val,
                    ln_pre_err,
                    F.val,
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
            }
        }
    } else {
        let mut F_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lng: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn: libc::c_double = 0.;
        let mut stat_g: libc::c_int = gsl_sf_lngamma_sgn_e(c, &mut lng, &mut sgn);
        let mut stat_F_0: libc::c_int = gsl_sf_hyperg_2F1_conj_e(aR, aI, c, x, &mut F_0);
        let mut stat_e_0: libc::c_int = gsl_sf_exp_mult_err_e(
            -lng.val,
            lng.err,
            sgn * F_0.val,
            F_0.err,
            result,
        );
        return if stat_e_0 != GSL_SUCCESS as libc::c_int {
            stat_e_0
        } else if stat_F_0 != GSL_SUCCESS as libc::c_int {
            stat_F_0
        } else if stat_g != GSL_SUCCESS as libc::c_int {
            stat_g
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_2F1_e(a, b, c, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_2F1_e(a, b, c, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            948 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1_conj(
    mut aR: libc::c_double,
    mut aI: libc::c_double,
    mut c: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_2F1_conj_e(aR, aI, c, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_2F1_conj_e(aR, aI, c, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            953 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1_renorm(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_2F1_renorm_e(a, b, c, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_2F1_renorm_e(a, b, c, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            958 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hyperg_2F1_conj_renorm(
    mut aR: libc::c_double,
    mut aI: libc::c_double,
    mut c: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hyperg_2F1_conj_renorm_e(
        aR,
        aI,
        c,
        x,
        &mut result,
    );
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hyperg_2F1_conj_renorm_e(aR, aI, c, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"hyperg_2F1.c\0" as *const u8 as *const libc::c_char,
            963 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
