use ::libc;
extern "C" {
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
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_EDOM: C2RustUnnamed = 1;
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
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub unsafe extern "C" fn gsl_integration_workspace_alloc(
    n: size_t,
) -> *mut gsl_integration_workspace {
    let mut w: *mut gsl_integration_workspace = 0 as *mut gsl_integration_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"workspace length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    w = malloc(::std::mem::size_of::<gsl_integration_workspace>() as libc::c_ulong)
        as *mut gsl_integration_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace struct\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    (*w)
        .alist = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).alist).is_null() {
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for alist ranges\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    (*w)
        .blist = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).blist).is_null() {
        free((*w).alist as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for blist ranges\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    (*w)
        .rlist = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).rlist).is_null() {
        free((*w).blist as *mut libc::c_void);
        free((*w).alist as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for rlist ranges\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    (*w)
        .elist = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).elist).is_null() {
        free((*w).rlist as *mut libc::c_void);
        free((*w).blist as *mut libc::c_void);
        free((*w).alist as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for elist ranges\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    (*w)
        .order = malloc(n.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong))
        as *mut size_t;
    if ((*w).order).is_null() {
        free((*w).elist as *mut libc::c_void);
        free((*w).rlist as *mut libc::c_void);
        free((*w).blist as *mut libc::c_void);
        free((*w).alist as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for order ranges\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    (*w)
        .level = malloc(n.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong))
        as *mut size_t;
    if ((*w).level).is_null() {
        free((*w).order as *mut libc::c_void);
        free((*w).elist as *mut libc::c_void);
        free((*w).rlist as *mut libc::c_void);
        free((*w).blist as *mut libc::c_void);
        free((*w).alist as *mut libc::c_void);
        free(w as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for order ranges\0" as *const u8
                as *const libc::c_char,
            b"workspace.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_integration_workspace;
    }
    (*w).size = 0 as libc::c_int as size_t;
    (*w).limit = n;
    (*w).maximum_level = 0 as libc::c_int as size_t;
    return w;
}
pub unsafe extern "C" fn gsl_integration_workspace_free(
    mut w: *mut gsl_integration_workspace,
) {
    if w.is_null() {
        return;
    }
    free((*w).level as *mut libc::c_void);
    free((*w).order as *mut libc::c_void);
    free((*w).elist as *mut libc::c_void);
    free((*w).rlist as *mut libc::c_void);
    free((*w).blist as *mut libc::c_void);
    free((*w).alist as *mut libc::c_void);
    free(w as *mut libc::c_void);
}
