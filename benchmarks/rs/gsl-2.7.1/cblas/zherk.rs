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
pub unsafe extern "C" fn cblas_zherk(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: libc::c_int,
    K: libc::c_int,
    alpha: libc::c_double,
    mut A: *const libc::c_void,
    lda: libc::c_int,
    beta: libc::c_double,
    mut C: *mut libc::c_void,
    ldc: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut uplo: libc::c_int = 0;
    let mut trans: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut __dimA: libc::c_int = 0 as libc::c_int;
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        if Trans as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            __dimA = K;
        } else {
            __dimA = N;
        }
    } else if Trans as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
        __dimA = N;
    } else {
        __dimA = K;
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
    if lda < (if 1 as libc::c_int > __dimA { 1 as libc::c_int } else { __dimA }) {
        pos = 8 as libc::c_int;
    }
    if ldc < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
        pos = 11 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_herk.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if beta == 1.0f64 && (alpha == 0.0f64 || K == 0 as libc::c_int) {
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
    }
    if beta == 0.0f64 {
        if uplo == CblasUpper as libc::c_int {
            i = 0 as libc::c_int;
            while i < N {
                j = i;
                while j < N {
                    *(C as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldc * i + j)) as isize) = 0.0f64;
                    *(C as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = 0.0f64;
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
                    *(C as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldc * i + j)) as isize) = 0.0f64;
                    *(C as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = 0.0f64;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    } else if beta != 1.0f64 {
        if uplo == CblasUpper as libc::c_int {
            i = 0 as libc::c_int;
            while i < N {
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldc * i + i)) as isize) *= beta;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + i) + 1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as libc::c_double;
                j = i + 1 as libc::c_int;
                while j < N {
                    *(C as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldc * i + j)) as isize) *= beta;
                    *(C as *mut libc::c_double)
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
                    *(C as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldc * i + j)) as isize) *= beta;
                    *(C as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                as isize,
                        ) *= beta;
                    j += 1;
                    j;
                }
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldc * i + i)) as isize) *= beta;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + i) + 1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as libc::c_double;
                i += 1;
                i;
            }
        }
    } else {
        i = 0 as libc::c_int;
        while i < N {
            *(C as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * (ldc * i + i) + 1 as libc::c_int) as isize,
                ) = 0.0f64;
            i += 1;
            i;
        }
    }
    if alpha == 0.0f64 {
        return;
    }
    if uplo == CblasUpper as libc::c_int && trans == CblasNoTrans as libc::c_int {
        i = 0 as libc::c_int;
        while i < N {
            j = i;
            while j < N {
                let mut temp_real: libc::c_double = 0.0f64;
                let mut temp_imag: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    let Aik_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Ajk_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_double = -*(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real += Aik_real * Ajk_real - Aik_imag * Ajk_imag;
                    temp_imag += Aik_real * Ajk_imag + Aik_imag * Ajk_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha * temp_real;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha * temp_imag;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasUpper as libc::c_int && trans == CblasConjTrans as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < N {
            j = i;
            while j < N {
                let mut temp_real_0: libc::c_double = 0.0f64;
                let mut temp_imag_0: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    let Aki_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (k * lda + i)) as isize);
                    let Aki_imag: libc::c_double = -*(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int)
                                as isize,
                        );
                    let Akj_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let Akj_imag: libc::c_double = *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_0 += Aki_real * Akj_real - Aki_imag * Akj_imag;
                    temp_imag_0 += Aki_real * Akj_imag + Aki_imag * Akj_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha * temp_real_0;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha * temp_imag_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasLower as libc::c_int && trans == CblasNoTrans as libc::c_int {
        i = 0 as libc::c_int;
        while i < N {
            j = 0 as libc::c_int;
            while j <= i {
                let mut temp_real_1: libc::c_double = 0.0f64;
                let mut temp_imag_1: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    let Aik_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    let Ajk_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_double = -*(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_1 += Aik_real_0 * Ajk_real_0 - Aik_imag_0 * Ajk_imag_0;
                    temp_imag_1 += Aik_real_0 * Ajk_imag_0 + Aik_imag_0 * Ajk_real_0;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha * temp_real_1;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha * temp_imag_1;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasLower as libc::c_int && trans == CblasConjTrans as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < N {
            j = 0 as libc::c_int;
            while j <= i {
                let mut temp_real_2: libc::c_double = 0.0f64;
                let mut temp_imag_2: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    let Aki_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (k * lda + i)) as isize);
                    let Aki_imag_0: libc::c_double = -*(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int)
                                as isize,
                        );
                    let Akj_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let Akj_imag_0: libc::c_double = *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_2 += Aki_real_0 * Akj_real_0 - Aki_imag_0 * Akj_imag_0;
                    temp_imag_2 += Aki_real_0 * Akj_imag_0 + Aki_imag_0 * Akj_real_0;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (i * ldc + j)) as isize)
                    += alpha * temp_real_2;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (i * ldc + j) + 1 as libc::c_int) as isize,
                    ) += alpha * temp_imag_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_herk.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
