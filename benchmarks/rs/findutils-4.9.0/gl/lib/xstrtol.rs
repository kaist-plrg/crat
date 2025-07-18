use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
unsafe extern "C" fn bkm_scale(
    mut x: *mut libc::c_long,
    mut scale_factor: libc::c_int,
) -> strtol_error {
    let mut scaled: libc::c_long = 0;
    if if ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if !((0 as libc::c_int as libc::c_long) < -(1 as libc::c_int) as libc::c_long) {
            if if scale_factor < 0 as libc::c_int {
                if *x < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int
                        }) + scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (*x < (127 as libc::c_int / scale_factor) as libc::c_long)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (scale_factor
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < scale_factor) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 127 as libc::c_int
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            127 as libc::c_int / -scale_factor
                        }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - *x)
                            as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + (-(127 as libc::c_int) - 1 as libc::c_int)
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                    }) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)) as libc::c_int
                }) != 0 && scale_factor == -(1 as libc::c_int)
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < *x
                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < *x
                            && ((-(1 as libc::c_int)
                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long) < *x - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((((-(127 as libc::c_int) - 1 as libc::c_int) / scale_factor)
                        as libc::c_long) < *x) as libc::c_int
                }
            } else if scale_factor == 0 as libc::c_int {
                0 as libc::c_int
            } else if *x < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                            *x
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                *x
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
                                    *x
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
                                    *x
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        as libc::c_int
                }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int)
                            < scale_factor + (-(127 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    } else {
                        (-(1 as libc::c_int) - (-(127 as libc::c_int) - 1 as libc::c_int)
                            < scale_factor - 1 as libc::c_int) as libc::c_int
                    }
                } else {
                    ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long / *x
                        < scale_factor as libc::c_long) as libc::c_int
                }
            } else {
                (((127 as libc::c_int / scale_factor) as libc::c_long) < *x)
                    as libc::c_int
            } != 0
            {
                scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                    as libc::c_schar as libc::c_long;
                1 as libc::c_int
            } else {
                scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                    as libc::c_schar as libc::c_long;
                0 as libc::c_int
            }
        } else if if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    }) + scale_factor
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    (*x
                        < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            / scale_factor) as libc::c_long) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            / -scale_factor
                    }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - *x)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int
            }) - 1 as libc::c_int) < 0 as libc::c_int
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + 0 as libc::c_int) as libc::c_int
            }) != 0 && scale_factor == -(1 as libc::c_int)
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < *x + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < *x
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < *x - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (((0 as libc::c_int / scale_factor) as libc::c_long) < *x) as libc::c_int
            }
        } else if scale_factor == 0 as libc::c_int {
            0 as libc::c_int
        } else if *x < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
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
                        *x
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                                *x
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
                                *x
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((0 as libc::c_int) < scale_factor + 0 as libc::c_int) as libc::c_int
                } else {
                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                        < scale_factor - 1 as libc::c_int) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / *x < scale_factor as libc::c_long)
                    as libc::c_int
            }
        } else {
            ((((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) / scale_factor)
                as libc::c_long) < *x) as libc::c_int
        } != 0
        {
            scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                as libc::c_uchar as libc::c_long;
            1 as libc::c_int
        } else {
            scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                as libc::c_uchar as libc::c_long;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
    {
        if !((0 as libc::c_int as libc::c_long) < -(1 as libc::c_int) as libc::c_long) {
            if if scale_factor < 0 as libc::c_int {
                if *x < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            32767 as libc::c_int
                        }) + scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (*x < (32767 as libc::c_int / scale_factor) as libc::c_long)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (scale_factor
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < scale_factor) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 32767 as libc::c_int
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            32767 as libc::c_int / -scale_factor
                        }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - *x)
                            as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                    }) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)) as libc::c_int
                }) != 0 && scale_factor == -(1 as libc::c_int)
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < *x
                                + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < *x
                            && ((-(1 as libc::c_int)
                                - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long) < *x - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((((-(32767 as libc::c_int) - 1 as libc::c_int) / scale_factor)
                        as libc::c_long) < *x) as libc::c_int
                }
            } else if scale_factor == 0 as libc::c_int {
                0 as libc::c_int
            } else if *x < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                            *x
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                *x
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
                                    *x
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
                                    *x
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
                            *x
                        })
                            + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long) as libc::c_int
                }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int)
                            < scale_factor
                                + (-(32767 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    } else {
                        (-(1 as libc::c_int)
                            - (-(32767 as libc::c_int) - 1 as libc::c_int)
                            < scale_factor - 1 as libc::c_int) as libc::c_int
                    }
                } else {
                    ((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long / *x
                        < scale_factor as libc::c_long) as libc::c_int
                }
            } else {
                (((32767 as libc::c_int / scale_factor) as libc::c_long) < *x)
                    as libc::c_int
            } != 0
            {
                scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                    as libc::c_short as libc::c_long;
                1 as libc::c_int
            } else {
                scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                    as libc::c_short as libc::c_long;
                0 as libc::c_int
            }
        } else if if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    }) + scale_factor
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    (*x
                        < ((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            / scale_factor) as libc::c_long) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            / -scale_factor
                    }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - *x)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int
            }) - 1 as libc::c_int) < 0 as libc::c_int
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + 0 as libc::c_int) as libc::c_int
            }) != 0 && scale_factor == -(1 as libc::c_int)
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < *x + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < *x
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < *x - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (((0 as libc::c_int / scale_factor) as libc::c_long) < *x) as libc::c_int
            }
        } else if scale_factor == 0 as libc::c_int {
            0 as libc::c_int
        } else if *x < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
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
                        *x
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                                *x
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
                                *x
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((0 as libc::c_int) < scale_factor + 0 as libc::c_int) as libc::c_int
                } else {
                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                        < scale_factor - 1 as libc::c_int) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / *x < scale_factor as libc::c_long)
                    as libc::c_int
            }
        } else {
            ((((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                / scale_factor) as libc::c_long) < *x) as libc::c_int
        } != 0
        {
            scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                as libc::c_ushort as libc::c_long;
            1 as libc::c_int
        } else {
            scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                as libc::c_ushort as libc::c_long;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            scaled
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            if if scale_factor < 0 as libc::c_int {
                if *x < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2147483647 as libc::c_int
                        }) + scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (*x < (2147483647 as libc::c_int / scale_factor) as libc::c_long)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (scale_factor
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < scale_factor) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 2147483647 as libc::c_int
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            2147483647 as libc::c_int / -scale_factor
                        }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - *x)
                            as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                    }) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                        as libc::c_int
                }) != 0 && scale_factor == -(1 as libc::c_int)
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < *x
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < *x
                            && ((-(1 as libc::c_int)
                                - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_long) < *x - 1 as libc::c_int as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((((-(2147483647 as libc::c_int) - 1 as libc::c_int) / scale_factor)
                        as libc::c_long) < *x) as libc::c_int
                }
            } else if scale_factor == 0 as libc::c_int {
                0 as libc::c_int
            } else if *x < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
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
                            *x
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
                            *x
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    })
                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                *x
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
                                    *x
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
                                    *x
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
                            *x
                        })
                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long) as libc::c_int
                }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int)
                            < scale_factor
                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    } else {
                        (-(1 as libc::c_int)
                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            < scale_factor - 1 as libc::c_int) as libc::c_int
                    }
                } else {
                    ((-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                        / *x < scale_factor as libc::c_long) as libc::c_int
                }
            } else {
                (((2147483647 as libc::c_int / scale_factor) as libc::c_long) < *x)
                    as libc::c_int
            } != 0
            {
                scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                    as libc::c_int as libc::c_long;
                1 as libc::c_int
            } else {
                scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                    as libc::c_int as libc::c_long;
                0 as libc::c_int
            }
        } else if if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_long {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_uint
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                    })
                        .wrapping_add(scale_factor as libc::c_uint)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    < 0 as libc::c_int as libc::c_uint
                {
                    (*x
                        < (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_div(scale_factor as libc::c_uint) as libc::c_long)
                        as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_uint)
                            .wrapping_add(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint),
                            )
                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_div(-scale_factor as libc::c_uint)
                    }) as libc::c_long <= -(1 as libc::c_int) as libc::c_long - *x)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int
            }) - 1 as libc::c_int) < 0 as libc::c_int
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + 0 as libc::c_int) as libc::c_int
            }) != 0 && scale_factor == -(1 as libc::c_int)
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < *x + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < *x
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < *x - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (((0 as libc::c_int / scale_factor) as libc::c_long) < *x) as libc::c_int
            }
        } else if scale_factor == 0 as libc::c_int {
            0 as libc::c_int
        } else if *x < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
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
                        *x
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                                *x
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
                                *x
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((0 as libc::c_int) < scale_factor + 0 as libc::c_int) as libc::c_int
                } else {
                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                        < scale_factor - 1 as libc::c_int) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / *x < scale_factor as libc::c_long)
                    as libc::c_int
            }
        } else {
            (((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_div(scale_factor as libc::c_uint) as libc::c_long) < *x)
                as libc::c_int
        } != 0
        {
            scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                as libc::c_long;
            1 as libc::c_int
        } else {
            scaled = (*x as libc::c_uint).wrapping_mul(scale_factor as libc::c_uint)
                as libc::c_long;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
    {
        if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            scaled
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            if if scale_factor < 0 as libc::c_int {
                if *x < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            9223372036854775807 as libc::c_long
                        }) + scale_factor as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (*x
                            < 9223372036854775807 as libc::c_long
                                / scale_factor as libc::c_long) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (scale_factor
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        scale_factor
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < scale_factor) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) as libc::c_long + 9223372036854775807 as libc::c_long
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            9223372036854775807 as libc::c_long
                                / -scale_factor as libc::c_long
                        }) <= -(1 as libc::c_int) as libc::c_long - *x) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        as libc::c_long
                        + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_long
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
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_long
                            + (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) as libc::c_long
                        + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) as libc::c_long
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) as libc::c_long
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
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) as libc::c_long
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_long
                            + (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)) as libc::c_int
                }) != 0 && scale_factor == -(1 as libc::c_int)
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < *x
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < *x
                            && -(1 as libc::c_int) as libc::c_long
                                - (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                                < *x - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        / scale_factor as libc::c_long) < *x) as libc::c_int
                }
            } else if scale_factor == 0 as libc::c_int {
                0 as libc::c_int
            } else if *x < 0 as libc::c_int as libc::c_long {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                            *x
                        }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                *x
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
                                    *x
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
                                    *x
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
                            *x
                        })
                            + (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)) as libc::c_int
                }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < scale_factor as libc::c_long
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_int
                    } else {
                        (-(1 as libc::c_int) as libc::c_long
                            - (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)
                            < (scale_factor - 1 as libc::c_int) as libc::c_long)
                            as libc::c_int
                    }
                } else {
                    ((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long) / *x
                        < scale_factor as libc::c_long) as libc::c_int
                }
            } else {
                ((9223372036854775807 as libc::c_long / scale_factor as libc::c_long)
                    < *x) as libc::c_int
            } != 0
            {
                scaled = (*x as libc::c_ulong)
                    .wrapping_mul(scale_factor as libc::c_ulong) as libc::c_long;
                1 as libc::c_int
            } else {
                scaled = (*x as libc::c_ulong)
                    .wrapping_mul(scale_factor as libc::c_ulong) as libc::c_long;
                0 as libc::c_int
            }
        } else if if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_long {
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
                        .wrapping_add(scale_factor as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((*x as libc::c_ulong)
                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(scale_factor as libc::c_ulong)) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_ulong)
                            .wrapping_add(
                                (9223372036854775807 as libc::c_long as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_ulong),
                            )
                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(-scale_factor as libc::c_ulong)
                    }) <= (-(1 as libc::c_int) as libc::c_long - *x) as libc::c_ulong)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int
            }) - 1 as libc::c_int) < 0 as libc::c_int
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + 0 as libc::c_int) as libc::c_int
            }) != 0 && scale_factor == -(1 as libc::c_int)
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_long)
                        < *x + 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < *x
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                            < *x - 1 as libc::c_int as libc::c_long) as libc::c_int
                }
            } else {
                (((0 as libc::c_int / scale_factor) as libc::c_long) < *x) as libc::c_int
            }
        } else if scale_factor == 0 as libc::c_int {
            0 as libc::c_int
        } else if *x < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
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
                        *x
                    }) + 0 as libc::c_int as libc::c_long
                }) + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                                *x
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
                                *x
                            }) + 0 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((0 as libc::c_int) < scale_factor + 0 as libc::c_int) as libc::c_int
                } else {
                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                        < scale_factor - 1 as libc::c_int) as libc::c_int
                }
            } else {
                (0 as libc::c_int as libc::c_long / *x < scale_factor as libc::c_long)
                    as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_div(scale_factor as libc::c_ulong) < *x as libc::c_ulong)
                as libc::c_int
        } != 0
        {
            scaled = (*x as libc::c_ulong).wrapping_mul(scale_factor as libc::c_ulong)
                as libc::c_long;
            1 as libc::c_int
        } else {
            scaled = (*x as libc::c_ulong).wrapping_mul(scale_factor as libc::c_ulong)
                as libc::c_long;
            0 as libc::c_int
        }
    } else if ((if 1 as libc::c_int != 0 {
        0 as libc::c_int as libc::c_long
    } else {
        scaled
    }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
    {
        if if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        9223372036854775807 as libc::c_longlong
                    }) + scale_factor as libc::c_longlong
                }) - 1 as libc::c_int as libc::c_longlong)
                    < 0 as libc::c_int as libc::c_longlong
                {
                    ((*x as libc::c_longlong)
                        < 9223372036854775807 as libc::c_longlong
                            / scale_factor as libc::c_longlong) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_longlong + 9223372036854775807 as libc::c_longlong
                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        9223372036854775807 as libc::c_longlong
                            / -scale_factor as libc::c_longlong
                    }) <= (-(1 as libc::c_int) as libc::c_long - *x) as libc::c_longlong)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_longlong
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
            }) - 1 as libc::c_int as libc::c_longlong)
                < 0 as libc::c_int as libc::c_longlong
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        as libc::c_longlong
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
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as libc::c_int as libc::c_longlong
            }) < 0 as libc::c_int as libc::c_longlong
            {
                ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
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
                                0 as libc::c_int
                            } else {
                                scale_factor
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
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as libc::c_int as libc::c_longlong
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_longlong)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as libc::c_int
            }) != 0 && scale_factor == -(1 as libc::c_int)
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((0 as libc::c_int as libc::c_longlong)
                        < *x as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long) < *x
                        && -(1 as libc::c_int) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                            < (*x - 1 as libc::c_int as libc::c_long)
                                as libc::c_longlong) as libc::c_int
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / scale_factor as libc::c_longlong) < *x as libc::c_longlong)
                    as libc::c_int
            }
        } else if scale_factor == 0 as libc::c_int {
            0 as libc::c_int
        } else if *x < 0 as libc::c_int as libc::c_long {
            if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_longlong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
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
                        *x
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
                        *x
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as libc::c_int as libc::c_longlong
            }) < 0 as libc::c_int as libc::c_longlong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                                *x
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
                                *x
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
                        *x
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as libc::c_int
            }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
            {
                if ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((0 as libc::c_int as libc::c_longlong)
                        < scale_factor as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_int
                } else {
                    (-(1 as libc::c_int) as libc::c_longlong
                        - (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                        < (scale_factor - 1 as libc::c_int) as libc::c_longlong)
                        as libc::c_int
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / *x as libc::c_longlong) < scale_factor as libc::c_longlong)
                    as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_longlong / scale_factor as libc::c_longlong)
                < *x as libc::c_longlong) as libc::c_int
        } != 0
        {
            scaled = (*x as libc::c_ulonglong)
                .wrapping_mul(scale_factor as libc::c_ulonglong) as libc::c_longlong
                as libc::c_long;
            1 as libc::c_int
        } else {
            scaled = (*x as libc::c_ulonglong)
                .wrapping_mul(scale_factor as libc::c_ulonglong) as libc::c_longlong
                as libc::c_long;
            0 as libc::c_int
        }
    } else if if scale_factor < 0 as libc::c_int {
        if *x < 0 as libc::c_int as libc::c_long {
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
                    .wrapping_add(scale_factor as libc::c_ulonglong)
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                < 0 as libc::c_int as libc::c_ulonglong
            {
                ((*x as libc::c_ulonglong)
                    < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(scale_factor as libc::c_ulonglong)) as libc::c_int
            } else {
                ((if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    scale_factor
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    (scale_factor
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int) < scale_factor) as libc::c_int
                }) != 0
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong),
                        )
                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(-scale_factor as libc::c_ulonglong)
                }) <= (-(1 as libc::c_int) as libc::c_long - *x) as libc::c_ulonglong)
                    as libc::c_int
            }
        } else if (if (if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                + 0 as libc::c_int
        }) - 1 as libc::c_int) < 0 as libc::c_int
        {
            !(((((if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int
            }) + 1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
                * 2 as libc::c_int + 1 as libc::c_int)
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int
            }) + 0 as libc::c_int
        }) < 0 as libc::c_int
        {
            (((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                + 0 as libc::c_int)
                < -(if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                        + 0 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int
                })) as libc::c_int
        } else {
            ((0 as libc::c_int)
                < (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                    + 0 as libc::c_int) as libc::c_int
        }) != 0 && scale_factor == -(1 as libc::c_int)
        {
            if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                *x
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                ((0 as libc::c_int as libc::c_long)
                    < *x + 0 as libc::c_int as libc::c_long) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_long) < *x
                    && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_long)
                        < *x - 1 as libc::c_int as libc::c_long) as libc::c_int
            }
        } else {
            (((0 as libc::c_int / scale_factor) as libc::c_long) < *x) as libc::c_int
        }
    } else if scale_factor == 0 as libc::c_int {
        0 as libc::c_int
    } else if *x < 0 as libc::c_int as libc::c_long {
        if (if (if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            (if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { *x })
                + 0 as libc::c_int as libc::c_long
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            !(((((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
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
                    *x
                }) + 0 as libc::c_int as libc::c_long
            }) + 0 as libc::c_int as libc::c_long
        }) < 0 as libc::c_int as libc::c_long
        {
            (((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { *x })
                + 0 as libc::c_int as libc::c_long)
                < -(if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        *x
                    }) + 0 as libc::c_int as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
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
                            *x
                        }) + 0 as libc::c_int as libc::c_long
                    }) - 1 as libc::c_int as libc::c_long
                })) as libc::c_int
        } else {
            ((0 as libc::c_int as libc::c_long)
                < (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    *x
                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
        }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
        {
            if ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
                - 1 as libc::c_int) < 0 as libc::c_int
            {
                ((0 as libc::c_int) < scale_factor + 0 as libc::c_int) as libc::c_int
            } else {
                ((-(1 as libc::c_int) - 0 as libc::c_int)
                    < scale_factor - 1 as libc::c_int) as libc::c_int
            }
        } else {
            (0 as libc::c_int as libc::c_long / *x < scale_factor as libc::c_long)
                as libc::c_int
        }
    } else {
        ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(scale_factor as libc::c_ulonglong) < *x as libc::c_ulonglong)
            as libc::c_int
    } != 0
    {
        scaled = (*x as libc::c_ulonglong)
            .wrapping_mul(scale_factor as libc::c_ulonglong) as libc::c_long;
        1 as libc::c_int
    } else {
        scaled = (*x as libc::c_ulonglong)
            .wrapping_mul(scale_factor as libc::c_ulonglong) as libc::c_long;
        0 as libc::c_int
    } != 0
    {
        *x = if *x < 0 as libc::c_int as libc::c_long {
            !if (0 as libc::c_int as libc::c_long) < -(1 as libc::c_int) as libc::c_long
            {
                -(1 as libc::c_int) as libc::c_long
            } else {
                (((1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }
        } else if (0 as libc::c_int as libc::c_long)
            < -(1 as libc::c_int) as libc::c_long
        {
            -(1 as libc::c_int) as libc::c_long
        } else {
            (((1 as libc::c_int as libc::c_long)
                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        };
        return LONGINT_OVERFLOW;
    }
    *x = scaled;
    return LONGINT_OK;
}
unsafe extern "C" fn bkm_scale_by_power(
    mut x: *mut libc::c_long,
    mut base: libc::c_int,
    mut power: libc::c_int,
) -> strtol_error {
    let mut err: strtol_error = LONGINT_OK;
    loop {
        let fresh0 = power;
        power = power - 1;
        if !(fresh0 != 0) {
            break;
        }
        err = ::std::mem::transmute::<
            libc::c_uint,
            strtol_error,
        >(err as libc::c_uint | bkm_scale(x, base) as libc::c_uint);
    }
    return err;
}
pub unsafe extern "C" fn xstrtol(
    mut s: *const libc::c_char,
    mut ptr: *mut *mut libc::c_char,
    mut strtol_base: libc::c_int,
    mut val: *mut libc::c_long,
    mut valid_suffixes: *const libc::c_char,
) -> strtol_error {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: libc::c_long = 0;
    let mut err: strtol_error = LONGINT_OK;
    if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
        __assert_fail(
            b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                as *const libc::c_char,
            b"xstrtol.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"strtol_error xstrtol(const char *, char **, int, long *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11669: {
        if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
            __assert_fail(
                b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                    as *const libc::c_char,
                b"xstrtol.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"strtol_error xstrtol(const char *, char **, int, long *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    p = if !ptr.is_null() { ptr } else { &mut t_ptr };
    *__errno_location() = 0 as libc::c_int;
    if (0 as libc::c_int as libc::c_long) < -(1 as libc::c_int) as libc::c_long {
        let mut q: *const libc::c_char = s;
        let mut ch: libc::c_uchar = *q as libc::c_uchar;
        while *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            q = q.offset(1);
            ch = *q as libc::c_uchar;
        }
        if ch as libc::c_int == '-' as i32 {
            return LONGINT_INVALID;
        }
    }
    tmp = strtol(s, p, strtol_base);
    if *p == s as *mut libc::c_char {
        if !valid_suffixes.is_null() && **p as libc::c_int != 0
            && !(strchr(valid_suffixes, **p as libc::c_int)).is_null()
        {
            tmp = 1 as libc::c_int as libc::c_long;
        } else {
            return LONGINT_INVALID
        }
    } else if *__errno_location() != 0 as libc::c_int {
        if *__errno_location() != 34 as libc::c_int {
            return LONGINT_INVALID;
        }
        err = LONGINT_OVERFLOW;
    }
    if valid_suffixes.is_null() {
        *val = tmp;
        return err;
    }
    if **p as libc::c_int != '\0' as i32 {
        let mut base: libc::c_int = 1024 as libc::c_int;
        let mut suffixes: libc::c_int = 1 as libc::c_int;
        let mut overflow: strtol_error = LONGINT_OK;
        if (strchr(valid_suffixes, **p as libc::c_int)).is_null() {
            *val = tmp;
            return (err as libc::c_uint
                | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint)
                as strtol_error;
        }
        match **p as libc::c_int {
            69 | 71 | 103 | 107 | 75 | 77 | 109 | 80 | 84 | 116 | 89 | 90 => {
                if !(strchr(valid_suffixes, '0' as i32)).is_null() {
                    match *(*p.offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int
                    {
                        105 => {
                            if *(*p.offset(0 as libc::c_int as isize))
                                .offset(2 as libc::c_int as isize) as libc::c_int
                                == 'B' as i32
                            {
                                suffixes += 2 as libc::c_int;
                            }
                        }
                        66 | 68 => {
                            base = 1000 as libc::c_int;
                            suffixes += 1;
                            suffixes;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        match **p as libc::c_int {
            98 => {
                overflow = bkm_scale(&mut tmp, 512 as libc::c_int);
            }
            66 => {
                overflow = bkm_scale(&mut tmp, 1024 as libc::c_int);
            }
            99 => {
                overflow = LONGINT_OK;
            }
            69 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 6 as libc::c_int);
            }
            71 | 103 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 3 as libc::c_int);
            }
            107 | 75 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 1 as libc::c_int);
            }
            77 | 109 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 2 as libc::c_int);
            }
            80 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 5 as libc::c_int);
            }
            84 | 116 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 4 as libc::c_int);
            }
            119 => {
                overflow = bkm_scale(&mut tmp, 2 as libc::c_int);
            }
            89 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 8 as libc::c_int);
            }
            90 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 7 as libc::c_int);
            }
            _ => {
                *val = tmp;
                return (err as libc::c_uint
                    | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint)
                    as strtol_error;
            }
        }
        err = ::std::mem::transmute::<
            libc::c_uint,
            strtol_error,
        >(err as libc::c_uint | overflow as libc::c_uint);
        *p = (*p).offset(suffixes as isize);
        if **p != 0 {
            err = ::std::mem::transmute::<
                libc::c_uint,
                strtol_error,
            >(
                err as libc::c_uint
                    | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint,
            );
        }
    }
    *val = tmp;
    return err;
}
