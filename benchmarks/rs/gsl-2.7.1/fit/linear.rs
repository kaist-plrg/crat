use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub unsafe extern "C" fn gsl_fit_linear(
    mut x: *const libc::c_double,
    xstride: size_t,
    mut y: *const libc::c_double,
    ystride: size_t,
    n: size_t,
    mut c0: *mut libc::c_double,
    mut c1: *mut libc::c_double,
    mut cov_00: *mut libc::c_double,
    mut cov_01: *mut libc::c_double,
    mut cov_11: *mut libc::c_double,
    mut sumsq: *mut libc::c_double,
) -> libc::c_int {
    let mut m_x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut m_y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut m_dx2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut m_dxdy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        m_x
            += (*x.offset(i.wrapping_mul(xstride) as isize) - m_x)
                / (i as libc::c_double + 1.0f64);
        m_y
            += (*y.offset(i.wrapping_mul(ystride) as isize) - m_y)
                / (i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let dx: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize) - m_x;
        let dy: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize) - m_y;
        m_dx2 += (dx * dx - m_dx2) / (i as libc::c_double + 1.0f64);
        m_dxdy += (dx * dy - m_dxdy) / (i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    let mut s2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut d2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = m_dxdy / m_dx2;
    let mut a: libc::c_double = m_y - m_x * b;
    *c0 = a;
    *c1 = b;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let dx_0: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize) - m_x;
        let dy_0: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize) - m_y;
        let d: libc::c_double = dy_0 - b * dx_0;
        d2 += d * d;
        i = i.wrapping_add(1);
        i;
    }
    s2 = d2 / (n as libc::c_double - 2.0f64);
    *cov_00 = s2 * (1.0f64 / n as libc::c_double)
        * (1 as libc::c_int as libc::c_double + m_x * m_x / m_dx2);
    *cov_11 = s2 * 1.0f64 / (n as libc::c_double * m_dx2);
    *cov_01 = s2 * -m_x / (n as libc::c_double * m_dx2);
    *sumsq = d2;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_fit_wlinear(
    mut x: *const libc::c_double,
    xstride: size_t,
    mut w: *const libc::c_double,
    wstride: size_t,
    mut y: *const libc::c_double,
    ystride: size_t,
    n: size_t,
    mut c0: *mut libc::c_double,
    mut c1: *mut libc::c_double,
    mut cov_00: *mut libc::c_double,
    mut cov_01: *mut libc::c_double,
    mut cov_11: *mut libc::c_double,
    mut chisq: *mut libc::c_double,
) -> libc::c_int {
    let mut W: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_dx2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_dxdy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let wi: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            W += wi;
            wm_x += (*x.offset(i.wrapping_mul(xstride) as isize) - wm_x) * (wi / W);
            wm_y += (*y.offset(i.wrapping_mul(ystride) as isize) - wm_y) * (wi / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    W = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let wi_0: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi_0 > 0 as libc::c_int as libc::c_double {
            let dx: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize) - wm_x;
            let dy: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize) - wm_y;
            W += wi_0;
            wm_dx2 += (dx * dx - wm_dx2) * (wi_0 / W);
            wm_dxdy += (dx * dy - wm_dxdy) * (wi_0 / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut d2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = wm_dxdy / wm_dx2;
    let mut a: libc::c_double = wm_y - wm_x * b;
    *c0 = a;
    *c1 = b;
    *cov_00 = 1 as libc::c_int as libc::c_double / W
        * (1 as libc::c_int as libc::c_double + wm_x * wm_x / wm_dx2);
    *cov_11 = 1 as libc::c_int as libc::c_double / (W * wm_dx2);
    *cov_01 = -wm_x / (W * wm_dx2);
    i = 0 as libc::c_int as size_t;
    while i < n {
        let wi_1: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi_1 > 0 as libc::c_int as libc::c_double {
            let dx_0: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize)
                - wm_x;
            let dy_0: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize)
                - wm_y;
            let d: libc::c_double = dy_0 - b * dx_0;
            d2 += wi_1 * d * d;
        }
        i = i.wrapping_add(1);
        i;
    }
    *chisq = d2;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_fit_linear_est(
    x: libc::c_double,
    c0: libc::c_double,
    c1: libc::c_double,
    cov00: libc::c_double,
    cov01: libc::c_double,
    cov11: libc::c_double,
    mut y: *mut libc::c_double,
    mut y_err: *mut libc::c_double,
) -> libc::c_int {
    *y = c0 + c1 * x;
    *y_err = sqrt(cov00 + x * (2 as libc::c_int as libc::c_double * cov01 + cov11 * x));
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_fit_mul(
    mut x: *const libc::c_double,
    xstride: size_t,
    mut y: *const libc::c_double,
    ystride: size_t,
    n: size_t,
    mut c1: *mut libc::c_double,
    mut cov_11: *mut libc::c_double,
    mut sumsq: *mut libc::c_double,
) -> libc::c_int {
    let mut m_x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut m_y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut m_dx2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut m_dxdy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        m_x
            += (*x.offset(i.wrapping_mul(xstride) as isize) - m_x)
                / (i as libc::c_double + 1.0f64);
        m_y
            += (*y.offset(i.wrapping_mul(ystride) as isize) - m_y)
                / (i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let dx: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize) - m_x;
        let dy: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize) - m_y;
        m_dx2 += (dx * dx - m_dx2) / (i as libc::c_double + 1.0f64);
        m_dxdy += (dx * dy - m_dxdy) / (i as libc::c_double + 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    let mut s2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut d2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = (m_x * m_y + m_dxdy) / (m_x * m_x + m_dx2);
    *c1 = b;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let dx_0: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize) - m_x;
        let dy_0: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize) - m_y;
        let d: libc::c_double = m_y - b * m_x + dy_0 - b * dx_0;
        d2 += d * d;
        i = i.wrapping_add(1);
        i;
    }
    s2 = d2 / (n as libc::c_double - 1.0f64);
    *cov_11 = s2 * 1.0f64 / (n as libc::c_double * (m_x * m_x + m_dx2));
    *sumsq = d2;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_fit_wmul(
    mut x: *const libc::c_double,
    xstride: size_t,
    mut w: *const libc::c_double,
    wstride: size_t,
    mut y: *const libc::c_double,
    ystride: size_t,
    n: size_t,
    mut c1: *mut libc::c_double,
    mut cov_11: *mut libc::c_double,
    mut chisq: *mut libc::c_double,
) -> libc::c_int {
    let mut W: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_dx2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wm_dxdy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let wi: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi > 0 as libc::c_int as libc::c_double {
            W += wi;
            wm_x += (*x.offset(i.wrapping_mul(xstride) as isize) - wm_x) * (wi / W);
            wm_y += (*y.offset(i.wrapping_mul(ystride) as isize) - wm_y) * (wi / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    W = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let wi_0: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi_0 > 0 as libc::c_int as libc::c_double {
            let dx: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize) - wm_x;
            let dy: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize) - wm_y;
            W += wi_0;
            wm_dx2 += (dx * dx - wm_dx2) * (wi_0 / W);
            wm_dxdy += (dx * dy - wm_dxdy) * (wi_0 / W);
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut d2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = (wm_x * wm_y + wm_dxdy) / (wm_x * wm_x + wm_dx2);
    *c1 = b;
    *cov_11 = 1 as libc::c_int as libc::c_double / (W * (wm_x * wm_x + wm_dx2));
    i = 0 as libc::c_int as size_t;
    while i < n {
        let wi_1: libc::c_double = *w.offset(i.wrapping_mul(wstride) as isize);
        if wi_1 > 0 as libc::c_int as libc::c_double {
            let dx_0: libc::c_double = *x.offset(i.wrapping_mul(xstride) as isize)
                - wm_x;
            let dy_0: libc::c_double = *y.offset(i.wrapping_mul(ystride) as isize)
                - wm_y;
            let d: libc::c_double = wm_y - b * wm_x + (dy_0 - b * dx_0);
            d2 += wi_1 * d * d;
        }
        i = i.wrapping_add(1);
        i;
    }
    *chisq = d2;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_fit_mul_est(
    x: libc::c_double,
    c1: libc::c_double,
    cov11: libc::c_double,
    mut y: *mut libc::c_double,
    mut y_err: *mut libc::c_double,
) -> libc::c_int {
    *y = c1 * x;
    *y_err = sqrt(cov11) * fabs(x);
    return GSL_SUCCESS as libc::c_int;
}
