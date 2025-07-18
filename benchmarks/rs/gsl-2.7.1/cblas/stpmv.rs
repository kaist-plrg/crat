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
pub unsafe extern "C" fn cblas_stpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: libc::c_int,
    mut Ap: *const libc::c_float,
    mut X: *mut libc::c_float,
    incX: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let nonunit: libc::c_int = (Diag as libc::c_uint
        == CblasNonUnit as libc::c_int as libc::c_uint) as libc::c_int;
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
    if incX == 0 as libc::c_int {
        pos = 8 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_tpmv_r.h\0" as *const u8 as *const libc::c_char,
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
        let mut ix: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        i = 0 as libc::c_int;
        while i < N {
            let mut atmp: libc::c_float = *Ap
                .offset(
                    ((i - 1 as libc::c_int + 1 as libc::c_int)
                        * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                        / 2 as libc::c_int + i - i) as isize,
                );
            let mut temp: libc::c_float = if nonunit != 0 {
                *X.offset(ix as isize) * atmp
            } else {
                *X.offset(ix as isize)
            };
            let mut jx: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + (i + 1 as libc::c_int) * incX;
            j = i + 1 as libc::c_int;
            while j < N {
                atmp = *Ap
                    .offset(
                        ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + j - i) as isize,
                    );
                temp += atmp * *X.offset(jx as isize);
                jx += incX;
                j += 1;
                j;
            }
            *X.offset(ix as isize) = temp;
            ix += incX;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasNoTrans as libc::c_int
        && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasTrans as libc::c_int
            && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
    {
        let mut ix_0: libc::c_int = (if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        }) + (N - 1 as libc::c_int) * incX;
        i = N;
        while i > 0 as libc::c_int
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let mut atmp_0: libc::c_float = *Ap
                .offset((i * (i + 1 as libc::c_int) / 2 as libc::c_int + i) as isize);
            let mut temp_0: libc::c_float = if nonunit != 0 {
                *X.offset(ix_0 as isize) * atmp_0
            } else {
                *X.offset(ix_0 as isize)
            };
            let mut jx_0: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                atmp_0 = *Ap
                    .offset(
                        (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j) as isize,
                    );
                temp_0 += atmp_0 * *X.offset(jx_0 as isize);
                jx_0 += incX;
                j += 1;
                j;
            }
            *X.offset(ix_0 as isize) = temp_0;
            ix_0 -= incX;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasTrans as libc::c_int
        && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasNoTrans as libc::c_int
            && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
    {
        let mut ix_1: libc::c_int = (if incX > 0 as libc::c_int {
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
            let mut atmp_1: libc::c_float = *Ap
                .offset(
                    ((i - 1 as libc::c_int + 1 as libc::c_int)
                        * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                        / 2 as libc::c_int + i - i) as isize,
                );
            let mut temp_1: libc::c_float = if nonunit != 0 {
                *X.offset(ix_1 as isize) * atmp_1
            } else {
                *X.offset(ix_1 as isize)
            };
            let mut jx_1: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                atmp_1 = *Ap
                    .offset(
                        ((j - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (j - 1 as libc::c_int))
                            / 2 as libc::c_int + i - j) as isize,
                    );
                temp_1 += atmp_1 * *X.offset(jx_1 as isize);
                jx_1 += incX;
                j += 1;
                j;
            }
            *X.offset(ix_1 as isize) = temp_1;
            ix_1 -= incX;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasTrans as libc::c_int
        && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasNoTrans as libc::c_int
            && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
    {
        let mut ix_2: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        i = 0 as libc::c_int;
        while i < N {
            let mut atmp_2: libc::c_float = *Ap
                .offset((i * (i + 1 as libc::c_int) / 2 as libc::c_int + i) as isize);
            let mut temp_2: libc::c_float = if nonunit != 0 {
                *X.offset(ix_2 as isize) * atmp_2
            } else {
                *X.offset(ix_2 as isize)
            };
            let mut jx_2: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + (i + 1 as libc::c_int) * incX;
            j = i + 1 as libc::c_int;
            while j < N {
                atmp_2 = *Ap
                    .offset(
                        (j * (j + 1 as libc::c_int) / 2 as libc::c_int + i) as isize,
                    );
                temp_2 += atmp_2 * *X.offset(jx_2 as isize);
                jx_2 += incX;
                j += 1;
                j;
            }
            *X.offset(ix_2 as isize) = temp_2;
            ix_2 += incX;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_tpmv_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
