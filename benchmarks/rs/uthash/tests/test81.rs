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
static mut ut_int_icd: UT_icd = {
    let mut init = UT_icd {
        sz: ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        init: None,
        copy: None,
        dtor: None,
    };
    init
};
unsafe fn main_0() -> libc::c_int {
    let mut a: *mut UT_array = 0 as *mut UT_array;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    a = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong) as *mut UT_array;
    if a.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        a as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*a).icd = ut_int_icd;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if ((*a).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*a).n {
            let mut utarray_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*a).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*a).n {
                (*a)
                    .n = if (*a).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*a).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp = realloc(
                (*a).d as *mut libc::c_void,
                ((*a).n as libc::c_ulong).wrapping_mul((*a).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*a).d = utarray_tmp;
        }
        if ((*a).icd.copy).is_some() {
            let fresh0 = (*a).i;
            (*a).i = ((*a).i).wrapping_add(1);
            ((*a).icd.copy)
                .unwrap()(
                ((*a).d)
                    .offset(((*a).icd.sz).wrapping_mul(fresh0 as libc::c_ulong) as isize)
                    as *mut libc::c_void,
                &mut i as *mut libc::c_int as *const libc::c_void,
            );
        } else {
            let fresh1 = (*a).i;
            (*a).i = ((*a).i).wrapping_add(1);
            memcpy(
                ((*a).d)
                    .offset(((*a).icd.sz).wrapping_mul(fresh1 as libc::c_ulong) as isize)
                    as *mut libc::c_void,
                &mut i as *mut libc::c_int as *const libc::c_void,
                (*a).icd.sz,
            );
        }
        i += 1;
        i;
    }
    p = (if (*a).i != 0 {
        ((*a).d)
            .offset(
                ((*a).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_int;
    while !p.is_null() {
        printf(b"%d \0" as *const u8 as *const libc::c_char, *p);
        p = (if p.is_null() {
            if (*a).i != 0 {
                ((*a).d)
                    .offset(
                        ((*a).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*a).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*a).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*a).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*a).d)
                .offset(
                    ((*a).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*a).d) as libc::c_long
                                as libc::c_ulong)
                                .wrapping_div((*a).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_int;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"len: %u\n\n\0" as *const u8 as *const libc::c_char, (*a).i);
    i = 10 as libc::c_int;
    if 10 as libc::c_int as libc::c_uint > (*a).i {
        let mut _ut_i: libc::c_uint = 0;
        if (*a).i > 10 as libc::c_int as libc::c_uint {
            if ((*a).icd.dtor).is_some() {
                _ut_i = 10 as libc::c_int as libc::c_uint;
                while _ut_i < (*a).i {
                    ((*a).icd.dtor)
                        .unwrap()(
                        ((*a).d)
                            .offset(
                                ((*a).icd.sz).wrapping_mul(_ut_i as libc::c_ulong) as isize,
                            ) as *mut libc::c_void,
                    );
                    _ut_i = _ut_i.wrapping_add(1);
                    _ut_i;
                }
            }
        } else if (*a).i < 10 as libc::c_int as libc::c_uint {
            if ((*a).i)
                .wrapping_add((10 as libc::c_int as libc::c_uint).wrapping_sub((*a).i))
                > (*a).n
            {
                let mut utarray_tmp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                while ((*a).i)
                    .wrapping_add(
                        (10 as libc::c_int as libc::c_uint).wrapping_sub((*a).i),
                    ) > (*a).n
                {
                    (*a)
                        .n = if (*a).n != 0 {
                        (2 as libc::c_int as libc::c_uint).wrapping_mul((*a).n)
                    } else {
                        8 as libc::c_int as libc::c_uint
                    };
                }
                utarray_tmp_0 = realloc(
                    (*a).d as *mut libc::c_void,
                    ((*a).n as libc::c_ulong).wrapping_mul((*a).icd.sz),
                ) as *mut libc::c_char;
                if utarray_tmp_0.is_null() {
                    exit(-(1 as libc::c_int));
                }
                (*a).d = utarray_tmp_0;
            }
            if ((*a).icd.init).is_some() {
                _ut_i = (*a).i;
                while _ut_i < 10 as libc::c_int as libc::c_uint {
                    ((*a).icd.init)
                        .unwrap()(
                        ((*a).d)
                            .offset(
                                ((*a).icd.sz).wrapping_mul(_ut_i as libc::c_ulong) as isize,
                            ) as *mut libc::c_void,
                    );
                    _ut_i = _ut_i.wrapping_add(1);
                    _ut_i;
                }
            } else {
                memset(
                    ((*a).d)
                        .offset(
                            ((*a).icd.sz).wrapping_mul((*a).i as libc::c_ulong) as isize,
                        ) as *mut libc::c_void,
                    0 as libc::c_int,
                    ((*a).icd.sz)
                        .wrapping_mul(
                            (10 as libc::c_int as libc::c_uint).wrapping_sub((*a).i)
                                as libc::c_ulong,
                        ),
                );
            }
        }
        (*a).i = 10 as libc::c_int as libc::c_uint;
    }
    if ((*a).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*a).n {
        let mut utarray_tmp_1: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*a).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*a).n {
            (*a)
                .n = if (*a).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*a).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_1 = realloc(
            (*a).d as *mut libc::c_void,
            ((*a).n as libc::c_ulong).wrapping_mul((*a).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_1.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*a).d = utarray_tmp_1;
    }
    if (10 as libc::c_int as libc::c_uint) < (*a).i {
        memmove(
            ((*a).d)
                .offset(
                    ((*a).icd.sz)
                        .wrapping_mul(
                            (10 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_void,
            ((*a).d)
                .offset(
                    ((*a).icd.sz).wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            (((*a).i).wrapping_sub(10 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul((*a).icd.sz),
        );
    }
    if ((*a).icd.copy).is_some() {
        ((*a).icd.copy)
            .unwrap()(
            ((*a).d)
                .offset(
                    ((*a).icd.sz).wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            &mut i as *mut libc::c_int as *const libc::c_void,
        );
    } else {
        memcpy(
            ((*a).d)
                .offset(
                    ((*a).icd.sz).wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            &mut i as *mut libc::c_int as *const libc::c_void,
            (*a).icd.sz,
        );
    }
    (*a).i = ((*a).i).wrapping_add(1);
    (*a).i;
    loop {
        p = (if p.is_null() {
            if (*a).i != 0 {
                ((*a).d)
                    .offset(
                        ((*a).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*a).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*a).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*a).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*a).d)
                .offset(
                    ((*a).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*a).d) as libc::c_long
                                as libc::c_ulong)
                                .wrapping_div((*a).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_int;
        if p.is_null() {
            break;
        }
        printf(b"%d \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"len: %u\n\n\0" as *const u8 as *const libc::c_char, (*a).i);
    if (*a).n != 0 {
        if ((*a).icd.dtor).is_some() {
            let mut _ut_i_0: libc::c_uint = 0;
            _ut_i_0 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_0 < (*a).i {
                ((*a).icd.dtor)
                    .unwrap()(
                    if _ut_i_0 < (*a).i {
                        ((*a).d)
                            .offset(
                                ((*a).icd.sz).wrapping_mul(_ut_i_0 as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    },
                );
                _ut_i_0 = _ut_i_0.wrapping_add(1);
                _ut_i_0;
            }
        }
        free((*a).d as *mut libc::c_void);
    }
    (*a).n = 0 as libc::c_int as libc::c_uint;
    free(a as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
