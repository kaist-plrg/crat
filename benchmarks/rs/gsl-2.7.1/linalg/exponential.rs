use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_blas_dgemm(
        TransA: CBLAS_TRANSPOSE_t,
        TransB: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> libc::c_int;
    fn gsl_matrix_minmax(
        m: *const gsl_matrix,
        min_out: *mut libc::c_double,
        max_out: *mut libc::c_double,
    );
    fn gsl_matrix_scale(a: *mut gsl_matrix, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_add_diagonal(a: *mut gsl_matrix, x: libc::c_double) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type gsl_mode_t = libc::c_uint;
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
pub type mvl_suggestion_t = moler_vanloan_optimal_suggestion;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moler_vanloan_optimal_suggestion {
    pub k: libc::c_int,
    pub j: libc::c_int,
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MODE_PREC(mut mt: gsl_mode_t) -> libc::c_uint {
    return mt & 7 as libc::c_int as libc::c_uint;
}
static mut mvl_tab: [[mvl_suggestion_t; 6]; 3] = [
    [
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 5 as libc::c_int,
                j: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 5 as libc::c_int,
                j: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 7 as libc::c_int,
                j: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 9 as libc::c_int,
                j: 7 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 10 as libc::c_int,
                j: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 8 as libc::c_int,
                j: 14 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 2 as libc::c_int,
                j: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 4 as libc::c_int,
                j: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 7 as libc::c_int,
                j: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 6 as libc::c_int,
                j: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 5 as libc::c_int,
                j: 9 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 7 as libc::c_int,
                j: 11 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 1 as libc::c_int,
                j: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 3 as libc::c_int,
                j: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 5 as libc::c_int,
                j: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 4 as libc::c_int,
                j: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 4 as libc::c_int,
                j: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = moler_vanloan_optimal_suggestion {
                k: 2 as libc::c_int,
                j: 11 as libc::c_int,
            };
            init
        },
    ],
];
#[inline]
unsafe extern "C" fn sup_norm(mut A: *const gsl_matrix) -> libc::c_double {
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    gsl_matrix_minmax(A, &mut min, &mut max);
    return GSL_MAX_DBL(fabs(min), fabs(max));
}
unsafe extern "C" fn obtain_suggestion(
    mut A: *const gsl_matrix,
    mut mode: gsl_mode_t,
) -> mvl_suggestion_t {
    let mode_prec: libc::c_uint = GSL_MODE_PREC(mode);
    let norm_A: libc::c_double = sup_norm(A);
    if norm_A < 0.01f64 {
        return mvl_tab[mode_prec as usize][0 as libc::c_int as usize]
    } else if norm_A < 0.1f64 {
        return mvl_tab[mode_prec as usize][1 as libc::c_int as usize]
    } else if norm_A < 1.0f64 {
        return mvl_tab[mode_prec as usize][2 as libc::c_int as usize]
    } else if norm_A < 10.0f64 {
        return mvl_tab[mode_prec as usize][3 as libc::c_int as usize]
    } else if norm_A < 100.0f64 {
        return mvl_tab[mode_prec as usize][4 as libc::c_int as usize]
    } else if norm_A < 1000.0f64 {
        return mvl_tab[mode_prec as usize][5 as libc::c_int as usize]
    } else {
        let extra: libc::c_double = log(1.01f64 * norm_A / 1000.0f64)
            / 0.69314718055994530942f64;
        let extra_i: libc::c_int = ceil(extra) as libc::c_uint as libc::c_int;
        let mut s: mvl_suggestion_t = mvl_tab[mode as usize][5 as libc::c_int as usize];
        s.j += extra_i;
        return s;
    };
}
unsafe extern "C" fn matrix_exp_series(
    mut B: *const gsl_matrix,
    mut eB: *mut gsl_matrix,
    mut number_of_terms: libc::c_int,
) {
    let mut count: libc::c_int = 0;
    let mut temp: *mut gsl_matrix = gsl_matrix_calloc((*B).size1, (*B).size2);
    gsl_matrix_memcpy(eB, B);
    gsl_matrix_scale(eB, 1.0f64 / number_of_terms as libc::c_double);
    gsl_matrix_add_diagonal(eB, 1.0f64);
    count = number_of_terms - 1 as libc::c_int;
    while count >= 1 as libc::c_int {
        gsl_blas_dgemm(CblasNoTrans, CblasNoTrans, 1.0f64, B, eB, 0.0f64, temp);
        gsl_matrix_scale(temp, 1.0f64 / count as libc::c_double);
        gsl_matrix_add_diagonal(temp, 1.0f64);
        gsl_matrix_memcpy(eB, temp);
        count -= 1;
        count;
    }
    gsl_matrix_free(temp);
}
pub unsafe extern "C" fn gsl_linalg_exponential_ss(
    mut A: *const gsl_matrix,
    mut eA: *mut gsl_matrix,
    mut mode: gsl_mode_t,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"cannot exponentiate a non-square matrix\0" as *const u8
                as *const libc::c_char,
            b"exponential.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*A).size1 != (*eA).size1 || (*A).size2 != (*eA).size2 {
        gsl_error(
            b"exponential of matrix must have same dimension as matrix\0" as *const u8
                as *const libc::c_char,
            b"exponential.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut i: libc::c_int = 0;
        let sugg: mvl_suggestion_t = obtain_suggestion(A, mode);
        let divisor: libc::c_double = exp(
            0.69314718055994530942f64 * sugg.j as libc::c_double,
        );
        let mut reduced_A: *mut gsl_matrix = gsl_matrix_alloc((*A).size1, (*A).size2);
        gsl_matrix_memcpy(reduced_A, A);
        gsl_matrix_scale(reduced_A, 1.0f64 / divisor);
        matrix_exp_series(reduced_A, eA, sugg.k);
        i = 0 as libc::c_int;
        while i < sugg.j {
            gsl_blas_dgemm(
                CblasNoTrans,
                CblasNoTrans,
                1.0f64,
                eA,
                eA,
                0.0f64,
                reduced_A,
            );
            gsl_matrix_memcpy(eA, reduced_A);
            i += 1;
            i;
        }
        gsl_matrix_free(reduced_A);
        return GSL_SUCCESS as libc::c_int;
    };
}
