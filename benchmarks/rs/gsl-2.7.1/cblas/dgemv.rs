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
pub unsafe extern "C" fn cblas_dgemv(
    order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    M: libc::c_int,
    N: libc::c_int,
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
    let mut lenX: libc::c_int = 0;
    let mut lenY: libc::c_int = 0;
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
    if TransA as libc::c_uint != CblasNoTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasConjTrans as libc::c_int as libc::c_uint
    {
        pos = 2 as libc::c_int;
    }
    if M < 0 as libc::c_int {
        pos = 3 as libc::c_int;
    }
    if N < 0 as libc::c_int {
        pos = 4 as libc::c_int;
    }
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        if lda < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
            pos = 7 as libc::c_int;
        }
    } else if order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint {
        if lda < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
            pos = 7 as libc::c_int;
        }
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
            b"./source_gemv_r.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if M == 0 as libc::c_int || N == 0 as libc::c_int {
        return;
    }
    if alpha == 0.0f64 && beta == 1.0f64 {
        return;
    }
    if Trans == CblasNoTrans as libc::c_int {
        lenX = N;
        lenY = M;
    } else {
        lenX = M;
        lenY = N;
    }
    if beta == 0.0f64 {
        let mut iy: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenY - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < lenY {
            *Y.offset(iy as isize) = 0.0f64;
            iy += incY;
            i += 1;
            i;
        }
    } else if beta != 1.0f64 {
        let mut iy_0: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenY - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < lenY {
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
        && Trans == CblasNoTrans as libc::c_int
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasTrans as libc::c_int
    {
        let mut iy_1: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenY - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < lenY {
            let mut temp: libc::c_double = 0.0f64;
            let mut ix: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (lenX - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < lenX {
                temp += *X.offset(ix as isize) * *A.offset((lda * i + j) as isize);
                ix += incX;
                j += 1;
                j;
            }
            *Y.offset(iy_1 as isize) += alpha * temp;
            iy_1 += incY;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Trans == CblasTrans as libc::c_int
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Trans == CblasNoTrans as libc::c_int
    {
        let mut ix_0: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenX - 1 as libc::c_int) * -incX
        };
        j = 0 as libc::c_int;
        while j < lenX {
            let temp_0: libc::c_double = alpha * *X.offset(ix_0 as isize);
            if temp_0 != 0.0f64 {
                let mut iy_2: libc::c_int = if incY > 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (lenY - 1 as libc::c_int) * -incY
                };
                i = 0 as libc::c_int;
                while i < lenY {
                    *Y.offset(iy_2 as isize)
                        += temp_0 * *A.offset((lda * j + i) as isize);
                    iy_2 += incY;
                    i += 1;
                    i;
                }
            }
            ix_0 += incX;
            j += 1;
            j;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_gemv_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
