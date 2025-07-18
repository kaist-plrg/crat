use ::libc;
pub unsafe extern "C" fn cblas_sswap(
    N: libc::c_int,
    mut X: *mut libc::c_float,
    incX: libc::c_int,
    mut Y: *mut libc::c_float,
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
        let tmp: libc::c_float = *X.offset(ix as isize);
        *X.offset(ix as isize) = *Y.offset(iy as isize);
        *Y.offset(iy as isize) = tmp;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}
