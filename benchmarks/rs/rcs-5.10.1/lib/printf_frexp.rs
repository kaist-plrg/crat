use ::libc;
extern "C" {
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
}
pub unsafe extern "C" fn printf_frexp(
    mut x: libc::c_double,
    mut expptr: *mut libc::c_int,
) -> libc::c_double {
    let mut exponent: libc::c_int = 0;
    x = frexp(x, &mut exponent);
    x = x + x;
    exponent -= 1 as libc::c_int;
    if exponent < -(1021 as libc::c_int) - 1 as libc::c_int {
        x = ldexp(x, exponent - (-(1021 as libc::c_int) - 1 as libc::c_int));
        exponent = -(1021 as libc::c_int) - 1 as libc::c_int;
    }
    *expptr = exponent;
    return x;
}
