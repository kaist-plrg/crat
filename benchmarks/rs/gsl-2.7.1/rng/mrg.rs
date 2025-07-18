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
pub struct mrg_state_t {
    pub x1: libc::c_long,
    pub x2: libc::c_long,
    pub x3: libc::c_long,
    pub x4: libc::c_long,
    pub x5: libc::c_long,
}
static mut m: libc::c_long = 2147483647 as libc::c_int as libc::c_long;
static mut a1: libc::c_long = 107374182 as libc::c_int as libc::c_long;
static mut q1: libc::c_long = 20 as libc::c_int as libc::c_long;
static mut r1: libc::c_long = 7 as libc::c_int as libc::c_long;
static mut a5: libc::c_long = 104480 as libc::c_int as libc::c_long;
static mut q5: libc::c_long = 20554 as libc::c_int as libc::c_long;
static mut r5: libc::c_long = 1727 as libc::c_int as libc::c_long;
#[inline]
unsafe extern "C" fn mrg_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut mrg_state_t = vstate as *mut mrg_state_t;
    let mut p1: libc::c_long = 0;
    let mut h1: libc::c_long = 0;
    let mut p5: libc::c_long = 0;
    let mut h5: libc::c_long = 0;
    h5 = (*state).x5 / q5;
    p5 = a5 * ((*state).x5 - h5 * q5) - h5 * r5;
    if p5 > 0 as libc::c_int as libc::c_long {
        p5 -= m;
    }
    h1 = (*state).x1 / q1;
    p1 = a1 * ((*state).x1 - h1 * q1) - h1 * r1;
    if p1 < 0 as libc::c_int as libc::c_long {
        p1 += m;
    }
    (*state).x5 = (*state).x4;
    (*state).x4 = (*state).x3;
    (*state).x3 = (*state).x2;
    (*state).x2 = (*state).x1;
    (*state).x1 = p1 + p5;
    if (*state).x1 < 0 as libc::c_int as libc::c_long {
        (*state).x1 += m;
    }
    return (*state).x1 as libc::c_ulong;
}
unsafe extern "C" fn mrg_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return mrg_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn mrg_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut mrg_state_t = vstate as *mut mrg_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x1 = s.wrapping_rem(m as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x2 = s.wrapping_rem(m as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x3 = s.wrapping_rem(m as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x4 = s.wrapping_rem(m as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x5 = s.wrapping_rem(m as libc::c_ulong) as libc::c_long;
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
    mrg_get(state as *mut libc::c_void);
}
static mut mrg_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mrg\0" as *const u8 as *const libc::c_char,
        max: 2147483646 as libc::c_int as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<mrg_state_t>() as libc::c_ulong,
        set: Some(
            mrg_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(mrg_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            mrg_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_mrg: *const gsl_rng_type = unsafe {
    &mrg_type as *const gsl_rng_type
};
