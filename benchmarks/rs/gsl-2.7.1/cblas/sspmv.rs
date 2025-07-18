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
pub unsafe extern "C" fn cblas_sspmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: libc::c_int,
    alpha: libc::c_float,
    mut Ap: *const libc::c_float,
    mut X: *const libc::c_float,
    incX: libc::c_int,
    beta: libc::c_float,
    mut Y: *mut libc::c_float,
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
    if incX == 0 as libc::c_int {
        pos = 7 as libc::c_int;
    }
    if incY == 0 as libc::c_int {
        pos = 10 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_spmv.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if alpha as libc::c_double == 0.0f64 && beta as libc::c_double == 1.0f64 {
        return;
    }
    if beta as libc::c_double == 0.0f64 {
        let mut iy: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            *Y.offset(iy as isize) = 0.0f64 as libc::c_float;
            iy += incY;
            i += 1;
            i;
        }
    } else if beta as libc::c_double != 1.0f64 {
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
    if alpha as libc::c_double == 0.0f64 {
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
            let mut tmp1: libc::c_float = alpha * *X.offset(ix as isize);
            let mut tmp2: libc::c_float = 0.0f64 as libc::c_float;
            let j_min: libc::c_int = i + 1 as libc::c_int;
            let j_max: libc::c_int = N;
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
                += tmp1
                    * *Ap
                        .offset(
                            ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + i - i) as isize,
                        );
            j = j_min;
            while j < j_max {
                let apk: libc::c_float = *Ap
                    .offset(
                        ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + j - i) as isize,
                    );
                *Y.offset(jy as isize) += tmp1 * apk;
                tmp2 += apk * *X.offset(jx as isize);
                jy += incY;
                jx += incX;
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
            let mut tmp1_0: libc::c_float = alpha * *X.offset(ix_0 as isize);
            let mut tmp2_0: libc::c_float = 0.0f64 as libc::c_float;
            let j_min_0: libc::c_int = 0 as libc::c_int;
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
            *Y.offset(iy_2 as isize)
                += tmp1_0
                    * *Ap
                        .offset(
                            (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i) as isize,
                        );
            j = j_min_0;
            while j < j_max_0 {
                let apk_0: libc::c_float = *Ap
                    .offset(
                        (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j) as isize,
                    );
                *Y.offset(jy_0 as isize) += tmp1_0 * apk_0;
                tmp2_0 += apk_0 * *X.offset(jx_0 as isize);
                jy_0 += incY;
                jx_0 += incX;
                j += 1;
                j;
            }
            *Y.offset(iy_2 as isize) += alpha * tmp2_0;
            ix_0 += incX;
            iy_2 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_spmv.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
