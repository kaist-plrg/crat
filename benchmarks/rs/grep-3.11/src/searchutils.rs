use ::libc;
extern "C" {
    pub type kwset;
    fn __ctype_get_mb_cur_max() -> size_t;
    static mut localeinfo: localeinfo;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn rpl_mbrlen(s: *const libc::c_char, n: size_t, ps: *mut mbstate_t) -> size_t;
    fn iswalnum(__wc: wint_t) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut match_icase: bool;
    fn kwsalloc(_: *const libc::c_char) -> kwset_t;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
}
pub type __int32_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
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
pub type ptrdiff_t = libc::c_long;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NCHAR: C2RustUnnamed_0 = 256;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
pub type kwset_t = *mut kwset;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn imbrlen(
    mut s: *const libc::c_char,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: size_t = rpl_mbrlen(s, n as size_t, mbs);
    if len <= 16 as libc::c_int as libc::c_ulong {
        return len as ptrdiff_t;
    }
    let mut neglen: ptrdiff_t = len.wrapping_neg() as ptrdiff_t;
    return -neglen;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_clen(
    mut s: *const libc::c_char,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: libc::c_schar = localeinfo.sbclen[to_uchar(*s) as usize];
    return if len as libc::c_int == -(2 as libc::c_int) {
        imbrlen(s, n, mbs)
    } else {
        len as libc::c_long
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
static mut sbwordchar: [bool; 256] = [false; 256];
unsafe extern "C" fn wordchar(mut wc: wint_t) -> bool {
    return wc == '_' as i32 as libc::c_uint || iswalnum(wc) != 0;
}
pub unsafe extern "C" fn wordinit() {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < NCHAR as libc::c_int {
        sbwordchar[i as usize] = wordchar(localeinfo.sbctowc[i as usize]);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn kwsinit(mut mb_trans: bool) -> kwset_t {
    let mut trans: *mut libc::c_char = 0 as *mut libc::c_char;
    if match_icase as libc::c_int != 0
        && (__ctype_get_mb_cur_max() == 1 as libc::c_int as libc::c_ulong
            || mb_trans as libc::c_int != 0)
    {
        trans = ximalloc(NCHAR as libc::c_int as idx_t) as *mut libc::c_char;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < NCHAR as libc::c_int {
            *trans
                .offset(
                    i as isize,
                ) = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = i;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(i);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(i as isize);
                }
                __res
            }) as libc::c_char;
            i += 1;
            i;
        }
    }
    return kwsalloc(trans);
}
pub unsafe extern "C" fn mb_goback(
    mut mb_start: *mut *const libc::c_char,
    mut mbclen: *mut idx_t,
    mut cur: *const libc::c_char,
    mut end: *const libc::c_char,
) -> ptrdiff_t {
    let mut p: *const libc::c_char = *mb_start;
    let mut p0: *const libc::c_char = p;
    if cur <= p {
        return cur.offset_from(p) as libc::c_long;
    }
    if localeinfo.using_utf8 {
        p = cur;
        if *cur as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int {
            let mut i: libc::c_int = 1 as libc::c_int;
            while i <= 3 as libc::c_int {
                if *cur.offset(-i as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    let mut long_enough: bool = (!(*cur.offset(-i as isize)
                        as libc::c_int) & 0xff as libc::c_int) >> 7 as libc::c_int - i
                        == 0 as libc::c_int;
                    if long_enough {
                        let mut mbs: mbstate_t = {
                            let mut init = __mbstate_t {
                                __count: 0 as libc::c_int,
                                __value: C2RustUnnamed { __wch: 0 },
                            };
                            init
                        };
                        let mut clen: ptrdiff_t = imbrlen(
                            cur.offset(-(i as isize)),
                            end.offset_from(cur.offset(-(i as isize))) as libc::c_long,
                            &mut mbs,
                        );
                        if 0 as libc::c_int as libc::c_long <= clen {
                            p0 = cur.offset(-(i as isize));
                            p = p0.offset(clen as isize);
                        }
                    }
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
    } else {
        let mut mbs_0: mbstate_t = {
            let mut init = __mbstate_t {
                __count: 0 as libc::c_int,
                __value: C2RustUnnamed { __wch: 0 },
            };
            init
        };
        let mut clen_0: ptrdiff_t = 0;
        loop {
            clen_0 = mb_clen(p, end.offset_from(p) as libc::c_long, &mut mbs_0);
            if clen_0 < 0 as libc::c_int as libc::c_long {
                clen_0 = 1 as libc::c_int as ptrdiff_t;
                memset(
                    &mut mbs_0 as *mut mbstate_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                );
            }
            p0 = p;
            p = p.offset(clen_0 as isize);
            if !(p < cur) {
                break;
            }
        }
        if !mbclen.is_null() {
            *mbclen = clen_0;
        }
    }
    *mb_start = p;
    return if p == cur {
        0 as libc::c_int as libc::c_long
    } else {
        cur.offset_from(p0) as libc::c_long
    };
}
unsafe extern "C" fn wordchars_count(
    mut buf: *const libc::c_char,
    mut end: *const libc::c_char,
    mut countall: bool,
) -> idx_t {
    let mut n: idx_t = 0 as libc::c_int as idx_t;
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    while n < end.offset_from(buf) as libc::c_long {
        let mut b: libc::c_uchar = *buf.offset(n as isize) as libc::c_uchar;
        if sbwordchar[b as usize] {
            n += 1;
            n;
        } else {
            if localeinfo.sbclen[b as usize] as libc::c_int != -(2 as libc::c_int) {
                break;
            }
            let mut wc: wchar_t = 0 as libc::c_int;
            let mut wcbytes: size_t = rpl_mbrtowc(
                &mut wc,
                buf.offset(n as isize),
                (end.offset_from(buf) as libc::c_long - n) as size_t,
                &mut mbs,
            );
            if !wordchar(wc as wint_t) {
                break;
            }
            n = (n as libc::c_ulong)
                .wrapping_add(
                    wcbytes.wrapping_add((wcbytes == 0) as libc::c_int as libc::c_ulong),
                ) as idx_t as idx_t;
        }
        if !countall {
            break;
        }
    }
    return n;
}
pub unsafe extern "C" fn wordchars_size(
    mut buf: *const libc::c_char,
    mut end: *const libc::c_char,
) -> idx_t {
    return wordchars_count(buf, end, 1 as libc::c_int != 0);
}
pub unsafe extern "C" fn wordchar_next(
    mut buf: *const libc::c_char,
    mut end: *const libc::c_char,
) -> idx_t {
    return wordchars_count(buf, end, 0 as libc::c_int != 0);
}
pub unsafe extern "C" fn wordchar_prev(
    mut buf: *const libc::c_char,
    mut cur: *const libc::c_char,
    mut end: *const libc::c_char,
) -> idx_t {
    if buf == cur {
        return 0 as libc::c_int as idx_t;
    }
    cur = cur.offset(-1);
    let mut b: libc::c_uchar = *cur as libc::c_uchar;
    if !localeinfo.multibyte
        || localeinfo.using_utf8 as libc::c_int & !(b as libc::c_int >> 7 as libc::c_int)
            != 0
    {
        return sbwordchar[b as usize] as idx_t;
    }
    let mut p: *const libc::c_char = buf;
    cur = cur.offset(-(mb_goback(&mut p, 0 as *mut idx_t, cur, end) as isize));
    return wordchar_next(cur, end);
}
