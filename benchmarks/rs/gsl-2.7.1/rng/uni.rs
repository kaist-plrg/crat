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
pub struct uni_state_t {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub m: [libc::c_ulong; 17],
}
static mut m1: libc::c_uint = 32767 as libc::c_int as libc::c_uint;
static mut m2: libc::c_uint = 256 as libc::c_int as libc::c_uint;
#[inline]
unsafe extern "C" fn uni_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut uni_state_t = vstate as *mut uni_state_t;
    let i: libc::c_int = (*state).i;
    let j: libc::c_int = (*state).j;
    let mut k: libc::c_long = ((*state).m[i as usize])
        .wrapping_sub((*state).m[j as usize]) as libc::c_long;
    if k < 0 as libc::c_int as libc::c_long {
        k += m1 as libc::c_long;
    }
    (*state).m[j as usize] = k as libc::c_ulong;
    if i == 0 as libc::c_int {
        (*state).i = 16 as libc::c_int;
    } else {
        (*state).i -= 1;
        (*state).i;
    }
    if j == 0 as libc::c_int {
        (*state).j = 16 as libc::c_int;
    } else {
        (*state).j -= 1;
        (*state).j;
    }
    return k as libc::c_ulong;
}
unsafe extern "C" fn uni_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return uni_get(vstate) as libc::c_double / 32767.0f64;
}
unsafe extern "C" fn uni_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut i: libc::c_uint = 0;
    let mut seed: libc::c_uint = 0;
    let mut k0: libc::c_uint = 0;
    let mut k1: libc::c_uint = 0;
    let mut j0: libc::c_uint = 0;
    let mut j1: libc::c_uint = 0;
    let mut state: *mut uni_state_t = vstate as *mut uni_state_t;
    s = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(s)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    seed = (if s < m1 as libc::c_ulong { s } else { m1 as libc::c_ulong })
        as libc::c_uint;
    k0 = (9069 as libc::c_int as libc::c_uint).wrapping_rem(m2);
    k1 = (9069 as libc::c_int as libc::c_uint).wrapping_div(m2);
    j0 = seed.wrapping_rem(m2);
    j1 = seed.wrapping_div(m2);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 17 as libc::c_int as libc::c_uint {
        seed = j0.wrapping_mul(k0);
        j1 = seed
            .wrapping_div(m2)
            .wrapping_add(j0.wrapping_mul(k1))
            .wrapping_add(j1.wrapping_mul(k0))
            .wrapping_rem(m2.wrapping_div(2 as libc::c_int as libc::c_uint));
        j0 = seed.wrapping_rem(m2);
        (*state).m[i as usize] = j0.wrapping_add(m2.wrapping_mul(j1)) as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    (*state).i = 4 as libc::c_int;
    (*state).j = 16 as libc::c_int;
}
static mut uni_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"uni\0" as *const u8 as *const libc::c_char,
        max: 32766 as libc::c_int as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<uni_state_t>() as libc::c_ulong,
        set: Some(
            uni_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(uni_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            uni_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_uni: *const gsl_rng_type = unsafe {
    &uni_type as *const gsl_rng_type
};
