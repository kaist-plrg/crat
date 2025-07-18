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
pub unsafe extern "C" fn cblas_dsbmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: libc::c_int,
    K: libc::c_int,
    alpha: libc::c_double,
    mut A: *const libc::c_double,
    lda: libc::c_int,
    mut X: *const libc::c_double,
    incX: libc::c_int,
    beta: libc::c_double,
    mut Y: *mut libc::c_double,
    incY: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
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
    if N < 0 as libc::c_int {
        pos = 3 as libc::c_int;
    }
    if K < 0 as libc::c_int {
        pos = 4 as libc::c_int;
    }
    if lda
        < (if 1 as libc::c_int > K + 1 as libc::c_int {
            1 as libc::c_int
        } else {
            K + 1 as libc::c_int
        })
    {
        pos = 7 as libc::c_int;
    }
    if incX == 0 as libc::c_int {
        pos = 9 as libc::c_int;
    }
    if incY == 0 as libc::c_int {
        pos = 12 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_sbmv.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if N == 0 as libc::c_int {
        return;
    }
    if alpha == 0.0f64 && beta == 1.0f64 {
        return;
    }
    if beta == 0.0f64 {
        let mut iy: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            *Y.offset(iy as isize) = 0.0f64;
            iy += incY;
            i += 1;
            i;
        }
    } else if beta != 1.0f64 {
        let mut iy_0: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            *Y.offset(iy_0 as isize) *= beta;
            iy_0 += incY;
            i += 1;
            i;
        }
    }
    if alpha == 0.0f64 {
        return;
    }
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
    {
        let mut ix: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        let mut iy_1: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            let mut tmp1: libc::c_double = alpha * *X.offset(ix as isize);
            let mut tmp2: libc::c_double = 0.0f64;
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
            let mut jy: libc::c_int = (if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incY
            }) + j_min * incY;
            *Y.offset(iy_1 as isize)
                += tmp1 * *A.offset((0 as libc::c_int + i * lda) as isize);
            j = j_min;
            while j < j_max {
                let mut Aij: libc::c_double = *A.offset((j - i + i * lda) as isize);
                *Y.offset(jy as isize) += tmp1 * Aij;
                tmp2 += Aij * *X.offset(jx as isize);
                jx += incX;
                jy += incY;
                j += 1;
                j;
            }
            *Y.offset(iy_1 as isize) += alpha * tmp2;
            ix += incX;
            iy_1 += incY;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
    {
        let mut ix_0: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        let mut iy_2: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            let mut tmp1_0: libc::c_double = alpha * *X.offset(ix_0 as isize);
            let mut tmp2_0: libc::c_double = 0.0f64;
            let j_min_0: libc::c_int = if i > K { i - K } else { 0 as libc::c_int };
            let j_max_0: libc::c_int = i;
            let mut jx_0: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + j_min_0 * incX;
            let mut jy_0: libc::c_int = (if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incY
            }) + j_min_0 * incY;
            j = j_min_0;
            while j < j_max_0 {
                let mut Aij_0: libc::c_double = *A
                    .offset((K - i + j + i * lda) as isize);
                *Y.offset(jy_0 as isize) += tmp1_0 * Aij_0;
                tmp2_0 += Aij_0 * *X.offset(jx_0 as isize);
                jx_0 += incX;
                jy_0 += incY;
                j += 1;
                j;
            }
            *Y.offset(iy_2 as isize)
                += tmp1_0 * *A.offset((K + i * lda) as isize) + alpha * tmp2_0;
            ix_0 += incX;
            iy_2 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_sbmv.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
