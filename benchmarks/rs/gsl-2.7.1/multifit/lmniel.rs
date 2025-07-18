use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_max(v: *const gsl_vector) -> libc::c_double;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_vector_add_constant(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_diagonal(m: *mut gsl_matrix) -> _gsl_vector_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_transpose_tricpy(
        Uplo_src: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        dest: *mut gsl_matrix,
        src: *const gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_decomp(A: *mut gsl_matrix, tau: *mut gsl_vector) -> libc::c_int;
    fn gsl_linalg_QR_solve(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        b: *const gsl_vector,
        x: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_multifit_eval_wf(
        fdf: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_multifit_eval_wdf(
        fdf: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        dy: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_multifit_fdfsolver_dif_df(
        x: *const gsl_vector,
        wts: *const gsl_vector,
        fdf: *mut gsl_multifit_function_fdf,
        f: *const gsl_vector,
        J: *mut gsl_matrix,
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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
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
pub struct gsl_multifit_function_fdf_struct {
    pub f: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub df: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub fdf: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
            *mut gsl_matrix,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
}
pub type gsl_multifit_function_fdf = gsl_multifit_function_fdf_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_fdfsolver_type {
    pub name: *const libc::c_char,
    pub size: size_t,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> libc::c_int,
    >,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const gsl_vector,
            *mut gsl_multifit_function_fdf,
            *mut gsl_vector,
            *mut gsl_vector,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub gradient: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut gsl_vector) -> libc::c_int,
    >,
    pub jac: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut gsl_matrix) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lmniel_state_t {
    pub A: *mut gsl_matrix,
    pub A_copy: *mut gsl_matrix,
    pub J: *mut gsl_matrix,
    pub diag: *mut gsl_vector,
    pub rhs: *mut gsl_vector,
    pub x_trial: *mut gsl_vector,
    pub f_trial: *mut gsl_vector,
    pub work: *mut gsl_vector,
    pub nu: libc::c_long,
    pub mu: libc::c_double,
    pub tau: libc::c_double,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
unsafe extern "C" fn lmniel_calc_dx(
    mu: libc::c_double,
    mut A: *const gsl_matrix,
    mut rhs: *const gsl_vector,
    mut dx: *mut gsl_vector,
    mut state: *mut lmniel_state_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut A_copy: *mut gsl_matrix = (*state).A_copy;
    let mut diag: gsl_vector_view = gsl_matrix_diagonal(A_copy);
    gsl_matrix_memcpy(A_copy, A);
    gsl_vector_add_constant(&mut diag.vector, mu);
    status = gsl_linalg_QR_decomp(A_copy, (*state).work);
    if status != 0 {
        return status;
    }
    status = gsl_linalg_QR_solve(A_copy, (*state).work, rhs, dx);
    if status != 0 {
        return status;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lmniel_trial_step(
    mut x: *const gsl_vector,
    mut dx: *const gsl_vector,
    mut x_trial: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut N: size_t = (*x).size;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut dxi: libc::c_double = gsl_vector_get(dx, i);
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        gsl_vector_set(x_trial, i, xi + dxi);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn lmniel_calc_dF(
    mut f: *const gsl_vector,
    mut f_new: *const gsl_vector,
) -> libc::c_double {
    let N: size_t = (*f).size;
    let mut i: size_t = 0;
    let mut dF: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut fnewi: libc::c_double = gsl_vector_get(f_new, i);
        dF += (fi - fnewi) * (fi + fnewi);
        i = i.wrapping_add(1);
        i;
    }
    dF *= 0.5f64;
    return dF;
}
unsafe extern "C" fn lmniel_calc_dL(
    mu: libc::c_double,
    mut diag: *const gsl_vector,
    mut dx: *const gsl_vector,
    mut mg: *const gsl_vector,
) -> libc::c_double {
    let p: size_t = (*dx).size;
    let mut i: size_t = 0;
    let mut dL: libc::c_double = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < p {
        let mut dxi: libc::c_double = gsl_vector_get(dx, i);
        let mut di: libc::c_double = gsl_vector_get(diag, i);
        let mut mgi: libc::c_double = gsl_vector_get(mg, i);
        dL += dxi * (mu * di * di * dxi + mgi);
        i = i.wrapping_add(1);
        i;
    }
    dL *= 0.5f64;
    return dL;
}
unsafe extern "C" fn lmniel_alloc(
    mut vstate: *mut libc::c_void,
    n: size_t,
    p: size_t,
) -> libc::c_int {
    let mut state: *mut lmniel_state_t = vstate as *mut lmniel_state_t;
    (*state).A = gsl_matrix_alloc(p, p);
    if ((*state).A).is_null() {
        gsl_error(
            b"failed to allocate space for A\0" as *const u8 as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).J = gsl_matrix_alloc(n, p);
    if ((*state).J).is_null() {
        gsl_error(
            b"failed to allocate space for J\0" as *const u8 as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).diag = gsl_vector_alloc(p);
    if ((*state).diag).is_null() {
        gsl_error(
            b"failed to allocate space for diag\0" as *const u8 as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).rhs = gsl_vector_alloc(p);
    if ((*state).rhs).is_null() {
        gsl_error(
            b"failed to allocate space for rhs\0" as *const u8 as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).work = gsl_vector_alloc(p);
    if ((*state).work).is_null() {
        gsl_error(
            b"failed to allocate space for work\0" as *const u8 as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).A_copy = gsl_matrix_alloc(p, p);
    if ((*state).A_copy).is_null() {
        gsl_error(
            b"failed to allocate space for A_copy\0" as *const u8 as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x_trial = gsl_vector_alloc(p);
    if ((*state).x_trial).is_null() {
        gsl_error(
            b"failed to allocate space for x_trial\0" as *const u8
                as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).f_trial = gsl_vector_alloc(n);
    if ((*state).f_trial).is_null() {
        gsl_error(
            b"failed to allocate space for f_trial\0" as *const u8
                as *const libc::c_char,
            b"lmniel.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).tau = 1.0e-3f64;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lmniel_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut lmniel_state_t = vstate as *mut lmniel_state_t;
    if !((*state).A).is_null() {
        gsl_matrix_free((*state).A);
    }
    if !((*state).J).is_null() {
        gsl_matrix_free((*state).J);
    }
    if !((*state).diag).is_null() {
        gsl_vector_free((*state).diag);
    }
    if !((*state).rhs).is_null() {
        gsl_vector_free((*state).rhs);
    }
    if !((*state).work).is_null() {
        gsl_vector_free((*state).work);
    }
    if !((*state).A_copy).is_null() {
        gsl_matrix_free((*state).A_copy);
    }
    if !((*state).x_trial).is_null() {
        gsl_vector_free((*state).x_trial);
    }
    if !((*state).f_trial).is_null() {
        gsl_vector_free((*state).f_trial);
    }
}
unsafe extern "C" fn lmniel_set(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut state: *mut lmniel_state_t = vstate as *mut lmniel_state_t;
    let p: size_t = (*x).size;
    let mut i: size_t = 0;
    (*fdf).nevalf = 0 as libc::c_int as size_t;
    (*fdf).nevaldf = 0 as libc::c_int as size_t;
    status = gsl_multifit_eval_wf(fdf, x, swts, f);
    if status != 0 {
        return status;
    }
    if ((*fdf).df).is_some() {
        status = gsl_multifit_eval_wdf(fdf, x, swts, (*state).J);
    } else {
        status = gsl_multifit_fdfsolver_dif_df(x, swts, fdf, f, (*state).J);
    }
    if status != 0 {
        return status;
    }
    gsl_blas_dgemv(CblasTrans, -1.0f64, (*state).J, f, 0.0f64, (*state).rhs);
    gsl_vector_set_all((*state).diag, 1.0f64);
    (*state).nu = 2 as libc::c_int as libc::c_long;
    (*state).mu = -1.0f64;
    i = 0 as libc::c_int as size_t;
    while i < p {
        let mut c: gsl_vector_view = gsl_matrix_column((*state).J, i);
        let mut result: libc::c_double = 0.;
        gsl_blas_ddot(&mut c.vector, &mut c.vector, &mut result);
        (*state).mu = if (*state).mu > result { (*state).mu } else { result };
        i = i.wrapping_add(1);
        i;
    }
    (*state).mu *= (*state).tau;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lmniel_iterate(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut state: *mut lmniel_state_t = vstate as *mut lmniel_state_t;
    let mut J: *mut gsl_matrix = (*state).J;
    let mut A: *mut gsl_matrix = (*state).A;
    let mut rhs: *mut gsl_vector = (*state).rhs;
    let mut x_trial: *mut gsl_vector = (*state).x_trial;
    let mut f_trial: *mut gsl_vector = (*state).f_trial;
    let mut diag: *mut gsl_vector = (*state).diag;
    let mut dF: libc::c_double = 0.;
    let mut dL: libc::c_double = 0.;
    let mut foundstep: libc::c_int = 0 as libc::c_int;
    status = gsl_blas_dsyrk(CblasLower, CblasTrans, 1.0f64, J, 0.0f64, A);
    if status != 0 {
        return status;
    }
    gsl_matrix_transpose_tricpy(CblasLower, CblasUnit, A, A);
    while foundstep == 0 {
        status = lmniel_calc_dx((*state).mu, A, rhs, dx, state);
        if status != 0 {
            return status;
        }
        lmniel_trial_step(x, dx, x_trial);
        status = gsl_multifit_eval_wf(fdf, x_trial, swts, f_trial);
        if status != 0 {
            return status;
        }
        dF = lmniel_calc_dF(f, f_trial);
        dL = lmniel_calc_dL((*state).mu, diag, dx, rhs);
        if dL > 0.0f64 && dF >= 0.0f64 {
            let mut tmp: libc::c_double = 0.;
            tmp = 2.0f64 * (dF / dL) - 1.0f64;
            tmp = 1.0f64 - tmp * tmp * tmp;
            (*state).mu
                *= if 0.333333333333333f64 > tmp { 0.333333333333333f64 } else { tmp };
            (*state).nu = 2 as libc::c_int as libc::c_long;
            if ((*fdf).df).is_some() {
                status = gsl_multifit_eval_wdf(fdf, x_trial, swts, J);
            } else {
                status = gsl_multifit_fdfsolver_dif_df(x_trial, swts, fdf, f_trial, J);
            }
            if status != 0 {
                return status;
            }
            gsl_vector_memcpy(x, x_trial);
            gsl_vector_memcpy(f, f_trial);
            gsl_blas_dgemv(CblasTrans, -1.0f64, J, f, 0.0f64, rhs);
            foundstep = 1 as libc::c_int;
        } else {
            let mut nu2: libc::c_long = 0;
            (*state).mu *= (*state).nu as libc::c_double;
            nu2 = (*state).nu << 1 as libc::c_int;
            if nu2 <= (*state).nu {
                let mut d: gsl_vector_view = gsl_matrix_diagonal(A);
                (*state).nu = 2 as libc::c_int as libc::c_long;
                (*state).mu = (*state).tau * gsl_vector_max(&mut d.vector);
                break;
            } else {
                (*state).nu = nu2;
            }
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lmniel_gradient(
    mut vstate: *mut libc::c_void,
    mut g: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut lmniel_state_t = vstate as *mut lmniel_state_t;
    gsl_vector_memcpy(g, (*state).rhs);
    gsl_vector_scale(g, -1.0f64);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lmniel_jac(
    mut vstate: *mut libc::c_void,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    let mut state: *mut lmniel_state_t = vstate as *mut lmniel_state_t;
    let mut s: libc::c_int = gsl_matrix_memcpy(J, (*state).J);
    return s;
}
static mut lmniel_type: gsl_multifit_fdfsolver_type = {
    let mut init = gsl_multifit_fdfsolver_type {
        name: b"lmniel\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<lmniel_state_t>() as libc::c_ulong,
        alloc: Some(
            lmniel_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> libc::c_int,
        ),
        set: Some(
            lmniel_set
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_vector,
                    *mut gsl_multifit_function_fdf,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        iterate: Some(
            lmniel_iterate
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const gsl_vector,
                    *mut gsl_multifit_function_fdf,
                    *mut gsl_vector,
                    *mut gsl_vector,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        gradient: Some(
            lmniel_gradient
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        jac: Some(
            lmniel_jac
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_matrix,
                ) -> libc::c_int,
        ),
        free: Some(lmniel_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multifit_fdfsolver_lmniel: *const gsl_multifit_fdfsolver_type = unsafe {
    &lmniel_type as *const gsl_multifit_fdfsolver_type
};
