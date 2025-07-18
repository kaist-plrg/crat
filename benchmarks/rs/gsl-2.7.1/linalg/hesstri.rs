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
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subrow(
        m: *mut gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_blas_drot(
        X: *mut gsl_vector,
        Y: *mut gsl_vector,
        c: libc::c_double,
        s: libc::c_double,
    ) -> libc::c_int;
    fn gsl_linalg_QR_unpack(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        Q: *mut gsl_matrix,
        R: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_QTmat(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_QR_decomp(A: *mut gsl_matrix, tau: *mut gsl_vector) -> libc::c_int;
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
unsafe extern "C" fn gsl_linalg_givens(
    a: libc::c_double,
    b: libc::c_double,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) {
    if b == 0 as libc::c_int as libc::c_double {
        *c = 1 as libc::c_int as libc::c_double;
        *s = 0 as libc::c_int as libc::c_double;
    } else if fabs(b) > fabs(a) {
        let mut t: libc::c_double = -a / b;
        let mut s1: libc::c_double = 1.0f64
            / sqrt(1 as libc::c_int as libc::c_double + t * t);
        *s = s1;
        *c = s1 * t;
    } else {
        let mut t_0: libc::c_double = -b / a;
        let mut c1: libc::c_double = 1.0f64
            / sqrt(1 as libc::c_int as libc::c_double + t_0 * t_0);
        *c = c1;
        *s = c1 * t_0;
    };
}
pub unsafe extern "C" fn gsl_linalg_hesstri_decomp(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut U: *mut gsl_matrix,
    mut V: *mut gsl_matrix,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 || N != (*B).size1 || N != (*B).size2 {
        gsl_error(
            b"Hessenberg-triangular reduction requires square matrices\0" as *const u8
                as *const libc::c_char,
            b"hesstri.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*work).size {
        gsl_error(
            b"length of workspace must match matrix dimension\0" as *const u8
                as *const libc::c_char,
            b"hesstri.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut cs: libc::c_double = 0.;
        let mut sn: libc::c_double = 0.;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut xv: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut yv: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        gsl_linalg_QR_decomp(B, work);
        gsl_linalg_QR_QTmat(B, work, A);
        if !U.is_null() {
            gsl_linalg_QR_unpack(B, work, U, B);
        } else {
            j = 0 as libc::c_int as size_t;
            while j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                i = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
                while i < N {
                    gsl_matrix_set(B, i, j, 0.0f64);
                    i = i.wrapping_add(1);
                    i;
                }
                j = j.wrapping_add(1);
                j;
            }
        }
        if !V.is_null() {
            gsl_matrix_set_identity(V);
        }
        if N < 3 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        j = 0 as libc::c_int as size_t;
        while j < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            i = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            while i >= j.wrapping_add(2 as libc::c_int as libc::c_ulong) {
                gsl_linalg_givens(
                    gsl_matrix_get(
                        A,
                        i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        j,
                    ),
                    gsl_matrix_get(A, i, j),
                    &mut cs,
                    &mut sn,
                );
                sn = -sn;
                xv = gsl_matrix_subrow(
                    A,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    j,
                    N.wrapping_sub(j),
                );
                yv = gsl_matrix_subrow(A, i, j, N.wrapping_sub(j));
                gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
                xv = gsl_matrix_subrow(
                    B,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(i).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                yv = gsl_matrix_subrow(
                    B,
                    i,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(i).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
                if !U.is_null() {
                    xv = gsl_matrix_column(
                        U,
                        i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    yv = gsl_matrix_column(U, i);
                    gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
                }
                gsl_linalg_givens(
                    -gsl_matrix_get(B, i, i),
                    gsl_matrix_get(
                        B,
                        i,
                        i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
                    &mut cs,
                    &mut sn,
                );
                sn = -sn;
                xv = gsl_matrix_subcolumn(
                    B,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    0 as libc::c_int as size_t,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                yv = gsl_matrix_subcolumn(
                    B,
                    i,
                    0 as libc::c_int as size_t,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
                xv = gsl_matrix_column(
                    A,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                yv = gsl_matrix_column(A, i);
                gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
                if !V.is_null() {
                    xv = gsl_matrix_column(
                        V,
                        i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    yv = gsl_matrix_column(V, i);
                    gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
                }
                i = i.wrapping_sub(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
