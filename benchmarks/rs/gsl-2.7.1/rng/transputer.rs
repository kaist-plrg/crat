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
pub struct transputer_state_t {
    pub x: libc::c_ulong,
}
unsafe extern "C" fn transputer_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut transputer_state_t = vstate as *mut transputer_state_t;
    (*state)
        .x = (1664525 as libc::c_int as libc::c_ulong).wrapping_mul((*state).x)
        & 0xffffffff as libc::c_ulong;
    return (*state).x;
}
unsafe extern "C" fn transputer_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return transputer_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn transputer_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut transputer_state_t = vstate as *mut transputer_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).x = s;
}
static mut transputer_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"transputer\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 1 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<transputer_state_t>() as libc::c_ulong,
        set: Some(
            transputer_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            transputer_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            transputer_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_transputer: *const gsl_rng_type = unsafe {
    &transputer_type as *const gsl_rng_type
};
