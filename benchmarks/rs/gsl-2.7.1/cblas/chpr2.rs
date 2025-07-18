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
pub unsafe extern "C" fn cblas_chpr2(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut X: *const libc::c_void,
    incX: libc::c_int,
    mut Y: *const libc::c_void,
    incY: libc::c_int,
    mut Ap: *mut libc::c_void,
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
        pos = 6 as libc::c_int;
    }
    if incY == 0 as libc::c_int {
        pos = 8 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_hpr2.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
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
        let mut iy: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            let Xi_real: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix) as isize);
            let Xi_imag: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
            let tmp1_real: libc::c_float = alpha_real * Xi_real - alpha_imag * Xi_imag;
            let tmp1_imag: libc::c_float = alpha_imag * Xi_real + alpha_real * Xi_imag;
            let Yi_real: libc::c_float = *(Y as *const libc::c_float)
                .offset((2 as libc::c_int * iy) as isize);
            let Yi_imag: libc::c_float = *(Y as *const libc::c_float)
                .offset((2 as libc::c_int * iy + 1 as libc::c_int) as isize);
            let tmp2_real: libc::c_float = alpha_real * Yi_real + alpha_imag * Yi_imag;
            let tmp2_imag: libc::c_float = -alpha_imag * Yi_real + alpha_real * Yi_imag;
            let mut jx: libc::c_int = ix + incX;
            let mut jy: libc::c_int = iy + incY;
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + i - i)) as isize,
                )
                += 2 as libc::c_int as libc::c_float
                    * (tmp1_real * Yi_real + tmp1_imag * Yi_imag);
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + i - i) + 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_float;
            j = i + 1 as libc::c_int;
            while j < N {
                let Xj_real: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx) as isize);
                let Xj_imag: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx + 1 as libc::c_int) as isize);
                let Yj_real: libc::c_float = *(Y as *const libc::c_float)
                    .offset((2 as libc::c_int * jy) as isize);
                let Yj_imag: libc::c_float = *(Y as *const libc::c_float)
                    .offset((2 as libc::c_int * jy + 1 as libc::c_int) as isize);
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + j - i)) as isize,
                    )
                    += tmp1_real * Yj_real + tmp1_imag * Yj_imag
                        + (tmp2_real * Xj_real + tmp2_imag * Xj_imag);
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + j - i) + 1 as libc::c_int) as isize,
                    )
                    += conj as libc::c_float
                        * (tmp1_imag * Yj_real - tmp1_real * Yj_imag
                            + (tmp2_imag * Xj_real - tmp2_real * Xj_imag));
                jx += incX;
                jy += incY;
                j += 1;
                j;
            }
            ix += incX;
            iy += incY;
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
        let mut iy_0: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            let Xi_real_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_0) as isize);
            let Xi_imag_0: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
            let tmp1_real_0: libc::c_float = alpha_real * Xi_real_0
                - alpha_imag * Xi_imag_0;
            let tmp1_imag_0: libc::c_float = alpha_imag * Xi_real_0
                + alpha_real * Xi_imag_0;
            let Yi_real_0: libc::c_float = *(Y as *const libc::c_float)
                .offset((2 as libc::c_int * iy_0) as isize);
            let Yi_imag_0: libc::c_float = *(Y as *const libc::c_float)
                .offset((2 as libc::c_int * iy_0 + 1 as libc::c_int) as isize);
            let tmp2_real_0: libc::c_float = alpha_real * Yi_real_0
                + alpha_imag * Yi_imag_0;
            let tmp2_imag_0: libc::c_float = -alpha_imag * Yi_real_0
                + alpha_real * Yi_imag_0;
            let mut jx_0: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            let mut jy_0: libc::c_int = if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incY
            };
            j = 0 as libc::c_int;
            while j < i {
                let Xj_real_0: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx_0) as isize);
                let Xj_imag_0: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx_0 + 1 as libc::c_int) as isize);
                let Yj_real_0: libc::c_float = *(Y as *const libc::c_float)
                    .offset((2 as libc::c_int * jy_0) as isize);
                let Yj_imag_0: libc::c_float = *(Y as *const libc::c_float)
                    .offset((2 as libc::c_int * jy_0 + 1 as libc::c_int) as isize);
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j))
                            as isize,
                    )
                    += tmp1_real_0 * Yj_real_0 + tmp1_imag_0 * Yj_imag_0
                        + (tmp2_real_0 * Xj_real_0 + tmp2_imag_0 * Xj_imag_0);
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j)
                            + 1 as libc::c_int) as isize,
                    )
                    += conj as libc::c_float
                        * (tmp1_imag_0 * Yj_real_0 - tmp1_real_0 * Yj_imag_0
                            + (tmp2_imag_0 * Xj_real_0 - tmp2_real_0 * Xj_imag_0));
                jx_0 += incX;
                jy_0 += incY;
                j += 1;
                j;
            }
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)) as isize,
                )
                += 2 as libc::c_int as libc::c_float
                    * (tmp1_real_0 * Yi_real_0 + tmp1_imag_0 * Yi_imag_0);
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)
                        + 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_float;
            ix_0 += incX;
            iy_0 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_hpr2.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
