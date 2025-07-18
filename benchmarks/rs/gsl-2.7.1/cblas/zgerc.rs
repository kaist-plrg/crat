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
pub unsafe extern "C" fn cblas_zgerc(
    order: CBLAS_ORDER,
    M: libc::c_int,
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut X: *const libc::c_void,
    incX: libc::c_int,
    mut Y: *const libc::c_void,
    incY: libc::c_int,
    mut A: *mut libc::c_void,
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
            b"./source_gerc.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as libc::c_int as isize);
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint {
        let mut ix: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (M - 1 as libc::c_int) * -incX
        };
        i = 0 as libc::c_int;
        while i < M {
            let X_real: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as libc::c_int * ix) as isize);
            let X_imag: libc::c_double = *(X as *const libc::c_double)
                .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
            let tmp_real: libc::c_double = alpha_real * X_real - alpha_imag * X_imag;
            let tmp_imag: libc::c_double = alpha_imag * X_real + alpha_real * X_imag;
            let mut jy: libc::c_int = if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incY
            };
            j = 0 as libc::c_int;
            while j < N {
                let Y_real: libc::c_double = *(Y as *const libc::c_double)
                    .offset((2 as libc::c_int * jy) as isize);
                let Y_imag: libc::c_double = -*(Y as *const libc::c_double)
                    .offset((2 as libc::c_int * jy + 1 as libc::c_int) as isize);
                *(A as *mut libc::c_double)
                    .offset((2 as libc::c_int * (lda * i + j)) as isize)
                    += Y_real * tmp_real - Y_imag * tmp_imag;
                *(A as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (lda * i + j) + 1 as libc::c_int) as isize,
                    ) += Y_imag * tmp_real + Y_real * tmp_imag;
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
            let Y_real_0: libc::c_double = *(Y as *const libc::c_double)
                .offset((2 as libc::c_int * jy_0) as isize);
            let Y_imag_0: libc::c_double = -*(Y as *const libc::c_double)
                .offset((2 as libc::c_int * jy_0 + 1 as libc::c_int) as isize);
            let tmp_real_0: libc::c_double = alpha_real * Y_real_0
                - alpha_imag * Y_imag_0;
            let tmp_imag_0: libc::c_double = alpha_imag * Y_real_0
                + alpha_real * Y_imag_0;
            let mut ix_0: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (M - 1 as libc::c_int) * -incX
            };
            i = 0 as libc::c_int;
            while i < M {
                let X_real_0: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as libc::c_int * ix_0) as isize);
                let X_imag_0: libc::c_double = *(X as *const libc::c_double)
                    .offset((2 as libc::c_int * ix_0 + 1 as libc::c_int) as isize);
                *(A as *mut libc::c_double)
                    .offset((2 as libc::c_int * (i + lda * j)) as isize)
                    += X_real_0 * tmp_real_0 - X_imag_0 * tmp_imag_0;
                *(A as *mut libc::c_double)
                    .offset(
                        (2 as libc::c_int * (i + lda * j) + 1 as libc::c_int) as isize,
                    ) += X_imag_0 * tmp_real_0 + X_real_0 * tmp_imag_0;
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
            b"./source_gerc.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
