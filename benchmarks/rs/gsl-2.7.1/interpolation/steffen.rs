use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp_accel {
    pub cache: size_t,
    pub miss_count: size_t,
    pub hit_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_interp_type {
    pub name: *const libc::c_char,
    pub min_size: libc::c_uint,
    pub alloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
        ) -> libc::c_int,
    >,
    pub eval: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_deriv2: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            libc::c_double,
            *mut gsl_interp_accel,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub eval_integ: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_double,
            *const libc::c_double,
            size_t,
            *mut gsl_interp_accel,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct steffen_state_t {
    pub a: *mut libc::c_double,
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub d: *mut libc::c_double,
    pub y_prime: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn gsl_interp_bsearch(
    mut x_array: *const libc::c_double,
    mut x: libc::c_double,
    mut index_lo: size_t,
    mut index_hi: size_t,
) -> size_t {
    let mut ilo: size_t = index_lo;
    let mut ihi: size_t = index_hi;
    while ihi > ilo.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut i: size_t = ihi
            .wrapping_add(ilo)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if *x_array.offset(i as isize) > x {
            ihi = i;
        } else {
            ilo = i;
        }
    }
    return ilo;
}
#[inline]
unsafe extern "C" fn gsl_interp_accel_find(
    mut a: *mut gsl_interp_accel,
    mut xa: *const libc::c_double,
    mut len: size_t,
    mut x: libc::c_double,
) -> size_t {
    let mut x_index: size_t = (*a).cache;
    if x < *xa.offset(x_index as isize) {
        (*a).miss_count = ((*a).miss_count).wrapping_add(1);
        (*a).miss_count;
        (*a).cache = gsl_interp_bsearch(xa, x, 0 as libc::c_int as size_t, x_index);
    } else if x
        >= *xa.offset(x_index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
    {
        (*a).miss_count = ((*a).miss_count).wrapping_add(1);
        (*a).miss_count;
        (*a)
            .cache = gsl_interp_bsearch(
            xa,
            x,
            x_index,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        (*a).hit_count = ((*a).hit_count).wrapping_add(1);
        (*a).hit_count;
    }
    return (*a).cache;
}
unsafe extern "C" fn steffen_alloc(mut size: size_t) -> *mut libc::c_void {
    let mut state: *mut steffen_state_t = 0 as *mut steffen_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<steffen_state_t>() as libc::c_ulong,
    ) as *mut steffen_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for state\0" as *const u8 as *const libc::c_char,
            b"steffen.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .a = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).a).is_null() {
        steffen_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for a\0" as *const u8 as *const libc::c_char,
            b"steffen.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .b = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).b).is_null() {
        steffen_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for b\0" as *const u8 as *const libc::c_char,
            b"steffen.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .c = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).c).is_null() {
        steffen_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for c\0" as *const u8 as *const libc::c_char,
            b"steffen.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .d = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).d).is_null() {
        steffen_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"steffen.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y_prime = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).y_prime).is_null() {
        steffen_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y_prime\0" as *const u8
                as *const libc::c_char,
            b"steffen.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn steffen_init(
    mut vstate: *mut libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut state: *mut steffen_state_t = vstate as *mut steffen_state_t;
    let mut i: size_t = 0;
    let mut a: *mut libc::c_double = (*state).a;
    let mut b: *mut libc::c_double = (*state).b;
    let mut c: *mut libc::c_double = (*state).c;
    let mut d: *mut libc::c_double = (*state).d;
    let mut y_prime: *mut libc::c_double = (*state).y_prime;
    let mut h0: libc::c_double = *x_array.offset(1 as libc::c_int as isize)
        - *x_array.offset(0 as libc::c_int as isize);
    let mut s0: libc::c_double = (*y_array.offset(1 as libc::c_int as isize)
        - *y_array.offset(0 as libc::c_int as isize)) / h0;
    *y_prime.offset(0 as libc::c_int as isize) = s0;
    i = 1 as libc::c_int as size_t;
    while i < size.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut pi: libc::c_double = 0.;
        let mut hi: libc::c_double = *x_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *x_array.offset(i as isize);
        let mut him1: libc::c_double = *x_array.offset(i as isize)
            - *x_array
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        let mut si: libc::c_double = (*y_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *y_array.offset(i as isize)) / hi;
        let mut sim1: libc::c_double = (*y_array.offset(i as isize)
            - *y_array
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            / him1;
        pi = (sim1 * hi + si * him1) / (him1 + hi);
        *y_prime
            .offset(
                i as isize,
            ) = (steffen_copysign(1.0f64, sim1) + steffen_copysign(1.0f64, si))
            * (if fabs(sim1)
                < (if fabs(si) < 0.5f64 * fabs(pi) {
                    fabs(si)
                } else {
                    0.5f64 * fabs(pi)
                })
            {
                fabs(sim1)
            } else {
                (if fabs(si) < 0.5f64 * fabs(pi) { fabs(si) } else { 0.5f64 * fabs(pi) })
            });
        i = i.wrapping_add(1);
        i;
    }
    *y_prime
        .offset(
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*y_array
        .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        - *y_array.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize))
        / (*x_array.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            - *x_array
                .offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize));
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut hi_0: libc::c_double = *x_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *x_array.offset(i as isize);
        let mut si_0: libc::c_double = (*y_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *y_array.offset(i as isize)) / hi_0;
        *a
            .offset(
                i as isize,
            ) = (*y_prime.offset(i as isize)
            + *y_prime.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - 2 as libc::c_int as libc::c_double * si_0) / hi_0 / hi_0;
        *b
            .offset(
                i as isize,
            ) = (3 as libc::c_int as libc::c_double * si_0
            - 2 as libc::c_int as libc::c_double * *y_prime.offset(i as isize)
            - *y_prime
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
            / hi_0;
        *c.offset(i as isize) = *y_prime.offset(i as isize);
        *d.offset(i as isize) = *y_array.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steffen_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut steffen_state_t = vstate as *mut steffen_state_t;
    if state.is_null() {
        return;
    }
    if !((*state).a).is_null() {
        free((*state).a as *mut libc::c_void);
    }
    if !((*state).b).is_null() {
        free((*state).b as *mut libc::c_void);
    }
    if !((*state).c).is_null() {
        free((*state).c as *mut libc::c_void);
    }
    if !((*state).d).is_null() {
        free((*state).d as *mut libc::c_void);
    }
    if !((*state).y_prime).is_null() {
        free((*state).y_prime as *mut libc::c_void);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn steffen_eval(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const steffen_state_t = vstate as *const steffen_state_t;
    let mut index: size_t = 0;
    if !a.is_null() {
        index = gsl_interp_accel_find(a, x_array, size, x);
    } else {
        index = gsl_interp_bsearch(
            x_array,
            x,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    let x_lo: libc::c_double = *x_array.offset(index as isize);
    let delx: libc::c_double = x - x_lo;
    let a_0: libc::c_double = *((*state).a).offset(index as isize);
    let b: libc::c_double = *((*state).b).offset(index as isize);
    let c: libc::c_double = *((*state).c).offset(index as isize);
    let d: libc::c_double = *((*state).d).offset(index as isize);
    *y = d + delx * (c + delx * (b + delx * a_0));
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steffen_eval_deriv(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut dydx: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const steffen_state_t = vstate as *const steffen_state_t;
    let mut index: size_t = 0;
    if !a.is_null() {
        index = gsl_interp_accel_find(a, x_array, size, x);
    } else {
        index = gsl_interp_bsearch(
            x_array,
            x,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    let mut x_lo: libc::c_double = *x_array.offset(index as isize);
    let mut delx: libc::c_double = x - x_lo;
    let mut a_0: libc::c_double = *((*state).a).offset(index as isize);
    let mut b: libc::c_double = *((*state).b).offset(index as isize);
    let mut c: libc::c_double = *((*state).c).offset(index as isize);
    *dydx = c
        + delx
            * (2 as libc::c_int as libc::c_double * b
                + delx * 3 as libc::c_int as libc::c_double * a_0);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steffen_eval_deriv2(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y_pp: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const steffen_state_t = vstate as *const steffen_state_t;
    let mut index: size_t = 0;
    if !a.is_null() {
        index = gsl_interp_accel_find(a, x_array, size, x);
    } else {
        index = gsl_interp_bsearch(
            x_array,
            x,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    let x_lo: libc::c_double = *x_array.offset(index as isize);
    let delx: libc::c_double = x - x_lo;
    let a_0: libc::c_double = *((*state).a).offset(index as isize);
    let b: libc::c_double = *((*state).b).offset(index as isize);
    *y_pp = 6 as libc::c_int as libc::c_double * a_0 * delx
        + 2 as libc::c_int as libc::c_double * b;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steffen_eval_integ(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut acc: *mut gsl_interp_accel,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const steffen_state_t = vstate as *const steffen_state_t;
    let mut i: size_t = 0;
    let mut index_a: size_t = 0;
    let mut index_b: size_t = 0;
    if !acc.is_null() {
        index_a = gsl_interp_accel_find(acc, x_array, size, a);
        index_b = gsl_interp_accel_find(acc, x_array, size, b);
    } else {
        index_a = gsl_interp_bsearch(
            x_array,
            a,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        index_b = gsl_interp_bsearch(
            x_array,
            b,
            0 as libc::c_int as size_t,
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    *result = 0.0f64;
    i = index_a;
    while i <= index_b {
        let x_hi: libc::c_double = *x_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let x_lo: libc::c_double = *x_array.offset(i as isize);
        let dx: libc::c_double = x_hi - x_lo;
        if dx != 0.0f64 {
            let mut x1: libc::c_double = if i == index_a { a - x_lo } else { 0.0f64 };
            let mut x2: libc::c_double = if i == index_b {
                b - x_lo
            } else {
                x_hi - x_lo
            };
            *result
                += 1.0f64 / 4.0f64 * *((*state).a).offset(i as isize)
                    * (x2 * x2 * x2 * x2 - x1 * x1 * x1 * x1)
                    + 1.0f64 / 3.0f64 * *((*state).b).offset(i as isize)
                        * (x2 * x2 * x2 - x1 * x1 * x1)
                    + 1.0f64 / 2.0f64 * *((*state).c).offset(i as isize)
                        * (x2 * x2 - x1 * x1)
                    + *((*state).d).offset(i as isize) * (x2 - x1);
        } else {
            *result = 0.0f64;
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn steffen_copysign(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double && y > 0 as libc::c_int as libc::c_double
        || x > 0 as libc::c_int as libc::c_double
            && y < 0 as libc::c_int as libc::c_double
    {
        return -x;
    }
    return x;
}
static mut steffen_type: gsl_interp_type = {
    let mut init = gsl_interp_type {
        name: b"steffen\0" as *const u8 as *const libc::c_char,
        min_size: 3 as libc::c_int as libc::c_uint,
        alloc: Some(steffen_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        init: Some(
            steffen_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            steffen_eval
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv: Some(
            steffen_eval_deriv
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_deriv2: Some(
            steffen_eval_deriv2
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    libc::c_double,
                    *mut gsl_interp_accel,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        eval_integ: Some(
            steffen_eval_integ
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                    *mut gsl_interp_accel,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        free: Some(steffen_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_interp_steffen: *const gsl_interp_type = unsafe {
    &steffen_type as *const gsl_interp_type
};
