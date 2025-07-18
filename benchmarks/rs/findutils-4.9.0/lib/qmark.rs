use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type __mbstate_t = mbstate_t;
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
unsafe extern "C" fn unibyte_qmark_chars(
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut p: *mut libc::c_char = buf;
    let mut plimit: *const libc::c_char = buf.offset(len as isize);
    while p < plimit as *mut libc::c_char {
        if !(1 as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(to_uchar(*p) as libc::c_int as isize)
                as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
        {
            *p = '?' as i32 as libc::c_char;
        }
        p = p.offset(1);
        p;
    }
    return len;
}
pub unsafe extern "C" fn qmark_chars(
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> size_t {
    if __ctype_get_mb_cur_max() <= 1 as libc::c_int as libc::c_ulong {
        return unibyte_qmark_chars(buf, len)
    } else {
        let mut p: *const libc::c_char = buf;
        let mut plimit: *const libc::c_char = buf.offset(len as isize);
        let mut q: *mut libc::c_char = buf;
        while p < plimit {
            match *p as libc::c_int {
                32 | 33 | 34 | 35 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47
                | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61
                | 62 | 63 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76
                | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90
                | 91 | 92 | 93 | 94 | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
                | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116
                | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 => {
                    let fresh0 = p;
                    p = p.offset(1);
                    let fresh1 = q;
                    q = q.offset(1);
                    *fresh1 = *fresh0;
                }
                _ => {
                    let mut mbstate: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed_0 { __wch: 0 },
                    };
                    memset(
                        &mut mbstate as *mut mbstate_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                    );
                    loop {
                        let mut wc: wchar_t = 0;
                        let mut bytes: size_t = 0;
                        let mut w: libc::c_int = 0;
                        bytes = rpl_mbrtowc(
                            &mut wc,
                            p,
                            plimit.offset_from(p) as libc::c_long as size_t,
                            &mut mbstate,
                        );
                        if bytes == -(1 as libc::c_int) as size_t {
                            p = p.offset(1);
                            p;
                            let fresh2 = q;
                            q = q.offset(1);
                            *fresh2 = '?' as i32 as libc::c_char;
                            break;
                        } else if bytes == -(2 as libc::c_int) as size_t {
                            p = plimit;
                            let fresh3 = q;
                            q = q.offset(1);
                            *fresh3 = '?' as i32 as libc::c_char;
                            break;
                        } else {
                            if bytes == 0 as libc::c_int as libc::c_ulong {
                                bytes = 1 as libc::c_int as size_t;
                            }
                            w = wcwidth(wc);
                            if w >= 0 as libc::c_int {
                                while bytes > 0 as libc::c_int as libc::c_ulong {
                                    let fresh4 = p;
                                    p = p.offset(1);
                                    let fresh5 = q;
                                    q = q.offset(1);
                                    *fresh5 = *fresh4;
                                    bytes = bytes.wrapping_sub(1);
                                    bytes;
                                }
                            } else {
                                p = p.offset(bytes as isize);
                                let fresh6 = q;
                                q = q.offset(1);
                                *fresh6 = '?' as i32 as libc::c_char;
                            }
                            if !(mbsinit(&mut mbstate) == 0) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        len = q.offset_from(buf) as libc::c_long as size_t;
        return len;
    };
}
