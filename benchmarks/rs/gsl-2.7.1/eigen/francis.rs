use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
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
    fn gsl_blas_drot(
        X: *mut gsl_vector,
        Y: *mut gsl_vector,
        c: libc::c_double,
        s: libc::c_double,
    ) -> libc::c_int;
    fn gsl_hypot(x: libc::c_double, y: libc::c_double) -> libc::c_double;
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
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
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
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
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
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_view = _gsl_matrix_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_francis_workspace {
    pub size: size_t,
    pub max_iterations: size_t,
    pub n_iter: size_t,
    pub n_evals: size_t,
    pub compute_t: libc::c_int,
    pub H: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_set(
    mut v: *mut gsl_vector_complex,
    i: size_t,
    mut z: gsl_complex,
) {
    *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex) = z;
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
pub unsafe extern "C" fn gsl_eigen_francis_alloc() -> *mut gsl_eigen_francis_workspace {
    let mut w: *mut gsl_eigen_francis_workspace = 0 as *mut gsl_eigen_francis_workspace;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_eigen_francis_workspace>() as libc::c_ulong,
    ) as *mut gsl_eigen_francis_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"francis.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_francis_workspace;
    }
    (*w).size = 0 as libc::c_int as size_t;
    (*w).max_iterations = 0 as libc::c_int as size_t;
    (*w).n_iter = 0 as libc::c_int as size_t;
    (*w).n_evals = 0 as libc::c_int as size_t;
    (*w).compute_t = 0 as libc::c_int;
    (*w).Z = 0 as *mut gsl_matrix;
    (*w).H = 0 as *mut gsl_matrix;
    return w;
}
pub unsafe extern "C" fn gsl_eigen_francis_free(
    mut w: *mut gsl_eigen_francis_workspace,
) {
    if w.is_null() {
        return;
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_eigen_francis_T(
    compute_t: libc::c_int,
    mut w: *mut gsl_eigen_francis_workspace,
) {
    (*w).compute_t = compute_t;
}
pub unsafe extern "C" fn gsl_eigen_francis(
    mut H: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut w: *mut gsl_eigen_francis_workspace,
) -> libc::c_int {
    if (*H).size1 != (*H).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8
                as *const libc::c_char,
            b"francis.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*eval).size != (*H).size1 {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"francis.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*H).size1;
        let mut j: libc::c_int = 0;
        (*w).size = N;
        (*w).max_iterations = (30 as libc::c_int as libc::c_ulong).wrapping_mul(N);
        (*w).H = H;
        (*w).n_iter = 0 as libc::c_int as size_t;
        (*w).n_evals = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int;
        while j < N as libc::c_int - 3 as libc::c_int {
            gsl_matrix_set(
                H,
                (j as size_t).wrapping_add(2 as libc::c_int as libc::c_ulong),
                j as size_t,
                0.0f64,
            );
            gsl_matrix_set(
                H,
                (j as size_t).wrapping_add(3 as libc::c_int as libc::c_ulong),
                j as size_t,
                0.0f64,
            );
            j += 1;
            j;
        }
        if N > 2 as libc::c_int as libc::c_ulong {
            gsl_matrix_set(
                H,
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(3 as libc::c_int as libc::c_ulong),
                0.0f64,
            );
        }
        francis_schur_decomp(H, eval, w);
        if (*w).n_evals != N {
            gsl_error(
                b"maximum iterations reached without finding all eigenvalues\0"
                    as *const u8 as *const libc::c_char,
                b"francis.c\0" as *const u8 as *const libc::c_char,
                209 as libc::c_int,
                GSL_EMAXITER as libc::c_int,
            );
            return GSL_EMAXITER as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_eigen_francis_Z(
    mut H: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut Z: *mut gsl_matrix,
    mut w: *mut gsl_eigen_francis_workspace,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    (*w).Z = Z;
    s = gsl_eigen_francis(H, eval, w);
    (*w).Z = 0 as *mut gsl_matrix;
    return s;
}
#[inline]
unsafe extern "C" fn francis_schur_decomp(
    mut H: *mut gsl_matrix,
    mut eval: *mut gsl_vector_complex,
    mut w: *mut gsl_eigen_francis_workspace,
) {
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
    let mut N: size_t = 0;
    let mut q: size_t = 0;
    let mut lambda1: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut lambda2: gsl_complex = gsl_complex { dat: [0.; 2] };
    N = (*H).size1;
    m = gsl_matrix_submatrix(
        H,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        N,
        N,
    );
    while N > 2 as libc::c_int as libc::c_ulong
        && {
            let fresh0 = (*w).n_iter;
            (*w).n_iter = ((*w).n_iter).wrapping_add(1);
            fresh0 < (*w).max_iterations
        }
    {
        q = francis_search_subdiag_small_elements(&mut m.matrix);
        if q == 0 as libc::c_int as libc::c_ulong {
            francis_qrstep(&mut m.matrix, w);
        } else if q == N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            lambda1.dat[0 as libc::c_int as usize] = gsl_matrix_get(&mut m.matrix, q, q);
            lambda1.dat[1 as libc::c_int as usize] = 0.0f64;
            gsl_vector_complex_set(eval, (*w).n_evals, lambda1);
            (*w)
                .n_evals = ((*w).n_evals as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*w).n_iter = 0 as libc::c_int as size_t;
            N = N.wrapping_sub(1);
            N;
            m = gsl_matrix_submatrix(
                &mut m.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                N,
            );
        } else if q == N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            let mut v: gsl_matrix_view = gsl_matrix_view {
                matrix: gsl_matrix {
                    size1: 0,
                    size2: 0,
                    tda: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            v = gsl_matrix_submatrix(
                &mut m.matrix,
                q,
                q,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            );
            francis_schur_standardize(&mut v.matrix, &mut lambda1, &mut lambda2, w);
            gsl_vector_complex_set(eval, (*w).n_evals, lambda1);
            gsl_vector_complex_set(
                eval,
                ((*w).n_evals).wrapping_add(1 as libc::c_int as libc::c_ulong),
                lambda2,
            );
            (*w)
                .n_evals = ((*w).n_evals as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*w).n_iter = 0 as libc::c_int as size_t;
            N = (N as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            m = gsl_matrix_submatrix(
                &mut m.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                N,
            );
        } else if q == 1 as libc::c_int as libc::c_ulong {
            lambda1
                .dat[0 as libc::c_int
                as usize] = gsl_matrix_get(
                &mut m.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
            );
            lambda1.dat[1 as libc::c_int as usize] = 0.0f64;
            gsl_vector_complex_set(eval, (*w).n_evals, lambda1);
            (*w)
                .n_evals = ((*w).n_evals as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*w).n_iter = 0 as libc::c_int as size_t;
            N = N.wrapping_sub(1);
            N;
            m = gsl_matrix_submatrix(
                &mut m.matrix,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                N,
                N,
            );
        } else if q == 2 as libc::c_int as libc::c_ulong {
            let mut v_0: gsl_matrix_view = gsl_matrix_view {
                matrix: gsl_matrix {
                    size1: 0,
                    size2: 0,
                    tda: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            v_0 = gsl_matrix_submatrix(
                &mut m.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            );
            francis_schur_standardize(&mut v_0.matrix, &mut lambda1, &mut lambda2, w);
            gsl_vector_complex_set(eval, (*w).n_evals, lambda1);
            gsl_vector_complex_set(
                eval,
                ((*w).n_evals).wrapping_add(1 as libc::c_int as libc::c_ulong),
                lambda2,
            );
            (*w)
                .n_evals = ((*w).n_evals as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*w).n_iter = 0 as libc::c_int as size_t;
            N = (N as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            m = gsl_matrix_submatrix(
                &mut m.matrix,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                N,
                N,
            );
        } else {
            let mut v_1: gsl_matrix_view = gsl_matrix_view {
                matrix: gsl_matrix {
                    size1: 0,
                    size2: 0,
                    tda: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            v_1 = gsl_matrix_submatrix(
                &mut m.matrix,
                q,
                q,
                N.wrapping_sub(q),
                N.wrapping_sub(q),
            );
            francis_schur_decomp(&mut v_1.matrix, eval, w);
            v_1 = gsl_matrix_submatrix(
                &mut m.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                q,
                q,
            );
            francis_schur_decomp(&mut v_1.matrix, eval, w);
            N = 0 as libc::c_int as size_t;
        }
    }
    if N == 1 as libc::c_int as libc::c_ulong {
        lambda1
            .dat[0 as libc::c_int
            as usize] = gsl_matrix_get(
            &mut m.matrix,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        lambda1.dat[1 as libc::c_int as usize] = 0.0f64;
        gsl_vector_complex_set(eval, (*w).n_evals, lambda1);
        (*w)
            .n_evals = ((*w).n_evals as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*w).n_iter = 0 as libc::c_int as size_t;
    } else if N == 2 as libc::c_int as libc::c_ulong {
        francis_schur_standardize(&mut m.matrix, &mut lambda1, &mut lambda2, w);
        gsl_vector_complex_set(eval, (*w).n_evals, lambda1);
        gsl_vector_complex_set(
            eval,
            ((*w).n_evals).wrapping_add(1 as libc::c_int as libc::c_ulong),
            lambda2,
        );
        (*w)
            .n_evals = ((*w).n_evals as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*w).n_iter = 0 as libc::c_int as size_t;
    }
}
#[inline]
unsafe extern "C" fn francis_qrstep(
    mut H: *mut gsl_matrix,
    mut w: *mut gsl_eigen_francis_workspace,
) -> libc::c_int {
    let N: size_t = (*H).size1;
    let mut i: size_t = 0;
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
    let mut dat: [libc::c_double; 3] = [0.; 3];
    let mut scale: libc::c_double = 0.;
    let mut v2: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut v3: gsl_vector_view = gsl_vector_view {
        vector: gsl_vector {
            size: 0,
            stride: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut q: size_t = 0;
    let mut r: size_t = 0;
    let mut top: size_t = 0 as libc::c_int as size_t;
    let mut s: libc::c_double = 0.;
    let mut disc: libc::c_double = 0.;
    let mut h_nn: libc::c_double = 0.;
    let mut h_nm1nm1: libc::c_double = 0.;
    let mut h_cross: libc::c_double = 0.;
    let mut h_tmp1: libc::c_double = 0.;
    let mut h_tmp2: libc::c_double = 0.;
    v2 = gsl_vector_view_array(dat.as_mut_ptr(), 2 as libc::c_int as size_t);
    v3 = gsl_vector_view_array(dat.as_mut_ptr(), 3 as libc::c_int as size_t);
    if ((*w).n_iter).wrapping_rem(10 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        s = fabs(
            gsl_matrix_get(
                H,
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            ),
        )
            + fabs(
                gsl_matrix_get(
                    H,
                    N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(3 as libc::c_int as libc::c_ulong),
                ),
            );
        h_nn = gsl_matrix_get(
            H,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) + 0.75f64 * s;
        h_nm1nm1 = h_nn;
        h_cross = -0.4375f64 * s * s;
    } else {
        h_nn = gsl_matrix_get(
            H,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        h_nm1nm1 = gsl_matrix_get(
            H,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        h_cross = gsl_matrix_get(
            H,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        )
            * gsl_matrix_get(
                H,
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        disc = 0.5f64 * (h_nm1nm1 - h_nn);
        disc = disc * disc + h_cross;
        if disc > 0.0f64 {
            let mut ave: libc::c_double = 0.;
            disc = sqrt(disc);
            ave = 0.5f64 * (h_nm1nm1 + h_nn);
            if fabs(h_nm1nm1) - fabs(h_nn) > 0.0f64 {
                h_nm1nm1 = h_nm1nm1 * h_nn - h_cross;
                h_nn = h_nm1nm1
                    / (disc
                        * (if ave >= 0.0f64 {
                            1 as libc::c_int
                        } else {
                            -(1 as libc::c_int)
                        }) as libc::c_double + ave);
            } else {
                h_nn = disc
                    * (if ave >= 0.0f64 {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    }) as libc::c_double + ave;
            }
            h_nm1nm1 = h_nn;
            h_cross = 0.0f64;
        }
    }
    h_tmp1 = h_nm1nm1
        - gsl_matrix_get(H, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    h_tmp2 = h_nn
        - gsl_matrix_get(H, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    dat[0 as libc::c_int
        as usize] = (h_tmp1 * h_tmp2 - h_cross)
        / gsl_matrix_get(H, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t)
        + gsl_matrix_get(H, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    dat[1 as libc::c_int
        as usize] = gsl_matrix_get(
        H,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    ) - gsl_matrix_get(H, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t)
        - h_tmp1 - h_tmp2;
    dat[2 as libc::c_int
        as usize] = gsl_matrix_get(
        H,
        2 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    );
    scale = fabs(dat[0 as libc::c_int as usize]) + fabs(dat[1 as libc::c_int as usize])
        + fabs(dat[2 as libc::c_int as usize]);
    if scale != 0.0f64 {
        dat[0 as libc::c_int as usize] /= scale;
        dat[1 as libc::c_int as usize] /= scale;
        dat[2 as libc::c_int as usize] /= scale;
    }
    if !((*w).Z).is_null() || (*w).compute_t != 0 {
        top = francis_get_submatrix((*w).H, H);
    }
    i = 0 as libc::c_int as size_t;
    while i < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
        tau_i = gsl_linalg_householder_transform(&mut v3.vector);
        if tau_i != 0.0f64 {
            q = if 1 as libc::c_int > i as libc::c_int - 1 as libc::c_int {
                0 as libc::c_int as libc::c_ulong
            } else {
                i.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            r = if i.wrapping_add(3 as libc::c_int as libc::c_ulong)
                < N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                i.wrapping_add(3 as libc::c_int as libc::c_ulong)
            } else {
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            if (*w).compute_t != 0 {
                m = gsl_matrix_submatrix(
                    (*w).H,
                    top.wrapping_add(i),
                    top.wrapping_add(q),
                    3 as libc::c_int as size_t,
                    ((*w).size).wrapping_sub(top).wrapping_sub(q),
                );
                gsl_linalg_householder_hm(tau_i, &mut v3.vector, &mut m.matrix);
                m = gsl_matrix_submatrix(
                    (*w).H,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(i),
                    top.wrapping_add(r).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau_i, &mut v3.vector, &mut m.matrix);
            } else {
                m = gsl_matrix_submatrix(
                    H,
                    i,
                    q,
                    3 as libc::c_int as size_t,
                    N.wrapping_sub(q),
                );
                gsl_linalg_householder_hm(tau_i, &mut v3.vector, &mut m.matrix);
                m = gsl_matrix_submatrix(
                    H,
                    0 as libc::c_int as size_t,
                    i,
                    r.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau_i, &mut v3.vector, &mut m.matrix);
            }
            if !((*w).Z).is_null() {
                m = gsl_matrix_submatrix(
                    (*w).Z,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(i),
                    (*w).size,
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau_i, &mut v3.vector, &mut m.matrix);
            }
        }
        dat[0 as libc::c_int
            as usize] = gsl_matrix_get(
            H,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
            i,
        );
        dat[1 as libc::c_int
            as usize] = gsl_matrix_get(
            H,
            i.wrapping_add(2 as libc::c_int as libc::c_ulong),
            i,
        );
        if i < N.wrapping_sub(3 as libc::c_int as libc::c_ulong) {
            dat[2 as libc::c_int
                as usize] = gsl_matrix_get(
                H,
                i.wrapping_add(3 as libc::c_int as libc::c_ulong),
                i,
            );
        }
        scale = fabs(dat[0 as libc::c_int as usize])
            + fabs(dat[1 as libc::c_int as usize])
            + fabs(dat[2 as libc::c_int as usize]);
        if scale != 0.0f64 {
            dat[0 as libc::c_int as usize] /= scale;
            dat[1 as libc::c_int as usize] /= scale;
            dat[2 as libc::c_int as usize] /= scale;
        }
        i = i.wrapping_add(1);
        i;
    }
    scale = fabs(dat[0 as libc::c_int as usize]) + fabs(dat[1 as libc::c_int as usize]);
    if scale != 0.0f64 {
        dat[0 as libc::c_int as usize] /= scale;
        dat[1 as libc::c_int as usize] /= scale;
    }
    tau_i = gsl_linalg_householder_transform(&mut v2.vector);
    if (*w).compute_t != 0 {
        m = gsl_matrix_submatrix(
            (*w).H,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            top.wrapping_add(N).wrapping_sub(3 as libc::c_int as libc::c_ulong),
            2 as libc::c_int as size_t,
            ((*w).size)
                .wrapping_sub(top)
                .wrapping_sub(N)
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        );
        gsl_linalg_householder_hm(tau_i, &mut v2.vector, &mut m.matrix);
        m = gsl_matrix_submatrix(
            (*w).H,
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            top.wrapping_add(N),
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau_i, &mut v2.vector, &mut m.matrix);
    } else {
        m = gsl_matrix_submatrix(
            H,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(3 as libc::c_int as libc::c_ulong),
            2 as libc::c_int as size_t,
            3 as libc::c_int as size_t,
        );
        gsl_linalg_householder_hm(tau_i, &mut v2.vector, &mut m.matrix);
        m = gsl_matrix_submatrix(
            H,
            0 as libc::c_int as size_t,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N,
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau_i, &mut v2.vector, &mut m.matrix);
    }
    if !((*w).Z).is_null() {
        m = gsl_matrix_submatrix(
            (*w).Z,
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            (*w).size,
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau_i, &mut v2.vector, &mut m.matrix);
    }
    return GSL_SUCCESS as libc::c_int;
}
#[inline]
unsafe extern "C" fn francis_search_subdiag_small_elements(
    mut A: *mut gsl_matrix,
) -> size_t {
    let N: size_t = (*A).size1;
    let mut i: size_t = 0;
    let mut dpel: libc::c_double = gsl_matrix_get(
        A,
        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    i = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > 0 as libc::c_int as libc::c_ulong {
        let mut sel: libc::c_double = gsl_matrix_get(
            A,
            i,
            i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut del: libc::c_double = gsl_matrix_get(A, i, i);
        if sel == 0.0f64
            || fabs(sel) < 2.2204460492503131e-16f64 * (fabs(del) + fabs(dpel))
        {
            gsl_matrix_set(
                A,
                i,
                i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0.0f64,
            );
            return i;
        }
        dpel = del;
        i = i.wrapping_sub(1);
        i;
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn francis_schur_standardize(
    mut A: *mut gsl_matrix,
    mut eval1: *mut gsl_complex,
    mut eval2: *mut gsl_complex,
    mut w: *mut gsl_eigen_francis_workspace,
) {
    let N: size_t = (*w).size;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
    let mut top: size_t = 0;
    top = francis_get_submatrix((*w).H, A);
    francis_standard_form(A, &mut cs, &mut sn);
    (*eval1)
        .dat[0 as libc::c_int
        as usize] = gsl_matrix_get(
        A,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    (*eval2)
        .dat[0 as libc::c_int
        as usize] = gsl_matrix_get(
        A,
        1 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    );
    if gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t)
        == 0.0f64
    {
        (*eval1).dat[1 as libc::c_int as usize] = 0.0f64;
        (*eval2).dat[1 as libc::c_int as usize] = 0.0f64;
    } else {
        let mut tmp: libc::c_double = sqrt(
            fabs(
                gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t),
            )
                * fabs(
                    gsl_matrix_get(
                        A,
                        1 as libc::c_int as size_t,
                        0 as libc::c_int as size_t,
                    ),
                ),
        );
        (*eval1).dat[1 as libc::c_int as usize] = tmp;
        (*eval2).dat[1 as libc::c_int as usize] = -tmp;
    }
    if (*w).compute_t != 0 {
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
        if top < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            xv = gsl_matrix_subrow(
                (*w).H,
                top,
                top.wrapping_add(2 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(top).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subrow(
                (*w).H,
                top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(2 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(top).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        }
        if top > 0 as libc::c_int as libc::c_ulong {
            xv = gsl_matrix_subcolumn((*w).H, top, 0 as libc::c_int as size_t, top);
            yv = gsl_matrix_subcolumn(
                (*w).H,
                top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                top,
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        }
    }
    if !((*w).Z).is_null() {
        let mut xv_0: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut yv_0: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        xv_0 = gsl_matrix_column((*w).Z, top);
        yv_0 = gsl_matrix_column(
            (*w).Z,
            top.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        gsl_blas_drot(&mut xv_0.vector, &mut yv_0.vector, cs, sn);
    }
}
#[inline]
unsafe extern "C" fn francis_get_submatrix(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
) -> size_t {
    let mut diff: size_t = 0;
    let mut ratio: libc::c_double = 0.;
    let mut top: size_t = 0;
    diff = ((*B).data).offset_from((*A).data) as libc::c_long as size_t;
    ratio = diff as libc::c_double
        / ((*A).tda).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
    top = floor(ratio) as size_t;
    return top;
}
unsafe extern "C" fn francis_standard_form(
    mut A: *mut gsl_matrix,
    mut cs: *mut libc::c_double,
    mut sn: *mut libc::c_double,
) {
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut bcmax: libc::c_double = 0.;
    let mut bcmis: libc::c_double = 0.;
    let mut scale: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut sigma: libc::c_double = 0.;
    let mut cs1: libc::c_double = 0.;
    let mut sn1: libc::c_double = 0.;
    let mut aa: libc::c_double = 0.;
    let mut bb: libc::c_double = 0.;
    let mut cc: libc::c_double = 0.;
    let mut dd: libc::c_double = 0.;
    let mut sab: libc::c_double = 0.;
    let mut sac: libc::c_double = 0.;
    a = gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    b = gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    c = gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    d = gsl_matrix_get(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    if c == 0.0f64 {
        *cs = 1.0f64;
        *sn = 0.0f64;
    } else if b == 0.0f64 {
        *cs = 0.0f64;
        *sn = 1.0f64;
        tmp = d;
        d = a;
        a = tmp;
        b = -c;
        c = 0.0f64;
    } else if a - d == 0.0f64
        && (if b >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
            != (if c >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
    {
        *cs = 1.0f64;
        *sn = 0.0f64;
    } else {
        tmp = a - d;
        p = 0.5f64 * tmp;
        bcmax = if fabs(b) > fabs(c) { fabs(b) } else { fabs(c) };
        bcmis = (if fabs(b) < fabs(c) { fabs(b) } else { fabs(c) })
            * (if b >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                as libc::c_double
            * (if c >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                as libc::c_double;
        scale = if fabs(p) > bcmax { fabs(p) } else { bcmax };
        z = p / scale * p + bcmax / scale * bcmis;
        if z >= 4.0f64 * 2.2204460492503131e-16f64 {
            z = p
                + (if p >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                    as libc::c_double * fabs(sqrt(scale) * sqrt(z));
            a = d + z;
            d -= bcmax / z * bcmis;
            tau = gsl_hypot(c, z);
            *cs = z / tau;
            *sn = c / tau;
            b -= c;
            c = 0.0f64;
        } else {
            sigma = b + c;
            tau = gsl_hypot(sigma, tmp);
            *cs = sqrt(0.5f64 * (1.0f64 + fabs(sigma) / tau));
            *sn = -(p / (tau * *cs))
                * (if sigma >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                    as libc::c_double;
            aa = a * *cs + b * *sn;
            bb = -a * *sn + b * *cs;
            cc = c * *cs + d * *sn;
            dd = -c * *sn + d * *cs;
            a = aa * *cs + cc * *sn;
            b = bb * *cs + dd * *sn;
            c = -aa * *sn + cc * *cs;
            d = -bb * *sn + dd * *cs;
            tmp = 0.5f64 * (a + d);
            d = tmp;
            a = d;
            if c != 0.0f64 {
                if b != 0.0f64 {
                    if (if b >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                        == (if c >= 0.0f64 {
                            1 as libc::c_int
                        } else {
                            -(1 as libc::c_int)
                        })
                    {
                        sab = sqrt(fabs(b));
                        sac = sqrt(fabs(c));
                        p = (if c >= 0.0f64 {
                            1 as libc::c_int
                        } else {
                            -(1 as libc::c_int)
                        }) as libc::c_double * fabs(sab * sac);
                        tau = 1.0f64 / sqrt(fabs(b + c));
                        a = tmp + p;
                        d = tmp - p;
                        b -= c;
                        c = 0.0f64;
                        cs1 = sab * tau;
                        sn1 = sac * tau;
                        tmp = *cs * cs1 - *sn * sn1;
                        *sn = *cs * sn1 + *sn * cs1;
                        *cs = tmp;
                    }
                } else {
                    b = -c;
                    c = 0.0f64;
                    tmp = *cs;
                    *cs = -*sn;
                    *sn = tmp;
                }
            }
        }
    }
    gsl_matrix_set(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t, a);
    gsl_matrix_set(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t, b);
    gsl_matrix_set(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t, c);
    gsl_matrix_set(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t, d);
}
