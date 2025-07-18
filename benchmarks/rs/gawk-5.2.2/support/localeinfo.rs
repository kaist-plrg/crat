use ::libc;
extern "C" {
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type mbstate_t = __mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
pub const native_c_charset: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
unsafe extern "C" fn is_using_utf8() -> bool {
    let mut wc: wchar_t = 0;
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    return mbrtowc(
        &mut wc,
        b"\xC4\x80\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
        &mut mbs,
    ) == 2 as libc::c_int as libc::c_ulong && wc == 0x100 as libc::c_int;
}
unsafe extern "C" fn using_simple_locale(mut multibyte: bool) -> bool {
    if native_c_charset as libc::c_int == 0 || multibyte as libc::c_int != 0 {
        return 0 as libc::c_int != 0;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
        if 0 as libc::c_int
            <= strcoll(
                [i as libc::c_char, 0 as libc::c_int as libc::c_char].as_mut_ptr(),
                [
                    (i + 1 as libc::c_int) as libc::c_char,
                    0 as libc::c_int as libc::c_char,
                ]
                    .as_mut_ptr(),
            )
        {
            return 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn init_localeinfo(mut localeinfo: *mut localeinfo) {
    (*localeinfo)
        .multibyte = __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong;
    (*localeinfo).simple = using_simple_locale((*localeinfo).multibyte);
    (*localeinfo).using_utf8 = is_using_utf8();
    let mut i: libc::c_int = -(127 as libc::c_int) - 1 as libc::c_int;
    while i <= 127 as libc::c_int {
        let mut c: libc::c_char = i as libc::c_char;
        let mut uc: libc::c_uchar = i as libc::c_uchar;
        let mut s: mbstate_t = {
            let mut init = __mbstate_t {
                __count: 0 as libc::c_int,
                __value: C2RustUnnamed { __wch: 0 },
            };
            init
        };
        let mut wc: wchar_t = 0;
        let mut len: size_t = mbrtowc(
            &mut wc,
            &mut c,
            1 as libc::c_int as size_t,
            &mut s,
        );
        (*localeinfo)
            .sbclen[uc
            as usize] = (if len <= 1 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            -(len.wrapping_neg() as libc::c_int)
        }) as libc::c_schar;
        (*localeinfo)
            .sbctowc[uc
            as usize] = if len <= 1 as libc::c_int as libc::c_ulong {
            wc as libc::c_uint
        } else {
            0xffffffff as libc::c_uint
        };
        i += 1;
        i;
    }
}
static mut lonesome_lower: [libc::c_short; 19] = [
    0xb5 as libc::c_int as libc::c_short,
    0x131 as libc::c_int as libc::c_short,
    0x17f as libc::c_int as libc::c_short,
    0x1c5 as libc::c_int as libc::c_short,
    0x1c8 as libc::c_int as libc::c_short,
    0x1cb as libc::c_int as libc::c_short,
    0x1f2 as libc::c_int as libc::c_short,
    0x345 as libc::c_int as libc::c_short,
    0x3c2 as libc::c_int as libc::c_short,
    0x3d0 as libc::c_int as libc::c_short,
    0x3d1 as libc::c_int as libc::c_short,
    0x3d5 as libc::c_int as libc::c_short,
    0x3d6 as libc::c_int as libc::c_short,
    0x3f0 as libc::c_int as libc::c_short,
    0x3f1 as libc::c_int as libc::c_short,
    0x3f2 as libc::c_int as libc::c_short,
    0x3f5 as libc::c_int as libc::c_short,
    0x1e9b as libc::c_int as libc::c_short,
    0x1fbe as libc::c_int as libc::c_short,
];
pub unsafe extern "C" fn case_folded_counterparts(
    mut c: wint_t,
    mut folded: *mut wchar_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut uc: wint_t = towupper(c);
    let mut lc: wint_t = towlower(uc);
    if uc != c {
        let fresh0 = n;
        n = n + 1;
        *folded.offset(fresh0 as isize) = uc as wchar_t;
    }
    if lc != uc && lc != c && towupper(lc) == uc {
        let fresh1 = n;
        n = n + 1;
        *folded.offset(fresh1 as isize) = lc as wchar_t;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_short; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
    {
        let mut li: wint_t = lonesome_lower[i as usize] as wint_t;
        if li != lc && li != uc && li != c && towupper(li) == uc {
            let fresh2 = n;
            n = n + 1;
            *folded.offset(fresh2 as isize) = li as wchar_t;
        }
        i += 1;
        i;
    }
    return n;
}
