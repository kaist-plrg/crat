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
pub struct rand_state_t {
    pub x: libc::c_ulong,
}
#[inline]
unsafe extern "C" fn rand_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut rand_state_t = vstate as *mut rand_state_t;
    (*state)
        .x = (1103515245 as libc::c_int as libc::c_ulong)
        .wrapping_mul((*state).x)
        .wrapping_add(12345 as libc::c_int as libc::c_ulong)
        & 0x7fffffff as libc::c_ulong;
    return (*state).x;
}
unsafe extern "C" fn rand_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return rand_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn rand_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut rand_state_t = vstate as *mut rand_state_t;
    (*state).x = s;
}
static mut rand_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"rand\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<rand_state_t>() as libc::c_ulong,
        set: Some(
            rand_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(rand_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            rand_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_rand: *const gsl_rng_type = unsafe {
    &rand_type as *const gsl_rng_type
};
