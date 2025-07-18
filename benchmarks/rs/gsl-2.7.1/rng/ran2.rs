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
pub struct ran2_state_t {
    pub x: libc::c_ulong,
    pub y: libc::c_ulong,
    pub n: libc::c_ulong,
    pub shuffle: [libc::c_ulong; 32],
}
static mut m1: libc::c_long = 2147483563 as libc::c_int as libc::c_long;
static mut a1: libc::c_long = 40014 as libc::c_int as libc::c_long;
static mut q1: libc::c_long = 53668 as libc::c_int as libc::c_long;
static mut r1: libc::c_long = 12211 as libc::c_int as libc::c_long;
static mut m2: libc::c_long = 2147483399 as libc::c_int as libc::c_long;
static mut a2: libc::c_long = 40692 as libc::c_int as libc::c_long;
static mut q2: libc::c_long = 52774 as libc::c_int as libc::c_long;
static mut r2: libc::c_long = 3791 as libc::c_int as libc::c_long;
#[inline]
unsafe extern "C" fn ran2_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran2_state_t = vstate as *mut ran2_state_t;
    let x: libc::c_ulong = (*state).x;
    let y: libc::c_ulong = (*state).y;
    let mut h1: libc::c_long = x.wrapping_div(q1 as libc::c_ulong) as libc::c_long;
    let mut t1: libc::c_long = (a1 as libc::c_ulong)
        .wrapping_mul(x.wrapping_sub((h1 * q1) as libc::c_ulong))
        .wrapping_sub((h1 * r1) as libc::c_ulong) as libc::c_long;
    let mut h2: libc::c_long = y.wrapping_div(q2 as libc::c_ulong) as libc::c_long;
    let mut t2: libc::c_long = (a2 as libc::c_ulong)
        .wrapping_mul(y.wrapping_sub((h2 * q2) as libc::c_ulong))
        .wrapping_sub((h2 * r2) as libc::c_ulong) as libc::c_long;
    if t1 < 0 as libc::c_int as libc::c_long {
        t1 += m1;
    }
    if t2 < 0 as libc::c_int as libc::c_long {
        t2 += m2;
    }
    (*state).x = t1 as libc::c_ulong;
    (*state).y = t2 as libc::c_ulong;
    let mut j: libc::c_ulong = ((*state).n)
        .wrapping_div(
            (1 as libc::c_int + 2147483562 as libc::c_int / 32 as libc::c_int)
                as libc::c_ulong,
        );
    let mut delta: libc::c_long = ((*state).shuffle[j as usize])
        .wrapping_sub(t2 as libc::c_ulong) as libc::c_long;
    if delta < 1 as libc::c_int as libc::c_long {
        delta += m1 - 1 as libc::c_int as libc::c_long;
    }
    (*state).n = delta as libc::c_ulong;
    (*state).shuffle[j as usize] = t1 as libc::c_ulong;
    return (*state).n;
}
unsafe extern "C" fn ran2_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut x_max: libc::c_float = 1 as libc::c_int as libc::c_float - 1.2e-7f32;
    let mut x: libc::c_float = ran2_get(vstate) as libc::c_float / 2147483563.0f32;
    if x > x_max {
        return x_max as libc::c_double;
    }
    return x as libc::c_double;
}
unsafe extern "C" fn ran2_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran2_state_t = vstate as *mut ran2_state_t;
    let mut i: libc::c_int = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).y = s;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut h: libc::c_long = s.wrapping_div(q1 as libc::c_ulong) as libc::c_long;
        let mut t: libc::c_long = (a1 as libc::c_ulong)
            .wrapping_mul(s.wrapping_sub((h * q1) as libc::c_ulong))
            .wrapping_sub((h * r1) as libc::c_ulong) as libc::c_long;
        if t < 0 as libc::c_int as libc::c_long {
            t += m1;
        }
        s = t as libc::c_ulong;
        i += 1;
        i;
    }
    i = 32 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut h_0: libc::c_long = s.wrapping_div(q1 as libc::c_ulong) as libc::c_long;
        let mut t_0: libc::c_long = (a1 as libc::c_ulong)
            .wrapping_mul(s.wrapping_sub((h_0 * q1) as libc::c_ulong))
            .wrapping_sub((h_0 * r1) as libc::c_ulong) as libc::c_long;
        if t_0 < 0 as libc::c_int as libc::c_long {
            t_0 += m1;
        }
        s = t_0 as libc::c_ulong;
        (*state).shuffle[i as usize] = s;
        i -= 1;
        i;
    }
    (*state).x = s;
    (*state).n = s;
}
static mut ran2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran2\0" as *const u8 as *const libc::c_char,
        max: 2147483562 as libc::c_int as libc::c_ulong,
        min: 1 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ran2_state_t>() as libc::c_ulong,
        set: Some(
            ran2_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(ran2_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            ran2_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ran2: *const gsl_rng_type = unsafe {
    &ran2_type as *const gsl_rng_type
};
