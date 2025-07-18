use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub unsafe extern "C" fn utf8_encode(
    mut codepoint: int32_t,
    mut buffer: *mut libc::c_char,
    mut size: *mut libc::c_int,
) -> libc::c_int {
    if codepoint < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else if codepoint < 0x80 as libc::c_int {
        *buffer.offset(0 as libc::c_int as isize) = codepoint as libc::c_char;
        *size = 1 as libc::c_int;
    } else if codepoint < 0x800 as libc::c_int {
        *buffer
            .offset(
                0 as libc::c_int as isize,
            ) = (0xc0 as libc::c_int
            + ((codepoint & 0x7c0 as libc::c_int) >> 6 as libc::c_int)) as libc::c_char;
        *buffer
            .offset(
                1 as libc::c_int as isize,
            ) = (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int))
            as libc::c_char;
        *size = 2 as libc::c_int;
    } else if codepoint < 0x10000 as libc::c_int {
        *buffer
            .offset(
                0 as libc::c_int as isize,
            ) = (0xe0 as libc::c_int
            + ((codepoint & 0xf000 as libc::c_int) >> 12 as libc::c_int))
            as libc::c_char;
        *buffer
            .offset(
                1 as libc::c_int as isize,
            ) = (0x80 as libc::c_int
            + ((codepoint & 0xfc0 as libc::c_int) >> 6 as libc::c_int)) as libc::c_char;
        *buffer
            .offset(
                2 as libc::c_int as isize,
            ) = (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int))
            as libc::c_char;
        *size = 3 as libc::c_int;
    } else if codepoint <= 0x10ffff as libc::c_int {
        *buffer
            .offset(
                0 as libc::c_int as isize,
            ) = (0xf0 as libc::c_int
            + ((codepoint & 0x1c0000 as libc::c_int) >> 18 as libc::c_int))
            as libc::c_char;
        *buffer
            .offset(
                1 as libc::c_int as isize,
            ) = (0x80 as libc::c_int
            + ((codepoint & 0x3f000 as libc::c_int) >> 12 as libc::c_int))
            as libc::c_char;
        *buffer
            .offset(
                2 as libc::c_int as isize,
            ) = (0x80 as libc::c_int
            + ((codepoint & 0xfc0 as libc::c_int) >> 6 as libc::c_int)) as libc::c_char;
        *buffer
            .offset(
                3 as libc::c_int as isize,
            ) = (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int))
            as libc::c_char;
        *size = 4 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn utf8_check_first(mut byte: libc::c_char) -> libc::c_int {
    let mut u: libc::c_uchar = byte as libc::c_uchar;
    if (u as libc::c_int) < 0x80 as libc::c_int {
        return 1 as libc::c_int;
    }
    if u as libc::c_int <= 0xbf as libc::c_int {
        return 0 as libc::c_int
    } else if u as libc::c_int == 0xc0 as libc::c_int
        || u as libc::c_int == 0xc1 as libc::c_int
    {
        return 0 as libc::c_int
    } else if u as libc::c_int <= 0xdf as libc::c_int {
        return 2 as libc::c_int
    } else if u as libc::c_int <= 0xef as libc::c_int {
        return 3 as libc::c_int
    } else if u as libc::c_int <= 0xf4 as libc::c_int {
        return 4 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn utf8_check_full(
    mut buffer: *const libc::c_char,
    mut size: libc::c_int,
    mut codepoint: *mut int32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut value: int32_t = 0 as libc::c_int;
    let mut u: libc::c_uchar = *buffer.offset(0 as libc::c_int as isize)
        as libc::c_uchar;
    if size == 2 as libc::c_int {
        value = u as libc::c_int & 0x1f as libc::c_int;
    } else if size == 3 as libc::c_int {
        value = u as libc::c_int & 0xf as libc::c_int;
    } else if size == 4 as libc::c_int {
        value = u as libc::c_int & 0x7 as libc::c_int;
    } else {
        return 0 as libc::c_int
    }
    i = 1 as libc::c_int;
    while i < size {
        u = *buffer.offset(i as isize) as libc::c_uchar;
        if (u as libc::c_int) < 0x80 as libc::c_int
            || u as libc::c_int > 0xbf as libc::c_int
        {
            return 0 as libc::c_int;
        }
        value = (value << 6 as libc::c_int) + (u as libc::c_int & 0x3f as libc::c_int);
        i += 1;
        i;
    }
    if value > 0x10ffff as libc::c_int {
        return 0 as libc::c_int
    } else if 0xd800 as libc::c_int <= value && value <= 0xdfff as libc::c_int {
        return 0 as libc::c_int
    } else if size == 2 as libc::c_int && value < 0x80 as libc::c_int
        || size == 3 as libc::c_int && value < 0x800 as libc::c_int
        || size == 4 as libc::c_int && value < 0x10000 as libc::c_int
    {
        return 0 as libc::c_int
    }
    if !codepoint.is_null() {
        *codepoint = value;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn utf8_iterate(
    mut buffer: *const libc::c_char,
    mut codepoint: *mut int32_t,
) -> *const libc::c_char {
    let mut count: libc::c_int = 0;
    let mut value: int32_t = 0;
    if *buffer == 0 {
        return buffer;
    }
    count = utf8_check_first(*buffer.offset(0 as libc::c_int as isize));
    if count <= 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if count == 1 as libc::c_int {
        value = *buffer.offset(0 as libc::c_int as isize) as libc::c_uchar as int32_t;
    } else if utf8_check_full(buffer, count, &mut value) == 0 {
        return 0 as *const libc::c_char
    }
    if !codepoint.is_null() {
        *codepoint = value;
    }
    return buffer.offset(count as isize);
}
pub unsafe extern "C" fn utf8_check_string(
    mut string: *const libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if length == -(1 as libc::c_int) {
        length = strlen(string) as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < length {
        let mut count: libc::c_int = utf8_check_first(*string.offset(i as isize));
        if count == 0 as libc::c_int {
            return 0 as libc::c_int
        } else if count > 1 as libc::c_int {
            if i + count > length {
                return 0 as libc::c_int;
            }
            if utf8_check_full(&*string.offset(i as isize), count, 0 as *mut int32_t)
                == 0
            {
                return 0 as libc::c_int;
            }
            i += count - 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
