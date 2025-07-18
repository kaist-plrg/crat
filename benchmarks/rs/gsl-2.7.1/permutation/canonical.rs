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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
pub unsafe extern "C" fn gsl_permutation_linear_to_canonical(
    mut q: *mut gsl_permutation,
    mut p: *const gsl_permutation,
) -> libc::c_int {
    let n: size_t = (*p).size;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut s: size_t = 0;
    let mut t: size_t = n;
    let pp: *const size_t = (*p).data;
    let qq: *mut size_t = (*q).data;
    if (*q).size != (*p).size {
        gsl_error(
            b"size of q does not match size of p\0" as *const u8 as *const libc::c_char,
            b"canonical.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        k = *pp.offset(i as isize);
        s = 1 as libc::c_int as size_t;
        while k > i {
            k = *pp.offset(k as isize);
            s = s.wrapping_add(1);
            s;
        }
        if !(k < i) {
            t = (t as libc::c_ulong).wrapping_sub(s) as size_t as size_t;
            *qq.offset(t as isize) = i;
            k = *pp.offset(i as isize);
            s = 1 as libc::c_int as size_t;
            while k > i {
                *qq.offset(t.wrapping_add(s) as isize) = k;
                k = *pp.offset(k as isize);
                s = s.wrapping_add(1);
                s;
            }
            if t == 0 as libc::c_int as libc::c_ulong {
                break;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_canonical_to_linear(
    mut p: *mut gsl_permutation,
    mut q: *const gsl_permutation,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut kk: size_t = 0;
    let mut first: size_t = 0;
    let n: size_t = (*p).size;
    let pp: *mut size_t = (*p).data;
    let qq: *const size_t = (*q).data;
    if (*q).size != (*p).size {
        gsl_error(
            b"size of q does not match size of p\0" as *const u8 as *const libc::c_char,
            b"canonical.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *pp.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    k = *qq.offset(0 as libc::c_int as isize);
    first = *pp.offset(k as isize);
    i = 1 as libc::c_int as size_t;
    while i < n {
        kk = *qq.offset(i as isize);
        if kk > first {
            *pp.offset(k as isize) = *pp.offset(kk as isize);
            k = kk;
        } else {
            *pp.offset(k as isize) = first;
            k = kk;
            first = *pp.offset(kk as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    *pp.offset(k as isize) = first;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_inversions(
    mut p: *const gsl_permutation,
) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let size: size_t = (*p).size;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < size {
            if *((*p).data).offset(i as isize) > *((*p).data).offset(j as isize) {
                count = count.wrapping_add(1);
                count;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return count;
}
pub unsafe extern "C" fn gsl_permutation_linear_cycles(
    mut p: *const gsl_permutation,
) -> size_t {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let size: size_t = (*p).size;
    i = 0 as libc::c_int as size_t;
    while i < size {
        k = *((*p).data).offset(i as isize);
        while k > i {
            k = *((*p).data).offset(k as isize);
        }
        if !(k < i) {
            count = count.wrapping_add(1);
            count;
        }
        i = i.wrapping_add(1);
        i;
    }
    return count;
}
pub unsafe extern "C" fn gsl_permutation_canonical_cycles(
    mut p: *const gsl_permutation,
) -> size_t {
    let mut i: size_t = 0;
    let mut count: size_t = 1 as libc::c_int as size_t;
    let mut min: size_t = *((*p).data).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i < (*p).size {
        if *((*p).data).offset(i as isize) < min {
            min = *((*p).data).offset(i as isize);
            count = count.wrapping_add(1);
            count;
        }
        i = i.wrapping_add(1);
        i;
    }
    return count;
}
