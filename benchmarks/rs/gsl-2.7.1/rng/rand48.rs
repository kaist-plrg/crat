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
pub struct rand48_state_t {
    pub x0: libc::c_ushort,
    pub x1: libc::c_ushort,
    pub x2: libc::c_ushort,
}
static mut a0: libc::c_ushort = 0xe66d as libc::c_int as libc::c_ushort;
static mut a1: libc::c_ushort = 0xdeec as libc::c_int as libc::c_ushort;
static mut a2: libc::c_ushort = 0x5 as libc::c_int as libc::c_ushort;
static mut c0: libc::c_ushort = 0xb as libc::c_int as libc::c_ushort;
#[inline]
unsafe extern "C" fn rand48_advance(mut vstate: *mut libc::c_void) {
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    let x0: libc::c_ulong = (*state).x0 as libc::c_ulong;
    let x1: libc::c_ulong = (*state).x1 as libc::c_ulong;
    let x2: libc::c_ulong = (*state).x2 as libc::c_ulong;
    let mut a: libc::c_ulong = 0;
    a = (a0 as libc::c_ulong).wrapping_mul(x0).wrapping_add(c0 as libc::c_ulong);
    (*state).x0 = (a & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
    a >>= 16 as libc::c_int;
    a = a
        .wrapping_add(
            (a0 as libc::c_ulong)
                .wrapping_mul(x1)
                .wrapping_add((a1 as libc::c_ulong).wrapping_mul(x0)),
        );
    (*state).x1 = (a & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
    a >>= 16 as libc::c_int;
    a = a
        .wrapping_add(
            (a0 as libc::c_ulong)
                .wrapping_mul(x2)
                .wrapping_add((a1 as libc::c_ulong).wrapping_mul(x1))
                .wrapping_add((a2 as libc::c_ulong).wrapping_mul(x0)),
        );
    (*state).x2 = (a & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
}
unsafe extern "C" fn rand48_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut x1: libc::c_ulong = 0;
    let mut x2: libc::c_ulong = 0;
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    rand48_advance(state as *mut libc::c_void);
    x2 = (*state).x2 as libc::c_ulong;
    x1 = (*state).x1 as libc::c_ulong;
    return (x2 << 16 as libc::c_int).wrapping_add(x1);
}
unsafe extern "C" fn rand48_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    rand48_advance(state as *mut libc::c_void);
    return ldexp((*state).x2 as libc::c_double, -(16 as libc::c_int))
        + ldexp((*state).x1 as libc::c_double, -(32 as libc::c_int))
        + ldexp((*state).x0 as libc::c_double, -(48 as libc::c_int));
}
unsafe extern "C" fn rand48_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        (*state).x0 = 0x330e as libc::c_int as libc::c_ushort;
        (*state).x1 = 0xabcd as libc::c_int as libc::c_ushort;
        (*state).x2 = 0x1234 as libc::c_int as libc::c_ushort;
    } else {
        (*state).x0 = 0x330e as libc::c_int as libc::c_ushort;
        (*state).x1 = (s & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
        (*state)
            .x2 = (s >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong)
            as libc::c_ushort;
    };
}
static mut rand48_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"rand48\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<rand48_state_t>() as libc::c_ulong,
        set: Some(
            rand48_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            rand48_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            rand48_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_rand48: *const gsl_rng_type = unsafe {
    &rand48_type as *const gsl_rng_type
};
