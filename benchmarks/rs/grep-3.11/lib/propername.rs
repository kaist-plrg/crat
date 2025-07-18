use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn mbsstr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn trim2(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    static is_basic_table: [libc::c_uint; 0];
    fn iswalnum(__wc: wint_t) -> libc::c_int;
    fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t;
    fn locale_charset() -> *const libc::c_char;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn xstr_iconv(
        src: *const libc::c_char,
        from_codeset: *const libc::c_char,
        to_codeset: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub const _ISalnum: C2RustUnnamed_0 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type wchar_t = libc::c_int;
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
pub type mbchar_t = mbchar;
pub type C2RustUnnamed_0 = libc::c_uint;
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
unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 11675860686847285950;
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
        'c_786: {
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
        current_block = 11675860686847285950;
    }
    match current_block {
        11675860686847285950 => {
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
                    'c_610: {
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
                    'c_563: {
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
unsafe extern "C" fn mbsstr_trimmed_wordbounded(
    mut string: *const libc::c_char,
    mut sub: *const libc::c_char,
) -> bool {
    let mut tsub: *mut libc::c_char = trim2(sub, 2 as libc::c_int);
    let mut found: bool = 0 as libc::c_int != 0;
    while *string as libc::c_int != '\0' as i32 {
        let mut tsub_in_string: *const libc::c_char = mbsstr(string, tsub);
        if tsub_in_string.is_null() {
            break;
        }
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
            let mut string_iter: mbui_iterator_t = mbui_iterator_t {
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
            let mut word_boundary_before: bool = false;
            let mut word_boundary_after: bool = false;
            string_iter.cur.ptr = string;
            string_iter.in_shift = 0 as libc::c_int != 0;
            memset(
                &mut string_iter.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            string_iter.next_done = 0 as libc::c_int != 0;
            word_boundary_before = 1 as libc::c_int != 0;
            if string_iter.cur.ptr < tsub_in_string {
                let mut last_char_before_tsub: mbchar_t = mbchar {
                    ptr: 0 as *const libc::c_char,
                    bytes: 0,
                    wc_valid: false,
                    wc: 0,
                    buf: [0; 24],
                };
                loop {
                    mbuiter_multi_next(&mut string_iter);
                    if !(string_iter.cur.wc_valid as libc::c_int != 0
                        && string_iter.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                    {
                        abort();
                    }
                    last_char_before_tsub = string_iter.cur;
                    string_iter
                        .cur
                        .ptr = (string_iter.cur.ptr)
                        .offset(string_iter.cur.bytes as isize);
                    string_iter.next_done = 0 as libc::c_int != 0;
                    if !(string_iter.cur.ptr < tsub_in_string) {
                        break;
                    }
                }
                if last_char_before_tsub.wc_valid as libc::c_int != 0
                    && iswalnum(last_char_before_tsub.wc as wint_t) != 0
                {
                    word_boundary_before = 0 as libc::c_int != 0;
                }
            }
            string_iter.cur.ptr = tsub_in_string;
            string_iter.in_shift = 0 as libc::c_int != 0;
            memset(
                &mut string_iter.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            string_iter.next_done = 0 as libc::c_int != 0;
            let mut tsub_iter: mbui_iterator_t = mbui_iterator_t {
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
            tsub_iter.cur.ptr = tsub;
            tsub_iter.in_shift = 0 as libc::c_int != 0;
            memset(
                &mut tsub_iter.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            tsub_iter.next_done = 0 as libc::c_int != 0;
            loop {
                mbuiter_multi_next(&mut tsub_iter);
                if !(!(tsub_iter.cur.wc_valid as libc::c_int != 0
                    && tsub_iter.cur.wc == 0 as libc::c_int) as libc::c_int != 0)
                {
                    break;
                }
                mbuiter_multi_next(&mut string_iter);
                if !(string_iter.cur.wc_valid as libc::c_int != 0
                    && string_iter.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                {
                    abort();
                }
                string_iter
                    .cur
                    .ptr = (string_iter.cur.ptr).offset(string_iter.cur.bytes as isize);
                string_iter.next_done = 0 as libc::c_int != 0;
                tsub_iter
                    .cur
                    .ptr = (tsub_iter.cur.ptr).offset(tsub_iter.cur.bytes as isize);
                tsub_iter.next_done = 0 as libc::c_int != 0;
            }
            word_boundary_after = 1 as libc::c_int != 0;
            mbuiter_multi_next(&mut string_iter);
            if !(string_iter.cur.wc_valid as libc::c_int != 0
                && string_iter.cur.wc == 0 as libc::c_int) as libc::c_int != 0
            {
                let mut first_char_after_tsub: mbchar_t = string_iter.cur;
                if first_char_after_tsub.wc_valid as libc::c_int != 0
                    && iswalnum(first_char_after_tsub.wc as wint_t) != 0
                {
                    word_boundary_after = 0 as libc::c_int != 0;
                }
            }
            if word_boundary_before as libc::c_int != 0
                && word_boundary_after as libc::c_int != 0
            {
                found = 1 as libc::c_int != 0;
                break;
            } else {
                string_iter.cur.ptr = tsub_in_string;
                string_iter.in_shift = 0 as libc::c_int != 0;
                memset(
                    &mut string_iter.state as *mut mbstate_t as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                );
                string_iter.next_done = 0 as libc::c_int != 0;
                mbuiter_multi_next(&mut string_iter);
                if !(string_iter.cur.wc_valid as libc::c_int != 0
                    && string_iter.cur.wc == 0 as libc::c_int) as libc::c_int == 0
                {
                    break;
                }
                string = tsub_in_string.offset(string_iter.cur.bytes as isize);
            }
        } else {
            let mut word_boundary_before_0: bool = false;
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            let mut word_boundary_after_0: bool = false;
            word_boundary_before_0 = 1 as libc::c_int != 0;
            if string < tsub_in_string {
                if *(*__ctype_b_loc())
                    .offset(
                        *tsub_in_string.offset(-(1 as libc::c_int) as isize)
                            as libc::c_uchar as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    word_boundary_before_0 = 0 as libc::c_int != 0;
                }
            }
            p = tsub_in_string.offset(strlen(tsub) as isize);
            word_boundary_after_0 = 1 as libc::c_int != 0;
            if *p as libc::c_int != '\0' as i32 {
                if *(*__ctype_b_loc())
                    .offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    word_boundary_after_0 = 0 as libc::c_int != 0;
                }
            }
            if word_boundary_before_0 as libc::c_int != 0
                && word_boundary_after_0 as libc::c_int != 0
            {
                found = 1 as libc::c_int != 0;
                break;
            } else {
                if *tsub_in_string as libc::c_int == '\0' as i32 {
                    break;
                }
                string = tsub_in_string.offset(1 as libc::c_int as isize);
            }
        }
    }
    rpl_free(tsub as *mut libc::c_void);
    return found;
}
pub unsafe extern "C" fn proper_name(
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut translation: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        name,
        5 as libc::c_int,
    );
    if translation != name {
        if mbsstr_trimmed_wordbounded(translation, name) {
            return translation
        } else {
            let mut result: *mut libc::c_char = (if ::std::mem::size_of::<libc::c_char>()
                as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
            {
                xmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
            } else {
                xnmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                )
            }) as *mut libc::c_char;
            sprintf(
                result,
                b"%s (%s)\0" as *const u8 as *const libc::c_char,
                translation,
                name,
            );
            return result;
        }
    } else {
        return name
    };
}
pub unsafe extern "C" fn proper_name_utf8(
    mut name_ascii: *const libc::c_char,
    mut name_utf8: *const libc::c_char,
) -> *const libc::c_char {
    let mut translation: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        name_ascii,
        5 as libc::c_int,
    );
    let mut locale_code: *const libc::c_char = locale_charset();
    let mut alloc_name_converted: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alloc_name_converted_translit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name_converted: *const libc::c_char = 0 as *const libc::c_char;
    let mut name_converted_translit: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if c_strcasecmp(locale_code, b"UTF-8\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        alloc_name_converted = xstr_iconv(
            name_utf8,
            b"UTF-8\0" as *const u8 as *const libc::c_char,
            locale_code,
        );
        name_converted = alloc_name_converted;
        let mut converted_translit: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = strlen(locale_code);
        let mut locale_code_translit: *mut libc::c_char = (if ::std::mem::size_of::<
            libc::c_char,
        >() as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
        {
            xmalloc(
                len
                    .wrapping_add(10 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
        } else {
            xnmalloc(
                len
                    .wrapping_add(10 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            )
        }) as *mut libc::c_char;
        memcpy(
            locale_code_translit as *mut libc::c_void,
            locale_code as *const libc::c_void,
            len,
        );
        memcpy(
            locale_code_translit.offset(len as isize) as *mut libc::c_void,
            b"//TRANSLIT\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (10 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        converted_translit = xstr_iconv(
            name_utf8,
            b"UTF-8\0" as *const u8 as *const libc::c_char,
            locale_code_translit,
        );
        rpl_free(locale_code_translit as *mut libc::c_void);
        if !converted_translit.is_null() {
            if !(strchr(converted_translit, '?' as i32)).is_null() {
                rpl_free(converted_translit as *mut libc::c_void);
            } else {
                alloc_name_converted_translit = converted_translit;
                name_converted_translit = alloc_name_converted_translit;
            }
        }
    } else {
        name_converted = name_utf8;
        name_converted_translit = name_utf8;
    }
    name = if !name_converted.is_null() {
        name_converted
    } else if !name_converted_translit.is_null() {
        name_converted_translit
    } else {
        name_ascii
    };
    if strcmp(translation, name_ascii) != 0 as libc::c_int {
        if mbsstr_trimmed_wordbounded(translation, name_ascii) as libc::c_int != 0
            || !name_converted.is_null()
                && mbsstr_trimmed_wordbounded(translation, name_converted) as libc::c_int
                    != 0
            || !name_converted_translit.is_null()
                && mbsstr_trimmed_wordbounded(translation, name_converted_translit)
                    as libc::c_int != 0
        {
            if !alloc_name_converted.is_null() {
                rpl_free(alloc_name_converted as *mut libc::c_void);
            }
            if !alloc_name_converted_translit.is_null() {
                rpl_free(alloc_name_converted_translit as *mut libc::c_void);
            }
            return translation;
        } else {
            let mut result: *mut libc::c_char = (if ::std::mem::size_of::<libc::c_char>()
                as libc::c_ulong == 1 as libc::c_int as libc::c_ulong
            {
                xmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
            } else {
                xnmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                )
            }) as *mut libc::c_char;
            sprintf(
                result,
                b"%s (%s)\0" as *const u8 as *const libc::c_char,
                translation,
                name,
            );
            if !alloc_name_converted.is_null() {
                rpl_free(alloc_name_converted as *mut libc::c_void);
            }
            if !alloc_name_converted_translit.is_null() {
                rpl_free(alloc_name_converted_translit as *mut libc::c_void);
            }
            return result;
        }
    } else {
        if !alloc_name_converted.is_null()
            && alloc_name_converted != name as *mut libc::c_char
        {
            rpl_free(alloc_name_converted as *mut libc::c_void);
        }
        if !alloc_name_converted_translit.is_null()
            && alloc_name_converted_translit != name as *mut libc::c_char
        {
            rpl_free(alloc_name_converted_translit as *mut libc::c_void);
        }
        return name;
    };
}
