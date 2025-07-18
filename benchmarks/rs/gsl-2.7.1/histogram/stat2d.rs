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
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram2d_sum(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let n: size_t = ((*h).nx).wrapping_mul((*h).ny);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n {
        let fresh0 = i;
        i = i.wrapping_add(1);
        sum += *((*h).bin).offset(fresh0 as isize);
    }
    return sum;
}
pub unsafe extern "C" fn gsl_histogram2d_xmean(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut wmean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < nx {
        let mut xi: libc::c_double = (*((*h).xrange)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            + *((*h).xrange).offset(i as isize)) / 2.0f64;
        let mut wi: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as size_t;
        while j < ny {
            let mut wij: libc::c_double = *((*h).bin)
                .offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
            if wij > 0 as libc::c_int as libc::c_double {
                wi += wij;
            }
            j = j.wrapping_add(1);
            j;
        }
        if wi > 0 as libc::c_int as libc::c_double {
            W += f128::f128::new(wi);
            wmean += (f128::f128::new(xi) - wmean) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    return wmean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_histogram2d_ymean(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut wmean: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    j = 0 as libc::c_int as size_t;
    while j < ny {
        let mut yj: libc::c_double = (*((*h).yrange)
            .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            + *((*h).yrange).offset(j as isize)) / 2.0f64;
        let mut wj: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < nx {
            let mut wij: libc::c_double = *((*h).bin)
                .offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
            if wij > 0 as libc::c_int as libc::c_double {
                wj += wij;
            }
            i = i.wrapping_add(1);
            i;
        }
        if wj > 0 as libc::c_int as libc::c_double {
            W += f128::f128::new(wj);
            wmean += (f128::f128::new(yj) - wmean) * (f128::f128::new(wj) / W);
        }
        j = j.wrapping_add(1);
        j;
    }
    return wmean.to_f64().unwrap();
}
pub unsafe extern "C" fn gsl_histogram2d_xsigma(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let xmean: libc::c_double = gsl_histogram2d_xmean(h);
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut wvariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < nx {
        let mut xi: libc::c_double = (*((*h).xrange)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            + *((*h).xrange).offset(i as isize)) / 2 as libc::c_int as libc::c_double
            - xmean;
        let mut wi: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as size_t;
        while j < ny {
            let mut wij: libc::c_double = *((*h).bin)
                .offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
            if wij > 0 as libc::c_int as libc::c_double {
                wi += wij;
            }
            j = j.wrapping_add(1);
            j;
        }
        if wi > 0 as libc::c_int as libc::c_double {
            W += f128::f128::new(wi);
            wvariance
                += (f128::f128::new(xi * xi) - wvariance) * (f128::f128::new(wi) / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut xsigma: libc::c_double = sqrt(wvariance.to_f64().unwrap());
    return xsigma;
}
pub unsafe extern "C" fn gsl_histogram2d_ysigma(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let ymean: libc::c_double = gsl_histogram2d_ymean(h);
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut wvariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    j = 0 as libc::c_int as size_t;
    while j < ny {
        let mut yj: libc::c_double = (*((*h).yrange)
            .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            + *((*h).yrange).offset(j as isize)) / 2.0f64 - ymean;
        let mut wj: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < nx {
            let mut wij: libc::c_double = *((*h).bin)
                .offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
            if wij > 0 as libc::c_int as libc::c_double {
                wj += wij;
            }
            i = i.wrapping_add(1);
            i;
        }
        if wj > 0 as libc::c_int as libc::c_double {
            W += f128::f128::new(wj);
            wvariance
                += (f128::f128::new(yj * yj) - wvariance) * (f128::f128::new(wj) / W);
        }
        j = j.wrapping_add(1);
        j;
    }
    let mut ysigma: libc::c_double = sqrt(wvariance.to_f64().unwrap());
    return ysigma;
}
pub unsafe extern "C" fn gsl_histogram2d_cov(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let xmean: libc::c_double = gsl_histogram2d_xmean(h);
    let ymean: libc::c_double = gsl_histogram2d_ymean(h);
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut wcovariance: f128::f128 = f128::f128::new(0 as libc::c_int);
    let mut W: f128::f128 = f128::f128::new(0 as libc::c_int);
    j = 0 as libc::c_int as size_t;
    while j < ny {
        i = 0 as libc::c_int as size_t;
        while i < nx {
            let mut xi: libc::c_double = (*((*h).xrange)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                + *((*h).xrange).offset(i as isize)) / 2.0f64 - xmean;
            let mut yj: libc::c_double = (*((*h).yrange)
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                + *((*h).yrange).offset(j as isize)) / 2.0f64 - ymean;
            let mut wij: libc::c_double = *((*h).bin)
                .offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
            if wij > 0 as libc::c_int as libc::c_double {
                W += f128::f128::new(wij);
                wcovariance
                    += (f128::f128::new(xi * yj) - wcovariance)
                        * (f128::f128::new(wij) / W);
            }
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return wcovariance.to_f64().unwrap();
}
