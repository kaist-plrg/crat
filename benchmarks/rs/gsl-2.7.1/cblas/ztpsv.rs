use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
unsafe extern "C" fn xhypot(x: libc::c_double, y: libc::c_double) -> libc::c_double {
    let mut xabs: libc::c_double = fabs(x);
    let mut yabs: libc::c_double = fabs(y);
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    if xabs < yabs {
        min = xabs;
        max = yabs;
    } else {
        min = yabs;
        max = xabs;
    }
    if min == 0 as libc::c_int as libc::c_double {
        return max;
    }
    let mut u: libc::c_double = min / max;
    return max * sqrt(1 as libc::c_int as libc::c_double + u * u);
}
pub unsafe extern "C" fn cblas_ztpsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: libc::c_int,
    mut Ap: *const libc::c_void,
    mut X: *mut libc::c_void,
    incX: libc::c_int,
) {
    let conj: libc::c_int = if TransA as libc::c_uint
        == CblasConjTrans as libc::c_int as libc::c_uint
    {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    let Trans: libc::c_int = (if TransA as libc::c_uint
        != CblasConjTrans as libc::c_int as libc::c_uint
    {
        TransA as libc::c_uint
    } else {
        CblasTrans as libc::c_int as libc::c_uint
    }) as libc::c_int;
    let nonunit: libc::c_int = (Diag as libc::c_uint
        == CblasNonUnit as libc::c_int as libc::c_uint) as libc::c_int;
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
            b"./source_tpsv_c.h\0" as *const u8 as *const libc::c_char,
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
        if nonunit != 0 {
            let a_real: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * ((N - 1 as libc::c_int - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N
                                - (N - 1 as libc::c_int - 1 as libc::c_int))
                            / 2 as libc::c_int + (N - 1 as libc::c_int)
                            - (N - 1 as libc::c_int))) as isize,
                );
            let a_imag: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((N - 1 as libc::c_int - 1 as libc::c_int
                                + 1 as libc::c_int)
                                * (2 as libc::c_int * N
                                    - (N - 1 as libc::c_int - 1 as libc::c_int))
                                / 2 as libc::c_int + (N - 1 as libc::c_int)
                                - (N - 1 as libc::c_int)) + 1 as libc::c_int) as isize,
                    );
            let x_real: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix) as isize);
            let x_imag: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
            let s: libc::c_double = xhypot(a_real, a_imag);
            let b_real: libc::c_double = a_real / s;
            let b_imag: libc::c_double = a_imag / s;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix) as isize,
                ) = (x_real * b_real + x_imag * b_imag) / s;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix + 1 as libc::c_int) as isize,
                ) = (x_imag * b_real - b_imag * x_real) / s;
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
            let mut tmp_real: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix) as isize);
            let mut tmp_imag: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
            let mut jx: libc::c_int = ix + incX;
            j = i + 1 as libc::c_int;
            while j < N {
                let Aij_real: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + j - i)) as isize,
                    );
                let Aij_imag: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                    * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                    / 2 as libc::c_int + j - i) + 1 as libc::c_int) as isize,
                        );
                let x_real_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx) as isize);
                let x_imag_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx + 1 as libc::c_int) as isize);
                tmp_real -= Aij_real * x_real_0 - Aij_imag * x_imag_0;
                tmp_imag -= Aij_real * x_imag_0 + Aij_imag * x_real_0;
                jx += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_0: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + i - i)) as isize,
                    );
                let a_imag_0: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                    * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                    / 2 as libc::c_int + i - i) + 1 as libc::c_int) as isize,
                        );
                let s_0: libc::c_double = xhypot(a_real_0, a_imag_0);
                let b_real_0: libc::c_double = a_real_0 / s_0;
                let b_imag_0: libc::c_double = a_imag_0 / s_0;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix) as isize,
                    ) = (tmp_real * b_real_0 + tmp_imag * b_imag_0) / s_0;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix + 1 as libc::c_int) as isize,
                    ) = (tmp_imag * b_real_0 - tmp_real * b_imag_0) / s_0;
            } else {
                *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix) as isize) = tmp_real;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix + 1 as libc::c_int) as isize,
                    ) = tmp_imag;
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
        if nonunit != 0 {
            let a_real_1: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * (0 as libc::c_int * (0 as libc::c_int + 1 as libc::c_int)
                            / 2 as libc::c_int + 0 as libc::c_int)) as isize,
                );
            let a_imag_1: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (0 as libc::c_int * (0 as libc::c_int + 1 as libc::c_int)
                                / 2 as libc::c_int + 0 as libc::c_int) + 1 as libc::c_int)
                            as isize,
                    );
            let x_real_1: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_0) as isize);
            let x_imag_1: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
            let s_1: libc::c_double = xhypot(a_real_1, a_imag_1);
            let b_real_1: libc::c_double = a_real_1 / s_1;
            let b_imag_1: libc::c_double = a_imag_1 / s_1;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_0) as isize,
                ) = (x_real_1 * b_real_1 + x_imag_1 * b_imag_1) / s_1;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize,
                ) = (x_imag_1 * b_real_1 - b_imag_1 * x_real_1) / s_1;
        }
        ix_0 += incX;
        i = 1 as libc::c_int;
        while i < N {
            let mut tmp_real_0: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_0) as isize);
            let mut tmp_imag_0: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
            let mut jx_0: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                let Aij_real_0: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j))
                            as isize,
                    );
                let Aij_imag_0: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j)
                                + 1 as libc::c_int) as isize,
                        );
                let x_real_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_0) as isize);
                let x_imag_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_0 + 1 as libc::c_int) as isize);
                tmp_real_0 -= Aij_real_0 * x_real_2 - Aij_imag_0 * x_imag_2;
                tmp_imag_0 -= Aij_real_0 * x_imag_2 + Aij_imag_0 * x_real_2;
                jx_0 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_2: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i))
                            as isize,
                    );
                let a_imag_2: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)
                                + 1 as libc::c_int) as isize,
                        );
                let s_2: libc::c_double = xhypot(a_real_2, a_imag_2);
                let b_real_2: libc::c_double = a_real_2 / s_2;
                let b_imag_2: libc::c_double = a_imag_2 / s_2;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_0) as isize,
                    ) = (tmp_real_0 * b_real_2 + tmp_imag_0 * b_imag_2) / s_2;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize,
                    ) = (tmp_imag_0 * b_real_2 - tmp_real_0 * b_imag_2) / s_2;
            } else {
                *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_0) as isize) = tmp_real_0;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize,
                    ) = tmp_imag_0;
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
        if nonunit != 0 {
            let a_real_3: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * ((0 as libc::c_int - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N
                                - (0 as libc::c_int - 1 as libc::c_int)) / 2 as libc::c_int
                            + 0 as libc::c_int - 0 as libc::c_int)) as isize,
                );
            let a_imag_3: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((0 as libc::c_int - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N
                                    - (0 as libc::c_int - 1 as libc::c_int)) / 2 as libc::c_int
                                + 0 as libc::c_int - 0 as libc::c_int) + 1 as libc::c_int)
                            as isize,
                    );
            let x_real_3: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_1) as isize);
            let x_imag_3: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize);
            let s_3: libc::c_double = xhypot(a_real_3, a_imag_3);
            let b_real_3: libc::c_double = a_real_3 / s_3;
            let b_imag_3: libc::c_double = a_imag_3 / s_3;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_1) as isize,
                ) = (x_real_3 * b_real_3 + x_imag_3 * b_imag_3) / s_3;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize,
                ) = (x_imag_3 * b_real_3 - b_imag_3 * x_real_3) / s_3;
        }
        ix_1 += incX;
        i = 1 as libc::c_int;
        while i < N {
            let mut tmp_real_1: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_1) as isize);
            let mut tmp_imag_1: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize);
            let mut jx_1: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                let Aij_real_1: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((j - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (j - 1 as libc::c_int))
                                / 2 as libc::c_int + i - j)) as isize,
                    );
                let Aij_imag_1: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * ((j - 1 as libc::c_int + 1 as libc::c_int)
                                    * (2 as libc::c_int * N - (j - 1 as libc::c_int))
                                    / 2 as libc::c_int + i - j) + 1 as libc::c_int) as isize,
                        );
                let x_real_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_1) as isize);
                let x_imag_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_1 + 1 as libc::c_int) as isize);
                tmp_real_1 -= Aij_real_1 * x_real_4 - Aij_imag_1 * x_imag_4;
                tmp_imag_1 -= Aij_real_1 * x_imag_4 + Aij_imag_1 * x_real_4;
                jx_1 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_4: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + i - i)) as isize,
                    );
                let a_imag_4: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                    * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                    / 2 as libc::c_int + i - i) + 1 as libc::c_int) as isize,
                        );
                let s_4: libc::c_double = xhypot(a_real_4, a_imag_4);
                let b_real_4: libc::c_double = a_real_4 / s_4;
                let b_imag_4: libc::c_double = a_imag_4 / s_4;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_1) as isize,
                    ) = (tmp_real_1 * b_real_4 + tmp_imag_1 * b_imag_4) / s_4;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize,
                    ) = (tmp_imag_1 * b_real_4 - tmp_real_1 * b_imag_4) / s_4;
            } else {
                *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_1) as isize) = tmp_real_1;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize,
                    ) = tmp_imag_1;
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
        }) + incX * (N - 1 as libc::c_int);
        if nonunit != 0 {
            let a_real_5: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * ((N - 1 as libc::c_int)
                            * (N - 1 as libc::c_int + 1 as libc::c_int)
                            / 2 as libc::c_int + (N - 1 as libc::c_int))) as isize,
                );
            let a_imag_5: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((N - 1 as libc::c_int)
                                * (N - 1 as libc::c_int + 1 as libc::c_int)
                                / 2 as libc::c_int + (N - 1 as libc::c_int))
                            + 1 as libc::c_int) as isize,
                    );
            let x_real_5: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_2) as isize);
            let x_imag_5: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize);
            let s_5: libc::c_double = xhypot(a_real_5, a_imag_5);
            let b_real_5: libc::c_double = a_real_5 / s_5;
            let b_imag_5: libc::c_double = a_imag_5 / s_5;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_2) as isize,
                ) = (x_real_5 * b_real_5 + x_imag_5 * b_imag_5) / s_5;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize,
                ) = (x_imag_5 * b_real_5 - b_imag_5 * x_real_5) / s_5;
        }
        ix_2 -= incX;
        i = N - 1 as libc::c_int;
        while i > 0 as libc::c_int
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let mut tmp_real_2: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_2) as isize);
            let mut tmp_imag_2: libc::c_double = *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize);
            let mut jx_2: libc::c_int = ix_2 + incX;
            j = i + 1 as libc::c_int;
            while j < N {
                let Aij_real_2: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (j * (j + 1 as libc::c_int) / 2 as libc::c_int + i))
                            as isize,
                    );
                let Aij_imag_2: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * (j * (j + 1 as libc::c_int) / 2 as libc::c_int + i)
                                + 1 as libc::c_int) as isize,
                        );
                let x_real_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_2) as isize);
                let x_imag_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_2 + 1 as libc::c_int) as isize);
                tmp_real_2 -= Aij_real_2 * x_real_6 - Aij_imag_2 * x_imag_6;
                tmp_imag_2 -= Aij_real_2 * x_imag_6 + Aij_imag_2 * x_real_6;
                jx_2 += incX;
                j += 1;
                j;
            }
            if nonunit != 0 {
                let a_real_6: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i))
                            as isize,
                    );
                let a_imag_6: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)
                                + 1 as libc::c_int) as isize,
                        );
                let s_6: libc::c_double = xhypot(a_real_6, a_imag_6);
                let b_real_6: libc::c_double = a_real_6 / s_6;
                let b_imag_6: libc::c_double = a_imag_6 / s_6;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_2) as isize,
                    ) = (tmp_real_2 * b_real_6 + tmp_imag_2 * b_imag_6) / s_6;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize,
                    ) = (tmp_imag_2 * b_real_6 - tmp_real_2 * b_imag_6) / s_6;
            } else {
                *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_2) as isize) = tmp_real_2;
                *(X as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize,
                    ) = tmp_imag_2;
            }
            ix_2 -= incX;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_tpsv_c.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
