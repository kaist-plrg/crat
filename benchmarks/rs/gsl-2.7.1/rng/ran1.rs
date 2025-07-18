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
pub struct ran1_state_t {
    pub x: libc::c_ulong,
    pub n: libc::c_ulong,
    pub shuffle: [libc::c_ulong; 32],
}
static mut m: libc::c_long = 2147483647 as libc::c_int as libc::c_long;
static mut a: libc::c_long = 16807 as libc::c_int as libc::c_long;
static mut q: libc::c_long = 127773 as libc::c_int as libc::c_long;
static mut r: libc::c_long = 2836 as libc::c_int as libc::c_long;
#[inline]
unsafe extern "C" fn ran1_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran1_state_t = vstate as *mut ran1_state_t;
    let x: libc::c_ulong = (*state).x;
    let h: libc::c_long = x.wrapping_div(q as libc::c_ulong) as libc::c_long;
    let t: libc::c_long = (a as libc::c_ulong)
        .wrapping_mul(x.wrapping_sub((h * q) as libc::c_ulong))
        .wrapping_sub((h * r) as libc::c_ulong) as libc::c_long;
    if t < 0 as libc::c_int as libc::c_long {
        (*state).x = (t + m) as libc::c_ulong;
    } else {
        (*state).x = t as libc::c_ulong;
    }
    let mut j: libc::c_ulong = ((*state).n)
        .wrapping_div(
            (1 as libc::c_int + 2147483646 as libc::c_int / 32 as libc::c_int)
                as libc::c_ulong,
        );
    (*state).n = (*state).shuffle[j as usize];
    (*state).shuffle[j as usize] = (*state).x;
    return (*state).n;
}
unsafe extern "C" fn ran1_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut x_max: libc::c_float = 1 as libc::c_int as libc::c_float - 1.2e-7f32;
    let mut x: libc::c_float = ran1_get(vstate) as libc::c_float / 2147483647.0f32;
    if x > x_max {
        return x_max as libc::c_double;
    }
    return x as libc::c_double;
}
unsafe extern "C" fn ran1_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran1_state_t = vstate as *mut ran1_state_t;
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut h: libc::c_long = s.wrapping_div(q as libc::c_ulong) as libc::c_long;
        let mut t: libc::c_long = (a as libc::c_ulong)
            .wrapping_mul(s.wrapping_sub((h * q) as libc::c_ulong))
            .wrapping_sub((h * r) as libc::c_ulong) as libc::c_long;
        if t < 0 as libc::c_int as libc::c_long {
            t += m;
        }
        s = t as libc::c_ulong;
        i += 1;
        i;
    }
    i = 32 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut h_0: libc::c_long = s.wrapping_div(q as libc::c_ulong) as libc::c_long;
        let mut t_0: libc::c_long = (a as libc::c_ulong)
            .wrapping_mul(s.wrapping_sub((h_0 * q) as libc::c_ulong))
            .wrapping_sub((h_0 * r) as libc::c_ulong) as libc::c_long;
        if t_0 < 0 as libc::c_int as libc::c_long {
            t_0 += m;
        }
        s = t_0 as libc::c_ulong;
        (*state).shuffle[i as usize] = s;
        i -= 1;
        i;
    }
    (*state).x = s;
    (*state).n = s;
}
static mut ran1_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran1\0" as *const u8 as *const libc::c_char,
        max: 2147483646 as libc::c_int as libc::c_ulong,
        min: 1 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ran1_state_t>() as libc::c_ulong,
        set: Some(
            ran1_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(ran1_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            ran1_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ran1: *const gsl_rng_type = unsafe {
    &ran1_type as *const gsl_rng_type
};
