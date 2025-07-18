use ::libc;
extern "C" {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
unsafe extern "C" fn find(
    n: size_t,
    mut range: *const libc::c_double,
    x: libc::c_double,
    mut i: *mut size_t,
) -> libc::c_int {
    let mut i_linear: size_t = 0;
    let mut lower: size_t = 0;
    let mut upper: size_t = 0;
    let mut mid: size_t = 0;
    if x < *range.offset(0 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    if x >= *range.offset(n as isize) {
        return 1 as libc::c_int;
    }
    let mut u: libc::c_double = (x - *range.offset(0 as libc::c_int as isize))
        / (*range.offset(n as isize) - *range.offset(0 as libc::c_int as isize));
    i_linear = (u * n as libc::c_double) as size_t;
    if x >= *range.offset(i_linear as isize)
        && x
            < *range
                .offset(
                    i_linear.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
    {
        *i = i_linear;
        return 0 as libc::c_int;
    }
    upper = n;
    lower = 0 as libc::c_int as size_t;
    while upper.wrapping_sub(lower) > 1 as libc::c_int as libc::c_ulong {
        mid = upper.wrapping_add(lower).wrapping_div(2 as libc::c_int as libc::c_ulong);
        if x >= *range.offset(mid as isize) {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    *i = lower;
    if x < *range.offset(lower as isize)
        || x
            >= *range
                .offset(lower.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
    {
        gsl_error(
            b"x not found in range\0" as *const u8 as *const libc::c_char,
            b"./find.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_get(
    mut h: *const gsl_histogram,
    mut i: size_t,
) -> libc::c_double {
    let n: size_t = (*h).n;
    if i >= n {
        gsl_error(
            b"index lies outside valid range of 0 .. n - 1\0" as *const u8
                as *const libc::c_char,
            b"get.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_double;
    }
    return *((*h).bin).offset(i as isize);
}
pub unsafe extern "C" fn gsl_histogram_get_range(
    mut h: *const gsl_histogram,
    mut i: size_t,
    mut lower: *mut libc::c_double,
    mut upper: *mut libc::c_double,
) -> libc::c_int {
    let n: size_t = (*h).n;
    if i >= n {
        gsl_error(
            b"index lies outside valid range of 0 .. n - 1\0" as *const u8
                as *const libc::c_char,
            b"get.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    *lower = *((*h).range).offset(i as isize);
    *upper = *((*h).range)
        .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_find(
    mut h: *const gsl_histogram,
    x: libc::c_double,
    mut i: *mut size_t,
) -> libc::c_int {
    let mut status: libc::c_int = find(
        (*h).n,
        (*h).range as *const libc::c_double,
        x,
        i,
    );
    if status != 0 {
        gsl_error(
            b"x not found in range of h\0" as *const u8 as *const libc::c_char,
            b"get.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
