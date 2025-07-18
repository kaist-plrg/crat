use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_schur_gen_eigvals(
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        wr1: *mut libc::c_double,
        wr2: *mut libc::c_double,
        wi: *mut libc::c_double,
        scale1: *mut libc::c_double,
        scale2: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
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
    fn gsl_matrix_view_array(
        base: *mut libc::c_double,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_blas_drot(
        X: *mut gsl_vector,
        Y: *mut gsl_vector,
        c: libc::c_double,
        s: libc::c_double,
    ) -> libc::c_int;
    fn gsl_linalg_hesstri_decomp(
        A: *mut gsl_matrix,
        B: *mut gsl_matrix,
        U: *mut gsl_matrix,
        V: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_householder_hm(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
    fn gsl_linalg_householder_mh(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_SV_decomp(
        A: *mut gsl_matrix,
        V: *mut gsl_matrix,
        S: *mut gsl_vector,
        work: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_hypot3(
        x: libc::c_double,
        y: libc::c_double,
        z: libc::c_double,
    ) -> libc::c_double;
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
pub struct gsl_eigen_gen_workspace {
    pub size: size_t,
    pub work: *mut gsl_vector,
    pub n_evals: size_t,
    pub max_iterations: size_t,
    pub n_iter: size_t,
    pub eshift: libc::c_double,
    pub needtop: libc::c_int,
    pub atol: libc::c_double,
    pub btol: libc::c_double,
    pub ascale: libc::c_double,
    pub bscale: libc::c_double,
    pub H: *mut gsl_matrix,
    pub R: *mut gsl_matrix,
    pub compute_s: libc::c_int,
    pub compute_t: libc::c_int,
    pub Q: *mut gsl_matrix,
    pub Z: *mut gsl_matrix,
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
pub unsafe extern "C" fn gsl_eigen_gen_alloc(n: size_t) -> *mut gsl_eigen_gen_workspace {
    let mut w: *mut gsl_eigen_gen_workspace = 0 as *mut gsl_eigen_gen_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_eigen_gen_workspace;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_eigen_gen_workspace>() as libc::c_ulong,
    ) as *mut gsl_eigen_gen_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_gen_workspace;
    }
    (*w).size = n;
    (*w).max_iterations = (30 as libc::c_int as libc::c_ulong).wrapping_mul(n);
    (*w).n_evals = 0 as libc::c_int as size_t;
    (*w).n_iter = 0 as libc::c_int as size_t;
    (*w).needtop = 0 as libc::c_int;
    (*w).atol = 0.0f64;
    (*w).btol = 0.0f64;
    (*w).ascale = 0.0f64;
    (*w).bscale = 0.0f64;
    (*w).eshift = 0.0f64;
    (*w).H = 0 as *mut gsl_matrix;
    (*w).R = 0 as *mut gsl_matrix;
    (*w).compute_s = 0 as libc::c_int;
    (*w).compute_t = 0 as libc::c_int;
    (*w).Q = 0 as *mut gsl_matrix;
    (*w).Z = 0 as *mut gsl_matrix;
    (*w).work = gsl_vector_alloc(n);
    if ((*w).work).is_null() {
        gsl_eigen_gen_free(w);
        gsl_error(
            b"failed to allocate space for additional workspace\0" as *const u8
                as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_gen_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_eigen_gen_free(mut w: *mut gsl_eigen_gen_workspace) {
    if w.is_null() {
        return;
    }
    if !((*w).work).is_null() {
        gsl_vector_free((*w).work);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_eigen_gen_params(
    compute_s: libc::c_int,
    compute_t: libc::c_int,
    balance: libc::c_int,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    (*w).compute_s = compute_s;
    (*w).compute_t = compute_t;
}
pub unsafe extern "C" fn gsl_eigen_gen(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut w: *mut gsl_eigen_gen_workspace,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8
                as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*B).size1 || N != (*B).size2 {
        gsl_error(
            b"B matrix dimensions must match A\0" as *const u8 as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*alpha).size != N || (*beta).size != N {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*w).size != N {
        gsl_error(
            b"matrix size does not match workspace\0" as *const u8
                as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut anorm: libc::c_double = 0.;
        let mut bnorm: libc::c_double = 0.;
        gsl_linalg_hesstri_decomp(A, B, (*w).Q, (*w).Z, (*w).work);
        (*w).H = A;
        (*w).R = B;
        (*w).n_evals = 0 as libc::c_int as size_t;
        (*w).n_iter = 0 as libc::c_int as size_t;
        (*w).eshift = 0.0f64;
        (*w)
            .needtop = (!((*w).Q).is_null() || !((*w).Z).is_null() || (*w).compute_t != 0
            || (*w).compute_s != 0) as libc::c_int;
        anorm = normF(A);
        bnorm = normF(B);
        (*w)
            .atol = if 2.2250738585072014e-308f64 > 2.2204460492503131e-16f64 * anorm {
            2.2250738585072014e-308f64
        } else {
            2.2204460492503131e-16f64 * anorm
        };
        (*w)
            .btol = if 2.2250738585072014e-308f64 > 2.2204460492503131e-16f64 * bnorm {
            2.2250738585072014e-308f64
        } else {
            2.2204460492503131e-16f64 * bnorm
        };
        (*w)
            .ascale = 1.0f64
            / (if 2.2250738585072014e-308f64 > anorm {
                2.2250738585072014e-308f64
            } else {
                anorm
            });
        (*w)
            .bscale = 1.0f64
            / (if 2.2250738585072014e-308f64 > bnorm {
                2.2250738585072014e-308f64
            } else {
                bnorm
            });
        gen_schur_decomp(A, B, alpha, beta, w);
        if (*w).n_evals != N {
            return GSL_EMAXITER as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_eigen_gen_QZ(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut Q: *mut gsl_matrix,
    mut Z: *mut gsl_matrix,
    mut w: *mut gsl_eigen_gen_workspace,
) -> libc::c_int {
    if !Q.is_null() && ((*A).size1 != (*Q).size1 || (*A).size1 != (*Q).size2) {
        gsl_error(
            b"Q matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            305 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if !Z.is_null() && ((*A).size1 != (*Z).size1 || (*A).size1 != (*Z).size2) {
        gsl_error(
            b"Z matrix has wrong dimensions\0" as *const u8 as *const libc::c_char,
            b"gen.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut s: libc::c_int = 0;
        (*w).Q = Q;
        (*w).Z = Z;
        s = gsl_eigen_gen(A, B, alpha, beta, w);
        (*w).Q = 0 as *mut gsl_matrix;
        (*w).Z = 0 as *mut gsl_matrix;
        return s;
    };
}
unsafe extern "C" fn gen_schur_decomp(
    mut H: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    let mut N: size_t = 0;
    let mut h: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut r: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut vh: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut vr: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut q: size_t = 0;
    let mut z1: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut z2: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut s: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    N = (*H).size1;
    h = gsl_matrix_submatrix(
        H,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        N,
        N,
    );
    r = gsl_matrix_submatrix(
        R,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        N,
        N,
    );
    while N > 1 as libc::c_int as libc::c_ulong
        && {
            let fresh0 = (*w).n_iter;
            (*w).n_iter = ((*w).n_iter).wrapping_add(1);
            fresh0 < (*w).max_iterations
        }
    {
        q = gen_search_small_elements(&mut h.matrix, &mut r.matrix, &mut flag, w);
        if flag == 0 as libc::c_int {
            s = gen_qzstep(&mut h.matrix, &mut r.matrix, w);
            if !(s == GSL_CONTINUE as libc::c_int) {
                continue;
            }
            s = gen_schur_standardize2(
                &mut h.matrix,
                &mut r.matrix,
                &mut z1,
                &mut z2,
                &mut a,
                &mut b,
                w,
            );
            if !(s != GSL_SUCCESS as libc::c_int) {
                gen_store_eigval2(&mut h.matrix, &mut z1, a, &mut z2, b, alpha, beta, w);
                N = 0 as libc::c_int as size_t;
            }
        } else if flag == 2 as libc::c_int {
            if q == 0 as libc::c_int as libc::c_ulong {
                gen_tri_split_top(&mut h.matrix, &mut r.matrix, w);
            } else {
                if q != N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    gen_tri_chase_zero(&mut h.matrix, &mut r.matrix, q, w);
                }
                gen_tri_zero_H(&mut h.matrix, &mut r.matrix, w);
            }
        } else if q == N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            vh = gsl_matrix_submatrix(
                &mut h.matrix,
                q,
                q,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
            );
            vr = gsl_matrix_submatrix(
                &mut r.matrix,
                q,
                q,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
            );
            gen_schur_standardize1(&mut vh.matrix, &mut vr.matrix, &mut a, &mut b, w);
            gen_store_eigval1(&mut vh.matrix, a, b, alpha, beta, w);
            N = N.wrapping_sub(1);
            N;
            h = gsl_matrix_submatrix(
                &mut h.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                N,
            );
            r = gsl_matrix_submatrix(
                &mut r.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                N,
            );
        } else if q == N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            vh = gsl_matrix_submatrix(
                &mut h.matrix,
                q,
                q,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            );
            vr = gsl_matrix_submatrix(
                &mut r.matrix,
                q,
                q,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            );
            s = gen_schur_standardize2(
                &mut vh.matrix,
                &mut vr.matrix,
                &mut z1,
                &mut z2,
                &mut a,
                &mut b,
                w,
            );
            if s != GSL_SUCCESS as libc::c_int {
                gen_schur_decomp(&mut vh.matrix, &mut vr.matrix, alpha, beta, w);
            } else {
                gen_store_eigval2(
                    &mut vh.matrix,
                    &mut z1,
                    a,
                    &mut z2,
                    b,
                    alpha,
                    beta,
                    w,
                );
            }
            N = (N as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            h = gsl_matrix_submatrix(
                &mut h.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                N,
            );
            r = gsl_matrix_submatrix(
                &mut r.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                N,
                N,
            );
        } else if q == 1 as libc::c_int as libc::c_ulong {
            vh = gsl_matrix_submatrix(
                &mut h.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
            );
            vr = gsl_matrix_submatrix(
                &mut r.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
            );
            gen_schur_standardize1(&mut vh.matrix, &mut vr.matrix, &mut a, &mut b, w);
            gen_store_eigval1(&mut vh.matrix, a, b, alpha, beta, w);
            N = N.wrapping_sub(1);
            N;
            h = gsl_matrix_submatrix(
                &mut h.matrix,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                N,
                N,
            );
            r = gsl_matrix_submatrix(
                &mut r.matrix,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                N,
                N,
            );
        } else if q == 2 as libc::c_int as libc::c_ulong {
            vh = gsl_matrix_submatrix(
                &mut h.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            );
            vr = gsl_matrix_submatrix(
                &mut r.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            );
            s = gen_schur_standardize2(
                &mut vh.matrix,
                &mut vr.matrix,
                &mut z1,
                &mut z2,
                &mut a,
                &mut b,
                w,
            );
            if s != GSL_SUCCESS as libc::c_int {
                gen_schur_decomp(&mut vh.matrix, &mut vr.matrix, alpha, beta, w);
            } else {
                gen_store_eigval2(
                    &mut vh.matrix,
                    &mut z1,
                    a,
                    &mut z2,
                    b,
                    alpha,
                    beta,
                    w,
                );
            }
            N = (N as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            h = gsl_matrix_submatrix(
                &mut h.matrix,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                N,
                N,
            );
            r = gsl_matrix_submatrix(
                &mut r.matrix,
                2 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                N,
                N,
            );
        } else {
            vh = gsl_matrix_submatrix(
                &mut h.matrix,
                q,
                q,
                N.wrapping_sub(q),
                N.wrapping_sub(q),
            );
            vr = gsl_matrix_submatrix(
                &mut r.matrix,
                q,
                q,
                N.wrapping_sub(q),
                N.wrapping_sub(q),
            );
            gen_schur_decomp(&mut vh.matrix, &mut vr.matrix, alpha, beta, w);
            vh = gsl_matrix_submatrix(
                &mut h.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                q,
                q,
            );
            vr = gsl_matrix_submatrix(
                &mut r.matrix,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                q,
                q,
            );
            gen_schur_decomp(&mut vh.matrix, &mut vr.matrix, alpha, beta, w);
            N = 0 as libc::c_int as size_t;
        }
    }
    if N == 1 as libc::c_int as libc::c_ulong {
        gen_schur_standardize1(&mut h.matrix, &mut r.matrix, &mut a, &mut b, w);
        gen_store_eigval1(&mut h.matrix, a, b, alpha, beta, w);
    }
}
#[inline]
unsafe extern "C" fn gen_qzstep(
    mut H: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut w: *mut gsl_eigen_gen_workspace,
) -> libc::c_int {
    let N: size_t = (*H).size1;
    let mut vh: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut vr: gsl_matrix_view = gsl_matrix_view {
        matrix: gsl_matrix {
            size1: 0,
            size2: 0,
            tda: 0,
            data: 0 as *mut libc::c_double,
            block: 0 as *mut gsl_block,
            owner: 0,
        },
    };
    let mut wr1: libc::c_double = 0.;
    let mut wr2: libc::c_double = 0.;
    let mut wi: libc::c_double = 0.;
    let mut scale1: libc::c_double = 0.;
    let mut scale2: libc::c_double = 0.;
    let mut scale: libc::c_double = 0.;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut temp2: libc::c_double = 0.;
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
    let mut top: size_t = 0 as libc::c_int as size_t;
    let mut rows: size_t = 0;
    if ((*w).n_iter).wrapping_rem(10 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        if 2.2250738585072014e-308f64 * (*w).max_iterations as libc::c_double
            * fabs(
                gsl_matrix_get(
                    H,
                    N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ),
            )
            < fabs(
                gsl_matrix_get(
                    R,
                    N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                ),
            )
        {
            (*w).eshift
                += gsl_matrix_get(
                    H,
                    N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                    / gsl_matrix_get(
                        R,
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    );
        } else {
            (*w).eshift
                += 1.0f64
                    / (2.2250738585072014e-308f64
                        * (*w).max_iterations as libc::c_double);
        }
        if (*w).eshift < 2.2204460492503131e-16f64
            && 2.2250738585072014e-308f64 * (*w).max_iterations as libc::c_double
                * fabs(
                    gsl_matrix_get(
                        H,
                        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    ),
                )
                < fabs(
                    gsl_matrix_get(
                        R,
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    ),
                )
        {
            (*w)
                .eshift = 1.736f64
                * ((*w).ascale
                    * gsl_matrix_get(
                        H,
                        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    ))
                / ((*w).bscale
                    * gsl_matrix_get(
                        R,
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    ));
        }
        scale1 = 1.0f64;
        wr1 = (*w).eshift;
    } else {
        vh = gsl_matrix_submatrix(
            H,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            2 as libc::c_int as size_t,
            2 as libc::c_int as size_t,
        );
        vr = gsl_matrix_submatrix(
            R,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            2 as libc::c_int as size_t,
            2 as libc::c_int as size_t,
        );
        gsl_schur_gen_eigvals(
            &mut vh.matrix,
            &mut vr.matrix,
            &mut wr1,
            &mut wr2,
            &mut wi,
            &mut scale1,
            &mut scale2,
        );
        if wi != 0.0f64 {
            if N == 2 as libc::c_int as libc::c_ulong {
                return GSL_CONTINUE as libc::c_int
            } else {
                gen_qzstep_d(H, R, w);
            }
            return GSL_SUCCESS as libc::c_int;
        }
    }
    temp = (if (*w).ascale < 1.0f64 { (*w).ascale } else { 1.0f64 })
        * (0.5f64 / 2.2250738585072014e-308f64);
    if scale1 > temp {
        scale = temp / scale1;
    } else {
        scale = 1.0f64;
    }
    temp = (if (*w).bscale < 1.0f64 { (*w).bscale } else { 1.0f64 })
        * (0.5f64 / 2.2250738585072014e-308f64);
    if fabs(wr1) > temp {
        scale = if scale < temp / fabs(wr1) { scale } else { temp / fabs(wr1) };
    }
    scale1 *= scale;
    wr1 *= scale;
    if (*w).needtop != 0 {
        top = gen_get_submatrix((*w).H, H);
    }
    temp = scale1
        * gsl_matrix_get(H, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t)
        - wr1
            * gsl_matrix_get(R, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    temp2 = scale1
        * gsl_matrix_get(H, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    gsl_linalg_givens(temp, temp2, &mut cs, &mut sn);
    sn = -sn;
    j = 0 as libc::c_int as size_t;
    while j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        if j > 0 as libc::c_int as libc::c_ulong {
            temp = gsl_matrix_get(
                H,
                j,
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            temp2 = gsl_matrix_get(
                H,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_linalg_givens(temp, temp2, &mut cs, &mut sn);
            sn = -sn;
            temp = cs
                * gsl_matrix_get(H, j, j.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                + sn
                    * gsl_matrix_get(
                        H,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
            gsl_matrix_set(
                H,
                j,
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                temp,
            );
            gsl_matrix_set(
                H,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0.0f64,
            );
        }
        if (*w).compute_s != 0 {
            xv = gsl_matrix_subrow(
                (*w).H,
                top.wrapping_add(j),
                top.wrapping_add(j),
                ((*w).size).wrapping_sub(top).wrapping_sub(j),
            );
            yv = gsl_matrix_subrow(
                (*w).H,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(j),
                ((*w).size).wrapping_sub(top).wrapping_sub(j),
            );
        } else {
            xv = gsl_matrix_subrow(H, j, j, N.wrapping_sub(j));
            yv = gsl_matrix_subrow(
                H,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j,
                N.wrapping_sub(j),
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        if (*w).compute_t != 0 {
            xv = gsl_matrix_subrow(
                (*w).R,
                top.wrapping_add(j),
                top.wrapping_add(j),
                ((*w).size).wrapping_sub(top).wrapping_sub(j),
            );
            yv = gsl_matrix_subrow(
                (*w).R,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(j),
                ((*w).size).wrapping_sub(top).wrapping_sub(j),
            );
        } else {
            xv = gsl_matrix_subrow(R, j, j, N.wrapping_sub(j));
            yv = gsl_matrix_subrow(
                R,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j,
                N.wrapping_sub(j),
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        if !((*w).Q).is_null() {
            xv = gsl_matrix_column((*w).Q, top.wrapping_add(j));
            yv = gsl_matrix_column(
                (*w).Q,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        }
        temp = gsl_matrix_get(
            R,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        temp2 = gsl_matrix_get(R, j.wrapping_add(1 as libc::c_int as libc::c_ulong), j);
        gsl_linalg_givens(temp, temp2, &mut cs, &mut sn);
        rows = if j.wrapping_add(3 as libc::c_int as libc::c_ulong) < N {
            j.wrapping_add(3 as libc::c_int as libc::c_ulong)
        } else {
            N
        };
        if (*w).compute_s != 0 {
            xv = gsl_matrix_subcolumn(
                (*w).H,
                top.wrapping_add(j),
                0 as libc::c_int as size_t,
                top.wrapping_add(rows),
            );
            yv = gsl_matrix_subcolumn(
                (*w).H,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                top.wrapping_add(rows),
            );
        } else {
            xv = gsl_matrix_subcolumn(H, j, 0 as libc::c_int as size_t, rows);
            yv = gsl_matrix_subcolumn(
                H,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                rows,
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        rows = if j.wrapping_add(2 as libc::c_int as libc::c_ulong) < N {
            j.wrapping_add(2 as libc::c_int as libc::c_ulong)
        } else {
            N
        };
        if (*w).compute_t != 0 {
            xv = gsl_matrix_subcolumn(
                (*w).R,
                top.wrapping_add(j),
                0 as libc::c_int as size_t,
                top.wrapping_add(rows),
            );
            yv = gsl_matrix_subcolumn(
                (*w).R,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                top.wrapping_add(rows),
            );
        } else {
            xv = gsl_matrix_subcolumn(R, j, 0 as libc::c_int as size_t, rows);
            yv = gsl_matrix_subcolumn(
                R,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                rows,
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        if !((*w).Z).is_null() {
            xv = gsl_matrix_column((*w).Z, top.wrapping_add(j));
            yv = gsl_matrix_column(
                (*w).Z,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[inline]
unsafe extern "C" fn gen_qzstep_d(
    mut H: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    let N: size_t = (*H).size1;
    let mut j: size_t = 0;
    let mut dat: [libc::c_double; 3] = [0.; 3];
    let mut tau: libc::c_double = 0.;
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
    let mut tmp: libc::c_double = 0.;
    let mut q: size_t = 0;
    let mut r: size_t = 0;
    let mut top: size_t = 0 as libc::c_int as size_t;
    let mut scale: libc::c_double = 0.;
    let mut AB11: libc::c_double = 0.;
    let mut AB22: libc::c_double = 0.;
    let mut ABNN: libc::c_double = 0.;
    let mut ABMM: libc::c_double = 0.;
    let mut AMNBNN: libc::c_double = 0.;
    let mut ANMBMM: libc::c_double = 0.;
    let mut A21B11: libc::c_double = 0.;
    let mut A12B22: libc::c_double = 0.;
    let mut A32B22: libc::c_double = 0.;
    let mut B12B22: libc::c_double = 0.;
    let mut BMNBNN: libc::c_double = 0.;
    v2 = gsl_vector_view_array(dat.as_mut_ptr(), 2 as libc::c_int as size_t);
    v3 = gsl_vector_view_array(dat.as_mut_ptr(), 3 as libc::c_int as size_t);
    if (*w).needtop != 0 {
        top = gen_get_submatrix((*w).H, H);
    }
    ABMM = (*w).ascale
        * gsl_matrix_get(
            H,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        )
        / ((*w).bscale
            * gsl_matrix_get(
                R,
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            ));
    ABNN = (*w).ascale
        * gsl_matrix_get(
            H,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        / ((*w).bscale
            * gsl_matrix_get(
                R,
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ));
    AB11 = (*w).ascale
        * gsl_matrix_get(H, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t)
        / ((*w).bscale
            * gsl_matrix_get(R, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t));
    AB22 = (*w).ascale
        * gsl_matrix_get(H, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
        / ((*w).bscale
            * gsl_matrix_get(R, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t));
    AMNBNN = (*w).ascale
        * gsl_matrix_get(
            H,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        / ((*w).bscale
            * gsl_matrix_get(
                R,
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ));
    ANMBMM = (*w).ascale
        * gsl_matrix_get(
            H,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        )
        / ((*w).bscale
            * gsl_matrix_get(
                R,
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            ));
    BMNBNN = gsl_matrix_get(
        R,
        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    )
        / gsl_matrix_get(
            R,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    A21B11 = (*w).ascale
        * gsl_matrix_get(H, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t)
        / ((*w).bscale
            * gsl_matrix_get(R, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t));
    A12B22 = (*w).ascale
        * gsl_matrix_get(H, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t)
        / ((*w).bscale
            * gsl_matrix_get(R, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t));
    A32B22 = (*w).ascale
        * gsl_matrix_get(H, 2 as libc::c_int as size_t, 1 as libc::c_int as size_t)
        / ((*w).bscale
            * gsl_matrix_get(R, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t));
    B12B22 = gsl_matrix_get(R, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t)
        / gsl_matrix_get(R, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    dat[0 as libc::c_int
        as usize] = (ABMM - AB11) * (ABNN - AB11) - AMNBNN * ANMBMM
        + ANMBMM * BMNBNN * AB11 + (A12B22 - AB11 * B12B22) * A21B11;
    dat[1 as libc::c_int
        as usize] = (AB22 - AB11 - A21B11 * B12B22 - (ABMM - AB11) - (ABNN - AB11)
        + ANMBMM * BMNBNN) * A21B11;
    dat[2 as libc::c_int as usize] = A32B22 * A21B11;
    scale = fabs(dat[0 as libc::c_int as usize]) + fabs(dat[1 as libc::c_int as usize])
        + fabs(dat[2 as libc::c_int as usize]);
    if scale != 0.0f64 {
        dat[0 as libc::c_int as usize] /= scale;
        dat[1 as libc::c_int as usize] /= scale;
        dat[2 as libc::c_int as usize] /= scale;
    }
    j = 0 as libc::c_int as size_t;
    while j < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
        r = if j.wrapping_add(4 as libc::c_int as libc::c_ulong) < N {
            j.wrapping_add(4 as libc::c_int as libc::c_ulong)
        } else {
            N
        };
        tau = gsl_linalg_householder_transform(&mut v3.vector);
        if tau != 0.0f64 {
            q = (if 0 as libc::c_int > j as libc::c_int - 1 as libc::c_int {
                0 as libc::c_int
            } else {
                j as libc::c_int - 1 as libc::c_int
            }) as size_t;
            if (*w).compute_s != 0 {
                m = gsl_matrix_submatrix(
                    (*w).H,
                    top.wrapping_add(j),
                    top.wrapping_add(q),
                    3 as libc::c_int as size_t,
                    ((*w).size).wrapping_sub(top).wrapping_sub(q),
                );
                gsl_linalg_householder_hm(tau, &mut v3.vector, &mut m.matrix);
            } else {
                m = gsl_matrix_submatrix(
                    H,
                    j,
                    q,
                    3 as libc::c_int as size_t,
                    N.wrapping_sub(q),
                );
                gsl_linalg_householder_hm(tau, &mut v3.vector, &mut m.matrix);
            }
            if (*w).compute_t != 0 {
                m = gsl_matrix_submatrix(
                    (*w).R,
                    top.wrapping_add(j),
                    top.wrapping_add(j),
                    3 as libc::c_int as size_t,
                    ((*w).size).wrapping_sub(top).wrapping_sub(j),
                );
                gsl_linalg_householder_hm(tau, &mut v3.vector, &mut m.matrix);
            } else {
                m = gsl_matrix_submatrix(
                    R,
                    j,
                    j,
                    3 as libc::c_int as size_t,
                    N.wrapping_sub(j),
                );
                gsl_linalg_householder_hm(tau, &mut v3.vector, &mut m.matrix);
            }
            if !((*w).Q).is_null() {
                m = gsl_matrix_submatrix(
                    (*w).Q,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(j),
                    (*w).size,
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v3.vector, &mut m.matrix);
            }
        }
        dat[0 as libc::c_int
            as usize] = gsl_matrix_get(
            R,
            j.wrapping_add(2 as libc::c_int as libc::c_ulong),
            j.wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        dat[1 as libc::c_int
            as usize] = gsl_matrix_get(
            R,
            j.wrapping_add(2 as libc::c_int as libc::c_ulong),
            j,
        );
        dat[2 as libc::c_int
            as usize] = gsl_matrix_get(
            R,
            j.wrapping_add(2 as libc::c_int as libc::c_ulong),
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        scale = fabs(dat[0 as libc::c_int as usize])
            + fabs(dat[1 as libc::c_int as usize])
            + fabs(dat[2 as libc::c_int as usize]);
        if scale != 0.0f64 {
            dat[0 as libc::c_int as usize] /= scale;
            dat[1 as libc::c_int as usize] /= scale;
            dat[2 as libc::c_int as usize] /= scale;
        }
        tau = gsl_linalg_householder_transform(&mut v3.vector);
        if tau != 0.0f64 {
            tmp = gsl_vector_get(&mut v3.vector, 1 as libc::c_int as size_t);
            gsl_vector_set(
                &mut v3.vector,
                1 as libc::c_int as size_t,
                gsl_vector_get(&mut v3.vector, 2 as libc::c_int as size_t) / tmp,
            );
            gsl_vector_set(&mut v3.vector, 2 as libc::c_int as size_t, 1.0f64 / tmp);
            tau *= tmp * tmp;
            if (*w).compute_s != 0 {
                m = gsl_matrix_submatrix(
                    (*w).H,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(j),
                    top.wrapping_add(r),
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v3.vector, &mut m.matrix);
            } else {
                m = gsl_matrix_submatrix(
                    H,
                    0 as libc::c_int as size_t,
                    j,
                    r,
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v3.vector, &mut m.matrix);
            }
            if (*w).compute_t != 0 {
                m = gsl_matrix_submatrix(
                    (*w).R,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(j),
                    top.wrapping_add(j).wrapping_add(3 as libc::c_int as libc::c_ulong),
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v3.vector, &mut m.matrix);
            } else {
                m = gsl_matrix_submatrix(
                    R,
                    0 as libc::c_int as size_t,
                    j,
                    j.wrapping_add(3 as libc::c_int as libc::c_ulong),
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v3.vector, &mut m.matrix);
            }
            if !((*w).Z).is_null() {
                m = gsl_matrix_submatrix(
                    (*w).Z,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(j),
                    (*w).size,
                    3 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v3.vector, &mut m.matrix);
            }
        }
        dat[0 as libc::c_int
            as usize] = gsl_matrix_get(
            R,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        dat[1 as libc::c_int
            as usize] = gsl_matrix_get(
            R,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            j,
        );
        scale = fabs(dat[0 as libc::c_int as usize])
            + fabs(dat[1 as libc::c_int as usize]);
        if scale != 0.0f64 {
            dat[0 as libc::c_int as usize] /= scale;
            dat[1 as libc::c_int as usize] /= scale;
        }
        tau = gsl_linalg_householder_transform(&mut v2.vector);
        if tau != 0.0f64 {
            tmp = gsl_vector_get(&mut v2.vector, 1 as libc::c_int as size_t);
            gsl_vector_set(&mut v2.vector, 1 as libc::c_int as size_t, 1.0f64 / tmp);
            tau *= tmp * tmp;
            if (*w).compute_s != 0 {
                m = gsl_matrix_submatrix(
                    (*w).H,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(j),
                    top.wrapping_add(r),
                    2 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
            } else {
                m = gsl_matrix_submatrix(
                    H,
                    0 as libc::c_int as size_t,
                    j,
                    r,
                    2 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
            }
            if (*w).compute_t != 0 {
                m = gsl_matrix_submatrix(
                    (*w).R,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(j),
                    top.wrapping_add(j).wrapping_add(3 as libc::c_int as libc::c_ulong),
                    2 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
            } else {
                m = gsl_matrix_submatrix(
                    R,
                    0 as libc::c_int as size_t,
                    j,
                    j.wrapping_add(3 as libc::c_int as libc::c_ulong),
                    2 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
            }
            if !((*w).Z).is_null() {
                m = gsl_matrix_submatrix(
                    (*w).Z,
                    0 as libc::c_int as size_t,
                    top.wrapping_add(j),
                    (*w).size,
                    2 as libc::c_int as size_t,
                );
                gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
            }
        }
        dat[0 as libc::c_int
            as usize] = gsl_matrix_get(
            H,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            j,
        );
        dat[1 as libc::c_int
            as usize] = gsl_matrix_get(
            H,
            j.wrapping_add(2 as libc::c_int as libc::c_ulong),
            j,
        );
        if j < N.wrapping_sub(3 as libc::c_int as libc::c_ulong) {
            dat[2 as libc::c_int
                as usize] = gsl_matrix_get(
                H,
                j.wrapping_add(3 as libc::c_int as libc::c_ulong),
                j,
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
        j = j.wrapping_add(1);
        j;
    }
    scale = fabs(dat[0 as libc::c_int as usize]) + fabs(dat[1 as libc::c_int as usize]);
    if scale != 0.0f64 {
        dat[0 as libc::c_int as usize] /= scale;
        dat[1 as libc::c_int as usize] /= scale;
    }
    tau = gsl_linalg_householder_transform(&mut v2.vector);
    if (*w).compute_s != 0 {
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
        gsl_linalg_householder_hm(tau, &mut v2.vector, &mut m.matrix);
    } else {
        m = gsl_matrix_submatrix(
            H,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(3 as libc::c_int as libc::c_ulong),
            2 as libc::c_int as size_t,
            3 as libc::c_int as size_t,
        );
        gsl_linalg_householder_hm(tau, &mut v2.vector, &mut m.matrix);
    }
    if (*w).compute_t != 0 {
        m = gsl_matrix_submatrix(
            (*w).R,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            2 as libc::c_int as size_t,
            ((*w).size)
                .wrapping_sub(top)
                .wrapping_sub(N)
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        gsl_linalg_householder_hm(tau, &mut v2.vector, &mut m.matrix);
    } else {
        m = gsl_matrix_submatrix(
            R,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            2 as libc::c_int as size_t,
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_hm(tau, &mut v2.vector, &mut m.matrix);
    }
    if !((*w).Q).is_null() {
        m = gsl_matrix_submatrix(
            (*w).Q,
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            (*w).size,
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
    }
    dat[0 as libc::c_int
        as usize] = gsl_matrix_get(
        R,
        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    dat[1 as libc::c_int
        as usize] = gsl_matrix_get(
        R,
        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    scale = fabs(dat[0 as libc::c_int as usize]) + fabs(dat[1 as libc::c_int as usize]);
    if scale != 0.0f64 {
        dat[0 as libc::c_int as usize] /= scale;
        dat[1 as libc::c_int as usize] /= scale;
    }
    tau = gsl_linalg_householder_transform(&mut v2.vector);
    tmp = gsl_vector_get(&mut v2.vector, 1 as libc::c_int as size_t);
    gsl_vector_set(&mut v2.vector, 1 as libc::c_int as size_t, 1.0f64 / tmp);
    tau *= tmp * tmp;
    if (*w).compute_s != 0 {
        m = gsl_matrix_submatrix(
            (*w).H,
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            top.wrapping_add(N),
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
    } else {
        m = gsl_matrix_submatrix(
            H,
            0 as libc::c_int as size_t,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N,
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
    }
    if (*w).compute_t != 0 {
        m = gsl_matrix_submatrix(
            (*w).R,
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            top.wrapping_add(N),
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
    } else {
        m = gsl_matrix_submatrix(
            R,
            0 as libc::c_int as size_t,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            N,
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
    }
    if !((*w).Z).is_null() {
        m = gsl_matrix_submatrix(
            (*w).Z,
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            (*w).size,
            2 as libc::c_int as size_t,
        );
        gsl_linalg_householder_mh(tau, &mut v2.vector, &mut m.matrix);
    }
}
unsafe extern "C" fn gen_tri_split_top(
    mut H: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    let N: size_t = (*H).size1;
    let mut j: size_t = 0;
    let mut top: size_t = 0 as libc::c_int as size_t;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
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
    if (*w).needtop != 0 {
        top = gen_get_submatrix((*w).H, H);
    }
    j = 0 as libc::c_int as size_t;
    gsl_linalg_givens(
        gsl_matrix_get(H, j, j),
        gsl_matrix_get(H, j.wrapping_add(1 as libc::c_int as libc::c_ulong), j),
        &mut cs,
        &mut sn,
    );
    sn = -sn;
    if (*w).compute_s != 0 {
        xv = gsl_matrix_subrow(
            (*w).H,
            top.wrapping_add(j),
            top,
            ((*w).size).wrapping_sub(top),
        );
        yv = gsl_matrix_subrow(
            (*w).H,
            top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            top,
            ((*w).size).wrapping_sub(top),
        );
    } else {
        xv = gsl_matrix_row(H, j);
        yv = gsl_matrix_row(H, j.wrapping_add(1 as libc::c_int as libc::c_ulong));
    }
    gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
    gsl_matrix_set(H, j.wrapping_add(1 as libc::c_int as libc::c_ulong), j, 0.0f64);
    if (*w).compute_t != 0 {
        xv = gsl_matrix_subrow(
            (*w).R,
            top.wrapping_add(j),
            top.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ((*w).size).wrapping_sub(top).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        yv = gsl_matrix_subrow(
            (*w).R,
            top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            top.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ((*w).size).wrapping_sub(top).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        xv = gsl_matrix_subrow(
            R,
            j,
            1 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        yv = gsl_matrix_subrow(
            R,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            1 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
    if !((*w).Q).is_null() {
        xv = gsl_matrix_column((*w).Q, top.wrapping_add(j));
        yv = gsl_matrix_column(
            (*w).Q,
            top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
    }
}
#[inline]
unsafe extern "C" fn gen_tri_chase_zero(
    mut H: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut q: size_t,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    let N: size_t = (*H).size1;
    let mut j: size_t = 0;
    let mut top: size_t = 0 as libc::c_int as size_t;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
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
    if (*w).needtop != 0 {
        top = gen_get_submatrix((*w).H, H);
    }
    j = q;
    while j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        gsl_linalg_givens(
            gsl_matrix_get(R, j, j.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            gsl_matrix_get(
                R,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
            &mut cs,
            &mut sn,
        );
        sn = -sn;
        if (*w).compute_t != 0 {
            xv = gsl_matrix_subrow(
                (*w).R,
                top.wrapping_add(j),
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ((*w).size)
                    .wrapping_sub(top)
                    .wrapping_sub(j)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subrow(
                (*w).R,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ((*w).size)
                    .wrapping_sub(top)
                    .wrapping_sub(j)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        } else {
            xv = gsl_matrix_subrow(
                R,
                j,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subrow(
                R,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        gsl_matrix_set(
            R,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            0.0f64,
        );
        if (*w).compute_s != 0 {
            xv = gsl_matrix_subrow(
                (*w).H,
                top.wrapping_add(j),
                top.wrapping_add(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ((*w).size)
                    .wrapping_sub(top)
                    .wrapping_sub(j)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subrow(
                (*w).H,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ((*w).size)
                    .wrapping_sub(top)
                    .wrapping_sub(j)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        } else {
            xv = gsl_matrix_subrow(
                H,
                j,
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subrow(
                H,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                N.wrapping_sub(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        if !((*w).Q).is_null() {
            xv = gsl_matrix_column((*w).Q, top.wrapping_add(j));
            yv = gsl_matrix_column(
                (*w).Q,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        }
        gsl_linalg_givens(
            gsl_matrix_get(H, j.wrapping_add(1 as libc::c_int as libc::c_ulong), j),
            gsl_matrix_get(
                H,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ),
            &mut cs,
            &mut sn,
        );
        sn = -sn;
        if (*w).compute_s != 0 {
            xv = gsl_matrix_subcolumn(
                (*w).H,
                top.wrapping_add(j),
                0 as libc::c_int as size_t,
                top.wrapping_add(j).wrapping_add(2 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subcolumn(
                (*w).H,
                top.wrapping_add(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                top.wrapping_add(j).wrapping_add(2 as libc::c_int as libc::c_ulong),
            );
        } else {
            xv = gsl_matrix_subcolumn(
                H,
                j,
                0 as libc::c_int as size_t,
                j.wrapping_add(2 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subcolumn(
                H,
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                j.wrapping_add(2 as libc::c_int as libc::c_ulong),
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        gsl_matrix_set(
            H,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            0.0f64,
        );
        if (*w).compute_t != 0 {
            xv = gsl_matrix_subcolumn(
                (*w).R,
                top.wrapping_add(j),
                0 as libc::c_int as size_t,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subcolumn(
                (*w).R,
                top.wrapping_add(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                top.wrapping_add(j).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        } else {
            xv = gsl_matrix_subcolumn(
                R,
                j,
                0 as libc::c_int as size_t,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subcolumn(
                R,
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        if !((*w).Z).is_null() {
            xv = gsl_matrix_column((*w).Z, top.wrapping_add(j));
            yv = gsl_matrix_column(
                (*w).Z,
                top.wrapping_add(j).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
        }
        j = j.wrapping_add(1);
        j;
    }
}
#[inline]
unsafe extern "C" fn gen_tri_zero_H(
    mut H: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    let N: size_t = (*H).size1;
    let mut top: size_t = 0 as libc::c_int as size_t;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
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
    if (*w).needtop != 0 {
        top = gen_get_submatrix((*w).H, H);
    }
    gsl_linalg_givens(
        gsl_matrix_get(
            H,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ),
        gsl_matrix_get(
            H,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        ),
        &mut cs,
        &mut sn,
    );
    sn = -sn;
    if (*w).compute_s != 0 {
        xv = gsl_matrix_subcolumn(
            (*w).H,
            top.wrapping_add(N).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            top.wrapping_add(N),
        );
        yv = gsl_matrix_subcolumn(
            (*w).H,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            top.wrapping_add(N),
        );
    } else {
        xv = gsl_matrix_column(H, N.wrapping_sub(1 as libc::c_int as libc::c_ulong));
        yv = gsl_matrix_column(H, N.wrapping_sub(2 as libc::c_int as libc::c_ulong));
    }
    gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
    gsl_matrix_set(
        H,
        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        0.0f64,
    );
    if (*w).compute_t != 0 {
        xv = gsl_matrix_subcolumn(
            (*w).R,
            top.wrapping_add(N).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        yv = gsl_matrix_subcolumn(
            (*w).R,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            top.wrapping_add(N).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        xv = gsl_matrix_subcolumn(
            R,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        yv = gsl_matrix_subcolumn(
            R,
            N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
    if !((*w).Z).is_null() {
        xv = gsl_matrix_column(
            (*w).Z,
            top.wrapping_add(N).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        yv = gsl_matrix_column(
            (*w).Z,
            top.wrapping_add(N).wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cs, sn);
    }
}
#[inline]
unsafe extern "C" fn gen_search_small_elements(
    mut H: *mut gsl_matrix,
    mut R: *mut gsl_matrix,
    mut flag: *mut libc::c_int,
    mut w: *mut gsl_eigen_gen_workspace,
) -> size_t {
    let N: size_t = (*H).size1;
    let mut k: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut pass1: libc::c_int = 0 as libc::c_int;
    let mut pass2: libc::c_int = 0 as libc::c_int;
    k = N as libc::c_int - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        i = k as size_t;
        if i != 0 as libc::c_int as libc::c_ulong
            && fabs(
                gsl_matrix_get(H, i, i.wrapping_sub(1 as libc::c_int as libc::c_ulong)),
            ) <= (*w).atol
        {
            gsl_matrix_set(
                H,
                i,
                i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0.0f64,
            );
            pass1 = 1 as libc::c_int;
        }
        if fabs(gsl_matrix_get(R, i, i)) < (*w).btol {
            gsl_matrix_set(R, i, i, 0.0f64);
            pass2 = 1 as libc::c_int;
        }
        if pass1 != 0 && pass2 == 0 {
            *flag = 1 as libc::c_int;
            return i;
        } else if pass1 == 0 && pass2 != 0 {
            *flag = 2 as libc::c_int;
            return i;
        } else if pass1 != 0 && pass2 != 0 {
            *flag = 3 as libc::c_int;
            return i;
        }
        k -= 1;
        k;
    }
    *flag = 0 as libc::c_int;
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn gen_schur_standardize1(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut alphar: *mut libc::c_double,
    mut beta: *mut libc::c_double,
    mut w: *mut gsl_eigen_gen_workspace,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut top: size_t = 0 as libc::c_int as size_t;
    if gsl_matrix_get(B, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t) < 0.0f64
    {
        if (*w).needtop != 0 {
            top = gen_get_submatrix((*w).H, A);
        }
        if (*w).compute_t != 0 {
            i = 0 as libc::c_int as size_t;
            while i <= top {
                gsl_matrix_set((*w).R, i, top, -gsl_matrix_get((*w).R, i, top));
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_matrix_set(
                B,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                -gsl_matrix_get(
                    B,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                ),
            );
        }
        if (*w).compute_s != 0 {
            i = 0 as libc::c_int as size_t;
            while i <= top {
                gsl_matrix_set((*w).H, i, top, -gsl_matrix_get((*w).H, i, top));
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_matrix_set(
                A,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                -gsl_matrix_get(
                    A,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                ),
            );
        }
        if !((*w).Z).is_null() {
            i = 0 as libc::c_int as size_t;
            while i < (*w).size {
                gsl_matrix_set((*w).Z, i, top, -gsl_matrix_get((*w).Z, i, top));
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    *alphar = gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    *beta = gsl_matrix_get(B, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gen_schur_standardize2(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut alpha1: *mut gsl_complex,
    mut alpha2: *mut gsl_complex,
    mut beta1: *mut libc::c_double,
    mut beta2: *mut libc::c_double,
    mut w: *mut gsl_eigen_gen_workspace,
) -> libc::c_int {
    let mut datB: [libc::c_double; 4] = [0.; 4];
    let mut datV: [libc::c_double; 4] = [0.; 4];
    let mut datS: [libc::c_double; 2] = [0.; 2];
    let mut work: [libc::c_double; 2] = [0.; 2];
    let mut uv: gsl_matrix_view = gsl_matrix_view_array(
        datB.as_mut_ptr(),
        2 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
    );
    let mut vv: gsl_matrix_view = gsl_matrix_view_array(
        datV.as_mut_ptr(),
        2 as libc::c_int as size_t,
        2 as libc::c_int as size_t,
    );
    let mut sv: gsl_vector_view = gsl_vector_view_array(
        datS.as_mut_ptr(),
        2 as libc::c_int as size_t,
    );
    let mut wv: gsl_vector_view = gsl_vector_view_array(
        work.as_mut_ptr(),
        2 as libc::c_int as size_t,
    );
    let mut B11: libc::c_double = 0.;
    let mut B22: libc::c_double = 0.;
    let mut top: size_t = 0 as libc::c_int as size_t;
    let mut det: libc::c_double = 0.;
    let mut cr: libc::c_double = 0.;
    let mut sr: libc::c_double = 0.;
    let mut cl: libc::c_double = 0.;
    let mut sl: libc::c_double = 0.;
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
    let mut s: libc::c_int = 0;
    if (*w).needtop != 0 {
        top = gen_get_submatrix((*w).H, A);
    }
    gsl_matrix_memcpy(&mut uv.matrix, B);
    gsl_linalg_SV_decomp(&mut uv.matrix, &mut vv.matrix, &mut sv.vector, &mut wv.vector);
    det = gsl_matrix_get(
        &mut vv.matrix,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    )
        * gsl_matrix_get(
            &mut vv.matrix,
            1 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
        )
        - gsl_matrix_get(
            &mut vv.matrix,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
        )
            * gsl_matrix_get(
                &mut vv.matrix,
                1 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
            );
    if det < 0.0f64 {
        datS[1 as libc::c_int as usize] = -datS[1 as libc::c_int as usize];
    }
    cr = gsl_matrix_get(
        &mut vv.matrix,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    sr = gsl_matrix_get(
        &mut vv.matrix,
        1 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    det = gsl_matrix_get(
        &mut uv.matrix,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    )
        * gsl_matrix_get(
            &mut uv.matrix,
            1 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
        )
        - gsl_matrix_get(
            &mut uv.matrix,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
        )
            * gsl_matrix_get(
                &mut uv.matrix,
                1 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
            );
    if det < 0.0f64 {
        datS[1 as libc::c_int as usize] = -datS[1 as libc::c_int as usize];
    }
    cl = gsl_matrix_get(
        &mut uv.matrix,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    sl = gsl_matrix_get(
        &mut uv.matrix,
        1 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    B11 = gsl_vector_get(&mut sv.vector, 0 as libc::c_int as size_t);
    B22 = gsl_vector_get(&mut sv.vector, 1 as libc::c_int as size_t);
    if B11 < 0.0f64 {
        B11 = -B11;
        B22 = -B22;
        cr = -cr;
        sr = -sr;
    }
    if (*w).compute_s != 0 {
        xv = gsl_matrix_subrow((*w).H, top, top, ((*w).size).wrapping_sub(top));
        yv = gsl_matrix_subrow(
            (*w).H,
            top.wrapping_add(1 as libc::c_int as libc::c_ulong),
            top,
            ((*w).size).wrapping_sub(top),
        );
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cl, sl);
        xv = gsl_matrix_subcolumn(
            (*w).H,
            top,
            0 as libc::c_int as size_t,
            top.wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        yv = gsl_matrix_subcolumn(
            (*w).H,
            top.wrapping_add(1 as libc::c_int as libc::c_ulong),
            0 as libc::c_int as size_t,
            top.wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cr, sr);
    } else {
        xv = gsl_matrix_row(A, 0 as libc::c_int as size_t);
        yv = gsl_matrix_row(A, 1 as libc::c_int as size_t);
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cl, sl);
        xv = gsl_matrix_column(A, 0 as libc::c_int as size_t);
        yv = gsl_matrix_column(A, 1 as libc::c_int as size_t);
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cr, sr);
    }
    if (*w).compute_t != 0 {
        if top != ((*w).size).wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            xv = gsl_matrix_subrow(
                (*w).R,
                top,
                top.wrapping_add(2 as libc::c_int as libc::c_ulong),
                ((*w).size)
                    .wrapping_sub(top)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            yv = gsl_matrix_subrow(
                (*w).R,
                top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                top.wrapping_add(2 as libc::c_int as libc::c_ulong),
                ((*w).size)
                    .wrapping_sub(top)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cl, sl);
        }
        if top != 0 as libc::c_int as libc::c_ulong {
            xv = gsl_matrix_subcolumn((*w).R, top, 0 as libc::c_int as size_t, top);
            yv = gsl_matrix_subcolumn(
                (*w).R,
                top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as libc::c_int as size_t,
                top,
            );
            gsl_blas_drot(&mut xv.vector, &mut yv.vector, cr, sr);
        }
    }
    if !((*w).Q).is_null() {
        xv = gsl_matrix_column((*w).Q, top);
        yv = gsl_matrix_column(
            (*w).Q,
            top.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cl, sl);
    }
    if !((*w).Z).is_null() {
        xv = gsl_matrix_column((*w).Z, top);
        yv = gsl_matrix_column(
            (*w).Z,
            top.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        gsl_blas_drot(&mut xv.vector, &mut yv.vector, cr, sr);
    }
    gsl_matrix_set(B, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t, B11);
    gsl_matrix_set(B, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t, 0.0f64);
    gsl_matrix_set(B, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t, 0.0f64);
    gsl_matrix_set(B, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t, B22);
    if B22 < 0.0f64 {
        let mut i: size_t = 0;
        if (*w).compute_s != 0 {
            i = 0 as libc::c_int as size_t;
            while i < top.wrapping_add(2 as libc::c_int as libc::c_ulong) {
                gsl_matrix_set(
                    (*w).H,
                    i,
                    top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    -gsl_matrix_get(
                        (*w).H,
                        i,
                        top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
                );
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_matrix_set(
                A,
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                -gsl_matrix_get(
                    A,
                    0 as libc::c_int as size_t,
                    1 as libc::c_int as size_t,
                ),
            );
            gsl_matrix_set(
                A,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                -gsl_matrix_get(
                    A,
                    1 as libc::c_int as size_t,
                    1 as libc::c_int as size_t,
                ),
            );
        }
        if (*w).compute_t != 0 {
            i = 0 as libc::c_int as size_t;
            while i < top.wrapping_add(2 as libc::c_int as libc::c_ulong) {
                gsl_matrix_set(
                    (*w).R,
                    i,
                    top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    -gsl_matrix_get(
                        (*w).R,
                        i,
                        top.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
                );
                i = i.wrapping_add(1);
                i;
            }
        } else {
            gsl_matrix_set(
                B,
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                -gsl_matrix_get(
                    B,
                    0 as libc::c_int as size_t,
                    1 as libc::c_int as size_t,
                ),
            );
            gsl_matrix_set(
                B,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                -gsl_matrix_get(
                    B,
                    1 as libc::c_int as size_t,
                    1 as libc::c_int as size_t,
                ),
            );
        }
        if !((*w).Z).is_null() {
            xv = gsl_matrix_column(
                (*w).Z,
                top.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_scale(&mut xv.vector, -1.0f64);
        }
    }
    s = gen_compute_eigenvals(A, B, alpha1, alpha2, beta1, beta2);
    return s;
}
unsafe extern "C" fn gen_compute_eigenvals(
    mut A: *mut gsl_matrix,
    mut B: *mut gsl_matrix,
    mut alpha1: *mut gsl_complex,
    mut alpha2: *mut gsl_complex,
    mut beta1: *mut libc::c_double,
    mut beta2: *mut libc::c_double,
) -> libc::c_int {
    let mut wr1: libc::c_double = 0.;
    let mut wr2: libc::c_double = 0.;
    let mut wi: libc::c_double = 0.;
    let mut scale1: libc::c_double = 0.;
    let mut scale2: libc::c_double = 0.;
    let mut s1inv: libc::c_double = 0.;
    let mut A11: libc::c_double = 0.;
    let mut A12: libc::c_double = 0.;
    let mut A21: libc::c_double = 0.;
    let mut A22: libc::c_double = 0.;
    let mut B11: libc::c_double = 0.;
    let mut B22: libc::c_double = 0.;
    let mut c11r: libc::c_double = 0.;
    let mut c11i: libc::c_double = 0.;
    let mut c12: libc::c_double = 0.;
    let mut c21: libc::c_double = 0.;
    let mut c22r: libc::c_double = 0.;
    let mut c22i: libc::c_double = 0.;
    let mut cz: libc::c_double = 0.;
    let mut cq: libc::c_double = 0.;
    let mut szr: libc::c_double = 0.;
    let mut szi: libc::c_double = 0.;
    let mut sqr: libc::c_double = 0.;
    let mut sqi: libc::c_double = 0.;
    let mut a1r: libc::c_double = 0.;
    let mut a1i: libc::c_double = 0.;
    let mut a2r: libc::c_double = 0.;
    let mut a2i: libc::c_double = 0.;
    let mut b1r: libc::c_double = 0.;
    let mut b1i: libc::c_double = 0.;
    let mut b1a: libc::c_double = 0.;
    let mut b2r: libc::c_double = 0.;
    let mut b2i: libc::c_double = 0.;
    let mut b2a: libc::c_double = 0.;
    let mut alphar: libc::c_double = 0.;
    let mut alphai: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut an: libc::c_double = 0.;
    let mut bn: libc::c_double = 0.;
    let mut tempr: libc::c_double = 0.;
    let mut tempi: libc::c_double = 0.;
    let mut wabs: libc::c_double = 0.;
    gsl_schur_gen_eigvals(A, B, &mut wr1, &mut wr2, &mut wi, &mut scale1, &mut scale2);
    if wi == 0.0f64 {
        return GSL_CONTINUE as libc::c_int;
    }
    s1inv = 1.0f64 / scale1;
    A11 = gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    A12 = gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    A21 = gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    A22 = gsl_matrix_get(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    B11 = gsl_matrix_get(B, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    B22 = gsl_matrix_get(B, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    c11r = scale1 * A11 - wr1 * B11;
    c11i = -wi * B11;
    c12 = scale1 * A12;
    c21 = scale1 * A21;
    c22r = scale1 * A22 - wr1 * B22;
    c22i = -wi * B22;
    if fabs(c11r) + fabs(c11i) + fabs(c12) > fabs(c21) + fabs(c22r) + fabs(c22i) {
        t1 = gsl_hypot3(c12, c11r, c11i);
        if t1 != 0.0f64 {
            cz = c12 / t1;
            szr = -c11r / t1;
            szi = -c11i / t1;
        } else {
            cz = 0.0f64;
            szr = 1.0f64;
            szi = 0.0f64;
        }
    } else {
        cz = hypot(c22r, c22i);
        if cz <= 2.2250738585072014e-308f64 {
            cz = 0.0f64;
            szr = 1.0f64;
            szi = 0.0f64;
        } else {
            tempr = c22r / cz;
            tempi = c22i / cz;
            t1 = hypot(cz, c21);
            cz /= t1;
            szr = -c21 * tempr / t1;
            szi = c21 * tempi / t1;
        }
    }
    an = fabs(A11) + fabs(A12) + fabs(A21) + fabs(A22);
    bn = fabs(B11) + fabs(B22);
    wabs = fabs(wr1) + fabs(wi);
    if scale1 * an > wabs * bn {
        cq = cz * B11;
        if cq <= 2.2250738585072014e-308f64 {
            cq = 0.0f64;
            sqr = 1.0f64;
            sqi = 0.0f64;
        } else {
            sqr = szr * B22;
            sqi = -szi * B22;
        }
    } else {
        a1r = cz * A11 + szr * A12;
        a1i = szi * A12;
        a2r = cz * A21 + szr * A22;
        a2i = szi * A22;
        cq = hypot(a1r, a1i);
        if cq <= 2.2250738585072014e-308f64 {
            cq = 0.0f64;
            sqr = 1.0f64;
            sqi = 0.0f64;
        } else {
            tempr = a1r / cq;
            tempi = a1i / cq;
            sqr = tempr * a2r + tempi * a2i;
            sqi = tempi * a2r - tempr * a2i;
        }
    }
    t1 = gsl_hypot3(cq, sqr, sqi);
    cq /= t1;
    sqr /= t1;
    sqi /= t1;
    tempr = sqr * szr - sqi * szi;
    tempi = sqr * szi + sqi * szr;
    b1r = cq * cz * B11 + tempr * B22;
    b1i = tempi * B22;
    b1a = hypot(b1r, b1i);
    b2r = cq * cz * B22 + tempr * B11;
    b2i = -tempi * B11;
    b2a = hypot(b2r, b2i);
    *beta1 = b1a;
    *beta2 = b2a;
    alphar = wr1 * b1a * s1inv;
    alphai = wi * b1a * s1inv;
    (*alpha1).dat[0 as libc::c_int as usize] = alphar;
    (*alpha1).dat[1 as libc::c_int as usize] = alphai;
    alphar = wr1 * b2a * s1inv;
    alphai = -(wi * b2a) * s1inv;
    (*alpha2).dat[0 as libc::c_int as usize] = alphar;
    (*alpha2).dat[1 as libc::c_int as usize] = alphai;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gen_store_eigval1(
    mut H: *const gsl_matrix,
    a: libc::c_double,
    b: libc::c_double,
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    let mut top: size_t = gen_get_submatrix((*w).H, H);
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a;
    z.dat[1 as libc::c_int as usize] = 0.0f64;
    gsl_vector_complex_set(alpha, top, z);
    gsl_vector_set(beta, top, b);
    (*w)
        .n_evals = ((*w).n_evals as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    (*w).n_iter = 0 as libc::c_int as size_t;
    (*w).eshift = 0.0f64;
}
unsafe extern "C" fn gen_store_eigval2(
    mut H: *const gsl_matrix,
    mut alpha1: *const gsl_complex,
    beta1: libc::c_double,
    mut alpha2: *const gsl_complex,
    beta2: libc::c_double,
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut w: *mut gsl_eigen_gen_workspace,
) {
    let mut top: size_t = gen_get_submatrix((*w).H, H);
    gsl_vector_complex_set(alpha, top, *alpha1);
    gsl_vector_set(beta, top, beta1);
    gsl_vector_complex_set(
        alpha,
        top.wrapping_add(1 as libc::c_int as libc::c_ulong),
        *alpha2,
    );
    gsl_vector_set(beta, top.wrapping_add(1 as libc::c_int as libc::c_ulong), beta2);
    (*w)
        .n_evals = ((*w).n_evals as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    (*w).n_iter = 0 as libc::c_int as size_t;
    (*w).eshift = 0.0f64;
}
#[inline]
unsafe extern "C" fn gen_get_submatrix(
    mut A: *const gsl_matrix,
    mut B: *const gsl_matrix,
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
#[inline]
unsafe extern "C" fn normF(mut A: *mut gsl_matrix) -> libc::c_double {
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
            if Aij != 0.0f64 {
                let mut ax: libc::c_double = fabs(Aij);
                if scale < ax {
                    ssq = 1.0f64 + ssq * (scale / ax) * (scale / ax);
                    scale = ax;
                } else {
                    ssq += ax / scale * (ax / scale);
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
