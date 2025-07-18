use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_eigen_symmv_sort(
        eval: *mut gsl_vector,
        evec: *mut gsl_matrix,
        sort_type: gsl_eigen_sort_t,
    ) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_view_array(
        base: *mut libc::c_double,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_eigen_symmv(
        A: *mut gsl_matrix,
        eval: *mut gsl_vector,
        evec: *mut gsl_matrix,
        w: *mut gsl_eigen_symmv_workspace,
    ) -> libc::c_int;
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
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_view = _gsl_matrix_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_symmv_workspace {
    pub size: size_t,
    pub d: *mut libc::c_double,
    pub sd: *mut libc::c_double,
    pub gc: *mut libc::c_double,
    pub gs: *mut libc::c_double,
}
pub type gsl_eigen_sort_t = libc::c_uint;
pub const GSL_EIGEN_SORT_ABS_DESC: gsl_eigen_sort_t = 3;
pub const GSL_EIGEN_SORT_ABS_ASC: gsl_eigen_sort_t = 2;
pub const GSL_EIGEN_SORT_VAL_DESC: gsl_eigen_sort_t = 1;
pub const GSL_EIGEN_SORT_VAL_ASC: gsl_eigen_sort_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_mathieu_workspace {
    pub size: size_t,
    pub even_order: size_t,
    pub odd_order: size_t,
    pub extra_values: libc::c_int,
    pub qa: libc::c_double,
    pub qb: libc::c_double,
    pub aa: *mut libc::c_double,
    pub bb: *mut libc::c_double,
    pub dd: *mut libc::c_double,
    pub ee: *mut libc::c_double,
    pub tt: *mut libc::c_double,
    pub e2: *mut libc::c_double,
    pub zz: *mut libc::c_double,
    pub eval: *mut gsl_vector,
    pub evec: *mut gsl_matrix,
    pub wmat: *mut gsl_eigen_symmv_workspace,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
unsafe extern "C" fn ceer(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut aa: libc::c_double,
    mut nterms: libc::c_int,
) -> libc::c_double {
    let mut term: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut ii: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    if order == 0 as libc::c_int {
        term = 0.0f64;
    } else {
        term = 2.0f64 * qq * qq / aa;
        if order != 2 as libc::c_int {
            n1 = order / 2 as libc::c_int - 1 as libc::c_int;
            ii = 0 as libc::c_int;
            while ii < n1 {
                term = qq * qq
                    / (aa
                        - 4.0f64 * (ii + 1 as libc::c_int) as libc::c_double
                            * (ii + 1 as libc::c_int) as libc::c_double - term);
                ii += 1;
                ii;
            }
        }
    }
    term += (order * order) as libc::c_double;
    term1 = 0.0f64;
    ii = 0 as libc::c_int;
    while ii < nterms {
        term1 = qq * qq
            / (aa
                - (order as libc::c_double + 2.0f64 * (nterms - ii) as libc::c_double)
                    * (order as libc::c_double
                        + 2.0f64 * (nterms - ii) as libc::c_double) - term1);
        ii += 1;
        ii;
    }
    if order == 0 as libc::c_int {
        term1 *= 2.0f64;
    }
    return term + term1 - aa;
}
unsafe extern "C" fn ceor(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut aa: libc::c_double,
    mut nterms: libc::c_int,
) -> libc::c_double {
    let mut term: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut ii: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    term = qq;
    n1 = (order as libc::c_float as libc::c_double / 2.0f64 - 0.5f64) as libc::c_int;
    ii = 0 as libc::c_int;
    while ii < n1 {
        term = qq * qq
            / (aa
                - (2.0f64 * ii as libc::c_double + 1.0f64)
                    * (2.0f64 * ii as libc::c_double + 1.0f64) - term);
        ii += 1;
        ii;
    }
    term += (order * order) as libc::c_double;
    term1 = 0.0f64;
    ii = 0 as libc::c_int;
    while ii < nterms {
        term1 = qq * qq
            / (aa
                - (order as libc::c_double + 2.0f64 * (nterms - ii) as libc::c_double)
                    * (order as libc::c_double
                        + 2.0f64 * (nterms - ii) as libc::c_double) - term1);
        ii += 1;
        ii;
    }
    return term + term1 - aa;
}
unsafe extern "C" fn seer(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut aa: libc::c_double,
    mut nterms: libc::c_int,
) -> libc::c_double {
    let mut term: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut ii: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    term = 0.0f64;
    n1 = order / 2 as libc::c_int - 1 as libc::c_int;
    ii = 0 as libc::c_int;
    while ii < n1 {
        term = qq * qq
            / (aa
                - 4.0f64 * (ii + 1 as libc::c_int) as libc::c_double
                    * (ii + 1 as libc::c_int) as libc::c_double - term);
        ii += 1;
        ii;
    }
    term += (order * order) as libc::c_double;
    term1 = 0.0f64;
    ii = 0 as libc::c_int;
    while ii < nterms {
        term1 = qq * qq
            / (aa
                - (order as libc::c_double + 2.0f64 * (nterms - ii) as libc::c_double)
                    * (order as libc::c_double
                        + 2.0f64 * (nterms - ii) as libc::c_double) - term1);
        ii += 1;
        ii;
    }
    return term + term1 - aa;
}
unsafe extern "C" fn seor(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut aa: libc::c_double,
    mut nterms: libc::c_int,
) -> libc::c_double {
    let mut term: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut ii: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    term = -1.0f64 * qq;
    n1 = (order as libc::c_float as libc::c_double / 2.0f64 - 0.5f64) as libc::c_int;
    ii = 0 as libc::c_int;
    while ii < n1 {
        term = qq * qq
            / (aa
                - (2.0f64 * ii as libc::c_double + 1.0f64)
                    * (2.0f64 * ii as libc::c_double + 1.0f64) - term);
        ii += 1;
        ii;
    }
    term += (order * order) as libc::c_double;
    term1 = 0.0f64;
    ii = 0 as libc::c_int;
    while ii < nterms {
        term1 = qq * qq
            / (aa
                - (order as libc::c_double + 2.0f64 * (nterms - ii) as libc::c_double)
                    * (order as libc::c_double
                        + 2.0f64 * (nterms - ii) as libc::c_double) - term1);
        ii += 1;
        ii;
    }
    return term + term1 - aa;
}
unsafe extern "C" fn asymptotic(
    mut order: libc::c_int,
    mut qq: libc::c_double,
) -> libc::c_double {
    let mut asymp: libc::c_double = 0.;
    let mut nn: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    let mut n4: libc::c_double = 0.;
    let mut n6: libc::c_double = 0.;
    let mut hh: libc::c_double = 0.;
    let mut ah: libc::c_double = 0.;
    let mut ah2: libc::c_double = 0.;
    let mut ah3: libc::c_double = 0.;
    let mut ah4: libc::c_double = 0.;
    let mut ah5: libc::c_double = 0.;
    nn = (2 as libc::c_int * order + 1 as libc::c_int) as libc::c_double;
    n2 = nn * nn;
    n4 = n2 * n2;
    n6 = n4 * n2;
    hh = 2 as libc::c_int as libc::c_double * sqrt(qq);
    ah = 16 as libc::c_int as libc::c_double * hh;
    ah2 = ah * ah;
    ah3 = ah2 * ah;
    ah4 = ah3 * ah;
    ah5 = ah4 * ah;
    asymp = -(2 as libc::c_int) as libc::c_double * qq + nn * hh
        - 0.125f64 * (n2 + 1 as libc::c_int as libc::c_double);
    asymp -= 0.25f64 * nn * (n2 + 3 as libc::c_int as libc::c_double) / ah;
    asymp
        -= 0.25f64
            * (5 as libc::c_int as libc::c_double * n4
                + 34 as libc::c_int as libc::c_double * n2
                + 9 as libc::c_int as libc::c_double) / ah2;
    asymp
        -= 0.25f64 * nn
            * (33 as libc::c_int as libc::c_double * n4
                + 410 as libc::c_int as libc::c_double * n2
                + 405 as libc::c_int as libc::c_double) / ah3;
    asymp
        -= (63 as libc::c_int as libc::c_double * n6
            + 1260 as libc::c_int as libc::c_double * n4
            + 2943 as libc::c_int as libc::c_double * n2
            + 486 as libc::c_int as libc::c_double) / ah4;
    asymp
        -= nn
            * (527 as libc::c_int as libc::c_double * n6
                + 15617 as libc::c_int as libc::c_double * n4
                + 69001 as libc::c_int as libc::c_double * n2
                + 41607 as libc::c_int as libc::c_double) / ah5;
    return asymp;
}
unsafe extern "C" fn solve_cubic(
    mut c2: libc::c_double,
    mut c1: libc::c_double,
    mut c0: libc::c_double,
) -> libc::c_double {
    let mut qq: libc::c_double = 0.;
    let mut rr: libc::c_double = 0.;
    let mut ww: libc::c_double = 0.;
    let mut ss: libc::c_double = 0.;
    let mut tt: libc::c_double = 0.;
    qq = (3 as libc::c_int as libc::c_double * c1 - c2 * c2)
        / 9 as libc::c_int as libc::c_double;
    rr = (9 as libc::c_int as libc::c_double * c2 * c1
        - 27 as libc::c_int as libc::c_double * c0
        - 2 as libc::c_int as libc::c_double * c2 * c2 * c2)
        / 54 as libc::c_int as libc::c_double;
    ww = qq * qq * qq + rr * rr;
    if ww >= 0 as libc::c_int as libc::c_double {
        let mut t1: libc::c_double = rr + sqrt(ww);
        ss = fabs(t1) / t1 * pow(fabs(t1), 1 as libc::c_int as libc::c_double / 3.0f64);
        t1 = rr - sqrt(ww);
        tt = fabs(t1) / t1 * pow(fabs(t1), 1 as libc::c_int as libc::c_double / 3.0f64);
    } else {
        let mut theta: libc::c_double = acos(rr / sqrt(-qq * qq * qq));
        ss = 2 as libc::c_int as libc::c_double * sqrt(-qq)
            * cos(
                (theta + 4 as libc::c_int as libc::c_double * 3.14159265358979323846f64)
                    / 3.0f64,
            );
        tt = 0.0f64;
    }
    return ss + tt - c2 / 3 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn approx_c(
    mut order: libc::c_int,
    mut qq: libc::c_double,
) -> libc::c_double {
    let mut approx: libc::c_double = 0.;
    let mut c0: libc::c_double = 0.;
    let mut c1: libc::c_double = 0.;
    let mut c2: libc::c_double = 0.;
    if order < 0 as libc::c_int {
        gsl_error(
            b"Undefined order for Mathieu function\0" as *const u8
                as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0.0f64;
    }
    match order {
        0 => {
            if qq <= 4 as libc::c_int as libc::c_double {
                return 2 as libc::c_int as libc::c_double
                    - sqrt(
                        4 as libc::c_int as libc::c_double
                            + 2 as libc::c_int as libc::c_double * qq * qq,
                    )
            } else {
                return asymptotic(order, qq)
            }
        }
        1 => {
            if qq <= 4 as libc::c_int as libc::c_double {
                return 5 as libc::c_int as libc::c_double
                    + 0.5f64
                        * (qq
                            - sqrt(
                                5 as libc::c_int as libc::c_double * qq * qq
                                    - 16 as libc::c_int as libc::c_double * qq
                                    + 64 as libc::c_int as libc::c_double,
                            ))
            } else {
                return asymptotic(order, qq)
            }
        }
        2 => {
            if qq <= 3 as libc::c_int as libc::c_double {
                c2 = -8.0f64;
                c1 = -(48 as libc::c_int) as libc::c_double
                    - 3 as libc::c_int as libc::c_double * qq * qq;
                c0 = 20 as libc::c_int as libc::c_double * qq * qq;
            } else {
                return asymptotic(order, qq)
            }
        }
        3 => {
            if qq <= 6.25f64 {
                c2 = -qq - 8 as libc::c_int as libc::c_double;
                c1 = 16 as libc::c_int as libc::c_double * qq
                    - 128 as libc::c_int as libc::c_double
                    - 2 as libc::c_int as libc::c_double * qq * qq;
                c0 = qq * qq * (qq + 8 as libc::c_int as libc::c_double);
            } else {
                return asymptotic(order, qq)
            }
        }
        _ => {
            if order < 70 as libc::c_int {
                if 1.7f64 * order as libc::c_double
                    > 2 as libc::c_int as libc::c_double * sqrt(qq)
                {
                    let mut n2: libc::c_double = (order * order) as libc::c_double;
                    let mut n22: libc::c_double = (n2
                        - 1 as libc::c_int as libc::c_double)
                        * (n2 - 1 as libc::c_int as libc::c_double);
                    let mut q2: libc::c_double = qq * qq;
                    let mut q4: libc::c_double = q2 * q2;
                    approx = n2
                        + 0.5f64 * q2 / (n2 - 1 as libc::c_int as libc::c_double);
                    approx
                        += (5 as libc::c_int as libc::c_double * n2
                            + 7 as libc::c_int as libc::c_double) * q4
                            / (32 as libc::c_int as libc::c_double * n22
                                * (n2 - 1 as libc::c_int as libc::c_double)
                                * (n2 - 4 as libc::c_int as libc::c_double));
                    approx
                        += (9 as libc::c_int as libc::c_double * n2 * n2
                            + 58 as libc::c_int as libc::c_double * n2
                            + 29 as libc::c_int as libc::c_double) * q4 * q2
                            / (64 as libc::c_int as libc::c_double * n22 * n22
                                * (n2 - 1 as libc::c_int as libc::c_double)
                                * (n2 - 4 as libc::c_int as libc::c_double)
                                * (n2 - 9 as libc::c_int as libc::c_double));
                    if (1.4f64 * order as libc::c_double)
                        < 2 as libc::c_int as libc::c_double * sqrt(qq)
                    {
                        approx += asymptotic(order, qq);
                        approx *= 0.5f64;
                    }
                } else {
                    approx = asymptotic(order, qq);
                }
                return approx;
            } else {
                return (order * order) as libc::c_double
            }
        }
    }
    approx = solve_cubic(c2, c1, c0);
    if approx < 0 as libc::c_int as libc::c_double
        && sqrt(qq) > 0.1f64 * order as libc::c_double
    {
        return asymptotic(order - 1 as libc::c_int, qq)
    } else {
        return (order * order) as libc::c_double + fabs(approx)
    };
}
unsafe extern "C" fn approx_s(
    mut order: libc::c_int,
    mut qq: libc::c_double,
) -> libc::c_double {
    let mut approx: libc::c_double = 0.;
    let mut c0: libc::c_double = 0.;
    let mut c1: libc::c_double = 0.;
    let mut c2: libc::c_double = 0.;
    if order < 1 as libc::c_int {
        gsl_error(
            b"Undefined order for Mathieu function\0" as *const u8
                as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0.0f64;
    }
    match order {
        1 => {
            if qq <= 4 as libc::c_int as libc::c_double {
                return 5 as libc::c_int as libc::c_double
                    - 0.5f64
                        * (qq
                            + sqrt(
                                5 as libc::c_int as libc::c_double * qq * qq
                                    + 16 as libc::c_int as libc::c_double * qq
                                    + 64 as libc::c_int as libc::c_double,
                            ))
            } else {
                return asymptotic(order - 1 as libc::c_int, qq)
            }
        }
        2 => {
            if qq <= 5 as libc::c_int as libc::c_double {
                return 10 as libc::c_int as libc::c_double
                    - sqrt(36 as libc::c_int as libc::c_double + qq * qq)
            } else {
                return asymptotic(order - 1 as libc::c_int, qq)
            }
        }
        3 => {
            if qq <= 6.25f64 {
                c2 = qq - 8 as libc::c_int as libc::c_double;
                c1 = -(128 as libc::c_int) as libc::c_double
                    - 16 as libc::c_int as libc::c_double * qq
                    - 2 as libc::c_int as libc::c_double * qq * qq;
                c0 = qq * qq * (8 as libc::c_int as libc::c_double - qq);
            } else {
                return asymptotic(order - 1 as libc::c_int, qq)
            }
        }
        _ => {
            if order < 70 as libc::c_int {
                if 1.7f64 * order as libc::c_double
                    > 2 as libc::c_int as libc::c_double * sqrt(qq)
                {
                    let mut n2: libc::c_double = (order * order) as libc::c_double;
                    let mut n22: libc::c_double = (n2
                        - 1 as libc::c_int as libc::c_double)
                        * (n2 - 1 as libc::c_int as libc::c_double);
                    let mut q2: libc::c_double = qq * qq;
                    let mut q4: libc::c_double = q2 * q2;
                    approx = n2
                        + 0.5f64 * q2 / (n2 - 1 as libc::c_int as libc::c_double);
                    approx
                        += (5 as libc::c_int as libc::c_double * n2
                            + 7 as libc::c_int as libc::c_double) * q4
                            / (32 as libc::c_int as libc::c_double * n22
                                * (n2 - 1 as libc::c_int as libc::c_double)
                                * (n2 - 4 as libc::c_int as libc::c_double));
                    approx
                        += (9 as libc::c_int as libc::c_double * n2 * n2
                            + 58 as libc::c_int as libc::c_double * n2
                            + 29 as libc::c_int as libc::c_double) * q4 * q2
                            / (64 as libc::c_int as libc::c_double * n22 * n22
                                * (n2 - 1 as libc::c_int as libc::c_double)
                                * (n2 - 4 as libc::c_int as libc::c_double)
                                * (n2 - 9 as libc::c_int as libc::c_double));
                    if (1.4f64 * order as libc::c_double)
                        < 2 as libc::c_int as libc::c_double * sqrt(qq)
                    {
                        approx += asymptotic(order - 1 as libc::c_int, qq);
                        approx *= 0.5f64;
                    }
                } else {
                    approx = asymptotic(order - 1 as libc::c_int, qq);
                }
                return approx;
            } else {
                return (order * order) as libc::c_double
            }
        }
    }
    approx = solve_cubic(c2, c1, c0);
    if approx < 0 as libc::c_int as libc::c_double
        && sqrt(qq) > 0.1f64 * order as libc::c_double
    {
        return asymptotic(order - 1 as libc::c_int, qq)
    } else {
        return (order * order) as libc::c_double + fabs(approx)
    };
}
pub unsafe extern "C" fn gsl_sf_mathieu_a_e(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut even_odd: libc::c_int = 0;
    let mut nterms: libc::c_int = 50 as libc::c_int;
    let mut ii: libc::c_int = 0;
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut maxcount: libc::c_int = 1000 as libc::c_int;
    let mut dir: libc::c_int = 0 as libc::c_int;
    let mut a1: libc::c_double = 0.;
    let mut a2: libc::c_double = 0.;
    let mut fa: libc::c_double = 0.;
    let mut fa1: libc::c_double = 0.;
    let mut dela: libc::c_double = 0.;
    let mut aa_orig: libc::c_double = 0.;
    let mut da: libc::c_double = 0.025f64;
    let mut aa: libc::c_double = 0.;
    let mut aa_approx: libc::c_double = 0.;
    even_odd = 0 as libc::c_int;
    if order % 2 as libc::c_int != 0 as libc::c_int {
        even_odd = 1 as libc::c_int;
    }
    if qq == 0 as libc::c_int as libc::c_double {
        (*result).val = (order * order) as libc::c_double;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    }
    if order < 0 as libc::c_int {
        order *= -(1 as libc::c_int);
    }
    if qq < 0.0f64 {
        if even_odd == 0 as libc::c_int {
            return gsl_sf_mathieu_a_e(order, -qq, result)
        } else {
            return gsl_sf_mathieu_b_e(order, -qq, result)
        }
    }
    aa_approx = approx_c(order, qq);
    aa = aa_approx;
    aa_orig = aa;
    while counter < maxcount {
        a1 = aa + 0.001f64;
        ii = 0 as libc::c_int;
        if even_odd == 0 as libc::c_int {
            fa1 = ceer(order, qq, a1, nterms);
        } else {
            fa1 = ceor(order, qq, a1, nterms);
        }
        loop {
            if even_odd == 0 as libc::c_int {
                fa = ceer(order, qq, aa, nterms);
            } else {
                fa = ceor(order, qq, aa, nterms);
            }
            a2 = a1;
            a1 = aa;
            if fa == fa1 {
                (*result).err = 2.2204460492503131e-16f64;
                break;
            } else {
                aa -= (aa - a2) / (fa - fa1) * fa;
                dela = fabs(aa - a2);
                if dela < 2.2204460492503131e-16f64 {
                    (*result).err = 2.2204460492503131e-16f64;
                    break;
                } else if ii > 40 as libc::c_int {
                    (*result).err = dela;
                    break;
                } else {
                    fa1 = fa;
                    ii += 1;
                    ii;
                }
            }
        }
        if !(fabs(aa - aa_orig)
            > 3 as libc::c_int as libc::c_double
                + 0.01f64 * order as libc::c_double * fabs(aa_orig)
            || order > 10 as libc::c_int
                && fabs(aa - aa_orig) > 1.5f64 * order as libc::c_double)
        {
            break;
        }
        counter += 1;
        counter;
        if counter == maxcount {
            (*result).err = fabs(aa - aa_orig);
            break;
        } else {
            if aa > aa_orig {
                if dir == 1 as libc::c_int {
                    da /= 2 as libc::c_int as libc::c_double;
                }
                dir = -(1 as libc::c_int);
            } else {
                if dir == -(1 as libc::c_int) {
                    da /= 2 as libc::c_int as libc::c_double;
                }
                dir = 1 as libc::c_int;
            }
            aa_approx += dir as libc::c_double * da * counter as libc::c_double;
            aa = aa_approx;
        }
    }
    (*result).val = aa;
    if counter == maxcount {
        gsl_error(
            b"Wrong characteristic Mathieu value\0" as *const u8 as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_mathieu_b_e(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut even_odd: libc::c_int = 0;
    let mut nterms: libc::c_int = 50 as libc::c_int;
    let mut ii: libc::c_int = 0;
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut maxcount: libc::c_int = 1000 as libc::c_int;
    let mut dir: libc::c_int = 0 as libc::c_int;
    let mut a1: libc::c_double = 0.;
    let mut a2: libc::c_double = 0.;
    let mut fa: libc::c_double = 0.;
    let mut fa1: libc::c_double = 0.;
    let mut dela: libc::c_double = 0.;
    let mut aa_orig: libc::c_double = 0.;
    let mut da: libc::c_double = 0.025f64;
    let mut aa: libc::c_double = 0.;
    let mut aa_approx: libc::c_double = 0.;
    even_odd = 0 as libc::c_int;
    if order % 2 as libc::c_int != 0 as libc::c_int {
        even_odd = 1 as libc::c_int;
    }
    if order == 0 as libc::c_int {
        gsl_error(
            b"Characteristic value undefined for order 0\0" as *const u8
                as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            511 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    if qq == 0 as libc::c_int as libc::c_double {
        (*result).val = (order * order) as libc::c_double;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    }
    if order < 0 as libc::c_int {
        order *= -(1 as libc::c_int);
    }
    if qq < 0.0f64 {
        if even_odd == 0 as libc::c_int {
            return gsl_sf_mathieu_b_e(order, -qq, result)
        } else {
            return gsl_sf_mathieu_a_e(order, -qq, result)
        }
    }
    aa_approx = approx_s(order, qq);
    aa = aa_approx;
    aa_orig = aa;
    while counter < maxcount {
        a1 = aa + 0.001f64;
        ii = 0 as libc::c_int;
        if even_odd == 0 as libc::c_int {
            fa1 = seer(order, qq, a1, nterms);
        } else {
            fa1 = seor(order, qq, a1, nterms);
        }
        loop {
            if even_odd == 0 as libc::c_int {
                fa = seer(order, qq, aa, nterms);
            } else {
                fa = seor(order, qq, aa, nterms);
            }
            a2 = a1;
            a1 = aa;
            if fa == fa1 {
                (*result).err = 2.2204460492503131e-16f64;
                break;
            } else {
                aa -= (aa - a2) / (fa - fa1) * fa;
                dela = fabs(aa - a2);
                if dela < 1e-18f64 {
                    (*result).err = 2.2204460492503131e-16f64;
                    break;
                } else if ii > 40 as libc::c_int {
                    (*result).err = dela;
                    break;
                } else {
                    fa1 = fa;
                    ii += 1;
                    ii;
                }
            }
        }
        if !(fabs(aa - aa_orig)
            > 3 as libc::c_int as libc::c_double
                + 0.01f64 * order as libc::c_double * fabs(aa_orig)
            || order > 10 as libc::c_int
                && fabs(aa - aa_orig) > 1.5f64 * order as libc::c_double)
        {
            break;
        }
        counter += 1;
        counter;
        if counter == maxcount {
            (*result).err = fabs(aa - aa_orig);
            break;
        } else {
            if aa > aa_orig {
                if dir == 1 as libc::c_int {
                    da /= 2 as libc::c_int as libc::c_double;
                }
                dir = -(1 as libc::c_int);
            } else {
                if dir == -(1 as libc::c_int) {
                    da /= 2 as libc::c_int as libc::c_double;
                }
                dir = 1 as libc::c_int;
            }
            aa_approx += dir as libc::c_double * da * counter as libc::c_double;
            aa = aa_approx;
        }
    }
    (*result).val = aa;
    if counter == maxcount {
        gsl_error(
            b"Wrong characteristic Mathieu value\0" as *const u8 as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            621 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn figi(
    mut nn: libc::c_int,
    mut tt: *mut libc::c_double,
    mut dd: *mut libc::c_double,
    mut ee: *mut libc::c_double,
    mut e2: *mut libc::c_double,
) -> libc::c_int {
    let mut ii: libc::c_int = 0;
    ii = 0 as libc::c_int;
    while ii < nn {
        if ii != 0 as libc::c_int {
            *e2
                .offset(
                    ii as isize,
                ) = *tt.offset((3 as libc::c_int * ii) as isize)
                * *tt
                    .offset(
                        (3 as libc::c_int * (ii - 1 as libc::c_int) + 2 as libc::c_int)
                            as isize,
                    );
            if *e2.offset(ii as isize) < 0.0f64 {
                return nn + ii;
            }
            if *e2.offset(ii as isize) == 0.0f64
                && (*tt.offset((3 as libc::c_int * ii) as isize) != 0.0f64
                    || *tt
                        .offset(
                            (3 as libc::c_int * (ii - 1 as libc::c_int)
                                + 2 as libc::c_int) as isize,
                        ) != 0.0f64)
            {
                return -(1 as libc::c_int) * (3 as libc::c_int * nn + ii);
            }
            *ee.offset(ii as isize) = sqrt(*e2.offset(ii as isize));
        }
        *dd
            .offset(
                ii as isize,
            ) = *tt.offset((3 as libc::c_int * ii + 1 as libc::c_int) as isize);
        ii += 1;
        ii;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_mathieu_a_array(
    mut order_min: libc::c_int,
    mut order_max: libc::c_int,
    mut qq: libc::c_double,
    mut work: *mut gsl_sf_mathieu_workspace,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut even_order: libc::c_uint = (*work).even_order as libc::c_uint;
    let mut odd_order: libc::c_uint = (*work).odd_order as libc::c_uint;
    let mut extra_values: libc::c_uint = (*work).extra_values as libc::c_uint;
    let mut ii: libc::c_uint = 0;
    let mut jj: libc::c_uint = 0;
    let mut status: libc::c_int = 0;
    let mut tt: *mut libc::c_double = (*work).tt;
    let mut dd: *mut libc::c_double = (*work).dd;
    let mut ee: *mut libc::c_double = (*work).ee;
    let mut e2: *mut libc::c_double = (*work).e2;
    let mut zz: *mut libc::c_double = (*work).zz;
    let mut aa: *mut libc::c_double = (*work).aa;
    let mut mat: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut evec: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut eval: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut wmat: *mut gsl_eigen_symmv_workspace = (*work).wmat;
    if order_max as libc::c_ulong > (*work).size || order_max <= order_min
        || order_min < 0 as libc::c_int
    {
        gsl_error(
            b"invalid range [order_min,order_max]\0" as *const u8 as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            723 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    *tt.offset(0 as libc::c_int as isize) = 0.0f64;
    *tt.offset(1 as libc::c_int as isize) = 0.0f64;
    *tt.offset(2 as libc::c_int as isize) = qq;
    ii = 1 as libc::c_int as libc::c_uint;
    while ii < even_order.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        *tt.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(ii) as isize) = qq;
        *tt
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = (4 as libc::c_int as libc::c_uint).wrapping_mul(ii).wrapping_mul(ii)
            as libc::c_double;
        *tt
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) = qq;
        ii = ii.wrapping_add(1);
        ii;
    }
    *tt
        .offset(
            (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(even_order)
                .wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
        ) = qq;
    *tt
        .offset(
            (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(even_order)
                .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
        ) = (4 as libc::c_int as libc::c_uint)
        .wrapping_mul(even_order.wrapping_sub(1 as libc::c_int as libc::c_uint))
        .wrapping_mul(even_order.wrapping_sub(1 as libc::c_int as libc::c_uint))
        as libc::c_double;
    *tt
        .offset(
            (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(even_order)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) = 0.0f64;
    *tt.offset(3 as libc::c_int as isize) *= 2 as libc::c_int as libc::c_double;
    status = figi(even_order as libc::c_int, tt, dd, ee, e2);
    if status != 0 {
        gsl_error(
            b"Internal error in tridiagonal Mathieu matrix\0" as *const u8
                as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            748 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < even_order.wrapping_mul(even_order) {
        *zz.offset(ii as isize) = 0.0f64;
        ii = ii.wrapping_add(1);
        ii;
    }
    *zz.offset(0 as libc::c_int as isize) = *dd.offset(0 as libc::c_int as isize);
    *zz.offset(1 as libc::c_int as isize) = *ee.offset(1 as libc::c_int as isize);
    ii = 1 as libc::c_int as libc::c_uint;
    while ii < even_order.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        *zz
            .offset(
                ii
                    .wrapping_mul(even_order)
                    .wrapping_add(ii)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = *ee.offset(ii as isize);
        *zz
            .offset(
                ii.wrapping_mul(even_order).wrapping_add(ii) as isize,
            ) = *dd.offset(ii as isize);
        *zz
            .offset(
                ii
                    .wrapping_mul(even_order)
                    .wrapping_add(ii)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = *ee.offset(ii.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        ii = ii.wrapping_add(1);
        ii;
    }
    *zz
        .offset(
            even_order
                .wrapping_mul(even_order.wrapping_sub(1 as libc::c_int as libc::c_uint))
                .wrapping_add(even_order)
                .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
        ) = *ee
        .offset(even_order.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    *zz
        .offset(
            even_order
                .wrapping_mul(even_order)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) = *dd
        .offset(even_order.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    mat = gsl_matrix_view_array(zz, even_order as size_t, even_order as size_t);
    eval = gsl_vector_subvector(
        (*work).eval,
        0 as libc::c_int as size_t,
        even_order as size_t,
    );
    evec = gsl_matrix_submatrix(
        (*work).evec,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        even_order as size_t,
        even_order as size_t,
    );
    gsl_eigen_symmv(&mut mat.matrix, &mut eval.vector, &mut evec.matrix, wmat);
    gsl_eigen_symmv_sort(&mut eval.vector, &mut evec.matrix, GSL_EIGEN_SORT_VAL_ASC);
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < even_order.wrapping_sub(extra_values) {
        *aa
            .offset(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(ii) as isize,
            ) = gsl_vector_get(&mut eval.vector, ii as size_t);
        ii = ii.wrapping_add(1);
        ii;
    }
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < odd_order.wrapping_mul(odd_order) {
        *zz.offset(ii as isize) = 0.0f64;
        ii = ii.wrapping_add(1);
        ii;
    }
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < odd_order {
        jj = 0 as libc::c_int as libc::c_uint;
        while jj < odd_order {
            if ii == jj {
                *zz
                    .offset(
                        ii.wrapping_mul(odd_order).wrapping_add(jj) as isize,
                    ) = (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(ii)
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    ) as libc::c_double;
            } else if ii == jj.wrapping_add(1 as libc::c_int as libc::c_uint)
                || ii.wrapping_add(1 as libc::c_int as libc::c_uint) == jj
            {
                *zz.offset(ii.wrapping_mul(odd_order).wrapping_add(jj) as isize) = qq;
            }
            jj = jj.wrapping_add(1);
            jj;
        }
        ii = ii.wrapping_add(1);
        ii;
    }
    *zz.offset(0 as libc::c_int as isize) += qq;
    mat = gsl_matrix_view_array(zz, odd_order as size_t, odd_order as size_t);
    eval = gsl_vector_subvector(
        (*work).eval,
        0 as libc::c_int as size_t,
        odd_order as size_t,
    );
    evec = gsl_matrix_submatrix(
        (*work).evec,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        odd_order as size_t,
        odd_order as size_t,
    );
    gsl_eigen_symmv(&mut mat.matrix, &mut eval.vector, &mut evec.matrix, wmat);
    gsl_eigen_symmv_sort(&mut eval.vector, &mut evec.matrix, GSL_EIGEN_SORT_VAL_ASC);
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < odd_order.wrapping_sub(extra_values) {
        *aa
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = gsl_vector_get(&mut eval.vector, ii as size_t);
        ii = ii.wrapping_add(1);
        ii;
    }
    ii = order_min as libc::c_uint;
    while ii <= order_max as libc::c_uint {
        *result_array
            .offset(
                ii.wrapping_sub(order_min as libc::c_uint) as isize,
            ) = *aa.offset(ii as isize);
        ii = ii.wrapping_add(1);
        ii;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_mathieu_b_array(
    mut order_min: libc::c_int,
    mut order_max: libc::c_int,
    mut qq: libc::c_double,
    mut work: *mut gsl_sf_mathieu_workspace,
    mut result_array: *mut libc::c_double,
) -> libc::c_int {
    let mut even_order: libc::c_uint = ((*work).even_order)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
    let mut odd_order: libc::c_uint = (*work).odd_order as libc::c_uint;
    let mut extra_values: libc::c_uint = (*work).extra_values as libc::c_uint;
    let mut ii: libc::c_uint = 0;
    let mut jj: libc::c_uint = 0;
    let mut zz: *mut libc::c_double = (*work).zz;
    let mut bb: *mut libc::c_double = (*work).bb;
    let mut mat: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut evec: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut eval: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut wmat: *mut gsl_eigen_symmv_workspace = (*work).wmat;
    if order_max as libc::c_ulong > (*work).size || order_max <= order_min
        || order_min < 0 as libc::c_int
    {
        gsl_error(
            b"invalid range [order_min,order_max]\0" as *const u8 as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            819 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < even_order.wrapping_mul(even_order) {
        *zz.offset(ii as isize) = 0.0f64;
        ii = ii.wrapping_add(1);
        ii;
    }
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < even_order {
        jj = 0 as libc::c_int as libc::c_uint;
        while jj < even_order {
            if ii == jj {
                *zz
                    .offset(
                        ii.wrapping_mul(even_order).wrapping_add(jj) as isize,
                    ) = (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii.wrapping_add(1 as libc::c_int as libc::c_uint))
                    .wrapping_mul(ii.wrapping_add(1 as libc::c_int as libc::c_uint))
                    as libc::c_double;
            } else if ii == jj.wrapping_add(1 as libc::c_int as libc::c_uint)
                || ii.wrapping_add(1 as libc::c_int as libc::c_uint) == jj
            {
                *zz.offset(ii.wrapping_mul(even_order).wrapping_add(jj) as isize) = qq;
            }
            jj = jj.wrapping_add(1);
            jj;
        }
        ii = ii.wrapping_add(1);
        ii;
    }
    mat = gsl_matrix_view_array(zz, even_order as size_t, even_order as size_t);
    eval = gsl_vector_subvector(
        (*work).eval,
        0 as libc::c_int as size_t,
        even_order as size_t,
    );
    evec = gsl_matrix_submatrix(
        (*work).evec,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        even_order as size_t,
        even_order as size_t,
    );
    gsl_eigen_symmv(&mut mat.matrix, &mut eval.vector, &mut evec.matrix, wmat);
    gsl_eigen_symmv_sort(&mut eval.vector, &mut evec.matrix, GSL_EIGEN_SORT_VAL_ASC);
    *bb.offset(0 as libc::c_int as isize) = 0.0f64;
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < even_order.wrapping_sub(extra_values) {
        *bb
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii.wrapping_add(1 as libc::c_int as libc::c_uint))
                    as isize,
            ) = gsl_vector_get(&mut eval.vector, ii as size_t);
        ii = ii.wrapping_add(1);
        ii;
    }
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < odd_order.wrapping_mul(odd_order) {
        *zz.offset(ii as isize) = 0.0f64;
        ii = ii.wrapping_add(1);
        ii;
    }
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < odd_order {
        jj = 0 as libc::c_int as libc::c_uint;
        while jj < odd_order {
            if ii == jj {
                *zz
                    .offset(
                        ii.wrapping_mul(odd_order).wrapping_add(jj) as isize,
                    ) = (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(ii)
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    ) as libc::c_double;
            } else if ii == jj.wrapping_add(1 as libc::c_int as libc::c_uint)
                || ii.wrapping_add(1 as libc::c_int as libc::c_uint) == jj
            {
                *zz.offset(ii.wrapping_mul(odd_order).wrapping_add(jj) as isize) = qq;
            }
            jj = jj.wrapping_add(1);
            jj;
        }
        ii = ii.wrapping_add(1);
        ii;
    }
    *zz.offset(0 as libc::c_int as isize) -= qq;
    mat = gsl_matrix_view_array(zz, odd_order as size_t, odd_order as size_t);
    eval = gsl_vector_subvector(
        (*work).eval,
        0 as libc::c_int as size_t,
        odd_order as size_t,
    );
    evec = gsl_matrix_submatrix(
        (*work).evec,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        odd_order as size_t,
        odd_order as size_t,
    );
    gsl_eigen_symmv(&mut mat.matrix, &mut eval.vector, &mut evec.matrix, wmat);
    gsl_eigen_symmv_sort(&mut eval.vector, &mut evec.matrix, GSL_EIGEN_SORT_VAL_ASC);
    ii = 0 as libc::c_int as libc::c_uint;
    while ii < odd_order.wrapping_sub(extra_values) {
        *bb
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul(ii)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = gsl_vector_get(&mut eval.vector, ii as size_t);
        ii = ii.wrapping_add(1);
        ii;
    }
    ii = order_min as libc::c_uint;
    while ii <= order_max as libc::c_uint {
        *result_array
            .offset(
                ii.wrapping_sub(order_min as libc::c_uint) as isize,
            ) = *bb.offset(ii as isize);
        ii = ii.wrapping_add(1);
        ii;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_mathieu_a(
    mut order: libc::c_int,
    mut qq: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_mathieu_a_e(order, qq, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_mathieu_a_e(order, qq, &result)\0" as *const u8
                as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            884 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_mathieu_b(
    mut order: libc::c_int,
    mut qq: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_mathieu_b_e(order, qq, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_mathieu_b_e(order, qq, &result)\0" as *const u8
                as *const libc::c_char,
            b"mathieu_charv.c\0" as *const u8 as *const libc::c_char,
            889 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
