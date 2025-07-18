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
pub unsafe extern "C" fn cblas_dtrsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: libc::c_int,
    mut A: *const libc::c_double,
    lda: libc::c_int,
    mut X: *mut libc::c_double,
    incX: libc::c_int,
) {
    let nonunit: libc::c_int = (Diag as libc::c_uint
        == CblasNonUnit as libc::c_int as libc::c_uint) as libc::c_int;
    let mut ix: libc::c_int = 0;
    let mut jx: libc::c_int = 0;
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
    if lda < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
        pos = 7 as libc::c_int;
    }
    if incX == 0 as libc::c_int {
        pos = 9 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_trsv_r.h\0" as *const u8 as *const libc::c_char,
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
        ix = (if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        }) + incX * (N - 1 as libc::c_int);
        if nonunit != 0 {
            *X
                .offset(
                    ix as isize,
                ) = *X.offset(ix as isize)
                / *A
                    .offset(
                        (lda * (N - 1 as libc::c_int) + (N - 1 as libc::c_int)) as isize,
                    );
        }
        ix -= incX;
        i = N - 1 as libc::c_int;
        while i > 0 as libc::c_int
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let mut tmp: libc::c_double = *X.offset(ix as isize);
            jx = ix + incX;
            j = i + 1 as libc::c_int;
            while j < N {
                let Aij: libc::c_double = *A.offset((lda * i + j) as isize);
                tmp -= Aij * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp / *A.offset((lda * i + i) as isize);
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
        ix = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        if nonunit != 0 {
            *X
                .offset(
                    ix as isize,
                ) = *X.offset(ix as isize)
                / *A.offset((lda * 0 as libc::c_int + 0 as libc::c_int) as isize);
        }
        ix += incX;
        i = 1 as libc::c_int;
        while i < N {
            let mut tmp_0: libc::c_double = *X.offset(ix as isize);
            jx = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                let Aij_0: libc::c_double = *A.offset((lda * i + j) as isize);
                tmp_0 -= Aij_0 * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp_0 / *A.offset((lda * i + i) as isize);
            } else {
                *X.offset(ix as isize) = tmp_0;
            }
            ix += incX;
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
        ix = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        if nonunit != 0 {
            *X
                .offset(
                    ix as isize,
                ) = *X.offset(ix as isize)
                / *A.offset((lda * 0 as libc::c_int + 0 as libc::c_int) as isize);
        }
        ix += incX;
        i = 1 as libc::c_int;
        while i < N {
            let mut tmp_1: libc::c_double = *X.offset(ix as isize);
            jx = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                let Aji: libc::c_double = *A.offset((lda * j + i) as isize);
                tmp_1 -= Aji * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp_1 / *A.offset((lda * i + i) as isize);
            } else {
                *X.offset(ix as isize) = tmp_1;
            }
            ix += incX;
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
        ix = (if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        }) + (N - 1 as libc::c_int) * incX;
        if nonunit != 0 {
            *X
                .offset(
                    ix as isize,
                ) = *X.offset(ix as isize)
                / *A
                    .offset(
                        (lda * (N - 1 as libc::c_int) + (N - 1 as libc::c_int)) as isize,
                    );
        }
        ix -= incX;
        i = N - 1 as libc::c_int;
        while i > 0 as libc::c_int
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let mut tmp_2: libc::c_double = *X.offset(ix as isize);
            jx = ix + incX;
            j = i + 1 as libc::c_int;
            while j < N {
                let Aji_0: libc::c_double = *A.offset((lda * j + i) as isize);
                tmp_2 -= Aji_0 * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                *X.offset(ix as isize) = tmp_2 / *A.offset((lda * i + i) as isize);
            } else {
                *X.offset(ix as isize) = tmp_2;
            }
            ix -= incX;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_trsv_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
