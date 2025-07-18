use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_stats_long_double_wmean(
        w: *const f128::f128,
        wstride: size_t,
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_wmean(
        w: *const libc::c_double,
        wstride: size_t,
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_wmean(
        w: *const libc::c_float,
        wstride: size_t,
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn compute_long_double_wvariance(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let mut wvariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: f128::f128 = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > f128::f128::new(0 as libc::c_int) {
            let delta: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize)
                - f128::f128::new(wmean);
            W += wi;
            wvariance += (delta * delta - wvariance) * (wi / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wvariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_wvariance(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let mut wvariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            let delta: f128::f128 = f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) - wmean,
            );
            W += f128::f128::new(wi);
            wvariance += (delta * delta - wvariance) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wvariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_float_wvariance(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let mut wvariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_float = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_float {
            let delta: f128::f128 = f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - wmean,
            );
            W += f128::f128::new(wi);
            wvariance += (delta * delta - wvariance) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wvariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_long_double_wtss(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let mut wtss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: f128::f128 = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > f128::f128::new(0 as libc::c_int) {
            let delta: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize)
                - f128::f128::new(wmean);
            wtss += wi * delta * delta;
        }
        i = i.wrapping_add(1);
        i;
    }
    return wtss.to_f64().unwrap();
}
unsafe extern "C" fn compute_wtss(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let mut wtss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            let delta: f128::f128 = f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) - wmean,
            );
            wtss += f128::f128::new(wi) * delta * delta;
        }
        i = i.wrapping_add(1);
        i;
    }
    return wtss.to_f64().unwrap();
}
unsafe extern "C" fn compute_float_wtss(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let mut wtss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_float = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_float {
            let delta: f128::f128 = f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - wmean,
            );
            wtss += f128::f128::new(wi) * delta * delta;
        }
        i = i.wrapping_add(1);
        i;
    }
    return wtss.to_f64().unwrap();
}
unsafe extern "C" fn compute_factor(
    mut w: *const libc::c_double,
    wstride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut a: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut b: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut factor: f128::f128 = f128::f128::ZERO;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            a += f128::f128::new(wi);
            b += f128::f128::new(wi * wi);
        }
        i = i.wrapping_add(1);
        i;
    }
    factor = a * a / (a * a - b);
    return factor.to_f64().unwrap();
}
unsafe extern "C" fn compute_long_double_factor(
    mut w: *const f128::f128,
    wstride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut a: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut b: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut factor: f128::f128 = f128::f128::ZERO;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: f128::f128 = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > f128::f128::new(0 as libc::c_int) {
            a += wi;
            b += wi * wi;
        }
        i = i.wrapping_add(1);
        i;
    }
    factor = a * a / (a * a - b);
    return factor.to_f64().unwrap();
}
unsafe extern "C" fn compute_float_factor(
    mut w: *const libc::c_float,
    wstride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut a: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut b: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut factor: f128::f128 = f128::f128::ZERO;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_float = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_float {
            a += f128::f128::new(wi);
            b += f128::f128::new(wi * wi);
        }
        i = i.wrapping_add(1);
        i;
    }
    factor = a * a / (a * a - b);
    return factor.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_float_wvariance_with_fixed_mean(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wvariance: libc::c_double = compute_float_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    return wvariance;
}
pub unsafe extern "C" fn gsl_stats_wvariance_with_fixed_mean(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wvariance: libc::c_double = compute_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    return wvariance;
}
pub unsafe extern "C" fn gsl_stats_long_double_wvariance_with_fixed_mean(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wvariance: libc::c_double = compute_long_double_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    return wvariance;
}
pub unsafe extern "C" fn gsl_stats_wsd_with_fixed_mean(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wvariance: libc::c_double = compute_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    let wsd: libc::c_double = sqrt(wvariance);
    return wsd;
}
pub unsafe extern "C" fn gsl_stats_float_wsd_with_fixed_mean(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wvariance: libc::c_double = compute_float_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    let wsd: libc::c_double = sqrt(wvariance);
    return wsd;
}
pub unsafe extern "C" fn gsl_stats_long_double_wsd_with_fixed_mean(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wvariance: libc::c_double = compute_long_double_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    let wsd: libc::c_double = sqrt(wvariance);
    return wsd;
}
pub unsafe extern "C" fn gsl_stats_long_double_wvariance_m(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_double_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    let scale: libc::c_double = compute_long_double_factor(w, wstride, n);
    return scale * variance;
}
pub unsafe extern "C" fn gsl_stats_float_wvariance_m(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_float_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    let scale: libc::c_double = compute_float_factor(w, wstride, n);
    return scale * variance;
}
pub unsafe extern "C" fn gsl_stats_wvariance_m(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_wvariance(w, wstride, data, stride, n, wmean);
    let scale: libc::c_double = compute_factor(w, wstride, n);
    return scale * variance;
}
pub unsafe extern "C" fn gsl_stats_float_wsd_m(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_float_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    let scale: libc::c_double = compute_float_factor(w, wstride, n);
    let wsd: libc::c_double = sqrt(scale * variance);
    return wsd;
}
pub unsafe extern "C" fn gsl_stats_long_double_wsd_m(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_double_wvariance(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    let scale: libc::c_double = compute_long_double_factor(w, wstride, n);
    let wsd: libc::c_double = sqrt(scale * variance);
    return wsd;
}
pub unsafe extern "C" fn gsl_stats_wsd_m(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_wvariance(w, wstride, data, stride, n, wmean);
    let scale: libc::c_double = compute_factor(w, wstride, n);
    let wsd: libc::c_double = sqrt(scale * variance);
    return wsd;
}
pub unsafe extern "C" fn gsl_stats_wsd(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_wmean(w, wstride, data, stride, n);
    return gsl_stats_wsd_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_long_double_wsd(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_long_double_wmean(w, wstride, data, stride, n);
    return gsl_stats_long_double_wsd_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_float_wsd(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_float_wmean(w, wstride, data, stride, n);
    return gsl_stats_float_wsd_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_wvariance(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_wmean(w, wstride, data, stride, n);
    return gsl_stats_wvariance_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_float_wvariance(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_float_wmean(w, wstride, data, stride, n);
    return gsl_stats_float_wvariance_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_long_double_wvariance(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_long_double_wmean(w, wstride, data, stride, n);
    return gsl_stats_long_double_wvariance_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_wtss_m(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wtss: libc::c_double = compute_wtss(w, wstride, data, stride, n, wmean);
    return wtss;
}
pub unsafe extern "C" fn gsl_stats_long_double_wtss_m(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wtss: libc::c_double = compute_long_double_wtss(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    return wtss;
}
pub unsafe extern "C" fn gsl_stats_float_wtss_m(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
) -> libc::c_double {
    let wtss: libc::c_double = compute_float_wtss(w, wstride, data, stride, n, wmean);
    return wtss;
}
pub unsafe extern "C" fn gsl_stats_long_double_wtss(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_long_double_wmean(w, wstride, data, stride, n);
    return gsl_stats_long_double_wtss_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_float_wtss(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_float_wmean(w, wstride, data, stride, n);
    return gsl_stats_float_wtss_m(w, wstride, data, stride, n, wmean);
}
pub unsafe extern "C" fn gsl_stats_wtss(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_wmean(w, wstride, data, stride, n);
    return gsl_stats_wtss_m(w, wstride, data, stride, n, wmean);
}
