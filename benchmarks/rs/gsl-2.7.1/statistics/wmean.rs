use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_float_wmean(
    mut w: *const libc::c_float,
    wstride: size_t,
    mut data: *const libc::c_float,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut wmean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        let mut wi: libc::c_float = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_float {
            W += f128::f128::new(wi);
            wmean
                += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize))
                    - wmean) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wmean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_wmean(
    mut w: *const libc::c_double,
    wstride: size_t,
    mut data: *const libc::c_double,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut wmean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        let mut wi: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            W += f128::f128::new(wi);
            wmean
                += (f128::f128::new(*data.offset(i.wrapping_mul(stride) as isize))
                    - wmean) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wmean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_stats_long_double_wmean(
    mut w: *const f128::f128,
    wstride: size_t,
    mut data: *const f128::f128,
    stride: size_t,
    size: size_t,
) -> libc::c_double {
    let mut wmean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        let mut wi: f128::f128 = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > f128::f128::new(0 as libc::c_int) {
            W += wi;
            wmean += (*data.offset(i.wrapping_mul(stride) as isize) - wmean) * (wi / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wmean.to_f64().unwrap();
}
