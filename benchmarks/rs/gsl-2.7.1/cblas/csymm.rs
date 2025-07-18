use ::libc;
extern "C" {
    fn cblas_xerbla(
        p: libc::c_int,
        rout: *const libc::c_char,
        form: *const libc::c_char,
        _: ...
    );
}
pub type CBLAS_ORDER = libc::c_uint;
pub const CblasColMajor: CBLAS_ORDER = 102;
pub const CblasRowMajor: CBLAS_ORDER = 101;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_SIDE = libc::c_uint;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
pub unsafe extern "C" fn cblas_csymm(
    Order: CBLAS_ORDER,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: libc::c_int,
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: libc::c_int,
    mut B: *const libc::c_void,
    ldb: libc::c_int,
    mut beta: *const libc::c_void,
    mut C: *mut libc::c_void,
    ldc: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut uplo: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut __dimA: libc::c_int = 0 as libc::c_int;
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint {
        __dimA = M;
    } else {
        __dimA = N;
    }
    if Order as libc::c_uint != CblasRowMajor as libc::c_int as libc::c_uint
        && Order as libc::c_uint != CblasColMajor as libc::c_int as libc::c_uint
    {
        pos = 1 as libc::c_int;
    }
    if Side as libc::c_uint != CblasLeft as libc::c_int as libc::c_uint
        && Side as libc::c_uint != CblasRight as libc::c_int as libc::c_uint
    {
        pos = 2 as libc::c_int;
    }
    if Uplo as libc::c_uint != CblasUpper as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint != CblasLower as libc::c_int as libc::c_uint
    {
        pos = 3 as libc::c_int;
    }
    if M < 0 as libc::c_int {
        pos = 4 as libc::c_int;
    }
    if N < 0 as libc::c_int {
        pos = 5 as libc::c_int;
    }
    if lda < (if 1 as libc::c_int > __dimA { 1 as libc::c_int } else { __dimA }) {
        pos = 8 as libc::c_int;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        if ldb < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
            pos = 10 as libc::c_int;
        }
        if ldc < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
            pos = 13 as libc::c_int;
        }
    } else if Order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint {
        if ldb < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
            pos = 10 as libc::c_int;
        }
        if ldc < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
            pos = 13 as libc::c_int;
        }
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_symm_c.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
    let beta_real: libc::c_float = *(beta as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let beta_imag: libc::c_float = *(beta as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64
        && (beta_real as libc::c_double == 1.0f64
            && beta_imag as libc::c_double == 0.0f64)
    {
        return;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        n1 = M;
        n2 = N;
        uplo = Uplo as libc::c_int;
        side = Side as libc::c_int;
    } else {
        n1 = N;
        n2 = M;
        uplo = if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
            CblasLower as libc::c_int
        } else {
            CblasUpper as libc::c_int
        };
        side = if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint {
            CblasRight as libc::c_int
        } else {
            CblasLeft as libc::c_int
        };
    }
    if beta_real as libc::c_double == 0.0f64 && beta_imag as libc::c_double == 0.0f64 {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j)) as isize,
                    ) = 0.0f64 as libc::c_float;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    ) = 0.0f64 as libc::c_float;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if !(beta_real as libc::c_double == 1.0f64
        && beta_imag as libc::c_double == 0.0f64)
    {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let Cij_real: libc::c_float = *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (ldc * i + j)) as isize);
                let Cij_imag: libc::c_float = *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    );
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j)) as isize,
                    ) = beta_real * Cij_real - beta_imag * Cij_imag;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    ) = beta_real * Cij_imag + beta_imag * Cij_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64 {
        return;
    }
    if side == CblasLeft as libc::c_int && uplo == CblasUpper as libc::c_int {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let Bij_real: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                let temp1_real: libc::c_float = alpha_real * Bij_real
                    - alpha_imag * Bij_imag;
                let temp1_imag: libc::c_float = alpha_real * Bij_imag
                    + alpha_imag * Bij_real;
                let mut temp2_real: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp2_imag: libc::c_float = 0.0f64 as libc::c_float;
                let Aii_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (i * lda + i)) as isize);
                let Aii_imag: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int) as isize,
                    );
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += temp1_real * Aii_real - temp1_imag * Aii_imag;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += temp1_real * Aii_imag + temp1_imag * Aii_real;
                k = i + 1 as libc::c_int;
                while k < n1 {
                    let Aik_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Bkj_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (ldb * k + j)) as isize);
                    let Bkj_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldb * k + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (k * ldc + j)) as isize)
                        += Aik_real * temp1_real - Aik_imag * temp1_imag;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldc + j) + 1 as libc::c_int)
                                as isize,
                        ) += Aik_real * temp1_imag + Aik_imag * temp1_real;
                    temp2_real += Aik_real * Bkj_real - Aik_imag * Bkj_imag;
                    temp2_imag += Aik_real * Bkj_imag + Aik_imag * Bkj_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real - alpha_imag * temp2_imag;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha_real * temp2_imag + alpha_imag * temp2_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasLeft as libc::c_int && uplo == CblasLower as libc::c_int {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let Bij_real_0: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag_0: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                let temp1_real_0: libc::c_float = alpha_real * Bij_real_0
                    - alpha_imag * Bij_imag_0;
                let temp1_imag_0: libc::c_float = alpha_real * Bij_imag_0
                    + alpha_imag * Bij_real_0;
                let mut temp2_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp2_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < i {
                    let Aik_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Bkj_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (ldb * k + j)) as isize);
                    let Bkj_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldb * k + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (k * ldc + j)) as isize)
                        += Aik_real_0 * temp1_real_0 - Aik_imag_0 * temp1_imag_0;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldc + j) + 1 as libc::c_int)
                                as isize,
                        ) += Aik_real_0 * temp1_imag_0 + Aik_imag_0 * temp1_real_0;
                    temp2_real_0 += Aik_real_0 * Bkj_real_0 - Aik_imag_0 * Bkj_imag_0;
                    temp2_imag_0 += Aik_real_0 * Bkj_imag_0 + Aik_imag_0 * Bkj_real_0;
                    k += 1;
                    k;
                }
                let Aii_real_0: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (i * lda + i)) as isize);
                let Aii_imag_0: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int) as isize,
                    );
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += temp1_real_0 * Aii_real_0 - temp1_imag_0 * Aii_imag_0;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += temp1_real_0 * Aii_imag_0 + temp1_imag_0 * Aii_real_0;
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real_0 - alpha_imag * temp2_imag_0;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha_real * temp2_imag_0 + alpha_imag * temp2_real_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasUpper as libc::c_int {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let Bij_real_1: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag_1: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                let temp1_real_1: libc::c_float = alpha_real * Bij_real_1
                    - alpha_imag * Bij_imag_1;
                let temp1_imag_1: libc::c_float = alpha_real * Bij_imag_1
                    + alpha_imag * Bij_real_1;
                let mut temp2_real_1: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp2_imag_1: libc::c_float = 0.0f64 as libc::c_float;
                let Ajj_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (j * lda + j)) as isize);
                let Ajj_imag: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (j * lda + j) + 1 as libc::c_int) as isize,
                    );
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += temp1_real_1 * Ajj_real - temp1_imag_1 * Ajj_imag;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += temp1_real_1 * Ajj_imag + temp1_imag_1 * Ajj_real;
                k = j + 1 as libc::c_int;
                while k < n2 {
                    let Ajk_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Bik_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (ldb * i + k)) as isize);
                    let Bik_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldb * i + k) + 1 as libc::c_int)
                                as isize,
                        );
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldc + k)) as isize)
                        += temp1_real_1 * Ajk_real - temp1_imag_1 * Ajk_imag;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldc + k) + 1 as libc::c_int)
                                as isize,
                        ) += temp1_real_1 * Ajk_imag + temp1_imag_1 * Ajk_real;
                    temp2_real_1 += Bik_real * Ajk_real - Bik_imag * Ajk_imag;
                    temp2_imag_1 += Bik_real * Ajk_imag + Bik_imag * Ajk_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real_1 - alpha_imag * temp2_imag_1;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha_real * temp2_imag_1 + alpha_imag * temp2_real_1;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasLower as libc::c_int {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let Bij_real_2: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag_2: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                let temp1_real_2: libc::c_float = alpha_real * Bij_real_2
                    - alpha_imag * Bij_imag_2;
                let temp1_imag_2: libc::c_float = alpha_real * Bij_imag_2
                    + alpha_imag * Bij_real_2;
                let mut temp2_real_2: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp2_imag_2: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < j {
                    let Ajk_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Bik_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (ldb * i + k)) as isize);
                    let Bik_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldb * i + k) + 1 as libc::c_int)
                                as isize,
                        );
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldc + k)) as isize)
                        += temp1_real_2 * Ajk_real_0 - temp1_imag_2 * Ajk_imag_0;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldc + k) + 1 as libc::c_int)
                                as isize,
                        ) += temp1_real_2 * Ajk_imag_0 + temp1_imag_2 * Ajk_real_0;
                    temp2_real_2 += Bik_real_0 * Ajk_real_0 - Bik_imag_0 * Ajk_imag_0;
                    temp2_imag_2 += Bik_real_0 * Ajk_imag_0 + Bik_imag_0 * Ajk_real_0;
                    k += 1;
                    k;
                }
                let Ajj_real_0: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (j * lda + j)) as isize);
                let Ajj_imag_0: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (j * lda + j) + 1 as libc::c_int) as isize,
                    );
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += temp1_real_2 * Ajj_real_0 - temp1_imag_2 * Ajj_imag_0;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += temp1_real_2 * Ajj_imag_0 + temp1_imag_2 * Ajj_real_0;
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha_real * temp2_real_2 - alpha_imag * temp2_imag_2;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha_real * temp2_imag_2 + alpha_imag * temp2_real_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_symm_c.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
