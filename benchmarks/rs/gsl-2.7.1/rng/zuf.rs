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
pub struct zuf_state_t {
    pub n: libc::c_int,
    pub u: [libc::c_ulong; 607],
}
static mut zuf_randmax: libc::c_ulong = 16777216 as libc::c_int as libc::c_ulong;
#[inline]
unsafe extern "C" fn zuf_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut zuf_state_t = vstate as *mut zuf_state_t;
    let n: libc::c_int = (*state).n;
    let m: libc::c_int = (n - 273 as libc::c_int + 607 as libc::c_int)
        % 607 as libc::c_int;
    let mut t: libc::c_ulong = ((*state).u[n as usize])
        .wrapping_add((*state).u[m as usize]);
    while t > zuf_randmax {
        t = t.wrapping_sub(zuf_randmax);
    }
    (*state).u[n as usize] = t;
    if n == 606 as libc::c_int {
        (*state).n = 0 as libc::c_int;
    } else {
        (*state).n = n + 1 as libc::c_int;
    }
    return t;
}
unsafe extern "C" fn zuf_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return zuf_get(vstate) as libc::c_double / 16777216.0f64;
}
unsafe extern "C" fn zuf_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut kl: libc::c_long = 9373 as libc::c_int as libc::c_long;
    let mut ij: libc::c_long = 1802 as libc::c_int as libc::c_long;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut ii: libc::c_long = 0;
    let mut jj: libc::c_long = 0;
    let mut state: *mut zuf_state_t = vstate as *mut zuf_state_t;
    (*state).n = 0 as libc::c_int;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1802 as libc::c_int as libc::c_ulong;
    }
    ij = s as libc::c_long;
    i = ij / 177 as libc::c_int as libc::c_long % 177 as libc::c_int as libc::c_long
        + 2 as libc::c_int as libc::c_long;
    j = ij % 177 as libc::c_int as libc::c_long + 2 as libc::c_int as libc::c_long;
    k = kl / 169 as libc::c_int as libc::c_long % 178 as libc::c_int as libc::c_long
        + 1 as libc::c_int as libc::c_long;
    l = kl % 169 as libc::c_int as libc::c_long;
    ii = 0 as libc::c_int as libc::c_long;
    while ii < 607 as libc::c_int as libc::c_long {
        x = 0.0f64;
        y = 0.5f64;
        jj = 1 as libc::c_int as libc::c_long;
        while jj <= 24 as libc::c_int as libc::c_long {
            m = i * j % 179 as libc::c_int as libc::c_long * k
                % 179 as libc::c_int as libc::c_long;
            i = j;
            j = k;
            k = m;
            l = (l * 53 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long) % 169 as libc::c_int as libc::c_long;
            if l * m % 64 as libc::c_int as libc::c_long
                >= 32 as libc::c_int as libc::c_long
            {
                x += y;
            }
            y *= 0.5f64;
            jj += 1;
            jj;
        }
        (*state).u[ii as usize] = (x * zuf_randmax as libc::c_double) as libc::c_ulong;
        ii += 1;
        ii;
    }
}
static mut zuf_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"zuf\0" as *const u8 as *const libc::c_char,
        max: 0xffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<zuf_state_t>() as libc::c_ulong,
        set: Some(
            zuf_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(zuf_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            zuf_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_zuf: *const gsl_rng_type = unsafe {
    &zuf_type as *const gsl_rng_type
};
