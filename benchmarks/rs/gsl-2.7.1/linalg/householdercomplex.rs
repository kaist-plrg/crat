use ::libc;
extern "C" {
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_complex_subvector(
        base: *mut gsl_vector_complex,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_vector_complex_const_subvector(
        base: *const gsl_vector_complex,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_complex_const_view;
    fn gsl_blas_zdotc(
        X: *const gsl_vector_complex,
        Y: *const gsl_vector_complex,
        dotc: *mut gsl_complex,
    ) -> libc::c_int;
    fn gsl_blas_dznrm2(X: *const gsl_vector_complex) -> libc::c_double;
    fn gsl_blas_zaxpy(
        alpha: gsl_complex,
        X: *const gsl_vector_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zscal(alpha: gsl_complex, X: *mut gsl_vector_complex);
    fn gsl_blas_zgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: gsl_complex,
        A: *const gsl_matrix_complex,
        X: *const gsl_vector_complex,
        beta: gsl_complex,
        Y: *mut gsl_vector_complex,
    ) -> libc::c_int;
    fn gsl_blas_zgerc(
        alpha: gsl_complex,
        X: *const gsl_vector_complex,
        Y: *const gsl_vector_complex,
        A: *mut gsl_matrix_complex,
    ) -> libc::c_int;
    fn gsl_complex_abs(z: gsl_complex) -> libc::c_double;
    fn gsl_complex_add(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_sub(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_mul(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_sub_real(a: gsl_complex, x: libc::c_double) -> gsl_complex;
    fn gsl_complex_conjugate(z: gsl_complex) -> gsl_complex;
    fn gsl_complex_inverse(a: gsl_complex) -> gsl_complex;
    fn gsl_complex_negative(a: gsl_complex) -> gsl_complex;
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
pub struct _gsl_vector_complex_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_view = _gsl_vector_complex_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_complex_const_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_const_view = _gsl_vector_complex_const_view;
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
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
unsafe extern "C" fn gsl_matrix_complex_get(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> gsl_complex {
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_matrix_complex_set(
    mut m: *mut gsl_matrix_complex,
    i: size_t,
    j: size_t,
    x: gsl_complex,
) {
    *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex) = x;
}
#[inline]
unsafe extern "C" fn gsl_complex_rect(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = x;
    z.dat[1 as libc::c_int as usize] = y;
    return z;
}
pub unsafe extern "C" fn gsl_linalg_complex_householder_transform(
    mut v: *mut gsl_vector_complex,
) -> gsl_complex {
    let n: size_t = (*v).size;
    if n == 1 as libc::c_int as libc::c_ulong {
        let mut alpha: gsl_complex = gsl_vector_complex_get(
            v,
            0 as libc::c_int as size_t,
        );
        let mut absa: libc::c_double = gsl_complex_abs(alpha);
        let mut beta_r: libc::c_double = -(if alpha.dat[0 as libc::c_int as usize]
            >= 0 as libc::c_int as libc::c_double
        {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_double * absa;
        let mut tau: gsl_complex = gsl_complex { dat: [0.; 2] };
        if beta_r == 0.0f64 {
            tau.dat[0 as libc::c_int as usize] = 0.0f64;
            tau.dat[1 as libc::c_int as usize] = 0.0f64;
        } else {
            tau
                .dat[0 as libc::c_int
                as usize] = (beta_r - alpha.dat[0 as libc::c_int as usize]) / beta_r;
            tau
                .dat[1 as libc::c_int
                as usize] = -alpha.dat[1 as libc::c_int as usize] / beta_r;
            let mut beta: gsl_complex = gsl_complex_rect(beta_r, 0.0f64);
            gsl_vector_complex_set(v, 0 as libc::c_int as size_t, beta);
        }
        return tau;
    } else {
        let mut tau_0: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut beta_r_0: libc::c_double = 0.;
        let mut x: gsl_vector_complex_view = gsl_vector_complex_subvector(
            v,
            1 as libc::c_int as size_t,
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut alpha_0: gsl_complex = gsl_vector_complex_get(
            v,
            0 as libc::c_int as size_t,
        );
        let mut absa_0: libc::c_double = gsl_complex_abs(alpha_0);
        let mut xnorm: libc::c_double = gsl_blas_dznrm2(&mut x.vector);
        if xnorm == 0 as libc::c_int as libc::c_double
            && alpha_0.dat[1 as libc::c_int as usize]
                == 0 as libc::c_int as libc::c_double
        {
            let mut zero: gsl_complex = gsl_complex_rect(0.0f64, 0.0f64);
            return zero;
        }
        beta_r_0 = -(if alpha_0.dat[0 as libc::c_int as usize]
            >= 0 as libc::c_int as libc::c_double
        {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_double * hypot(absa_0, xnorm);
        tau_0
            .dat[0 as libc::c_int
            as usize] = (beta_r_0 - alpha_0.dat[0 as libc::c_int as usize]) / beta_r_0;
        tau_0
            .dat[1 as libc::c_int
            as usize] = -alpha_0.dat[1 as libc::c_int as usize] / beta_r_0;
        let mut amb: gsl_complex = gsl_complex_sub_real(alpha_0, beta_r_0);
        let mut s: gsl_complex = gsl_complex_inverse(amb);
        gsl_blas_zscal(s, &mut x.vector);
        let mut beta_0: gsl_complex = gsl_complex_rect(beta_r_0, 0.0f64);
        gsl_vector_complex_set(v, 0 as libc::c_int as size_t, beta_0);
        return tau_0;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_householder_hv(
    mut tau: gsl_complex,
    mut v: *const gsl_vector_complex,
    mut w: *mut gsl_vector_complex,
) -> libc::c_int {
    let N: size_t = (*v).size;
    if tau.dat[0 as libc::c_int as usize] == 0.0f64
        && tau.dat[1 as libc::c_int as usize] == 0.0f64
    {
        return GSL_SUCCESS as libc::c_int;
    }
    if N == 1 as libc::c_int as libc::c_ulong {
        let mut w0: gsl_complex = gsl_vector_complex_get(w, 0 as libc::c_int as size_t);
        let mut a: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut b: gsl_complex = gsl_complex { dat: [0.; 2] };
        a.dat[0 as libc::c_int as usize] = 1.0f64 - tau.dat[0 as libc::c_int as usize];
        a.dat[1 as libc::c_int as usize] = -tau.dat[1 as libc::c_int as usize];
        b = gsl_complex_mul(a, w0);
        gsl_vector_complex_set(w, 0 as libc::c_int as size_t, b);
    } else {
        let mut z0: gsl_complex = gsl_vector_complex_get(w, 0 as libc::c_int as size_t);
        let mut z1: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut tz: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut ntz: gsl_complex = gsl_complex { dat: [0.; 2] };
        let v1: gsl_vector_complex_const_view = gsl_vector_complex_const_subvector(
            v,
            1 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut w1: gsl_vector_complex_view = gsl_vector_complex_subvector(
            w,
            1 as libc::c_int as size_t,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        gsl_blas_zdotc(&v1.vector, &mut w1.vector, &mut z1);
        z = gsl_complex_add(z0, z1);
        tz = gsl_complex_mul(tau, z);
        ntz = gsl_complex_negative(tz);
        let mut w0_0: gsl_complex = gsl_vector_complex_get(
            w,
            0 as libc::c_int as size_t,
        );
        let mut w0ntz: gsl_complex = gsl_complex_add(w0_0, ntz);
        gsl_vector_complex_set(w, 0 as libc::c_int as size_t, w0ntz);
        gsl_blas_zaxpy(ntz, &v1.vector, &mut w1.vector);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_complex_householder_left(
    tau: gsl_complex,
    mut v: *const gsl_vector_complex,
    mut A: *mut gsl_matrix_complex,
    mut work: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*v).size != M {
        gsl_error(
            b"matrix must match Householder vector dimensions\0" as *const u8
                as *const libc::c_char,
            b"householdercomplex.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != N {
        gsl_error(
            b"workspace must match matrix\0" as *const u8 as *const libc::c_char,
            b"householdercomplex.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut v0: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut mtau: gsl_complex = gsl_complex { dat: [0.; 2] };
        if tau.dat[0 as libc::c_int as usize] == 0.0f64
            && tau.dat[1 as libc::c_int as usize] == 0.0f64
        {
            return GSL_SUCCESS as libc::c_int;
        }
        v0 = gsl_vector_complex_get(v, 0 as libc::c_int as size_t);
        *((*v).data).offset(0 as libc::c_int as isize) = 1.0f64;
        *((*v).data).offset(1 as libc::c_int as isize) = 0.0f64;
        gsl_blas_zgemv(
            CblasConjTrans,
            gsl_complex_rect(1.0f64, 0.0f64),
            A,
            v,
            gsl_complex_rect(0.0f64, 0.0f64),
            work,
        );
        mtau.dat[0 as libc::c_int as usize] = -tau.dat[0 as libc::c_int as usize];
        mtau.dat[1 as libc::c_int as usize] = -tau.dat[1 as libc::c_int as usize];
        gsl_blas_zgerc(mtau, v, work, A);
        *((*v).data)
            .offset(0 as libc::c_int as isize) = v0.dat[0 as libc::c_int as usize];
        *((*v).data)
            .offset(1 as libc::c_int as isize) = v0.dat[1 as libc::c_int as usize];
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_complex_householder_hm(
    mut tau: gsl_complex,
    mut v: *const gsl_vector_complex,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if tau.dat[0 as libc::c_int as usize] == 0.0f64
        && tau.dat[1 as libc::c_int as usize] == 0.0f64
    {
        return GSL_SUCCESS as libc::c_int;
    }
    j = 0 as libc::c_int as size_t;
    while j < (*A).size2 {
        let mut tauwj: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut wj: gsl_complex = gsl_matrix_complex_get(
            A,
            0 as libc::c_int as size_t,
            j,
        );
        i = 1 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut Aij: gsl_complex = gsl_matrix_complex_get(A, i, j);
            let mut vi: gsl_complex = gsl_vector_complex_get(v, i);
            let mut Av: gsl_complex = gsl_complex_mul(Aij, gsl_complex_conjugate(vi));
            wj = gsl_complex_add(wj, Av);
            i = i.wrapping_add(1);
            i;
        }
        tauwj = gsl_complex_mul(tau, wj);
        let mut A0j: gsl_complex = gsl_matrix_complex_get(
            A,
            0 as libc::c_int as size_t,
            j,
        );
        let mut Atw: gsl_complex = gsl_complex_sub(A0j, tauwj);
        gsl_matrix_complex_set(A, 0 as libc::c_int as size_t, j, Atw);
        i = 1 as libc::c_int as size_t;
        while i < (*A).size1 {
            let mut vi_0: gsl_complex = gsl_vector_complex_get(v, i);
            let mut tauvw: gsl_complex = gsl_complex_mul(vi_0, tauwj);
            let mut Aij_0: gsl_complex = gsl_matrix_complex_get(A, i, j);
            let mut Atwv: gsl_complex = gsl_complex_sub(Aij_0, tauvw);
            gsl_matrix_complex_set(A, i, j, Atwv);
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_linalg_complex_householder_mh(
    mut tau: gsl_complex,
    mut v: *const gsl_vector_complex,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if tau.dat[0 as libc::c_int as usize] == 0.0f64
        && tau.dat[1 as libc::c_int as usize] == 0.0f64
    {
        return GSL_SUCCESS as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*A).size1 {
        let mut tauwi: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut Ai0: gsl_complex = gsl_matrix_complex_get(
            A,
            i,
            0 as libc::c_int as size_t,
        );
        let mut wi: gsl_complex = Ai0;
        j = 1 as libc::c_int as size_t;
        while j < (*A).size2 {
            let mut Aij: gsl_complex = gsl_matrix_complex_get(A, i, j);
            let mut vj: gsl_complex = gsl_vector_complex_get(v, j);
            let mut Av: gsl_complex = gsl_complex_mul(Aij, vj);
            wi = gsl_complex_add(wi, Av);
            j = j.wrapping_add(1);
            j;
        }
        tauwi = gsl_complex_mul(tau, wi);
        let mut Atw: gsl_complex = gsl_complex_sub(Ai0, tauwi);
        gsl_matrix_complex_set(A, i, 0 as libc::c_int as size_t, Atw);
        j = 1 as libc::c_int as size_t;
        while j < (*A).size2 {
            let mut vj_0: gsl_complex = gsl_vector_complex_get(v, j);
            let mut tauwv: gsl_complex = gsl_complex_mul(
                gsl_complex_conjugate(vj_0),
                tauwi,
            );
            let mut Aij_0: gsl_complex = gsl_matrix_complex_get(A, i, j);
            let mut Atwv: gsl_complex = gsl_complex_sub(Aij_0, tauwv);
            gsl_matrix_complex_set(A, i, j, Atwv);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
