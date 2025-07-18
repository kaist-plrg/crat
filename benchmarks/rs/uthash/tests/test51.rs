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
    let mut ip: intpair_t = intpair_t { a: 0, b: 0 };
    let mut p: *mut intpair_t = 0 as *mut intpair_t;
    let mut intpair_icd: UT_icd = {
        let mut init = UT_icd {
            sz: ::std::mem::size_of::<intpair_t>() as libc::c_ulong,
            init: None,
            copy: None,
            dtor: None,
        };
        init
    };
    pairs = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong) as *mut UT_array;
    if pairs.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        pairs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*pairs).icd = intpair_icd;
    ip.a = 1 as libc::c_int;
    ip.b = 2 as libc::c_int;
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
            &mut ip as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh1 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh1 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut ip as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    ip.a = 10 as libc::c_int;
    ip.b = 20 as libc::c_int;
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
            &mut ip as *mut intpair_t as *const libc::c_void,
        );
    } else {
        let fresh3 = (*pairs).i;
        (*pairs).i = ((*pairs).i).wrapping_add(1);
        memcpy(
            ((*pairs).d)
                .offset(((*pairs).icd.sz).wrapping_mul(fresh3 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            &mut ip as *mut intpair_t as *const libc::c_void,
            (*pairs).icd.sz,
        );
    }
    p = (if (*pairs).i != 0 {
        ((*pairs).d)
            .offset(
                ((*pairs).icd.sz).wrapping_mul(0 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as *mut intpair_t;
    while !p.is_null() {
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, (*p).a, (*p).b);
        p = (if p.is_null() {
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
            != ((p as *mut libc::c_char).offset_from((*pairs).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*pairs).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*pairs).d)
                .offset(
                    ((*pairs).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*pairs).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*pairs).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intpair_t;
    }
    if (*pairs).n != 0 {
        if ((*pairs).icd.dtor).is_some() {
            let mut _ut_i: libc::c_uint = 0;
            _ut_i = 0 as libc::c_int as libc::c_uint;
            while _ut_i < (*pairs).i {
                ((*pairs).icd.dtor)
                    .unwrap()(
                    if _ut_i < (*pairs).i {
                        ((*pairs).d)
                            .offset(
                                ((*pairs).icd.sz).wrapping_mul(_ut_i as libc::c_ulong)
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
        free((*pairs).d as *mut libc::c_void);
    }
    (*pairs).n = 0 as libc::c_int as libc::c_uint;
    free(pairs as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
