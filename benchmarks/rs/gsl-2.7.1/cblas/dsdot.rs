use ::libc;
pub unsafe extern "C" fn cblas_dsdot(
    N: libc::c_int,
    mut X: *const libc::c_float,
    incX: libc::c_int,
    mut Y: *const libc::c_float,
    incY: libc::c_int,
) -> libc::c_double {
    let mut r: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = if incX > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incX
    };
    let mut iy: libc::c_int = if incY > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incY
    };
    i = 0 as libc::c_int;
    while i < N {
        r += (*X.offset(ix as isize) * *Y.offset(iy as isize)) as libc::c_double;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
    return r;
}
