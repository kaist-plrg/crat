use ::libc;
extern "C" {
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    static mut as_an_at: bool;
    static mut word_chars: *mut libc::c_char;
    static mut tabsize: ssize_t;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn iswalnum(__wc: wint_t) -> libc::c_int;
    fn iswalpha(__wc: wint_t) -> libc::c_int;
    fn iswpunct(__wc: wint_t) -> libc::c_int;
    fn iswblank(__wc: wint_t) -> libc::c_int;
    fn towlower(__wc: wint_t) -> wint_t;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
static mut use_utf8: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn utf8_init() {
    use_utf8 = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn using_utf8() -> bool {
    return use_utf8;
}
pub unsafe extern "C" fn is_alpha_char(mut c: *const libc::c_char) -> bool {
    let mut wc: wchar_t = 0;
    if mbtowide(&mut wc, c) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return iswalpha(wc as wint_t) != 0;
}
pub unsafe extern "C" fn is_alnum_char(mut c: *const libc::c_char) -> bool {
    let mut wc: wchar_t = 0;
    if mbtowide(&mut wc, c) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return iswalnum(wc as wint_t) != 0;
}
pub unsafe extern "C" fn is_blank_char(mut c: *const libc::c_char) -> bool {
    let mut wc: wchar_t = 0;
    if *c as libc::c_schar as libc::c_int >= 0 as libc::c_int {
        return *c as libc::c_int == ' ' as i32 || *c as libc::c_int == '\t' as i32;
    }
    if mbtowide(&mut wc, c) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return iswblank(wc as wint_t) != 0;
}
pub unsafe extern "C" fn is_cntrl_char(mut c: *const libc::c_char) -> bool {
    if use_utf8 {
        return *c.offset(0 as libc::c_int as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0 as libc::c_int
            || *c.offset(0 as libc::c_int as isize) as libc::c_int == 0x7f as libc::c_int
            || *c.offset(0 as libc::c_int as isize) as libc::c_schar as libc::c_int
                == -(62 as libc::c_int)
                && (*c.offset(1 as libc::c_int as isize) as libc::c_schar as libc::c_int)
                    < -(96 as libc::c_int)
    } else {
        return *c as libc::c_int & 0x60 as libc::c_int == 0 as libc::c_int
            || *c as libc::c_int == 0x7f as libc::c_int
    };
}
pub unsafe extern "C" fn is_punct_char(mut c: *const libc::c_char) -> bool {
    let mut wc: wchar_t = 0;
    if mbtowide(&mut wc, c) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return iswpunct(wc as wint_t) != 0;
}
pub unsafe extern "C" fn is_word_char(
    mut c: *const libc::c_char,
    mut allow_punct: bool,
) -> bool {
    if *c as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    if is_alnum_char(c) {
        return 1 as libc::c_int != 0;
    }
    if allow_punct as libc::c_int != 0 && is_punct_char(c) as libc::c_int != 0 {
        return 1 as libc::c_int != 0;
    }
    if !word_chars.is_null() && *word_chars as libc::c_int != '\0' as i32 {
        let mut symbol: [libc::c_char; 5] = [0; 5];
        let mut symlen: libc::c_int = collect_char(c, symbol.as_mut_ptr());
        symbol[symlen as usize] = '\0' as i32 as libc::c_char;
        return !(strstr(word_chars, symbol.as_mut_ptr())).is_null();
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn control_rep(c: libc::c_schar) -> libc::c_char {
    if c as libc::c_int == 0x7f as libc::c_int {
        return '?' as i32 as libc::c_char
    } else if c as libc::c_int == -(97 as libc::c_int) {
        return '=' as i32 as libc::c_char
    } else if (c as libc::c_int) < 0 as libc::c_int {
        return (c as libc::c_int + 224 as libc::c_int) as libc::c_char
    } else {
        return (c as libc::c_int + 64 as libc::c_int) as libc::c_char
    };
}
pub unsafe extern "C" fn control_mbrep(
    mut c: *const libc::c_char,
    mut isdata: bool,
) -> libc::c_char {
    if *c as libc::c_int == '\n' as i32
        && (isdata as libc::c_int != 0 || as_an_at as libc::c_int != 0)
    {
        return '@' as i32 as libc::c_char;
    }
    if use_utf8 {
        if (*c.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
            < 128 as libc::c_int
        {
            return control_rep(*c.offset(0 as libc::c_int as isize) as libc::c_schar)
        } else {
            return control_rep(*c.offset(1 as libc::c_int as isize) as libc::c_schar)
        }
    } else {
        return control_rep(*c as libc::c_schar)
    };
}
pub unsafe extern "C" fn mbtowide(
    mut wc: *mut wchar_t,
    mut c: *const libc::c_char,
) -> libc::c_int {
    if (*c as libc::c_schar as libc::c_int) < 0 as libc::c_int
        && use_utf8 as libc::c_int != 0
    {
        let mut v1: libc::c_uchar = *c.offset(0 as libc::c_int as isize)
            as libc::c_uchar;
        let mut v2: libc::c_uchar = (*c.offset(1 as libc::c_int as isize)
            as libc::c_uchar as libc::c_int ^ 0x80 as libc::c_int) as libc::c_uchar;
        if v2 as libc::c_int > 0x3f as libc::c_int
            || (v1 as libc::c_int) < 0xc2 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (v1 as libc::c_int) < 0xe0 as libc::c_int {
            *wc = (((v1 as libc::c_int & 0x1f as libc::c_int) as libc::c_uint)
                << 6 as libc::c_int | v2 as libc::c_uint) as wchar_t;
            return 2 as libc::c_int;
        }
        let mut v3: libc::c_uchar = (*c.offset(2 as libc::c_int as isize)
            as libc::c_uchar as libc::c_int ^ 0x80 as libc::c_int) as libc::c_uchar;
        if v3 as libc::c_int > 0x3f as libc::c_int {
            return -(1 as libc::c_int);
        }
        if (v1 as libc::c_int) < 0xf0 as libc::c_int {
            if (v1 as libc::c_int > 0xe0 as libc::c_int
                || v2 as libc::c_int >= 0x20 as libc::c_int)
                && (v1 as libc::c_int != 0xed as libc::c_int
                    || (v2 as libc::c_int) < 0x20 as libc::c_int)
            {
                *wc = (((v1 as libc::c_int & 0xf as libc::c_int) as libc::c_uint)
                    << 12 as libc::c_int | (v2 as libc::c_uint) << 6 as libc::c_int
                    | v3 as libc::c_uint) as wchar_t;
                return 3 as libc::c_int;
            } else {
                return -(1 as libc::c_int)
            }
        }
        let mut v4: libc::c_uchar = (*c.offset(3 as libc::c_int as isize)
            as libc::c_uchar as libc::c_int ^ 0x80 as libc::c_int) as libc::c_uchar;
        if v4 as libc::c_int > 0x3f as libc::c_int
            || v1 as libc::c_int > 0xf4 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (v1 as libc::c_int > 0xf0 as libc::c_int
            || v2 as libc::c_int >= 0x10 as libc::c_int)
            && (v1 as libc::c_int != 0xf4 as libc::c_int
                || (v2 as libc::c_int) < 0x10 as libc::c_int)
        {
            *wc = (((v1 as libc::c_int & 0x7 as libc::c_int) as libc::c_uint)
                << 18 as libc::c_int | (v2 as libc::c_uint) << 12 as libc::c_int
                | (v3 as libc::c_uint) << 6 as libc::c_int | v4 as libc::c_uint)
                as wchar_t;
            return 4 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    }
    *wc = *c as libc::c_uint as wchar_t;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn is_doublewidth(mut ch: *const libc::c_char) -> bool {
    let mut wc: wchar_t = 0;
    if (*ch as libc::c_uchar as libc::c_int) < 0xe1 as libc::c_int || !use_utf8 {
        return 0 as libc::c_int != 0;
    }
    if mbtowide(&mut wc, ch) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return wcwidth(wc) == 2 as libc::c_int;
}
pub unsafe extern "C" fn is_zerowidth(mut ch: *const libc::c_char) -> bool {
    let mut wc: wchar_t = 0;
    if (*ch as libc::c_uchar as libc::c_int) < 0xcc as libc::c_int || !use_utf8 {
        return 0 as libc::c_int != 0;
    }
    if mbtowide(&mut wc, ch) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return wcwidth(wc) == 0 as libc::c_int;
}
pub unsafe extern "C" fn char_length(mut pointer: *const libc::c_char) -> libc::c_int {
    if *pointer as libc::c_uchar as libc::c_int > 0xc1 as libc::c_int
        && use_utf8 as libc::c_int != 0
    {
        let mut c1: libc::c_uchar = *pointer.offset(0 as libc::c_int as isize)
            as libc::c_uchar;
        let mut c2: libc::c_uchar = *pointer.offset(1 as libc::c_int as isize)
            as libc::c_uchar;
        if c2 as libc::c_int ^ 0x80 as libc::c_int > 0x3f as libc::c_int {
            return 1 as libc::c_int;
        }
        if (c1 as libc::c_int) < 0xe0 as libc::c_int {
            return 2 as libc::c_int;
        }
        if *pointer.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            ^ 0x80 as libc::c_int > 0x3f as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if (c1 as libc::c_int) < 0xf0 as libc::c_int {
            if (c1 as libc::c_int > 0xe0 as libc::c_int
                || c2 as libc::c_int >= 0xa0 as libc::c_int)
                && (c1 as libc::c_int != 0xed as libc::c_int
                    || (c2 as libc::c_int) < 0xa0 as libc::c_int)
            {
                return 3 as libc::c_int
            } else {
                return 1 as libc::c_int
            }
        }
        if *pointer.offset(3 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            ^ 0x80 as libc::c_int > 0x3f as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if c1 as libc::c_int > 0xf4 as libc::c_int {
            return 1 as libc::c_int;
        }
        if (c1 as libc::c_int > 0xf0 as libc::c_int
            || c2 as libc::c_int >= 0x90 as libc::c_int)
            && (c1 as libc::c_int != 0xf4 as libc::c_int
                || (c2 as libc::c_int) < 0x90 as libc::c_int)
        {
            return 4 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn mbstrlen(mut pointer: *const libc::c_char) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    while *pointer as libc::c_int != '\0' as i32 {
        pointer = pointer.offset(char_length(pointer) as isize);
        count = count.wrapping_add(1);
        count;
    }
    return count;
}
pub unsafe extern "C" fn collect_char(
    mut string: *const libc::c_char,
    mut thechar: *mut libc::c_char,
) -> libc::c_int {
    let mut charlen: libc::c_int = char_length(string);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < charlen {
        *thechar.offset(i as isize) = *string.offset(i as isize);
        i += 1;
        i;
    }
    return charlen;
}
pub unsafe extern "C" fn advance_over(
    mut string: *const libc::c_char,
    mut column: *mut size_t,
) -> libc::c_int {
    if (*string as libc::c_schar as libc::c_int) < 0 as libc::c_int
        && use_utf8 as libc::c_int != 0
    {
        if *string.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xc2 as libc::c_int
            && (*string.offset(1 as libc::c_int as isize) as libc::c_schar
                as libc::c_int) < -(96 as libc::c_int)
        {
            *column = (*column as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return 2 as libc::c_int;
        } else {
            let mut wc: wchar_t = 0;
            let mut charlen: libc::c_int = mbtowide(&mut wc, string);
            if charlen < 0 as libc::c_int {
                *column = (*column as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
                return 1 as libc::c_int;
            }
            let mut width: libc::c_int = wcwidth(wc);
            *column = (*column as libc::c_ulong)
                .wrapping_add(
                    (if width < 0 as libc::c_int { 1 as libc::c_int } else { width })
                        as libc::c_ulong,
                ) as size_t as size_t;
            return charlen;
        }
    }
    if (*string as libc::c_uchar as libc::c_int) < 0x20 as libc::c_int {
        if *string as libc::c_int == '\t' as i32 {
            *column = (*column as libc::c_ulong)
                .wrapping_add(
                    (tabsize as libc::c_ulong)
                        .wrapping_sub((*column).wrapping_rem(tabsize as libc::c_ulong)),
                ) as size_t as size_t;
        } else {
            *column = (*column as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
    } else if (0x7e as libc::c_int) < *string as libc::c_uchar as libc::c_int
        && (*string as libc::c_uchar as libc::c_int) < 0xa0 as libc::c_int
    {
        *column = (*column as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    } else {
        *column = (*column as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn step_left(
    mut buf: *const libc::c_char,
    mut pos: size_t,
) -> size_t {
    if use_utf8 {
        let mut before: size_t = 0;
        let mut charlen: size_t = 0 as libc::c_int as size_t;
        if pos < 4 as libc::c_int as libc::c_ulong {
            before = 0 as libc::c_int as size_t;
        } else {
            let mut ptr: *const libc::c_char = buf.offset(pos as isize);
            ptr = ptr.offset(-1);
            if *ptr as libc::c_schar as libc::c_int > -(65 as libc::c_int) {
                before = pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            } else {
                ptr = ptr.offset(-1);
                if *ptr as libc::c_schar as libc::c_int > -(65 as libc::c_int) {
                    before = pos.wrapping_sub(2 as libc::c_int as libc::c_ulong);
                } else {
                    ptr = ptr.offset(-1);
                    if *ptr as libc::c_schar as libc::c_int > -(65 as libc::c_int) {
                        before = pos.wrapping_sub(3 as libc::c_int as libc::c_ulong);
                    } else {
                        ptr = ptr.offset(-1);
                        if *ptr as libc::c_schar as libc::c_int > -(65 as libc::c_int) {
                            before = pos.wrapping_sub(4 as libc::c_int as libc::c_ulong);
                        } else {
                            before = pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        }
                    }
                }
            }
        }
        while before < pos {
            charlen = char_length(buf.offset(before as isize)) as size_t;
            before = (before as libc::c_ulong).wrapping_add(charlen) as size_t as size_t;
        }
        return before.wrapping_sub(charlen);
    } else {
        return if pos == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int as libc::c_ulong
        } else {
            pos.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        }
    };
}
pub unsafe extern "C" fn step_right(
    mut buf: *const libc::c_char,
    mut pos: size_t,
) -> size_t {
    return pos.wrapping_add(char_length(buf.offset(pos as isize)) as libc::c_ulong);
}
pub unsafe extern "C" fn mbstrcasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    return mbstrncasecmp(s1, s2, !(0 as libc::c_int as size_t) >> 1 as libc::c_int);
}
pub unsafe extern "C" fn mbstrncasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    if use_utf8 {
        let mut wc1: wchar_t = 0;
        let mut wc2: wchar_t = 0;
        while *s1 as libc::c_int != '\0' as i32 && *s2 as libc::c_int != '\0' as i32
            && n > 0 as libc::c_int as libc::c_ulong
        {
            if *s1 as libc::c_schar as libc::c_int >= 0 as libc::c_int
                && *s2 as libc::c_schar as libc::c_int >= 0 as libc::c_int
            {
                if 'A' as i32 <= *s1 as libc::c_int & 0x5f as libc::c_int
                    && *s1 as libc::c_int & 0x5f as libc::c_int <= 'Z' as i32
                {
                    if 'A' as i32 <= *s2 as libc::c_int & 0x5f as libc::c_int
                        && *s2 as libc::c_int & 0x5f as libc::c_int <= 'Z' as i32
                    {
                        if *s1 as libc::c_int & 0x5f as libc::c_int
                            != *s2 as libc::c_int & 0x5f as libc::c_int
                        {
                            return (*s1 as libc::c_int & 0x5f as libc::c_int)
                                - (*s2 as libc::c_int & 0x5f as libc::c_int);
                        }
                    } else {
                        return (*s1 as libc::c_int | 0x20 as libc::c_int)
                            - *s2 as libc::c_int
                    }
                } else if 'A' as i32 <= *s2 as libc::c_int & 0x5f as libc::c_int
                    && *s2 as libc::c_int & 0x5f as libc::c_int <= 'Z' as i32
                {
                    return *s1 as libc::c_int
                        - (*s2 as libc::c_int | 0x20 as libc::c_int)
                } else if *s1 as libc::c_int != *s2 as libc::c_int {
                    return *s1 as libc::c_int - *s2 as libc::c_int
                }
                s1 = s1.offset(1);
                s1;
                s2 = s2.offset(1);
                s2;
                n = n.wrapping_sub(1);
                n;
            } else {
                let mut bad1: bool = mbtowide(&mut wc1, s1) < 0 as libc::c_int;
                let mut bad2: bool = mbtowide(&mut wc2, s2) < 0 as libc::c_int;
                if bad1 as libc::c_int != 0 || bad2 as libc::c_int != 0 {
                    if *s1 as libc::c_int != *s2 as libc::c_int {
                        return *s1 as libc::c_uchar as libc::c_int
                            - *s2 as libc::c_uchar as libc::c_int;
                    }
                    if bad1 as libc::c_int != bad2 as libc::c_int {
                        return if bad1 as libc::c_int != 0 {
                            1 as libc::c_int
                        } else {
                            -(1 as libc::c_int)
                        };
                    }
                } else {
                    let mut difference: libc::c_int = (towlower(wc1 as wint_t))
                        .wrapping_sub(towlower(wc2 as wint_t)) as libc::c_int;
                    if difference != 0 as libc::c_int {
                        return difference;
                    }
                }
                s1 = s1.offset(char_length(s1) as isize);
                s2 = s2.offset(char_length(s2) as isize);
                n = n.wrapping_sub(1);
                n;
            }
        }
        return if n > 0 as libc::c_int as libc::c_ulong {
            *s1 as libc::c_uchar as libc::c_int - *s2 as libc::c_uchar as libc::c_int
        } else {
            0 as libc::c_int
        };
    } else {
        return strncasecmp(s1, s2, n)
    };
}
pub unsafe extern "C" fn mbstrcasestr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    if use_utf8 {
        let mut needle_len: size_t = mbstrlen(needle);
        while *haystack as libc::c_int != '\0' as i32 {
            if mbstrncasecmp(haystack, needle, needle_len) == 0 as libc::c_int {
                return haystack as *mut libc::c_char;
            }
            haystack = haystack.offset(char_length(haystack) as isize);
        }
        return 0 as *mut libc::c_char;
    } else {
        return strcasestr(haystack, needle)
    };
}
pub unsafe extern "C" fn revstrstr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
    mut pointer: *const libc::c_char,
) -> *mut libc::c_char {
    let mut needle_len: size_t = strlen(needle);
    let mut tail_len: size_t = strlen(pointer);
    if tail_len < needle_len {
        pointer = pointer.offset(-(needle_len.wrapping_sub(tail_len) as isize));
    }
    while pointer >= haystack {
        if strncmp(pointer, needle, needle_len) == 0 as libc::c_int {
            return pointer as *mut libc::c_char;
        }
        pointer = pointer.offset(-1);
        pointer;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn revstrcasestr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
    mut pointer: *const libc::c_char,
) -> *mut libc::c_char {
    let mut needle_len: size_t = strlen(needle);
    let mut tail_len: size_t = strlen(pointer);
    if tail_len < needle_len {
        pointer = pointer.offset(-(needle_len.wrapping_sub(tail_len) as isize));
    }
    while pointer >= haystack {
        if strncasecmp(pointer, needle, needle_len) == 0 as libc::c_int {
            return pointer as *mut libc::c_char;
        }
        pointer = pointer.offset(-1);
        pointer;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn mbrevstrcasestr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
    mut pointer: *const libc::c_char,
) -> *mut libc::c_char {
    if use_utf8 {
        let mut needle_len: size_t = mbstrlen(needle);
        let mut tail_len: size_t = mbstrlen(pointer);
        if tail_len < needle_len {
            pointer = pointer.offset(-(needle_len.wrapping_sub(tail_len) as isize));
        }
        if pointer < haystack {
            return 0 as *mut libc::c_char;
        }
        loop {
            if mbstrncasecmp(pointer, needle, needle_len) == 0 as libc::c_int {
                return pointer as *mut libc::c_char;
            }
            if pointer == haystack {
                return 0 as *mut libc::c_char;
            }
            pointer = haystack
                .offset(
                    step_left(
                        haystack,
                        pointer.offset_from(haystack) as libc::c_long as size_t,
                    ) as isize,
                );
        }
    } else {
        return revstrcasestr(haystack, needle, pointer)
    };
}
pub unsafe extern "C" fn mbstrchr(
    mut string: *const libc::c_char,
    mut chr: *const libc::c_char,
) -> *mut libc::c_char {
    if use_utf8 {
        let mut bad_s: bool = 0 as libc::c_int != 0;
        let mut bad_c: bool = 0 as libc::c_int != 0;
        let mut ws: wchar_t = 0;
        let mut wc: wchar_t = 0;
        if mbtowide(&mut wc, chr) < 0 as libc::c_int {
            wc = *chr as libc::c_uchar as wchar_t;
            bad_c = 1 as libc::c_int != 0;
        }
        while *string as libc::c_int != '\0' as i32 {
            let mut symlen: libc::c_int = mbtowide(&mut ws, string);
            if symlen < 0 as libc::c_int {
                ws = *string as libc::c_uchar as wchar_t;
                bad_s = 1 as libc::c_int != 0;
            }
            if ws == wc && bad_s as libc::c_int == bad_c as libc::c_int {
                break;
            }
            string = string.offset(symlen as isize);
        }
        if *string as libc::c_int == '\0' as i32 {
            return 0 as *mut libc::c_char;
        }
        return string as *mut libc::c_char;
    } else {
        return strchr(string, *chr as libc::c_int)
    };
}
pub unsafe extern "C" fn mbstrpbrk(
    mut string: *const libc::c_char,
    mut accept: *const libc::c_char,
) -> *mut libc::c_char {
    while *string as libc::c_int != '\0' as i32 {
        if !(mbstrchr(accept, string)).is_null() {
            return string as *mut libc::c_char;
        }
        string = string.offset(char_length(string) as isize);
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn mbrevstrpbrk(
    mut head: *const libc::c_char,
    mut accept: *const libc::c_char,
    mut pointer: *const libc::c_char,
) -> *mut libc::c_char {
    if *pointer as libc::c_int == '\0' as i32 {
        if pointer == head {
            return 0 as *mut libc::c_char;
        }
        pointer = head
            .offset(
                step_left(head, pointer.offset_from(head) as libc::c_long as size_t)
                    as isize,
            );
    }
    loop {
        if !(mbstrchr(accept, pointer)).is_null() {
            return pointer as *mut libc::c_char;
        }
        if pointer == head {
            return 0 as *mut libc::c_char;
        }
        pointer = head
            .offset(
                step_left(head, pointer.offset_from(head) as libc::c_long as size_t)
                    as isize,
            );
    };
}
pub unsafe extern "C" fn has_blank_char(mut string: *const libc::c_char) -> bool {
    while *string as libc::c_int != '\0' as i32 && !is_blank_char(string) {
        string = string.offset(char_length(string) as isize);
    }
    return *string != 0;
}
pub unsafe extern "C" fn white_string(mut string: *const libc::c_char) -> bool {
    while *string as libc::c_int != '\0' as i32
        && (is_blank_char(string) as libc::c_int != 0
            || *string as libc::c_int == '\r' as i32)
    {
        string = string.offset(char_length(string) as isize);
    }
    return *string == 0;
}
