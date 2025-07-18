use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base32_decode_context {
    pub i: libc::c_int,
    pub buf: [libc::c_char; 8],
}
#[inline]
unsafe extern "C" fn imalloc(mut s: idx_t) -> *mut libc::c_void {
    return if s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong {
        malloc(s as libc::c_ulong)
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
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
pub unsafe extern "C" fn base32_encode(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: idx_t,
) {
    static mut b32str: [libc::c_char; 32] = unsafe {
        *::std::mem::transmute::<
            &[u8; 32],
            &[libc::c_char; 32],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567")
    };
    while inlen != 0 && outlen != 0 {
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = b32str[(to_uchar(*in_0.offset(0 as libc::c_int as isize))
            as libc::c_int >> 3 as libc::c_int & 0x1f as libc::c_int) as usize];
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        inlen -= 1;
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 = b32str[(((to_uchar(*in_0.offset(0 as libc::c_int as isize))
            as libc::c_int) << 2 as libc::c_int)
            + (if inlen != 0 {
                to_uchar(*in_0.offset(1 as libc::c_int as isize)) as libc::c_int
                    >> 6 as libc::c_int
            } else {
                0 as libc::c_int
            }) & 0x1f as libc::c_int) as usize];
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh2 = out;
        out = out.offset(1);
        *fresh2 = (if inlen != 0 {
            b32str[(to_uchar(*in_0.offset(1 as libc::c_int as isize)) as libc::c_int
                >> 1 as libc::c_int & 0x1f as libc::c_int) as usize] as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh3 = out;
        out = out.offset(1);
        *fresh3 = (if inlen != 0 {
            inlen -= 1;
            b32str[(((to_uchar(*in_0.offset(1 as libc::c_int as isize)) as libc::c_int)
                << 4 as libc::c_int)
                + (if inlen != 0 {
                    to_uchar(*in_0.offset(2 as libc::c_int as isize)) as libc::c_int
                        >> 4 as libc::c_int
                } else {
                    0 as libc::c_int
                }) & 0x1f as libc::c_int) as usize] as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh4 = out;
        out = out.offset(1);
        *fresh4 = (if inlen != 0 {
            inlen -= 1;
            b32str[(((to_uchar(*in_0.offset(2 as libc::c_int as isize)) as libc::c_int)
                << 1 as libc::c_int)
                + (if inlen != 0 {
                    to_uchar(*in_0.offset(3 as libc::c_int as isize)) as libc::c_int
                        >> 7 as libc::c_int
                } else {
                    0 as libc::c_int
                }) & 0x1f as libc::c_int) as usize] as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh5 = out;
        out = out.offset(1);
        *fresh5 = (if inlen != 0 {
            b32str[(to_uchar(*in_0.offset(3 as libc::c_int as isize)) as libc::c_int
                >> 2 as libc::c_int & 0x1f as libc::c_int) as usize] as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh6 = out;
        out = out.offset(1);
        *fresh6 = (if inlen != 0 {
            inlen -= 1;
            b32str[(((to_uchar(*in_0.offset(3 as libc::c_int as isize)) as libc::c_int)
                << 3 as libc::c_int)
                + (if inlen != 0 {
                    to_uchar(*in_0.offset(4 as libc::c_int as isize)) as libc::c_int
                        >> 5 as libc::c_int
                } else {
                    0 as libc::c_int
                }) & 0x1f as libc::c_int) as usize] as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh7 = out;
        out = out.offset(1);
        *fresh7 = (if inlen != 0 {
            b32str[(to_uchar(*in_0.offset(4 as libc::c_int as isize)) as libc::c_int
                & 0x1f as libc::c_int) as usize] as libc::c_int
        } else {
            '=' as i32
        }) as libc::c_char;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        if inlen != 0 {
            inlen -= 1;
            inlen;
        }
        if inlen != 0 {
            in_0 = in_0.offset(5 as libc::c_int as isize);
        }
    }
    if outlen != 0 {
        *out = '\0' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn base32_encode_alloc(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut *mut libc::c_char,
) -> idx_t {
    let mut in_over_5: idx_t = inlen / 5 as libc::c_int as libc::c_long
        + (inlen % 5 as libc::c_int as libc::c_long != 0 as libc::c_int as libc::c_long)
            as libc::c_int as libc::c_long;
    let mut outlen: idx_t = 0;
    if (if ::std::mem::size_of::<idx_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t) {
            if (if (8 as libc::c_int) < 0 as libc::c_int {
                if in_over_5 < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int
                        }) + 8 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (in_over_5
                            < (127 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((8 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 127 as libc::c_int
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            127 as libc::c_int / -(8 as libc::c_int)
                        }) as libc::c_long
                            <= -(1 as libc::c_int) as libc::c_long - in_over_5)
                            as libc::c_int
                    }
                } else {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
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
                                8 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
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
                                        8 as libc::c_int
                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            in_over_5
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < in_over_5
                                    + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < in_over_5
                                && ((-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_long)
                                    < in_over_5 - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        ((((-(127 as libc::c_int) - 1 as libc::c_int) / 8 as libc::c_int)
                            as libc::c_long) < in_over_5) as libc::c_int
                    }
                }
            } else {
                if 8 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    if in_over_5 < 0 as libc::c_int as libc::c_long {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                in_over_5
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
                                    in_over_5
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
                                    in_over_5
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                in_over_5
                            })
                                + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
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
                                            in_over_5
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
                                            in_over_5
                                        })
                                            + (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
                                })
                                    + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                        }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((0 as libc::c_int)
                                    < 8 as libc::c_int
                                        + (-(127 as libc::c_int) - 1 as libc::c_int)) as libc::c_int
                            } else {
                                (-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int)
                                    < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                            }
                        } else {
                            ((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                / in_over_5 < 8 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        (((127 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                            < in_over_5) as libc::c_int
                    }
                }
            }) != 0
            {
                outlen = (in_over_5 as libc::c_uint)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_schar
                    as idx_t;
                1 as libc::c_int
            } else {
                outlen = (in_over_5 as libc::c_uint)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_schar
                    as idx_t;
                0 as libc::c_int
            }
        } else {
            if (if (8 as libc::c_int) < 0 as libc::c_int {
                if in_over_5 < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        }) + 8 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (in_over_5
                            < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                / 8 as libc::c_int) as libc::c_long) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((8 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            })
                                + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                / -(8 as libc::c_int)
                        }) as libc::c_long
                            <= -(1 as libc::c_int) as libc::c_long - in_over_5)
                            as libc::c_int
                    }
                } else {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 0 as libc::c_int
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
                                8 as libc::c_int
                            }) + 0 as libc::c_int
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }) + 0 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
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
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 0 as libc::c_int) as libc::c_int
                    }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            in_over_5
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < in_over_5 + 0 as libc::c_int as libc::c_long)
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < in_over_5
                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    as libc::c_long)
                                    < in_over_5 - 1 as libc::c_int as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        (((0 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                            < in_over_5) as libc::c_int
                    }
                }
            } else {
                if 8 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    if in_over_5 < 0 as libc::c_int as libc::c_long {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                in_over_5
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
                                    in_over_5
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
                                    in_over_5
                                }) + 0 as libc::c_int as libc::c_long
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                in_over_5
                            }) + 0 as libc::c_int as libc::c_long)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
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
                                            in_over_5
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
                                            in_over_5
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
                                }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                        }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((0 as libc::c_int) < 8 as libc::c_int + 0 as libc::c_int)
                                    as libc::c_int
                            } else {
                                ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                            }
                        } else {
                            (0 as libc::c_int as libc::c_long / in_over_5
                                < 8 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        ((((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            / 8 as libc::c_int) as libc::c_long) < in_over_5)
                            as libc::c_int
                    }
                }
            }) != 0
            {
                outlen = (in_over_5 as libc::c_uint)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_uchar
                    as idx_t;
                1 as libc::c_int
            } else {
                outlen = (in_over_5 as libc::c_uint)
                    .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_uchar
                    as idx_t;
                0 as libc::c_int
            }
        }
    } else {
        if ::std::mem::size_of::<idx_t>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if !((0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t) {
                if (if (8 as libc::c_int) < 0 as libc::c_int {
                    if in_over_5 < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int
                            }) + 8 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            (in_over_5
                                < (32767 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((8 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 32767 as libc::c_int
                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                32767 as libc::c_int / -(8 as libc::c_int)
                            }) as libc::c_long
                                <= -(1 as libc::c_int) as libc::c_long - in_over_5)
                                as libc::c_int
                        }
                    } else {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
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
                                    8 as libc::c_int
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
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
                                            8 as libc::c_int
                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_int
                        }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                in_over_5
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < in_over_5
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < in_over_5
                                    && ((-(1 as libc::c_int)
                                        - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                        as libc::c_long)
                                        < in_over_5 - 1 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            }
                        } else {
                            ((((-(32767 as libc::c_int) - 1 as libc::c_int)
                                / 8 as libc::c_int) as libc::c_long) < in_over_5)
                                as libc::c_int
                        }
                    }
                } else {
                    if 8 as libc::c_int == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        if in_over_5 < 0 as libc::c_int as libc::c_long {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
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
                                        in_over_5
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
                                        in_over_5
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
                                })
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
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
                                                in_over_5
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
                                                in_over_5
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
                                        in_over_5
                                    })
                                        + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long) as libc::c_int
                            }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((0 as libc::c_int)
                                        < 8 as libc::c_int
                                            + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                        as libc::c_int
                                } else {
                                    (-(1 as libc::c_int)
                                        - (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                }
                            } else {
                                ((-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long / in_over_5
                                    < 8 as libc::c_int as libc::c_long) as libc::c_int
                            }
                        } else {
                            (((32767 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                                < in_over_5) as libc::c_int
                        }
                    }
                }) != 0
                {
                    outlen = (in_over_5 as libc::c_uint)
                        .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_short
                        as idx_t;
                    1 as libc::c_int
                } else {
                    outlen = (in_over_5 as libc::c_uint)
                        .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_short
                        as idx_t;
                    0 as libc::c_int
                }
            } else {
                if (if (8 as libc::c_int) < 0 as libc::c_int {
                    if in_over_5 < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                            }) + 8 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            (in_over_5
                                < ((32767 as libc::c_int * 2 as libc::c_int
                                    + 1 as libc::c_int) / 8 as libc::c_int) as libc::c_long)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((8 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                })
                                    + (32767 as libc::c_int * 2 as libc::c_int
                                        + 1 as libc::c_int)
                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                    / -(8 as libc::c_int)
                            }) as libc::c_long
                                <= -(1 as libc::c_int) as libc::c_long - in_over_5)
                                as libc::c_int
                        }
                    } else {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 0 as libc::c_int
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
                                    8 as libc::c_int
                                }) + 0 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                8 as libc::c_int
                            }) + 0 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
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
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 0 as libc::c_int) as libc::c_int
                        }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                in_over_5
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < in_over_5 + 0 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < in_over_5
                                    && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                        as libc::c_long)
                                        < in_over_5 - 1 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            }
                        } else {
                            (((0 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                                < in_over_5) as libc::c_int
                        }
                    }
                } else {
                    if 8 as libc::c_int == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        if in_over_5 < 0 as libc::c_int as libc::c_long {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
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
                                        in_over_5
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
                                        in_over_5
                                    }) + 0 as libc::c_int as libc::c_long
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
                                }) + 0 as libc::c_int as libc::c_long)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
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
                                                in_over_5
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
                                                in_over_5
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
                                    }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                            }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((0 as libc::c_int) < 8 as libc::c_int + 0 as libc::c_int)
                                        as libc::c_int
                                } else {
                                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                                        < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                }
                            } else {
                                (0 as libc::c_int as libc::c_long / in_over_5
                                    < 8 as libc::c_int as libc::c_long) as libc::c_int
                            }
                        } else {
                            ((((32767 as libc::c_int * 2 as libc::c_int
                                + 1 as libc::c_int) / 8 as libc::c_int) as libc::c_long)
                                < in_over_5) as libc::c_int
                        }
                    }
                }) != 0
                {
                    outlen = (in_over_5 as libc::c_uint)
                        .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_ushort
                        as idx_t;
                    1 as libc::c_int
                } else {
                    outlen = (in_over_5 as libc::c_uint)
                        .wrapping_mul(8 as libc::c_int as libc::c_uint) as libc::c_ushort
                        as idx_t;
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
                    outlen
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    if (if (8 as libc::c_int) < 0 as libc::c_int {
                        if in_over_5 < 0 as libc::c_int as libc::c_long {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2147483647 as libc::c_int
                                }) + 8 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                (in_over_5
                                    < (2147483647 as libc::c_int / 8 as libc::c_int)
                                        as libc::c_long) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    ((8 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 2147483647 as libc::c_int
                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    2147483647 as libc::c_int / -(8 as libc::c_int)
                                }) as libc::c_long
                                    <= -(1 as libc::c_int) as libc::c_long - in_over_5)
                                    as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
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
                                        8 as libc::c_int
                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
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
                                                8 as libc::c_int
                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_int
                            }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < in_over_5
                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < in_over_5
                                        && ((-(1 as libc::c_int)
                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_long)
                                            < in_over_5 - 1 as libc::c_int as libc::c_long)
                                        as libc::c_int
                                }
                            } else {
                                ((((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    / 8 as libc::c_int) as libc::c_long) < in_over_5)
                                    as libc::c_int
                            }
                        }
                    } else {
                        if 8 as libc::c_int == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if in_over_5 < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
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
                                            in_over_5
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
                                            in_over_5
                                        })
                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
                                    })
                                        + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                in_over_5
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
                                                    in_over_5
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
                                                    in_over_5
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
                                            in_over_5
                                        })
                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_long) as libc::c_int
                                }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((0 as libc::c_int)
                                            < 8 as libc::c_int
                                                + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_int
                                    } else {
                                        (-(1 as libc::c_int)
                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    ((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long / in_over_5
                                        < 8 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                (((2147483647 as libc::c_int / 8 as libc::c_int)
                                    as libc::c_long) < in_over_5) as libc::c_int
                            }
                        }
                    }) != 0
                    {
                        outlen = (in_over_5 as libc::c_uint)
                            .wrapping_mul(8 as libc::c_int as libc::c_uint)
                            as libc::c_int as idx_t;
                        1 as libc::c_int
                    } else {
                        outlen = (in_over_5 as libc::c_uint)
                            .wrapping_mul(8 as libc::c_int as libc::c_uint)
                            as libc::c_int as idx_t;
                        0 as libc::c_int
                    }
                } else {
                    if (if (8 as libc::c_int) < 0 as libc::c_int {
                        if in_over_5 < 0 as libc::c_int as libc::c_long {
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
                                    .wrapping_add(8 as libc::c_int as libc::c_uint)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                < 0 as libc::c_int as libc::c_uint
                            {
                                (in_over_5
                                    < (2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint)
                                        .wrapping_div(8 as libc::c_int as libc::c_uint)
                                        as libc::c_long) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    ((8 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                                }) != 0
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
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
                                        .wrapping_div(-(8 as libc::c_int) as libc::c_uint)
                                }) as libc::c_long
                                    <= -(1 as libc::c_int) as libc::c_long - in_over_5)
                                    as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
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
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    8 as libc::c_int
                                }) + 0 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
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
                                                8 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int) as libc::c_int
                            }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    in_over_5
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < in_over_5 + 0 as libc::c_int as libc::c_long)
                                        as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < in_over_5
                                        && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                            as libc::c_long)
                                            < in_over_5 - 1 as libc::c_int as libc::c_long)
                                        as libc::c_int
                                }
                            } else {
                                (((0 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                                    < in_over_5) as libc::c_int
                            }
                        }
                    } else {
                        if 8 as libc::c_int == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if in_over_5 < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
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
                                            in_over_5
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
                                            in_over_5
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
                                    }) + 0 as libc::c_int as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                in_over_5
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
                                                    in_over_5
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
                                                    in_over_5
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
                                        }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((0 as libc::c_int) < 8 as libc::c_int + 0 as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                                            < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as libc::c_long / in_over_5
                                        < 8 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                (((2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint)
                                    .wrapping_div(8 as libc::c_int as libc::c_uint)
                                    as libc::c_long) < in_over_5) as libc::c_int
                            }
                        }
                    }) != 0
                    {
                        outlen = (in_over_5 as libc::c_uint)
                            .wrapping_mul(8 as libc::c_int as libc::c_uint) as idx_t;
                        1 as libc::c_int
                    } else {
                        outlen = (in_over_5 as libc::c_uint)
                            .wrapping_mul(8 as libc::c_int as libc::c_uint) as idx_t;
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
                        outlen
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        if (if (8 as libc::c_int) < 0 as libc::c_int {
                            if in_over_5 < 0 as libc::c_int as libc::c_long {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        9223372036854775807 as libc::c_long
                                    }) + 8 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (in_over_5
                                        < 9223372036854775807 as libc::c_long
                                            / 8 as libc::c_int as libc::c_long) as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((8 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) as libc::c_long + 9223372036854775807 as libc::c_long
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        9223372036854775807 as libc::c_long
                                            / -(8 as libc::c_int) as libc::c_long
                                    }) <= -(1 as libc::c_int) as libc::c_long - in_over_5)
                                        as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) as libc::c_long
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
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
                                            8 as libc::c_int
                                        }) as libc::c_long
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) as libc::c_long
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
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
                                                    8 as libc::c_int
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
                                                    8 as libc::c_int
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
                                            8 as libc::c_int
                                        }) as libc::c_long
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)) as libc::c_int
                                }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < in_over_5
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < in_over_5
                                            && -(1 as libc::c_int) as libc::c_long
                                                - (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                                < in_over_5 - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (((-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long) / 8 as libc::c_int as libc::c_long)
                                        < in_over_5) as libc::c_int
                                }
                            }
                        } else {
                            if 8 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if in_over_5 < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
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
                                                in_over_5
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
                                                in_over_5
                                            })
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
                                        })
                                            + (-(9223372036854775807 as libc::c_long)
                                                - 1 as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    in_over_5
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
                                                        in_over_5
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
                                                        in_over_5
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
                                                in_over_5
                                            })
                                                + (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)) as libc::c_int
                                    }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int as libc::c_long)
                                                < 8 as libc::c_int as libc::c_long
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)) as libc::c_int
                                        } else {
                                            (-(1 as libc::c_int) as libc::c_long
                                                - (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)
                                                < (8 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                                as libc::c_int
                                        }
                                    } else {
                                        ((-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long) / in_over_5
                                            < 8 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_long
                                        / 8 as libc::c_int as libc::c_long) < in_over_5)
                                        as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            outlen = (in_over_5 as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                as libc::c_long;
                            1 as libc::c_int
                        } else {
                            outlen = (in_over_5 as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                as libc::c_long;
                            0 as libc::c_int
                        }
                    } else {
                        if (if (8 as libc::c_int) < 0 as libc::c_int {
                            if in_over_5 < 0 as libc::c_int as libc::c_long {
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
                                        .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((in_over_5 as libc::c_ulong)
                                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_ulong)
                                            .wrapping_div(8 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((8 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
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
                                            .wrapping_div(-(8 as libc::c_int) as libc::c_ulong)
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_long - in_over_5)
                                            as libc::c_ulong) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
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
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
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
                                                    8 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int) as libc::c_int
                                }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < in_over_5 + 0 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < in_over_5
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_long)
                                                < in_over_5 - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (((0 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                                        < in_over_5) as libc::c_int
                                }
                            }
                        } else {
                            if 8 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if in_over_5 < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
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
                                                in_over_5
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
                                                in_over_5
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
                                        }) + 0 as libc::c_int as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    in_over_5
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
                                                        in_over_5
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
                                                        in_over_5
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                in_over_5
                                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int) < 8 as libc::c_int + 0 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int as libc::c_long / in_over_5
                                            < 8 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_long as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_ulong)
                                        .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                        < in_over_5 as libc::c_ulong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            outlen = (in_over_5 as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as idx_t;
                            1 as libc::c_int
                        } else {
                            outlen = (in_over_5 as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as idx_t;
                            0 as libc::c_int
                        }
                    }
                } else {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        outlen
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        if (if (8 as libc::c_int) < 0 as libc::c_int {
                            if in_over_5 < 0 as libc::c_int as libc::c_long {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_longlong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_longlong
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                    }) + 8 as libc::c_int as libc::c_longlong
                                }) - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    ((in_over_5 as libc::c_longlong)
                                        < 9223372036854775807 as libc::c_longlong
                                            / 8 as libc::c_int as libc::c_longlong) as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((8 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) as libc::c_longlong
                                            + 9223372036854775807 as libc::c_longlong
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                            / -(8 as libc::c_int) as libc::c_longlong
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_long - in_over_5)
                                            as libc::c_longlong) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_longlong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
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
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
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
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) + 0 as libc::c_int as libc::c_longlong
                                }) < 0 as libc::c_int as libc::c_longlong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) as libc::c_longlong
                                        + (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_longlong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
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
                                                    8 as libc::c_int
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
                                                    8 as libc::c_int
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
                                            8 as libc::c_int
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)) as libc::c_int
                                }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_longlong)
                                            < in_over_5 as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < in_over_5
                                            && -(1 as libc::c_int) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (in_over_5 - 1 as libc::c_int as libc::c_long)
                                                    as libc::c_longlong) as libc::c_int
                                    }
                                } else {
                                    (((-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                                        / 8 as libc::c_int as libc::c_longlong)
                                        < in_over_5 as libc::c_longlong) as libc::c_int
                                }
                            }
                        } else {
                            if 8 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if in_over_5 < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_longlong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
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
                                                in_over_5
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
                                                in_over_5
                                            }) as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) + 0 as libc::c_int as libc::c_longlong
                                    }) < 0 as libc::c_int as libc::c_longlong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
                                        }) as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    in_over_5
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
                                                        in_over_5
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
                                                        in_over_5
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
                                                in_over_5
                                            }) as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as libc::c_int
                                    }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int as libc::c_longlong)
                                                < 8 as libc::c_int as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)) as libc::c_int
                                        } else {
                                            (-(1 as libc::c_int) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (8 as libc::c_int - 1 as libc::c_int) as libc::c_longlong)
                                                as libc::c_int
                                        }
                                    } else {
                                        (((-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong) / in_over_5 as libc::c_longlong)
                                            < 8 as libc::c_int as libc::c_longlong) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        / 8 as libc::c_int as libc::c_longlong)
                                        < in_over_5 as libc::c_longlong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            1 as libc::c_int
                        } else {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            0 as libc::c_int
                        }
                    } else {
                        if (if (8 as libc::c_int) < 0 as libc::c_int {
                            if in_over_5 < 0 as libc::c_int as libc::c_long {
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
                                        .wrapping_add(8 as libc::c_int as libc::c_ulonglong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                    < 0 as libc::c_int as libc::c_ulonglong
                                {
                                    ((in_over_5 as libc::c_ulonglong)
                                        < (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(8 as libc::c_int as libc::c_ulonglong))
                                        as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((8 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 8 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
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
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(-(8 as libc::c_int) as libc::c_ulonglong)
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_long - in_over_5)
                                            as libc::c_ulonglong) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
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
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        8 as libc::c_int
                                    }) + 0 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                8 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    8 as libc::c_int
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
                                                    8 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) + 0 as libc::c_int) as libc::c_int
                                }) != 0 && 8 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        in_over_5
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < in_over_5 + 0 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < in_over_5
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_long)
                                                < in_over_5 - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (((0 as libc::c_int / 8 as libc::c_int) as libc::c_long)
                                        < in_over_5) as libc::c_int
                                }
                            }
                        } else {
                            if 8 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if in_over_5 < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
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
                                                in_over_5
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
                                                in_over_5
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            in_over_5
                                        }) + 0 as libc::c_int as libc::c_long)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    in_over_5
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
                                                        in_over_5
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
                                                        in_over_5
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                in_over_5
                                            }) + 0 as libc::c_int as libc::c_long) as libc::c_int
                                    }) != 0 && in_over_5 == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            8 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int) < 8 as libc::c_int + 0 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                < 8 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int as libc::c_long / in_over_5
                                            < 8 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        as libc::c_ulonglong)
                                        .wrapping_mul(2 as libc::c_ulonglong)
                                        .wrapping_add(1 as libc::c_ulonglong)
                                        .wrapping_div(8 as libc::c_int as libc::c_ulonglong)
                                        < in_over_5 as libc::c_ulonglong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulonglong)
                                as idx_t;
                            1 as libc::c_int
                        } else {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulonglong)
                                as idx_t;
                            0 as libc::c_int
                        }
                    }
                }
            }
        }
    }) != 0 || inlen < 0 as libc::c_int as libc::c_long
    {
        *out = 0 as *mut libc::c_char;
        return 0 as libc::c_int as idx_t;
    }
    outlen += 1;
    outlen;
    *out = imalloc(outlen) as *mut libc::c_char;
    if (*out).is_null() {
        return outlen;
    }
    base32_encode(in_0, inlen, *out, outlen);
    return outlen - 1 as libc::c_int as libc::c_long;
}
static mut b32: [libc::c_schar; 256] = [0; 256];
pub unsafe extern "C" fn isbase32(mut ch: libc::c_char) -> bool {
    return 1 as libc::c_int != 0
        && 0 as libc::c_int <= b32[to_uchar(ch) as usize] as libc::c_int;
}
pub unsafe extern "C" fn base32_decode_ctx_init(mut ctx: *mut base32_decode_context) {
    (*ctx).i = 0 as libc::c_int;
}
unsafe extern "C" fn get_8(
    mut ctx: *mut base32_decode_context,
    mut in_0: *mut *const libc::c_char,
    mut in_end: *const libc::c_char,
    mut n_non_newline: *mut idx_t,
) -> *mut libc::c_char {
    if (*ctx).i == 8 as libc::c_int {
        (*ctx).i = 0 as libc::c_int;
    }
    if (*ctx).i == 0 as libc::c_int {
        let mut t: *const libc::c_char = *in_0;
        if 8 as libc::c_int as libc::c_long <= in_end.offset_from(*in_0) as libc::c_long
            && (memchr(
                t as *const libc::c_void,
                '\n' as i32,
                8 as libc::c_int as libc::c_ulong,
            ))
                .is_null()
        {
            *in_0 = (*in_0).offset(8 as libc::c_int as isize);
            *n_non_newline = 8 as libc::c_int as idx_t;
            return t as *mut libc::c_char;
        }
    }
    let mut p: *const libc::c_char = *in_0;
    while p < in_end {
        let fresh8 = p;
        p = p.offset(1);
        let mut c: libc::c_char = *fresh8;
        if !(c as libc::c_int != '\n' as i32) {
            continue;
        }
        let fresh9 = (*ctx).i;
        (*ctx).i = (*ctx).i + 1;
        (*ctx).buf[fresh9 as usize] = c;
        if (*ctx).i == 8 as libc::c_int {
            break;
        }
    }
    *in_0 = p;
    *n_non_newline = (*ctx).i as idx_t;
    return ((*ctx).buf).as_mut_ptr();
}
unsafe extern "C" fn decode_8(
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut outp: *mut *mut libc::c_char,
    mut outleft: *mut idx_t,
) -> bool {
    let mut out: *mut libc::c_char = *outp;
    if inlen < 8 as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    if !isbase32(*in_0.offset(0 as libc::c_int as isize))
        || !isbase32(*in_0.offset(1 as libc::c_int as isize))
    {
        return 0 as libc::c_int != 0;
    }
    if *outleft != 0 {
        let fresh10 = out;
        out = out.offset(1);
        *fresh10 = ((b32[to_uchar(*in_0.offset(0 as libc::c_int as isize)) as usize]
            as libc::c_int) << 3 as libc::c_int
            | b32[to_uchar(*in_0.offset(1 as libc::c_int as isize)) as usize]
                as libc::c_int >> 2 as libc::c_int) as libc::c_char;
        *outleft -= 1;
        *outleft;
    }
    if *in_0.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32 {
        if *in_0.offset(3 as libc::c_int as isize) as libc::c_int != '=' as i32
            || *in_0.offset(4 as libc::c_int as isize) as libc::c_int != '=' as i32
            || *in_0.offset(5 as libc::c_int as isize) as libc::c_int != '=' as i32
            || *in_0.offset(6 as libc::c_int as isize) as libc::c_int != '=' as i32
            || *in_0.offset(7 as libc::c_int as isize) as libc::c_int != '=' as i32
        {
            *outp = out;
            return 0 as libc::c_int != 0;
        }
    } else {
        if !isbase32(*in_0.offset(2 as libc::c_int as isize))
            || !isbase32(*in_0.offset(3 as libc::c_int as isize))
        {
            *outp = out;
            return 0 as libc::c_int != 0;
        }
        if *outleft != 0 {
            let fresh11 = out;
            out = out.offset(1);
            *fresh11 = ((b32[to_uchar(*in_0.offset(1 as libc::c_int as isize)) as usize]
                as libc::c_int) << 6 as libc::c_int
                | (b32[to_uchar(*in_0.offset(2 as libc::c_int as isize)) as usize]
                    as libc::c_int) << 1 as libc::c_int
                | b32[to_uchar(*in_0.offset(3 as libc::c_int as isize)) as usize]
                    as libc::c_int >> 4 as libc::c_int) as libc::c_char;
            *outleft -= 1;
            *outleft;
        }
        if *in_0.offset(4 as libc::c_int as isize) as libc::c_int == '=' as i32 {
            if *in_0.offset(5 as libc::c_int as isize) as libc::c_int != '=' as i32
                || *in_0.offset(6 as libc::c_int as isize) as libc::c_int != '=' as i32
                || *in_0.offset(7 as libc::c_int as isize) as libc::c_int != '=' as i32
            {
                *outp = out;
                return 0 as libc::c_int != 0;
            }
        } else {
            if !isbase32(*in_0.offset(4 as libc::c_int as isize)) {
                *outp = out;
                return 0 as libc::c_int != 0;
            }
            if *outleft != 0 {
                let fresh12 = out;
                out = out.offset(1);
                *fresh12 = ((b32[to_uchar(*in_0.offset(3 as libc::c_int as isize))
                    as usize] as libc::c_int) << 4 as libc::c_int
                    | b32[to_uchar(*in_0.offset(4 as libc::c_int as isize)) as usize]
                        as libc::c_int >> 1 as libc::c_int) as libc::c_char;
                *outleft -= 1;
                *outleft;
            }
            if *in_0.offset(5 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                if *in_0.offset(6 as libc::c_int as isize) as libc::c_int != '=' as i32
                    || *in_0.offset(7 as libc::c_int as isize) as libc::c_int
                        != '=' as i32
                {
                    *outp = out;
                    return 0 as libc::c_int != 0;
                }
            } else {
                if !isbase32(*in_0.offset(5 as libc::c_int as isize))
                    || !isbase32(*in_0.offset(6 as libc::c_int as isize))
                {
                    *outp = out;
                    return 0 as libc::c_int != 0;
                }
                if *outleft != 0 {
                    let fresh13 = out;
                    out = out.offset(1);
                    *fresh13 = ((b32[to_uchar(*in_0.offset(4 as libc::c_int as isize))
                        as usize] as libc::c_int) << 7 as libc::c_int
                        | (b32[to_uchar(*in_0.offset(5 as libc::c_int as isize))
                            as usize] as libc::c_int) << 2 as libc::c_int
                        | b32[to_uchar(*in_0.offset(6 as libc::c_int as isize)) as usize]
                            as libc::c_int >> 3 as libc::c_int) as libc::c_char;
                    *outleft -= 1;
                    *outleft;
                }
                if *in_0.offset(7 as libc::c_int as isize) as libc::c_int != '=' as i32 {
                    if !isbase32(*in_0.offset(7 as libc::c_int as isize)) {
                        *outp = out;
                        return 0 as libc::c_int != 0;
                    }
                    if *outleft != 0 {
                        let fresh14 = out;
                        out = out.offset(1);
                        *fresh14 = ((b32[to_uchar(
                            *in_0.offset(6 as libc::c_int as isize),
                        ) as usize] as libc::c_int) << 5 as libc::c_int
                            | b32[to_uchar(*in_0.offset(7 as libc::c_int as isize))
                                as usize] as libc::c_int) as libc::c_char;
                        *outleft -= 1;
                        *outleft;
                    }
                }
            }
        }
    }
    *outp = out;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn base32_decode_ctx(
    mut ctx: *mut base32_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut outleft: idx_t = *outlen;
    let mut ignore_newlines: bool = !ctx.is_null();
    let mut flush_ctx: bool = 0 as libc::c_int != 0;
    let mut ctx_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if ignore_newlines {
        ctx_i = (*ctx).i as libc::c_uint;
        flush_ctx = inlen == 0 as libc::c_int as libc::c_long;
    }
    loop {
        let mut outleft_save: idx_t = outleft;
        if ctx_i == 0 as libc::c_int as libc::c_uint && !flush_ctx {
            loop {
                outleft_save = outleft;
                if !decode_8(in_0, inlen, &mut out, &mut outleft) {
                    break;
                }
                in_0 = in_0.offset(8 as libc::c_int as isize);
                inlen -= 8 as libc::c_int as libc::c_long;
            }
        }
        if inlen == 0 as libc::c_int as libc::c_long && !flush_ctx {
            break;
        }
        if inlen != 0 && *in_0 as libc::c_int == '\n' as i32
            && ignore_newlines as libc::c_int != 0
        {
            in_0 = in_0.offset(1);
            in_0;
            inlen -= 1;
            inlen;
        } else {
            out = out.offset(-((outleft_save - outleft) as isize));
            outleft = outleft_save;
            let mut in_end: *const libc::c_char = in_0.offset(inlen as isize);
            let mut non_nl: *const libc::c_char = 0 as *const libc::c_char;
            if ignore_newlines {
                non_nl = get_8(ctx, &mut in_0, in_end, &mut inlen);
            } else {
                non_nl = in_0;
            }
            if inlen == 0 as libc::c_int as libc::c_long
                || inlen < 8 as libc::c_int as libc::c_long && !flush_ctx
                    && ignore_newlines as libc::c_int != 0
            {
                inlen = 0 as libc::c_int as idx_t;
                break;
            } else {
                if !decode_8(non_nl, inlen, &mut out, &mut outleft) {
                    break;
                }
                inlen = in_end.offset_from(in_0) as libc::c_long;
            }
        }
    }
    *outlen -= outleft;
    return inlen == 0 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn base32_decode_alloc_ctx(
    mut ctx: *mut base32_decode_context,
    mut in_0: *const libc::c_char,
    mut inlen: idx_t,
    mut out: *mut *mut libc::c_char,
    mut outlen: *mut idx_t,
) -> bool {
    let mut needlen: idx_t = 5 as libc::c_int as libc::c_long
        * ((inlen >> 3 as libc::c_int) + 1 as libc::c_int as libc::c_long);
    *out = imalloc(needlen) as *mut libc::c_char;
    if (*out).is_null() {
        return 1 as libc::c_int != 0;
    }
    if !base32_decode_ctx(ctx, in_0, inlen, *out, &mut needlen) {
        rpl_free(*out as *mut libc::c_void);
        *out = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    }
    if !outlen.is_null() {
        *outlen = needlen;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn run_static_initializers() {
    b32 = [
        (if 0 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 0 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 0 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 0 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 0 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 0 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 0 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 0 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 0 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 0 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 0 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 0 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 0 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 0 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 0 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 0 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 0 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 0 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 0 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 0 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 0 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 0 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 0 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 0 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 0 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 0 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 0 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 0 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 0 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 0 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 0 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 0 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 1 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 1 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 1 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 1 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 1 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 1 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 1 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 1 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 1 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 1 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 1 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 1 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 1 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 1 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 1 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 1 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 1 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 1 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 1 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 1 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 1 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 1 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 1 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 1 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 1 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 1 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 1 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 1 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 1 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 1 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 1 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 1 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 2 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 2 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 2 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 2 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 2 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 2 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 2 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 2 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 2 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 2 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 2 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 2 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 2 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 2 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 2 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 2 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 2 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 2 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 2 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 2 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 2 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 2 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 2 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 2 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 2 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 2 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 2 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 2 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 2 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 2 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 2 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 2 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 3 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 3 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 3 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 3 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 3 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 3 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 3 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 3 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 3 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 3 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 3 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 3 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 3 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 3 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 3 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 3 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 3 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 3 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 3 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 3 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 3 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 3 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 3 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 3 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 3 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 3 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 3 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 3 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 3 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 3 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 3 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 3 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 4 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 4 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 4 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 4 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 4 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 4 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 4 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 4 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 4 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 4 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 4 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 4 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 4 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 4 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 4 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 4 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 4 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 4 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 4 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 4 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 4 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 4 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 4 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 4 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 4 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 4 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 4 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 4 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 4 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 4 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 4 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 4 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 5 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 5 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 5 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 5 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 5 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 5 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 5 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 5 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 5 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 5 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 5 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 5 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 5 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 5 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 5 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 5 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 5 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 5 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 5 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 5 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 5 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 5 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 5 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 5 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 5 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 5 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 5 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 5 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 5 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 5 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 5 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 5 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 6 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 6 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 6 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 6 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 6 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 6 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 6 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 6 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 6 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 6 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 6 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 6 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 6 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 6 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 6 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 6 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 6 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 6 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 6 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 6 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 6 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 6 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 6 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 6 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 6 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 6 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 6 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 6 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 6 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 6 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 6 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 6 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 7 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 7 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 7 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 7 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 7 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 7 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 7 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 7 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 7 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 7 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 7 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 7 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 7 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 7 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 7 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 7 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 7 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 7 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 7 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 7 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 7 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 7 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 7 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 7 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 7 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 7 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 7 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 7 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 7 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 7 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 7 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 7 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 8 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 8 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 8 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 8 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 8 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 8 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 8 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 8 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 8 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 8 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 8 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 8 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 8 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 8 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 8 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 8 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 8 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 8 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 8 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 8 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 8 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 8 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 8 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 8 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 8 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 8 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 8 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 8 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 8 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 8 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 8 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 8 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 9 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 9 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 9 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 9 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 9 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 9 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 9 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 9 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 9 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 9 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 9 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 9 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 9 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 9 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 9 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 9 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 9 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 9 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 9 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 9 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 9 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 9 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 9 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 9 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 9 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 9 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 9 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 9 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 9 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 9 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 9 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 9 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 10 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 10 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 10 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 10 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 10 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 10 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 10 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 10 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 10 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 10 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 10 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 10 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 10 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 10 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 10 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 10 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 10 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 10 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 10 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 10 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 10 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 10 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 10 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 10 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 10 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 10 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 10 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 10 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 10 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 10 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 10 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 10 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 11 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 11 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 11 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 11 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 11 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 11 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 11 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 11 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 11 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 11 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 11 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 11 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 11 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 11 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 11 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 11 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 11 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 11 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 11 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 11 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 11 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 11 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 11 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 11 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 11 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 11 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 11 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 11 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 11 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 11 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 11 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 11 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 12 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 12 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 12 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 12 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 12 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 12 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 12 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 12 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 12 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 12 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 12 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 12 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 12 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 12 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 12 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 12 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 12 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 12 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 12 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 12 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 12 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 12 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 12 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 12 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 12 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 12 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 12 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 12 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 12 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 12 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 12 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 12 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 13 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 13 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 13 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 13 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 13 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 13 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 13 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 13 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 13 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 13 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 13 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 13 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 13 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 13 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 13 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 13 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 13 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 13 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 13 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 13 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 13 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 13 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 13 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 13 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 13 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 13 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 13 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 13 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 13 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 13 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 13 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 13 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 14 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 14 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 14 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 14 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 14 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 14 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 14 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 14 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 14 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 14 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 14 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 14 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 14 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 14 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 14 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 14 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 14 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 14 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 14 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 14 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 14 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 14 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 14 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 14 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 14 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 14 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 14 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 14 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 14 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 14 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 14 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 14 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 15 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 15 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 15 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 15 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 15 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 15 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 15 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 15 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 15 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 15 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 15 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 15 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 15 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 15 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 15 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 15 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 15 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 15 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 15 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 15 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 15 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 15 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 15 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 15 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 15 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 15 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 15 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 15 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 15 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 15 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 15 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 15 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 16 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 16 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 16 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 16 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 16 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 16 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 16 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 16 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 16 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 16 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 16 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 16 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 16 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 16 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 16 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 16 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 16 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 16 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 16 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 16 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 16 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 16 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 16 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 16 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 16 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 16 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 16 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 16 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 16 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 16 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 16 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 16 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 17 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 17 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 17 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 17 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 17 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 17 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 17 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 17 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 17 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 17 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 17 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 17 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 17 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 17 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 17 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 17 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 17 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 17 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 17 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 17 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 17 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 17 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 17 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 17 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 17 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 17 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 17 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 17 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 17 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 17 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 17 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 17 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 18 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 18 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 18 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 18 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 18 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 18 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 18 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 18 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 18 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 18 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 18 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 18 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 18 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 18 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 18 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 18 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 18 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 18 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 18 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 18 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 18 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 18 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 18 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 18 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 18 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 18 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 18 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 18 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 18 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 18 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 18 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 18 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 19 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 19 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 19 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 19 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 19 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 19 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 19 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 19 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 19 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 19 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 19 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 19 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 19 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 19 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 19 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 19 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 19 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 19 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 19 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 19 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 19 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 19 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 19 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 19 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 19 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 19 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 19 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 19 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 19 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 19 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 19 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 19 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 20 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 20 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 20 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 20 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 20 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 20 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 20 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 20 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 20 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 20 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 20 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 20 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 20 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 20 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 20 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 20 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 20 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 20 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 20 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 20 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 20 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 20 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 20 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 20 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 20 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 20 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 20 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 20 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 20 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 20 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 20 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 20 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 21 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 21 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 21 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 21 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 21 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 21 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 21 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 21 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 21 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 21 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 21 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 21 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 21 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 21 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 21 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 21 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 21 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 21 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 21 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 21 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 21 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 21 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 21 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 21 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 21 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 21 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 21 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 21 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 21 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 21 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 21 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 21 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 22 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 22 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 22 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 22 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 22 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 22 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 22 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 22 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 22 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 22 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 22 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 22 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 22 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 22 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 22 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 22 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 22 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 22 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 22 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 22 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 22 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 22 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 22 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 22 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 22 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 22 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 22 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 22 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 22 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 22 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 22 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 22 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 23 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 23 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 23 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 23 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 23 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 23 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 23 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 23 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 23 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 23 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 23 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 23 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 23 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 23 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 23 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 23 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 23 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 23 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 23 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 23 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 23 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 23 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 23 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 23 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 23 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 23 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 23 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 23 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 23 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 23 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 23 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 23 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 24 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 24 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 24 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 24 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 24 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 24 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 24 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 24 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 24 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 24 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 24 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 24 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 24 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 24 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 24 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 24 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 24 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 24 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 24 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 24 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 24 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 24 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 24 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 24 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 24 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 24 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 24 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 24 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 24 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 24 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 24 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 24 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 25 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 25 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 25 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 25 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 25 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 25 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 25 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 25 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 25 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 25 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 25 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 25 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 25 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 25 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 25 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 25 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 25 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 25 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 25 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 25 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 25 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 25 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 25 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 25 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 25 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 25 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 25 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 25 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 25 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 25 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 25 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 25 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 26 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 26 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 26 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 26 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 26 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 26 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 26 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 26 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 26 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 26 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 26 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 26 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 26 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 26 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 26 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 26 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 26 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 26 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 26 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 26 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 26 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 26 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 26 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 26 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 26 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 26 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 26 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 26 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 26 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 26 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 26 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 26 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 27 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 27 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 27 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 27 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 27 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 27 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 27 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 27 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 27 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 27 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 27 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 27 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 27 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 27 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 27 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 27 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 27 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 27 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 27 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 27 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 27 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 27 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 27 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 27 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 27 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 27 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 27 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 27 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 27 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 27 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 27 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 27 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 28 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 28 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 28 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 28 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 28 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 28 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 28 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 28 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 28 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 28 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 28 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 28 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 28 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 28 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 28 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 28 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 28 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 28 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 28 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 28 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 28 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 28 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 28 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 28 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 28 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 28 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 28 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 28 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 28 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 28 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 28 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 28 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 29 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 29 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 29 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 29 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 29 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 29 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 29 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 29 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 29 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 29 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 29 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 29 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 29 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 29 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 29 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 29 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 29 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 29 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 29 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 29 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 29 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 29 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 29 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 29 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 29 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 29 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 29 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 29 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 29 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 29 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 29 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 29 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 30 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 30 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 30 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 30 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 30 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 30 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 30 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 30 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 30 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 30 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 30 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 30 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 30 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 30 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 30 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 30 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 30 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 30 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 30 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 30 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 30 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 30 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 30 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 30 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 30 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 30 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 30 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 30 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 30 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 30 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 30 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 30 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 31 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 31 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 31 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 31 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 31 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 31 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 31 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 31 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 31 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 31 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 31 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 31 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 31 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 31 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 31 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 31 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 31 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 31 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 31 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 31 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 31 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 31 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 31 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 31 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 31 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 31 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 31 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 31 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 31 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 31 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 31 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 31 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 32 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 32 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 32 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 32 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 32 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 32 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 32 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 32 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 32 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 32 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 32 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 32 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 32 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 32 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 32 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 32 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 32 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 32 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 32 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 32 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 32 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 32 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 32 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 32 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 32 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 32 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 32 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 32 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 32 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 32 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 32 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 32 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 33 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 33 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 33 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 33 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 33 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 33 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 33 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 33 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 33 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 33 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 33 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 33 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 33 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 33 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 33 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 33 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 33 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 33 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 33 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 33 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 33 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 33 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 33 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 33 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 33 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 33 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 33 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 33 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 33 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 33 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 33 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 33 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 34 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 34 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 34 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 34 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 34 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 34 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 34 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 34 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 34 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 34 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 34 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 34 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 34 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 34 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 34 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 34 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 34 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 34 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 34 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 34 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 34 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 34 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 34 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 34 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 34 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 34 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 34 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 34 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 34 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 34 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 34 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 34 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 35 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 35 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 35 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 35 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 35 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 35 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 35 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 35 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 35 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 35 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 35 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 35 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 35 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 35 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 35 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 35 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 35 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 35 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 35 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 35 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 35 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 35 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 35 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 35 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 35 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 35 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 35 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 35 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 35 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 35 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 35 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 35 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 36 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 36 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 36 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 36 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 36 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 36 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 36 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 36 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 36 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 36 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 36 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 36 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 36 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 36 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 36 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 36 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 36 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 36 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 36 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 36 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 36 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 36 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 36 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 36 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 36 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 36 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 36 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 36 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 36 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 36 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 36 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 36 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 37 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 37 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 37 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 37 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 37 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 37 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 37 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 37 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 37 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 37 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 37 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 37 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 37 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 37 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 37 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 37 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 37 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 37 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 37 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 37 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 37 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 37 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 37 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 37 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 37 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 37 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 37 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 37 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 37 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 37 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 37 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 37 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 38 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 38 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 38 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 38 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 38 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 38 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 38 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 38 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 38 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 38 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 38 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 38 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 38 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 38 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 38 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 38 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 38 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 38 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 38 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 38 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 38 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 38 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 38 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 38 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 38 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 38 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 38 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 38 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 38 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 38 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 38 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 38 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 39 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 39 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 39 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 39 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 39 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 39 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 39 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 39 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 39 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 39 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 39 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 39 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 39 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 39 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 39 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 39 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 39 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 39 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 39 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 39 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 39 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 39 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 39 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 39 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 39 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 39 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 39 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 39 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 39 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 39 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 39 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 39 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 40 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 40 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 40 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 40 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 40 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 40 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 40 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 40 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 40 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 40 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 40 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 40 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 40 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 40 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 40 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 40 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 40 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 40 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 40 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 40 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 40 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 40 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 40 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 40 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 40 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 40 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 40 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 40 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 40 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 40 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 40 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 40 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 41 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 41 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 41 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 41 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 41 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 41 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 41 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 41 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 41 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 41 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 41 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 41 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 41 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 41 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 41 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 41 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 41 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 41 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 41 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 41 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 41 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 41 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 41 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 41 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 41 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 41 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 41 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 41 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 41 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 41 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 41 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 41 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 42 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 42 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 42 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 42 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 42 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 42 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 42 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 42 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 42 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 42 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 42 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 42 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 42 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 42 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 42 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 42 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 42 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 42 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 42 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 42 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 42 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 42 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 42 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 42 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 42 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 42 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 42 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 42 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 42 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 42 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 42 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 42 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 43 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 43 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 43 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 43 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 43 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 43 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 43 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 43 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 43 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 43 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 43 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 43 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 43 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 43 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 43 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 43 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 43 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 43 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 43 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 43 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 43 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 43 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 43 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 43 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 43 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 43 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 43 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 43 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 43 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 43 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 43 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 43 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 44 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 44 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 44 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 44 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 44 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 44 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 44 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 44 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 44 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 44 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 44 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 44 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 44 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 44 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 44 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 44 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 44 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 44 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 44 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 44 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 44 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 44 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 44 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 44 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 44 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 44 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 44 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 44 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 44 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 44 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 44 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 44 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 45 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 45 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 45 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 45 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 45 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 45 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 45 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 45 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 45 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 45 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 45 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 45 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 45 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 45 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 45 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 45 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 45 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 45 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 45 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 45 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 45 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 45 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 45 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 45 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 45 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 45 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 45 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 45 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 45 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 45 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 45 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 45 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 46 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 46 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 46 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 46 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 46 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 46 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 46 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 46 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 46 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 46 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 46 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 46 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 46 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 46 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 46 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 46 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 46 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 46 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 46 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 46 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 46 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 46 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 46 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 46 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 46 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 46 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 46 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 46 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 46 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 46 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 46 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 46 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 47 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 47 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 47 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 47 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 47 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 47 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 47 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 47 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 47 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 47 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 47 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 47 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 47 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 47 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 47 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 47 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 47 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 47 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 47 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 47 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 47 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 47 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 47 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 47 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 47 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 47 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 47 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 47 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 47 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 47 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 47 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 47 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 48 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 48 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 48 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 48 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 48 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 48 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 48 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 48 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 48 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 48 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 48 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 48 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 48 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 48 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 48 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 48 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 48 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 48 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 48 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 48 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 48 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 48 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 48 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 48 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 48 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 48 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 48 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 48 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 48 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 48 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 48 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 48 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 49 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 49 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 49 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 49 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 49 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 49 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 49 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 49 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 49 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 49 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 49 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 49 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 49 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 49 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 49 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 49 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 49 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 49 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 49 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 49 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 49 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 49 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 49 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 49 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 49 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 49 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 49 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 49 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 49 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 49 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 49 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 49 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 50 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 50 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 50 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 50 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 50 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 50 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 50 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 50 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 50 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 50 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 50 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 50 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 50 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 50 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 50 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 50 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 50 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 50 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 50 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 50 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 50 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 50 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 50 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 50 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 50 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 50 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 50 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 50 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 50 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 50 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 50 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 50 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 51 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 51 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 51 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 51 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 51 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 51 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 51 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 51 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 51 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 51 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 51 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 51 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 51 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 51 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 51 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 51 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 51 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 51 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 51 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 51 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 51 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 51 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 51 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 51 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 51 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 51 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 51 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 51 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 51 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 51 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 51 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 51 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 52 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 52 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 52 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 52 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 52 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 52 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 52 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 52 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 52 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 52 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 52 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 52 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 52 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 52 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 52 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 52 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 52 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 52 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 52 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 52 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 52 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 52 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 52 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 52 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 52 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 52 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 52 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 52 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 52 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 52 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 52 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 52 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 53 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 53 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 53 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 53 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 53 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 53 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 53 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 53 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 53 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 53 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 53 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 53 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 53 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 53 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 53 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 53 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 53 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 53 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 53 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 53 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 53 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 53 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 53 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 53 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 53 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 53 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 53 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 53 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 53 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 53 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 53 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 53 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 54 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 54 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 54 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 54 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 54 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 54 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 54 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 54 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 54 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 54 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 54 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 54 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 54 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 54 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 54 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 54 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 54 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 54 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 54 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 54 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 54 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 54 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 54 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 54 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 54 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 54 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 54 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 54 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 54 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 54 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 54 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 54 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 55 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 55 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 55 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 55 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 55 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 55 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 55 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 55 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 55 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 55 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 55 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 55 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 55 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 55 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 55 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 55 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 55 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 55 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 55 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 55 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 55 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 55 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 55 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 55 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 55 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 55 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 55 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 55 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 55 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 55 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 55 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 55 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 56 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 56 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 56 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 56 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 56 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 56 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 56 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 56 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 56 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 56 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 56 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 56 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 56 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 56 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 56 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 56 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 56 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 56 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 56 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 56 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 56 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 56 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 56 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 56 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 56 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 56 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 56 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 56 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 56 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 56 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 56 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 56 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 57 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 57 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 57 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 57 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 57 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 57 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 57 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 57 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 57 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 57 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 57 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 57 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 57 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 57 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 57 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 57 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 57 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 57 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 57 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 57 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 57 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 57 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 57 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 57 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 57 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 57 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 57 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 57 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 57 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 57 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 57 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 57 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 58 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 58 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 58 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 58 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 58 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 58 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 58 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 58 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 58 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 58 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 58 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 58 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 58 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 58 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 58 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 58 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 58 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 58 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 58 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 58 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 58 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 58 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 58 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 58 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 58 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 58 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 58 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 58 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 58 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 58 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 58 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 58 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 59 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 59 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 59 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 59 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 59 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 59 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 59 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 59 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 59 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 59 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 59 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 59 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 59 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 59 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 59 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 59 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 59 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 59 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 59 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 59 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 59 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 59 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 59 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 59 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 59 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 59 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 59 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 59 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 59 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 59 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 59 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 59 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 60 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 60 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 60 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 60 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 60 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 60 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 60 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 60 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 60 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 60 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 60 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 60 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 60 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 60 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 60 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 60 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 60 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 60 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 60 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 60 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 60 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 60 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 60 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 60 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 60 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 60 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 60 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 60 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 60 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 60 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 60 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 60 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 61 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 61 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 61 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 61 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 61 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 61 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 61 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 61 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 61 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 61 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 61 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 61 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 61 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 61 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 61 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 61 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 61 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 61 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 61 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 61 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 61 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 61 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 61 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 61 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 61 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 61 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 61 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 61 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 61 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 61 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 61 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 61 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 62 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 62 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 62 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 62 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 62 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 62 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 62 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 62 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 62 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 62 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 62 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 62 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 62 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 62 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 62 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 62 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 62 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 62 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 62 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 62 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 62 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 62 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 62 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 62 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 62 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 62 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 62 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 62 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 62 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 62 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 62 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 62 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 63 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 63 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 63 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 63 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 63 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 63 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 63 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 63 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 63 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 63 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 63 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 63 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 63 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 63 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 63 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 63 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 63 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 63 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 63 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 63 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 63 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 63 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 63 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 63 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 63 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 63 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 63 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 63 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 63 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 63 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 63 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 63 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 32 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 32 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 32 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 32 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 32 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 32 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 32 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 32 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 32 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 32 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 32 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 32 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 32 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 32 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 32 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 32 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 32 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 32 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 32 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 32 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 32 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 32 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 32 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 32 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 32 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 32 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 32 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 32 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 32 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 32 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 32 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 32 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 65 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 65 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 65 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 65 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 65 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 65 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 65 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 65 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 65 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 65 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 65 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 65 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 65 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 65 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 65 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 65 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 65 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 65 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 65 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 65 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 65 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 65 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 65 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 65 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 65 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 65 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 65 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 65 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 65 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 65 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 65 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 65 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 66 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 66 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 66 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 66 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 66 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 66 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 66 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 66 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 66 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 66 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 66 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 66 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 66 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 66 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 66 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 66 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 66 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 66 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 66 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 66 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 66 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 66 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 66 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 66 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 66 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 66 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 66 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 66 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 66 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 66 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 66 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 66 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 67 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 67 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 67 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 67 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 67 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 67 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 67 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 67 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 67 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 67 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 67 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 67 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 67 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 67 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 67 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 67 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 67 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 67 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 67 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 67 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 67 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 67 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 67 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 67 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 67 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 67 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 67 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 67 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 67 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 67 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 67 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 67 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 68 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 68 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 68 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 68 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 68 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 68 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 68 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 68 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 68 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 68 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 68 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 68 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 68 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 68 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 68 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 68 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 68 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 68 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 68 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 68 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 68 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 68 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 68 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 68 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 68 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 68 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 68 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 68 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 68 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 68 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 68 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 68 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 69 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 69 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 69 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 69 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 69 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 69 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 69 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 69 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 69 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 69 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 69 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 69 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 69 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 69 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 69 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 69 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 69 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 69 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 69 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 69 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 69 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 69 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 69 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 69 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 69 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 69 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 69 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 69 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 69 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 69 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 69 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 69 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 70 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 70 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 70 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 70 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 70 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 70 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 70 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 70 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 70 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 70 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 70 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 70 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 70 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 70 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 70 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 70 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 70 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 70 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 70 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 70 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 70 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 70 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 70 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 70 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 70 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 70 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 70 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 70 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 70 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 70 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 70 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 70 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 71 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 71 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 71 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 71 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 71 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 71 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 71 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 71 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 71 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 71 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 71 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 71 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 71 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 71 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 71 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 71 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 71 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 71 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 71 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 71 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 71 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 71 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 71 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 71 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 71 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 71 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 71 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 71 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 71 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 71 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 71 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 71 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 72 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 72 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 72 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 72 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 72 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 72 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 72 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 72 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 72 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 72 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 72 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 72 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 72 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 72 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 72 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 72 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 72 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 72 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 72 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 72 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 72 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 72 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 72 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 72 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 72 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 72 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 72 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 72 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 72 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 72 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 72 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 72 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 73 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 73 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 73 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 73 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 73 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 73 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 73 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 73 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 73 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 73 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 73 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 73 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 73 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 73 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 73 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 73 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 73 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 73 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 73 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 73 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 73 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 73 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 73 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 73 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 73 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 73 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 73 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 73 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 73 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 73 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 73 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 73 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 74 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 74 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 74 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 74 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 74 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 74 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 74 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 74 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 74 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 74 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 74 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 74 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 74 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 74 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 74 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 74 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 74 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 74 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 74 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 74 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 74 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 74 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 74 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 74 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 74 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 74 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 74 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 74 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 74 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 74 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 74 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 74 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 75 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 75 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 75 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 75 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 75 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 75 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 75 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 75 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 75 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 75 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 75 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 75 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 75 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 75 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 75 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 75 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 75 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 75 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 75 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 75 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 75 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 75 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 75 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 75 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 75 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 75 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 75 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 75 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 75 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 75 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 75 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 75 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 76 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 76 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 76 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 76 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 76 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 76 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 76 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 76 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 76 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 76 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 76 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 76 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 76 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 76 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 76 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 76 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 76 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 76 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 76 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 76 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 76 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 76 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 76 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 76 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 76 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 76 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 76 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 76 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 76 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 76 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 76 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 76 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 77 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 77 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 77 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 77 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 77 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 77 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 77 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 77 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 77 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 77 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 77 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 77 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 77 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 77 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 77 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 77 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 77 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 77 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 77 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 77 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 77 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 77 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 77 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 77 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 77 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 77 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 77 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 77 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 77 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 77 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 77 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 77 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 78 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 78 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 78 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 78 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 78 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 78 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 78 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 78 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 78 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 78 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 78 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 78 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 78 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 78 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 78 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 78 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 78 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 78 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 78 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 78 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 78 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 78 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 78 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 78 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 78 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 78 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 78 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 78 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 78 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 78 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 78 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 78 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 79 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 79 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 79 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 79 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 79 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 79 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 79 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 79 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 79 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 79 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 79 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 79 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 79 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 79 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 79 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 79 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 79 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 79 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 79 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 79 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 79 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 79 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 79 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 79 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 79 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 79 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 79 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 79 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 79 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 79 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 79 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 79 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 80 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 80 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 80 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 80 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 80 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 80 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 80 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 80 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 80 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 80 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 80 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 80 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 80 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 80 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 80 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 80 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 80 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 80 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 80 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 80 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 80 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 80 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 80 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 80 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 80 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 80 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 80 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 80 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 80 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 80 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 80 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 80 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 81 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 81 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 81 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 81 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 81 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 81 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 81 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 81 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 81 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 81 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 81 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 81 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 81 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 81 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 81 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 81 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 81 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 81 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 81 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 81 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 81 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 81 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 81 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 81 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 81 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 81 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 81 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 81 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 81 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 81 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 81 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 81 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 82 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 82 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 82 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 82 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 82 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 82 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 82 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 82 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 82 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 82 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 82 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 82 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 82 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 82 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 82 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 82 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 82 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 82 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 82 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 82 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 82 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 82 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 82 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 82 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 82 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 82 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 82 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 82 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 82 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 82 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 82 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 82 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 83 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 83 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 83 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 83 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 83 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 83 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 83 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 83 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 83 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 83 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 83 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 83 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 83 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 83 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 83 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 83 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 83 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 83 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 83 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 83 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 83 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 83 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 83 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 83 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 83 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 83 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 83 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 83 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 83 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 83 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 83 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 83 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 84 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 84 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 84 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 84 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 84 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 84 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 84 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 84 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 84 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 84 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 84 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 84 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 84 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 84 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 84 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 84 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 84 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 84 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 84 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 84 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 84 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 84 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 84 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 84 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 84 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 84 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 84 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 84 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 84 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 84 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 84 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 84 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 85 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 85 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 85 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 85 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 85 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 85 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 85 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 85 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 85 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 85 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 85 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 85 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 85 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 85 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 85 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 85 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 85 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 85 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 85 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 85 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 85 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 85 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 85 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 85 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 85 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 85 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 85 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 85 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 85 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 85 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 85 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 85 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 86 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 86 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 86 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 86 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 86 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 86 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 86 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 86 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 86 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 86 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 86 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 86 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 86 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 86 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 86 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 86 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 86 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 86 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 86 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 86 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 86 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 86 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 86 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 86 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 86 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 86 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 86 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 86 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 86 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 86 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 86 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 86 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 87 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 87 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 87 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 87 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 87 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 87 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 87 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 87 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 87 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 87 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 87 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 87 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 87 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 87 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 87 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 87 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 87 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 87 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 87 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 87 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 87 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 87 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 87 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 87 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 87 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 87 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 87 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 87 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 87 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 87 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 87 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 87 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 88 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 88 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 88 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 88 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 88 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 88 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 88 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 88 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 88 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 88 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 88 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 88 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 88 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 88 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 88 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 88 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 88 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 88 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 88 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 88 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 88 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 88 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 88 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 88 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 88 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 88 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 88 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 88 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 88 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 88 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 88 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 88 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 89 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 89 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 89 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 89 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 89 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 89 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 89 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 89 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 89 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 89 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 89 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 89 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 89 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 89 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 89 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 89 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 89 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 89 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 89 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 89 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 89 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 89 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 89 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 89 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 89 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 89 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 89 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 89 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 89 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 89 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 89 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 89 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 90 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 90 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 90 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 90 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 90 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 90 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 90 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 90 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 90 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 90 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 90 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 90 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 90 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 90 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 90 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 90 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 90 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 90 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 90 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 90 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 90 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 90 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 90 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 90 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 90 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 90 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 90 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 90 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 90 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 90 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 90 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 90 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 91 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 91 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 91 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 91 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 91 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 91 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 91 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 91 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 91 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 91 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 91 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 91 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 91 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 91 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 91 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 91 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 91 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 91 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 91 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 91 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 91 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 91 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 91 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 91 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 91 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 91 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 91 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 91 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 91 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 91 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 91 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 91 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 92 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 92 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 92 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 92 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 92 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 92 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 92 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 92 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 92 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 92 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 92 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 92 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 92 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 92 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 92 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 92 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 92 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 92 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 92 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 92 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 92 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 92 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 92 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 92 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 92 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 92 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 92 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 92 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 92 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 92 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 92 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 92 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 93 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 93 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 93 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 93 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 93 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 93 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 93 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 93 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 93 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 93 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 93 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 93 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 93 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 93 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 93 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 93 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 93 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 93 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 93 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 93 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 93 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 93 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 93 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 93 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 93 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 93 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 93 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 93 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 93 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 93 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 93 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 93 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 94 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 94 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 94 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 94 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 94 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 94 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 94 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 94 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 94 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 94 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 94 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 94 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 94 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 94 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 94 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 94 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 94 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 94 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 94 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 94 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 94 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 94 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 94 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 94 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 94 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 94 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 94 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 94 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 94 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 94 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 94 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 94 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 95 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 95 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 95 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 95 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 95 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 95 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 95 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 95 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 95 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 95 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 95 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 95 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 95 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 95 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 95 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 95 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 95 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 95 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 95 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 95 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 95 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 95 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 95 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 95 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 95 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 95 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 95 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 95 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 95 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 95 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 95 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 95 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 96 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 96 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 96 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 96 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 96 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 96 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 96 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 96 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 96 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 96 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 96 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 96 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 96 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 96 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 96 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 96 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 96 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 96 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 96 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 96 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 96 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 96 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 96 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 96 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 96 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 96 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 96 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 96 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 96 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 96 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 96 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 96 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 97 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 97 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 97 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 97 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 97 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 97 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 97 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 97 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 97 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 97 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 97 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 97 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 97 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 97 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 97 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 97 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 97 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 97 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 97 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 97 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 97 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 97 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 97 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 97 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 97 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 97 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 97 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 97 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 97 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 97 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 97 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 97 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 98 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 98 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 98 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 98 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 98 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 98 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 98 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 98 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 98 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 98 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 98 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 98 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 98 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 98 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 98 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 98 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 98 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 98 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 98 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 98 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 98 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 98 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 98 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 98 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 98 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 98 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 98 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 98 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 98 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 98 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 98 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 98 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 99 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 99 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 99 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 99 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 99 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 99 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 99 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 99 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 99 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 99 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 99 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 99 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 99 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 99 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 99 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 99 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 99 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 99 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 99 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 99 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 99 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 99 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 99 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 99 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 99 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 99 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 99 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 99 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 99 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 99 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 99 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 99 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 100 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 100 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 100 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 100 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 100 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 100 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 100 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 100 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 100 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 100 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 100 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 100 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 100 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 100 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 100 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 100 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 100 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 100 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 100 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 100 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 100 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 100 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 100 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 100 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 100 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 100 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 100 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 100 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 100 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 100 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 100 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 100 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 101 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 101 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 101 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 101 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 101 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 101 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 101 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 101 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 101 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 101 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 101 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 101 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 101 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 101 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 101 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 101 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 101 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 101 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 101 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 101 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 101 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 101 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 101 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 101 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 101 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 101 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 101 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 101 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 101 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 101 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 101 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 101 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 102 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 102 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 102 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 102 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 102 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 102 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 102 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 102 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 102 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 102 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 102 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 102 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 102 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 102 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 102 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 102 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 102 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 102 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 102 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 102 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 102 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 102 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 102 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 102 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 102 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 102 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 102 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 102 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 102 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 102 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 102 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 102 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 103 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 103 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 103 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 103 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 103 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 103 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 103 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 103 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 103 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 103 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 103 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 103 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 103 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 103 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 103 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 103 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 103 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 103 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 103 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 103 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 103 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 103 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 103 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 103 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 103 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 103 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 103 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 103 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 103 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 103 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 103 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 103 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 104 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 104 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 104 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 104 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 104 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 104 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 104 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 104 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 104 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 104 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 104 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 104 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 104 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 104 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 104 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 104 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 104 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 104 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 104 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 104 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 104 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 104 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 104 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 104 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 104 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 104 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 104 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 104 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 104 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 104 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 104 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 104 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 105 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 105 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 105 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 105 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 105 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 105 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 105 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 105 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 105 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 105 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 105 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 105 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 105 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 105 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 105 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 105 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 105 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 105 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 105 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 105 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 105 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 105 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 105 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 105 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 105 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 105 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 105 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 105 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 105 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 105 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 105 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 105 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 106 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 106 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 106 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 106 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 106 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 106 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 106 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 106 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 106 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 106 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 106 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 106 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 106 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 106 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 106 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 106 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 106 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 106 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 106 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 106 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 106 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 106 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 106 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 106 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 106 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 106 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 106 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 106 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 106 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 106 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 106 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 106 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 107 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 107 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 107 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 107 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 107 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 107 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 107 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 107 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 107 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 107 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 107 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 107 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 107 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 107 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 107 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 107 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 107 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 107 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 107 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 107 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 107 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 107 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 107 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 107 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 107 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 107 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 107 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 107 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 107 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 107 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 107 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 107 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 108 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 108 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 108 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 108 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 108 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 108 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 108 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 108 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 108 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 108 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 108 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 108 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 108 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 108 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 108 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 108 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 108 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 108 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 108 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 108 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 108 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 108 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 108 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 108 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 108 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 108 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 108 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 108 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 108 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 108 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 108 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 108 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 109 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 109 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 109 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 109 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 109 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 109 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 109 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 109 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 109 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 109 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 109 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 109 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 109 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 109 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 109 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 109 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 109 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 109 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 109 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 109 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 109 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 109 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 109 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 109 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 109 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 109 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 109 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 109 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 109 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 109 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 109 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 109 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 110 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 110 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 110 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 110 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 110 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 110 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 110 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 110 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 110 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 110 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 110 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 110 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 110 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 110 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 110 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 110 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 110 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 110 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 110 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 110 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 110 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 110 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 110 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 110 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 110 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 110 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 110 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 110 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 110 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 110 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 110 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 110 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 111 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 111 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 111 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 111 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 111 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 111 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 111 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 111 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 111 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 111 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 111 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 111 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 111 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 111 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 111 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 111 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 111 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 111 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 111 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 111 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 111 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 111 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 111 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 111 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 111 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 111 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 111 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 111 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 111 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 111 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 111 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 111 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 112 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 112 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 112 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 112 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 112 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 112 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 112 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 112 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 112 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 112 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 112 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 112 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 112 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 112 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 112 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 112 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 112 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 112 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 112 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 112 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 112 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 112 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 112 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 112 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 112 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 112 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 112 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 112 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 112 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 112 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 112 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 112 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 113 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 113 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 113 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 113 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 113 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 113 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 113 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 113 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 113 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 113 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 113 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 113 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 113 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 113 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 113 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 113 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 113 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 113 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 113 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 113 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 113 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 113 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 113 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 113 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 113 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 113 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 113 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 113 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 113 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 113 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 113 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 113 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 114 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 114 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 114 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 114 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 114 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 114 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 114 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 114 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 114 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 114 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 114 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 114 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 114 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 114 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 114 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 114 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 114 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 114 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 114 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 114 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 114 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 114 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 114 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 114 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 114 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 114 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 114 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 114 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 114 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 114 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 114 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 114 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 115 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 115 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 115 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 115 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 115 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 115 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 115 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 115 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 115 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 115 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 115 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 115 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 115 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 115 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 115 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 115 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 115 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 115 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 115 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 115 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 115 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 115 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 115 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 115 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 115 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 115 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 115 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 115 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 115 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 115 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 115 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 115 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 116 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 116 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 116 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 116 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 116 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 116 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 116 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 116 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 116 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 116 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 116 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 116 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 116 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 116 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 116 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 116 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 116 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 116 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 116 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 116 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 116 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 116 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 116 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 116 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 116 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 116 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 116 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 116 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 116 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 116 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 116 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 116 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 117 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 117 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 117 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 117 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 117 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 117 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 117 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 117 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 117 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 117 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 117 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 117 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 117 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 117 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 117 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 117 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 117 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 117 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 117 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 117 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 117 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 117 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 117 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 117 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 117 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 117 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 117 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 117 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 117 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 117 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 117 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 117 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 118 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 118 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 118 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 118 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 118 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 118 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 118 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 118 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 118 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 118 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 118 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 118 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 118 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 118 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 118 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 118 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 118 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 118 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 118 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 118 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 118 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 118 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 118 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 118 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 118 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 118 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 118 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 118 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 118 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 118 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 118 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 118 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 119 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 119 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 119 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 119 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 119 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 119 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 119 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 119 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 119 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 119 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 119 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 119 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 119 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 119 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 119 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 119 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 119 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 119 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 119 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 119 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 119 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 119 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 119 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 119 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 119 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 119 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 119 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 119 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 119 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 119 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 119 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 119 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 120 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 120 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 120 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 120 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 120 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 120 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 120 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 120 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 120 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 120 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 120 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 120 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 120 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 120 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 120 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 120 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 120 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 120 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 120 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 120 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 120 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 120 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 120 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 120 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 120 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 120 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 120 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 120 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 120 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 120 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 120 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 120 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 121 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 121 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 121 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 121 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 121 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 121 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 121 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 121 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 121 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 121 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 121 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 121 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 121 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 121 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 121 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 121 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 121 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 121 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 121 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 121 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 121 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 121 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 121 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 121 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 121 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 121 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 121 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 121 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 121 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 121 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 121 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 121 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 122 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 122 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 122 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 122 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 122 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 122 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 122 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 122 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 122 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 122 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 122 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 122 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 122 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 122 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 122 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 122 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 122 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 122 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 122 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 122 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 122 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 122 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 122 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 122 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 122 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 122 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 122 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 122 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 122 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 122 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 122 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 122 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 123 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 123 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 123 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 123 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 123 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 123 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 123 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 123 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 123 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 123 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 123 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 123 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 123 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 123 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 123 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 123 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 123 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 123 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 123 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 123 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 123 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 123 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 123 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 123 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 123 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 123 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 123 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 123 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 123 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 123 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 123 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 123 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 124 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 124 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 124 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 124 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 124 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 124 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 124 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 124 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 124 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 124 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 124 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 124 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 124 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 124 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 124 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 124 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 124 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 124 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 124 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 124 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 124 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 124 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 124 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 124 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 124 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 124 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 124 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 124 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 124 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 124 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 124 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 124 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 125 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 125 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 125 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 125 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 125 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 125 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 125 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 125 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 125 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 125 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 125 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 125 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 125 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 125 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 125 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 125 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 125 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 125 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 125 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 125 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 125 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 125 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 125 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 125 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 125 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 125 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 125 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 125 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 125 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 125 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 125 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 125 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 126 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 126 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 126 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 126 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 126 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 126 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 126 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 126 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 126 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 126 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 126 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 126 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 126 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 126 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 126 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 126 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 126 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 126 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 126 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 126 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 126 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 126 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 126 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 126 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 126 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 126 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 126 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 126 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 126 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 126 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 126 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 126 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 127 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 127 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 127 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 127 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 127 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 127 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 127 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 127 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 127 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 127 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 127 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 127 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 127 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 127 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 127 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 127 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 127 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 127 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 127 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 127 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 127 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 127 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 127 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 127 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 127 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 127 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 127 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 127 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 127 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 127 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 127 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 127 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 128 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 128 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 128 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 128 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 128 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 128 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 128 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 128 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 128 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 128 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 128 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 128 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 128 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 128 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 128 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 128 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 128 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 128 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 128 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 128 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 128 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 128 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 128 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 128 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 128 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 128 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 128 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 128 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 128 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 128 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 128 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 128 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 129 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 129 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 129 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 129 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 129 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 129 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 129 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 129 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 129 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 129 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 129 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 129 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 129 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 129 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 129 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 129 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 129 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 129 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 129 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 129 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 129 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 129 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 129 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 129 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 129 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 129 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 129 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 129 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 129 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 129 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 129 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 129 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 130 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 130 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 130 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 130 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 130 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 130 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 130 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 130 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 130 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 130 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 130 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 130 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 130 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 130 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 130 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 130 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 130 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 130 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 130 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 130 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 130 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 130 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 130 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 130 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 130 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 130 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 130 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 130 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 130 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 130 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 130 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 130 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 131 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 131 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 131 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 131 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 131 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 131 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 131 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 131 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 131 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 131 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 131 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 131 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 131 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 131 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 131 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 131 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 131 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 131 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 131 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 131 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 131 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 131 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 131 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 131 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 131 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 131 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 131 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 131 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 131 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 131 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 131 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 131 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 132 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 132 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 132 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 132 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 132 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 132 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 132 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 132 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 132 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 132 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 132 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 132 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 132 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 132 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 132 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 132 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 132 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 132 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 132 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 132 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 132 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 132 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 132 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 132 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 132 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 132 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 132 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 132 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 132 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 132 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 132 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 132 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 133 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 133 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 133 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 133 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 133 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 133 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 133 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 133 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 133 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 133 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 133 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 133 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 133 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 133 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 133 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 133 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 133 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 133 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 133 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 133 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 133 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 133 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 133 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 133 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 133 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 133 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 133 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 133 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 133 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 133 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 133 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 133 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 134 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 134 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 134 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 134 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 134 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 134 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 134 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 134 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 134 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 134 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 134 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 134 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 134 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 134 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 134 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 134 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 134 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 134 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 134 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 134 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 134 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 134 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 134 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 134 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 134 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 134 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 134 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 134 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 134 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 134 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 134 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 134 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 135 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 135 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 135 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 135 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 135 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 135 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 135 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 135 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 135 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 135 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 135 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 135 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 135 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 135 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 135 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 135 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 135 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 135 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 135 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 135 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 135 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 135 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 135 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 135 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 135 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 135 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 135 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 135 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 135 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 135 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 135 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 135 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 136 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 136 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 136 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 136 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 136 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 136 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 136 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 136 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 136 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 136 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 136 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 136 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 136 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 136 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 136 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 136 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 136 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 136 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 136 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 136 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 136 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 136 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 136 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 136 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 136 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 136 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 136 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 136 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 136 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 136 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 136 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 136 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 137 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 137 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 137 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 137 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 137 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 137 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 137 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 137 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 137 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 137 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 137 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 137 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 137 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 137 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 137 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 137 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 137 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 137 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 137 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 137 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 137 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 137 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 137 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 137 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 137 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 137 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 137 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 137 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 137 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 137 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 137 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 137 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 138 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 138 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 138 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 138 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 138 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 138 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 138 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 138 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 138 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 138 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 138 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 138 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 138 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 138 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 138 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 138 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 138 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 138 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 138 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 138 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 138 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 138 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 138 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 138 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 138 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 138 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 138 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 138 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 138 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 138 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 138 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 138 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 139 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 139 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 139 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 139 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 139 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 139 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 139 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 139 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 139 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 139 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 139 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 139 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 139 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 139 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 139 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 139 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 139 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 139 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 139 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 139 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 139 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 139 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 139 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 139 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 139 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 139 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 139 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 139 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 139 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 139 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 139 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 139 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 140 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 140 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 140 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 140 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 140 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 140 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 140 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 140 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 140 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 140 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 140 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 140 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 140 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 140 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 140 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 140 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 140 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 140 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 140 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 140 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 140 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 140 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 140 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 140 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 140 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 140 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 140 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 140 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 140 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 140 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 140 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 140 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 141 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 141 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 141 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 141 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 141 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 141 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 141 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 141 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 141 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 141 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 141 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 141 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 141 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 141 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 141 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 141 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 141 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 141 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 141 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 141 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 141 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 141 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 141 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 141 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 141 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 141 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 141 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 141 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 141 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 141 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 141 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 141 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 142 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 142 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 142 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 142 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 142 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 142 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 142 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 142 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 142 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 142 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 142 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 142 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 142 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 142 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 142 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 142 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 142 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 142 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 142 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 142 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 142 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 142 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 142 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 142 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 142 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 142 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 142 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 142 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 142 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 142 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 142 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 142 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 143 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 143 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 143 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 143 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 143 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 143 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 143 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 143 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 143 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 143 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 143 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 143 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 143 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 143 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 143 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 143 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 143 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 143 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 143 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 143 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 143 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 143 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 143 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 143 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 143 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 143 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 143 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 143 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 143 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 143 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 143 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 143 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 144 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 144 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 144 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 144 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 144 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 144 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 144 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 144 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 144 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 144 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 144 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 144 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 144 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 144 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 144 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 144 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 144 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 144 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 144 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 144 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 144 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 144 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 144 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 144 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 144 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 144 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 144 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 144 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 144 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 144 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 144 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 144 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 145 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 145 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 145 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 145 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 145 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 145 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 145 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 145 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 145 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 145 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 145 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 145 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 145 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 145 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 145 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 145 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 145 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 145 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 145 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 145 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 145 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 145 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 145 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 145 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 145 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 145 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 145 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 145 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 145 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 145 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 145 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 145 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 146 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 146 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 146 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 146 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 146 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 146 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 146 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 146 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 146 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 146 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 146 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 146 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 146 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 146 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 146 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 146 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 146 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 146 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 146 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 146 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 146 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 146 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 146 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 146 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 146 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 146 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 146 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 146 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 146 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 146 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 146 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 146 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 147 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 147 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 147 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 147 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 147 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 147 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 147 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 147 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 147 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 147 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 147 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 147 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 147 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 147 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 147 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 147 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 147 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 147 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 147 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 147 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 147 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 147 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 147 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 147 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 147 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 147 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 147 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 147 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 147 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 147 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 147 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 147 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 148 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 148 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 148 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 148 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 148 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 148 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 148 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 148 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 148 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 148 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 148 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 148 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 148 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 148 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 148 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 148 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 148 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 148 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 148 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 148 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 148 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 148 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 148 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 148 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 148 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 148 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 148 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 148 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 148 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 148 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 148 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 148 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 149 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 149 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 149 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 149 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 149 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 149 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 149 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 149 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 149 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 149 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 149 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 149 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 149 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 149 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 149 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 149 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 149 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 149 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 149 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 149 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 149 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 149 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 149 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 149 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 149 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 149 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 149 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 149 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 149 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 149 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 149 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 149 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 150 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 150 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 150 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 150 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 150 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 150 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 150 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 150 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 150 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 150 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 150 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 150 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 150 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 150 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 150 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 150 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 150 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 150 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 150 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 150 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 150 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 150 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 150 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 150 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 150 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 150 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 150 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 150 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 150 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 150 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 150 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 150 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 151 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 151 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 151 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 151 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 151 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 151 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 151 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 151 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 151 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 151 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 151 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 151 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 151 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 151 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 151 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 151 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 151 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 151 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 151 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 151 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 151 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 151 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 151 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 151 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 151 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 151 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 151 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 151 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 151 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 151 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 151 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 151 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 152 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 152 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 152 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 152 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 152 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 152 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 152 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 152 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 152 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 152 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 152 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 152 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 152 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 152 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 152 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 152 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 152 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 152 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 152 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 152 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 152 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 152 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 152 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 152 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 152 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 152 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 152 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 152 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 152 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 152 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 152 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 152 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 153 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 153 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 153 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 153 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 153 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 153 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 153 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 153 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 153 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 153 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 153 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 153 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 153 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 153 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 153 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 153 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 153 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 153 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 153 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 153 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 153 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 153 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 153 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 153 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 153 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 153 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 153 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 153 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 153 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 153 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 153 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 153 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 154 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 154 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 154 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 154 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 154 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 154 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 154 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 154 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 154 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 154 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 154 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 154 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 154 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 154 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 154 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 154 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 154 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 154 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 154 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 154 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 154 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 154 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 154 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 154 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 154 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 154 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 154 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 154 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 154 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 154 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 154 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 154 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 155 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 155 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 155 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 155 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 155 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 155 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 155 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 155 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 155 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 155 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 155 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 155 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 155 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 155 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 155 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 155 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 155 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 155 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 155 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 155 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 155 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 155 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 155 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 155 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 155 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 155 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 155 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 155 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 155 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 155 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 155 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 155 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 156 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 156 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 156 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 156 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 156 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 156 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 156 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 156 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 156 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 156 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 156 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 156 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 156 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 156 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 156 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 156 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 156 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 156 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 156 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 156 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 156 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 156 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 156 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 156 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 156 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 156 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 156 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 156 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 156 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 156 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 156 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 156 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 157 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 157 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 157 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 157 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 157 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 157 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 157 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 157 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 157 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 157 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 157 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 157 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 157 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 157 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 157 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 157 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 157 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 157 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 157 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 157 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 157 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 157 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 157 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 157 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 157 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 157 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 157 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 157 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 157 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 157 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 157 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 157 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 158 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 158 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 158 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 158 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 158 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 158 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 158 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 158 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 158 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 158 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 158 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 158 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 158 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 158 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 158 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 158 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 158 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 158 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 158 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 158 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 158 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 158 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 158 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 158 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 158 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 158 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 158 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 158 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 158 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 158 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 158 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 158 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 159 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 159 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 159 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 159 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 159 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 159 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 159 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 159 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 159 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 159 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 159 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 159 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 159 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 159 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 159 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 159 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 159 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 159 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 159 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 159 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 159 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 159 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 159 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 159 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 159 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 159 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 159 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 159 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 159 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 159 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 159 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 159 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 160 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 160 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 160 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 160 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 160 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 160 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 160 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 160 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 160 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 160 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 160 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 160 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 160 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 160 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 160 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 160 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 160 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 160 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 160 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 160 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 160 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 160 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 160 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 160 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 160 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 160 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 160 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 160 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 160 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 160 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 160 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 160 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 161 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 161 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 161 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 161 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 161 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 161 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 161 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 161 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 161 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 161 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 161 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 161 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 161 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 161 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 161 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 161 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 161 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 161 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 161 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 161 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 161 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 161 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 161 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 161 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 161 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 161 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 161 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 161 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 161 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 161 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 161 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 161 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 162 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 162 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 162 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 162 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 162 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 162 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 162 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 162 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 162 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 162 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 162 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 162 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 162 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 162 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 162 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 162 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 162 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 162 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 162 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 162 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 162 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 162 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 162 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 162 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 162 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 162 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 162 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 162 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 162 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 162 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 162 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 162 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 163 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 163 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 163 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 163 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 163 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 163 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 163 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 163 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 163 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 163 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 163 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 163 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 163 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 163 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 163 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 163 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 163 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 163 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 163 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 163 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 163 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 163 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 163 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 163 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 163 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 163 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 163 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 163 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 163 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 163 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 163 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 163 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 132 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 132 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 132 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 132 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 132 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 132 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 132 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 132 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 132 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 132 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 132 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 132 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 132 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 132 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 132 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 132 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 132 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 132 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 132 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 132 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 132 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 132 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 132 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 132 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 132 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 132 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 132 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 132 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 132 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 132 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 132 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 132 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 165 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 165 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 165 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 165 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 165 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 165 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 165 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 165 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 165 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 165 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 165 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 165 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 165 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 165 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 165 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 165 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 165 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 165 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 165 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 165 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 165 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 165 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 165 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 165 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 165 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 165 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 165 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 165 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 165 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 165 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 165 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 165 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 166 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 166 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 166 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 166 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 166 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 166 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 166 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 166 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 166 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 166 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 166 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 166 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 166 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 166 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 166 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 166 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 166 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 166 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 166 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 166 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 166 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 166 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 166 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 166 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 166 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 166 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 166 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 166 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 166 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 166 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 166 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 166 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 167 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 167 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 167 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 167 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 167 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 167 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 167 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 167 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 167 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 167 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 167 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 167 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 167 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 167 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 167 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 167 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 167 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 167 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 167 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 167 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 167 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 167 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 167 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 167 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 167 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 167 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 167 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 167 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 167 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 167 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 167 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 167 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 168 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 168 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 168 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 168 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 168 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 168 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 168 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 168 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 168 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 168 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 168 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 168 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 168 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 168 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 168 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 168 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 168 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 168 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 168 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 168 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 168 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 168 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 168 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 168 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 168 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 168 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 168 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 168 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 168 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 168 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 168 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 168 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 169 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 169 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 169 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 169 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 169 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 169 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 169 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 169 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 169 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 169 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 169 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 169 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 169 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 169 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 169 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 169 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 169 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 169 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 169 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 169 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 169 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 169 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 169 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 169 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 169 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 169 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 169 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 169 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 169 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 169 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 169 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 169 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 170 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 170 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 170 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 170 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 170 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 170 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 170 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 170 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 170 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 170 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 170 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 170 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 170 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 170 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 170 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 170 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 170 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 170 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 170 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 170 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 170 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 170 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 170 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 170 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 170 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 170 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 170 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 170 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 170 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 170 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 170 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 170 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 171 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 171 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 171 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 171 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 171 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 171 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 171 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 171 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 171 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 171 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 171 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 171 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 171 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 171 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 171 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 171 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 171 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 171 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 171 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 171 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 171 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 171 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 171 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 171 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 171 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 171 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 171 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 171 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 171 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 171 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 171 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 171 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 172 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 172 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 172 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 172 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 172 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 172 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 172 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 172 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 172 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 172 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 172 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 172 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 172 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 172 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 172 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 172 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 172 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 172 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 172 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 172 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 172 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 172 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 172 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 172 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 172 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 172 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 172 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 172 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 172 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 172 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 172 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 172 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 173 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 173 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 173 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 173 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 173 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 173 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 173 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 173 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 173 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 173 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 173 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 173 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 173 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 173 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 173 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 173 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 173 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 173 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 173 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 173 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 173 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 173 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 173 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 173 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 173 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 173 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 173 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 173 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 173 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 173 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 173 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 173 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 174 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 174 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 174 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 174 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 174 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 174 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 174 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 174 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 174 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 174 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 174 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 174 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 174 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 174 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 174 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 174 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 174 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 174 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 174 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 174 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 174 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 174 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 174 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 174 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 174 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 174 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 174 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 174 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 174 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 174 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 174 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 174 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 175 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 175 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 175 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 175 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 175 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 175 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 175 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 175 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 175 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 175 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 175 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 175 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 175 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 175 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 175 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 175 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 175 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 175 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 175 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 175 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 175 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 175 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 175 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 175 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 175 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 175 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 175 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 175 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 175 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 175 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 175 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 175 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 176 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 176 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 176 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 176 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 176 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 176 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 176 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 176 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 176 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 176 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 176 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 176 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 176 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 176 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 176 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 176 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 176 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 176 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 176 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 176 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 176 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 176 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 176 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 176 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 176 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 176 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 176 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 176 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 176 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 176 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 176 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 176 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 177 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 177 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 177 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 177 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 177 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 177 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 177 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 177 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 177 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 177 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 177 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 177 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 177 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 177 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 177 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 177 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 177 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 177 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 177 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 177 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 177 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 177 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 177 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 177 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 177 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 177 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 177 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 177 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 177 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 177 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 177 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 177 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 178 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 178 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 178 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 178 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 178 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 178 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 178 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 178 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 178 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 178 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 178 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 178 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 178 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 178 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 178 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 178 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 178 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 178 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 178 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 178 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 178 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 178 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 178 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 178 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 178 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 178 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 178 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 178 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 178 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 178 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 178 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 178 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 179 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 179 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 179 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 179 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 179 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 179 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 179 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 179 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 179 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 179 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 179 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 179 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 179 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 179 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 179 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 179 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 179 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 179 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 179 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 179 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 179 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 179 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 179 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 179 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 179 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 179 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 179 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 179 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 179 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 179 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 179 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 179 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 180 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 180 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 180 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 180 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 180 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 180 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 180 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 180 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 180 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 180 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 180 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 180 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 180 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 180 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 180 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 180 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 180 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 180 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 180 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 180 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 180 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 180 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 180 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 180 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 180 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 180 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 180 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 180 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 180 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 180 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 180 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 180 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 181 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 181 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 181 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 181 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 181 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 181 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 181 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 181 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 181 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 181 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 181 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 181 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 181 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 181 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 181 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 181 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 181 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 181 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 181 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 181 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 181 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 181 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 181 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 181 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 181 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 181 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 181 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 181 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 181 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 181 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 181 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 181 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 182 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 182 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 182 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 182 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 182 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 182 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 182 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 182 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 182 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 182 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 182 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 182 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 182 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 182 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 182 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 182 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 182 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 182 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 182 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 182 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 182 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 182 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 182 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 182 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 182 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 182 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 182 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 182 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 182 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 182 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 182 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 182 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 183 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 183 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 183 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 183 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 183 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 183 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 183 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 183 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 183 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 183 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 183 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 183 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 183 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 183 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 183 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 183 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 183 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 183 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 183 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 183 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 183 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 183 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 183 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 183 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 183 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 183 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 183 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 183 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 183 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 183 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 183 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 183 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 184 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 184 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 184 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 184 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 184 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 184 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 184 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 184 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 184 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 184 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 184 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 184 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 184 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 184 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 184 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 184 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 184 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 184 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 184 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 184 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 184 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 184 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 184 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 184 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 184 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 184 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 184 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 184 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 184 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 184 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 184 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 184 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 185 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 185 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 185 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 185 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 185 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 185 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 185 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 185 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 185 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 185 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 185 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 185 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 185 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 185 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 185 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 185 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 185 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 185 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 185 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 185 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 185 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 185 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 185 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 185 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 185 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 185 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 185 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 185 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 185 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 185 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 185 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 185 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 186 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 186 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 186 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 186 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 186 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 186 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 186 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 186 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 186 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 186 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 186 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 186 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 186 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 186 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 186 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 186 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 186 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 186 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 186 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 186 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 186 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 186 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 186 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 186 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 186 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 186 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 186 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 186 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 186 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 186 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 186 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 186 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 187 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 187 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 187 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 187 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 187 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 187 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 187 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 187 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 187 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 187 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 187 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 187 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 187 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 187 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 187 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 187 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 187 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 187 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 187 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 187 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 187 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 187 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 187 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 187 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 187 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 187 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 187 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 187 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 187 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 187 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 187 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 187 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 188 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 188 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 188 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 188 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 188 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 188 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 188 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 188 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 188 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 188 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 188 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 188 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 188 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 188 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 188 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 188 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 188 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 188 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 188 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 188 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 188 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 188 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 188 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 188 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 188 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 188 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 188 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 188 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 188 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 188 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 188 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 188 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 189 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 189 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 189 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 189 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 189 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 189 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 189 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 189 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 189 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 189 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 189 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 189 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 189 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 189 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 189 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 189 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 189 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 189 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 189 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 189 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 189 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 189 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 189 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 189 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 189 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 189 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 189 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 189 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 189 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 189 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 189 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 189 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 190 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 190 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 190 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 190 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 190 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 190 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 190 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 190 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 190 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 190 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 190 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 190 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 190 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 190 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 190 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 190 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 190 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 190 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 190 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 190 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 190 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 190 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 190 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 190 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 190 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 190 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 190 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 190 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 190 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 190 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 190 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 190 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 191 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 191 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 191 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 191 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 191 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 191 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 191 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 191 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 191 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 191 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 191 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 191 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 191 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 191 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 191 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 191 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 191 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 191 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 191 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 191 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 191 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 191 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 191 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 191 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 191 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 191 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 191 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 191 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 191 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 191 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 191 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 191 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 192 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 192 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 192 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 192 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 192 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 192 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 192 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 192 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 192 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 192 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 192 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 192 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 192 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 192 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 192 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 192 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 192 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 192 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 192 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 192 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 192 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 192 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 192 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 192 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 192 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 192 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 192 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 192 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 192 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 192 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 192 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 192 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 193 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 193 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 193 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 193 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 193 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 193 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 193 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 193 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 193 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 193 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 193 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 193 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 193 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 193 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 193 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 193 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 193 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 193 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 193 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 193 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 193 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 193 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 193 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 193 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 193 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 193 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 193 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 193 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 193 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 193 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 193 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 193 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 194 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 194 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 194 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 194 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 194 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 194 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 194 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 194 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 194 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 194 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 194 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 194 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 194 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 194 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 194 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 194 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 194 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 194 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 194 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 194 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 194 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 194 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 194 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 194 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 194 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 194 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 194 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 194 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 194 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 194 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 194 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 194 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 195 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 195 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 195 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 195 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 195 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 195 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 195 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 195 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 195 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 195 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 195 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 195 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 195 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 195 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 195 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 195 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 195 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 195 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 195 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 195 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 195 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 195 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 195 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 195 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 195 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 195 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 195 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 195 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 195 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 195 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 195 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 195 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 196 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 196 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 196 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 196 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 196 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 196 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 196 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 196 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 196 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 196 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 196 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 196 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 196 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 196 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 196 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 196 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 196 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 196 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 196 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 196 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 196 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 196 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 196 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 196 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 196 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 196 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 196 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 196 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 196 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 196 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 196 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 196 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 197 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 197 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 197 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 197 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 197 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 197 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 197 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 197 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 197 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 197 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 197 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 197 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 197 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 197 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 197 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 197 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 197 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 197 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 197 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 197 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 197 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 197 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 197 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 197 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 197 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 197 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 197 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 197 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 197 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 197 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 197 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 197 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 198 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 198 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 198 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 198 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 198 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 198 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 198 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 198 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 198 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 198 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 198 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 198 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 198 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 198 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 198 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 198 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 198 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 198 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 198 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 198 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 198 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 198 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 198 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 198 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 198 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 198 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 198 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 198 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 198 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 198 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 198 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 198 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 199 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 199 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 199 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 199 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 199 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 199 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 199 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 199 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 199 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 199 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 199 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 199 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 199 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 199 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 199 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 199 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 199 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 199 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 199 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 199 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 199 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 199 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 199 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 199 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 199 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 199 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 199 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 199 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 199 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 199 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 199 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 199 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 200 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 200 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 200 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 200 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 200 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 200 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 200 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 200 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 200 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 200 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 200 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 200 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 200 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 200 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 200 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 200 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 200 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 200 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 200 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 200 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 200 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 200 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 200 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 200 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 200 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 200 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 200 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 200 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 200 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 200 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 200 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 200 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 201 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 201 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 201 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 201 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 201 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 201 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 201 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 201 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 201 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 201 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 201 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 201 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 201 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 201 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 201 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 201 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 201 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 201 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 201 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 201 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 201 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 201 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 201 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 201 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 201 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 201 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 201 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 201 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 201 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 201 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 201 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 201 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 202 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 202 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 202 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 202 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 202 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 202 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 202 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 202 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 202 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 202 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 202 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 202 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 202 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 202 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 202 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 202 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 202 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 202 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 202 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 202 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 202 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 202 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 202 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 202 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 202 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 202 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 202 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 202 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 202 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 202 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 202 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 202 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 203 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 203 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 203 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 203 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 203 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 203 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 203 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 203 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 203 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 203 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 203 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 203 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 203 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 203 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 203 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 203 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 203 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 203 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 203 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 203 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 203 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 203 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 203 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 203 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 203 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 203 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 203 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 203 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 203 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 203 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 203 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 203 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 204 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 204 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 204 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 204 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 204 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 204 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 204 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 204 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 204 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 204 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 204 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 204 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 204 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 204 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 204 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 204 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 204 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 204 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 204 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 204 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 204 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 204 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 204 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 204 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 204 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 204 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 204 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 204 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 204 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 204 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 204 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 204 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 205 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 205 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 205 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 205 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 205 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 205 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 205 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 205 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 205 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 205 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 205 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 205 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 205 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 205 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 205 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 205 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 205 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 205 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 205 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 205 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 205 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 205 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 205 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 205 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 205 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 205 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 205 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 205 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 205 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 205 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 205 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 205 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 206 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 206 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 206 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 206 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 206 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 206 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 206 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 206 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 206 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 206 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 206 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 206 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 206 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 206 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 206 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 206 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 206 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 206 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 206 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 206 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 206 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 206 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 206 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 206 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 206 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 206 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 206 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 206 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 206 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 206 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 206 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 206 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 207 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 207 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 207 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 207 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 207 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 207 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 207 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 207 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 207 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 207 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 207 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 207 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 207 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 207 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 207 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 207 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 207 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 207 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 207 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 207 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 207 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 207 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 207 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 207 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 207 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 207 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 207 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 207 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 207 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 207 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 207 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 207 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 208 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 208 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 208 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 208 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 208 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 208 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 208 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 208 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 208 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 208 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 208 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 208 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 208 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 208 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 208 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 208 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 208 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 208 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 208 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 208 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 208 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 208 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 208 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 208 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 208 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 208 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 208 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 208 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 208 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 208 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 208 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 208 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 209 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 209 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 209 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 209 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 209 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 209 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 209 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 209 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 209 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 209 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 209 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 209 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 209 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 209 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 209 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 209 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 209 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 209 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 209 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 209 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 209 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 209 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 209 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 209 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 209 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 209 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 209 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 209 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 209 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 209 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 209 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 209 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 210 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 210 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 210 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 210 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 210 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 210 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 210 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 210 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 210 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 210 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 210 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 210 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 210 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 210 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 210 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 210 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 210 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 210 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 210 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 210 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 210 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 210 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 210 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 210 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 210 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 210 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 210 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 210 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 210 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 210 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 210 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 210 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 211 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 211 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 211 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 211 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 211 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 211 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 211 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 211 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 211 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 211 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 211 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 211 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 211 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 211 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 211 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 211 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 211 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 211 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 211 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 211 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 211 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 211 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 211 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 211 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 211 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 211 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 211 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 211 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 211 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 211 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 211 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 211 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 212 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 212 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 212 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 212 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 212 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 212 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 212 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 212 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 212 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 212 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 212 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 212 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 212 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 212 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 212 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 212 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 212 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 212 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 212 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 212 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 212 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 212 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 212 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 212 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 212 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 212 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 212 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 212 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 212 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 212 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 212 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 212 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 213 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 213 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 213 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 213 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 213 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 213 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 213 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 213 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 213 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 213 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 213 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 213 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 213 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 213 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 213 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 213 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 213 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 213 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 213 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 213 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 213 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 213 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 213 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 213 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 213 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 213 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 213 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 213 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 213 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 213 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 213 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 213 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 214 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 214 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 214 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 214 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 214 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 214 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 214 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 214 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 214 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 214 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 214 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 214 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 214 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 214 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 214 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 214 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 214 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 214 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 214 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 214 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 214 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 214 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 214 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 214 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 214 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 214 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 214 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 214 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 214 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 214 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 214 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 214 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 215 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 215 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 215 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 215 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 215 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 215 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 215 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 215 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 215 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 215 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 215 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 215 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 215 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 215 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 215 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 215 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 215 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 215 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 215 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 215 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 215 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 215 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 215 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 215 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 215 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 215 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 215 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 215 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 215 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 215 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 215 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 215 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 216 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 216 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 216 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 216 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 216 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 216 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 216 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 216 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 216 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 216 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 216 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 216 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 216 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 216 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 216 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 216 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 216 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 216 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 216 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 216 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 216 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 216 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 216 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 216 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 216 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 216 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 216 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 216 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 216 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 216 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 216 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 216 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 217 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 217 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 217 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 217 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 217 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 217 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 217 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 217 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 217 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 217 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 217 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 217 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 217 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 217 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 217 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 217 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 217 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 217 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 217 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 217 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 217 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 217 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 217 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 217 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 217 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 217 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 217 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 217 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 217 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 217 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 217 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 217 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 218 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 218 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 218 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 218 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 218 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 218 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 218 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 218 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 218 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 218 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 218 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 218 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 218 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 218 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 218 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 218 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 218 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 218 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 218 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 218 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 218 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 218 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 218 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 218 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 218 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 218 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 218 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 218 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 218 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 218 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 218 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 218 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 219 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 219 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 219 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 219 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 219 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 219 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 219 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 219 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 219 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 219 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 219 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 219 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 219 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 219 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 219 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 219 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 219 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 219 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 219 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 219 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 219 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 219 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 219 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 219 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 219 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 219 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 219 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 219 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 219 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 219 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 219 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 219 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 220 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 220 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 220 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 220 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 220 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 220 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 220 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 220 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 220 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 220 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 220 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 220 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 220 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 220 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 220 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 220 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 220 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 220 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 220 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 220 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 220 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 220 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 220 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 220 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 220 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 220 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 220 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 220 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 220 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 220 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 220 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 220 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 221 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 221 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 221 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 221 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 221 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 221 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 221 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 221 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 221 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 221 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 221 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 221 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 221 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 221 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 221 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 221 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 221 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 221 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 221 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 221 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 221 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 221 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 221 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 221 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 221 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 221 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 221 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 221 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 221 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 221 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 221 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 221 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 222 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 222 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 222 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 222 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 222 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 222 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 222 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 222 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 222 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 222 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 222 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 222 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 222 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 222 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 222 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 222 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 222 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 222 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 222 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 222 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 222 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 222 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 222 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 222 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 222 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 222 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 222 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 222 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 222 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 222 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 222 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 222 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 223 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 223 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 223 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 223 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 223 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 223 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 223 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 223 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 223 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 223 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 223 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 223 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 223 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 223 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 223 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 223 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 223 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 223 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 223 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 223 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 223 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 223 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 223 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 223 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 223 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 223 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 223 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 223 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 223 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 223 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 223 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 223 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 224 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 224 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 224 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 224 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 224 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 224 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 224 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 224 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 224 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 224 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 224 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 224 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 224 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 224 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 224 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 224 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 224 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 224 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 224 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 224 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 224 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 224 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 224 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 224 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 224 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 224 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 224 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 224 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 224 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 224 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 224 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 224 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 225 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 225 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 225 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 225 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 225 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 225 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 225 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 225 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 225 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 225 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 225 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 225 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 225 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 225 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 225 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 225 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 225 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 225 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 225 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 225 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 225 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 225 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 225 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 225 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 225 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 225 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 225 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 225 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 225 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 225 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 225 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 225 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 226 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 226 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 226 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 226 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 226 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 226 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 226 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 226 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 226 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 226 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 226 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 226 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 226 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 226 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 226 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 226 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 226 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 226 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 226 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 226 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 226 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 226 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 226 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 226 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 226 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 226 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 226 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 226 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 226 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 226 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 226 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 226 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 227 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 227 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 227 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 227 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 227 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 227 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 227 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 227 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 227 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 227 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 227 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 227 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 227 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 227 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 227 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 227 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 227 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 227 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 227 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 227 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 227 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 227 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 227 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 227 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 227 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 227 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 227 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 227 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 227 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 227 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 227 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 227 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 228 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 228 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 228 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 228 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 228 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 228 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 228 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 228 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 228 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 228 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 228 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 228 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 228 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 228 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 228 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 228 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 228 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 228 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 228 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 228 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 228 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 228 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 228 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 228 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 228 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 228 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 228 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 228 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 228 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 228 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 228 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 228 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 229 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 229 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 229 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 229 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 229 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 229 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 229 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 229 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 229 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 229 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 229 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 229 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 229 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 229 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 229 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 229 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 229 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 229 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 229 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 229 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 229 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 229 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 229 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 229 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 229 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 229 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 229 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 229 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 229 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 229 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 229 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 229 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 230 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 230 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 230 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 230 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 230 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 230 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 230 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 230 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 230 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 230 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 230 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 230 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 230 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 230 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 230 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 230 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 230 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 230 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 230 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 230 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 230 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 230 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 230 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 230 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 230 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 230 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 230 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 230 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 230 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 230 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 230 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 230 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 231 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 231 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 231 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 231 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 231 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 231 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 231 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 231 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 231 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 231 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 231 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 231 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 231 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 231 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 231 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 231 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 231 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 231 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 231 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 231 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 231 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 231 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 231 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 231 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 231 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 231 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 231 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 231 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 231 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 231 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 231 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 231 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 232 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 232 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 232 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 232 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 232 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 232 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 232 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 232 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 232 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 232 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 232 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 232 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 232 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 232 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 232 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 232 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 232 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 232 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 232 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 232 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 232 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 232 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 232 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 232 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 232 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 232 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 232 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 232 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 232 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 232 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 232 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 232 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 233 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 233 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 233 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 233 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 233 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 233 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 233 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 233 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 233 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 233 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 233 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 233 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 233 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 233 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 233 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 233 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 233 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 233 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 233 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 233 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 233 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 233 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 233 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 233 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 233 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 233 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 233 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 233 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 233 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 233 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 233 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 233 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 234 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 234 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 234 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 234 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 234 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 234 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 234 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 234 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 234 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 234 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 234 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 234 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 234 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 234 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 234 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 234 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 234 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 234 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 234 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 234 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 234 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 234 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 234 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 234 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 234 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 234 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 234 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 234 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 234 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 234 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 234 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 234 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 235 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 235 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 235 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 235 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 235 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 235 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 235 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 235 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 235 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 235 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 235 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 235 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 235 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 235 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 235 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 235 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 235 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 235 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 235 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 235 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 235 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 235 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 235 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 235 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 235 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 235 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 235 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 235 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 235 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 235 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 235 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 235 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 236 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 236 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 236 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 236 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 236 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 236 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 236 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 236 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 236 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 236 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 236 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 236 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 236 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 236 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 236 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 236 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 236 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 236 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 236 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 236 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 236 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 236 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 236 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 236 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 236 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 236 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 236 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 236 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 236 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 236 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 236 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 236 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 237 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 237 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 237 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 237 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 237 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 237 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 237 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 237 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 237 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 237 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 237 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 237 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 237 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 237 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 237 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 237 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 237 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 237 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 237 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 237 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 237 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 237 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 237 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 237 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 237 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 237 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 237 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 237 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 237 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 237 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 237 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 237 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 238 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 238 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 238 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 238 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 238 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 238 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 238 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 238 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 238 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 238 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 238 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 238 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 238 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 238 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 238 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 238 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 238 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 238 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 238 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 238 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 238 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 238 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 238 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 238 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 238 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 238 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 238 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 238 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 238 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 238 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 238 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 238 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 239 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 239 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 239 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 239 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 239 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 239 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 239 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 239 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 239 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 239 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 239 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 239 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 239 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 239 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 239 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 239 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 239 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 239 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 239 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 239 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 239 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 239 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 239 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 239 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 239 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 239 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 239 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 239 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 239 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 239 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 239 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 239 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 240 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 240 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 240 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 240 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 240 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 240 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 240 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 240 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 240 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 240 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 240 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 240 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 240 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 240 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 240 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 240 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 240 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 240 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 240 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 240 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 240 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 240 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 240 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 240 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 240 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 240 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 240 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 240 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 240 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 240 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 240 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 240 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 241 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 241 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 241 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 241 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 241 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 241 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 241 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 241 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 241 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 241 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 241 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 241 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 241 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 241 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 241 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 241 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 241 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 241 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 241 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 241 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 241 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 241 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 241 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 241 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 241 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 241 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 241 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 241 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 241 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 241 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 241 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 241 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 242 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 242 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 242 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 242 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 242 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 242 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 242 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 242 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 242 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 242 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 242 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 242 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 242 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 242 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 242 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 242 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 242 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 242 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 242 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 242 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 242 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 242 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 242 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 242 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 242 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 242 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 242 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 242 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 242 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 242 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 242 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 242 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 243 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 243 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 243 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 243 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 243 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 243 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 243 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 243 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 243 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 243 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 243 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 243 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 243 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 243 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 243 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 243 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 243 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 243 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 243 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 243 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 243 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 243 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 243 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 243 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 243 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 243 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 243 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 243 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 243 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 243 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 243 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 243 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 244 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 244 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 244 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 244 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 244 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 244 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 244 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 244 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 244 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 244 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 244 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 244 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 244 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 244 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 244 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 244 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 244 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 244 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 244 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 244 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 244 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 244 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 244 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 244 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 244 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 244 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 244 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 244 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 244 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 244 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 244 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 244 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 245 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 245 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 245 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 245 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 245 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 245 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 245 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 245 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 245 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 245 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 245 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 245 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 245 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 245 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 245 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 245 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 245 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 245 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 245 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 245 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 245 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 245 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 245 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 245 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 245 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 245 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 245 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 245 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 245 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 245 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 245 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 245 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 246 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 246 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 246 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 246 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 246 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 246 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 246 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 246 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 246 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 246 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 246 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 246 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 246 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 246 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 246 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 246 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 246 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 246 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 246 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 246 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 246 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 246 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 246 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 246 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 246 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 246 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 246 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 246 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 246 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 246 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 246 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 246 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 247 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 247 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 247 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 247 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 247 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 247 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 247 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 247 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 247 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 247 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 247 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 247 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 247 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 247 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 247 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 247 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 247 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 247 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 247 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 247 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 247 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 247 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 247 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 247 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 247 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 247 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 247 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 247 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 247 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 247 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 247 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 247 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 248 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 248 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 248 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 248 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 248 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 248 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 248 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 248 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 248 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 248 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 248 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 248 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 248 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 248 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 248 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 248 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 248 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 248 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 248 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 248 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 248 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 248 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 248 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 248 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 248 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 248 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 248 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 248 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 248 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 248 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 248 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 248 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 249 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 249 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 249 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 249 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 249 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 249 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 249 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 249 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 249 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 249 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 249 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 249 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 249 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 249 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 249 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 249 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 249 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 249 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 249 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 249 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 249 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 249 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 249 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 249 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 249 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 249 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 249 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 249 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 249 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 249 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 249 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 249 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 250 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 250 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 250 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 250 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 250 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 250 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 250 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 250 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 250 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 250 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 250 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 250 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 250 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 250 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 250 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 250 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 250 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 250 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 250 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 250 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 250 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 250 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 250 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 250 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 250 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 250 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 250 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 250 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 250 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 250 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 250 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 250 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 251 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 251 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 251 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 251 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 251 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 251 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 251 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 251 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 251 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 251 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 251 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 251 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 251 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 251 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 251 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 251 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 251 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 251 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 251 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 251 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 251 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 251 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 251 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 251 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 251 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 251 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 251 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 251 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 251 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 251 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 251 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 251 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 252 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 252 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 252 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 252 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 252 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 252 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 252 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 252 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 252 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 252 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 252 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 252 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 252 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 252 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 252 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 252 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 252 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 252 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 252 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 252 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 252 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 252 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 252 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 252 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 252 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 252 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 252 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 252 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 252 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 252 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 252 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 252 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 253 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 253 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 253 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 253 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 253 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 253 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 253 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 253 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 253 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 253 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 253 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 253 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 253 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 253 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 253 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 253 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 253 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 253 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 253 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 253 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 253 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 253 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 253 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 253 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 253 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 253 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 253 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 253 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 253 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 253 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 253 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 253 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 254 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 254 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 254 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 254 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 254 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 254 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 254 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 254 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 254 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 254 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 254 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 254 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 254 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 254 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 254 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 254 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 254 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 254 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 254 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 254 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 254 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 254 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 254 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 254 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 254 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 254 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 254 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 254 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 254 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 254 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 254 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 254 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
        (if 255 as libc::c_int == 'A' as i32 {
            0 as libc::c_int
        } else if 255 as libc::c_int == 'B' as i32 {
            1 as libc::c_int
        } else if 255 as libc::c_int == 'C' as i32 {
            2 as libc::c_int
        } else if 255 as libc::c_int == 'D' as i32 {
            3 as libc::c_int
        } else if 255 as libc::c_int == 'E' as i32 {
            4 as libc::c_int
        } else if 255 as libc::c_int == 'F' as i32 {
            5 as libc::c_int
        } else if 255 as libc::c_int == 'G' as i32 {
            6 as libc::c_int
        } else if 255 as libc::c_int == 'H' as i32 {
            7 as libc::c_int
        } else if 255 as libc::c_int == 'I' as i32 {
            8 as libc::c_int
        } else if 255 as libc::c_int == 'J' as i32 {
            9 as libc::c_int
        } else if 255 as libc::c_int == 'K' as i32 {
            10 as libc::c_int
        } else if 255 as libc::c_int == 'L' as i32 {
            11 as libc::c_int
        } else if 255 as libc::c_int == 'M' as i32 {
            12 as libc::c_int
        } else if 255 as libc::c_int == 'N' as i32 {
            13 as libc::c_int
        } else if 255 as libc::c_int == 'O' as i32 {
            14 as libc::c_int
        } else if 255 as libc::c_int == 'P' as i32 {
            15 as libc::c_int
        } else if 255 as libc::c_int == 'Q' as i32 {
            16 as libc::c_int
        } else if 255 as libc::c_int == 'R' as i32 {
            17 as libc::c_int
        } else if 255 as libc::c_int == 'S' as i32 {
            18 as libc::c_int
        } else if 255 as libc::c_int == 'T' as i32 {
            19 as libc::c_int
        } else if 255 as libc::c_int == 'U' as i32 {
            20 as libc::c_int
        } else if 255 as libc::c_int == 'V' as i32 {
            21 as libc::c_int
        } else if 255 as libc::c_int == 'W' as i32 {
            22 as libc::c_int
        } else if 255 as libc::c_int == 'X' as i32 {
            23 as libc::c_int
        } else if 255 as libc::c_int == 'Y' as i32 {
            24 as libc::c_int
        } else if 255 as libc::c_int == 'Z' as i32 {
            25 as libc::c_int
        } else if 255 as libc::c_int == '2' as i32 {
            26 as libc::c_int
        } else if 255 as libc::c_int == '3' as i32 {
            27 as libc::c_int
        } else if 255 as libc::c_int == '4' as i32 {
            28 as libc::c_int
        } else if 255 as libc::c_int == '5' as i32 {
            29 as libc::c_int
        } else if 255 as libc::c_int == '6' as i32 {
            30 as libc::c_int
        } else if 255 as libc::c_int == '7' as i32 {
            31 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_schar,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
