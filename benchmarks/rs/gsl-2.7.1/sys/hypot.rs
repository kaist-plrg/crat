use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_isinf(x: libc::c_double) -> libc::c_int;
}
pub unsafe extern "C" fn gsl_hypot(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    let mut xabs: libc::c_double = fabs(x);
    let mut yabs: libc::c_double = fabs(y);
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    if gsl_isinf(x) != 0 || gsl_isinf(y) != 0 {
        return ::std::f32::INFINITY as libc::c_double;
    }
    if xabs < yabs {
        min = xabs;
        max = yabs;
    } else {
        min = yabs;
        max = xabs;
    }
    if min == 0 as libc::c_int as libc::c_double {
        return max;
    }
    let mut u: libc::c_double = min / max;
    return max * sqrt(1 as libc::c_int as libc::c_double + u * u);
}
pub unsafe extern "C" fn gsl_hypot3(
    x: libc::c_double,
    y: libc::c_double,
    z: libc::c_double,
) -> libc::c_double {
    let mut xabs: libc::c_double = fabs(x);
    let mut yabs: libc::c_double = fabs(y);
    let mut zabs: libc::c_double = fabs(z);
    let mut w: libc::c_double = if xabs > (if yabs > zabs { yabs } else { zabs }) {
        xabs
    } else if yabs > zabs {
        yabs
    } else {
        zabs
    };
    if w == 0.0f64 {
        return 0.0f64
    } else {
        let mut r: libc::c_double = w
            * sqrt(
                xabs / w * (xabs / w) + yabs / w * (yabs / w) + zabs / w * (zabs / w),
            );
        return r;
    };
}
