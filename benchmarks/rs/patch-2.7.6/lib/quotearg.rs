use ::libc;
extern "C" {
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn locale_charset() -> *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswprint(__wc: wint_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type quoting_flags = libc::c_uint;
pub const QA_SPLIT_TRIGRAPHS: quoting_flags = 4;
pub const QA_ELIDE_OUTER_QUOTES: quoting_flags = 2;
pub const QA_ELIDE_NULL_BYTES: quoting_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quoting_options {
    pub style: quoting_style,
    pub flags: libc::c_int,
    pub quote_these_too: [libc::c_uint; 8],
    pub left_quote: *const libc::c_char,
    pub right_quote: *const libc::c_char,
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
pub const _ISprint: C2RustUnnamed_0 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slotvec {
    pub size: size_t,
    pub val: *mut libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[inline]
unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    })
        .wrapping_div(s) < n
    {
        xalloc_die();
    }
    return xmalloc(n.wrapping_mul(s));
}
#[inline]
unsafe extern "C" fn xcharalloc(mut n: size_t) -> *mut libc::c_char {
    return (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xmalloc(n)
    } else {
        xnmalloc(n, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
    }) as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn strcaseeq9(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    return (c_strcasecmp(
        s1.offset(9 as libc::c_int as isize),
        s2.offset(9 as libc::c_int as isize),
    ) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn strcaseeq8(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s28 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(8 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s28 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(8 as libc::c_int as isize) as libc::c_int == s28 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s28 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq9(s1, s2)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq7(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s27 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(7 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s27 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(7 as libc::c_int as isize) as libc::c_int == s27 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s27 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq8(s1, s2, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq6(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s26: libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s26 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(6 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s26 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(6 as libc::c_int as isize) as libc::c_int == s26 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s26 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq7(s1, s2, s27, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq5(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s25: libc::c_char,
    mut s26: libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s25 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(5 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s25 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(5 as libc::c_int as isize) as libc::c_int == s25 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s25 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq6(s1, s2, s26, s27, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq4(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s24: libc::c_char,
    mut s25: libc::c_char,
    mut s26: libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s24 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(4 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s24 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(4 as libc::c_int as isize) as libc::c_int == s24 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s24 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq5(s1, s2, s25, s26, s27, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq3(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s23: libc::c_char,
    mut s24: libc::c_char,
    mut s25: libc::c_char,
    mut s26: libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s23 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(3 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s23 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(3 as libc::c_int as isize) as libc::c_int == s23 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s23 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq4(s1, s2, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq2(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s22: libc::c_char,
    mut s23: libc::c_char,
    mut s24: libc::c_char,
    mut s25: libc::c_char,
    mut s26: libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s22 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(2 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s22 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(2 as libc::c_int as isize) as libc::c_int == s22 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s22 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq3(s1, s2, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq1(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s21: libc::c_char,
    mut s22: libc::c_char,
    mut s23: libc::c_char,
    mut s24: libc::c_char,
    mut s25: libc::c_char,
    mut s26: libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s21 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(1 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s21 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(1 as libc::c_int as isize) as libc::c_int == s21 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s21 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq2(s1, s2, s22, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn strcaseeq0(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut s20: libc::c_char,
    mut s21: libc::c_char,
    mut s22: libc::c_char,
    mut s23: libc::c_char,
    mut s24: libc::c_char,
    mut s25: libc::c_char,
    mut s26: libc::c_char,
    mut s27: libc::c_char,
    mut s28: libc::c_char,
) -> libc::c_int {
    if if c_isupper(s20 as libc::c_int) as libc::c_int != 0 {
        (*s1.offset(0 as libc::c_int as isize) as libc::c_int & !(0x20 as libc::c_int)
            == s20 as libc::c_int) as libc::c_int
    } else {
        (*s1.offset(0 as libc::c_int as isize) as libc::c_int == s20 as libc::c_int)
            as libc::c_int
    } != 0
    {
        if s20 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            return strcaseeq1(s1, s2, s21, s22, s23, s24, s25, s26, s27, s28)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn c_isupper(mut c: libc::c_int) -> bool {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
pub static mut quoting_style_args: [*const libc::c_char; 11] = [
    b"literal\0" as *const u8 as *const libc::c_char,
    b"shell\0" as *const u8 as *const libc::c_char,
    b"shell-always\0" as *const u8 as *const libc::c_char,
    b"shell-escape\0" as *const u8 as *const libc::c_char,
    b"shell-escape-always\0" as *const u8 as *const libc::c_char,
    b"c\0" as *const u8 as *const libc::c_char,
    b"c-maybe\0" as *const u8 as *const libc::c_char,
    b"escape\0" as *const u8 as *const libc::c_char,
    b"locale\0" as *const u8 as *const libc::c_char,
    b"clocale\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub static mut quoting_style_vals: [quoting_style; 10] = [
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
];
static mut default_quoting_options: quoting_options = quoting_options {
    style: literal_quoting_style,
    flags: 0,
    quote_these_too: [0; 8],
    left_quote: 0 as *const libc::c_char,
    right_quote: 0 as *const libc::c_char,
};
pub unsafe extern "C" fn clone_quoting_options(
    mut o: *mut quoting_options,
) -> *mut quoting_options {
    let mut e: libc::c_int = *__errno_location();
    let mut p: *mut quoting_options = xmemdup(
        (if !o.is_null() {
            o
        } else {
            &mut default_quoting_options as *mut quoting_options
        }) as *const libc::c_void,
        ::std::mem::size_of::<quoting_options>() as libc::c_ulong,
    ) as *mut quoting_options;
    *__errno_location() = e;
    return p;
}
pub unsafe extern "C" fn get_quoting_style(
    mut o: *const quoting_options,
) -> quoting_style {
    return (*if !o.is_null() {
        o
    } else {
        &mut default_quoting_options as *mut quoting_options as *const quoting_options
    })
        .style;
}
pub unsafe extern "C" fn set_quoting_style(
    mut o: *mut quoting_options,
    mut s: quoting_style,
) {
    (*if !o.is_null() { o } else { &mut default_quoting_options }).style = s;
}
pub unsafe extern "C" fn set_char_quoting(
    mut o: *mut quoting_options,
    mut c: libc::c_char,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut uc: libc::c_uchar = c as libc::c_uchar;
    let mut p: *mut libc::c_uint = ((*(if !o.is_null() {
        o
    } else {
        &mut default_quoting_options as *mut quoting_options
    }))
        .quote_these_too)
        .as_mut_ptr()
        .offset(
            (uc as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as isize,
        );
    let mut shift: libc::c_int = (uc as libc::c_ulong)
        .wrapping_rem(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
    let mut r: libc::c_int = (*p >> shift & 1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    *p ^= ((i & 1 as libc::c_int ^ r) << shift) as libc::c_uint;
    return r;
}
pub unsafe extern "C" fn set_quoting_flags(
    mut o: *mut quoting_options,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if o.is_null() {
        o = &mut default_quoting_options;
    }
    r = (*o).flags;
    (*o).flags = i;
    return r;
}
pub unsafe extern "C" fn set_custom_quoting(
    mut o: *mut quoting_options,
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
) {
    if o.is_null() {
        o = &mut default_quoting_options;
    }
    (*o).style = custom_quoting_style;
    if left_quote.is_null() || right_quote.is_null() {
        abort();
    }
    (*o).left_quote = left_quote;
    (*o).right_quote = right_quote;
}
unsafe extern "C" fn quoting_options_from_style(
    mut style: quoting_style,
) -> quoting_options {
    let mut o: quoting_options = {
        let mut init = quoting_options {
            style: literal_quoting_style,
            flags: 0 as libc::c_int,
            quote_these_too: [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0, 0],
            left_quote: 0 as *const libc::c_char,
            right_quote: 0 as *const libc::c_char,
        };
        init
    };
    if style as libc::c_uint == custom_quoting_style as libc::c_int as libc::c_uint {
        abort();
    }
    o.style = style;
    return o;
}
unsafe extern "C" fn gettext_quote(
    mut msgid: *const libc::c_char,
    mut s: quoting_style,
) -> *const libc::c_char {
    let mut translation: *const libc::c_char = msgid;
    let mut locale_code: *const libc::c_char = 0 as *const libc::c_char;
    if translation != msgid {
        return translation;
    }
    locale_code = locale_charset();
    if strcaseeq0(
        locale_code,
        b"UTF-8\0" as *const u8 as *const libc::c_char,
        'U' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'F' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ) != 0
    {
        return if *msgid.offset(0 as libc::c_int as isize) as libc::c_int == '`' as i32 {
            b"\xE2\x80\x98\0" as *const u8 as *const libc::c_char
        } else {
            b"\xE2\x80\x99\0" as *const u8 as *const libc::c_char
        };
    }
    if strcaseeq0(
        locale_code,
        b"GB18030\0" as *const u8 as *const libc::c_char,
        'G' as i32 as libc::c_char,
        'B' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ) != 0
    {
        return if *msgid.offset(0 as libc::c_int as isize) as libc::c_int == '`' as i32 {
            b"\xA1\x07e\0" as *const u8 as *const libc::c_char
        } else {
            b"\xA1\xAF\0" as *const u8 as *const libc::c_char
        };
    }
    return if s as libc::c_uint == clocale_quoting_style as libc::c_int as libc::c_uint {
        b"\"\0" as *const u8 as *const libc::c_char
    } else {
        b"'\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn quotearg_buffer_restyled(
    mut buffer: *mut libc::c_char,
    mut buffersize: size_t,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut quoting_style: quoting_style,
    mut flags: libc::c_int,
    mut quote_these_too: *const libc::c_uint,
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
) -> size_t {
    let mut current_block: u64;
    let mut i: size_t = 0;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut orig_buffersize: size_t = 0 as libc::c_int as size_t;
    let mut quote_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut quote_string_len: size_t = 0 as libc::c_int as size_t;
    let mut backslash_escapes: bool = 0 as libc::c_int != 0;
    let mut unibyte_locale: bool = __ctype_get_mb_cur_max()
        == 1 as libc::c_int as libc::c_ulong;
    let mut elide_outer_quotes: bool = flags & QA_ELIDE_OUTER_QUOTES as libc::c_int
        != 0 as libc::c_int;
    let mut pending_shell_escape_end: bool = 0 as libc::c_int != 0;
    let mut encountered_single_quote: bool = 0 as libc::c_int != 0;
    let mut all_c_and_shell_quote_compat: bool = 1 as libc::c_int != 0;
    '_process_input: loop {
        let mut current_block_47: u64;
        match quoting_style as libc::c_uint {
            6 => {
                quoting_style = c_quoting_style;
                elide_outer_quotes = 1 as libc::c_int != 0;
                current_block_47 = 8539537703548529647;
            }
            5 => {
                current_block_47 = 8539537703548529647;
            }
            7 => {
                backslash_escapes = 1 as libc::c_int != 0;
                elide_outer_quotes = 0 as libc::c_int != 0;
                current_block_47 = 14775119014532381840;
            }
            8 | 9 | 10 => {
                if quoting_style as libc::c_uint
                    != custom_quoting_style as libc::c_int as libc::c_uint
                {
                    left_quote = gettext_quote(
                        b"`\0" as *const u8 as *const libc::c_char,
                        quoting_style,
                    );
                    right_quote = gettext_quote(
                        b"'\0" as *const u8 as *const libc::c_char,
                        quoting_style,
                    );
                }
                if !elide_outer_quotes {
                    quote_string = left_quote;
                    while *quote_string != 0 {
                        if len < buffersize {
                            *buffer.offset(len as isize) = *quote_string;
                        }
                        len = len.wrapping_add(1);
                        len;
                        quote_string = quote_string.offset(1);
                        quote_string;
                    }
                }
                backslash_escapes = 1 as libc::c_int != 0;
                quote_string = right_quote;
                quote_string_len = strlen(quote_string);
                current_block_47 = 14775119014532381840;
            }
            3 => {
                backslash_escapes = 1 as libc::c_int != 0;
                current_block_47 = 6882008175243909280;
            }
            1 => {
                current_block_47 = 6882008175243909280;
            }
            4 => {
                current_block_47 = 14530424005953587871;
            }
            2 => {
                current_block_47 = 7189539299038128042;
            }
            0 => {
                elide_outer_quotes = 0 as libc::c_int != 0;
                current_block_47 = 14775119014532381840;
            }
            _ => {
                abort();
            }
        }
        match current_block_47 {
            8539537703548529647 => {
                if !elide_outer_quotes {
                    if len < buffersize {
                        *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                }
                backslash_escapes = 1 as libc::c_int != 0;
                quote_string = b"\"\0" as *const u8 as *const libc::c_char;
                quote_string_len = 1 as libc::c_int as size_t;
                current_block_47 = 14775119014532381840;
            }
            6882008175243909280 => {
                elide_outer_quotes = 1 as libc::c_int != 0;
                current_block_47 = 14530424005953587871;
            }
            _ => {}
        }
        match current_block_47 {
            14530424005953587871 => {
                if !elide_outer_quotes {
                    backslash_escapes = 1 as libc::c_int != 0;
                }
                current_block_47 = 7189539299038128042;
            }
            _ => {}
        }
        match current_block_47 {
            7189539299038128042 => {
                quoting_style = shell_always_quoting_style;
                if !elide_outer_quotes {
                    if len < buffersize {
                        *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                }
                quote_string = b"'\0" as *const u8 as *const libc::c_char;
                quote_string_len = 1 as libc::c_int as size_t;
            }
            _ => {}
        }
        i = 0 as libc::c_int as size_t;
        while if argsize == 18446744073709551615 as libc::c_ulong {
            (*arg.offset(i as isize) as libc::c_int == '\0' as i32) as libc::c_int
        } else {
            (i == argsize) as libc::c_int
        } == 0
        {
            let mut c: libc::c_uchar = 0;
            let mut esc: libc::c_uchar = 0;
            let mut is_right_quote: bool = 0 as libc::c_int != 0;
            let mut escaping: bool = 0 as libc::c_int != 0;
            let mut c_and_shell_quote_compat: bool = 0 as libc::c_int != 0;
            if backslash_escapes as libc::c_int != 0
                && quoting_style as libc::c_uint
                    != shell_always_quoting_style as libc::c_int as libc::c_uint
                && quote_string_len != 0
                && i.wrapping_add(quote_string_len)
                    <= (if argsize == 18446744073709551615 as libc::c_ulong
                        && (1 as libc::c_int as libc::c_ulong) < quote_string_len
                    {
                        argsize = strlen(arg);
                        argsize
                    } else {
                        argsize
                    })
                && memcmp(
                    arg.offset(i as isize) as *const libc::c_void,
                    quote_string as *const libc::c_void,
                    quote_string_len,
                ) == 0 as libc::c_int
            {
                if elide_outer_quotes {
                    current_block = 1591135554063658034;
                    break '_process_input;
                }
                is_right_quote = 1 as libc::c_int != 0;
            }
            c = *arg.offset(i as isize) as libc::c_uchar;
            match c as libc::c_int {
                0 => {
                    if backslash_escapes {
                        if elide_outer_quotes {
                            current_block = 1591135554063658034;
                            break '_process_input;
                        }
                        escaping = 1 as libc::c_int != 0;
                        if quoting_style as libc::c_uint
                            == shell_always_quoting_style as libc::c_int as libc::c_uint
                            && !pending_shell_escape_end
                        {
                            if len < buffersize {
                                *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                            }
                            len = len.wrapping_add(1);
                            len;
                            if len < buffersize {
                                *buffer.offset(len as isize) = '$' as i32 as libc::c_char;
                            }
                            len = len.wrapping_add(1);
                            len;
                            if len < buffersize {
                                *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                            }
                            len = len.wrapping_add(1);
                            len;
                            pending_shell_escape_end = 1 as libc::c_int != 0;
                        }
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\\' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if quoting_style as libc::c_uint
                            != shell_always_quoting_style as libc::c_int as libc::c_uint
                            && i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                < argsize
                            && '0' as i32
                                <= *arg
                                    .offset(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) as libc::c_int
                            && *arg
                                .offset(
                                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int <= '9' as i32
                        {
                            if len < buffersize {
                                *buffer.offset(len as isize) = '0' as i32 as libc::c_char;
                            }
                            len = len.wrapping_add(1);
                            len;
                            if len < buffersize {
                                *buffer.offset(len as isize) = '0' as i32 as libc::c_char;
                            }
                            len = len.wrapping_add(1);
                            len;
                        }
                        c = '0' as i32 as libc::c_uchar;
                        current_block = 12544326035781039657;
                    } else if flags & QA_ELIDE_NULL_BYTES as libc::c_int != 0 {
                        current_block = 16738040538446813684;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
                63 => {
                    match quoting_style as libc::c_uint {
                        2 => {
                            current_block = 17382069803779283506;
                            match current_block {
                                17382069803779283506 => {
                                    if elide_outer_quotes {
                                        current_block = 1591135554063658034;
                                        break '_process_input;
                                    }
                                }
                                _ => {
                                    if flags & QA_SPLIT_TRIGRAPHS as libc::c_int != 0
                                        && i.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            < argsize
                                        && *arg
                                            .offset(
                                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int == '?' as i32
                                    {
                                        match *arg
                                            .offset(
                                                i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int
                                        {
                                            33 | 39 | 40 | 41 | 45 | 47 | 60 | 61 | 62 => {
                                                if elide_outer_quotes {
                                                    current_block = 1591135554063658034;
                                                    break '_process_input;
                                                }
                                                c = *arg
                                                    .offset(
                                                        i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                                    ) as libc::c_uchar;
                                                i = (i as libc::c_ulong)
                                                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                    as size_t;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            current_block = 12544326035781039657;
                        }
                        5 => {
                            current_block = 10235769315957426978;
                            match current_block {
                                17382069803779283506 => {
                                    if elide_outer_quotes {
                                        current_block = 1591135554063658034;
                                        break '_process_input;
                                    }
                                }
                                _ => {
                                    if flags & QA_SPLIT_TRIGRAPHS as libc::c_int != 0
                                        && i.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            < argsize
                                        && *arg
                                            .offset(
                                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int == '?' as i32
                                    {
                                        match *arg
                                            .offset(
                                                i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int
                                        {
                                            33 | 39 | 40 | 41 | 45 | 47 | 60 | 61 | 62 => {
                                                if elide_outer_quotes {
                                                    current_block = 1591135554063658034;
                                                    break '_process_input;
                                                }
                                                c = *arg
                                                    .offset(
                                                        i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                                    ) as libc::c_uchar;
                                                i = (i as libc::c_ulong)
                                                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                    as size_t;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            current_block = 12544326035781039657;
                        }
                        _ => {
                            current_block = 12544326035781039657;
                        }
                    }
                }
                7 => {
                    esc = 'a' as i32 as libc::c_uchar;
                    current_block = 6054433690534082278;
                }
                8 => {
                    esc = 'b' as i32 as libc::c_uchar;
                    current_block = 6054433690534082278;
                }
                12 => {
                    esc = 'f' as i32 as libc::c_uchar;
                    current_block = 6054433690534082278;
                }
                10 => {
                    esc = 'n' as i32 as libc::c_uchar;
                    current_block = 10132356175846169896;
                }
                13 => {
                    esc = 'r' as i32 as libc::c_uchar;
                    current_block = 10132356175846169896;
                }
                9 => {
                    esc = 't' as i32 as libc::c_uchar;
                    current_block = 10132356175846169896;
                }
                11 => {
                    esc = 'v' as i32 as libc::c_uchar;
                    current_block = 6054433690534082278;
                }
                92 => {
                    esc = c;
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                    {
                        if elide_outer_quotes {
                            current_block = 1591135554063658034;
                            break '_process_input;
                        }
                        current_block = 7113191324323038822;
                    } else if backslash_escapes as libc::c_int != 0
                        && elide_outer_quotes as libc::c_int != 0
                        && quote_string_len != 0
                    {
                        current_block = 7113191324323038822;
                    } else {
                        current_block = 10132356175846169896;
                    }
                }
                123 | 125 => {
                    if if argsize == 18446744073709551615 as libc::c_ulong {
                        (*arg.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32) as libc::c_int
                    } else {
                        (argsize == 1 as libc::c_int as libc::c_ulong) as libc::c_int
                    } == 0
                    {
                        current_block = 12544326035781039657;
                    } else {
                        current_block = 13947733080808567182;
                    }
                }
                35 | 126 => {
                    current_block = 13947733080808567182;
                }
                32 => {
                    current_block = 15816705110997260606;
                }
                33 => {
                    current_block = 528731246963496274;
                }
                34 | 36 | 38 | 40 | 41 | 42 | 59 | 60 | 61 => {
                    current_block = 4690494878423956849;
                }
                62 | 91 | 94 => {
                    current_block = 7342314452318261063;
                }
                96 | 124 => {
                    current_block = 17102632830190198796;
                }
                39 => {
                    encountered_single_quote = 1 as libc::c_int != 0;
                    c_and_shell_quote_compat = 1 as libc::c_int != 0;
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                    {
                        if elide_outer_quotes {
                            current_block = 1591135554063658034;
                            break '_process_input;
                        }
                        if buffersize != 0 && orig_buffersize == 0 {
                            orig_buffersize = buffersize;
                            buffersize = 0 as libc::c_int as size_t;
                        }
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\\' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 0 as libc::c_int != 0;
                        current_block = 12544326035781039657;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
                37 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56
                | 57 | 58 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76
                | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90
                | 93 | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107
                | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119
                | 120 | 121 | 122 => {
                    c_and_shell_quote_compat = 1 as libc::c_int != 0;
                    current_block = 12544326035781039657;
                }
                _ => {
                    let mut m: size_t = 0;
                    let mut printable: bool = false;
                    if unibyte_locale {
                        m = 1 as libc::c_int as size_t;
                        printable = *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                            as libc::c_int
                            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                            != 0 as libc::c_int;
                    } else {
                        let mut mbstate: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: C2RustUnnamed { __wch: 0 },
                        };
                        memset(
                            &mut mbstate as *mut mbstate_t as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                        );
                        m = 0 as libc::c_int as size_t;
                        printable = 1 as libc::c_int != 0;
                        if argsize == 18446744073709551615 as libc::c_ulong {
                            argsize = strlen(arg);
                        }
                        loop {
                            let mut w: wchar_t = 0;
                            let mut bytes: size_t = rpl_mbrtowc(
                                &mut w,
                                &*arg.offset(i.wrapping_add(m) as isize),
                                argsize.wrapping_sub(i.wrapping_add(m)),
                                &mut mbstate,
                            );
                            if bytes == 0 as libc::c_int as libc::c_ulong {
                                break;
                            }
                            if bytes == -(1 as libc::c_int) as size_t {
                                printable = 0 as libc::c_int != 0;
                                break;
                            } else if bytes == -(2 as libc::c_int) as size_t {
                                printable = 0 as libc::c_int != 0;
                                while i.wrapping_add(m) < argsize
                                    && *arg.offset(i.wrapping_add(m) as isize) as libc::c_int
                                        != 0
                                {
                                    m = m.wrapping_add(1);
                                    m;
                                }
                                break;
                            } else {
                                if '[' as i32 == 0x5b as libc::c_int
                                    && elide_outer_quotes as libc::c_int != 0
                                    && quoting_style as libc::c_uint
                                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                                {
                                    let mut j: size_t = 0;
                                    j = 1 as libc::c_int as size_t;
                                    while j < bytes {
                                        match *arg
                                            .offset(i.wrapping_add(m).wrapping_add(j) as isize)
                                            as libc::c_int
                                        {
                                            91 | 92 | 94 | 96 | 124 => {
                                                current_block = 1591135554063658034;
                                                break '_process_input;
                                            }
                                            _ => {}
                                        }
                                        j = j.wrapping_add(1);
                                        j;
                                    }
                                }
                                if iswprint(w as wint_t) == 0 {
                                    printable = 0 as libc::c_int != 0;
                                }
                                m = (m as libc::c_ulong).wrapping_add(bytes) as size_t
                                    as size_t;
                                if !(mbsinit(&mut mbstate) == 0) {
                                    break;
                                }
                            }
                        }
                    }
                    c_and_shell_quote_compat = printable;
                    if (1 as libc::c_int as libc::c_ulong) < m
                        || backslash_escapes as libc::c_int != 0 && !printable
                    {
                        let mut ilim: size_t = i.wrapping_add(m);
                        loop {
                            if backslash_escapes as libc::c_int != 0 && !printable {
                                if elide_outer_quotes {
                                    current_block = 1591135554063658034;
                                    break '_process_input;
                                }
                                escaping = 1 as libc::c_int != 0;
                                if quoting_style as libc::c_uint
                                    == shell_always_quoting_style as libc::c_int as libc::c_uint
                                    && !pending_shell_escape_end
                                {
                                    if len < buffersize {
                                        *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                                    }
                                    len = len.wrapping_add(1);
                                    len;
                                    if len < buffersize {
                                        *buffer.offset(len as isize) = '$' as i32 as libc::c_char;
                                    }
                                    len = len.wrapping_add(1);
                                    len;
                                    if len < buffersize {
                                        *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                                    }
                                    len = len.wrapping_add(1);
                                    len;
                                    pending_shell_escape_end = 1 as libc::c_int != 0;
                                }
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\\' as i32 as libc::c_char;
                                }
                                len = len.wrapping_add(1);
                                len;
                                if len < buffersize {
                                    *buffer
                                        .offset(
                                            len as isize,
                                        ) = ('0' as i32 + (c as libc::c_int >> 6 as libc::c_int))
                                        as libc::c_char;
                                }
                                len = len.wrapping_add(1);
                                len;
                                if len < buffersize {
                                    *buffer
                                        .offset(
                                            len as isize,
                                        ) = ('0' as i32
                                        + (c as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int))
                                        as libc::c_char;
                                }
                                len = len.wrapping_add(1);
                                len;
                                c = ('0' as i32 + (c as libc::c_int & 7 as libc::c_int))
                                    as libc::c_uchar;
                            } else if is_right_quote {
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\\' as i32 as libc::c_char;
                                }
                                len = len.wrapping_add(1);
                                len;
                                is_right_quote = 0 as libc::c_int != 0;
                            }
                            if ilim <= i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            {
                                break;
                            }
                            if pending_shell_escape_end as libc::c_int != 0 && !escaping
                            {
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                                }
                                len = len.wrapping_add(1);
                                len;
                                if len < buffersize {
                                    *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                                }
                                len = len.wrapping_add(1);
                                len;
                                pending_shell_escape_end = 0 as libc::c_int != 0;
                            }
                            if len < buffersize {
                                *buffer.offset(len as isize) = c as libc::c_char;
                            }
                            len = len.wrapping_add(1);
                            len;
                            i = i.wrapping_add(1);
                            c = *arg.offset(i as isize) as libc::c_uchar;
                        }
                        current_block = 7113191324323038822;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
            }
            match current_block {
                13947733080808567182 => {
                    if i != 0 as libc::c_int as libc::c_ulong {
                        current_block = 12544326035781039657;
                    } else {
                        current_block = 15816705110997260606;
                    }
                }
                10132356175846169896 => {
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                        && elide_outer_quotes as libc::c_int != 0
                    {
                        current_block = 1591135554063658034;
                        break '_process_input;
                    }
                    current_block = 6054433690534082278;
                }
                _ => {}
            }
            match current_block {
                6054433690534082278 => {
                    if backslash_escapes {
                        c = esc;
                        current_block = 7121273192085788486;
                    } else {
                        current_block = 12544326035781039657;
                    }
                }
                15816705110997260606 => {
                    c_and_shell_quote_compat = 1 as libc::c_int != 0;
                    current_block = 528731246963496274;
                }
                _ => {}
            }
            match current_block {
                528731246963496274 => {
                    current_block = 4690494878423956849;
                }
                _ => {}
            }
            match current_block {
                4690494878423956849 => {
                    current_block = 7342314452318261063;
                }
                _ => {}
            }
            match current_block {
                7342314452318261063 => {
                    current_block = 17102632830190198796;
                }
                _ => {}
            }
            match current_block {
                17102632830190198796 => {
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                        && elide_outer_quotes as libc::c_int != 0
                    {
                        current_block = 1591135554063658034;
                        break '_process_input;
                    }
                    current_block = 12544326035781039657;
                }
                _ => {}
            }
            match current_block {
                12544326035781039657 => {
                    if !((backslash_escapes as libc::c_int != 0
                        && quoting_style as libc::c_uint
                            != shell_always_quoting_style as libc::c_int as libc::c_uint
                        || elide_outer_quotes as libc::c_int != 0)
                        && !quote_these_too.is_null()
                        && *quote_these_too
                            .offset(
                                (c as libc::c_ulong)
                                    .wrapping_div(
                                        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            )
                            >> (c as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) & 1 as libc::c_int as libc::c_uint != 0)
                        && !is_right_quote
                    {
                        current_block = 7113191324323038822;
                    } else {
                        current_block = 7121273192085788486;
                    }
                }
                _ => {}
            }
            match current_block {
                7121273192085788486 => {
                    if elide_outer_quotes {
                        current_block = 1591135554063658034;
                        break '_process_input;
                    }
                    escaping = 1 as libc::c_int != 0;
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                        && !pending_shell_escape_end
                    {
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '$' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 1 as libc::c_int != 0;
                    }
                    if len < buffersize {
                        *buffer.offset(len as isize) = '\\' as i32 as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                    current_block = 7113191324323038822;
                }
                _ => {}
            }
            match current_block {
                7113191324323038822 => {
                    if pending_shell_escape_end as libc::c_int != 0 && !escaping {
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 0 as libc::c_int != 0;
                    }
                    if len < buffersize {
                        *buffer.offset(len as isize) = c as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                    if !c_and_shell_quote_compat {
                        all_c_and_shell_quote_compat = 0 as libc::c_int != 0;
                    }
                }
                _ => {}
            }
            i = i.wrapping_add(1);
            i;
        }
        if len == 0 as libc::c_int as libc::c_ulong
            && quoting_style as libc::c_uint
                == shell_always_quoting_style as libc::c_int as libc::c_uint
            && elide_outer_quotes as libc::c_int != 0
        {
            current_block = 1591135554063658034;
            break;
        }
        if !(quoting_style as libc::c_uint
            == shell_always_quoting_style as libc::c_int as libc::c_uint
            && !elide_outer_quotes && encountered_single_quote as libc::c_int != 0)
        {
            current_block = 5102396516157810314;
            break;
        }
        if all_c_and_shell_quote_compat {
            return quotearg_buffer_restyled(
                buffer,
                orig_buffersize,
                arg,
                argsize,
                c_quoting_style,
                flags,
                quote_these_too,
                left_quote,
                right_quote,
            )
        } else {
            if !(buffersize == 0 && orig_buffersize != 0) {
                current_block = 5102396516157810314;
                break;
            }
            buffersize = orig_buffersize;
            len = 0 as libc::c_int as size_t;
        }
    }
    match current_block {
        1591135554063658034 => {
            if quoting_style as libc::c_uint
                == shell_always_quoting_style as libc::c_int as libc::c_uint
                && backslash_escapes as libc::c_int != 0
            {
                quoting_style = shell_escape_always_quoting_style;
            }
            return quotearg_buffer_restyled(
                buffer,
                buffersize,
                arg,
                argsize,
                quoting_style,
                flags & !(QA_ELIDE_OUTER_QUOTES as libc::c_int),
                0 as *const libc::c_uint,
                left_quote,
                right_quote,
            );
        }
        _ => {
            if !quote_string.is_null() && !elide_outer_quotes {
                while *quote_string != 0 {
                    if len < buffersize {
                        *buffer.offset(len as isize) = *quote_string;
                    }
                    len = len.wrapping_add(1);
                    len;
                    quote_string = quote_string.offset(1);
                    quote_string;
                }
            }
            if len < buffersize {
                *buffer.offset(len as isize) = '\0' as i32 as libc::c_char;
            }
            return len;
        }
    };
}
pub unsafe extern "C" fn quotearg_buffer(
    mut buffer: *mut libc::c_char,
    mut buffersize: size_t,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut o: *const quoting_options,
) -> size_t {
    let mut p: *const quoting_options = if !o.is_null() {
        o
    } else {
        &mut default_quoting_options as *mut quoting_options as *const quoting_options
    };
    let mut e: libc::c_int = *__errno_location();
    let mut r: size_t = quotearg_buffer_restyled(
        buffer,
        buffersize,
        arg,
        argsize,
        (*p).style,
        (*p).flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    );
    *__errno_location() = e;
    return r;
}
pub unsafe extern "C" fn quotearg_alloc(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut o: *const quoting_options,
) -> *mut libc::c_char {
    return quotearg_alloc_mem(arg, argsize, 0 as *mut size_t, o);
}
pub unsafe extern "C" fn quotearg_alloc_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut size: *mut size_t,
    mut o: *const quoting_options,
) -> *mut libc::c_char {
    let mut p: *const quoting_options = if !o.is_null() {
        o
    } else {
        &mut default_quoting_options as *mut quoting_options as *const quoting_options
    };
    let mut e: libc::c_int = *__errno_location();
    let mut flags: libc::c_int = (*p).flags
        | (if !size.is_null() {
            0 as libc::c_int
        } else {
            QA_ELIDE_NULL_BYTES as libc::c_int
        });
    let mut bufsize: size_t = (quotearg_buffer_restyled(
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
        arg,
        argsize,
        (*p).style,
        flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    ))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut buf: *mut libc::c_char = xcharalloc(bufsize);
    quotearg_buffer_restyled(
        buf,
        bufsize,
        arg,
        argsize,
        (*p).style,
        flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    );
    *__errno_location() = e;
    if !size.is_null() {
        *size = bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    return buf;
}
static mut slot0: [libc::c_char; 256] = [0; 256];
static mut nslots: libc::c_int = 1 as libc::c_int;
static mut slotvec0: slotvec = unsafe {
    {
        let mut init = slotvec {
            size: ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            val: slot0.as_ptr() as *mut _,
        };
        init
    }
};
static mut slotvec: *mut slotvec = unsafe {
    &slotvec0 as *const slotvec as *mut slotvec
};
pub unsafe extern "C" fn quotearg_free() {
    let mut sv: *mut slotvec = slotvec;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < nslots {
        free((*sv.offset(i as isize)).val as *mut libc::c_void);
        i += 1;
        i;
    }
    if (*sv.offset(0 as libc::c_int as isize)).val != slot0.as_mut_ptr() {
        free((*sv.offset(0 as libc::c_int as isize)).val as *mut libc::c_void);
        slotvec0.size = ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong;
        slotvec0.val = slot0.as_mut_ptr();
    }
    if sv != &mut slotvec0 as *mut slotvec {
        free(sv as *mut libc::c_void);
        slotvec = &mut slotvec0;
    }
    nslots = 1 as libc::c_int;
}
unsafe extern "C" fn quotearg_n_options(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut options: *const quoting_options,
) -> *mut libc::c_char {
    let mut e: libc::c_int = *__errno_location();
    let mut sv: *mut slotvec = slotvec;
    if n < 0 as libc::c_int {
        abort();
    }
    if nslots <= n {
        let mut preallocated: bool = sv == &mut slotvec0 as *mut slotvec;
        let mut nmax: libc::c_int = (if (2147483647 as libc::c_int as libc::c_ulong)
            < (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                18446744073709551615 as libc::c_ulong
            })
                .wrapping_div(::std::mem::size_of::<slotvec>() as libc::c_ulong)
        {
            2147483647 as libc::c_int as libc::c_ulong
        } else {
            (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                18446744073709551615 as libc::c_ulong
            })
                .wrapping_div(::std::mem::size_of::<slotvec>() as libc::c_ulong)
        })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        if nmax < n {
            xalloc_die();
        }
        sv = xrealloc(
            (if preallocated as libc::c_int != 0 { 0 as *mut slotvec } else { sv })
                as *mut libc::c_void,
            ((n + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<slotvec>() as libc::c_ulong),
        ) as *mut slotvec;
        slotvec = sv;
        if preallocated {
            *sv = slotvec0;
        }
        memset(
            sv.offset(nslots as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((n + 1 as libc::c_int - nslots) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<slotvec>() as libc::c_ulong),
        );
        nslots = n + 1 as libc::c_int;
    }
    let mut size: size_t = (*sv.offset(n as isize)).size;
    let mut val: *mut libc::c_char = (*sv.offset(n as isize)).val;
    let mut flags: libc::c_int = (*options).flags | QA_ELIDE_NULL_BYTES as libc::c_int;
    let mut qsize: size_t = quotearg_buffer_restyled(
        val,
        size,
        arg,
        argsize,
        (*options).style,
        flags,
        ((*options).quote_these_too).as_ptr(),
        (*options).left_quote,
        (*options).right_quote,
    );
    if size <= qsize {
        size = qsize.wrapping_add(1 as libc::c_int as libc::c_ulong);
        (*sv.offset(n as isize)).size = size;
        if val != slot0.as_mut_ptr() {
            free(val as *mut libc::c_void);
        }
        val = xcharalloc(size);
        let ref mut fresh0 = (*sv.offset(n as isize)).val;
        *fresh0 = val;
        quotearg_buffer_restyled(
            val,
            size,
            arg,
            argsize,
            (*options).style,
            flags,
            ((*options).quote_these_too).as_ptr(),
            (*options).left_quote,
            (*options).right_quote,
        );
    }
    *__errno_location() = e;
    return val;
}
pub unsafe extern "C" fn quotearg_n(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    return quotearg_n_options(
        n,
        arg,
        18446744073709551615 as libc::c_ulong,
        &mut default_quoting_options,
    );
}
pub unsafe extern "C" fn quotearg_n_mem(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    return quotearg_n_options(n, arg, argsize, &mut default_quoting_options);
}
pub unsafe extern "C" fn quotearg(mut arg: *const libc::c_char) -> *mut libc::c_char {
    return quotearg_n(0 as libc::c_int, arg);
}
pub unsafe extern "C" fn quotearg_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    return quotearg_n_mem(0 as libc::c_int, arg, argsize);
}
pub unsafe extern "C" fn quotearg_n_style(
    mut n: libc::c_int,
    mut s: quoting_style,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    let o: quoting_options = quoting_options_from_style(s);
    return quotearg_n_options(n, arg, 18446744073709551615 as libc::c_ulong, &o);
}
pub unsafe extern "C" fn quotearg_n_style_mem(
    mut n: libc::c_int,
    mut s: quoting_style,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    let o: quoting_options = quoting_options_from_style(s);
    return quotearg_n_options(n, arg, argsize, &o);
}
pub unsafe extern "C" fn quotearg_style(
    mut s: quoting_style,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    return quotearg_n_style(0 as libc::c_int, s, arg);
}
pub unsafe extern "C" fn quotearg_style_mem(
    mut s: quoting_style,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    return quotearg_n_style_mem(0 as libc::c_int, s, arg, argsize);
}
pub unsafe extern "C" fn quotearg_char_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut ch: libc::c_char,
) -> *mut libc::c_char {
    let mut options: quoting_options = quoting_options {
        style: literal_quoting_style,
        flags: 0,
        quote_these_too: [0; 8],
        left_quote: 0 as *const libc::c_char,
        right_quote: 0 as *const libc::c_char,
    };
    options = default_quoting_options;
    set_char_quoting(&mut options, ch, 1 as libc::c_int);
    return quotearg_n_options(0 as libc::c_int, arg, argsize, &mut options);
}
pub unsafe extern "C" fn quotearg_char(
    mut arg: *const libc::c_char,
    mut ch: libc::c_char,
) -> *mut libc::c_char {
    return quotearg_char_mem(arg, 18446744073709551615 as libc::c_ulong, ch);
}
pub unsafe extern "C" fn quotearg_colon(
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    return quotearg_char(arg, ':' as i32 as libc::c_char);
}
pub unsafe extern "C" fn quotearg_colon_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    return quotearg_char_mem(arg, argsize, ':' as i32 as libc::c_char);
}
pub unsafe extern "C" fn quotearg_n_style_colon(
    mut n: libc::c_int,
    mut s: quoting_style,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    let mut options: quoting_options = quoting_options {
        style: literal_quoting_style,
        flags: 0,
        quote_these_too: [0; 8],
        left_quote: 0 as *const libc::c_char,
        right_quote: 0 as *const libc::c_char,
    };
    options = quoting_options_from_style(s);
    set_char_quoting(&mut options, ':' as i32 as libc::c_char, 1 as libc::c_int);
    return quotearg_n_options(
        n,
        arg,
        18446744073709551615 as libc::c_ulong,
        &mut options,
    );
}
pub unsafe extern "C" fn quotearg_n_custom(
    mut n: libc::c_int,
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    return quotearg_n_custom_mem(
        n,
        left_quote,
        right_quote,
        arg,
        18446744073709551615 as libc::c_ulong,
    );
}
pub unsafe extern "C" fn quotearg_n_custom_mem(
    mut n: libc::c_int,
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    let mut o: quoting_options = default_quoting_options;
    set_custom_quoting(&mut o, left_quote, right_quote);
    return quotearg_n_options(n, arg, argsize, &mut o);
}
pub unsafe extern "C" fn quotearg_custom(
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    return quotearg_n_custom(0 as libc::c_int, left_quote, right_quote, arg);
}
pub unsafe extern "C" fn quotearg_custom_mem(
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    return quotearg_n_custom_mem(
        0 as libc::c_int,
        left_quote,
        right_quote,
        arg,
        argsize,
    );
}
pub static mut quote_quoting_options: quoting_options = {
    let mut init = quoting_options {
        style: locale_quoting_style,
        flags: 0 as libc::c_int,
        quote_these_too: [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0, 0],
        left_quote: 0 as *const libc::c_char,
        right_quote: 0 as *const libc::c_char,
    };
    init
};
pub unsafe extern "C" fn quote_n_mem(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *const libc::c_char {
    return quotearg_n_options(n, arg, argsize, &mut quote_quoting_options);
}
pub unsafe extern "C" fn quote_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *const libc::c_char {
    return quote_n_mem(0 as libc::c_int, arg, argsize);
}
pub unsafe extern "C" fn quote_n(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
) -> *const libc::c_char {
    return quote_n_mem(n, arg, 18446744073709551615 as libc::c_ulong);
}
pub unsafe extern "C" fn quote(mut arg: *const libc::c_char) -> *const libc::c_char {
    return quote_n(0 as libc::c_int, arg);
}
