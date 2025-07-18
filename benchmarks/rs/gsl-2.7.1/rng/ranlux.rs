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
pub struct ranlux_state_t {
    pub i: libc::c_uint,
    pub j: libc::c_uint,
    pub n: libc::c_uint,
    pub skip: libc::c_uint,
    pub carry: libc::c_uint,
    pub u: [libc::c_ulong; 24],
}
static mut mask_lo: libc::c_ulong = 0xffffff as libc::c_ulong;
static mut mask_hi: libc::c_ulong = !(0xffffff as libc::c_ulong);
static mut two24: libc::c_ulong = 16777216 as libc::c_int as libc::c_ulong;
#[inline]
unsafe extern "C" fn increment_state(mut state: *mut ranlux_state_t) -> libc::c_ulong {
    let mut i: libc::c_uint = (*state).i;
    let mut j: libc::c_uint = (*state).j;
    let mut delta: libc::c_long = ((*state).u[j as usize])
        .wrapping_sub((*state).u[i as usize])
        .wrapping_sub((*state).carry as libc::c_ulong) as libc::c_long;
    if delta as libc::c_ulong & mask_hi != 0 {
        (*state).carry = 1 as libc::c_int as libc::c_uint;
        delta = (delta as libc::c_ulong & mask_lo) as libc::c_long;
    } else {
        (*state).carry = 0 as libc::c_int as libc::c_uint;
    }
    (*state).u[i as usize] = delta as libc::c_ulong;
    if i == 0 as libc::c_int as libc::c_uint {
        i = 23 as libc::c_int as libc::c_uint;
    } else {
        i = i.wrapping_sub(1);
        i;
    }
    (*state).i = i;
    if j == 0 as libc::c_int as libc::c_uint {
        j = 23 as libc::c_int as libc::c_uint;
    } else {
        j = j.wrapping_sub(1);
        j;
    }
    (*state).j = j;
    return delta as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn ranlux_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ranlux_state_t = vstate as *mut ranlux_state_t;
    let skip: libc::c_uint = (*state).skip;
    let mut r: libc::c_ulong = increment_state(state);
    (*state).n = ((*state).n).wrapping_add(1);
    (*state).n;
    if (*state).n == 24 as libc::c_int as libc::c_uint {
        let mut i: libc::c_uint = 0;
        (*state).n = 0 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int as libc::c_uint;
        while i < skip {
            increment_state(state);
            i = i.wrapping_add(1);
            i;
        }
    }
    return r;
}
unsafe extern "C" fn ranlux_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ranlux_get(vstate) as libc::c_double / 16777216.0f64;
}
unsafe extern "C" fn ranlux_set_lux(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
    mut luxury: libc::c_uint,
) {
    let mut state: *mut ranlux_state_t = vstate as *mut ranlux_state_t;
    let mut i: libc::c_int = 0;
    let mut seed: libc::c_long = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 314159265 as libc::c_int as libc::c_ulong;
    }
    seed = s as libc::c_long;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        let mut k: libc::c_ulong = (seed / 53668 as libc::c_int as libc::c_long)
            as libc::c_ulong;
        seed = (40014 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (seed as libc::c_ulong)
                    .wrapping_sub(k.wrapping_mul(53668 as libc::c_int as libc::c_ulong)),
            )
            .wrapping_sub(k.wrapping_mul(12211 as libc::c_int as libc::c_ulong))
            as libc::c_long;
        if seed < 0 as libc::c_int as libc::c_long {
            seed += 2147483563 as libc::c_int as libc::c_long;
        }
        (*state).u[i as usize] = (seed as libc::c_ulong).wrapping_rem(two24);
        i += 1;
        i;
    }
    (*state).i = 23 as libc::c_int as libc::c_uint;
    (*state).j = 9 as libc::c_int as libc::c_uint;
    (*state).n = 0 as libc::c_int as libc::c_uint;
    (*state).skip = luxury.wrapping_sub(24 as libc::c_int as libc::c_uint);
    if (*state).u[23 as libc::c_int as usize] & mask_hi != 0 {
        (*state).carry = 1 as libc::c_int as libc::c_uint;
    } else {
        (*state).carry = 0 as libc::c_int as libc::c_uint;
    };
}
unsafe extern "C" fn ranlux_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    ranlux_set_lux(vstate, s, 223 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn ranlux389_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    ranlux_set_lux(vstate, s, 389 as libc::c_int as libc::c_uint);
}
static mut ranlux_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlux\0" as *const u8 as *const libc::c_char,
        max: 0xffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ranlux_state_t>() as libc::c_ulong,
        set: Some(
            ranlux_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            ranlux_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            ranlux_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut ranlux389_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlux389\0" as *const u8 as *const libc::c_char,
        max: 0xffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ranlux_state_t>() as libc::c_ulong,
        set: Some(
            ranlux389_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            ranlux_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            ranlux_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ranlux: *const gsl_rng_type = unsafe {
    &ranlux_type as *const gsl_rng_type
};
pub static mut gsl_rng_ranlux389: *const gsl_rng_type = unsafe {
    &ranlux389_type as *const gsl_rng_type
};
