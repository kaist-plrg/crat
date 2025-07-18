use ::libc;
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
pub unsafe extern "C" fn gsl_poly_eval_derivs(
    mut c: *const libc::c_double,
    lenc: size_t,
    x: libc::c_double,
    mut res: *mut libc::c_double,
    lenres: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut nmax: size_t = 0;
    let mut k: size_t = 0;
    let mut l: size_t = 0;
    let mut lmax: size_t = 0;
    i = 0 as libc::c_int as size_t;
    n = 0 as libc::c_int as size_t;
    nmax = 0 as libc::c_int as size_t;
    while i < lenres {
        if n < lenc {
            *res
                .offset(
                    i as isize,
                ) = *c
                .offset(lenc.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            nmax = n;
            n = n.wrapping_add(1);
            n;
        } else {
            *res.offset(i as isize) = 0.0f64;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < lenc.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        k = lenc.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_sub(i);
        *res
            .offset(
                0 as libc::c_int as isize,
            ) = x * *res.offset(0 as libc::c_int as isize)
            + *c.offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        lmax = if nmax < k {
            nmax
        } else {
            k.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        l = 1 as libc::c_int as size_t;
        while l <= lmax {
            *res
                .offset(
                    l as isize,
                ) = x * *res.offset(l as isize)
                + *res
                    .offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            l = l.wrapping_add(1);
            l;
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut f: libc::c_double = 1.0f64;
    i = 2 as libc::c_int as size_t;
    while i <= nmax {
        f *= i as libc::c_double;
        *res.offset(i as isize) *= f;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
