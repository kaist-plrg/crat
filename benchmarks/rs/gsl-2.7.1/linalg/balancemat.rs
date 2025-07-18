use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_set_all(v: *mut gsl_vector, x: libc::c_double);
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
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
pub unsafe extern "C" fn gsl_linalg_balance_matrix(
    mut A: *mut gsl_matrix,
    mut D: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*D).size {
        gsl_error(
            b"vector must match matrix size\0" as *const u8 as *const libc::c_char,
            b"balancemat.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut row_norm: libc::c_double = 0.;
        let mut col_norm: libc::c_double = 0.;
        let mut not_converged: libc::c_int = 0;
        let mut v: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        gsl_vector_set_all(D, 1.0f64);
        not_converged = 1 as libc::c_int;
        while not_converged != 0 {
            let mut i: size_t = 0;
            let mut j: size_t = 0;
            let mut g: libc::c_double = 0.;
            let mut f: libc::c_double = 0.;
            let mut s: libc::c_double = 0.;
            not_converged = 0 as libc::c_int;
            i = 0 as libc::c_int as size_t;
            while i < N {
                row_norm = 0.0f64;
                col_norm = 0.0f64;
                j = 0 as libc::c_int as size_t;
                while j < N {
                    if j != i {
                        col_norm += fabs(gsl_matrix_get(A, j, i));
                        row_norm += fabs(gsl_matrix_get(A, i, j));
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                if !(col_norm == 0.0f64 || row_norm == 0.0f64) {
                    g = row_norm / 2.0f64;
                    f = 1.0f64;
                    s = col_norm + row_norm;
                    while col_norm < g {
                        f *= 2.0f64;
                        col_norm *= 2.0f64 * 2.0f64;
                    }
                    g = row_norm * 2.0f64;
                    while col_norm > g {
                        f /= 2.0f64;
                        col_norm /= 2.0f64 * 2.0f64;
                    }
                    if row_norm + col_norm < 0.95f64 * s * f {
                        not_converged = 1 as libc::c_int;
                        g = 1.0f64 / f;
                        v = gsl_matrix_row(A, i);
                        gsl_blas_dscal(g, &mut v.vector);
                        v = gsl_matrix_column(A, i);
                        gsl_blas_dscal(f, &mut v.vector);
                        gsl_vector_set(D, i, gsl_vector_get(D, i) * f);
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_balance_accum(
    mut A: *mut gsl_matrix,
    mut D: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*D).size {
        gsl_error(
            b"vector must match matrix size\0" as *const u8 as *const libc::c_char,
            b"balancemat.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut s: libc::c_double = 0.;
        let mut r: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        i = 0 as libc::c_int as size_t;
        while i < N {
            s = gsl_vector_get(D, i);
            r = gsl_matrix_row(A, i);
            gsl_blas_dscal(s, &mut r.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
