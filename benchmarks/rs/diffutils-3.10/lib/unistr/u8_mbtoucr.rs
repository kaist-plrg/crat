use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn u8_mbtoucr(
    mut puc: *mut ucs4_t,
    mut s: *const uint8_t,
    mut n: size_t,
) -> libc::c_int {
    let mut c: uint8_t = *s;
    if (c as libc::c_int) < 0x80 as libc::c_int {
        *puc = c as ucs4_t;
        return 1 as libc::c_int;
    } else if c as libc::c_int >= 0xc2 as libc::c_int {
        if (c as libc::c_int) < 0xe0 as libc::c_int {
            if n >= 2 as libc::c_int as libc::c_ulong {
                if (*s.offset(1 as libc::c_int as isize) as libc::c_int
                    ^ 0x80 as libc::c_int) < 0x40 as libc::c_int
                {
                    *puc = ((c as libc::c_int & 0x1f as libc::c_int) as libc::c_uint)
                        << 6 as libc::c_int
                        | (*s.offset(1 as libc::c_int as isize) as libc::c_int
                            ^ 0x80 as libc::c_int) as libc::c_uint;
                    return 2 as libc::c_int;
                }
            } else {
                *puc = 0xfffd as libc::c_int as ucs4_t;
                return -(2 as libc::c_int);
            }
        } else if (c as libc::c_int) < 0xf0 as libc::c_int {
            if n >= 2 as libc::c_int as libc::c_ulong {
                if (*s.offset(1 as libc::c_int as isize) as libc::c_int
                    ^ 0x80 as libc::c_int) < 0x40 as libc::c_int
                    && (c as libc::c_int >= 0xe1 as libc::c_int
                        || *s.offset(1 as libc::c_int as isize) as libc::c_int
                            >= 0xa0 as libc::c_int)
                    && (c as libc::c_int != 0xed as libc::c_int
                        || (*s.offset(1 as libc::c_int as isize) as libc::c_int)
                            < 0xa0 as libc::c_int)
                {
                    if n >= 3 as libc::c_int as libc::c_ulong {
                        if (*s.offset(2 as libc::c_int as isize) as libc::c_int
                            ^ 0x80 as libc::c_int) < 0x40 as libc::c_int
                        {
                            *puc = ((c as libc::c_int & 0xf as libc::c_int)
                                as libc::c_uint) << 12 as libc::c_int
                                | ((*s.offset(1 as libc::c_int as isize) as libc::c_int
                                    ^ 0x80 as libc::c_int) as libc::c_uint) << 6 as libc::c_int
                                | (*s.offset(2 as libc::c_int as isize) as libc::c_int
                                    ^ 0x80 as libc::c_int) as libc::c_uint;
                            return 3 as libc::c_int;
                        }
                    } else {
                        *puc = 0xfffd as libc::c_int as ucs4_t;
                        return -(2 as libc::c_int);
                    }
                }
            } else {
                *puc = 0xfffd as libc::c_int as ucs4_t;
                return -(2 as libc::c_int);
            }
        } else if (c as libc::c_int) < 0xf8 as libc::c_int {
            if n >= 2 as libc::c_int as libc::c_ulong {
                if (*s.offset(1 as libc::c_int as isize) as libc::c_int
                    ^ 0x80 as libc::c_int) < 0x40 as libc::c_int
                    && (c as libc::c_int >= 0xf1 as libc::c_int
                        || *s.offset(1 as libc::c_int as isize) as libc::c_int
                            >= 0x90 as libc::c_int)
                    && ((c as libc::c_int) < 0xf4 as libc::c_int
                        || c as libc::c_int == 0xf4 as libc::c_int
                            && (*s.offset(1 as libc::c_int as isize) as libc::c_int)
                                < 0x90 as libc::c_int)
                {
                    if n >= 3 as libc::c_int as libc::c_ulong {
                        if (*s.offset(2 as libc::c_int as isize) as libc::c_int
                            ^ 0x80 as libc::c_int) < 0x40 as libc::c_int
                        {
                            if n >= 4 as libc::c_int as libc::c_ulong {
                                if (*s.offset(3 as libc::c_int as isize) as libc::c_int
                                    ^ 0x80 as libc::c_int) < 0x40 as libc::c_int
                                {
                                    *puc = ((c as libc::c_int & 0x7 as libc::c_int)
                                        as libc::c_uint) << 18 as libc::c_int
                                        | ((*s.offset(1 as libc::c_int as isize) as libc::c_int
                                            ^ 0x80 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
                                        | ((*s.offset(2 as libc::c_int as isize) as libc::c_int
                                            ^ 0x80 as libc::c_int) as libc::c_uint) << 6 as libc::c_int
                                        | (*s.offset(3 as libc::c_int as isize) as libc::c_int
                                            ^ 0x80 as libc::c_int) as libc::c_uint;
                                    return 4 as libc::c_int;
                                }
                            } else {
                                *puc = 0xfffd as libc::c_int as ucs4_t;
                                return -(2 as libc::c_int);
                            }
                        }
                    } else {
                        *puc = 0xfffd as libc::c_int as ucs4_t;
                        return -(2 as libc::c_int);
                    }
                }
            } else {
                *puc = 0xfffd as libc::c_int as ucs4_t;
                return -(2 as libc::c_int);
            }
        }
    }
    *puc = 0xfffd as libc::c_int as ucs4_t;
    return -(1 as libc::c_int);
}
