use ::libc;
extern "C" {
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_isnan(x: libc::c_double) -> libc::c_int;
    fn gsl_coerce_double(x: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_swap_elements(v: *mut gsl_vector, i: size_t, j: size_t) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_const_row(m: *const gsl_matrix, i: size_t) -> _gsl_vector_const_view;
    fn gsl_matrix_set_identity(m: *mut gsl_matrix);
    fn gsl_matrix_swap_columns(m: *mut gsl_matrix, i: size_t, j: size_t) -> libc::c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
    fn gsl_linalg_householder_hm(
        tau: libc::c_double,
        v: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_householder_hm1(
        tau: libc::c_double,
        A: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_bidiag_decomp(
        A: *mut gsl_matrix,
        tau_U: *mut gsl_vector,
        tau_V: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_bidiag_unpack2(
        A: *mut gsl_matrix,
        tau_U: *mut gsl_vector,
        tau_V: *mut gsl_vector,
        V: *mut gsl_matrix,
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
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
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
unsafe extern "C" fn chop_small_elements(
    mut d: *mut gsl_vector,
    mut f: *mut gsl_vector,
) {
    let N: size_t = (*d).size;
    let mut d_i: libc::c_double = gsl_vector_get(d, 0 as libc::c_int as size_t);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut f_i: libc::c_double = gsl_vector_get(f, i);
        let mut d_ip1: libc::c_double = gsl_vector_get(
            d,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if fabs(f_i) < 2.2204460492503131e-16f64 * (fabs(d_i) + fabs(d_ip1)) {
            gsl_vector_set(f, i, 0.0f64);
        }
        d_i = d_ip1;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn trailing_eigenvalue(
    mut d: *const gsl_vector,
    mut f: *const gsl_vector,
) -> libc::c_double {
    let n: size_t = (*d).size;
    let mut da: libc::c_double = gsl_vector_get(
        d,
        n.wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    let mut db: libc::c_double = gsl_vector_get(
        d,
        n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    let mut fa: libc::c_double = if n > 2 as libc::c_int as libc::c_ulong {
        gsl_vector_get(f, n.wrapping_sub(3 as libc::c_int as libc::c_ulong))
    } else {
        0.0f64
    };
    let mut fb: libc::c_double = gsl_vector_get(
        f,
        n.wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    let mut mu: libc::c_double = 0.;
    let mut ta: libc::c_double = da * da + fa * fa;
    let mut tb: libc::c_double = db * db + fb * fb;
    let mut tab: libc::c_double = da * fb;
    let mut dt: libc::c_double = (ta - tb) / 2.0f64;
    let mut S: libc::c_double = ta + tb;
    let mut da2: libc::c_double = da * da;
    let mut db2: libc::c_double = db * db;
    let mut fa2: libc::c_double = fa * fa;
    let mut fb2: libc::c_double = fb * fb;
    let mut P: libc::c_double = da2 * db2 + fa2 * db2 + fa2 * fb2;
    let mut D: libc::c_double = hypot(dt, tab);
    let mut r1: libc::c_double = S / 2 as libc::c_int as libc::c_double + D;
    if dt >= 0 as libc::c_int as libc::c_double {
        mu = if r1 > 0 as libc::c_int as libc::c_double { P / r1 } else { 0.0f64 };
    } else {
        mu = r1;
    }
    return mu;
}
unsafe extern "C" fn create_schur(
    mut d0: libc::c_double,
    mut f0: libc::c_double,
    mut d1: libc::c_double,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) {
    let mut apq: libc::c_double = 2.0f64 * d0 * f0;
    if d0 == 0 as libc::c_int as libc::c_double
        || f0 == 0 as libc::c_int as libc::c_double
    {
        *c = 1.0f64;
        *s = 0.0f64;
        return;
    }
    if fabs(d0) < 1.4916681462400413e-154f64 || fabs(d0) > 1.3407807929942596e+154f64
        || fabs(f0) < 1.4916681462400413e-154f64 || fabs(f0) > 1.3407807929942596e+154f64
        || fabs(d1) < 1.4916681462400413e-154f64 || fabs(d1) > 1.3407807929942596e+154f64
    {
        let mut scale: libc::c_double = 0.;
        let mut d0_exp: libc::c_int = 0;
        let mut f0_exp: libc::c_int = 0;
        frexp(d0, &mut d0_exp);
        frexp(f0, &mut f0_exp);
        scale = ldexp(1.0f64, -(d0_exp + f0_exp) / 4 as libc::c_int);
        d0 *= scale;
        f0 *= scale;
        d1 *= scale;
        apq = 2.0f64 * d0 * f0;
    }
    if apq != 0.0f64 {
        let mut t: libc::c_double = 0.;
        let mut tau: libc::c_double = (f0 * f0 + (d1 + d0) * (d1 - d0)) / apq;
        if tau >= 0.0f64 {
            t = 1.0f64 / (tau + hypot(1.0f64, tau));
        } else {
            t = -1.0f64 / (-tau + hypot(1.0f64, tau));
        }
        *c = 1.0f64 / hypot(1.0f64, t);
        *s = t * *c;
    } else {
        *c = 1.0f64;
        *s = 0.0f64;
    };
}
unsafe extern "C" fn svd2(
    mut d: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut U: *mut gsl_matrix,
    mut V: *mut gsl_matrix,
) {
    let mut i: size_t = 0;
    let mut c: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut a11: libc::c_double = 0.;
    let mut a12: libc::c_double = 0.;
    let mut a21: libc::c_double = 0.;
    let mut a22: libc::c_double = 0.;
    let M: size_t = (*U).size1;
    let N: size_t = (*V).size1;
    let mut d0: libc::c_double = gsl_vector_get(d, 0 as libc::c_int as size_t);
    let mut f0: libc::c_double = gsl_vector_get(f, 0 as libc::c_int as size_t);
    let mut d1: libc::c_double = gsl_vector_get(d, 1 as libc::c_int as size_t);
    if d0 == 0.0f64 {
        gsl_linalg_givens(f0, d1, &mut c, &mut s);
        gsl_vector_set(d, 0 as libc::c_int as size_t, c * f0 - s * d1);
        gsl_vector_set(f, 0 as libc::c_int as size_t, s * f0 + c * d1);
        gsl_vector_set(d, 1 as libc::c_int as size_t, 0.0f64);
        i = 0 as libc::c_int as size_t;
        while i < M {
            let mut Uip: libc::c_double = gsl_matrix_get(
                U,
                i,
                0 as libc::c_int as size_t,
            );
            let mut Uiq: libc::c_double = gsl_matrix_get(
                U,
                i,
                1 as libc::c_int as size_t,
            );
            gsl_matrix_set(U, i, 0 as libc::c_int as size_t, c * Uip - s * Uiq);
            gsl_matrix_set(U, i, 1 as libc::c_int as size_t, s * Uip + c * Uiq);
            i = i.wrapping_add(1);
            i;
        }
        gsl_matrix_swap_columns(
            V,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
        );
        return;
    } else if d1 == 0.0f64 {
        gsl_linalg_givens(d0, f0, &mut c, &mut s);
        gsl_vector_set(d, 0 as libc::c_int as size_t, d0 * c - f0 * s);
        gsl_vector_set(f, 0 as libc::c_int as size_t, 0.0f64);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut Vip: libc::c_double = gsl_matrix_get(
                V,
                i,
                0 as libc::c_int as size_t,
            );
            let mut Viq: libc::c_double = gsl_matrix_get(
                V,
                i,
                1 as libc::c_int as size_t,
            );
            gsl_matrix_set(V, i, 0 as libc::c_int as size_t, c * Vip - s * Viq);
            gsl_matrix_set(V, i, 1 as libc::c_int as size_t, s * Vip + c * Viq);
            i = i.wrapping_add(1);
            i;
        }
        return;
    } else {
        create_schur(d0, f0, d1, &mut c, &mut s);
        a11 = c * d0 - s * f0;
        a21 = -s * d1;
        a12 = s * d0 + c * f0;
        a22 = c * d1;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut Vip_0: libc::c_double = gsl_matrix_get(
                V,
                i,
                0 as libc::c_int as size_t,
            );
            let mut Viq_0: libc::c_double = gsl_matrix_get(
                V,
                i,
                1 as libc::c_int as size_t,
            );
            gsl_matrix_set(V, i, 0 as libc::c_int as size_t, c * Vip_0 - s * Viq_0);
            gsl_matrix_set(V, i, 1 as libc::c_int as size_t, s * Vip_0 + c * Viq_0);
            i = i.wrapping_add(1);
            i;
        }
        if hypot(a11, a21) < hypot(a12, a22) {
            let mut t1: libc::c_double = 0.;
            let mut t2: libc::c_double = 0.;
            t1 = a11;
            a11 = a12;
            a12 = t1;
            t2 = a21;
            a21 = a22;
            a22 = t2;
            gsl_matrix_swap_columns(
                V,
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
            );
        }
        gsl_linalg_givens(a11, a21, &mut c, &mut s);
        gsl_vector_set(d, 0 as libc::c_int as size_t, c * a11 - s * a21);
        gsl_vector_set(f, 0 as libc::c_int as size_t, c * a12 - s * a22);
        gsl_vector_set(d, 1 as libc::c_int as size_t, s * a12 + c * a22);
        i = 0 as libc::c_int as size_t;
        while i < M {
            let mut Uip_0: libc::c_double = gsl_matrix_get(
                U,
                i,
                0 as libc::c_int as size_t,
            );
            let mut Uiq_0: libc::c_double = gsl_matrix_get(
                U,
                i,
                1 as libc::c_int as size_t,
            );
            gsl_matrix_set(U, i, 0 as libc::c_int as size_t, c * Uip_0 - s * Uiq_0);
            gsl_matrix_set(U, i, 1 as libc::c_int as size_t, s * Uip_0 + c * Uiq_0);
            i = i.wrapping_add(1);
            i;
        }
        return;
    };
}
unsafe extern "C" fn chase_out_intermediate_zero(
    mut d: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut U: *mut gsl_matrix,
    mut k0: size_t,
) {
    let M: size_t = (*U).size1;
    let n: size_t = (*d).size;
    let mut c: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut k: size_t = 0;
    x = gsl_vector_get(f, k0);
    y = gsl_vector_get(d, k0.wrapping_add(1 as libc::c_int as libc::c_ulong));
    k = k0;
    while k < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        gsl_linalg_givens(y, -x, &mut c, &mut s);
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let mut Uip: libc::c_double = gsl_matrix_get(U, i, k0);
            let mut Uiq: libc::c_double = gsl_matrix_get(
                U,
                i,
                k.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_matrix_set(U, i, k0, c * Uip - s * Uiq);
            gsl_matrix_set(
                U,
                i,
                k.wrapping_add(1 as libc::c_int as libc::c_ulong),
                s * Uip + c * Uiq,
            );
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(
            d,
            k.wrapping_add(1 as libc::c_int as libc::c_ulong),
            s * x + c * y,
        );
        if k == k0 {
            gsl_vector_set(f, k, c * x - s * y);
        }
        if k < n.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            let mut z: libc::c_double = gsl_vector_get(
                f,
                k.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_set(f, k.wrapping_add(1 as libc::c_int as libc::c_ulong), c * z);
            x = -s * z;
            y = gsl_vector_get(d, k.wrapping_add(2 as libc::c_int as libc::c_ulong));
        }
        k = k.wrapping_add(1);
        k;
    }
}
unsafe extern "C" fn chase_out_trailing_zero(
    mut d: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut V: *mut gsl_matrix,
) {
    let N: size_t = (*V).size1;
    let n: size_t = (*d).size;
    let mut c: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut k: size_t = 0;
    x = gsl_vector_get(d, n.wrapping_sub(2 as libc::c_int as libc::c_ulong));
    y = gsl_vector_get(f, n.wrapping_sub(2 as libc::c_int as libc::c_ulong));
    k = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let fresh0 = k;
        k = k.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        gsl_linalg_givens(x, y, &mut c, &mut s);
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut Vip: libc::c_double = gsl_matrix_get(V, i, k);
            let mut Viq: libc::c_double = gsl_matrix_get(
                V,
                i,
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_matrix_set(V, i, k, c * Vip - s * Viq);
            gsl_matrix_set(
                V,
                i,
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                s * Vip + c * Viq,
            );
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_set(d, k, c * x - s * y);
        if k == n.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            gsl_vector_set(f, k, s * x + c * y);
        }
        if k > 0 as libc::c_int as libc::c_ulong {
            let mut z: libc::c_double = gsl_vector_get(
                f,
                k.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            gsl_vector_set(f, k.wrapping_sub(1 as libc::c_int as libc::c_ulong), c * z);
            x = gsl_vector_get(d, k.wrapping_sub(1 as libc::c_int as libc::c_ulong));
            y = s * z;
        }
    };
}
unsafe extern "C" fn qrstep(
    mut d: *mut gsl_vector,
    mut f: *mut gsl_vector,
    mut U: *mut gsl_matrix,
    mut V: *mut gsl_matrix,
) {
    let M: size_t = (*U).size1;
    let N: size_t = (*V).size1;
    let n: size_t = (*d).size;
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut ak: libc::c_double = 0.;
    let mut bk: libc::c_double = 0.;
    let mut zk: libc::c_double = 0.;
    let mut ap: libc::c_double = 0.;
    let mut bp: libc::c_double = 0.;
    let mut aq: libc::c_double = 0.;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    if n == 2 as libc::c_int as libc::c_ulong {
        svd2(d, f, U, V);
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut d_i: libc::c_double = gsl_vector_get(d, i);
        if d_i == 0.0f64 {
            chase_out_intermediate_zero(d, f, U, i);
            return;
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut d_nm1: libc::c_double = gsl_vector_get(
        d,
        n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if d_nm1 == 0.0f64 {
        chase_out_trailing_zero(d, f, V);
        return;
    }
    let mut d0: libc::c_double = gsl_vector_get(d, 0 as libc::c_int as size_t);
    let mut f0: libc::c_double = gsl_vector_get(f, 0 as libc::c_int as size_t);
    let mut d1: libc::c_double = gsl_vector_get(d, 1 as libc::c_int as size_t);
    let mut mu: libc::c_double = trailing_eigenvalue(d, f);
    y = d0 * d0 - mu;
    z = d0 * f0;
    ak = 0 as libc::c_int as libc::c_double;
    bk = 0 as libc::c_int as libc::c_double;
    ap = d0;
    bp = f0;
    aq = d1;
    k = 0 as libc::c_int as size_t;
    while k < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut c: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        gsl_linalg_givens(y, z, &mut c, &mut s);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut Vip: libc::c_double = gsl_matrix_get(V, i, k);
            let mut Viq: libc::c_double = gsl_matrix_get(
                V,
                i,
                k.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_matrix_set(V, i, k, c * Vip - s * Viq);
            gsl_matrix_set(
                V,
                i,
                k.wrapping_add(1 as libc::c_int as libc::c_ulong),
                s * Vip + c * Viq,
            );
            i = i.wrapping_add(1);
            i;
        }
        let mut bk1: libc::c_double = c * bk - s * z;
        let mut ap1: libc::c_double = c * ap - s * bp;
        let mut bp1: libc::c_double = s * ap + c * bp;
        let mut zp1: libc::c_double = -s * aq;
        let mut aq1: libc::c_double = c * aq;
        if k > 0 as libc::c_int as libc::c_ulong {
            gsl_vector_set(f, k.wrapping_sub(1 as libc::c_int as libc::c_ulong), bk1);
        }
        ak = ap1;
        bk = bp1;
        zk = zp1;
        ap = aq1;
        if k < n.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            bp = gsl_vector_get(f, k.wrapping_add(1 as libc::c_int as libc::c_ulong));
        } else {
            bp = 0.0f64;
        }
        y = ak;
        z = zk;
        gsl_linalg_givens(y, z, &mut c, &mut s);
        i = 0 as libc::c_int as size_t;
        while i < M {
            let mut Uip: libc::c_double = gsl_matrix_get(U, i, k);
            let mut Uiq: libc::c_double = gsl_matrix_get(
                U,
                i,
                k.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            gsl_matrix_set(U, i, k, c * Uip - s * Uiq);
            gsl_matrix_set(
                U,
                i,
                k.wrapping_add(1 as libc::c_int as libc::c_ulong),
                s * Uip + c * Uiq,
            );
            i = i.wrapping_add(1);
            i;
        }
        let mut ak1: libc::c_double = c * ak - s * zk;
        let mut bk1_0: libc::c_double = c * bk - s * ap;
        let mut zk1: libc::c_double = -s * bp;
        let mut ap1_0: libc::c_double = s * bk + c * ap;
        let mut bp1_0: libc::c_double = c * bp;
        gsl_vector_set(d, k, ak1);
        ak = ak1;
        bk = bk1_0;
        zk = zk1;
        ap = ap1_0;
        bp = bp1_0;
        if k < n.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            aq = gsl_vector_get(d, k.wrapping_add(2 as libc::c_int as libc::c_ulong));
        } else {
            aq = 0.0f64;
        }
        y = bk;
        z = zk;
        k = k.wrapping_add(1);
        k;
    }
    gsl_vector_set(f, n.wrapping_sub(2 as libc::c_int as libc::c_ulong), bk);
    gsl_vector_set(d, n.wrapping_sub(1 as libc::c_int as libc::c_ulong), ap);
}
pub unsafe extern "C" fn gsl_linalg_SV_decomp(
    mut A: *mut gsl_matrix,
    mut V: *mut gsl_matrix,
    mut S: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut a: size_t = 0;
    let mut b: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut iter: size_t = 0;
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    let K: size_t = if M < N { M } else { N };
    if M < N {
        gsl_error(
            b"svd of MxN matrix, M<N, is not implemented\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    } else if (*V).size1 != N {
        gsl_error(
            b"square matrix V must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*V).size1 != (*V).size2 {
        gsl_error(
            b"matrix V must be square\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*S).size != N {
        gsl_error(
            b"length of vector S must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != N {
        gsl_error(
            b"length of workspace must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if N == 1 as libc::c_int as libc::c_ulong {
        let mut column: gsl_vector_view = gsl_matrix_column(
            A,
            0 as libc::c_int as size_t,
        );
        let mut norm: libc::c_double = gsl_blas_dnrm2(&mut column.vector);
        gsl_vector_set(S, 0 as libc::c_int as size_t, norm);
        gsl_matrix_set(
            V,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            1.0f64,
        );
        if norm != 0.0f64 {
            gsl_blas_dscal(1.0f64 / norm, &mut column.vector);
        }
        return GSL_SUCCESS as libc::c_int;
    }
    let mut f: gsl_vector_view = gsl_vector_subvector(
        work,
        0 as libc::c_int as size_t,
        K.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    gsl_linalg_bidiag_decomp(A, S, &mut f.vector);
    gsl_linalg_bidiag_unpack2(A, S, &mut f.vector, V);
    chop_small_elements(S, &mut f.vector);
    b = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    iter = 0 as libc::c_int as size_t;
    while b > 0 as libc::c_int as libc::c_ulong {
        let mut fbm1: libc::c_double = gsl_vector_get(
            &mut f.vector,
            b.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        if fbm1 == 0.0f64 || gsl_isnan(fbm1) != 0 {
            b = b.wrapping_sub(1);
            b;
        } else {
            a = b.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            while a > 0 as libc::c_int as libc::c_ulong {
                let mut fam1: libc::c_double = gsl_vector_get(
                    &mut f.vector,
                    a.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if fam1 == 0.0f64 || gsl_isnan(fam1) != 0 {
                    break;
                }
                a = a.wrapping_sub(1);
                a;
            }
            iter = iter.wrapping_add(1);
            iter;
            if iter > (100 as libc::c_int as libc::c_ulong).wrapping_mul(N) {
                gsl_error(
                    b"SVD decomposition failed to converge\0" as *const u8
                        as *const libc::c_char,
                    b"svd.c\0" as *const u8 as *const libc::c_char,
                    148 as libc::c_int,
                    GSL_EMAXITER as libc::c_int,
                );
                return GSL_EMAXITER as libc::c_int;
            }
            let n_block: size_t = b
                .wrapping_sub(a)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut S_block: gsl_vector_view = gsl_vector_subvector(S, a, n_block);
            let mut f_block: gsl_vector_view = gsl_vector_subvector(
                &mut f.vector,
                a,
                n_block.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            let mut U_block: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                0 as libc::c_int as size_t,
                a,
                (*A).size1,
                n_block,
            );
            let mut V_block: gsl_matrix_view = gsl_matrix_submatrix(
                V,
                0 as libc::c_int as size_t,
                a,
                (*V).size1,
                n_block,
            );
            let mut rescale: libc::c_int = 0 as libc::c_int;
            let mut scale: libc::c_double = 1 as libc::c_int as libc::c_double;
            let mut norm_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            i = 0 as libc::c_int as size_t;
            while i < n_block {
                let mut s_i: libc::c_double = gsl_vector_get(&mut S_block.vector, i);
                let mut a_0: libc::c_double = fabs(s_i);
                if a_0 > norm_0 {
                    norm_0 = a_0;
                }
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < n_block.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                let mut f_i: libc::c_double = gsl_vector_get(&mut f_block.vector, i);
                let mut a_1: libc::c_double = fabs(f_i);
                if a_1 > norm_0 {
                    norm_0 = a_1;
                }
                i = i.wrapping_add(1);
                i;
            }
            if norm_0 > 1.3407807929942596e+154f64 {
                scale = norm_0 / 1.3407807929942596e+154f64;
                rescale = 1 as libc::c_int;
            } else if norm_0 < 1.4916681462400413e-154f64
                && norm_0 > 0 as libc::c_int as libc::c_double
            {
                scale = norm_0 / 1.4916681462400413e-154f64;
                rescale = 1 as libc::c_int;
            }
            if rescale != 0 {
                gsl_blas_dscal(1.0f64 / scale, &mut S_block.vector);
                gsl_blas_dscal(1.0f64 / scale, &mut f_block.vector);
            }
            qrstep(
                &mut S_block.vector,
                &mut f_block.vector,
                &mut U_block.matrix,
                &mut V_block.matrix,
            );
            chop_small_elements(&mut S_block.vector, &mut f_block.vector);
            if rescale != 0 {
                gsl_blas_dscal(scale, &mut S_block.vector);
                gsl_blas_dscal(scale, &mut f_block.vector);
            }
        }
    }
    j = 0 as libc::c_int as size_t;
    while j < K {
        let mut Sj: libc::c_double = gsl_vector_get(S, j);
        if Sj < 0.0f64 {
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut Vij: libc::c_double = gsl_matrix_get(V, i, j);
                gsl_matrix_set(V, i, j, -Vij);
                i = i.wrapping_add(1);
                i;
            }
            gsl_vector_set(S, j, -Sj);
        }
        j = j.wrapping_add(1);
        j;
    }
    i = 0 as libc::c_int as size_t;
    while i < K {
        let mut S_max: libc::c_double = gsl_vector_get(S, i);
        let mut i_max: size_t = i;
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < K {
            let mut Sj_0: libc::c_double = gsl_vector_get(S, j);
            if Sj_0 > S_max {
                S_max = Sj_0;
                i_max = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        if i_max != i {
            gsl_vector_swap_elements(S, i, i_max);
            gsl_matrix_swap_columns(A, i, i_max);
            gsl_matrix_swap_columns(V, i, i_max);
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_SV_decomp_mod(
    mut A: *mut gsl_matrix,
    mut X: *mut gsl_matrix,
    mut V: *mut gsl_matrix,
    mut S: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M < N {
        gsl_error(
            b"svd of MxN matrix, M<N, is not implemented\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    } else if (*V).size1 != N {
        gsl_error(
            b"square matrix V must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*V).size1 != (*V).size2 {
        gsl_error(
            b"matrix V must be square\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size1 != N {
        gsl_error(
            b"square matrix X must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*X).size1 != (*X).size2 {
        gsl_error(
            b"matrix X must be square\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*S).size != N {
        gsl_error(
            b"length of vector S must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != N {
        gsl_error(
            b"length of workspace must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    if N == 1 as libc::c_int as libc::c_ulong {
        let mut column: gsl_vector_view = gsl_matrix_column(
            A,
            0 as libc::c_int as size_t,
        );
        let mut norm: libc::c_double = gsl_blas_dnrm2(&mut column.vector);
        gsl_vector_set(S, 0 as libc::c_int as size_t, norm);
        gsl_matrix_set(
            V,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            1.0f64,
        );
        if norm != 0.0f64 {
            gsl_blas_dscal(1.0f64 / norm, &mut column.vector);
        }
        return GSL_SUCCESS as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < N {
        let mut c: gsl_vector_view = gsl_matrix_column(A, i);
        let mut v: gsl_vector_view = gsl_vector_subvector(
            &mut c.vector,
            i,
            M.wrapping_sub(i),
        );
        let mut tau_i: libc::c_double = gsl_linalg_householder_transform(&mut v.vector);
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
            let mut m: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                i,
                i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                M.wrapping_sub(i),
                N.wrapping_sub(i.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            gsl_linalg_householder_hm(tau_i, &mut v.vector, &mut m.matrix);
        }
        gsl_vector_set(S, i, tau_i);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < N {
        j = 0 as libc::c_int as size_t;
        while j < i {
            gsl_matrix_set(X, i, j, 0.0f64);
            j = j.wrapping_add(1);
            j;
        }
        let mut Aii: libc::c_double = gsl_matrix_get(A, i, i);
        gsl_matrix_set(X, i, i, Aii);
        j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < N {
            let mut Aij: libc::c_double = gsl_matrix_get(A, i, j);
            gsl_matrix_set(X, i, j, Aij);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    j = N;
    loop {
        let fresh1 = j;
        j = j.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut tj: libc::c_double = gsl_vector_get(S, j);
        let mut m_0: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            j,
            j,
            M.wrapping_sub(j),
            N.wrapping_sub(j),
        );
        gsl_linalg_householder_hm1(tj, &mut m_0.matrix);
    }
    gsl_linalg_SV_decomp(X, V, S, work);
    let mut sum: gsl_vector_view = gsl_vector_subvector(
        work,
        0 as libc::c_int as size_t,
        N,
    );
    i = 0 as libc::c_int as size_t;
    while i < M {
        let mut L_i: gsl_vector_view = gsl_matrix_row(A, i);
        gsl_vector_set_zero(&mut sum.vector);
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut Lij: libc::c_double = gsl_vector_get(&mut L_i.vector, j);
            let mut X_j: gsl_vector_view = gsl_matrix_row(X, j);
            gsl_blas_daxpy(Lij, &mut X_j.vector, &mut sum.vector);
            j = j.wrapping_add(1);
            j;
        }
        gsl_vector_memcpy(&mut L_i.vector, &mut sum.vector);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_SV_solve(
    mut U: *const gsl_matrix,
    mut V: *const gsl_matrix,
    mut S: *const gsl_vector,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> libc::c_int {
    if (*U).size1 != (*b).size {
        gsl_error(
            b"first dimension of matrix U must size of vector b\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            429 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*U).size2 != (*S).size {
        gsl_error(
            b"length of vector S must match second dimension of matrix U\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*V).size1 != (*V).size2 {
        gsl_error(
            b"matrix V must be square\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*S).size != (*V).size1 {
        gsl_error(
            b"length of vector S must match size of matrix V\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*V).size2 != (*x).size {
        gsl_error(
            b"size of matrix V must match size of vector x\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            447 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*U).size2;
        let mut i: size_t = 0;
        let mut w: *mut gsl_vector = gsl_vector_calloc(N);
        gsl_blas_dgemv(CblasTrans, 1.0f64, U, b, 0.0f64, w);
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut wi: libc::c_double = gsl_vector_get(w, i);
            let mut alpha: libc::c_double = gsl_vector_get(S, i);
            if alpha != 0 as libc::c_int as libc::c_double {
                alpha = 1.0f64 / alpha;
            }
            gsl_vector_set(w, i, alpha * wi);
            i = i.wrapping_add(1);
            i;
        }
        gsl_blas_dgemv(CblasNoTrans, 1.0f64, V, w, 0.0f64, x);
        gsl_vector_free(w);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_SV_leverage(
    mut U: *const gsl_matrix,
    mut h: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*U).size1;
    if M != (*h).size {
        gsl_error(
            b"first dimension of matrix U must match size of vector h\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            495 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let v: gsl_vector_const_view = gsl_matrix_const_row(U, i);
            let mut hi: libc::c_double = 0.;
            gsl_blas_ddot(&v.vector, &v.vector, &mut hi);
            gsl_vector_set(h, i, hi);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_SV_decomp_jacobi(
    mut A: *mut gsl_matrix,
    mut Q: *mut gsl_matrix,
    mut S: *mut gsl_vector,
) -> libc::c_int {
    if (*A).size1 < (*A).size2 {
        gsl_error(
            b"svd of MxN matrix, M<N, is not implemented\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    } else if (*Q).size1 != (*A).size2 {
        gsl_error(
            b"square matrix Q must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            547 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*Q).size1 != (*Q).size2 {
        gsl_error(
            b"matrix Q must be square\0" as *const u8 as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            551 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*S).size != (*A).size2 {
        gsl_error(
            b"length of vector S must match second dimension of matrix A\0" as *const u8
                as *const libc::c_char,
            b"svd.c\0" as *const u8 as *const libc::c_char,
            556 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let M: size_t = (*A).size1;
        let N: size_t = (*A).size2;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        let mut count: libc::c_int = 1 as libc::c_int;
        let mut sweep: libc::c_int = 0 as libc::c_int;
        let mut sweepmax: libc::c_int = (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(N) as libc::c_int;
        let mut tolerance: libc::c_double = (10 as libc::c_int as libc::c_ulong)
            .wrapping_mul(M) as libc::c_double * 2.2204460492503131e-16f64;
        sweepmax = if sweepmax > 12 as libc::c_int {
            sweepmax
        } else {
            12 as libc::c_int
        };
        gsl_matrix_set_identity(Q);
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut cj: gsl_vector_view = gsl_matrix_column(A, j);
            let mut sj: libc::c_double = gsl_blas_dnrm2(&mut cj.vector);
            gsl_vector_set(S, j, 2.2204460492503131e-16f64 * sj);
            j = j.wrapping_add(1);
            j;
        }
        while count > 0 as libc::c_int && sweep <= sweepmax {
            count = N
                .wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
            j = 0 as libc::c_int as size_t;
            while j < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                k = j.wrapping_add(1 as libc::c_int as libc::c_ulong);
                while k < N {
                    let mut a: libc::c_double = 0.0f64;
                    let mut b: libc::c_double = 0.0f64;
                    let mut p: libc::c_double = 0.0f64;
                    let mut q: libc::c_double = 0.0f64;
                    let mut cosine: libc::c_double = 0.;
                    let mut sine: libc::c_double = 0.;
                    let mut v: libc::c_double = 0.;
                    let mut abserr_a: libc::c_double = 0.;
                    let mut abserr_b: libc::c_double = 0.;
                    let mut sorted: libc::c_int = 0;
                    let mut orthog: libc::c_int = 0;
                    let mut noisya: libc::c_int = 0;
                    let mut noisyb: libc::c_int = 0;
                    let mut cj_0: gsl_vector_view = gsl_matrix_column(A, j);
                    let mut ck: gsl_vector_view = gsl_matrix_column(A, k);
                    gsl_blas_ddot(&mut cj_0.vector, &mut ck.vector, &mut p);
                    p *= 2.0f64;
                    a = gsl_blas_dnrm2(&mut cj_0.vector);
                    b = gsl_blas_dnrm2(&mut ck.vector);
                    q = a * a - b * b;
                    v = hypot(p, q);
                    abserr_a = gsl_vector_get(S, j);
                    abserr_b = gsl_vector_get(S, k);
                    sorted = (gsl_coerce_double(a) >= gsl_coerce_double(b))
                        as libc::c_int;
                    orthog = (fabs(p) <= tolerance * gsl_coerce_double(a * b))
                        as libc::c_int;
                    noisya = (a < abserr_a) as libc::c_int;
                    noisyb = (b < abserr_b) as libc::c_int;
                    if sorted != 0 && (orthog != 0 || noisya != 0 || noisyb != 0) {
                        count -= 1;
                        count;
                    } else {
                        if v == 0 as libc::c_int as libc::c_double || sorted == 0 {
                            cosine = 0.0f64;
                            sine = 1.0f64;
                        } else {
                            cosine = sqrt((v + q) / (2.0f64 * v));
                            sine = p / (2.0f64 * v * cosine);
                        }
                        i = 0 as libc::c_int as size_t;
                        while i < M {
                            let Aik: libc::c_double = gsl_matrix_get(A, i, k);
                            let Aij: libc::c_double = gsl_matrix_get(A, i, j);
                            gsl_matrix_set(A, i, j, Aij * cosine + Aik * sine);
                            gsl_matrix_set(A, i, k, -Aij * sine + Aik * cosine);
                            i = i.wrapping_add(1);
                            i;
                        }
                        gsl_vector_set(
                            S,
                            j,
                            fabs(cosine) * abserr_a + fabs(sine) * abserr_b,
                        );
                        gsl_vector_set(
                            S,
                            k,
                            fabs(sine) * abserr_a + fabs(cosine) * abserr_b,
                        );
                        i = 0 as libc::c_int as size_t;
                        while i < N {
                            let Qij: libc::c_double = gsl_matrix_get(Q, i, j);
                            let Qik: libc::c_double = gsl_matrix_get(Q, i, k);
                            gsl_matrix_set(Q, i, j, Qij * cosine + Qik * sine);
                            gsl_matrix_set(Q, i, k, -Qij * sine + Qik * cosine);
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            sweep += 1;
            sweep;
        }
        let mut prev_norm: libc::c_double = -1.0f64;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let mut column: gsl_vector_view = gsl_matrix_column(A, j);
            let mut norm: libc::c_double = gsl_blas_dnrm2(&mut column.vector);
            if norm == 0.0f64 || prev_norm == 0.0f64
                || j > 0 as libc::c_int as libc::c_ulong && norm <= tolerance * prev_norm
            {
                gsl_vector_set(S, j, 0.0f64);
                gsl_vector_set_zero(&mut column.vector);
                prev_norm = 0.0f64;
            } else {
                gsl_vector_set(S, j, norm);
                gsl_vector_scale(&mut column.vector, 1.0f64 / norm);
                prev_norm = norm;
            }
            j = j.wrapping_add(1);
            j;
        }
        if count > 0 as libc::c_int {
            gsl_error(
                b"Jacobi iterations did not reach desired tolerance\0" as *const u8
                    as *const libc::c_char,
                b"svd.c\0" as *const u8 as *const libc::c_char,
                712 as libc::c_int,
                GSL_ETOL as libc::c_int,
            );
            return GSL_ETOL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
