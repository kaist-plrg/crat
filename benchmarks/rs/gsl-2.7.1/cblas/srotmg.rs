use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn cblas_srotmg(
    mut d1: *mut libc::c_float,
    mut d2: *mut libc::c_float,
    mut b1: *mut libc::c_float,
    b2: libc::c_float,
    mut P: *mut libc::c_float,
) {
    let G: libc::c_float = 4096.0f64 as libc::c_float;
    let G2: libc::c_float = G * G;
    let mut D1: libc::c_float = *d1;
    let mut D2: libc::c_float = *d2;
    let mut x: libc::c_float = *b1;
    let mut y: libc::c_float = b2;
    let mut h11: libc::c_float = 0.;
    let mut h12: libc::c_float = 0.;
    let mut h21: libc::c_float = 0.;
    let mut h22: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    if (D1 as libc::c_double) < 0.0f64 {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float;
        *P.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
        *P.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
        *P.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
        *P.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
        *d1 = 0 as libc::c_int as libc::c_float;
        *d2 = 0 as libc::c_int as libc::c_float;
        *b1 = 0 as libc::c_int as libc::c_float;
        return;
    }
    if (D2 * y) as libc::c_double == 0.0f64 {
        *P.offset(0 as libc::c_int as isize) = -(2 as libc::c_int) as libc::c_float;
        return;
    }
    c = fabs((D1 * x * x) as libc::c_double) as libc::c_float;
    s = fabs((D2 * y * y) as libc::c_double) as libc::c_float;
    if c > s {
        *P.offset(0 as libc::c_int as isize) = 0.0f64 as libc::c_float;
        h11 = 1 as libc::c_int as libc::c_float;
        h12 = D2 * y / (D1 * x);
        h21 = -y / x;
        h22 = 1 as libc::c_int as libc::c_float;
        u = 1 as libc::c_int as libc::c_float - h21 * h12;
        if u as libc::c_double <= 0.0f64 {
            *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float;
            *P.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *P.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *P.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *P.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *d1 = 0 as libc::c_int as libc::c_float;
            *d2 = 0 as libc::c_int as libc::c_float;
            *b1 = 0 as libc::c_int as libc::c_float;
            return;
        }
        D1 /= u;
        D2 /= u;
        x *= u;
    } else {
        if ((D2 * y * y) as libc::c_double) < 0.0f64 {
            *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float;
            *P.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *P.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *P.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *P.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
            *d1 = 0 as libc::c_int as libc::c_float;
            *d2 = 0 as libc::c_int as libc::c_float;
            *b1 = 0 as libc::c_int as libc::c_float;
            return;
        }
        *P.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_float;
        h11 = D1 * x / (D2 * y);
        h12 = 1 as libc::c_int as libc::c_float;
        h21 = -(1 as libc::c_int) as libc::c_float;
        h22 = x / y;
        u = 1 as libc::c_int as libc::c_float + h11 * h22;
        D1 /= u;
        D2 /= u;
        let mut tmp: libc::c_float = D2;
        D2 = D1;
        D1 = tmp;
        x = y * u;
    }
    while D1 as libc::c_double <= 1.0f64 / G2 as libc::c_double
        && D1 as libc::c_double != 0.0f64
    {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float;
        D1 *= G2;
        x /= G;
        h11 /= G;
        h12 /= G;
    }
    while D1 >= G2 {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float;
        D1 /= G2;
        x *= G;
        h11 *= G;
        h12 *= G;
    }
    while fabs(D2 as libc::c_double) <= 1.0f64 / G2 as libc::c_double
        && D2 as libc::c_double != 0.0f64
    {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float;
        D2 *= G2;
        h21 /= G;
        h22 /= G;
    }
    while fabs(D2 as libc::c_double) >= G2 as libc::c_double {
        *P.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_float;
        D2 /= G2;
        h21 *= G;
        h22 *= G;
    }
    *d1 = D1;
    *d2 = D2;
    *b1 = x;
    if *P.offset(0 as libc::c_int as isize) as libc::c_double == -1.0f64 {
        *P.offset(1 as libc::c_int as isize) = h11;
        *P.offset(2 as libc::c_int as isize) = h21;
        *P.offset(3 as libc::c_int as isize) = h12;
        *P.offset(4 as libc::c_int as isize) = h22;
    } else if *P.offset(0 as libc::c_int as isize) as libc::c_double == 0.0f64 {
        *P.offset(2 as libc::c_int as isize) = h21;
        *P.offset(3 as libc::c_int as isize) = h12;
    } else if *P.offset(0 as libc::c_int as isize) as libc::c_double == 1.0f64 {
        *P.offset(1 as libc::c_int as isize) = h11;
        *P.offset(4 as libc::c_int as isize) = h22;
    }
}
