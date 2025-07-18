use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_multifit_fdfsolver_dif_df(
        x: *const gsl_vector,
        wts: *const gsl_vector,
        fdf: *mut gsl_multifit_function_fdf,
        f: *const gsl_vector,
        J: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_multifit_eval_wdf(
        fdf: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        dy: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_multifit_eval_wf(
        fdf: *mut gsl_multifit_function_fdf,
        x: *const gsl_vector,
        wts: *const gsl_vector,
        y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_permutation_calloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_const_column(
        m: *const gsl_matrix,
        j: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_idamax(X: *const gsl_vector) -> CBLAS_INDEX_t;
    fn gsl_linalg_QR_QTvec(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        v: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QRPT_decomp(
        A: *mut gsl_matrix,
        tau: *mut gsl_vector,
        p: *mut gsl_permutation,
        signum: *mut libc::c_int,
        norm: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_permute_vector_inverse(
        p: *const gsl_permutation,
        v: *mut gsl_vector,
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
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
pub type CBLAS_INDEX_t = size_t;
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
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
pub struct lmder_state_t {
    pub iter: size_t,
    pub xnorm: libc::c_double,
    pub fnorm: libc::c_double,
    pub delta: libc::c_double,
    pub par: libc::c_double,
    pub J: *mut gsl_matrix,
    pub r: *mut gsl_matrix,
    pub tau: *mut gsl_vector,
    pub diag: *mut gsl_vector,
    pub qtf: *mut gsl_vector,
    pub newton: *mut gsl_vector,
    pub gradient: *mut gsl_vector,
    pub x_trial: *mut gsl_vector,
    pub f_trial: *mut gsl_vector,
    pub df: *mut gsl_vector,
    pub sdiag: *mut gsl_vector,
    pub rptdx: *mut gsl_vector,
    pub weights: *const gsl_vector,
    pub w: *mut gsl_vector,
    pub work1: *mut gsl_vector,
    pub perm: *mut gsl_permutation,
}
#[inline]
unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
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
#[inline]
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_permutation_get(
    mut p: *const gsl_permutation,
    i: size_t,
) -> size_t {
    return *((*p).data).offset(i as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
unsafe extern "C" fn enorm(mut f: *const gsl_vector) -> libc::c_double {
    return gsl_blas_dnrm2(f);
}
unsafe extern "C" fn scaled_enorm(
    mut d: *const gsl_vector,
    mut f: *const gsl_vector,
) -> libc::c_double {
    let mut e2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: size_t = 0;
    let mut n: size_t = (*f).size;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut fi: libc::c_double = gsl_vector_get(f, i);
        let mut di: libc::c_double = gsl_vector_get(d, i);
        let mut u: libc::c_double = di * fi;
        e2 += u * u;
        i = i.wrapping_add(1);
        i;
    }
    return sqrt(e2);
}
unsafe extern "C" fn compute_delta(
    mut diag: *mut gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_double {
    let mut Dx: libc::c_double = scaled_enorm(diag, x);
    let mut factor: libc::c_double = 100 as libc::c_int as libc::c_double;
    return if Dx > 0 as libc::c_int as libc::c_double { factor * Dx } else { factor };
}
unsafe extern "C" fn compute_actual_reduction(
    mut fnorm: libc::c_double,
    mut fnorm1: libc::c_double,
) -> libc::c_double {
    let mut actred: libc::c_double = 0.;
    if 0.1f64 * fnorm1 < fnorm {
        let mut u: libc::c_double = fnorm1 / fnorm;
        actred = 1 as libc::c_int as libc::c_double - u * u;
    } else {
        actred = -(1 as libc::c_int) as libc::c_double;
    }
    return actred;
}
unsafe extern "C" fn compute_diag(mut J: *const gsl_matrix, mut diag: *mut gsl_vector) {
    let mut j: size_t = 0;
    let mut p: size_t = (*J).size2;
    j = 0 as libc::c_int as size_t;
    while j < p {
        let v: gsl_vector_const_view = gsl_matrix_const_column(J, j);
        let mut norm: libc::c_double = gsl_blas_dnrm2(&v.vector);
        if norm == 0 as libc::c_int as libc::c_double {
            norm = 1.0f64;
        }
        gsl_vector_set(diag, j, norm);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn update_diag(mut J: *const gsl_matrix, mut diag: *mut gsl_vector) {
    let mut j: size_t = 0;
    let mut p: size_t = (*J).size2;
    j = 0 as libc::c_int as size_t;
    while j < p {
        let v: gsl_vector_const_view = gsl_matrix_const_column(J, j);
        let mut norm: libc::c_double = gsl_blas_dnrm2(&v.vector);
        let mut diagj: *mut libc::c_double = gsl_vector_ptr(diag, j);
        if norm == 0 as libc::c_int as libc::c_double {
            norm = 1.0f64;
        }
        *diagj = if *diagj > norm { *diagj } else { norm };
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn compute_rptdx(
    mut r: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut dx: *const gsl_vector,
    mut rptdx: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut N: size_t = (*dx).size;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = i;
        while j < N {
            let mut pj: size_t = gsl_permutation_get(p, j);
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(dx, pj);
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_set(rptdx, i, sum);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn compute_trial_step(
    mut x: *mut gsl_vector,
    mut dx: *mut gsl_vector,
    mut x_trial: *mut gsl_vector,
) {
    let mut i: size_t = 0;
    let mut N: size_t = (*x).size;
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut pi: libc::c_double = gsl_vector_get(dx, i);
        let mut xi: libc::c_double = gsl_vector_get(x, i);
        gsl_vector_set(x_trial, i, xi + pi);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn count_nsing(mut r: *const gsl_matrix) -> size_t {
    let mut n: size_t = (*r).size2;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut rii: libc::c_double = gsl_matrix_get(r, i, i);
        if rii == 0 as libc::c_int as libc::c_double {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    return i;
}
unsafe extern "C" fn compute_newton_direction(
    mut r: *const gsl_matrix,
    mut perm: *const gsl_permutation,
    mut qtf: *const gsl_vector,
    mut x: *mut gsl_vector,
) {
    let n: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut nsing: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut qtfi: libc::c_double = gsl_vector_get(qtf, i);
        gsl_vector_set(x, i, qtfi);
        i = i.wrapping_add(1);
        i;
    }
    nsing = count_nsing(r);
    i = nsing;
    while i < n {
        gsl_vector_set(x, i, 0.0f64);
        i = i.wrapping_add(1);
        i;
    }
    if nsing > 0 as libc::c_int as libc::c_ulong {
        j = nsing;
        while j > 0 as libc::c_int as libc::c_ulong
            && {
                let fresh0 = j;
                j = j.wrapping_sub(1);
                fresh0 != 0
            }
        {
            let mut rjj: libc::c_double = gsl_matrix_get(r, j, j);
            let mut temp: libc::c_double = gsl_vector_get(x, j) / rjj;
            gsl_vector_set(x, j, temp);
            i = 0 as libc::c_int as size_t;
            while i < j {
                let mut rij: libc::c_double = gsl_matrix_get(r, i, j);
                let mut xi: libc::c_double = gsl_vector_get(x, i);
                gsl_vector_set(x, i, xi - rij * temp);
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    gsl_permute_vector_inverse(perm, x);
}
unsafe extern "C" fn compute_newton_correction(
    mut r: *const gsl_matrix,
    mut sdiag: *const gsl_vector,
    mut p: *const gsl_permutation,
    mut x: *mut gsl_vector,
    mut dxnorm: libc::c_double,
    mut diag: *const gsl_vector,
    mut w: *mut gsl_vector,
) {
    let mut n: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut pi: size_t = gsl_permutation_get(p, i);
        let mut dpi: libc::c_double = gsl_vector_get(diag, pi);
        let mut xpi: libc::c_double = gsl_vector_get(x, pi);
        gsl_vector_set(w, i, dpi * (dpi * xpi) / dxnorm);
        i = i.wrapping_add(1);
        i;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut sj: libc::c_double = gsl_vector_get(sdiag, j);
        let mut wj: libc::c_double = gsl_vector_get(w, j);
        let mut tj: libc::c_double = wj / sj;
        gsl_vector_set(w, j, tj);
        i = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while i < n {
            let mut rij: libc::c_double = gsl_matrix_get(r, i, j);
            let mut wi: libc::c_double = gsl_vector_get(w, i);
            gsl_vector_set(w, i, wi - rij * tj);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn compute_newton_bound(
    mut r: *const gsl_matrix,
    mut x: *const gsl_vector,
    mut dxnorm: libc::c_double,
    mut perm: *const gsl_permutation,
    mut diag: *const gsl_vector,
    mut w: *mut gsl_vector,
) {
    let mut n: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut nsing: size_t = count_nsing(r);
    if nsing < n {
        gsl_vector_set_zero(w);
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut pi: size_t = gsl_permutation_get(perm, i);
        let mut dpi: libc::c_double = gsl_vector_get(diag, pi);
        let mut xpi: libc::c_double = gsl_vector_get(x, pi);
        gsl_vector_set(w, i, dpi * (dpi * xpi / dxnorm));
        i = i.wrapping_add(1);
        i;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i < j {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(w, i);
            i = i.wrapping_add(1);
            i;
        }
        let mut rjj: libc::c_double = gsl_matrix_get(r, j, j);
        let mut wj: libc::c_double = gsl_vector_get(w, j);
        gsl_vector_set(w, j, (wj - sum) / rjj);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn compute_gradient_direction(
    mut r: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut qtf: *const gsl_vector,
    mut diag: *const gsl_vector,
    mut g: *mut gsl_vector,
) {
    let n: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i <= j {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(qtf, i);
            i = i.wrapping_add(1);
            i;
        }
        let mut pj: size_t = gsl_permutation_get(p, j);
        let mut dpj: libc::c_double = gsl_vector_get(diag, pj);
        gsl_vector_set(g, j, sum / dpj);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn compute_gradient(
    mut r: *const gsl_matrix,
    mut qtf: *const gsl_vector,
    mut g: *mut gsl_vector,
) {
    let n: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int as size_t;
        while i <= j {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(qtf, i);
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(g, j, sum);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn lmpar(
    mut r: *mut gsl_matrix,
    mut perm: *const gsl_permutation,
    mut qtf: *const gsl_vector,
    mut diag: *const gsl_vector,
    mut delta: libc::c_double,
    mut par_inout: *mut libc::c_double,
    mut newton: *mut gsl_vector,
    mut gradient: *mut gsl_vector,
    mut sdiag: *mut gsl_vector,
    mut x: *mut gsl_vector,
    mut w: *mut gsl_vector,
) -> libc::c_int {
    let mut dxnorm: libc::c_double = 0.;
    let mut gnorm: libc::c_double = 0.;
    let mut fp: libc::c_double = 0.;
    let mut fp_old: libc::c_double = 0.;
    let mut par_lower: libc::c_double = 0.;
    let mut par_upper: libc::c_double = 0.;
    let mut par_c: libc::c_double = 0.;
    let mut par: libc::c_double = *par_inout;
    let mut iter: size_t = 0 as libc::c_int as size_t;
    compute_newton_direction(r, perm, qtf, newton);
    dxnorm = scaled_enorm(diag, newton);
    fp = dxnorm - delta;
    if fp <= 0.1f64 * delta {
        gsl_vector_memcpy(x, newton);
        *par_inout = 0 as libc::c_int as libc::c_double;
        return GSL_SUCCESS as libc::c_int;
    }
    compute_newton_bound(r, newton, dxnorm, perm, diag, w);
    let mut wnorm: libc::c_double = enorm(w);
    let mut phider: libc::c_double = wnorm * wnorm;
    if wnorm > 0 as libc::c_int as libc::c_double {
        par_lower = fp / (delta * phider);
    } else {
        par_lower = 0.0f64;
    }
    compute_gradient_direction(r, perm, qtf, diag, gradient);
    gnorm = enorm(gradient);
    par_upper = gnorm / delta;
    if par_upper == 0 as libc::c_int as libc::c_double {
        par_upper = 2.2250738585072014e-308f64 / GSL_MIN_DBL(delta, 0.1f64);
    }
    if par > par_upper {
        par = par_upper;
    } else if par < par_lower {
        par = par_lower;
    }
    if par == 0 as libc::c_int as libc::c_double {
        par = gnorm / dxnorm;
    }
    loop {
        iter = iter.wrapping_add(1);
        iter;
        if par == 0 as libc::c_int as libc::c_double {
            par = GSL_MAX_DBL(0.001f64 * par_upper, 2.2250738585072014e-308f64);
        }
        let mut sqrt_par: libc::c_double = sqrt(par);
        qrsolv(r, perm, sqrt_par, diag, qtf, x, sdiag, w);
        dxnorm = scaled_enorm(diag, x);
        fp_old = fp;
        fp = dxnorm - delta;
        if fabs(fp) <= 0.1f64 * delta {
            break;
        }
        if par_lower == 0 as libc::c_int as libc::c_double && fp <= fp_old
            && fp_old < 0 as libc::c_int as libc::c_double
        {
            break;
        }
        if iter == 10 as libc::c_int as libc::c_ulong {
            break;
        }
        compute_newton_correction(r, sdiag, perm, x, dxnorm, diag, w);
        let mut wnorm_0: libc::c_double = enorm(w);
        par_c = fp / (delta * wnorm_0 * wnorm_0);
        if fp > 0 as libc::c_int as libc::c_double {
            if par > par_lower {
                par_lower = par;
            }
        } else if fp < 0 as libc::c_int as libc::c_double {
            if par < par_upper {
                par_upper = par;
            }
        }
        par = GSL_MAX_DBL(par_lower, par + par_c);
    }
    *par_inout = par;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn qrsolv(
    mut r: *mut gsl_matrix,
    mut p: *const gsl_permutation,
    lambda: libc::c_double,
    mut diag: *const gsl_vector,
    mut qtb: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut sdiag: *mut gsl_vector,
    mut wa: *mut gsl_vector,
) -> libc::c_int {
    let mut n: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut nsing: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut rjj: libc::c_double = gsl_matrix_get(r, j, j);
        let mut qtbj: libc::c_double = gsl_vector_get(qtb, j);
        i = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while i < n {
            let mut rji: libc::c_double = gsl_matrix_get(r, j, i);
            gsl_matrix_set(r, i, j, rji);
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(x, j, rjj);
        gsl_vector_set(wa, j, qtbj);
        j = j.wrapping_add(1);
        j;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut qtbpj: libc::c_double = 0.;
        let mut pj: size_t = gsl_permutation_get(p, j);
        let mut diagpj: libc::c_double = lambda * gsl_vector_get(diag, pj);
        if !(diagpj == 0 as libc::c_int as libc::c_double) {
            gsl_vector_set(sdiag, j, diagpj);
            k = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < n {
                gsl_vector_set(sdiag, k, 0.0f64);
                k = k.wrapping_add(1);
                k;
            }
            qtbpj = 0 as libc::c_int as libc::c_double;
            k = j;
            while k < n {
                let mut sine: libc::c_double = 0.;
                let mut cosine: libc::c_double = 0.;
                let mut wak: libc::c_double = gsl_vector_get(wa, k);
                let mut rkk: libc::c_double = gsl_matrix_get(r, k, k);
                let mut sdiagk: libc::c_double = gsl_vector_get(sdiag, k);
                if !(sdiagk == 0 as libc::c_int as libc::c_double) {
                    if fabs(rkk) < fabs(sdiagk) {
                        let mut cotangent: libc::c_double = rkk / sdiagk;
                        sine = 0.5f64 / sqrt(0.25f64 + 0.25f64 * cotangent * cotangent);
                        cosine = sine * cotangent;
                    } else {
                        let mut tangent: libc::c_double = sdiagk / rkk;
                        cosine = 0.5f64 / sqrt(0.25f64 + 0.25f64 * tangent * tangent);
                        sine = cosine * tangent;
                    }
                    let mut new_rkk: libc::c_double = cosine * rkk + sine * sdiagk;
                    let mut new_wak: libc::c_double = cosine * wak + sine * qtbpj;
                    qtbpj = -sine * wak + cosine * qtbpj;
                    gsl_matrix_set(r, k, k, new_rkk);
                    gsl_vector_set(wa, k, new_wak);
                    i = k.wrapping_add(1 as libc::c_int as libc::c_ulong);
                    while i < n {
                        let mut rik: libc::c_double = gsl_matrix_get(r, i, k);
                        let mut sdiagi: libc::c_double = gsl_vector_get(sdiag, i);
                        let mut new_rik: libc::c_double = cosine * rik + sine * sdiagi;
                        let mut new_sdiagi: libc::c_double = -sine * rik
                            + cosine * sdiagi;
                        gsl_matrix_set(r, i, k, new_rik);
                        gsl_vector_set(sdiag, i, new_sdiagi);
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                k = k.wrapping_add(1);
                k;
            }
            let mut rjj_0: libc::c_double = gsl_matrix_get(r, j, j);
            let mut xj: libc::c_double = gsl_vector_get(x, j);
            gsl_vector_set(sdiag, j, rjj_0);
            gsl_matrix_set(r, j, j, xj);
        }
        j = j.wrapping_add(1);
        j;
    }
    nsing = n;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut sdiagj: libc::c_double = gsl_vector_get(sdiag, j);
        if sdiagj == 0 as libc::c_int as libc::c_double {
            nsing = j;
            break;
        } else {
            j = j.wrapping_add(1);
            j;
        }
    }
    j = nsing;
    while j < n {
        gsl_vector_set(wa, j, 0.0f64);
        j = j.wrapping_add(1);
        j;
    }
    k = 0 as libc::c_int as size_t;
    while k < nsing {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = nsing.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_sub(k);
        i = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while i < nsing {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(wa, i);
            i = i.wrapping_add(1);
            i;
        }
        let mut waj: libc::c_double = gsl_vector_get(wa, j);
        let mut sdiagj_0: libc::c_double = gsl_vector_get(sdiag, j);
        gsl_vector_set(wa, j, (waj - sum) / sdiagj_0);
        k = k.wrapping_add(1);
        k;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut pj_0: size_t = gsl_permutation_get(p, j);
        let mut waj_0: libc::c_double = gsl_vector_get(wa, j);
        gsl_vector_set(x, pj_0, waj_0);
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn set(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut state: *mut lmder_state_t = vstate as *mut lmder_state_t;
    let mut r: *mut gsl_matrix = (*state).r;
    let mut tau: *mut gsl_vector = (*state).tau;
    let mut qtf: *mut gsl_vector = (*state).qtf;
    let mut diag: *mut gsl_vector = (*state).diag;
    let mut work1: *mut gsl_vector = (*state).work1;
    let mut perm: *mut gsl_permutation = (*state).perm;
    let mut signum: libc::c_int = 0;
    (*fdf).nevalf = 0 as libc::c_int as size_t;
    (*fdf).nevaldf = 0 as libc::c_int as size_t;
    let mut status: libc::c_int = 0;
    status = gsl_multifit_eval_wf(fdf, x, swts, f);
    if status != 0 {
        return status;
    }
    if ((*fdf).df).is_some() {
        status = gsl_multifit_eval_wdf(fdf, x, swts, r);
    } else {
        status = gsl_multifit_fdfsolver_dif_df(x, swts, fdf, f, r);
    }
    gsl_matrix_memcpy((*state).J, r);
    if status != 0 {
        return status;
    }
    (*state).par = 0 as libc::c_int as libc::c_double;
    (*state).iter = 1 as libc::c_int as size_t;
    (*state).fnorm = enorm(f);
    gsl_vector_set_all(dx, 0.0f64);
    if scale != 0 {
        compute_diag(r, diag);
    } else {
        gsl_vector_set_all(diag, 1.0f64);
    }
    (*state).xnorm = scaled_enorm(diag, x);
    (*state).delta = compute_delta(diag, x);
    gsl_linalg_QRPT_decomp(r, tau, perm, &mut signum, work1);
    gsl_vector_memcpy(qtf, f);
    gsl_linalg_QR_QTvec(r, tau, qtf);
    gsl_vector_set_zero((*state).rptdx);
    gsl_vector_set_zero((*state).w);
    gsl_vector_set_zero((*state).f_trial);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn iterate(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut state: *mut lmder_state_t = vstate as *mut lmder_state_t;
    let mut r: *mut gsl_matrix = (*state).r;
    let mut tau: *mut gsl_vector = (*state).tau;
    let mut diag: *mut gsl_vector = (*state).diag;
    let mut qtf: *mut gsl_vector = (*state).qtf;
    let mut x_trial: *mut gsl_vector = (*state).x_trial;
    let mut f_trial: *mut gsl_vector = (*state).f_trial;
    let mut rptdx: *mut gsl_vector = (*state).rptdx;
    let mut newton: *mut gsl_vector = (*state).newton;
    let mut gradient: *mut gsl_vector = (*state).gradient;
    let mut sdiag: *mut gsl_vector = (*state).sdiag;
    let mut w: *mut gsl_vector = (*state).w;
    let mut work1: *mut gsl_vector = (*state).work1;
    let mut perm: *mut gsl_permutation = (*state).perm;
    let mut prered: libc::c_double = 0.;
    let mut actred: libc::c_double = 0.;
    let mut pnorm: libc::c_double = 0.;
    let mut fnorm1: libc::c_double = 0.;
    let mut fnorm1p: libc::c_double = 0.;
    let mut gnorm: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    let mut dirder: libc::c_double = 0.;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut p1: libc::c_double = 0.1f64;
    let mut p25: libc::c_double = 0.25f64;
    let mut p5: libc::c_double = 0.5f64;
    let mut p75: libc::c_double = 0.75f64;
    let mut p0001: libc::c_double = 0.0001f64;
    if (*state).fnorm == 0.0f64 {
        return GSL_SUCCESS as libc::c_int;
    }
    compute_gradient_direction(r, perm, qtf, diag, gradient);
    let mut iamax: size_t = gsl_blas_idamax(gradient);
    gnorm = fabs(gsl_vector_get(gradient, iamax) / (*state).fnorm);
    loop {
        iter += 1;
        iter;
        let mut status: libc::c_int = lmpar(
            r,
            perm,
            qtf,
            diag,
            (*state).delta,
            &mut (*state).par,
            newton,
            gradient,
            sdiag,
            dx,
            w,
        );
        if status != 0 {
            return status;
        }
        gsl_vector_scale(dx, -1.0f64);
        compute_trial_step(x, dx, (*state).x_trial);
        pnorm = scaled_enorm(diag, dx);
        if (*state).iter == 1 as libc::c_int as libc::c_ulong {
            if pnorm < (*state).delta {
                (*state).delta = pnorm;
            }
        }
        let mut status_0: libc::c_int = gsl_multifit_eval_wf(
            fdf,
            x_trial,
            swts,
            f_trial,
        );
        if status_0 != 0 {
            return status_0;
        }
        fnorm1 = enorm(f_trial);
        actred = compute_actual_reduction((*state).fnorm, fnorm1);
        compute_rptdx(r, perm, dx, rptdx);
        fnorm1p = enorm(rptdx);
        let mut t1: libc::c_double = fnorm1p / (*state).fnorm;
        let mut t2: libc::c_double = sqrt((*state).par) * pnorm / (*state).fnorm;
        prered = t1 * t1 + t2 * t2 / p5;
        dirder = -(t1 * t1 + t2 * t2);
        if prered > 0 as libc::c_int as libc::c_double {
            ratio = actred / prered;
        } else {
            ratio = 0 as libc::c_int as libc::c_double;
        }
        if ratio > p25 {
            if (*state).par == 0 as libc::c_int as libc::c_double || ratio >= p75 {
                (*state).delta = pnorm / p5;
                (*state).par *= p5;
            }
        } else {
            let mut temp: libc::c_double = if actred
                >= 0 as libc::c_int as libc::c_double
            {
                p5
            } else {
                p5 * dirder / (dirder + p5 * actred)
            };
            if p1 * fnorm1 >= (*state).fnorm || temp < p1 {
                temp = p1;
            }
            (*state).delta = temp * GSL_MIN_DBL((*state).delta, pnorm / p1);
            (*state).par /= temp;
        }
        if ratio >= p0001 {
            gsl_vector_memcpy(x, x_trial);
            gsl_vector_memcpy(f, f_trial);
            let mut status_1: libc::c_int = 0;
            if ((*fdf).df).is_some() {
                status_1 = gsl_multifit_eval_wdf(fdf, x_trial, swts, r);
            } else {
                status_1 = gsl_multifit_fdfsolver_dif_df(x_trial, swts, fdf, f_trial, r);
            }
            if status_1 != 0 {
                return status_1;
            }
            (*state).xnorm = scaled_enorm(diag, x);
            (*state).fnorm = fnorm1;
            (*state).iter = ((*state).iter).wrapping_add(1);
            (*state).iter;
            if scale != 0 {
                update_diag(r, diag);
            }
            let mut signum: libc::c_int = 0;
            gsl_matrix_memcpy((*state).J, r);
            gsl_linalg_QRPT_decomp(r, tau, perm, &mut signum, work1);
            gsl_vector_memcpy(qtf, f);
            gsl_linalg_QR_QTvec(r, tau, qtf);
            return GSL_SUCCESS as libc::c_int;
        } else if fabs(actred) <= 2.2204460492503131e-16f64
            && prered <= 2.2204460492503131e-16f64 && p5 * ratio <= 1.0f64
        {
            return GSL_ETOLF as libc::c_int
        } else if (*state).delta <= 2.2204460492503131e-16f64 * (*state).xnorm {
            return GSL_ETOLX as libc::c_int
        } else if gnorm <= 2.2204460492503131e-16f64 {
            return GSL_ETOLG as libc::c_int
        } else {
            if iter < 10 as libc::c_int {
                continue;
            }
            return GSL_ENOPROG as libc::c_int;
        }
    };
}
unsafe extern "C" fn lmder_alloc(
    mut vstate: *mut libc::c_void,
    mut n: size_t,
    mut p: size_t,
) -> libc::c_int {
    let mut state: *mut lmder_state_t = vstate as *mut lmder_state_t;
    let mut r: *mut gsl_matrix = 0 as *mut gsl_matrix;
    let mut J: *mut gsl_matrix = 0 as *mut gsl_matrix;
    let mut tau: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut diag: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut qtf: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut newton: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut gradient: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut x_trial: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut f_trial: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut df: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut sdiag: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut rptdx: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut w: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut work1: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut perm: *mut gsl_permutation = 0 as *mut gsl_permutation;
    J = gsl_matrix_alloc(n, p);
    if J.is_null() {
        gsl_error(
            b"failed to allocate space for J\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).J = J;
    r = gsl_matrix_alloc(n, p);
    if r.is_null() {
        gsl_error(
            b"failed to allocate space for r\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).r = r;
    tau = gsl_vector_calloc(if n < p { n } else { p });
    if tau.is_null() {
        gsl_matrix_free(r);
        gsl_error(
            b"failed to allocate space for tau\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).tau = tau;
    diag = gsl_vector_calloc(p);
    if diag.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_error(
            b"failed to allocate space for diag\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).diag = diag;
    qtf = gsl_vector_calloc(n);
    if qtf.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_error(
            b"failed to allocate space for qtf\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).qtf = qtf;
    newton = gsl_vector_calloc(p);
    if newton.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_error(
            b"failed to allocate space for newton\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).newton = newton;
    gradient = gsl_vector_calloc(p);
    if gradient.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_error(
            b"failed to allocate space for gradient\0" as *const u8
                as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).gradient = gradient;
    x_trial = gsl_vector_calloc(p);
    if x_trial.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_error(
            b"failed to allocate space for x_trial\0" as *const u8
                as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).x_trial = x_trial;
    f_trial = gsl_vector_calloc(n);
    if f_trial.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_error(
            b"failed to allocate space for f_trial\0" as *const u8
                as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).f_trial = f_trial;
    df = gsl_vector_calloc(n);
    if df.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_error(
            b"failed to allocate space for df\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).df = df;
    sdiag = gsl_vector_calloc(p);
    if sdiag.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_error(
            b"failed to allocate space for sdiag\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).sdiag = sdiag;
    rptdx = gsl_vector_calloc(n);
    if rptdx.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_vector_free(sdiag);
        gsl_error(
            b"failed to allocate space for rptdx\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).rptdx = rptdx;
    w = gsl_vector_calloc(n);
    if w.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_vector_free(sdiag);
        gsl_vector_free(rptdx);
        gsl_error(
            b"failed to allocate space for w\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).w = w;
    work1 = gsl_vector_calloc(p);
    if work1.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_vector_free(sdiag);
        gsl_vector_free(rptdx);
        gsl_vector_free(w);
        gsl_error(
            b"failed to allocate space for work1\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).work1 = work1;
    perm = gsl_permutation_calloc(p);
    if perm.is_null() {
        gsl_matrix_free(r);
        gsl_vector_free(tau);
        gsl_vector_free(diag);
        gsl_vector_free(qtf);
        gsl_vector_free(newton);
        gsl_vector_free(gradient);
        gsl_vector_free(x_trial);
        gsl_vector_free(f_trial);
        gsl_vector_free(df);
        gsl_vector_free(sdiag);
        gsl_vector_free(rptdx);
        gsl_vector_free(w);
        gsl_vector_free(work1);
        gsl_error(
            b"failed to allocate space for perm\0" as *const u8 as *const libc::c_char,
            b"lmder.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    (*state).perm = perm;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lmder_set(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = set(vstate, swts, fdf, x, f, dx, 0 as libc::c_int);
    return status;
}
unsafe extern "C" fn lmsder_set(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = set(vstate, swts, fdf, x, f, dx, 1 as libc::c_int);
    return status;
}
unsafe extern "C" fn lmder_iterate(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = iterate(vstate, swts, fdf, x, f, dx, 0 as libc::c_int);
    return status;
}
unsafe extern "C" fn lmsder_iterate(
    mut vstate: *mut libc::c_void,
    mut swts: *const gsl_vector,
    mut fdf: *mut gsl_multifit_function_fdf,
    mut x: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut dx: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = iterate(vstate, swts, fdf, x, f, dx, 1 as libc::c_int);
    return status;
}
unsafe extern "C" fn lmder_gradient(
    mut vstate: *mut libc::c_void,
    mut g: *mut gsl_vector,
) -> libc::c_int {
    let mut state: *mut lmder_state_t = vstate as *mut lmder_state_t;
    compute_gradient((*state).r, (*state).qtf, g);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lmder_jac(
    mut vstate: *mut libc::c_void,
    mut J: *mut gsl_matrix,
) -> libc::c_int {
    let mut state: *mut lmder_state_t = vstate as *mut lmder_state_t;
    let mut s: libc::c_int = gsl_matrix_memcpy(J, (*state).J);
    return s;
}
unsafe extern "C" fn lmder_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut lmder_state_t = vstate as *mut lmder_state_t;
    if !((*state).perm).is_null() {
        gsl_permutation_free((*state).perm);
    }
    if !((*state).work1).is_null() {
        gsl_vector_free((*state).work1);
    }
    if !((*state).w).is_null() {
        gsl_vector_free((*state).w);
    }
    if !((*state).rptdx).is_null() {
        gsl_vector_free((*state).rptdx);
    }
    if !((*state).sdiag).is_null() {
        gsl_vector_free((*state).sdiag);
    }
    if !((*state).df).is_null() {
        gsl_vector_free((*state).df);
    }
    if !((*state).f_trial).is_null() {
        gsl_vector_free((*state).f_trial);
    }
    if !((*state).x_trial).is_null() {
        gsl_vector_free((*state).x_trial);
    }
    if !((*state).gradient).is_null() {
        gsl_vector_free((*state).gradient);
    }
    if !((*state).newton).is_null() {
        gsl_vector_free((*state).newton);
    }
    if !((*state).qtf).is_null() {
        gsl_vector_free((*state).qtf);
    }
    if !((*state).diag).is_null() {
        gsl_vector_free((*state).diag);
    }
    if !((*state).tau).is_null() {
        gsl_vector_free((*state).tau);
    }
    if !((*state).r).is_null() {
        gsl_matrix_free((*state).r);
    }
    if !((*state).J).is_null() {
        gsl_matrix_free((*state).J);
    }
}
static mut lmder_type: gsl_multifit_fdfsolver_type = {
    let mut init = gsl_multifit_fdfsolver_type {
        name: b"lmder\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<lmder_state_t>() as libc::c_ulong,
        alloc: Some(
            lmder_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> libc::c_int,
        ),
        set: Some(
            lmder_set
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
            lmder_iterate
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
            lmder_gradient
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        jac: Some(
            lmder_jac
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_matrix,
                ) -> libc::c_int,
        ),
        free: Some(lmder_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
static mut lmsder_type: gsl_multifit_fdfsolver_type = {
    let mut init = gsl_multifit_fdfsolver_type {
        name: b"lmsder\0" as *const u8 as *const libc::c_char,
        size: ::std::mem::size_of::<lmder_state_t>() as libc::c_ulong,
        alloc: Some(
            lmder_alloc
                as unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> libc::c_int,
        ),
        set: Some(
            lmsder_set
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
            lmsder_iterate
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
            lmder_gradient
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_vector,
                ) -> libc::c_int,
        ),
        jac: Some(
            lmder_jac
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut gsl_matrix,
                ) -> libc::c_int,
        ),
        free: Some(lmder_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_multifit_fdfsolver_lmder: *const gsl_multifit_fdfsolver_type = unsafe {
    &lmder_type as *const gsl_multifit_fdfsolver_type
};
pub static mut gsl_multifit_fdfsolver_lmsder: *const gsl_multifit_fdfsolver_type = unsafe {
    &lmsder_type as *const gsl_multifit_fdfsolver_type
};
