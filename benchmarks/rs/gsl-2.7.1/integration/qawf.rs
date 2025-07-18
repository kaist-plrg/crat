use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_integration_qawo(
        f: *mut gsl_function,
        a: libc::c_double,
        epsabs: libc::c_double,
        epsrel: libc::c_double,
        limit: size_t,
        workspace: *mut gsl_integration_workspace,
        wf: *mut gsl_integration_qawo_table,
        result: *mut libc::c_double,
        abserr: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_integration_qawo_table_set_length(
        t: *mut gsl_integration_qawo_table,
        L: libc::c_double,
    ) -> libc::c_int;
    fn gsl_integration_qagiu(
        f: *mut gsl_function,
        a: libc::c_double,
        epsabs: libc::c_double,
        epsrel: libc::c_double,
        limit: size_t,
        workspace: *mut gsl_integration_workspace,
        result: *mut libc::c_double,
        abserr: *mut libc::c_double,
    ) -> libc::c_int;
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
pub type gsl_integration_qawo_enum = libc::c_uint;
pub const GSL_INTEG_SINE: gsl_integration_qawo_enum = 1;
pub const GSL_INTEG_COSINE: gsl_integration_qawo_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_integration_qawo_table {
    pub n: size_t,
    pub omega: libc::c_double,
    pub L: libc::c_double,
    pub par: libc::c_double,
    pub sine: gsl_integration_qawo_enum,
    pub chebmo: *mut libc::c_double,
}
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
pub unsafe extern "C" fn gsl_integration_qawf(
    mut f: *mut gsl_function,
    a: libc::c_double,
    epsabs: libc::c_double,
    limit: size_t,
    mut workspace: *mut gsl_integration_workspace,
    mut cycle_workspace: *mut gsl_integration_workspace,
    mut wf: *mut gsl_integration_qawo_table,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut area: libc::c_double = 0.;
    let mut errsum: libc::c_double = 0.;
    let mut res_ext: libc::c_double = 0.;
    let mut err_ext: libc::c_double = 0.;
    let mut correc: libc::c_double = 0.;
    let mut total_error: libc::c_double = 0.0f64;
    let mut truncation_error: libc::c_double = 0.;
    let mut ktmin: size_t = 0 as libc::c_int as size_t;
    let mut iteration: size_t = 0 as libc::c_int as size_t;
    let mut table: extrapolation_table = extrapolation_table {
        n: 0,
        rlist2: [0.; 52],
        nres: 0,
        res3la: [0.; 3],
    };
    let mut cycle: libc::c_double = 0.;
    let mut omega: libc::c_double = (*wf).omega;
    let p: libc::c_double = 0.9f64;
    let mut factor: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut initial_eps: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut error_type: libc::c_int = 0 as libc::c_int;
    initialise(workspace, a, a);
    *result = 0 as libc::c_int as libc::c_double;
    *abserr = 0 as libc::c_int as libc::c_double;
    if limit > (*workspace).limit {
        gsl_error(
            b"iteration limit exceeds available workspace\0" as *const u8
                as *const libc::c_char,
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if epsabs <= 0 as libc::c_int as libc::c_double {
        gsl_error(
            b"absolute tolerance epsabs must be positive\0" as *const u8
                as *const libc::c_char,
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    if omega == 0.0f64 {
        if (*wf).sine as libc::c_uint == GSL_INTEG_SINE as libc::c_int as libc::c_uint {
            *result = 0 as libc::c_int as libc::c_double;
            *abserr = 0 as libc::c_int as libc::c_double;
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut status: libc::c_int = gsl_integration_qagiu(
                f,
                a,
                epsabs,
                0.0f64,
                (*cycle_workspace).limit,
                cycle_workspace,
                result,
                abserr,
            );
            return status;
        }
    }
    if epsabs > 2.2250738585072014e-308f64 / (1 as libc::c_int as libc::c_double - p) {
        eps = epsabs * (1 as libc::c_int as libc::c_double - p);
    } else {
        eps = epsabs;
    }
    initial_eps = eps;
    area = 0 as libc::c_int as libc::c_double;
    errsum = 0 as libc::c_int as libc::c_double;
    res_ext = 0 as libc::c_int as libc::c_double;
    err_ext = 1.7976931348623157e+308f64;
    correc = 0 as libc::c_int as libc::c_double;
    cycle = (2 as libc::c_int as libc::c_double * floor(fabs(omega))
        + 1 as libc::c_int as libc::c_double) * 3.14159265358979323846f64 / fabs(omega);
    gsl_integration_qawo_table_set_length(wf, cycle);
    initialise_table(&mut table);
    iteration = 0 as libc::c_int as size_t;
    loop {
        if !(iteration < limit) {
            current_block = 13325891313334703151;
            break;
        }
        let mut area1: libc::c_double = 0.;
        let mut error1: libc::c_double = 0.;
        let mut reseps: libc::c_double = 0.;
        let mut erreps: libc::c_double = 0.;
        let mut a1: libc::c_double = a + iteration as libc::c_double * cycle;
        let mut b1: libc::c_double = a1 + cycle;
        let mut epsabs1: libc::c_double = eps * factor;
        let mut status_0: libc::c_int = gsl_integration_qawo(
            f,
            a1,
            epsabs1,
            0.0f64,
            limit,
            cycle_workspace,
            wf,
            &mut area1,
            &mut error1,
        );
        append_interval(workspace, a1, b1, area1, error1);
        factor *= p;
        area = area + area1;
        errsum = errsum + error1;
        truncation_error = 50 as libc::c_int as libc::c_double * fabs(area1);
        total_error = errsum + truncation_error;
        if total_error < epsabs && iteration > 4 as libc::c_int as libc::c_ulong {
            current_block = 1688691817779762092;
            break;
        }
        if error1 > correc {
            correc = error1;
        }
        if status_0 != 0 {
            eps = GSL_MAX_DBL(initial_eps, correc * (1.0f64 - p));
        }
        if status_0 != 0 && total_error < 10 as libc::c_int as libc::c_double * correc
            && iteration > 3 as libc::c_int as libc::c_ulong
        {
            current_block = 1688691817779762092;
            break;
        }
        append_table(&mut table, area);
        if !(table.n < 2 as libc::c_int as libc::c_ulong) {
            qelg(&mut table, &mut reseps, &mut erreps);
            ktmin = ktmin.wrapping_add(1);
            ktmin;
            if ktmin >= 15 as libc::c_int as libc::c_ulong
                && err_ext < 0.001f64 * total_error
            {
                error_type = 4 as libc::c_int;
            }
            if erreps < err_ext {
                ktmin = 0 as libc::c_int as size_t;
                err_ext = erreps;
                res_ext = reseps;
                if err_ext + 10 as libc::c_int as libc::c_double * correc <= epsabs {
                    current_block = 13325891313334703151;
                    break;
                }
                if err_ext <= epsabs
                    && 10 as libc::c_int as libc::c_double * correc >= epsabs
                {
                    current_block = 13325891313334703151;
                    break;
                }
            }
        }
        iteration = iteration.wrapping_add(1);
        iteration;
    }
    match current_block {
        13325891313334703151 => {
            if iteration == limit {
                error_type = 1 as libc::c_int;
            }
            if err_ext == 1.7976931348623157e+308f64 {
                current_block = 1688691817779762092;
            } else {
                err_ext = err_ext + 10 as libc::c_int as libc::c_double * correc;
                *result = res_ext;
                *abserr = err_ext;
                if error_type == 0 as libc::c_int {
                    return GSL_SUCCESS as libc::c_int;
                }
                if res_ext != 0.0f64 && area != 0.0f64 {
                    if err_ext / fabs(res_ext) > errsum / fabs(area) {
                        current_block = 1688691817779762092;
                    } else {
                        current_block = 10853015579903106591;
                    }
                } else if err_ext > errsum {
                    current_block = 1688691817779762092;
                } else if area == 0.0f64 {
                    current_block = 11501305976936858646;
                } else {
                    current_block = 10853015579903106591;
                }
                match current_block {
                    11501305976936858646 => {}
                    1688691817779762092 => {}
                    _ => {
                        if error_type == 4 as libc::c_int {
                            err_ext = err_ext + truncation_error;
                        }
                        current_block = 11501305976936858646;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        1688691817779762092 => {
            *result = area;
            *abserr = total_error;
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
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else if error_type == 2 as libc::c_int {
        gsl_error(
            b"cannot reach tolerance because of roundoff error\0" as *const u8
                as *const libc::c_char,
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            258 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    } else if error_type == 3 as libc::c_int {
        gsl_error(
            b"bad integrand behavior found in the integration interval\0" as *const u8
                as *const libc::c_char,
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            GSL_ESING as libc::c_int,
        );
        return GSL_ESING as libc::c_int;
    } else if error_type == 4 as libc::c_int {
        gsl_error(
            b"roundoff error detected in the extrapolation table\0" as *const u8
                as *const libc::c_char,
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    } else if error_type == 5 as libc::c_int {
        gsl_error(
            b"integral is divergent, or slowly convergent\0" as *const u8
                as *const libc::c_char,
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int,
            GSL_EDIVERGE as libc::c_int,
        );
        return GSL_EDIVERGE as libc::c_int;
    } else {
        gsl_error(
            b"could not integrate function\0" as *const u8 as *const libc::c_char,
            b"qawf.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    };
}
