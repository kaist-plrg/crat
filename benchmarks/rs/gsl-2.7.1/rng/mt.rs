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
pub struct mt_state_t {
    pub mt: [libc::c_ulong; 624],
    pub mti: libc::c_int,
}
static mut UPPER_MASK: libc::c_ulong = 0x80000000 as libc::c_ulong;
static mut LOWER_MASK: libc::c_ulong = 0x7fffffff as libc::c_ulong;
#[inline]
unsafe extern "C" fn mt_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut k: libc::c_ulong = 0;
    let mt: *mut libc::c_ulong = ((*state).mt).as_mut_ptr();
    if (*state).mti >= 624 as libc::c_int {
        let mut kk: libc::c_int = 0;
        kk = 0 as libc::c_int;
        while kk < 624 as libc::c_int - 397 as libc::c_int {
            let mut y: libc::c_ulong = *mt.offset(kk as isize) & UPPER_MASK
                | *mt.offset((kk + 1 as libc::c_int) as isize) & LOWER_MASK;
            *mt
                .offset(
                    kk as isize,
                ) = *mt.offset((kk + 397 as libc::c_int) as isize)
                ^ y >> 1 as libc::c_int
                ^ (if y & 0x1 as libc::c_int as libc::c_ulong != 0 {
                    0x9908b0df as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                });
            kk += 1;
            kk;
        }
        while kk < 624 as libc::c_int - 1 as libc::c_int {
            let mut y_0: libc::c_ulong = *mt.offset(kk as isize) & UPPER_MASK
                | *mt.offset((kk + 1 as libc::c_int) as isize) & LOWER_MASK;
            *mt
                .offset(
                    kk as isize,
                ) = *mt.offset((kk + (397 as libc::c_int - 624 as libc::c_int)) as isize)
                ^ y_0 >> 1 as libc::c_int
                ^ (if y_0 & 0x1 as libc::c_int as libc::c_ulong != 0 {
                    0x9908b0df as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                });
            kk += 1;
            kk;
        }
        let mut y_1: libc::c_ulong = *mt
            .offset((624 as libc::c_int - 1 as libc::c_int) as isize) & UPPER_MASK
            | *mt.offset(0 as libc::c_int as isize) & LOWER_MASK;
        *mt
            .offset(
                (624 as libc::c_int - 1 as libc::c_int) as isize,
            ) = *mt.offset((397 as libc::c_int - 1 as libc::c_int) as isize)
            ^ y_1 >> 1 as libc::c_int
            ^ (if y_1 & 0x1 as libc::c_int as libc::c_ulong != 0 {
                0x9908b0df as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            });
        (*state).mti = 0 as libc::c_int;
    }
    k = *mt.offset((*state).mti as isize);
    k ^= k >> 11 as libc::c_int;
    k ^= k << 7 as libc::c_int & 0x9d2c5680 as libc::c_ulong;
    k ^= k << 15 as libc::c_int & 0xefc60000 as libc::c_ulong;
    k ^= k >> 18 as libc::c_int;
    (*state).mti += 1;
    (*state).mti;
    return k;
}
unsafe extern "C" fn mt_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return mt_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn mt_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 4357 as libc::c_int as libc::c_ulong;
    }
    (*state).mt[0 as libc::c_int as usize] = s & 0xffffffff as libc::c_ulong;
    i = 1 as libc::c_int;
    while i < 624 as libc::c_int {
        (*state)
            .mt[i
            as usize] = (1812433253 as libc::c_ulong)
            .wrapping_mul(
                (*state).mt[(i - 1 as libc::c_int) as usize]
                    ^ (*state).mt[(i - 1 as libc::c_int) as usize] >> 30 as libc::c_int,
            )
            .wrapping_add(i as libc::c_ulong);
        (*state).mt[i as usize] &= 0xffffffff as libc::c_ulong;
        i += 1;
        i;
    }
    (*state).mti = i;
}
unsafe extern "C" fn mt_1999_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 4357 as libc::c_int as libc::c_ulong;
    }
    i = 0 as libc::c_int;
    while i < 624 as libc::c_int {
        (*state).mt[i as usize] = s & 0xffff0000 as libc::c_ulong;
        s = (69069 as libc::c_int as libc::c_ulong)
            .wrapping_mul(s)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            & 0xffffffff as libc::c_ulong;
        (*state).mt[i as usize]
            |= (s & 0xffff0000 as libc::c_ulong) >> 16 as libc::c_int;
        s = (69069 as libc::c_int as libc::c_ulong)
            .wrapping_mul(s)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            & 0xffffffff as libc::c_ulong;
        i += 1;
        i;
    }
    (*state).mti = i;
}
unsafe extern "C" fn mt_1998_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 4357 as libc::c_int as libc::c_ulong;
    }
    (*state).mt[0 as libc::c_int as usize] = s & 0xffffffff as libc::c_ulong;
    i = 1 as libc::c_int;
    while i < 624 as libc::c_int {
        (*state)
            .mt[i
            as usize] = (69069 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*state).mt[(i - 1 as libc::c_int) as usize])
            & 0xffffffff as libc::c_ulong;
        i += 1;
        i;
    }
    (*state).mti = i;
}
static mut mt_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mt19937\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<mt_state_t>() as libc::c_ulong,
        set: Some(
            mt_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(mt_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            mt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut mt_1999_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mt19937_1999\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<mt_state_t>() as libc::c_ulong,
        set: Some(
            mt_1999_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(mt_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            mt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut mt_1998_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mt19937_1998\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<mt_state_t>() as libc::c_ulong,
        set: Some(
            mt_1998_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(mt_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            mt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_mt19937: *const gsl_rng_type = unsafe {
    &mt_type as *const gsl_rng_type
};
pub static mut gsl_rng_mt19937_1999: *const gsl_rng_type = unsafe {
    &mt_1999_type as *const gsl_rng_type
};
pub static mut gsl_rng_mt19937_1998: *const gsl_rng_type = unsafe {
    &mt_1998_type as *const gsl_rng_type
};
pub static mut gsl_rng_default: *const gsl_rng_type = unsafe {
    &mt_type as *const gsl_rng_type
};
pub static mut gsl_rng_default_seed: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
