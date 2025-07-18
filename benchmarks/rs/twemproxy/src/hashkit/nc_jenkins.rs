use ::libc;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ptr: *const libc::c_void,
    pub i: size_t,
}
pub unsafe extern "C" fn hash_jenkins(
    mut key: *const libc::c_char,
    mut length: size_t,
) -> uint32_t {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut u: C2RustUnnamed = C2RustUnnamed {
        ptr: 0 as *const libc::c_void,
    };
    c = (0xdeadbeef as libc::c_uint)
        .wrapping_add(length as uint32_t)
        .wrapping_add(13 as libc::c_int as libc::c_uint);
    b = c;
    a = b;
    u.ptr = key as *const libc::c_void;
    if u.i & 0x3 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong {
        let mut k: *const uint32_t = key as *const uint32_t;
        while length > 12 as libc::c_int as libc::c_ulong {
            a = (a as libc::c_uint).wrapping_add(*k.offset(0 as libc::c_int as isize))
                as uint32_t as uint32_t;
            b = (b as libc::c_uint).wrapping_add(*k.offset(1 as libc::c_int as isize))
                as uint32_t as uint32_t;
            c = (c as libc::c_uint).wrapping_add(*k.offset(2 as libc::c_int as isize))
                as uint32_t as uint32_t;
            a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int;
            c = (c as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
            a = (a as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 8 as libc::c_int | b >> 32 as libc::c_int - 8 as libc::c_int;
            b = (b as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
            c = (c as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int;
            a = (a as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 4 as libc::c_int | b >> 32 as libc::c_int - 4 as libc::c_int;
            b = (b as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            length = (length as libc::c_ulong)
                .wrapping_sub(12 as libc::c_int as libc::c_ulong) as size_t as size_t;
            k = k.offset(3 as libc::c_int as isize);
        }
        match length {
            12 => {
                c = (c as libc::c_uint)
                    .wrapping_add(*k.offset(2 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
                b = (b as libc::c_uint)
                    .wrapping_add(*k.offset(1 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            11 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        *k.offset(2 as libc::c_int as isize)
                            & 0xffffff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
                b = (b as libc::c_uint)
                    .wrapping_add(*k.offset(1 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            10 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        *k.offset(2 as libc::c_int as isize)
                            & 0xffff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
                b = (b as libc::c_uint)
                    .wrapping_add(*k.offset(1 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            9 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        *k.offset(2 as libc::c_int as isize)
                            & 0xff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
                b = (b as libc::c_uint)
                    .wrapping_add(*k.offset(1 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            8 => {
                b = (b as libc::c_uint)
                    .wrapping_add(*k.offset(1 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            7 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        *k.offset(1 as libc::c_int as isize)
                            & 0xffffff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            6 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        *k.offset(1 as libc::c_int as isize)
                            & 0xffff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            5 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        *k.offset(1 as libc::c_int as isize)
                            & 0xff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            4 => {
                a = (a as libc::c_uint)
                    .wrapping_add(*k.offset(0 as libc::c_int as isize)) as uint32_t
                    as uint32_t;
            }
            3 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        *k.offset(0 as libc::c_int as isize)
                            & 0xffffff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
            }
            2 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        *k.offset(0 as libc::c_int as isize)
                            & 0xffff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
            }
            1 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        *k.offset(0 as libc::c_int as isize)
                            & 0xff as libc::c_int as libc::c_uint,
                    ) as uint32_t as uint32_t;
            }
            0 => return c,
            _ => return c,
        }
    } else if u.i & 0x1 as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        let mut k_0: *const uint16_t = key as *const uint16_t;
        let mut k8: *const uint8_t = 0 as *const uint8_t;
        while length > 12 as libc::c_int as libc::c_ulong {
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*k_0.offset(1 as libc::c_int as isize) as uint32_t)
                                << 16 as libc::c_int,
                        ),
                ) as uint32_t as uint32_t;
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*k_0.offset(3 as libc::c_int as isize) as uint32_t)
                                << 16 as libc::c_int,
                        ),
                ) as uint32_t as uint32_t;
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k_0.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*k_0.offset(5 as libc::c_int as isize) as uint32_t)
                                << 16 as libc::c_int,
                        ),
                ) as uint32_t as uint32_t;
            a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int;
            c = (c as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
            a = (a as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 8 as libc::c_int | b >> 32 as libc::c_int - 8 as libc::c_int;
            b = (b as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
            c = (c as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int;
            a = (a as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 4 as libc::c_int | b >> 32 as libc::c_int - 4 as libc::c_int;
            b = (b as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            length = (length as libc::c_ulong)
                .wrapping_sub(12 as libc::c_int as libc::c_ulong) as size_t as size_t;
            k_0 = k_0.offset(6 as libc::c_int as isize);
        }
        k8 = k_0 as *const uint8_t;
        let mut current_block_104: u64;
        match length {
            12 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(4 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(5 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
                b = (b as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(3 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(1 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
                current_block_104 = 8102658916883067714;
            }
            11 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        (*k8.offset(10 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_104 = 4548594400583410787;
            }
            10 => {
                current_block_104 = 4548594400583410787;
            }
            9 => {
                c = (c as libc::c_uint)
                    .wrapping_add(*k8.offset(8 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
                current_block_104 = 1677079002017831167;
            }
            8 => {
                current_block_104 = 1677079002017831167;
            }
            7 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        (*k8.offset(6 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_104 = 13123768761735600930;
            }
            6 => {
                current_block_104 = 13123768761735600930;
            }
            5 => {
                b = (b as libc::c_uint)
                    .wrapping_add(*k8.offset(4 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
                current_block_104 = 6801053210060364487;
            }
            4 => {
                current_block_104 = 6801053210060364487;
            }
            3 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k8.offset(2 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_104 = 14111752540665342400;
            }
            2 => {
                current_block_104 = 14111752540665342400;
            }
            1 => {
                a = (a as libc::c_uint)
                    .wrapping_add(*k8.offset(0 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
                current_block_104 = 8102658916883067714;
            }
            0 => return c,
            _ => return c,
        }
        match current_block_104 {
            13123768761735600930 => {
                b = (b as libc::c_uint)
                    .wrapping_add(*k_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(1 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
            }
            1677079002017831167 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(3 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(1 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
            }
            4548594400583410787 => {
                c = (c as libc::c_uint)
                    .wrapping_add(*k_0.offset(4 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
                b = (b as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(3 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(1 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
            }
            6801053210060364487 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*k_0.offset(1 as libc::c_int as isize) as uint32_t)
                                    << 16 as libc::c_int,
                            ),
                    ) as uint32_t as uint32_t;
            }
            14111752540665342400 => {
                a = (a as libc::c_uint)
                    .wrapping_add(*k_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
            }
            _ => {}
        }
    } else {
        let mut k_1: *const uint8_t = key as *const uint8_t;
        while length > 12 as libc::c_int as libc::c_ulong {
            a = (a as libc::c_uint)
                .wrapping_add(*k_1.offset(0 as libc::c_int as isize) as libc::c_uint)
                as uint32_t as uint32_t;
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(1 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int,
                ) as uint32_t as uint32_t;
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(2 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int,
                ) as uint32_t as uint32_t;
            a = (a as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(3 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int,
                ) as uint32_t as uint32_t;
            b = (b as libc::c_uint)
                .wrapping_add(*k_1.offset(4 as libc::c_int as isize) as libc::c_uint)
                as uint32_t as uint32_t;
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(5 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int,
                ) as uint32_t as uint32_t;
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(6 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int,
                ) as uint32_t as uint32_t;
            b = (b as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(7 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int,
                ) as uint32_t as uint32_t;
            c = (c as libc::c_uint)
                .wrapping_add(*k_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                as uint32_t as uint32_t;
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(9 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int,
                ) as uint32_t as uint32_t;
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(10 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int,
                ) as uint32_t as uint32_t;
            c = (c as libc::c_uint)
                .wrapping_add(
                    (*k_1.offset(11 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int,
                ) as uint32_t as uint32_t;
            a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int;
            c = (c as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
            a = (a as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 8 as libc::c_int | b >> 32 as libc::c_int - 8 as libc::c_int;
            b = (b as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as libc::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
            c = (c as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as libc::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 19 as libc::c_int | a >> 32 as libc::c_int - 19 as libc::c_int;
            a = (a as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as libc::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 4 as libc::c_int | b >> 32 as libc::c_int - 4 as libc::c_int;
            b = (b as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            length = (length as libc::c_ulong)
                .wrapping_sub(12 as libc::c_int as libc::c_ulong) as size_t as size_t;
            k_1 = k_1.offset(12 as libc::c_int as isize);
        }
        let mut current_block_156: u64;
        match length {
            12 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(11 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 2253044032609351678;
            }
            11 => {
                current_block_156 = 2253044032609351678;
            }
            10 => {
                current_block_156 = 18308612092773212808;
            }
            9 => {
                current_block_156 = 4577159841354843635;
            }
            8 => {
                current_block_156 = 18268539312520417794;
            }
            7 => {
                current_block_156 = 6585473732656942518;
            }
            6 => {
                current_block_156 = 9959828968968616484;
            }
            5 => {
                current_block_156 = 6098740501232225045;
            }
            4 => {
                current_block_156 = 17065436883805192427;
            }
            3 => {
                current_block_156 = 8079634320966389348;
            }
            2 => {
                current_block_156 = 11468061896745076941;
            }
            1 => {
                current_block_156 = 16245293535968970436;
            }
            0 => return c,
            _ => return c,
        }
        match current_block_156 {
            2253044032609351678 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(10 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 18308612092773212808;
            }
            _ => {}
        }
        match current_block_156 {
            18308612092773212808 => {
                c = (c as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(9 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 4577159841354843635;
            }
            _ => {}
        }
        match current_block_156 {
            4577159841354843635 => {
                c = (c as libc::c_uint)
                    .wrapping_add(*k_1.offset(8 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
                current_block_156 = 18268539312520417794;
            }
            _ => {}
        }
        match current_block_156 {
            18268539312520417794 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(7 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 6585473732656942518;
            }
            _ => {}
        }
        match current_block_156 {
            6585473732656942518 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(6 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 9959828968968616484;
            }
            _ => {}
        }
        match current_block_156 {
            9959828968968616484 => {
                b = (b as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(5 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 6098740501232225045;
            }
            _ => {}
        }
        match current_block_156 {
            6098740501232225045 => {
                b = (b as libc::c_uint)
                    .wrapping_add(*k_1.offset(4 as libc::c_int as isize) as libc::c_uint)
                    as uint32_t as uint32_t;
                current_block_156 = 17065436883805192427;
            }
            _ => {}
        }
        match current_block_156 {
            17065436883805192427 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(3 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 8079634320966389348;
            }
            _ => {}
        }
        match current_block_156 {
            8079634320966389348 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(2 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int,
                    ) as uint32_t as uint32_t;
                current_block_156 = 11468061896745076941;
            }
            _ => {}
        }
        match current_block_156 {
            11468061896745076941 => {
                a = (a as libc::c_uint)
                    .wrapping_add(
                        (*k_1.offset(1 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int,
                    ) as uint32_t as uint32_t;
            }
            _ => {}
        }
        a = (a as libc::c_uint)
            .wrapping_add(*k_1.offset(0 as libc::c_int as isize) as libc::c_uint)
            as uint32_t as uint32_t;
    }
    c ^= b;
    c = (c as libc::c_uint)
        .wrapping_sub(
            b << 14 as libc::c_int | b >> 32 as libc::c_int - 14 as libc::c_int,
        ) as uint32_t as uint32_t;
    a ^= c;
    a = (a as libc::c_uint)
        .wrapping_sub(
            c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int,
        ) as uint32_t as uint32_t;
    b ^= a;
    b = (b as libc::c_uint)
        .wrapping_sub(
            a << 25 as libc::c_int | a >> 32 as libc::c_int - 25 as libc::c_int,
        ) as uint32_t as uint32_t;
    c ^= b;
    c = (c as libc::c_uint)
        .wrapping_sub(
            b << 16 as libc::c_int | b >> 32 as libc::c_int - 16 as libc::c_int,
        ) as uint32_t as uint32_t;
    a ^= c;
    a = (a as libc::c_uint)
        .wrapping_sub(c << 4 as libc::c_int | c >> 32 as libc::c_int - 4 as libc::c_int)
        as uint32_t as uint32_t;
    b ^= a;
    b = (b as libc::c_uint)
        .wrapping_sub(
            a << 14 as libc::c_int | a >> 32 as libc::c_int - 14 as libc::c_int,
        ) as uint32_t as uint32_t;
    c ^= b;
    c = (c as libc::c_uint)
        .wrapping_sub(
            b << 24 as libc::c_int | b >> 32 as libc::c_int - 24 as libc::c_int,
        ) as uint32_t as uint32_t;
    return c;
}
