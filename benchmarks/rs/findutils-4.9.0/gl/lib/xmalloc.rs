use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
    fn xalloc_die();
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub const DEFAULT_MXFAST_0: C2RustUnnamed_0 = 128;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn icalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    if (18446744073709551615 as libc::c_ulong) < n as libc::c_ulong {
        if s != 0 as libc::c_int as libc::c_long {
            return _gl_alloc_nomem();
        }
        n = 0 as libc::c_int as idx_t;
    }
    if (18446744073709551615 as libc::c_ulong) < s as libc::c_ulong {
        if n != 0 as libc::c_int as libc::c_long {
            return _gl_alloc_nomem();
        }
        s = 0 as libc::c_int as idx_t;
    }
    return calloc(n as libc::c_ulong, s as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn ireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    if n == 0 as libc::c_int as libc::c_long || s == 0 as libc::c_int as libc::c_long {
        s = 1 as libc::c_int as idx_t;
        n = s;
    }
    return if n as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
        && s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
    {
        reallocarray(p, n as size_t, s as size_t)
    } else {
        _gl_alloc_nomem()
    };
}
#[cold]
#[inline]
unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn imalloc(mut s: idx_t) -> *mut libc::c_void {
    return if s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong {
        malloc(s as libc::c_ulong)
    } else {
        _gl_alloc_nomem()
    };
}
#[inline]
unsafe extern "C" fn irealloc(
    mut p: *mut libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return if s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong {
        realloc(p, (s | (s == 0) as libc::c_int as libc::c_long) as libc::c_ulong)
    } else {
        _gl_alloc_nomem()
    };
}
unsafe extern "C" fn nonnull(mut p: *mut libc::c_void) -> *mut libc::c_void {
    if p.is_null() {
        xalloc_die();
    }
    return p;
}
pub unsafe extern "C" fn xmalloc(mut s: size_t) -> *mut libc::c_void {
    return nonnull(malloc(s));
}
pub unsafe extern "C" fn ximalloc(mut s: idx_t) -> *mut libc::c_void {
    return nonnull(imalloc(s));
}
pub unsafe extern "C" fn xcharalloc(mut n: size_t) -> *mut libc::c_char {
    return (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xmalloc(n)
    } else {
        xnmalloc(n, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn xrealloc(
    mut p: *mut libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = realloc(p, s);
    if r.is_null() && (p.is_null() || s != 0) {
        xalloc_die();
    }
    return r;
}
pub unsafe extern "C" fn xirealloc(
    mut p: *mut libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return nonnull(irealloc(p, s));
}
pub unsafe extern "C" fn xreallocarray(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = reallocarray(p, n, s);
    if r.is_null() && (p.is_null() || n != 0 && s != 0) {
        xalloc_die();
    }
    return r;
}
pub unsafe extern "C" fn xireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    return nonnull(ireallocarray(p, n, s));
}
pub unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    return xreallocarray(0 as *mut libc::c_void, n, s);
}
pub unsafe extern "C" fn xinmalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    return xireallocarray(0 as *mut libc::c_void, n, s);
}
pub unsafe extern "C" fn x2realloc(
    mut p: *mut libc::c_void,
    mut ps: *mut size_t,
) -> *mut libc::c_void {
    return x2nrealloc(p, ps, 1 as libc::c_int as size_t);
}
pub unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
    } else {
        let (fresh0, fresh1) = n
            .overflowing_add(
                (n >> 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        *(&mut n as *mut size_t) = fresh0;
        if fresh1 {
            xalloc_die();
        }
    }
    p = xreallocarray(p, n, s);
    *pn = n;
    return p;
}
pub unsafe extern "C" fn xpalloc(
    mut pa: *mut libc::c_void,
    mut pn: *mut idx_t,
    mut n_incr_min: idx_t,
    mut n_max: ptrdiff_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    let mut n0: idx_t = *pn;
    let mut n: idx_t = 0;
    let (fresh2, fresh3) = n0.overflowing_add(n0 >> 1 as libc::c_int);
    *(&mut n as *mut idx_t) = fresh2;
    if fresh3 {
        n = 9223372036854775807 as libc::c_long;
    }
    if 0 as libc::c_int as libc::c_long <= n_max && n_max < n {
        n = n_max;
    }
    let mut nbytes: idx_t = 0;
    let mut adjusted_nbytes: idx_t = (if if ::std::mem::size_of::<idx_t>()
        as libc::c_ulong == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t) {
            if if s < 0 as libc::c_int as libc::c_long {
                if n < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int
                        }) as libc::c_long + s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (n < 127 as libc::c_int as libc::c_long / s) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (s
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 127 as libc::c_int as libc::c_long
                                >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            127 as libc::c_int as libc::c_long / -s
                        }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        as libc::c_int
                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < n
                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < n
                            && ((-(1 as libc::c_int)
                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long / s < n)
                        as libc::c_int
                }
            } else if s == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if n < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        as libc::c_int
                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < s
                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int)
                            - (-(127 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long / n < s)
                        as libc::c_int
                }
            } else {
                (127 as libc::c_int as libc::c_long / s < n) as libc::c_int
            } != 0
            {
                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                    as libc::c_schar as idx_t;
                1 as libc::c_int
            } else {
                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                    as libc::c_schar as idx_t;
                0 as libc::c_int
            }
        } else if if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    }) as libc::c_long + s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (n
                        < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_long / s) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                as libc::c_long
                            >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_long / -s
                    }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < n
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
            }
        } else if s == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else if n < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && n == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                        < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
            }
        } else {
            ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
                / s < n) as libc::c_int
        } != 0
        {
            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint) as libc::c_uchar
                as idx_t;
            1 as libc::c_int
        } else {
            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint) as libc::c_uchar
                as idx_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<idx_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
    {
        if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t) {
            if if s < 0 as libc::c_int as libc::c_long {
                if n < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            32767 as libc::c_int
                        }) as libc::c_long + s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (n < 32767 as libc::c_int as libc::c_long / s) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (s
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 32767 as libc::c_int as libc::c_long
                                >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            32767 as libc::c_int as libc::c_long / -s
                        }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long) as libc::c_int
                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < n
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < n
                            && ((-(1 as libc::c_int)
                                - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long / s
                        < n) as libc::c_int
                }
            } else if s == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if n < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            })
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        })
                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long) as libc::c_int
                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < s
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int)
                            - (-(32767 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long / n
                        < s) as libc::c_int
                }
            } else {
                (32767 as libc::c_int as libc::c_long / s < n) as libc::c_int
            } != 0
            {
                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                    as libc::c_short as idx_t;
                1 as libc::c_int
            } else {
                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                    as libc::c_short as idx_t;
                0 as libc::c_int
            }
        } else if if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    }) as libc::c_long + s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (n
                        < (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_long / s) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (32767 as libc::c_int * 2 as libc::c_int
                                + 1 as libc::c_int) as libc::c_long
                            >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_long / -s
                    }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < n
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
            }
        } else if s == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else if n < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && n == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                        < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
            }
        } else {
            ((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
                / s < n) as libc::c_int
        } != 0
        {
            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                as libc::c_ushort as idx_t;
            1 as libc::c_int
        } else {
            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                as libc::c_ushort as idx_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<idx_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            nbytes
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            if if s < 0 as libc::c_int as libc::c_long {
                if n < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2147483647 as libc::c_int
                        }) as libc::c_long + s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (n < 2147483647 as libc::c_int as libc::c_long / s)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (s
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 2147483647 as libc::c_int as libc::c_long
                                >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            2147483647 as libc::c_int as libc::c_long / -s
                        }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    })
                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    })
                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long) as libc::c_int
                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < n
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < n
                            && ((-(1 as libc::c_int)
                                - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        / s < n) as libc::c_int
                }
            } else if s == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if n < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    })
                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    })
                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            })
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long) as libc::c_int
                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < s
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int)
                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        / n < s) as libc::c_int
                }
            } else {
                (2147483647 as libc::c_int as libc::c_long / s < n) as libc::c_int
            } != 0
            {
                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                    as libc::c_int as idx_t;
                1 as libc::c_int
            } else {
                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                    as libc::c_int as idx_t;
                0 as libc::c_int
            }
        } else if if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                    }) as libc::c_long + s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (n
                        < (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_long / s)
                        as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint) as libc::c_long
                            >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_long / -s
                    }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < n
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
            }
        } else if s == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else if n < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && n == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                        < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
            }
        } else {
            ((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_long / s < n) as libc::c_int
        } != 0
        {
            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint) as idx_t;
            1 as libc::c_int
        } else {
            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint) as idx_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<idx_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
    {
        if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            nbytes
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            if if s < 0 as libc::c_int as libc::c_long {
                if n < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            9223372036854775807 as libc::c_long
                        }) + s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (n < 9223372036854775807 as libc::c_long / s) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (s
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 9223372036854775807 as libc::c_long
                                >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            9223372036854775807 as libc::c_long / -s
                        }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                })
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        })
                            + (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)) as libc::c_int
                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < n
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < n
                            && -(1 as libc::c_int) as libc::c_long
                                - (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long) / s
                        < n) as libc::c_int
                }
            } else if s == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if n < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            })
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                })
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        })
                            + (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)) as libc::c_int
                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < s
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_int
                    } else {
                        (-(1 as libc::c_int) as libc::c_long
                            - (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long) / n
                        < s) as libc::c_int
                }
            } else {
                (9223372036854775807 as libc::c_long / s < n) as libc::c_int
            } != 0
            {
                nbytes = (n as libc::c_ulong).wrapping_mul(s as libc::c_ulong)
                    as libc::c_long;
                1 as libc::c_int
            } else {
                nbytes = (n as libc::c_ulong).wrapping_mul(s as libc::c_ulong)
                    as libc::c_long;
                0 as libc::c_int
            }
        } else if if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                    })
                        .wrapping_add(s as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((n as libc::c_ulong)
                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(s as libc::c_ulong)) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) as libc::c_ulong)
                            .wrapping_add(
                                (9223372036854775807 as libc::c_long as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_ulong),
                            )
                            >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(-s as libc::c_ulong)
                    }) <= (-(1 as libc::c_int) as libc::c_long - n) as libc::c_ulong)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < n
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
            }
        } else if s == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else if n < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && n == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                        < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_div(s as libc::c_ulong) < n as libc::c_ulong) as libc::c_int
        } != 0
        {
            nbytes = (n as libc::c_ulong).wrapping_mul(s as libc::c_ulong) as idx_t;
            1 as libc::c_int
        } else {
            nbytes = (n as libc::c_ulong).wrapping_mul(s as libc::c_ulong) as idx_t;
            0 as libc::c_int
        }
    } else if ((if 1 as libc::c_int != 0 {
        0 as libc::c_int as libc::c_long
    } else {
        nbytes
    }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
    {
        if if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        9223372036854775807 as libc::c_longlong
                    }) + s as libc::c_longlong
                }) - 1 as libc::c_int as libc::c_longlong)
                    < 0 as libc::c_int as libc::c_longlong
                {
                    ((n as libc::c_longlong)
                        < 9223372036854775807 as libc::c_longlong
                            / s as libc::c_longlong) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) as libc::c_longlong + 9223372036854775807 as libc::c_longlong
                            >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        9223372036854775807 as libc::c_longlong / -s as libc::c_longlong
                    }) <= (-(1 as libc::c_int) as libc::c_long - n) as libc::c_longlong)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_longlong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
            }) - 1 as libc::c_int as libc::c_longlong)
                < 0 as libc::c_int as libc::c_longlong
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 1 as libc::c_int as libc::c_longlong)
                    << (::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_longlong)
                    * 2 as libc::c_int as libc::c_longlong
                    + 1 as libc::c_int as libc::c_longlong)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as libc::c_int as libc::c_longlong
            }) < 0 as libc::c_int as libc::c_longlong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) - 1 as libc::c_int as libc::c_longlong)
                        < 0 as libc::c_int as libc::c_longlong
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) + 1 as libc::c_int as libc::c_longlong)
                            << (::std::mem::size_of::<libc::c_longlong>()
                                as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_longlong)
                            * 2 as libc::c_int as libc::c_longlong
                            + 1 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as libc::c_int as libc::c_longlong
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_longlong)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as libc::c_int
            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_longlong)
                        < n as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < n
                        && -(1 as libc::c_int) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                            < (n - 1 as libc::c_int as libc::c_long) as libc::c_longlong)
                        as libc::c_int
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / s as libc::c_longlong) < n as libc::c_longlong) as libc::c_int
            }
        } else if s == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else if n < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_longlong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
            }) - 1 as libc::c_int as libc::c_longlong)
                < 0 as libc::c_int as libc::c_longlong
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 1 as libc::c_int as libc::c_longlong)
                    << (::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_longlong)
                    * 2 as libc::c_int as libc::c_longlong
                    + 1 as libc::c_int as libc::c_longlong)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as libc::c_int as libc::c_longlong
            }) < 0 as libc::c_int as libc::c_longlong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) - 1 as libc::c_int as libc::c_longlong)
                        < 0 as libc::c_int as libc::c_longlong
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) + 1 as libc::c_int as libc::c_longlong)
                            << (::std::mem::size_of::<libc::c_longlong>()
                                as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_longlong)
                            * 2 as libc::c_int as libc::c_longlong
                            + 1 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as libc::c_int as libc::c_longlong
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_longlong)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as libc::c_int
            }) != 0 && n == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_longlong)
                        < s as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_int
                } else {
                    (-(1 as libc::c_int) as libc::c_longlong
                        - (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                        < (s - 1 as libc::c_int as libc::c_long) as libc::c_longlong)
                        as libc::c_int
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / n as libc::c_longlong) < s as libc::c_longlong) as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_longlong / s as libc::c_longlong)
                < n as libc::c_longlong) as libc::c_int
        } != 0
        {
            nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong)
                as libc::c_longlong as idx_t;
            1 as libc::c_int
        } else {
            nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong)
                as libc::c_longlong as idx_t;
            0 as libc::c_int
        }
    } else if if s < 0 as libc::c_int as libc::c_long {
        if n < 0 as libc::c_int as libc::c_long {
            if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulonglong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulonglong
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                })
                    .wrapping_add(s as libc::c_ulonglong)
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                < 0 as libc::c_int as libc::c_ulonglong
            {
                ((n as libc::c_ulonglong)
                    < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(s as libc::c_ulonglong)) as libc::c_int
            } else {
                ((if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (s
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                }) != 0
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong),
                        )
                        >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(-s as libc::c_ulonglong)
                }) <= (-(1 as libc::c_int) as libc::c_long - n) as libc::c_ulonglong)
                    as libc::c_int
            }
        } else if (if (if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            (if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { s })
                + 0 as libc::c_int as libc::c_long
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            !(((((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long
            }) + 1 as libc::c_int as libc::c_long)
                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long)
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long
            }) + 0 as libc::c_int as libc::c_long
        }) < 0 as libc::c_int as libc::c_long
        {
            (((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { s })
                + 0 as libc::c_int as libc::c_long)
                < -(if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long
                })) as libc::c_int
        } else {
            ((0 as libc::c_int as libc::c_long)
                < (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s
                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
        }) != 0 && s == -(1 as libc::c_int) as libc::c_long
        {
            if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                n
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                ((0 as libc::c_int as libc::c_long)
                    < n + 0 as libc::c_int as libc::c_long) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long) < n
                    && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                        < n - 1 as libc::c_int as libc::c_long) as libc::c_int
            }
        } else {
            (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
        }
    } else if s == 0 as libc::c_int as libc::c_long {
        0 as libc::c_int
    } else if n < 0 as libc::c_int as libc::c_long {
        if (if (if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            (if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { n })
                + 0 as libc::c_int as libc::c_long
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            !(((((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long
            }) + 1 as libc::c_int as libc::c_long)
                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long)
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long
            }) + 0 as libc::c_int as libc::c_long
        }) < 0 as libc::c_int as libc::c_long
        {
            (((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { n })
                + 0 as libc::c_int as libc::c_long)
                < -(if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as libc::c_long
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long
                })) as libc::c_int
        } else {
            ((0 as libc::c_int as libc::c_long)
                < (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
        }) != 0 && n == -(1 as libc::c_int) as libc::c_long
        {
            if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                s
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                ((0 as libc::c_int as libc::c_long)
                    < s + 0 as libc::c_int as libc::c_long) as libc::c_int
            } else {
                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                    < s - 1 as libc::c_int as libc::c_long) as libc::c_int
            }
        } else {
            (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
        }
    } else {
        ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(s as libc::c_ulonglong) < n as libc::c_ulonglong)
            as libc::c_int
    } != 0
    {
        nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong) as idx_t;
        1 as libc::c_int
    } else {
        nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong) as idx_t;
        0 as libc::c_int
    } != 0
    {
        if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        }
    } else {
        (if nbytes < DEFAULT_MXFAST_0 as libc::c_int as libc::c_long {
            DEFAULT_MXFAST_0 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_ulong
    }) as idx_t;
    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }
    if pa.is_null() {
        *pn = 0 as libc::c_int as idx_t;
    }
    if n - n0 < n_incr_min
        && {
            let (fresh4, fresh5) = n0.overflowing_add(n_incr_min);
            *(&mut n as *mut idx_t) = fresh4;
            fresh5 as libc::c_int != 0
                || 0 as libc::c_int as libc::c_long <= n_max && n_max < n
                || (if ::std::mem::size_of::<idx_t>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
                {
                    if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t) {
                        if (if s < 0 as libc::c_int as libc::c_long {
                            if n < 0 as libc::c_int as libc::c_long {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        127 as libc::c_int
                                    }) as libc::c_long + s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (n < 127 as libc::c_int as libc::c_long / s) as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (s
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 127 as libc::c_int as libc::c_long
                                            >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        127 as libc::c_int as libc::c_long / -s
                                    }) <= -(1 as libc::c_int) as libc::c_long - n)
                                        as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    })
                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    })
                                        + (-(127 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            })
                                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long) as libc::c_int
                                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < n
                                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < n
                                            && ((-(1 as libc::c_int)
                                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                                as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                        / s < n) as libc::c_int
                                }
                            }
                        } else {
                            if s == 0 as libc::c_int as libc::c_long {
                                0 as libc::c_int
                            } else {
                                if n < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            })
                                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            })
                                                + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                })
                                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            })
                                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long) as libc::c_int
                                    }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((0 as libc::c_int as libc::c_long)
                                                < s
                                                    + (-(127 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long) as libc::c_int
                                        } else {
                                            (((-(1 as libc::c_int)
                                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                                as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                                                as libc::c_int
                                        }
                                    } else {
                                        ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                            / n < s) as libc::c_int
                                    }
                                } else {
                                    (127 as libc::c_int as libc::c_long / s < n) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                as libc::c_schar as idx_t;
                            1 as libc::c_int
                        } else {
                            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                as libc::c_schar as idx_t;
                            0 as libc::c_int
                        }
                    } else {
                        if (if s < 0 as libc::c_int as libc::c_long {
                            if n < 0 as libc::c_int as libc::c_long {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                    }) as libc::c_long + s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (n
                                        < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                            as libc::c_long / s) as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (s
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        })
                                            + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                as libc::c_long
                                            >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                            as libc::c_long / -s
                                    }) <= -(1 as libc::c_int) as libc::c_long - n)
                                        as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < n
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
                                }
                            }
                        } else {
                            if s == 0 as libc::c_int as libc::c_long {
                                0 as libc::c_int
                            } else {
                                if n < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((0 as libc::c_int as libc::c_long)
                                                < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                                        } else {
                                            (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                                < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
                                    }
                                } else {
                                    ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                        as libc::c_long / s < n) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                as libc::c_uchar as idx_t;
                            1 as libc::c_int
                        } else {
                            nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                as libc::c_uchar as idx_t;
                            0 as libc::c_int
                        }
                    }
                } else {
                    if ::std::mem::size_of::<idx_t>() as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                    {
                        if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t)
                        {
                            if (if s < 0 as libc::c_int as libc::c_long {
                                if n < 0 as libc::c_int as libc::c_long {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            32767 as libc::c_int
                                        }) as libc::c_long + s
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (n < 32767 as libc::c_int as libc::c_long / s)
                                            as libc::c_int
                                    } else {
                                        ((if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (s
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                        }) != 0
                                        {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 32767 as libc::c_int as libc::c_long
                                                >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            32767 as libc::c_int as libc::c_long / -s
                                        }) <= -(1 as libc::c_int) as libc::c_long - n)
                                            as libc::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        })
                                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            })
                                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            })
                                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        })
                                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    })
                                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    })
                                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            })
                                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long) as libc::c_int
                                    }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((0 as libc::c_int as libc::c_long)
                                                < n
                                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long) < n
                                                && ((-(1 as libc::c_int)
                                                    - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                    as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                                as libc::c_int
                                        }
                                    } else {
                                        ((-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long / s < n) as libc::c_int
                                    }
                                }
                            } else {
                                if s == 0 as libc::c_int as libc::c_long {
                                    0 as libc::c_int
                                } else {
                                    if n < 0 as libc::c_int as libc::c_long {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            })
                                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                })
                                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                })
                                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            })
                                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        })
                                                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_long
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        })
                                                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                })
                                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long) as libc::c_int
                                        }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < s
                                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long) as libc::c_int
                                            } else {
                                                (((-(1 as libc::c_int)
                                                    - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                    as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                                                    as libc::c_int
                                            }
                                        } else {
                                            ((-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long / n < s) as libc::c_int
                                        }
                                    } else {
                                        (32767 as libc::c_int as libc::c_long / s < n)
                                            as libc::c_int
                                    }
                                }
                            }) != 0
                            {
                                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                    as libc::c_short as idx_t;
                                1 as libc::c_int
                            } else {
                                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                    as libc::c_short as idx_t;
                                0 as libc::c_int
                            }
                        } else {
                            if (if s < 0 as libc::c_int as libc::c_long {
                                if n < 0 as libc::c_int as libc::c_long {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                        }) as libc::c_long + s
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (n
                                            < (32767 as libc::c_int * 2 as libc::c_int
                                                + 1 as libc::c_int) as libc::c_long / s) as libc::c_int
                                    } else {
                                        ((if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (s
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                        }) != 0
                                        {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            })
                                                + (32767 as libc::c_int * 2 as libc::c_int
                                                    + 1 as libc::c_int) as libc::c_long
                                                >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                as libc::c_long / -s
                                        }) <= -(1 as libc::c_int) as libc::c_long - n)
                                            as libc::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((0 as libc::c_int as libc::c_long)
                                                < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long) < n
                                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                    as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                                as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
                                    }
                                }
                            } else {
                                if s == 0 as libc::c_int as libc::c_long {
                                    0 as libc::c_int
                                } else {
                                    if n < 0 as libc::c_int as libc::c_long {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as libc::c_long)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                        }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                                            } else {
                                                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                                    < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                            }
                                        } else {
                                            (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
                                        }
                                    } else {
                                        ((32767 as libc::c_int * 2 as libc::c_int
                                            + 1 as libc::c_int) as libc::c_long / s < n) as libc::c_int
                                    }
                                }
                            }) != 0
                            {
                                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                    as libc::c_ushort as idx_t;
                                1 as libc::c_int
                            } else {
                                nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                    as libc::c_ushort as idx_t;
                                0 as libc::c_int
                            }
                        }
                    } else {
                        if ::std::mem::size_of::<idx_t>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                nbytes
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                if (if s < 0 as libc::c_int as libc::c_long {
                                    if n < 0 as libc::c_int as libc::c_long {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                2147483647 as libc::c_int
                                            }) as libc::c_long + s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            (n < 2147483647 as libc::c_int as libc::c_long / s)
                                                as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (s
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 2147483647 as libc::c_int as libc::c_long
                                                    >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                2147483647 as libc::c_int as libc::c_long / -s
                                            }) <= -(1 as libc::c_int) as libc::c_long - n)
                                                as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            })
                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            })
                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    })
                                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        })
                                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_long
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        })
                                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long) as libc::c_int
                                        }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < n
                                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long) < n
                                                    && ((-(1 as libc::c_int)
                                                        - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                        as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                                    as libc::c_int
                                            }
                                        } else {
                                            ((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long / s < n) as libc::c_int
                                        }
                                    }
                                } else {
                                    if s == 0 as libc::c_int as libc::c_long {
                                        0 as libc::c_int
                                    } else {
                                        if n < 0 as libc::c_int as libc::c_long {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                })
                                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                })
                                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_long)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        })
                                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            })
                                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                    as libc::c_long
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            })
                                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                    as libc::c_long
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_long) as libc::c_int
                                            }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < s
                                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_long) as libc::c_int
                                                } else {
                                                    (((-(1 as libc::c_int)
                                                        - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                        as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                                                        as libc::c_int
                                                }
                                            } else {
                                                ((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_long / n < s) as libc::c_int
                                            }
                                        } else {
                                            (2147483647 as libc::c_int as libc::c_long / s < n)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                        as libc::c_int as idx_t;
                                    1 as libc::c_int
                                } else {
                                    nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                        as libc::c_int as idx_t;
                                    0 as libc::c_int
                                }
                            } else {
                                if (if s < 0 as libc::c_int as libc::c_long {
                                    if n < 0 as libc::c_int as libc::c_long {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_uint
                                            } else {
                                                (2147483647 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(2 as libc::c_uint)
                                                    .wrapping_add(1 as libc::c_uint)
                                            }) as libc::c_long + s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            (n
                                                < (2147483647 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(2 as libc::c_uint)
                                                    .wrapping_add(1 as libc::c_uint) as libc::c_long / s)
                                                as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (s
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (2147483647 as libc::c_int as libc::c_uint)
                                                        .wrapping_mul(2 as libc::c_uint)
                                                        .wrapping_add(1 as libc::c_uint) as libc::c_long
                                                    >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (2147483647 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(2 as libc::c_uint)
                                                    .wrapping_add(1 as libc::c_uint) as libc::c_long / -s
                                            }) <= -(1 as libc::c_int) as libc::c_long - n)
                                                as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as libc::c_long)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                        }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long) < n
                                                    && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                        as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                                    as libc::c_int
                                            }
                                        } else {
                                            (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
                                        }
                                    }
                                } else {
                                    if s == 0 as libc::c_int as libc::c_long {
                                        0 as libc::c_int
                                    } else {
                                        if n < 0 as libc::c_int as libc::c_long {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as libc::c_long)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                            }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                                                } else {
                                                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                                        < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                                }
                                            } else {
                                                (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
                                            }
                                        } else {
                                            ((2147483647 as libc::c_int as libc::c_uint)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_add(1 as libc::c_uint) as libc::c_long / s < n)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                        as idx_t;
                                    1 as libc::c_int
                                } else {
                                    nbytes = (n as libc::c_uint).wrapping_mul(s as libc::c_uint)
                                        as idx_t;
                                    0 as libc::c_int
                                }
                            }
                        } else {
                            if ::std::mem::size_of::<idx_t>() as libc::c_ulong
                                == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    nbytes
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    if (if s < 0 as libc::c_int as libc::c_long {
                                        if n < 0 as libc::c_int as libc::c_long {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    9223372036854775807 as libc::c_long
                                                }) + s
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                (n < 9223372036854775807 as libc::c_long / s) as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    (s
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                                }) != 0
                                                {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 9223372036854775807 as libc::c_long
                                                        >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    9223372036854775807 as libc::c_long / -s
                                                }) <= -(1 as libc::c_int) as libc::c_long - n)
                                                    as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    })
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    })
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                })
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        })
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            })
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            })
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    })
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)) as libc::c_int
                                            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < n
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < n
                                                        && -(1 as libc::c_int) as libc::c_long
                                                            - (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                                        as libc::c_int
                                                }
                                            } else {
                                                ((-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long) / s < n) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if s == 0 as libc::c_int as libc::c_long {
                                            0 as libc::c_int
                                        } else {
                                            if n < 0 as libc::c_int as libc::c_long {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        })
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        })
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    })
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            })
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                })
                                                                    + (-(9223372036854775807 as libc::c_long)
                                                                        - 1 as libc::c_long)
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                })
                                                                    + (-(9223372036854775807 as libc::c_long)
                                                                        - 1 as libc::c_long)
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        })
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)) as libc::c_int
                                                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((0 as libc::c_int as libc::c_long)
                                                            < s
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)) as libc::c_int
                                                    } else {
                                                        (-(1 as libc::c_int) as libc::c_long
                                                            - (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long) < s - 1 as libc::c_int as libc::c_long)
                                                            as libc::c_int
                                                    }
                                                } else {
                                                    ((-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long) / n < s) as libc::c_int
                                                }
                                            } else {
                                                (9223372036854775807 as libc::c_long / s < n) as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        nbytes = (n as libc::c_ulong)
                                            .wrapping_mul(s as libc::c_ulong) as libc::c_long;
                                        1 as libc::c_int
                                    } else {
                                        nbytes = (n as libc::c_ulong)
                                            .wrapping_mul(s as libc::c_ulong) as libc::c_long;
                                        0 as libc::c_int
                                    }
                                } else {
                                    if (if s < 0 as libc::c_int as libc::c_long {
                                        if n < 0 as libc::c_int as libc::c_long {
                                            if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_ulong)
                                                })
                                                    .wrapping_add(s as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((n as libc::c_ulong)
                                                    < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_ulong)
                                                        .wrapping_div(s as libc::c_ulong)) as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    (s
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                                }) != 0
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) as libc::c_ulong)
                                                        .wrapping_add(
                                                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                                .wrapping_mul(2 as libc::c_ulong)
                                                                .wrapping_add(1 as libc::c_ulong),
                                                        )
                                                        >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_ulong)
                                                        .wrapping_div(-s as libc::c_ulong)
                                                })
                                                    <= (-(1 as libc::c_int) as libc::c_long - n)
                                                        as libc::c_ulong) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < n
                                                        && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                            as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                                        as libc::c_int
                                                }
                                            } else {
                                                (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if s == 0 as libc::c_int as libc::c_long {
                                            0 as libc::c_int
                                        } else {
                                            if n < 0 as libc::c_int as libc::c_long {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    (((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                }) + 0 as libc::c_int as libc::c_long
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                }) + 0 as libc::c_int as libc::c_long
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((0 as libc::c_int as libc::c_long)
                                                            < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                                                    } else {
                                                        (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                                            < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                                    }
                                                } else {
                                                    (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
                                                }
                                            } else {
                                                ((9223372036854775807 as libc::c_long as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_ulong)
                                                    .wrapping_div(s as libc::c_ulong) < n as libc::c_ulong)
                                                    as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        nbytes = (n as libc::c_ulong)
                                            .wrapping_mul(s as libc::c_ulong) as idx_t;
                                        1 as libc::c_int
                                    } else {
                                        nbytes = (n as libc::c_ulong)
                                            .wrapping_mul(s as libc::c_ulong) as idx_t;
                                        0 as libc::c_int
                                    }
                                }
                            } else {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    nbytes
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    if (if s < 0 as libc::c_int as libc::c_long {
                                        if n < 0 as libc::c_int as libc::c_long {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    9223372036854775807 as libc::c_longlong
                                                }) + s as libc::c_longlong
                                            }) - 1 as libc::c_int as libc::c_longlong)
                                                < 0 as libc::c_int as libc::c_longlong
                                            {
                                                ((n as libc::c_longlong)
                                                    < 9223372036854775807 as libc::c_longlong
                                                        / s as libc::c_longlong) as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    (s
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                                }) != 0
                                                {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) as libc::c_longlong
                                                        + 9223372036854775807 as libc::c_longlong
                                                        >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    9223372036854775807 as libc::c_longlong
                                                        / -s as libc::c_longlong
                                                })
                                                    <= (-(1 as libc::c_int) as libc::c_long - n)
                                                        as libc::c_longlong) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as libc::c_int as libc::c_longlong)
                                                < 0 as libc::c_int as libc::c_longlong
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 1 as libc::c_int as libc::c_longlong)
                                                    << (::std::mem::size_of::<libc::c_longlong>()
                                                        as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_longlong)
                                                    * 2 as libc::c_int as libc::c_longlong
                                                    + 1 as libc::c_int as libc::c_longlong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 0 as libc::c_int as libc::c_longlong
                                            }) < 0 as libc::c_int as libc::c_longlong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) - 1 as libc::c_int as libc::c_longlong)
                                                        < 0 as libc::c_int as libc::c_longlong
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) + 1 as libc::c_int as libc::c_longlong)
                                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                                as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_longlong)
                                                            * 2 as libc::c_int as libc::c_longlong
                                                            + 1 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) - 1 as libc::c_int as libc::c_longlong
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_longlong)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)) as libc::c_int
                                            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_longlong)
                                                        < n as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < n
                                                        && -(1 as libc::c_int) as libc::c_longlong
                                                            - (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                            < (n - 1 as libc::c_int as libc::c_long)
                                                                as libc::c_longlong) as libc::c_int
                                                }
                                            } else {
                                                (((-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong) / s as libc::c_longlong)
                                                    < n as libc::c_longlong) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if s == 0 as libc::c_int as libc::c_long {
                                            0 as libc::c_int
                                        } else {
                                            if n < 0 as libc::c_int as libc::c_long {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) - 1 as libc::c_int as libc::c_longlong)
                                                    < 0 as libc::c_int as libc::c_longlong
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) + 1 as libc::c_int as libc::c_longlong)
                                                        << (::std::mem::size_of::<libc::c_longlong>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_longlong)
                                                        * 2 as libc::c_int as libc::c_longlong
                                                        + 1 as libc::c_int as libc::c_longlong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) + 0 as libc::c_int as libc::c_longlong
                                                }) < 0 as libc::c_int as libc::c_longlong
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) - 1 as libc::c_int as libc::c_longlong)
                                                            < 0 as libc::c_int as libc::c_longlong
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_longlong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                }) as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) + 1 as libc::c_int as libc::c_longlong)
                                                                << (::std::mem::size_of::<libc::c_longlong>()
                                                                    as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_longlong)
                                                                * 2 as libc::c_int as libc::c_longlong
                                                                + 1 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_longlong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                }) as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) - 1 as libc::c_int as libc::c_longlong
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_longlong)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)) as libc::c_int
                                                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((0 as libc::c_int as libc::c_longlong)
                                                            < s as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)) as libc::c_int
                                                    } else {
                                                        (-(1 as libc::c_int) as libc::c_longlong
                                                            - (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                            < (s - 1 as libc::c_int as libc::c_long)
                                                                as libc::c_longlong) as libc::c_int
                                                    }
                                                } else {
                                                    (((-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) / n as libc::c_longlong)
                                                        < s as libc::c_longlong) as libc::c_int
                                                }
                                            } else {
                                                ((9223372036854775807 as libc::c_longlong
                                                    / s as libc::c_longlong) < n as libc::c_longlong)
                                                    as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as libc::c_longlong
                                            as idx_t;
                                        1 as libc::c_int
                                    } else {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as libc::c_longlong
                                            as idx_t;
                                        0 as libc::c_int
                                    }
                                } else {
                                    if (if s < 0 as libc::c_int as libc::c_long {
                                        if n < 0 as libc::c_int as libc::c_long {
                                            if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulonglong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulonglong
                                                } else {
                                                    (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                })
                                                    .wrapping_add(s as libc::c_ulonglong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                < 0 as libc::c_int as libc::c_ulonglong
                                            {
                                                ((n as libc::c_ulonglong)
                                                    < (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(s as libc::c_ulonglong)) as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    (s
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                                }) != 0
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) as libc::c_ulonglong)
                                                        .wrapping_add(
                                                            (9223372036854775807 as libc::c_longlong
                                                                as libc::c_ulonglong)
                                                                .wrapping_mul(2 as libc::c_ulonglong)
                                                                .wrapping_add(1 as libc::c_ulonglong),
                                                        )
                                                        >> (::std::mem::size_of::<idx_t>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(-s as libc::c_ulonglong)
                                                })
                                                    <= (-(1 as libc::c_int) as libc::c_long - n)
                                                        as libc::c_ulonglong) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    s
                                                }) + 0 as libc::c_int as libc::c_long)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            s
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                s
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < n + 0 as libc::c_int as libc::c_long) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long) < n
                                                        && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                            as libc::c_long) < n - 1 as libc::c_int as libc::c_long)
                                                        as libc::c_int
                                                }
                                            } else {
                                                (0 as libc::c_int as libc::c_long / s < n) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if s == 0 as libc::c_int as libc::c_long {
                                            0 as libc::c_int
                                        } else {
                                            if n < 0 as libc::c_int as libc::c_long {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    (((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        n
                                                    }) + 0 as libc::c_int as libc::c_long)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                n
                                                            }) + 0 as libc::c_int as libc::c_long
                                                        }) - 1 as libc::c_int as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                }) + 0 as libc::c_int as libc::c_long
                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int as libc::c_long)
                                                                * 2 as libc::c_int as libc::c_long
                                                                + 1 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    n
                                                                }) + 0 as libc::c_int as libc::c_long
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            n
                                                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        s
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((0 as libc::c_int as libc::c_long)
                                                            < s + 0 as libc::c_int as libc::c_long) as libc::c_int
                                                    } else {
                                                        (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                                                            < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                                    }
                                                } else {
                                                    (0 as libc::c_int as libc::c_long / n < s) as libc::c_int
                                                }
                                            } else {
                                                ((9223372036854775807 as libc::c_longlong
                                                    as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_ulonglong)
                                                    .wrapping_div(s as libc::c_ulonglong)
                                                    < n as libc::c_ulonglong) as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as idx_t;
                                        1 as libc::c_int
                                    } else {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as idx_t;
                                        0 as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                }) != 0
        }
    {
        xalloc_die();
    }
    pa = xrealloc(pa, nbytes as size_t);
    *pn = n;
    return pa;
}
pub unsafe extern "C" fn xzalloc(mut s: size_t) -> *mut libc::c_void {
    return xcalloc(s, 1 as libc::c_int as size_t);
}
pub unsafe extern "C" fn xizalloc(mut s: idx_t) -> *mut libc::c_void {
    return xicalloc(s, 1 as libc::c_int as idx_t);
}
pub unsafe extern "C" fn xcalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    return nonnull(calloc(n, s));
}
pub unsafe extern "C" fn xicalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    return nonnull(icalloc(n, s));
}
pub unsafe extern "C" fn xmemdup(
    mut p: *const libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    return memcpy(xmalloc(s), p, s);
}
pub unsafe extern "C" fn ximemdup(
    mut p: *const libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return memcpy(ximalloc(s), p, s as libc::c_ulong);
}
pub unsafe extern "C" fn ximemdup0(
    mut p: *const libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = ximalloc(s + 1 as libc::c_int as libc::c_long)
        as *mut libc::c_char;
    *result.offset(s as isize) = 0 as libc::c_int as libc::c_char;
    return memcpy(result as *mut libc::c_void, p, s as libc::c_ulong)
        as *mut libc::c_char;
}
pub unsafe extern "C" fn xstrdup(mut string: *const libc::c_char) -> *mut libc::c_char {
    return xmemdup(
        string as *const libc::c_void,
        (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
}
