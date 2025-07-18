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
pub struct ran3_state_t {
    pub x: libc::c_uint,
    pub y: libc::c_uint,
    pub buffer: [libc::c_ulong; 56],
}
#[inline]
unsafe extern "C" fn ran3_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran3_state_t = vstate as *mut ran3_state_t;
    let mut j: libc::c_long = 0;
    (*state).x = ((*state).x).wrapping_add(1);
    (*state).x;
    if (*state).x == 56 as libc::c_int as libc::c_uint {
        (*state).x = 1 as libc::c_int as libc::c_uint;
    }
    (*state).y = ((*state).y).wrapping_add(1);
    (*state).y;
    if (*state).y == 56 as libc::c_int as libc::c_uint {
        (*state).y = 1 as libc::c_int as libc::c_uint;
    }
    j = ((*state).buffer[(*state).x as usize])
        .wrapping_sub((*state).buffer[(*state).y as usize]) as libc::c_long;
    if j < 0 as libc::c_int as libc::c_long {
        j += 1000000000 as libc::c_int as libc::c_long;
    }
    (*state).buffer[(*state).x as usize] = j as libc::c_ulong;
    return j as libc::c_ulong;
}
unsafe extern "C" fn ran3_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ran3_get(vstate) as libc::c_double
        / 1000000000 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn ran3_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran3_state_t = vstate as *mut ran3_state_t;
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    j = (161803398 as libc::c_int as libc::c_ulong)
        .wrapping_sub(s)
        .wrapping_rem(1000000000 as libc::c_int as libc::c_ulong) as libc::c_long;
    if j < 0 as libc::c_int as libc::c_long {
        j += 1000000000 as libc::c_int as libc::c_long;
    }
    (*state).buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ulong;
    (*state).buffer[55 as libc::c_int as usize] = j as libc::c_ulong;
    k = 1 as libc::c_int as libc::c_long;
    i = 1 as libc::c_int;
    while i < 55 as libc::c_int {
        let mut n: libc::c_int = 21 as libc::c_int * i % 55 as libc::c_int;
        (*state).buffer[n as usize] = k as libc::c_ulong;
        k = j - k;
        if k < 0 as libc::c_int as libc::c_long {
            k += 1000000000 as libc::c_int as libc::c_long;
        }
        j = (*state).buffer[n as usize] as libc::c_long;
        i += 1;
        i;
    }
    i1 = 0 as libc::c_int;
    while i1 < 4 as libc::c_int {
        i = 1 as libc::c_int;
        while i < 56 as libc::c_int {
            let mut t: libc::c_long = ((*state).buffer[i as usize])
                .wrapping_sub(
                    (*state)
                        .buffer[(1 as libc::c_int
                        + (i + 30 as libc::c_int) % 55 as libc::c_int) as usize],
                ) as libc::c_long;
            if t < 0 as libc::c_int as libc::c_long {
                t += 1000000000 as libc::c_int as libc::c_long;
            }
            (*state).buffer[i as usize] = t as libc::c_ulong;
            i += 1;
            i;
        }
        i1 += 1;
        i1;
    }
    (*state).x = 0 as libc::c_int as libc::c_uint;
    (*state).y = 31 as libc::c_int as libc::c_uint;
}
static mut ran3_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran3\0" as *const u8 as *const libc::c_char,
        max: 1000000000 as libc::c_int as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ran3_state_t>() as libc::c_ulong,
        set: Some(
            ran3_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(ran3_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            ran3_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ran3: *const gsl_rng_type = unsafe {
    &ran3_type as *const gsl_rng_type
};
