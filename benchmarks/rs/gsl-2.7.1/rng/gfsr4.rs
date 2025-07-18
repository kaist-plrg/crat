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
pub struct gfsr4_state_t {
    pub nd: libc::c_int,
    pub ra: [libc::c_ulong; 16384],
}
#[inline]
unsafe extern "C" fn gfsr4_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut gfsr4_state_t = vstate as *mut gfsr4_state_t;
    (*state).nd = (*state).nd + 1 as libc::c_int & 16383 as libc::c_int;
    (*state)
        .ra[(*state).nd
        as usize] = (*state)
        .ra[((*state).nd + (16383 as libc::c_int + 1 as libc::c_int - 471 as libc::c_int)
        & 16383 as libc::c_int) as usize]
        ^ (*state)
            .ra[((*state).nd
            + (16383 as libc::c_int + 1 as libc::c_int - 1586 as libc::c_int)
            & 16383 as libc::c_int) as usize]
        ^ (*state)
            .ra[((*state).nd
            + (16383 as libc::c_int + 1 as libc::c_int - 6988 as libc::c_int)
            & 16383 as libc::c_int) as usize]
        ^ (*state)
            .ra[((*state).nd
            + (16383 as libc::c_int + 1 as libc::c_int - 9689 as libc::c_int)
            & 16383 as libc::c_int) as usize];
    return (*state).ra[(*state).nd as usize];
}
unsafe extern "C" fn gfsr4_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return gfsr4_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn gfsr4_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut gfsr4_state_t = vstate as *mut gfsr4_state_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut msb: libc::c_ulong = 0x80000000 as libc::c_ulong;
    let mut mask: libc::c_ulong = 0xffffffff as libc::c_ulong;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 4357 as libc::c_int as libc::c_ulong;
    }
    i = 0 as libc::c_int;
    while i <= 16383 as libc::c_int {
        let mut t: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut bit: libc::c_ulong = msb;
        j = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            s = (69069 as libc::c_int as libc::c_ulong).wrapping_mul(s)
                & 0xffffffff as libc::c_ulong;
            if s & msb != 0 {
                t |= bit;
            }
            bit >>= 1 as libc::c_int;
            j += 1;
            j;
        }
        (*state).ra[i as usize] = t;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut k: libc::c_int = 7 as libc::c_int + i * 3 as libc::c_int;
        (*state).ra[k as usize] &= mask;
        (*state).ra[k as usize] |= msb;
        mask >>= 1 as libc::c_int;
        msb >>= 1 as libc::c_int;
        i += 1;
        i;
    }
    (*state).nd = i;
}
static mut gfsr4_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"gfsr4\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<gfsr4_state_t>() as libc::c_ulong,
        set: Some(
            gfsr4_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(gfsr4_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            gfsr4_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_gfsr4: *const gsl_rng_type = unsafe {
    &gfsr4_type as *const gsl_rng_type
};
