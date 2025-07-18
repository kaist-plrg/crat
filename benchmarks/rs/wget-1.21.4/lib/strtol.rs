use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __int32_t = libc::c_int;
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
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn rpl_strtol(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> libc::c_long {
    let mut negative: libc::c_int = 0;
    let mut cutoff: libc::c_ulong = 0;
    let mut cutlim: libc::c_uint = 0;
    let mut i: libc::c_ulong = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_uchar = 0;
    let mut save: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut overflow: libc::c_int = 0;
    if base < 0 as libc::c_int || base == 1 as libc::c_int || base > 36 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int as libc::c_long;
    }
    s = nptr;
    save = s;
    while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
    }
    if !(*s as libc::c_int == '\0' as i32) {
        if *s as libc::c_int == '-' as i32 {
            negative = 1 as libc::c_int;
            s = s.offset(1);
            s;
        } else if *s as libc::c_int == '+' as i32 {
            negative = 0 as libc::c_int;
            s = s.offset(1);
            s;
        } else {
            negative = 0 as libc::c_int;
        }
        if *s as libc::c_int == '0' as i32 {
            if (base == 0 as libc::c_int || base == 16 as libc::c_int)
                && ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *s
                                .offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(
                                *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(
                                *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int as isize,
                            );
                    }
                    __res
                }) == 'X' as i32
            {
                s = s.offset(2 as libc::c_int as isize);
                base = 16 as libc::c_int;
            } else if (base == 0 as libc::c_int || base == 2 as libc::c_int)
                && ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *s
                                .offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(
                                *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(
                                *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int as isize,
                            );
                    }
                    __res
                }) == 'B' as i32
            {
                s = s.offset(2 as libc::c_int as isize);
                base = 2 as libc::c_int;
            } else if base == 0 as libc::c_int {
                base = 8 as libc::c_int;
            }
        } else if base == 0 as libc::c_int {
            base = 10 as libc::c_int;
        }
        save = s;
        end = 0 as *const libc::c_char;
        cutoff = (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
            .wrapping_div(base as libc::c_ulong);
        cutlim = (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
            .wrapping_rem(base as libc::c_ulong) as libc::c_uint;
        overflow = 0 as libc::c_int;
        i = 0 as libc::c_int as libc::c_ulong;
        c = *s as libc::c_uchar;
        while c as libc::c_int != '\0' as i32 {
            if s == end {
                break;
            }
            if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                c = (c as libc::c_int - '0' as i32) as libc::c_uchar;
            } else {
                if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    break;
                }
                c = (({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = c as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(c as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(c as libc::c_int as isize);
                    }
                    __res
                }) - 'A' as i32 + 10 as libc::c_int) as libc::c_uchar;
            }
            if c as libc::c_int >= base {
                break;
            }
            if i > cutoff || i == cutoff && c as libc::c_uint > cutlim {
                overflow = 1 as libc::c_int;
            } else {
                i = i.wrapping_mul(base as libc::c_ulong);
                i = i.wrapping_add(c as libc::c_ulong);
            }
            s = s.offset(1);
            c = *s as libc::c_uchar;
        }
        if !(s == save) {
            if !endptr.is_null() {
                *endptr = s as *mut libc::c_char;
            }
            if overflow == 0 as libc::c_int
                && i
                    > (if negative != 0 {
                        ((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                            .wrapping_neg()
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        9223372036854775807 as libc::c_long as libc::c_ulong
                    })
            {
                overflow = 1 as libc::c_int;
            }
            if overflow != 0 {
                *__errno_location() = 34 as libc::c_int;
                return if negative != 0 {
                    -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                } else {
                    9223372036854775807 as libc::c_long
                };
            }
            return (if negative != 0 { i.wrapping_neg() } else { i }) as libc::c_long;
        }
    }
    if !endptr.is_null() {
        if save.offset_from(nptr) as libc::c_long >= 2 as libc::c_int as libc::c_long
            && (({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *save
                            .offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(
                            *save.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(
                            *save.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        );
                }
                __res
            }) == 'X' as i32
                || ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *save
                                .offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(
                                *save.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                    as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(
                                *save.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                    as libc::c_int as isize,
                            );
                    }
                    __res
                }) == 'B' as i32)
            && *save.offset(-(2 as libc::c_int) as isize) as libc::c_int == '0' as i32
        {
            *endptr = &*save.offset(-(1 as libc::c_int) as isize) as *const libc::c_char
                as *mut libc::c_char;
        } else {
            *endptr = nptr as *mut libc::c_char;
        }
    }
    return 0 as libc::c_long;
}
