use ::libc;
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswcntrl(__wc: wint_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISprint: C2RustUnnamed_0 = 16384;
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
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub unsafe extern "C" fn gnu_mbswidth(
    mut string: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    return mbsnwidth(string, strlen(string), flags);
}
pub unsafe extern "C" fn mbsnwidth(
    mut string: *const libc::c_char,
    mut nbytes: size_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const libc::c_char = string;
    let mut plimit: *const libc::c_char = p.offset(nbytes as isize);
    let mut width: libc::c_int = 0;
    width = 0 as libc::c_int;
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong {
        's_13: loop {
            if !(p < plimit) {
                current_block = 6281126495347172768;
                break;
            }
            match *p as libc::c_int {
                32 | 33 | 34 | 35 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47
                | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61
                | 62 | 63 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76
                | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90
                | 91 | 92 | 93 | 94 | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
                | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116
                | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 => {
                    p = p.offset(1);
                    p;
                    width += 1;
                    width;
                }
                _ => {
                    let mut mbstate: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed { __wch: 0 },
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
                            if flags & 1 as libc::c_int == 0 {
                                p = p.offset(1);
                                p;
                                width += 1;
                                width;
                                break;
                            } else {
                                return -(1 as libc::c_int)
                            }
                        } else if bytes == -(2 as libc::c_int) as size_t {
                            if flags & 1 as libc::c_int == 0 {
                                p = plimit;
                                width += 1;
                                width;
                                break;
                            } else {
                                return -(1 as libc::c_int)
                            }
                        } else {
                            if bytes == 0 as libc::c_int as libc::c_ulong {
                                bytes = 1 as libc::c_int as size_t;
                            }
                            w = wcwidth(wc);
                            if w >= 0 as libc::c_int {
                                if w > 2147483647 as libc::c_int - width {
                                    current_block = 8529002811395984474;
                                    break 's_13;
                                }
                                width += w;
                            } else if flags & 2 as libc::c_int == 0 {
                                if iswcntrl(wc as wint_t) == 0 {
                                    if width == 2147483647 as libc::c_int {
                                        current_block = 8529002811395984474;
                                        break 's_13;
                                    }
                                    width += 1;
                                    width;
                                }
                            } else {
                                return -(1 as libc::c_int)
                            }
                            p = p.offset(bytes as isize);
                            if !(mbsinit(&mut mbstate) == 0) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            8529002811395984474 => {}
            _ => return width,
        }
    } else {
        loop {
            if !(p < plimit) {
                current_block = 10007731352114176167;
                break;
            }
            let fresh0 = p;
            p = p.offset(1);
            let mut c: libc::c_uchar = *fresh0 as libc::c_uchar;
            if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if width == 2147483647 as libc::c_int {
                    current_block = 8529002811395984474;
                    break;
                }
                width += 1;
                width;
            } else if flags & 2 as libc::c_int == 0 {
                if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int == 0)
                {
                    continue;
                }
                if width == 2147483647 as libc::c_int {
                    current_block = 8529002811395984474;
                    break;
                }
                width += 1;
                width;
            } else {
                return -(1 as libc::c_int)
            }
        }
        match current_block {
            8529002811395984474 => {}
            _ => return width,
        }
    }
    return 2147483647 as libc::c_int;
}
