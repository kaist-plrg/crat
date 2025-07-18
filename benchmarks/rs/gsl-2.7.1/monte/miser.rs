use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_function_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *mut libc::c_double,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_double,
    >,
    pub dim: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_monte_function = gsl_monte_function_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_miser_state {
    pub min_calls: size_t,
    pub min_calls_per_bisection: size_t,
    pub dither: libc::c_double,
    pub estimate_frac: libc::c_double,
    pub alpha: libc::c_double,
    pub dim: size_t,
    pub estimate_style: libc::c_int,
    pub depth: libc::c_int,
    pub verbose: libc::c_int,
    pub x: *mut libc::c_double,
    pub xmid: *mut libc::c_double,
    pub sigma_l: *mut libc::c_double,
    pub sigma_r: *mut libc::c_double,
    pub fmax_l: *mut libc::c_double,
    pub fmax_r: *mut libc::c_double,
    pub fmin_l: *mut libc::c_double,
    pub fmin_r: *mut libc::c_double,
    pub fsum_l: *mut libc::c_double,
    pub fsum_r: *mut libc::c_double,
    pub fsum2_l: *mut libc::c_double,
    pub fsum2_r: *mut libc::c_double,
    pub hits_l: *mut size_t,
    pub hits_r: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_miser_params {
    pub estimate_frac: libc::c_double,
    pub min_calls: size_t,
    pub min_calls_per_bisection: size_t,
    pub alpha: libc::c_double,
    pub dither: libc::c_double,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_pos(mut r: *const gsl_rng) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    loop {
        x = ((*(*r).type_0).get_double).unwrap()((*r).state);
        if !(x == 0 as libc::c_int as libc::c_double) {
            break;
        }
    }
    return x;
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_int(
    mut r: *const gsl_rng,
    mut n: libc::c_ulong,
) -> libc::c_ulong {
    let mut offset: libc::c_ulong = (*(*r).type_0).min;
    let mut range: libc::c_ulong = ((*(*r).type_0).max).wrapping_sub(offset);
    let mut scale: libc::c_ulong = 0;
    let mut k: libc::c_ulong = 0;
    if n > range || n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"invalid n, either 0 or exceeds maximum value of generator\0" as *const u8
                as *const libc::c_char,
            b"../gsl/gsl_rng.h\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ulong;
    }
    scale = range.wrapping_div(n);
    loop {
        k = (((*(*r).type_0).get).unwrap()((*r).state))
            .wrapping_sub(offset)
            .wrapping_div(scale);
        if !(k >= n) {
            break;
        }
    }
    return k;
}
pub unsafe extern "C" fn gsl_monte_miser_integrate(
    mut f: *mut gsl_monte_function,
    mut xl: *const libc::c_double,
    mut xu: *const libc::c_double,
    mut dim: size_t,
    mut calls: size_t,
    mut r: *mut gsl_rng,
    mut state: *mut gsl_monte_miser_state,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut n: size_t = 0;
    let mut estimate_calls: size_t = 0;
    let mut calls_l: size_t = 0;
    let mut calls_r: size_t = 0;
    let min_calls: size_t = (*state).min_calls;
    let mut i: size_t = 0;
    let mut i_bisect: size_t = 0;
    let mut found_best: libc::c_int = 0;
    let mut res_est: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut err_est: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut res_r: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut err_r: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut res_l: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut err_l: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xbi_l: libc::c_double = 0.;
    let mut xbi_m: libc::c_double = 0.;
    let mut xbi_r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut vol: libc::c_double = 0.;
    let mut weight_l: libc::c_double = 0.;
    let mut weight_r: libc::c_double = 0.;
    let mut x: *mut libc::c_double = (*state).x;
    let mut xmid: *mut libc::c_double = (*state).xmid;
    let mut sigma_l: *mut libc::c_double = (*state).sigma_l;
    let mut sigma_r: *mut libc::c_double = (*state).sigma_r;
    if dim != (*state).dim {
        gsl_error(
            b"number of dimensions must match allocated size\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        if *xu.offset(i as isize) <= *xl.offset(i as isize) {
            gsl_error(
                b"xu must be greater than xl\0" as *const u8 as *const libc::c_char,
                b"miser.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if *xu.offset(i as isize) - *xl.offset(i as isize) > 1.7976931348623157e+308f64 {
            gsl_error(
                b"Range of integration is too large, please rescale\0" as *const u8
                    as *const libc::c_char,
                b"miser.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*state).alpha < 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"alpha must be non-negative\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    vol = 1 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        vol *= *xu.offset(i as isize) - *xl.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    if calls < (*state).min_calls_per_bisection {
        let mut m: libc::c_double = 0.0f64;
        let mut q: libc::c_double = 0.0f64;
        if calls < 2 as libc::c_int as libc::c_ulong {
            gsl_error(
                b"insufficient calls for subvolume\0" as *const u8
                    as *const libc::c_char,
                b"miser.c\0" as *const u8 as *const libc::c_char,
                116 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        n = 0 as libc::c_int as size_t;
        while n < calls {
            i = 0 as libc::c_int as size_t;
            while i < dim {
                *x
                    .offset(
                        i as isize,
                    ) = *xl.offset(i as isize)
                    + gsl_rng_uniform_pos(r)
                        * (*xu.offset(i as isize) - *xl.offset(i as isize));
                i = i.wrapping_add(1);
                i;
            }
            let mut fval: libc::c_double = (Some(((*f).f).unwrap()))
                .unwrap()(x, (*f).dim, (*f).params);
            let mut d: libc::c_double = fval - m;
            m += d / (n as libc::c_double + 1.0f64);
            q += d * d * (n as libc::c_double / (n as libc::c_double + 1.0f64));
            n = n.wrapping_add(1);
            n;
        }
        *result = vol * m;
        *abserr = vol
            * sqrt(q / (calls as libc::c_double * (calls as libc::c_double - 1.0f64)));
        return GSL_SUCCESS as libc::c_int;
    }
    estimate_calls = (if min_calls as libc::c_double
        > calls as libc::c_double * (*state).estimate_frac
    {
        min_calls as libc::c_double
    } else {
        calls as libc::c_double * (*state).estimate_frac
    }) as size_t;
    if estimate_calls < (4 as libc::c_int as libc::c_ulong).wrapping_mul(dim) {
        gsl_error(
            b"insufficient calls to sample all halfspaces\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        s = if gsl_rng_uniform(r) - 0.5f64 >= 0.0f64 {
            (*state).dither
        } else {
            -(*state).dither
        };
        *((*state).xmid)
            .offset(
                i as isize,
            ) = (0.5f64 + s) * *xl.offset(i as isize)
            + (0.5f64 - s) * *xu.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    estimate_corrmc(
        f,
        xl,
        xu,
        dim,
        estimate_calls,
        r,
        state,
        &mut res_est,
        &mut err_est,
        xmid as *const libc::c_double,
        sigma_l,
        sigma_r,
    );
    calls = (calls as libc::c_ulong).wrapping_sub(estimate_calls) as size_t as size_t;
    let mut best_var: libc::c_double = 1.7976931348623157e+308f64;
    let mut beta: libc::c_double = 2.0f64 / (1.0f64 + (*state).alpha);
    found_best = 0 as libc::c_int;
    i_bisect = 0 as libc::c_int as size_t;
    weight_r = 1.0f64;
    weight_l = weight_r;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        if *sigma_l.offset(i as isize) >= 0 as libc::c_int as libc::c_double
            && *sigma_r.offset(i as isize) >= 0 as libc::c_int as libc::c_double
        {
            let mut var: libc::c_double = pow(*sigma_l.offset(i as isize), beta)
                + pow(*sigma_r.offset(i as isize), beta);
            if var <= best_var {
                found_best = 1 as libc::c_int;
                best_var = var;
                i_bisect = i;
                weight_l = pow(*sigma_l.offset(i as isize), beta);
                weight_r = pow(*sigma_r.offset(i as isize), beta);
                if weight_l == 0 as libc::c_int as libc::c_double
                    && weight_r == 0 as libc::c_int as libc::c_double
                {
                    weight_l = 1 as libc::c_int as libc::c_double;
                    weight_r = 1 as libc::c_int as libc::c_double;
                }
            }
        } else {
            if *sigma_l.offset(i as isize) < 0 as libc::c_int as libc::c_double {
                gsl_error(
                    b"no points in left-half space!\0" as *const u8
                        as *const libc::c_char,
                    b"miser.c\0" as *const u8 as *const libc::c_char,
                    209 as libc::c_int,
                    GSL_ESANITY as libc::c_int,
                );
                return GSL_ESANITY as libc::c_int;
            }
            if *sigma_r.offset(i as isize) < 0 as libc::c_int as libc::c_double {
                gsl_error(
                    b"no points in right-half space!\0" as *const u8
                        as *const libc::c_char,
                    b"miser.c\0" as *const u8 as *const libc::c_char,
                    213 as libc::c_int,
                    GSL_ESANITY as libc::c_int,
                );
                return GSL_ESANITY as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if found_best == 0 {
        i_bisect = gsl_rng_uniform_int(r, dim);
    }
    xbi_l = *xl.offset(i_bisect as isize);
    xbi_m = *xmid.offset(i_bisect as isize);
    xbi_r = *xu.offset(i_bisect as isize);
    let mut fraction_l: libc::c_double = fabs((xbi_m - xbi_l) / (xbi_r - xbi_l));
    let mut fraction_r: libc::c_double = 1 as libc::c_int as libc::c_double - fraction_l;
    let mut a: libc::c_double = fraction_l * weight_l;
    let mut b: libc::c_double = fraction_r * weight_r;
    calls_l = (min_calls as libc::c_double
        + calls.wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(min_calls))
            as libc::c_double * a / (a + b)) as size_t;
    calls_r = (min_calls as libc::c_double
        + calls.wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(min_calls))
            as libc::c_double * b / (a + b)) as size_t;
    let mut status: libc::c_int = 0;
    let mut xu_tmp: *mut libc::c_double = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if xu_tmp.is_null() {
        gsl_error(
            b"out of memory for left workspace\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *xu_tmp.offset(i as isize) = *xu.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    *xu_tmp.offset(i_bisect as isize) = xbi_m;
    status = gsl_monte_miser_integrate(
        f,
        xl,
        xu_tmp as *const libc::c_double,
        dim,
        calls_l,
        r,
        state,
        &mut res_l,
        &mut err_l,
    );
    free(xu_tmp as *mut libc::c_void);
    if status != GSL_SUCCESS as libc::c_int {
        return status;
    }
    let mut status_0: libc::c_int = 0;
    let mut xl_tmp: *mut libc::c_double = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if xl_tmp.is_null() {
        gsl_error(
            b"out of memory for right workspace\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        *xl_tmp.offset(i as isize) = *xl.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    *xl_tmp.offset(i_bisect as isize) = xbi_m;
    status_0 = gsl_monte_miser_integrate(
        f,
        xl_tmp as *const libc::c_double,
        xu,
        dim,
        calls_r,
        r,
        state,
        &mut res_r,
        &mut err_r,
    );
    free(xl_tmp as *mut libc::c_void);
    if status_0 != GSL_SUCCESS as libc::c_int {
        return status_0;
    }
    *result = res_l + res_r;
    *abserr = sqrt(err_l * err_l + err_r * err_r);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_monte_miser_alloc(
    mut dim: size_t,
) -> *mut gsl_monte_miser_state {
    let mut s: *mut gsl_monte_miser_state = malloc(
        ::std::mem::size_of::<gsl_monte_miser_state>() as libc::c_ulong,
    ) as *mut gsl_monte_miser_state;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for miser state struct\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .x = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).x).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .xmid = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).xmid).is_null() {
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for xmid\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .sigma_l = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).sigma_l).is_null() {
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for sigma_l\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .sigma_r = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).sigma_r).is_null() {
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for sigma_r\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fmax_l = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fmax_l).is_null() {
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fmax_l\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fmax_r = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fmax_r).is_null() {
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fmax_r\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            385 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fmin_l = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fmin_l).is_null() {
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fmin_l\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fmin_r = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fmin_r).is_null() {
        free((*s).fmin_l as *mut libc::c_void);
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fmin_r\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fsum_l = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fsum_l).is_null() {
        free((*s).fmin_r as *mut libc::c_void);
        free((*s).fmin_l as *mut libc::c_void);
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fsum_l\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fsum_r = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fsum_r).is_null() {
        free((*s).fsum_l as *mut libc::c_void);
        free((*s).fmin_r as *mut libc::c_void);
        free((*s).fmin_l as *mut libc::c_void);
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fsum_r\0" as *const u8 as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            447 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fsum2_l = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fsum2_l).is_null() {
        free((*s).fsum_r as *mut libc::c_void);
        free((*s).fsum_l as *mut libc::c_void);
        free((*s).fmin_r as *mut libc::c_void);
        free((*s).fmin_l as *mut libc::c_void);
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fsum2_l\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            465 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .fsum2_r = malloc(
        dim.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).fsum2_r).is_null() {
        free((*s).fsum2_l as *mut libc::c_void);
        free((*s).fsum_r as *mut libc::c_void);
        free((*s).fsum_l as *mut libc::c_void);
        free((*s).fmin_r as *mut libc::c_void);
        free((*s).fmin_l as *mut libc::c_void);
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fsum2_r\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .hits_r = malloc(
        dim.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    if ((*s).hits_r).is_null() {
        free((*s).fsum2_r as *mut libc::c_void);
        free((*s).fsum2_l as *mut libc::c_void);
        free((*s).fsum_r as *mut libc::c_void);
        free((*s).fsum_l as *mut libc::c_void);
        free((*s).fmin_r as *mut libc::c_void);
        free((*s).fmin_l as *mut libc::c_void);
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fsum2_r\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            505 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s)
        .hits_l = malloc(
        dim.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    if ((*s).hits_l).is_null() {
        free((*s).hits_r as *mut libc::c_void);
        free((*s).fsum2_r as *mut libc::c_void);
        free((*s).fsum2_l as *mut libc::c_void);
        free((*s).fsum_r as *mut libc::c_void);
        free((*s).fsum_l as *mut libc::c_void);
        free((*s).fmin_r as *mut libc::c_void);
        free((*s).fmin_l as *mut libc::c_void);
        free((*s).fmax_r as *mut libc::c_void);
        free((*s).fmax_l as *mut libc::c_void);
        free((*s).sigma_r as *mut libc::c_void);
        free((*s).sigma_l as *mut libc::c_void);
        free((*s).xmid as *mut libc::c_void);
        free((*s).x as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for fsum2_r\0" as *const u8
                as *const libc::c_char,
            b"miser.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_monte_miser_state;
    }
    (*s).dim = dim;
    gsl_monte_miser_init(s);
    return s;
}
pub unsafe extern "C" fn gsl_monte_miser_init(
    mut s: *mut gsl_monte_miser_state,
) -> libc::c_int {
    (*s).min_calls = (16 as libc::c_int as libc::c_ulong).wrapping_mul((*s).dim);
    (*s)
        .min_calls_per_bisection = (32 as libc::c_int as libc::c_ulong)
        .wrapping_mul((*s).min_calls);
    (*s).estimate_frac = 0.1f64;
    (*s).alpha = 2.0f64;
    (*s).dither = 0.0f64;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_monte_miser_free(mut s: *mut gsl_monte_miser_state) {
    if s.is_null() {
        return;
    }
    free((*s).hits_r as *mut libc::c_void);
    free((*s).hits_l as *mut libc::c_void);
    free((*s).fsum2_r as *mut libc::c_void);
    free((*s).fsum2_l as *mut libc::c_void);
    free((*s).fsum_r as *mut libc::c_void);
    free((*s).fsum_l as *mut libc::c_void);
    free((*s).fmin_r as *mut libc::c_void);
    free((*s).fmin_l as *mut libc::c_void);
    free((*s).fmax_r as *mut libc::c_void);
    free((*s).fmax_l as *mut libc::c_void);
    free((*s).sigma_r as *mut libc::c_void);
    free((*s).sigma_l as *mut libc::c_void);
    free((*s).xmid as *mut libc::c_void);
    free((*s).x as *mut libc::c_void);
    free(s as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_monte_miser_params_get(
    mut s: *const gsl_monte_miser_state,
    mut p: *mut gsl_monte_miser_params,
) {
    (*p).estimate_frac = (*s).estimate_frac;
    (*p).min_calls = (*s).min_calls;
    (*p).min_calls_per_bisection = (*s).min_calls_per_bisection;
    (*p).alpha = (*s).alpha;
    (*p).dither = (*s).dither;
}
pub unsafe extern "C" fn gsl_monte_miser_params_set(
    mut s: *mut gsl_monte_miser_state,
    mut p: *const gsl_monte_miser_params,
) {
    (*s).estimate_frac = (*p).estimate_frac;
    (*s).min_calls = (*p).min_calls;
    (*s).min_calls_per_bisection = (*p).min_calls_per_bisection;
    (*s).alpha = (*p).alpha;
    (*s).dither = (*p).dither;
}
unsafe extern "C" fn estimate_corrmc(
    mut f: *mut gsl_monte_function,
    mut xl: *const libc::c_double,
    mut xu: *const libc::c_double,
    mut dim: size_t,
    mut calls: size_t,
    mut r: *mut gsl_rng,
    mut state: *mut gsl_monte_miser_state,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut xmid: *const libc::c_double,
    mut sigma_l: *mut libc::c_double,
    mut sigma_r: *mut libc::c_double,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut x: *mut libc::c_double = (*state).x;
    let mut fsum_l: *mut libc::c_double = (*state).fsum_l;
    let mut fsum_r: *mut libc::c_double = (*state).fsum_r;
    let mut fsum2_l: *mut libc::c_double = (*state).fsum2_l;
    let mut fsum2_r: *mut libc::c_double = (*state).fsum2_r;
    let mut hits_l: *mut size_t = (*state).hits_l;
    let mut hits_r: *mut size_t = (*state).hits_r;
    let mut m: libc::c_double = 0.0f64;
    let mut q: libc::c_double = 0.0f64;
    let mut vol: libc::c_double = 1.0f64;
    i = 0 as libc::c_int as size_t;
    while i < dim {
        vol *= *xu.offset(i as isize) - *xl.offset(i as isize);
        let ref mut fresh0 = *hits_r.offset(i as isize);
        *fresh0 = 0 as libc::c_int as size_t;
        *hits_l.offset(i as isize) = *fresh0;
        let ref mut fresh1 = *fsum_r.offset(i as isize);
        *fresh1 = 0.0f64;
        *fsum_l.offset(i as isize) = *fresh1;
        let ref mut fresh2 = *fsum2_r.offset(i as isize);
        *fresh2 = 0.0f64;
        *fsum2_l.offset(i as isize) = *fresh2;
        let ref mut fresh3 = *sigma_r.offset(i as isize);
        *fresh3 = -(1 as libc::c_int) as libc::c_double;
        *sigma_l.offset(i as isize) = *fresh3;
        i = i.wrapping_add(1);
        i;
    }
    n = 0 as libc::c_int as size_t;
    while n < calls {
        let mut fval: libc::c_double = 0.;
        let mut j: libc::c_uint = n
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_rem(dim) as libc::c_uint;
        let mut side: libc::c_uint = n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            as libc::c_uint;
        i = 0 as libc::c_int as size_t;
        while i < dim {
            let mut z: libc::c_double = gsl_rng_uniform_pos(r);
            if i != j as libc::c_ulong {
                *x
                    .offset(
                        i as isize,
                    ) = *xl.offset(i as isize)
                    + z * (*xu.offset(i as isize) - *xl.offset(i as isize));
            } else if side == 0 as libc::c_int as libc::c_uint {
                *x
                    .offset(
                        i as isize,
                    ) = *xmid.offset(i as isize)
                    + z * (*xu.offset(i as isize) - *xmid.offset(i as isize));
            } else {
                *x
                    .offset(
                        i as isize,
                    ) = *xl.offset(i as isize)
                    + z * (*xmid.offset(i as isize) - *xl.offset(i as isize));
            }
            i = i.wrapping_add(1);
            i;
        }
        fval = (Some(((*f).f).unwrap())).unwrap()(x, (*f).dim, (*f).params);
        let mut d: libc::c_double = fval - m;
        m += d / (n as libc::c_double + 1.0f64);
        q += d * d * (n as libc::c_double / (n as libc::c_double + 1.0f64));
        i = 0 as libc::c_int as size_t;
        while i < dim {
            if *x.offset(i as isize) <= *xmid.offset(i as isize) {
                *fsum_l.offset(i as isize) += fval;
                *fsum2_l.offset(i as isize) += fval * fval;
                let ref mut fresh4 = *hits_l.offset(i as isize);
                *fresh4 = (*fresh4).wrapping_add(1);
                *fresh4;
            } else {
                *fsum_r.offset(i as isize) += fval;
                *fsum2_r.offset(i as isize) += fval * fval;
                let ref mut fresh5 = *hits_r.offset(i as isize);
                *fresh5 = (*fresh5).wrapping_add(1);
                *fresh5;
            }
            i = i.wrapping_add(1);
            i;
        }
        n = n.wrapping_add(1);
        n;
    }
    i = 0 as libc::c_int as size_t;
    while i < dim {
        let mut fraction_l: libc::c_double = (*xmid.offset(i as isize)
            - *xl.offset(i as isize))
            / (*xu.offset(i as isize) - *xl.offset(i as isize));
        if *hits_l.offset(i as isize) > 0 as libc::c_int as libc::c_ulong {
            *fsum_l.offset(i as isize) /= *hits_l.offset(i as isize) as libc::c_double;
            *sigma_l
                .offset(
                    i as isize,
                ) = sqrt(
                *fsum2_l.offset(i as isize)
                    - *fsum_l.offset(i as isize) * *fsum_l.offset(i as isize)
                        / *hits_l.offset(i as isize) as libc::c_double,
            );
            *sigma_l.offset(i as isize)
                *= fraction_l * vol / *hits_l.offset(i as isize) as libc::c_double;
        }
        if *hits_r.offset(i as isize) > 0 as libc::c_int as libc::c_ulong {
            *fsum_r.offset(i as isize) /= *hits_r.offset(i as isize) as libc::c_double;
            *sigma_r
                .offset(
                    i as isize,
                ) = sqrt(
                *fsum2_r.offset(i as isize)
                    - *fsum_r.offset(i as isize) * *fsum_r.offset(i as isize)
                        / *hits_r.offset(i as isize) as libc::c_double,
            );
            *sigma_r.offset(i as isize)
                *= (1 as libc::c_int as libc::c_double - fraction_l) * vol
                    / *hits_r.offset(i as isize) as libc::c_double;
        }
        i = i.wrapping_add(1);
        i;
    }
    *result = vol * m;
    if calls < 2 as libc::c_int as libc::c_ulong {
        *abserr = ::std::f32::INFINITY as libc::c_double;
    } else {
        *abserr = vol
            * sqrt(q / (calls as libc::c_double * (calls as libc::c_double - 1.0f64)));
    }
    return GSL_SUCCESS as libc::c_int;
}
