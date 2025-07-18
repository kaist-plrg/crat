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
pub unsafe extern "C" fn cblas_ztpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: libc::c_int,
    mut Ap: *const libc::c_void,
    mut X: *mut libc::c_void,
    incX: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
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
            b"./source_tpmv_c.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
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
            let Aii_real: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + i - i)) as isize,
                );
            let Aii_imag: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + i - i) + 1 as libc::c_int) as isize,
                    );
            let mut temp_r: libc::c_double = 0.;
            let mut temp_i: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix) as isize);
                let mut x_imag: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
                temp_r = Aii_real * x_real - Aii_imag * x_imag;
                temp_i = Aii_real * x_imag + Aii_imag * x_real;
            } else {
                temp_r = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix) as isize);
                temp_i = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
            }
            let mut jx: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + (i + 1 as libc::c_int) * incX;
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
                let mut x_real_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx) as isize);
                let mut x_imag_0: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx + 1 as libc::c_int) as isize);
                temp_r += Aij_real * x_real_0 - Aij_imag * x_imag_0;
                temp_i += Aij_real * x_imag_0 + Aij_imag * x_real_0;
                jx += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix) as isize) = temp_r;
            *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize) = temp_i;
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
        }) + incX * (N - 1 as libc::c_int);
        i = N;
        while i > 0 as libc::c_int
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            let Aii_real_0: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)) as isize,
                );
            let Aii_imag_0: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)
                            + 1 as libc::c_int) as isize,
                    );
            let mut temp_r_0: libc::c_double = 0.;
            let mut temp_i_0: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real_1: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_0) as isize);
                let mut x_imag_1: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
                temp_r_0 = Aii_real_0 * x_real_1 - Aii_imag_0 * x_imag_1;
                temp_i_0 = Aii_real_0 * x_imag_1 + Aii_imag_0 * x_real_1;
            } else {
                temp_r_0 = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_0) as isize);
                temp_i_0 = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
            }
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
                let mut x_real_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_0) as isize);
                let mut x_imag_2: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_0 + 1 as libc::c_int) as isize);
                temp_r_0 += Aij_real_0 * x_real_2 - Aij_imag_0 * x_imag_2;
                temp_i_0 += Aij_real_0 * x_imag_2 + Aij_imag_0 * x_real_2;
                jx_0 += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_0) as isize) = temp_r_0;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize,
                ) = temp_i_0;
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
        }) + incX * (N - 1 as libc::c_int);
        i = N;
        while i > 0 as libc::c_int
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            let Aii_real_1: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + i - i)) as isize,
                );
            let Aii_imag_1: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((i - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                                / 2 as libc::c_int + i - i) + 1 as libc::c_int) as isize,
                    );
            let mut temp_r_1: libc::c_double = 0.;
            let mut temp_i_1: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real_3: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_1) as isize);
                let mut x_imag_3: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize);
                temp_r_1 = Aii_real_1 * x_real_3 - Aii_imag_1 * x_imag_3;
                temp_i_1 = Aii_real_1 * x_imag_3 + Aii_imag_1 * x_real_3;
            } else {
                temp_r_1 = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_1) as isize);
                temp_i_1 = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize);
            }
            let mut jx_1: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            j = 0 as libc::c_int;
            while j < i {
                let mut x_real_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_1) as isize);
                let mut x_imag_4: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_1 + 1 as libc::c_int) as isize);
                let Aji_real: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * ((j - 1 as libc::c_int + 1 as libc::c_int)
                                * (2 as libc::c_int * N - (j - 1 as libc::c_int))
                                / 2 as libc::c_int + i - j)) as isize,
                    );
                let Aji_imag: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * ((j - 1 as libc::c_int + 1 as libc::c_int)
                                    * (2 as libc::c_int * N - (j - 1 as libc::c_int))
                                    / 2 as libc::c_int + i - j) + 1 as libc::c_int) as isize,
                        );
                temp_r_1 += Aji_real * x_real_4 - Aji_imag * x_imag_4;
                temp_i_1 += Aji_real * x_imag_4 + Aji_imag * x_real_4;
                jx_1 += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_1) as isize) = temp_r_1;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_1 + 1 as libc::c_int) as isize,
                ) = temp_i_1;
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
            let Aii_real_2: libc::c_double = *(Ap as *const libc::c_double)
                .offset(
                    (2 as libc::c_int
                        * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)) as isize,
                );
            let Aii_imag_2: libc::c_double = conj as libc::c_double
                * *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (i * (i + 1 as libc::c_int) / 2 as libc::c_int + i)
                            + 1 as libc::c_int) as isize,
                    );
            let mut temp_r_2: libc::c_double = 0.;
            let mut temp_i_2: libc::c_double = 0.;
            if nonunit != 0 {
                let mut x_real_5: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_2) as isize);
                let mut x_imag_5: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize);
                temp_r_2 = Aii_real_2 * x_real_5 - Aii_imag_2 * x_imag_5;
                temp_i_2 = Aii_real_2 * x_imag_5 + Aii_imag_2 * x_real_5;
            } else {
                temp_r_2 = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_2) as isize);
                temp_i_2 = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize);
            }
            let mut jx_2: libc::c_int = (if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            }) + (i + 1 as libc::c_int) * incX;
            j = i + 1 as libc::c_int;
            while j < N {
                let mut x_real_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_2) as isize);
                let mut x_imag_6: libc::c_double = *(X as *mut libc::c_double)
                    .offset((2 as libc::c_int * jx_2 + 1 as libc::c_int) as isize);
                let Aji_real_0: libc::c_double = *(Ap as *const libc::c_double)
                    .offset(
                        (2 as libc::c_int
                            * (j * (j + 1 as libc::c_int) / 2 as libc::c_int + i))
                            as isize,
                    );
                let Aji_imag_0: libc::c_double = conj as libc::c_double
                    * *(Ap as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int
                                * (j * (j + 1 as libc::c_int) / 2 as libc::c_int + i)
                                + 1 as libc::c_int) as isize,
                        );
                temp_r_2 += Aji_real_0 * x_real_6 - Aji_imag_0 * x_imag_6;
                temp_i_2 += Aji_real_0 * x_imag_6 + Aji_imag_0 * x_real_6;
                jx_2 += incX;
                j += 1;
                j;
            }
            *(X as *mut libc::c_double)
                .offset((2 as libc::c_int * ix_2) as isize) = temp_r_2;
            *(X as *mut libc::c_double)
                .offset(
                    (2 as libc::c_int * ix_2 + 1 as libc::c_int) as isize,
                ) = temp_i_2;
            ix_2 += incX;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_tpmv_c.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
