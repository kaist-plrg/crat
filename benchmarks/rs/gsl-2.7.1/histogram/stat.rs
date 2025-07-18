use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram_mean(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    let n: size_t = (*h).n;
    let mut i: size_t = 0;
    let mut wmean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = (*((*h).range)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            + *((*h).range).offset(i as isize)) / 2 as libc::c_int as libc::c_double;
        let mut wi: libc::c_double = *((*h).bin).offset(i as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            W += f128::f128::new(wi);
            wmean += (f128::f128::new(xi) - wmean) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wmean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_histogram_sigma(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    let n: size_t = (*h).n;
    let mut i: size_t = 0;
    let mut wvariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut wmean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi: libc::c_double = (*((*h).range)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            + *((*h).range).offset(i as isize)) / 2 as libc::c_int as libc::c_double;
        let mut wi: libc::c_double = *((*h).bin).offset(i as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            W += f128::f128::new(wi);
            wmean += (f128::f128::new(xi) - wmean) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    W = f128::f128::new(0.0f64);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut xi_0: libc::c_double = (*((*h).range)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            + *((*h).range).offset(i as isize)) / 2 as libc::c_int as libc::c_double;
        let mut wi_0: libc::c_double = *((*h).bin).offset(i as isize);
        if wi_0 > 0 as libc::c_int as libc::c_double {
            let delta: f128::f128 = f128::f128::new(xi_0) - wmean;
            W += f128::f128::new(wi_0);
            wvariance += (delta * delta - wvariance) * (f128::f128::new(wi_0) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut sigma: libc::c_double = sqrt(wvariance.to_f64().unwrap());
    return sigma;
}
pub unsafe extern "C" fn gsl_histogram_sum(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = 0;
    n = (*h).n;
    while i < n {
        let fresh0 = i;
        i = i.wrapping_add(1);
        sum += *((*h).bin).offset(fresh0 as isize);
    }
    return sum;
}
