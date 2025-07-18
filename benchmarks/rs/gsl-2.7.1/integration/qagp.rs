use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_integration_qk21(
        f: *const gsl_function,
        a: libc::c_double,
        b: libc::c_double,
        result: *mut libc::c_double,
        abserr: *mut libc::c_double,
        resabs: *mut libc::c_double,
        resasc: *mut libc::c_double,
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
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
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
pub type gsl_integration_rule = unsafe extern "C" fn(
    *const gsl_function,
    libc::c_double,
    libc::c_double,
    *mut libc::c_double,
    *mut libc::c_double,
    *mut libc::c_double,
    *mut libc::c_double,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extrapolation_table {
    pub n: size_t,
    pub rlist2: [libc::c_double; 52],
    pub nres: size_t,
    pub res3la: [libc::c_double; 3],
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
unsafe extern "C" fn reset_nrmax(mut workspace: *mut gsl_integration_workspace) {
    (*workspace).nrmax = 0 as libc::c_int as size_t;
    (*workspace).i = *((*workspace).order).offset(0 as libc::c_int as isize);
}
unsafe extern "C" fn initialise_table(mut table: *mut extrapolation_table) {
    (*table).n = 0 as libc::c_int as size_t;
    (*table).nres = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn append_table(
    mut table: *mut extrapolation_table,
    mut y: libc::c_double,
) {
    let mut n: size_t = 0;
    n = (*table).n;
    (*table).rlist2[n as usize] = y;
    (*table).n = ((*table).n).wrapping_add(1);
    (*table).n;
}
#[inline]
unsafe extern "C" fn qelg(
    mut table: *mut extrapolation_table,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) {
    let mut epstab: *mut libc::c_double = ((*table).rlist2).as_mut_ptr();
    let mut res3la: *mut libc::c_double = ((*table).res3la).as_mut_ptr();
    let n: size_t = ((*table).n).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let current: libc::c_double = *epstab.offset(n as isize);
    let mut absolute: libc::c_double = 1.7976931348623157e+308f64;
    let mut relative: libc::c_double = 5 as libc::c_int as libc::c_double
        * 2.2204460492503131e-16f64 * fabs(current);
    let newelm: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    let n_orig: size_t = n;
    let mut n_final: size_t = n;
    let mut i: size_t = 0;
    let nres_orig: size_t = (*table).nres;
    *result = current;
    *abserr = 1.7976931348623157e+308f64;
    if n < 2 as libc::c_int as libc::c_ulong {
        *result = current;
        *abserr = GSL_MAX_DBL(absolute, relative);
        return;
    }
    *epstab
        .offset(
            n.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = *epstab.offset(n as isize);
    *epstab.offset(n as isize) = 1.7976931348623157e+308f64;
    i = 0 as libc::c_int as size_t;
    while i < newelm {
        let mut res: libc::c_double = *epstab
            .offset(
                n
                    .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(i))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            );
        let mut e0: libc::c_double = *epstab
            .offset(
                n
                    .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(i))
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
            );
        let mut e1: libc::c_double = *epstab
            .offset(
                n
                    .wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(i))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        let mut e2: libc::c_double = res;
        let mut e1abs: libc::c_double = fabs(e1);
        let mut delta2: libc::c_double = e2 - e1;
        let mut err2: libc::c_double = fabs(delta2);
        let mut tol2: libc::c_double = GSL_MAX_DBL(fabs(e2), e1abs)
            * 2.2204460492503131e-16f64;
        let mut delta3: libc::c_double = e1 - e0;
        let mut err3: libc::c_double = fabs(delta3);
        let mut tol3: libc::c_double = GSL_MAX_DBL(e1abs, fabs(e0))
            * 2.2204460492503131e-16f64;
        let mut e3: libc::c_double = 0.;
        let mut delta1: libc::c_double = 0.;
        let mut err1: libc::c_double = 0.;
        let mut tol1: libc::c_double = 0.;
        let mut ss: libc::c_double = 0.;
        if err2 <= tol2 && err3 <= tol3 {
            *result = res;
            absolute = err2 + err3;
            relative = 5 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
                * fabs(res);
            *abserr = GSL_MAX_DBL(absolute, relative);
            return;
        }
        e3 = *epstab
            .offset(
                n.wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(i))
                    as isize,
            );
        *epstab
            .offset(
                n.wrapping_sub((2 as libc::c_int as libc::c_ulong).wrapping_mul(i))
                    as isize,
            ) = e1;
        delta1 = e1 - e3;
        err1 = fabs(delta1);
        tol1 = GSL_MAX_DBL(e1abs, fabs(e3)) * 2.2204460492503131e-16f64;
        if err1 <= tol1 || err2 <= tol2 || err3 <= tol3 {
            n_final = (2 as libc::c_int as libc::c_ulong).wrapping_mul(i);
            break;
        } else {
            ss = 1 as libc::c_int as libc::c_double / delta1
                + 1 as libc::c_int as libc::c_double / delta2
                - 1 as libc::c_int as libc::c_double / delta3;
            if fabs(ss * e1) <= 0.0001f64 {
                n_final = (2 as libc::c_int as libc::c_ulong).wrapping_mul(i);
                break;
            } else {
                res = e1 + 1 as libc::c_int as libc::c_double / ss;
                *epstab
                    .offset(
                        n
                            .wrapping_sub(
                                (2 as libc::c_int as libc::c_ulong).wrapping_mul(i),
                            ) as isize,
                    ) = res;
                let error: libc::c_double = err2 + fabs(res - e2) + err3;
                if error <= *abserr {
                    *abserr = error;
                    *result = res;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    let limexp: size_t = (50 as libc::c_int - 1 as libc::c_int) as size_t;
    if n_final == limexp {
        n_final = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(limexp.wrapping_div(2 as libc::c_int as libc::c_ulong));
    }
    if n_orig.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        i = 0 as libc::c_int as size_t;
        while i <= newelm {
            *epstab
                .offset(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(i.wrapping_mul(2 as libc::c_int as libc::c_ulong))
                        as isize,
                ) = *epstab
                .offset(
                    i
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i <= newelm {
            *epstab
                .offset(
                    i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *epstab
                .offset(
                    i
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
    }
    if n_orig != n_final {
        i = 0 as libc::c_int as size_t;
        while i <= n_final {
            *epstab
                .offset(
                    i as isize,
                ) = *epstab
                .offset(n_orig.wrapping_sub(n_final).wrapping_add(i) as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    (*table).n = n_final.wrapping_add(1 as libc::c_int as libc::c_ulong);
    if nres_orig < 3 as libc::c_int as libc::c_ulong {
        *res3la.offset(nres_orig as isize) = *result;
        *abserr = 1.7976931348623157e+308f64;
    } else {
        *abserr = fabs(*result - *res3la.offset(2 as libc::c_int as isize))
            + fabs(*result - *res3la.offset(1 as libc::c_int as isize))
            + fabs(*result - *res3la.offset(0 as libc::c_int as isize));
        *res3la
            .offset(
                0 as libc::c_int as isize,
            ) = *res3la.offset(1 as libc::c_int as isize);
        *res3la
            .offset(
                1 as libc::c_int as isize,
            ) = *res3la.offset(2 as libc::c_int as isize);
        *res3la.offset(2 as libc::c_int as isize) = *result;
    }
    (*table).nres = nres_orig.wrapping_add(1 as libc::c_int as libc::c_ulong);
    *abserr = GSL_MAX_DBL(
        *abserr,
        5 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64 * fabs(*result),
    );
}
unsafe extern "C" fn increase_nrmax(
    mut workspace: *mut gsl_integration_workspace,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut id: libc::c_int = (*workspace).nrmax as libc::c_int;
    let mut jupbnd: libc::c_int = 0;
    let mut level: *const size_t = (*workspace).level;
    let mut order: *const size_t = (*workspace).order;
    let mut limit: size_t = (*workspace).limit;
    let mut last: size_t = ((*workspace).size)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if last
        > (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(limit.wrapping_div(2 as libc::c_int as libc::c_ulong))
    {
        jupbnd = limit.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_sub(last)
            as libc::c_int;
    } else {
        jupbnd = last as libc::c_int;
    }
    k = id;
    while k <= jupbnd {
        let mut i_max: size_t = *order.offset((*workspace).nrmax as isize);
        (*workspace).i = i_max;
        if *level.offset(i_max as isize) < (*workspace).maximum_level {
            return 1 as libc::c_int;
        }
        (*workspace).nrmax = ((*workspace).nrmax).wrapping_add(1);
        (*workspace).nrmax;
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn large_interval(
    mut workspace: *mut gsl_integration_workspace,
) -> libc::c_int {
    let mut i: size_t = (*workspace).i;
    let mut level: *const size_t = (*workspace).level;
    if *level.offset(i as isize) < (*workspace).maximum_level {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn sort_results(mut workspace: *mut gsl_integration_workspace) {
    let mut i: size_t = 0;
    let mut elist: *mut libc::c_double = (*workspace).elist;
    let mut order: *mut size_t = (*workspace).order;
    let mut nint: size_t = (*workspace).size;
    i = 0 as libc::c_int as size_t;
    while i < nint {
        let mut i1: size_t = *order.offset(i as isize);
        let mut e1: libc::c_double = *elist.offset(i1 as isize);
        let mut i_max: size_t = i1;
        let mut j: size_t = 0;
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < nint {
            let mut i2: size_t = *order.offset(j as isize);
            let mut e2: libc::c_double = *elist.offset(i2 as isize);
            if e2 >= e1 {
                i_max = i2;
                e1 = e2;
            }
            j = j.wrapping_add(1);
            j;
        }
        if i_max != i1 {
            *order.offset(i as isize) = *order.offset(i_max as isize);
            *order.offset(i_max as isize) = i1;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*workspace).i = *order.offset(0 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn test_positivity(
    mut result: libc::c_double,
    mut resabs: libc::c_double,
) -> libc::c_int {
    let mut status: libc::c_int = (fabs(result)
        >= (1 as libc::c_int as libc::c_double
            - 50 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64) * resabs)
        as libc::c_int;
    return status;
}
pub unsafe extern "C" fn gsl_integration_qagp(
    mut f: *const gsl_function,
    mut pts: *mut libc::c_double,
    mut npts: size_t,
    mut epsabs: libc::c_double,
    mut epsrel: libc::c_double,
    mut limit: size_t,
    mut workspace: *mut gsl_integration_workspace,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut status: libc::c_int = qagp(
        f,
        pts,
        npts,
        epsabs,
        epsrel,
        limit,
        workspace,
        result,
        abserr,
        Some(
            gsl_integration_qk21
                as unsafe extern "C" fn(
                    *const gsl_function,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> (),
        ),
    );
    return status;
}
unsafe extern "C" fn qagp(
    mut f: *const gsl_function,
    mut pts: *const libc::c_double,
    npts: size_t,
    epsabs: libc::c_double,
    epsrel: libc::c_double,
    limit: size_t,
    mut workspace: *mut gsl_integration_workspace,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut q: Option::<gsl_integration_rule>,
) -> libc::c_int {
    let mut current_block: u64;
    let mut area: libc::c_double = 0.;
    let mut errsum: libc::c_double = 0.;
    let mut res_ext: libc::c_double = 0.;
    let mut err_ext: libc::c_double = 0.;
    let mut result0: libc::c_double = 0.;
    let mut abserr0: libc::c_double = 0.;
    let mut resabs0: libc::c_double = 0.;
    let mut tolerance: libc::c_double = 0.;
    let mut ertest: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut error_over_large_intervals: libc::c_double = 0 as libc::c_int
        as libc::c_double;
    let mut reseps: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut abseps: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut correc: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ktmin: size_t = 0 as libc::c_int as size_t;
    let mut roundoff_type1: libc::c_int = 0 as libc::c_int;
    let mut roundoff_type2: libc::c_int = 0 as libc::c_int;
    let mut roundoff_type3: libc::c_int = 0 as libc::c_int;
    let mut error_type: libc::c_int = 0 as libc::c_int;
    let mut error_type2: libc::c_int = 0 as libc::c_int;
    let mut iteration: size_t = 0 as libc::c_int as size_t;
    let mut positive_integrand: libc::c_int = 0 as libc::c_int;
    let mut extrapolate: libc::c_int = 0 as libc::c_int;
    let mut disallow_extrapolation: libc::c_int = 0 as libc::c_int;
    let mut table: extrapolation_table = extrapolation_table {
        n: 0,
        rlist2: [0.; 52],
        nres: 0,
        res3la: [0.; 3],
    };
    let nint: size_t = npts.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut ndin: *mut size_t = (*workspace).level;
    let mut i: size_t = 0;
    *result = 0 as libc::c_int as libc::c_double;
    *abserr = 0 as libc::c_int as libc::c_double;
    if limit > (*workspace).limit {
        gsl_error(
            b"iteration limit exceeds available workspace\0" as *const u8
                as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if npts > (*workspace).limit {
        gsl_error(
            b"npts exceeds size of workspace\0" as *const u8 as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
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
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < nint {
        if *pts.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            < *pts.offset(i as isize)
        {
            gsl_error(
                b"points are not in an ascending sequence\0" as *const u8
                    as *const libc::c_char,
                b"qagp.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    result0 = 0 as libc::c_int as libc::c_double;
    abserr0 = 0 as libc::c_int as libc::c_double;
    resabs0 = 0 as libc::c_int as libc::c_double;
    initialise(workspace, 0.0f64, 0.0f64);
    i = 0 as libc::c_int as size_t;
    while i < nint {
        let mut area1: libc::c_double = 0.;
        let mut error1: libc::c_double = 0.;
        let mut resabs1: libc::c_double = 0.;
        let mut resasc1: libc::c_double = 0.;
        let a1: libc::c_double = *pts.offset(i as isize);
        let b1: libc::c_double = *pts
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        q.unwrap()(f, a1, b1, &mut area1, &mut error1, &mut resabs1, &mut resasc1);
        result0 = result0 + area1;
        abserr0 = abserr0 + error1;
        resabs0 = resabs0 + resabs1;
        append_interval(workspace, a1, b1, area1, error1);
        if error1 == resasc1 && error1 != 0.0f64 {
            *ndin.offset(i as isize) = 1 as libc::c_int as size_t;
        } else {
            *ndin.offset(i as isize) = 0 as libc::c_int as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    errsum = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < nint {
        if *ndin.offset(i as isize) != 0 {
            *((*workspace).elist).offset(i as isize) = abserr0;
        }
        errsum = errsum + *((*workspace).elist).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < nint {
        *((*workspace).level).offset(i as isize) = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
    sort_results(workspace);
    tolerance = GSL_MAX_DBL(epsabs, epsrel * fabs(result0));
    if abserr0
        <= 100 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64 * resabs0
        && abserr0 > tolerance
    {
        *result = result0;
        *abserr = abserr0;
        gsl_error(
            b"cannot reach tolerance because of roundoff erroron first attempt\0"
                as *const u8 as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    } else if abserr0 <= tolerance {
        *result = result0;
        *abserr = abserr0;
        return GSL_SUCCESS as libc::c_int;
    } else if limit == 1 as libc::c_int as libc::c_ulong {
        *result = result0;
        *abserr = abserr0;
        gsl_error(
            b"a maximum of one iteration was insufficient\0" as *const u8
                as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    }
    initialise_table(&mut table);
    append_table(&mut table, result0);
    area = result0;
    res_ext = result0;
    err_ext = 1.7976931348623157e+308f64;
    error_over_large_intervals = errsum;
    ertest = tolerance;
    positive_integrand = test_positivity(result0, resabs0);
    iteration = nint.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let mut current_level: size_t = 0;
        let mut a1_0: libc::c_double = 0.;
        let mut b1_0: libc::c_double = 0.;
        let mut a2: libc::c_double = 0.;
        let mut b2: libc::c_double = 0.;
        let mut a_i: libc::c_double = 0.;
        let mut b_i: libc::c_double = 0.;
        let mut r_i: libc::c_double = 0.;
        let mut e_i: libc::c_double = 0.;
        let mut area1_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut area2: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut area12: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error1_0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error2: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut error12: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut resasc1_0: libc::c_double = 0.;
        let mut resasc2: libc::c_double = 0.;
        let mut resabs1_0: libc::c_double = 0.;
        let mut resabs2: libc::c_double = 0.;
        let mut last_e_i: libc::c_double = 0.;
        retrieve(workspace, &mut a_i, &mut b_i, &mut r_i, &mut e_i);
        current_level = (*((*workspace).level).offset((*workspace).i as isize))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        a1_0 = a_i;
        b1_0 = 0.5f64 * (a_i + b_i);
        a2 = b1_0;
        b2 = b_i;
        iteration = iteration.wrapping_add(1);
        iteration;
        q
            .unwrap()(
            f,
            a1_0,
            b1_0,
            &mut area1_0,
            &mut error1_0,
            &mut resabs1_0,
            &mut resasc1_0,
        );
        q.unwrap()(f, a2, b2, &mut area2, &mut error2, &mut resabs2, &mut resasc2);
        area12 = area1_0 + area2;
        error12 = error1_0 + error2;
        last_e_i = e_i;
        errsum = errsum + error12 - e_i;
        area = area + area12 - r_i;
        tolerance = GSL_MAX_DBL(epsabs, epsrel * fabs(area));
        if resasc1_0 != error1_0 && resasc2 != error2 {
            let mut delta: libc::c_double = r_i - area12;
            if fabs(delta) <= 1.0e-5f64 * fabs(area12) && error12 >= 0.99f64 * e_i {
                if extrapolate == 0 {
                    roundoff_type1 += 1;
                    roundoff_type1;
                } else {
                    roundoff_type2 += 1;
                    roundoff_type2;
                }
            }
            if i > 10 as libc::c_int as libc::c_ulong && error12 > e_i {
                roundoff_type3 += 1;
                roundoff_type3;
            }
        }
        if roundoff_type1 + roundoff_type2 >= 10 as libc::c_int
            || roundoff_type3 >= 20 as libc::c_int
        {
            error_type = 2 as libc::c_int;
        }
        if roundoff_type2 >= 5 as libc::c_int {
            error_type2 = 1 as libc::c_int;
        }
        if subinterval_too_small(a1_0, a2, b2) != 0 {
            error_type = 4 as libc::c_int;
        }
        update(workspace, a1_0, b1_0, area1_0, error1_0, a2, b2, area2, error2);
        if errsum <= tolerance {
            current_block = 15750385526522368608;
            break;
        }
        if error_type != 0 {
            current_block = 14579489411542934868;
            break;
        }
        if iteration >= limit.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            error_type = 1 as libc::c_int;
            current_block = 14579489411542934868;
            break;
        } else {
            if !(disallow_extrapolation != 0) {
                error_over_large_intervals += -last_e_i;
                if current_level < (*workspace).maximum_level {
                    error_over_large_intervals += error12;
                }
                if extrapolate == 0 {
                    if large_interval(workspace) != 0 {
                        current_block = 6367734732029634840;
                    } else {
                        extrapolate = 1 as libc::c_int;
                        (*workspace).nrmax = 1 as libc::c_int as size_t;
                        current_block = 10041771570435381152;
                    }
                } else {
                    current_block = 10041771570435381152;
                }
                match current_block {
                    6367734732029634840 => {}
                    _ => {
                        if error_type2 == 0 && error_over_large_intervals > ertest {
                            if increase_nrmax(workspace) != 0 {
                                current_block = 6367734732029634840;
                            } else {
                                current_block = 12963528325254160332;
                            }
                        } else {
                            current_block = 12963528325254160332;
                        }
                        match current_block {
                            6367734732029634840 => {}
                            _ => {
                                append_table(&mut table, area);
                                if !(table.n < 3 as libc::c_int as libc::c_ulong) {
                                    qelg(&mut table, &mut reseps, &mut abseps);
                                    ktmin = ktmin.wrapping_add(1);
                                    ktmin;
                                    if ktmin > 5 as libc::c_int as libc::c_ulong
                                        && err_ext < 0.001f64 * errsum
                                    {
                                        error_type = 5 as libc::c_int;
                                    }
                                    if abseps < err_ext {
                                        ktmin = 0 as libc::c_int as size_t;
                                        err_ext = abseps;
                                        res_ext = reseps;
                                        correc = error_over_large_intervals;
                                        ertest = GSL_MAX_DBL(epsabs, epsrel * fabs(reseps));
                                        if err_ext <= ertest {
                                            current_block = 14579489411542934868;
                                            break;
                                        }
                                    }
                                    if table.n == 1 as libc::c_int as libc::c_ulong {
                                        disallow_extrapolation = 1 as libc::c_int;
                                    }
                                    if error_type == 5 as libc::c_int {
                                        current_block = 14579489411542934868;
                                        break;
                                    }
                                }
                                reset_nrmax(workspace);
                                extrapolate = 0 as libc::c_int;
                                error_over_large_intervals = errsum;
                            }
                        }
                    }
                }
            }
            if !(iteration < limit) {
                current_block = 14579489411542934868;
                break;
            }
        }
    }
    match current_block {
        14579489411542934868 => {
            *result = res_ext;
            *abserr = err_ext;
            if err_ext == 1.7976931348623157e+308f64 {
                current_block = 15750385526522368608;
            } else {
                if error_type != 0 || error_type2 != 0 {
                    if error_type2 != 0 {
                        err_ext += correc;
                    }
                    if error_type == 0 as libc::c_int {
                        error_type = 3 as libc::c_int;
                    }
                    if !result.is_null() && area != 0 as libc::c_int as libc::c_double {
                        if err_ext / fabs(res_ext) > errsum / fabs(area) {
                            current_block = 15750385526522368608;
                        } else {
                            current_block = 4330759529560430365;
                        }
                    } else if err_ext > errsum {
                        current_block = 15750385526522368608;
                    } else if area == 0.0f64 {
                        current_block = 14404513160173047454;
                    } else {
                        current_block = 4330759529560430365;
                    }
                } else {
                    current_block = 4330759529560430365;
                }
                match current_block {
                    14404513160173047454 => {}
                    15750385526522368608 => {}
                    _ => {
                        let mut max_area: libc::c_double = GSL_MAX_DBL(
                            fabs(res_ext),
                            fabs(area),
                        );
                        if positive_integrand == 0 && max_area < 0.01f64 * resabs0 {
                            current_block = 14404513160173047454;
                        } else {
                            let mut ratio: libc::c_double = res_ext / area;
                            if ratio < 0.01f64
                                || ratio > 100 as libc::c_int as libc::c_double
                                || errsum > fabs(area)
                            {
                                error_type = 6 as libc::c_int;
                            }
                            current_block = 14404513160173047454;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        15750385526522368608 => {
            *result = sum_results(workspace);
            *abserr = errsum;
        }
        _ => {}
    }
    if error_type > 2 as libc::c_int {
        error_type -= 1;
        error_type;
    }
    if error_type == 0 as libc::c_int {
        return GSL_SUCCESS as libc::c_int
    } else if error_type == 1 as libc::c_int {
        gsl_error(
            b"number of iterations was insufficient\0" as *const u8
                as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else if error_type == 2 as libc::c_int {
        gsl_error(
            b"cannot reach tolerance because of roundoff error\0" as *const u8
                as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    } else if error_type == 3 as libc::c_int {
        gsl_error(
            b"bad integrand behavior found in the integration interval\0" as *const u8
                as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            494 as libc::c_int,
            GSL_ESING as libc::c_int,
        );
        return GSL_ESING as libc::c_int;
    } else if error_type == 4 as libc::c_int {
        gsl_error(
            b"roundoff error detected in the extrapolation table\0" as *const u8
                as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    } else if error_type == 5 as libc::c_int {
        gsl_error(
            b"integral is divergent, or slowly convergent\0" as *const u8
                as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            504 as libc::c_int,
            GSL_EDIVERGE as libc::c_int,
        );
        return GSL_EDIVERGE as libc::c_int;
    } else {
        gsl_error(
            b"could not integrate function\0" as *const u8 as *const libc::c_char,
            b"qagp.c\0" as *const u8 as *const libc::c_char,
            508 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    };
}
