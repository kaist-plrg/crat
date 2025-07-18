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
    let mut nums: *mut UT_array = 0 as *mut UT_array;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    nums = malloc(::std::mem::size_of::<UT_array>() as libc::c_ulong) as *mut UT_array;
    if nums.is_null() {
        exit(-(1 as libc::c_int));
    }
    memset(
        nums as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<UT_array>() as libc::c_ulong,
    );
    (*nums).icd = ut_int_icd;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if ((*nums).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*nums).n {
            let mut utarray_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
            while ((*nums).i).wrapping_add(1 as libc::c_int as libc::c_uint) > (*nums).n
            {
                (*nums)
                    .n = if (*nums).n != 0 {
                    (2 as libc::c_int as libc::c_uint).wrapping_mul((*nums).n)
                } else {
                    8 as libc::c_int as libc::c_uint
                };
            }
            utarray_tmp = realloc(
                (*nums).d as *mut libc::c_void,
                ((*nums).n as libc::c_ulong).wrapping_mul((*nums).icd.sz),
            ) as *mut libc::c_char;
            if utarray_tmp.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*nums).d = utarray_tmp;
        }
        if ((*nums).icd.copy).is_some() {
            let fresh0 = (*nums).i;
            (*nums).i = ((*nums).i).wrapping_add(1);
            ((*nums).icd.copy)
                .unwrap()(
                ((*nums).d)
                    .offset(
                        ((*nums).icd.sz).wrapping_mul(fresh0 as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
                &mut i as *mut libc::c_int as *const libc::c_void,
            );
        } else {
            let fresh1 = (*nums).i;
            (*nums).i = ((*nums).i).wrapping_add(1);
            memcpy(
                ((*nums).d)
                    .offset(
                        ((*nums).icd.sz).wrapping_mul(fresh1 as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
                &mut i as *mut libc::c_int as *const libc::c_void,
                (*nums).icd.sz,
            );
        }
        i += 1;
        i;
    }
    p = (if (*nums).i != 0 {
        ((*nums).d)
            .offset(
                ((*nums).icd.sz)
                    .wrapping_mul(
                        ((*nums).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_int;
    while !p.is_null() {
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, *p);
        p = (if p.is_null() {
            if (*nums).i != 0 {
                ((*nums).d)
                    .offset(
                        ((*nums).icd.sz)
                            .wrapping_mul(
                                ((*nums).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong,
                            ) as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if ((p as *mut libc::c_char).offset_from((*nums).d) as libc::c_long
            as libc::c_ulong)
            .wrapping_div((*nums).icd.sz) != 0 as libc::c_int as libc::c_ulong
        {
            ((*nums).d)
                .offset(
                    ((*nums).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*nums).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*nums).icd.sz)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_int;
    }
    p = 0 as *mut libc::c_int;
    loop {
        p = (if p.is_null() {
            if (*nums).i != 0 {
                ((*nums).d)
                    .offset(
                        ((*nums).icd.sz)
                            .wrapping_mul(
                                ((*nums).i).wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong,
                            ) as isize,
                    ) as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }
        } else if ((p as *mut libc::c_char).offset_from((*nums).d) as libc::c_long
            as libc::c_ulong)
            .wrapping_div((*nums).icd.sz) != 0 as libc::c_int as libc::c_ulong
        {
            ((*nums).d)
                .offset(
                    ((*nums).icd.sz)
                        .wrapping_mul(
                            ((p as *mut libc::c_char).offset_from((*nums).d)
                                as libc::c_long as libc::c_ulong)
                                .wrapping_div((*nums).icd.sz)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                ) as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as *mut libc::c_int;
        if p.is_null() {
            break;
        }
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, *p);
    }
    if (*nums).n != 0 {
        if ((*nums).icd.dtor).is_some() {
            let mut _ut_i: libc::c_uint = 0;
            _ut_i = 0 as libc::c_int as libc::c_uint;
            while _ut_i < (*nums).i {
                ((*nums).icd.dtor)
                    .unwrap()(
                    if _ut_i < (*nums).i {
                        ((*nums).d)
                            .offset(
                                ((*nums).icd.sz).wrapping_mul(_ut_i as libc::c_ulong)
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
        free((*nums).d as *mut libc::c_void);
    }
    (*nums).n = 0 as libc::c_int as libc::c_uint;
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
