use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nettle_write_le32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md4_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
pub unsafe extern "C" fn nettle_md4_init(mut ctx: *mut md4_ctx) {
    let iv: [uint32_t; 4] = [
        0x67452301 as libc::c_int as uint32_t,
        0xefcdab89 as libc::c_uint,
        0x98badcfe as libc::c_uint,
        0x10325476 as libc::c_int as uint32_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        iv.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
    (*ctx).count = 0 as libc::c_int as uint64_t;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_md4_update(
    mut ctx: *mut md4_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block_16: u64;
    if (*ctx).index != 0 {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 64]>()
            as libc::c_ulong)
            .wrapping_sub((*ctx).index as libc::c_ulong) as libc::c_uint;
        if length < __md_left as libc::c_ulong {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx)
                .index = ((*ctx).index as libc::c_ulong).wrapping_add(length)
                as libc::c_uint as libc::c_uint;
            current_block_16 = 13056961889198038528;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            md4_compress(ctx, ((*ctx).block).as_mut_ptr());
            (*ctx).count = ((*ctx).count).wrapping_add(1);
            (*ctx).count;
            data = data.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_16 = 5720623009719927633;
        }
    } else {
        current_block_16 = 5720623009719927633;
    }
    match current_block_16 {
        5720623009719927633 => {
            while length >= ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong {
                md4_compress(ctx, data);
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                (*ctx).count;
                data = data
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong as isize,
                    );
                length = (length as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            memcpy(
                ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx).index = length as libc::c_uint;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn nettle_md4_digest(
    mut ctx: *mut md4_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    let mut data: [uint32_t; 16] = [0; 16];
    let mut i: libc::c_uint = 0;
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= MD4_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"md4.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3511: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= MD4_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"md4.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: libc::c_uint = 0;
    __md_i = (*ctx).index;
    if (__md_i as libc::c_ulong)
        < ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
            b"md4.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3451: {
        if (__md_i as libc::c_ulong)
            < ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
                b"md4.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh0 = __md_i;
    __md_i = __md_i.wrapping_add(1);
    (*ctx).block[fresh0 as usize] = 0x80 as libc::c_int as uint8_t;
    if __md_i as libc::c_ulong
        > (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
                .wrapping_sub(__md_i as libc::c_ulong),
        );
        md4_compress(ctx, ((*ctx).block).as_mut_ptr());
        __md_i = 0 as libc::c_int as libc::c_uint;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(__md_i as libc::c_ulong),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (16 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
        data[i
            as usize] = (*((*ctx).block)
            .as_mut_ptr()
            .offset((4 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*((*ctx).block)
                .as_mut_ptr()
                .offset((4 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize)
                .offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*((*ctx).block)
                .as_mut_ptr()
                .offset((4 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *((*ctx).block)
                .as_mut_ptr()
                .offset((4 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize)
                .offset(0 as libc::c_int as isize) as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
    bit_count = (*ctx).count << 9 as libc::c_int
        | ((*ctx).index << 3 as libc::c_int) as libc::c_ulong;
    data[(16 as libc::c_int - 2 as libc::c_int) as usize] = bit_count as uint32_t;
    data[(16 as libc::c_int - 1 as libc::c_int)
        as usize] = (bit_count >> 32 as libc::c_int) as uint32_t;
    md4_transform(((*ctx).state).as_mut_ptr(), data.as_mut_ptr());
    _nettle_write_le32(length, digest, ((*ctx).state).as_mut_ptr());
    nettle_md4_init(ctx);
}
unsafe extern "C" fn md4_transform(
    mut digest: *mut uint32_t,
    mut data: *const uint32_t,
) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    a = *digest.offset(0 as libc::c_int as isize);
    b = *digest.offset(1 as libc::c_int as isize);
    c = *digest.offset(2 as libc::c_int as isize);
    d = *digest.offset(3 as libc::c_int as isize);
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & !b).wrapping_add(*data.offset(0 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & !a).wrapping_add(*data.offset(1 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & !d).wrapping_add(*data.offset(2 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & !c).wrapping_add(*data.offset(3 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & !b).wrapping_add(*data.offset(4 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & !a).wrapping_add(*data.offset(5 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & !d).wrapping_add(*data.offset(6 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & !c).wrapping_add(*data.offset(7 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & !b).wrapping_add(*data.offset(8 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & !a).wrapping_add(*data.offset(9 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & !d).wrapping_add(*data.offset(10 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & !c).wrapping_add(*data.offset(11 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & !b).wrapping_add(*data.offset(12 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & !a).wrapping_add(*data.offset(13 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & !d).wrapping_add(*data.offset(14 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & !c).wrapping_add(*data.offset(15 as libc::c_int as isize)),
        ) as uint32_t as uint32_t;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(0 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(4 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(8 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(12 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(1 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(5 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(9 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(13 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(2 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(6 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(10 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(14 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(3 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(7 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(11 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(15 as libc::c_int as isize))
                .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(0 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(8 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(4 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(12 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(2 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(10 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(6 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(14 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(1 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(9 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(5 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(13 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(3 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(11 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(7 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(15 as libc::c_int as isize))
                .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    let ref mut fresh1 = *digest.offset(0 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh2 = *digest.offset(1 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh3 = *digest.offset(2 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh4 = *digest.offset(3 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
}
unsafe extern "C" fn md4_compress(mut ctx: *mut md4_ctx, mut block: *const uint8_t) {
    let mut data: [uint32_t; 16] = [0; 16];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        data[i
            as usize] = (*block.offset(3 as libc::c_int as isize) as uint32_t)
            << 24 as libc::c_int
            | (*block.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*block.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *block.offset(0 as libc::c_int as isize) as uint32_t;
        i = i.wrapping_add(1);
        i;
        block = block.offset(4 as libc::c_int as isize);
    }
    md4_transform(((*ctx).state).as_mut_ptr(), data.as_mut_ptr());
}
