use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
pub type gsl_sf_legendre_t = libc::c_uint;
pub const GSL_SF_LEGENDRE_NONE: gsl_sf_legendre_t = 3;
pub const GSL_SF_LEGENDRE_FULL: gsl_sf_legendre_t = 2;
pub const GSL_SF_LEGENDRE_SPHARM: gsl_sf_legendre_t = 1;
pub const GSL_SF_LEGENDRE_SCHMIDT: gsl_sf_legendre_t = 0;
#[inline]
unsafe extern "C" fn gsl_sf_legendre_array_index(l: size_t, m: size_t) -> size_t {
    return (l.wrapping_mul(l.wrapping_add(1 as libc::c_int as libc::c_ulong))
        >> 1 as libc::c_int)
        .wrapping_add(m);
}
pub unsafe extern "C" fn gsl_sf_legendre_array(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = gsl_sf_legendre_array_e(
        norm,
        lmax,
        x,
        1.0f64,
        result_array,
    );
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv2_alt_array(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = gsl_sf_legendre_deriv2_alt_array_e(
        norm,
        lmax,
        x,
        1.0f64,
        result_array,
        result_deriv_array,
        result_deriv2_array,
    );
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv_array(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = gsl_sf_legendre_deriv_array_e(
        norm,
        lmax,
        x,
        1.0f64,
        result_array,
        result_deriv_array,
    );
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv2_array(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = gsl_sf_legendre_deriv2_array_e(
        norm,
        lmax,
        x,
        1.0f64,
        result_array,
        result_deriv_array,
        result_deriv2_array,
    );
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv_alt_array(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = gsl_sf_legendre_deriv_alt_array_e(
        norm,
        lmax,
        x,
        1.0f64,
        result_array,
        result_deriv_array,
    );
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv2_alt_array_e(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let nlm: size_t = gsl_sf_legendre_nlm(lmax);
    let mut fac1: libc::c_double = 0.0f64;
    let mut fac2: libc::c_double = 0.0f64;
    if norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint {
        s = legendre_deriv2_alt_array_none_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
            result_deriv2_array,
        );
    } else {
        s = legendre_deriv2_alt_array_schmidt_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
            result_deriv2_array,
        );
    }
    if norm as libc::c_uint == GSL_SF_LEGENDRE_SCHMIDT as libc::c_int as libc::c_uint
        || norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint
    {
        return s
    } else if norm as libc::c_uint
        == GSL_SF_LEGENDRE_SPHARM as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(4.0f64 * 3.14159265358979323846f64);
        fac2 = 1.0f64 / sqrt(8.0f64 * 3.14159265358979323846f64);
    } else if norm as libc::c_uint == GSL_SF_LEGENDRE_FULL as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(2.0f64);
        fac2 = 1.0f64 / sqrt(4.0f64);
    }
    let mut l: size_t = 0;
    let mut m: size_t = 0;
    let mut twoellp1: size_t = 1 as libc::c_int as size_t;
    let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
        as *mut libc::c_double;
    l = 0 as libc::c_int as size_t;
    while l <= lmax {
        *result_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        *result_deriv_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        *result_deriv2_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        m = 1 as libc::c_int as size_t;
        while m <= l {
            *result_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            *result_deriv_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            *result_deriv2_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            m = m.wrapping_add(1);
            m;
        }
        twoellp1 = (twoellp1 as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        l = l.wrapping_add(1);
        l;
    }
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv_array_e(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let nlm: size_t = gsl_sf_legendre_nlm(lmax);
    let mut i: size_t = 0;
    let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
    let uinv: libc::c_double = 1.0f64 / u;
    let mut fac1: libc::c_double = 0.0f64;
    let mut fac2: libc::c_double = 0.0f64;
    if norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint {
        s = legendre_deriv_array_none_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
        );
    } else {
        s = legendre_deriv_array_schmidt_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
        );
    }
    i = 0 as libc::c_int as size_t;
    while i < nlm {
        *result_deriv_array.offset(i as isize) *= -uinv;
        i = i.wrapping_add(1);
        i;
    }
    if norm as libc::c_uint == GSL_SF_LEGENDRE_SCHMIDT as libc::c_int as libc::c_uint
        || norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint
    {
        return s
    } else if norm as libc::c_uint
        == GSL_SF_LEGENDRE_SPHARM as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(4.0f64 * 3.14159265358979323846f64);
        fac2 = 1.0f64 / sqrt(8.0f64 * 3.14159265358979323846f64);
    } else if norm as libc::c_uint == GSL_SF_LEGENDRE_FULL as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(2.0f64);
        fac2 = 1.0f64 / sqrt(4.0f64);
    }
    let mut l: size_t = 0;
    let mut m: size_t = 0;
    let mut twoellp1: size_t = 1 as libc::c_int as size_t;
    let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
        as *mut libc::c_double;
    l = 0 as libc::c_int as size_t;
    while l <= lmax {
        *result_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        *result_deriv_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        m = 1 as libc::c_int as size_t;
        while m <= l {
            *result_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            *result_deriv_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            m = m.wrapping_add(1);
            m;
        }
        twoellp1 = (twoellp1 as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        l = l.wrapping_add(1);
        l;
    }
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv2_array_e(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let nlm: size_t = gsl_sf_legendre_nlm(lmax);
    let mut i: size_t = 0;
    let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
    let uinv: libc::c_double = 1.0f64 / u;
    let uinv2: libc::c_double = uinv * uinv;
    let mut fac1: libc::c_double = 0.0f64;
    let mut fac2: libc::c_double = 0.0f64;
    if norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint {
        s = legendre_deriv2_array_none_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
            result_deriv2_array,
        );
    } else {
        s = legendre_deriv2_array_schmidt_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
            result_deriv2_array,
        );
    }
    i = 0 as libc::c_int as size_t;
    while i < nlm {
        let mut dp: libc::c_double = *result_deriv_array.offset(i as isize);
        let mut d2p: libc::c_double = *result_deriv2_array.offset(i as isize);
        *result_deriv2_array.offset(i as isize) = (d2p - x * uinv * dp) * uinv2;
        *result_deriv_array.offset(i as isize) *= -uinv;
        i = i.wrapping_add(1);
        i;
    }
    if norm as libc::c_uint == GSL_SF_LEGENDRE_SCHMIDT as libc::c_int as libc::c_uint
        || norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint
    {
        return s
    } else if norm as libc::c_uint
        == GSL_SF_LEGENDRE_SPHARM as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(4.0f64 * 3.14159265358979323846f64);
        fac2 = 1.0f64 / sqrt(8.0f64 * 3.14159265358979323846f64);
    } else if norm as libc::c_uint == GSL_SF_LEGENDRE_FULL as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(2.0f64);
        fac2 = 1.0f64 / sqrt(4.0f64);
    }
    let mut l: size_t = 0;
    let mut m: size_t = 0;
    let mut twoellp1: size_t = 1 as libc::c_int as size_t;
    let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
        as *mut libc::c_double;
    l = 0 as libc::c_int as size_t;
    while l <= lmax {
        *result_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        *result_deriv_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        *result_deriv2_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        m = 1 as libc::c_int as size_t;
        while m <= l {
            *result_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            *result_deriv_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            *result_deriv2_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            m = m.wrapping_add(1);
            m;
        }
        twoellp1 = (twoellp1 as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        l = l.wrapping_add(1);
        l;
    }
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_deriv_alt_array_e(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let nlm: size_t = gsl_sf_legendre_nlm(lmax);
    let mut fac1: libc::c_double = 0.0f64;
    let mut fac2: libc::c_double = 0.0f64;
    if norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint {
        s = legendre_deriv_alt_array_none_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
        );
    } else {
        s = legendre_deriv_alt_array_schmidt_e(
            lmax,
            x,
            csphase,
            result_array,
            result_deriv_array,
        );
    }
    if norm as libc::c_uint == GSL_SF_LEGENDRE_SCHMIDT as libc::c_int as libc::c_uint
        || norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint
    {
        return s
    } else if norm as libc::c_uint
        == GSL_SF_LEGENDRE_SPHARM as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(4.0f64 * 3.14159265358979323846f64);
        fac2 = 1.0f64 / sqrt(8.0f64 * 3.14159265358979323846f64);
    } else if norm as libc::c_uint == GSL_SF_LEGENDRE_FULL as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(2.0f64);
        fac2 = 1.0f64 / sqrt(4.0f64);
    }
    let mut l: size_t = 0;
    let mut m: size_t = 0;
    let mut twoellp1: size_t = 1 as libc::c_int as size_t;
    let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
        as *mut libc::c_double;
    l = 0 as libc::c_int as size_t;
    while l <= lmax {
        *result_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        *result_deriv_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        m = 1 as libc::c_int as size_t;
        while m <= l {
            *result_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            *result_deriv_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            m = m.wrapping_add(1);
            m;
        }
        twoellp1 = (twoellp1 as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        l = l.wrapping_add(1);
        l;
    }
    return s;
}
pub unsafe extern "C" fn gsl_sf_legendre_array_e(
    norm: gsl_sf_legendre_t,
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let nlm: size_t = gsl_sf_legendre_nlm(lmax);
    let mut i: size_t = 0;
    let mut fac1: libc::c_double = 0.0f64;
    let mut fac2: libc::c_double = 0.0f64;
    if norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint {
        s = legendre_array_none_e(lmax, x, csphase, result_array);
    } else {
        s = legendre_array_schmidt_e(lmax, x, csphase, result_array);
    }
    i = 0 as libc::c_int as size_t;
    while i < nlm {
        i = i.wrapping_add(1);
        i;
    }
    if norm as libc::c_uint == GSL_SF_LEGENDRE_SCHMIDT as libc::c_int as libc::c_uint
        || norm as libc::c_uint == GSL_SF_LEGENDRE_NONE as libc::c_int as libc::c_uint
    {
        return s
    } else if norm as libc::c_uint
        == GSL_SF_LEGENDRE_SPHARM as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(4.0f64 * 3.14159265358979323846f64);
        fac2 = 1.0f64 / sqrt(8.0f64 * 3.14159265358979323846f64);
    } else if norm as libc::c_uint == GSL_SF_LEGENDRE_FULL as libc::c_int as libc::c_uint
    {
        fac1 = 1.0f64 / sqrt(2.0f64);
        fac2 = 1.0f64 / sqrt(4.0f64);
    }
    let mut l: size_t = 0;
    let mut m: size_t = 0;
    let mut twoellp1: size_t = 1 as libc::c_int as size_t;
    let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
        as *mut libc::c_double;
    l = 0 as libc::c_int as size_t;
    while l <= lmax {
        *result_array
            .offset(gsl_sf_legendre_array_index(l, 0 as libc::c_int as size_t) as isize)
            *= *sqrts.offset(twoellp1 as isize) * fac1;
        m = 1 as libc::c_int as size_t;
        while m <= l {
            *result_array.offset(gsl_sf_legendre_array_index(l, m) as isize)
                *= *sqrts.offset(twoellp1 as isize) * fac2;
            m = m.wrapping_add(1);
            m;
        }
        twoellp1 = (twoellp1 as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        l = l.wrapping_add(1);
        l;
    }
    return s;
}
unsafe extern "C" fn legendre_deriv2_array_schmidt_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let eps: libc::c_double = 1.0e-280f64;
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let uinv2: libc::c_double = 1.0f64 / u / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut rescalem: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut nlm: size_t = gsl_sf_legendre_nlm(lmax);
        let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
            as *mut libc::c_double;
        legendre_sqrts(lmax, sqrts);
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        *result_deriv2_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        *result_deriv2_array.offset(1 as libc::c_int as isize) = -x;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            let mut linv: libc::c_double = 1.0f64 / l as libc::c_double;
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = (2.0f64 - linv) * x * pm1 - (1.0f64 - linv) * pm2;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = uinv * l as libc::c_double * (x * plm - pm1);
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = -(l as libc::c_double) * (l as libc::c_double + 1.0f64) * plm
                - xbyu * *result_deriv_array.offset(k as isize);
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = sqrt(2.0f64) * eps;
        rescalem = 1.0f64 / eps;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m < lmax {
            rescalem *= u;
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            pmm
                *= csphase
                    * *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                    / *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(m) as isize,
                        );
            *result_array.offset(idxmm as isize) = pmm * rescalem;
            *result_deriv_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
            *result_deriv2_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double
                * (uinv2 * m as libc::c_double - (m as libc::c_double + 1.0f64))
                * *result_array.offset(idxmm as isize)
                - xbyu * *result_deriv_array.offset(idxmm as isize);
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(m)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) * pm2;
            *result_array.offset(k as isize) = pm1 * rescalem;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = uinv
                * ((m as libc::c_double + 1.0f64) * x * *result_array.offset(k as isize)
                    - *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) * *result_array.offset(idxmm as isize));
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                - (m as libc::c_double + 1.0f64) * (m as libc::c_double + 2.0f64))
                * *result_array.offset(k as isize)
                - xbyu * *result_deriv_array.offset(k as isize);
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    / *sqrts.offset(l.wrapping_add(m) as isize)
                    / *sqrts.offset(l.wrapping_sub(m) as isize) * x * pm1
                    - *sqrts
                        .offset(
                            l
                                .wrapping_sub(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                        * *sqrts
                            .offset(
                                l
                                    .wrapping_add(m)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) / *sqrts.offset(l.wrapping_add(m) as isize)
                        / *sqrts.offset(l.wrapping_sub(m) as isize) * pm2;
                *result_array.offset(k as isize) = plm * rescalem;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = uinv
                    * (l as libc::c_double * x * *result_array.offset(k as isize)
                        - *sqrts.offset(l.wrapping_add(m) as isize)
                            * *sqrts.offset(l.wrapping_sub(m) as isize)
                            * *result_array.offset(k.wrapping_sub(l) as isize));
                *result_deriv2_array
                    .offset(
                        k as isize,
                    ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                    - l as libc::c_double * (l as libc::c_double + 1.0f64))
                    * *result_array.offset(k as isize)
                    - xbyu * *result_deriv_array.offset(k as isize);
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        rescalem *= u;
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        pmm
            *= csphase
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(lmax)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                / *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(lmax) as isize,
                    );
        *result_array.offset(idxmm as isize) = pmm * rescalem;
        *result_deriv_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
        *result_deriv2_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double
            * (uinv2 * lmax as libc::c_double - (lmax as libc::c_double + 1.0f64))
            * *result_array.offset(idxmm as isize)
            - xbyu * *result_deriv_array.offset(idxmm as isize);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_array_schmidt_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let eps: libc::c_double = 1.0e-280f64;
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut rescalem: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut nlm: size_t = gsl_sf_legendre_nlm(lmax);
        let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
            as *mut libc::c_double;
        legendre_sqrts(lmax, sqrts);
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            let mut linv: libc::c_double = 1.0f64 / l as libc::c_double;
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = (2.0f64 - linv) * x * pm1 - (1.0f64 - linv) * pm2;
            *result_array.offset(k as isize) = plm;
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = sqrt(2.0f64) * eps;
        rescalem = 1.0f64 / eps;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m < lmax {
            rescalem *= u;
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            pmm
                *= csphase
                    * *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                    / *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(m) as isize,
                        );
            *result_array.offset(idxmm as isize) = pmm * rescalem;
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(m)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) * pm2;
            *result_array.offset(k as isize) = pm1 * rescalem;
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    / *sqrts.offset(l.wrapping_add(m) as isize)
                    / *sqrts.offset(l.wrapping_sub(m) as isize) * x * pm1
                    - *sqrts
                        .offset(
                            l
                                .wrapping_sub(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                        * *sqrts
                            .offset(
                                l
                                    .wrapping_add(m)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) / *sqrts.offset(l.wrapping_add(m) as isize)
                        / *sqrts.offset(l.wrapping_sub(m) as isize) * pm2;
                *result_array.offset(k as isize) = plm * rescalem;
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        rescalem *= u;
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        pmm
            *= csphase
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(lmax)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                / *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(lmax) as isize,
                    );
        *result_array.offset(idxmm as isize) = pmm * rescalem;
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_deriv_array_schmidt_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let eps: libc::c_double = 1.0e-280f64;
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut rescalem: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut nlm: size_t = gsl_sf_legendre_nlm(lmax);
        let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
            as *mut libc::c_double;
        legendre_sqrts(lmax, sqrts);
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            let mut linv: libc::c_double = 1.0f64 / l as libc::c_double;
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = (2.0f64 - linv) * x * pm1 - (1.0f64 - linv) * pm2;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = uinv * l as libc::c_double * (x * plm - pm1);
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = sqrt(2.0f64) * eps;
        rescalem = 1.0f64 / eps;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m < lmax {
            rescalem *= u;
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            pmm
                *= csphase
                    * *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                    / *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(m) as isize,
                        );
            *result_array.offset(idxmm as isize) = pmm * rescalem;
            *result_deriv_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(m)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) * pm2;
            *result_array.offset(k as isize) = pm1 * rescalem;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = uinv
                * ((m as libc::c_double + 1.0f64) * x * *result_array.offset(k as isize)
                    - *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) * *result_array.offset(idxmm as isize));
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    / *sqrts.offset(l.wrapping_add(m) as isize)
                    / *sqrts.offset(l.wrapping_sub(m) as isize) * x * pm1
                    - *sqrts
                        .offset(
                            l
                                .wrapping_sub(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                        * *sqrts
                            .offset(
                                l
                                    .wrapping_add(m)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) / *sqrts.offset(l.wrapping_add(m) as isize)
                        / *sqrts.offset(l.wrapping_sub(m) as isize) * pm2;
                *result_array.offset(k as isize) = plm * rescalem;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = uinv
                    * (l as libc::c_double * x * *result_array.offset(k as isize)
                        - *sqrts.offset(l.wrapping_add(m) as isize)
                            * *sqrts.offset(l.wrapping_sub(m) as isize)
                            * *result_array.offset(k.wrapping_sub(l) as isize));
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        rescalem *= u;
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        pmm
            *= csphase
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(lmax)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                / *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(lmax) as isize,
                    );
        *result_array.offset(idxmm as isize) = pmm * rescalem;
        *result_deriv_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_deriv2_alt_array_schmidt_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let eps: libc::c_double = 1.0e-280f64;
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let uinv2: libc::c_double = 1.0f64 / u / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut rescalem: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut nlm: size_t = gsl_sf_legendre_nlm(lmax);
        let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
            as *mut libc::c_double;
        legendre_sqrts(lmax, sqrts);
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        *result_deriv2_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        *result_deriv2_array.offset(1 as libc::c_int as isize) = -x;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            let mut linv: libc::c_double = 1.0f64 / l as libc::c_double;
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = (2.0f64 - linv) * x * pm1 - (1.0f64 - linv) * pm2;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = uinv * l as libc::c_double * (x * plm - pm1);
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = -(l as libc::c_double) * (l as libc::c_double + 1.0f64) * plm
                - xbyu * *result_deriv_array.offset(k as isize);
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = sqrt(2.0f64) * eps;
        rescalem = 1.0f64 / eps;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m < lmax {
            rescalem *= u;
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            pmm
                *= csphase
                    * *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                    / *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(m) as isize,
                        );
            *result_array.offset(idxmm as isize) = pmm * rescalem;
            *result_deriv_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
            *result_deriv2_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double
                * (uinv2 * m as libc::c_double - (m as libc::c_double + 1.0f64))
                * *result_array.offset(idxmm as isize)
                - xbyu * *result_deriv_array.offset(idxmm as isize);
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(m)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) * pm2;
            *result_array.offset(k as isize) = pm1 * rescalem;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = uinv
                * ((m as libc::c_double + 1.0f64) * x * *result_array.offset(k as isize)
                    - *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) * *result_array.offset(idxmm as isize));
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                - (m as libc::c_double + 1.0f64) * (m as libc::c_double + 2.0f64))
                * *result_array.offset(k as isize)
                - xbyu * *result_deriv_array.offset(k as isize);
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    / *sqrts.offset(l.wrapping_add(m) as isize)
                    / *sqrts.offset(l.wrapping_sub(m) as isize) * x * pm1
                    - *sqrts
                        .offset(
                            l
                                .wrapping_sub(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                        * *sqrts
                            .offset(
                                l
                                    .wrapping_add(m)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) / *sqrts.offset(l.wrapping_add(m) as isize)
                        / *sqrts.offset(l.wrapping_sub(m) as isize) * pm2;
                *result_array.offset(k as isize) = plm * rescalem;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = uinv
                    * (l as libc::c_double * x * *result_array.offset(k as isize)
                        - *sqrts.offset(l.wrapping_add(m) as isize)
                            * *sqrts.offset(l.wrapping_sub(m) as isize)
                            * *result_array.offset(k.wrapping_sub(l) as isize));
                *result_deriv2_array
                    .offset(
                        k as isize,
                    ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                    - l as libc::c_double * (l as libc::c_double + 1.0f64))
                    * *result_array.offset(k as isize)
                    - xbyu * *result_deriv_array.offset(k as isize);
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        rescalem *= u;
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        pmm
            *= csphase
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(lmax)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                / *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(lmax) as isize,
                    );
        *result_array.offset(idxmm as isize) = pmm * rescalem;
        *result_deriv_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
        *result_deriv2_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double
            * (uinv2 * lmax as libc::c_double - (lmax as libc::c_double + 1.0f64))
            * *result_array.offset(idxmm as isize)
            - xbyu * *result_deriv_array.offset(idxmm as isize);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_deriv_alt_array_schmidt_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let eps: libc::c_double = 1.0e-280f64;
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut rescalem: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut nlm: size_t = gsl_sf_legendre_nlm(lmax);
        let mut sqrts: *mut libc::c_double = &mut *result_array.offset(nlm as isize)
            as *mut libc::c_double;
        legendre_sqrts(lmax, sqrts);
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            let mut linv: libc::c_double = 1.0f64 / l as libc::c_double;
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = (2.0f64 - linv) * x * pm1 - (1.0f64 - linv) * pm2;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = uinv * l as libc::c_double * (x * plm - pm1);
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = sqrt(2.0f64) * eps;
        rescalem = 1.0f64 / eps;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m < lmax {
            rescalem *= u;
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            pmm
                *= csphase
                    * *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                    / *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(m) as isize,
                        );
            *result_array.offset(idxmm as isize) = pmm * rescalem;
            *result_deriv_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(m)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) * pm2;
            *result_array.offset(k as isize) = pm1 * rescalem;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = uinv
                * ((m as libc::c_double + 1.0f64) * x * *result_array.offset(k as isize)
                    - *sqrts
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(m)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) * *result_array.offset(idxmm as isize));
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    / *sqrts.offset(l.wrapping_add(m) as isize)
                    / *sqrts.offset(l.wrapping_sub(m) as isize) * x * pm1
                    - *sqrts
                        .offset(
                            l
                                .wrapping_sub(m)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                        * *sqrts
                            .offset(
                                l
                                    .wrapping_add(m)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) / *sqrts.offset(l.wrapping_add(m) as isize)
                        / *sqrts.offset(l.wrapping_sub(m) as isize) * pm2;
                *result_array.offset(k as isize) = plm * rescalem;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = uinv
                    * (l as libc::c_double * x * *result_array.offset(k as isize)
                        - *sqrts.offset(l.wrapping_add(m) as isize)
                            * *sqrts.offset(l.wrapping_sub(m) as isize)
                            * *result_array.offset(k.wrapping_sub(l) as isize));
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        rescalem *= u;
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        pmm
            *= csphase
                * *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(lmax)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                / *sqrts
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(lmax) as isize,
                    );
        *result_array.offset(idxmm as isize) = pmm * rescalem;
        *result_deriv_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double * xbyu * *result_array.offset(idxmm as isize);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_array_none_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut twomm1: libc::c_double = 0.;
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = ((2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(l)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * x
                * pm1
                - l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pm2) / l as libc::c_double;
            *result_array.offset(k as isize) = plm;
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = 1.0f64;
        twomm1 = -1.0f64;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m <= lmax.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            twomm1 += 2.0f64;
            pmm *= csphase * u * twomm1;
            *result_array.offset(idxmm as isize) = pmm;
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x * pmm
                * (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
            *result_array.offset(k as isize) = pm1;
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * x * pm1
                    - l.wrapping_add(m).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_double * pm2) / l.wrapping_sub(m) as libc::c_double;
                *result_array.offset(k as isize) = plm;
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        twomm1 += 2.0f64;
        pmm *= csphase * u * twomm1;
        *result_array.offset(idxmm as isize) = pmm;
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_deriv2_alt_array_none_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            435 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let uinv2: libc::c_double = 1.0f64 / u / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut twomm1: libc::c_double = 0.;
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        *result_deriv2_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        *result_deriv2_array.offset(1 as libc::c_int as isize) = -x;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = ((2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(l)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * x
                * pm1
                - l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pm2) / l as libc::c_double;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = -(l as libc::c_double) * (pm1 - x * plm) * uinv;
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = -(l as libc::c_double) * (l as libc::c_double + 1.0f64) * plm
                - xbyu * *result_deriv_array.offset(k as isize);
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = 1.0f64;
        twomm1 = -1.0f64;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m <= lmax.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            twomm1 += 2.0f64;
            pmm *= csphase * u * twomm1;
            *result_array.offset(idxmm as isize) = pmm;
            *result_deriv_array
                .offset(idxmm as isize) = m as libc::c_double * xbyu * pmm;
            *result_deriv2_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double
                * (uinv2 * m as libc::c_double - (m as libc::c_double + 1.0f64))
                * *result_array.offset(idxmm as isize)
                - xbyu * *result_deriv_array.offset(idxmm as isize);
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x * pmm
                * (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
            *result_array.offset(k as isize) = pm1;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = -uinv
                * ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pmm
                    - m.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                        * x * pm1);
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                - (m as libc::c_double + 1.0f64) * (m as libc::c_double + 2.0f64))
                * *result_array.offset(k as isize)
                - xbyu * *result_deriv_array.offset(k as isize);
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * x * pm1
                    - l.wrapping_add(m).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_double * pm2) / l.wrapping_sub(m) as libc::c_double;
                *result_array.offset(k as isize) = plm;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = -uinv
                    * (l.wrapping_add(m) as libc::c_double * pm1
                        - l as libc::c_double * x * plm);
                *result_deriv2_array
                    .offset(
                        k as isize,
                    ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                    - l as libc::c_double * (l as libc::c_double + 1.0f64))
                    * *result_array.offset(k as isize)
                    - xbyu * *result_deriv_array.offset(k as isize);
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        twomm1 += 2.0f64;
        pmm *= csphase * u * twomm1;
        *result_array.offset(idxmm as isize) = pmm;
        *result_deriv_array
            .offset(idxmm as isize) = lmax as libc::c_double * x * pmm * uinv;
        *result_deriv2_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double
            * (uinv2 * lmax as libc::c_double - (lmax as libc::c_double + 1.0f64))
            * *result_array.offset(idxmm as isize)
            - xbyu * *result_deriv_array.offset(idxmm as isize);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_deriv2_array_none_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
    mut result_deriv2_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            435 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let uinv2: libc::c_double = 1.0f64 / u / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut twomm1: libc::c_double = 0.;
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        *result_deriv2_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        *result_deriv2_array.offset(1 as libc::c_int as isize) = -x;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = ((2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(l)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * x
                * pm1
                - l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pm2) / l as libc::c_double;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = -(l as libc::c_double) * (pm1 - x * plm) * uinv;
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = -(l as libc::c_double) * (l as libc::c_double + 1.0f64) * plm
                - xbyu * *result_deriv_array.offset(k as isize);
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = 1.0f64;
        twomm1 = -1.0f64;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m <= lmax.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            twomm1 += 2.0f64;
            pmm *= csphase * u * twomm1;
            *result_array.offset(idxmm as isize) = pmm;
            *result_deriv_array
                .offset(idxmm as isize) = m as libc::c_double * xbyu * pmm;
            *result_deriv2_array
                .offset(
                    idxmm as isize,
                ) = m as libc::c_double
                * (uinv2 * m as libc::c_double - (m as libc::c_double + 1.0f64))
                * *result_array.offset(idxmm as isize)
                - xbyu * *result_deriv_array.offset(idxmm as isize);
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x * pmm
                * (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
            *result_array.offset(k as isize) = pm1;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = -uinv
                * ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pmm
                    - m.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                        * x * pm1);
            *result_deriv2_array
                .offset(
                    k as isize,
                ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                - (m as libc::c_double + 1.0f64) * (m as libc::c_double + 2.0f64))
                * *result_array.offset(k as isize)
                - xbyu * *result_deriv_array.offset(k as isize);
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * x * pm1
                    - l.wrapping_add(m).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_double * pm2) / l.wrapping_sub(m) as libc::c_double;
                *result_array.offset(k as isize) = plm;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = -uinv
                    * (l.wrapping_add(m) as libc::c_double * pm1
                        - l as libc::c_double * x * plm);
                *result_deriv2_array
                    .offset(
                        k as isize,
                    ) = (m.wrapping_mul(m) as libc::c_double * uinv2
                    - l as libc::c_double * (l as libc::c_double + 1.0f64))
                    * *result_array.offset(k as isize)
                    - xbyu * *result_deriv_array.offset(k as isize);
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        twomm1 += 2.0f64;
        pmm *= csphase * u * twomm1;
        *result_array.offset(idxmm as isize) = pmm;
        *result_deriv_array
            .offset(idxmm as isize) = lmax as libc::c_double * x * pmm * uinv;
        *result_deriv2_array
            .offset(
                idxmm as isize,
            ) = lmax as libc::c_double
            * (uinv2 * lmax as libc::c_double - (lmax as libc::c_double + 1.0f64))
            * *result_array.offset(idxmm as isize)
            - xbyu * *result_deriv_array.offset(idxmm as isize);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_deriv_alt_array_none_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            435 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut twomm1: libc::c_double = 0.;
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = ((2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(l)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * x
                * pm1
                - l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pm2) / l as libc::c_double;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = -(l as libc::c_double) * (pm1 - x * plm) * uinv;
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = 1.0f64;
        twomm1 = -1.0f64;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m <= lmax.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            twomm1 += 2.0f64;
            pmm *= csphase * u * twomm1;
            *result_array.offset(idxmm as isize) = pmm;
            *result_deriv_array
                .offset(idxmm as isize) = m as libc::c_double * xbyu * pmm;
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x * pmm
                * (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
            *result_array.offset(k as isize) = pm1;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = -uinv
                * ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pmm
                    - m.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                        * x * pm1);
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * x * pm1
                    - l.wrapping_add(m).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_double * pm2) / l.wrapping_sub(m) as libc::c_double;
                *result_array.offset(k as isize) = plm;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = -uinv
                    * (l.wrapping_add(m) as libc::c_double * pm1
                        - l as libc::c_double * x * plm);
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        twomm1 += 2.0f64;
        pmm *= csphase * u * twomm1;
        *result_array.offset(idxmm as isize) = pmm;
        *result_deriv_array
            .offset(idxmm as isize) = lmax as libc::c_double * x * pmm * uinv;
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn legendre_deriv_array_none_e(
    lmax: size_t,
    x: libc::c_double,
    csphase: libc::c_double,
    mut result_array: *mut libc::c_double,
    mut result_deriv_array: *mut libc::c_double,
) -> libc::c_int {
    if x > 1.0f64 || x < -1.0f64 {
        gsl_error(
            b"x is outside [-1,1]\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) == 1.0f64 {
        gsl_error(
            b"x cannot equal 1 or -1 for derivative computation\0" as *const u8
                as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            435 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if csphase != 1.0f64 && csphase != -1.0f64 {
        gsl_error(
            b"csphase has invalid value\0" as *const u8 as *const libc::c_char,
            b"./legendre_source.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let u: libc::c_double = sqrt((1.0f64 - x) * (1.0f64 + x));
        let uinv: libc::c_double = 1.0f64 / u;
        let xbyu: libc::c_double = x * uinv;
        let mut l: size_t = 0;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut idxmm: size_t = 0;
        let mut plm: libc::c_double = 0.;
        let mut pmm: libc::c_double = 0.;
        let mut pm1: libc::c_double = 0.;
        let mut pm2: libc::c_double = 0.;
        let mut twomm1: libc::c_double = 0.;
        pm2 = 1.0f64;
        pm1 = x;
        *result_array.offset(0 as libc::c_int as isize) = pm2;
        *result_deriv_array.offset(0 as libc::c_int as isize) = 0.0f64;
        if lmax == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        *result_array.offset(1 as libc::c_int as isize) = pm1;
        *result_deriv_array.offset(1 as libc::c_int as isize) = -u;
        k = 1 as libc::c_int as size_t;
        l = 2 as libc::c_int as size_t;
        while l <= lmax {
            k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            plm = ((2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(l)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double * x
                * pm1
                - l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pm2) / l as libc::c_double;
            *result_array.offset(k as isize) = plm;
            *result_deriv_array
                .offset(k as isize) = -(l as libc::c_double) * (pm1 - x * plm) * uinv;
            pm2 = pm1;
            pm1 = plm;
            l = l.wrapping_add(1);
            l;
        }
        pmm = 1.0f64;
        twomm1 = -1.0f64;
        idxmm = 0 as libc::c_int as size_t;
        m = 1 as libc::c_int as size_t;
        while m <= lmax.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            idxmm = (idxmm as libc::c_ulong)
                .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            twomm1 += 2.0f64;
            pmm *= csphase * u * twomm1;
            *result_array.offset(idxmm as isize) = pmm;
            *result_deriv_array
                .offset(idxmm as isize) = m as libc::c_double * xbyu * pmm;
            pm2 = pmm;
            k = idxmm.wrapping_add(m).wrapping_add(1 as libc::c_int as libc::c_ulong);
            pm1 = x * pmm
                * (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
            *result_array.offset(k as isize) = pm1;
            *result_deriv_array
                .offset(
                    k as isize,
                ) = -uinv
                * ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(m)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * pmm
                    - m.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                        * x * pm1);
            l = m.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while l <= lmax {
                k = (k as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                plm = ((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(l)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * x * pm1
                    - l.wrapping_add(m).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as libc::c_double * pm2) / l.wrapping_sub(m) as libc::c_double;
                *result_array.offset(k as isize) = plm;
                *result_deriv_array
                    .offset(
                        k as isize,
                    ) = -uinv
                    * (l.wrapping_add(m) as libc::c_double * pm1
                        - l as libc::c_double * x * plm);
                pm2 = pm1;
                pm1 = plm;
                l = l.wrapping_add(1);
                l;
            }
            m = m.wrapping_add(1);
            m;
        }
        idxmm = (idxmm as libc::c_ulong)
            .wrapping_add(m.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        twomm1 += 2.0f64;
        pmm *= csphase * u * twomm1;
        *result_array.offset(idxmm as isize) = pmm;
        *result_deriv_array
            .offset(idxmm as isize) = lmax as libc::c_double * x * pmm * uinv;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_legendre_nlm(lmax: size_t) -> size_t {
    return lmax
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(lmax.wrapping_add(2 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn gsl_sf_legendre_array_n(lmax: size_t) -> size_t {
    let mut nlm: size_t = gsl_sf_legendre_nlm(lmax);
    let mut nsqrt: size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(lmax)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    return nlm.wrapping_add(nsqrt);
}
unsafe extern "C" fn legendre_sqrts(lmax: size_t, mut array: *mut libc::c_double) {
    let mut l: size_t = 0;
    l = 0 as libc::c_int as size_t;
    while l
        <= (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(lmax)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        *array.offset(l as isize) = sqrt(l as libc::c_double);
        l = l.wrapping_add(1);
        l;
    }
}
