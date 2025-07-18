use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_stats_long_double_mean(
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_pvariance(
        data1: *const f128::f128,
        stride1: size_t,
        n1: size_t,
        data2: *const f128::f128,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_mean(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_pvariance(
        data1: *const libc::c_double,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_double,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_mean(
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_pvariance(
        data1: *const libc::c_float,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_float,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_mean(
        data: *const libc::c_ulong,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_pvariance(
        data1: *const libc::c_ulong,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_ulong,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_mean(
        data: *const libc::c_long,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_pvariance(
        data1: *const libc::c_long,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_long,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_mean(
        data: *const libc::c_uint,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_pvariance(
        data1: *const libc::c_uint,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_uint,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_mean(
        data: *const libc::c_int,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_pvariance(
        data1: *const libc::c_int,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_int,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_mean(
        data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_pvariance(
        data1: *const libc::c_ushort,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_ushort,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_mean(
        data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_pvariance(
        data1: *const libc::c_short,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_short,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_mean(
        data: *const libc::c_uchar,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_pvariance(
        data1: *const libc::c_uchar,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_uchar,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
    fn gsl_stats_char_mean(
        data: *const libc::c_char,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_char_pvariance(
        data1: *const libc::c_char,
        stride1: size_t,
        n1: size_t,
        data2: *const libc::c_char,
        stride2: size_t,
        n2: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_char_ttest(
    mut data1: *const libc::c_char,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_char,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_char_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_char_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_char_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_ulong_ttest(
    mut data1: *const libc::c_ulong,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_ulong,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_ulong_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_ulong_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_ulong_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_long_ttest(
    mut data1: *const libc::c_long,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_long,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_long_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_long_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_long_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_uint_ttest(
    mut data1: *const libc::c_uint,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_uint,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_uint_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_uint_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_uint_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_int_ttest(
    mut data1: *const libc::c_int,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_int,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_int_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_int_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_int_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_ushort_ttest(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_ushort_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_ushort_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_ushort_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_short_ttest(
    mut data1: *const libc::c_short,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_short_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_short_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_short_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_uchar_ttest(
    mut data1: *const libc::c_uchar,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_uchar,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_uchar_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_uchar_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_uchar_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_float_ttest(
    mut data1: *const libc::c_float,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_float_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_float_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_float_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_ttest(
    mut data1: *const libc::c_double,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_pvariance(data1, stride1, n1, data2, stride2, n2);
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
pub unsafe extern "C" fn gsl_stats_long_double_ttest(
    mut data1: *const f128::f128,
    stride1: size_t,
    n1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_long_double_mean(data1, stride1, n1);
    let mean2: libc::c_double = gsl_stats_long_double_mean(data2, stride2, n2);
    let pv: libc::c_double = gsl_stats_long_double_pvariance(
        data1,
        stride1,
        n1,
        data2,
        stride2,
        n2,
    );
    let t: libc::c_double = (mean1 - mean2)
        / sqrt(pv * (1.0f64 / n1 as libc::c_double + 1.0f64 / n2 as libc::c_double));
    return t;
}
