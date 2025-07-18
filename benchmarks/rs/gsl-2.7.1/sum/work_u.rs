use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub struct gsl_sum_levin_u_workspace {
    pub size: size_t,
    pub i: size_t,
    pub terms_used: size_t,
    pub sum_plain: libc::c_double,
    pub q_num: *mut libc::c_double,
    pub q_den: *mut libc::c_double,
    pub dq_num: *mut libc::c_double,
    pub dq_den: *mut libc::c_double,
    pub dsum: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_sum_levin_u_alloc(
    mut n: size_t,
) -> *mut gsl_sum_levin_u_workspace {
    let mut w: *mut gsl_sum_levin_u_workspace = 0 as *mut gsl_sum_levin_u_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"length n must be positive integer\0" as *const u8 as *const libc::c_char,
            b"work_u.c\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_sum_levin_u_workspace;
    }
    w = malloc(::std::mem::size_of::<gsl_sum_levin_u_workspace>() as libc::c_ulong)
        as *mut gsl_sum_levin_u_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate struct\0" as *const u8 as *const libc::c_char,
            b"work_u.c\0" as *const u8 as *const libc::c_char,
            20 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_sum_levin_u_workspace;
    }
    (*w)
        .q_num = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).q_num).is_null() {
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for q_num\0" as *const u8 as *const libc::c_char,
            b"work_u.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_sum_levin_u_workspace;
    }
    (*w)
        .q_den = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).q_den).is_null() {
        free((*w).q_num as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for q_den\0" as *const u8 as *const libc::c_char,
            b"work_u.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_sum_levin_u_workspace;
    }
    (*w)
        .dq_num = malloc(
        n
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).dq_num).is_null() {
        free((*w).q_den as *mut libc::c_void);
        free((*w).q_num as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dq_num\0" as *const u8 as *const libc::c_char,
            b"work_u.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_sum_levin_u_workspace;
    }
    (*w)
        .dq_den = malloc(
        n
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).dq_den).is_null() {
        free((*w).dq_num as *mut libc::c_void);
        free((*w).q_den as *mut libc::c_void);
        free((*w).q_num as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dq_den\0" as *const u8 as *const libc::c_char,
            b"work_u.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_sum_levin_u_workspace;
    }
    (*w)
        .dsum = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).dsum).is_null() {
        free((*w).dq_den as *mut libc::c_void);
        free((*w).dq_num as *mut libc::c_void);
        free((*w).q_den as *mut libc::c_void);
        free((*w).q_num as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for dsum\0" as *const u8 as *const libc::c_char,
            b"work_u.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_sum_levin_u_workspace;
    }
    (*w).size = n;
    (*w).terms_used = 0 as libc::c_int as size_t;
    (*w).sum_plain = 0 as libc::c_int as libc::c_double;
    return w;
}
pub unsafe extern "C" fn gsl_sum_levin_u_free(mut w: *mut gsl_sum_levin_u_workspace) {
    if w.is_null() {
        return;
    }
    free((*w).dsum as *mut libc::c_void);
    free((*w).dq_den as *mut libc::c_void);
    free((*w).dq_num as *mut libc::c_void);
    free((*w).q_den as *mut libc::c_void);
    free((*w).q_num as *mut libc::c_void);
    free(w as *mut libc::c_void);
}
