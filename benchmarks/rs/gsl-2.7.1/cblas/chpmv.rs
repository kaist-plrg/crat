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
pub unsafe extern "C" fn cblas_chpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut Ap: *const libc::c_void,
    mut X: *const libc::c_void,
    incX: libc::c_int,
    mut beta: *const libc::c_void,
    mut Y: *mut libc::c_void,
    incY: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let conj: libc::c_int = if order as libc::c_uint
        == CblasColMajor as libc::c_int as libc::c_uint
    {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
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
            b"./source_hpmv.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
    let beta_real: libc::c_float = *(beta as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let beta_imag: libc::c_float = *(beta as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
    if alpha_real as libc::c_double == 0.0f64 && alpha_imag as libc::c_double == 0.0f64
        && (beta_real as libc::c_double == 1.0f64
            && beta_imag as libc::c_double == 0.0f64)
    {
        return;
    }
    if beta_real as libc::c_double == 0.0f64 && beta_imag as libc::c_double == 0.0f64 {
        let mut iy: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
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
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
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
            let mut x_real: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix) as isize);
            let mut x_imag: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
            let mut temp1_real: libc::c_float = alpha_real * x_real
                - alpha_imag * x_imag;
            let mut temp1_imag: libc::c_float = alpha_real * x_imag
                + alpha_imag * x_real;
            let mut temp2_real: libc::c_float = 0.0f64 as libc::c_float;
            let mut temp2_imag: libc::c_float = 0.0f64 as libc::c_float;
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
            let mut Aii_real: libc::c_float = *(Ap as *const libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + i - i)) as isize,
                );
            *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_1) as isize)
                += temp1_real * Aii_real;
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_1 + 1 as libc::c_int) as isize)
                += temp1_imag * Aii_real;
            j = j_min;
            while j < j_max {
                let mut Aij_real: libc::c_float = *(Ap as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + j - i)) as isize,
                    );
                let mut Aij_imag: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int
                                * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                    * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                    / 2 as libc::c_int + j - i) + 1 as libc::c_int) as isize,
                        );
                *(Y as *mut libc::c_float).offset((2 as libc::c_int * jy) as isize)
                    += temp1_real * Aij_real - temp1_imag * -Aij_imag;
                *(Y as *mut libc::c_float)
                    .offset((2 as libc::c_int * jy + 1 as libc::c_int) as isize)
                    += temp1_real * -Aij_imag + temp1_imag * Aij_real;
                x_real = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx) as isize);
                x_imag = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx + 1 as libc::c_int) as isize);
                temp2_real += x_real * Aij_real - x_imag * Aij_imag;
                temp2_imag += x_real * Aij_imag + x_imag * Aij_real;
                jx += incX;
                jy += incY;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_1) as isize)
                += alpha_real * temp2_real - alpha_imag * temp2_imag;
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_1 + 1 as libc::c_int) as isize)
                += alpha_real * temp2_imag + alpha_imag * temp2_real;
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
            let mut x_real_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_0) as isize);
            let mut x_imag_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
            let mut temp1_real_0: libc::c_float = alpha_real * x_real_0
                - alpha_imag * x_imag_0;
            let mut temp1_imag_0: libc::c_float = alpha_real * x_imag_0
                + alpha_imag * x_real_0;
            let mut temp2_real_0: libc::c_float = 0.0f64 as libc::c_float;
            let mut temp2_imag_0: libc::c_float = 0.0f64 as libc::c_float;
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
            let mut Aii_real_0: libc::c_float = *(Ap as *const libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)) as isize,
                );
            *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_2) as isize)
                += temp1_real_0 * Aii_real_0;
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_2 + 1 as libc::c_int) as isize)
                += temp1_imag_0 * Aii_real_0;
            j = j_min_0;
            while j < j_max_0 {
                let mut Aij_real_0: libc::c_float = *(Ap as *const libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j))
                            as isize,
                    );
                let mut Aij_imag_0: libc::c_float = conj as libc::c_float
                    * *(Ap as *const libc::c_float)
                        .offset(
                            (2 as libc::c_int
                                * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j)
                                + 1 as libc::c_int) as isize,
                        );
                *(Y as *mut libc::c_float).offset((2 as libc::c_int * jy_0) as isize)
                    += temp1_real_0 * Aij_real_0 - temp1_imag_0 * -Aij_imag_0;
                *(Y as *mut libc::c_float)
                    .offset((2 as libc::c_int * jy_0 + 1 as libc::c_int) as isize)
                    += temp1_real_0 * -Aij_imag_0 + temp1_imag_0 * Aij_real_0;
                x_real_0 = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx_0) as isize);
                x_imag_0 = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx_0 + 1 as libc::c_int) as isize);
                temp2_real_0 += x_real_0 * Aij_real_0 - x_imag_0 * Aij_imag_0;
                temp2_imag_0 += x_real_0 * Aij_imag_0 + x_imag_0 * Aij_real_0;
                jx_0 += incX;
                jy_0 += incY;
                j += 1;
                j;
            }
            *(Y as *mut libc::c_float).offset((2 as libc::c_int * iy_2) as isize)
                += alpha_real * temp2_real_0 - alpha_imag * temp2_imag_0;
            *(Y as *mut libc::c_float)
                .offset((2 as libc::c_int * iy_2 + 1 as libc::c_int) as isize)
                += alpha_real * temp2_imag_0 + alpha_imag * temp2_real_0;
            ix_0 += incX;
            iy_2 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_hpmv.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
