use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type PDOUBLE = *mut libc::c_double;
pub unsafe extern "C" fn LogTransformArray(
    mut nElems: libc::c_long,
    mut rgdSrc: *mut libc::c_double,
    mut rgdDes: *mut libc::c_double,
) -> *mut libc::c_double {
    let mut l: libc::c_long = 0;
    l = 0 as libc::c_int as libc::c_long;
    while l < nElems {
        *rgdDes.offset(l as isize) = log(*rgdSrc.offset(l as isize));
        l += 1;
        l;
    }
    return rgdDes;
}
pub unsafe extern "C" fn InitdVector(mut cVectors: libc::c_long) -> *mut libc::c_double {
    if cVectors == 0 as libc::c_int as libc::c_long {
        printf(
            b"Error: zero length array allocation in InitdVector - Exiting\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    } else {
        return malloc(
            (cVectors as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    };
}
pub unsafe extern "C" fn InitiVector(mut cVectors: libc::c_long) -> *mut libc::c_int {
    if cVectors == 0 as libc::c_int as libc::c_long {
        printf(
            b"Error: zero length array allocation in InitiVector - Exiting\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    } else {
        return malloc(
            (cVectors as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int
    };
}
pub unsafe extern "C" fn InitlVector(mut cVectors: libc::c_long) -> *mut libc::c_long {
    if cVectors == 0 as libc::c_int as libc::c_long {
        printf(
            b"Error: zero length array allocation in InitlVector - Exiting\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    } else {
        return malloc(
            (cVectors as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        ) as *mut libc::c_long
    };
}
pub unsafe extern "C" fn InitpdVector(
    mut cVectors: libc::c_long,
) -> *mut *mut libc::c_double {
    if cVectors == 0 as libc::c_int as libc::c_long {
        printf(
            b"Error: zero length array allocation in InitpdVector - Exiting\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    } else {
        return malloc(
            (cVectors as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_double
    };
}
pub unsafe extern "C" fn InitdMatrix(
    mut cVectors: libc::c_long,
    mut cElemsEach: libc::c_long,
) -> *mut *mut libc::c_double {
    let mut i: libc::c_long = 0;
    let mut rgp: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    if cVectors == 0 as libc::c_int as libc::c_long
        || cElemsEach == 0 as libc::c_int as libc::c_long
    {
        printf(
            b"Error: zero length array allocation in InitdMatrix - Exiting\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    rgp = malloc(
        (cVectors as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong),
    ) as *mut *mut libc::c_double;
    if !rgp.is_null() {
        i = 0 as libc::c_int as libc::c_long;
        while i < cVectors {
            let ref mut fresh0 = *rgp.offset(i as isize);
            *fresh0 = malloc(
                (cElemsEach as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_double;
            if (*rgp.offset(i as isize)).is_null() {
                rgp = 0 as *mut *mut libc::c_double;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    return rgp;
}
pub unsafe extern "C" fn InitlMatrix(
    mut cVectors: libc::c_long,
    mut cElemsEach: libc::c_long,
) -> *mut *mut libc::c_long {
    let mut i: libc::c_long = 0;
    let mut rgp: *mut *mut libc::c_long = 0 as *mut *mut libc::c_long;
    if cVectors == 0 as libc::c_int as libc::c_long
        || cElemsEach == 0 as libc::c_int as libc::c_long
    {
        printf(
            b"Error: zero length array allocation in InitlMatrix - Exiting\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    rgp = malloc(
        (cVectors as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_long>() as libc::c_ulong),
    ) as *mut *mut libc::c_long;
    if !rgp.is_null() {
        i = 0 as libc::c_int as libc::c_long;
        while i < cVectors {
            let ref mut fresh1 = *rgp.offset(i as isize);
            *fresh1 = malloc(
                (cElemsEach as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
            ) as *mut libc::c_long;
            if (*rgp.offset(i as isize)).is_null() {
                rgp = 0 as *mut *mut libc::c_long;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    return rgp;
}
pub unsafe extern "C" fn ColumnMeans(
    mut cRows: libc::c_long,
    mut cCols: libc::c_long,
    mut x: *mut *mut libc::c_double,
    mut x_bar: *mut libc::c_double,
) {
    let mut i: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    l = 0 as libc::c_int as libc::c_long;
    while l < cCols {
        *x_bar.offset(l as isize) = 0.0f64;
        l += 1;
        l;
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < cRows {
        l = 0 as libc::c_int as libc::c_long;
        while l < cCols {
            *x_bar.offset(l as isize) += *(*x.offset(i as isize)).offset(l as isize);
            l += 1;
            l;
        }
        i += 1;
        i;
    }
    l = 0 as libc::c_int as libc::c_long;
    while l < cCols {
        *x_bar.offset(l as isize) /= cRows as libc::c_double;
        l += 1;
        l;
    }
}
pub unsafe extern "C" fn Cholesky(
    mut prgdVariance: *mut PDOUBLE,
    mut prgdComponent: *mut PDOUBLE,
    mut lNparams: libc::c_long,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dSum: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < lNparams {
        j = 0 as libc::c_int;
        while (j as libc::c_long) < lNparams {
            *(*prgdComponent.offset(i as isize)).offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_long) < lNparams {
        j = i;
        while (j as libc::c_long) < lNparams {
            dSum = *(*prgdVariance.offset(i as isize)).offset(j as isize);
            k = i - 1 as libc::c_int;
            while k >= 0 as libc::c_int {
                dSum = dSum
                    - *(*prgdVariance.offset(i as isize)).offset(k as isize)
                        * *(*prgdVariance.offset(j as isize)).offset(k as isize);
                k -= 1;
                k;
            }
            if i == j {
                if dSum <= 0.0f64 {
                    printf(
                        b"Warning: input matrix for Cholesky is not positive definite\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                } else {
                    *(*prgdComponent.offset(i as isize)).offset(i as isize) = sqrt(dSum);
                }
            } else {
                *(*prgdVariance.offset(j as isize))
                    .offset(
                        i as isize,
                    ) = dSum / *(*prgdComponent.offset(i as isize)).offset(i as isize);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_long) < lNparams {
        j = i + 1 as libc::c_int;
        while (j as libc::c_long) < lNparams {
            *(*prgdComponent.offset(j as isize))
                .offset(
                    i as isize,
                ) = *(*prgdVariance.offset(j as isize)).offset(i as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
