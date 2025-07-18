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
pub unsafe extern "C" fn cblas_dgemm(
    Order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: libc::c_int,
    N: libc::c_int,
    K: libc::c_int,
    alpha: libc::c_double,
    mut A: *const libc::c_double,
    lda: libc::c_int,
    mut B: *const libc::c_double,
    ldb: libc::c_int,
    beta: libc::c_double,
    mut C: *mut libc::c_double,
    ldc: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut ldf: libc::c_int = 0;
    let mut ldg: libc::c_int = 0;
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
            b"./source_gemm_r.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if alpha == 0.0f64 && beta == 1.0f64 {
        return;
    }
    if Order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        n1 = M;
        n2 = N;
        F = A;
        ldf = lda;
        TransF = (if TransA as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            CblasTrans as libc::c_int as libc::c_uint
        } else {
            TransA as libc::c_uint
        }) as libc::c_int;
        G = B;
        ldg = ldb;
        TransG = (if TransB as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            CblasTrans as libc::c_int as libc::c_uint
        } else {
            TransB as libc::c_uint
        }) as libc::c_int;
    } else {
        n1 = N;
        n2 = M;
        F = B;
        ldf = ldb;
        TransF = (if TransB as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            CblasTrans as libc::c_int as libc::c_uint
        } else {
            TransB as libc::c_uint
        }) as libc::c_int;
        G = A;
        ldg = lda;
        TransG = (if TransA as libc::c_uint
            == CblasConjTrans as libc::c_int as libc::c_uint
        {
            CblasTrans as libc::c_int as libc::c_uint
        } else {
            TransA as libc::c_uint
        }) as libc::c_int;
    }
    if beta == 0.0f64 {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                *C.offset((ldc * i + j) as isize) = 0.0f64;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if beta != 1.0f64 {
        i = 0 as libc::c_int;
        while i < n1 {
            j = 0 as libc::c_int;
            while j < n2 {
                *C.offset((ldc * i + j) as isize) *= beta;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    if alpha == 0.0f64 {
        return;
    }
    if TransF == CblasNoTrans as libc::c_int && TransG == CblasNoTrans as libc::c_int {
        k = 0 as libc::c_int;
        while k < K {
            i = 0 as libc::c_int;
            while i < n1 {
                let temp: libc::c_double = alpha * *F.offset((ldf * i + k) as isize);
                if temp != 0.0f64 {
                    j = 0 as libc::c_int;
                    while j < n2 {
                        *C.offset((ldc * i + j) as isize)
                            += temp * *G.offset((ldg * k + j) as isize);
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
                let mut temp_0: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    temp_0
                        += *F.offset((ldf * i + k) as isize)
                            * *G.offset((ldg * j + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((ldc * i + j) as isize) += alpha * temp_0;
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
                let temp_1: libc::c_double = alpha * *F.offset((ldf * k + i) as isize);
                if temp_1 != 0.0f64 {
                    j = 0 as libc::c_int;
                    while j < n2 {
                        *C.offset((ldc * i + j) as isize)
                            += temp_1 * *G.offset((ldg * k + j) as isize);
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
                let mut temp_2: libc::c_double = 0.0f64;
                k = 0 as libc::c_int;
                while k < K {
                    temp_2
                        += *F.offset((ldf * k + i) as isize)
                            * *G.offset((ldg * j + k) as isize);
                    k += 1;
                    k;
                }
                *C.offset((ldc * i + j) as isize) += alpha * temp_2;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_gemm_r.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
