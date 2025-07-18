use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
}
pub unsafe extern "C" fn gsl_ldexp(x: libc::c_double, e: libc::c_int) -> libc::c_double {
    let mut ex: libc::c_int = 0;
    if x == 0.0f64 {
        return x;
    }
    let mut y: libc::c_double = gsl_frexp(x, &mut ex);
    let mut e2: libc::c_double = (e + ex) as libc::c_double;
    let mut p2: libc::c_double = 0.;
    if e2 >= 1024 as libc::c_int as libc::c_double {
        y
            *= pow(
                2.0f64,
                e2 - 1024 as libc::c_int as libc::c_double
                    + 1 as libc::c_int as libc::c_double,
            );
        e2 = (1024 as libc::c_int - 1 as libc::c_int) as libc::c_double;
    } else if e2 <= -(1021 as libc::c_int) as libc::c_double {
        y
            *= pow(
                2.0f64,
                e2 - -(1021 as libc::c_int) as libc::c_double
                    - 1 as libc::c_int as libc::c_double,
            );
        e2 = (-(1021 as libc::c_int) + 1 as libc::c_int) as libc::c_double;
    }
    p2 = pow(2.0f64, e2);
    return y * p2;
}
pub unsafe extern "C" fn gsl_frexp(
    x: libc::c_double,
    mut e: *mut libc::c_int,
) -> libc::c_double {
    if x == 0.0f64 {
        *e = 0 as libc::c_int;
        return 0.0f64;
    } else if gsl_finite(x) == 0 {
        *e = 0 as libc::c_int;
        return x;
    } else if fabs(x) >= 0.5f64 && fabs(x) < 1 as libc::c_int as libc::c_double {
        *e = 0 as libc::c_int;
        return x;
    } else {
        let mut ex: libc::c_double = ceil(log(fabs(x)) / 0.69314718055994530942f64);
        let mut ei: libc::c_int = ex as libc::c_int;
        let mut f: libc::c_double = 0.;
        if ei < -(1021 as libc::c_int) {
            ei = -(1021 as libc::c_int);
        }
        if ei > --(1021 as libc::c_int) {
            ei = --(1021 as libc::c_int);
        }
        f = x * pow(2.0f64, -ei as libc::c_double);
        if gsl_finite(f) == 0 {
            *e = 0 as libc::c_int;
            return f;
        }
        while fabs(f) >= 1.0f64 {
            ei += 1;
            ei;
            f /= 2.0f64;
        }
        while fabs(f) > 0 as libc::c_int as libc::c_double && fabs(f) < 0.5f64 {
            ei -= 1;
            ei;
            f *= 2.0f64;
        }
        *e = ei;
        return f;
    };
}
