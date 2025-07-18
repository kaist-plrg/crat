use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_acosh(x: libc::c_double) -> libc::c_double {
    if x > 1.0f64 / 1.4901161193847656e-08f64 {
        return log(x) + 0.69314718055994530942f64
    } else if x > 2 as libc::c_int as libc::c_double {
        return log(
            2 as libc::c_int as libc::c_double * x
                - 1 as libc::c_int as libc::c_double
                    / (sqrt(x * x - 1 as libc::c_int as libc::c_double) + x),
        )
    } else if x > 1 as libc::c_int as libc::c_double {
        let mut t: libc::c_double = x - 1 as libc::c_int as libc::c_double;
        return log1p(t + sqrt(2 as libc::c_int as libc::c_double * t + t * t));
    } else if x == 1 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double
    } else {
        return ::std::f32::NAN as libc::c_double
    };
}
pub unsafe extern "C" fn gsl_asinh(x: libc::c_double) -> libc::c_double {
    let mut a: libc::c_double = fabs(x);
    let mut s: libc::c_double = (if x < 0 as libc::c_int as libc::c_double {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as libc::c_double;
    if a > 1 as libc::c_int as libc::c_double / 1.4901161193847656e-08f64 {
        return s * (log(a) + 0.69314718055994530942f64)
    } else if a > 2 as libc::c_int as libc::c_double {
        return s
            * log(
                2 as libc::c_int as libc::c_double * a
                    + 1 as libc::c_int as libc::c_double
                        / (a + sqrt(a * a + 1 as libc::c_int as libc::c_double)),
            )
    } else if a > 1.4901161193847656e-08f64 {
        let mut a2: libc::c_double = a * a;
        return s
            * log1p(
                a
                    + a2
                        / (1 as libc::c_int as libc::c_double
                            + sqrt(1 as libc::c_int as libc::c_double + a2)),
            );
    } else {
        return x
    };
}
pub unsafe extern "C" fn gsl_atanh(x: libc::c_double) -> libc::c_double {
    let mut a: libc::c_double = fabs(x);
    let mut s: libc::c_double = (if x < 0 as libc::c_int as libc::c_double {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as libc::c_double;
    if a > 1 as libc::c_int as libc::c_double {
        return ::std::f32::NAN as libc::c_double
    } else if a == 1 as libc::c_int as libc::c_double {
        return (if x < 0 as libc::c_int as libc::c_double {
            -::std::f32::INFINITY
        } else {
            ::std::f32::INFINITY
        }) as libc::c_double
    } else if a >= 0.5f64 {
        return s * 0.5f64
            * log1p(
                2 as libc::c_int as libc::c_double * a
                    / (1 as libc::c_int as libc::c_double - a),
            )
    } else if a > 2.2204460492503131e-16f64 {
        return s * 0.5f64
            * log1p(
                2 as libc::c_int as libc::c_double * a
                    + 2 as libc::c_int as libc::c_double * a * a
                        / (1 as libc::c_int as libc::c_double - a),
            )
    } else {
        return x
    };
}
