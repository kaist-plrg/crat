use ::libc;
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
unsafe extern "C" fn linear_init(
    mut vstate: *mut libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
) -> libc::c_int {
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn linear_eval(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut x_lo: libc::c_double = 0.;
    let mut x_hi: libc::c_double = 0.;
    let mut y_lo: libc::c_double = 0.;
    let mut y_hi: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
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
    x_lo = *x_array.offset(index as isize);
    x_hi = *x_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    y_lo = *y_array.offset(index as isize);
    y_hi = *y_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    dx = x_hi - x_lo;
    if dx > 0.0f64 {
        *y = y_lo + (x - x_lo) / dx * (y_hi - y_lo);
        return GSL_SUCCESS as libc::c_int;
    } else {
        *y = 0.0f64;
        return GSL_EINVAL as libc::c_int;
    };
}
unsafe extern "C" fn linear_eval_deriv(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut dydx: *mut libc::c_double,
) -> libc::c_int {
    let mut x_lo: libc::c_double = 0.;
    let mut x_hi: libc::c_double = 0.;
    let mut y_lo: libc::c_double = 0.;
    let mut y_hi: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
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
    x_lo = *x_array.offset(index as isize);
    x_hi = *x_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    y_lo = *y_array.offset(index as isize);
    y_hi = *y_array
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    dx = x_hi - x_lo;
    dy = y_hi - y_lo;
    if dx > 0.0f64 {
        *dydx = dy / dx;
        return GSL_SUCCESS as libc::c_int;
    } else {
        *dydx = 0.0f64;
        return GSL_EINVAL as libc::c_int;
    };
}
unsafe extern "C" fn linear_eval_deriv2(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut x: libc::c_double,
    mut a: *mut gsl_interp_accel,
    mut y_pp: *mut libc::c_double,
) -> libc::c_int {
    *y_pp = 0.0f64;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn linear_eval_integ(
    mut vstate: *const libc::c_void,
    mut x_array: *const libc::c_double,
    mut y_array: *const libc::c_double,
    mut size: size_t,
    mut acc: *mut gsl_interp_accel,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
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
        let y_hi: libc::c_double = *y_array
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        let dx: libc::c_double = x_hi - x_lo;
        if dx != 0.0f64 {
            if i == index_a || i == index_b {
                let mut x1: libc::c_double = if i == index_a { a } else { x_lo };
                let mut x2: libc::c_double = if i == index_b { b } else { x_hi };
                let D: libc::c_double = (y_hi - y_lo) / dx;
                *result += (x2 - x1) * (y_lo + 0.5f64 * D * (x2 - x_lo + (x1 - x_lo)));
            } else {
                *result += 0.5f64 * dx * (y_lo + y_hi);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
static mut linear_type: gsl_interp_type = {
    let mut init = gsl_interp_type {
        name: b"linear\0" as *const u8 as *const libc::c_char,
        min_size: 2 as libc::c_int as libc::c_uint,
        alloc: None,
        init: Some(
            linear_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_double,
                    *const libc::c_double,
                    size_t,
                ) -> libc::c_int,
        ),
        eval: Some(
            linear_eval
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
            linear_eval_deriv
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
            linear_eval_deriv2
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
            linear_eval_integ
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
        free: None,
    };
    init
};
pub static mut gsl_interp_linear: *const gsl_interp_type = unsafe {
    &linear_type as *const gsl_interp_type
};
