use ::libc;
pub unsafe extern "C" fn cblas_dcopy(
    N: libc::c_int,
    mut X: *const libc::c_double,
    incX: libc::c_int,
    mut Y: *mut libc::c_double,
    incY: libc::c_int,
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
        *Y.offset(iy as isize) = *X.offset(ix as isize);
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}
