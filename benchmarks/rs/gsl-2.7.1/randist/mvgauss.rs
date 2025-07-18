use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_add(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_matrix_const_column(
        m: *const gsl_matrix,
        j: size_t,
    ) -> _gsl_vector_const_view;
    fn gsl_ran_ugaussian(r: *const gsl_rng) -> libc::c_double;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut libc::c_double,
    ) -> libc::c_int;
    fn gsl_blas_dtrmv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_stats_mean(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_variance(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_covariance(
        data1: *const libc::c_double,
        stride1: size_t,
        data2: *const libc::c_double,
        stride2: size_t,
        n: size_t,
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
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
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
pub struct _gsl_vector_const_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_const_view = _gsl_vector_const_view;
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
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
pub unsafe extern "C" fn gsl_ran_multivariate_gaussian(
    mut r: *const gsl_rng,
    mut mu: *const gsl_vector,
    mut L: *const gsl_matrix,
    mut result: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*L).size1;
    let N: size_t = (*L).size2;
    if M != N {
        gsl_error(
            b"requires square matrix\0" as *const u8 as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*mu).size != M {
        gsl_error(
            b"incompatible dimension of mean vector with variance-covariance matrix\0"
                as *const u8 as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*result).size != M {
        gsl_error(
            b"incompatible dimension of result vector\0" as *const u8
                as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < M {
            gsl_vector_set(result, i, gsl_ran_ugaussian(r));
            i = i.wrapping_add(1);
            i;
        }
        gsl_blas_dtrmv(CblasLower, CblasNoTrans, CblasNonUnit, L, result);
        gsl_vector_add(result, mu);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_ran_multivariate_gaussian_log_pdf(
    mut x: *const gsl_vector,
    mut mu: *const gsl_vector,
    mut L: *const gsl_matrix,
    mut result: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*L).size1;
    let N: size_t = (*L).size2;
    if M != N {
        gsl_error(
            b"requires square matrix\0" as *const u8 as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*mu).size != M {
        gsl_error(
            b"incompatible dimension of mean vector with variance-covariance matrix\0"
                as *const u8 as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*x).size != M {
        gsl_error(
            b"incompatible dimension of quantile vector\0" as *const u8
                as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size != M {
        gsl_error(
            b"incompatible dimension of work vector\0" as *const u8
                as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut quadForm: libc::c_double = 0.;
        let mut logSqrtDetSigma: libc::c_double = 0.;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let mut xi: libc::c_double = gsl_vector_get(x, i);
            let mut mui: libc::c_double = gsl_vector_get(mu, i);
            gsl_vector_set(work, i, xi - mui);
            i = i.wrapping_add(1);
            i;
        }
        gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasNonUnit, L, work);
        gsl_blas_ddot(work, work, &mut quadForm);
        logSqrtDetSigma = 0.0f64;
        i = 0 as libc::c_int as size_t;
        while i < M {
            let mut Lii: libc::c_double = gsl_matrix_get(L, i, i);
            logSqrtDetSigma += log(Lii);
            i = i.wrapping_add(1);
            i;
        }
        *result = -0.5f64 * quadForm - logSqrtDetSigma
            - 0.5f64 * M as libc::c_double * log(2.0f64 * 3.14159265358979323846f64);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_ran_multivariate_gaussian_pdf(
    mut x: *const gsl_vector,
    mut mu: *const gsl_vector,
    mut L: *const gsl_matrix,
    mut result: *mut libc::c_double,
    mut work: *mut gsl_vector,
) -> libc::c_int {
    let mut logpdf: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_ran_multivariate_gaussian_log_pdf(
        x,
        mu,
        L,
        &mut logpdf,
        work,
    );
    if status == GSL_SUCCESS as libc::c_int {
        *result = exp(logpdf);
    }
    return status;
}
pub unsafe extern "C" fn gsl_ran_multivariate_gaussian_mean(
    mut X: *const gsl_matrix,
    mut mu_hat: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*X).size1;
    let N: size_t = (*X).size2;
    if N != (*mu_hat).size {
        gsl_error(
            b"mu_hat vector has wrong size\0" as *const u8 as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < N {
            let c: gsl_vector_const_view = gsl_matrix_const_column(X, j);
            let mut mean: libc::c_double = gsl_stats_mean(
                c.vector.data as *const libc::c_double,
                c.vector.stride,
                M,
            );
            gsl_vector_set(mu_hat, j, mean);
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_ran_multivariate_gaussian_vcov(
    mut X: *const gsl_matrix,
    mut sigma_hat: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*X).size1;
    let N: size_t = (*X).size2;
    if (*sigma_hat).size1 != (*sigma_hat).size2 {
        gsl_error(
            b"sigma_hat must be a square matrix\0" as *const u8 as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*sigma_hat).size1 {
        gsl_error(
            b"sigma_hat does not match X matrix dimensions\0" as *const u8
                as *const libc::c_char,
            b"mvgauss.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        return multivar_vcov(
            (*X).data as *const libc::c_double,
            N,
            (*X).tda,
            M,
            (*sigma_hat).data,
            (*sigma_hat).tda,
        )
    };
}
unsafe extern "C" fn multivar_vcov(
    mut data: *const libc::c_double,
    mut d: size_t,
    mut tda: size_t,
    mut n: size_t,
    mut vcov: *mut libc::c_double,
    mut tda2: size_t,
) -> libc::c_int {
    let mut j1: size_t = 0 as libc::c_int as size_t;
    let mut j2: size_t = 0 as libc::c_int as size_t;
    j1 = 0 as libc::c_int as size_t;
    while j1 < d {
        *vcov
            .offset(
                j1.wrapping_mul(tda2).wrapping_add(j1) as isize,
            ) = gsl_stats_variance(&*data.offset(j1 as isize), tda, n);
        j2 = j1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j2 < d {
            *vcov
                .offset(
                    j1.wrapping_mul(tda2).wrapping_add(j2) as isize,
                ) = gsl_stats_covariance(
                &*data.offset(j1 as isize),
                tda,
                &*data.offset(j2 as isize),
                tda,
                n,
            );
            *vcov
                .offset(
                    j2.wrapping_mul(tda2).wrapping_add(j1) as isize,
                ) = *vcov.offset(j1.wrapping_mul(tda2).wrapping_add(j2) as isize);
            j2 = j2.wrapping_add(1);
            j2;
        }
        j1 = j1.wrapping_add(1);
        j1;
    }
    return GSL_SUCCESS as libc::c_int;
}
