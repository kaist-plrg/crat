use ::libc;
pub type size_t = libc::c_ulong;
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
pub struct slatec_state_t {
    pub x0: libc::c_long,
    pub x1: libc::c_long,
}
static mut P: libc::c_long = 4194304 as libc::c_int as libc::c_long;
static mut a1: libc::c_long = 1536 as libc::c_int as libc::c_long;
static mut a0: libc::c_long = 1029 as libc::c_int as libc::c_long;
static mut a1ma0: libc::c_long = 507 as libc::c_int as libc::c_long;
static mut c: libc::c_long = 1731 as libc::c_int as libc::c_long;
#[inline]
unsafe extern "C" fn slatec_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut y0: libc::c_long = 0;
    let mut y1: libc::c_long = 0;
    let mut state: *mut slatec_state_t = vstate as *mut slatec_state_t;
    y0 = a0 * (*state).x0;
    y1 = a1 * (*state).x1 + a1ma0 * ((*state).x0 - (*state).x1) + y0;
    y0 = y0 + c;
    (*state).x0 = y0 % 2048 as libc::c_int as libc::c_long;
    y1 = y1 + (y0 - (*state).x0) / 2048 as libc::c_int as libc::c_long;
    (*state).x1 = y1 % 2048 as libc::c_int as libc::c_long;
    return ((*state).x1 * 2048 as libc::c_int as libc::c_long + (*state).x0)
        as libc::c_ulong;
}
unsafe extern "C" fn slatec_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return slatec_get(vstate) as libc::c_double / 4194304.0f64;
}
unsafe extern "C" fn slatec_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut slatec_state_t = vstate as *mut slatec_state_t;
    s = s.wrapping_rem(8 as libc::c_int as libc::c_ulong);
    s = s.wrapping_mul((P / 8 as libc::c_int as libc::c_long) as libc::c_ulong);
    (*state).x0 = s.wrapping_rem(2048 as libc::c_int as libc::c_ulong) as libc::c_long;
    (*state)
        .x1 = s
        .wrapping_sub((*state).x0 as libc::c_ulong)
        .wrapping_div(2048 as libc::c_int as libc::c_ulong) as libc::c_long;
}
static mut slatec_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"slatec\0" as *const u8 as *const libc::c_char,
        max: 4194303 as libc::c_int as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<slatec_state_t>() as libc::c_ulong,
        set: Some(
            slatec_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            slatec_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            slatec_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_slatec: *const gsl_rng_type = unsafe {
    &slatec_type as *const gsl_rng_type
};
