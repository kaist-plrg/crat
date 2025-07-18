use ::libc;
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t;
    static is_basic_table: [libc::c_uint; 0];
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
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
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type mbchar_t = mbchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbuiter_multi_copy(
    mut new_iter: *mut mbuiter_multi,
    mut old_iter: *const mbuiter_multi,
) {
    (*new_iter).in_shift = (*old_iter).in_shift;
    if (*new_iter).in_shift {
        memcpy(
            &mut (*new_iter).state as *mut mbstate_t as *mut libc::c_void,
            &(*old_iter).state as *const mbstate_t as *const libc::c_void,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    } else {
        memset(
            &mut (*new_iter).state as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    }
    (*new_iter).next_done = (*old_iter).next_done;
    mb_copy(&mut (*new_iter).cur, &(*old_iter).cur);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbuiter_multi_reloc(
    mut iter: *mut mbuiter_multi,
    mut ptrdiff: ptrdiff_t,
) {
    (*iter).cur.ptr = ((*iter).cur.ptr).offset(ptrdiff as isize);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 9814799094603046640;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as libc::c_int as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as libc::c_int != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                b"lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_2888: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                    b"lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                    143 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as libc::c_int != 0;
        current_block = 9814799094603046640;
    }
    match current_block {
        9814799094603046640 => {
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
                            b"lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                            171 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_2725: {
                        if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8
                                    as *const libc::c_char,
                                b"lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                                171 as libc::c_int as libc::c_uint,
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
                            b"lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                            172 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_2678: {
                        if (*iter).cur.wc == 0 as libc::c_int {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                                b"lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                                172 as libc::c_int as libc::c_uint,
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
#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
#[inline]
unsafe extern "C" fn mb_copy(mut new_mbc: *mut mbchar_t, mut old_mbc: *const mbchar_t) {
    if (*old_mbc).ptr
        == &*((*old_mbc).buf).as_ptr().offset(0 as libc::c_int as isize)
            as *const libc::c_char
    {
        memcpy(
            &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_char as *mut libc::c_void,
            &*((*old_mbc).buf).as_ptr().offset(0 as libc::c_int as isize)
                as *const libc::c_char as *const libc::c_void,
            (*old_mbc).bytes,
        );
        (*new_mbc)
            .ptr = &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_char;
    } else {
        (*new_mbc).ptr = (*old_mbc).ptr;
    }
    (*new_mbc).bytes = (*old_mbc).bytes;
    (*new_mbc).wc_valid = (*old_mbc).wc_valid;
    if (*new_mbc).wc_valid {
        (*new_mbc).wc = (*old_mbc).wc;
    }
}
