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
pub unsafe extern "C" fn cblas_cgemv(
    order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    M: libc::c_int,
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: libc::c_int,
    mut X: *const libc::c_void,
    incX: libc::c_int,
    mut beta: *const libc::c_void,
    mut Y: *mut libc::c_void,
    incY: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lenX: libc::c_int = 0;
    let mut lenY: libc::c_int = 0;
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
    let beta_real: libc::c_float = *(beta as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let beta_imag: libc::c_float = *(beta as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
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
            b"./source_gemv_c.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if M == 0 as libc::c_int || N == 0 as libc::c_int {
        return;
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64
        && (beta_real as libc::c_double == 1.0f64
            && beta_imag as libc::c_double == 0.0f64)
    {
        return;
    }
    if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
        lenX = N;
        lenY = M;
    } else {
        lenX = M;
        lenY = N;
    }
    if beta_real as libc::c_double == 0.0f64 && beta_imag as libc::c_double == 0.0f64 {
        let mut iy: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenY - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < lenY {
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy) as isize) = 0.0f64 as libc::c_float;
            *(Y as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int * iy + 1 as libc::c_int) as isize,
                ) = 0.0f64 as libc::c_float;
            iy += incY;
            i += 1;
            i;
        }
    } else if !(beta_real as libc::c_double == 1.0f64
        && beta_imag as libc::c_double == 0.0f64)
    {
        let mut iy_0: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenY - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < lenY {
            let y_real: libc::c_float = *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_0) as isize);
            let y_imag: libc::c_float = *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_0 + 1 as libc::c_int) as isize);
            let tmpR: libc::c_float = y_real * beta_real - y_imag * beta_imag;
            let tmpI: libc::c_float = y_real * beta_imag + y_imag * beta_real;
            *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_0) as isize) = tmpR;
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_0 + 1 as libc::c_int) as isize) = tmpI;
            iy_0 += incY;
            i += 1;
            i;
        }
    }
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64 {
        return;
    }
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
    {
        let mut iy_1: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenY - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < lenY {
            let mut dotR: libc::c_float = 0.0f64 as libc::c_float;
            let mut dotI: libc::c_float = 0.0f64 as libc::c_float;
            let mut ix: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (lenX - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < lenX {
                let x_real: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix) as isize);
                let x_imag: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
                let A_real: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (lda * i + j)) as isize);
                let A_imag: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (lda * i + j) + 1 as libc::c_int) as isize,
                    );
                dotR += A_real * x_real - A_imag * x_imag;
                dotI += A_real * x_imag + A_imag * x_real;
                ix += incX;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_1) as isize)
                += alpha_real * dotR - alpha_imag * dotI;
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_1 + 1 as libc::c_int) as isize)
                += alpha_real * dotI + alpha_imag * dotR;
            iy_1 += incY;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
    {
        let mut ix_0: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenX - 1 as libc::c_int) * -incX
        };
        j = 0 as libc::c_int;
        while j < lenX {
            let mut x_real_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_0) as isize);
            let mut x_imag_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
            let mut tmpR_0: libc::c_float = alpha_real * x_real_0
                - alpha_imag * x_imag_0;
            let mut tmpI_0: libc::c_float = alpha_real * x_imag_0
                + alpha_imag * x_real_0;
            let mut iy_2: libc::c_int = if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (lenY - 1 as libc::c_int) * -incY
            };
            i = 0 as libc::c_int;
            while i < lenY {
                let A_real_0: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (lda * j + i)) as isize);
                let A_imag_0: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (lda * j + i) + 1 as libc::c_int) as isize,
                    );
                *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_2) as isize)
                    += A_real_0 * tmpR_0 - A_imag_0 * tmpI_0;
                *(Y as *mut libc::c_float)
                    .offset((2 as libc::c_int * iy_2 + 1 as libc::c_int) as isize)
                    += A_real_0 * tmpI_0 + A_imag_0 * tmpR_0;
                iy_2 += incY;
                i += 1;
                i;
            }
            ix_0 += incX;
            j += 1;
            j;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && TransA as libc::c_uint == CblasConjTrans as libc::c_int as libc::c_uint
    {
        let mut ix_1: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenX - 1 as libc::c_int) * -incX
        };
        j = 0 as libc::c_int;
        while j < lenX {
            let mut x_real_1: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_1) as isize);
            let mut x_imag_1: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize);
            let mut tmpR_1: libc::c_float = alpha_real * x_real_1
                - alpha_imag * x_imag_1;
            let mut tmpI_1: libc::c_float = alpha_real * x_imag_1
                + alpha_imag * x_real_1;
            let mut iy_3: libc::c_int = if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (lenY - 1 as libc::c_int) * -incY
            };
            i = 0 as libc::c_int;
            while i < lenY {
                let A_real_1: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (lda * j + i)) as isize);
                let A_imag_1: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (lda * j + i) + 1 as libc::c_int) as isize,
                    );
                *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_3) as isize)
                    += A_real_1 * tmpR_1 - -A_imag_1 * tmpI_1;
                *(Y as *mut libc::c_float)
                    .offset((2 as libc::c_int * iy_3 + 1 as libc::c_int) as isize)
                    += A_real_1 * tmpI_1 + -A_imag_1 * tmpR_1;
                iy_3 += incY;
                i += 1;
                i;
            }
            ix_1 += incX;
            j += 1;
            j;
        }
    } else if order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
        && TransA as libc::c_uint == CblasConjTrans as libc::c_int as libc::c_uint
    {
        let mut iy_4: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (lenY - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < lenY {
            let mut dotR_0: libc::c_float = 0.0f64 as libc::c_float;
            let mut dotI_0: libc::c_float = 0.0f64 as libc::c_float;
            let mut ix_2: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (lenX - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < lenX {
                let x_real_2: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix_2) as isize);
                let x_imag_2: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize);
                let A_real_2: libc::c_float = *(A as *const libc::c_float)
                    .offset((2 as libc::c_int * (lda * i + j)) as isize);
                let A_imag_2: libc::c_float = *(A as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int * (lda * i + j) + 1 as libc::c_int) as isize,
                    );
                dotR_0 += A_real_2 * x_real_2 - -A_imag_2 * x_imag_2;
                dotI_0 += A_real_2 * x_imag_2 + -A_imag_2 * x_real_2;
                ix_2 += incX;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_4) as isize)
                += alpha_real * dotR_0 - alpha_imag * dotI_0;
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_4 + 1 as libc::c_int) as isize)
                += alpha_real * dotI_0 + alpha_imag * dotR_0;
            iy_4 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_gemv_c.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
