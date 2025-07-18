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
pub struct ran_state_t {
    pub x: libc::c_ulong,
}
#[inline]
unsafe extern "C" fn schrage(
    mut a: libc::c_ulong,
    mut b: libc::c_ulong,
    mut m: libc::c_ulong,
) -> libc::c_ulong {
    let mut q: libc::c_ulong = 0;
    let mut t: libc::c_ulong = 0;
    if a == 0 as libc::c_ulong {
        return 0 as libc::c_ulong;
    }
    q = m.wrapping_div(a);
    t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(m)
        .wrapping_sub(m.wrapping_rem(a).wrapping_mul(b.wrapping_div(q)));
    if t >= m {
        t = t.wrapping_sub(m);
    }
    t = t.wrapping_add(a.wrapping_mul(b.wrapping_rem(q)));
    return if t >= m { t.wrapping_sub(m) } else { t };
}
#[inline]
unsafe extern "C" fn schrage_mult(
    mut a: libc::c_ulong,
    mut b: libc::c_ulong,
    mut m: libc::c_ulong,
    mut sqrtm: libc::c_ulong,
) -> libc::c_ulong {
    let mut t0: libc::c_ulong = schrage(sqrtm, b, m);
    let mut t1: libc::c_ulong = schrage(a.wrapping_div(sqrtm), t0, m);
    let mut t2: libc::c_ulong = schrage(a.wrapping_rem(sqrtm), b, m);
    let mut t: libc::c_ulong = t1.wrapping_add(t2);
    return if t >= m { t.wrapping_sub(m) } else { t };
}
#[inline]
unsafe extern "C" fn ran_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    (*state)
        .x = schrage_mult(
        62089911 as libc::c_ulong,
        (*state).x,
        0x7fffffff as libc::c_ulong,
        46341 as libc::c_ulong,
    );
    return (*state).x;
}
unsafe extern "C" fn ran_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    return ran_get(state as *mut libc::c_void) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn ran_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    if s.wrapping_rem(0x7fffffff as libc::c_ulong) == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).x = s.wrapping_rem(0x7fffffff as libc::c_ulong);
}
static mut ran_type: gsl_rng_type = gsl_rng_type {
    name: 0 as *const libc::c_char,
    max: 0,
    min: 0,
    size: 0,
    set: None,
    get: None,
    get_double: None,
};
pub static mut gsl_rng_fishman18: *const gsl_rng_type = unsafe {
    &ran_type as *const gsl_rng_type
};
unsafe extern "C" fn run_static_initializers() {
    ran_type = {
        let mut init = gsl_rng_type {
            name: b"fishman18\0" as *const u8 as *const libc::c_char,
            max: (0x7fffffff as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            min: 1 as libc::c_int as libc::c_ulong,
            size: ::std::mem::size_of::<ran_state_t>() as libc::c_ulong,
            set: Some(
                ran_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
            ),
            get: Some(
                ran_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
            ),
            get_double: Some(
                ran_get_double
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
            ),
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
