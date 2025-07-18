use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn block_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut nbytes: size_t,
) -> size_t {
    let mut bp: *mut libc::c_char = buf;
    let mut buflim: *const libc::c_char = buf.offset(nbytes as isize);
    let mut readlim: size_t = if 9223372036854775807 as libc::c_long as libc::c_ulong
        <= 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        18446744073709551615 as libc::c_ulong
    };
    loop {
        let mut bytes_remaining: size_t = buflim.offset_from(bp) as libc::c_long
            as size_t;
        let mut bytes_to_read: size_t = if bytes_remaining <= readlim {
            bytes_remaining
        } else {
            readlim
        };
        let mut nread: ssize_t = read(fd, bp as *mut libc::c_void, bytes_to_read);
        if nread <= 0 as libc::c_int as libc::c_long {
            if nread == 0 as libc::c_int as libc::c_long {
                break;
            }
            if *__errno_location() == 22 as libc::c_int
                && (2147483647 as libc::c_int as libc::c_ulong) < bytes_to_read
            {
                readlim = 2147483647 as libc::c_int as size_t;
            } else if !(0x10000000 as libc::c_int == 0
                && *__errno_location() == 4 as libc::c_int)
            {
                return 18446744073709551615 as libc::c_ulong
            }
        } else {
            bp = bp.offset(nread as isize);
        }
        if !(bp < buflim as *mut libc::c_char) {
            break;
        }
    }
    return bp.offset_from(buf) as libc::c_long as size_t;
}
pub unsafe extern "C" fn buffer_lcm(
    mut a: size_t,
    mut b: size_t,
    mut lcm_max: size_t,
) -> size_t {
    let mut lcm: size_t = 0;
    let mut m: size_t = 0;
    let mut n: size_t = 0;
    let mut q: size_t = 0;
    let mut r: size_t = 0;
    if a == 0 {
        return if b != 0 {
            b
        } else {
            (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        };
    }
    if b == 0 {
        return a;
    }
    m = a;
    n = b;
    loop {
        r = m.wrapping_rem(n);
        if !(r != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        m = n;
        n = r;
    }
    q = a.wrapping_div(n);
    return if (if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
            if (if b < 0 as libc::c_int as libc::c_ulong {
                if q < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int
                        }) as libc::c_ulong)
                            .wrapping_add(b)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (q < (127 as libc::c_int as libc::c_ulong).wrapping_div(b))
                            as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            b
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (b
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(127 as libc::c_int as libc::c_ulong)
                                >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (127 as libc::c_int as libc::c_ulong)
                                .wrapping_div(b.wrapping_neg())
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                            as libc::c_int
                    }
                } else {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            b
                        })
                            .wrapping_add(
                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                            )
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            b
                        })
                            .wrapping_add(
                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                            )
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(
                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )) as libc::c_int
                    }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            q
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < q
                                    .wrapping_add(
                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                    )) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < q
                                && ((-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_ulong)
                                    < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }
                    } else {
                        (((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_div(b) < q) as libc::c_int
                    }
                }
            } else {
                if b == 0 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int
                } else {
                    if q < 0 as libc::c_int as libc::c_ulong {
                        if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                q
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(
                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                    )
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(
                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                    )
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                q
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(
                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(
                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                    )) as libc::c_int
                        }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                        {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < b
                                        .wrapping_add(
                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                        )) as libc::c_int
                            } else {
                                (((-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_ulong)
                                    < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            }
                        } else {
                            (((-(127 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_div(q) < b) as libc::c_int
                        }
                    } else {
                        ((127 as libc::c_int as libc::c_ulong).wrapping_div(b) < q)
                            as libc::c_int
                    }
                }
            }) != 0
            {
                lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                    as libc::c_schar as size_t;
                1 as libc::c_int
            } else {
                lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                    as libc::c_schar as size_t;
                0 as libc::c_int
            }
        } else {
            if (if b < 0 as libc::c_int as libc::c_ulong {
                if q < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        }) as libc::c_ulong)
                            .wrapping_add(b)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (q
                            < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_div(b)) as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            b
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (b
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(
                                    (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                        as libc::c_ulong,
                                )
                                >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_div(b.wrapping_neg())
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                            as libc::c_int
                    }
                } else {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            b
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            b
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            q
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < q.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < q
                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                    as libc::c_ulong)
                                    < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }
                    } else {
                        ((0 as libc::c_int as libc::c_ulong).wrapping_div(b) < q)
                            as libc::c_int
                    }
                }
            } else {
                if b == 0 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int
                } else {
                    if q < 0 as libc::c_int as libc::c_ulong {
                        if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                q
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                q
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                        {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < b.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            } else {
                                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                                    < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            }
                        } else {
                            ((0 as libc::c_int as libc::c_ulong).wrapping_div(q) < b)
                                as libc::c_int
                        }
                    } else {
                        (((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_div(b) < q) as libc::c_int
                    }
                }
            }) != 0
            {
                lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                    as libc::c_uchar as size_t;
                1 as libc::c_int
            } else {
                lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                    as libc::c_uchar as size_t;
                0 as libc::c_int
            }
        }
    } else {
        if ::std::mem::size_of::<size_t>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
                if (if b < 0 as libc::c_int as libc::c_ulong {
                    if q < 0 as libc::c_int as libc::c_ulong {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_add(b)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            (q < (32767 as libc::c_int as libc::c_ulong).wrapping_div(b))
                                as libc::c_int
                        } else {
                            ((if (if (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                !((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            }) < 0 as libc::c_int as libc::c_ulong
                            {
                                (b
                                    < (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_neg()) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(32767 as libc::c_int as libc::c_ulong)
                                    >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (32767 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(b.wrapping_neg())
                            }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                                as libc::c_int
                        }
                    } else {
                        if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(
                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(
                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(
                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )) as libc::c_int
                        }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                        {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                q
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < q
                                        .wrapping_add(
                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong) < q
                                    && ((-(1 as libc::c_int)
                                        - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                        as libc::c_ulong)
                                        < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            }
                        } else {
                            (((-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_div(b) < q) as libc::c_int
                        }
                    }
                } else {
                    if b == 0 as libc::c_int as libc::c_ulong {
                        0 as libc::c_int
                    } else {
                        if q < 0 as libc::c_int as libc::c_ulong {
                            if (if (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(
                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                !((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(
                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(
                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            }) < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(
                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                                    < (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(
                                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(
                                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_neg()) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(
                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )) as libc::c_int
                            }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                            {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < b
                                            .wrapping_add(
                                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )) as libc::c_int
                                } else {
                                    (((-(1 as libc::c_int)
                                        - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                        as libc::c_ulong)
                                        < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                }
                            } else {
                                (((-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_div(q) < b) as libc::c_int
                            }
                        } else {
                            ((32767 as libc::c_int as libc::c_ulong).wrapping_div(b) < q)
                                as libc::c_int
                        }
                    }
                }) != 0
                {
                    lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                        as libc::c_short as size_t;
                    1 as libc::c_int
                } else {
                    lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                        as libc::c_short as size_t;
                    0 as libc::c_int
                }
            } else {
                if (if b < 0 as libc::c_int as libc::c_ulong {
                    if q < 0 as libc::c_int as libc::c_ulong {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_add(b)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            (q
                                < ((32767 as libc::c_int * 2 as libc::c_int
                                    + 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_div(b)) as libc::c_int
                        } else {
                            ((if (if (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                !((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            }) < 0 as libc::c_int as libc::c_ulong
                            {
                                (b
                                    < (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_neg()) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(
                                        (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                                    >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                ((32767 as libc::c_int * 2 as libc::c_int
                                    + 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_div(b.wrapping_neg())
                            }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                                as libc::c_int
                        }
                    } else {
                        if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                b
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                        {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                q
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < q.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong) < q
                                    && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                        as libc::c_ulong)
                                        < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            }
                        } else {
                            ((0 as libc::c_int as libc::c_ulong).wrapping_div(b) < q)
                                as libc::c_int
                        }
                    }
                } else {
                    if b == 0 as libc::c_int as libc::c_ulong {
                        0 as libc::c_int
                    } else {
                        if q < 0 as libc::c_int as libc::c_ulong {
                            if (if (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                !((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            }) < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    < (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_neg()) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                            {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < b.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                } else {
                                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                                        < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                }
                            } else {
                                ((0 as libc::c_int as libc::c_ulong).wrapping_div(q) < b)
                                    as libc::c_int
                            }
                        } else {
                            (((32767 as libc::c_int * 2 as libc::c_int
                                + 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_div(b) < q) as libc::c_int
                        }
                    }
                }) != 0
                {
                    lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                        as libc::c_ushort as size_t;
                    1 as libc::c_int
                } else {
                    lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                        as libc::c_ushort as size_t;
                    0 as libc::c_int
                }
            }
        } else {
            if ::std::mem::size_of::<size_t>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    lcm
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    if (if b < 0 as libc::c_int as libc::c_ulong {
                        if q < 0 as libc::c_int as libc::c_ulong {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2147483647 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_add(b)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                (q
                                    < (2147483647 as libc::c_int as libc::c_ulong)
                                        .wrapping_div(b)) as libc::c_int
                            } else {
                                ((if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                }) < 0 as libc::c_int as libc::c_ulong
                                {
                                    (b
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(2147483647 as libc::c_int as libc::c_ulong)
                                        >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (2147483647 as libc::c_int as libc::c_ulong)
                                        .wrapping_div(b.wrapping_neg())
                                })
                                    <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                                    as libc::c_int
                            }
                        } else {
                            if (if (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(
                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                !((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            }) < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(
                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                                    < (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(
                                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(
                                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_neg()) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )) as libc::c_int
                            }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                            {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < q
                                            .wrapping_add(
                                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong) < q
                                        && ((-(1 as libc::c_int)
                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_ulong)
                                            < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                }
                            } else {
                                (((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_div(b) < q) as libc::c_int
                            }
                        }
                    } else {
                        if b == 0 as libc::c_int as libc::c_ulong {
                            0 as libc::c_int
                        } else {
                            if q < 0 as libc::c_int as libc::c_ulong {
                                if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(
                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                }) < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(
                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(
                                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                })
                                                    .wrapping_add(
                                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                })
                                                    .wrapping_add(
                                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong,
                                            )) as libc::c_int
                                }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < b
                                                .wrapping_add(
                                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )) as libc::c_int
                                    } else {
                                        (((-(1 as libc::c_int)
                                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_ulong)
                                            < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }
                                } else {
                                    (((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_div(q) < b) as libc::c_int
                                }
                            } else {
                                ((2147483647 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(b) < q) as libc::c_int
                            }
                        }
                    }) != 0
                    {
                        lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                            as libc::c_int as size_t;
                        1 as libc::c_int
                    } else {
                        lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                            as libc::c_int as size_t;
                        0 as libc::c_int
                    }
                } else {
                    if (if b < 0 as libc::c_int as libc::c_ulong {
                        if q < 0 as libc::c_int as libc::c_ulong {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint)
                                }) as libc::c_ulong)
                                    .wrapping_add(b)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                (q
                                    < ((2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                                        .wrapping_div(b)) as libc::c_int
                            } else {
                                ((if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                }) < 0 as libc::c_int as libc::c_ulong
                                {
                                    (b
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (2147483647 as libc::c_int as libc::c_uint)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_add(1 as libc::c_uint) as libc::c_ulong,
                                        )
                                        >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    ((2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                                        .wrapping_div(b.wrapping_neg())
                                })
                                    <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                                    as libc::c_int
                            }
                        } else {
                            if (if (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                !((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            }) < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    b
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    < (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_neg()) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                            {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    q
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < q.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong) < q
                                        && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                            as libc::c_ulong)
                                            < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                }
                            } else {
                                ((0 as libc::c_int as libc::c_ulong).wrapping_div(b) < q)
                                    as libc::c_int
                            }
                        }
                    } else {
                        if b == 0 as libc::c_int as libc::c_ulong {
                            0 as libc::c_int
                        } else {
                            if q < 0 as libc::c_int as libc::c_ulong {
                                if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                }) < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < b.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    } else {
                                        (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                                            < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong).wrapping_div(q) < b)
                                        as libc::c_int
                                }
                            } else {
                                (((2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                                    .wrapping_div(b) < q) as libc::c_int
                            }
                        }
                    }) != 0
                    {
                        lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                            as size_t;
                        1 as libc::c_int
                    } else {
                        lcm = (q as libc::c_uint).wrapping_mul(b as libc::c_uint)
                            as size_t;
                        0 as libc::c_int
                    }
                }
            } else {
                if ::std::mem::size_of::<size_t>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        lcm
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        if (if b < 0 as libc::c_int as libc::c_ulong {
                            if q < 0 as libc::c_int as libc::c_ulong {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        9223372036854775807 as libc::c_long
                                    }) as libc::c_ulong)
                                        .wrapping_add(b)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    (q
                                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_div(b)) as libc::c_int
                                } else {
                                    ((if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        (b
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                9223372036854775807 as libc::c_long as libc::c_ulong,
                                            )
                                            >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_div(b.wrapping_neg())
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                                        as libc::c_int
                                }
                            } else {
                                if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                }) < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(
                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                as libc::c_ulong,
                                        )
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                    as libc::c_ulong,
                                            )) as libc::c_int
                                }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < q
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                        as libc::c_ulong,
                                                )) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < q
                                            && ((-(1 as libc::c_int) as libc::c_long
                                                - (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)) as libc::c_ulong)
                                                < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }
                                } else {
                                    (((-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long) as libc::c_ulong)
                                        .wrapping_div(b) < q) as libc::c_int
                                }
                            }
                        } else {
                            if b == 0 as libc::c_int as libc::c_ulong {
                                0 as libc::c_int
                            } else {
                                if q < 0 as libc::c_int as libc::c_ulong {
                                    if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                    as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                    as libc::c_ulong,
                                            )
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                })
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    })
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    })
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                        as libc::c_ulong,
                                                )) as libc::c_int
                                    }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                                    {
                                        if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((0 as libc::c_int as libc::c_ulong)
                                                < b
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                            as libc::c_ulong,
                                                    )) as libc::c_int
                                        } else {
                                            (((-(1 as libc::c_int) as libc::c_long
                                                - (-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long)) as libc::c_ulong)
                                                < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        }
                                    } else {
                                        (((-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long) as libc::c_ulong)
                                            .wrapping_div(q) < b) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_long as libc::c_ulong)
                                        .wrapping_div(b) < q) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            lcm = q.wrapping_mul(b) as libc::c_long as size_t;
                            1 as libc::c_int
                        } else {
                            lcm = q.wrapping_mul(b) as libc::c_long as size_t;
                            0 as libc::c_int
                        }
                    } else {
                        if (if b < 0 as libc::c_int as libc::c_ulong {
                            if q < 0 as libc::c_int as libc::c_ulong {
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
                                        .wrapping_add(b)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    (q
                                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_ulong)
                                            .wrapping_div(b)) as libc::c_int
                                } else {
                                    ((if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        (b
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(
                                                (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_ulong),
                                            )
                                            >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (9223372036854775807 as libc::c_long as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_ulong)
                                            .wrapping_div(b.wrapping_neg())
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q))
                                        as libc::c_int
                                }
                            } else {
                                if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                }) < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < q.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < q
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_ulong)
                                                < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong).wrapping_div(b) < q)
                                        as libc::c_int
                                }
                            }
                        } else {
                            if b == 0 as libc::c_int as libc::c_ulong {
                                0 as libc::c_int
                            } else {
                                if q < 0 as libc::c_int as libc::c_ulong {
                                    if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                                    {
                                        if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((0 as libc::c_int as libc::c_ulong)
                                                < b.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        } else {
                                            (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                                                < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        }
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong).wrapping_div(q) < b)
                                            as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_long as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_ulong)
                                        .wrapping_div(b) < q) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            lcm = q.wrapping_mul(b);
                            1 as libc::c_int
                        } else {
                            lcm = q.wrapping_mul(b);
                            0 as libc::c_int
                        }
                    }
                } else {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        lcm
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        if (if b < 0 as libc::c_int as libc::c_ulong {
                            if q < 0 as libc::c_int as libc::c_ulong {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulonglong
                                } else {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_longlong
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                    }) as libc::c_ulonglong)
                                        .wrapping_add(b as libc::c_ulonglong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                    < 0 as libc::c_int as libc::c_ulonglong
                                {
                                    ((q as libc::c_ulonglong)
                                        < (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_div(b as libc::c_ulonglong)) as libc::c_int
                                } else {
                                    ((if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        (b
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                                    }) != 0
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                9223372036854775807 as libc::c_longlong as libc::c_ulonglong,
                                            )
                                            >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_div(b.wrapping_neg() as libc::c_ulonglong)
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q)
                                            as libc::c_ulonglong) as libc::c_int
                                }
                            } else {
                                if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulonglong
                                } else {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    }) as libc::c_ulonglong)
                                        .wrapping_add(
                                            (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                    < 0 as libc::c_int as libc::c_ulonglong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulonglong
                                    } else {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                            )
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                        << (::std::mem::size_of::<libc::c_ulonglong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulonglong
                                    } else {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                            )
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulonglong)
                                }) < 0 as libc::c_int as libc::c_ulonglong
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    }) as libc::c_ulonglong)
                                        .wrapping_add(
                                            (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                                        )
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulonglong
                                        } else {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            }) as libc::c_ulonglong)
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                )
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                            < 0 as libc::c_int as libc::c_ulonglong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulonglong
                                            } else {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                }) as libc::c_ulonglong)
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                    )
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                                << (::std::mem::size_of::<libc::c_ulonglong>()
                                                    as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulonglong
                                            } else {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                }) as libc::c_ulonglong)
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                    )
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulonglong)
                                        < ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                            )) as libc::c_int
                                }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulonglong)
                                            < (q as libc::c_ulonglong)
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                )) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < q
                                            && ((-(1 as libc::c_int) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as libc::c_ulonglong)
                                                < q.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    as libc::c_ulonglong) as libc::c_int
                                    }
                                } else {
                                    (((-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) as libc::c_ulonglong)
                                        .wrapping_div(b as libc::c_ulonglong)
                                        < q as libc::c_ulonglong) as libc::c_int
                                }
                            }
                        } else {
                            if b == 0 as libc::c_int as libc::c_ulong {
                                0 as libc::c_int
                            } else {
                                if q < 0 as libc::c_int as libc::c_ulong {
                                    if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulonglong
                                    } else {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                            )
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                        < 0 as libc::c_int as libc::c_ulonglong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulonglong
                                        } else {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            }) as libc::c_ulonglong)
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                )
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                            << (::std::mem::size_of::<libc::c_ulonglong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulonglong
                                        } else {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            }) as libc::c_ulonglong)
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                )
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulonglong)
                                    }) < 0 as libc::c_int as libc::c_ulonglong
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                            )
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulonglong
                                            } else {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                }) as libc::c_ulonglong)
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                    )
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                < 0 as libc::c_int as libc::c_ulonglong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulonglong
                                                } else {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    }) as libc::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                        )
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                                    << (::std::mem::size_of::<libc::c_ulonglong>()
                                                        as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulonglong
                                                } else {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    }) as libc::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                        )
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulonglong)
                                            < ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            }) as libc::c_ulonglong)
                                                .wrapping_add(
                                                    (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                )) as libc::c_int
                                    }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                                    {
                                        if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((0 as libc::c_int as libc::c_ulonglong)
                                                < (b as libc::c_ulonglong)
                                                    .wrapping_add(
                                                        (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                    )) as libc::c_int
                                        } else {
                                            (((-(1 as libc::c_int) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as libc::c_ulonglong)
                                                < b.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    as libc::c_ulonglong) as libc::c_int
                                        }
                                    } else {
                                        (((-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong) as libc::c_ulonglong)
                                            .wrapping_div(q as libc::c_ulonglong)
                                            < b as libc::c_ulonglong) as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        as libc::c_ulonglong)
                                        .wrapping_div(b as libc::c_ulonglong)
                                        < q as libc::c_ulonglong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            lcm = (q as libc::c_ulonglong)
                                .wrapping_mul(b as libc::c_ulonglong) as libc::c_longlong
                                as size_t;
                            1 as libc::c_int
                        } else {
                            lcm = (q as libc::c_ulonglong)
                                .wrapping_mul(b as libc::c_ulonglong) as libc::c_longlong
                                as size_t;
                            0 as libc::c_int
                        }
                    } else {
                        if (if b < 0 as libc::c_int as libc::c_ulong {
                            if q < 0 as libc::c_int as libc::c_ulong {
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
                                        .wrapping_add(b as libc::c_ulonglong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                    < 0 as libc::c_int as libc::c_ulonglong
                                {
                                    ((q as libc::c_ulonglong)
                                        < (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(b as libc::c_ulonglong)) as libc::c_int
                                } else {
                                    ((if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        (b
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < b) as libc::c_int
                                    }) != 0
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        }) as libc::c_ulonglong)
                                            .wrapping_add(
                                                (9223372036854775807 as libc::c_longlong
                                                    as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_ulonglong),
                                            )
                                            >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(b.wrapping_neg() as libc::c_ulonglong)
                                    })
                                        <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(q)
                                            as libc::c_ulonglong) as libc::c_int
                                }
                            } else {
                                if (if (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    !((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                }) < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        b
                                    })
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        < (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                b
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    b
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_neg()) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                        as libc::c_int
                                }) != 0 && b == -(1 as libc::c_int) as libc::c_ulong
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        q
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < q.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < q
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_ulong)
                                                < q.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }
                                } else {
                                    ((0 as libc::c_int as libc::c_ulong).wrapping_div(b) < q)
                                        as libc::c_int
                                }
                            }
                        } else {
                            if b == 0 as libc::c_int as libc::c_ulong {
                                0 as libc::c_int
                            } else {
                                if q < 0 as libc::c_int as libc::c_ulong {
                                    if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            q
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    q
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        q
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                q
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }) != 0 && q == -(1 as libc::c_int) as libc::c_ulong
                                    {
                                        if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            b
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((0 as libc::c_int as libc::c_ulong)
                                                < b.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        } else {
                                            (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                                                < b.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        }
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong).wrapping_div(q) < b)
                                            as libc::c_int
                                    }
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        as libc::c_ulonglong)
                                        .wrapping_mul(2 as libc::c_ulonglong)
                                        .wrapping_add(1 as libc::c_ulonglong)
                                        .wrapping_div(b as libc::c_ulonglong)
                                        < q as libc::c_ulonglong) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            lcm = (q as libc::c_ulonglong)
                                .wrapping_mul(b as libc::c_ulonglong) as size_t;
                            1 as libc::c_int
                        } else {
                            lcm = (q as libc::c_ulonglong)
                                .wrapping_mul(b as libc::c_ulonglong) as size_t;
                            0 as libc::c_int
                        }
                    }
                }
            }
        }
    }) == 0 && lcm <= lcm_max
    {
        lcm
    } else {
        a
    };
}
