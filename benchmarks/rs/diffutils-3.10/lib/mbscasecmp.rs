use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn towlower(__wc: wint_t) -> wint_t;
    static is_basic_table: [libc::c_uint; 0];
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __int32_t = libc::c_int;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type mbui_iterator_t = mbuiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type wint_t = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
#[inline]
unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 2066641915258423827;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as libc::c_int as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as libc::c_int != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_1130: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                    b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                    144 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as libc::c_int != 0;
        current_block = 2066641915258423827;
    }
    match current_block {
        2066641915258423827 => {
            (*iter)
                .cur
                .bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                strnlen1((*iter).cur.ptr, __ctype_get_mb_cur_max()),
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as libc::c_int) as size_t {
                (*iter).cur.bytes = 1 as libc::c_int as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else if (*iter).cur.bytes == -(2 as libc::c_int) as size_t {
                (*iter).cur.bytes = strlen((*iter).cur.ptr);
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else {
                if (*iter).cur.bytes == 0 as libc::c_int as libc::c_ulong {
                    (*iter).cur.bytes = 1 as libc::c_int as size_t;
                    if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                        __assert_fail(
                            b"*iter->cur.ptr == '\\0'\0" as *const u8
                                as *const libc::c_char,
                            b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                            172 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_954: {
                        if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8
                                    as *const libc::c_char,
                                b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                                172 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*iter).cur.wc == 0 as libc::c_int {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                            b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                            173 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_907: {
                        if (*iter).cur.wc == 0 as libc::c_int {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                                b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                                173 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                (*iter).cur.wc_valid = 1 as libc::c_int != 0;
                if mbsinit(&mut (*iter).state) != 0 {
                    (*iter).in_shift = 0 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    (*iter).next_done = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn mbscasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    if s1 == s2 {
        return 0 as libc::c_int;
    }
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        let mut iter1: mbui_iterator_t = mbui_iterator_t {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const libc::c_char,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        let mut iter2: mbui_iterator_t = mbui_iterator_t {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const libc::c_char,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        iter1.cur.ptr = s1;
        iter1.in_shift = 0 as libc::c_int != 0;
        memset(
            &mut iter1.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        iter1.next_done = 0 as libc::c_int != 0;
        iter2.cur.ptr = s2;
        iter2.in_shift = 0 as libc::c_int != 0;
        memset(
            &mut iter2.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        iter2.next_done = 0 as libc::c_int != 0;
        loop {
            mbuiter_multi_next(&mut iter1);
            if !(!(iter1.cur.wc_valid as libc::c_int != 0
                && iter1.cur.wc == 0 as libc::c_int) as libc::c_int != 0
                && {
                    mbuiter_multi_next(&mut iter2);
                    !(iter2.cur.wc_valid as libc::c_int != 0
                        && iter2.cur.wc == 0 as libc::c_int) as libc::c_int != 0
                })
            {
                break;
            }
            let mut cmp: libc::c_int = if iter1.cur.wc_valid as libc::c_int != 0 {
                if iter2.cur.wc_valid as libc::c_int != 0 {
                    towlower(iter1.cur.wc as wint_t) as libc::c_int
                        - towlower(iter2.cur.wc as wint_t) as libc::c_int
                } else {
                    -(1 as libc::c_int)
                }
            } else if iter2.cur.wc_valid as libc::c_int != 0 {
                1 as libc::c_int
            } else if iter1.cur.bytes == iter2.cur.bytes {
                memcmp(
                    iter1.cur.ptr as *const libc::c_void,
                    iter2.cur.ptr as *const libc::c_void,
                    iter1.cur.bytes,
                )
            } else if iter1.cur.bytes < iter2.cur.bytes {
                if memcmp(
                    iter1.cur.ptr as *const libc::c_void,
                    iter2.cur.ptr as *const libc::c_void,
                    iter1.cur.bytes,
                ) > 0 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                }
            } else if memcmp(
                iter1.cur.ptr as *const libc::c_void,
                iter2.cur.ptr as *const libc::c_void,
                iter2.cur.bytes,
            ) >= 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
            if cmp != 0 as libc::c_int {
                return cmp;
            }
            iter1.cur.ptr = (iter1.cur.ptr).offset(iter1.cur.bytes as isize);
            iter1.next_done = 0 as libc::c_int != 0;
            iter2.cur.ptr = (iter2.cur.ptr).offset(iter2.cur.bytes as isize);
            iter2.next_done = 0 as libc::c_int != 0;
        }
        mbuiter_multi_next(&mut iter1);
        if !(iter1.cur.wc_valid as libc::c_int != 0 && iter1.cur.wc == 0 as libc::c_int)
            as libc::c_int != 0
        {
            return 1 as libc::c_int;
        }
        mbuiter_multi_next(&mut iter2);
        if !(iter2.cur.wc_valid as libc::c_int != 0 && iter2.cur.wc == 0 as libc::c_int)
            as libc::c_int != 0
        {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else {
        let mut p1: *const libc::c_uchar = s1 as *const libc::c_uchar;
        let mut p2: *const libc::c_uchar = s2 as *const libc::c_uchar;
        let mut c1: libc::c_uchar = 0;
        let mut c2: libc::c_uchar = 0;
        loop {
            c1 = (if *(*__ctype_b_loc()).offset(*p1 as libc::c_int as isize)
                as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                {
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *p1 as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*p1 as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*p1 as libc::c_int as isize);
                    }
                    __res
                }
            } else {
                *p1 as libc::c_int
            }) as libc::c_uchar;
            c2 = (if *(*__ctype_b_loc()).offset(*p2 as libc::c_int as isize)
                as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                {
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *p2 as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*p2 as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*p2 as libc::c_int as isize);
                    }
                    __res
                }
            } else {
                *p2 as libc::c_int
            }) as libc::c_uchar;
            if c1 as libc::c_int == '\0' as i32 {
                break;
            }
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
            if !(c1 as libc::c_int == c2 as libc::c_int) {
                break;
            }
        }
        if 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
            <= 2147483647 as libc::c_int
        {
            return c1 as libc::c_int - c2 as libc::c_int
        } else {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int
                - ((c1 as libc::c_int) < c2 as libc::c_int) as libc::c_int
        }
    };
}
