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
pub struct cmrg_state_t {
    pub x1: libc::c_long,
    pub x2: libc::c_long,
    pub x3: libc::c_long,
    pub y1: libc::c_long,
    pub y2: libc::c_long,
    pub y3: libc::c_long,
}
static mut m1: libc::c_long = 2147483647 as libc::c_int as libc::c_long;
static mut m2: libc::c_long = 2145483479 as libc::c_int as libc::c_long;
static mut a2: libc::c_long = 63308 as libc::c_int as libc::c_long;
static mut qa2: libc::c_long = 33921 as libc::c_int as libc::c_long;
static mut ra2: libc::c_long = 12979 as libc::c_int as libc::c_long;
static mut a3: libc::c_long = -(183326 as libc::c_int) as libc::c_long;
static mut qa3: libc::c_long = 11714 as libc::c_int as libc::c_long;
static mut ra3: libc::c_long = 2883 as libc::c_int as libc::c_long;
static mut b1: libc::c_long = 86098 as libc::c_int as libc::c_long;
static mut qb1: libc::c_long = 24919 as libc::c_int as libc::c_long;
static mut rb1: libc::c_long = 7417 as libc::c_int as libc::c_long;
static mut b3: libc::c_long = -(539608 as libc::c_int) as libc::c_long;
static mut qb3: libc::c_long = 3976 as libc::c_int as libc::c_long;
static mut rb3: libc::c_long = 2071 as libc::c_int as libc::c_long;
#[inline]
unsafe extern "C" fn cmrg_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut cmrg_state_t = vstate as *mut cmrg_state_t;
    let mut h3: libc::c_long = (*state).x3 / qa3;
    let mut p3: libc::c_long = -a3 * ((*state).x3 - h3 * qa3) - h3 * ra3;
    let mut h2: libc::c_long = (*state).x2 / qa2;
    let mut p2: libc::c_long = a2 * ((*state).x2 - h2 * qa2) - h2 * ra2;
    if p3 < 0 as libc::c_int as libc::c_long {
        p3 += m1;
    }
    if p2 < 0 as libc::c_int as libc::c_long {
        p2 += m1;
    }
    (*state).x3 = (*state).x2;
    (*state).x2 = (*state).x1;
    (*state).x1 = p2 - p3;
    if (*state).x1 < 0 as libc::c_int as libc::c_long {
        (*state).x1 += m1;
    }
    let mut h3_0: libc::c_long = (*state).y3 / qb3;
    let mut p3_0: libc::c_long = -b3 * ((*state).y3 - h3_0 * qb3) - h3_0 * rb3;
    let mut h1: libc::c_long = (*state).y1 / qb1;
    let mut p1: libc::c_long = b1 * ((*state).y1 - h1 * qb1) - h1 * rb1;
    if p3_0 < 0 as libc::c_int as libc::c_long {
        p3_0 += m2;
    }
    if p1 < 0 as libc::c_int as libc::c_long {
        p1 += m2;
    }
    (*state).y3 = (*state).y2;
    (*state).y2 = (*state).y1;
    (*state).y1 = p1 - p3_0;
    if (*state).y1 < 0 as libc::c_int as libc::c_long {
        (*state).y1 += m2;
    }
    if (*state).x1 < (*state).y1 {
        return ((*state).x1 - (*state).y1 + m1) as libc::c_ulong
    } else {
        return ((*state).x1 - (*state).y1) as libc::c_ulong
    };
}
unsafe extern "C" fn cmrg_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return cmrg_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn cmrg_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut cmrg_state_t = vstate as *mut cmrg_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x1 = s.wrapping_rem(m1 as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x2 = s.wrapping_rem(m1 as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).x3 = s.wrapping_rem(m1 as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).y1 = s.wrapping_rem(m2 as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).y2 = s.wrapping_rem(m2 as libc::c_ulong) as libc::c_long;
    s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
        & 0xffffffff as libc::c_ulong;
    (*state).y3 = s.wrapping_rem(m2 as libc::c_ulong) as libc::c_long;
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
    cmrg_get(state as *mut libc::c_void);
}
static mut cmrg_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"cmrg\0" as *const u8 as *const libc::c_char,
        max: 2147483646 as libc::c_int as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<cmrg_state_t>() as libc::c_ulong,
        set: Some(
            cmrg_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(cmrg_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            cmrg_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_cmrg: *const gsl_rng_type = unsafe {
    &cmrg_type as *const gsl_rng_type
};
