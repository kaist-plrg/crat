use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub struct akima_state_t {
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub d: *mut libc::c_double,
    pub _m: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn integ_eval(
    mut ai: libc::c_double,
    mut bi: libc::c_double,
    mut ci: libc::c_double,
    mut di: libc::c_double,
    mut xi: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let r1: libc::c_double = a - xi;
    let r2: libc::c_double = b - xi;
    let r12: libc::c_double = r1 + r2;
    let bterm: libc::c_double = 0.5f64 * bi * r12;
    let cterm: libc::c_double = 1.0f64 / 3.0f64 * ci * (r1 * r1 + r2 * r2 + r1 * r2);
    let dterm: libc::c_double = 0.25f64 * di * r12 * (r1 * r1 + r2 * r2);
    return (b - a) * (ai + bterm + cterm + dterm);
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
unsafe extern "C" fn akima_alloc(mut size: size_t) -> *mut libc::c_void {
    let mut state: *mut akima_state_t = malloc(
        ::std::mem::size_of::<akima_state_t>() as libc::c_ulong,
    ) as *mut akima_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for state\0" as *const u8 as *const libc::c_char,
            b"akima.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .b = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).b).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for b\0" as *const u8 as *const libc::c_char,
            b"akima.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .c = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).c).is_null() {
        free((*state).b as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for c\0" as *const u8 as *const libc::c_char,
            b"akima.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .d = malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).d).is_null() {
        free((*state).c as *mut libc::c_void);
        free((*state).b as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const libc::c_char,
            b"akima.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        ._m = malloc(
        size
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state)._m).is_null() {
        free((*state).d as *mut libc::c_void);
        free((*state).c as *mut libc::c_void);
        free((*state).b as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for _m\0" as *const u8 as *const libc::c_char,
            b"akima.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn akima_calc(
    mut x_array: *const libc::c_double,
    mut b: *mut libc::c_double,
    mut c: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut size: size_t,
    mut m: *mut libc::c_double,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let NE: libc::c_double = fabs(
            *m.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *m.offset(i as isize),
        )
            + fabs(
                *m.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    - *m
                        .offset(
                            i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                        ),
            );
        if NE == 0.0f64 {
            *b.offset(i as isize) = *m.offset(i as isize);
            *c.offset(i as isize) = 0.0f64;
            *d.offset(i as isize) = 0.0f64;
        } else {
            let h_i: libc::c_double = *x_array
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *x_array.offset(i as isize);
            let NE_next: libc::c_double = fabs(
                *m.offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                    - *m
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
            )
                + fabs(
                    *m.offset(i as isize)
                        - *m
                            .offset(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                );
            let alpha_i: libc::c_double = fabs(
                *m.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    - *m
                        .offset(
                            i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                        ),
            ) / NE;
            let mut alpha_ip1: libc::c_double = 0.;
            let mut tL_ip1: libc::c_double = 0.;
            if NE_next == 0.0f64 {
                tL_ip1 = *m.offset(i as isize);
            } else {
                alpha_ip1 = fabs(
                    *m.offset(i as isize)
                        - *m
                            .offset(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                ) / NE_next;
                tL_ip1 = (1.0f64 - alpha_ip1) * *m.offset(i as isize)
                    + alpha_ip1
                        * *m
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            );
            }
            *b
                .offset(
                    i as isize,
                ) = (1.0f64 - alpha_i)
                * *m.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                + alpha_i * *m.offset(i as isize);
            *c
                .offset(
                    i as isize,
                ) = (3.0f64 * *m.offset(i as isize) - 2.0f64 * *b.offset(i as isize)
                - tL_ip1) / h_i;
            *d
                .offset(
                    i as isize,
                ) = (*b.offset(i as isize) + tL_ip1 - 2.0f64 * *m.offset(i as isize))
                / (h_i * h_i);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn akima_init(
    mut vstate: *mut libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut state: *mut akima_state_t = vstate as *mut akima_state_t;
    let mut m: *mut libc::c_double = ((*state)._m).offset(2 as libc::c_int as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= size.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
        *m
            .offset(
                i as isize,
            ) = (*y_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *y_array.offset(i as isize))
            / (*x_array
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *x_array.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    *m
        .offset(
            -(2 as libc::c_int) as isize,
        ) = 3.0f64 * *m.offset(0 as libc::c_int as isize)
        - 2.0f64 * *m.offset(1 as libc::c_int as isize);
    *m
        .offset(
            -(1 as libc::c_int) as isize,
        ) = 2.0f64 * *m.offset(0 as libc::c_int as isize)
        - *m.offset(1 as libc::c_int as isize);
    *m
        .offset(
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 2.0f64
        * *m.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
        - *m.offset(size.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize);
    *m
        .offset(
            size as isize,
        ) = 3.0f64
        * *m.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
        - 2.0f64
            * *m.offset(size.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize);
    akima_calc(x_array, (*state).b, (*state).c, (*state).d, size, m);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn akima_init_periodic(
    mut vstate: *mut libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    let mut state: *mut akima_state_t = vstate as *mut akima_state_t;
    let mut m: *mut libc::c_double = ((*state)._m).offset(2 as libc::c_int as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= size.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
        *m
            .offset(
                i as isize,
            ) = (*y_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            - *y_array.offset(i as isize))
            / (*x_array
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *x_array.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    *m
        .offset(
            -(2 as libc::c_int) as isize,
        ) = *m
        .offset(
            size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
        );
    *m
        .offset(
            -(1 as libc::c_int) as isize,
        ) = *m
        .offset(
            size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    *m
        .offset(
            size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *m.offset(0 as libc::c_int as isize);
    *m.offset(size as isize) = *m.offset(1 as libc::c_int as isize);
    akima_calc(x_array, (*state).b, (*state).c, (*state).d, size, m);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn akima_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut akima_state_t = vstate as *mut akima_state_t;
    free((*state).b as *mut libc::c_void);
    free((*state).c as *mut libc::c_void);
    free((*state).d as *mut libc::c_void);
    free((*state)._m as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn akima_eval(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const akima_state_t = vstate as *const akima_state_t;
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
    let b: libc::c_double = *((*state).b).offset(index as isize);
    let c: libc::c_double = *((*state).c).offset(index as isize);
    let d: libc::c_double = *((*state).d).offset(index as isize);
    *y = *y_array.offset(index as isize) + delx * (b + delx * (c + d * delx));
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn akima_eval_deriv(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut dydx: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const akima_state_t = vstate as *const akima_state_t;
    let mut index: size_t = 0;
    while if !y_array.is_null() { 0 as libc::c_int } else { 0 as libc::c_int } != 0 {}
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
    let mut b: libc::c_double = *((*state).b).offset(index as isize);
    let mut c: libc::c_double = *((*state).c).offset(index as isize);
    let mut d: libc::c_double = *((*state).d).offset(index as isize);
    *dydx = b + delx * (2.0f64 * c + 3.0f64 * d * delx);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn akima_eval_deriv2(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y_pp: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const akima_state_t = vstate as *const akima_state_t;
    let mut index: size_t = 0;
    while if !y_array.is_null() { 0 as libc::c_int } else { 0 as libc::c_int } != 0 {}
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
    let c: libc::c_double = *((*state).c).offset(index as isize);
    let d: libc::c_double = *((*state).d).offset(index as isize);
    *y_pp = 2.0f64 * c + 6.0f64 * d * delx;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn akima_eval_integ(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut acc: *mut gsl_interp_accel,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let mut state: *const akima_state_t = vstate as *const akima_state_t;
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
        let y_lo: libc::c_double = *y_array.offset(i as isize);
        let dx: libc::c_double = x_hi - x_lo;
        if dx != 0.0f64 {
            if i == index_a || i == index_b {
                let mut x1: libc::c_double = if i == index_a { a } else { x_lo };
                let mut x2: libc::c_double = if i == index_b { b } else { x_hi };
                *result
                    += integ_eval(
                        y_lo,
                        *((*state).b).offset(i as isize),
                        *((*state).c).offset(i as isize),
                        *((*state).d).offset(i as isize),
                        x_lo,
                        x1,
                        x2,
                    );
            } else {
                *result
                    += dx
                        * (y_lo
                            + dx
                                * (0.5f64 * *((*state).b).offset(i as isize)
                                    + dx
                                        * (*((*state).c).offset(i as isize) / 3.0f64
                                            + 0.25f64 * *((*state).d).offset(i as isize) * dx)));
            }
        } else {
            *result = 0.0f64;
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut akima_type: gsl_interp_type = {
    let mut init = gsl_interp_type {
        name: b"akima\0" as *const u8 as *const libc::c_char,
        min_size: 5 as libc::c_int as libc::c_uint,
        alloc: Some(akima_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        init: Some(
            akima_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            akima_eval
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
            akima_eval_deriv
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
            akima_eval_deriv2
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
            akima_eval_integ
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
        free: Some(akima_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_interp_akima: *const gsl_interp_type = unsafe {
    &akima_type as *const gsl_interp_type
};
static mut akima_periodic_type: gsl_interp_type = {
    let mut init = gsl_interp_type {
        name: b"akima-periodic\0" as *const u8 as *const libc::c_char,
        min_size: 5 as libc::c_int as libc::c_uint,
        alloc: Some(akima_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        init: Some(
            akima_init_periodic
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            akima_eval
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
            akima_eval_deriv
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
            akima_eval_deriv2
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
            akima_eval_integ
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
        free: Some(akima_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_interp_akima_periodic: *const gsl_interp_type = unsafe {
    &akima_periodic_type as *const gsl_interp_type
};
