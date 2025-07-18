use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
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
unsafe extern "C" fn symschur2(
    mut A: *mut gsl_matrix,
    mut p: size_t,
    mut q: size_t,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) -> libc::c_double {
    let mut Apq: libc::c_double = gsl_matrix_get(A, p, q);
    if Apq != 0.0f64 {
        let mut App: libc::c_double = gsl_matrix_get(A, p, p);
        let mut Aqq: libc::c_double = gsl_matrix_get(A, q, q);
        let mut tau: libc::c_double = (Aqq - App) / (2.0f64 * Apq);
        let mut t: libc::c_double = 0.;
        let mut c1: libc::c_double = 0.;
        if tau >= 0.0f64 {
            t = 1.0f64 / (tau + hypot(1.0f64, tau));
        } else {
            t = -1.0f64 / (-tau + hypot(1.0f64, tau));
        }
        c1 = 1.0f64 / hypot(1.0f64, t);
        *c = c1;
        *s = t * c1;
    } else {
        *c = 1.0f64;
        *s = 0.0f64;
    }
    return fabs(Apq);
}
#[inline]
unsafe extern "C" fn apply_jacobi_L(
    mut A: *mut gsl_matrix,
    mut p: size_t,
    mut q: size_t,
    mut c: libc::c_double,
    mut s: libc::c_double,
) {
    let mut j: size_t = 0;
    let N: size_t = (*A).size2;
    j = 0 as libc::c_int as size_t;
    while j < N {
        let mut Apj: libc::c_double = gsl_matrix_get(A, p, j);
        let mut Aqj: libc::c_double = gsl_matrix_get(A, q, j);
        gsl_matrix_set(A, p, j, Apj * c - Aqj * s);
        gsl_matrix_set(A, q, j, Apj * s + Aqj * c);
        j = j.wrapping_add(1);
        j;
    }
}
#[inline]
unsafe extern "C" fn apply_jacobi_R(
    mut A: *mut gsl_matrix,
    mut p: size_t,
    mut q: size_t,
    mut c: libc::c_double,
    mut s: libc::c_double,
) {
    let mut i: size_t = 0;
    let M: size_t = (*A).size1;
    i = 0 as libc::c_int as size_t;
    while i < M {
        let mut Aip: libc::c_double = gsl_matrix_get(A, i, p);
        let mut Aiq: libc::c_double = gsl_matrix_get(A, i, q);
        gsl_matrix_set(A, i, p, Aip * c - Aiq * s);
        gsl_matrix_set(A, i, q, Aip * s + Aiq * c);
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn norm(mut A: *mut gsl_matrix) -> libc::c_double {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut M: size_t = (*A).size1;
    let mut N: size_t = (*A).size2;
    let mut sum: libc::c_double = 0.0f64;
    let mut scale: libc::c_double = 0.0f64;
    let mut ssq: libc::c_double = 1.0f64;
    i = 0 as libc::c_int as size_t;
    while i < M {
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut Aij: libc::c_double = gsl_matrix_get(A, i, j);
            if !(i == j) {
                if Aij != 0.0f64 {
                    let mut ax: libc::c_double = fabs(Aij);
                    if scale < ax {
                        ssq = 1.0f64 + ssq * (scale / ax) * (scale / ax);
                        scale = ax;
                    } else {
                        ssq += ax / scale * (ax / scale);
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    sum = scale * sqrt(ssq);
    return sum;
}
pub unsafe extern "C" fn gsl_eigen_jacobi(
    mut a: *mut gsl_matrix,
    mut eval: *mut gsl_vector,
    mut evec: *mut gsl_matrix,
    mut max_rot: libc::c_uint,
    mut nrot: *mut libc::c_uint,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut p: size_t = 0;
    let mut q: size_t = 0;
    let M: size_t = (*a).size1;
    let N: size_t = (*a).size2;
    let mut red: libc::c_double = 0.;
    let mut redsum: libc::c_double = 0.0f64;
    if M != N {
        gsl_error(
            b"eigenproblem requires square matrix\0" as *const u8 as *const libc::c_char,
            b"jacobi.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if M != (*evec).size1 || M != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must match input matrix\0" as *const u8
                as *const libc::c_char,
            b"jacobi.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if M != (*eval).size {
        gsl_error(
            b"eigenvalue vector must match input matrix\0" as *const u8
                as *const libc::c_char,
            b"jacobi.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    gsl_vector_set_zero(eval);
    gsl_matrix_set_identity(evec);
    i = 0 as libc::c_int as size_t;
    while i < max_rot as libc::c_ulong {
        let mut nrm: libc::c_double = norm(a);
        if nrm == 0.0f64 {
            break;
        }
        p = 0 as libc::c_int as size_t;
        while p < N {
            q = p.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while q < N {
                let mut c: libc::c_double = 0.;
                let mut s: libc::c_double = 0.;
                red = symschur2(a, p, q, &mut c, &mut s);
                redsum += red;
                apply_jacobi_L(a, p, q, c, s);
                apply_jacobi_R(a, p, q, c, s);
                apply_jacobi_R(evec, p, q, c, s);
                q = q.wrapping_add(1);
                q;
            }
            p = p.wrapping_add(1);
            p;
        }
        i = i.wrapping_add(1);
        i;
    }
    *nrot = i as libc::c_uint;
    p = 0 as libc::c_int as size_t;
    while p < N {
        let mut ep: libc::c_double = gsl_matrix_get(a, p, p);
        gsl_vector_set(eval, p, ep);
        p = p.wrapping_add(1);
        p;
    }
    if i == max_rot as libc::c_ulong {
        return GSL_EMAXITER as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_eigen_invert_jacobi(
    mut a: *const gsl_matrix,
    mut ainv: *mut gsl_matrix,
    mut max_rot: libc::c_uint,
) -> libc::c_int {
    if (*a).size1 != (*a).size2 || (*ainv).size1 != (*ainv).size2 {
        gsl_error(
            b"jacobi method requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"jacobi.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*a).size1 != (*ainv).size2 {
        gsl_error(
            b"inverse matrix must match input matrix\0" as *const u8
                as *const libc::c_char,
            b"jacobi.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let n: size_t = (*a).size2;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut nrot: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut status: libc::c_int = 0;
    let mut eval: *mut gsl_vector = gsl_vector_alloc(n);
    let mut evec: *mut gsl_matrix = gsl_matrix_alloc(n, n);
    let mut tmp: *mut gsl_matrix = gsl_matrix_alloc(n, n);
    gsl_matrix_memcpy(tmp, a);
    status = gsl_eigen_jacobi(tmp, eval, evec, max_rot, &mut nrot);
    i = 0 as libc::c_int as size_t;
    while i < n {
        j = 0 as libc::c_int as size_t;
        while j < n {
            let mut ainv_ij: libc::c_double = 0.0f64;
            k = 0 as libc::c_int as size_t;
            while k < n {
                let mut f: libc::c_double = 1.0f64 / gsl_vector_get(eval, k);
                let mut vik: libc::c_double = gsl_matrix_get(evec, i, k);
                let mut vjk: libc::c_double = gsl_matrix_get(evec, j, k);
                ainv_ij += vik * vjk * f;
                k = k.wrapping_add(1);
                k;
            }
            gsl_matrix_set(ainv, i, j, ainv_ij);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    gsl_vector_free(eval);
    gsl_matrix_free(evec);
    gsl_matrix_free(tmp);
    if status != 0 { return status } else { return GSL_SUCCESS as libc::c_int };
}
