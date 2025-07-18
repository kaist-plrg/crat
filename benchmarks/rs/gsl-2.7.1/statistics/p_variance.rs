use ::libc;
extern "C" {
    fn gsl_stats_variance(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_variance(
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_variance(
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_variance(
        data: *const libc::c_ulong,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_variance(
        data: *const libc::c_long,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_variance(
        data: *const libc::c_uint,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_variance(
        data: *const libc::c_int,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_variance(
        data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_variance(
        data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_variance(
        data: *const libc::c_uchar,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_char_variance(
        data: *const libc::c_char,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_float_pvariance(
    mut data1: *const libc::c_float,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_float_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_float_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_uchar_pvariance(
    mut data1: *const libc::c_uchar,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_uchar,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_uchar_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_uchar_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_char_pvariance(
    mut data1: *const libc::c_char,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_char,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_char_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_char_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_ulong_pvariance(
    mut data1: *const libc::c_ulong,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_ulong,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_ulong_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_ulong_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_uint_pvariance(
    mut data1: *const libc::c_uint,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_uint,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_uint_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_uint_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_long_pvariance(
    mut data1: *const libc::c_long,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_long,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_long_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_long_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_int_pvariance(
    mut data1: *const libc::c_int,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_int,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_int_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_int_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_ushort_pvariance(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_ushort_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_ushort_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_short_pvariance(
    mut data1: *const libc::c_short,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_short_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_short_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_pvariance(
    mut data1: *const libc::c_double,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
pub unsafe extern "C" fn gsl_stats_long_double_pvariance(
    mut data1: *const f128::f128,
    stride1: size_t,
    n1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_long_double_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_long_double_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var1
        + n2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as libc::c_double;
    return pooled_variance;
}
