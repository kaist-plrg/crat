use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type ctor_f = unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> ();
pub type dtor_f = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type init_f = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_icd {
    pub sz: size_t,
    pub init: Option::<init_f>,
    pub copy: Option::<ctor_f>,
    pub dtor: Option::<dtor_f>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_array {
    pub i: libc::c_uint,
    pub n: libc::c_uint,
    pub icd: UT_icd,
    pub d: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intpair_t {
    pub a: libc::c_int,
    pub b: libc::c_int,
}
unsafe fn main_0() -> libc::c_int {
    let mut pairs: *mut UT_array = 0 as *mut UT_array;
    let mut pairs_cpy: *mut UT_array = 0 as *mut UT_array;
    let mut it: intpair_t = intpair_t { a: 0, b: 0 };
    let mut ip: *mut intpair_t = 0 as *mut intpair_t;
    let mut pairicd: UT_icd = {
        let mut init = UT_icd {
            sz: ::std::mem::size_of::<intpair_t>() as libc::c_ulong,
            init: None,
            copy: None,
            dtor: None,
        };
        init
    };
    let mut zero: size_t = 0 as libc::c_int as size_t;
    pairs = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong) as *mut UT_array;
    if pairs.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        pairs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*pairs).icd = pairicd;
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    it.a = 1 as libc::c_int;
    it.b = 2 as libc::c_int;
    if ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
        let mut utarray_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
            (*pairs)
                .n = if (*pairs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp = realloc(
            (*pairs).d as *mut libc::c_void,
            ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs).d = utarray_tmp;
    }
    if ((*pairs).icd.copy).is_some() {
        let fresh0 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        ((*pairs).icd.copy)
            .unwrap()(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh0 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh1 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh1 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    printf(b"push\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    ip = (if (*pairs).i != 0 {
        ((*pairs).d)
            .offset(
                ((*pairs).icd.sz)
                    .wrapping_mul(
                        ((*pairs).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as *mut intpair_t;
    printf(b"back is %d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    if ((*pairs).icd.dtor).is_some() {
        (*pairs).i = ((*pairs).i).wrapping_sub(1);
        ((*pairs).icd.dtor)
            .unwrap()(
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz).wrapping_mul((*pairs).i as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
        );
    } else {
        (*pairs).i = ((*pairs).i).wrapping_sub(1);
        (*pairs).i;
    }
    printf(b"pop\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    it.a = 1 as libc::c_int;
    it.b = 2 as libc::c_int;
    if ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
        let mut utarray_tmp_0: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
            (*pairs)
                .n = if (*pairs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_0 = realloc(
            (*pairs).d as *mut libc::c_void,
            ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_0.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs).d = utarray_tmp_0;
    }
    if ((*pairs).icd.copy).is_some() {
        let fresh2 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        ((*pairs).icd.copy)
            .unwrap()(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh2 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh3 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh3 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    printf(b"push\n\0" as *const u8 as *const libc::c_char);
    it.a = 3 as libc::c_int;
    it.b = 4 as libc::c_int;
    if ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
        let mut utarray_tmp_1: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
            (*pairs)
                .n = if (*pairs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_1 = realloc(
            (*pairs).d as *mut libc::c_void,
            ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_1.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs).d = utarray_tmp_1;
    }
    if ((*pairs).icd.copy).is_some() {
        let fresh4 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        ((*pairs).icd.copy)
            .unwrap()(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh4 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh5 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh5 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    printf(b"push\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    ip = 0 as *mut intpair_t;
    loop {
        ip = (if ip.is_null() {
            if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    if ((*pairs).icd.dtor).is_some() {
        let mut _ut_i: libc::c_uint = 0;
        _ut_i = 0 as libc::c_int as libc::c_uint;
        while _ut_i < 1 as libc::c_int as libc::c_uint {
            ((*pairs).icd.dtor)
                .unwrap()(
                if (0 as libc::c_int as libc::c_uint).wrapping_add(_ut_i) < (*pairs).i {
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz)
                                .wrapping_mul(
                                    (0 as libc::c_int as libc::c_uint).wrapping_add(_ut_i)
                                        as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                },
            );
            _ut_i = _ut_i.wrapping_add(1);
            _ut_i;
        }
    }
    if (*pairs).i > (0 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
        memmove(
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            (0 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_void,
            (((*pairs).i)
                .wrapping_sub((0 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul((*pairs).icd.sz),
        );
    }
    (*pairs).i = ((*pairs).i).wrapping_sub(1 as libc::c_int as libc::c_uint);
    printf(b"erase [0]\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    loop {
        ip = (if ip.is_null() {
            if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    it.a = 1 as libc::c_int;
    it.b = 2 as libc::c_int;
    if ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
        let mut utarray_tmp_2: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
            (*pairs)
                .n = if (*pairs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_2 = realloc(
            (*pairs).d as *mut libc::c_void,
            ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_2.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs).d = utarray_tmp_2;
    }
    if ((*pairs).icd.copy).is_some() {
        let fresh6 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        ((*pairs).icd.copy)
            .unwrap()(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh6 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh7 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh7 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    printf(b"push\n\0" as *const u8 as *const libc::c_char);
    loop {
        ip = (if ip.is_null() {
            if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    if (*pairs).i > 0 as libc::c_int as libc::c_uint {
        if ((*pairs).icd.dtor).is_some() {
            let mut _ut_i_0: libc::c_uint = 0;
            _ut_i_0 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_0 < (*pairs).i {
                ((*pairs).icd.dtor)
                    .unwrap()(
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_0 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_0 = _ut_i_0.wrapping_add(1);
                _ut_i_0;
            }
        }
        (*pairs).i = 0 as libc::c_int as libc::c_uint;
    }
    printf(b"clear\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    if ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
        let mut utarray_tmp_3: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
            (*pairs)
                .n = if (*pairs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_3 = realloc(
            (*pairs).d as *mut libc::c_void,
            ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_3.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs).d = utarray_tmp_3;
    }
    if ((*pairs).icd.init).is_some() {
        ((*pairs).icd.init)
            .unwrap()(
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz).wrapping_mul((*pairs).i as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
        );
    } else {
        memset(
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz).wrapping_mul((*pairs).i as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            0 as libc::c_int,
            (*pairs).icd.sz,
        );
    }
    (*pairs).i = ((*pairs).i).wrapping_add(1);
    (*pairs).i;
    printf(b"extend\n\0" as *const u8 as *const libc::c_char);
    ip = (if (*pairs).i != 0 {
        ((*pairs).d)
            .offset(
                ((*pairs).icd.sz)
                    .wrapping_mul(
                        ((*pairs).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as *mut intpair_t;
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    printf(
        b"ip points to [0] ? %s\n\0" as *const u8 as *const libc::c_char,
        if ip
            == (if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as *mut intpair_t
        {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    it.a = 1 as libc::c_int;
    it.b = 2 as libc::c_int;
    if ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
        let mut utarray_tmp_4: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
            (*pairs)
                .n = if (*pairs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_4 = realloc(
            (*pairs).d as *mut libc::c_void,
            ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_4.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs).d = utarray_tmp_4;
    }
    if ((*pairs).icd.copy).is_some() {
        let fresh8 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        ((*pairs).icd.copy)
            .unwrap()(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh8 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh9 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh9 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    printf(b"push\n\0" as *const u8 as *const libc::c_char);
    ip = 0 as *mut intpair_t;
    loop {
        ip = (if ip.is_null() {
            if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    if ((*pairs).icd.dtor).is_some() {
        let mut _ut_i_1: libc::c_uint = 0;
        _ut_i_1 = 0 as libc::c_int as libc::c_uint;
        while _ut_i_1 < 1 as libc::c_int as libc::c_uint {
            ((*pairs).icd.dtor)
                .unwrap()(
                if (1 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_1) < (*pairs).i
                {
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz)
                                .wrapping_mul(
                                    (1 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_1)
                                        as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                },
            );
            _ut_i_1 = _ut_i_1.wrapping_add(1);
            _ut_i_1;
        }
    }
    if (*pairs).i > (1 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
        memmove(
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz).wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            (1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_void,
            (((*pairs).i)
                .wrapping_sub((1 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul((*pairs).icd.sz),
        );
    }
    (*pairs).i = ((*pairs).i).wrapping_sub(1 as libc::c_int as libc::c_uint);
    printf(b"erase [1]\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    loop {
        ip = (if ip.is_null() {
            if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    it.a = 3 as libc::c_int;
    it.b = 4 as libc::c_int;
    if ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
        let mut utarray_tmp_5: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs).n {
            (*pairs)
                .n = if (*pairs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_5 = realloc(
            (*pairs).d as *mut libc::c_void,
            ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_5.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs).d = utarray_tmp_5;
    }
    if ((*pairs).icd.copy).is_some() {
        let fresh10 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        ((*pairs).icd.copy)
            .unwrap()(
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz).wrapping_mul(fresh10 as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh11 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz).wrapping_mul(fresh11 as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    printf(b"push\n\0" as *const u8 as *const libc::c_char);
    ip = (if (*pairs).i != 0 {
        ((*pairs).d)
            .offset(
                ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as *mut intpair_t;
    while !ip.is_null() {
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
        ip = (if ip.is_null() {
            if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
    }
    ip = (if (*pairs).i != 0 {
        ((*pairs).d)
            .offset(
                ((*pairs).icd.sz)
                    .wrapping_mul(
                        ((*pairs).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as *mut intpair_t;
    printf(b"back is %d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    pairs_cpy = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong)
        as *mut UT_array;
    if pairs_cpy.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        pairs_cpy as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*pairs_cpy).icd = pairicd;
    if !((*pairs).i == 0 as libc::c_int as libc::c_uint) {
        if (*pairs_cpy).i > (*pairs_cpy).i {
            let mut _ut_i_2: libc::c_uint = 0;
            if (*pairs_cpy).i > (*pairs_cpy).i {
                if ((*pairs_cpy).icd.dtor).is_some() {
                    _ut_i_2 = (*pairs_cpy).i;
                    while _ut_i_2 < (*pairs_cpy).i {
                        ((*pairs_cpy).icd.dtor)
                            .unwrap()(
                            ((*pairs_cpy).d)
                                .offset(
                                    ((*pairs_cpy).icd.sz).wrapping_mul(_ut_i_2 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_2 = _ut_i_2.wrapping_add(1);
                        _ut_i_2;
                    }
                }
            } else if (*pairs_cpy).i < (*pairs_cpy).i {
                if ((*pairs_cpy).i)
                    .wrapping_add(((*pairs_cpy).i).wrapping_sub((*pairs_cpy).i))
                    > (*pairs_cpy).n
                {
                    let mut utarray_tmp_6: *mut libc::c_char = 0 as *mut libc::c_char;
                    while ((*pairs_cpy).i)
                        .wrapping_add(((*pairs_cpy).i).wrapping_sub((*pairs_cpy).i))
                        > (*pairs_cpy).n
                    {
                        (*pairs_cpy)
                            .n = if (*pairs_cpy).n != 0 {
                            (2 as libc::c_int as libc::c_uint)
                                .wrapping_mul((*pairs_cpy).n)
                        } else {
                            8 as libc::c_int as libc::c_uint
                        };
                    }
                    utarray_tmp_6 = realloc(
                        (*pairs_cpy).d as *mut libc::c_void,
                        ((*pairs_cpy).n as libc::c_ulong)
                            .wrapping_mul((*pairs_cpy).icd.sz),
                    ) as *mut libc::c_char;
                    if utarray_tmp_6.is_null() {
                        exit(-(1 as libc::c_int));
                    }
                    (*pairs_cpy).d = utarray_tmp_6;
                }
                if ((*pairs_cpy).icd.init).is_some() {
                    _ut_i_2 = (*pairs_cpy).i;
                    while _ut_i_2 < (*pairs_cpy).i {
                        ((*pairs_cpy).icd.init)
                            .unwrap()(
                            ((*pairs_cpy).d)
                                .offset(
                                    ((*pairs_cpy).icd.sz).wrapping_mul(_ut_i_2 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_2 = _ut_i_2.wrapping_add(1);
                        _ut_i_2;
                    }
                } else {
                    memset(
                        ((*pairs_cpy).d)
                            .offset(
                                ((*pairs_cpy).icd.sz)
                                    .wrapping_mul((*pairs_cpy).i as libc::c_ulong) as isize,
                            ) as *mut libc::c_void,
                        0 as libc::c_int,
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(
                                ((*pairs_cpy).i).wrapping_sub((*pairs_cpy).i)
                                    as libc::c_ulong,
                            ),
                    );
                }
            }
            (*pairs_cpy).i = (*pairs_cpy).i;
        }
        if ((*pairs_cpy).i).wrapping_add((*pairs).i) > (*pairs_cpy).n {
            let mut utarray_tmp_7: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*pairs_cpy).i).wrapping_add((*pairs).i) > (*pairs_cpy).n {
                (*pairs_cpy)
                    .n = if (*pairs_cpy).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs_cpy).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_7 = realloc(
                (*pairs_cpy).d as *mut libc::c_void,
                ((*pairs_cpy).n as libc::c_ulong).wrapping_mul((*pairs_cpy).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_7.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*pairs_cpy).d = utarray_tmp_7;
        }
        if (*pairs_cpy).i < (*pairs_cpy).i {
            memmove(
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(
                                ((*pairs_cpy).i).wrapping_add((*pairs).i) as libc::c_ulong,
                            ) as isize,
                    ) as *mut libc::c_void,
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul((*pairs_cpy).i as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
                (((*pairs_cpy).i).wrapping_sub((*pairs_cpy).i) as libc::c_ulong)
                    .wrapping_mul((*pairs_cpy).icd.sz),
            );
        }
        if ((*pairs_cpy).icd.copy).is_some() {
            let mut _ut_i_3: libc::c_uint = 0;
            _ut_i_3 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_3 < (*pairs).i {
                ((*pairs_cpy).icd.copy)
                    .unwrap()(
                    ((*pairs_cpy).d)
                        .offset(
                            ((*pairs_cpy).icd.sz)
                                .wrapping_mul(
                                    ((*pairs_cpy).i).wrapping_add(_ut_i_3) as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void,
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_3 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_3 = _ut_i_3.wrapping_add(1);
                _ut_i_3;
            }
        } else {
            memcpy(
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul((*pairs_cpy).i as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*pairs).i as libc::c_ulong).wrapping_mul((*pairs_cpy).icd.sz),
            );
        }
        (*pairs_cpy).i = ((*pairs_cpy).i).wrapping_add((*pairs).i);
    }
    printf(b"copy\n\0" as *const u8 as *const libc::c_char);
    printf(b"cpy length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs_cpy).i);
    ip = 0 as *mut intpair_t;
    loop {
        ip = (if ip.is_null() {
            if (*pairs_cpy).i != 0 {
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs_cpy).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs_cpy).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs_cpy).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"cpy %d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    it.a = 5 as libc::c_int;
    it.b = 6 as libc::c_int;
    if 0 as libc::c_int as libc::c_uint > (*pairs_cpy).i {
        let mut _ut_i_4: libc::c_uint = 0;
        if (*pairs_cpy).i > 0 as libc::c_int as libc::c_uint {
            if ((*pairs_cpy).icd.dtor).is_some() {
                _ut_i_4 = 0 as libc::c_int as libc::c_uint;
                while _ut_i_4 < (*pairs_cpy).i {
                    ((*pairs_cpy).icd.dtor)
                        .unwrap()(
                        ((*pairs_cpy).d)
                            .offset(
                                ((*pairs_cpy).icd.sz).wrapping_mul(_ut_i_4 as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void,
                    );
                    _ut_i_4 = _ut_i_4.wrapping_add(1);
                    _ut_i_4;
                }
            }
        } else if (*pairs_cpy).i < 0 as libc::c_int as libc::c_uint {
            if ((*pairs_cpy).i)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_uint).wrapping_sub((*pairs_cpy).i),
                ) > (*pairs_cpy).n
            {
                let mut utarray_tmp_8: *mut libc::c_char = 0 as *mut libc::c_char;
                while ((*pairs_cpy).i)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_uint).wrapping_sub((*pairs_cpy).i),
                    ) > (*pairs_cpy).n
                {
                    (*pairs_cpy)
                        .n = if (*pairs_cpy).n != 0 {
                        (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs_cpy).n)
                    } else {
                        8 as libc::c_int as libc::c_uint
                    };
                }
                utarray_tmp_8 = realloc(
                    (*pairs_cpy).d as *mut libc::c_void,
                    ((*pairs_cpy).n as libc::c_ulong).wrapping_mul((*pairs_cpy).icd.sz),
                ) as *mut libc::c_char;
                if utarray_tmp_8.is_null() {
                    exit(-(1 as libc::c_int));
                }
                (*pairs_cpy).d = utarray_tmp_8;
            }
            if ((*pairs_cpy).icd.init).is_some() {
                _ut_i_4 = (*pairs_cpy).i;
                while _ut_i_4 < 0 as libc::c_int as libc::c_uint {
                    ((*pairs_cpy).icd.init)
                        .unwrap()(
                        ((*pairs_cpy).d)
                            .offset(
                                ((*pairs_cpy).icd.sz).wrapping_mul(_ut_i_4 as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void,
                    );
                    _ut_i_4 = _ut_i_4.wrapping_add(1);
                    _ut_i_4;
                }
            } else {
                memset(
                    ((*pairs_cpy).d)
                        .offset(
                            ((*pairs_cpy).icd.sz)
                                .wrapping_mul((*pairs_cpy).i as libc::c_ulong) as isize,
                        ) as *mut libc::c_void,
                    0 as libc::c_int,
                    ((*pairs_cpy).icd.sz)
                        .wrapping_mul(
                            (0 as libc::c_int as libc::c_uint)
                                .wrapping_sub((*pairs_cpy).i) as libc::c_ulong,
                        ),
                );
            }
        }
        (*pairs_cpy).i = 0 as libc::c_int as libc::c_uint;
    }
    if ((*pairs_cpy).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*pairs_cpy).n {
        let mut utarray_tmp_9: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*pairs_cpy).i).wrapping_add(1 as libc::c_int as libc::c_uint)
            > (*pairs_cpy).n
        {
            (*pairs_cpy)
                .n = if (*pairs_cpy).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs_cpy).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_9 = realloc(
            (*pairs_cpy).d as *mut libc::c_void,
            ((*pairs_cpy).n as libc::c_ulong).wrapping_mul((*pairs_cpy).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_9.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*pairs_cpy).d = utarray_tmp_9;
    }
    if (0 as libc::c_int as libc::c_uint) < (*pairs_cpy).i {
        memmove(
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz)
                        .wrapping_mul(
                            (0 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_void,
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            (((*pairs_cpy).i).wrapping_sub(0 as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul((*pairs_cpy).icd.sz),
        );
    }
    if ((*pairs_cpy).icd.copy).is_some() {
        ((*pairs_cpy).icd.copy)
            .unwrap()(
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
        );
    } else {
        memcpy(
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            &mut it as *mut intpair_t as *const libc::c_void,
            (*pairs_cpy).icd.sz,
        );
    }
    (*pairs_cpy).i = ((*pairs_cpy).i).wrapping_add(1);
    (*pairs_cpy).i;
    printf(b"insert cpy[0]\n\0" as *const u8 as *const libc::c_char);
    printf(b"cpy length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs_cpy).i);
    loop {
        ip = (if ip.is_null() {
            if (*pairs_cpy).i != 0 {
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs_cpy).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs_cpy).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs_cpy).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"cpy %d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    if ((*pairs_cpy).icd.dtor).is_some() {
        let mut _ut_i_5: libc::c_uint = 0;
        _ut_i_5 = 0 as libc::c_int as libc::c_uint;
        while _ut_i_5 < 2 as libc::c_int as libc::c_uint {
            ((*pairs_cpy).icd.dtor)
                .unwrap()(
                if (0 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_5)
                    < (*pairs_cpy).i
                {
                    ((*pairs_cpy).d)
                        .offset(
                            ((*pairs_cpy).icd.sz)
                                .wrapping_mul(
                                    (0 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_5)
                                        as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                },
            );
            _ut_i_5 = _ut_i_5.wrapping_add(1);
            _ut_i_5;
        }
    }
    if (*pairs_cpy).i > (0 as libc::c_int + 2 as libc::c_int) as libc::c_uint {
        memmove(
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz)
                        .wrapping_mul(
                            (0 as libc::c_int + 2 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_void,
            (((*pairs_cpy).i)
                .wrapping_sub((0 as libc::c_int + 2 as libc::c_int) as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul((*pairs_cpy).icd.sz),
        );
    }
    (*pairs_cpy).i = ((*pairs_cpy).i).wrapping_sub(2 as libc::c_int as libc::c_uint);
    printf(b"erase cpy [0] [1]\n\0" as *const u8 as *const libc::c_char);
    printf(b"cpy length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs_cpy).i);
    loop {
        ip = (if ip.is_null() {
            if (*pairs_cpy).i != 0 {
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs_cpy).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs_cpy).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs_cpy).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"cpy %d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    if !((*pairs).i == 0 as libc::c_int as libc::c_uint) {
        if 1 as libc::c_int as libc::c_uint > (*pairs_cpy).i {
            let mut _ut_i_6: libc::c_uint = 0;
            if (*pairs_cpy).i > 1 as libc::c_int as libc::c_uint {
                if ((*pairs_cpy).icd.dtor).is_some() {
                    _ut_i_6 = 1 as libc::c_int as libc::c_uint;
                    while _ut_i_6 < (*pairs_cpy).i {
                        ((*pairs_cpy).icd.dtor)
                            .unwrap()(
                            ((*pairs_cpy).d)
                                .offset(
                                    ((*pairs_cpy).icd.sz).wrapping_mul(_ut_i_6 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_6 = _ut_i_6.wrapping_add(1);
                        _ut_i_6;
                    }
                }
            } else if (*pairs_cpy).i < 1 as libc::c_int as libc::c_uint {
                if ((*pairs_cpy).i)
                    .wrapping_add(
                        (1 as libc::c_int as libc::c_uint).wrapping_sub((*pairs_cpy).i),
                    ) > (*pairs_cpy).n
                {
                    let mut utarray_tmp_10: *mut libc::c_char = 0 as *mut libc::c_char;
                    while ((*pairs_cpy).i)
                        .wrapping_add(
                            (1 as libc::c_int as libc::c_uint)
                                .wrapping_sub((*pairs_cpy).i),
                        ) > (*pairs_cpy).n
                    {
                        (*pairs_cpy)
                            .n = if (*pairs_cpy).n != 0 {
                            (2 as libc::c_int as libc::c_uint)
                                .wrapping_mul((*pairs_cpy).n)
                        } else {
                            8 as libc::c_int as libc::c_uint
                        };
                    }
                    utarray_tmp_10 = realloc(
                        (*pairs_cpy).d as *mut libc::c_void,
                        ((*pairs_cpy).n as libc::c_ulong)
                            .wrapping_mul((*pairs_cpy).icd.sz),
                    ) as *mut libc::c_char;
                    if utarray_tmp_10.is_null() {
                        exit(-(1 as libc::c_int));
                    }
                    (*pairs_cpy).d = utarray_tmp_10;
                }
                if ((*pairs_cpy).icd.init).is_some() {
                    _ut_i_6 = (*pairs_cpy).i;
                    while _ut_i_6 < 1 as libc::c_int as libc::c_uint {
                        ((*pairs_cpy).icd.init)
                            .unwrap()(
                            ((*pairs_cpy).d)
                                .offset(
                                    ((*pairs_cpy).icd.sz).wrapping_mul(_ut_i_6 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_6 = _ut_i_6.wrapping_add(1);
                        _ut_i_6;
                    }
                } else {
                    memset(
                        ((*pairs_cpy).d)
                            .offset(
                                ((*pairs_cpy).icd.sz)
                                    .wrapping_mul((*pairs_cpy).i as libc::c_ulong) as isize,
                            ) as *mut libc::c_void,
                        0 as libc::c_int,
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(
                                (1 as libc::c_int as libc::c_uint)
                                    .wrapping_sub((*pairs_cpy).i) as libc::c_ulong,
                            ),
                    );
                }
            }
            (*pairs_cpy).i = 1 as libc::c_int as libc::c_uint;
        }
        if ((*pairs_cpy).i).wrapping_add((*pairs).i) > (*pairs_cpy).n {
            let mut utarray_tmp_11: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*pairs_cpy).i).wrapping_add((*pairs).i) > (*pairs_cpy).n {
                (*pairs_cpy)
                    .n = if (*pairs_cpy).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs_cpy).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_11 = realloc(
                (*pairs_cpy).d as *mut libc::c_void,
                ((*pairs_cpy).n as libc::c_ulong).wrapping_mul((*pairs_cpy).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_11.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*pairs_cpy).d = utarray_tmp_11;
        }
        if (1 as libc::c_int as libc::c_uint) < (*pairs_cpy).i {
            memmove(
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(
                                (1 as libc::c_int as libc::c_uint).wrapping_add((*pairs).i)
                                    as libc::c_ulong,
                            ) as isize,
                    ) as *mut libc::c_void,
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
                (((*pairs_cpy).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong)
                    .wrapping_mul((*pairs_cpy).icd.sz),
            );
        }
        if ((*pairs_cpy).icd.copy).is_some() {
            let mut _ut_i_7: libc::c_uint = 0;
            _ut_i_7 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_7 < (*pairs).i {
                ((*pairs_cpy).icd.copy)
                    .unwrap()(
                    ((*pairs_cpy).d)
                        .offset(
                            ((*pairs_cpy).icd.sz)
                                .wrapping_mul(
                                    (1 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_7)
                                        as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void,
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_7 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_7 = _ut_i_7.wrapping_add(1);
                _ut_i_7;
            }
        } else {
            memcpy(
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*pairs).i as libc::c_ulong).wrapping_mul((*pairs_cpy).icd.sz),
            );
        }
        (*pairs_cpy).i = ((*pairs_cpy).i).wrapping_add((*pairs).i);
    }
    printf(b"inserta at cpy[1]\n\0" as *const u8 as *const libc::c_char);
    printf(b"cpy length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs_cpy).i);
    loop {
        ip = (if ip.is_null() {
            if (*pairs_cpy).i != 0 {
                ((*pairs_cpy).d)
                    .offset(
                        ((*pairs_cpy).icd.sz)
                            .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs_cpy).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs_cpy).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs_cpy).d)
                .offset(
                    ((*pairs_cpy).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs_cpy).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs_cpy).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"cpy %d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    if (*pairs_cpy).n != 0 {
        if ((*pairs_cpy).icd.dtor).is_some() {
            let mut _ut_i_8: libc::c_uint = 0;
            _ut_i_8 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_8 < (*pairs_cpy).i {
                ((*pairs_cpy).icd.dtor)
                    .unwrap()(
                    if _ut_i_8 < (*pairs_cpy).i {
                        ((*pairs_cpy).d)
                            .offset(
                                ((*pairs_cpy).icd.sz).wrapping_mul(_ut_i_8 as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    },
                );
                _ut_i_8 = _ut_i_8.wrapping_add(1);
                _ut_i_8;
            }
        }
        free((*pairs_cpy).d as *mut libc::c_void);
    }
    (*pairs_cpy).n = 0 as libc::c_int as libc::c_uint;
    free(pairs_cpy as *mut libc::c_void);
    printf(b"free cpy\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    let mut _ut_i_9: libc::c_uint = 0;
    if (*pairs).i > 30 as libc::c_int as libc::c_uint {
        if ((*pairs).icd.dtor).is_some() {
            _ut_i_9 = 30 as libc::c_int as libc::c_uint;
            while _ut_i_9 < (*pairs).i {
                ((*pairs).icd.dtor)
                    .unwrap()(
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_9 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_9 = _ut_i_9.wrapping_add(1);
                _ut_i_9;
            }
        }
    } else if (*pairs).i < 30 as libc::c_int as libc::c_uint {
        if ((*pairs).i)
            .wrapping_add((30 as libc::c_int as libc::c_uint).wrapping_sub((*pairs).i))
            > (*pairs).n
        {
            let mut utarray_tmp_12: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*pairs).i)
                .wrapping_add(
                    (30 as libc::c_int as libc::c_uint).wrapping_sub((*pairs).i),
                ) > (*pairs).n
            {
                (*pairs)
                    .n = if (*pairs).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_12 = realloc(
                (*pairs).d as *mut libc::c_void,
                ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_12.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*pairs).d = utarray_tmp_12;
        }
        if ((*pairs).icd.init).is_some() {
            _ut_i_9 = (*pairs).i;
            while _ut_i_9 < 30 as libc::c_int as libc::c_uint {
                ((*pairs).icd.init)
                    .unwrap()(
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_9 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_9 = _ut_i_9.wrapping_add(1);
                _ut_i_9;
            }
        } else {
            memset(
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul((*pairs).i as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                0 as libc::c_int,
                ((*pairs).icd.sz)
                    .wrapping_mul(
                        (30 as libc::c_int as libc::c_uint).wrapping_sub((*pairs).i)
                            as libc::c_ulong,
                    ),
            );
        }
    }
    (*pairs).i = 30 as libc::c_int as libc::c_uint;
    printf(b"resize to 30\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    loop {
        ip = (if ip.is_null() {
            if (*pairs).i != 0 {
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*pairs).i as libc::c_ulong
            != ((ip as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((ip as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
        if ip.is_null() {
            break;
        }
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*ip).a, (*ip).b);
    }
    let mut _ut_i_10: libc::c_uint = 0;
    if (*pairs).i > 1 as libc::c_int as libc::c_uint {
        if ((*pairs).icd.dtor).is_some() {
            _ut_i_10 = 1 as libc::c_int as libc::c_uint;
            while _ut_i_10 < (*pairs).i {
                ((*pairs).icd.dtor)
                    .unwrap()(
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_10 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_10 = _ut_i_10.wrapping_add(1);
                _ut_i_10;
            }
        }
    } else if (*pairs).i < 1 as libc::c_int as libc::c_uint {
        if ((*pairs).i)
            .wrapping_add((1 as libc::c_int as libc::c_uint).wrapping_sub((*pairs).i))
            > (*pairs).n
        {
            let mut utarray_tmp_13: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*pairs).i)
                .wrapping_add(
                    (1 as libc::c_int as libc::c_uint).wrapping_sub((*pairs).i),
                ) > (*pairs).n
            {
                (*pairs)
                    .n = if (*pairs).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_13 = realloc(
                (*pairs).d as *mut libc::c_void,
                ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_13.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*pairs).d = utarray_tmp_13;
        }
        if ((*pairs).icd.init).is_some() {
            _ut_i_10 = (*pairs).i;
            while _ut_i_10 < 1 as libc::c_int as libc::c_uint {
                ((*pairs).icd.init)
                    .unwrap()(
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_10 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_10 = _ut_i_10.wrapping_add(1);
                _ut_i_10;
            }
        } else {
            memset(
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul((*pairs).i as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                0 as libc::c_int,
                ((*pairs).icd.sz)
                    .wrapping_mul(
                        (1 as libc::c_int as libc::c_uint).wrapping_sub((*pairs).i)
                            as libc::c_ulong,
                    ),
            );
        }
    }
    (*pairs).i = 1 as libc::c_int as libc::c_uint;
    printf(b"resize to 1\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    let mut _ut_i_11: libc::c_uint = 0;
    if (*pairs).i > zero as libc::c_uint {
        if ((*pairs).icd.dtor).is_some() {
            _ut_i_11 = zero as libc::c_uint;
            while _ut_i_11 < (*pairs).i {
                ((*pairs).icd.dtor)
                    .unwrap()(
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_11 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_11 = _ut_i_11.wrapping_add(1);
                _ut_i_11;
            }
        }
    } else if (*pairs).i < zero as libc::c_uint {
        if ((*pairs).i as libc::c_ulong)
            .wrapping_add(zero.wrapping_sub((*pairs).i as libc::c_ulong))
            > (*pairs).n as libc::c_ulong
        {
            let mut utarray_tmp_14: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*pairs).i as libc::c_ulong)
                .wrapping_add(zero.wrapping_sub((*pairs).i as libc::c_ulong))
                > (*pairs).n as libc::c_ulong
            {
                (*pairs)
                    .n = if (*pairs).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*pairs).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_14 = realloc(
                (*pairs).d as *mut libc::c_void,
                ((*pairs).n as libc::c_ulong).wrapping_mul((*pairs).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_14.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*pairs).d = utarray_tmp_14;
        }
        if ((*pairs).icd.init).is_some() {
            _ut_i_11 = (*pairs).i;
            while _ut_i_11 < zero as libc::c_uint {
                ((*pairs).icd.init)
                    .unwrap()(
                    ((*pairs).d)
                        .offset(
                            ((*pairs).icd.sz).wrapping_mul(_ut_i_11 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_11 = _ut_i_11.wrapping_add(1);
                _ut_i_11;
            }
        } else {
            memset(
                ((*pairs).d)
                    .offset(
                        ((*pairs).icd.sz).wrapping_mul((*pairs).i as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                0 as libc::c_int,
                ((*pairs).icd.sz)
                    .wrapping_mul(zero.wrapping_sub((*pairs).i as libc::c_ulong)),
            );
        }
    }
    (*pairs).i = zero as libc::c_uint;
    printf(b"resize to 0\n\0" as *const u8 as *const libc::c_char);
    printf(b"length is %u\n\0" as *const u8 as *const libc::c_char, (*pairs).i);
    if (*pairs).n != 0 {
        if ((*pairs).icd.dtor).is_some() {
            let mut _ut_i_12: libc::c_uint = 0;
            _ut_i_12 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_12 < (*pairs).i {
                ((*pairs).icd.dtor)
                    .unwrap()(
                    if _ut_i_12 < (*pairs).i {
                        ((*pairs).d)
                            .offset(
                                ((*pairs).icd.sz).wrapping_mul(_ut_i_12 as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    },
                );
                _ut_i_12 = _ut_i_12.wrapping_add(1);
                _ut_i_12;
            }
        }
        free((*pairs).d as *mut libc::c_void);
    }
    (*pairs).n = 0 as libc::c_int as libc::c_uint;
    free(pairs as *mut libc::c_void);
    printf(b"free\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
