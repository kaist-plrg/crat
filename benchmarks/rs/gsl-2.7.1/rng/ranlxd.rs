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
pub struct ranlxd_state_t {
    pub xdbl: [libc::c_double; 12],
    pub carry: libc::c_double,
    pub ir: libc::c_uint,
    pub jr: libc::c_uint,
    pub ir_old: libc::c_uint,
    pub pr: libc::c_uint,
}
static mut next: [libc::c_int; 12] = [
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    0 as libc::c_int,
];
static mut one_bit: libc::c_double = 1.0f64 / 281474976710656.0f64;
#[inline]
unsafe extern "C" fn increment_state(mut state: *mut ranlxd_state_t) {
    let mut k: libc::c_int = 0;
    let mut kmax: libc::c_int = 0;
    let mut y1: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut y3: libc::c_double = 0.;
    let mut xdbl: *mut libc::c_double = ((*state).xdbl).as_mut_ptr();
    let mut carry: libc::c_double = (*state).carry;
    let mut ir: libc::c_uint = (*state).ir;
    let mut jr: libc::c_uint = (*state).jr;
    k = 0 as libc::c_int;
    while ir > 0 as libc::c_int as libc::c_uint {
        y1 = *xdbl.offset(jr as isize) - *xdbl.offset(ir as isize);
        y2 = y1 - carry;
        if y2 < 0 as libc::c_int as libc::c_double {
            carry = one_bit;
            y2 += 1 as libc::c_int as libc::c_double;
        } else {
            carry = 0 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(ir as isize) = y2;
        ir = next[ir as usize] as libc::c_uint;
        jr = next[jr as usize] as libc::c_uint;
        k += 1;
        k;
    }
    kmax = ((*state).pr).wrapping_sub(12 as libc::c_int as libc::c_uint) as libc::c_int;
    while k <= kmax {
        y1 = *xdbl.offset(7 as libc::c_int as isize)
            - *xdbl.offset(0 as libc::c_int as isize);
        y1 -= carry;
        y2 = *xdbl.offset(8 as libc::c_int as isize)
            - *xdbl.offset(1 as libc::c_int as isize);
        if y1 < 0 as libc::c_int as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(0 as libc::c_int as isize) = y1;
        y3 = *xdbl.offset(9 as libc::c_int as isize)
            - *xdbl.offset(2 as libc::c_int as isize);
        if y2 < 0 as libc::c_int as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(1 as libc::c_int as isize) = y2;
        y1 = *xdbl.offset(10 as libc::c_int as isize)
            - *xdbl.offset(3 as libc::c_int as isize);
        if y3 < 0 as libc::c_int as libc::c_double {
            y1 -= one_bit;
            y3 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(2 as libc::c_int as isize) = y3;
        y2 = *xdbl.offset(11 as libc::c_int as isize)
            - *xdbl.offset(4 as libc::c_int as isize);
        if y1 < 0 as libc::c_int as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(3 as libc::c_int as isize) = y1;
        y3 = *xdbl.offset(0 as libc::c_int as isize)
            - *xdbl.offset(5 as libc::c_int as isize);
        if y2 < 0 as libc::c_int as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(4 as libc::c_int as isize) = y2;
        y1 = *xdbl.offset(1 as libc::c_int as isize)
            - *xdbl.offset(6 as libc::c_int as isize);
        if y3 < 0 as libc::c_int as libc::c_double {
            y1 -= one_bit;
            y3 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(5 as libc::c_int as isize) = y3;
        y2 = *xdbl.offset(2 as libc::c_int as isize)
            - *xdbl.offset(7 as libc::c_int as isize);
        if y1 < 0 as libc::c_int as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(6 as libc::c_int as isize) = y1;
        y3 = *xdbl.offset(3 as libc::c_int as isize)
            - *xdbl.offset(8 as libc::c_int as isize);
        if y2 < 0 as libc::c_int as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(7 as libc::c_int as isize) = y2;
        y1 = *xdbl.offset(4 as libc::c_int as isize)
            - *xdbl.offset(9 as libc::c_int as isize);
        if y3 < 0 as libc::c_int as libc::c_double {
            y1 -= one_bit;
            y3 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(8 as libc::c_int as isize) = y3;
        y2 = *xdbl.offset(5 as libc::c_int as isize)
            - *xdbl.offset(10 as libc::c_int as isize);
        if y1 < 0 as libc::c_int as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(9 as libc::c_int as isize) = y1;
        y3 = *xdbl.offset(6 as libc::c_int as isize)
            - *xdbl.offset(11 as libc::c_int as isize);
        if y2 < 0 as libc::c_int as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(10 as libc::c_int as isize) = y2;
        if y3 < 0 as libc::c_int as libc::c_double {
            carry = one_bit;
            y3 += 1 as libc::c_int as libc::c_double;
        } else {
            carry = 0 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(11 as libc::c_int as isize) = y3;
        k += 12 as libc::c_int;
    }
    kmax = (*state).pr as libc::c_int;
    while k < kmax {
        y1 = *xdbl.offset(jr as isize) - *xdbl.offset(ir as isize);
        y2 = y1 - carry;
        if y2 < 0 as libc::c_int as libc::c_double {
            carry = one_bit;
            y2 += 1 as libc::c_int as libc::c_double;
        } else {
            carry = 0 as libc::c_int as libc::c_double;
        }
        *xdbl.offset(ir as isize) = y2;
        ir = next[ir as usize] as libc::c_uint;
        jr = next[jr as usize] as libc::c_uint;
        k += 1;
        k;
    }
    (*state).ir = ir;
    (*state).ir_old = ir;
    (*state).jr = jr;
    (*state).carry = carry;
}
#[inline]
unsafe extern "C" fn ranlxd_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    return (ranlxd_get_double(vstate) * 4294967296.0f64) as libc::c_ulong;
}
unsafe extern "C" fn ranlxd_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ranlxd_state_t = vstate as *mut ranlxd_state_t;
    let mut ir: libc::c_int = (*state).ir as libc::c_int;
    (*state).ir = next[ir as usize] as libc::c_uint;
    if (*state).ir == (*state).ir_old {
        increment_state(state);
    }
    return (*state).xdbl[(*state).ir as usize];
}
unsafe extern "C" fn ranlxd_set_lux(
    mut vstate: *mut libc::c_void,
    mut s: libc::c_ulong,
    mut luxury: libc::c_uint,
) {
    let mut state: *mut ranlxd_state_t = vstate as *mut ranlxd_state_t;
    let mut ibit: libc::c_int = 0;
    let mut jbit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut xbit: [libc::c_int; 31] = [0; 31];
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut seed: libc::c_long = 0;
    if s == 0 as libc::c_int as libc::c_ulong {
        s = 1 as libc::c_int as libc::c_ulong;
    }
    seed = s as libc::c_long;
    i = (seed as libc::c_ulong & 0xffffffff as libc::c_ulong) as libc::c_int;
    k = 0 as libc::c_int;
    while k < 31 as libc::c_int {
        xbit[k as usize] = i % 2 as libc::c_int;
        i /= 2 as libc::c_int;
        k += 1;
        k;
    }
    ibit = 0 as libc::c_int;
    jbit = 18 as libc::c_int;
    k = 0 as libc::c_int;
    while k < 12 as libc::c_int {
        x = 0 as libc::c_int as libc::c_double;
        l = 1 as libc::c_int;
        while l <= 48 as libc::c_int {
            y = ((xbit[ibit as usize] + 1 as libc::c_int) % 2 as libc::c_int)
                as libc::c_double;
            x += x + y;
            xbit[ibit
                as usize] = (xbit[ibit as usize] + xbit[jbit as usize])
                % 2 as libc::c_int;
            ibit = (ibit + 1 as libc::c_int) % 31 as libc::c_int;
            jbit = (jbit + 1 as libc::c_int) % 31 as libc::c_int;
            l += 1;
            l;
        }
        (*state).xdbl[k as usize] = one_bit * x;
        k += 1;
        k;
    }
    (*state).carry = 0 as libc::c_int as libc::c_double;
    (*state).ir = 11 as libc::c_int as libc::c_uint;
    (*state).jr = 7 as libc::c_int as libc::c_uint;
    (*state).ir_old = 0 as libc::c_int as libc::c_uint;
    (*state).pr = luxury;
}
unsafe extern "C" fn ranlxd1_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    ranlxd_set_lux(vstate, s, 202 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn ranlxd2_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    ranlxd_set_lux(vstate, s, 397 as libc::c_int as libc::c_uint);
}
static mut ranlxd1_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlxd1\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ranlxd_state_t>() as libc::c_ulong,
        set: Some(
            ranlxd1_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            ranlxd_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            ranlxd_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut ranlxd2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlxd2\0" as *const u8 as *const libc::c_char,
        max: 0xffffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ranlxd_state_t>() as libc::c_ulong,
        set: Some(
            ranlxd2_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            ranlxd_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            ranlxd_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ranlxd1: *const gsl_rng_type = unsafe {
    &ranlxd1_type as *const gsl_rng_type
};
pub static mut gsl_rng_ranlxd2: *const gsl_rng_type = unsafe {
    &ranlxd2_type as *const gsl_rng_type
};
