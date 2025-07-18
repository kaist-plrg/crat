use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_integration_qk15(
        f: *const gsl_function,
        a: libc::c_double,
        b: libc::c_double,
        result: *mut libc::c_double,
        abserr: *mut libc::c_double,
        resabs: *mut libc::c_double,
        resasc: *mut libc::c_double,
    );
    fn gsl_integration_qcheb(
        f: *mut gsl_function,
        a: libc::c_double,
        b: libc::c_double,
        cheb12: *mut libc::c_double,
        cheb24: *mut libc::c_double,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
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
pub struct gsl_integration_workspace {
    pub limit: size_t,
    pub size: size_t,
    pub nrmax: size_t,
    pub i: size_t,
    pub maximum_level: size_t,
    pub alist: *mut libc::c_double,
    pub blist: *mut libc::c_double,
    pub rlist: *mut libc::c_double,
    pub elist: *mut libc::c_double,
    pub order: *mut size_t,
    pub level: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fn_cauchy_params {
    pub function: *mut gsl_function,
    pub singularity: libc::c_double,
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn initialise(
    mut workspace: *mut gsl_integration_workspace,
    mut a: libc::c_double,
    mut b: libc::c_double,
) {
    (*workspace).size = 0 as libc::c_int as size_t;
    (*workspace).nrmax = 0 as libc::c_int as size_t;
    (*workspace).i = 0 as libc::c_int as size_t;
    *((*workspace).alist).offset(0 as libc::c_int as isize) = a;
    *((*workspace).blist).offset(0 as libc::c_int as isize) = b;
    *((*workspace).rlist).offset(0 as libc::c_int as isize) = 0.0f64;
    *((*workspace).elist).offset(0 as libc::c_int as isize) = 0.0f64;
    *((*workspace).order).offset(0 as libc::c_int as isize) = 0 as libc::c_int as size_t;
    *((*workspace).level).offset(0 as libc::c_int as isize) = 0 as libc::c_int as size_t;
    (*workspace).maximum_level = 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn set_initial_result(
    mut workspace: *mut gsl_integration_workspace,
    mut result: libc::c_double,
    mut error: libc::c_double,
) {
    (*workspace).size = 1 as libc::c_int as size_t;
    *((*workspace).rlist).offset(0 as libc::c_int as isize) = result;
    *((*workspace).elist).offset(0 as libc::c_int as isize) = error;
}
#[inline]
unsafe extern "C" fn qpsrt(mut workspace: *mut gsl_integration_workspace) {
    let last: size_t = ((*workspace).size)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let limit: size_t = (*workspace).limit;
    let mut elist: *mut libc::c_double = (*workspace).elist;
    let mut order: *mut size_t = (*workspace).order;
    let mut errmax: libc::c_double = 0.;
    let mut errmin: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut i_nrmax: size_t = (*workspace).nrmax;
    let mut i_maxerr: size_t = *order.offset(i_nrmax as isize);
    if last < 2 as libc::c_int as libc::c_ulong {
        *order.offset(0 as libc::c_int as isize) = 0 as libc::c_int as size_t;
        *order.offset(1 as libc::c_int as isize) = 1 as libc::c_int as size_t;
        (*workspace).i = i_maxerr;
        return;
    }
    errmax = *elist.offset(i_maxerr as isize);
    while i_nrmax > 0 as libc::c_int as libc::c_ulong
        && errmax
            > *elist
                .offset(
                    *order
                        .offset(
                            i_nrmax.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as isize,
                )
    {
        *order
            .offset(
                i_nrmax as isize,
            ) = *order
            .offset(i_nrmax.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        i_nrmax = i_nrmax.wrapping_sub(1);
        i_nrmax;
    }
    if last
        < limit
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
    {
        top = last as libc::c_int;
    } else {
        top = limit.wrapping_sub(last).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
    }
    i = i_nrmax.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i < top && errmax < *elist.offset(*order.offset(i as isize) as isize) {
        *order.offset((i - 1 as libc::c_int) as isize) = *order.offset(i as isize);
        i += 1;
        i;
    }
    *order.offset((i - 1 as libc::c_int) as isize) = i_maxerr;
    errmin = *elist.offset(last as isize);
    k = top - 1 as libc::c_int;
    while k > i - 2 as libc::c_int
        && errmin >= *elist.offset(*order.offset(k as isize) as isize)
    {
        *order.offset((k + 1 as libc::c_int) as isize) = *order.offset(k as isize);
        k -= 1;
        k;
    }
    *order.offset((k + 1 as libc::c_int) as isize) = last;
    i_maxerr = *order.offset(i_nrmax as isize);
    (*workspace).i = i_maxerr;
    (*workspace).nrmax = i_nrmax;
}
#[inline]
unsafe extern "C" fn update(
    mut workspace: *mut gsl_integration_workspace,
    mut a1: libc::c_double,
    mut b1: libc::c_double,
    mut area1: libc::c_double,
    mut error1: libc::c_double,
    mut a2: libc::c_double,
    mut b2: libc::c_double,
    mut area2: libc::c_double,
    mut error2: libc::c_double,
) {
    let mut alist: *mut libc::c_double = (*workspace).alist;
    let mut blist: *mut libc::c_double = (*workspace).blist;
    let mut rlist: *mut libc::c_double = (*workspace).rlist;
    let mut elist: *mut libc::c_double = (*workspace).elist;
    let mut level: *mut size_t = (*workspace).level;
    let i_max: size_t = (*workspace).i;
    let i_new: size_t = (*workspace).size;
    let new_level: size_t = (*((*workspace).level).offset(i_max as isize))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if error2 > error1 {
        *alist.offset(i_max as isize) = a2;
        *rlist.offset(i_max as isize) = area2;
        *elist.offset(i_max as isize) = error2;
        *level.offset(i_max as isize) = new_level;
        *alist.offset(i_new as isize) = a1;
        *blist.offset(i_new as isize) = b1;
        *rlist.offset(i_new as isize) = area1;
        *elist.offset(i_new as isize) = error1;
        *level.offset(i_new as isize) = new_level;
    } else {
        *blist.offset(i_max as isize) = b1;
        *rlist.offset(i_max as isize) = area1;
        *elist.offset(i_max as isize) = error1;
        *level.offset(i_max as isize) = new_level;
        *alist.offset(i_new as isize) = a2;
        *blist.offset(i_new as isize) = b2;
        *rlist.offset(i_new as isize) = area2;
        *elist.offset(i_new as isize) = error2;
        *level.offset(i_new as isize) = new_level;
    }
    (*workspace).size = ((*workspace).size).wrapping_add(1);
    (*workspace).size;
    if new_level > (*workspace).maximum_level {
        (*workspace).maximum_level = new_level;
    }
    qpsrt(workspace);
}
#[inline]
unsafe extern "C" fn retrieve(
    mut workspace: *const gsl_integration_workspace,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut r: *mut libc::c_double,
    mut e: *mut libc::c_double,
) {
    let i: size_t = (*workspace).i;
    let mut alist: *mut libc::c_double = (*workspace).alist;
    let mut blist: *mut libc::c_double = (*workspace).blist;
    let mut rlist: *mut libc::c_double = (*workspace).rlist;
    let mut elist: *mut libc::c_double = (*workspace).elist;
    *a = *alist.offset(i as isize);
    *b = *blist.offset(i as isize);
    *r = *rlist.offset(i as isize);
    *e = *elist.offset(i as isize);
}
#[inline]
unsafe extern "C" fn sum_results(
    mut workspace: *const gsl_integration_workspace,
) -> libc::c_double {
    let rlist: *const libc::c_double = (*workspace).rlist;
    let n: size_t = (*workspace).size;
    let mut k: size_t = 0;
    let mut result_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    k = 0 as libc::c_int as size_t;
    while k < n {
        result_sum += *rlist.offset(k as isize);
        k = k.wrapping_add(1);
        k;
    }
    return result_sum;
}
#[inline]
unsafe extern "C" fn subinterval_too_small(
    mut a1: libc::c_double,
    mut a2: libc::c_double,
    mut b2: libc::c_double,
) -> libc::c_int {
    let e: libc::c_double = 2.2204460492503131e-16f64;
    let u: libc::c_double = 2.2250738585072014e-308f64;
    let mut tmp: libc::c_double = (1 as libc::c_int as libc::c_double
        + 100 as libc::c_int as libc::c_double * e)
        * (fabs(a2) + 1000 as libc::c_int as libc::c_double * u);
    let mut status: libc::c_int = (fabs(a1) <= tmp && fabs(b2) <= tmp) as libc::c_int;
    return status;
}
unsafe extern "C" fn qc25c(
    mut f: *mut gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut err_reliable: *mut libc::c_int,
) {
    let mut cc: libc::c_double = (2 as libc::c_int as libc::c_double * c - b - a)
        / (b - a);
    if fabs(cc) > 1.1f64 {
        let mut resabs: libc::c_double = 0.;
        let mut resasc: libc::c_double = 0.;
        let mut weighted_function: gsl_function = gsl_function {
            function: None,
            params: 0 as *mut libc::c_void,
        };
        let mut fn_params: fn_cauchy_params = fn_cauchy_params {
            function: 0 as *mut gsl_function,
            singularity: 0.,
        };
        fn_params.function = f;
        fn_params.singularity = c;
        weighted_function
            .function = Some(
            fn_cauchy
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_void,
                ) -> libc::c_double,
        );
        weighted_function
            .params = &mut fn_params as *mut fn_cauchy_params as *mut libc::c_void;
        gsl_integration_qk15(
            &mut weighted_function,
            a,
            b,
            result,
            abserr,
            &mut resabs,
            &mut resasc,
        );
        if *abserr == resasc {
            *err_reliable = 0 as libc::c_int;
        } else {
            *err_reliable = 1 as libc::c_int;
        }
        return;
    } else {
        let mut cheb12: [libc::c_double; 13] = [0.; 13];
        let mut cheb24: [libc::c_double; 25] = [0.; 25];
        let mut moment: [libc::c_double; 25] = [0.; 25];
        let mut res12: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut res24: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i: size_t = 0;
        gsl_integration_qcheb(f, a, b, cheb12.as_mut_ptr(), cheb24.as_mut_ptr());
        compute_moments(cc, moment.as_mut_ptr());
        i = 0 as libc::c_int as size_t;
        while i < 13 as libc::c_int as libc::c_ulong {
            res12 += cheb12[i as usize] * moment[i as usize];
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < 25 as libc::c_int as libc::c_ulong {
            res24 += cheb24[i as usize] * moment[i as usize];
            i = i.wrapping_add(1);
            i;
        }
        *result = res24;
        *abserr = fabs(res24 - res12);
        *err_reliable = 0 as libc::c_int;
        return;
    };
}
unsafe extern "C" fn fn_cauchy(
    mut x: libc::c_double,
    mut params: *mut libc::c_void,
) -> libc::c_double {
    let mut p: *mut fn_cauchy_params = params as *mut fn_cauchy_params;
    let mut f: *mut gsl_function = (*p).function;
    let mut c: libc::c_double = (*p).singularity;
    return (Some(((*f).function).unwrap())).unwrap()(x, (*f).params) / (x - c);
}
unsafe extern "C" fn compute_moments(
    mut cc: libc::c_double,
    mut moment: *mut libc::c_double,
) {
    let mut k: size_t = 0;
    let mut a0: libc::c_double = log(fabs((1.0f64 - cc) / (1.0f64 + cc)));
    let mut a1: libc::c_double = 2 as libc::c_int as libc::c_double + a0 * cc;
    *moment.offset(0 as libc::c_int as isize) = a0;
    *moment.offset(1 as libc::c_int as isize) = a1;
    k = 2 as libc::c_int as size_t;
    while k < 25 as libc::c_int as libc::c_ulong {
        let mut a2: libc::c_double = 0.;
        if k.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            a2 = 2.0f64 * cc * a1 - a0;
        } else {
            let km1: libc::c_double = k as libc::c_double - 1.0f64;
            a2 = 2.0f64 * cc * a1 - a0 - 4.0f64 / (km1 * km1 - 1.0f64);
        }
        *moment.offset(k as isize) = a2;
        a0 = a1;
        a1 = a2;
        k = k.wrapping_add(1);
        k;
    }
}
pub unsafe extern "C" fn gsl_integration_qawc(
    mut f: *mut gsl_function,
    a: libc::c_double,
    b: libc::c_double,
    c: libc::c_double,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
    limit: size_t,
    mut workspace: *mut gsl_integration_workspace,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut area: libc::c_double = 0.;
    let mut errsum: libc::c_double = 0.;
    let mut result0: libc::c_double = 0.;
    let mut abserr0: libc::c_double = 0.;
    let mut tolerance: libc::c_double = 0.;
    let mut iteration: size_t = 0 as libc::c_int as size_t;
    let mut roundoff_type1: libc::c_int = 0 as libc::c_int;
    let mut roundoff_type2: libc::c_int = 0 as libc::c_int;
    let mut error_type: libc::c_int = 0 as libc::c_int;
    let mut err_reliable: libc::c_int = 0;
    let mut sign: libc::c_int = 1 as libc::c_int;
    let mut lower: libc::c_double = 0.;
    let mut higher: libc::c_double = 0.;
    *result = 0 as libc::c_int as libc::c_double;
    *abserr = 0 as libc::c_int as libc::c_double;
    if limit > (*workspace).limit {
        gsl_error(
            b"iteration limit exceeds available workspace\0" as *const u8
                as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if b < a {
        lower = b;
        higher = a;
        sign = -(1 as libc::c_int);
    } else {
        lower = a;
        higher = b;
    }
    initialise(workspace, lower, higher);
    if epsabs <= 0 as libc::c_int as libc::c_double
        && (epsrel < 50 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
            || epsrel < 0.5e-28f64)
    {
        gsl_error(
            b"tolerance cannot be achieved with given epsabs and epsrel\0" as *const u8
                as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    if c == a || c == b {
        gsl_error(
            b"cannot integrate with singularity on endpoint\0" as *const u8
                as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    qc25c(f, lower, higher, c, &mut result0, &mut abserr0, &mut err_reliable);
    set_initial_result(workspace, result0, abserr0);
    tolerance = GSL_MAX_DBL(epsabs, epsrel * fabs(result0));
    if abserr0 < tolerance && abserr0 < 0.01f64 * fabs(result0) {
        *result = sign as libc::c_double * result0;
        *abserr = abserr0;
        return GSL_SUCCESS as libc::c_int;
    } else if limit == 1 as libc::c_int as libc::c_ulong {
        *result = sign as libc::c_double * result0;
        *abserr = abserr0;
        gsl_error(
            b"a maximum of one iteration was insufficient\0" as *const u8
                as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    }
    area = result0;
    errsum = abserr0;
    iteration = 1 as libc::c_int as size_t;
    loop {
        let mut a1: libc::c_double = 0.;
        let mut b1: libc::c_double = 0.;
        let mut a2: libc::c_double = 0.;
        let mut b2: libc::c_double = 0.;
        let mut a_i: libc::c_double = 0.;
        let mut b_i: libc::c_double = 0.;
        let mut r_i: libc::c_double = 0.;
        let mut e_i: libc::c_double = 0.;
        let mut area1: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut area2: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut area12: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error1: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error2: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error12: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut err_reliable1: libc::c_int = 0;
        let mut err_reliable2: libc::c_int = 0;
        retrieve(workspace, &mut a_i, &mut b_i, &mut r_i, &mut e_i);
        a1 = a_i;
        b1 = 0.5f64 * (a_i + b_i);
        a2 = b1;
        b2 = b_i;
        if c > a1 && c <= b1 {
            b1 = 0.5f64 * (c + b2);
            a2 = b1;
        } else if c > b1 && c < b2 {
            b1 = 0.5f64 * (a1 + c);
            a2 = b1;
        }
        qc25c(f, a1, b1, c, &mut area1, &mut error1, &mut err_reliable1);
        qc25c(f, a2, b2, c, &mut area2, &mut error2, &mut err_reliable2);
        area12 = area1 + area2;
        error12 = error1 + error2;
        errsum += error12 - e_i;
        area += area12 - r_i;
        if err_reliable1 != 0 && err_reliable2 != 0 {
            let mut delta: libc::c_double = r_i - area12;
            if fabs(delta) <= 1.0e-5f64 * fabs(area12) && error12 >= 0.99f64 * e_i {
                roundoff_type1 += 1;
                roundoff_type1;
            }
            if iteration >= 10 as libc::c_int as libc::c_ulong && error12 > e_i {
                roundoff_type2 += 1;
                roundoff_type2;
            }
        }
        tolerance = GSL_MAX_DBL(epsabs, epsrel * fabs(area));
        if errsum > tolerance {
            if roundoff_type1 >= 6 as libc::c_int || roundoff_type2 >= 20 as libc::c_int
            {
                error_type = 2 as libc::c_int;
            }
            if subinterval_too_small(a1, a2, b2) != 0 {
                error_type = 3 as libc::c_int;
            }
        }
        update(workspace, a1, b1, area1, error1, a2, b2, area2, error2);
        retrieve(workspace, &mut a_i, &mut b_i, &mut r_i, &mut e_i);
        iteration = iteration.wrapping_add(1);
        iteration;
        if !(iteration < limit && error_type == 0 && errsum > tolerance) {
            break;
        }
    }
    *result = sign as libc::c_double * sum_results(workspace);
    *abserr = errsum;
    if errsum <= tolerance {
        return GSL_SUCCESS as libc::c_int
    } else if error_type == 2 as libc::c_int {
        gsl_error(
            b"roundoff error prevents tolerance from being achieved\0" as *const u8
                as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    } else if error_type == 3 as libc::c_int {
        gsl_error(
            b"bad integrand behavior found in the integration interval\0" as *const u8
                as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_ESING as libc::c_int,
        );
        return GSL_ESING as libc::c_int;
    } else if iteration == limit {
        gsl_error(
            b"maximum number of subdivisions reached\0" as *const u8
                as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        gsl_error(
            b"could not integrate function\0" as *const u8 as *const libc::c_char,
            b"qawc.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    };
}
