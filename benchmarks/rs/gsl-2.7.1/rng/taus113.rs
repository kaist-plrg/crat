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
pub struct taus113_state_t {
    pub z1: libc::c_ulong,
    pub z2: libc::c_ulong,
    pub z3: libc::c_ulong,
    pub z4: libc::c_ulong,
}
#[inline]
unsafe extern "C" fn taus113_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut taus113_state_t = vstate as *mut taus113_state_t;
    let mut b1: libc::c_ulong = 0;
    let mut b2: libc::c_ulong = 0;
    let mut b3: libc::c_ulong = 0;
    let mut b4: libc::c_ulong = 0;
    b1 = ((*state).z1 << 6 as libc::c_ulong & 0xffffffff as libc::c_ulong ^ (*state).z1)
        >> 13 as libc::c_ulong;
    (*state)
        .z1 = ((*state).z1 & 4294967294 as libc::c_ulong) << 18 as libc::c_ulong
        & 0xffffffff as libc::c_ulong ^ b1;
    b2 = ((*state).z2 << 2 as libc::c_ulong & 0xffffffff as libc::c_ulong ^ (*state).z2)
        >> 27 as libc::c_ulong;
    (*state)
        .z2 = ((*state).z2 & 4294967288 as libc::c_ulong) << 2 as libc::c_ulong
        & 0xffffffff as libc::c_ulong ^ b2;
    b3 = ((*state).z3 << 13 as libc::c_ulong & 0xffffffff as libc::c_ulong ^ (*state).z3)
        >> 21 as libc::c_ulong;
    (*state)
        .z3 = ((*state).z3 & 4294967280 as libc::c_ulong) << 7 as libc::c_ulong
        & 0xffffffff as libc::c_ulong ^ b3;
    b4 = ((*state).z4 << 3 as libc::c_ulong & 0xffffffff as libc::c_ulong ^ (*state).z4)
        >> 12 as libc::c_ulong;
    (*state)
        .z4 = ((*state).z4 & 4294967168 as libc::c_ulong) << 13 as libc::c_ulong
        & 0xffffffff as libc::c_ulong ^ b4;
    return (*state).z1 ^ (*state).z2 ^ (*state).z3 ^ (*state).z4;
}
unsafe extern "C" fn taus113_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return taus113_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn taus113_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut taus113_state_t = vstate as *mut taus113_state_t;
    if s == 0 {
        s = 1 as libc::c_ulong;
    }
    (*state).z1 = (69069 as libc::c_ulong).wrapping_mul(s) & 0xffffffff as libc::c_ulong;
    if (*state).z1 < 2 as libc::c_ulong {
        (*state).z1 = ((*state).z1).wrapping_add(2 as libc::c_ulong);
    }
    (*state)
        .z2 = (69069 as libc::c_ulong).wrapping_mul((*state).z1)
        & 0xffffffff as libc::c_ulong;
    if (*state).z2 < 8 as libc::c_ulong {
        (*state).z2 = ((*state).z2).wrapping_add(8 as libc::c_ulong);
    }
    (*state)
        .z3 = (69069 as libc::c_ulong).wrapping_mul((*state).z2)
        & 0xffffffff as libc::c_ulong;
    if (*state).z3 < 16 as libc::c_ulong {
        (*state).z3 = ((*state).z3).wrapping_add(16 as libc::c_ulong);
    }
    (*state)
        .z4 = (69069 as libc::c_ulong).wrapping_mul((*state).z3)
        & 0xffffffff as libc::c_ulong;
    if (*state).z4 < 128 as libc::c_ulong {
        (*state).z4 = ((*state).z4).wrapping_add(128 as libc::c_ulong);
    }
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
    taus113_get(state as *mut libc::c_void);
}
static mut taus113_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"taus113\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<taus113_state_t>() as libc::c_ulong,
        set: Some(
            taus113_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            taus113_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            taus113_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_taus113: *const gsl_rng_type = unsafe {
    &taus113_type as *const gsl_rng_type
};
