use ::libc;
extern "C" {
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_round2n(mut x: libc::c_double) -> libc::c_double {
    let mut e: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    (x > 0.0f64
        || {
            glp_assert_(
                b"x > 0.0\0" as *const u8 as *const libc::c_char,
                b"misc/round2n.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    f = frexp(x, &mut e);
    return ldexp(1.0f64, if f <= 0.75f64 { e - 1 as libc::c_int } else { e });
}
