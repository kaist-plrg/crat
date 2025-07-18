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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub unsafe extern "C" fn cblas_cher2k(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: libc::c_int,
    K: libc::c_int,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: libc::c_int,
    mut B: *const libc::c_void,
    ldb: libc::c_int,
    beta: libc::c_float,
    mut C: *mut libc::c_void,
    ldc: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut uplo: libc::c_int = 0;
    let mut trans: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut __dim: libc::c_int = 0 as libc::c_int;
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        if Trans as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            __dim = K;
        } else {
            __dim = N;
        }
    } else if Trans as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
        __dim = N;
    } else {
        __dim = K;
    }
    if Order as libc::c_uint != CblasRowMajor as libc::c_int as libc::c_uint
        && Order as libc::c_uint != CblasColMajor as libc::c_int as libc::c_uint
    {
        pos = 1 as libc::c_int;
    }
    if Uplo as libc::c_uint != CblasUpper as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint != CblasLower as libc::c_int as libc::c_uint
    {
        pos = 2 as libc::c_int;
    }
    if Trans as libc::c_uint != CblasNoTrans as libc::c_int as libc::c_uint
        && Trans as libc::c_uint != CblasTrans as libc::c_int as libc::c_uint
        && Trans as libc::c_uint != CblasConjTrans as libc::c_int as libc::c_uint
    {
        pos = 3 as libc::c_int;
    }
    if N < 0 as libc::c_int {
        pos = 4 as libc::c_int;
    }
    if K < 0 as libc::c_int {
        pos = 5 as libc::c_int;
    }
    if lda < (if 1 as libc::c_int > __dim { 1 as libc::c_int } else { __dim }) {
        pos = 8 as libc::c_int;
    }
    if ldb < (if 1 as libc::c_int > __dim { 1 as libc::c_int } else { __dim }) {
        pos = 11 as libc::c_int;
    }
    if ldc < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
        pos = 14 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_her2k.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let mut alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
    if beta as libc::c_double == 1.0f64
        && (alpha_real as libc::c_double == 0.0f64
            && alpha_imag as libc::c_double == 0.0f64 || K == 0 as libc::c_int)
    {
        return;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        uplo = Uplo as libc::c_int;
        trans = Trans as libc::c_int;
    } else {
        uplo = if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
            CblasLower as libc::c_int
        } else {
            CblasUpper as libc::c_int
        };
        trans = if Trans as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            CblasConjTrans as libc::c_int
        } else {
            CblasNoTrans as libc::c_int
        };
        alpha_imag *= -(1 as libc::c_int) as libc::c_float;
    }
    if beta as libc::c_double == 0.0f64 {
        if uplo == CblasUpper as libc::c_int {
            i = 0 as libc::c_int;
            while i < N {
                j = i;
                while j < N {
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j)) as isize,
                        ) = 0.0f64 as libc::c_float;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = 0.0f64 as libc::c_float;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i < N {
                j = 0 as libc::c_int;
                while j <= i {
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j)) as isize,
                        ) = 0.0f64 as libc::c_float;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = 0.0f64 as libc::c_float;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    } else if beta as libc::c_double != 1.0f64 {
        if uplo == CblasUpper as libc::c_int {
            i = 0 as libc::c_int;
            while i < N {
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (ldc * i + i)) as isize) *= beta;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldc * i + i) + 1 as libc::c_int) as isize,
                    ) = 0.0f64 as libc::c_float;
                j = i + 1 as libc::c_int;
                while j < N {
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (ldc * i + j)) as isize) *= beta;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                as isize,
                        ) *= beta;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i < N {
                j = 0 as libc::c_int;
                while j < i {
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (ldc * i + j)) as isize) *= beta;
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                as isize,
                        ) *= beta;
                    j += 1;
                    j;
                }
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (ldc * i + i)) as isize) *= beta;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldc * i + i) + 1 as libc::c_int) as isize,
                    ) = 0.0f64 as libc::c_float;
                i += 1;
                i;
            }
        }
    } else {
        i = 0 as libc::c_int;
        while i < N {
            *(C as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int * (ldc * i + i) + 1 as libc::c_int) as isize,
                ) = 0.0f64 as libc::c_float;
            i += 1;
            i;
        }
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64 {
        return;
    }
    if uplo == CblasUpper as libc::c_int && trans == CblasNoTrans as libc::c_int {
        i = 0 as libc::c_int;
        while i < N {
            let mut temp_real: libc::c_float = 0.0f64 as libc::c_float;
            k = 0 as libc::c_int;
            while k < K {
                let Aik_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (i * lda + k)) as isize);
                let Aik_imag: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int) as isize,
                    );
                let temp1_real: libc::c_float = alpha_real * Aik_real
                    - alpha_imag * Aik_imag;
                let temp1_imag: libc::c_float = alpha_real * Aik_imag
                    + alpha_imag * Aik_real;
                let Bik_real: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                let Bik_imag: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int) as isize,
                    );
                temp_real += temp1_real * Bik_real + temp1_imag * Bik_imag;
                k += 1;
                k;
            }
            *(C as *mut libc::c_float)
                .offset((2 as libc::c_int * (i * ldc + i)) as isize)
                += 2 as libc::c_int as libc::c_float * temp_real;
            *(C as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int * (i * ldc + i) + 1 as libc::c_int) as isize,
                ) = 0.0f64 as libc::c_float;
            j = i + 1 as libc::c_int;
            while j < N {
                let mut temp_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < K {
                    let Aik_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let temp1_real_0: libc::c_float = alpha_real * Aik_real_0
                        - alpha_imag * Aik_imag_0;
                    let temp1_imag_0: libc::c_float = alpha_real * Aik_imag_0
                        + alpha_imag * Aik_real_0;
                    let Bik_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                    let Bik_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Ajk_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let temp2_real: libc::c_float = alpha_real * Ajk_real
                        - alpha_imag * Ajk_imag;
                    let temp2_imag: libc::c_float = alpha_real * Ajk_imag
                        + alpha_imag * Ajk_real;
                    let Bjk_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * ldb + k)) as isize);
                    let Bjk_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (j * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_0
                        += temp1_real_0 * Bjk_real + temp1_imag_0 * Bjk_imag
                            + (Bik_real_0 * temp2_real + Bik_imag_0 * temp2_imag);
                    temp_imag
                        += temp1_real_0 * -Bjk_imag + temp1_imag_0 * Bjk_real
                            + (Bik_real_0 * -temp2_imag + Bik_imag_0 * temp2_real);
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize) += temp_real_0;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += temp_imag;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasUpper as libc::c_int && trans == CblasConjTrans as libc::c_int
    {
        k = 0 as libc::c_int;
        while k < K {
            i = 0 as libc::c_int;
            while i < N {
                let mut Aki_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (k * lda + i)) as isize);
                let mut Aki_imag: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int) as isize,
                    );
                let mut Bki_real: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (k * ldb + i)) as isize);
                let mut Bki_imag: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (k * ldb + i) + 1 as libc::c_int) as isize,
                    );
                let mut temp1_real_1: libc::c_float = alpha_real * Aki_real
                    - alpha_imag * -Aki_imag;
                let mut temp1_imag_1: libc::c_float = alpha_real * -Aki_imag
                    + alpha_imag * Aki_real;
                let mut temp2_real_0: libc::c_float = alpha_real * Bki_real
                    - alpha_imag * Bki_imag;
                let mut temp2_imag_0: libc::c_float = -(alpha_real * Bki_imag
                    + alpha_imag * Bki_real);
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * lda + i)) as isize)
                    += 2 as libc::c_int as libc::c_float
                        * (temp1_real_1 * Bki_real - temp1_imag_1 * Bki_imag);
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int) as isize,
                    ) = 0.0f64 as libc::c_float;
                j = i + 1 as libc::c_int;
                while j < N {
                    let mut Akj_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let mut Akj_imag: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                as isize,
                        );
                    let mut Bkj_real: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * ldb + j)) as isize);
                    let mut Bkj_imag: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + j)) as isize)
                        += temp1_real_1 * Bkj_real - temp1_imag_1 * Bkj_imag
                            + (temp2_real_0 * Akj_real - temp2_imag_0 * Akj_imag);
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * lda + j) + 1 as libc::c_int)
                                as isize,
                        )
                        += temp1_real_1 * Bkj_imag + temp1_imag_1 * Bkj_real
                            + (temp2_real_0 * Akj_imag + temp2_imag_0 * Akj_real);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else if uplo == CblasLower as libc::c_int && trans == CblasNoTrans as libc::c_int {
        i = 0 as libc::c_int;
        while i < N {
            j = 0 as libc::c_int;
            while j < i {
                let mut temp_real_1: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < K {
                    let Aik_real_1: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag_1: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let temp1_real_2: libc::c_float = alpha_real * Aik_real_1
                        - alpha_imag * Aik_imag_1;
                    let temp1_imag_2: libc::c_float = alpha_real * Aik_imag_1
                        + alpha_imag * Aik_real_1;
                    let Bik_real_1: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                    let Bik_imag_1: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Ajk_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let temp2_real_1: libc::c_float = alpha_real * Ajk_real_0
                        - alpha_imag * Ajk_imag_0;
                    let temp2_imag_1: libc::c_float = alpha_real * Ajk_imag_0
                        + alpha_imag * Ajk_real_0;
                    let Bjk_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * ldb + k)) as isize);
                    let Bjk_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (j * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_1
                        += temp1_real_2 * Bjk_real_0 + temp1_imag_2 * Bjk_imag_0
                            + (Bik_real_1 * temp2_real_1 + Bik_imag_1 * temp2_imag_1);
                    temp_imag_0
                        += temp1_real_2 * -Bjk_imag_0 + temp1_imag_2 * Bjk_real_0
                            + (Bik_real_1 * -temp2_imag_1 + Bik_imag_1 * temp2_real_1);
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize) += temp_real_1;
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += temp_imag_0;
                j += 1;
                j;
            }
            let mut temp_real_2: libc::c_float = 0.0f64 as libc::c_float;
            k = 0 as libc::c_int;
            while k < K {
                let Aik_real_2: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (i * lda + k)) as isize);
                let Aik_imag_2: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int) as isize,
                    );
                let temp1_real_3: libc::c_float = alpha_real * Aik_real_2
                    - alpha_imag * Aik_imag_2;
                let temp1_imag_3: libc::c_float = alpha_real * Aik_imag_2
                    + alpha_imag * Aik_real_2;
                let Bik_real_2: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                let Bik_imag_2: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int) as isize,
                    );
                temp_real_2 += temp1_real_3 * Bik_real_2 + temp1_imag_3 * Bik_imag_2;
                k += 1;
                k;
            }
            *(C as *mut libc::c_float)
                .offset((2 as libc::c_int * (i * ldc + i)) as isize)
                += 2 as libc::c_int as libc::c_float * temp_real_2;
            *(C as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int * (i * ldc + i) + 1 as libc::c_int) as isize,
                ) = 0.0f64 as libc::c_float;
            i += 1;
            i;
        }
    } else if uplo == CblasLower as libc::c_int && trans == CblasConjTrans as libc::c_int
    {
        k = 0 as libc::c_int;
        while k < K {
            i = 0 as libc::c_int;
            while i < N {
                let mut Aki_real_0: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (k * lda + i)) as isize);
                let mut Aki_imag_0: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int) as isize,
                    );
                let mut Bki_real_0: libc::c_float = *(B as *const libc::c_float)
                    .offset((2 as libc::c_int * (k * ldb + i)) as isize);
                let mut Bki_imag_0: libc::c_float = *(B as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (k * ldb + i) + 1 as libc::c_int) as isize,
                    );
                let mut temp1_real_4: libc::c_float = alpha_real * Aki_real_0
                    - alpha_imag * -Aki_imag_0;
                let mut temp1_imag_4: libc::c_float = alpha_real * -Aki_imag_0
                    + alpha_imag * Aki_real_0;
                let mut temp2_real_2: libc::c_float = alpha_real * Bki_real_0
                    - alpha_imag * Bki_imag_0;
                let mut temp2_imag_2: libc::c_float = -(alpha_real * Bki_imag_0
                    + alpha_imag * Bki_real_0);
                j = 0 as libc::c_int;
                while j < i {
                    let mut Akj_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let mut Akj_imag_0: libc::c_float = *(A as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                as isize,
                        );
                    let mut Bkj_real_0: libc::c_float = *(B as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * ldb + j)) as isize);
                    let mut Bkj_imag_0: libc::c_float = *(B as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(C as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + j)) as isize)
                        += temp1_real_4 * Bkj_real_0 - temp1_imag_4 * Bkj_imag_0
                            + (temp2_real_2 * Akj_real_0 - temp2_imag_2 * Akj_imag_0);
                    *(C as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * lda + j) + 1 as libc::c_int)
                                as isize,
                        )
                        += temp1_real_4 * Bkj_imag_0 + temp1_imag_4 * Bkj_real_0
                            + (temp2_real_2 * Akj_imag_0 + temp2_imag_2 * Akj_real_0);
                    j += 1;
                    j;
                }
                *(C as *mut libc::c_float)
                    .offset((2 as libc::c_int * (i * lda + i)) as isize)
                    += 2 as libc::c_int as libc::c_float
                        * (temp1_real_4 * Bki_real_0 - temp1_imag_4 * Bki_imag_0);
                *(C as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int) as isize,
                    ) = 0.0f64 as libc::c_float;
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_her2k.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
