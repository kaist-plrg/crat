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
pub unsafe extern "C" fn cblas_dsyr2k(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: libc::c_int,
    K: libc::c_int,
    alpha: libc::c_double,
    mut A: *const libc::c_double,
    lda: libc::c_int,
    mut B: *const libc::c_double,
    ldb: libc::c_int,
    beta: libc::c_double,
    mut C: *mut libc::c_double,
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
            b"./source_syr2k_r.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if alpha == 0.0f64 && beta == 1.0f64 {
        return;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        uplo = Uplo as libc::c_int;
        trans = (if Trans as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            CblasTrans as libc::c_int as libc::c_uint
        } else {
            Trans as libc::c_uint
        }) as libc::c_int;
    } else {
        uplo = if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
            CblasLower as libc::c_int
        } else {
            CblasUpper as libc::c_int
        };
        if Trans as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            || Trans as libc::c_uint == CblasConjTrans as libc::c_int as libc::c_uint
        {
            trans = CblasNoTrans as libc::c_int;
        } else {
            trans = CblasTrans as libc::c_int;
        }
    }
    if beta == 0.0f64 {
        if uplo == CblasUpper as libc::c_int {
            i = 0 as libc::c_int;
            while i < N {
                j = i;
                while j < N {
                    *C.offset((ldc * i + j) as isize) = 0.0f64;
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
                    *C.offset((ldc * i + j) as isize) = 0.0f64;
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
                j = i;
                while j < N {
                    *C.offset((ldc * i + j) as isize) *= beta;
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
                    *C.offset((ldc * i + j) as isize) *= beta;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
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
                let mut temp: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    temp
                        += *A.offset((i * lda + k) as isize)
                            * *B.offset((j * ldb + k) as isize)
                            + *B.offset((i * ldb + k) as isize)
                                * *A.offset((j * lda + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasUpper as libc::c_int && trans == CblasTrans as libc::c_int {
        k = 0 as libc::c_int;
        while k < K {
            i = 0 as libc::c_int;
            while i < N {
                let mut temp1: libc::c_double = alpha
                    * *A.offset((k * lda + i) as isize);
                let mut temp2: libc::c_double = alpha
                    * *B.offset((k * ldb + i) as isize);
                j = i;
                while j < N {
                    *C.offset((i * lda + j) as isize)
                        += temp1 * *B.offset((k * ldb + j) as isize)
                            + temp2 * *A.offset((k * lda + j) as isize);
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
            while j <= i {
                let mut temp_0: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    temp_0
                        += *A.offset((i * lda + k) as isize)
                            * *B.offset((j * ldb + k) as isize)
                            + *B.offset((i * ldb + k) as isize)
                                * *A.offset((j * lda + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if uplo == CblasLower as libc::c_int && trans == CblasTrans as libc::c_int {
        k = 0 as libc::c_int;
        while k < K {
            i = 0 as libc::c_int;
            while i < N {
                let mut temp1_0: libc::c_double = alpha
                    * *A.offset((k * lda + i) as isize);
                let mut temp2_0: libc::c_double = alpha
                    * *B.offset((k * ldb + i) as isize);
                j = 0 as libc::c_int;
                while j <= i {
                    *C.offset((i * lda + j) as isize)
                        += temp1_0 * *B.offset((k * ldb + j) as isize)
                            + temp2_0 * *A.offset((k * lda + j) as isize);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_syr2k_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
