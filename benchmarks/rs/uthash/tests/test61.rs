use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
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
    fn bsearch(
        __key: *const libc::c_void,
        __base: *const libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    ) -> *mut libc::c_void;
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
unsafe fn main_0() -> libc::c_int {
    let mut strs: *mut UT_array = 0 as *mut UT_array;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
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
    s = b"hello\0" as *const u8 as *const libc::c_char;
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
            &mut s as *mut *const libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh1 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        memcpy(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh1 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    s = b"world\0" as *const u8 as *const libc::c_char;
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
            &mut s as *mut *const libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh3 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        memcpy(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh3 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    s = b"one\0" as *const u8 as *const libc::c_char;
    if ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
        let mut utarray_tmp_1: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
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
    if ((*strs).icd.copy).is_some() {
        let fresh4 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        ((*strs).icd.copy)
            .unwrap()(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh4 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh5 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        memcpy(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh5 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    s = b"two\0" as *const u8 as *const libc::c_char;
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
    if ((*strs).icd.copy).is_some() {
        let fresh6 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        ((*strs).icd.copy)
            .unwrap()(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh6 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh7 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        memcpy(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh7 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    s = b"three\0" as *const u8 as *const libc::c_char;
    if ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
        let mut utarray_tmp_3: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*strs).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*strs).n {
            (*strs)
                .n = if (*strs).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*strs).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_3 = realloc(
            (*strs).d as *mut libc::c_void,
            ((*strs).n as libc::c_ulong).wrapping_mul((*strs).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_3.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*strs).d = utarray_tmp_3;
    }
    if ((*strs).icd.copy).is_some() {
        let fresh8 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        ((*strs).icd.copy)
            .unwrap()(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh8 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
        );
    } else {
        let fresh9 = (*strs).i;
        (*strs).i = ((*strs).i).wrapping_add(1);
        memcpy(
            ((*strs).d)
                .offset(((*strs).icd.sz).wrapping_mul(fresh9 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut s as *mut *const libc::c_char as *const libc::c_void,
            (*strs).icd.sz,
        );
    }
    p = 0 as *mut *const libc::c_char;
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
        }) as *mut *const libc::c_char;
        if p.is_null() {
            break;
        }
        s = *p;
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, s);
    }
    printf(b"sorting\n\0" as *const u8 as *const libc::c_char);
    qsort(
        (*strs).d as *mut libc::c_void,
        (*strs).i as size_t,
        (*strs).icd.sz,
        Some(
            strsort
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    p = 0 as *mut *const libc::c_char;
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
        }) as *mut *const libc::c_char;
        if p.is_null() {
            break;
        }
        s = *p;
        printf(b"finding %s\n\0" as *const u8 as *const libc::c_char, s);
        p = bsearch(
            &mut s as *mut *const libc::c_char as *const libc::c_void,
            (*strs).d as *const libc::c_void,
            (*strs).i as size_t,
            (*strs).icd.sz,
            Some(
                strsort
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut *const libc::c_char;
        printf(
            b" %s\n\0" as *const u8 as *const libc::c_char,
            if !p.is_null() {
                *p
            } else {
                b"failed\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if (*strs).n != 0 {
        if ((*strs).icd.dtor).is_some() {
            let mut _ut_i: libc::c_uint = 0;
            _ut_i = 0 as libc::c_int as libc::c_uint;
            while _ut_i < (*strs).i {
                ((*strs).icd.dtor)
                    .unwrap()(
                    if _ut_i < (*strs).i {
                        ((*strs).d)
                            .offset(
                                ((*strs).icd.sz).wrapping_mul(_ut_i as libc::c_ulong)
                                    as isize,
                            ) as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    },
                );
                _ut_i = _ut_i.wrapping_add(1);
                _ut_i;
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
