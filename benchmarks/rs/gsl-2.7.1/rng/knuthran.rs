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
    pub i: libc::c_uint,
    pub aa: [libc::c_ulong; 2009],
    pub ran_x: [libc::c_ulong; 100],
}
#[inline]
unsafe extern "C" fn ran_array(
    mut aa: *mut libc::c_ulong,
    mut n: libc::c_uint,
    mut ran_x: *mut libc::c_ulong,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    j = 0 as libc::c_int as libc::c_uint;
    while j < 100 as libc::c_int as libc::c_uint {
        *aa.offset(j as isize) = *ran_x.offset(j as isize);
        j = j.wrapping_add(1);
        j;
    }
    while j < n {
        *aa
            .offset(
                j as isize,
            ) = (*aa.offset(j.wrapping_sub(100 as libc::c_int as libc::c_uint) as isize))
            .wrapping_sub(
                *aa.offset(j.wrapping_sub(37 as libc::c_int as libc::c_uint) as isize),
            )
            & (((1 as libc::c_long) << 30 as libc::c_int)
                - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
        j = j.wrapping_add(1);
        j;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 37 as libc::c_int as libc::c_uint {
        *ran_x
            .offset(
                i as isize,
            ) = (*aa.offset(j.wrapping_sub(100 as libc::c_int as libc::c_uint) as isize))
            .wrapping_sub(
                *aa.offset(j.wrapping_sub(37 as libc::c_int as libc::c_uint) as isize),
            )
            & (((1 as libc::c_long) << 30 as libc::c_int)
                - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(1);
        j;
    }
    while i < 100 as libc::c_int as libc::c_uint {
        *ran_x
            .offset(
                i as isize,
            ) = (*aa.offset(j.wrapping_sub(100 as libc::c_int as libc::c_uint) as isize))
            .wrapping_sub(
                *ran_x.offset(i.wrapping_sub(37 as libc::c_int as libc::c_uint) as isize),
            )
            & (((1 as libc::c_long) << 30 as libc::c_int)
                - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(1);
        j;
    }
}
#[inline]
unsafe extern "C" fn ran_get(mut vstate: *mut libc::c_void) -> libc::c_ulong {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    let mut i: libc::c_uint = (*state).i;
    if i == 0 as libc::c_int as libc::c_uint {
        ran_array(
            ((*state).aa).as_mut_ptr(),
            2009 as libc::c_int as libc::c_uint,
            ((*state).ran_x).as_mut_ptr(),
        );
    }
    (*state)
        .i = i
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_rem(2009 as libc::c_int as libc::c_uint);
    return (*state).aa[i as usize];
}
unsafe extern "C" fn ran_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    return ran_get(state as *mut libc::c_void) as libc::c_double / 1073741824.0f64;
}
unsafe extern "C" fn ran_set(mut vstate: *mut libc::c_void, mut s: libc::c_ulong) {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    let mut x: [libc::c_long; 199] = [0; 199];
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ss: libc::c_long = (s.wrapping_add(2 as libc::c_int as libc::c_ulong)
        & (((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_int as libc::c_long)
            as libc::c_ulong) as libc::c_long;
    j = 0 as libc::c_int;
    while j < 100 as libc::c_int {
        x[j as usize] = ss;
        ss <<= 1 as libc::c_int;
        if ss >= (1 as libc::c_long) << 30 as libc::c_int {
            ss
                -= ((1 as libc::c_long) << 30 as libc::c_int)
                    - 2 as libc::c_int as libc::c_long;
        }
        j += 1;
        j;
    }
    while j < 100 as libc::c_int + 100 as libc::c_int - 1 as libc::c_int {
        x[j as usize] = 0 as libc::c_int as libc::c_long;
        j += 1;
        j;
    }
    x[1 as libc::c_int as usize] += 1;
    x[1 as libc::c_int as usize];
    ss = (s
        & (((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_int as libc::c_long)
            as libc::c_ulong) as libc::c_long;
    t = 70 as libc::c_int - 1 as libc::c_int;
    while t != 0 {
        j = 100 as libc::c_int - 1 as libc::c_int;
        while j > 0 as libc::c_int {
            x[(j + j) as usize] = x[j as usize];
            j -= 1;
            j;
        }
        j = 100 as libc::c_int + 100 as libc::c_int - 2 as libc::c_int;
        while j > 100 as libc::c_int - 37 as libc::c_int {
            x[(100 as libc::c_int + 100 as libc::c_int - 1 as libc::c_int - j)
                as usize] = x[j as usize]
                & ((1 as libc::c_long) << 30 as libc::c_int)
                    - 2 as libc::c_int as libc::c_long;
            j -= 2 as libc::c_int;
        }
        j = 100 as libc::c_int + 100 as libc::c_int - 2 as libc::c_int;
        while j >= 100 as libc::c_int {
            if x[j as usize] & 1 as libc::c_int as libc::c_long != 0 {
                x[(j - (100 as libc::c_int - 37 as libc::c_int))
                    as usize] = x[(j - (100 as libc::c_int - 37 as libc::c_int))
                    as usize] - x[j as usize]
                    & ((1 as libc::c_long) << 30 as libc::c_int)
                        - 1 as libc::c_int as libc::c_long;
                x[(j - 100 as libc::c_int)
                    as usize] = x[(j - 100 as libc::c_int) as usize] - x[j as usize]
                    & ((1 as libc::c_long) << 30 as libc::c_int)
                        - 1 as libc::c_int as libc::c_long;
            }
            j -= 1;
            j;
        }
        if ss & 1 as libc::c_int as libc::c_long != 0 {
            j = 100 as libc::c_int;
            while j > 0 as libc::c_int {
                x[j as usize] = x[(j - 1 as libc::c_int) as usize];
                j -= 1;
                j;
            }
            x[0 as libc::c_int as usize] = x[100 as libc::c_int as usize];
            if x[100 as libc::c_int as usize] & 1 as libc::c_int as libc::c_long != 0 {
                x[37 as libc::c_int
                    as usize] = x[37 as libc::c_int as usize]
                    - x[100 as libc::c_int as usize]
                    & ((1 as libc::c_long) << 30 as libc::c_int)
                        - 1 as libc::c_int as libc::c_long;
            }
        }
        if ss != 0 {
            ss >>= 1 as libc::c_int;
        } else {
            t -= 1;
            t;
        }
    }
    (*state).i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int;
    while j < 37 as libc::c_int {
        (*state)
            .ran_x[(j + 100 as libc::c_int - 37 as libc::c_int)
            as usize] = x[j as usize] as libc::c_ulong;
        j += 1;
        j;
    }
    while j < 100 as libc::c_int {
        (*state)
            .ran_x[(j - 37 as libc::c_int) as usize] = x[j as usize] as libc::c_ulong;
        j += 1;
        j;
    }
}
static mut ran_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"knuthran\0" as *const u8 as *const libc::c_char,
        max: 0x3fffffff as libc::c_ulong,
        min: 0 as libc::c_int as libc::c_ulong,
        size: ::std::mem::size_of::<ran_state_t>() as libc::c_ulong,
        set: Some(
            ran_set as unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> (),
        ),
        get: Some(ran_get as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong),
        get_double: Some(
            ran_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
pub static mut gsl_rng_knuthran: *const gsl_rng_type = unsafe {
    &ran_type as *const gsl_rng_type
};
