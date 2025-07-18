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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_wavelet_type {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut size_t,
            *mut size_t,
            size_t,
        ) -> libc::c_int,
    >,
}
static mut ch_2: [libc::c_double; 2] = [
    0.70710678118654752440f64,
    0.70710678118654752440f64,
];
static mut cg_2: [libc::c_double; 2] = [
    0.70710678118654752440f64,
    -0.70710678118654752440f64,
];
unsafe extern "C" fn haar_init(
    mut h1: *mut *const libc::c_double,
    mut g1: *mut *const libc::c_double,
    mut h2: *mut *const libc::c_double,
    mut g2: *mut *const libc::c_double,
    mut nc: *mut size_t,
    mut offset: *mut size_t,
    member: size_t,
) -> libc::c_int {
    if member != 2 as libc::c_int as libc::c_ulong {
        return GSL_FAILURE as libc::c_int;
    }
    *h1 = ch_2.as_ptr();
    *g1 = cg_2.as_ptr();
    *h2 = ch_2.as_ptr();
    *g2 = cg_2.as_ptr();
    *nc = 2 as libc::c_int as size_t;
    *offset = 0 as libc::c_int as size_t;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn haar_centered_init(
    mut h1: *mut *const libc::c_double,
    mut g1: *mut *const libc::c_double,
    mut h2: *mut *const libc::c_double,
    mut g2: *mut *const libc::c_double,
    mut nc: *mut size_t,
    mut offset: *mut size_t,
    member: size_t,
) -> libc::c_int {
    if member != 2 as libc::c_int as libc::c_ulong {
        return GSL_FAILURE as libc::c_int;
    }
    *h1 = ch_2.as_ptr();
    *g1 = cg_2.as_ptr();
    *h2 = ch_2.as_ptr();
    *g2 = cg_2.as_ptr();
    *nc = 2 as libc::c_int as size_t;
    *offset = 1 as libc::c_int as size_t;
    return GSL_SUCCESS as libc::c_int;
}
static mut haar_type: gsl_wavelet_type = {
    let mut init = gsl_wavelet_type {
        name: b"haar\0" as *const u8 as *const libc::c_char,
        init: Some(
            haar_init
                as unsafe extern "C" fn(
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut size_t,
                    *mut size_t,
                    size_t,
                ) -> libc::c_int,
        ),
    };
    init
};
static mut haar_centered_type: gsl_wavelet_type = {
    let mut init = gsl_wavelet_type {
        name: b"haar-centered\0" as *const u8 as *const libc::c_char,
        init: Some(
            haar_centered_init
                as unsafe extern "C" fn(
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut size_t,
                    *mut size_t,
                    size_t,
                ) -> libc::c_int,
        ),
    };
    init
};
pub static mut gsl_wavelet_haar: *const gsl_wavelet_type = unsafe {
    &haar_type as *const gsl_wavelet_type
};
pub static mut gsl_wavelet_haar_centered: *const gsl_wavelet_type = unsafe {
    &haar_centered_type as *const gsl_wavelet_type
};
