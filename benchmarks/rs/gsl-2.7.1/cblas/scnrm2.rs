use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_scnrm2(
    N: libc::c_int,
    mut X: *const libc::c_void,
    incX: libc::c_int,
) -> libc::c_float {
    let mut scale: libc::c_float = 0.0f64 as libc::c_float;
    let mut ssq: libc::c_float = 1.0f64 as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if N == 0 as libc::c_int || incX < 1 as libc::c_int {
        return 0 as libc::c_int as libc::c_float;
    }
    i = 0 as libc::c_int;
    while i < N {
        let x: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as libc::c_int * ix) as isize);
        let y: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
        if x as libc::c_double != 0.0f64 {
            let ax: libc::c_float = fabs(x as libc::c_double) as libc::c_float;
            if scale < ax {
                ssq = (1.0f64 + (ssq * (scale / ax) * (scale / ax)) as libc::c_double)
                    as libc::c_float;
                scale = ax;
            } else {
                ssq += ax / scale * (ax / scale);
            }
        }
        if y as libc::c_double != 0.0f64 {
            let ay: libc::c_float = fabs(y as libc::c_double) as libc::c_float;
            if scale < ay {
                ssq = (1.0f64 + (ssq * (scale / ay) * (scale / ay)) as libc::c_double)
                    as libc::c_float;
                scale = ay;
            } else {
                ssq += ay / scale * (ay / scale);
            }
        }
        ix += incX;
        i += 1;
        i;
    }
    return (scale as libc::c_double * sqrt(ssq as libc::c_double)) as libc::c_float;
}
