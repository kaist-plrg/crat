use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_stats_long_double_mean(
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_mean(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_mean(
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_mean(
        data: *const libc::c_ulong,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_mean(
        data: *const libc::c_long,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_mean(
        data: *const libc::c_uint,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_mean(
        data: *const libc::c_int,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_mean(
        data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_mean(
        data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_mean(
        data: *const libc::c_uchar,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_char_mean(
        data: *const libc::c_char,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn compute_ushort_variance(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_long_variance(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_ulong_variance(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_short_variance(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_uchar_variance(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_int_variance(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_uint_variance(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_float_variance(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_long_double_variance(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize)
            - f128::f128::new(mean);
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_char_variance(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_variance(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut variance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) - mean,
        );
        variance
            += (delta * delta - variance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return variance.to_f64().unwrap();
}
unsafe extern "C" fn compute_tss(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_ushort_tss(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_long_double_tss(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize)
            - f128::f128::new(mean);
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_long_tss(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_int_tss(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_ulong_tss(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_uint_tss(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_char_tss(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_uchar_tss(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_float_tss(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
unsafe extern "C" fn compute_short_tss(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut tss: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        tss += delta * delta;
        i = i.wrapping_add(1);
        i;
    }
    return tss.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_int_variance_with_fixed_mean(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_int_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_uint_variance_with_fixed_mean(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uint_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_short_variance_with_fixed_mean(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_short_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_float_variance_with_fixed_mean(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_float_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_ulong_variance_with_fixed_mean(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ulong_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_long_double_variance_with_fixed_mean(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_double_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_long_variance_with_fixed_mean(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_variance_with_fixed_mean(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_ushort_variance_with_fixed_mean(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ushort_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_uchar_variance_with_fixed_mean(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uchar_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_char_variance_with_fixed_mean(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_char_variance(data, stride, n, mean);
    return variance;
}
pub unsafe extern "C" fn gsl_stats_long_sd_with_fixed_mean(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_short_sd_with_fixed_mean(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_short_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_uchar_sd_with_fixed_mean(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uchar_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_sd_with_fixed_mean(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_long_double_sd_with_fixed_mean(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_double_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_char_sd_with_fixed_mean(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_char_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_ushort_sd_with_fixed_mean(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ushort_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_int_sd_with_fixed_mean(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_int_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_ulong_sd_with_fixed_mean(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ulong_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_float_sd_with_fixed_mean(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_float_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_uint_sd_with_fixed_mean(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uint_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(variance);
    return sd;
}
pub unsafe extern "C" fn gsl_stats_long_variance_m(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_ulong_variance_m(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ulong_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_long_double_variance_m(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_double_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_short_variance_m(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_short_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_float_variance_m(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_float_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_int_variance_m(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_int_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_char_variance_m(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_char_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_variance_m(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_uint_variance_m(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uint_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_ushort_variance_m(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ushort_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_uchar_variance_m(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uchar_variance(data, stride, n, mean);
    return variance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_char_sd_m(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_char_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_short_sd_m(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_short_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_sd_m(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_long_sd_m(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_ushort_sd_m(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ushort_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_uchar_sd_m(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uchar_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_float_sd_m(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_float_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_uint_sd_m(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_uint_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_int_sd_m(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_int_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_long_double_sd_m(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_long_double_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_ulong_sd_m(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let variance: libc::c_double = compute_ulong_variance(data, stride, n, mean);
    let sd: libc::c_double = sqrt(
        variance
            * (n as libc::c_double
                / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double),
    );
    return sd;
}
pub unsafe extern "C" fn gsl_stats_variance(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_mean(data, stride, n);
    return gsl_stats_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_variance(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_mean(data, stride, n);
    return gsl_stats_long_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_int_variance(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_int_mean(data, stride, n);
    return gsl_stats_int_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_char_variance(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_char_mean(data, stride, n);
    return gsl_stats_char_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_double_variance(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_double_mean(data, stride, n);
    return gsl_stats_long_double_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_float_variance(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_float_mean(data, stride, n);
    return gsl_stats_float_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_ushort_variance(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ushort_mean(data, stride, n);
    return gsl_stats_ushort_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_ulong_variance(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ulong_mean(data, stride, n);
    return gsl_stats_ulong_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uint_variance(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uint_mean(data, stride, n);
    return gsl_stats_uint_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_short_variance(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_short_mean(data, stride, n);
    return gsl_stats_short_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uchar_variance(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uchar_mean(data, stride, n);
    return gsl_stats_uchar_variance_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_int_sd(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_int_mean(data, stride, n);
    return gsl_stats_int_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_float_sd(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_float_mean(data, stride, n);
    return gsl_stats_float_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uchar_sd(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uchar_mean(data, stride, n);
    return gsl_stats_uchar_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uint_sd(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uint_mean(data, stride, n);
    return gsl_stats_uint_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_double_sd(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_double_mean(data, stride, n);
    return gsl_stats_long_double_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_char_sd(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_char_mean(data, stride, n);
    return gsl_stats_char_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_sd(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_mean(data, stride, n);
    return gsl_stats_long_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_short_sd(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_short_mean(data, stride, n);
    return gsl_stats_short_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_ushort_sd(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ushort_mean(data, stride, n);
    return gsl_stats_ushort_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_sd(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_mean(data, stride, n);
    return gsl_stats_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_ulong_sd(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ulong_mean(data, stride, n);
    return gsl_stats_ulong_sd_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_int_tss_m(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_int_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_long_double_tss_m(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_long_double_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_uint_tss_m(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_uint_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_long_tss_m(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_long_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_uchar_tss_m(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_uchar_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_char_tss_m(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_char_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_short_tss_m(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_short_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_tss_m(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_ushort_tss_m(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_ushort_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_ulong_tss_m(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_ulong_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_float_tss_m(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let tss: libc::c_double = compute_float_tss(data, stride, n, mean);
    return tss;
}
pub unsafe extern "C" fn gsl_stats_ulong_tss(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ulong_mean(data, stride, n);
    return gsl_stats_ulong_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_tss(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_mean(data, stride, n);
    return gsl_stats_long_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_tss(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_mean(data, stride, n);
    return gsl_stats_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_double_tss(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_double_mean(data, stride, n);
    return gsl_stats_long_double_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_ushort_tss(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ushort_mean(data, stride, n);
    return gsl_stats_ushort_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_short_tss(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_short_mean(data, stride, n);
    return gsl_stats_short_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uchar_tss(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uchar_mean(data, stride, n);
    return gsl_stats_uchar_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_int_tss(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_int_mean(data, stride, n);
    return gsl_stats_int_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_char_tss(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_char_mean(data, stride, n);
    return gsl_stats_char_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_float_tss(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_float_mean(data, stride, n);
    return gsl_stats_float_tss_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uint_tss(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uint_mean(data, stride, n);
    return gsl_stats_uint_tss_m(data, stride, n, mean);
}
