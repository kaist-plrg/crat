use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static is_basic_table: [libc::c_uint; 0];
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn iswspace(__wc: wint_t) -> libc::c_int;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub const _ISspace: C2RustUnnamed_0 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type mbi_iterator_t = mbiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbiter_multi {
    pub limit: *const libc::c_char,
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
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
#[inline]
unsafe extern "C" fn mbiter_multi_next(mut iter: *mut mbiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 17072428756486872483;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as libc::c_int as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as libc::c_int != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                b"./mbiter.h\0" as *const u8 as *const libc::c_char,
                136 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_2021: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                    b"./mbiter.h\0" as *const u8 as *const libc::c_char,
                    136 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 46],
                        &[libc::c_char; 46],
                    >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as libc::c_int != 0;
        current_block = 17072428756486872483;
    }
    match current_block {
        17072428756486872483 => {
            (*iter)
                .cur
                .bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                ((*iter).limit).offset_from((*iter).cur.ptr) as libc::c_long as size_t,
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as libc::c_int) as size_t {
                (*iter).cur.bytes = 1 as libc::c_int as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else if (*iter).cur.bytes == -(2 as libc::c_int) as size_t {
                (*iter)
                    .cur
                    .bytes = ((*iter).limit).offset_from((*iter).cur.ptr) as libc::c_long
                    as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else {
                if (*iter).cur.bytes == 0 as libc::c_int as libc::c_ulong {
                    (*iter).cur.bytes = 1 as libc::c_int as size_t;
                    if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                        __assert_fail(
                            b"*iter->cur.ptr == '\\0'\0" as *const u8
                                as *const libc::c_char,
                            b"./mbiter.h\0" as *const u8 as *const libc::c_char,
                            163 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 46],
                                &[libc::c_char; 46],
                            >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_1853: {
                        if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8
                                    as *const libc::c_char,
                                b"./mbiter.h\0" as *const u8 as *const libc::c_char,
                                163 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 46],
                                    &[libc::c_char; 46],
                                >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*iter).cur.wc == 0 as libc::c_int {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                            b"./mbiter.h\0" as *const u8 as *const libc::c_char,
                            164 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 46],
                                &[libc::c_char; 46],
                            >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_1806: {
                        if (*iter).cur.wc == 0 as libc::c_int {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                                b"./mbiter.h\0" as *const u8 as *const libc::c_char,
                                164 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 46],
                                    &[libc::c_char; 46],
                                >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
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
pub unsafe extern "C" fn trim2(
    mut s: *const libc::c_char,
    mut how: libc::c_int,
) -> *mut libc::c_char {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    d = strdup(s);
    if d.is_null() {
        xalloc_die();
    }
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        let mut i: mbi_iterator_t = mbi_iterator_t {
            limit: 0 as *const libc::c_char,
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
        if how != 0 as libc::c_int {
            i.cur.ptr = d;
            i.limit = (i.cur.ptr).offset(strlen(d) as isize);
            i.in_shift = 0 as libc::c_int != 0;
            memset(
                &mut i.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            i.next_done = 0 as libc::c_int != 0;
            while i.cur.ptr < i.limit
                && {
                    mbiter_multi_next(&mut i);
                    1 as libc::c_int != 0
                }
                && (i.cur.wc_valid as libc::c_int != 0
                    && iswspace(i.cur.wc as wint_t) != 0)
            {
                i.cur.ptr = (i.cur.ptr).offset(i.cur.bytes as isize);
                i.next_done = 0 as libc::c_int != 0;
            }
            memmove(
                d as *mut libc::c_void,
                i.cur.ptr as *const libc::c_void,
                (strlen(i.cur.ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        if how != 1 as libc::c_int {
            let mut start_of_spaces: *mut libc::c_char = 0 as *mut libc::c_char;
            i.cur.ptr = d;
            i.limit = (i.cur.ptr).offset(strlen(d) as isize);
            i.in_shift = 0 as libc::c_int != 0;
            memset(
                &mut i.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            i.next_done = 0 as libc::c_int != 0;
            while i.cur.ptr < i.limit
                && {
                    mbiter_multi_next(&mut i);
                    1 as libc::c_int != 0
                }
            {
                if i.cur.wc_valid as libc::c_int != 0
                    && iswspace(i.cur.wc as wint_t) != 0
                {
                    if start_of_spaces.is_null() {
                        start_of_spaces = i.cur.ptr as *mut libc::c_char;
                    }
                } else {
                    start_of_spaces = 0 as *mut libc::c_char;
                }
                i.cur.ptr = (i.cur.ptr).offset(i.cur.bytes as isize);
                i.next_done = 0 as libc::c_int != 0;
            }
            if !start_of_spaces.is_null() {
                *start_of_spaces = '\0' as i32 as libc::c_char;
            }
        }
    } else {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if how != 0 as libc::c_int {
            p = d;
            while *p as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                p = p.offset(1);
                p;
            }
            memmove(
                d as *mut libc::c_void,
                p as *const libc::c_void,
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        if how != 1 as libc::c_int {
            p = d.offset(strlen(d) as isize).offset(-(1 as libc::c_int as isize));
            while p >= d
                && *(*__ctype_b_loc())
                    .offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                *p = '\0' as i32 as libc::c_char;
                p = p.offset(-1);
                p;
            }
        }
    }
    return d;
}
