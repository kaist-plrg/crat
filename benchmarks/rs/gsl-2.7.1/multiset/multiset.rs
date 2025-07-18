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
pub struct gsl_multiset_struct {
    pub n: size_t,
    pub k: size_t,
    pub data: *mut size_t,
}
pub type gsl_multiset = gsl_multiset_struct;
pub unsafe extern "C" fn gsl_multiset_n(mut c: *const gsl_multiset) -> size_t {
    return (*c).n;
}
pub unsafe extern "C" fn gsl_multiset_k(mut c: *const gsl_multiset) -> size_t {
    return (*c).k;
}
pub unsafe extern "C" fn gsl_multiset_data(mut c: *const gsl_multiset) -> *mut size_t {
    return (*c).data;
}
pub unsafe extern "C" fn gsl_multiset_valid(mut c: *mut gsl_multiset) -> libc::c_int {
    let n: size_t = (*c).n;
    let k: size_t = (*c).k;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < k {
        let ci: size_t = *((*c).data).offset(i as isize);
        if ci >= n {
            gsl_error(
                b"multiset index outside range\0" as *const u8 as *const libc::c_char,
                b"multiset.c\0" as *const u8 as *const libc::c_char,
                59 as libc::c_int,
                GSL_FAILURE as libc::c_int,
            );
            return GSL_FAILURE as libc::c_int;
        }
        j = 0 as libc::c_int as size_t;
        while j < i {
            if *((*c).data).offset(j as isize) > ci {
                gsl_error(
                    b"multiset indices not in increasing order\0" as *const u8
                        as *const libc::c_char,
                    b"multiset.c\0" as *const u8 as *const libc::c_char,
                    67 as libc::c_int,
                    GSL_FAILURE as libc::c_int,
                );
                return GSL_FAILURE as libc::c_int;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_multiset_next(mut c: *mut gsl_multiset) -> libc::c_int {
    let n: size_t = (*c).n;
    let k: size_t = (*c).k;
    let mut data: *mut size_t = (*c).data;
    let mut i: size_t = 0;
    if k == 0 as libc::c_int as libc::c_ulong {
        return GSL_FAILURE as libc::c_int;
    }
    i = k.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > 0 as libc::c_int as libc::c_ulong
        && *data.offset(i as isize) == n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        i = i.wrapping_sub(1);
        i;
    }
    if i == 0 as libc::c_int as libc::c_ulong
        && *data.offset(0 as libc::c_int as isize)
            == n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return GSL_FAILURE as libc::c_int;
    }
    let ref mut fresh0 = *data.offset(i as isize);
    *fresh0 = (*fresh0).wrapping_add(1);
    *fresh0;
    while i < k.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        *data
            .offset(
                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *data.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_multiset_prev(mut c: *mut gsl_multiset) -> libc::c_int {
    let n: size_t = (*c).n;
    let k: size_t = (*c).k;
    let mut data: *mut size_t = (*c).data;
    let mut i: size_t = 0;
    if k == 0 as libc::c_int as libc::c_ulong {
        return GSL_FAILURE as libc::c_int;
    }
    i = k.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > 0 as libc::c_int as libc::c_ulong
        && *data.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == *data.offset(i as isize)
    {
        i = i.wrapping_sub(1);
        i;
    }
    if i == 0 as libc::c_int as libc::c_ulong
        && *data.offset(i as isize) == 0 as libc::c_int as libc::c_ulong
    {
        return GSL_FAILURE as libc::c_int;
    }
    let ref mut fresh1 = *data.offset(i as isize);
    *fresh1 = (*fresh1).wrapping_sub(1);
    *fresh1;
    if *data.offset(i as isize) < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        while i < k.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            i = i.wrapping_add(1);
            *data.offset(i as isize) = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_multiset_memcpy(
    mut dest: *mut gsl_multiset,
    mut src: *const gsl_multiset,
) -> libc::c_int {
    let src_n: size_t = (*src).n;
    let src_k: size_t = (*src).k;
    let dest_n: size_t = (*dest).n;
    let dest_k: size_t = (*dest).k;
    if src_n != dest_n || src_k != dest_k {
        gsl_error(
            b"multiset lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"multiset.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_k {
        *((*dest).data).offset(j as isize) = *((*src).data).offset(j as isize);
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
