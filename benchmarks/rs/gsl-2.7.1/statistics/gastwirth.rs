use ::libc;
extern "C" {
    fn gsl_stats_median_from_sorted_data(
        sorted_data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_median_from_sorted_data(
        sorted_data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_quantile_from_sorted_data(
        sorted_data: *const f128::f128,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_quantile_from_sorted_data(
        sorted_data: *const libc::c_double,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_char_quantile_from_sorted_data(
        sorted_data: *const libc::c_char,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_char_median_from_sorted_data(
        sorted_data: *const libc::c_char,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_median_from_sorted_data(
        sorted_data: *const libc::c_ulong,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_quantile_from_sorted_data(
        sorted_data: *const libc::c_ulong,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_long_median_from_sorted_data(
        sorted_data: *const libc::c_long,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_quantile_from_sorted_data(
        sorted_data: *const libc::c_long,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_uint_median_from_sorted_data(
        sorted_data: *const libc::c_uint,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_quantile_from_sorted_data(
        sorted_data: *const libc::c_uint,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_int_median_from_sorted_data(
        sorted_data: *const libc::c_int,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_quantile_from_sorted_data(
        sorted_data: *const libc::c_int,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_ushort_median_from_sorted_data(
        sorted_data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_quantile_from_sorted_data(
        sorted_data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_short_median_from_sorted_data(
        sorted_data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_quantile_from_sorted_data(
        sorted_data: *const libc::c_short,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_uchar_median_from_sorted_data(
        sorted_data: *const libc::c_uchar,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_quantile_from_sorted_data(
        sorted_data: *const libc::c_uchar,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_float_quantile_from_sorted_data(
        sorted_data: *const libc::c_float,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_float_median_from_sorted_data(
        sorted_data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_char_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_char,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_char_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_char_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_char_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_long_double_gastwirth_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_long_double_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_long_double_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_long_double_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_uint_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_uint_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_uint_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_uint_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_long_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_long,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_long_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_long_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_long_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_ulong_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_ulong_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_ulong_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_ulong_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_uchar_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_uchar_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_uchar_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_uchar_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_int_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_int,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_int_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_int_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_int_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_ushort_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_ushort_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_ushort_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_ushort_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_short_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_short_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_short_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_short_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
pub unsafe extern "C" fn gsl_stats_float_gastwirth_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut a: libc::c_double = gsl_stats_float_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            1.0f64 / 3.0f64,
        );
        let mut b: libc::c_double = gsl_stats_float_median_from_sorted_data(
            sorted_data,
            stride,
            n,
        );
        let mut c: libc::c_double = gsl_stats_float_quantile_from_sorted_data(
            sorted_data,
            stride,
            n,
            2.0f64 / 3.0f64,
        );
        let mut gastwirth: libc::c_double = 0.3f64 * a + 0.4f64 * b + 0.3f64 * c;
        return gastwirth;
    };
}
