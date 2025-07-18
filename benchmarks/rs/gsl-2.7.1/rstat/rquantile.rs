use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sort(data: *mut libc::c_double, stride: size_t, n: size_t);
    fn gsl_stats_quantile_from_sorted_data(
        sorted_data: *const libc::c_double,
        stride: size_t,
        n: size_t,
        f: libc::c_double,
    ) -> libc::c_double;
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
pub struct gsl_rstat_quantile_workspace {
    pub p: libc::c_double,
    pub q: [libc::c_double; 5],
    pub npos: [libc::c_int; 5],
    pub np: [libc::c_double; 5],
    pub dnp: [libc::c_double; 5],
    pub n: size_t,
}
pub unsafe extern "C" fn gsl_rstat_quantile_alloc(
    p: libc::c_double,
) -> *mut gsl_rstat_quantile_workspace {
    let mut w: *mut gsl_rstat_quantile_workspace = 0
        as *mut gsl_rstat_quantile_workspace;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_rstat_quantile_workspace>() as libc::c_ulong,
    ) as *mut gsl_rstat_quantile_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"rquantile.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_rstat_quantile_workspace;
    }
    (*w).p = p;
    gsl_rstat_quantile_reset(w);
    return w;
}
pub unsafe extern "C" fn gsl_rstat_quantile_free(
    mut w: *mut gsl_rstat_quantile_workspace,
) {
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_rstat_quantile_reset(
    mut w: *mut gsl_rstat_quantile_workspace,
) -> libc::c_int {
    let p: libc::c_double = (*w).p;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 5 as libc::c_int as libc::c_ulong {
        (*w)
            .npos[i
            as usize] = i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    (*w).np[0 as libc::c_int as usize] = 1.0f64;
    (*w).np[1 as libc::c_int as usize] = 1.0f64 + 2.0f64 * p;
    (*w).np[2 as libc::c_int as usize] = 1.0f64 + 4.0f64 * p;
    (*w).np[3 as libc::c_int as usize] = 3.0f64 + 2.0f64 * p;
    (*w).np[4 as libc::c_int as usize] = 5.0f64;
    (*w).dnp[0 as libc::c_int as usize] = 0.0f64;
    (*w).dnp[1 as libc::c_int as usize] = 0.5f64 * p;
    (*w).dnp[2 as libc::c_int as usize] = p;
    (*w).dnp[3 as libc::c_int as usize] = 0.5f64 * (1.0f64 + p);
    (*w).dnp[4 as libc::c_int as usize] = 1.0f64;
    (*w).n = 0 as libc::c_int as size_t;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_rstat_quantile_add(
    x: libc::c_double,
    mut w: *mut gsl_rstat_quantile_workspace,
) -> libc::c_int {
    if (*w).n < 5 as libc::c_int as libc::c_ulong {
        (*w).q[(*w).n as usize] = x;
    } else {
        let mut i: libc::c_int = 0;
        let mut k: libc::c_int = -(1 as libc::c_int);
        if (*w).n == 5 as libc::c_int as libc::c_ulong {
            gsl_sort(((*w).q).as_mut_ptr(), 1 as libc::c_int as size_t, (*w).n);
        }
        if x < (*w).q[0 as libc::c_int as usize] {
            (*w).q[0 as libc::c_int as usize] = x;
            k = 0 as libc::c_int;
        } else if x >= (*w).q[4 as libc::c_int as usize] {
            (*w).q[4 as libc::c_int as usize] = x;
            k = 3 as libc::c_int;
        } else {
            i = 0 as libc::c_int;
            while i <= 3 as libc::c_int {
                if (*w).q[i as usize] <= x && x < (*w).q[(i + 1 as libc::c_int) as usize]
                {
                    k = i;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        if k < 0 as libc::c_int {
            gsl_error(
                b"invalid input argument x\0" as *const u8 as *const libc::c_char,
                b"rquantile.c\0" as *const u8 as *const libc::c_char,
                136 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        i = k + 1 as libc::c_int;
        while i <= 4 as libc::c_int {
            (*w).npos[i as usize] += 1;
            (*w).npos[i as usize];
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            (*w).np[i as usize] += (*w).dnp[i as usize];
            i += 1;
            i;
        }
        i = 1 as libc::c_int;
        while i <= 3 as libc::c_int {
            let mut ni: libc::c_double = (*w).npos[i as usize] as libc::c_double;
            let mut d: libc::c_double = (*w).np[i as usize] - ni;
            if d >= 1.0f64
                && (*w).npos[(i + 1 as libc::c_int) as usize] - (*w).npos[i as usize]
                    > 1 as libc::c_int
                || d <= -1.0f64
                    && (*w).npos[(i - 1 as libc::c_int) as usize] - (*w).npos[i as usize]
                        < -(1 as libc::c_int)
            {
                let mut dsign: libc::c_int = if d > 0.0f64 {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                };
                let mut qp1: libc::c_double = (*w).q[(i + 1 as libc::c_int) as usize];
                let mut qi: libc::c_double = (*w).q[i as usize];
                let mut qm1: libc::c_double = (*w).q[(i - 1 as libc::c_int) as usize];
                let mut np1: libc::c_double = (*w).npos[(i + 1 as libc::c_int) as usize]
                    as libc::c_double;
                let mut nm1: libc::c_double = (*w).npos[(i - 1 as libc::c_int) as usize]
                    as libc::c_double;
                let mut qp: libc::c_double = calc_psq(
                    qp1,
                    qi,
                    qm1,
                    dsign as libc::c_double,
                    np1,
                    ni,
                    nm1,
                );
                if qm1 < qp && qp < qp1 {
                    (*w).q[i as usize] = qp;
                } else {
                    (*w).q[i as usize]
                        += dsign as libc::c_double * ((*w).q[(i + dsign) as usize] - qi)
                            / ((*w).npos[(i + dsign) as usize] as libc::c_double - ni);
                }
                (*w).npos[i as usize] += dsign;
            }
            i += 1;
            i;
        }
    }
    (*w).n = ((*w).n).wrapping_add(1);
    (*w).n;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_rstat_quantile_get(
    mut w: *mut gsl_rstat_quantile_workspace,
) -> libc::c_double {
    if (*w).n > 5 as libc::c_int as libc::c_ulong {
        return (*w).q[2 as libc::c_int as usize]
    } else {
        gsl_sort(((*w).q).as_mut_ptr(), 1 as libc::c_int as size_t, (*w).n);
        return gsl_stats_quantile_from_sorted_data(
            ((*w).q).as_mut_ptr() as *const libc::c_double,
            1 as libc::c_int as size_t,
            (*w).n,
            (*w).p,
        );
    };
}
unsafe extern "C" fn calc_psq(
    qp1: libc::c_double,
    q: libc::c_double,
    qm1: libc::c_double,
    d: libc::c_double,
    np1: libc::c_double,
    n: libc::c_double,
    nm1: libc::c_double,
) -> libc::c_double {
    let mut outer: libc::c_double = d / (np1 - nm1);
    let mut inner_left: libc::c_double = (n - nm1 + d) * (qp1 - q) / (np1 - n);
    let mut inner_right: libc::c_double = (np1 - n - d) * (q - qm1) / (n - nm1);
    return q + outer * (inner_left + inner_right);
}
