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
pub unsafe extern "C" fn gsl_histogram_equal_bins_p(
    mut h1: *const gsl_histogram,
    mut h2: *const gsl_histogram,
) -> libc::c_int {
    if (*h1).n != (*h2).n {
        return 0 as libc::c_int;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= (*h1).n {
        if *((*h1).range).offset(i as isize) != *((*h2).range).offset(i as isize) {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_add(
    mut h1: *mut gsl_histogram,
    mut h2: *const gsl_histogram,
) -> libc::c_int {
    let mut i: size_t = 0;
    if gsl_histogram_equal_bins_p(h1, h2) == 0 {
        gsl_error(
            b"histograms have different binning\0" as *const u8 as *const libc::c_char,
            b"oper.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*h1).n {
        *((*h1).bin).offset(i as isize) += *((*h2).bin).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_sub(
    mut h1: *mut gsl_histogram,
    mut h2: *const gsl_histogram,
) -> libc::c_int {
    let mut i: size_t = 0;
    if gsl_histogram_equal_bins_p(h1, h2) == 0 {
        gsl_error(
            b"histograms have different binning\0" as *const u8 as *const libc::c_char,
            b"oper.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*h1).n {
        *((*h1).bin).offset(i as isize) -= *((*h2).bin).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_mul(
    mut h1: *mut gsl_histogram,
    mut h2: *const gsl_histogram,
) -> libc::c_int {
    let mut i: size_t = 0;
    if gsl_histogram_equal_bins_p(h1, h2) == 0 {
        gsl_error(
            b"histograms have different binning\0" as *const u8 as *const libc::c_char,
            b"oper.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*h1).n {
        *((*h1).bin).offset(i as isize) *= *((*h2).bin).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_div(
    mut h1: *mut gsl_histogram,
    mut h2: *const gsl_histogram,
) -> libc::c_int {
    let mut i: size_t = 0;
    if gsl_histogram_equal_bins_p(h1, h2) == 0 {
        gsl_error(
            b"histograms have different binning\0" as *const u8 as *const libc::c_char,
            b"oper.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*h1).n {
        *((*h1).bin).offset(i as isize) /= *((*h2).bin).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_scale(
    mut h: *mut gsl_histogram,
    mut scale: libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*h).n {
        *((*h).bin).offset(i as isize) *= scale;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_histogram_shift(
    mut h: *mut gsl_histogram,
    mut shift: libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*h).n {
        *((*h).bin).offset(i as isize) += shift;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
