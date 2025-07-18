use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _DoubleBitsConv {
    pub bits64: uint64_t,
    pub bits32: [uint32_t; 2],
    pub num: libc::c_double,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
pub unsafe extern "C" fn utilPowerOf2Ceil(mut n: libc::c_int) -> libc::c_int {
    n -= 1;
    n;
    n |= n >> 1 as libc::c_int;
    n |= n >> 2 as libc::c_int;
    n |= n >> 4 as libc::c_int;
    n |= n >> 8 as libc::c_int;
    n |= n >> 16 as libc::c_int;
    n += 1;
    n;
    return n;
}
pub unsafe extern "C" fn utilIsSpace(mut c: libc::c_char) -> bool {
    return c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
        || c as libc::c_int == '\n' as i32 || c as libc::c_int == '\u{b}' as i32;
}
pub unsafe extern "C" fn utilIsName(mut c: libc::c_char) -> bool {
    return 'a' as i32 <= c as libc::c_int && c as libc::c_int <= 'z' as i32
        || 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'Z' as i32
        || c as libc::c_int == '_' as i32;
}
pub unsafe extern "C" fn utilIsDigit(mut c: libc::c_char) -> bool {
    return '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32;
}
pub unsafe extern "C" fn utilIsCharHex(mut c: libc::c_char) -> bool {
    return '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32
        || 'a' as i32 <= c as libc::c_int && c as libc::c_int <= 'z' as i32
        || 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'Z' as i32;
}
pub unsafe extern "C" fn utilCharHexVal(mut c: libc::c_char) -> uint8_t {
    if utilIsCharHex(c) {} else {
        __assert_fail(
            b"utilIsCharHex(c)\0" as *const u8 as *const libc::c_char,
            b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"uint8_t utilCharHexVal(char)\0"))
                .as_ptr(),
        );
    }
    'c_486: {
        if utilIsCharHex(c) {} else {
            __assert_fail(
                b"utilIsCharHex(c)\0" as *const u8 as *const libc::c_char,
                b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"uint8_t utilCharHexVal(char)\0"))
                    .as_ptr(),
            );
        }
    };
    if '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32 {
        return (c as libc::c_int - '0' as i32) as uint8_t
    } else if 'a' as i32 <= c as libc::c_int && c as libc::c_int <= 'z' as i32 {
        return (c as libc::c_int - 'a' as i32 + 10 as libc::c_int) as uint8_t
    } else if 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'Z' as i32 {
        return (c as libc::c_int - 'A' as i32 + 10 as libc::c_int) as uint8_t
    }
    __assert_fail(
        b"false\0" as *const u8 as *const libc::c_char,
        b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
        61 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 29],
            &[libc::c_char; 29],
        >(b"uint8_t utilCharHexVal(char)\0"))
            .as_ptr(),
    );
    'c_361: {
        __assert_fail(
            b"false\0" as *const u8 as *const libc::c_char,
            b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"uint8_t utilCharHexVal(char)\0"))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int as uint8_t;
}
pub unsafe extern "C" fn utilHexDigit(
    mut value: uint8_t,
    mut uppercase: bool,
) -> libc::c_char {
    if 0 as libc::c_int <= value as libc::c_int
        && value as libc::c_int <= 0xf as libc::c_int
    {} else {
        __assert_fail(
            b"_BETWEEN(0x0, value, 0xf)\0" as *const u8 as *const libc::c_char,
            b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"char utilHexDigit(uint8_t, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_581: {
        if 0 as libc::c_int <= value as libc::c_int
            && value as libc::c_int <= 0xf as libc::c_int
        {} else {
            __assert_fail(
                b"_BETWEEN(0x0, value, 0xf)\0" as *const u8 as *const libc::c_char,
                b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"char utilHexDigit(uint8_t, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int <= value as libc::c_int
        && value as libc::c_int <= 9 as libc::c_int
    {
        return ('0' as i32 + value as libc::c_int) as libc::c_char;
    }
    return (if uppercase as libc::c_int != 0 {
        'A' as i32 + (value as libc::c_int - 10 as libc::c_int)
    } else {
        'a' as i32 + (value as libc::c_int - 10 as libc::c_int)
    }) as libc::c_char;
}
pub unsafe extern "C" fn utilDoubleToBits(mut value: libc::c_double) -> uint64_t {
    let mut bits: _DoubleBitsConv = _DoubleBitsConv { bits64: 0 };
    bits.num = value;
    return bits.bits64;
}
pub unsafe extern "C" fn utilDoubleFromBits(mut value: uint64_t) -> libc::c_double {
    let mut bits: _DoubleBitsConv = _DoubleBitsConv { bits64: 0 };
    bits.bits64 = value;
    return bits.num;
}
pub unsafe extern "C" fn utilHashBits(mut hash: uint64_t) -> uint32_t {
    hash = (!hash).wrapping_add(hash << 18 as libc::c_int);
    hash = hash ^ hash >> 31 as libc::c_int;
    hash = hash.wrapping_mul(21 as libc::c_int as libc::c_ulong);
    hash = hash ^ hash >> 11 as libc::c_int;
    hash = hash.wrapping_add(hash << 6 as libc::c_int);
    hash = hash ^ hash >> 22 as libc::c_int;
    return (hash & 0x3fffffff as libc::c_int as libc::c_ulong) as uint32_t;
}
pub unsafe extern "C" fn utilHashNumber(mut num: libc::c_double) -> uint32_t {
    return utilHashBits(utilDoubleToBits(num));
}
pub unsafe extern "C" fn utilHashString(mut string: *const libc::c_char) -> uint32_t {
    let mut hash: uint32_t = 2166136261 as libc::c_uint;
    let mut c: *const libc::c_char = string;
    while *c as libc::c_int != '\0' as i32 {
        hash ^= *c as libc::c_uint;
        hash = (hash as libc::c_uint).wrapping_mul(16777619 as libc::c_uint) as uint32_t
            as uint32_t;
        c = c.offset(1);
        c;
    }
    return hash;
}
pub unsafe extern "C" fn utilToNumber(
    mut str: *const libc::c_char,
    mut num: *mut libc::c_double,
) -> *const libc::c_char {
    if !str.is_null() && !num.is_null() {} else {
        __assert_fail(
            b"(str != NULL) && (num != NULL)\0" as *const u8 as *const libc::c_char,
            b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"const char *utilToNumber(const char *, double *)\0"))
                .as_ptr(),
        );
    }
    'c_1504: {
        if !str.is_null() && !num.is_null() {} else {
            __assert_fail(
                b"(str != NULL) && (num != NULL)\0" as *const u8 as *const libc::c_char,
                b"src/core/utils.c\0" as *const u8 as *const libc::c_char,
                147 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"const char *utilToNumber(const char *, double *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut length: uint32_t = strlen(str) as uint32_t;
    let mut sign: libc::c_double = 1 as libc::c_int as libc::c_double;
    if *str as libc::c_int == '-' as i32 {
        sign = -(1 as libc::c_int) as libc::c_double;
        str = str.offset(1);
        str;
    } else if *str as libc::c_int == '+' as i32 {
        str = str.offset(1);
        str;
    }
    if length >= 3 as libc::c_int as libc::c_uint
        && (strncmp(
            str,
            b"0b\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                str,
                b"0B\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
    {
        let mut bin: uint64_t = 0 as libc::c_int as uint64_t;
        let mut c: *const libc::c_char = str.offset(2 as libc::c_int as isize);
        if *c as libc::c_int == '\0' as i32 {
            return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
        }
        loop {
            if *c as libc::c_int == '\0' as i32 {
                *num = sign * bin as libc::c_double;
                return 0 as *const libc::c_char;
            }
            if !(*c as libc::c_int == '0' as i32 || *c as libc::c_int == '1' as i32) {
                return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
            }
            if c.offset_from(str) as libc::c_long
                > (68 as libc::c_int - 2 as libc::c_int) as libc::c_long
            {
                return b"Binary literal is too long.\0" as *const u8
                    as *const libc::c_char;
            }
            bin = bin << 1 as libc::c_int
                | (*c as libc::c_int - '0' as i32) as libc::c_ulong;
            c = c.offset(1);
            c;
        }
    }
    if length >= 3 as libc::c_int as libc::c_uint
        && (strncmp(
            str,
            b"0x\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                str,
                b"0X\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
    {
        let mut hex: uint64_t = 0 as libc::c_int as uint64_t;
        let mut c_0: *const libc::c_char = str.offset(2 as libc::c_int as isize);
        if *c_0 as libc::c_int == '\0' as i32 {
            return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
        }
        loop {
            if *c_0 as libc::c_int == '\0' as i32 {
                *num = sign * hex as libc::c_double;
                return 0 as *const libc::c_char;
            }
            if !('0' as i32 <= *c_0 as libc::c_int && *c_0 as libc::c_int <= '9' as i32
                || 'a' as i32 <= *c_0 as libc::c_int
                    && *c_0 as libc::c_int <= 'f' as i32)
            {
                return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
            }
            if c_0.offset_from(str) as libc::c_long
                > (20 as libc::c_int - 2 as libc::c_int) as libc::c_long
            {
                return b"Hex literal is too long.\0" as *const u8 as *const libc::c_char;
            }
            let mut digit: uint8_t = (if '0' as i32 <= *c_0 as libc::c_int
                && *c_0 as libc::c_int <= '9' as i32
            {
                (*c_0 as libc::c_int - '0' as i32) as uint8_t as libc::c_int
            } else {
                (*c_0 as libc::c_int - 'a' as i32 + 10 as libc::c_int) as uint8_t
                    as libc::c_int
            }) as uint8_t;
            hex = hex << 4 as libc::c_int | digit as libc::c_ulong;
            c_0 = c_0.offset(1);
            c_0;
        }
    }
    if *str as libc::c_int == '\0' as i32 {
        return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
    }
    let mut c_1: *const libc::c_char = str;
    while '0' as i32 <= *c_1 as libc::c_int && *c_1 as libc::c_int <= '9' as i32 {
        c_1 = c_1.offset(1);
        c_1;
    }
    if *c_1 as libc::c_int == '.' as i32 {
        c_1 = c_1.offset(1);
        c_1;
        while '0' as i32 <= *c_1 as libc::c_int && *c_1 as libc::c_int <= '9' as i32 {
            c_1 = c_1.offset(1);
            c_1;
        }
    }
    if !(*c_1 as libc::c_int == '\0' as i32) {
        if *c_1 as libc::c_int == 'e' as i32 || *c_1 as libc::c_int == 'E' as i32 {
            c_1 = c_1.offset(1);
            c_1;
            if *c_1 as libc::c_int == '+' as i32 || *c_1 as libc::c_int == '-' as i32 {
                c_1 = c_1.offset(1);
                c_1;
            }
            if !('0' as i32 <= *c_1 as libc::c_int && *c_1 as libc::c_int <= '9' as i32)
            {
                return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
            }
            while '0' as i32 <= *c_1 as libc::c_int && *c_1 as libc::c_int <= '9' as i32
            {
                c_1 = c_1.offset(1);
                c_1;
            }
            if *c_1 as libc::c_int != '\0' as i32 {
                return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
            }
        }
        if *c_1 as libc::c_int != '\0' as i32 {
            return b"Invalid numeric string.\0" as *const u8 as *const libc::c_char;
        }
    }
    *__errno_location() = 0 as libc::c_int;
    *num = atof(str) * sign;
    if *__errno_location() == 34 as libc::c_int {
        return b"Numeric string is too long.\0" as *const u8 as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn utf8_encodeBytesCount(mut value: libc::c_int) -> libc::c_int {
    if value <= 0x7f as libc::c_int {
        return 1 as libc::c_int;
    }
    if value <= 0x7ff as libc::c_int {
        return 2 as libc::c_int;
    }
    if value <= 0xffff as libc::c_int {
        return 3 as libc::c_int;
    }
    if value <= 0x10ffff as libc::c_int {
        return 4 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn utf8_decodeBytesCount(mut byte: uint8_t) -> libc::c_int {
    if byte as libc::c_int >> 7 as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if byte as libc::c_int >> 6 as libc::c_int == 0o2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if byte as libc::c_int >> 5 as libc::c_int == 0o6 as libc::c_int {
        return 2 as libc::c_int;
    }
    if byte as libc::c_int >> 4 as libc::c_int == 0o16 as libc::c_int {
        return 3 as libc::c_int;
    }
    if byte as libc::c_int >> 3 as libc::c_int == 0o36 as libc::c_int {
        return 4 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn utf8_encodeValue(
    mut value: libc::c_int,
    mut bytes: *mut uint8_t,
) -> libc::c_int {
    if value <= 0x7f as libc::c_int {
        *bytes = (value & 0x7f as libc::c_int) as uint8_t;
        return 1 as libc::c_int;
    }
    if value <= 0x7ff as libc::c_int {
        let fresh0 = bytes;
        bytes = bytes.offset(1);
        *fresh0 = (0o300 as libc::c_int
            | (value & 0o3700 as libc::c_int) >> 6 as libc::c_int) as uint8_t;
        *bytes = (0o200 as libc::c_int | value & 111111 as libc::c_int) as uint8_t;
        return 2 as libc::c_int;
    }
    if value <= 0xffff as libc::c_int {
        let fresh1 = bytes;
        bytes = bytes.offset(1);
        *fresh1 = (0o340 as libc::c_int
            | (value & 0o170000 as libc::c_int) >> 12 as libc::c_int) as uint8_t;
        let fresh2 = bytes;
        bytes = bytes.offset(1);
        *fresh2 = (0o200 as libc::c_int
            | (value & 0o7700 as libc::c_int) >> 6 as libc::c_int) as uint8_t;
        *bytes = (0o200 as libc::c_int | value & 0o77 as libc::c_int) as uint8_t;
        return 3 as libc::c_int;
    }
    if value <= 0x10ffff as libc::c_int {
        let fresh3 = bytes;
        bytes = bytes.offset(1);
        *fresh3 = (0o360 as libc::c_int
            | (value & (0o7 as libc::c_int) << 18 as libc::c_int) >> 18 as libc::c_int)
            as uint8_t;
        let fresh4 = bytes;
        bytes = bytes.offset(1);
        *fresh4 = (0o200 as libc::c_int
            | (value & (0o77 as libc::c_int) << 12 as libc::c_int) >> 12 as libc::c_int)
            as uint8_t;
        let fresh5 = bytes;
        bytes = bytes.offset(1);
        *fresh5 = (0o200 as libc::c_int
            | (value & (0o77 as libc::c_int) << 6 as libc::c_int) >> 6 as libc::c_int)
            as uint8_t;
        *bytes = (0o200 as libc::c_int | value & 0o77 as libc::c_int) as uint8_t;
        return 4 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn utf8_decodeBytes(
    mut bytes: *mut uint8_t,
    mut value: *mut libc::c_int,
) -> libc::c_int {
    let mut continue_bytes: libc::c_int = 0 as libc::c_int;
    let mut byte_count: libc::c_int = 1 as libc::c_int;
    let mut _value: libc::c_int = 0 as libc::c_int;
    if *bytes as libc::c_int & 0o300 as libc::c_int == 0o200 as libc::c_int {
        *value = *bytes as libc::c_int;
        return byte_count;
    } else if *bytes as libc::c_int & 0o340 as libc::c_int == 0o300 as libc::c_int {
        continue_bytes = 1 as libc::c_int;
        _value = *bytes as libc::c_int & 0o37 as libc::c_int;
    } else if *bytes as libc::c_int & 0o360 as libc::c_int == 0o340 as libc::c_int {
        continue_bytes = 2 as libc::c_int;
        _value = *bytes as libc::c_int & 0o17 as libc::c_int;
    } else if *bytes as libc::c_int & 0o370 as libc::c_int == 0o360 as libc::c_int {
        continue_bytes = 3 as libc::c_int;
        _value = *bytes as libc::c_int & 0o7 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    }
    loop {
        let fresh6 = continue_bytes;
        continue_bytes = continue_bytes - 1;
        if !(fresh6 != 0) {
            break;
        }
        bytes = bytes.offset(1);
        bytes;
        byte_count += 1;
        byte_count;
        if *bytes as libc::c_int & 0o300 as libc::c_int != 0o200 as libc::c_int {
            return -(1 as libc::c_int);
        }
        _value = _value << 6 as libc::c_int
            | *bytes as libc::c_int & 0o77 as libc::c_int;
    }
    *value = _value;
    return byte_count;
}
