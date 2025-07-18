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
pub struct tt_state_t {
    pub n: libc::c_int,
    pub x: [libc::c_ulong; 25],
}
#[inline]
unsafe extern "C" fn tt_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut tt_state_t = vstate as *mut tt_state_t;
    let mag01: [libc::c_ulong; 2] = [
        0 as libc::c_int as libc::c_ulong,
        0x8ebfd028 as libc::c_ulong,
    ];
    let mut y: libc::c_ulong = 0;
    let x: *mut libc::c_ulong = ((*state).x).as_mut_ptr();
    let mut n: libc::c_int = (*state).n;
    if n >= 25 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 25 as libc::c_int - 7 as libc::c_int {
            *x
                .offset(
                    i as isize,
                ) = *x.offset((i + 7 as libc::c_int) as isize)
                ^ *x.offset(i as isize) >> 1 as libc::c_int
                ^ mag01[(*x.offset(i as isize))
                    .wrapping_rem(2 as libc::c_int as libc::c_ulong) as usize];
            i += 1;
            i;
        }
        while i < 25 as libc::c_int {
            *x
                .offset(
                    i as isize,
                ) = *x.offset((i + (7 as libc::c_int - 25 as libc::c_int)) as isize)
                ^ *x.offset(i as isize) >> 1 as libc::c_int
                ^ mag01[(*x.offset(i as isize))
                    .wrapping_rem(2 as libc::c_int as libc::c_ulong) as usize];
            i += 1;
            i;
        }
        n = 0 as libc::c_int;
    }
    y = *x.offset(n as isize);
    y ^= y << 7 as libc::c_int & 0x2b5b2500 as libc::c_ulong;
    y ^= y << 15 as libc::c_int & 0xdb8b0000 as libc::c_ulong;
    y &= 0xffffffff as libc::c_ulong;
    y ^= y >> 16 as libc::c_int;
    (*state).n = n + 1 as libc::c_int;
    return y;
}
unsafe extern "C" fn tt_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return tt_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn tt_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut tt_state_t = vstate as *mut tt_state_t;
    let init_state: tt_state_t = {
        let mut init = tt_state_t {
            n: 0 as libc::c_int,
            x: [
                0x95f24dab as libc::c_ulong,
                0xb685215 as libc::c_ulong,
                0xe76ccae7 as libc::c_ulong,
                0xaf3ec239 as libc::c_ulong,
                0x715fad23 as libc::c_ulong,
                0x24a590ad as libc::c_ulong,
                0x69e4b5ef as libc::c_ulong,
                0xbf456141 as libc::c_ulong,
                0x96bc1b7b as libc::c_ulong,
                0xa7bdf825 as libc::c_ulong,
                0xc1de75b7 as libc::c_ulong,
                0x8858a9c9 as libc::c_ulong,
                0x2da87693 as libc::c_ulong,
                0xb657f9dd as libc::c_ulong,
                0xffdc8a9f as libc::c_ulong,
                0x8121da71 as libc::c_ulong,
                0x8b823ecb as libc::c_ulong,
                0x885d05f5 as libc::c_ulong,
                0x4e20cd47 as libc::c_ulong,
                0x5a9ad5d9 as libc::c_ulong,
                0x512c0c03 as libc::c_ulong,
                0xea857ccd as libc::c_ulong,
                0x4cc1d30f as libc::c_ulong,
                0x8891a8a1 as libc::c_ulong,
                0xa6b7aadb as libc::c_ulong,
            ],
        };
        init
    };
    if s == 0 as libc::c_int as libc::c_ulong {
        *state = init_state;
    } else {
        let mut i: libc::c_int = 0;
        (*state).n = 0 as libc::c_int;
        (*state).x[0 as libc::c_int as usize] = s & 0xffffffff as libc::c_ulong;
        i = 1 as libc::c_int;
        while i < 25 as libc::c_int {
            (*state)
                .x[i
                as usize] = (69069 as libc::c_int as libc::c_ulong)
                .wrapping_mul((*state).x[(i - 1 as libc::c_int) as usize])
                & 0xffffffff as libc::c_ulong;
            i += 1;
            i;
        }
    };
}
static mut tt_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"tt800\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<tt_state_t>() as libc::c_ulong,
        set: Some(
            tt_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(tt_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            tt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_tt800: *const gsl_rng_type = unsafe {
    &tt_type as *const gsl_rng_type
};
