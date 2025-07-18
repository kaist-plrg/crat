use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
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
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
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
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub unsafe extern "C" fn gsl_integration_qaws_table_alloc(
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
    mut mu: libc::c_int,
    mut nu: libc::c_int,
) -> *mut gsl_integration_qaws_table {
    let mut t: *mut gsl_integration_qaws_table = 0 as *mut gsl_integration_qaws_table;
    if alpha < -1.0f64 {
        gsl_error(
            b"alpha must be greater than -1.0\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_integration_qaws_table;
    }
    if beta < -1.0f64 {
        gsl_error(
            b"beta must be greater than -1.0\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_integration_qaws_table;
    }
    if mu != 0 as libc::c_int && mu != 1 as libc::c_int {
        gsl_error(
            b"mu must be 0 or 1\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_integration_qaws_table;
    }
    if nu != 0 as libc::c_int && nu != 1 as libc::c_int {
        gsl_error(
            b"nu must be 0 or 1\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_integration_qaws_table;
    }
    t = malloc(::std::mem::size_of::<gsl_integration_qaws_table>() as libc::c_ulong)
        as *mut gsl_integration_qaws_table;
    if t.is_null() {
        gsl_error(
            b"failed to allocate space for qaws_table struct\0" as *const u8
                as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_qaws_table;
    }
    (*t).alpha = alpha;
    (*t).beta = beta;
    (*t).mu = mu;
    (*t).nu = nu;
    initialise(
        ((*t).ri).as_mut_ptr(),
        ((*t).rj).as_mut_ptr(),
        ((*t).rg).as_mut_ptr(),
        ((*t).rh).as_mut_ptr(),
        alpha,
        beta,
    );
    return t;
}
pub unsafe extern "C" fn gsl_integration_qaws_table_set(
    mut t: *mut gsl_integration_qaws_table,
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
    mut mu: libc::c_int,
    mut nu: libc::c_int,
) -> libc::c_int {
    if alpha < -1.0f64 {
        gsl_error(
            b"alpha must be greater than -1.0\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if beta < -1.0f64 {
        gsl_error(
            b"beta must be greater than -1.0\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if mu != 0 as libc::c_int && mu != 1 as libc::c_int {
        gsl_error(
            b"mu must be 0 or 1\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if nu != 0 as libc::c_int && nu != 1 as libc::c_int {
        gsl_error(
            b"nu must be 0 or 1\0" as *const u8 as *const libc::c_char,
            b"qmomo.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    (*t).alpha = alpha;
    (*t).beta = beta;
    (*t).mu = mu;
    (*t).nu = nu;
    initialise(
        ((*t).ri).as_mut_ptr(),
        ((*t).rj).as_mut_ptr(),
        ((*t).rg).as_mut_ptr(),
        ((*t).rh).as_mut_ptr(),
        alpha,
        beta,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_integration_qaws_table_free(
    mut t: *mut gsl_integration_qaws_table,
) {
    if t.is_null() {
        return;
    }
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn initialise(
    mut ri: *mut libc::c_double,
    mut rj: *mut libc::c_double,
    mut rg: *mut libc::c_double,
    mut rh: *mut libc::c_double,
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
) {
    let alpha_p1: libc::c_double = alpha + 1.0f64;
    let beta_p1: libc::c_double = beta + 1.0f64;
    let alpha_p2: libc::c_double = alpha + 2.0f64;
    let beta_p2: libc::c_double = beta + 2.0f64;
    let r_alpha: libc::c_double = pow(2.0f64, alpha_p1);
    let r_beta: libc::c_double = pow(2.0f64, beta_p1);
    let mut i: size_t = 0;
    let mut an: libc::c_double = 0.;
    let mut anm1: libc::c_double = 0.;
    *ri.offset(0 as libc::c_int as isize) = r_alpha / alpha_p1;
    *ri
        .offset(
            1 as libc::c_int as isize,
        ) = *ri.offset(0 as libc::c_int as isize) * alpha / alpha_p2;
    an = 2.0f64;
    anm1 = 1.0f64;
    i = 2 as libc::c_int as size_t;
    while i < 25 as libc::c_int as libc::c_ulong {
        *ri
            .offset(
                i as isize,
            ) = -(r_alpha
            + an * (an - alpha_p2)
                * *ri.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            / (anm1 * (an + alpha_p1));
        anm1 = an;
        an = an + 1.0f64;
        i = i.wrapping_add(1);
        i;
    }
    *rj.offset(0 as libc::c_int as isize) = r_beta / beta_p1;
    *rj
        .offset(
            1 as libc::c_int as isize,
        ) = *rj.offset(0 as libc::c_int as isize) * beta / beta_p2;
    an = 2.0f64;
    anm1 = 1.0f64;
    i = 2 as libc::c_int as size_t;
    while i < 25 as libc::c_int as libc::c_ulong {
        *rj
            .offset(
                i as isize,
            ) = -(r_beta
            + an * (an - beta_p2)
                * *rj.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            / (anm1 * (an + beta_p1));
        anm1 = an;
        an = an + 1.0f64;
        i = i.wrapping_add(1);
        i;
    }
    *rg
        .offset(
            0 as libc::c_int as isize,
        ) = -*ri.offset(0 as libc::c_int as isize) / alpha_p1;
    *rg
        .offset(
            1 as libc::c_int as isize,
        ) = -*rg.offset(0 as libc::c_int as isize)
        - 2.0f64 * r_alpha / (alpha_p2 * alpha_p2);
    an = 2.0f64;
    anm1 = 1.0f64;
    i = 2 as libc::c_int as size_t;
    while i < 25 as libc::c_int as libc::c_ulong {
        *rg
            .offset(
                i as isize,
            ) = -(an * (an - alpha_p2)
            * *rg.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            - an * *ri.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            + anm1 * *ri.offset(i as isize)) / (anm1 * (an + alpha_p1));
        anm1 = an;
        an = an + 1.0f64;
        i = i.wrapping_add(1);
        i;
    }
    *rh
        .offset(
            0 as libc::c_int as isize,
        ) = -*rj.offset(0 as libc::c_int as isize) / beta_p1;
    *rh
        .offset(
            1 as libc::c_int as isize,
        ) = -*rh.offset(0 as libc::c_int as isize)
        - 2.0f64 * r_beta / (beta_p2 * beta_p2);
    an = 2.0f64;
    anm1 = 1.0f64;
    i = 2 as libc::c_int as size_t;
    while i < 25 as libc::c_int as libc::c_ulong {
        *rh
            .offset(
                i as isize,
            ) = -(an * (an - beta_p2)
            * *rh.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            - an * *rj.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            + anm1 * *rj.offset(i as isize)) / (anm1 * (an + beta_p1));
        anm1 = an;
        an = an + 1.0f64;
        i = i.wrapping_add(1);
        i;
    }
    i = 1 as libc::c_int as size_t;
    while i < 25 as libc::c_int as libc::c_ulong {
        *rj.offset(i as isize) *= -(1 as libc::c_int) as libc::c_double;
        *rh.offset(i as isize) *= -(1 as libc::c_int) as libc::c_double;
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
}
