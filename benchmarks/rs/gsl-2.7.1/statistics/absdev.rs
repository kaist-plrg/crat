use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn gsl_stats_char_absdev(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_char_mean(data, stride, n);
    return gsl_stats_char_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uint_absdev(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uint_mean(data, stride, n);
    return gsl_stats_uint_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_double_absdev(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_double_mean(data, stride, n);
    return gsl_stats_long_double_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_absdev(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_mean(data, stride, n);
    return gsl_stats_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_float_absdev(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_float_mean(data, stride, n);
    return gsl_stats_float_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_short_absdev(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_short_mean(data, stride, n);
    return gsl_stats_short_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_long_absdev(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_mean(data, stride, n);
    return gsl_stats_long_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_int_absdev(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_int_mean(data, stride, n);
    return gsl_stats_int_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uchar_absdev(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uchar_mean(data, stride, n);
    return gsl_stats_uchar_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_ushort_absdev(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ushort_mean(data, stride, n);
    return gsl_stats_ushort_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_ulong_absdev(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ulong_mean(data, stride, n);
    return gsl_stats_ulong_absdev_m(data, stride, n, mean);
}
pub unsafe extern "C" fn gsl_stats_uint_absdev_m(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_long_double_absdev_m(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            (*data.offset(i.wrapping_mul(stride) as isize) - f128::f128::new(mean))
                .to_f64()
                .unwrap(),
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_ushort_absdev_m(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_uchar_absdev_m(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_int_absdev_m(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_float_absdev_m(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_ulong_absdev_m(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_absdev_m(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_long_absdev_m(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_short_absdev_m(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
pub unsafe extern "C" fn gsl_stats_char_absdev_m(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut absdev: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta: libc::c_double = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double - mean,
        );
        sum += delta;
        i = i.wrapping_add(1);
        i;
    }
    absdev = sum / n as libc::c_double;
    return absdev;
}
