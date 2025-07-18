use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
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
pub unsafe extern "C" fn gsl_linalg_HH_solve(
    mut A: *mut gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 > (*A).size2 {
        gsl_error(
            b"System is underdetermined\0" as *const u8 as *const libc::c_char,
            b"hh.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*A).size2 != (*x).size {
        gsl_error(
            b"matrix and vector sizes must be equal\0" as *const u8
                as *const libc::c_char,
            b"hh.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_HH_svx(A, x);
        return status;
    };
}
pub unsafe extern "C" fn gsl_linalg_HH_svx(
    mut A: *mut gsl_matrix,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 > (*A).size2 {
        gsl_error(
            b"System is underdetermined\0" as *const u8 as *const libc::c_char,
            b"hh.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*A).size2 != (*x).size {
        gsl_error(
            b"matrix and vector sizes must be equal\0" as *const u8
                as *const libc::c_char,
            b"hh.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*A).size1;
        let M: size_t = (*A).size2;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        let mut d: *mut libc::c_double = malloc(
            N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        if d.is_null() {
            gsl_error(
                b"could not allocate memory for workspace\0" as *const u8
                    as *const libc::c_char,
                b"hh.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        i = 0 as libc::c_int as size_t;
        while i < N {
            let aii: libc::c_double = gsl_matrix_get(A, i, i);
            let mut alpha: libc::c_double = 0.;
            let mut f: libc::c_double = 0.;
            let mut ak: libc::c_double = 0.;
            let mut max_norm: libc::c_double = 0.0f64;
            let mut r: libc::c_double = 0.0f64;
            k = i;
            while k < M {
                let mut aki: libc::c_double = gsl_matrix_get(A, k, i);
                r += aki * aki;
                k = k.wrapping_add(1);
                k;
            }
            if r == 0.0f64 {
                free(d as *mut libc::c_void);
                gsl_error(
                    b"matrix is rank deficient\0" as *const u8 as *const libc::c_char,
                    b"hh.c\0" as *const u8 as *const libc::c_char,
                    105 as libc::c_int,
                    GSL_ESING as libc::c_int,
                );
                return GSL_ESING as libc::c_int;
            }
            alpha = sqrt(r)
                * (if aii >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                    as libc::c_double;
            ak = 1.0f64 / (r + alpha * aii);
            gsl_matrix_set(A, i, i, aii + alpha);
            *d.offset(i as isize) = -alpha;
            k = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < N {
                let mut norm: libc::c_double = 0.0f64;
                f = 0.0f64;
                j = i;
                while j < M {
                    let mut ajk: libc::c_double = gsl_matrix_get(A, j, k);
                    let mut aji: libc::c_double = gsl_matrix_get(A, j, i);
                    norm += ajk * ajk;
                    f += ajk * aji;
                    j = j.wrapping_add(1);
                    j;
                }
                max_norm = if max_norm > norm { max_norm } else { norm };
                f *= ak;
                j = i;
                while j < M {
                    let mut ajk_0: libc::c_double = gsl_matrix_get(A, j, k);
                    let mut aji_0: libc::c_double = gsl_matrix_get(A, j, i);
                    gsl_matrix_set(A, j, k, ajk_0 - f * aji_0);
                    j = j.wrapping_add(1);
                    j;
                }
                k = k.wrapping_add(1);
                k;
            }
            if fabs(alpha) < 2.0f64 * 2.2204460492503131e-16f64 * sqrt(max_norm) {
                free(d as *mut libc::c_void);
                gsl_error(
                    b"apparent singularity detected\0" as *const u8
                        as *const libc::c_char,
                    b"hh.c\0" as *const u8 as *const libc::c_char,
                    142 as libc::c_int,
                    GSL_ESING as libc::c_int,
                );
                return GSL_ESING as libc::c_int;
            }
            f = 0.0f64;
            j = i;
            while j < M {
                f += gsl_vector_get(x, j) * gsl_matrix_get(A, j, i);
                j = j.wrapping_add(1);
                j;
            }
            f *= ak;
            j = i;
            while j < M {
                let mut xj: libc::c_double = gsl_vector_get(x, j);
                let mut aji_1: libc::c_double = gsl_matrix_get(A, j, i);
                gsl_vector_set(x, j, xj - f * aji_1);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        i = N;
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let mut xi: libc::c_double = gsl_vector_get(x, i);
            let mut sum: libc::c_double = 0.0f64;
            k = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < N {
                sum += gsl_matrix_get(A, i, k) * gsl_vector_get(x, k);
                k = k.wrapping_add(1);
                k;
            }
            gsl_vector_set(x, i, (xi - sum) / *d.offset(i as isize));
        }
        free(d as *mut libc::c_void);
        return GSL_SUCCESS as libc::c_int;
    };
}
