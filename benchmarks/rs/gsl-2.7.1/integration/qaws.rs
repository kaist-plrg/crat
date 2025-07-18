use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub struct gsl_integration_qaws_table {
    pub alpha: libc::c_double,
    pub beta: libc::c_double,
    pub mu: libc::c_int,
    pub nu: libc::c_int,
    pub ri: [libc::c_double; 25],
    pub rj: [libc::c_double; 25],
    pub rg: [libc::c_double; 25],
    pub rh: [libc::c_double; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fn_qaws_params {
    pub function: *mut gsl_function,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub table: *mut gsl_integration_qaws_table,
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
unsafe extern "C" fn append_interval(
    mut workspace: *mut gsl_integration_workspace,
    mut a1: libc::c_double,
    mut b1: libc::c_double,
    mut area1: libc::c_double,
    mut error1: libc::c_double,
) {
    let i_new: size_t = (*workspace).size;
    *((*workspace).alist).offset(i_new as isize) = a1;
    *((*workspace).blist).offset(i_new as isize) = b1;
    *((*workspace).rlist).offset(i_new as isize) = area1;
    *((*workspace).elist).offset(i_new as isize) = error1;
    *((*workspace).order).offset(i_new as isize) = i_new;
    *((*workspace).level).offset(i_new as isize) = 0 as libc::c_int as size_t;
    (*workspace).size = ((*workspace).size).wrapping_add(1);
    (*workspace).size;
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
unsafe extern "C" fn qc25s(
    mut f: *mut gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut a1: libc::c_double,
    mut b1: libc::c_double,
    mut t: *mut gsl_integration_qaws_table,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut err_reliable: *mut libc::c_int,
) {
    let mut weighted_function: gsl_function = gsl_function {
        function: None,
        params: 0 as *mut libc::c_void,
    };
    let mut fn_params: fn_qaws_params = fn_qaws_params {
        function: 0 as *mut gsl_function,
        a: 0.,
        b: 0.,
        table: 0 as *mut gsl_integration_qaws_table,
    };
    fn_params.function = f;
    fn_params.a = a;
    fn_params.b = b;
    fn_params.table = t;
    weighted_function
        .params = &mut fn_params as *mut fn_qaws_params as *mut libc::c_void;
    if a1 == a && ((*t).alpha != 0.0f64 || (*t).mu != 0 as libc::c_int) {
        let mut cheb12: [libc::c_double; 13] = [0.; 13];
        let mut cheb24: [libc::c_double; 25] = [0.; 25];
        let mut factor: libc::c_double = pow(0.5f64 * (b1 - a1), (*t).alpha + 1.0f64);
        weighted_function
            .function = Some(
            fn_qaws_R
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_void,
                ) -> libc::c_double,
        );
        gsl_integration_qcheb(
            &mut weighted_function,
            a1,
            b1,
            cheb12.as_mut_ptr(),
            cheb24.as_mut_ptr(),
        );
        if (*t).mu == 0 as libc::c_int {
            let mut res12: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res24: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut u: libc::c_double = factor;
            compute_result(
                ((*t).ri).as_mut_ptr(),
                cheb12.as_mut_ptr(),
                cheb24.as_mut_ptr(),
                &mut res12,
                &mut res24,
            );
            *result = u * res24;
            *abserr = fabs(u * (res24 - res12));
        } else {
            let mut res12a: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res24a: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res12b: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res24b: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut u_0: libc::c_double = factor * log(b1 - a1);
            let mut v: libc::c_double = factor;
            compute_result(
                ((*t).ri).as_mut_ptr(),
                cheb12.as_mut_ptr(),
                cheb24.as_mut_ptr(),
                &mut res12a,
                &mut res24a,
            );
            compute_result(
                ((*t).rg).as_mut_ptr(),
                cheb12.as_mut_ptr(),
                cheb24.as_mut_ptr(),
                &mut res12b,
                &mut res24b,
            );
            *result = u_0 * res24a + v * res24b;
            *abserr = fabs(u_0 * (res24a - res12a)) + fabs(v * (res24b - res12b));
        }
        *err_reliable = 0 as libc::c_int;
        return;
    } else if b1 == b && ((*t).beta != 0.0f64 || (*t).nu != 0 as libc::c_int) {
        let mut cheb12_0: [libc::c_double; 13] = [0.; 13];
        let mut cheb24_0: [libc::c_double; 25] = [0.; 25];
        let mut factor_0: libc::c_double = pow(0.5f64 * (b1 - a1), (*t).beta + 1.0f64);
        weighted_function
            .function = Some(
            fn_qaws_L
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_void,
                ) -> libc::c_double,
        );
        gsl_integration_qcheb(
            &mut weighted_function,
            a1,
            b1,
            cheb12_0.as_mut_ptr(),
            cheb24_0.as_mut_ptr(),
        );
        if (*t).nu == 0 as libc::c_int {
            let mut res12_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res24_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut u_1: libc::c_double = factor_0;
            compute_result(
                ((*t).rj).as_mut_ptr(),
                cheb12_0.as_mut_ptr(),
                cheb24_0.as_mut_ptr(),
                &mut res12_0,
                &mut res24_0,
            );
            *result = u_1 * res24_0;
            *abserr = fabs(u_1 * (res24_0 - res12_0));
        } else {
            let mut res12a_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res24a_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res12b_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut res24b_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut u_2: libc::c_double = factor_0 * log(b1 - a1);
            let mut v_0: libc::c_double = factor_0;
            compute_result(
                ((*t).rj).as_mut_ptr(),
                cheb12_0.as_mut_ptr(),
                cheb24_0.as_mut_ptr(),
                &mut res12a_0,
                &mut res24a_0,
            );
            compute_result(
                ((*t).rh).as_mut_ptr(),
                cheb12_0.as_mut_ptr(),
                cheb24_0.as_mut_ptr(),
                &mut res12b_0,
                &mut res24b_0,
            );
            *result = u_2 * res24a_0 + v_0 * res24b_0;
            *abserr = fabs(u_2 * (res24a_0 - res12a_0))
                + fabs(v_0 * (res24b_0 - res12b_0));
        }
        *err_reliable = 0 as libc::c_int;
        return;
    } else {
        let mut resabs: libc::c_double = 0.;
        let mut resasc: libc::c_double = 0.;
        weighted_function
            .function = Some(
            fn_qaws
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_void,
                ) -> libc::c_double,
        );
        gsl_integration_qk15(
            &mut weighted_function,
            a1,
            b1,
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
    };
}
unsafe extern "C" fn fn_qaws(
    mut x: libc::c_double,
    mut params: *mut libc::c_void,
) -> libc::c_double {
    let mut p: *mut fn_qaws_params = params as *mut fn_qaws_params;
    let mut f: *mut gsl_function = (*p).function;
    let mut t: *mut gsl_integration_qaws_table = (*p).table;
    let mut factor: libc::c_double = 1.0f64;
    if (*t).alpha != 0.0f64 {
        factor *= pow(x - (*p).a, (*t).alpha);
    }
    if (*t).beta != 0.0f64 {
        factor *= pow((*p).b - x, (*t).beta);
    }
    if (*t).mu == 1 as libc::c_int {
        factor *= log(x - (*p).a);
    }
    if (*t).nu == 1 as libc::c_int {
        factor *= log((*p).b - x);
    }
    return factor * (Some(((*f).function).unwrap())).unwrap()(x, (*f).params);
}
unsafe extern "C" fn fn_qaws_L(
    mut x: libc::c_double,
    mut params: *mut libc::c_void,
) -> libc::c_double {
    let mut p: *mut fn_qaws_params = params as *mut fn_qaws_params;
    let mut f: *mut gsl_function = (*p).function;
    let mut t: *mut gsl_integration_qaws_table = (*p).table;
    let mut factor: libc::c_double = 1.0f64;
    if (*t).alpha != 0.0f64 {
        factor *= pow(x - (*p).a, (*t).alpha);
    }
    if (*t).mu == 1 as libc::c_int {
        factor *= log(x - (*p).a);
    }
    return factor * (Some(((*f).function).unwrap())).unwrap()(x, (*f).params);
}
unsafe extern "C" fn fn_qaws_R(
    mut x: libc::c_double,
    mut params: *mut libc::c_void,
) -> libc::c_double {
    let mut p: *mut fn_qaws_params = params as *mut fn_qaws_params;
    let mut f: *mut gsl_function = (*p).function;
    let mut t: *mut gsl_integration_qaws_table = (*p).table;
    let mut factor: libc::c_double = 1.0f64;
    if (*t).beta != 0.0f64 {
        factor *= pow((*p).b - x, (*t).beta);
    }
    if (*t).nu == 1 as libc::c_int {
        factor *= log((*p).b - x);
    }
    return factor * (Some(((*f).function).unwrap())).unwrap()(x, (*f).params);
}
unsafe extern "C" fn compute_result(
    mut r: *const libc::c_double,
    mut cheb12: *const libc::c_double,
    mut cheb24: *const libc::c_double,
    mut result12: *mut libc::c_double,
    mut result24: *mut libc::c_double,
) {
    let mut i: size_t = 0;
    let mut res12: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut res24: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < 13 as libc::c_int as libc::c_ulong {
        res12 += *r.offset(i as isize) * *cheb12.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 25 as libc::c_int as libc::c_ulong {
        res24 += *r.offset(i as isize) * *cheb24.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    *result12 = res12;
    *result24 = res24;
}
pub unsafe extern "C" fn gsl_integration_qaws(
    mut f: *mut gsl_function,
    a: libc::c_double,
    b: libc::c_double,
    mut t: *mut gsl_integration_qaws_table,
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
    initialise(workspace, a, b);
    *result = 0 as libc::c_int as libc::c_double;
    *abserr = 0 as libc::c_int as libc::c_double;
    if limit > (*workspace).limit {
        gsl_error(
            b"iteration limit exceeds available workspace\0" as *const u8
                as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if b <= a {
        gsl_error(
            b"limits must form an ascending sequence, a < b\0" as *const u8
                as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if epsabs <= 0 as libc::c_int as libc::c_double
        && (epsrel < 50 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
            || epsrel < 0.5e-28f64)
    {
        gsl_error(
            b"tolerance cannot be achieved with given epsabs and epsrel\0" as *const u8
                as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    let mut area1: libc::c_double = 0.;
    let mut area2: libc::c_double = 0.;
    let mut error1: libc::c_double = 0.;
    let mut error2: libc::c_double = 0.;
    let mut err_reliable1: libc::c_int = 0;
    let mut err_reliable2: libc::c_int = 0;
    let mut a1: libc::c_double = a;
    let mut b1: libc::c_double = 0.5f64 * (a + b);
    let mut a2: libc::c_double = b1;
    let mut b2: libc::c_double = b;
    qc25s(f, a, b, a1, b1, t, &mut area1, &mut error1, &mut err_reliable1);
    qc25s(f, a, b, a2, b2, t, &mut area2, &mut error2, &mut err_reliable2);
    if error1 > error2 {
        append_interval(workspace, a1, b1, area1, error1);
        append_interval(workspace, a2, b2, area2, error2);
    } else {
        append_interval(workspace, a2, b2, area2, error2);
        append_interval(workspace, a1, b1, area1, error1);
    }
    result0 = area1 + area2;
    abserr0 = error1 + error2;
    tolerance = GSL_MAX_DBL(epsabs, epsrel * fabs(result0));
    if abserr0 < tolerance && abserr0 < 0.01f64 * fabs(result0) {
        *result = result0;
        *abserr = abserr0;
        return GSL_SUCCESS as libc::c_int;
    } else if limit == 1 as libc::c_int as libc::c_ulong {
        *result = result0;
        *abserr = abserr0;
        gsl_error(
            b"a maximum of one iteration was insufficient\0" as *const u8
                as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    }
    area = result0;
    errsum = abserr0;
    iteration = 2 as libc::c_int as size_t;
    loop {
        let mut a1_0: libc::c_double = 0.;
        let mut b1_0: libc::c_double = 0.;
        let mut a2_0: libc::c_double = 0.;
        let mut b2_0: libc::c_double = 0.;
        let mut a_i: libc::c_double = 0.;
        let mut b_i: libc::c_double = 0.;
        let mut r_i: libc::c_double = 0.;
        let mut e_i: libc::c_double = 0.;
        let mut area1_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut area2_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut area12: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error1_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error2_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error12: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut err_reliable1_0: libc::c_int = 0;
        let mut err_reliable2_0: libc::c_int = 0;
        retrieve(workspace, &mut a_i, &mut b_i, &mut r_i, &mut e_i);
        a1_0 = a_i;
        b1_0 = 0.5f64 * (a_i + b_i);
        a2_0 = b1_0;
        b2_0 = b_i;
        qc25s(f, a, b, a1_0, b1_0, t, &mut area1_0, &mut error1_0, &mut err_reliable1_0);
        qc25s(f, a, b, a2_0, b2_0, t, &mut area2_0, &mut error2_0, &mut err_reliable2_0);
        area12 = area1_0 + area2_0;
        error12 = error1_0 + error2_0;
        errsum += error12 - e_i;
        area += area12 - r_i;
        if err_reliable1_0 != 0 && err_reliable2_0 != 0 {
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
            if subinterval_too_small(a1_0, a2_0, b2_0) != 0 {
                error_type = 3 as libc::c_int;
            }
        }
        update(workspace, a1_0, b1_0, area1_0, error1_0, a2_0, b2_0, area2_0, error2_0);
        retrieve(workspace, &mut a_i, &mut b_i, &mut r_i, &mut e_i);
        iteration = iteration.wrapping_add(1);
        iteration;
        if !(iteration < limit && error_type == 0 && errsum > tolerance) {
            break;
        }
    }
    *result = sum_results(workspace);
    *abserr = errsum;
    if errsum <= tolerance {
        return GSL_SUCCESS as libc::c_int
    } else if error_type == 2 as libc::c_int {
        gsl_error(
            b"roundoff error prevents tolerance from being achieved\0" as *const u8
                as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    } else if error_type == 3 as libc::c_int {
        gsl_error(
            b"bad integrand behavior found in the integration interval\0" as *const u8
                as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_ESING as libc::c_int,
        );
        return GSL_ESING as libc::c_int;
    } else if iteration == limit {
        gsl_error(
            b"maximum number of subdivisions reached\0" as *const u8
                as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        gsl_error(
            b"could not integrate function\0" as *const u8 as *const libc::c_char,
            b"qaws.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    };
}
