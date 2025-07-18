use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_dnrm2(
    N: libc::c_int,
    mut X: *const libc::c_double,
    incX: libc::c_int,
) -> libc::c_double {
    let mut scale: libc::c_double = 0.0f64;
    let mut ssq: libc::c_double = 1.0f64;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if N <= 0 as libc::c_int || incX <= 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double
    } else if N == 1 as libc::c_int {
        return fabs(*X.offset(0 as libc::c_int as isize))
    }
    i = 0 as libc::c_int;
    while i < N {
        let x: libc::c_double = *X.offset(ix as isize);
        if x != 0.0f64 {
            let ax: libc::c_double = fabs(x);
            if scale < ax {
                ssq = 1.0f64 + ssq * (scale / ax) * (scale / ax);
                scale = ax;
            } else {
                ssq += ax / scale * (ax / scale);
            }
        }
        ix += incX;
        i += 1;
        i;
    }
    return scale * sqrt(ssq);
}
