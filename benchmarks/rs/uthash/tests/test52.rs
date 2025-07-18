use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
pub struct intchar_t {
    pub a: libc::c_int,
    pub s: *mut libc::c_char,
}
unsafe extern "C" fn intchar_copy(
    mut _dst: *mut libc::c_void,
    mut _src: *const libc::c_void,
) {
    let mut dst: *mut intchar_t = _dst as *mut intchar_t;
    let mut src: *const intchar_t = _src as *const intchar_t;
    (*dst).a = (*src).a;
    (*dst)
        .s = if !((*src).s).is_null() {
        strdup((*src).s)
    } else {
        0 as *mut libc::c_char
    };
}
unsafe extern "C" fn intchar_dtor(mut _elt: *mut libc::c_void) {
    let mut elt: *mut intchar_t = _elt as *mut intchar_t;
    if !((*elt).s).is_null() {
        free((*elt).s as *mut libc::c_void);
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut intchars: *mut UT_array = 0 as *mut UT_array;
    let mut ic: intchar_t = intchar_t {
        a: 0,
        s: 0 as *mut libc::c_char,
    };
    let mut p: *mut intchar_t = 0 as *mut intchar_t;
    let mut intchar_icd: UT_icd = {
        let mut init = UT_icd {
            sz: ::std::mem::size_of::<intchar_t>() as libc::c_ulong,
            init: None,
            copy: Some(
                intchar_copy
                    as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> (),
            ),
            dtor: Some(intchar_dtor as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    };
    intchars = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong)
        as *mut UT_array;
    if intchars.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        intchars as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*intchars).icd = intchar_icd;
    ic.a = 1 as libc::c_int;
    ic.s = b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if ((*intchars).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*intchars).n {
        let mut utarray_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*intchars).i).wrapping_add(1 as libc::c_int as libc::c_uint)
            > (*intchars).n
        {
            (*intchars)
                .n = if (*intchars).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*intchars).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp = realloc(
            (*intchars).d as *mut libc::c_void,
            ((*intchars).n as libc::c_ulong).wrapping_mul((*intchars).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*intchars).d = utarray_tmp;
    }
    if ((*intchars).icd.copy).is_some() {
        let fresh0 = (*intchars).i;
        (*intchars).i = ((*intchars).i).wrapping_add(1);
        ((*intchars).icd.copy)
            .unwrap()(
            ((*intchars).d)
                .offset(
                    ((*intchars).icd.sz).wrapping_mul(fresh0 as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            &mut ic as *mut intchar_t as *const libc::c_void,
        );
    } else {
        let fresh1 = (*intchars).i;
        (*intchars).i = ((*intchars).i).wrapping_add(1);
        memcpy(
            ((*intchars).d)
                .offset(
                    ((*intchars).icd.sz).wrapping_mul(fresh1 as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            &mut ic as *mut intchar_t as *const libc::c_void,
            (*intchars).icd.sz,
        );
    }
    ic.a = 2 as libc::c_int;
    ic.s = b"world\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if ((*intchars).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*intchars).n {
        let mut utarray_tmp_0: *mut libc::c_char = 0 as *mut libc::c_char;
        while ((*intchars).i).wrapping_add(1 as libc::c_int as libc::c_uint)
            > (*intchars).n
        {
            (*intchars)
                .n = if (*intchars).n != 0 {
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*intchars).n)
            } else {
                8 as libc::c_int as libc::c_uint
            };
        }
        utarray_tmp_0 = realloc(
            (*intchars).d as *mut libc::c_void,
            ((*intchars).n as libc::c_ulong).wrapping_mul((*intchars).icd.sz),
        ) as *mut libc::c_char;
        if utarray_tmp_0.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*intchars).d = utarray_tmp_0;
    }
    if ((*intchars).icd.copy).is_some() {
        let fresh2 = (*intchars).i;
        (*intchars).i = ((*intchars).i).wrapping_add(1);
        ((*intchars).icd.copy)
            .unwrap()(
            ((*intchars).d)
                .offset(
                    ((*intchars).icd.sz).wrapping_mul(fresh2 as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            &mut ic as *mut intchar_t as *const libc::c_void,
        );
    } else {
        let fresh3 = (*intchars).i;
        (*intchars).i = ((*intchars).i).wrapping_add(1);
        memcpy(
            ((*intchars).d)
                .offset(
                    ((*intchars).icd.sz).wrapping_mul(fresh3 as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            &mut ic as *mut intchar_t as *const libc::c_void,
            (*intchars).icd.sz,
        );
    }
    p = 0 as *mut intchar_t;
    loop {
        p = (if p.is_null() {
            if (*intchars).i != 0 {
                ((*intchars).d)
                    .offset(
                        ((*intchars).icd.sz)
                            .wrapping_mul(0 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if (*intchars).i as libc::c_ulong
            != ((p as *mut libc::c_char).offset_from((*intchars).d) as libc::c_long
                as libc::c_ulong)
                .wrapping_div((*intchars).icd.sz)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            ((*intchars).d)
                .offset(
                    ((*intchars).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*intchars).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*intchars).icd.sz)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut intchar_t;
        if p.is_null() {
            break;
        }
        printf(
            b"%d %s\n\0" as *const u8 as *const libc::c_char,
            (*p).a,
            if !((*p).s).is_null() {
                (*p).s as *const libc::c_char
            } else {
                b"null\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if (*intchars).n != 0 {
        if ((*intchars).icd.dtor).is_some() {
            let mut _ut_i: libc::c_uint = 0;
            _ut_i = 0 as libc::c_int as libc::c_uint;
            while _ut_i < (*intchars).i {
                ((*intchars).icd.dtor)
                    .unwrap()(
                    if _ut_i < (*intchars).i {
                        ((*intchars).d)
                            .offset(
                                ((*intchars).icd.sz).wrapping_mul(_ut_i as libc::c_ulong)
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
        free((*intchars).d as *mut libc::c_void);
    }
    (*intchars).n = 0 as libc::c_int as libc::c_uint;
    free(intchars as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
