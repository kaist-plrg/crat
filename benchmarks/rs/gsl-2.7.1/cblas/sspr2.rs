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
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub unsafe extern "C" fn cblas_sspr2(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: libc::c_int,
    alpha: libc::c_float,
    mut X: *const libc::c_float,
    incX: libc::c_int,
    mut Y: *const libc::c_float,
    incY: libc::c_int,
    mut Ap: *mut libc::c_float,
) {
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
    if N < 0 as libc::c_int {
        pos = 3 as libc::c_int;
    }
    if incX == 0 as libc::c_int {
        pos = 6 as libc::c_int;
    }
    if incY == 0 as libc::c_int {
        pos = 8 as libc::c_int;
    }
    if pos != 0 {
        cblas_xerbla(
            pos,
            b"./source_spr2.h\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if N == 0 as libc::c_int {
        return;
    }
    if alpha as libc::c_double == 0.0f64 {
        return;
    }
    if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
    {
        let mut ix: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        let mut iy: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            let tmp1: libc::c_double = (alpha * *X.offset(ix as isize))
                as libc::c_double;
            let tmp2: libc::c_double = (alpha * *Y.offset(iy as isize))
                as libc::c_double;
            let mut jx: libc::c_int = ix;
            let mut jy: libc::c_int = iy;
            j = i;
            while j < N {
                let ref mut fresh0 = *Ap
                    .offset(
                        ((i - 1 as libc::c_int + 1 as libc::c_int)
                            * (2 as libc::c_int * N - (i - 1 as libc::c_int))
                            / 2 as libc::c_int + j - i) as isize,
                    );
                *fresh0 = (*fresh0 as libc::c_double
                    + (tmp1 * *Y.offset(jy as isize) as libc::c_double
                        + tmp2 * *X.offset(jx as isize) as libc::c_double))
                    as libc::c_float;
                jx += incX;
                jy += incY;
                j += 1;
                j;
            }
            ix += incX;
            iy += incY;
            i += 1;
            i;
        }
    } else if order as libc::c_uint == CblasRowMajor as libc::c_int as libc::c_uint
        && Uplo as libc::c_uint == CblasLower as libc::c_int as libc::c_uint
        || order as libc::c_uint == CblasColMajor as libc::c_int as libc::c_uint
            && Uplo as libc::c_uint == CblasUpper as libc::c_int as libc::c_uint
    {
        let mut ix_0: libc::c_int = if incX > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incX
        };
        let mut iy_0: libc::c_int = if incY > 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (N - 1 as libc::c_int) * -incY
        };
        i = 0 as libc::c_int;
        while i < N {
            let tmp1_0: libc::c_double = (alpha * *X.offset(ix_0 as isize))
                as libc::c_double;
            let tmp2_0: libc::c_double = (alpha * *Y.offset(iy_0 as isize))
                as libc::c_double;
            let mut jx_0: libc::c_int = if incX > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incX
            };
            let mut jy_0: libc::c_int = if incY > 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (N - 1 as libc::c_int) * -incY
            };
            j = 0 as libc::c_int;
            while j <= i {
                let ref mut fresh1 = *Ap
                    .offset(
                        (i * (i + 1 as libc::c_int) / 2 as libc::c_int + j) as isize,
                    );
                *fresh1 = (*fresh1 as libc::c_double
                    + (tmp1_0 * *Y.offset(jy_0 as isize) as libc::c_double
                        + tmp2_0 * *X.offset(jx_0 as isize) as libc::c_double))
                    as libc::c_float;
                jx_0 += incX;
                jy_0 += incY;
                j += 1;
                j;
            }
            ix_0 += incX;
            iy_0 += incY;
            i += 1;
            i;
        }
    } else {
        cblas_xerbla(
            0 as libc::c_int,
            b"./source_spr2.h\0" as *const u8 as *const libc::c_char,
            b"unrecognized operation\0" as *const u8 as *const libc::c_char,
        );
    };
}
