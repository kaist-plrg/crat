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
pub struct random128_state_t {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub x: [libc::c_long; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random256_state_t {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub x: [libc::c_long; 63],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random32_state_t {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub x: [libc::c_long; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random64_state_t {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub x: [libc::c_long; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random8_state_t {
    pub x: libc::c_long,
}
#[inline]
unsafe extern "C" fn random8_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    (*state)
        .x = ((1103515245 as libc::c_int as libc::c_long * (*state).x
        + 12345 as libc::c_int as libc::c_long) as libc::c_ulong
        & 0x7fffffff as libc::c_ulong) as libc::c_long;
    return (*state).x as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn random_get(
    mut i: *mut libc::c_int,
    mut j: *mut libc::c_int,
    mut n: libc::c_int,
    mut x: *mut libc::c_long,
) -> libc::c_long {
    let mut k: libc::c_long = 0;
    *x.offset(*i as isize) += *x.offset(*j as isize);
    k = *x.offset(*i as isize) >> 1 as libc::c_int
        & 0x7fffffff as libc::c_int as libc::c_long;
    *i += 1;
    *i;
    if *i == n {
        *i = 0 as libc::c_int;
    }
    *j += 1;
    *j;
    if *j == n {
        *j = 0 as libc::c_int;
    }
    return k;
}
#[inline]
unsafe extern "C" fn random32_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut k: libc::c_ulong = random_get(
        &mut (*state).i,
        &mut (*state).j,
        7 as libc::c_int,
        ((*state).x).as_mut_ptr(),
    ) as libc::c_ulong;
    return k;
}
#[inline]
unsafe extern "C" fn random64_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut k: libc::c_long = random_get(
        &mut (*state).i,
        &mut (*state).j,
        15 as libc::c_int,
        ((*state).x).as_mut_ptr(),
    );
    return k as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn random128_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut k: libc::c_ulong = random_get(
        &mut (*state).i,
        &mut (*state).j,
        31 as libc::c_int,
        ((*state).x).as_mut_ptr(),
    ) as libc::c_ulong;
    return k;
}
#[inline]
unsafe extern "C" fn random256_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut k: libc::c_long = random_get(
        &mut (*state).i,
        &mut (*state).j,
        63 as libc::c_int,
        ((*state).x).as_mut_ptr(),
    );
    return k as libc::c_ulong;
}
unsafe extern "C" fn random8_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random8_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random32_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random32_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random64_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random64_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random128_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random128_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random256_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random256_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random8_bsd_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).x = s as libc::c_long;
}
unsafe extern "C" fn random32_bsd_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut i: libc::c_int = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 7 as libc::c_int, s);
    (*state).i = 3 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 7 as libc::c_int {
        random32_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random64_bsd_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut i: libc::c_int = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 15 as libc::c_int, s);
    (*state).i = 1 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 15 as libc::c_int {
        random64_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random128_bsd_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut i: libc::c_int = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 31 as libc::c_int, s);
    (*state).i = 3 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 31 as libc::c_int {
        random128_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random256_bsd_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut i: libc::c_int = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 63 as libc::c_int, s);
    (*state).i = 1 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 63 as libc::c_int {
        random256_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn bsd_initialize(
    mut x: *mut libc::c_long,
    mut n: libc::c_int,
    mut s: libc::c_ulong,
) {
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    *x.offset(0 as libc::c_int as isize) = s as libc::c_long;
    i = 1 as libc::c_int;
    while i < n {
        *x
            .offset(
                i as isize,
            ) = 1103515245 as libc::c_int as libc::c_long
            * *x.offset((i - 1 as libc::c_int) as isize)
            + 12345 as libc::c_int as libc::c_long;
        i += 1;
        i;
    }
}
unsafe extern "C" fn libc5_initialize(
    mut x: *mut libc::c_long,
    mut n: libc::c_int,
    mut s: libc::c_ulong,
) {
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    *x.offset(0 as libc::c_int as isize) = s as libc::c_long;
    i = 1 as libc::c_int;
    while i < n {
        *x
            .offset(
                i as isize,
            ) = 1103515145 as libc::c_int as libc::c_long
            * *x.offset((i - 1 as libc::c_int) as isize)
            + 12345 as libc::c_int as libc::c_long;
        i += 1;
        i;
    }
}
unsafe extern "C" fn glibc2_initialize(
    mut x: *mut libc::c_long,
    mut n: libc::c_int,
    mut s: libc::c_ulong,
) {
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    *x.offset(0 as libc::c_int as isize) = s as libc::c_long;
    i = 1 as libc::c_int;
    while i < n {
        let h: libc::c_long = s.wrapping_div(127773 as libc::c_int as libc::c_ulong)
            as libc::c_long;
        let t: libc::c_long = (16807 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                s
                    .wrapping_sub(
                        (h * 127773 as libc::c_int as libc::c_long) as libc::c_ulong,
                    ),
            )
            .wrapping_sub((h * 2836 as libc::c_int as libc::c_long) as libc::c_ulong)
            as libc::c_long;
        if t < 0 as libc::c_int as libc::c_long {
            s = (t + 2147483647 as libc::c_int as libc::c_long) as libc::c_ulong;
        } else {
            s = t as libc::c_ulong;
        }
        *x.offset(i as isize) = s as libc::c_long;
        i += 1;
        i;
    }
}
unsafe extern "C" fn random8_glibc2_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).x = s as libc::c_long;
}
unsafe extern "C" fn random32_glibc2_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut i: libc::c_int = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 7 as libc::c_int, s);
    (*state).i = 3 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 7 as libc::c_int {
        random32_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random64_glibc2_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut i: libc::c_int = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 15 as libc::c_int, s);
    (*state).i = 1 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 15 as libc::c_int {
        random64_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random128_glibc2_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut i: libc::c_int = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 31 as libc::c_int, s);
    (*state).i = 3 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 31 as libc::c_int {
        random128_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random256_glibc2_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut i: libc::c_int = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 63 as libc::c_int, s);
    (*state).i = 1 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 63 as libc::c_int {
        random256_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random8_libc5_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).x = s as libc::c_long;
}
unsafe extern "C" fn random32_libc5_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut i: libc::c_int = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 7 as libc::c_int, s);
    (*state).i = 3 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 7 as libc::c_int {
        random32_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random64_libc5_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut i: libc::c_int = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 15 as libc::c_int, s);
    (*state).i = 1 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 15 as libc::c_int {
        random64_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random128_libc5_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut i: libc::c_int = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 31 as libc::c_int, s);
    (*state).i = 3 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 31 as libc::c_int {
        random128_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random256_libc5_set(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
) {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut i: libc::c_int = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 63 as libc::c_int, s);
    (*state).i = 1 as libc::c_int;
    (*state).j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int * 63 as libc::c_int {
        random256_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
static mut random_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random-glibc2\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random128_state_t>() as libc::c_ulong,
        set: Some(
            random128_glibc2_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random128_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random8_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random8-glibc2\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random8_state_t>() as libc::c_ulong,
        set: Some(
            random8_glibc2_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random8_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random8_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random32_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random32-glibc2\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random32_state_t>() as libc::c_ulong,
        set: Some(
            random32_glibc2_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random32_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random32_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random64_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random64-glibc2\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random64_state_t>() as libc::c_ulong,
        set: Some(
            random64_glibc2_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random64_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random64_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random128_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random128-glibc2\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random128_state_t>() as libc::c_ulong,
        set: Some(
            random128_glibc2_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random128_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random256_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random256-glibc2\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random256_state_t>() as libc::c_ulong,
        set: Some(
            random256_glibc2_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random256_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random256_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random-libc5\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random128_state_t>() as libc::c_ulong,
        set: Some(
            random128_libc5_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random128_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random8_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random8-libc5\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random8_state_t>() as libc::c_ulong,
        set: Some(
            random8_libc5_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random8_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random8_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random32_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random32-libc5\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random32_state_t>() as libc::c_ulong,
        set: Some(
            random32_libc5_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random32_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random32_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random64_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random64-libc5\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random64_state_t>() as libc::c_ulong,
        set: Some(
            random64_libc5_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random64_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random64_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random128_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random128-libc5\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random128_state_t>() as libc::c_ulong,
        set: Some(
            random128_libc5_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random128_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random256_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random256-libc5\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random256_state_t>() as libc::c_ulong,
        set: Some(
            random256_libc5_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random256_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random256_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random-bsd\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random128_state_t>() as libc::c_ulong,
        set: Some(
            random128_bsd_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random128_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random8_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random8-bsd\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random8_state_t>() as libc::c_ulong,
        set: Some(
            random8_bsd_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random8_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random8_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random32_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random32-bsd\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random32_state_t>() as libc::c_ulong,
        set: Some(
            random32_bsd_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random32_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random32_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random64_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random64-bsd\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random64_state_t>() as libc::c_ulong,
        set: Some(
            random64_bsd_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random64_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random64_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random128_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random128-bsd\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random128_state_t>() as libc::c_ulong,
        set: Some(
            random128_bsd_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random128_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random256_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random256-bsd\0" as *const u8 as *const libc::c_char,
        max: 0x7fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<random256_state_t>() as libc::c_ulong,
        set: Some(
            random256_bsd_set
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            random256_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            random256_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_random_libc5: *const gsl_rng_type = unsafe {
    &random_libc5_type as *const gsl_rng_type
};
pub static mut gsl_rng_random8_libc5: *const gsl_rng_type = unsafe {
    &random8_libc5_type as *const gsl_rng_type
};
pub static mut gsl_rng_random32_libc5: *const gsl_rng_type = unsafe {
    &random32_libc5_type as *const gsl_rng_type
};
pub static mut gsl_rng_random64_libc5: *const gsl_rng_type = unsafe {
    &random64_libc5_type as *const gsl_rng_type
};
pub static mut gsl_rng_random128_libc5: *const gsl_rng_type = unsafe {
    &random128_libc5_type as *const gsl_rng_type
};
pub static mut gsl_rng_random256_libc5: *const gsl_rng_type = unsafe {
    &random256_libc5_type as *const gsl_rng_type
};
pub static mut gsl_rng_random_glibc2: *const gsl_rng_type = unsafe {
    &random_glibc2_type as *const gsl_rng_type
};
pub static mut gsl_rng_random8_glibc2: *const gsl_rng_type = unsafe {
    &random8_glibc2_type as *const gsl_rng_type
};
pub static mut gsl_rng_random32_glibc2: *const gsl_rng_type = unsafe {
    &random32_glibc2_type as *const gsl_rng_type
};
pub static mut gsl_rng_random64_glibc2: *const gsl_rng_type = unsafe {
    &random64_glibc2_type as *const gsl_rng_type
};
pub static mut gsl_rng_random128_glibc2: *const gsl_rng_type = unsafe {
    &random128_glibc2_type as *const gsl_rng_type
};
pub static mut gsl_rng_random256_glibc2: *const gsl_rng_type = unsafe {
    &random256_glibc2_type as *const gsl_rng_type
};
pub static mut gsl_rng_random_bsd: *const gsl_rng_type = unsafe {
    &random_bsd_type as *const gsl_rng_type
};
pub static mut gsl_rng_random8_bsd: *const gsl_rng_type = unsafe {
    &random8_bsd_type as *const gsl_rng_type
};
pub static mut gsl_rng_random32_bsd: *const gsl_rng_type = unsafe {
    &random32_bsd_type as *const gsl_rng_type
};
pub static mut gsl_rng_random64_bsd: *const gsl_rng_type = unsafe {
    &random64_bsd_type as *const gsl_rng_type
};
pub static mut gsl_rng_random128_bsd: *const gsl_rng_type = unsafe {
    &random128_bsd_type as *const gsl_rng_type
};
pub static mut gsl_rng_random256_bsd: *const gsl_rng_type = unsafe {
    &random256_bsd_type as *const gsl_rng_type
};
