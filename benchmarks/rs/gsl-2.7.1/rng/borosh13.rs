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
pub struct ran_state_t {
    pub x: libc::c_ulong,
}
#[inline]
unsafe extern "C" fn ran_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    (*state)
        .x = (1812433253 as libc::c_ulong).wrapping_mul((*state).x)
        & 0xffffffff as libc::c_ulong;
    return (*state).x;
}
unsafe extern "C" fn ran_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    return ran_get(state as *mut libc::c_void) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn ran_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).x = s & 0xffffffff as libc::c_ulong;
}
static mut ran_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"borosh13\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 1 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ran_state_t>() as libc::c_ulong,
        set: Some(
            ran_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(ran_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            ran_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_borosh13: *const gsl_rng_type = unsafe {
    &ran_type as *const gsl_rng_type
};
