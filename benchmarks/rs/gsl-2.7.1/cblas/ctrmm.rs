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
pub type CBLAS_SIDE = libc::c_uint;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
pub unsafe extern "C" fn cblas_ctrmm(
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
            b"./source_trmm_c.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as libc::c_int as isize);
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        n1 = M;
        n2 = N;
        side = Side as libc::c_int;
        uplo = Uplo as libc::c_int;
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
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag: libc::c_float = 0.0f64 as libc::c_float;
                if nonunit != 0 {
                    let Aii_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + i)) as isize);
                    let Aii_imag: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real = Aii_real * Bij_real - Aii_imag * Bij_imag;
                    temp_imag = Aii_real * Bij_imag + Aii_imag * Bij_real;
                } else {
                    temp_real = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                }
                k = i + 1 as libc::c_int;
                while k < n1 {
                    let Aik_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bkj_real: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (k * ldb + j)) as isize);
                    let Bkj_imag: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real += Aik_real * Bkj_real - Aik_imag * Bkj_imag;
                    temp_imag += Aik_real * Bkj_imag + Aik_imag * Bkj_real;
                    k += 1;
                    k;
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real - alpha_imag * temp_imag;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag + alpha_imag * temp_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasLeft as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
        i = n1;
        while i > 0 as libc::c_int
            && {
                let fresh0 = i;
                i = i - 1;
                fresh0 != 0
            }
        {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real_0: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_0: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < i {
                    let Aki_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * lda + i)) as isize);
                    let Aki_imag: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bkj_real_0: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (k * ldb + j)) as isize);
                    let Bkj_imag_0: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_0 += Aki_real * Bkj_real_0 - Aki_imag * Bkj_imag_0;
                    temp_imag_0 += Aki_real * Bkj_imag_0 + Aki_imag * Bkj_real_0;
                    k += 1;
                    k;
                }
                if nonunit != 0 {
                    let Aii_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + i)) as isize);
                    let Aii_imag_0: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real_0: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag_0: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_0 += Aii_real_0 * Bij_real_0 - Aii_imag_0 * Bij_imag_0;
                    temp_imag_0 += Aii_real_0 * Bij_imag_0 + Aii_imag_0 * Bij_real_0;
                } else {
                    temp_real_0
                        += *(B as *mut libc::c_float)
                            .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag_0
                        += *(B as *mut libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                    as isize,
                            );
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real_0 - alpha_imag * temp_imag_0;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag_0 + alpha_imag * temp_real_0;
                j += 1;
                j;
            }
        }
    } else if side == CblasLeft as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
        i = n1;
        while i > 0 as libc::c_int
            && {
                let fresh1 = i;
                i = i - 1;
                fresh1 != 0
            }
        {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real_1: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_1: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < i {
                    let Aik_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + k)) as isize);
                    let Aik_imag_0: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * lda + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bkj_real_1: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (k * ldb + j)) as isize);
                    let Bkj_imag_1: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_1 += Aik_real_0 * Bkj_real_1 - Aik_imag_0 * Bkj_imag_1;
                    temp_imag_1 += Aik_real_0 * Bkj_imag_1 + Aik_imag_0 * Bkj_real_1;
                    k += 1;
                    k;
                }
                if nonunit != 0 {
                    let Aii_real_1: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + i)) as isize);
                    let Aii_imag_1: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real_1: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag_1: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_1 += Aii_real_1 * Bij_real_1 - Aii_imag_1 * Bij_imag_1;
                    temp_imag_1 += Aii_real_1 * Bij_imag_1 + Aii_imag_1 * Bij_real_1;
                } else {
                    temp_real_1
                        += *(B as *mut libc::c_float)
                            .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag_1
                        += *(B as *mut libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                    as isize,
                            );
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real_1 - alpha_imag * temp_imag_1;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag_1 + alpha_imag * temp_real_1;
                j += 1;
                j;
            }
        }
    } else if side == CblasLeft as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real_2: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_2: libc::c_float = 0.0f64 as libc::c_float;
                if nonunit != 0 {
                    let Aii_real_2: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (i * lda + i)) as isize);
                    let Aii_imag_2: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * lda + i) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real_2: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag_2: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_2 = Aii_real_2 * Bij_real_2 - Aii_imag_2 * Bij_imag_2;
                    temp_imag_2 = Aii_real_2 * Bij_imag_2 + Aii_imag_2 * Bij_real_2;
                } else {
                    temp_real_2 = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag_2 = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                }
                k = i + 1 as libc::c_int;
                while k < n1 {
                    let Aki_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * lda + i)) as isize);
                    let Aki_imag_0: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (k * lda + i) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bkj_real_2: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (k * ldb + j)) as isize);
                    let Bkj_imag_2: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (k * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_2 += Aki_real_0 * Bkj_real_2 - Aki_imag_0 * Bkj_imag_2;
                    temp_imag_2 += Aki_real_0 * Bkj_imag_2 + Aki_imag_0 * Bkj_real_2;
                    k += 1;
                    k;
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real_2 - alpha_imag * temp_imag_2;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag_2 + alpha_imag * temp_real_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
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
                let mut temp_real_3: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_3: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < j {
                    let Akj_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let Akj_imag: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bik_real: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                    let Bik_imag: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_3 += Akj_real * Bik_real - Akj_imag * Bik_imag;
                    temp_imag_3 += Akj_real * Bik_imag + Akj_imag * Bik_real;
                    k += 1;
                    k;
                }
                if nonunit != 0 {
                    let Ajj_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + j)) as isize);
                    let Ajj_imag: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (j * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real_3: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag_3: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_3 += Ajj_real * Bij_real_3 - Ajj_imag * Bij_imag_3;
                    temp_imag_3 += Ajj_real * Bij_imag_3 + Ajj_imag * Bij_real_3;
                } else {
                    temp_real_3
                        += *(B as *mut libc::c_float)
                            .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag_3
                        += *(B as *mut libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                    as isize,
                            );
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real_3 - alpha_imag * temp_imag_3;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag_3 + alpha_imag * temp_real_3;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real_4: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_4: libc::c_float = 0.0f64 as libc::c_float;
                if nonunit != 0 {
                    let Ajj_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + j)) as isize);
                    let Ajj_imag_0: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (j * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real_4: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag_4: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_4 = Ajj_real_0 * Bij_real_4 - Ajj_imag_0 * Bij_imag_4;
                    temp_imag_4 = Ajj_real_0 * Bij_imag_4 + Ajj_imag_0 * Bij_real_4;
                } else {
                    temp_real_4 = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag_4 = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                }
                k = j + 1 as libc::c_int;
                while k < n2 {
                    let Ajk_real: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bik_real_0: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                    let Bik_imag_0: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_4 += Ajk_real * Bik_real_0 - Ajk_imag * Bik_imag_0;
                    temp_imag_4 += Ajk_real * Bik_imag_0 + Ajk_imag * Bik_real_0;
                    k += 1;
                    k;
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real_4 - alpha_imag * temp_imag_4;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag_4 + alpha_imag * temp_real_4;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real_5: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_5: libc::c_float = 0.0f64 as libc::c_float;
                if nonunit != 0 {
                    let Ajj_real_1: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + j)) as isize);
                    let Ajj_imag_1: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (j * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real_5: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag_5: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_5 = Ajj_real_1 * Bij_real_5 - Ajj_imag_1 * Bij_imag_5;
                    temp_imag_5 = Ajj_real_1 * Bij_imag_5 + Ajj_imag_1 * Bij_real_5;
                } else {
                    temp_real_5 = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag_5 = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                }
                k = j + 1 as libc::c_int;
                while k < n2 {
                    let Akj_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (k * lda + j)) as isize);
                    let Akj_imag_0: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (k * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bik_real_1: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                    let Bik_imag_1: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_5 += Akj_real_0 * Bik_real_1 - Akj_imag_0 * Bik_imag_1;
                    temp_imag_5 += Akj_real_0 * Bik_imag_1 + Akj_imag_0 * Bik_real_1;
                    k += 1;
                    k;
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real_5 - alpha_imag * temp_imag_5;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag_5 + alpha_imag * temp_real_5;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if side == CblasRight as libc::c_int && uplo == CblasLower as libc::c_int
        && trans == CblasTrans as libc::c_int
    {
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
                let mut temp_real_6: libc::c_float = 0.0f64 as libc::c_float;
                let mut temp_imag_6: libc::c_float = 0.0f64 as libc::c_float;
                k = 0 as libc::c_int;
                while k < j {
                    let Ajk_real_0: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + k)) as isize);
                    let Ajk_imag_0: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (j * lda + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bik_real_2: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + k)) as isize);
                    let Bik_imag_2: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + k) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_6 += Ajk_real_0 * Bik_real_2 - Ajk_imag_0 * Bik_imag_2;
                    temp_imag_6 += Ajk_real_0 * Bik_imag_2 + Ajk_imag_0 * Bik_real_2;
                    k += 1;
                    k;
                }
                if nonunit != 0 {
                    let Ajj_real_2: libc::c_float = *(A as *const libc::c_float)
                        .offset((2 as libc::c_int * (j * lda + j)) as isize);
                    let Ajj_imag_2: libc::c_float = conj as libc::c_float
                        * *(A as *const libc::c_float)
                            .offset(
                                (2 as libc::c_int * (j * lda + j) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Bij_real_6: libc::c_float = *(B as *mut libc::c_float)
                        .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    let Bij_imag_6: libc::c_float = *(B as *mut libc::c_float)
                        .offset(
                            (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                as isize,
                        );
                    temp_real_6 += Ajj_real_2 * Bij_real_6 - Ajj_imag_2 * Bij_imag_6;
                    temp_imag_6 += Ajj_real_2 * Bij_imag_6 + Ajj_imag_2 * Bij_real_6;
                } else {
                    temp_real_6
                        += *(B as *mut libc::c_float)
                            .offset((2 as libc::c_int * (i * ldb + j)) as isize);
                    temp_imag_6
                        += *(B as *mut libc::c_float)
                            .offset(
                                (2 as libc::c_int * (i * ldb + j) + 1 as libc::c_int)
                                    as isize,
                            );
                }
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j)) as isize,
                    ) = alpha_real * temp_real_6 - alpha_imag * temp_imag_6;
                *(B as *mut libc::c_float)
                    .offset(
                        (2 as libc::c_int * (ldb * i + j) + 1 as libc::c_int) as isize,
                    ) = alpha_real * temp_imag_6 + alpha_imag * temp_real_6;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_trmm_c.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
