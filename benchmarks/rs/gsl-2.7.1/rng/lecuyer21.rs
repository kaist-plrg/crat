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
unsafe extern "C" fn ran_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    let mut y: libc::c_long = (*state).x as libc::c_long;
    let mut r: libc::c_long = 3791 as libc::c_int as libc::c_long
        * (y / 52774 as libc::c_int as libc::c_long);
    y = 40692 as libc::c_int as libc::c_long * (y % 52774 as libc::c_int as libc::c_long)
        - r;
    if y < 0 as libc::c_int as libc::c_long {
        y = (y as libc::c_ulong).wrapping_add(2147483399 as libc::c_ulong)
            as libc::c_long as libc::c_long;
    }
    (*state).x = y as libc::c_ulong;
    return (*state).x;
}
unsafe extern "C" fn ran_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    return ran_get(state as *mut libc::c_void) as libc::c_double / 2147483399.0f64;
}
unsafe extern "C" fn ran_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    if s.wrapping_rem(2147483399 as libc::c_ulong) == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    (*state).x = s.wrapping_rem(2147483399 as libc::c_ulong);
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
pub static mut gsl_rng_lecuyer21: *const gsl_rng_type = unsafe {
    &ran_type as *const gsl_rng_type
};
unsafe extern "C" fn run_static_initializers() {
    ran_type = {
        let mut init = gsl_rng_type {
            name: b"lecuyer21\0" as *const u8 as *const libc::c_char,
            max: (2147483399 as libc::c_ulong)
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
