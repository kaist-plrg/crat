use ::libc;
extern "C" {
    fn cblas_xerbla(
        p: libc::c_int,
        rout: *const libc::c_char,
        form: *const libc::c_char,
        _: ...
    );
}
pub unsafe extern "C" fn cblas_drotm(
    N: libc::c_int,
    mut X: *mut libc::c_double,
    incX: libc::c_int,
    mut Y: *mut libc::c_double,
    incY: libc::c_int,
    mut P: *const libc::c_double,
) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = if incX > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incX
    };
    let mut j: libc::c_int = if incY > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incY
    };
    let mut h11: libc::c_double = 0.;
    let mut h21: libc::c_double = 0.;
    let mut h12: libc::c_double = 0.;
    let mut h22: libc::c_double = 0.;
    if *P.offset(0 as libc::c_int as isize) == -1.0f64 {
        h11 = *P.offset(1 as libc::c_int as isize);
        h21 = *P.offset(2 as libc::c_int as isize);
        h12 = *P.offset(3 as libc::c_int as isize);
        h22 = *P.offset(4 as libc::c_int as isize);
    } else if *P.offset(0 as libc::c_int as isize) == 0.0f64 {
        h11 = 1.0f64;
        h21 = *P.offset(2 as libc::c_int as isize);
        h12 = *P.offset(3 as libc::c_int as isize);
        h22 = 1.0f64;
    } else if *P.offset(0 as libc::c_int as isize) == 1.0f64 {
        h11 = *P.offset(1 as libc::c_int as isize);
        h21 = -1.0f64;
        h12 = 1.0f64;
        h22 = *P.offset(4 as libc::c_int as isize);
    } else if *P.offset(0 as libc::c_int as isize) == -2.0f64 {
        return
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_rotm.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized value of P[0]\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    n = 0 as libc::c_int;
    while n < N {
        let w: libc::c_double = *X.offset(i as isize);
        let z: libc::c_double = *Y.offset(j as isize);
        *X.offset(i as isize) = h11 * w + h12 * z;
        *Y.offset(j as isize) = h21 * w + h22 * z;
        i += incX;
        j += incY;
        n += 1;
        n;
    }
}
