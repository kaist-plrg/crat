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
pub unsafe extern "C" fn cblas_strsm(
    Order: CBLAS_ORDER,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: libc::c_int,
    N: libc::c_int,
    alpha: libc::c_float,
    mut A: *const libc::c_float,
    lda: libc::c_int,
    mut B: *mut libc::c_float,
    ldb: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let nonunit: libc::c_int = (Diag as libc::c_uint
        == CblasNonUnit as libc::c_int as libc::c_uint) as libc::c_int;
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
            b"./source_trsm_r.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        n1 = M;
        n2 = N;
        side = Side as libc::c_int;
        uplo = Uplo as libc::c_int;
        trans = (if TransA as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            CblasTrans as libc::c_int as libc::c_uint
        } else {
            TransA as libc::c_uint
        }) as libc::c_int;
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
        trans = (if TransA as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            CblasTrans as libc::c_int as libc::c_uint
        } else {
            TransA as libc::c_uint
        }) as libc::c_int;
    }
    if side == CblasLeft as libc::c_int && uplo == CblasUpper as libc::c_int
        && trans == CblasNoTrans as libc::c_int
    {
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                let mut Aii: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii;
                    j += 1;
                    j;
                }
            }
            k = 0 as libc::c_int;
            while k < i {
                let Aki: libc::c_float = *A.offset((k * lda + i) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aki * *B.offset((ldb * i + j) as isize);
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
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                let mut Aii_0: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii_0;
                    j += 1;
                    j;
                }
            }
            k = i + 1 as libc::c_int;
            while k < n1 {
                let Aik: libc::c_float = *A.offset((i * lda + k) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aik * *B.offset((ldb * i + j) as isize);
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
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                let mut Aii_1: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii_1;
                    j += 1;
                    j;
                }
            }
            k = i + 1 as libc::c_int;
            while k < n1 {
                let Aki_0: libc::c_float = *A.offset((k * lda + i) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aki_0 * *B.offset((ldb * i + j) as isize);
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
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                let mut Aii_2: libc::c_float = *A.offset((lda * i + i) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) /= Aii_2;
                    j += 1;
                    j;
                }
            }
            k = 0 as libc::c_int;
            while k < i {
                let Aik_0: libc::c_float = *A.offset((i * lda + k) as isize);
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * k + j) as isize)
                        -= Aik_0 * *B.offset((ldb * i + j) as isize);
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
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                    let mut Ajj: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj;
                }
                let mut Bij: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = j + 1 as libc::c_int;
                while k < n2 {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((j * lda + k) as isize) * Bij;
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
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                    let mut Ajj_0: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj_0;
                }
                let mut Bij_0: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = 0 as libc::c_int;
                while k < j {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((k * lda + j) as isize) * Bij_0;
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
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                    let mut Ajj_1: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj_1;
                }
                let mut Bij_1: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = 0 as libc::c_int;
                while k < j {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((j * lda + k) as isize) * Bij_1;
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
        if alpha as libc::c_double != 1.0f64 {
            i = 0 as libc::c_int;
            while i < n1 {
                j = 0 as libc::c_int;
                while j < n2 {
                    *B.offset((ldb * i + j) as isize) *= alpha;
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
                    let mut Ajj_2: libc::c_float = *A.offset((lda * j + j) as isize);
                    *B.offset((ldb * i + j) as isize) /= Ajj_2;
                }
                let mut Bij_2: libc::c_float = *B.offset((ldb * i + j) as isize);
                k = j + 1 as libc::c_int;
                while k < n2 {
                    *B.offset((ldb * i + k) as isize)
                        -= *A.offset((k * lda + j) as isize) * Bij_2;
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
            b"./source_trsm_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
