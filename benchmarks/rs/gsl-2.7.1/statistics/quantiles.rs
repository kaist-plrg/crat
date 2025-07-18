use ::libc;
use ::num_traits;
use ::f128;
use num_traits::ToPrimitive;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_float_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_long_double_quantile_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize))
            .to_f64()
            .unwrap();
    } else {
        result = (f128::f128::new(1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + f128::f128::new(delta)
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ))
            .to_f64()
            .unwrap();
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_uint_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_char_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_long_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_ulong_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_uchar_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_int_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_ushort_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_short_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_int
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int as libc::c_double;
    }
    return result;
}
pub unsafe extern "C" fn gsl_stats_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f
        * n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    let lhs: size_t = index as libc::c_int as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize);
    } else {
        result = (1 as libc::c_int as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + delta
                * *sorted_data
                    .offset(
                        lhs
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
    }
    return result;
}
