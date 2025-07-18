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
pub unsafe extern "C" fn cblas_zgemm(
    Order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: libc::c_int,
    N: libc::c_int,
    K: libc::c_int,
    mut alpha: *const libc::c_void,
    mut A: *const libc::c_void,
    lda: libc::c_int,
    mut B: *const libc::c_void,
    ldb: libc::c_int,
    mut beta: *const libc::c_void,
    mut C: *mut libc::c_void,
    ldc: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut ldf: libc::c_int = 0;
    let mut ldg: libc::c_int = 0;
    let mut conjF: libc::c_int = 0;
    let mut conjG: libc::c_int = 0;
    let mut TransF: libc::c_int = 0;
    let mut TransG: libc::c_int = 0;
    let mut F: *const libc::c_double = 0 as *const libc::c_double;
    let mut G: *const libc::c_double = 0 as *const libc::c_double;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut __transF: CBLAS_TRANSPOSE = CblasNoTrans;
    let mut __transG: CBLAS_TRANSPOSE = CblasNoTrans;
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        __transF = (if TransA as libc::c_uint
            != CblasConjTrans as libc::c_int as libc::c_uint
        {
            TransA as libc::c_uint
        } else {
            CblasTrans as libc::c_int as libc::c_uint
        }) as CBLAS_TRANSPOSE;
        __transG = (if TransB as libc::c_uint
            != CblasConjTrans as libc::c_int as libc::c_uint
        {
            TransB as libc::c_uint
        } else {
            CblasTrans as libc::c_int as libc::c_uint
        }) as CBLAS_TRANSPOSE;
    } else {
        __transF = (if TransB as libc::c_uint
            != CblasConjTrans as libc::c_int as libc::c_uint
        {
            TransB as libc::c_uint
        } else {
            CblasTrans as libc::c_int as libc::c_uint
        }) as CBLAS_TRANSPOSE;
        __transG = (if TransA as libc::c_uint
            != CblasConjTrans as libc::c_int as libc::c_uint
        {
            TransA as libc::c_uint
        } else {
            CblasTrans as libc::c_int as libc::c_uint
        }) as CBLAS_TRANSPOSE;
    }
    if Order as libc::c_uint != CblasRowMajor as libc::c_int as libc::c_uint
        && Order as libc::c_uint != CblasColMajor as libc::c_int as libc::c_uint
    {
        pos = 1 as libc::c_int;
    }
    if TransA as libc::c_uint != CblasNoTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasTrans as libc::c_int as libc::c_uint
        && TransA as libc::c_uint != CblasConjTrans as libc::c_int as libc::c_uint
    {
        pos = 2 as libc::c_int;
    }
    if TransB as libc::c_uint != CblasNoTrans as libc::c_int as libc::c_uint
        && TransB as libc::c_uint != CblasTrans as libc::c_int as libc::c_uint
        && TransB as libc::c_uint != CblasConjTrans as libc::c_int as libc::c_uint
    {
        pos = 3 as libc::c_int;
    }
    if M < 0 as libc::c_int {
        pos = 4 as libc::c_int;
    }
    if N < 0 as libc::c_int {
        pos = 5 as libc::c_int;
    }
    if K < 0 as libc::c_int {
        pos = 6 as libc::c_int;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        if __transF as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            if lda < (if 1 as libc::c_int > K { 1 as libc::c_int } else { K }) {
                pos = 9 as libc::c_int;
            }
        } else if lda < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
            pos = 9 as libc::c_int;
        }
        if __transG as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            if ldb < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
                pos = 11 as libc::c_int;
            }
        } else if ldb < (if 1 as libc::c_int > K { 1 as libc::c_int } else { K }) {
            pos = 11 as libc::c_int;
        }
        if ldc < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
            pos = 14 as libc::c_int;
        }
    } else if Order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint {
        if __transF as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            if ldb < (if 1 as libc::c_int > K { 1 as libc::c_int } else { K }) {
                pos = 11 as libc::c_int;
            }
        } else if ldb < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
            pos = 11 as libc::c_int;
        }
        if __transG as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            if lda < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
                pos = 9 as libc::c_int;
            }
        } else if lda < (if 1 as libc::c_int > K { 1 as libc::c_int } else { K }) {
            pos = 9 as libc::c_int;
        }
        if ldc < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
            pos = 14 as libc::c_int;
        }
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_gemm_c.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as libc::c_int as isize);
    let beta_real: libc::c_double = *(beta as *const libc::c_double)
        .offset(0 as libc::c_int as isize);
    let beta_imag: libc::c_double = *(beta as *const libc::c_double)
        .offset(1 as libc::c_int as isize);
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64
        && (beta_real == 1.0f64 && beta_imag == 0.0f64)
    {
        return;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        n1 = M;
        n2 = N;
        F = A as *const libc::c_double;
        ldf = lda;
        conjF = if TransA as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        TransF = if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        {
            CblasNoTrans as libc::c_int
        } else {
            CblasTrans as libc::c_int
        };
        G = B as *const libc::c_double;
        ldg = ldb;
        conjG = if TransB as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        TransG = if TransB as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        {
            CblasNoTrans as libc::c_int
        } else {
            CblasTrans as libc::c_int
        };
    } else {
        n1 = N;
        n2 = M;
        F = B as *const libc::c_double;
        ldf = ldb;
        conjF = if TransB as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        TransF = if TransB as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        {
            CblasNoTrans as libc::c_int
        } else {
            CblasTrans as libc::c_int
        };
        G = A as *const libc::c_double;
        ldg = lda;
        conjG = if TransA as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        TransG = if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        {
            CblasNoTrans as libc::c_int
        } else {
            CblasTrans as libc::c_int
        };
    }
    if beta_real == 0.0f64 && beta_imag == 0.0f64 {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldc * i + j)) as isize) = 0.0f64;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    ) = 0.0f64;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if !(beta_real == 1.0f64 && beta_imag == 0.0f64) {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let Cij_real: libc::c_double = *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldc * i + j)) as isize);
                let Cij_imag: libc::c_double = *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    );
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j)) as isize,
                    ) = beta_real * Cij_real - beta_imag * Cij_imag;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    ) = beta_real * Cij_imag + beta_imag * Cij_real;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    if alpha_real == 0.0f64 && alpha_imag == 0.0f64 {
        return;
    }
    if TransF == CblasNoTrans as libc::c_int && TransG == CblasNoTrans as libc::c_int {
        k = 0 as libc::c_int;
        while k < K {
            i = 0 as libc::c_int;
            while i < n1 {
                let Fik_real: libc::c_double = *F
                    .offset((2 as libc::c_int * (ldf * i + k)) as isize);
                let Fik_imag: libc::c_double = conjF as libc::c_double
                    * *F
                        .offset(
                            (2 as libc::c_int * (ldf * i + k) + 1 as libc::c_int)
                                as isize,
                        );
                let temp_real: libc::c_double = alpha_real * Fik_real
                    - alpha_imag * Fik_imag;
                let temp_imag: libc::c_double = alpha_real * Fik_imag
                    + alpha_imag * Fik_real;
                if !(temp_real == 0.0f64 && temp_imag == 0.0f64) {
                    j = 0 as libc::c_int;
                    while j < n2 {
                        let Gkj_real: libc::c_double = *G
                            .offset((2 as libc::c_int * (ldg * k + j)) as isize);
                        let Gkj_imag: libc::c_double = conjG as libc::c_double
                            * *G
                                .offset(
                                    (2 as libc::c_int * (ldg * k + j) + 1 as libc::c_int)
                                        as isize,
                                );
                        *(C as *mut libc::c_double)
                            .offset((2 as libc::c_int * (ldc * i + j)) as isize)
                            += temp_real * Gkj_real - temp_imag * Gkj_imag;
                        *(C as *mut libc::c_double)
                            .offset(
                                (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                    as isize,
                            ) += temp_real * Gkj_imag + temp_imag * Gkj_real;
                        j += 1;
                        j;
                    }
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else if TransF == CblasNoTrans as libc::c_int
        && TransG == CblasTrans as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real_0: libc::c_double = 0.0f64;
                let mut temp_imag_0: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    let Fik_real_0: libc::c_double = *F
                        .offset((2 as libc::c_int * (ldf * i + k)) as isize);
                    let Fik_imag_0: libc::c_double = conjF as libc::c_double
                        * *F
                            .offset(
                                (2 as libc::c_int * (ldf * i + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Gjk_real: libc::c_double = *G
                        .offset((2 as libc::c_int * (ldg * j + k)) as isize);
                    let Gjk_imag: libc::c_double = conjG as libc::c_double
                        * *G
                            .offset(
                                (2 as libc::c_int * (ldg * j + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    temp_real_0 += Fik_real_0 * Gjk_real - Fik_imag_0 * Gjk_imag;
                    temp_imag_0 += Fik_real_0 * Gjk_imag + Fik_imag_0 * Gjk_real;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldc * i + j)) as isize)
                    += alpha_real * temp_real_0 - alpha_imag * temp_imag_0;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    ) += alpha_real * temp_imag_0 + alpha_imag * temp_real_0;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if TransF == CblasTrans as libc::c_int
        && TransG == CblasNoTrans as libc::c_int
    {
        k = 0 as libc::c_int;
        while k < K {
            i = 0 as libc::c_int;
            while i < n1 {
                let Fki_real: libc::c_double = *F
                    .offset((2 as libc::c_int * (ldf * k + i)) as isize);
                let Fki_imag: libc::c_double = conjF as libc::c_double
                    * *F
                        .offset(
                            (2 as libc::c_int * (ldf * k + i) + 1 as libc::c_int)
                                as isize,
                        );
                let temp_real_1: libc::c_double = alpha_real * Fki_real
                    - alpha_imag * Fki_imag;
                let temp_imag_1: libc::c_double = alpha_real * Fki_imag
                    + alpha_imag * Fki_real;
                if !(temp_real_1 == 0.0f64 && temp_imag_1 == 0.0f64) {
                    j = 0 as libc::c_int;
                    while j < n2 {
                        let Gkj_real_0: libc::c_double = *G
                            .offset((2 as libc::c_int * (ldg * k + j)) as isize);
                        let Gkj_imag_0: libc::c_double = conjG as libc::c_double
                            * *G
                                .offset(
                                    (2 as libc::c_int * (ldg * k + j) + 1 as libc::c_int)
                                        as isize,
                                );
                        *(C as *mut libc::c_double)
                            .offset((2 as libc::c_int * (ldc * i + j)) as isize)
                            += temp_real_1 * Gkj_real_0 - temp_imag_1 * Gkj_imag_0;
                        *(C as *mut libc::c_double)
                            .offset(
                                (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int)
                                    as isize,
                            ) += temp_real_1 * Gkj_imag_0 + temp_imag_1 * Gkj_real_0;
                        j += 1;
                        j;
                    }
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
    } else if TransF == CblasTrans as libc::c_int && TransG == CblasTrans as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                let mut temp_real_2: libc::c_double = 0.0f64;
                let mut temp_imag_2: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    let Fki_real_0: libc::c_double = *F
                        .offset((2 as libc::c_int * (ldf * k + i)) as isize);
                    let Fki_imag_0: libc::c_double = conjF as libc::c_double
                        * *F
                            .offset(
                                (2 as libc::c_int * (ldf * k + i) + 1 as libc::c_int)
                                    as isize,
                            );
                    let Gjk_real_0: libc::c_double = *G
                        .offset((2 as libc::c_int * (ldg * j + k)) as isize);
                    let Gjk_imag_0: libc::c_double = conjG as libc::c_double
                        * *G
                            .offset(
                                (2 as libc::c_int * (ldg * j + k) + 1 as libc::c_int)
                                    as isize,
                            );
                    temp_real_2 += Fki_real_0 * Gjk_real_0 - Fki_imag_0 * Gjk_imag_0;
                    temp_imag_2 += Fki_real_0 * Gjk_imag_0 + Fki_imag_0 * Gjk_real_0;
                    k += 1;
                    k;
                }
                *(C as *mut libc::c_double)
                    .offset((2 as libc::c_int * (ldc * i + j)) as isize)
                    += alpha_real * temp_real_2 - alpha_imag * temp_imag_2;
                *(C as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (ldc * i + j) + 1 as libc::c_int) as isize,
                    ) += alpha_real * temp_imag_2 + alpha_imag * temp_real_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_gemm_c.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
