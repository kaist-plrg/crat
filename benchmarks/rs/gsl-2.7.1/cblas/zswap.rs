use ::libc;
pub unsafe extern "C" fn cblas_zswap(
    N: libc::c_int,
    mut X: *mut libc::c_void,
    incX: libc::c_int,
    mut Y: *mut libc::c_void,
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
        let tmp_real: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as libc::c_int * ix) as isize);
        let tmp_imag: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
        *(X as *mut libc::c_double)
            .offset(
                (2 as libc::c_int * ix) as isize,
            ) = *(Y as *mut libc::c_double).offset((2 as libc::c_int * iy) as isize);
        *(X as *mut libc::c_double)
            .offset(
                (2 as libc::c_int * ix + 1 as libc::c_int) as isize,
            ) = *(Y as *mut libc::c_double)
            .offset((2 as libc::c_int * iy + 1 as libc::c_int) as isize);
        *(Y as *mut libc::c_double).offset((2 as libc::c_int * iy) as isize) = tmp_real;
        *(Y as *mut libc::c_double)
            .offset((2 as libc::c_int * iy + 1 as libc::c_int) as isize) = tmp_imag;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}
