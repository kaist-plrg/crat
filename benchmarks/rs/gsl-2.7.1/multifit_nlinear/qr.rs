use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
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
    fn gsl_linalg_QRPT_lssolve2(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        p: *const gsl_permutation,
        b: *const gsl_vector,
        rank: size_t,
        x: *mut gsl_vector,
        residual: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_QRPT_rank(QR: *const gsl_matrix, tol: libc::c_double) -> size_t;
    fn gsl_linalg_QRPT_rcond(
        QR: *const gsl_matrix,
        rcond: *mut libc::c_double,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_permutation_calloc(n: size_t) -> *mut gsl_permutation;
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
pub type gsl_multifit_nlinear_fdtype = libc::c_uint;
pub const GSL_MULTIFIT_NLINEAR_CTRDIFF: gsl_multifit_nlinear_fdtype = 1;
pub const GSL_MULTIFIT_NLINEAR_FWDIFF: gsl_multifit_nlinear_fdtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_fdf {
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
    pub fvv: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *const gsl_vector,
            *mut libc::c_void,
            *mut gsl_vector,
        ) -> libc::c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut libc::c_void,
    pub nevalf: size_t,
    pub nevaldf: size_t,
    pub nevalfvv: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trs {
    pub name: *const libc::c_char,
    pub alloc: Option::<
        unsafe extern "C" fn(*const libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub preloop: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub step: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            libc::c_double,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub preduction: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const gsl_vector,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_scale {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
    pub update: Option::<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_solver {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub init: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub presolve: Option::<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub solve: Option::<
        unsafe extern "C" fn(
            *const gsl_vector,
            *mut gsl_vector,
            *const libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub rcond: Option::<
        unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_parameters {
    pub trs: *const gsl_multifit_nlinear_trs,
    pub scale: *const gsl_multifit_nlinear_scale,
    pub solver: *const gsl_multifit_nlinear_solver,
    pub fdtype: gsl_multifit_nlinear_fdtype,
    pub factor_up: libc::c_double,
    pub factor_down: libc::c_double,
    pub avmax: libc::c_double,
    pub h_df: libc::c_double,
    pub h_fvv: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multifit_nlinear_trust_state {
    pub x: *const gsl_vector,
    pub f: *const gsl_vector,
    pub g: *const gsl_vector,
    pub J: *const gsl_matrix,
    pub diag: *const gsl_vector,
    pub sqrt_wts: *const gsl_vector,
    pub mu: *const libc::c_double,
    pub params: *const gsl_multifit_nlinear_parameters,
    pub solver_state: *mut libc::c_void,
    pub fdf: *mut gsl_multifit_nlinear_fdf,
    pub avratio: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qr_state_t {
    pub p: size_t,
    pub QR: *mut gsl_matrix,
    pub tau_Q: *mut gsl_vector,
    pub T: *mut gsl_matrix,
    pub perm: *mut gsl_permutation,
    pub rank: size_t,
    pub residual: *mut gsl_vector,
    pub qtf: *mut gsl_vector,
    pub workn: *mut gsl_vector,
    pub workp: *mut gsl_vector,
    pub work3p: *mut gsl_vector,
    pub mu: libc::c_double,
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
#[inline]
unsafe extern "C" fn gsl_permutation_get(
    mut p: *const gsl_permutation,
    i: size_t,
) -> size_t {
    return *((*p).data).offset(i as isize);
}
unsafe extern "C" fn qrsolv(
    mut r: *mut gsl_matrix,
    mut p: *const gsl_permutation,
    lambda: libc::c_double,
    mut diag: *const gsl_vector,
    mut qtb: *const gsl_vector,
    mut S: *mut gsl_matrix,
    mut x: *mut gsl_vector,
    mut work: *mut gsl_vector,
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
            gsl_matrix_set(S, i, j, rji);
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(x, j, rjj);
        gsl_vector_set(work, j, qtbj);
        j = j.wrapping_add(1);
        j;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut qtbpj: libc::c_double = 0.;
        let mut pj: size_t = gsl_permutation_get(p, j);
        let mut diagpj: libc::c_double = lambda * gsl_vector_get(diag, pj);
        if !(diagpj == 0 as libc::c_int as libc::c_double) {
            gsl_matrix_set(S, j, j, diagpj);
            k = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < n {
                gsl_matrix_set(S, k, k, 0.0f64);
                k = k.wrapping_add(1);
                k;
            }
            qtbpj = 0 as libc::c_int as libc::c_double;
            k = j;
            while k < n {
                let mut sine: libc::c_double = 0.;
                let mut cosine: libc::c_double = 0.;
                let mut wk: libc::c_double = gsl_vector_get(work, k);
                let mut rkk: libc::c_double = gsl_matrix_get(r, k, k);
                let mut skk: libc::c_double = gsl_matrix_get(S, k, k);
                if !(skk == 0 as libc::c_int as libc::c_double) {
                    if fabs(rkk) < fabs(skk) {
                        let mut cotangent: libc::c_double = rkk / skk;
                        sine = 0.5f64 / sqrt(0.25f64 + 0.25f64 * cotangent * cotangent);
                        cosine = sine * cotangent;
                    } else {
                        let mut tangent: libc::c_double = skk / rkk;
                        cosine = 0.5f64 / sqrt(0.25f64 + 0.25f64 * tangent * tangent);
                        sine = cosine * tangent;
                    }
                    let mut new_rkk: libc::c_double = cosine * rkk + sine * skk;
                    let mut new_wk: libc::c_double = cosine * wk + sine * qtbpj;
                    qtbpj = -sine * wk + cosine * qtbpj;
                    gsl_matrix_set(r, k, k, new_rkk);
                    gsl_matrix_set(S, k, k, new_rkk);
                    gsl_vector_set(work, k, new_wk);
                    i = k.wrapping_add(1 as libc::c_int as libc::c_ulong);
                    while i < n {
                        let mut sik: libc::c_double = gsl_matrix_get(S, i, k);
                        let mut sii: libc::c_double = gsl_matrix_get(S, i, i);
                        let mut new_sik: libc::c_double = cosine * sik + sine * sii;
                        let mut new_sii: libc::c_double = -sine * sik + cosine * sii;
                        gsl_matrix_set(S, i, k, new_sik);
                        gsl_matrix_set(S, i, i, new_sii);
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                k = k.wrapping_add(1);
                k;
            }
            let mut xj: libc::c_double = gsl_vector_get(x, j);
            gsl_matrix_set(r, j, j, xj);
        }
        j = j.wrapping_add(1);
        j;
    }
    nsing = n;
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut sjj: libc::c_double = gsl_matrix_get(S, j, j);
        if sjj == 0 as libc::c_int as libc::c_double {
            nsing = j;
            break;
        } else {
            j = j.wrapping_add(1);
            j;
        }
    }
    j = nsing;
    while j < n {
        gsl_vector_set(work, j, 0.0f64);
        j = j.wrapping_add(1);
        j;
    }
    k = 0 as libc::c_int as size_t;
    while k < nsing {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = nsing.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_sub(k);
        i = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while i < nsing {
            sum += gsl_matrix_get(S, i, j) * gsl_vector_get(work, i);
            i = i.wrapping_add(1);
            i;
        }
        let mut wj: libc::c_double = gsl_vector_get(work, j);
        let mut sjj_0: libc::c_double = gsl_matrix_get(S, j, j);
        gsl_vector_set(work, j, (wj - sum) / sjj_0);
        k = k.wrapping_add(1);
        k;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut pj_0: size_t = gsl_permutation_get(p, j);
        let mut wj_0: libc::c_double = gsl_vector_get(work, j);
        gsl_vector_set(x, pj_0, wj_0);
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn qr_alloc(n: size_t, p: size_t) -> *mut libc::c_void {
    let mut state: *mut qr_state_t = 0 as *mut qr_state_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<qr_state_t>() as libc::c_ulong,
    ) as *mut qr_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate qr state\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).QR = gsl_matrix_alloc(n, p);
    if ((*state).QR).is_null() {
        gsl_error(
            b"failed to allocate space for QR\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).tau_Q = gsl_vector_alloc(p);
    if ((*state).tau_Q).is_null() {
        gsl_error(
            b"failed to allocate space for tau_Q\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).T = gsl_matrix_alloc(p, p);
    if ((*state).T).is_null() {
        gsl_error(
            b"failed to allocate space for T\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).qtf = gsl_vector_alloc(n);
    if ((*state).qtf).is_null() {
        gsl_error(
            b"failed to allocate space for qtf\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).residual = gsl_vector_alloc(n);
    if ((*state).residual).is_null() {
        gsl_error(
            b"failed to allocate space for residual\0" as *const u8
                as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).perm = gsl_permutation_calloc(p);
    if ((*state).perm).is_null() {
        gsl_error(
            b"failed to allocate space for perm\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workn = gsl_vector_alloc(n);
    if ((*state).workn).is_null() {
        gsl_error(
            b"failed to allocate space for workn\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).workp = gsl_vector_alloc(p);
    if ((*state).workp).is_null() {
        gsl_error(
            b"failed to allocate space for workp\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .work3p = gsl_vector_alloc((3 as libc::c_int as libc::c_ulong).wrapping_mul(p));
    if ((*state).work3p).is_null() {
        gsl_error(
            b"failed to allocate space for work3p\0" as *const u8 as *const libc::c_char,
            b"qr.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).p = p;
    (*state).mu = 0.0f64;
    (*state).rank = 0 as libc::c_int as size_t;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn qr_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut qr_state_t = vstate as *mut qr_state_t;
    if !((*state).QR).is_null() {
        gsl_matrix_free((*state).QR);
    }
    if !((*state).tau_Q).is_null() {
        gsl_vector_free((*state).tau_Q);
    }
    if !((*state).T).is_null() {
        gsl_matrix_free((*state).T);
    }
    if !((*state).qtf).is_null() {
        gsl_vector_free((*state).qtf);
    }
    if !((*state).residual).is_null() {
        gsl_vector_free((*state).residual);
    }
    if !((*state).perm).is_null() {
        gsl_permutation_free((*state).perm);
    }
    if !((*state).workn).is_null() {
        gsl_vector_free((*state).workn);
    }
    if !((*state).workp).is_null() {
        gsl_vector_free((*state).workp);
    }
    if !((*state).work3p).is_null() {
        gsl_vector_free((*state).work3p);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn qr_init(
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
        as *const gsl_multifit_nlinear_trust_state;
    let mut state: *mut qr_state_t = vstate as *mut qr_state_t;
    let mut signum: libc::c_int = 0;
    gsl_matrix_memcpy((*state).QR, (*trust_state).J);
    gsl_linalg_QRPT_decomp(
        (*state).QR,
        (*state).tau_Q,
        (*state).perm,
        &mut signum,
        (*state).workp,
    );
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn qr_presolve(
    mu: libc::c_double,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut qr_state_t = vstate as *mut qr_state_t;
    (*state).mu = mu;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn qr_solve(
    mut f: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut vtrust_state: *const libc::c_void,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut qr_state_t = vstate as *mut qr_state_t;
    let mut status: libc::c_int = 0;
    if (*state).mu == 0.0f64 {
        let mut rank: size_t = gsl_linalg_QRPT_rank((*state).QR, -1.0f64);
        status = gsl_linalg_QRPT_lssolve2(
            (*state).QR,
            (*state).tau_Q,
            (*state).perm,
            f,
            rank,
            x,
            (*state).residual,
        );
    } else {
        let mut trust_state: *const gsl_multifit_nlinear_trust_state = vtrust_state
            as *const gsl_multifit_nlinear_trust_state;
        let mut sqrt_mu: libc::c_double = sqrt((*state).mu);
        gsl_vector_memcpy((*state).qtf, f);
        gsl_linalg_QR_QTvec((*state).QR, (*state).tau_Q, (*state).qtf);
        status = qrsolv(
            (*state).QR,
            (*state).perm,
            sqrt_mu,
            (*trust_state).diag,
            (*state).qtf,
            (*state).T,
            x,
            (*state).workn,
        );
    }
    gsl_vector_scale(x, -1.0f64);
    return status;
}
unsafe extern "C" fn qr_rcond(
    mut rcond: *mut libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut state: *mut qr_state_t = vstate as *mut qr_state_t;
    status = gsl_linalg_QRPT_rcond((*state).QR, rcond, (*state).work3p);
    return status;
}
static mut qr_type: gsl_multifit_nlinear_solver = unsafe {
    {
        let mut init = gsl_multifit_nlinear_solver {
            name: b"qr\0" as *const u8 as *const libc::c_char,
            alloc: Some(
                qr_alloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
            ),
            init: Some(
                qr_init
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            presolve: Some(
                qr_presolve
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            solve: Some(
                qr_solve
                    as unsafe extern "C" fn(
                        *const gsl_vector,
                        *mut gsl_vector,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            rcond: Some(
                qr_rcond
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            free: Some(qr_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub static mut gsl_multifit_nlinear_solver_qr: *const gsl_multifit_nlinear_solver = unsafe {
    &qr_type as *const gsl_multifit_nlinear_solver
};
