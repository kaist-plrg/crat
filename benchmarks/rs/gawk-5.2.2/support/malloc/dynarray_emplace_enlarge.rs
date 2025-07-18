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
pub unsafe extern "C" fn __libc_dynarray_emplace_enlarge(
    mut list: *mut dynarray_header,
    mut scratch: *mut libc::c_void,
    mut element_size: size_t,
) -> bool {
    let mut new_allocated: size_t = 0;
    if (*list).allocated == 0 as libc::c_int as libc::c_ulong {
        if element_size < 4 as libc::c_int as libc::c_ulong {
            new_allocated = 16 as libc::c_int as size_t;
        } else if element_size < 8 as libc::c_int as libc::c_ulong {
            new_allocated = 8 as libc::c_int as size_t;
        } else {
            new_allocated = 4 as libc::c_int as size_t;
        }
    } else {
        new_allocated = ((*list).allocated)
            .wrapping_add(
                ((*list).allocated).wrapping_div(2 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if new_allocated <= (*list).allocated {
            *__errno_location() = 12 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
    }
    let mut new_size: size_t = 0;
    if if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                        (new_allocated
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
                        })
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(new_allocated)) as libc::c_int
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
                        new_allocated
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < new_allocated
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < new_allocated
                            && ((-(1 as libc::c_int)
                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_ulong)
                                < new_allocated
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(element_size) < new_allocated) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        new_allocated
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
                            new_allocated
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
                            new_allocated
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
                        new_allocated
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
                                new_allocated
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
                                    new_allocated
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
                                    new_allocated
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
                            new_allocated
                        })
                            .wrapping_add(
                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                        .wrapping_div(new_allocated) < element_size) as libc::c_int
                }
            } else {
                ((127 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < new_allocated) as libc::c_int
            } != 0
            {
                new_size = (new_allocated as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_schar
                    as size_t;
                1 as libc::c_int
            } else {
                new_size = (new_allocated as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_schar
                    as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                    (new_allocated
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
                    })
                        <= (-(1 as libc::c_int) as libc::c_ulong)
                            .wrapping_sub(new_allocated)) as libc::c_int
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
                    new_allocated
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < new_allocated.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < new_allocated
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < new_allocated
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < new_allocated) as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
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
                        new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            new_allocated
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
                                new_allocated
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
                                new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(new_allocated)
                    < element_size) as libc::c_int
            }
        } else {
            (((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_div(element_size) < new_allocated) as libc::c_int
        } != 0
        {
            new_size = (new_allocated as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_uchar as size_t;
            1 as libc::c_int
        } else {
            new_size = (new_allocated as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_uchar as size_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
    {
        if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                        (new_allocated
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
                        })
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(new_allocated)) as libc::c_int
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
                        new_allocated
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < new_allocated
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < new_allocated
                            && ((-(1 as libc::c_int)
                                - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_ulong)
                                < new_allocated
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(element_size) < new_allocated) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        new_allocated
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
                            new_allocated
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
                            new_allocated
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
                        new_allocated
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
                                new_allocated
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
                                    new_allocated
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
                                    new_allocated
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
                            new_allocated
                        })
                            .wrapping_add(
                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                        .wrapping_div(new_allocated) < element_size) as libc::c_int
                }
            } else {
                ((32767 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < new_allocated) as libc::c_int
            } != 0
            {
                new_size = (new_allocated as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_short
                    as size_t;
                1 as libc::c_int
            } else {
                new_size = (new_allocated as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_short
                    as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                    (new_allocated
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
                    })
                        <= (-(1 as libc::c_int) as libc::c_ulong)
                            .wrapping_sub(new_allocated)) as libc::c_int
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
                    new_allocated
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < new_allocated.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < new_allocated
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < new_allocated
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < new_allocated) as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
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
                        new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            new_allocated
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
                                new_allocated
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
                                new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(new_allocated)
                    < element_size) as libc::c_int
            }
        } else {
            (((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_div(element_size) < new_allocated) as libc::c_int
        } != 0
        {
            new_size = (new_allocated as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_ushort as size_t;
            1 as libc::c_int
        } else {
            new_size = (new_allocated as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as libc::c_ushort as size_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            new_size
        })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < 0 as libc::c_int as libc::c_ulong
        {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                        (new_allocated
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
                        })
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(new_allocated)) as libc::c_int
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
                        new_allocated
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < new_allocated
                                .wrapping_add(
                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < new_allocated
                            && ((-(1 as libc::c_int)
                                - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_ulong)
                                < new_allocated
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_div(element_size) < new_allocated) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        new_allocated
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
                            new_allocated
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
                            new_allocated
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
                        new_allocated
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
                                new_allocated
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
                                    new_allocated
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
                                    new_allocated
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
                            new_allocated
                        })
                            .wrapping_add(
                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                        .wrapping_div(new_allocated) < element_size) as libc::c_int
                }
            } else {
                ((2147483647 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < new_allocated) as libc::c_int
            } != 0
            {
                new_size = (new_allocated as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_int as size_t;
                1 as libc::c_int
            } else {
                new_size = (new_allocated as libc::c_uint)
                    .wrapping_mul(element_size as libc::c_uint) as libc::c_int as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                    (new_allocated
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
                    })
                        <= (-(1 as libc::c_int) as libc::c_ulong)
                            .wrapping_sub(new_allocated)) as libc::c_int
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
                    new_allocated
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < new_allocated.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < new_allocated
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < new_allocated
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < new_allocated) as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
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
                        new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            new_allocated
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
                                new_allocated
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
                                new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(new_allocated)
                    < element_size) as libc::c_int
            }
        } else {
            (((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                .wrapping_div(element_size) < new_allocated) as libc::c_int
        } != 0
        {
            new_size = (new_allocated as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as size_t;
            1 as libc::c_int
        } else {
            new_size = (new_allocated as libc::c_uint)
                .wrapping_mul(element_size as libc::c_uint) as size_t;
            0 as libc::c_int
        }
    } else if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
    {
        if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            new_size
        })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < 0 as libc::c_int as libc::c_ulong
        {
            if if element_size < 0 as libc::c_int as libc::c_ulong {
                if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                        (new_allocated
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
                        })
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(new_allocated)) as libc::c_int
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
                        new_allocated
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < new_allocated
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < new_allocated
                            && ((-(1 as libc::c_int) as libc::c_long
                                - (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_ulong)
                                < new_allocated
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                        as libc::c_ulong)
                        .wrapping_div(element_size) < new_allocated) as libc::c_int
                }
            } else if element_size == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int
            } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        new_allocated
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
                            new_allocated
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
                            new_allocated
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
                        new_allocated
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
                                new_allocated
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
                                    new_allocated
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
                                    new_allocated
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
                            new_allocated
                        })
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                    as libc::c_ulong,
                            )) as libc::c_int
                }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                        .wrapping_div(new_allocated) < element_size) as libc::c_int
                }
            } else {
                ((9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_div(element_size) < new_allocated) as libc::c_int
            } != 0
            {
                new_size = new_allocated.wrapping_mul(element_size) as libc::c_long
                    as size_t;
                1 as libc::c_int
            } else {
                new_size = new_allocated.wrapping_mul(element_size) as libc::c_long
                    as size_t;
                0 as libc::c_int
            }
        } else if if element_size < 0 as libc::c_int as libc::c_ulong {
            if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                    (new_allocated
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
                    })
                        <= (-(1 as libc::c_int) as libc::c_ulong)
                            .wrapping_sub(new_allocated)) as libc::c_int
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
                    new_allocated
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < new_allocated.wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < new_allocated
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < new_allocated
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                    < new_allocated) as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
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
                        new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            new_allocated
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
                                new_allocated
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
                                new_allocated
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
                        new_allocated
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                ((0 as libc::c_int as libc::c_ulong).wrapping_div(new_allocated)
                    < element_size) as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_div(element_size) < new_allocated) as libc::c_int
        } != 0
        {
            new_size = new_allocated.wrapping_mul(element_size);
            1 as libc::c_int
        } else {
            new_size = new_allocated.wrapping_mul(element_size);
            0 as libc::c_int
        }
    } else if (if 1 as libc::c_int != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        new_size
    })
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        < 0 as libc::c_int as libc::c_ulong
    {
        if if element_size < 0 as libc::c_int as libc::c_ulong {
            if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                    ((new_allocated as libc::c_ulonglong)
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
                        <= (-(1 as libc::c_int) as libc::c_ulong)
                            .wrapping_sub(new_allocated) as libc::c_ulonglong)
                        as libc::c_int
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
                    new_allocated
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulonglong)
                        < (new_allocated as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < new_allocated
                        && ((-(1 as libc::c_int) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_ulonglong)
                            < new_allocated
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as libc::c_ulonglong) as libc::c_int
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    as libc::c_ulonglong)
                    .wrapping_div(element_size as libc::c_ulonglong)
                    < new_allocated as libc::c_ulonglong) as libc::c_int
            }
        } else if element_size == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulonglong
            } else {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    new_allocated
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
                        new_allocated
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
                        new_allocated
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
                    new_allocated
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
                            new_allocated
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
                                new_allocated
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
                                new_allocated
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
                        new_allocated
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )) as libc::c_int
            }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
                    .wrapping_div(new_allocated as libc::c_ulonglong)
                    < element_size as libc::c_ulonglong) as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_div(element_size as libc::c_ulonglong)
                < new_allocated as libc::c_ulonglong) as libc::c_int
        } != 0
        {
            new_size = (new_allocated as libc::c_ulonglong)
                .wrapping_mul(element_size as libc::c_ulonglong) as libc::c_longlong
                as size_t;
            1 as libc::c_int
        } else {
            new_size = (new_allocated as libc::c_ulonglong)
                .wrapping_mul(element_size as libc::c_ulonglong) as libc::c_longlong
                as size_t;
            0 as libc::c_int
        }
    } else if if element_size < 0 as libc::c_int as libc::c_ulong {
        if new_allocated < 0 as libc::c_int as libc::c_ulong {
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
                ((new_allocated as libc::c_ulonglong)
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
                    <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(new_allocated)
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
                new_allocated
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            {
                ((0 as libc::c_int as libc::c_ulong)
                    < new_allocated.wrapping_add(0 as libc::c_int as libc::c_ulong))
                    as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_ulong) < new_allocated
                    && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                        < new_allocated.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as libc::c_int
            }
        } else {
            ((0 as libc::c_int as libc::c_ulong).wrapping_div(element_size)
                < new_allocated) as libc::c_int
        }
    } else if element_size == 0 as libc::c_int as libc::c_ulong {
        0 as libc::c_int
    } else if new_allocated < 0 as libc::c_int as libc::c_ulong {
        if (if (if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                new_allocated
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
                    new_allocated
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
                    new_allocated
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            })
                .wrapping_add(0 as libc::c_int as libc::c_ulong)
        }) < 0 as libc::c_int as libc::c_ulong
        {
            ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                new_allocated
            })
                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                < (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        new_allocated
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
                            new_allocated
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
                            new_allocated
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
                    new_allocated
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
        }) != 0 && new_allocated == -(1 as libc::c_int) as libc::c_ulong
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
            ((0 as libc::c_int as libc::c_ulong).wrapping_div(new_allocated)
                < element_size) as libc::c_int
        }
    } else {
        ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(element_size as libc::c_ulonglong)
            < new_allocated as libc::c_ulonglong) as libc::c_int
    } != 0
    {
        new_size = (new_allocated as libc::c_ulonglong)
            .wrapping_mul(element_size as libc::c_ulonglong) as size_t;
        1 as libc::c_int
    } else {
        new_size = (new_allocated as libc::c_ulonglong)
            .wrapping_mul(element_size as libc::c_ulonglong) as size_t;
        0 as libc::c_int
    } != 0
    {
        return 0 as libc::c_int != 0;
    }
    let mut new_array: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*list).array == scratch {
        new_array = malloc(new_size);
        if !new_array.is_null() && !((*list).array).is_null() {
            memcpy(new_array, (*list).array, ((*list).used).wrapping_mul(element_size));
        }
    } else {
        new_array = realloc((*list).array, new_size);
    }
    if new_array.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*list).array = new_array;
    (*list).allocated = new_allocated;
    return 1 as libc::c_int != 0;
}
