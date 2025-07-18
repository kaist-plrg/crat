use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_drotmg(
    mut d1: *mut libc::c_double,
    mut d2: *mut libc::c_double,
    mut b1: *mut libc::c_double,
    b2: libc::c_double,
    mut P: *mut libc::c_double,
) {
    let G: libc::c_double = 4096.0f64;
    let G2: libc::c_double = G * G;
    let mut D1: libc::c_double = *d1;
    let mut D2: libc::c_double = *d2;
    let mut x: libc::c_double = *b1;
    let mut y: libc::c_double = b2;
    let mut h11: libc::c_double = 0.;
    let mut h12: libc::c_double = 0.;
    let mut h21: libc::c_double = 0.;
    let mut h22: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    if D1 < 0.0f64 {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
        *P.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        *P.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        *P.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        *P.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        *d1 = 0 as libc::c_int as libc::c_double;
        *d2 = 0 as libc::c_int as libc::c_double;
        *b1 = 0 as libc::c_int as libc::c_double;
        return;
    }
    if D2 * y == 0.0f64 {
        *P.offset(0 as libc::c_int as isize) = -(2 as libc::c_int) as libc::c_double;
        return;
    }
    c = fabs(D1 * x * x);
    s = fabs(D2 * y * y);
    if c > s {
        *P.offset(0 as libc::c_int as isize) = 0.0f64;
        h11 = 1 as libc::c_int as libc::c_double;
        h12 = D2 * y / (D1 * x);
        h21 = -y / x;
        h22 = 1 as libc::c_int as libc::c_double;
        u = 1 as libc::c_int as libc::c_double - h21 * h12;
        if u <= 0.0f64 {
            *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
            *P.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *P.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *P.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *P.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *d1 = 0 as libc::c_int as libc::c_double;
            *d2 = 0 as libc::c_int as libc::c_double;
            *b1 = 0 as libc::c_int as libc::c_double;
            return;
        }
        D1 /= u;
        D2 /= u;
        x *= u;
    } else {
        if D2 * y * y < 0.0f64 {
            *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
            *P.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *P.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *P.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *P.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *d1 = 0 as libc::c_int as libc::c_double;
            *d2 = 0 as libc::c_int as libc::c_double;
            *b1 = 0 as libc::c_int as libc::c_double;
            return;
        }
        *P.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
        h11 = D1 * x / (D2 * y);
        h12 = 1 as libc::c_int as libc::c_double;
        h21 = -(1 as libc::c_int) as libc::c_double;
        h22 = x / y;
        u = 1 as libc::c_int as libc::c_double + h11 * h22;
        D1 /= u;
        D2 /= u;
        let mut tmp: libc::c_double = D2;
        D2 = D1;
        D1 = tmp;
        x = y * u;
    }
    while D1 <= 1.0f64 / G2 && D1 != 0.0f64 {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
        D1 *= G2;
        x /= G;
        h11 /= G;
        h12 /= G;
    }
    while D1 >= G2 {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
        D1 /= G2;
        x *= G;
        h11 *= G;
        h12 *= G;
    }
    while fabs(D2) <= 1.0f64 / G2 && D2 != 0.0f64 {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
        D2 *= G2;
        h21 /= G;
        h22 /= G;
    }
    while fabs(D2) >= G2 {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
        D2 /= G2;
        h21 *= G;
        h22 *= G;
    }
    *d1 = D1;
    *d2 = D2;
    *b1 = x;
    if *P.offset(0 as libc::c_int as isize) == -1.0f64 {
        *P.offset(1 as libc::c_int as isize) = h11;
        *P.offset(2 as libc::c_int as isize) = h21;
        *P.offset(3 as libc::c_int as isize) = h12;
        *P.offset(4 as libc::c_int as isize) = h22;
    } else if *P.offset(0 as libc::c_int as isize) == 0.0f64 {
        *P.offset(2 as libc::c_int as isize) = h21;
        *P.offset(3 as libc::c_int as isize) = h12;
    } else if *P.offset(0 as libc::c_int as isize) == 1.0f64 {
        *P.offset(1 as libc::c_int as isize) = h11;
        *P.offset(4 as libc::c_int as isize) = h22;
    }
}
