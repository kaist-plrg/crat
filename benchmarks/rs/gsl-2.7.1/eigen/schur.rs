use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_zdscal(alpha: libc::c_double, X: *mut gsl_vector_complex);
    fn gsl_complex_div(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_mul_real(a: gsl_complex, x: libc::c_double) -> gsl_complex;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
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
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_set(
    mut v: *mut gsl_vector_complex,
    i: size_t,
    mut z: gsl_complex,
) {
    *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex) = z;
}
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
pub unsafe extern "C" fn gsl_schur_gen_eigvals(
    mut A: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut wr1: *mut libc::c_double,
    mut wr2: *mut libc::c_double,
    mut wi: *mut libc::c_double,
    mut scale1: *mut libc::c_double,
    mut scale2: *mut libc::c_double,
) -> libc::c_int {
    let safemin: libc::c_double = 2.2250738585072014e-308f64 * 1.0e2f64;
    let safemax: libc::c_double = 1.0f64 / safemin;
    let rtmin: libc::c_double = sqrt(safemin);
    let rtmax: libc::c_double = 1.0f64 / rtmin;
    let mut anorm: libc::c_double = 0.;
    let mut bnorm: libc::c_double = 0.;
    let mut ascale: libc::c_double = 0.;
    let mut bscale: libc::c_double = 0.;
    let mut bsize: libc::c_double = 0.;
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    let mut A11: libc::c_double = 0.;
    let mut A12: libc::c_double = 0.;
    let mut A21: libc::c_double = 0.;
    let mut A22: libc::c_double = 0.;
    let mut B11: libc::c_double = 0.;
    let mut B12: libc::c_double = 0.;
    let mut B22: libc::c_double = 0.;
    let mut binv11: libc::c_double = 0.;
    let mut binv22: libc::c_double = 0.;
    let mut bmin: libc::c_double = 0.;
    let mut as11: libc::c_double = 0.;
    let mut as12: libc::c_double = 0.;
    let mut as22: libc::c_double = 0.;
    let mut abi22: libc::c_double = 0.;
    let mut pp: libc::c_double = 0.;
    let mut qq: libc::c_double = 0.;
    let mut shift: libc::c_double = 0.;
    let mut ss: libc::c_double = 0.;
    let mut discr: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    anorm = if (if fabs(
        gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
    ) + fabs(gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t))
        > fabs(gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t))
            + fabs(
                gsl_matrix_get(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t),
            )
    {
        fabs(gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t))
            + fabs(
                gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t),
            )
    } else {
        fabs(gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t))
            + fabs(
                gsl_matrix_get(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t),
            )
    }) > safemin
    {
        if fabs(
            gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
        )
            + fabs(
                gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t),
            )
            > fabs(
                gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t),
            )
                + fabs(
                    gsl_matrix_get(
                        A,
                        1 as libc::c_int as size_t,
                        1 as libc::c_int as size_t,
                    ),
                )
        {
            fabs(
                gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t),
            )
                + fabs(
                    gsl_matrix_get(
                        A,
                        1 as libc::c_int as size_t,
                        0 as libc::c_int as size_t,
                    ),
                )
        } else {
            fabs(
                gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t),
            )
                + fabs(
                    gsl_matrix_get(
                        A,
                        1 as libc::c_int as size_t,
                        1 as libc::c_int as size_t,
                    ),
                )
        }
    } else {
        safemin
    };
    ascale = 1.0f64 / anorm;
    A11 = ascale
        * gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    A12 = ascale
        * gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    A21 = ascale
        * gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    A22 = ascale
        * gsl_matrix_get(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    B11 = gsl_matrix_get(B, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t);
    B12 = gsl_matrix_get(B, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    B22 = gsl_matrix_get(B, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
    bmin = rtmin
        * (if fabs(B11)
            > (if fabs(B12) > (if fabs(B22) > rtmin { fabs(B22) } else { rtmin }) {
                fabs(B12)
            } else {
                (if fabs(B22) > rtmin { fabs(B22) } else { rtmin })
            })
        {
            fabs(B11)
        } else {
            (if fabs(B12) > (if fabs(B22) > rtmin { fabs(B22) } else { rtmin }) {
                fabs(B12)
            } else {
                (if fabs(B22) > rtmin { fabs(B22) } else { rtmin })
            })
        });
    if fabs(B11) < bmin {
        B11 = (if B11 >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
            as libc::c_double * bmin;
    }
    if fabs(B22) < bmin {
        B22 = (if B22 >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
            as libc::c_double * bmin;
    }
    bnorm = if fabs(B11)
        > (if fabs(B12) + fabs(B22) > safemin { fabs(B12) + fabs(B22) } else { safemin })
    {
        fabs(B11)
    } else if fabs(B12) + fabs(B22) > safemin {
        fabs(B12) + fabs(B22)
    } else {
        safemin
    };
    bsize = if fabs(B11) > fabs(B22) { fabs(B11) } else { fabs(B22) };
    bscale = 1.0f64 / bsize;
    B11 *= bscale;
    B12 *= bscale;
    B22 *= bscale;
    binv11 = 1.0f64 / B11;
    binv22 = 1.0f64 / B22;
    s1 = A11 * binv11;
    s2 = A22 * binv22;
    if fabs(s1) <= fabs(s2) {
        as12 = A12 - s1 * B12;
        as22 = A22 - s1 * B22;
        ss = A21 * (binv11 * binv22);
        abi22 = as22 * binv22 - ss * B12;
        pp = 0.5f64 * abi22;
        shift = s1;
    } else {
        as12 = A12 - s2 * B12;
        as11 = A11 - s2 * B11;
        ss = A21 * (binv11 * binv22);
        abi22 = -ss * B12;
        pp = 0.5f64 * (as11 * binv11 + abi22);
        shift = s2;
    }
    qq = ss * as12;
    if fabs(pp * rtmin) >= 1.0f64 {
        discr = rtmin * pp * (rtmin * pp) + qq * safemin;
        r = sqrt(fabs(discr)) * rtmax;
    } else if pp * pp + fabs(qq) <= safemin {
        discr = rtmax * pp * (rtmax * pp) + qq * safemax;
        r = sqrt(fabs(discr)) * rtmin;
    } else {
        discr = pp * pp + qq;
        r = sqrt(fabs(discr));
    }
    if discr >= 0.0f64 || r == 0.0f64 {
        let mut sum: libc::c_double = pp
            + (if pp >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                as libc::c_double * r;
        let mut diff: libc::c_double = pp
            - (if pp >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                as libc::c_double * r;
        let mut wbig: libc::c_double = shift + sum;
        let mut wsmall: libc::c_double = shift + diff;
        if 0.5f64 * fabs(wbig)
            > (if fabs(wsmall) > safemin { fabs(wsmall) } else { safemin })
        {
            let mut wdet: libc::c_double = (A11 * A22 - A12 * A21) * (binv11 * binv22);
            wsmall = wdet / wbig;
        }
        if pp > abi22 {
            *wr1 = if wbig < wsmall { wbig } else { wsmall };
            *wr2 = if wbig > wsmall { wbig } else { wsmall };
        } else {
            *wr1 = if wbig > wsmall { wbig } else { wsmall };
            *wr2 = if wbig < wsmall { wbig } else { wsmall };
        }
        *wi = 0.0f64;
    } else {
        *wr1 = shift + pp;
        *wr2 = *wr1;
        *wi = r;
    }
    let fuzzy1: libc::c_double = 1.0f64 + 1.0e-5f64;
    let mut c1: libc::c_double = 0.;
    let mut c2: libc::c_double = 0.;
    let mut c3: libc::c_double = 0.;
    let mut c4: libc::c_double = 0.;
    let mut c5: libc::c_double = 0.;
    let mut wabs: libc::c_double = 0.;
    let mut wsize: libc::c_double = 0.;
    let mut wscale: libc::c_double = 0.;
    c1 = bsize * (safemin * (if 1.0f64 > ascale { 1.0f64 } else { ascale }));
    c2 = safemin * (if 1.0f64 > bnorm { 1.0f64 } else { bnorm });
    c3 = bsize * safemin;
    if ascale <= 1.0f64 && bsize <= 1.0f64 {
        c4 = if 1.0f64 < ascale / safemin * bsize {
            1.0f64
        } else {
            ascale / safemin * bsize
        };
    } else {
        c4 = 1.0f64;
    }
    if ascale <= 1.0f64 || bsize <= 1.0f64 {
        c5 = if 1.0f64 < ascale * bsize { 1.0f64 } else { ascale * bsize };
    } else {
        c5 = 1.0f64;
    }
    wabs = fabs(*wr1) + fabs(*wi);
    wsize = if safemin
        > (if c1
            > (if fuzzy1 * (wabs * c2 + c3)
                > (if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
                    c4
                } else {
                    0.5f64 * (if wabs > c5 { wabs } else { c5 })
                })
            {
                fuzzy1 * (wabs * c2 + c3)
            } else {
                (if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
                    c4
                } else {
                    0.5f64 * (if wabs > c5 { wabs } else { c5 })
                })
            })
        {
            c1
        } else {
            (if fuzzy1 * (wabs * c2 + c3)
                > (if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
                    c4
                } else {
                    0.5f64 * (if wabs > c5 { wabs } else { c5 })
                })
            {
                fuzzy1 * (wabs * c2 + c3)
            } else {
                (if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
                    c4
                } else {
                    0.5f64 * (if wabs > c5 { wabs } else { c5 })
                })
            })
        })
    {
        safemin
    } else if c1
        > (if fuzzy1 * (wabs * c2 + c3)
            > (if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
                c4
            } else {
                0.5f64 * (if wabs > c5 { wabs } else { c5 })
            })
        {
            fuzzy1 * (wabs * c2 + c3)
        } else {
            (if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
                c4
            } else {
                0.5f64 * (if wabs > c5 { wabs } else { c5 })
            })
        })
    {
        c1
    } else if fuzzy1 * (wabs * c2 + c3)
        > (if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
            c4
        } else {
            0.5f64 * (if wabs > c5 { wabs } else { c5 })
        })
    {
        fuzzy1 * (wabs * c2 + c3)
    } else if c4 < 0.5f64 * (if wabs > c5 { wabs } else { c5 }) {
        c4
    } else {
        0.5f64 * (if wabs > c5 { wabs } else { c5 })
    };
    if wsize != 1.0f64 {
        wscale = 1.0f64 / wsize;
        if wsize > 1.0f64 {
            *scale1 = (if ascale > bsize { ascale } else { bsize }) * wscale
                * (if ascale < bsize { ascale } else { bsize });
        } else {
            *scale1 = (if ascale < bsize { ascale } else { bsize }) * wscale
                * (if ascale > bsize { ascale } else { bsize });
        }
        *wr1 *= wscale;
        if *wi != 0.0f64 {
            *wi *= wscale;
            *wr2 = *wr1;
            *scale2 = *scale1;
        }
    } else {
        *scale1 = ascale * bsize;
        *scale2 = *scale1;
    }
    if *wi == 0.0f64 {
        wsize = if safemin
            > (if c1
                > (if fuzzy1 * (fabs(*wr2) * c2 + c3)
                    > (if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
                        c4
                    } else {
                        0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
                    })
                {
                    fuzzy1 * (fabs(*wr2) * c2 + c3)
                } else {
                    (if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
                        c4
                    } else {
                        0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
                    })
                })
            {
                c1
            } else {
                (if fuzzy1 * (fabs(*wr2) * c2 + c3)
                    > (if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
                        c4
                    } else {
                        0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
                    })
                {
                    fuzzy1 * (fabs(*wr2) * c2 + c3)
                } else {
                    (if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
                        c4
                    } else {
                        0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
                    })
                })
            })
        {
            safemin
        } else if c1
            > (if fuzzy1 * (fabs(*wr2) * c2 + c3)
                > (if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
                    c4
                } else {
                    0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
                })
            {
                fuzzy1 * (fabs(*wr2) * c2 + c3)
            } else {
                (if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
                    c4
                } else {
                    0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
                })
            })
        {
            c1
        } else if fuzzy1 * (fabs(*wr2) * c2 + c3)
            > (if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
                c4
            } else {
                0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
            })
        {
            fuzzy1 * (fabs(*wr2) * c2 + c3)
        } else if c4 < 0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 }) {
            c4
        } else {
            0.5f64 * (if fabs(*wr2) > c5 { fabs(*wr2) } else { c5 })
        };
        if wsize != 1.0f64 {
            wscale = 1.0f64 / wsize;
            if wsize > 1.0f64 {
                *scale2 = (if ascale > bsize { ascale } else { bsize }) * wscale
                    * (if ascale < bsize { ascale } else { bsize });
            } else {
                *scale2 = (if ascale < bsize { ascale } else { bsize }) * wscale
                    * (if ascale > bsize { ascale } else { bsize });
            }
            *wr2 *= wscale;
        } else {
            *scale2 = ascale * bsize;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_schur_solve_equation(
    mut ca: libc::c_double,
    mut A: *const gsl_matrix,
    mut z: libc::c_double,
    mut d1: libc::c_double,
    mut d2: libc::c_double,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut s: *mut libc::c_double,
    mut xnorm: *mut libc::c_double,
    mut smin: libc::c_double,
) -> libc::c_int {
    let mut N: size_t = (*A).size1;
    let mut bnorm: libc::c_double = 0.;
    let mut scale: libc::c_double = 1.0f64;
    if N == 1 as libc::c_int as libc::c_ulong {
        let mut c: libc::c_double = 0.;
        let mut cnorm: libc::c_double = 0.;
        c = ca
            * gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t)
            - z * d1;
        cnorm = fabs(c);
        if cnorm < smin {
            c = smin;
            cnorm = smin;
        }
        bnorm = fabs(gsl_vector_get(b, 0 as libc::c_int as size_t));
        if cnorm < 1.0f64 && bnorm > 1.0f64 {
            if bnorm
                > (1.0f64 - 2.2204460492503131e-16f64)
                    / (2.0f64 * 2.2250738585072014e-308f64) * cnorm
            {
                scale = 1.0f64 / bnorm;
            }
        }
        gsl_vector_set(
            x,
            0 as libc::c_int as size_t,
            gsl_vector_get(b, 0 as libc::c_int as size_t) * scale / c,
        );
        *xnorm = fabs(gsl_vector_get(x, 0 as libc::c_int as size_t));
    } else {
        let mut cr: [[libc::c_double; 2]; 2] = [[0.; 2]; 2];
        let mut crv: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut cmax: libc::c_double = 0.;
        let mut icmax: size_t = 0;
        let mut j: size_t = 0;
        let mut bval1: libc::c_double = 0.;
        let mut bval2: libc::c_double = 0.;
        let mut ur11: libc::c_double = 0.;
        let mut ur12: libc::c_double = 0.;
        let mut ur22: libc::c_double = 0.;
        let mut ur11r: libc::c_double = 0.;
        let mut cr21: libc::c_double = 0.;
        let mut cr22: libc::c_double = 0.;
        let mut lr21: libc::c_double = 0.;
        let mut b1: libc::c_double = 0.;
        let mut b2: libc::c_double = 0.;
        let mut bbnd: libc::c_double = 0.;
        let mut x1: libc::c_double = 0.;
        let mut x2: libc::c_double = 0.;
        let mut temp: libc::c_double = 0.;
        let mut ipivot: [[size_t; 4]; 4] = [
            [
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                3 as libc::c_int as size_t,
            ],
            [
                1 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                3 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            ],
            [
                2 as libc::c_int as size_t,
                3 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
            ],
            [
                3 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
            ],
        ];
        let mut rswap: [libc::c_int; 4] = [
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        ];
        let mut zswap: [libc::c_int; 4] = [
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        ];
        crv = cr.as_mut_ptr() as *mut libc::c_double;
        cr[0 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t)
            - z * d1;
        cr[1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
            - z * d2;
        cr[0 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t);
        cr[1 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t);
        cmax = 0.0f64;
        icmax = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while j < 4 as libc::c_int as libc::c_ulong {
            if fabs(*crv.offset(j as isize)) > cmax {
                cmax = fabs(*crv.offset(j as isize));
                icmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        bval1 = gsl_vector_get(b, 0 as libc::c_int as size_t);
        bval2 = gsl_vector_get(b, 1 as libc::c_int as size_t);
        if cmax < smin {
            bnorm = if fabs(bval1) > fabs(bval2) { fabs(bval1) } else { fabs(bval2) };
            if smin < 1.0f64 && bnorm > 1.0f64 {
                if bnorm
                    > (1.0f64 - 2.2204460492503131e-16f64)
                        / (2.0f64 * 2.2250738585072014e-308f64) * smin
                {
                    scale = 1.0f64 / bnorm;
                }
            }
            temp = scale / smin;
            gsl_vector_set(x, 0 as libc::c_int as size_t, temp * bval1);
            gsl_vector_set(x, 1 as libc::c_int as size_t, temp * bval2);
            *xnorm = temp * bnorm;
            *s = scale;
            return GSL_SUCCESS as libc::c_int;
        }
        ur11 = *crv.offset(icmax as isize);
        cr21 = *crv.offset(ipivot[1 as libc::c_int as usize][icmax as usize] as isize);
        ur12 = *crv.offset(ipivot[2 as libc::c_int as usize][icmax as usize] as isize);
        cr22 = *crv.offset(ipivot[3 as libc::c_int as usize][icmax as usize] as isize);
        ur11r = 1.0f64 / ur11;
        lr21 = ur11r * cr21;
        ur22 = cr22 - ur12 * lr21;
        if fabs(ur22) < smin {
            ur22 = smin;
        }
        if rswap[icmax as usize] != 0 {
            b1 = bval2;
            b2 = bval1;
        } else {
            b1 = bval1;
            b2 = bval2;
        }
        b2 -= lr21 * b1;
        bbnd = if fabs(b1 * (ur22 * ur11r)) > fabs(b2) {
            fabs(b1 * (ur22 * ur11r))
        } else {
            fabs(b2)
        };
        if bbnd > 1.0f64 && fabs(ur22) < 1.0f64 {
            if bbnd
                >= (1.0f64 - 2.2204460492503131e-16f64)
                    / (2.0f64 * 2.2250738585072014e-308f64) * fabs(ur22)
            {
                scale = 1.0f64 / bbnd;
            }
        }
        x2 = b2 * scale / ur22;
        x1 = scale * b1 * ur11r - x2 * (ur11r * ur12);
        if zswap[icmax as usize] != 0 {
            gsl_vector_set(x, 0 as libc::c_int as size_t, x2);
            gsl_vector_set(x, 1 as libc::c_int as size_t, x1);
        } else {
            gsl_vector_set(x, 0 as libc::c_int as size_t, x1);
            gsl_vector_set(x, 1 as libc::c_int as size_t, x2);
        }
        *xnorm = if fabs(x1) > fabs(x2) { fabs(x1) } else { fabs(x2) };
        if *xnorm > 1.0f64 && cmax > 1.0f64 {
            if *xnorm
                > (1.0f64 - 2.2204460492503131e-16f64)
                    / (2.0f64 * 2.2250738585072014e-308f64) / cmax
            {
                temp = cmax
                    / ((1.0f64 - 2.2204460492503131e-16f64)
                        / (2.0f64 * 2.2250738585072014e-308f64));
                gsl_blas_dscal(temp, x);
                *xnorm *= temp;
                scale *= temp;
            }
        }
    }
    *s = scale;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_schur_solve_equation_z(
    mut ca: libc::c_double,
    mut A: *const gsl_matrix,
    mut z: *mut gsl_complex,
    mut d1: libc::c_double,
    mut d2: libc::c_double,
    mut b: *const gsl_vector_complex,
    mut x: *mut gsl_vector_complex,
    mut s: *mut libc::c_double,
    mut xnorm: *mut libc::c_double,
    mut smin: libc::c_double,
) -> libc::c_int {
    let mut N: size_t = (*A).size1;
    let mut scale: libc::c_double = 1.0f64;
    let mut bnorm: libc::c_double = 0.;
    if N == 1 as libc::c_int as libc::c_ulong {
        let mut cr: libc::c_double = 0.;
        let mut ci: libc::c_double = 0.;
        let mut cnorm: libc::c_double = 0.;
        let mut bval: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut c: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut xval: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut tmp: gsl_complex = gsl_complex { dat: [0.; 2] };
        cr = ca
            * gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t)
            - (*z).dat[0 as libc::c_int as usize] * d1;
        ci = -(*z).dat[1 as libc::c_int as usize] * d1;
        cnorm = fabs(cr) + fabs(ci);
        if cnorm < smin {
            cr = smin;
            ci = 0.0f64;
            cnorm = smin;
        }
        bval = gsl_vector_complex_get(b, 0 as libc::c_int as size_t);
        bnorm = fabs(bval.dat[0 as libc::c_int as usize])
            + fabs(bval.dat[1 as libc::c_int as usize]);
        if cnorm < 1.0f64 && bnorm > 1.0f64 {
            if bnorm
                > (1.0f64 - 2.2204460492503131e-16f64)
                    / (2.0f64 * 2.2250738585072014e-308f64) * cnorm
            {
                scale = 1.0f64 / bnorm;
            }
        }
        tmp.dat[0 as libc::c_int as usize] = scale * bval.dat[0 as libc::c_int as usize];
        tmp.dat[1 as libc::c_int as usize] = scale * bval.dat[1 as libc::c_int as usize];
        c.dat[0 as libc::c_int as usize] = cr;
        c.dat[1 as libc::c_int as usize] = ci;
        xval = gsl_complex_div(tmp, c);
        gsl_vector_complex_set(x, 0 as libc::c_int as size_t, xval);
        *xnorm = fabs(xval.dat[0 as libc::c_int as usize])
            + fabs(xval.dat[1 as libc::c_int as usize]);
    } else {
        let mut cr_0: [[libc::c_double; 2]; 2] = [[0.; 2]; 2];
        let mut ci_0: [[libc::c_double; 2]; 2] = [[0.; 2]; 2];
        let mut civ: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut crv: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut cmax: libc::c_double = 0.;
        let mut bval1: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut bval2: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut xval1: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut xval2: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut xr1: libc::c_double = 0.;
        let mut xi1: libc::c_double = 0.;
        let mut icmax: size_t = 0;
        let mut j: size_t = 0;
        let mut temp: libc::c_double = 0.;
        let mut ur11: libc::c_double = 0.;
        let mut ur12: libc::c_double = 0.;
        let mut ur22: libc::c_double = 0.;
        let mut ui11: libc::c_double = 0.;
        let mut ui12: libc::c_double = 0.;
        let mut ui22: libc::c_double = 0.;
        let mut ur11r: libc::c_double = 0.;
        let mut ui11r: libc::c_double = 0.;
        let mut ur12s: libc::c_double = 0.;
        let mut ui12s: libc::c_double = 0.;
        let mut u22abs: libc::c_double = 0.;
        let mut lr21: libc::c_double = 0.;
        let mut li21: libc::c_double = 0.;
        let mut cr21: libc::c_double = 0.;
        let mut cr22: libc::c_double = 0.;
        let mut ci21: libc::c_double = 0.;
        let mut ci22: libc::c_double = 0.;
        let mut br1: libc::c_double = 0.;
        let mut bi1: libc::c_double = 0.;
        let mut br2: libc::c_double = 0.;
        let mut bi2: libc::c_double = 0.;
        let mut bbnd: libc::c_double = 0.;
        let mut b1: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut b2: gsl_complex = gsl_complex { dat: [0.; 2] };
        let mut ipivot: [[size_t; 4]; 4] = [
            [
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                3 as libc::c_int as size_t,
            ],
            [
                1 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                3 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
            ],
            [
                2 as libc::c_int as size_t,
                3 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
            ],
            [
                3 as libc::c_int as size_t,
                2 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
            ],
        ];
        let mut rswap: [libc::c_int; 4] = [
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        ];
        let mut zswap: [libc::c_int; 4] = [
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        ];
        civ = ci_0.as_mut_ptr() as *mut libc::c_double;
        crv = cr_0.as_mut_ptr() as *mut libc::c_double;
        cr_0[0 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 0 as libc::c_int as size_t, 0 as libc::c_int as size_t)
            - (*z).dat[0 as libc::c_int as usize] * d1;
        cr_0[1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
            - (*z).dat[0 as libc::c_int as usize] * d2;
        cr_0[0 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 1 as libc::c_int as size_t, 0 as libc::c_int as size_t);
        cr_0[1 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = ca
            * gsl_matrix_get(A, 0 as libc::c_int as size_t, 1 as libc::c_int as size_t);
        ci_0[0 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = -(*z).dat[1 as libc::c_int as usize] * d1;
        ci_0[0 as libc::c_int as usize][1 as libc::c_int as usize] = 0.0f64;
        ci_0[1 as libc::c_int as usize][0 as libc::c_int as usize] = 0.0f64;
        ci_0[1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = -(*z).dat[1 as libc::c_int as usize] * d2;
        cmax = 0.0f64;
        icmax = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while j < 4 as libc::c_int as libc::c_ulong {
            if fabs(*crv.offset(j as isize)) + fabs(*civ.offset(j as isize)) > cmax {
                cmax = fabs(*crv.offset(j as isize)) + fabs(*civ.offset(j as isize));
                icmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        bval1 = gsl_vector_complex_get(b, 0 as libc::c_int as size_t);
        bval2 = gsl_vector_complex_get(b, 1 as libc::c_int as size_t);
        if cmax < smin {
            bnorm = if fabs(bval1.dat[0 as libc::c_int as usize])
                + fabs(bval1.dat[1 as libc::c_int as usize])
                > fabs(bval2.dat[0 as libc::c_int as usize])
                    + fabs(bval2.dat[1 as libc::c_int as usize])
            {
                fabs(bval1.dat[0 as libc::c_int as usize])
                    + fabs(bval1.dat[1 as libc::c_int as usize])
            } else {
                fabs(bval2.dat[0 as libc::c_int as usize])
                    + fabs(bval2.dat[1 as libc::c_int as usize])
            };
            if smin < 1.0f64 && bnorm > 1.0f64 {
                if bnorm
                    > (1.0f64 - 2.2204460492503131e-16f64)
                        / (2.0f64 * 2.2250738585072014e-308f64) * smin
                {
                    scale = 1.0f64 / bnorm;
                }
            }
            temp = scale / smin;
            xval1 = gsl_complex_mul_real(bval1, temp);
            xval2 = gsl_complex_mul_real(bval2, temp);
            gsl_vector_complex_set(x, 0 as libc::c_int as size_t, xval1);
            gsl_vector_complex_set(x, 1 as libc::c_int as size_t, xval2);
            *xnorm = temp * bnorm;
            *s = scale;
            return GSL_SUCCESS as libc::c_int;
        }
        ur11 = *crv.offset(icmax as isize);
        ui11 = *civ.offset(icmax as isize);
        cr21 = *crv.offset(ipivot[1 as libc::c_int as usize][icmax as usize] as isize);
        ci21 = *civ.offset(ipivot[1 as libc::c_int as usize][icmax as usize] as isize);
        ur12 = *crv.offset(ipivot[2 as libc::c_int as usize][icmax as usize] as isize);
        ui12 = *civ.offset(ipivot[2 as libc::c_int as usize][icmax as usize] as isize);
        cr22 = *crv.offset(ipivot[3 as libc::c_int as usize][icmax as usize] as isize);
        ci22 = *civ.offset(ipivot[3 as libc::c_int as usize][icmax as usize] as isize);
        if icmax == 0 as libc::c_int as libc::c_ulong
            || icmax == 3 as libc::c_int as libc::c_ulong
        {
            if fabs(ur11) > fabs(ui11) {
                temp = ui11 / ur11;
                ur11r = 1.0f64 / (ur11 * (1.0f64 + temp * temp));
                ui11r = -temp * ur11r;
            } else {
                temp = ur11 / ui11;
                ui11r = -1.0f64 / (ui11 * (1.0f64 + temp * temp));
                ur11r = -temp * ui11r;
            }
            lr21 = cr21 * ur11r;
            li21 = cr21 * ui11r;
            ur12s = ur12 * ur11r;
            ui12s = ur12 * ui11r;
            ur22 = cr22 - ur12 * lr21;
            ui22 = ci22 - ur12 * li21;
        } else {
            ur11r = 1.0f64 / ur11;
            ui11r = 0.0f64;
            lr21 = cr21 * ur11r;
            li21 = ci21 * ur11r;
            ur12s = ur12 * ur11r;
            ui12s = ui12 * ur11r;
            ur22 = cr22 - ur12 * lr21 + ui12 * li21;
            ui22 = -ur12 * li21 - ui12 * lr21;
        }
        u22abs = fabs(ur22) + fabs(ui22);
        if u22abs < smin {
            ur22 = smin;
            ui22 = 0.0f64;
        }
        if rswap[icmax as usize] != 0 {
            br2 = bval1.dat[0 as libc::c_int as usize];
            bi2 = bval1.dat[1 as libc::c_int as usize];
            br1 = bval2.dat[0 as libc::c_int as usize];
            bi1 = bval2.dat[1 as libc::c_int as usize];
        } else {
            br1 = bval1.dat[0 as libc::c_int as usize];
            bi1 = bval1.dat[1 as libc::c_int as usize];
            br2 = bval2.dat[0 as libc::c_int as usize];
            bi2 = bval2.dat[1 as libc::c_int as usize];
        }
        br2 += li21 * bi1 - lr21 * br1;
        bi2 -= li21 * br1 + lr21 * bi1;
        bbnd = if (fabs(br1) + fabs(bi1)) * (u22abs * (fabs(ur11r) + fabs(ui11r)))
            > fabs(br2) + fabs(bi2)
        {
            (fabs(br1) + fabs(bi1)) * (u22abs * (fabs(ur11r) + fabs(ui11r)))
        } else {
            fabs(br2) + fabs(bi2)
        };
        if bbnd > 1.0f64 && u22abs < 1.0f64 {
            if bbnd
                >= (1.0f64 - 2.2204460492503131e-16f64)
                    / (2.0f64 * 2.2250738585072014e-308f64) * u22abs
            {
                scale = 1.0f64 / bbnd;
                br1 *= scale;
                bi1 *= scale;
                br2 *= scale;
                bi2 *= scale;
            }
        }
        b1.dat[0 as libc::c_int as usize] = br2;
        b1.dat[1 as libc::c_int as usize] = bi2;
        b2.dat[0 as libc::c_int as usize] = ur22;
        b2.dat[1 as libc::c_int as usize] = ui22;
        xval2 = gsl_complex_div(b1, b2);
        xr1 = ur11r * br1 - ui11r * bi1 - ur12s * xval2.dat[0 as libc::c_int as usize]
            + ui12s * xval2.dat[1 as libc::c_int as usize];
        xi1 = ui11r * br1 + ur11r * bi1 - ui12s * xval2.dat[0 as libc::c_int as usize]
            - ur12s * xval2.dat[1 as libc::c_int as usize];
        xval1.dat[0 as libc::c_int as usize] = xr1;
        xval1.dat[1 as libc::c_int as usize] = xi1;
        if zswap[icmax as usize] != 0 {
            gsl_vector_complex_set(x, 0 as libc::c_int as size_t, xval2);
            gsl_vector_complex_set(x, 1 as libc::c_int as size_t, xval1);
        } else {
            gsl_vector_complex_set(x, 0 as libc::c_int as size_t, xval1);
            gsl_vector_complex_set(x, 1 as libc::c_int as size_t, xval2);
        }
        *xnorm = if fabs(xval1.dat[0 as libc::c_int as usize])
            + fabs(xval1.dat[1 as libc::c_int as usize])
            > fabs(xval2.dat[0 as libc::c_int as usize])
                + fabs(xval2.dat[1 as libc::c_int as usize])
        {
            fabs(xval1.dat[0 as libc::c_int as usize])
                + fabs(xval1.dat[1 as libc::c_int as usize])
        } else {
            fabs(xval2.dat[0 as libc::c_int as usize])
                + fabs(xval2.dat[1 as libc::c_int as usize])
        };
        if *xnorm > 1.0f64 && cmax > 1.0f64 {
            if *xnorm
                > (1.0f64 - 2.2204460492503131e-16f64)
                    / (2.0f64 * 2.2250738585072014e-308f64) / cmax
            {
                temp = cmax
                    / ((1.0f64 - 2.2204460492503131e-16f64)
                        / (2.0f64 * 2.2250738585072014e-308f64));
                gsl_blas_zdscal(temp, x);
                *xnorm *= temp;
                scale *= temp;
            }
        }
    }
    *s = scale;
    return GSL_SUCCESS as libc::c_int;
}
