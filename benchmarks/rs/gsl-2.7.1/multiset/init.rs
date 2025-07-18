use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub unsafe extern "C" fn gsl_multiset_alloc(n: size_t, k: size_t) -> *mut gsl_multiset {
    let mut c: *mut gsl_multiset = 0 as *mut gsl_multiset;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"multiset parameter n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"init.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_multiset;
    }
    c = malloc(::std::mem::size_of::<gsl_multiset>() as libc::c_ulong)
        as *mut gsl_multiset;
    if c.is_null() {
        gsl_error(
            b"failed to allocate space for multiset struct\0" as *const u8
                as *const libc::c_char,
            b"init.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multiset;
    }
    if k > 0 as libc::c_int as libc::c_ulong {
        (*c)
            .data = malloc(
            k.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as *mut size_t;
        if ((*c).data).is_null() {
            free(c as *mut libc::c_void);
            gsl_error(
                b"failed to allocate space for multiset data\0" as *const u8
                    as *const libc::c_char,
                b"init.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return 0 as *mut gsl_multiset;
        }
    } else {
        (*c).data = 0 as *mut size_t;
    }
    (*c).n = n;
    (*c).k = k;
    return c;
}
pub unsafe extern "C" fn gsl_multiset_calloc(n: size_t, k: size_t) -> *mut gsl_multiset {
    let mut i: size_t = 0;
    let mut c: *mut gsl_multiset = gsl_multiset_alloc(n, k);
    if c.is_null() {
        return 0 as *mut gsl_multiset;
    }
    i = 0 as libc::c_int as size_t;
    while i < k {
        *((*c).data).offset(i as isize) = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return c;
}
pub unsafe extern "C" fn gsl_multiset_init_first(mut c: *mut gsl_multiset) {
    let k: size_t = (*c).k;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < k {
        *((*c).data).offset(i as isize) = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_multiset_init_last(mut c: *mut gsl_multiset) {
    let k: size_t = (*c).k;
    let mut i: size_t = 0;
    let mut n: size_t = (*c).n;
    i = 0 as libc::c_int as size_t;
    while i < k {
        *((*c).data)
            .offset(i as isize) = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_multiset_free(mut c: *mut gsl_multiset) {
    if c.is_null() {
        return;
    }
    if (*c).k > 0 as libc::c_int as libc::c_ulong {
        free((*c).data as *mut libc::c_void);
    }
    free(c as *mut libc::c_void);
}
