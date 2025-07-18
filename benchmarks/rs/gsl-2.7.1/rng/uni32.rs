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
pub struct uni32_state_t {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub m: [libc::c_ulong; 17],
}
static mut m1: libc::c_ulong = 2147483647 as libc::c_int as libc::c_ulong;
static mut m2: libc::c_ulong = 65536 as libc::c_int as libc::c_ulong;
#[inline]
unsafe extern "C" fn uni32_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut uni32_state_t = vstate as *mut uni32_state_t;
    let i: libc::c_long = (*state).i as libc::c_long;
    let j: libc::c_long = (*state).j as libc::c_long;
    let mut k: libc::c_long = ((*state).m[i as usize])
        .wrapping_sub((*state).m[j as usize]) as libc::c_long;
    if k < 0 as libc::c_int as libc::c_long {
        k = (k as libc::c_ulong).wrapping_add(m1) as libc::c_long as libc::c_long;
    }
    (*state).m[j as usize] = k as libc::c_ulong;
    if i == 0 as libc::c_int as libc::c_long {
        (*state).i = 16 as libc::c_int;
    } else {
        (*state).i -= 1;
        (*state).i;
    }
    if j == 0 as libc::c_int as libc::c_long {
        (*state).j = 16 as libc::c_int;
    } else {
        (*state).j -= 1;
        (*state).j;
    }
    return k as libc::c_ulong;
}
unsafe extern "C" fn uni32_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return uni32_get(vstate) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn uni32_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut seed: libc::c_long = 0;
    let mut k0: libc::c_long = 0;
    let mut k1: libc::c_long = 0;
    let mut j0: libc::c_long = 0;
    let mut j1: libc::c_long = 0;
    let mut i: libc::c_int = 0;
    let mut state: *mut uni32_state_t = vstate as *mut uni32_state_t;
    seed = (if s < m1 { s } else { m1 }) as libc::c_long;
    seed
        -= (if seed % 2 as libc::c_int as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_long;
    k0 = (9069 as libc::c_int as libc::c_ulong).wrapping_rem(m2) as libc::c_long;
    k1 = (9069 as libc::c_int as libc::c_ulong).wrapping_div(m2) as libc::c_long;
    j0 = (seed as libc::c_ulong).wrapping_rem(m2) as libc::c_long;
    j1 = (seed as libc::c_ulong).wrapping_div(m2) as libc::c_long;
    i = 0 as libc::c_int;
    while i < 17 as libc::c_int {
        seed = j0 * k0;
        j1 = (seed as libc::c_ulong)
            .wrapping_div(m2)
            .wrapping_add((j0 * k1) as libc::c_ulong)
            .wrapping_add((j1 * k0) as libc::c_ulong)
            .wrapping_rem(m2.wrapping_div(2 as libc::c_int as libc::c_ulong))
            as libc::c_long;
        j0 = (seed as libc::c_ulong).wrapping_rem(m2) as libc::c_long;
        (*state)
            .m[i
            as usize] = (j0 as libc::c_ulong)
            .wrapping_add(m2.wrapping_mul(j1 as libc::c_ulong));
        i += 1;
        i;
    }
    (*state).i = 4 as libc::c_int;
    (*state).j = 16 as libc::c_int;
}
static mut uni32_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"uni32\0" as *const u8 as *const libc::c_char,
        max: 2147483646 as libc::c_int as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<uni32_state_t>() as libc::c_ulong,
        set: Some(
            uni32_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(uni32_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            uni32_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_uni32: *const gsl_rng_type = unsafe {
    &uni32_type as *const gsl_rng_type
};
