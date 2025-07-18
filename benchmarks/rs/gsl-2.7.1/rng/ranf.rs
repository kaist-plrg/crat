use ::libc;
extern "C" {
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
}
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
pub struct ranf_state_t {
    pub x0: libc::c_ushort,
    pub x1: libc::c_ushort,
    pub x2: libc::c_ushort,
}
static mut a0: libc::c_ushort = 0xb175 as libc::c_int as libc::c_ushort;
static mut a1: libc::c_ushort = 0xa2e7 as libc::c_int as libc::c_ushort;
static mut a2: libc::c_ushort = 0x2875 as libc::c_int as libc::c_ushort;
#[inline]
unsafe extern "C" fn ranf_advance(mut vstate: *mut libc::c_void) {
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    let x0: libc::c_ulong = (*state).x0 as libc::c_ulong;
    let x1: libc::c_ulong = (*state).x1 as libc::c_ulong;
    let x2: libc::c_ulong = (*state).x2 as libc::c_ulong;
    let mut r: libc::c_ulong = 0;
    r = (a0 as libc::c_ulong).wrapping_mul(x0);
    (*state).x0 = (r & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
    r >>= 16 as libc::c_int;
    r = r
        .wrapping_add(
            (a0 as libc::c_ulong)
                .wrapping_mul(x1)
                .wrapping_add((a1 as libc::c_ulong).wrapping_mul(x0)),
        );
    (*state).x1 = (r & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
    r >>= 16 as libc::c_int;
    r = r
        .wrapping_add(
            (a0 as libc::c_ulong)
                .wrapping_mul(x2)
                .wrapping_add((a1 as libc::c_ulong).wrapping_mul(x1))
                .wrapping_add((a2 as libc::c_ulong).wrapping_mul(x0)),
        );
    (*state).x2 = (r & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
}
unsafe extern "C" fn ranf_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut x1: libc::c_ulong = 0;
    let mut x2: libc::c_ulong = 0;
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    ranf_advance(state as *mut libc::c_void);
    x1 = (*state).x1 as libc::c_ulong;
    x2 = (*state).x2 as libc::c_ulong;
    return (x2 << 16 as libc::c_int).wrapping_add(x1);
}
unsafe extern "C" fn ranf_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    ranf_advance(state as *mut libc::c_void);
    return ldexp((*state).x2 as libc::c_double, -(16 as libc::c_int))
        + ldexp((*state).x1 as libc::c_double, -(32 as libc::c_int))
        + ldexp((*state).x0 as libc::c_double, -(48 as libc::c_int));
}
unsafe extern "C" fn ranf_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    let mut x0: libc::c_ushort = 0;
    let mut x1: libc::c_ushort = 0;
    let mut x2: libc::c_ushort = 0;
    let mut r: libc::c_ulong = 0;
    let mut b0: libc::c_ulong = 0xd6dd as libc::c_int as libc::c_ulong;
    let mut b1: libc::c_ulong = 0xb894 as libc::c_int as libc::c_ulong;
    let mut b2: libc::c_ulong = 0x5cee as libc::c_int as libc::c_ulong;
    if s == 0 as libc::c_int as libc::c_ulong {
        x0 = 0x9cd1 as libc::c_int as libc::c_ushort;
        x1 = 0x53fc as libc::c_int as libc::c_ushort;
        x2 = 0x9482 as libc::c_int as libc::c_ushort;
    } else {
        x0 = ((s | 1 as libc::c_int as libc::c_ulong)
            & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
        x1 = (s >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong)
            as libc::c_ushort;
        x2 = 0 as libc::c_int as libc::c_ushort;
    }
    r = b0.wrapping_mul(x0 as libc::c_ulong);
    (*state).x0 = (r & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
    r >>= 16 as libc::c_int;
    r = r
        .wrapping_add(
            b0
                .wrapping_mul(x1 as libc::c_ulong)
                .wrapping_add(b1.wrapping_mul(x0 as libc::c_ulong)),
        );
    (*state).x1 = (r & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
    r >>= 16 as libc::c_int;
    r = r
        .wrapping_add(
            b0
                .wrapping_mul(x2 as libc::c_ulong)
                .wrapping_add(b1.wrapping_mul(x1 as libc::c_ulong))
                .wrapping_add(b2.wrapping_mul(x0 as libc::c_ulong)),
        );
    (*state).x2 = (r & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
}
static mut ranf_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranf\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ranf_state_t>() as libc::c_ulong,
        set: Some(
            ranf_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(ranf_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            ranf_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ranf: *const gsl_rng_type = unsafe {
    &ranf_type as *const gsl_rng_type
};
