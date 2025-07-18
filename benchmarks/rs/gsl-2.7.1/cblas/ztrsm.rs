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
pub type CBLAS_SIDE = libc::c_uint;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
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
pub unsafe extern "C" fn cblas_ztrsm(
    Order: CBLAS_ORDER,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: libc::c_int,
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: libc::c_int,
    mut B: *mut libc::c_void,
    ldb: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let nonunit: libc::c_int = (Diag as libc::c_uint
        == CblasNonUnit as libc::c_int as libc::c_uint) as libc::c_int;
    let conj: libc::c_int = if TransA as libc::c_uint
        == CblasConjTrans as libc::c_int as libc::c_uint
    {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    let mut side: libc::c_int = 0;
    let mut uplo: libc::c_int = 0;
    let mut trans: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut __dim: libc::c_int = 0 as libc::c_int;
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint {
        __dim = M;
    } else {
        __dim = N;
    }
    if Order as libc::c_uint != CblasRowMajor as libc::c_int as libc::c_uint
        && Order as libc::c_uint != CblasColMajor as libc::c_int as libc::c_uint
    {
        pos = 1 as libc::c_int;
    }
    if Side as libc::c_uint != CblasLeft as libc::c_int as libc::c_uint
        && Side as libc::c_uint != CblasRight as libc::c_int as libc::c_uint
    {
        pos = 2 as libc::c_int;
    }
    if Uplo as libc::c_uint != CblasUpper as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint != CblasLower as libc::c_int as libc::c_uint
    {
        pos = 3 as libc::c_int;
    }
    if TransA as libc::c_uint != CblasNoTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasConjTrans as libc::c_int as libc::c_uint
    {
        pos = 4 as libc::c_int;
    }
    if Diag as libc::c_uint != CblasNonUnit as libc::c_int as libc::c_uint
        && Diag as libc::c_uint != CblasUnit as libc::c_int as libc::c_uint
    {
        pos = 5 as libc::c_int;
    }
    if M < 0 as libc::c_int {
        pos = 6 as libc::c_int;
    }
    if N < 0 as libc::c_int {
        pos = 7 as libc::c_int;
    }
    if lda < (if 1 as libc::c_int > __dim { 1 as libc::c_int } else { __dim }) {
        pos = 10 as libc::c_int;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        if ldb < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
            pos = 12 as libc::c_int;
        }
    } else if ldb < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
        pos = 12 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_trsm_c.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as libc::c_int as isize);
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        n1 = M;
        n2 = N;
        side = Side as libc::c_int;
        uplo = Uplo as libc::c_int;
        trans = TransA as libc::c_int;
        trans = if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        {
            CblasNoTrans as libc::c_int
        } else {
            CblasTrans as libc::c_int
        };
    } else {
        n1 = N;
        n2 = M;
        side = if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint {
            CblasRight as libc::c_int
        } else {
            CblasLeft as libc::c_int
        };
        uplo = if Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint {
            CblasLower as libc::c_int
        } else {
            CblasUpper as libc::c_int
        };
        trans = if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        {
            CblasNoTrans as libc::c_int
        } else {
            CblasTrans as libc::c_int
        };
    }
    if side == CblasLeft as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real - alpha_imag * Bij_imag;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag + alpha_imag * Bij_real;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = n1;
        while i > 0 as libc::c_int
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            if nonunit != 0 {
                let Aii_real: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (lda * i + i)) as isize);
                let Aii_imag: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (lda * i + i) + 1 as libc::c_int)
                                as isize,
                        );
                let s: libc::c_double = xhypot(Aii_real, Aii_imag);
                let a_real: libc::c_double = Aii_real / s;
                let a_imag: libc::c_double = Aii_imag / s;
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_0: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_0: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_0 * a_real + Bij_imag_0 * a_imag) / s;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_0 * a_real - Bij_real_0 * a_imag) / s;
                    j += 1;
                    j;
                }
            }
            k = 0 as libc::c_int;
            while k < i {
                let Aki_real: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (k * lda + i)) as isize);
                let Aki_imag: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int)
                                as isize,
                        );
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_1: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_1: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * k + j)) as isize)
                        -= Aki_real * Bij_real_1 - Aki_imag * Bij_imag_1;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * k + j) + 1 as libc::c_int)
                                as isize,
                        ) -= Aki_real * Bij_imag_1 + Aki_imag * Bij_real_1;
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
        }
    } else if side == CblasLeft as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_2: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_2: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real_2 - alpha_imag * Bij_imag_2;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag_2 + alpha_imag * Bij_real_2;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as libc::c_int;
        while i < n1 {
            if nonunit != 0 {
                let Aii_real_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (lda * i + i)) as isize);
                let Aii_imag_0: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (lda * i + i) + 1 as libc::c_int)
                                as isize,
                        );
                let s_0: libc::c_double = xhypot(Aii_real_0, Aii_imag_0);
                let a_real_0: libc::c_double = Aii_real_0 / s_0;
                let a_imag_0: libc::c_double = Aii_imag_0 / s_0;
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_3: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_3: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_3 * a_real_0 + Bij_imag_3 * a_imag_0) / s_0;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_3 * a_real_0 - Bij_real_3 * a_imag_0) / s_0;
                    j += 1;
                    j;
                }
            }
            k = i + 1 as libc::c_int;
            while k < n1 {
                let Aik_real: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (i * lda + k)) as isize);
                let Aik_imag: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_4: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_4: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * k + j)) as isize)
                        -= Aik_real * Bij_real_4 - Aik_imag * Bij_imag_4;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * k + j) + 1 as libc::c_int)
                                as isize,
                        ) -= Aik_real * Bij_imag_4 + Aik_imag * Bij_real_4;
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
            i += 1;
            i;
        }
    } else if side == CblasLeft as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_5: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_5: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real_5 - alpha_imag * Bij_imag_5;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag_5 + alpha_imag * Bij_real_5;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as libc::c_int;
        while i < n1 {
            if nonunit != 0 {
                let Aii_real_1: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (lda * i + i)) as isize);
                let Aii_imag_1: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (lda * i + i) + 1 as libc::c_int)
                                as isize,
                        );
                let s_1: libc::c_double = xhypot(Aii_real_1, Aii_imag_1);
                let a_real_1: libc::c_double = Aii_real_1 / s_1;
                let a_imag_1: libc::c_double = Aii_imag_1 / s_1;
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_6: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_6: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_6 * a_real_1 + Bij_imag_6 * a_imag_1) / s_1;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_6 * a_real_1 - Bij_real_6 * a_imag_1) / s_1;
                    j += 1;
                    j;
                }
            }
            k = i + 1 as libc::c_int;
            while k < n1 {
                let Aki_real_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (k * lda + i)) as isize);
                let Aki_imag_0: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int)
                                as isize,
                        );
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_7: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_7: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * k + j)) as isize)
                        -= Aki_real_0 * Bij_real_7 - Aki_imag_0 * Bij_imag_7;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * k + j) + 1 as libc::c_int)
                                as isize,
                        ) -= Aki_real_0 * Bij_imag_7 + Aki_imag_0 * Bij_real_7;
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
            i += 1;
            i;
        }
    } else if side == CblasLeft as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_8: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_8: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real_8 - alpha_imag * Bij_imag_8;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag_8 + alpha_imag * Bij_real_8;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = n1;
        while i > 0 as libc::c_int
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            if nonunit != 0 {
                let Aii_real_2: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (lda * i + i)) as isize);
                let Aii_imag_2: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (lda * i + i) + 1 as libc::c_int)
                                as isize,
                        );
                let s_2: libc::c_double = xhypot(Aii_real_2, Aii_imag_2);
                let a_real_2: libc::c_double = Aii_real_2 / s_2;
                let a_imag_2: libc::c_double = Aii_imag_2 / s_2;
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_9: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_9: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_9 * a_real_2 + Bij_imag_9 * a_imag_2) / s_2;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_9 * a_real_2 - Bij_real_9 * a_imag_2) / s_2;
                    j += 1;
                    j;
                }
            }
            k = 0 as libc::c_int;
            while k < i {
                let Aik_real_0: libc::c_double = *(A as *const libc::c_double)
                    .offset((2 as libc::c_int * (i * lda + k)) as isize);
                let Aik_imag_0: libc::c_double = conj as libc::c_double
                    * *(A as *const libc::c_double)
                        .offset(
                            (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                as isize,
                        );
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_10: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_10: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * k + j)) as isize)
                        -= Aik_real_0 * Bij_real_10 - Aik_imag_0 * Bij_imag_10;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * k + j) + 1 as libc::c_int)
                                as isize,
                        ) -= Aik_real_0 * Bij_imag_10 + Aik_imag_0 * Bij_real_10;
                    j += 1;
                    j;
                }
                k += 1;
                k;
            }
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_11: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_11: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real_11 - alpha_imag * Bij_imag_11;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag_11 + alpha_imag * Bij_real_11;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                if nonunit != 0 {
                    let Ajj_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (lda * j + j)) as isize);
                    let Ajj_imag: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (lda * j + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let s_3: libc::c_double = xhypot(Ajj_real, Ajj_imag);
                    let a_real_3: libc::c_double = Ajj_real / s_3;
                    let a_imag_3: libc::c_double = Ajj_imag / s_3;
                    let Bij_real_12: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_12: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_12 * a_real_3 + Bij_imag_12 * a_imag_3) / s_3;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_12 * a_real_3 - Bij_real_12 * a_imag_3) / s_3;
                }
                let Bij_real_13: libc::c_double = *(B as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag_13: libc::c_double = *(B as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                k = j + 1 as libc::c_int;
                while k < n2 {
                    let Ajk_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + k)) as isize)
                        -= Ajk_real * Bij_real_13 - Ajk_imag * Bij_imag_13;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + k) + 1 as libc::c_int)
                                as isize,
                        ) -= Ajk_real * Bij_imag_13 + Ajk_imag * Bij_real_13;
                    k += 1;
                    k;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_14: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_14: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real_14 - alpha_imag * Bij_imag_14;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag_14 + alpha_imag * Bij_real_14;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as libc::c_int;
        while i < n1 {
            j = n2;
            while j > 0 as libc::c_int
                && {
                    let fresh2 = j;
                    j = j - 1;
                    fresh2 != 0
                }
            {
                if nonunit != 0 {
                    let Ajj_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (lda * j + j)) as isize);
                    let Ajj_imag_0: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (lda * j + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let s_4: libc::c_double = xhypot(Ajj_real_0, Ajj_imag_0);
                    let a_real_4: libc::c_double = Ajj_real_0 / s_4;
                    let a_imag_4: libc::c_double = Ajj_imag_0 / s_4;
                    let Bij_real_15: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_15: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_15 * a_real_4 + Bij_imag_15 * a_imag_4) / s_4;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_15 * a_real_4 - Bij_real_15 * a_imag_4) / s_4;
                }
                let Bij_real_16: libc::c_double = *(B as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag_16: libc::c_double = *(B as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                k = 0 as libc::c_int;
                while k < j {
                    let Akj_real: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let Akj_imag: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + k)) as isize)
                        -= Akj_real * Bij_real_16 - Akj_imag * Bij_imag_16;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + k) + 1 as libc::c_int)
                                as isize,
                        ) -= Akj_real * Bij_imag_16 + Akj_imag * Bij_real_16;
                    k += 1;
                    k;
                }
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_17: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_17: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real_17 - alpha_imag * Bij_imag_17;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag_17 + alpha_imag * Bij_real_17;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as libc::c_int;
        while i < n1 {
            j = n2;
            while j > 0 as libc::c_int
                && {
                    let fresh3 = j;
                    j = j - 1;
                    fresh3 != 0
                }
            {
                if nonunit != 0 {
                    let Ajj_real_1: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (lda * j + j)) as isize);
                    let Ajj_imag_1: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (lda * j + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let s_5: libc::c_double = xhypot(Ajj_real_1, Ajj_imag_1);
                    let a_real_5: libc::c_double = Ajj_real_1 / s_5;
                    let a_imag_5: libc::c_double = Ajj_imag_1 / s_5;
                    let Bij_real_18: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_18: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_18 * a_real_5 + Bij_imag_18 * a_imag_5) / s_5;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_18 * a_real_5 - Bij_real_18 * a_imag_5) / s_5;
                }
                let Bij_real_19: libc::c_double = *(B as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag_19: libc::c_double = *(B as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                k = 0 as libc::c_int;
                while k < j {
                    let Ajk_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + k)) as isize)
                        -= Ajk_real_0 * Bij_real_19 - Ajk_imag_0 * Bij_imag_19;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + k) + 1 as libc::c_int)
                                as isize,
                        ) -= Ajk_real_0 * Bij_imag_19 + Ajk_imag_0 * Bij_real_19;
                    k += 1;
                    k;
                }
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
        if !(alpha_real == 1.0f64 && alpha_imag == 0.0f64) {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    let Bij_real_20: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_20: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = alpha_real * Bij_real_20 - alpha_imag * Bij_imag_20;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = alpha_real * Bij_imag_20 + alpha_imag * Bij_real_20;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                if nonunit != 0 {
                    let Ajj_real_2: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (lda * j + j)) as isize);
                    let Ajj_imag_2: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (lda * j + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let s_6: libc::c_double = xhypot(Ajj_real_2, Ajj_imag_2);
                    let a_real_6: libc::c_double = Ajj_real_2 / s_6;
                    let a_imag_6: libc::c_double = Ajj_imag_2 / s_6;
                    let Bij_real_21: libc::c_double = *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                    let Bij_imag_21: libc::c_double = *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        );
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j)) as isize,
                        ) = (Bij_real_21 * a_real_6 + Bij_imag_21 * a_imag_6) / s_6;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int)
                                as isize,
                        ) = (Bij_imag_21 * a_real_6 - Bij_real_21 * a_imag_6) / s_6;
                }
                let Bij_real_22: libc::c_double = *(B as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldb * i + j)) as isize);
                let Bij_imag_22: libc::c_double = *(B as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    );
                k = j + 1 as libc::c_int;
                while k < n2 {
                    let Akj_real_0: libc::c_double = *(A as *const libc::c_double)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let Akj_imag_0: libc::c_double = conj as libc::c_double
                        * *(A as *const libc::c_double)
                            .offset(
                                (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    *(B as *mut libc::c_double)
                        .offset((2 as libc::c_int * (ldb * i + k)) as isize)
                        -= Akj_real_0 * Bij_real_22 - Akj_imag_0 * Bij_imag_22;
                    *(B as *mut libc::c_double)
                        .offset(
                            (2 as libc::c_int * (ldb * i + k) + 1 as libc::c_int)
                                as isize,
                        ) -= Akj_real_0 * Bij_imag_22 + Akj_imag_0 * Bij_real_22;
                    k += 1;
                    k;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_trsm_c.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
