use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn gsl_pow_uint(x: libc::c_double, n: libc::c_uint) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
#[inline]
unsafe extern "C" fn Stirling(mut y1: libc::c_double) -> libc::c_double {
    let mut y2: libc::c_double = y1 * y1;
    let mut s: libc::c_double = (13860.0f64
        - (462.0f64 - (132.0f64 - (99.0f64 - 140.0f64 / y2) / y2) / y2) / y2) / y1
        / 166320.0f64;
    return s;
}
pub unsafe extern "C" fn gsl_ran_binomial_tpe(
    mut rng: *const gsl_rng,
    mut p: libc::c_double,
    mut n: libc::c_uint,
) -> libc::c_uint {
    return gsl_ran_binomial(rng, p, n);
}
pub unsafe extern "C" fn gsl_ran_binomial(
    mut rng: *const gsl_rng,
    mut p: libc::c_double,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut ix: libc::c_int = 0;
    let mut flipped: libc::c_int = 0 as libc::c_int;
    let mut q: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut np: libc::c_double = 0.;
    if n == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    if p > 0.5f64 {
        p = 1.0f64 - p;
        flipped = 1 as libc::c_int;
    }
    q = 1 as libc::c_int as libc::c_double - p;
    s = p / q;
    np = n as libc::c_double * p;
    if np < 14 as libc::c_int as libc::c_double {
        let mut f0: libc::c_double = gsl_pow_uint(q, n);
        's_47: loop {
            let mut f: libc::c_double = f0;
            let mut u: libc::c_double = gsl_rng_uniform(rng);
            ix = 0 as libc::c_int;
            while ix <= 110 as libc::c_int {
                if u < f {
                    break 's_47;
                }
                u -= f;
                f
                    *= s * n.wrapping_sub(ix as libc::c_uint) as libc::c_double
                        / (ix + 1 as libc::c_int) as libc::c_double;
                ix += 1;
                ix;
            }
        }
    } else {
        let mut k: libc::c_int = 0;
        let mut ffm: libc::c_double = np + p;
        let mut m: libc::c_int = ffm as libc::c_int;
        let mut fm: libc::c_double = m as libc::c_double;
        let mut xm: libc::c_double = fm + 0.5f64;
        let mut npq: libc::c_double = np * q;
        let mut p1: libc::c_double = floor(2.195f64 * sqrt(npq) - 4.6f64 * q) + 0.5f64;
        let mut xl: libc::c_double = xm - p1;
        let mut xr: libc::c_double = xm + p1;
        let mut c: libc::c_double = 0.134f64 + 20.5f64 / (15.3f64 + fm);
        let mut p2: libc::c_double = p1 * (1.0f64 + c + c);
        let mut al: libc::c_double = (ffm - xl) / (ffm - xl * p);
        let mut lambda_l: libc::c_double = al * (1.0f64 + 0.5f64 * al);
        let mut ar: libc::c_double = (xr - ffm) / (xr * q);
        let mut lambda_r: libc::c_double = ar * (1.0f64 + 0.5f64 * ar);
        let mut p3: libc::c_double = p2 + c / lambda_l;
        let mut p4: libc::c_double = p3 + c / lambda_r;
        let mut var: libc::c_double = 0.;
        let mut accept: libc::c_double = 0.;
        let mut u_0: libc::c_double = 0.;
        let mut v: libc::c_double = 0.;
        loop {
            u_0 = gsl_rng_uniform(rng) * p4;
            v = gsl_rng_uniform(rng);
            if u_0 <= p1 {
                ix = (xm - p1 * v + u_0) as libc::c_int;
                break;
            } else {
                if u_0 <= p2 {
                    let mut x: libc::c_double = xl + (u_0 - p1) / c;
                    v = v * c + 1.0f64 - fabs(x - xm) / p1;
                    if v > 1.0f64 || v <= 0.0f64 {
                        continue;
                    }
                    ix = x as libc::c_int;
                } else if u_0 <= p3 {
                    ix = (xl + log(v) / lambda_l) as libc::c_int;
                    if ix < 0 as libc::c_int {
                        continue;
                    }
                    v *= (u_0 - p2) * lambda_l;
                } else {
                    ix = (xr - log(v) / lambda_r) as libc::c_int;
                    if ix as libc::c_double > n as libc::c_double {
                        continue;
                    }
                    v *= (u_0 - p3) * lambda_r;
                }
                k = abs(ix - m);
                if k <= 20 as libc::c_int {
                    let mut g: libc::c_double = n
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_double
                        * s;
                    let mut f_0: libc::c_double = 1.0f64;
                    var = v;
                    if m < ix {
                        let mut i: libc::c_int = 0;
                        i = m + 1 as libc::c_int;
                        while i <= ix {
                            f_0 *= g / i as libc::c_double - s;
                            i += 1;
                            i;
                        }
                    } else if m > ix {
                        let mut i_0: libc::c_int = 0;
                        i_0 = ix + 1 as libc::c_int;
                        while i_0 <= m {
                            f_0 /= g / i_0 as libc::c_double - s;
                            i_0 += 1;
                            i_0;
                        }
                    }
                    accept = f_0;
                } else {
                    var = log(v);
                    if (k as libc::c_double)
                        < npq / 2 as libc::c_int as libc::c_double
                            - 1 as libc::c_int as libc::c_double
                    {
                        let mut amaxp: libc::c_double = k as libc::c_double / npq
                            * ((k as libc::c_double
                                * (k as libc::c_double / 3.0f64 + 0.625f64)
                                + 1.0f64 / 6.0f64) / npq + 0.5f64);
                        let mut ynorm: libc::c_double = -((k * k) as libc::c_double
                            / (2.0f64 * npq));
                        if var < ynorm - amaxp {
                            break;
                        }
                        if var > ynorm + amaxp {
                            continue;
                        }
                    }
                    let mut x1: libc::c_double = ix as libc::c_double + 1.0f64;
                    let mut w1: libc::c_double = n.wrapping_sub(ix as libc::c_uint)
                        as libc::c_double + 1.0f64;
                    let mut f1: libc::c_double = fm + 1.0f64;
                    let mut z1: libc::c_double = n as libc::c_double + 1.0f64 - fm;
                    accept = xm * log(f1 / x1)
                        + (n.wrapping_sub(m as libc::c_uint) as libc::c_double + 0.5f64)
                            * log(z1 / w1)
                        + (ix - m) as libc::c_double * log(w1 * p / (x1 * q))
                        + Stirling(f1) + Stirling(z1) - Stirling(x1) - Stirling(w1);
                }
                if var <= accept {
                    break;
                }
            }
        }
    }
    return if flipped != 0 {
        n.wrapping_sub(ix as libc::c_uint)
    } else {
        ix as libc::c_uint
    };
}
