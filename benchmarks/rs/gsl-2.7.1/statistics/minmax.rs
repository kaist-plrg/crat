use ::libc;
use num_traits::Float;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_ushort_max(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_ushort {
    let mut max: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ushort = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_short_max(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_short {
    let mut max: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_short = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_max(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut max: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
        }
        if xi.is_nan() as i32 != 0 {
            return xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_char_max(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_char {
    let mut max: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_char = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_uint_max(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_uint {
    let mut max: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uint = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_float_max(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_float {
    let mut max: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_float = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
        }
        if xi.is_nan() as i32 != 0 {
            return xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_int_max(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut max: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_int = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_long_double_max(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> f128::f128 {
    let mut max: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
        }
        if xi.is_nan() as i32 != 0 {
            return xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_uchar_max(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_uchar {
    let mut max: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uchar = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_ulong_max(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_ulong {
    let mut max: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ulong = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_long_max(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_long {
    let mut max: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_long = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_stats_ulong_min(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_ulong {
    let mut min: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ulong = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_short_min(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_short {
    let mut min: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_short = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_ushort_min(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_ushort {
    let mut min: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ushort = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_long_min(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_long {
    let mut min: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_long = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_uchar_min(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_uchar {
    let mut min: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uchar = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_char_min(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_char {
    let mut min: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_char = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_int_min(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_int {
    let mut min: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_int = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_float_min(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_float {
    let mut min: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_float = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi.is_nan() as i32 != 0 {
            return xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_long_double_min(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> f128::f128 {
    let mut min: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi.is_nan() as i32 != 0 {
            return xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_uint_min(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_uint {
    let mut min: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uint = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_min(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut min: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi.is_nan() as i32 != 0 {
            return xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_stats_int_minmax(
    mut min_out: *mut libc::c_int,
    mut max_out: *mut libc::c_int,
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_int = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_short_minmax(
    mut min_out: *mut libc::c_short,
    mut max_out: *mut libc::c_short,
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_short = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_long_minmax(
    mut min_out: *mut libc::c_long,
    mut max_out: *mut libc::c_long,
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_long = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_char_minmax(
    mut min_out: *mut libc::c_char,
    mut max_out: *mut libc::c_char,
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_char = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_uchar_minmax(
    mut min_out: *mut libc::c_uchar,
    mut max_out: *mut libc::c_uchar,
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uchar = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_ulong_minmax(
    mut min_out: *mut libc::c_ulong,
    mut max_out: *mut libc::c_ulong,
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ulong = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_long_double_minmax(
    mut min_out: *mut f128::f128,
    mut max_out: *mut f128::f128,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) {
    let mut min: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi > max {
            max = xi;
        }
        if xi.is_nan() as i32 != 0 {
            min = xi;
            max = xi;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_minmax(
    mut min_out: *mut libc::c_double,
    mut max_out: *mut libc::c_double,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi > max {
            max = xi;
        }
        if xi.is_nan() as i32 != 0 {
            min = xi;
            max = xi;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_float_minmax(
    mut min_out: *mut libc::c_float,
    mut max_out: *mut libc::c_float,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_float = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi > max {
            max = xi;
        }
        if xi.is_nan() as i32 != 0 {
            min = xi;
            max = xi;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_uint_minmax(
    mut min_out: *mut libc::c_uint,
    mut max_out: *mut libc::c_uint,
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uint = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
        }
        if xi > max {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_ushort_minmax(
    mut min_out: *mut libc::c_ushort,
    mut max_out: *mut libc::c_ushort,
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ushort = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_out = min;
    *max_out = max;
}
pub unsafe extern "C" fn gsl_stats_max_index(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
            max_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_uint_max_index(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uint = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_float_max_index(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_float = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
            max_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_long_double_max_index(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
            max_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_ulong_max_index(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ulong = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_int_max_index(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_int = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_ushort_max_index(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ushort = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_short_max_index(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_short = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_uchar_max_index(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uchar = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_long_max_index(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_long = *data.offset(i.wrapping_mul(stride) as isize);
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_char_max_index(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut max: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_char = *data.offset(i.wrapping_mul(stride) as isize);
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max_index;
}
pub unsafe extern "C" fn gsl_stats_float_min_index(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_float = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_long_min_index(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_long = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_long_double_min_index(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_ulong_min_index(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ulong = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_uint_min_index(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uint = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_min_index(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_int_min_index(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_int = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_uchar_min_index(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uchar = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_ushort_min_index(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ushort = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_char_min_index(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_char = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_short_min_index(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> size_t {
    let mut min: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_short = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return min_index;
}
pub unsafe extern "C" fn gsl_stats_int_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_int = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_int = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_ulong_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_ulong = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ulong = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_float_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_float = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_float = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi > max {
            max = xi;
            max_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            min_index = i;
            max_index = i;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_double = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi > max {
            max = xi;
            max_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            min_index = i;
            max_index = i;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_ushort_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_ushort = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_ushort = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_long_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_long = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_long = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_char_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_char = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_char = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_short_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_short = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_short = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_uchar_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_uchar = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uchar = *data.offset(i.wrapping_mul(stride) as isize);
        if (xi as libc::c_int) < min as libc::c_int {
            min = xi;
            min_index = i;
        }
        if xi as libc::c_int > max as libc::c_int {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_long_double_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) {
    let mut min: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: f128::f128 = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi > max {
            max = xi;
            max_index = i;
        }
        if xi.is_nan() as i32 != 0 {
            min_index = i;
            max_index = i;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
pub unsafe extern "C" fn gsl_stats_uint_minmax_index(
    mut min_index_out: *mut size_t,
    mut max_index_out: *mut size_t,
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) {
    let mut min: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut max: libc::c_uint = *data
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride) as isize);
    let mut i: size_t = 0;
    let mut min_index: size_t = 0 as libc::c_int as size_t;
    let mut max_index: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_uint = *data.offset(i.wrapping_mul(stride) as isize);
        if xi < min {
            min = xi;
            min_index = i;
        }
        if xi > max {
            max = xi;
            max_index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    *min_index_out = min_index;
    *max_index_out = max_index;
}
