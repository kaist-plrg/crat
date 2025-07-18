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
pub unsafe extern "C" fn gsl_poly_dd_init(
    mut dd: *mut libc::c_double,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    *dd.offset(0 as libc::c_int as isize) = *ya.offset(0 as libc::c_int as isize);
    j = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while j >= 1 as libc::c_int as libc::c_ulong {
        *dd
            .offset(
                j as isize,
            ) = (*ya.offset(j as isize)
            - *ya.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            / (*xa.offset(j as isize)
                - *xa
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize));
        j = j.wrapping_sub(1);
        j;
    }
    i = 2 as libc::c_int as size_t;
    while i < size {
        j = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        while j >= i {
            *dd
                .offset(
                    j as isize,
                ) = (*dd.offset(j as isize)
                - *dd.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                / (*xa.offset(j as isize) - *xa.offset(j.wrapping_sub(i) as isize));
            j = j.wrapping_sub(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_poly_dd_taylor(
    mut c: *mut libc::c_double,
    mut xp: libc::c_double,
    mut dd: *const libc::c_double,
    mut xa: *const libc::c_double,
    mut size: size_t,
    mut w: *mut libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        *c.offset(i as isize) = 0.0f64;
        *w.offset(i as isize) = 0.0f64;
        i = i.wrapping_add(1);
        i;
    }
    *w.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) = 1.0f64;
    *c.offset(0 as libc::c_int as isize) = *dd.offset(0 as libc::c_int as isize);
    i = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        *w
            .offset(
                i as isize,
            ) = -*w.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            * (*xa
                .offset(
                    size.wrapping_sub(2 as libc::c_int as libc::c_ulong).wrapping_sub(i)
                        as isize,
                ) - xp);
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *w
                .offset(
                    j as isize,
                ) = *w.offset(j as isize)
                - *w.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    * (*xa
                        .offset(
                            size
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(i) as isize,
                        ) - xp);
            j = j.wrapping_add(1);
            j;
        }
        j = i;
        while j < size {
            *c.offset(j.wrapping_sub(i) as isize)
                += *w.offset(j as isize)
                    * *dd
                        .offset(
                            size
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
            j = j.wrapping_add(1);
            j;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_poly_dd_hermite_init(
    mut dd: *mut libc::c_double,
    mut za: *mut libc::c_double,
    mut xa: *const libc::c_double,
    mut ya: *const libc::c_double,
    mut dya: *const libc::c_double,
    size: size_t,
) -> libc::c_int {
    let N: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    *dd.offset(0 as libc::c_int as isize) = *ya.offset(0 as libc::c_int as isize);
    j = 0 as libc::c_int as size_t;
    while j < size {
        *za
            .offset(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize,
            ) = *xa.offset(j as isize);
        *za
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(j)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *xa.offset(j as isize);
        if j != 0 as libc::c_int as libc::c_ulong {
            *dd
                .offset(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize,
                ) = (*ya.offset(j as isize)
                - *ya.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                / (*xa.offset(j as isize)
                    - *xa
                        .offset(
                            j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ));
            *dd
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(j)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *dya
                .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        }
        j = j.wrapping_add(1);
        j;
    }
    *dd
        .offset(
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *dya.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    i = 2 as libc::c_int as size_t;
    while i < N {
        j = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        while j >= i {
            *dd
                .offset(
                    j as isize,
                ) = (*dd.offset(j as isize)
                - *dd.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                / (*za.offset(j as isize) - *za.offset(j.wrapping_sub(i) as isize));
            j = j.wrapping_sub(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
