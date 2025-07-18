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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
unsafe extern "C" fn utarray_str_cpy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
) {
    let mut srcc: *const *mut libc::c_char = src as *const *mut libc::c_char;
    let mut dstc: *mut *mut libc::c_char = dst as *mut *mut libc::c_char;
    *dstc = if (*srcc).is_null() { 0 as *mut libc::c_char } else { strdup(*srcc) };
}
unsafe extern "C" fn utarray_str_dtor(mut elt: *mut libc::c_void) {
    let mut eltc: *mut *mut libc::c_char = elt as *mut *mut libc::c_char;
    if !(*eltc).is_null() {
        free(*eltc as *mut libc::c_void);
    }
}
static mut ut_str_icd: UT_icd = unsafe {
    {
        let mut init = UT_icd {
            sz: ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            init: None,
            copy: Some(
                utarray_str_cpy
                    as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> (),
            ),
            dtor: Some(utarray_str_dtor as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
unsafe extern "C" fn strsort(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const libc::c_char = *(_a as *const *const libc::c_char);
    let mut b: *const libc::c_char = *(_b as *const *const libc::c_char);
    return strcmp(a, b);
}
unsafe extern "C" fn revsort(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const libc::c_char = *(_a as *const *const libc::c_char);
    let mut b: *const libc::c_char = *(_b as *const *const libc::c_char);
    return strcmp(b, a);
}
unsafe fn main_0() -> libc::c_int {
    let mut strs: *mut UT_array = 0 as *mut UT_array;
    let mut strs2: *mut UT_array = 0 as *mut UT_array;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    strs = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong) as *mut UT_array;
    if strs.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        strs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*strs).icd = ut_str_icd;
    s = b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
        let mut utarray_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
            (*strs)
                .n = if (*strs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp = realloc(
            (*strs).d as *mut libc::c_void,
            ((*strs).n as libc::c_ulong).wrapping_mul((*strs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*strs).d = utarray_tmp;
    }
    if ((*strs).icd.copy).is_some() {
        let fresh0 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        ((*strs).icd.copy)
            .unwrap()(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh0 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh1 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        memcpy(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh1 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    s = b"world\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
        let mut utarray_tmp_0: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
            (*strs)
                .n = if (*strs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_0 = realloc(
            (*strs).d as *mut libc::c_void,
            ((*strs).n as libc::c_ulong).wrapping_mul((*strs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_0.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*strs).d = utarray_tmp_0;
    }
    if ((*strs).icd.copy).is_some() {
        let fresh2 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        ((*strs).icd.copy)
            .unwrap()(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh2 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh3 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        memcpy(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh3 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    loop {
        p = (if p.is_null() {
            if (*strs).i != 0 {
                ((*strs).d)
                    .offset(
                        ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs).d)
                .offset(
                    ((*strs).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    s = b"begin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if 0 as libc::c_int as libc::c_uint > (*strs).i {
        let mut _ut_i: libc::c_uint = 0;
        if (*strs).i > 0 as libc::c_int as libc::c_uint {
            if ((*strs).icd.dtor).is_some() {
                _ut_i = 0 as libc::c_int as libc::c_uint;
                while _ut_i < (*strs).i {
                    ((*strs).icd.dtor)
                        .unwrap()(
                        ((*strs).d)
                            .offset(
                                ((*strs).icd.sz).wrapping_mul(_ut_i as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void,
                    );
                    _ut_i = _ut_i.wrapping_add(1);
                    _ut_i;
                }
            }
        } else if (*strs).i < 0 as libc::c_int as libc::c_uint {
            if ((*strs).i)
                .wrapping_add((0 as libc::c_int as libc::c_uint).wrapping_sub((*strs).i))
                > (*strs).n
            {
                let mut utarray_tmp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                while ((*strs).i)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_uint).wrapping_sub((*strs).i),
                    ) > (*strs).n
                {
                    (*strs)
                        .n = if (*strs).n != 0 {
                        (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs).n)
                    } else {
                        8 as libc::c_int as libc::c_uint
                    };
                }
                utarray_tmp_1 = realloc(
                    (*strs).d as *mut libc::c_void,
                    ((*strs).n as libc::c_ulong).wrapping_mul((*strs).icd.sz),
                ) as *mut libc::c_char;
                if utarray_tmp_1.is_null() {
                    exit(-(1 as libc::c_int));
                }
                (*strs).d = utarray_tmp_1;
            }
            if ((*strs).icd.init).is_some() {
                _ut_i = (*strs).i;
                while _ut_i < 0 as libc::c_int as libc::c_uint {
                    ((*strs).icd.init)
                        .unwrap()(
                        ((*strs).d)
                            .offset(
                                ((*strs).icd.sz).wrapping_mul(_ut_i as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void,
                    );
                    _ut_i = _ut_i.wrapping_add(1);
                    _ut_i;
                }
            } else {
                memset(
                    ((*strs).d)
                        .offset(
                            ((*strs).icd.sz).wrapping_mul((*strs).i as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                    0 as libc::c_int,
                    ((*strs).icd.sz)
                        .wrapping_mul(
                            (0 as libc::c_int as libc::c_uint).wrapping_sub((*strs).i)
                                as libc::c_ulong,
                        ),
                );
            }
        }
        (*strs).i = 0 as libc::c_int as libc::c_uint;
    }
    if ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
        let mut utarray_tmp_2: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
            (*strs)
                .n = if (*strs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_2 = realloc(
            (*strs).d as *mut libc::c_void,
            ((*strs).n as libc::c_ulong).wrapping_mul((*strs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_2.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*strs).d = utarray_tmp_2;
    }
    if (0 as libc::c_int as libc::c_uint) < (*strs).i {
        memmove(
            ((*strs).d)
                .offset(
                    ((*strs).icd.sz)
                        .wrapping_mul(
                            (0 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_void,
            ((*strs).d)
                .offset(
                    ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            (((*strs).i).wrapping_sub(0 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul((*strs).icd.sz),
        );
    }
    if ((*strs).icd.copy).is_some() {
        ((*strs).icd.copy)
            .unwrap()(
            ((*strs).d)
                .offset(
                    ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
        );
    } else {
        memcpy(
            ((*strs).d)
                .offset(
                    ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    (*strs).i = ((*strs).i).wrapping_add(1);
    (*strs).i;
    loop {
        p = (if p.is_null() {
            if (*strs).i != 0 {
                ((*strs).d)
                    .offset(
                        ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs).d)
                .offset(
                    ((*strs).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    strs2 = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong) as *mut UT_array;
    if strs2.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        strs2 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*strs2).icd = ut_str_icd;
    s = b"alt\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if ((*strs2).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs2).n {
        let mut utarray_tmp_3: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*strs2).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs2).n {
            (*strs2)
                .n = if (*strs2).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_3 = realloc(
            (*strs2).d as *mut libc::c_void,
            ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_3.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*strs2).d = utarray_tmp_3;
    }
    if ((*strs2).icd.copy).is_some() {
        let fresh4 = (*strs2).i;
        (*strs2).i = ((*strs2).i).wrapping_add(1);
        ((*strs2).icd.copy)
            .unwrap()(
            ((*strs2).d)
                .offset(((*strs2).icd.sz).wrapping_mul(fresh4 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh5 = (*strs2).i;
        (*strs2).i = ((*strs2).i).wrapping_add(1);
        memcpy(
            ((*strs2).d)
                .offset(((*strs2).icd.sz).wrapping_mul(fresh5 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
            (*strs2).icd.sz,
        );
    }
    s = b"oth\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if ((*strs2).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs2).n {
        let mut utarray_tmp_4: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*strs2).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs2).n {
            (*strs2)
                .n = if (*strs2).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_4 = realloc(
            (*strs2).d as *mut libc::c_void,
            ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_4.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*strs2).d = utarray_tmp_4;
    }
    if ((*strs2).icd.copy).is_some() {
        let fresh6 = (*strs2).i;
        (*strs2).i = ((*strs2).i).wrapping_add(1);
        ((*strs2).icd.copy)
            .unwrap()(
            ((*strs2).d)
                .offset(((*strs2).icd.sz).wrapping_mul(fresh6 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh7 = (*strs2).i;
        (*strs2).i = ((*strs2).i).wrapping_add(1);
        memcpy(
            ((*strs2).d)
                .offset(((*strs2).icd.sz).wrapping_mul(fresh7 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *mut libc::c_char as *const libc::c_void,
            (*strs2).icd.sz,
        );
    }
    if !((*strs).i == 0 as libc::c_int as libc::c_uint) {
        if 1 as libc::c_int as libc::c_uint > (*strs2).i {
            let mut _ut_i_0: libc::c_uint = 0;
            if (*strs2).i > 1 as libc::c_int as libc::c_uint {
                if ((*strs2).icd.dtor).is_some() {
                    _ut_i_0 = 1 as libc::c_int as libc::c_uint;
                    while _ut_i_0 < (*strs2).i {
                        ((*strs2).icd.dtor)
                            .unwrap()(
                            ((*strs2).d)
                                .offset(
                                    ((*strs2).icd.sz).wrapping_mul(_ut_i_0 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_0 = _ut_i_0.wrapping_add(1);
                        _ut_i_0;
                    }
                }
            } else if (*strs2).i < 1 as libc::c_int as libc::c_uint {
                if ((*strs2).i)
                    .wrapping_add(
                        (1 as libc::c_int as libc::c_uint).wrapping_sub((*strs2).i),
                    ) > (*strs2).n
                {
                    let mut utarray_tmp_5: *mut libc::c_char = 0 as *mut libc::c_char;
                    while ((*strs2).i)
                        .wrapping_add(
                            (1 as libc::c_int as libc::c_uint).wrapping_sub((*strs2).i),
                        ) > (*strs2).n
                    {
                        (*strs2)
                            .n = if (*strs2).n != 0 {
                            (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
                        } else {
                            8 as libc::c_int as libc::c_uint
                        };
                    }
                    utarray_tmp_5 = realloc(
                        (*strs2).d as *mut libc::c_void,
                        ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
                    ) as *mut libc::c_char;
                    if utarray_tmp_5.is_null() {
                        exit(-(1 as libc::c_int));
                    }
                    (*strs2).d = utarray_tmp_5;
                }
                if ((*strs2).icd.init).is_some() {
                    _ut_i_0 = (*strs2).i;
                    while _ut_i_0 < 1 as libc::c_int as libc::c_uint {
                        ((*strs2).icd.init)
                            .unwrap()(
                            ((*strs2).d)
                                .offset(
                                    ((*strs2).icd.sz).wrapping_mul(_ut_i_0 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_0 = _ut_i_0.wrapping_add(1);
                        _ut_i_0;
                    }
                } else {
                    memset(
                        ((*strs2).d)
                            .offset(
                                ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void,
                        0 as libc::c_int,
                        ((*strs2).icd.sz)
                            .wrapping_mul(
                                (1 as libc::c_int as libc::c_uint).wrapping_sub((*strs2).i)
                                    as libc::c_ulong,
                            ),
                    );
                }
            }
            (*strs2).i = 1 as libc::c_int as libc::c_uint;
        }
        if ((*strs2).i).wrapping_add((*strs).i) > (*strs2).n {
            let mut utarray_tmp_6: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*strs2).i).wrapping_add((*strs).i) > (*strs2).n {
                (*strs2)
                    .n = if (*strs2).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_6 = realloc(
                (*strs2).d as *mut libc::c_void,
                ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_6.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*strs2).d = utarray_tmp_6;
        }
        if (1 as libc::c_int as libc::c_uint) < (*strs2).i {
            memmove(
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz)
                            .wrapping_mul(
                                (1 as libc::c_int as libc::c_uint).wrapping_add((*strs).i)
                                    as libc::c_ulong,
                            ) as isize,
                    ) as *mut libc::c_void,
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                (((*strs2).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong)
                    .wrapping_mul((*strs2).icd.sz),
            );
        }
        if ((*strs2).icd.copy).is_some() {
            let mut _ut_i_1: libc::c_uint = 0;
            _ut_i_1 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_1 < (*strs).i {
                ((*strs2).icd.copy)
                    .unwrap()(
                    ((*strs2).d)
                        .offset(
                            ((*strs2).icd.sz)
                                .wrapping_mul(
                                    (1 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_1)
                                        as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void,
                    ((*strs).d)
                        .offset(
                            ((*strs).icd.sz).wrapping_mul(_ut_i_1 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_1 = _ut_i_1.wrapping_add(1);
                _ut_i_1;
            }
        } else {
            memcpy(
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*strs).d)
                    .offset(
                        ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*strs).i as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
            );
        }
        (*strs2).i = ((*strs2).i).wrapping_add((*strs).i);
    }
    loop {
        p = (if p.is_null() {
            if (*strs2).i != 0 {
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs2).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs2).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs2).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs2).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs2).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if ((*strs2).icd.dtor).is_some() {
        let mut _ut_i_2: libc::c_uint = 0;
        _ut_i_2 = 0 as libc::c_int as libc::c_uint;
        while _ut_i_2 < 2 as libc::c_int as libc::c_uint {
            ((*strs2).icd.dtor)
                .unwrap()(
                if (0 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_2) < (*strs2).i
                {
                    ((*strs2).d)
                        .offset(
                            ((*strs2).icd.sz)
                                .wrapping_mul(
                                    (0 as libc::c_int as libc::c_uint).wrapping_add(_ut_i_2)
                                        as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                },
            );
            _ut_i_2 = _ut_i_2.wrapping_add(1);
            _ut_i_2;
        }
    }
    if (*strs2).i > (0 as libc::c_int + 2 as libc::c_int) as libc::c_uint {
        memmove(
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            (0 as libc::c_int + 2 as libc::c_int) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_void,
            (((*strs2).i)
                .wrapping_sub((0 as libc::c_int + 2 as libc::c_int) as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul((*strs2).icd.sz),
        );
    }
    (*strs2).i = ((*strs2).i).wrapping_sub(2 as libc::c_int as libc::c_uint);
    loop {
        p = (if p.is_null() {
            if (*strs2).i != 0 {
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs2).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs2).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs2).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs2).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs2).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if ((*strs2).icd.dtor).is_some() {
        (*strs2).i = ((*strs2).i).wrapping_sub(1);
        ((*strs2).icd.dtor)
            .unwrap()(
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
        );
    } else {
        (*strs2).i = ((*strs2).i).wrapping_sub(1);
        (*strs2).i;
    }
    loop {
        p = (if p.is_null() {
            if (*strs2).i != 0 {
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs2).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs2).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs2).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs2).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs2).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !((*strs).i == 0 as libc::c_int as libc::c_uint) {
        if (*strs2).i > (*strs2).i {
            let mut _ut_i_3: libc::c_uint = 0;
            if (*strs2).i > (*strs2).i {
                if ((*strs2).icd.dtor).is_some() {
                    _ut_i_3 = (*strs2).i;
                    while _ut_i_3 < (*strs2).i {
                        ((*strs2).icd.dtor)
                            .unwrap()(
                            ((*strs2).d)
                                .offset(
                                    ((*strs2).icd.sz).wrapping_mul(_ut_i_3 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_3 = _ut_i_3.wrapping_add(1);
                        _ut_i_3;
                    }
                }
            } else if (*strs2).i < (*strs2).i {
                if ((*strs2).i).wrapping_add(((*strs2).i).wrapping_sub((*strs2).i))
                    > (*strs2).n
                {
                    let mut utarray_tmp_7: *mut libc::c_char = 0 as *mut libc::c_char;
                    while ((*strs2).i)
                        .wrapping_add(((*strs2).i).wrapping_sub((*strs2).i)) > (*strs2).n
                    {
                        (*strs2)
                            .n = if (*strs2).n != 0 {
                            (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
                        } else {
                            8 as libc::c_int as libc::c_uint
                        };
                    }
                    utarray_tmp_7 = realloc(
                        (*strs2).d as *mut libc::c_void,
                        ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
                    ) as *mut libc::c_char;
                    if utarray_tmp_7.is_null() {
                        exit(-(1 as libc::c_int));
                    }
                    (*strs2).d = utarray_tmp_7;
                }
                if ((*strs2).icd.init).is_some() {
                    _ut_i_3 = (*strs2).i;
                    while _ut_i_3 < (*strs2).i {
                        ((*strs2).icd.init)
                            .unwrap()(
                            ((*strs2).d)
                                .offset(
                                    ((*strs2).icd.sz).wrapping_mul(_ut_i_3 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_3 = _ut_i_3.wrapping_add(1);
                        _ut_i_3;
                    }
                } else {
                    memset(
                        ((*strs2).d)
                            .offset(
                                ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void,
                        0 as libc::c_int,
                        ((*strs2).icd.sz)
                            .wrapping_mul(
                                ((*strs2).i).wrapping_sub((*strs2).i) as libc::c_ulong,
                            ),
                    );
                }
            }
            (*strs2).i = (*strs2).i;
        }
        if ((*strs2).i).wrapping_add((*strs).i) > (*strs2).n {
            let mut utarray_tmp_8: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*strs2).i).wrapping_add((*strs).i) > (*strs2).n {
                (*strs2)
                    .n = if (*strs2).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_8 = realloc(
                (*strs2).d as *mut libc::c_void,
                ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_8.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*strs2).d = utarray_tmp_8;
        }
        if (*strs2).i < (*strs2).i {
            memmove(
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz)
                            .wrapping_mul(
                                ((*strs2).i).wrapping_add((*strs).i) as libc::c_ulong,
                            ) as isize,
                    ) as *mut libc::c_void,
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                (((*strs2).i).wrapping_sub((*strs2).i) as libc::c_ulong)
                    .wrapping_mul((*strs2).icd.sz),
            );
        }
        if ((*strs2).icd.copy).is_some() {
            let mut _ut_i_4: libc::c_uint = 0;
            _ut_i_4 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_4 < (*strs).i {
                ((*strs2).icd.copy)
                    .unwrap()(
                    ((*strs2).d)
                        .offset(
                            ((*strs2).icd.sz)
                                .wrapping_mul(
                                    ((*strs2).i).wrapping_add(_ut_i_4) as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void,
                    ((*strs).d)
                        .offset(
                            ((*strs).icd.sz).wrapping_mul(_ut_i_4 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_4 = _ut_i_4.wrapping_add(1);
                _ut_i_4;
            }
        } else {
            memcpy(
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*strs).d)
                    .offset(
                        ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*strs).i as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
            );
        }
        (*strs2).i = ((*strs2).i).wrapping_add((*strs).i);
    }
    loop {
        p = (if p.is_null() {
            if (*strs2).i != 0 {
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs2).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs2).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs2).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs2).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs2).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if (*strs2).i > 0 as libc::c_int as libc::c_uint {
        if ((*strs2).icd.dtor).is_some() {
            let mut _ut_i_5: libc::c_uint = 0;
            _ut_i_5 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_5 < (*strs2).i {
                ((*strs2).icd.dtor)
                    .unwrap()(
                    ((*strs2).d)
                        .offset(
                            ((*strs2).icd.sz).wrapping_mul(_ut_i_5 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_5 = _ut_i_5.wrapping_add(1);
                _ut_i_5;
            }
        }
        (*strs2).i = 0 as libc::c_int as libc::c_uint;
    }
    if !((*strs).i == 0 as libc::c_int as libc::c_uint) {
        if (*strs2).i > (*strs2).i {
            let mut _ut_i_6: libc::c_uint = 0;
            if (*strs2).i > (*strs2).i {
                if ((*strs2).icd.dtor).is_some() {
                    _ut_i_6 = (*strs2).i;
                    while _ut_i_6 < (*strs2).i {
                        ((*strs2).icd.dtor)
                            .unwrap()(
                            ((*strs2).d)
                                .offset(
                                    ((*strs2).icd.sz).wrapping_mul(_ut_i_6 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_6 = _ut_i_6.wrapping_add(1);
                        _ut_i_6;
                    }
                }
            } else if (*strs2).i < (*strs2).i {
                if ((*strs2).i).wrapping_add(((*strs2).i).wrapping_sub((*strs2).i))
                    > (*strs2).n
                {
                    let mut utarray_tmp_9: *mut libc::c_char = 0 as *mut libc::c_char;
                    while ((*strs2).i)
                        .wrapping_add(((*strs2).i).wrapping_sub((*strs2).i)) > (*strs2).n
                    {
                        (*strs2)
                            .n = if (*strs2).n != 0 {
                            (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
                        } else {
                            8 as libc::c_int as libc::c_uint
                        };
                    }
                    utarray_tmp_9 = realloc(
                        (*strs2).d as *mut libc::c_void,
                        ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
                    ) as *mut libc::c_char;
                    if utarray_tmp_9.is_null() {
                        exit(-(1 as libc::c_int));
                    }
                    (*strs2).d = utarray_tmp_9;
                }
                if ((*strs2).icd.init).is_some() {
                    _ut_i_6 = (*strs2).i;
                    while _ut_i_6 < (*strs2).i {
                        ((*strs2).icd.init)
                            .unwrap()(
                            ((*strs2).d)
                                .offset(
                                    ((*strs2).icd.sz).wrapping_mul(_ut_i_6 as libc::c_ulong)
                                        as isize,
                                ) as *mut libc::c_void,
                        );
                        _ut_i_6 = _ut_i_6.wrapping_add(1);
                        _ut_i_6;
                    }
                } else {
                    memset(
                        ((*strs2).d)
                            .offset(
                                ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void,
                        0 as libc::c_int,
                        ((*strs2).icd.sz)
                            .wrapping_mul(
                                ((*strs2).i).wrapping_sub((*strs2).i) as libc::c_ulong,
                            ),
                    );
                }
            }
            (*strs2).i = (*strs2).i;
        }
        if ((*strs2).i).wrapping_add((*strs).i) > (*strs2).n {
            let mut utarray_tmp_10: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*strs2).i).wrapping_add((*strs).i) > (*strs2).n {
                (*strs2)
                    .n = if (*strs2).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs2).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp_10 = realloc(
                (*strs2).d as *mut libc::c_void,
                ((*strs2).n as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp_10.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*strs2).d = utarray_tmp_10;
        }
        if (*strs2).i < (*strs2).i {
            memmove(
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz)
                            .wrapping_mul(
                                ((*strs2).i).wrapping_add((*strs).i) as libc::c_ulong,
                            ) as isize,
                    ) as *mut libc::c_void,
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                (((*strs2).i).wrapping_sub((*strs2).i) as libc::c_ulong)
                    .wrapping_mul((*strs2).icd.sz),
            );
        }
        if ((*strs2).icd.copy).is_some() {
            let mut _ut_i_7: libc::c_uint = 0;
            _ut_i_7 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_7 < (*strs).i {
                ((*strs2).icd.copy)
                    .unwrap()(
                    ((*strs2).d)
                        .offset(
                            ((*strs2).icd.sz)
                                .wrapping_mul(
                                    ((*strs2).i).wrapping_add(_ut_i_7) as libc::c_ulong,
                                ) as isize,
                        ) as *mut libc::c_void,
                    ((*strs).d)
                        .offset(
                            ((*strs).icd.sz).wrapping_mul(_ut_i_7 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_7 = _ut_i_7.wrapping_add(1);
                _ut_i_7;
            }
        } else {
            memcpy(
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul((*strs2).i as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*strs).d)
                    .offset(
                        ((*strs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void,
                ((*strs).i as libc::c_ulong).wrapping_mul((*strs2).icd.sz),
            );
        }
        (*strs2).i = ((*strs2).i).wrapping_add((*strs).i);
    }
    loop {
        p = (if p.is_null() {
            if (*strs2).i != 0 {
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs2).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs2).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs2).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs2).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs2).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"sorting strs2\n\0" as *const u8 as *const libc::c_char);
    qsort(
        (*strs2).d as *mut libc::c_void,
        (*strs2).i as size_t,
        (*strs2).icd.sz,
        Some(
            strsort
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    loop {
        p = (if p.is_null() {
            if (*strs2).i != 0 {
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs2).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs2).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs2).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs2).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs2).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"reverse sorting strs2\n\0" as *const u8 as *const libc::c_char);
    qsort(
        (*strs2).d as *mut libc::c_void,
        (*strs2).i as size_t,
        (*strs2).icd.sz,
        Some(
            revsort
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    loop {
        p = (if p.is_null() {
            if (*strs2).i != 0 {
                ((*strs2).d)
                    .offset(
                        ((*strs2).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*strs2).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*strs2).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*strs2).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*strs2).d)
                .offset(
                    ((*strs2).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*strs2).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*strs2).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut *mut libc::c_char;
        if p.is_null() {
            break;
        }
        printf(b"%s \0" as *const u8 as *const libc::c_char, *p);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if (*strs2).i > 0 as libc::c_int as libc::c_uint {
        if ((*strs2).icd.dtor).is_some() {
            let mut _ut_i_8: libc::c_uint = 0;
            _ut_i_8 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_8 < (*strs2).i {
                ((*strs2).icd.dtor)
                    .unwrap()(
                    ((*strs2).d)
                        .offset(
                            ((*strs2).icd.sz).wrapping_mul(_ut_i_8 as libc::c_ulong)
                                as isize,
                        ) as *mut libc::c_void,
                );
                _ut_i_8 = _ut_i_8.wrapping_add(1);
                _ut_i_8;
            }
        }
        (*strs2).i = 0 as libc::c_int as libc::c_uint;
    }
    if (*strs2).n != 0 {
        if ((*strs2).icd.dtor).is_some() {
            let mut _ut_i_9: libc::c_uint = 0;
            _ut_i_9 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_9 < (*strs2).i {
                ((*strs2).icd.dtor)
                    .unwrap()(
                    if _ut_i_9 < (*strs2).i {
                        ((*strs2).d)
                            .offset(
                                ((*strs2).icd.sz).wrapping_mul(_ut_i_9 as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    },
                );
                _ut_i_9 = _ut_i_9.wrapping_add(1);
                _ut_i_9;
            }
        }
        free((*strs2).d as *mut libc::c_void);
    }
    (*strs2).n = 0 as libc::c_int as libc::c_uint;
    free(strs2 as *mut libc::c_void);
    if (*strs).n != 0 {
        if ((*strs).icd.dtor).is_some() {
            let mut _ut_i_10: libc::c_uint = 0;
            _ut_i_10 = 0 as libc::c_int as libc::c_uint;
            while _ut_i_10 < (*strs).i {
                ((*strs).icd.dtor)
                    .unwrap()(
                    if _ut_i_10 < (*strs).i {
                        ((*strs).d)
                            .offset(
                                ((*strs).icd.sz).wrapping_mul(_ut_i_10 as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    },
                );
                _ut_i_10 = _ut_i_10.wrapping_add(1);
                _ut_i_10;
            }
        }
        free((*strs).d as *mut libc::c_void);
    }
    (*strs).n = 0 as libc::c_int as libc::c_uint;
    free(strs as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
