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
pub struct r250_state_t {
    pub i: libc::c_int,
    pub x: [libc::c_ulong; 250],
}
#[inline]
unsafe extern "C" fn r250_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut r250_state_t = vstate as *mut r250_state_t;
    let mut k: libc::c_ulong = 0;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = (*state).i;
    if i >= 147 as libc::c_int {
        j = i - 147 as libc::c_int;
    } else {
        j = i + 103 as libc::c_int;
    }
    k = (*state).x[i as usize] ^ (*state).x[j as usize];
    (*state).x[i as usize] = k;
    if i >= 249 as libc::c_int {
        (*state).i = 0 as libc::c_int;
    } else {
        (*state).i = i + 1 as libc::c_int;
    }
    return k;
}
unsafe extern "C" fn r250_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return r250_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn r250_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut r250_state_t = vstate as *mut r250_state_t;
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).i = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 250 as libc::c_int {
        s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
            & 0xffffffff as libc::c_ulong;
        (*state).x[i as usize] = s;
        i += 1;
        i;
    }
    let mut msb: libc::c_ulong = 0x80000000 as libc::c_ulong;
    let mut mask: libc::c_ulong = 0xffffffff as libc::c_ulong;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut k: libc::c_int = 7 as libc::c_int * i + 3 as libc::c_int;
        (*state).x[k as usize] &= mask;
        (*state).x[k as usize] |= msb;
        mask >>= 1 as libc::c_int;
        msb >>= 1 as libc::c_int;
        i += 1;
        i;
    }
}
static mut r250_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"r250\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<r250_state_t>() as libc::c_ulong,
        set: Some(
            r250_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(r250_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            r250_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_r250: *const gsl_rng_type = unsafe {
    &r250_type as *const gsl_rng_type
};
