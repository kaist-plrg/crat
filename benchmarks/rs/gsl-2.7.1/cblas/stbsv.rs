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
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub unsafe extern "C" fn cblas_stbsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: libc::c_int,
    K: libc::c_int,
    mut A: *const libc::c_float,
    lda: libc::c_int,
    mut X: *mut libc::c_float,
    incX: libc::c_int,
) {
    let nonunit: libc::c_int = (Diag as libc::c_uint
        == CblasNonUnit as libc::c_int as libc::c_uint) as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let Trans: libc::c_int = (if TransA as libc::c_uint
        != CblasConjTrans as libc::c_int as libc::c_uint
    {
        TransA as libc::c_uint
    } else {
        CblasTrans as libc::c_int as libc::c_uint
    }) as libc::c_int;
    let mut pos: libc::c_int = 0 as libc::c_int;
    if order as libc::c_uint != CblasRowMajor as libc::c_int as libc::c_uint
        && order as libc::c_uint != CblasColMajor as libc::c_int as libc::c_uint
    {
        pos = 1 as libc::c_int;
    }
    if Uplo as libc::c_uint != CblasUpper as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint != CblasLower as libc::c_int as libc::c_uint
    {
        pos = 2 as libc::c_int;
    }
    if TransA as libc::c_uint != CblasNoTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasConjTrans as libc::c_int as libc::c_uint
    {
        pos = 3 as libc::c_int;
    }
    if Diag as libc::c_uint != CblasNonUnit as libc::c_int as libc::c_uint
        && Diag as libc::c_uint != CblasUnit as libc::c_int as libc::c_uint
    {
        pos = 4 as libc::c_int;
    }
    if N < 0 as libc::c_int {
        pos = 5 as libc::c_int;
    }
    if K < 0 as libc::c_int {
        pos = 6 as libc::c_int;
    }
    if lda
        < (if 1 as libc::c_int > K + 1 as libc::c_int {
            1 as libc::c_int
        } else {
            K + 1 as libc::c_int
        })
    {
        pos = 8 as libc::c_int;
    }
    if incX == 0 as libc::c_int {
        pos = 10 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_tbsv_r.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if N == 0 as libc::c_int {
        return;
    }
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasNoTrans as libc::c_int
        && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasTrans as libc::c_int
            && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
    {
        let mut ix: libc::c_int = (if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        }) + incX * (N - 1 as libc::c_int);
        i = N;
        while i > 0 as libc::c_int
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let mut tmp: libc::c_float = *X.offset(ix as isize);
            let j_min: libc::c_int = i + 1 as libc::c_int;
            let j_max: libc::c_int = if N < i + K + 1 as libc::c_int {
                N
            } else {
                i + K + 1 as libc::c_int
            };
            let mut jx: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + j_min * incX;
            j = j_min;
            while j < j_max {
                let Aij: libc::c_float = *A.offset((lda * i + (j - i)) as isize);
                tmp -= Aij * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X
                    .offset(
                        ix as isize,
                    ) = tmp / *A.offset((lda * i + 0 as libc::c_int) as isize);
            } else {
                *X.offset(ix as isize) = tmp;
            }
            ix -= incX;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasNoTrans as libc::c_int
        && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasTrans as libc::c_int
            && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
    {
        let mut ix_0: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        i = 0 as libc::c_int;
        while i < N {
            let mut tmp_0: libc::c_float = *X.offset(ix_0 as isize);
            let j_min_0: libc::c_int = if i > K { i - K } else { 0 as libc::c_int };
            let j_max_0: libc::c_int = i;
            let mut jx_0: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + j_min_0 * incX;
            j = j_min_0;
            while j < j_max_0 {
                let Aij_0: libc::c_float = *A.offset((lda * i + (K + j - i)) as isize);
                tmp_0 -= Aij_0 * *X.offset(jx_0 as isize);
                jx_0 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix_0 as isize) = tmp_0 / *A.offset((lda * i + K) as isize);
            } else {
                *X.offset(ix_0 as isize) = tmp_0;
            }
            ix_0 += incX;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasTrans as libc::c_int
        && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasNoTrans as libc::c_int
            && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
    {
        let mut ix_1: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        i = 0 as libc::c_int;
        while i < N {
            let mut tmp_1: libc::c_float = *X.offset(ix_1 as isize);
            let j_min_1: libc::c_int = if K > i { 0 as libc::c_int } else { i - K };
            let j_max_1: libc::c_int = i;
            let mut jx_1: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + j_min_1 * incX;
            j = j_min_1;
            while j < j_max_1 {
                let Aji: libc::c_float = *A.offset((i - j + lda * j) as isize);
                tmp_1 -= Aji * *X.offset(jx_1 as isize);
                jx_1 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X
                    .offset(
                        ix_1 as isize,
                    ) = tmp_1 / *A.offset((0 as libc::c_int + lda * i) as isize);
            } else {
                *X.offset(ix_1 as isize) = tmp_1;
            }
            ix_1 += incX;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasTrans as libc::c_int
        && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasNoTrans as libc::c_int
            && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
    {
        let mut ix_2: libc::c_int = (if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        }) + (N - 1 as libc::c_int) * incX;
        i = N;
        while i > 0 as libc::c_int
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let mut tmp_2: libc::c_float = *X.offset(ix_2 as isize);
            let j_min_2: libc::c_int = i + 1 as libc::c_int;
            let j_max_2: libc::c_int = if N < i + K + 1 as libc::c_int {
                N
            } else {
                i + K + 1 as libc::c_int
            };
            let mut jx_2: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + j_min_2 * incX;
            j = j_min_2;
            while j < j_max_2 {
                let Aji_0: libc::c_float = *A.offset((K + i - j + lda * j) as isize);
                tmp_2 -= Aji_0 * *X.offset(jx_2 as isize);
                jx_2 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix_2 as isize) = tmp_2 / *A.offset((K + lda * i) as isize);
            } else {
                *X.offset(ix_2 as isize) = tmp_2;
            }
            ix_2 -= incX;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_tbsv_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
