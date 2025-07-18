use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_ran_chisq(r: *const gsl_rng, nu: libc::c_double) -> libc::c_double;
    fn gsl_ran_ugaussian(r: *const gsl_rng) -> libc::c_double;
    fn gsl_blas_dsyrk(
        Uplo: CBLAS_UPLO_t,
        Trans: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_blas_dtrmm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_linalg_cholesky_solve_mat(
        cholesky: *const gsl_matrix,
        B: *const gsl_matrix,
        X: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double;
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
pub type CBLAS_SIDE = libc::c_uint;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
pub type CBLAS_SIDE_t = CBLAS_SIDE;
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
pub unsafe extern "C" fn gsl_ran_wishart(
    mut r: *const gsl_rng,
    df: libc::c_double,
    mut L: *const gsl_matrix,
    mut result: *mut gsl_matrix,
    mut work: *mut gsl_matrix,
) -> libc::c_int {
    if (*L).size1 != (*L).size2 {
        gsl_error(
            b"L should be a square matrix\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*result).size1 != (*result).size2 {
        gsl_error(
            b"result should be a square matrix\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*work).size1 != (*work).size2 {
        gsl_error(
            b"work should be a square matrix\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*result).size1 != (*L).size1 {
        gsl_error(
            b"incompatible dimensions of result matrix\0" as *const u8
                as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*work).size1 != (*L).size1 {
        gsl_error(
            b"incompatible dimensions of work matrix\0" as *const u8
                as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if df
        <= ((*L).size1).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
    {
        gsl_error(
            b"incompatible degrees of freedom\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut d: size_t = (*L).size1;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        gsl_matrix_set_zero(work);
        i = 0 as libc::c_int as size_t;
        while i < d {
            gsl_matrix_set(work, i, i, sqrt(gsl_ran_chisq(r, df - i as libc::c_double)));
            j = 0 as libc::c_int as size_t;
            while j < i {
                gsl_matrix_set(work, i, j, gsl_ran_ugaussian(r));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        gsl_blas_dtrmm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasNonUnit,
            1.0f64,
            L,
            work,
        );
        gsl_blas_dsyrk(CblasUpper, CblasNoTrans, 1.0f64, work, 0.0f64, result);
        i = 0 as libc::c_int as size_t;
        while i < d {
            j = 0 as libc::c_int as size_t;
            while j < i {
                gsl_matrix_set(result, i, j, gsl_matrix_get(result, j, i));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_ran_wishart_log_pdf(
    mut X: *const gsl_matrix,
    mut L_X: *const gsl_matrix,
    df: libc::c_double,
    mut L: *const gsl_matrix,
    mut result: *mut libc::c_double,
    mut work: *mut gsl_matrix,
) -> libc::c_int {
    if (*L).size1 != (*L).size2 {
        gsl_error(
            b"L should be a square matrix\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size1 != (*X).size2 {
        gsl_error(
            b"X should be a square matrix\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*L_X).size1 != (*L_X).size2 {
        gsl_error(
            b"L_X should be a square matrix\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size1 != (*L).size1 {
        gsl_error(
            b"incompatible dimensions of X matrix\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*L_X).size1 != (*L).size1 {
        gsl_error(
            b"incompatible dimensions of L_X matrix\0" as *const u8
                as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if df
        <= ((*L).size1).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_double
    {
        gsl_error(
            b"incompatible degrees of freedom\0" as *const u8 as *const libc::c_char,
            b"wishart.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut d: size_t = (*L).size1;
        let mut i: size_t = 0;
        let mut status: libc::c_int = 0;
        let mut log_mv_Ga: libc::c_double = 0.;
        let mut log_det_V: libc::c_double = 0.;
        let mut log_det_X: libc::c_double = 0.;
        let mut tr_Vinv_X: libc::c_double = 0.;
        log_mv_Ga = d.wrapping_mul(d.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            as libc::c_double * 0.25f64 * log(3.14159265358979323846f64);
        i = 0 as libc::c_int as size_t;
        while i < d {
            log_mv_Ga
                += gsl_sf_lngamma(
                    (df - i as libc::c_double + 1 as libc::c_int as libc::c_double)
                        * 0.5f64,
                );
            i = i.wrapping_add(1);
            i;
        }
        log_det_V = log(
            gsl_matrix_get(L, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
        );
        i = 1 as libc::c_int as size_t;
        while i < d {
            log_det_V += log(gsl_matrix_get(L, i, i));
            i = i.wrapping_add(1);
            i;
        }
        log_det_V = 2 as libc::c_int as libc::c_double * log_det_V;
        log_det_X = log(
            gsl_matrix_get(L_X, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
        );
        i = 1 as libc::c_int as size_t;
        while i < d {
            log_det_X += log(gsl_matrix_get(L_X, i, i));
            i = i.wrapping_add(1);
            i;
        }
        log_det_X = 2 as libc::c_int as libc::c_double * log_det_X;
        status = gsl_linalg_cholesky_solve_mat(L, X, work);
        if status != 0 {
            return status;
        }
        tr_Vinv_X = gsl_matrix_get(
            work,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        i = 1 as libc::c_int as size_t;
        while i < d {
            tr_Vinv_X += gsl_matrix_get(work, i, i);
            i = i.wrapping_add(1);
            i;
        }
        *result = -(0.5f64 * df * d as libc::c_double) * log(2.0f64)
            - 0.5f64 * df * log_det_V - log_mv_Ga
            + 0.5f64 * (df - d as libc::c_double - 1 as libc::c_int as libc::c_double)
                * log_det_X - 0.5f64 * tr_Vinv_X;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_ran_wishart_pdf(
    mut X: *const gsl_matrix,
    mut L_X: *const gsl_matrix,
    df: libc::c_double,
    mut L: *const gsl_matrix,
    mut result: *mut libc::c_double,
    mut work: *mut gsl_matrix,
) -> libc::c_int {
    let mut logpdf: libc::c_double = 0.;
    let mut status: libc::c_int = gsl_ran_wishart_log_pdf(
        X,
        L_X,
        df,
        L,
        &mut logpdf,
        work,
    );
    if status == GSL_SUCCESS as libc::c_int {
        *result = exp(logpdf);
    }
    return status;
}
