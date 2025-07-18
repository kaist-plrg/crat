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
pub struct taus_state_t {
    pub s1: libc::c_ulong,
    pub s2: libc::c_ulong,
    pub s3: libc::c_ulong,
}
#[inline]
unsafe extern "C" fn taus_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut taus_state_t = vstate as *mut taus_state_t;
    (*state)
        .s1 = ((*state).s1 & 4294967294 as libc::c_ulong) << 12 as libc::c_int
        & 0xffffffff as libc::c_ulong
        ^ ((*state).s1 << 13 as libc::c_int & 0xffffffff as libc::c_ulong ^ (*state).s1)
            >> 19 as libc::c_int;
    (*state)
        .s2 = ((*state).s2 & 4294967288 as libc::c_ulong) << 4 as libc::c_int
        & 0xffffffff as libc::c_ulong
        ^ ((*state).s2 << 2 as libc::c_int & 0xffffffff as libc::c_ulong ^ (*state).s2)
            >> 25 as libc::c_int;
    (*state)
        .s3 = ((*state).s3 & 4294967280 as libc::c_ulong) << 17 as libc::c_int
        & 0xffffffff as libc::c_ulong
        ^ ((*state).s3 << 3 as libc::c_int & 0xffffffff as libc::c_ulong ^ (*state).s3)
            >> 11 as libc::c_int;
    return (*state).s1 ^ (*state).s2 ^ (*state).s3;
}
unsafe extern "C" fn taus_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return taus_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn taus_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut taus_state_t = vstate as *mut taus_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state)
        .s1 = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state)
        .s2 = (69069 as libc::c_int as libc::c_ulong).wrapping_mul((*state).s1)
        & 0xffffffff as libc::c_ulong;
    (*state)
        .s3 = (69069 as libc::c_int as libc::c_ulong).wrapping_mul((*state).s2)
        & 0xffffffff as libc::c_ulong;
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
}
unsafe extern "C" fn taus2_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut taus_state_t = vstate as *mut taus_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state)
        .s1 = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    if (*state).s1 < 2 as libc::c_int as libc::c_ulong {
        (*state).s1 = ((*state).s1).wrapping_add(2 as libc::c_ulong);
    }
    (*state)
        .s2 = (69069 as libc::c_int as libc::c_ulong).wrapping_mul((*state).s1)
        & 0xffffffff as libc::c_ulong;
    if (*state).s2 < 8 as libc::c_int as libc::c_ulong {
        (*state).s2 = ((*state).s2).wrapping_add(8 as libc::c_ulong);
    }
    (*state)
        .s3 = (69069 as libc::c_int as libc::c_ulong).wrapping_mul((*state).s2)
        & 0xffffffff as libc::c_ulong;
    if (*state).s3 < 16 as libc::c_int as libc::c_ulong {
        (*state).s3 = ((*state).s3).wrapping_add(16 as libc::c_ulong);
    }
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
    taus_get(state as *mut libc::c_void);
}
static mut taus_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"taus\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<taus_state_t>() as libc::c_ulong,
        set: Some(
            taus_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(taus_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            taus_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_taus: *const gsl_rng_type = unsafe {
    &taus_type as *const gsl_rng_type
};
static mut taus2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"taus2\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<taus_state_t>() as libc::c_ulong,
        set: Some(
            taus2_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(taus_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            taus_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_taus2: *const gsl_rng_type = unsafe {
    &taus2_type as *const gsl_rng_type
};
