use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
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
pub struct gsl_multifit_linear_workspace {
    pub nmax: size_t,
    pub pmax: size_t,
    pub n: size_t,
    pub p: size_t,
    pub A: *mut gsl_matrix,
    pub Q: *mut gsl_matrix,
    pub QSI: *mut gsl_matrix,
    pub S: *mut gsl_vector,
    pub t: *mut gsl_vector,
    pub xt: *mut gsl_vector,
    pub D: *mut gsl_vector,
    pub rcond: libc::c_double,
}
pub unsafe extern "C" fn gsl_multifit_linear_alloc(
    nmax: size_t,
    pmax: size_t,
) -> *mut gsl_multifit_linear_workspace {
    let mut w: *mut gsl_multifit_linear_workspace = 0
        as *mut gsl_multifit_linear_workspace;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_multifit_linear_workspace>() as libc::c_ulong,
    ) as *mut gsl_multifit_linear_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for multifit_linear struct\0" as *const u8
                as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    (*w).nmax = nmax;
    (*w).pmax = pmax;
    (*w).n = 0 as libc::c_int as size_t;
    (*w).p = 0 as libc::c_int as size_t;
    (*w).rcond = 0.0f64;
    (*w).A = gsl_matrix_alloc(nmax, pmax);
    if ((*w).A).is_null() {
        gsl_multifit_linear_free(w);
        gsl_error(
            b"failed to allocate space for A\0" as *const u8 as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    (*w).Q = gsl_matrix_alloc(pmax, pmax);
    if ((*w).Q).is_null() {
        gsl_multifit_linear_free(w);
        gsl_error(
            b"failed to allocate space for Q\0" as *const u8 as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    (*w).QSI = gsl_matrix_alloc(pmax, pmax);
    if ((*w).QSI).is_null() {
        gsl_multifit_linear_free(w);
        gsl_error(
            b"failed to allocate space for QSI\0" as *const u8 as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    (*w).S = gsl_vector_alloc(pmax);
    if ((*w).S).is_null() {
        gsl_multifit_linear_free(w);
        gsl_error(
            b"failed to allocate space for S\0" as *const u8 as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    (*w).t = gsl_vector_alloc(nmax);
    if ((*w).t).is_null() {
        gsl_multifit_linear_free(w);
        gsl_error(
            b"failed to allocate space for t\0" as *const u8 as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    (*w).xt = gsl_vector_calloc(pmax);
    if ((*w).xt).is_null() {
        gsl_multifit_linear_free(w);
        gsl_error(
            b"failed to allocate space for xt\0" as *const u8 as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    (*w).D = gsl_vector_calloc(pmax);
    if ((*w).D).is_null() {
        gsl_multifit_linear_free(w);
        gsl_error(
            b"failed to allocate space for D\0" as *const u8 as *const libc::c_char,
            b"work.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_multifit_linear_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_multifit_linear_free(
    mut w: *mut gsl_multifit_linear_workspace,
) {
    if w.is_null() {
        return;
    }
    if !((*w).A).is_null() {
        gsl_matrix_free((*w).A);
    }
    if !((*w).Q).is_null() {
        gsl_matrix_free((*w).Q);
    }
    if !((*w).QSI).is_null() {
        gsl_matrix_free((*w).QSI);
    }
    if !((*w).S).is_null() {
        gsl_vector_free((*w).S);
    }
    if !((*w).t).is_null() {
        gsl_vector_free((*w).t);
    }
    if !((*w).xt).is_null() {
        gsl_vector_free((*w).xt);
    }
    if !((*w).D).is_null() {
        gsl_vector_free((*w).D);
    }
    free(w as *mut libc::c_void);
}
