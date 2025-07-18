use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_stats_median(
        sorted_data: *mut libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_uint_mad0(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_ushort_mad0(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_ulong_mad0(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_uchar_mad0(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_long_mad0(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_long_double_mad0(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = (*data.offset(i.wrapping_mul(stride) as isize)).to_f64().unwrap();
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            (*data.offset(i.wrapping_mul(stride) as isize)).to_f64().unwrap() - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_short_mad0(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_char_mad0(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_int_mad0(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_mad0(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work.offset(i as isize) = *data.offset(i.wrapping_mul(stride) as isize);
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(*data.offset(i.wrapping_mul(stride) as isize) - median);
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_float_mad0(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let mut mad: libc::c_double = 0.;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    median = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    i = 0 as libc::c_int as size_t;
    while i < n {
        *work
            .offset(
                i as isize,
            ) = fabs(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - median,
        );
        i = i.wrapping_add(1);
        i;
    }
    mad = gsl_stats_median(work, 1 as libc::c_int as size_t, n);
    return mad;
}
pub unsafe extern "C" fn gsl_stats_long_mad(
    mut data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_long_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_short_mad(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_short_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_ulong_mad(
    mut data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_ulong_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_uint_mad(
    mut data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_uint_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_uchar_mad(
    mut data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_uchar_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_int_mad(
    mut data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_int_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_float_mad(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_float_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_ushort_mad(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_ushort_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_mad(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_char_mad(
    mut data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_char_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
pub unsafe extern "C" fn gsl_stats_long_double_mad(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut mad0: libc::c_double = gsl_stats_long_double_mad0(data, stride, n, work);
    let mut mad: libc::c_double = 1.482602218505602f64 * mad0;
    return mad;
}
