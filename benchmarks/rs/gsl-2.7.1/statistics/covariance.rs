use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_stats_char_mean(
        data: *const libc::c_char,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
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
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_sort_vector2(v1: *mut gsl_vector, v2: *mut gsl_vector);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_vector_view = _gsl_vector_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
unsafe extern "C" fn compute_long_double_covariance(
    mut data1: *const f128::f128,
    stride1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = *data1.offset(i.wrapping_mul(stride1) as isize)
            - f128::f128::new(mean1);
        let delta2: f128::f128 = *data2.offset(i.wrapping_mul(stride2) as isize)
            - f128::f128::new(mean2);
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_covariance(
    mut data1: *const libc::c_double,
    stride1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_ushort_covariance(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int
                as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int
                as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_int_covariance(
    mut data1: *const libc::c_int,
    stride1: size_t,
    mut data2: *const libc::c_int,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_long_covariance(
    mut data1: *const libc::c_long,
    stride1: size_t,
    mut data2: *const libc::c_long,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_short_covariance(
    mut data1: *const libc::c_short,
    stride1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int
                as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int
                as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_uchar_covariance(
    mut data1: *const libc::c_uchar,
    stride1: size_t,
    mut data2: *const libc::c_uchar,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int
                as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int
                as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_uint_covariance(
    mut data1: *const libc::c_uint,
    stride1: size_t,
    mut data2: *const libc::c_uint,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_ulong_covariance(
    mut data1: *const libc::c_ulong,
    stride1: size_t,
    mut data2: *const libc::c_ulong,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_float_covariance(
    mut data1: *const libc::c_float,
    stride1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
unsafe extern "C" fn compute_char_covariance(
    mut data1: *const libc::c_char,
    stride1: size_t,
    mut data2: *const libc::c_char,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let mut covariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let delta1: f128::f128 = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int
                as libc::c_double - mean1,
        );
        let delta2: f128::f128 = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int
                as libc::c_double - mean2,
        );
        covariance
            += (delta1 * delta2 - covariance)
                / f128::f128::new(i.wrapping_add(1 as libc::c_int as libc::c_ulong));
        i = i.wrapping_add(1);
        i;
    }
    return covariance.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_uchar_covariance_m(
    mut data1: *const libc::c_uchar,
    stride1: size_t,
    mut data2: *const libc::c_uchar,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_uchar_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_ushort_covariance_m(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_ushort_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_char_covariance_m(
    mut data1: *const libc::c_char,
    stride1: size_t,
    mut data2: *const libc::c_char,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_char_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_short_covariance_m(
    mut data1: *const libc::c_short,
    stride1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_short_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_uint_covariance_m(
    mut data1: *const libc::c_uint,
    stride1: size_t,
    mut data2: *const libc::c_uint,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_uint_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_long_double_covariance_m(
    mut data1: *const f128::f128,
    stride1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_long_double_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_covariance_m(
    mut data1: *const libc::c_double,
    stride1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_long_covariance_m(
    mut data1: *const libc::c_long,
    stride1: size_t,
    mut data2: *const libc::c_long,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_long_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_int_covariance_m(
    mut data1: *const libc::c_int,
    stride1: size_t,
    mut data2: *const libc::c_int,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_int_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_float_covariance_m(
    mut data1: *const libc::c_float,
    stride1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_float_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_ulong_covariance_m(
    mut data1: *const libc::c_ulong,
    stride1: size_t,
    mut data2: *const libc::c_ulong,
    stride2: size_t,
    n: size_t,
    mean1: libc::c_double,
    mean2: libc::c_double,
) -> libc::c_double {
    let covariance: libc::c_double = compute_ulong_covariance(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
    return covariance
        * (n as libc::c_double
            / n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double);
}
pub unsafe extern "C" fn gsl_stats_float_covariance(
    mut data1: *const libc::c_float,
    stride1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_float_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_float_mean(data2, stride2, n);
    return gsl_stats_float_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_covariance(
    mut data1: *const libc::c_double,
    stride1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_mean(data2, stride2, n);
    return gsl_stats_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_int_covariance(
    mut data1: *const libc::c_int,
    stride1: size_t,
    mut data2: *const libc::c_int,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_int_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_int_mean(data2, stride2, n);
    return gsl_stats_int_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_uchar_covariance(
    mut data1: *const libc::c_uchar,
    stride1: size_t,
    mut data2: *const libc::c_uchar,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_uchar_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_uchar_mean(data2, stride2, n);
    return gsl_stats_uchar_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_long_double_covariance(
    mut data1: *const f128::f128,
    stride1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_long_double_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_long_double_mean(data2, stride2, n);
    return gsl_stats_long_double_covariance_m(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
}
pub unsafe extern "C" fn gsl_stats_ushort_covariance(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_ushort_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_ushort_mean(data2, stride2, n);
    return gsl_stats_ushort_covariance_m(
        data1,
        stride1,
        data2,
        stride2,
        n,
        mean1,
        mean2,
    );
}
pub unsafe extern "C" fn gsl_stats_short_covariance(
    mut data1: *const libc::c_short,
    stride1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_short_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_short_mean(data2, stride2, n);
    return gsl_stats_short_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_char_covariance(
    mut data1: *const libc::c_char,
    stride1: size_t,
    mut data2: *const libc::c_char,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_char_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_char_mean(data2, stride2, n);
    return gsl_stats_char_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_ulong_covariance(
    mut data1: *const libc::c_ulong,
    stride1: size_t,
    mut data2: *const libc::c_ulong,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_ulong_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_ulong_mean(data2, stride2, n);
    return gsl_stats_ulong_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_uint_covariance(
    mut data1: *const libc::c_uint,
    stride1: size_t,
    mut data2: *const libc::c_uint,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_uint_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_uint_mean(data2, stride2, n);
    return gsl_stats_uint_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_long_covariance(
    mut data1: *const libc::c_long,
    stride1: size_t,
    mut data2: *const libc::c_long,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mean1: libc::c_double = gsl_stats_long_mean(data1, stride1, n);
    let mean2: libc::c_double = gsl_stats_long_mean(data2, stride2, n);
    return gsl_stats_long_covariance_m(data1, stride1, data2, stride2, n, mean1, mean2);
}
pub unsafe extern "C" fn gsl_stats_long_double_correlation(
    mut data1: *const f128::f128,
    stride1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = *data1
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize);
    mean_y = *data2
        .offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize);
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = *data1.offset(i.wrapping_mul(stride1) as isize) - mean_x;
        delta_y = *data2.offset(i.wrapping_mul(stride2) as isize) - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_ushort_correlation(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int,
        ) - mean_x;
        delta_y = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int,
        ) - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_ulong_correlation(
    mut data1: *const libc::c_ulong,
    stride1: size_t,
    mut data2: *const libc::c_ulong,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(*data1.offset(i.wrapping_mul(stride1) as isize))
            - mean_x;
        delta_y = f128::f128::new(*data2.offset(i.wrapping_mul(stride2) as isize))
            - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_long_correlation(
    mut data1: *const libc::c_long,
    stride1: size_t,
    mut data2: *const libc::c_long,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(*data1.offset(i.wrapping_mul(stride1) as isize))
            - mean_x;
        delta_y = f128::f128::new(*data2.offset(i.wrapping_mul(stride2) as isize))
            - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_correlation(
    mut data1: *const libc::c_double,
    stride1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(*data1.offset(i.wrapping_mul(stride1) as isize))
            - mean_x;
        delta_y = f128::f128::new(*data2.offset(i.wrapping_mul(stride2) as isize))
            - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_short_correlation(
    mut data1: *const libc::c_short,
    stride1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int,
        ) - mean_x;
        delta_y = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int,
        ) - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_uchar_correlation(
    mut data1: *const libc::c_uchar,
    stride1: size_t,
    mut data2: *const libc::c_uchar,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int,
        ) - mean_x;
        delta_y = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int,
        ) - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_int_correlation(
    mut data1: *const libc::c_int,
    stride1: size_t,
    mut data2: *const libc::c_int,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(*data1.offset(i.wrapping_mul(stride1) as isize))
            - mean_x;
        delta_y = f128::f128::new(*data2.offset(i.wrapping_mul(stride2) as isize))
            - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_uint_correlation(
    mut data1: *const libc::c_uint,
    stride1: size_t,
    mut data2: *const libc::c_uint,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(*data1.offset(i.wrapping_mul(stride1) as isize))
            - mean_x;
        delta_y = f128::f128::new(*data2.offset(i.wrapping_mul(stride2) as isize))
            - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_float_correlation(
    mut data1: *const libc::c_float,
    stride1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(*data1.offset(i.wrapping_mul(stride1) as isize))
            - mean_x;
        delta_y = f128::f128::new(*data2.offset(i.wrapping_mul(stride2) as isize))
            - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_char_correlation(
    mut data1: *const libc::c_char,
    stride1: size_t,
    mut data2: *const libc::c_char,
    stride2: size_t,
    n: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum_xsq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_ysq: f128::f128 = f128::f128::new(0.0f64);
    let mut sum_cross: f128::f128 = f128::f128::new(0.0f64);
    let mut ratio: f128::f128 = f128::f128::ZERO;
    let mut delta_x: f128::f128 = f128::f128::ZERO;
    let mut delta_y: f128::f128 = f128::f128::ZERO;
    let mut mean_x: f128::f128 = f128::f128::ZERO;
    let mut mean_y: f128::f128 = f128::f128::ZERO;
    let mut r: f128::f128 = f128::f128::ZERO;
    mean_x = f128::f128::new(
        *data1.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride1) as isize),
    );
    mean_y = f128::f128::new(
        *data2.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(stride2) as isize),
    );
    i = 1 as libc::c_int as size_t;
    while i < n {
        ratio = f128::f128::new(i as libc::c_double / (i as libc::c_double + 1.0f64));
        delta_x = f128::f128::new(
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_int,
        ) - mean_x;
        delta_y = f128::f128::new(
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_int,
        ) - mean_y;
        sum_xsq += delta_x * delta_x * ratio;
        sum_ysq += delta_y * delta_y * ratio;
        sum_cross += delta_x * delta_y * ratio;
        mean_x += delta_x / f128::f128::new(i as libc::c_double + 1.0f64);
        mean_y += delta_y / f128::f128::new(i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    r = sum_cross
        / f128::f128::new(
            sqrt(sum_xsq.to_f64().unwrap()) * sqrt(sum_ysq.to_f64().unwrap()),
        );
    return r.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_float_spearman(
    mut data1: *const libc::c_float,
    stride1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_short_spearman(
    mut data1: *const libc::c_short,
    stride1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_uchar_spearman(
    mut data1: *const libc::c_uchar,
    stride1: size_t,
    mut data2: *const libc::c_uchar,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_char_spearman(
    mut data1: *const libc::c_char,
    stride1: size_t,
    mut data2: *const libc::c_char,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_ushort_spearman(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_uint_spearman(
    mut data1: *const libc::c_uint,
    stride1: size_t,
    mut data2: *const libc::c_uint,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_long_double_spearman(
    mut data1: *const f128::f128,
    stride1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            (*data1.offset(i.wrapping_mul(stride1) as isize)).to_f64().unwrap(),
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            (*data2.offset(i.wrapping_mul(stride2) as isize)).to_f64().unwrap(),
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_ulong_spearman(
    mut data1: *const libc::c_ulong,
    stride1: size_t,
    mut data2: *const libc::c_ulong,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_spearman(
    mut data1: *const libc::c_double,
    stride1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize),
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_int_spearman(
    mut data1: *const libc::c_int,
    stride1: size_t,
    mut data2: *const libc::c_int,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
pub unsafe extern "C" fn gsl_stats_long_spearman(
    mut data1: *const libc::c_long,
    stride1: size_t,
    mut data2: *const libc::c_long,
    stride2: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut ranks1: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(0 as libc::c_int as isize),
        n,
    );
    let mut ranks2: gsl_vector_view = gsl_vector_view_array(
        &mut *work.offset(n as isize),
        n,
    );
    let mut r: libc::c_double = 0.;
    i = 0 as libc::c_int as size_t;
    while i < n {
        gsl_vector_set(
            &mut ranks1.vector,
            i,
            *data1.offset(i.wrapping_mul(stride1) as isize) as libc::c_double,
        );
        gsl_vector_set(
            &mut ranks2.vector,
            i,
            *data2.offset(i.wrapping_mul(stride2) as isize) as libc::c_double,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_sort_vector2(&mut ranks1.vector, &mut ranks2.vector);
    compute_rank(&mut ranks1.vector);
    gsl_sort_vector2(&mut ranks2.vector, &mut ranks1.vector);
    compute_rank(&mut ranks2.vector);
    r = gsl_stats_correlation(
        ranks1.vector.data as *const libc::c_double,
        ranks1.vector.stride,
        ranks2.vector.data as *const libc::c_double,
        ranks2.vector.stride,
        n,
    );
    return r;
}
unsafe extern "C" fn compute_rank(mut v: *mut gsl_vector) -> libc::c_int {
    let n: size_t = (*v).size;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut vi: libc::c_double = gsl_vector_get(v, i);
        if vi == gsl_vector_get(v, i.wrapping_add(1 as libc::c_int as libc::c_ulong)) {
            let mut j: size_t = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut k: size_t = 0;
            let mut rank: libc::c_double = 0.0f64;
            while j < n && vi == gsl_vector_get(v, j) {
                j = j.wrapping_add(1);
                j;
            }
            k = i;
            while k < j {
                rank += k as libc::c_double + 1.0f64;
                k = k.wrapping_add(1);
                k;
            }
            rank /= j.wrapping_sub(i) as libc::c_double;
            k = i;
            while k < j {
                gsl_vector_set(v, k, rank);
                k = k.wrapping_add(1);
                k;
            }
            i = j;
        } else {
            gsl_vector_set(v, i, i as libc::c_double + 1.0f64);
            i = i.wrapping_add(1);
            i;
        }
    }
    if i == n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        gsl_vector_set(
            v,
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            n as libc::c_double,
        );
    }
    return GSL_SUCCESS as libc::c_int;
}
