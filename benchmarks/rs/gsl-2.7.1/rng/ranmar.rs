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
pub struct ranmar_state_t {
    pub i: libc::c_uint,
    pub j: libc::c_uint,
    pub carry: libc::c_long,
    pub u: [libc::c_ulong; 97],
}
static mut two24: libc::c_ulong = 16777216 as libc::c_int as libc::c_ulong;
#[inline]
unsafe extern "C" fn ranmar_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ranmar_state_t = vstate as *mut ranmar_state_t;
    let mut i: libc::c_uint = (*state).i;
    let mut j: libc::c_uint = (*state).j;
    let mut carry: libc::c_long = (*state).carry;
    let mut delta: libc::c_long = ((*state).u[i as usize])
        .wrapping_sub((*state).u[j as usize]) as libc::c_long;
    if delta < 0 as libc::c_int as libc::c_long {
        delta = (delta as libc::c_ulong).wrapping_add(two24) as libc::c_long
            as libc::c_long;
    }
    (*state).u[i as usize] = delta as libc::c_ulong;
    if i == 0 as libc::c_int as libc::c_uint {
        i = 96 as libc::c_int as libc::c_uint;
    } else {
        i = i.wrapping_sub(1);
        i;
    }
    (*state).i = i;
    if j == 0 as libc::c_int as libc::c_uint {
        j = 96 as libc::c_int as libc::c_uint;
    } else {
        j = j.wrapping_sub(1);
        j;
    }
    (*state).j = j;
    carry += -(7654321 as libc::c_int) as libc::c_long;
    if carry < 0 as libc::c_int as libc::c_long {
        carry = (carry as libc::c_ulong)
            .wrapping_add(two24.wrapping_sub(3 as libc::c_int as libc::c_ulong))
            as libc::c_long as libc::c_long;
    }
    (*state).carry = carry;
    delta += -carry;
    if delta < 0 as libc::c_int as libc::c_long {
        delta = (delta as libc::c_ulong).wrapping_add(two24) as libc::c_long
            as libc::c_long;
    }
    return delta as libc::c_ulong;
}
unsafe extern "C" fn ranmar_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ranmar_get(vstate) as libc::c_double / 16777216.0f64;
}
unsafe extern "C" fn ranmar_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ranmar_state_t = vstate as *mut ranmar_state_t;
    let mut ij: libc::c_ulong = s.wrapping_div(30082 as libc::c_int as libc::c_ulong);
    let mut kl: libc::c_ulong = s.wrapping_rem(30082 as libc::c_int as libc::c_ulong);
    let mut i: libc::c_int = ij
        .wrapping_div(177 as libc::c_int as libc::c_ulong)
        .wrapping_rem(177 as libc::c_int as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut j: libc::c_int = ij
        .wrapping_rem(177 as libc::c_int as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut k: libc::c_int = kl
        .wrapping_div(169 as libc::c_int as libc::c_ulong)
        .wrapping_rem(178 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut l: libc::c_int = kl.wrapping_rem(169 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    a = 0 as libc::c_int;
    while a < 97 as libc::c_int {
        let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut t: libc::c_ulong = two24;
        b = 0 as libc::c_int;
        while b < 24 as libc::c_int {
            let mut m: libc::c_ulong = (i * j % 179 as libc::c_int * k
                % 179 as libc::c_int) as libc::c_ulong;
            i = j;
            j = k;
            k = m as libc::c_int;
            l = (53 as libc::c_int * l + 1 as libc::c_int) % 169 as libc::c_int;
            t >>= 1 as libc::c_int;
            if (l as libc::c_ulong)
                .wrapping_mul(m)
                .wrapping_rem(64 as libc::c_int as libc::c_ulong)
                >= 32 as libc::c_int as libc::c_ulong
            {
                sum = sum.wrapping_add(t);
            }
            b += 1;
            b;
        }
        (*state).u[a as usize] = sum;
        a += 1;
        a;
    }
    (*state).i = 96 as libc::c_int as libc::c_uint;
    (*state).j = 32 as libc::c_int as libc::c_uint;
    (*state).carry = 362436 as libc::c_int as libc::c_long;
}
static mut ranmar_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranmar\0" as *const u8 as *const libc::c_char,
        max: 0xffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ranmar_state_t>() as libc::c_ulong,
        set: Some(
            ranmar_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(
            ranmar_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong,
        ),
        get_double: Some(
            ranmar_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_ranmar: *const gsl_rng_type = unsafe {
    &ranmar_type as *const gsl_rng_type
};
