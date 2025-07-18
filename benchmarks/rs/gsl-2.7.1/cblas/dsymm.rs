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
pub unsafe extern "C" fn cblas_dsymm(
    Order: CBLAS_ORDER,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: libc::c_int,
    N: libc::c_int,
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
            b"./source_symm_r.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if alpha == 0.0f64 && beta == 1.0f64 {
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
    if beta == 0.0f64 {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                *C.offset((ldc * i + j) as isize) = 0.0f64;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if beta != 1.0f64 {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                *C.offset((ldc * i + j) as isize) *= beta;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    if alpha == 0.0f64 {
        return;
    }
    if side == CblasLeft as libc::c_int && uplo == CblasUpper as libc::c_int {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let temp1: libc::c_double = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2: libc::c_double = 0.0f64;
                *C.offset((i * ldc + j) as isize)
                    += temp1 * *A.offset((i * lda + i) as isize);
                k = i + 1 as libc::c_int;
                while k < n1 {
                    let Aik: libc::c_double = *A.offset((i * lda + k) as isize);
                    *C.offset((k * ldc + j) as isize) += Aik * temp1;
                    temp2 += Aik * *B.offset((ldb * k + j) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp2;
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
                let temp1_0: libc::c_double = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2_0: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < i {
                    let Aik_0: libc::c_double = *A.offset((i * lda + k) as isize);
                    *C.offset((k * ldc + j) as isize) += Aik_0 * temp1_0;
                    temp2_0 += Aik_0 * *B.offset((ldb * k + j) as isize);
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize)
                    += temp1_0 * *A.offset((i * lda + i) as isize) + alpha * temp2_0;
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
                let temp1_1: libc::c_double = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2_1: libc::c_double = 0.0f64;
                *C.offset((i * ldc + j) as isize)
                    += temp1_1 * *A.offset((j * lda + j) as isize);
                k = j + 1 as libc::c_int;
                while k < n2 {
                    let Ajk: libc::c_double = *A.offset((j * lda + k) as isize);
                    *C.offset((i * ldc + k) as isize) += temp1_1 * Ajk;
                    temp2_1 += *B.offset((ldb * i + k) as isize) * Ajk;
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize) += alpha * temp2_1;
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
                let temp1_2: libc::c_double = alpha * *B.offset((ldb * i + j) as isize);
                let mut temp2_2: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < j {
                    let Ajk_0: libc::c_double = *A.offset((j * lda + k) as isize);
                    *C.offset((i * ldc + k) as isize) += temp1_2 * Ajk_0;
                    temp2_2 += *B.offset((ldb * i + k) as isize) * Ajk_0;
                    k += 1;
                    k;
                }
                *C.offset((i * ldc + j) as isize)
                    += temp1_2 * *A.offset((j * lda + j) as isize) + alpha * temp2_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_symm_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
