use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
pub unsafe extern "C" fn __libc_dynarray_resize(
    mut list: *mut dynarray_header,
    mut size: size_t,
    mut scratch: *mut libc::c_void,
    mut element_size: size_t,
) -> bool {
    if size <= (*list).allocated {
        (*list).used = size;
        return 1 as libc::c_int != 0;
    }
    let mut new_size_bytes: size_t = 0;
    if if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if size < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int
                        }) as libc::c_ulong)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (size
                            < (127 as libc::c_int as libc::c_ulong)
                                .wrapping_div(element_size)) as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
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
                                element_size
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (element_size
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        element_size
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
                                        element_size
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < element_size)
                                as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_add(127 as libc::c_int as libc::c_ulong)
                                >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (127 as libc::c_int as libc::c_ulong)
                                .wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                            as libc::c_int
                    }
                } else if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
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
                            element_size
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
                            element_size
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
                        element_size
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
                                element_size
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
                                    element_size
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
                                    element_size
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
                            element_size
                        })
                            .wrapping_add(
                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < size
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < size
                            && ((-(1 as libc::c_int)
                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_ulong)
                                < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(element_size) < size) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if size < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
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
                            size
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
                            size
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
                        size
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
                                size
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
                                    size
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
                                    size
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
                            size
                        })
                            .wrapping_add(
                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < element_size
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int)
                            - (-(127 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_ulong)
                            < element_size
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(size) < element_size) as libc::c_int
                }
            } else {
                ((127 as libc::c_int as libc::c_ulong).wrapping_div(element_size) < size)
                    as libc::c_int
            } != 0
            {
                new_size_bytes = (size as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_schar
                    as size_t;
                1 as libc::c_int
            } else {
                new_size_bytes = (size as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_schar
                    as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if size < 0 as libc::c_int as libc::c_ulong {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    }) as libc::c_ulong)
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    (size
                        < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_div(element_size)) as libc::c_int
                } else {
                    ((if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        (element_size
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
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
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < element_size)
                            as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                        as libc::c_int
                }
            } else if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
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
                        element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                                element_size
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
                                element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < size
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size) < size)
                    as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if size < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
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
                        size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            size
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
                                size
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
                                size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < element_size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                        < element_size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(size) < element_size)
                    as libc::c_int
            }
        } else {
            (((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_div(element_size) < size) as libc::c_int
        } != 0
        {
            new_size_bytes = (size as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_uchar as size_t;
            1 as libc::c_int
        } else {
            new_size_bytes = (size as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_uchar as size_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
    {
        if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if size < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            32767 as libc::c_int
                        }) as libc::c_ulong)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (size
                            < (32767 as libc::c_int as libc::c_ulong)
                                .wrapping_div(element_size)) as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
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
                                element_size
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (element_size
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        element_size
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
                                        element_size
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < element_size)
                                as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_add(32767 as libc::c_int as libc::c_ulong)
                                >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (32767 as libc::c_int as libc::c_ulong)
                                .wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                            as libc::c_int
                    }
                } else if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_add(
                            (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
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
                            element_size
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
                            element_size
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
                        element_size
                    })
                        .wrapping_add(
                            (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                        )
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
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
                                    element_size
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
                                    element_size
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
                            element_size
                        })
                            .wrapping_add(
                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < size
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < size
                            && ((-(1 as libc::c_int)
                                - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_ulong)
                                < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(element_size) < size) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if size < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
                    })
                        .wrapping_add(
                            (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
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
                            size
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
                            size
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
                        size
                    })
                        .wrapping_add(
                            (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                        )
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                size
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
                                    size
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
                                    size
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
                            size
                        })
                            .wrapping_add(
                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < element_size
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int)
                            - (-(32767 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_ulong)
                            < element_size
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(size) < element_size) as libc::c_int
                }
            } else {
                ((32767 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < size) as libc::c_int
            } != 0
            {
                new_size_bytes = (size as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_short
                    as size_t;
                1 as libc::c_int
            } else {
                new_size_bytes = (size as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_short
                    as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if size < 0 as libc::c_int as libc::c_ulong {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    }) as libc::c_ulong)
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    (size
                        < ((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_div(element_size)) as libc::c_int
                } else {
                    ((if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        (element_size
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
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
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < element_size)
                            as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
                        })
                            .wrapping_add(
                                (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                    as libc::c_ulong,
                            )
                            >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        ((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                        as libc::c_int
                }
            } else if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
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
                        element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                                element_size
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
                                element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < size
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size) < size)
                    as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if size < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
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
                        size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            size
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
                                size
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
                                size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < element_size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                        < element_size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(size) < element_size)
                    as libc::c_int
            }
        } else {
            (((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_div(element_size) < size) as libc::c_int
        } != 0
        {
            new_size_bytes = (size as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_ushort as size_t;
            1 as libc::c_int
        } else {
            new_size_bytes = (size as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_ushort as size_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            new_size_bytes
        })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < 0 as libc::c_int as libc::c_ulong
        {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if size < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2147483647 as libc::c_int
                        }) as libc::c_ulong)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (size
                            < (2147483647 as libc::c_int as libc::c_ulong)
                                .wrapping_div(element_size)) as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
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
                                element_size
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (element_size
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        element_size
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
                                        element_size
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < element_size)
                                as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_add(2147483647 as libc::c_int as libc::c_ulong)
                                >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (2147483647 as libc::c_int as libc::c_ulong)
                                .wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                            as libc::c_int
                    }
                } else if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
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
                            element_size
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
                            element_size
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
                        element_size
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
                                element_size
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
                                    element_size
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
                                    element_size
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
                            element_size
                        })
                            .wrapping_add(
                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < size
                                .wrapping_add(
                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < size
                            && ((-(1 as libc::c_int)
                                - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_ulong)
                                < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(element_size) < size) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if size < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
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
                            size
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
                            size
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
                        size
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
                                size
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
                                    size
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
                                    size
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
                            size
                        })
                            .wrapping_add(
                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < element_size
                                .wrapping_add(
                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int)
                            - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_ulong)
                            < element_size
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(size) < element_size) as libc::c_int
                }
            } else {
                ((2147483647 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < size) as libc::c_int
            } != 0
            {
                new_size_bytes = (size as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_int as size_t;
                1 as libc::c_int
            } else {
                new_size_bytes = (size as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_int as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if size < 0 as libc::c_int as libc::c_ulong {
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
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    (size
                        < ((2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                            .wrapping_div(element_size)) as libc::c_int
                } else {
                    ((if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        (element_size
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
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
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < element_size)
                            as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                        as libc::c_int
                }
            } else if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
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
                        element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                                element_size
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
                                element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < size
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size) < size)
                    as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if size < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
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
                        size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            size
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
                                size
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
                                size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < element_size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                        < element_size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(size) < element_size)
                    as libc::c_int
            }
        } else {
            (((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                .wrapping_div(element_size) < size) as libc::c_int
        } != 0
        {
            new_size_bytes = (size as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as size_t;
            1 as libc::c_int
        } else {
            new_size_bytes = (size as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as size_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
    {
        if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            new_size_bytes
        })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < 0 as libc::c_int as libc::c_ulong
        {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if size < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            9223372036854775807 as libc::c_long
                        }) as libc::c_ulong)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        (size
                            < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_div(element_size)) as libc::c_int
                    } else {
                        ((if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
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
                                element_size
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (element_size
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        element_size
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
                                        element_size
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < element_size)
                                as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_add(
                                    9223372036854775807 as libc::c_long as libc::c_ulong,
                                )
                                >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                            as libc::c_int
                    }
                } else if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
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
                            element_size
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
                            element_size
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
                        element_size
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
                                element_size
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
                                    element_size
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
                                    element_size
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
                            element_size
                        })
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < size
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < size
                            && ((-(1 as libc::c_int) as libc::c_long
                                - (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_ulong)
                                < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        as libc::c_ulong)
                        .wrapping_div(element_size) < size) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if size < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
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
                            size
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
                            size
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
                        size
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
                                size
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
                                    size
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
                                    size
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
                            size
                        })
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < element_size
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        (((-(1 as libc::c_int) as libc::c_long
                            - (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)) as libc::c_ulong)
                            < element_size
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        as libc::c_ulong)
                        .wrapping_div(size) < element_size) as libc::c_int
                }
            } else {
                ((9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_div(element_size) < size) as libc::c_int
            } != 0
            {
                new_size_bytes = size.wrapping_mul(element_size) as libc::c_long
                    as size_t;
                1 as libc::c_int
            } else {
                new_size_bytes = size.wrapping_mul(element_size) as libc::c_long
                    as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if size < 0 as libc::c_int as libc::c_ulong {
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
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    (size
                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(element_size)) as libc::c_int
                } else {
                    ((if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        (element_size
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
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
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < element_size)
                            as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size))
                        as libc::c_int
                }
            } else if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
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
                        element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                                element_size
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
                                element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < size
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size) < size)
                    as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if size < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
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
                        size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            size
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
                                size
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
                                size
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
                        size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < element_size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                        < element_size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(size) < element_size)
                    as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_div(element_size) < size) as libc::c_int
        } != 0
        {
            new_size_bytes = size.wrapping_mul(element_size);
            1 as libc::c_int
        } else {
            new_size_bytes = size.wrapping_mul(element_size);
            0 as libc::c_int
        }
    } else if (if 1 as libc::c_int != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        new_size_bytes
    })
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        < 0 as libc::c_int as libc::c_ulong
    {
        if if element_size < 0 as libc::c_int as libc::c_ulong {
            if size < 0 as libc::c_int as libc::c_ulong {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulonglong
                } else {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        9223372036854775807 as libc::c_longlong
                    }) as libc::c_ulonglong)
                        .wrapping_add(element_size as libc::c_ulonglong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                    < 0 as libc::c_int as libc::c_ulonglong
                {
                    ((size as libc::c_ulonglong)
                        < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_div(element_size as libc::c_ulonglong))
                        as libc::c_int
                } else {
                    ((if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
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
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        (element_size
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    element_size
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
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < element_size)
                            as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
                        }) as libc::c_ulonglong)
                            .wrapping_add(
                                9223372036854775807 as libc::c_longlong as libc::c_ulonglong,
                            )
                            >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_div(
                                element_size.wrapping_neg() as libc::c_ulonglong,
                            )
                    })
                        <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size)
                            as libc::c_ulonglong) as libc::c_int
                }
            } else if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulonglong
            } else {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
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
                        element_size
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                    << (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
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
                        element_size
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
                    element_size
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
                            element_size
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
                                element_size
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
                                element_size
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
                        element_size
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )) as libc::c_int
            }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulonglong)
                        < (size as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < size
                        && ((-(1 as libc::c_int) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_ulonglong)
                            < size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as libc::c_ulonglong) as libc::c_int
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    as libc::c_ulonglong)
                    .wrapping_div(element_size as libc::c_ulonglong)
                    < size as libc::c_ulonglong) as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if size < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulonglong
            } else {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    size
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
                        size
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                    << (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
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
                        size
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
                    size
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
                            size
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
                                size
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
                                size
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
                        size
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )) as libc::c_int
            }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulonglong)
                        < (element_size as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )) as libc::c_int
                } else {
                    (((-(1 as libc::c_int) as libc::c_longlong
                        - (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as libc::c_ulonglong)
                        < element_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as libc::c_ulonglong) as libc::c_int
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    as libc::c_ulonglong)
                    .wrapping_div(size as libc::c_ulonglong)
                    < element_size as libc::c_ulonglong) as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_div(element_size as libc::c_ulonglong)
                < size as libc::c_ulonglong) as libc::c_int
        } != 0
        {
            new_size_bytes = (size as libc::c_ulonglong)
                .wrapping_mul(element_size as libc::c_ulonglong) as libc::c_longlong
                as size_t;
            1 as libc::c_int
        } else {
            new_size_bytes = (size as libc::c_ulonglong)
                .wrapping_mul(element_size as libc::c_ulonglong) as libc::c_longlong
                as size_t;
            0 as libc::c_int
        }
    } else if if element_size < 0 as libc::c_int as libc::c_ulong {
        if size < 0 as libc::c_int as libc::c_ulong {
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
                    .wrapping_add(element_size as libc::c_ulonglong)
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                < 0 as libc::c_int as libc::c_ulonglong
            {
                ((size as libc::c_ulonglong)
                    < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(element_size as libc::c_ulonglong)) as libc::c_int
            } else {
                ((if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    element_size
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    !((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
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
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                }) < 0 as libc::c_int as libc::c_ulong
                {
                    (element_size
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                element_size
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
                                element_size
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < element_size) as libc::c_int
                }) != 0
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
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
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(element_size.wrapping_neg() as libc::c_ulonglong)
                })
                    <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(size)
                        as libc::c_ulonglong) as libc::c_int
            }
        } else if (if (if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                element_size
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
                    element_size
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
                    element_size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            })
                .wrapping_add(0 as libc::c_int as libc::c_ulong)
        }) < 0 as libc::c_int as libc::c_ulong
        {
            ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                element_size
            })
                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                < (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        element_size
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
                            element_size
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
                            element_size
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
                    element_size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
        }) != 0 && element_size == -(1 as libc::c_int) as libc::c_ulong
        {
            if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                size
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            {
                ((0 as libc::c_int as libc::c_ulong)
                    < size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                    as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_ulong) < size
                    && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                        < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as libc::c_int
            }
        } else {
            ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size) < size)
                as libc::c_int
        }
    } else if element_size == 0 as libc::c_int as libc::c_ulong {
        0 as libc::c_int
    } else if size < 0 as libc::c_int as libc::c_ulong {
        if (if (if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                size
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
                    size
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
                    size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            })
                .wrapping_add(0 as libc::c_int as libc::c_ulong)
        }) < 0 as libc::c_int as libc::c_ulong
        {
            ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                size
            })
                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                < (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        size
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
                            size
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
                            size
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
                    size
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
        }) != 0 && size == -(1 as libc::c_int) as libc::c_ulong
        {
            if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                element_size
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            {
                ((0 as libc::c_int as libc::c_ulong)
                    < element_size.wrapping_add(0 as libc::c_int as libc::c_ulong))
                    as libc::c_int
            } else {
                (((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                    < element_size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as libc::c_int
            }
        } else {
            ((0 as libc::c_int as libc::c_ulong).wrapping_div(size) < element_size)
                as libc::c_int
        }
    } else {
        ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(element_size as libc::c_ulonglong) < size as libc::c_ulonglong)
            as libc::c_int
    } != 0
    {
        new_size_bytes = (size as libc::c_ulonglong)
            .wrapping_mul(element_size as libc::c_ulonglong) as size_t;
        1 as libc::c_int
    } else {
        new_size_bytes = (size as libc::c_ulonglong)
            .wrapping_mul(element_size as libc::c_ulonglong) as size_t;
        0 as libc::c_int
    } != 0
    {
        *__errno_location() = 12 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    let mut new_array: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*list).array == scratch {
        new_array = malloc(new_size_bytes);
        if !new_array.is_null() && !((*list).array).is_null() {
            memcpy(new_array, (*list).array, ((*list).used).wrapping_mul(element_size));
        }
    } else {
        new_array = realloc((*list).array, new_size_bytes);
    }
    if new_array.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*list).array = new_array;
    (*list).allocated = size;
    (*list).used = size;
    return 1 as libc::c_int != 0;
}
