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
pub unsafe extern "C" fn cblas_chpr(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: libc::c_int,
    alpha: libc::c_float,
    mut X: *const libc::c_void,
    incX: libc::c_int,
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
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_hpr.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
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
        i = 0 as libc::c_int;
        while i < N {
            let tmp_real: libc::c_float = alpha
                * *(X as *const libc::c_float).offset((2 as libc::c_int * ix) as isize);
            let tmp_imag: libc::c_float = alpha * conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
            let mut jx: libc::c_int = ix;
            let X_real: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * jx) as isize);
            let X_imag: libc::c_float = -conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx + 1 as libc::c_int) as isize);
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + i - i)) as isize,
                ) += X_real * tmp_real - X_imag * tmp_imag;
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + i - i) + 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_float;
            jx += incX;
            j = i + 1 as libc::c_int;
            while j < N {
                let X_real_0: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx) as isize);
                let X_imag_0: libc::c_float = -conj as libc::c_float
                    * *(X as *const libc::c_float)
                        .offset((2 as libc::c_int * jx + 1 as libc::c_int) as isize);
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + j - i)) as isize,
                    ) += X_real_0 * tmp_real - X_imag_0 * tmp_imag;
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + j - i) + 1 as libc::c_int) as isize,
                    ) += X_imag_0 * tmp_real + X_real_0 * tmp_imag;
                jx += incX;
                j += 1;
                j;
            }
            ix += incX;
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
        i = 0 as libc::c_int;
        while i < N {
            let tmp_real_0: libc::c_float = alpha
                * *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix_0) as isize);
            let tmp_imag_0: libc::c_float = alpha * conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
            let mut jx_0: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                let X_real_1: libc::c_float = *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx_0) as isize);
                let X_imag_1: libc::c_float = -conj as libc::c_float
                    * *(X as *const libc::c_float)
                        .offset((2 as libc::c_int * jx_0 + 1 as libc::c_int) as isize);
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j))
                            as isize,
                    ) += X_real_1 * tmp_real_0 - X_imag_1 * tmp_imag_0;
                *(Ap as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j)
                            + 1 as libc::c_int) as isize,
                    ) += X_imag_1 * tmp_real_0 + X_real_1 * tmp_imag_0;
                jx_0 += incX;
                j += 1;
                j;
            }
            let X_real_2: libc::c_float = *(X as *const libc::c_float)
                .offset((2 as libc::c_int * jx_0) as isize);
            let X_imag_2: libc::c_float = -conj as libc::c_float
                * *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * jx_0 + 1 as libc::c_int) as isize);
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)) as isize,
                ) += X_real_2 * tmp_real_0 - X_imag_2 * tmp_imag_0;
            *(Ap as *mut libc::c_float)
                .offset(
                    (2 as libc::c_int
                        * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)
                        + 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_float;
            jx_0 += incX;
            ix_0 += incX;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_hpr.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
