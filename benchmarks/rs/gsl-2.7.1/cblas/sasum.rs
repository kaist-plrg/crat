use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_sasum(
    N: libc::c_int,
    mut X: *const libc::c_float,
    incX: libc::c_int,
) -> libc::c_float {
    let mut r: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if incX <= 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_float;
    }
    i = 0 as libc::c_int;
    while i < N {
        r = (r as libc::c_double + fabs(*X.offset(ix as isize) as libc::c_double))
            as libc::c_float;
        ix += incX;
        i += 1;
        i;
    }
    return r;
}
