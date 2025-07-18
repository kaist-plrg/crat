use ::libc;
pub unsafe extern "C" fn cblas_saxpy(
    N: libc::c_int,
    alpha: libc::c_float,
    mut X: *const libc::c_float,
    incX: libc::c_int,
    mut Y: *mut libc::c_float,
    incY: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if alpha as libc::c_double == 0.0f64 {
        return;
    }
    if incX == 1 as libc::c_int && incY == 1 as libc::c_int {
        let m: libc::c_int = N % 4 as libc::c_int;
        i = 0 as libc::c_int;
        while i < m {
            *Y.offset(i as isize) += alpha * *X.offset(i as isize);
            i += 1;
            i;
        }
        i = m;
        while (i + 3 as libc::c_int) < N {
            *Y.offset(i as isize) += alpha * *X.offset(i as isize);
            *Y.offset((i + 1 as libc::c_int) as isize)
                += alpha * *X.offset((i + 1 as libc::c_int) as isize);
            *Y.offset((i + 2 as libc::c_int) as isize)
                += alpha * *X.offset((i + 2 as libc::c_int) as isize);
            *Y.offset((i + 3 as libc::c_int) as isize)
                += alpha * *X.offset((i + 3 as libc::c_int) as isize);
            i += 4 as libc::c_int;
        }
    } else {
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
            *Y.offset(iy as isize) += alpha * *X.offset(ix as isize);
            ix += incX;
            iy += incY;
            i += 1;
            i;
        }
    };
}
