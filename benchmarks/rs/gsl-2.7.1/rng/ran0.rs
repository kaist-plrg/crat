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
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ran0_state_t {
    pub x: libc::c_ulong,
}
static mut m: libc::c_long = 2147483647 as libc::c_int as libc::c_long;
static mut a: libc::c_long = 16807 as libc::c_int as libc::c_long;
static mut q: libc::c_long = 127773 as libc::c_int as libc::c_long;
static mut r: libc::c_long = 2836 as libc::c_int as libc::c_long;
static mut mask: libc::c_ulong = 123459876 as libc::c_int as libc::c_ulong;
#[inline]
unsafe extern "C" fn ran0_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran0_state_t = vstate as *mut ran0_state_t;
    let x: libc::c_ulong = (*state).x;
    let h: libc::c_long = x.wrapping_div(q as libc::c_ulong) as libc::c_long;
    let t: libc::c_long = (a as libc::c_ulong)
        .wrapping_mul(x.wrapping_sub((h * q) as libc::c_ulong))
        .wrapping_sub((h * r) as libc::c_ulong) as libc::c_long;
    if t < 0 as libc::c_int as libc::c_long {
        (*state).x = (t + m) as libc::c_ulong;
    } else {
        (*state).x = t as libc::c_ulong;
    }
    return (*state).x;
}
unsafe extern "C" fn ran0_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ran0_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn ran0_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran0_state_t = vstate as *mut ran0_state_t;
    if s == mask {
        gsl_error(
            b"ran0 should not use seed == mask\0" as *const u8 as *const libc::c_char,
            b"ran0.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return;
    }
    (*state).x = s ^ mask;
}
static mut ran0_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran0\0" as *const u8 as *const libc::c_char,
        max: 2147483646 as libc::c_int as libc::c_ulong,
        min: 1 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ran0_state_t>() as libc::c_ulong,
        set: Some(
            ran0_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(ran0_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            ran0_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ran0: *const gsl_rng_type = unsafe {
    &ran0_type as *const gsl_rng_type
};
