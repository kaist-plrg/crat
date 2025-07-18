use ::libc;
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_clausen_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_complex_log_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lnr: *mut gsl_sf_result,
        theta: *mut gsl_sf_result,
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
unsafe extern "C" fn dilog_series_1(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let kmax: libc::c_int = 1000 as libc::c_int;
    let mut sum: libc::c_double = x;
    let mut term: libc::c_double = x;
    let mut k: libc::c_int = 0;
    k = 2 as libc::c_int;
    while k < kmax {
        let rk: libc::c_double = (k as libc::c_double - 1.0f64) / k as libc::c_double;
        term *= x;
        term *= rk * rk;
        sum += term;
        if fabs(term / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        k += 1;
        k;
    }
    (*result).val = sum;
    (*result).err = 2.0f64 * fabs(term);
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    if k == kmax {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"dilog.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn series_2(
    mut r: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    static mut kmax: libc::c_int = 100 as libc::c_int;
    let mut rk: libc::c_double = r;
    let mut sum: libc::c_double = 0.5f64 * r;
    let mut k: libc::c_int = 0;
    k = 2 as libc::c_int;
    while k < 10 as libc::c_int {
        let mut ds: libc::c_double = 0.;
        rk *= r;
        ds = rk / ((k * k) as libc::c_double * (k as libc::c_double + 1.0f64));
        sum += ds;
        k += 1;
        k;
    }
    while k < kmax {
        let mut ds_0: libc::c_double = 0.;
        rk *= r;
        ds_0 = rk / ((k * k) as libc::c_double * (k as libc::c_double + 1.0f64));
        sum += ds_0;
        if fabs(ds_0 / sum) < 0.5f64 * 2.2204460492503131e-16f64 {
            break;
        }
        k += 1;
        k;
    }
    (*result).val = sum;
    (*result)
        .err = 2.0f64 * kmax as libc::c_double * 2.2204460492503131e-16f64 * fabs(sum);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dilog_series_2(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let stat_s3: libc::c_int = series_2(x, result);
    let mut t: libc::c_double = 0.;
    if x > 0.01f64 {
        t = (1.0f64 - x) * log(1.0f64 - x) / x;
    } else {
        static mut c3: libc::c_double = 1.0f64 / 3.0f64;
        static mut c4: libc::c_double = 1.0f64 / 4.0f64;
        static mut c5: libc::c_double = 1.0f64 / 5.0f64;
        static mut c6: libc::c_double = 1.0f64 / 6.0f64;
        static mut c7: libc::c_double = 1.0f64 / 7.0f64;
        static mut c8: libc::c_double = 1.0f64 / 8.0f64;
        let t68: libc::c_double = c6 + x * (c7 + x * c8);
        let t38: libc::c_double = c3 + x * (c4 + x * (c5 + x * t68));
        t = (x - 1.0f64) * (1.0f64 + x * (0.5f64 + x * t38));
    }
    (*result).val += 1.0f64 + t;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(t);
    return stat_s3;
}
unsafe extern "C" fn dilog_xge0(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x > 2.0f64 {
        let mut ser: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_ser: libc::c_int = dilog_series_2(1.0f64 / x, &mut ser);
        let log_x: libc::c_double = log(x);
        let t1: libc::c_double = 3.14159265358979323846f64 * 3.14159265358979323846f64
            / 3.0f64;
        let t2: libc::c_double = ser.val;
        let t3: libc::c_double = 0.5f64 * log_x * log_x;
        (*result).val = t1 - t2 - t3;
        (*result).err = 2.2204460492503131e-16f64 * fabs(log_x) + ser.err;
        (*result).err += 2.2204460492503131e-16f64 * (fabs(t1) + fabs(t2) + fabs(t3));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_ser;
    } else if x > 1.01f64 {
        let mut ser_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_ser_0: libc::c_int = dilog_series_2(1.0f64 - 1.0f64 / x, &mut ser_0);
        let log_x_0: libc::c_double = log(x);
        let log_term: libc::c_double = log_x_0
            * (log(1.0f64 - 1.0f64 / x) + 0.5f64 * log_x_0);
        let t1_0: libc::c_double = 3.14159265358979323846f64 * 3.14159265358979323846f64
            / 6.0f64;
        let t2_0: libc::c_double = ser_0.val;
        let t3_0: libc::c_double = log_term;
        (*result).val = t1_0 + t2_0 - t3_0;
        (*result).err = 2.2204460492503131e-16f64 * fabs(log_x_0) + ser_0.err;
        (*result).err
            += 2.2204460492503131e-16f64 * (fabs(t1_0) + fabs(t2_0) + fabs(t3_0));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_ser_0;
    } else if x > 1.0f64 {
        let eps: libc::c_double = x - 1.0f64;
        let lne: libc::c_double = log(eps);
        let c0: libc::c_double = 3.14159265358979323846f64 * 3.14159265358979323846f64
            / 6.0f64;
        let c1: libc::c_double = 1.0f64 - lne;
        let c2: libc::c_double = -(1.0f64 - 2.0f64 * lne) / 4.0f64;
        let c3: libc::c_double = (1.0f64 - 3.0f64 * lne) / 9.0f64;
        let c4: libc::c_double = -(1.0f64 - 4.0f64 * lne) / 16.0f64;
        let c5: libc::c_double = (1.0f64 - 5.0f64 * lne) / 25.0f64;
        let c6: libc::c_double = -(1.0f64 - 6.0f64 * lne) / 36.0f64;
        let c7: libc::c_double = (1.0f64 - 7.0f64 * lne) / 49.0f64;
        let c8: libc::c_double = -(1.0f64 - 8.0f64 * lne) / 64.0f64;
        (*result)
            .val = c0
            + eps
                * (c1
                    + eps
                        * (c2
                            + eps
                                * (c3
                                    + eps
                                        * (c4 + eps * (c5 + eps * (c6 + eps * (c7 + eps * c8)))))));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x == 1.0f64 {
        (*result).val = 3.14159265358979323846f64 * 3.14159265358979323846f64 / 6.0f64;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * 3.14159265358979323846f64
            * 3.14159265358979323846f64 / 6.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x > 0.5f64 {
        let mut ser_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_ser_1: libc::c_int = dilog_series_2(1.0f64 - x, &mut ser_1);
        let log_x_1: libc::c_double = log(x);
        let t1_1: libc::c_double = 3.14159265358979323846f64 * 3.14159265358979323846f64
            / 6.0f64;
        let t2_1: libc::c_double = ser_1.val;
        let t3_1: libc::c_double = log_x_1 * log(1.0f64 - x);
        (*result).val = t1_1 - t2_1 - t3_1;
        (*result).err = 2.2204460492503131e-16f64 * fabs(log_x_1) + ser_1.err;
        (*result).err
            += 2.2204460492503131e-16f64 * (fabs(t1_1) + fabs(t2_1) + fabs(t3_1));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_ser_1;
    } else if x > 0.25f64 {
        return dilog_series_2(x, result)
    } else if x > 0.0f64 {
        return dilog_series_1(x, result)
    } else {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn dilogc_series_1(
    r: libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut real_result: *mut gsl_sf_result,
    mut imag_result: *mut gsl_sf_result,
) -> libc::c_int {
    let cos_theta: libc::c_double = x / r;
    let sin_theta: libc::c_double = y / r;
    let alpha: libc::c_double = 1.0f64 - cos_theta;
    let beta: libc::c_double = sin_theta;
    let mut ck: libc::c_double = cos_theta;
    let mut sk: libc::c_double = sin_theta;
    let mut rk: libc::c_double = r;
    let mut real_sum: libc::c_double = r * ck;
    let mut imag_sum: libc::c_double = r * sk;
    let kmax: libc::c_int = 50 as libc::c_int + (22.0f64 / -log(r)) as libc::c_int;
    let mut k: libc::c_int = 0;
    k = 2 as libc::c_int;
    while k < kmax {
        let mut dr: libc::c_double = 0.;
        let mut di: libc::c_double = 0.;
        let mut ck_tmp: libc::c_double = ck;
        ck = ck - (alpha * ck + beta * sk);
        sk = sk - (alpha * sk - beta * ck_tmp);
        rk *= r;
        dr = rk / (k as libc::c_double * k as libc::c_double) * ck;
        di = rk / (k as libc::c_double * k as libc::c_double) * sk;
        real_sum += dr;
        imag_sum += di;
        if fabs((dr * dr + di * di) / (real_sum * real_sum + imag_sum * imag_sum))
            < 2.2204460492503131e-16f64 * 2.2204460492503131e-16f64
        {
            break;
        }
        k += 1;
        k;
    }
    (*real_result).val = real_sum;
    (*real_result)
        .err = 2.0f64 * kmax as libc::c_double * 2.2204460492503131e-16f64
        * fabs(real_sum);
    (*imag_result).val = imag_sum;
    (*imag_result)
        .err = 2.0f64 * kmax as libc::c_double * 2.2204460492503131e-16f64
        * fabs(imag_sum);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn series_2_c(
    mut r: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut sum_re: *mut gsl_sf_result,
    mut sum_im: *mut gsl_sf_result,
) -> libc::c_int {
    let cos_theta: libc::c_double = x / r;
    let sin_theta: libc::c_double = y / r;
    let alpha: libc::c_double = 1.0f64 - cos_theta;
    let beta: libc::c_double = sin_theta;
    let mut ck: libc::c_double = cos_theta;
    let mut sk: libc::c_double = sin_theta;
    let mut rk: libc::c_double = r;
    let mut real_sum: libc::c_double = 0.5f64 * r * ck;
    let mut imag_sum: libc::c_double = 0.5f64 * r * sk;
    let kmax: libc::c_int = 30 as libc::c_int + (18.0f64 / -log(r)) as libc::c_int;
    let mut k: libc::c_int = 0;
    k = 2 as libc::c_int;
    while k < kmax {
        let mut dr: libc::c_double = 0.;
        let mut di: libc::c_double = 0.;
        let ck_tmp: libc::c_double = ck;
        ck = ck - (alpha * ck + beta * sk);
        sk = sk - (alpha * sk - beta * ck_tmp);
        rk *= r;
        dr = rk
            / (k as libc::c_double * k as libc::c_double
                * (k as libc::c_double + 1.0f64)) * ck;
        di = rk
            / (k as libc::c_double * k as libc::c_double
                * (k as libc::c_double + 1.0f64)) * sk;
        real_sum += dr;
        imag_sum += di;
        if fabs((dr * dr + di * di) / (real_sum * real_sum + imag_sum * imag_sum))
            < 2.2204460492503131e-16f64 * 2.2204460492503131e-16f64
        {
            break;
        }
        k += 1;
        k;
    }
    (*sum_re).val = real_sum;
    (*sum_re)
        .err = 2.0f64 * kmax as libc::c_double * 2.2204460492503131e-16f64
        * fabs(real_sum);
    (*sum_im).val = imag_sum;
    (*sum_im)
        .err = 2.0f64 * kmax as libc::c_double * 2.2204460492503131e-16f64
        * fabs(imag_sum);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dilogc_series_2(
    r: libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut real_dl: *mut gsl_sf_result,
    mut imag_dl: *mut gsl_sf_result,
) -> libc::c_int {
    if r == 0.0f64 {
        (*real_dl).val = 0.0f64;
        (*imag_dl).val = 0.0f64;
        (*real_dl).err = 0.0f64;
        (*imag_dl).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut sum_re: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sum_im: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_s3: libc::c_int = series_2_c(r, x, y, &mut sum_re, &mut sum_im);
        let mut ln_omz_r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut ln_omz_theta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_log: libc::c_int = gsl_sf_complex_log_e(
            1.0f64 - x,
            -y,
            &mut ln_omz_r,
            &mut ln_omz_theta,
        );
        let t_x: libc::c_double = (ln_omz_r.val * x + ln_omz_theta.val * y) / (r * r);
        let t_y: libc::c_double = (-ln_omz_r.val * y + ln_omz_theta.val * x) / (r * r);
        let r_x: libc::c_double = (1.0f64 - x) * t_x + y * t_y;
        let r_y: libc::c_double = (1.0f64 - x) * t_y - y * t_x;
        (*real_dl).val = sum_re.val + r_x + 1.0f64;
        (*imag_dl).val = sum_im.val + r_y;
        (*real_dl)
            .err = sum_re.err
            + 2.0f64 * 2.2204460492503131e-16f64 * (fabs((*real_dl).val) + fabs(r_x));
        (*imag_dl)
            .err = sum_im.err
            + 2.0f64 * 2.2204460492503131e-16f64 * (fabs((*imag_dl).val) + fabs(r_y));
        return if stat_s3 != GSL_SUCCESS as libc::c_int {
            stat_s3
        } else if stat_log != GSL_SUCCESS as libc::c_int {
            stat_log
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
unsafe extern "C" fn dilogc_series_3(
    r: libc::c_double,
    x: libc::c_double,
    y: libc::c_double,
    mut real_result: *mut gsl_sf_result,
    mut imag_result: *mut gsl_sf_result,
) -> libc::c_int {
    let theta: libc::c_double = atan2(y, x);
    let cos_theta: libc::c_double = x / r;
    let sin_theta: libc::c_double = y / r;
    let a: libc::c_double = log(r);
    let omc: libc::c_double = 1.0f64 - cos_theta;
    let omc2: libc::c_double = omc * omc;
    let mut H_re: [libc::c_double; 7] = [0.; 7];
    let mut H_im: [libc::c_double; 7] = [0.; 7];
    let mut an: libc::c_double = 0.;
    let mut nfact: libc::c_double = 0.;
    let mut sum_re: libc::c_double = 0.;
    let mut sum_im: libc::c_double = 0.;
    let mut Him0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut n: libc::c_int = 0;
    H_re[0 as libc::c_int
        as usize] = 3.14159265358979323846f64 * 3.14159265358979323846f64 / 6.0f64
        + 0.25f64 * (theta * theta - 2.0f64 * 3.14159265358979323846f64 * fabs(theta));
    gsl_sf_clausen_e(theta, &mut Him0);
    H_im[0 as libc::c_int as usize] = Him0.val;
    H_re[1 as libc::c_int as usize] = -0.5f64 * log(2.0f64 * omc);
    H_im[1 as libc::c_int as usize] = -atan2(-sin_theta, omc);
    H_re[2 as libc::c_int as usize] = -0.5f64;
    H_im[2 as libc::c_int as usize] = 0.5f64 * sin_theta / omc;
    H_re[3 as libc::c_int as usize] = -0.5f64 / omc;
    H_im[3 as libc::c_int as usize] = 0.0f64;
    H_re[4 as libc::c_int as usize] = 0.0f64;
    H_im[4 as libc::c_int as usize] = -0.5f64 * sin_theta / omc2;
    H_re[5 as libc::c_int as usize] = 0.5f64 * (2.0f64 + cos_theta) / omc2;
    H_im[5 as libc::c_int as usize] = 0.0f64;
    H_re[6 as libc::c_int as usize] = 0.0f64;
    H_im[6 as libc::c_int
        as usize] = 0.5f64 * sin_theta / (omc2 * omc2 * omc)
        * (8.0f64 * omc - sin_theta * sin_theta * (3.0f64 + cos_theta));
    sum_re = H_re[0 as libc::c_int as usize];
    sum_im = H_im[0 as libc::c_int as usize];
    an = 1.0f64;
    nfact = 1.0f64;
    n = 1 as libc::c_int;
    while n <= 6 as libc::c_int {
        let mut t: libc::c_double = 0.;
        an *= a;
        nfact *= n as libc::c_double;
        t = an / nfact;
        sum_re += t * H_re[n as usize];
        sum_im += t * H_im[n as usize];
        n += 1;
        n;
    }
    (*real_result).val = sum_re;
    (*real_result)
        .err = 2.0f64 * 6.0f64 * 2.2204460492503131e-16f64 * fabs(sum_re)
        + fabs(an / nfact);
    (*imag_result).val = sum_im;
    (*imag_result)
        .err = 2.0f64 * 6.0f64 * 2.2204460492503131e-16f64 * fabs(sum_im) + Him0.err
        + fabs(an / nfact);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn dilogc_fundamental(
    mut r: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut real_dl: *mut gsl_sf_result,
    mut imag_dl: *mut gsl_sf_result,
) -> libc::c_int {
    if r > 0.98f64 {
        return dilogc_series_3(r, x, y, real_dl, imag_dl)
    } else if r > 0.25f64 {
        return dilogc_series_2(r, x, y, real_dl, imag_dl)
    } else {
        return dilogc_series_1(r, x, y, real_dl, imag_dl)
    };
}
unsafe extern "C" fn dilogc_unitdisk(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut real_dl: *mut gsl_sf_result,
    mut imag_dl: *mut gsl_sf_result,
) -> libc::c_int {
    static mut MAGIC_SPLIT_VALUE: libc::c_double = 0.732f64;
    static mut zeta2: libc::c_double = 3.14159265358979323846f64
        * 3.14159265358979323846f64 / 6.0f64;
    let r: libc::c_double = hypot(x, y);
    if x > MAGIC_SPLIT_VALUE {
        let x_tmp: libc::c_double = 1.0f64 - x;
        let y_tmp: libc::c_double = -y;
        let r_tmp: libc::c_double = hypot(x_tmp, y_tmp);
        let mut result_re_tmp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_im_tmp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_dilog: libc::c_int = dilogc_fundamental(
            r_tmp,
            x_tmp,
            y_tmp,
            &mut result_re_tmp,
            &mut result_im_tmp,
        );
        let lnz: libc::c_double = log(r);
        let lnomz: libc::c_double = log(r_tmp);
        let argz: libc::c_double = atan2(y, x);
        let argomz: libc::c_double = atan2(y_tmp, x_tmp);
        (*real_dl).val = -result_re_tmp.val + zeta2 - lnz * lnomz + argz * argomz;
        (*real_dl).err = result_re_tmp.err;
        (*real_dl).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * (zeta2 + fabs(lnz * lnomz) + fabs(argz * argomz));
        (*imag_dl).val = -result_im_tmp.val - argz * lnomz - argomz * lnz;
        (*imag_dl).err = result_im_tmp.err;
        (*imag_dl).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(argz * lnomz) + fabs(argomz * lnz));
        return stat_dilog;
    } else {
        return dilogc_fundamental(r, x, y, real_dl, imag_dl)
    };
}
pub unsafe extern "C" fn gsl_sf_dilog_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x >= 0.0f64 {
        return dilog_xge0(x, result)
    } else {
        let mut d1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut d2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_d1: libc::c_int = dilog_xge0(-x, &mut d1);
        let mut stat_d2: libc::c_int = dilog_xge0(x * x, &mut d2);
        (*result).val = -d1.val + 0.5f64 * d2.val;
        (*result).err = d1.err + 0.5f64 * d2.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_d1 != GSL_SUCCESS as libc::c_int {
            stat_d1
        } else if stat_d2 != GSL_SUCCESS as libc::c_int {
            stat_d2
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_complex_dilog_xy_e(
    x: libc::c_double,
    y: libc::c_double,
    mut real_dl: *mut gsl_sf_result,
    mut imag_dl: *mut gsl_sf_result,
) -> libc::c_int {
    let zeta2: libc::c_double = 3.14159265358979323846f64 * 3.14159265358979323846f64
        / 6.0f64;
    let r2: libc::c_double = x * x + y * y;
    if y == 0.0f64 {
        if x >= 1.0f64 {
            (*imag_dl).val = -3.14159265358979323846f64 * log(x);
            (*imag_dl).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*imag_dl).val);
        } else {
            (*imag_dl).val = 0.0f64;
            (*imag_dl).err = 0.0f64;
        }
        return gsl_sf_dilog_e(x, real_dl);
    } else if fabs(r2 - 1.0f64) < 2.2204460492503131e-16f64 {
        let theta: libc::c_double = atan2(y, x);
        let term1: libc::c_double = theta * theta / 4.0f64;
        let term2: libc::c_double = 3.14159265358979323846f64 * fabs(theta) / 2.0f64;
        (*real_dl).val = zeta2 + term1 - term2;
        (*real_dl).err = 2.0f64 * 2.2204460492503131e-16f64 * (zeta2 + term1 + term2);
        return gsl_sf_clausen_e(theta, imag_dl);
    } else if r2 < 1.0f64 {
        return dilogc_unitdisk(x, y, real_dl, imag_dl)
    } else {
        let r: libc::c_double = sqrt(r2);
        let x_tmp: libc::c_double = x / r2;
        let y_tmp: libc::c_double = -y / r2;
        let mut result_re_tmp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_im_tmp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_dilog: libc::c_int = dilogc_unitdisk(
            x_tmp,
            y_tmp,
            &mut result_re_tmp,
            &mut result_im_tmp,
        );
        let theta_0: libc::c_double = atan2(y, x);
        let theta_abs: libc::c_double = fabs(theta_0);
        let theta_sgn: libc::c_double = if theta_0 < 0.0f64 { -1.0f64 } else { 1.0f64 };
        let ln_minusz_re: libc::c_double = log(r);
        let ln_minusz_im: libc::c_double = theta_sgn
            * (theta_abs - 3.14159265358979323846f64);
        let lmz2_re: libc::c_double = ln_minusz_re * ln_minusz_re
            - ln_minusz_im * ln_minusz_im;
        let lmz2_im: libc::c_double = 2.0f64 * ln_minusz_re * ln_minusz_im;
        (*real_dl).val = -result_re_tmp.val - 0.5f64 * lmz2_re - zeta2;
        (*real_dl)
            .err = result_re_tmp.err
            + 2.0f64 * 2.2204460492503131e-16f64 * (0.5f64 * fabs(lmz2_re) + zeta2);
        (*imag_dl).val = -result_im_tmp.val - 0.5f64 * lmz2_im;
        (*imag_dl)
            .err = result_im_tmp.err
            + 2.0f64 * 2.2204460492503131e-16f64 * fabs(lmz2_im);
        return stat_dilog;
    };
}
pub unsafe extern "C" fn gsl_sf_complex_dilog_e(
    r: libc::c_double,
    theta: libc::c_double,
    mut real_dl: *mut gsl_sf_result,
    mut imag_dl: *mut gsl_sf_result,
) -> libc::c_int {
    let cos_theta: libc::c_double = cos(theta);
    let sin_theta: libc::c_double = sin(theta);
    let x: libc::c_double = r * cos_theta;
    let y: libc::c_double = r * sin_theta;
    return gsl_sf_complex_dilog_xy_e(x, y, real_dl, imag_dl);
}
pub unsafe extern "C" fn gsl_sf_complex_spence_xy_e(
    x: libc::c_double,
    y: libc::c_double,
    mut real_sp: *mut gsl_sf_result,
    mut imag_sp: *mut gsl_sf_result,
) -> libc::c_int {
    let oms_x: libc::c_double = 1.0f64 - x;
    let oms_y: libc::c_double = -y;
    return gsl_sf_complex_dilog_xy_e(oms_x, oms_y, real_sp, imag_sp);
}
pub unsafe extern "C" fn gsl_sf_dilog(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_dilog_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_dilog_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"dilog.c\0" as *const u8 as *const libc::c_char,
            661 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
