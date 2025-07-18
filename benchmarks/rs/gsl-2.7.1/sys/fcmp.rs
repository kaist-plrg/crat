use ::libc;
extern "C" {
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_fcmp(
    x1: libc::c_double,
    x2: libc::c_double,
    epsilon: libc::c_double,
) -> libc::c_int {
    let mut exponent: libc::c_int = 0;
    let mut delta: libc::c_double = 0.;
    let mut difference: libc::c_double = 0.;
    let mut max: libc::c_double = if fabs(x1) > fabs(x2) { x1 } else { x2 };
    frexp(max, &mut exponent);
    delta = ldexp(epsilon, exponent);
    difference = x1 - x2;
    if difference > delta {
        return 1 as libc::c_int
    } else if difference < -delta {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
