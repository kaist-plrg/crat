use ::libc;
pub unsafe extern "C" fn cblas_srot(
    N: libc::c_int,
    mut X: *mut libc::c_float,
    incX: libc::c_int,
    mut Y: *mut libc::c_float,
    incY: libc::c_int,
    c: libc::c_float,
    s: libc::c_float,
) {
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
        let x: libc::c_float = *X.offset(ix as isize);
        let y: libc::c_float = *Y.offset(iy as isize);
        *X.offset(ix as isize) = c * x + s * y;
        *Y.offset(iy as isize) = -s * x + c * y;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}
