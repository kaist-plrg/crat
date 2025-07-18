use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_uint_mean(
    mut data: *const libc::c_uint,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_ulong_mean(
    mut data: *const libc::c_ulong,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_ushort_mean(
    mut data: *const libc::c_ushort,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int,
            ) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_char_mean(
    mut data: *const libc::c_char,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int,
            ) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_mean(
    mut data: *const libc::c_double,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_long_mean(
    mut data: *const libc::c_long,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_int_mean(
    mut data: *const libc::c_int,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_short_mean(
    mut data: *const libc::c_short,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int,
            ) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_uchar_mean(
    mut data: *const libc::c_uchar,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(
                *data.offset(i.wrapping_mul(stride) as isize) as libc::c_int,
            ) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_float_mean(
    mut data: *const libc::c_float,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize)) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_long_double_mean(
    mut data: *const f128::f128,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut mean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mean
            += (*data.offset(i.wrapping_mul(stride) as isize) - mean)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return mean.to_f64().unwrap();
}
