use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn gsl_stats_long_double_wmean(
        w: *const f128::f128,
        wstride: size_t,
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_wsd_m(
        w: *const f128::f128,
        wstride: size_t,
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
        wmean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_wmean(
        w: *const libc::c_double,
        wstride: size_t,
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_wsd_m(
        w: *const libc::c_double,
        wstride: size_t,
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
        wmean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_float_wmean(
        w: *const libc::c_float,
        wstride: size_t,
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_wsd_m(
        w: *const libc::c_float,
        wstride: size_t,
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
        wmean: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_float_wskew(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_float_wmean(w, wstride, data, stride, n);
    let wsd: libc::c_double = gsl_stats_float_wsd_m(w, wstride, data, stride, n, wmean);
    return gsl_stats_float_wskew_m_sd(w, wstride, data, stride, n, wmean, wsd);
}
pub unsafe extern "C" fn gsl_stats_wskew(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_wmean(w, wstride, data, stride, n);
    let wsd: libc::c_double = gsl_stats_wsd_m(w, wstride, data, stride, n, wmean);
    return gsl_stats_wskew_m_sd(w, wstride, data, stride, n, wmean, wsd);
}
pub unsafe extern "C" fn gsl_stats_long_double_wskew(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let wmean: libc::c_double = gsl_stats_long_double_wmean(w, wstride, data, stride, n);
    let wsd: libc::c_double = gsl_stats_long_double_wsd_m(
        w,
        wstride,
        data,
        stride,
        n,
        wmean,
    );
    return gsl_stats_long_double_wskew_m_sd(w, wstride, data, stride, n, wmean, wsd);
}
pub unsafe extern "C" fn gsl_stats_float_wskew_m_sd(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
    wsd: libc::c_double,
) -> libc::c_double {
    let mut wskew: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_float = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_float {
            let x: f128::f128 = f128::f128::new(
                (*data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - wmean)
                    / wsd,
            );
            W += f128::f128::new(wi);
            wskew += (x * x * x - wskew) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wskew.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_wskew_m_sd(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
    wsd: libc::c_double,
) -> libc::c_double {
    let mut wskew: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            let x: f128::f128 = f128::f128::new(
                (*data.offset(i.wrapping_mul(stride) as isize) - wmean) / wsd,
            );
            W += f128::f128::new(wi);
            wskew += (x * x * x - wskew) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wskew.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_long_double_wskew_m_sd(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    wmean: libc::c_double,
    wsd: libc::c_double,
) -> libc::c_double {
    let mut wskew: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut wi: f128::f128 = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > f128::f128::new(0 as libc::c_int) {
            let x: f128::f128 = (*data.offset(i.wrapping_mul(stride) as isize)
                - f128::f128::new(wmean)) / f128::f128::new(wsd);
            W += wi;
            wskew += (x * x * x - wskew) * (wi / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wskew.to_f64().unwrap();
}
