use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn cblas_sdsdot(
        N: libc::c_int,
        alpha: libc::c_float,
        X: *const libc::c_float,
        incX: libc::c_int,
        Y: *const libc::c_float,
        incY: libc::c_int,
    ) -> libc::c_float;
    fn cblas_dsdot(
        N: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
        Y: *const libc::c_float,
        incY: libc::c_int,
    ) -> libc::c_double;
    fn cblas_sdot(
        N: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
        Y: *const libc::c_float,
        incY: libc::c_int,
    ) -> libc::c_float;
    fn cblas_ddot(
        N: libc::c_int,
        X: *const libc::c_double,
        incX: libc::c_int,
        Y: *const libc::c_double,
        incY: libc::c_int,
    ) -> libc::c_double;
    fn cblas_cdotu_sub(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        dotu: *mut libc::c_void,
    );
    fn cblas_cdotc_sub(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        dotc: *mut libc::c_void,
    );
    fn cblas_zdotu_sub(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        dotu: *mut libc::c_void,
    );
    fn cblas_zdotc_sub(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        dotc: *mut libc::c_void,
    );
    fn cblas_snrm2(
        N: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
    ) -> libc::c_float;
    fn cblas_sasum(
        N: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
    ) -> libc::c_float;
    fn cblas_dnrm2(
        N: libc::c_int,
        X: *const libc::c_double,
        incX: libc::c_int,
    ) -> libc::c_double;
    fn cblas_dasum(
        N: libc::c_int,
        X: *const libc::c_double,
        incX: libc::c_int,
    ) -> libc::c_double;
    fn cblas_scnrm2(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
    ) -> libc::c_float;
    fn cblas_scasum(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
    ) -> libc::c_float;
    fn cblas_dznrm2(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
    ) -> libc::c_double;
    fn cblas_dzasum(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
    ) -> libc::c_double;
    fn cblas_isamax(
        N: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
    ) -> size_t;
    fn cblas_idamax(
        N: libc::c_int,
        X: *const libc::c_double,
        incX: libc::c_int,
    ) -> size_t;
    fn cblas_icamax(N: libc::c_int, X: *const libc::c_void, incX: libc::c_int) -> size_t;
    fn cblas_izamax(N: libc::c_int, X: *const libc::c_void, incX: libc::c_int) -> size_t;
    fn cblas_sswap(
        N: libc::c_int,
        X: *mut libc::c_float,
        incX: libc::c_int,
        Y: *mut libc::c_float,
        incY: libc::c_int,
    );
    fn cblas_scopy(
        N: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
        Y: *mut libc::c_float,
        incY: libc::c_int,
    );
    fn cblas_saxpy(
        N: libc::c_int,
        alpha: libc::c_float,
        X: *const libc::c_float,
        incX: libc::c_int,
        Y: *mut libc::c_float,
        incY: libc::c_int,
    );
    fn cblas_dswap(
        N: libc::c_int,
        X: *mut libc::c_double,
        incX: libc::c_int,
        Y: *mut libc::c_double,
        incY: libc::c_int,
    );
    fn cblas_dcopy(
        N: libc::c_int,
        X: *const libc::c_double,
        incX: libc::c_int,
        Y: *mut libc::c_double,
        incY: libc::c_int,
    );
    fn cblas_daxpy(
        N: libc::c_int,
        alpha: libc::c_double,
        X: *const libc::c_double,
        incX: libc::c_int,
        Y: *mut libc::c_double,
        incY: libc::c_int,
    );
    fn cblas_cswap(
        N: libc::c_int,
        X: *mut libc::c_void,
        incX: libc::c_int,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_ccopy(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_caxpy(
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_zswap(
        N: libc::c_int,
        X: *mut libc::c_void,
        incX: libc::c_int,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_zcopy(
        N: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_zaxpy(
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_srotg(
        a: *mut libc::c_float,
        b: *mut libc::c_float,
        c: *mut libc::c_float,
        s: *mut libc::c_float,
    );
    fn cblas_srotmg(
        d1: *mut libc::c_float,
        d2: *mut libc::c_float,
        b1: *mut libc::c_float,
        b2: libc::c_float,
        P: *mut libc::c_float,
    );
    fn cblas_srot(
        N: libc::c_int,
        X: *mut libc::c_float,
        incX: libc::c_int,
        Y: *mut libc::c_float,
        incY: libc::c_int,
        c: libc::c_float,
        s: libc::c_float,
    );
    fn cblas_srotm(
        N: libc::c_int,
        X: *mut libc::c_float,
        incX: libc::c_int,
        Y: *mut libc::c_float,
        incY: libc::c_int,
        P: *const libc::c_float,
    );
    fn cblas_drotg(
        a: *mut libc::c_double,
        b: *mut libc::c_double,
        c: *mut libc::c_double,
        s: *mut libc::c_double,
    );
    fn cblas_drotmg(
        d1: *mut libc::c_double,
        d2: *mut libc::c_double,
        b1: *mut libc::c_double,
        b2: libc::c_double,
        P: *mut libc::c_double,
    );
    fn cblas_drot(
        N: libc::c_int,
        X: *mut libc::c_double,
        incX: libc::c_int,
        Y: *mut libc::c_double,
        incY: libc::c_int,
        c: libc::c_double,
        s: libc::c_double,
    );
    fn cblas_drotm(
        N: libc::c_int,
        X: *mut libc::c_double,
        incX: libc::c_int,
        Y: *mut libc::c_double,
        incY: libc::c_int,
        P: *const libc::c_double,
    );
    fn cblas_sscal(
        N: libc::c_int,
        alpha: libc::c_float,
        X: *mut libc::c_float,
        incX: libc::c_int,
    );
    fn cblas_dscal(
        N: libc::c_int,
        alpha: libc::c_double,
        X: *mut libc::c_double,
        incX: libc::c_int,
    );
    fn cblas_cscal(
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_zscal(
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_csscal(
        N: libc::c_int,
        alpha: libc::c_float,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_zdscal(
        N: libc::c_int,
        alpha: libc::c_double,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_sgemv(
        order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
        beta: libc::c_float,
        Y: *mut libc::c_float,
        incY: libc::c_int,
    );
    fn cblas_strmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_float,
        lda: libc::c_int,
        X: *mut libc::c_float,
        incX: libc::c_int,
    );
    fn cblas_strsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_float,
        lda: libc::c_int,
        X: *mut libc::c_float,
        incX: libc::c_int,
    );
    fn cblas_dgemv(
        order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        X: *const libc::c_double,
        incX: libc::c_int,
        beta: libc::c_double,
        Y: *mut libc::c_double,
        incY: libc::c_int,
    );
    fn cblas_dtrmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_double,
        lda: libc::c_int,
        X: *mut libc::c_double,
        incX: libc::c_int,
    );
    fn cblas_dtrsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_double,
        lda: libc::c_int,
        X: *mut libc::c_double,
        incX: libc::c_int,
    );
    fn cblas_cgemv(
        order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        beta: *const libc::c_void,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_ctrmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_ctrsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_zgemv(
        order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        beta: *const libc::c_void,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_ztrmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_ztrsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: libc::c_int,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *mut libc::c_void,
        incX: libc::c_int,
    );
    fn cblas_ssymv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        X: *const libc::c_float,
        incX: libc::c_int,
        beta: libc::c_float,
        Y: *mut libc::c_float,
        incY: libc::c_int,
    );
    fn cblas_sger(
        order: CBLAS_ORDER,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_float,
        X: *const libc::c_float,
        incX: libc::c_int,
        Y: *const libc::c_float,
        incY: libc::c_int,
        A: *mut libc::c_float,
        lda: libc::c_int,
    );
    fn cblas_ssyr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_float,
        X: *const libc::c_float,
        incX: libc::c_int,
        A: *mut libc::c_float,
        lda: libc::c_int,
    );
    fn cblas_ssyr2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_float,
        X: *const libc::c_float,
        incX: libc::c_int,
        Y: *const libc::c_float,
        incY: libc::c_int,
        A: *mut libc::c_float,
        lda: libc::c_int,
    );
    fn cblas_dsymv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        X: *const libc::c_double,
        incX: libc::c_int,
        beta: libc::c_double,
        Y: *mut libc::c_double,
        incY: libc::c_int,
    );
    fn cblas_dger(
        order: CBLAS_ORDER,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_double,
        X: *const libc::c_double,
        incX: libc::c_int,
        Y: *const libc::c_double,
        incY: libc::c_int,
        A: *mut libc::c_double,
        lda: libc::c_int,
    );
    fn cblas_dsyr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_double,
        X: *const libc::c_double,
        incX: libc::c_int,
        A: *mut libc::c_double,
        lda: libc::c_int,
    );
    fn cblas_dsyr2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_double,
        X: *const libc::c_double,
        incX: libc::c_int,
        Y: *const libc::c_double,
        incY: libc::c_int,
        A: *mut libc::c_double,
        lda: libc::c_int,
    );
    fn cblas_chemv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        beta: *const libc::c_void,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_cgeru(
        order: CBLAS_ORDER,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_cgerc(
        order: CBLAS_ORDER,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_cher(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_float,
        X: *const libc::c_void,
        incX: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_cher2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_zhemv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        X: *const libc::c_void,
        incX: libc::c_int,
        beta: *const libc::c_void,
        Y: *mut libc::c_void,
        incY: libc::c_int,
    );
    fn cblas_zgeru(
        order: CBLAS_ORDER,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_zgerc(
        order: CBLAS_ORDER,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_zher(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: libc::c_double,
        X: *const libc::c_void,
        incX: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_zher2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: libc::c_int,
        alpha: *const libc::c_void,
        X: *const libc::c_void,
        incX: libc::c_int,
        Y: *const libc::c_void,
        incY: libc::c_int,
        A: *mut libc::c_void,
        lda: libc::c_int,
    );
    fn cblas_sgemm(
        Order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        B: *const libc::c_float,
        ldb: libc::c_int,
        beta: libc::c_float,
        C: *mut libc::c_float,
        ldc: libc::c_int,
    );
    fn cblas_ssymm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        B: *const libc::c_float,
        ldb: libc::c_int,
        beta: libc::c_float,
        C: *mut libc::c_float,
        ldc: libc::c_int,
    );
    fn cblas_ssyrk(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        beta: libc::c_float,
        C: *mut libc::c_float,
        ldc: libc::c_int,
    );
    fn cblas_ssyr2k(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        B: *const libc::c_float,
        ldb: libc::c_int,
        beta: libc::c_float,
        C: *mut libc::c_float,
        ldc: libc::c_int,
    );
    fn cblas_strmm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        B: *mut libc::c_float,
        ldb: libc::c_int,
    );
    fn cblas_strsm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_float,
        lda: libc::c_int,
        B: *mut libc::c_float,
        ldb: libc::c_int,
    );
    fn cblas_dgemm(
        Order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        B: *const libc::c_double,
        ldb: libc::c_int,
        beta: libc::c_double,
        C: *mut libc::c_double,
        ldc: libc::c_int,
    );
    fn cblas_dsymm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        B: *const libc::c_double,
        ldb: libc::c_int,
        beta: libc::c_double,
        C: *mut libc::c_double,
        ldc: libc::c_int,
    );
    fn cblas_dsyrk(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        beta: libc::c_double,
        C: *mut libc::c_double,
        ldc: libc::c_int,
    );
    fn cblas_dsyr2k(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        B: *const libc::c_double,
        ldb: libc::c_int,
        beta: libc::c_double,
        C: *mut libc::c_double,
        ldc: libc::c_int,
    );
    fn cblas_dtrmm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        B: *mut libc::c_double,
        ldb: libc::c_int,
    );
    fn cblas_dtrsm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_double,
        lda: libc::c_int,
        B: *mut libc::c_double,
        ldb: libc::c_int,
    );
    fn cblas_cgemm(
        Order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_csymm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_csyrk(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_csyr2k(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_ctrmm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *mut libc::c_void,
        ldb: libc::c_int,
    );
    fn cblas_ctrsm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *mut libc::c_void,
        ldb: libc::c_int,
    );
    fn cblas_zgemm(
        Order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: libc::c_int,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_zsymm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_zsyrk(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_zsyr2k(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_ztrmm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *mut libc::c_void,
        ldb: libc::c_int,
    );
    fn cblas_ztrsm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *mut libc::c_void,
        ldb: libc::c_int,
    );
    fn cblas_chemm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_cherk(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_float,
        A: *const libc::c_void,
        lda: libc::c_int,
        beta: libc::c_float,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_cher2k(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: libc::c_float,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_zhemm(
        Order: CBLAS_ORDER,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: libc::c_int,
        N: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: *const libc::c_void,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_zherk(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: libc::c_double,
        A: *const libc::c_void,
        lda: libc::c_int,
        beta: libc::c_double,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
    fn cblas_zher2k(
        Order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: libc::c_int,
        K: libc::c_int,
        alpha: *const libc::c_void,
        A: *const libc::c_void,
        lda: libc::c_int,
        B: *const libc::c_void,
        ldb: libc::c_int,
        beta: libc::c_double,
        C: *mut libc::c_void,
        ldc: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
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
pub type CBLAS_INDEX_t = size_t;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
pub type CBLAS_SIDE_t = CBLAS_SIDE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex_float {
    pub dat: [libc::c_float; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
pub unsafe extern "C" fn gsl_blas_sdsdot(
    mut alpha: libc::c_float,
    mut X: *const gsl_vector_float,
    mut Y: *const gsl_vector_float,
    mut result: *mut libc::c_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        *result = cblas_sdsdot(
            (*X).size as libc::c_int,
            alpha,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dsdot(
    mut X: *const gsl_vector_float,
    mut Y: *const gsl_vector_float,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        *result = cblas_dsdot(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_sdot(
    mut X: *const gsl_vector_float,
    mut Y: *const gsl_vector_float,
    mut result: *mut libc::c_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        *result = cblas_sdot(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ddot(
    mut X: *const gsl_vector,
    mut Y: *const gsl_vector,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        *result = cblas_ddot(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cdotu(
    mut X: *const gsl_vector_complex_float,
    mut Y: *const gsl_vector_complex_float,
    mut dotu: *mut gsl_complex_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_cdotu_sub(
            (*X).size as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            ((*dotu).dat).as_mut_ptr() as *mut libc::c_void,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cdotc(
    mut X: *const gsl_vector_complex_float,
    mut Y: *const gsl_vector_complex_float,
    mut dotc: *mut gsl_complex_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_cdotc_sub(
            (*X).size as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            ((*dotc).dat).as_mut_ptr() as *mut libc::c_void,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zdotu(
    mut X: *const gsl_vector_complex,
    mut Y: *const gsl_vector_complex,
    mut dotu: *mut gsl_complex,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_zdotu_sub(
            (*X).size as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            ((*dotu).dat).as_mut_ptr() as *mut libc::c_void,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zdotc(
    mut X: *const gsl_vector_complex,
    mut Y: *const gsl_vector_complex,
    mut dotc: *mut gsl_complex,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_zdotc_sub(
            (*X).size as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            ((*dotc).dat).as_mut_ptr() as *mut libc::c_void,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_snrm2(
    mut X: *const gsl_vector_float,
) -> libc::c_float {
    return cblas_snrm2((*X).size as libc::c_int, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_dnrm2(mut X: *const gsl_vector) -> libc::c_double {
    return cblas_dnrm2((*X).size as libc::c_int, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_scnrm2(
    mut X: *const gsl_vector_complex_float,
) -> libc::c_float {
    return cblas_scnrm2(
        (*X).size as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_dznrm2(
    mut X: *const gsl_vector_complex,
) -> libc::c_double {
    return cblas_dznrm2(
        (*X).size as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_sasum(
    mut X: *const gsl_vector_float,
) -> libc::c_float {
    return cblas_sasum((*X).size as libc::c_int, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_dasum(mut X: *const gsl_vector) -> libc::c_double {
    return cblas_dasum((*X).size as libc::c_int, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_scasum(
    mut X: *const gsl_vector_complex_float,
) -> libc::c_float {
    return cblas_scasum(
        (*X).size as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_dzasum(
    mut X: *const gsl_vector_complex,
) -> libc::c_double {
    return cblas_dzasum(
        (*X).size as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_isamax(
    mut X: *const gsl_vector_float,
) -> CBLAS_INDEX_t {
    return cblas_isamax((*X).size as libc::c_int, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_idamax(mut X: *const gsl_vector) -> CBLAS_INDEX_t {
    return cblas_idamax((*X).size as libc::c_int, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_icamax(
    mut X: *const gsl_vector_complex_float,
) -> CBLAS_INDEX_t {
    return cblas_icamax(
        (*X).size as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_izamax(
    mut X: *const gsl_vector_complex,
) -> CBLAS_INDEX_t {
    return cblas_izamax(
        (*X).size as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_sswap(
    mut X: *mut gsl_vector_float,
    mut Y: *mut gsl_vector_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_sswap(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dswap(
    mut X: *mut gsl_vector,
    mut Y: *mut gsl_vector,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_dswap(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cswap(
    mut X: *mut gsl_vector_complex_float,
    mut Y: *mut gsl_vector_complex_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_cswap(
            (*X).size as libc::c_int,
            (*X).data as *mut libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zswap(
    mut X: *mut gsl_vector_complex,
    mut Y: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_zswap(
            (*X).size as libc::c_int,
            (*X).data as *mut libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_scopy(
    mut X: *const gsl_vector_float,
    mut Y: *mut gsl_vector_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_scopy(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dcopy(
    mut X: *const gsl_vector,
    mut Y: *mut gsl_vector,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_dcopy(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ccopy(
    mut X: *const gsl_vector_complex_float,
    mut Y: *mut gsl_vector_complex_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_ccopy(
            (*X).size as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            365 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zcopy(
    mut X: *const gsl_vector_complex,
    mut Y: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_zcopy(
            (*X).size as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            380 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_saxpy(
    mut alpha: libc::c_float,
    mut X: *const gsl_vector_float,
    mut Y: *mut gsl_vector_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_saxpy(
            (*X).size as libc::c_int,
            alpha,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            398 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_daxpy(
    mut alpha: libc::c_double,
    mut X: *const gsl_vector,
    mut Y: *mut gsl_vector,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_daxpy(
            (*X).size as libc::c_int,
            alpha,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_caxpy(
    alpha: gsl_complex_float,
    mut X: *const gsl_vector_complex_float,
    mut Y: *mut gsl_vector_complex_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_caxpy(
            (*X).size as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zaxpy(
    alpha: gsl_complex,
    mut X: *const gsl_vector_complex,
    mut Y: *mut gsl_vector_complex,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_zaxpy(
            (*X).size as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            446 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_srotg(
    mut a: *mut libc::c_float,
    mut b: *mut libc::c_float,
    mut c: *mut libc::c_float,
    mut s: *mut libc::c_float,
) -> libc::c_int {
    cblas_srotg(a, b, c, s);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_drotg(
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) -> libc::c_int {
    cblas_drotg(a, b, c, s);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_srot(
    mut X: *mut gsl_vector_float,
    mut Y: *mut gsl_vector_float,
    mut c: libc::c_float,
    mut s: libc::c_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_srot(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
            c,
            s,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_drot(
    mut X: *mut gsl_vector,
    mut Y: *mut gsl_vector,
    c: libc::c_double,
    s: libc::c_double,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_drot(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
            c,
            s,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            494 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_srotmg(
    mut d1: *mut libc::c_float,
    mut d2: *mut libc::c_float,
    mut b1: *mut libc::c_float,
    mut b2: libc::c_float,
    mut P: *mut libc::c_float,
) -> libc::c_int {
    cblas_srotmg(d1, d2, b1, b2, P);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_drotmg(
    mut d1: *mut libc::c_double,
    mut d2: *mut libc::c_double,
    mut b1: *mut libc::c_double,
    mut b2: libc::c_double,
    mut P: *mut libc::c_double,
) -> libc::c_int {
    cblas_drotmg(d1, d2, b1, b2, P);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_srotm(
    mut X: *mut gsl_vector_float,
    mut Y: *mut gsl_vector_float,
    mut P: *const libc::c_float,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_srotm(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
            P,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            529 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_drotm(
    mut X: *mut gsl_vector,
    mut Y: *mut gsl_vector,
    mut P: *const libc::c_double,
) -> libc::c_int {
    if (*X).size == (*Y).size {
        cblas_drotm(
            (*X).size as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
            P,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            544 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_sscal(
    mut alpha: libc::c_float,
    mut X: *mut gsl_vector_float,
) {
    cblas_sscal((*X).size as libc::c_int, alpha, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_dscal(
    mut alpha: libc::c_double,
    mut X: *mut gsl_vector,
) {
    cblas_dscal((*X).size as libc::c_int, alpha, (*X).data, (*X).stride as libc::c_int);
}
pub unsafe extern "C" fn gsl_blas_cscal(
    alpha: gsl_complex_float,
    mut X: *mut gsl_vector_complex_float,
) {
    cblas_cscal(
        (*X).size as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_zscal(
    alpha: gsl_complex,
    mut X: *mut gsl_vector_complex,
) {
    cblas_zscal(
        (*X).size as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_csscal(
    mut alpha: libc::c_float,
    mut X: *mut gsl_vector_complex_float,
) {
    cblas_csscal(
        (*X).size as libc::c_int,
        alpha,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_zdscal(
    mut alpha: libc::c_double,
    mut X: *mut gsl_vector_complex,
) {
    cblas_zdscal(
        (*X).size as libc::c_int,
        alpha,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
}
pub unsafe extern "C" fn gsl_blas_sgemv(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut X: *const gsl_vector_float,
    mut beta: libc::c_float,
    mut Y: *mut gsl_vector_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        && N == (*X).size && M == (*Y).size
        || TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            && M == (*X).size && N == (*Y).size
    {
        cblas_sgemv(
            CblasRowMajor,
            TransA,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            beta,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            614 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dgemv(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut X: *const gsl_vector,
    mut beta: libc::c_double,
    mut Y: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        && N == (*X).size && M == (*Y).size
        || TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            && M == (*X).size && N == (*Y).size
    {
        cblas_dgemv(
            CblasRowMajor,
            TransA,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*X).data,
            (*X).stride as libc::c_int,
            beta,
            (*Y).data,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            636 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cgemv(
    mut TransA: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut X: *const gsl_vector_complex_float,
    beta: gsl_complex_float,
    mut Y: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        && N == (*X).size && M == (*Y).size
        || TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            && M == (*X).size && N == (*Y).size
        || TransA as libc::c_uint == CblasConjTrans as libc::c_int as libc::c_uint
            && M == (*X).size && N == (*Y).size
    {
        cblas_cgemv(
            CblasRowMajor,
            TransA,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            662 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zgemv(
    mut TransA: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut X: *const gsl_vector_complex,
    beta: gsl_complex,
    mut Y: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        && N == (*X).size && M == (*Y).size
        || TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            && M == (*X).size && N == (*Y).size
        || TransA as libc::c_uint == CblasConjTrans as libc::c_int as libc::c_uint
            && M == (*X).size && N == (*Y).size
    {
        cblas_zgemv(
            CblasRowMajor,
            TransA,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*Y).data as *mut libc::c_void,
            (*Y).stride as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            687 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_chemv(
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut X: *const gsl_vector_complex_float,
    beta: gsl_complex_float,
    mut Y: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            706 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size || N != (*Y).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            710 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_chemv(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
        (beta.dat).as_ptr() as *const libc::c_void,
        (*Y).data as *mut libc::c_void,
        (*Y).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_zhemv(
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut X: *const gsl_vector_complex,
    beta: gsl_complex,
    mut Y: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            729 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size || N != (*Y).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            733 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_zhemv(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
        (beta.dat).as_ptr() as *const libc::c_void,
        (*Y).data as *mut libc::c_void,
        (*Y).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ssymv(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut X: *const gsl_vector_float,
    mut beta: libc::c_float,
    mut Y: *mut gsl_vector_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            754 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size || N != (*Y).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            758 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ssymv(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        alpha,
        (*A).data,
        (*A).tda as libc::c_int,
        (*X).data,
        (*X).stride as libc::c_int,
        beta,
        (*Y).data,
        (*Y).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_dsymv(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut X: *const gsl_vector,
    mut beta: libc::c_double,
    mut Y: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            775 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size || N != (*Y).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            779 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_dsymv(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        alpha,
        (*A).data,
        (*A).tda as libc::c_int,
        (*X).data,
        (*X).stride as libc::c_int,
        beta,
        (*Y).data,
        (*Y).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_strmv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix_float,
    mut X: *mut gsl_vector_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            800 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            804 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_strmv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
        (*X).data,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_dtrmv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix,
    mut X: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            822 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            826 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_dtrmv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
        (*X).data,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ctrmv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix_complex_float,
    mut X: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            845 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            849 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ctrmv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ztrmv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix_complex,
    mut X: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            868 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            872 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ztrmv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_strsv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix_float,
    mut X: *mut gsl_vector_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            893 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            897 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_strsv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
        (*X).data,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_dtrsv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix,
    mut X: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            915 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            919 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_dtrsv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
        (*X).data,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ctrsv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix_complex_float,
    mut X: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            938 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            942 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ctrsv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ztrsv(
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut A: *const gsl_matrix_complex,
    mut X: *mut gsl_vector_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            961 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*X).size {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            965 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ztrsv(
        CblasRowMajor,
        Uplo,
        TransA,
        Diag,
        N as libc::c_int,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*X).data as *mut libc::c_void,
        (*X).stride as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_sger(
    mut alpha: libc::c_float,
    mut X: *const gsl_vector_float,
    mut Y: *const gsl_vector_float,
    mut A: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*X).size == M && (*Y).size == N {
        cblas_sger(
            CblasRowMajor,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
            (*A).data,
            (*A).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            992 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dger(
    mut alpha: libc::c_double,
    mut X: *const gsl_vector,
    mut Y: *const gsl_vector,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*X).size == M && (*Y).size == N {
        cblas_dger(
            CblasRowMajor,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*X).data,
            (*X).stride as libc::c_int,
            (*Y).data,
            (*Y).stride as libc::c_int,
            (*A).data,
            (*A).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1013 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cgeru(
    alpha: gsl_complex_float,
    mut X: *const gsl_vector_complex_float,
    mut Y: *const gsl_vector_complex_float,
    mut A: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*X).size == M && (*Y).size == N {
        cblas_cgeru(
            CblasRowMajor,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            (*A).data as *mut libc::c_void,
            (*A).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1038 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zgeru(
    alpha: gsl_complex,
    mut X: *const gsl_vector_complex,
    mut Y: *const gsl_vector_complex,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*X).size == M && (*Y).size == N {
        cblas_zgeru(
            CblasRowMajor,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            (*A).data as *mut libc::c_void,
            (*A).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1058 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cgerc(
    alpha: gsl_complex_float,
    mut X: *const gsl_vector_complex_float,
    mut Y: *const gsl_vector_complex_float,
    mut A: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*X).size == M && (*Y).size == N {
        cblas_cgerc(
            CblasRowMajor,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            (*A).data as *mut libc::c_void,
            (*A).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1083 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zgerc(
    alpha: gsl_complex,
    mut X: *const gsl_vector_complex,
    mut Y: *const gsl_vector_complex,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if (*X).size == M && (*Y).size == N {
        cblas_zgerc(
            CblasRowMajor,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*X).data as *const libc::c_void,
            (*X).stride as libc::c_int,
            (*Y).data as *const libc::c_void,
            (*Y).stride as libc::c_int,
            (*A).data as *mut libc::c_void,
            (*A).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1104 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cher(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_float,
    mut X: *const gsl_vector_complex_float,
    mut A: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1120 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1124 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_cher(
        CblasRowMajor,
        Uplo,
        M as libc::c_int,
        alpha,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
        (*A).data as *mut libc::c_void,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_zher(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_double,
    mut X: *const gsl_vector_complex,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1142 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1146 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_zher(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        alpha,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
        (*A).data as *mut libc::c_void,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_cher2(
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex_float,
    mut X: *const gsl_vector_complex_float,
    mut Y: *const gsl_vector_complex_float,
    mut A: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1168 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N || (*Y).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1172 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_cher2(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
        (*Y).data as *const libc::c_void,
        (*Y).stride as libc::c_int,
        (*A).data as *mut libc::c_void,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_zher2(
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex,
    mut X: *const gsl_vector_complex,
    mut Y: *const gsl_vector_complex,
    mut A: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1192 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N || (*Y).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1196 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_zher2(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*X).data as *const libc::c_void,
        (*X).stride as libc::c_int,
        (*Y).data as *const libc::c_void,
        (*Y).stride as libc::c_int,
        (*A).data as *mut libc::c_void,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ssyr(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_float,
    mut X: *const gsl_vector_float,
    mut A: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1217 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1221 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ssyr(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        alpha,
        (*X).data,
        (*X).stride as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_dsyr(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_double,
    mut X: *const gsl_vector,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1239 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1243 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_dsyr(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        alpha,
        (*X).data,
        (*X).stride as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ssyr2(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_float,
    mut X: *const gsl_vector_float,
    mut Y: *const gsl_vector_float,
    mut A: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1263 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N || (*Y).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1267 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ssyr2(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        alpha,
        (*X).data,
        (*X).stride as libc::c_int,
        (*Y).data,
        (*Y).stride as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_dsyr2(
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_double,
    mut X: *const gsl_vector,
    mut Y: *const gsl_vector,
    mut A: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M != N {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1285 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*X).size != N || (*Y).size != N {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1289 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_dsyr2(
        CblasRowMajor,
        Uplo,
        N as libc::c_int,
        alpha,
        (*X).data,
        (*X).stride as libc::c_int,
        (*Y).data,
        (*Y).stride as libc::c_int,
        (*A).data,
        (*A).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_sgemm(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut TransB: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut B: *const gsl_matrix_float,
    mut beta: libc::c_float,
    mut C: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M == MA && N == NB && NA == MB {
        cblas_sgemm(
            CblasRowMajor,
            TransA,
            TransB,
            M as libc::c_int,
            N as libc::c_int,
            NA as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
            beta,
            (*C).data,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1328 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dgemm(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut TransB: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut beta: libc::c_double,
    mut C: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M == MA && N == NB && NA == MB {
        cblas_dgemm(
            CblasRowMajor,
            TransA,
            TransB,
            M as libc::c_int,
            N as libc::c_int,
            NA as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
            beta,
            (*C).data,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1354 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_cgemm(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut TransB: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut B: *const gsl_matrix_complex_float,
    beta: gsl_complex_float,
    mut C: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M == MA && N == NB && NA == MB {
        cblas_cgemm(
            CblasRowMajor,
            TransA,
            TransB,
            M as libc::c_int,
            N as libc::c_int,
            NA as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *const libc::c_void,
            (*B).tda as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*C).data as *mut libc::c_void,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1383 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zgemm(
    mut TransA: CBLAS_TRANSPOSE_t,
    mut TransB: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut B: *const gsl_matrix_complex,
    beta: gsl_complex,
    mut C: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if TransA as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if TransB as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M == MA && N == NB && NA == MB {
        cblas_zgemm(
            CblasRowMajor,
            TransA,
            TransB,
            M as libc::c_int,
            N as libc::c_int,
            NA as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *const libc::c_void,
            (*B).tda as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*C).data as *mut libc::c_void,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1411 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ssymm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut B: *const gsl_matrix_float,
    mut beta: libc::c_float,
    mut C: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    let MB: size_t = (*B).size1;
    let NB: size_t = (*B).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1432 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint
        && (M == MA && N == NB && NA == MB)
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint
            && (M == MB && N == NA && NB == MA)
    {
        cblas_ssymm(
            CblasRowMajor,
            Side,
            Uplo,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
            beta,
            (*C).data,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1445 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dsymm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut beta: libc::c_double,
    mut C: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    let MB: size_t = (*B).size1;
    let NB: size_t = (*B).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1465 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint
        && (M == MA && N == NB && NA == MB)
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint
            && (M == MB && N == NA && NB == MA)
    {
        cblas_dsymm(
            CblasRowMajor,
            Side,
            Uplo,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
            beta,
            (*C).data,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1478 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_csymm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut B: *const gsl_matrix_complex_float,
    beta: gsl_complex_float,
    mut C: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    let MB: size_t = (*B).size1;
    let NB: size_t = (*B).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1499 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint
        && (M == MA && N == NB && NA == MB)
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint
            && (M == MB && N == NA && NB == MA)
    {
        cblas_csymm(
            CblasRowMajor,
            Side,
            Uplo,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *const libc::c_void,
            (*B).tda as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*C).data as *mut libc::c_void,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1513 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zsymm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut B: *const gsl_matrix_complex,
    beta: gsl_complex,
    mut C: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    let MB: size_t = (*B).size1;
    let NB: size_t = (*B).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1532 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint
        && (M == MA && N == NB && NA == MB)
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint
            && (M == MB && N == NA && NB == MA)
    {
        cblas_zsymm(
            CblasRowMajor,
            Side,
            Uplo,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *const libc::c_void,
            (*B).tda as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*C).data as *mut libc::c_void,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1546 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_chemm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut B: *const gsl_matrix_complex_float,
    beta: gsl_complex_float,
    mut C: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    let MB: size_t = (*B).size1;
    let NB: size_t = (*B).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1569 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint
        && (M == MA && N == NB && NA == MB)
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint
            && (M == MB && N == NA && NB == MA)
    {
        cblas_chemm(
            CblasRowMajor,
            Side,
            Uplo,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *const libc::c_void,
            (*B).tda as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*C).data as *mut libc::c_void,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1583 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_zhemm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut B: *const gsl_matrix_complex,
    beta: gsl_complex,
    mut C: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    let MB: size_t = (*B).size1;
    let NB: size_t = (*B).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1604 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint
        && (M == MA && N == NB && NA == MB)
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint
            && (M == MB && N == NA && NB == MA)
    {
        cblas_zhemm(
            CblasRowMajor,
            Side,
            Uplo,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *const libc::c_void,
            (*B).tda as libc::c_int,
            (beta.dat).as_ptr() as *const libc::c_void,
            (*C).data as *mut libc::c_void,
            (*C).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1618 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ssyrk(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut beta: libc::c_float,
    mut C: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let J: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let K: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1635 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != J {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1639 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ssyrk(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        K as libc::c_int,
        alpha,
        (*A).data,
        (*A).tda as libc::c_int,
        beta,
        (*C).data,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_dsyrk(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut beta: libc::c_double,
    mut C: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let J: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let K: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1659 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != J {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1663 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_dsyrk(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        K as libc::c_int,
        alpha,
        (*A).data,
        (*A).tda as libc::c_int,
        beta,
        (*C).data,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_csyrk(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    beta: gsl_complex_float,
    mut C: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let J: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let K: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1686 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != J {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1690 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_csyrk(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        K as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (beta.dat).as_ptr() as *const libc::c_void,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_zsyrk(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    beta: gsl_complex,
    mut C: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let J: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let K: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1712 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != J {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1716 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_zsyrk(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        K as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (beta.dat).as_ptr() as *const libc::c_void,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_cherk(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_complex_float,
    mut beta: libc::c_float,
    mut C: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let J: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let K: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1739 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != J {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1743 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_cherk(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        K as libc::c_int,
        alpha,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        beta,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_zherk(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix_complex,
    mut beta: libc::c_double,
    mut C: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let J: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let K: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1764 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != J {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1768 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_zherk(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        K as libc::c_int,
        alpha,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        beta,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_ssyr2k(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut B: *const gsl_matrix_float,
    mut beta: libc::c_float,
    mut C: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1792 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != MA || N != MB || NA != NB {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1796 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_ssyr2k(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        NA as libc::c_int,
        alpha,
        (*A).data,
        (*A).tda as libc::c_int,
        (*B).data,
        (*B).tda as libc::c_int,
        beta,
        (*C).data,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_dsyr2k(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut beta: libc::c_double,
    mut C: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1820 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != MA || N != MB || NA != NB {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1824 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_dsyr2k(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        NA as libc::c_int,
        alpha,
        (*A).data,
        (*A).tda as libc::c_int,
        (*B).data,
        (*B).tda as libc::c_int,
        beta,
        (*C).data,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_csyr2k(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut B: *const gsl_matrix_complex_float,
    beta: gsl_complex_float,
    mut C: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1850 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != MA || N != MB || NA != NB {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1854 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_csyr2k(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        NA as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*B).data as *const libc::c_void,
        (*B).tda as libc::c_int,
        (beta.dat).as_ptr() as *const libc::c_void,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_zsyr2k(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut B: *const gsl_matrix_complex,
    beta: gsl_complex,
    mut C: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1880 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != MA || N != MB || NA != NB {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1884 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_zsyr2k(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        NA as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*B).data as *const libc::c_void,
        (*B).tda as libc::c_int,
        (beta.dat).as_ptr() as *const libc::c_void,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_cher2k(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut B: *const gsl_matrix_complex_float,
    mut beta: libc::c_float,
    mut C: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1911 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != MA || N != MB || NA != NB {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1915 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_cher2k(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        NA as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*B).data as *const libc::c_void,
        (*B).tda as libc::c_int,
        beta,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_zher2k(
    mut Uplo: CBLAS_UPLO_t,
    mut Trans: CBLAS_TRANSPOSE_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut B: *const gsl_matrix_complex,
    mut beta: libc::c_double,
    mut C: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*C).size1;
    let N: size_t = (*C).size2;
    let MA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size1
    } else {
        (*A).size2
    };
    let NA: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*A).size2
    } else {
        (*A).size1
    };
    let MB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size1
    } else {
        (*B).size2
    };
    let NB: size_t = if Trans as libc::c_uint
        == CblasNoTrans as libc::c_int as libc::c_uint
    {
        (*B).size2
    } else {
        (*B).size1
    };
    if M != N {
        gsl_error(
            b"matrix C must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1941 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != MA || N != MB || NA != NB {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1945 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    cblas_zher2k(
        CblasRowMajor,
        Uplo,
        Trans,
        N as libc::c_int,
        NA as libc::c_int,
        (alpha.dat).as_ptr() as *const libc::c_void,
        (*A).data as *const libc::c_void,
        (*A).tda as libc::c_int,
        (*B).data as *const libc::c_void,
        (*B).tda as libc::c_int,
        beta,
        (*C).data as *mut libc::c_void,
        (*C).tda as libc::c_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_blas_strmm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut B: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1969 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_strmm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1980 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dtrmm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut B: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            1997 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_dtrmm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2008 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ctrmm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut B: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2027 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_ctrmm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *mut libc::c_void,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2039 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ztrmm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut B: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2057 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_ztrmm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *mut libc::c_void,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2069 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_strsm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut alpha: libc::c_float,
    mut A: *const gsl_matrix_float,
    mut B: *mut gsl_matrix_float,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2088 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_strsm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2099 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_dtrsm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    mut alpha: libc::c_double,
    mut A: *const gsl_matrix,
    mut B: *mut gsl_matrix,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2116 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_dtrsm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            alpha,
            (*A).data,
            (*A).tda as libc::c_int,
            (*B).data,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2127 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ctrsm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    alpha: gsl_complex_float,
    mut A: *const gsl_matrix_complex_float,
    mut B: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2146 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_ctrsm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *mut libc::c_void,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2158 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_blas_ztrsm(
    mut Side: CBLAS_SIDE_t,
    mut Uplo: CBLAS_UPLO_t,
    mut TransA: CBLAS_TRANSPOSE_t,
    mut Diag: CBLAS_DIAG_t,
    alpha: gsl_complex,
    mut A: *const gsl_matrix_complex,
    mut B: *mut gsl_matrix_complex,
) -> libc::c_int {
    let M: size_t = (*B).size1;
    let N: size_t = (*B).size2;
    let MA: size_t = (*A).size1;
    let NA: size_t = (*A).size2;
    if MA != NA {
        gsl_error(
            b"matrix A must be square\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2176 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    }
    if Side as libc::c_uint == CblasLeft as libc::c_int as libc::c_uint && M == MA
        || Side as libc::c_uint == CblasRight as libc::c_int as libc::c_uint && N == MA
    {
        cblas_ztrsm(
            CblasRowMajor,
            Side,
            Uplo,
            TransA,
            Diag,
            M as libc::c_int,
            N as libc::c_int,
            (alpha.dat).as_ptr() as *const libc::c_void,
            (*A).data as *const libc::c_void,
            (*A).tda as libc::c_int,
            (*B).data as *mut libc::c_void,
            (*B).tda as libc::c_int,
        );
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_error(
            b"invalid length\0" as *const u8 as *const libc::c_char,
            b"blas.c\0" as *const u8 as *const libc::c_char,
            2188 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    };
}
