use ::libc;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mbslen(string: *const libc::c_char) -> size_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn abort() -> !;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn freea(p: *mut libc::c_void);
    fn mmalloca(n: size_t) -> *mut libc::c_void;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    static is_basic_table: [libc::c_uint; 0];
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type uintptr_t = libc::c_ulong;
pub const sa_alignment_max: C2RustUnnamed_0 = 16;
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
pub type mbchar_t = mbchar;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const sa_alignment_longdouble: C2RustUnnamed_0 = 16;
pub const sa_alignment_longlong: C2RustUnnamed_0 = 8;
pub const sa_alignment_double: C2RustUnnamed_0 = 8;
pub const sa_alignment_long: C2RustUnnamed_0 = 8;
#[inline]
unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 9213551160627794153;
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
        'c_1456: {
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
        current_block = 9213551160627794153;
    }
    match current_block {
        9213551160627794153 => {
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
                    'c_1282: {
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
                    'c_1229: {
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
unsafe extern "C" fn knuth_morris_pratt(
    mut haystack: *const libc::c_uchar,
    mut needle: *const libc::c_uchar,
    mut needle_len: size_t,
    mut resultp: *mut *const libc::c_uchar,
) -> bool {
    let mut m: size_t = needle_len;
    let mut table: *mut size_t = (if (if (9223372036854775807 as libc::c_long
        as libc::c_ulong) < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    })
        .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong) < m
    {
        0 as *mut libc::c_void
    } else if m.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        < (4032 as libc::c_int
            - (2 as libc::c_int * sa_alignment_max as libc::c_int - 1 as libc::c_int))
            as libc::c_ulong
    {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            m
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int * sa_alignment_max as libc::c_int) as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize,
        );
        ((fresh0.leak().as_mut_ptr() as *mut libc::c_char as uintptr_t)
            .wrapping_add(
                (2 as libc::c_int * sa_alignment_max as libc::c_int - 1 as libc::c_int)
                    as libc::c_ulong,
            )
            & !((2 as libc::c_int * sa_alignment_max as libc::c_int - 1 as libc::c_int)
                as uintptr_t)) as *mut libc::c_void
    } else {
        mmalloca(m.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong))
    }) as *mut size_t;
    if table.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    *table.offset(1 as libc::c_int as isize) = 1 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    i = 2 as libc::c_int as size_t;
    while i < m {
        let mut b: libc::c_uchar = *needle
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        loop {
            if b as libc::c_int == *needle.offset(j as isize) as libc::c_int {
                j = j.wrapping_add(1);
                *table.offset(i as isize) = i.wrapping_sub(j);
                break;
            } else if j == 0 as libc::c_int as libc::c_ulong {
                *table.offset(i as isize) = i;
                break;
            } else {
                j = j.wrapping_sub(*table.offset(j as isize));
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut j_0: size_t = 0;
    let mut rhaystack: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut phaystack: *const libc::c_uchar = 0 as *const libc::c_uchar;
    *resultp = 0 as *const libc::c_uchar;
    j_0 = 0 as libc::c_int as size_t;
    rhaystack = haystack;
    phaystack = haystack;
    while *phaystack as libc::c_int != 0 as libc::c_int {
        if *needle.offset(j_0 as isize) as libc::c_int == *phaystack as libc::c_int {
            j_0 = j_0.wrapping_add(1);
            j_0;
            phaystack = phaystack.offset(1);
            phaystack;
            if !(j_0 == m) {
                continue;
            }
            *resultp = rhaystack;
            break;
        } else if j_0 > 0 as libc::c_int as libc::c_ulong {
            rhaystack = rhaystack.offset(*table.offset(j_0 as isize) as isize);
            j_0 = (j_0 as libc::c_ulong).wrapping_sub(*table.offset(j_0 as isize))
                as size_t as size_t;
        } else {
            rhaystack = rhaystack.offset(1);
            rhaystack;
            phaystack = phaystack.offset(1);
            phaystack;
        }
    }
    freea(table as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn knuth_morris_pratt_multibyte(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
    mut resultp: *mut *const libc::c_char,
) -> bool {
    let mut m: size_t = mbslen(needle);
    let mut needle_mbchars: *mut mbchar_t = 0 as *mut mbchar_t;
    let mut table: *mut size_t = 0 as *mut size_t;
    let mut memory: *mut libc::c_void = if (if (9223372036854775807 as libc::c_long
        as libc::c_ulong) < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    })
        .wrapping_div(
            (::std::mem::size_of::<mbchar_t>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) < m
    {
        0 as *mut libc::c_void
    } else if m
        .wrapping_mul(
            (::std::mem::size_of::<mbchar_t>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
        )
        < (4032 as libc::c_int
            - (2 as libc::c_int * sa_alignment_max as libc::c_int - 1 as libc::c_int))
            as libc::c_ulong
    {
        let mut fresh1 = ::std::vec::from_elem(
            0,
            m
                .wrapping_mul(
                    (::std::mem::size_of::<mbchar_t>() as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
                )
                .wrapping_add(
                    (2 as libc::c_int * sa_alignment_max as libc::c_int) as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize,
        );
        ((fresh1.leak().as_mut_ptr() as *mut libc::c_char as uintptr_t)
            .wrapping_add(
                (2 as libc::c_int * sa_alignment_max as libc::c_int - 1 as libc::c_int)
                    as libc::c_ulong,
            )
            & !((2 as libc::c_int * sa_alignment_max as libc::c_int - 1 as libc::c_int)
                as uintptr_t)) as *mut libc::c_void
    } else {
        mmalloca(
            m
                .wrapping_mul(
                    (::std::mem::size_of::<mbchar_t>() as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong),
                ),
        )
    };
    let mut table_memory: *mut libc::c_void = 0 as *mut libc::c_void;
    if memory.is_null() {
        return 0 as libc::c_int != 0;
    }
    needle_mbchars = memory as *mut mbchar_t;
    table_memory = needle_mbchars.offset(m as isize) as *mut libc::c_void;
    table = table_memory as *mut size_t;
    let mut iter: mbui_iterator_t = mbui_iterator_t {
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
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    iter.cur.ptr = needle;
    iter.in_shift = 0 as libc::c_int != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    iter.next_done = 0 as libc::c_int != 0;
    loop {
        mbuiter_multi_next(&mut iter);
        if !(!(iter.cur.wc_valid as libc::c_int != 0 && iter.cur.wc == 0 as libc::c_int)
            as libc::c_int != 0)
        {
            break;
        }
        mb_copy(&mut *needle_mbchars.offset(j as isize), &mut iter.cur);
        iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
        iter.next_done = 0 as libc::c_int != 0;
        j = j.wrapping_add(1);
        j;
    }
    let mut i: size_t = 0;
    let mut j_0: size_t = 0;
    *table.offset(1 as libc::c_int as isize) = 1 as libc::c_int as size_t;
    j_0 = 0 as libc::c_int as size_t;
    i = 2 as libc::c_int as size_t;
    while i < m {
        let mut b: *mut mbchar_t = &mut *needle_mbchars
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as *mut mbchar_t;
        loop {
            if if (*b).wc_valid as libc::c_int != 0
                && (*needle_mbchars.offset(j_0 as isize)).wc_valid as libc::c_int != 0
            {
                ((*b).wc == (*needle_mbchars.offset(j_0 as isize)).wc) as libc::c_int
            } else {
                ((*b).bytes == (*needle_mbchars.offset(j_0 as isize)).bytes
                    && memcmp(
                        (*b).ptr as *const libc::c_void,
                        (*needle_mbchars.offset(j_0 as isize)).ptr
                            as *const libc::c_void,
                        (*b).bytes,
                    ) == 0 as libc::c_int) as libc::c_int
            } != 0
            {
                j_0 = j_0.wrapping_add(1);
                *table.offset(i as isize) = i.wrapping_sub(j_0);
                break;
            } else if j_0 == 0 as libc::c_int as libc::c_ulong {
                *table.offset(i as isize) = i;
                break;
            } else {
                j_0 = j_0.wrapping_sub(*table.offset(j_0 as isize));
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut j_1: size_t = 0;
    let mut rhaystack: mbui_iterator_t = mbui_iterator_t {
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
    let mut phaystack: mbui_iterator_t = mbui_iterator_t {
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
    *resultp = 0 as *const libc::c_char;
    j_1 = 0 as libc::c_int as size_t;
    rhaystack.cur.ptr = haystack;
    rhaystack.in_shift = 0 as libc::c_int != 0;
    memset(
        &mut rhaystack.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    rhaystack.next_done = 0 as libc::c_int != 0;
    phaystack.cur.ptr = haystack;
    phaystack.in_shift = 0 as libc::c_int != 0;
    memset(
        &mut phaystack.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    phaystack.next_done = 0 as libc::c_int != 0;
    loop {
        mbuiter_multi_next(&mut phaystack);
        if !(!(phaystack.cur.wc_valid as libc::c_int != 0
            && phaystack.cur.wc == 0 as libc::c_int) as libc::c_int != 0)
        {
            break;
        }
        if if (*needle_mbchars.offset(j_1 as isize)).wc_valid as libc::c_int != 0
            && phaystack.cur.wc_valid as libc::c_int != 0
        {
            ((*needle_mbchars.offset(j_1 as isize)).wc == phaystack.cur.wc)
                as libc::c_int
        } else {
            ((*needle_mbchars.offset(j_1 as isize)).bytes == phaystack.cur.bytes
                && memcmp(
                    (*needle_mbchars.offset(j_1 as isize)).ptr as *const libc::c_void,
                    phaystack.cur.ptr as *const libc::c_void,
                    (*needle_mbchars.offset(j_1 as isize)).bytes,
                ) == 0 as libc::c_int) as libc::c_int
        } != 0
        {
            j_1 = j_1.wrapping_add(1);
            j_1;
            phaystack.cur.ptr = (phaystack.cur.ptr).offset(phaystack.cur.bytes as isize);
            phaystack.next_done = 0 as libc::c_int != 0;
            if !(j_1 == m) {
                continue;
            }
            *resultp = rhaystack.cur.ptr;
            break;
        } else if j_1 > 0 as libc::c_int as libc::c_ulong {
            let mut count: size_t = *table.offset(j_1 as isize);
            j_1 = (j_1 as libc::c_ulong).wrapping_sub(count) as size_t as size_t;
            while count > 0 as libc::c_int as libc::c_ulong {
                mbuiter_multi_next(&mut rhaystack);
                if !(rhaystack.cur.wc_valid as libc::c_int != 0
                    && rhaystack.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                {
                    abort();
                }
                rhaystack
                    .cur
                    .ptr = (rhaystack.cur.ptr).offset(rhaystack.cur.bytes as isize);
                rhaystack.next_done = 0 as libc::c_int != 0;
                count = count.wrapping_sub(1);
                count;
            }
        } else {
            mbuiter_multi_next(&mut rhaystack);
            if !(rhaystack.cur.wc_valid as libc::c_int != 0
                && rhaystack.cur.wc == 0 as libc::c_int) as libc::c_int == 0
            {
                abort();
            }
            rhaystack.cur.ptr = (rhaystack.cur.ptr).offset(rhaystack.cur.bytes as isize);
            rhaystack.next_done = 0 as libc::c_int != 0;
            phaystack.cur.ptr = (phaystack.cur.ptr).offset(phaystack.cur.bytes as isize);
            phaystack.next_done = 0 as libc::c_int != 0;
        }
    }
    freea(memory);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn mbsstr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        let mut iter_needle: mbui_iterator_t = mbui_iterator_t {
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
        iter_needle.cur.ptr = needle;
        iter_needle.in_shift = 0 as libc::c_int != 0;
        memset(
            &mut iter_needle.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        iter_needle.next_done = 0 as libc::c_int != 0;
        mbuiter_multi_next(&mut iter_needle);
        if !(iter_needle.cur.wc_valid as libc::c_int != 0
            && iter_needle.cur.wc == 0 as libc::c_int) as libc::c_int != 0
        {
            let mut try_kmp: bool = 1 as libc::c_int != 0;
            let mut outer_loop_count: size_t = 0 as libc::c_int as size_t;
            let mut comparison_count: size_t = 0 as libc::c_int as size_t;
            let mut last_ccount: size_t = 0 as libc::c_int as size_t;
            let mut iter_needle_last_ccount: mbui_iterator_t = mbui_iterator_t {
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
            let mut iter_haystack: mbui_iterator_t = mbui_iterator_t {
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
            iter_needle_last_ccount.cur.ptr = needle;
            iter_needle_last_ccount.in_shift = 0 as libc::c_int != 0;
            memset(
                &mut iter_needle_last_ccount.state as *mut mbstate_t
                    as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            iter_needle_last_ccount.next_done = 0 as libc::c_int != 0;
            iter_haystack.cur.ptr = haystack;
            iter_haystack.in_shift = 0 as libc::c_int != 0;
            memset(
                &mut iter_haystack.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            iter_haystack.next_done = 0 as libc::c_int != 0;
            loop {
                mbuiter_multi_next(&mut iter_haystack);
                if !(iter_haystack.cur.wc_valid as libc::c_int != 0
                    && iter_haystack.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                {
                    return 0 as *mut libc::c_char;
                }
                if try_kmp as libc::c_int != 0
                    && outer_loop_count >= 10 as libc::c_int as libc::c_ulong
                    && comparison_count
                        >= (5 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(outer_loop_count)
                {
                    let mut count: size_t = comparison_count.wrapping_sub(last_ccount);
                    while count > 0 as libc::c_int as libc::c_ulong
                        && {
                            mbuiter_multi_next(&mut iter_needle_last_ccount);
                            !(iter_needle_last_ccount.cur.wc_valid as libc::c_int != 0
                                && iter_needle_last_ccount.cur.wc == 0 as libc::c_int)
                                as libc::c_int != 0
                        }
                    {
                        iter_needle_last_ccount
                            .cur
                            .ptr = (iter_needle_last_ccount.cur.ptr)
                            .offset(iter_needle_last_ccount.cur.bytes as isize);
                        iter_needle_last_ccount.next_done = 0 as libc::c_int != 0;
                        count = count.wrapping_sub(1);
                        count;
                    }
                    last_ccount = comparison_count;
                    mbuiter_multi_next(&mut iter_needle_last_ccount);
                    if !(iter_needle_last_ccount.cur.wc_valid as libc::c_int != 0
                        && iter_needle_last_ccount.cur.wc == 0 as libc::c_int)
                        as libc::c_int == 0
                    {
                        let mut result: *const libc::c_char = 0 as *const libc::c_char;
                        let mut success: bool = knuth_morris_pratt_multibyte(
                            haystack,
                            needle,
                            &mut result,
                        );
                        if success {
                            return result as *mut libc::c_char;
                        }
                        try_kmp = 0 as libc::c_int != 0;
                    }
                }
                outer_loop_count = outer_loop_count.wrapping_add(1);
                outer_loop_count;
                comparison_count = comparison_count.wrapping_add(1);
                comparison_count;
                if if iter_haystack.cur.wc_valid as libc::c_int != 0
                    && iter_needle.cur.wc_valid as libc::c_int != 0
                {
                    (iter_haystack.cur.wc == iter_needle.cur.wc) as libc::c_int
                } else {
                    (iter_haystack.cur.bytes == iter_needle.cur.bytes
                        && memcmp(
                            iter_haystack.cur.ptr as *const libc::c_void,
                            iter_needle.cur.ptr as *const libc::c_void,
                            iter_haystack.cur.bytes,
                        ) == 0 as libc::c_int) as libc::c_int
                } != 0
                {
                    let mut rhaystack: mbui_iterator_t = mbui_iterator_t {
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
                    let mut rneedle: mbui_iterator_t = mbui_iterator_t {
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
                    memcpy(
                        &mut rhaystack as *mut mbui_iterator_t as *mut libc::c_void,
                        &mut iter_haystack as *mut mbui_iterator_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<mbui_iterator_t>() as libc::c_ulong,
                    );
                    rhaystack
                        .cur
                        .ptr = (rhaystack.cur.ptr).offset(rhaystack.cur.bytes as isize);
                    rhaystack.next_done = 0 as libc::c_int != 0;
                    rneedle.cur.ptr = needle;
                    rneedle.in_shift = 0 as libc::c_int != 0;
                    memset(
                        &mut rneedle.state as *mut mbstate_t as *mut libc::c_void,
                        '\0' as i32,
                        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                    );
                    rneedle.next_done = 0 as libc::c_int != 0;
                    mbuiter_multi_next(&mut rneedle);
                    if !(rneedle.cur.wc_valid as libc::c_int != 0
                        && rneedle.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                    {
                        abort();
                    }
                    rneedle
                        .cur
                        .ptr = (rneedle.cur.ptr).offset(rneedle.cur.bytes as isize);
                    rneedle.next_done = 0 as libc::c_int != 0;
                    loop {
                        mbuiter_multi_next(&mut rneedle);
                        if !(rneedle.cur.wc_valid as libc::c_int != 0
                            && rneedle.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                        {
                            return iter_haystack.cur.ptr as *mut libc::c_char;
                        }
                        mbuiter_multi_next(&mut rhaystack);
                        if !(rhaystack.cur.wc_valid as libc::c_int != 0
                            && rhaystack.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                        {
                            return 0 as *mut libc::c_char;
                        }
                        comparison_count = comparison_count.wrapping_add(1);
                        comparison_count;
                        if if rhaystack.cur.wc_valid as libc::c_int != 0
                            && rneedle.cur.wc_valid as libc::c_int != 0
                        {
                            (rhaystack.cur.wc == rneedle.cur.wc) as libc::c_int
                        } else {
                            (rhaystack.cur.bytes == rneedle.cur.bytes
                                && memcmp(
                                    rhaystack.cur.ptr as *const libc::c_void,
                                    rneedle.cur.ptr as *const libc::c_void,
                                    rhaystack.cur.bytes,
                                ) == 0 as libc::c_int) as libc::c_int
                        } == 0
                        {
                            break;
                        }
                        rhaystack
                            .cur
                            .ptr = (rhaystack.cur.ptr)
                            .offset(rhaystack.cur.bytes as isize);
                        rhaystack.next_done = 0 as libc::c_int != 0;
                        rneedle
                            .cur
                            .ptr = (rneedle.cur.ptr).offset(rneedle.cur.bytes as isize);
                        rneedle.next_done = 0 as libc::c_int != 0;
                    }
                }
                iter_haystack
                    .cur
                    .ptr = (iter_haystack.cur.ptr)
                    .offset(iter_haystack.cur.bytes as isize);
                iter_haystack.next_done = 0 as libc::c_int != 0;
            }
        } else {
            return haystack as *mut libc::c_char
        }
    } else if *needle as libc::c_int != '\0' as i32 {
        let mut try_kmp_0: bool = 1 as libc::c_int != 0;
        let mut outer_loop_count_0: size_t = 0 as libc::c_int as size_t;
        let mut comparison_count_0: size_t = 0 as libc::c_int as size_t;
        let mut last_ccount_0: size_t = 0 as libc::c_int as size_t;
        let mut needle_last_ccount: *const libc::c_char = needle;
        let fresh2 = needle;
        needle = needle.offset(1);
        let mut b: libc::c_char = *fresh2;
        loop {
            if *haystack as libc::c_int == '\0' as i32 {
                return 0 as *mut libc::c_char;
            }
            if try_kmp_0 as libc::c_int != 0
                && outer_loop_count_0 >= 10 as libc::c_int as libc::c_ulong
                && comparison_count_0
                    >= (5 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(outer_loop_count_0)
            {
                if !needle_last_ccount.is_null() {
                    needle_last_ccount = needle_last_ccount
                        .offset(
                            strnlen(
                                needle_last_ccount,
                                comparison_count_0.wrapping_sub(last_ccount_0),
                            ) as isize,
                        );
                    if *needle_last_ccount as libc::c_int == '\0' as i32 {
                        needle_last_ccount = 0 as *const libc::c_char;
                    }
                    last_ccount_0 = comparison_count_0;
                }
                if needle_last_ccount.is_null() {
                    let mut result_0: *const libc::c_uchar = 0 as *const libc::c_uchar;
                    let mut success_0: bool = knuth_morris_pratt(
                        haystack as *const libc::c_uchar,
                        needle.offset(-(1 as libc::c_int as isize))
                            as *const libc::c_uchar,
                        strlen(needle.offset(-(1 as libc::c_int as isize))),
                        &mut result_0,
                    );
                    if success_0 {
                        return result_0 as *mut libc::c_char;
                    }
                    try_kmp_0 = 0 as libc::c_int != 0;
                }
            }
            outer_loop_count_0 = outer_loop_count_0.wrapping_add(1);
            outer_loop_count_0;
            comparison_count_0 = comparison_count_0.wrapping_add(1);
            comparison_count_0;
            if *haystack as libc::c_int == b as libc::c_int {
                let mut rhaystack_0: *const libc::c_char = haystack
                    .offset(1 as libc::c_int as isize);
                let mut rneedle_0: *const libc::c_char = needle;
                loop {
                    if *rneedle_0 as libc::c_int == '\0' as i32 {
                        return haystack as *mut libc::c_char;
                    }
                    if *rhaystack_0 as libc::c_int == '\0' as i32 {
                        return 0 as *mut libc::c_char;
                    }
                    comparison_count_0 = comparison_count_0.wrapping_add(1);
                    comparison_count_0;
                    if *rhaystack_0 as libc::c_int != *rneedle_0 as libc::c_int {
                        break;
                    }
                    rhaystack_0 = rhaystack_0.offset(1);
                    rhaystack_0;
                    rneedle_0 = rneedle_0.offset(1);
                    rneedle_0;
                }
            }
            haystack = haystack.offset(1);
            haystack;
        }
    } else {
        return haystack as *mut libc::c_char
    };
}
