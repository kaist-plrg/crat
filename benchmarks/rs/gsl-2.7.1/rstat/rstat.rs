use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_rstat_quantile_alloc(p: libc::c_double) -> *mut gsl_rstat_quantile_workspace;
    fn gsl_rstat_quantile_free(w: *mut gsl_rstat_quantile_workspace);
    fn gsl_rstat_quantile_reset(w: *mut gsl_rstat_quantile_workspace) -> libc::c_int;
    fn gsl_rstat_quantile_add(
        x: libc::c_double,
        w: *mut gsl_rstat_quantile_workspace,
    ) -> libc::c_int;
    fn gsl_rstat_quantile_get(w: *mut gsl_rstat_quantile_workspace) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rstat_workspace {
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub mean: libc::c_double,
    pub M2: libc::c_double,
    pub M3: libc::c_double,
    pub M4: libc::c_double,
    pub n: size_t,
    pub median_workspace_p: *mut gsl_rstat_quantile_workspace,
}
pub unsafe extern "C" fn gsl_rstat_alloc() -> *mut gsl_rstat_workspace {
    let mut w: *mut gsl_rstat_workspace = 0 as *mut gsl_rstat_workspace;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_rstat_workspace>() as libc::c_ulong,
    ) as *mut gsl_rstat_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"rstat.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_rstat_workspace;
    }
    (*w).median_workspace_p = gsl_rstat_quantile_alloc(0.5f64);
    if ((*w).median_workspace_p).is_null() {
        gsl_error(
            b"failed to allocate space for median workspace\0" as *const u8
                as *const libc::c_char,
            b"rstat.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_rstat_workspace;
    }
    gsl_rstat_reset(w);
    return w;
}
pub unsafe extern "C" fn gsl_rstat_free(mut w: *mut gsl_rstat_workspace) {
    if !((*w).median_workspace_p).is_null() {
        gsl_rstat_quantile_free((*w).median_workspace_p);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_rstat_n(mut w: *const gsl_rstat_workspace) -> size_t {
    return (*w).n;
}
pub unsafe extern "C" fn gsl_rstat_add(
    x: libc::c_double,
    mut w: *mut gsl_rstat_workspace,
) -> libc::c_int {
    let mut delta: libc::c_double = x - (*w).mean;
    let mut delta_n: libc::c_double = 0.;
    let mut delta_nsq: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut n: libc::c_double = 0.;
    if (*w).n == 0 as libc::c_int as libc::c_ulong {
        (*w).min = x;
        (*w).max = x;
    } else {
        if x < (*w).min {
            (*w).min = x;
        }
        if x > (*w).max {
            (*w).max = x;
        }
    }
    (*w).n = ((*w).n).wrapping_add(1);
    n = (*w).n as libc::c_double;
    delta_n = delta / n;
    delta_nsq = delta_n * delta_n;
    term1 = delta * delta_n * (n - 1.0f64);
    (*w).mean += delta_n;
    (*w).M4
        += term1 * delta_nsq * (n * n - 3.0f64 * n + 3.0f64)
            + 6.0f64 * delta_nsq * (*w).M2 - 4.0f64 * delta_n * (*w).M3;
    (*w).M3 += term1 * delta_n * (n - 2.0f64) - 3.0f64 * delta_n * (*w).M2;
    (*w).M2 += term1;
    gsl_rstat_quantile_add(x, (*w).median_workspace_p);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_rstat_min(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    return (*w).min;
}
pub unsafe extern "C" fn gsl_rstat_max(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    return (*w).max;
}
pub unsafe extern "C" fn gsl_rstat_mean(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    return (*w).mean;
}
pub unsafe extern "C" fn gsl_rstat_variance(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    if (*w).n > 1 as libc::c_int as libc::c_ulong {
        let mut n: libc::c_double = (*w).n as libc::c_double;
        return (*w).M2 / (n - 1.0f64);
    } else {
        return 0.0f64
    };
}
pub unsafe extern "C" fn gsl_rstat_sd(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    let mut var: libc::c_double = gsl_rstat_variance(w);
    return sqrt(var);
}
pub unsafe extern "C" fn gsl_rstat_rms(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    let mut rms: libc::c_double = 0.0f64;
    if (*w).n > 0 as libc::c_int as libc::c_ulong {
        let mut mean: libc::c_double = gsl_rstat_mean(w);
        let mut sigma: libc::c_double = gsl_rstat_sd(w);
        let mut n: libc::c_double = (*w).n as libc::c_double;
        let mut a: libc::c_double = sqrt((n - 1.0f64) / n);
        rms = gsl_hypot(mean, a * sigma);
    }
    return rms;
}
pub unsafe extern "C" fn gsl_rstat_sd_mean(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    if (*w).n > 0 as libc::c_int as libc::c_ulong {
        let mut sd: libc::c_double = gsl_rstat_sd(w);
        return sd / sqrt((*w).n as libc::c_double);
    } else {
        return 0.0f64
    };
}
pub unsafe extern "C" fn gsl_rstat_median(
    mut w: *mut gsl_rstat_workspace,
) -> libc::c_double {
    return gsl_rstat_quantile_get((*w).median_workspace_p);
}
pub unsafe extern "C" fn gsl_rstat_skew(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    if (*w).n > 0 as libc::c_int as libc::c_ulong {
        let mut n: libc::c_double = (*w).n as libc::c_double;
        let mut fac: libc::c_double = pow(n - 1.0f64, 1.5f64) / n;
        return fac * (*w).M3 / pow((*w).M2, 1.5f64);
    } else {
        return 0.0f64
    };
}
pub unsafe extern "C" fn gsl_rstat_kurtosis(
    mut w: *const gsl_rstat_workspace,
) -> libc::c_double {
    if (*w).n > 0 as libc::c_int as libc::c_ulong {
        let mut n: libc::c_double = (*w).n as libc::c_double;
        let mut fac: libc::c_double = (n - 1.0f64) / n * (n - 1.0f64);
        return fac * (*w).M4 / ((*w).M2 * (*w).M2) - 3.0f64;
    } else {
        return 0.0f64
    };
}
pub unsafe extern "C" fn gsl_rstat_reset(
    mut w: *mut gsl_rstat_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*w).min = 0.0f64;
    (*w).max = 0.0f64;
    (*w).mean = 0.0f64;
    (*w).M2 = 0.0f64;
    (*w).M3 = 0.0f64;
    (*w).M4 = 0.0f64;
    (*w).n = 0 as libc::c_int as size_t;
    status = gsl_rstat_quantile_reset((*w).median_workspace_p);
    return status;
}
