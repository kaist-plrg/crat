use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_permutation_free(p: *mut gsl_permutation);
    fn gsl_permutation_alloc(n: size_t) -> *mut gsl_permutation;
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_linalg_QRPT_decomp(
        A: *mut gsl_matrix,
        tau: *mut gsl_vector,
        p: *mut gsl_permutation,
        signum: *mut libc::c_int,
        norm: *mut gsl_vector,
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
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
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
pub unsafe extern "C" fn gsl_multifit_covar(
    mut J: *const gsl_matrix,
    epsrel: libc::c_double,
    mut covar: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut r: *mut gsl_matrix = 0 as *mut gsl_matrix;
    let mut tau: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut norm: *mut gsl_vector = 0 as *mut gsl_vector;
    let mut perm: *mut gsl_permutation = 0 as *mut gsl_permutation;
    let m: size_t = (*J).size1;
    let n: size_t = (*J).size2;
    if m < n {
        gsl_error(
            b"Jacobian be rectangular M x N with M >= N\0" as *const u8
                as *const libc::c_char,
            b"covar.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if (*covar).size1 != (*covar).size2 || (*covar).size1 != n {
        gsl_error(
            b"covariance matrix must be square and match second dimension of jacobian\0"
                as *const u8 as *const libc::c_char,
            b"covar.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    r = gsl_matrix_alloc(m, n);
    tau = gsl_vector_alloc(n);
    perm = gsl_permutation_alloc(n);
    norm = gsl_vector_alloc(n);
    let mut signum: libc::c_int = 0 as libc::c_int;
    gsl_matrix_memcpy(r, J);
    gsl_linalg_QRPT_decomp(r, tau, perm, &mut signum, norm);
    status = gsl_multifit_covar_QRPT(r, perm, epsrel, covar);
    gsl_matrix_free(r);
    gsl_permutation_free(perm);
    gsl_vector_free(tau);
    gsl_vector_free(norm);
    return status;
}
pub unsafe extern "C" fn gsl_multifit_covar_QRPT(
    mut r: *mut gsl_matrix,
    mut perm: *mut gsl_permutation,
    epsrel: libc::c_double,
    mut covar: *mut gsl_matrix,
) -> libc::c_int {
    let mut tolr: libc::c_double = epsrel
        * fabs(
            gsl_matrix_get(r, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
        );
    let n: size_t = (*r).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut kmax: size_t = 0 as libc::c_int as size_t;
    k = 0 as libc::c_int as size_t;
    while k < n {
        let mut rkk: libc::c_double = gsl_matrix_get(r, k, k);
        if fabs(rkk) <= tolr {
            break;
        }
        gsl_matrix_set(r, k, k, 1.0f64 / rkk);
        j = 0 as libc::c_int as size_t;
        while j < k {
            let mut t: libc::c_double = gsl_matrix_get(r, j, k) / rkk;
            gsl_matrix_set(r, j, k, 0.0f64);
            i = 0 as libc::c_int as size_t;
            while i <= j {
                let mut rik: libc::c_double = gsl_matrix_get(r, i, k);
                let mut rij: libc::c_double = gsl_matrix_get(r, i, j);
                gsl_matrix_set(r, i, k, rik - t * rij);
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        kmax = k;
        k = k.wrapping_add(1);
        k;
    }
    k = 0 as libc::c_int as size_t;
    while k <= kmax {
        j = 0 as libc::c_int as size_t;
        while j < k {
            let mut rjk: libc::c_double = gsl_matrix_get(r, j, k);
            i = 0 as libc::c_int as size_t;
            while i <= j {
                let mut rij_0: libc::c_double = gsl_matrix_get(r, i, j);
                let mut rik_0: libc::c_double = gsl_matrix_get(r, i, k);
                gsl_matrix_set(r, i, j, rij_0 + rjk * rik_0);
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        let mut t_0: libc::c_double = gsl_matrix_get(r, k, k);
        i = 0 as libc::c_int as size_t;
        while i <= k {
            let mut rik_1: libc::c_double = gsl_matrix_get(r, i, k);
            gsl_matrix_set(r, i, k, t_0 * rik_1);
            i = i.wrapping_add(1);
            i;
        }
        k = k.wrapping_add(1);
        k;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        let mut pj: size_t = gsl_permutation_get(perm, j);
        i = 0 as libc::c_int as size_t;
        while i <= j {
            let mut pi: size_t = gsl_permutation_get(perm, i);
            let mut rij_1: libc::c_double = 0.;
            if j > kmax {
                gsl_matrix_set(r, i, j, 0.0f64);
                rij_1 = 0.0f64;
            } else {
                rij_1 = gsl_matrix_get(r, i, j);
            }
            if pi > pj {
                gsl_matrix_set(r, pi, pj, rij_1);
            } else if pi < pj {
                gsl_matrix_set(r, pj, pi, rij_1);
            }
            i = i.wrapping_add(1);
            i;
        }
        let mut rjj: libc::c_double = gsl_matrix_get(r, j, j);
        gsl_matrix_set(covar, pj, pj, rjj);
        j = j.wrapping_add(1);
        j;
    }
    j = 0 as libc::c_int as size_t;
    while j < n {
        i = 0 as libc::c_int as size_t;
        while i < j {
            let mut rji: libc::c_double = gsl_matrix_get(r, j, i);
            gsl_matrix_set(covar, j, i, rji);
            gsl_matrix_set(covar, i, j, rji);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
