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
pub unsafe extern "C" fn cblas_sger(
    order: CBLAS_ORDER,
    M: libc::c_int,
    N: libc::c_int,
    alpha: libc::c_float,
    mut X: *const libc::c_float,
    incX: libc::c_int,
    mut Y: *const libc::c_float,
    incY: libc::c_int,
    mut A: *mut libc::c_float,
    lda: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    if order as libc::c_uint != CblasRowMajor as libc::c_int as libc::c_uint
        && order as libc::c_uint != CblasColMajor as libc::c_int as libc::c_uint
    {
        pos = 1 as libc::c_int;
    }
    if M < 0 as libc::c_int {
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
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        if lda < (if 1 as libc::c_int > N { 1 as libc::c_int } else { N }) {
            pos = 10 as libc::c_int;
        }
    } else if order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint {
        if lda < (if 1 as libc::c_int > M { 1 as libc::c_int } else { M }) {
            pos = 10 as libc::c_int;
        }
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_ger.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        let mut ix: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (M - 1 as libc::c_int) * -incX
        };
        i = 0 as libc::c_int;
        while i < M {
            let tmp: libc::c_float = alpha * *X.offset(ix as isize);
            let mut jy: libc::c_int = if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incY
            };
            j = 0 as libc::c_int;
            while j < N {
                *A.offset((lda * i + j) as isize) += *Y.offset(jy as isize) * tmp;
                jy += incY;
                j += 1;
                j;
            }
            ix += incX;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint {
        let mut jy_0: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        j = 0 as libc::c_int;
        while j < N {
            let tmp_0: libc::c_float = alpha * *Y.offset(jy_0 as isize);
            let mut ix_0: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (M - 1 as libc::c_int) * -incX
            };
            i = 0 as libc::c_int;
            while i < M {
                *A.offset((i + lda * j) as isize) += *X.offset(ix_0 as isize) * tmp_0;
                ix_0 += incX;
                i += 1;
                i;
            }
            jy_0 += incY;
            j += 1;
            j;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_ger.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
