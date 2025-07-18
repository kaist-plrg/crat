use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_stats_long_double_median_from_sorted_data(
        sorted_data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_median_from_sorted_data(
        sorted_data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_median_from_sorted_data(
        sorted_data: *const libc::c_ulong,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_median_from_sorted_data(
        sorted_data: *const libc::c_long,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_median_from_sorted_data(
        sorted_data: *const libc::c_uint,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_median_from_sorted_data(
        sorted_data: *const libc::c_int,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_median_from_sorted_data(
        sorted_data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_char_median_from_sorted_data(
        sorted_data: *const libc::c_char,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_median_from_sorted_data(
        sorted_data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_median_from_sorted_data(
        sorted_data: *const libc::c_uchar,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_median_from_sorted_data(
        sorted_data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_uchar_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_uchar,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_uchar_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_int as libc::c_double
                - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_ulong_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_ulong,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_ulong_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_long_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_long,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_long_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_float_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_float_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_char_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_char,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_char_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_int as libc::c_double
                - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_uint_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_uint,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_uint_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_int_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_int,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_int_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_ushort_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_ushort_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_int as libc::c_double
                - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_short_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_short_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) as libc::c_int as libc::c_double
                - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = *sorted_data
                .offset(i.wrapping_mul(stride) as isize) - mean;
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
pub unsafe extern "C" fn gsl_stats_long_double_trmean_from_sorted_data(
    trim: libc::c_double,
    mut sorted_data: *const f128::f128,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    if trim >= 0.5f64 {
        return gsl_stats_long_double_median_from_sorted_data(sorted_data, stride, size)
    } else {
        let mut ilow: size_t = floor(trim * size as libc::c_double) as size_t;
        let mut ihigh: size_t = size
            .wrapping_sub(ilow)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mean: libc::c_double = 0.0f64;
        let mut k: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        i = ilow;
        while i <= ihigh {
            let mut delta: libc::c_double = (*sorted_data
                .offset(i.wrapping_mul(stride) as isize) - f128::f128::new(mean))
                .to_f64()
                .unwrap();
            k += 1.0f64;
            mean += delta / k;
            i = i.wrapping_add(1);
            i;
        }
        return mean;
    };
}
