use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
    fn gsl_linalg_householder_hm(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_mh(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
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
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_view = _gsl_matrix_view;
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
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_linalg_hessenberg_decomp(
    mut A: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Hessenberg reduction requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"hessenberg.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*tau).size {
        gsl_error(
            b"tau vector must match matrix size\0" as *const u8 as *const libc::c_char,
            b"hessenberg.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N < 3 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int
    } else {
        let mut i: size_t = 0;
        let mut c: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut hv: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut m: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut tau_i: libc::c_double = 0.;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            c = gsl_matrix_subcolumn(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            hv = gsl_vector_subvector(
                tau,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            gsl_vector_memcpy(&mut hv.vector, &mut c.vector);
            tau_i = gsl_linalg_householder_transform(&mut hv.vector);
            m = gsl_matrix_submatrix(
                A,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                i,
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                N.wrapping_sub(i),
            );
            gsl_linalg_householder_hm(tau_i, &mut hv.vector, &mut m.matrix);
            m = gsl_matrix_submatrix(
                A,
                0 as libc::c_int as size_t,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N,
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            gsl_linalg_householder_mh(tau_i, &mut hv.vector, &mut m.matrix);
            gsl_vector_set(tau, i, tau_i);
            c = gsl_vector_subvector(
                &mut c.vector,
                1 as libc::c_int as size_t,
                (c.vector.size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            hv = gsl_vector_subvector(
                &mut hv.vector,
                1 as libc::c_int as size_t,
                (hv.vector.size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_memcpy(&mut c.vector, &mut hv.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_hessenberg_unpack(
    mut H: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
    mut U: *mut gsl_matrix,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    gsl_matrix_set_identity(U);
    s = gsl_linalg_hessenberg_unpack_accum(H, tau, U);
    return s;
}
pub unsafe extern "C" fn gsl_linalg_hessenberg_unpack_accum(
    mut H: *mut gsl_matrix,
    mut tau: *mut gsl_vector,
    mut V: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*H).size1;
    if N != (*H).size2 {
        gsl_error(
            b"Hessenberg reduction requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"hessenberg.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*tau).size {
        gsl_error(
            b"tau vector must match matrix size\0" as *const u8 as *const libc::c_char,
            b"hessenberg.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*V).size2 {
        gsl_error(
            b"V matrix has wrong dimension\0" as *const u8 as *const libc::c_char,
            b"hessenberg.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut j: size_t = 0;
        let mut tau_j: libc::c_double = 0.;
        let mut c: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut hv: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut m: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        if N < 3 as libc::c_int as libc::c_ulong {
            return GSL_SUCCESS as libc::c_int;
        }
        j = 0 as libc::c_int as size_t;
        while j < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            c = gsl_matrix_column(H, j);
            tau_j = gsl_vector_get(tau, j);
            hv = gsl_vector_subvector(
                &mut c.vector,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(j.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            m = gsl_matrix_submatrix(
                V,
                0 as libc::c_int as size_t,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                (*V).size1,
                N.wrapping_sub(j.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            gsl_linalg_householder_mh(tau_j, &mut hv.vector, &mut m.matrix);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_hessenberg_set_zero(
    mut H: *mut gsl_matrix,
) -> libc::c_int {
    let N: size_t = (*H).size1;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if N < 3 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
        i = j.wrapping_add(2 as libc::c_int as libc::c_ulong);
        while i < N {
            gsl_matrix_set(H, i, j, 0.0f64);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_hessenberg_submatrix(
    mut M: *mut gsl_matrix,
    mut A: *mut gsl_matrix,
    mut top: size_t,
    mut tau: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let N_M: size_t = (*M).size1;
    if N != (*A).size2 {
        gsl_error(
            b"Hessenberg reduction requires square matrix\0" as *const u8
                as *const libc::c_char,
            b"hessenberg.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*tau).size {
        gsl_error(
            b"tau vector must match matrix size\0" as *const u8 as *const libc::c_char,
            b"hessenberg.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N < 3 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int
    } else {
        let mut i: size_t = 0;
        let mut c: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut hv: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut m: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut tau_i: libc::c_double = 0.;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            c = gsl_matrix_subcolumn(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            hv = gsl_vector_subvector(
                tau,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            gsl_vector_memcpy(&mut hv.vector, &mut c.vector);
            tau_i = gsl_linalg_householder_transform(&mut hv.vector);
            m = gsl_matrix_submatrix(
                M,
                top.wrapping_add(i).wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(i),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                N_M.wrapping_sub(top).wrapping_sub(i),
            );
            gsl_linalg_householder_hm(tau_i, &mut hv.vector, &mut m.matrix);
            m = gsl_matrix_submatrix(
                M,
                0 as libc::c_int as size_t,
                top.wrapping_add(i).wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(N),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            gsl_linalg_householder_mh(tau_i, &mut hv.vector, &mut m.matrix);
            gsl_vector_set(tau, i, tau_i);
            c = gsl_vector_subvector(
                &mut c.vector,
                1 as libc::c_int as size_t,
                (c.vector.size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            hv = gsl_vector_subvector(
                &mut hv.vector,
                1 as libc::c_int as size_t,
                (hv.vector.size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_memcpy(&mut c.vector, &mut hv.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
