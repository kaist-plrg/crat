use ::libc;
extern "C" {
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
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memeql_sec(
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ocb_key {
    pub L: [nettle_block16; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ocb_ctx {
    pub initial: nettle_block16,
    pub offset: nettle_block16,
    pub sum: nettle_block16,
    pub checksum: nettle_block16,
    pub data_count: size_t,
    pub message_count: size_t,
}
#[inline]
unsafe extern "C" fn block16_mulx_be(
    mut dst: *mut nettle_block16,
    mut src: *const nettle_block16,
) {
    let mut carry: uint64_t = ((*src).u64_0[0 as libc::c_int as usize]
        & 0x80 as libc::c_int as libc::c_ulong) >> 7 as libc::c_int;
    (*dst)
        .u64_0[0 as libc::c_int
        as usize] = ((*src).u64_0[0 as libc::c_int as usize]
        & 0x7f7f7f7f7f7f7f7f as libc::c_ulong) << 1 as libc::c_int
        | ((*src).u64_0[0 as libc::c_int as usize] & 0x8080808080808080 as libc::c_ulong)
            >> 15 as libc::c_int
        | ((*src).u64_0[1 as libc::c_int as usize]
            & 0x80 as libc::c_int as libc::c_ulong) << 49 as libc::c_int;
    (*dst)
        .u64_0[1 as libc::c_int
        as usize] = (((*src).u64_0[1 as libc::c_int as usize]
        & 0x7f7f7f7f7f7f7f7f as libc::c_ulong) << 1 as libc::c_int
        | ((*src).u64_0[1 as libc::c_int as usize] & 0x8080808080808080 as libc::c_ulong)
            >> 15 as libc::c_int)
        ^ (0x87 as libc::c_ulong) << 56 as libc::c_int & carry.wrapping_neg();
}
#[inline]
unsafe extern "C" fn block16_xor(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as libc::c_int as usize] ^= (*x).u64_0[0 as libc::c_int as usize];
    (*r).u64_0[1 as libc::c_int as usize] ^= (*x).u64_0[1 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn block16_xor3(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
    mut y: *const nettle_block16,
) {
    (*r)
        .u64_0[0 as libc::c_int
        as usize] = (*x).u64_0[0 as libc::c_int as usize]
        ^ (*y).u64_0[0 as libc::c_int as usize];
    (*r)
        .u64_0[1 as libc::c_int
        as usize] = (*x).u64_0[1 as libc::c_int as usize]
        ^ (*y).u64_0[1 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn block16_set(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as libc::c_int as usize] = (*x).u64_0[0 as libc::c_int as usize];
    (*r).u64_0[1 as libc::c_int as usize] = (*x).u64_0[1 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn extract(
    mut u0: uint64_t,
    mut u1: uint64_t,
    mut offset: libc::c_uint,
) -> uint64_t {
    if offset == 0 as libc::c_int as libc::c_uint {
        return u0;
    }
    u0 = u0.swap_bytes();
    u1 = u1.swap_bytes();
    return (u0 << offset
        | u1 >> (64 as libc::c_int as libc::c_uint).wrapping_sub(offset))
        .swap_bytes();
}
pub unsafe extern "C" fn nettle_ocb_set_key(
    mut key: *mut ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        ((*key).L[0 as libc::c_int as usize].b).as_mut_ptr(),
        (zero_block.b).as_ptr(),
    );
    block16_mulx_be(
        &mut *((*key).L).as_mut_ptr().offset(1 as libc::c_int as isize),
        &mut *((*key).L).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    block16_mulx_be(
        &mut *((*key).L).as_mut_ptr().offset(2 as libc::c_int as isize),
        &mut *((*key).L).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
}
unsafe extern "C" fn update_offset(
    mut key: *const ocb_key,
    mut offset: *mut nettle_block16,
    mut i: size_t,
) {
    if i & 1 as libc::c_int as libc::c_ulong != 0 {
        block16_xor(offset, &*((*key).L).as_ptr().offset(2 as libc::c_int as isize));
    } else {
        if i > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"i > 0\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void update_offset(const struct ocb_key *, union nettle_block16 *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_1707: {
            if i > 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"i > 0\0" as *const u8 as *const libc::c_char,
                    b"ocb.c\0" as *const u8 as *const libc::c_char,
                    76 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"void update_offset(const struct ocb_key *, union nettle_block16 *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut diff: nettle_block16 = nettle_block16 { b: [0; 16] };
        block16_mulx_be(
            &mut diff,
            &*((*key).L).as_ptr().offset(2 as libc::c_int as isize),
        );
        i >>= 1 as libc::c_int;
        while i & 1 as libc::c_int as libc::c_ulong == 0 {
            block16_mulx_be(&mut diff, &mut diff);
            i >>= 1 as libc::c_int;
        }
        block16_xor(offset, &mut diff);
    };
}
unsafe extern "C" fn pad_block(
    mut block: *mut nettle_block16,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    memcpy(
        ((*block).b).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length,
    );
    (*block).b[length as usize] = 0x80 as libc::c_int as uint8_t;
    memset(
        ((*block).b)
            .as_mut_ptr()
            .offset(length as isize)
            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        ((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong).wrapping_sub(length),
    );
}
pub unsafe extern "C" fn nettle_ocb_set_nonce(
    mut ctx: *mut ocb_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut tag_length: size_t,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    let mut top: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut stretch: uint64_t = 0;
    let mut bottom: libc::c_uint = 0;
    if nonce_length < 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nonce_length < 16\0" as *const u8 as *const libc::c_char,
            b"ocb.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1206: {
        if nonce_length < 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"nonce_length < 16\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if tag_length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"tag_length > 0\0" as *const u8 as *const libc::c_char,
            b"ocb.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1167: {
        if tag_length > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"tag_length > 0\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if tag_length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"tag_length <= 16\0" as *const u8 as *const libc::c_char,
            b"ocb.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1118: {
        if tag_length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"tag_length <= 16\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    top
        .b[0 as libc::c_int
        as usize] = ((tag_length & 15 as libc::c_int as libc::c_ulong)
        << 4 as libc::c_int) as uint8_t;
    memset(
        (top.b).as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (15 as libc::c_int as libc::c_ulong).wrapping_sub(nonce_length),
    );
    top
        .b[(15 as libc::c_int as libc::c_ulong).wrapping_sub(nonce_length)
        as usize] = (top
        .b[(15 as libc::c_int as libc::c_ulong).wrapping_sub(nonce_length) as usize]
        as libc::c_int | 1 as libc::c_int) as uint8_t;
    memcpy(
        (top.b)
            .as_mut_ptr()
            .offset(16 as libc::c_int as isize)
            .offset(-(nonce_length as isize)) as *mut libc::c_void,
        nonce as *const libc::c_void,
        nonce_length,
    );
    bottom = (top.b[15 as libc::c_int as usize] as libc::c_int & 0x3f as libc::c_int)
        as libc::c_uint;
    top
        .b[15 as libc::c_int
        as usize] = (top.b[15 as libc::c_int as usize] as libc::c_int
        & 0xc0 as libc::c_int) as uint8_t;
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        (top.b).as_mut_ptr(),
        (top.b).as_mut_ptr(),
    );
    stretch = top.u64_0[0 as libc::c_int as usize];
    stretch
        ^= top.u64_0[0 as libc::c_int as usize] >> 8 as libc::c_int
            | top.u64_0[1 as libc::c_int as usize] << 56 as libc::c_int;
    (*ctx)
        .initial
        .u64_0[0 as libc::c_int
        as usize] = extract(
        top.u64_0[0 as libc::c_int as usize],
        top.u64_0[1 as libc::c_int as usize],
        bottom,
    );
    (*ctx)
        .initial
        .u64_0[1 as libc::c_int
        as usize] = extract(top.u64_0[1 as libc::c_int as usize], stretch, bottom);
    (*ctx).sum.u64_0[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*ctx)
        .sum
        .u64_0[0 as libc::c_int as usize] = (*ctx).sum.u64_0[1 as libc::c_int as usize];
    (*ctx).checksum.u64_0[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*ctx)
        .checksum
        .u64_0[0 as libc::c_int
        as usize] = (*ctx).checksum.u64_0[1 as libc::c_int as usize];
    (*ctx).message_count = 0 as libc::c_int as size_t;
    (*ctx).data_count = (*ctx).message_count;
}
unsafe extern "C" fn ocb_fill_n(
    mut key: *const ocb_key,
    mut offset: *mut nettle_block16,
    mut count: size_t,
    mut n: size_t,
    mut o: *mut nettle_block16,
) {
    if n > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"ocb.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"void ocb_fill_n(const struct ocb_key *, union nettle_block16 *, size_t, size_t, union nettle_block16 *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1990: {
        if n > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                138 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"void ocb_fill_n(const struct ocb_key *, union nettle_block16 *, size_t, size_t, union nettle_block16 *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut prev: *mut nettle_block16 = 0 as *mut nettle_block16;
    if count & 1 as libc::c_int as libc::c_ulong != 0 {
        prev = offset;
    } else {
        count = count.wrapping_add(1);
        count;
        block16_xor(offset, &*((*key).L).as_ptr().offset(2 as libc::c_int as isize));
        block16_set(&mut *o.offset(0 as libc::c_int as isize), offset);
        prev = o;
        n = n.wrapping_sub(1);
        n;
        o = o.offset(1);
        o;
    }
    while n >= 2 as libc::c_int as libc::c_ulong {
        let mut i: size_t = 0;
        count = (count as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        block16_mulx_be(
            &mut *o.offset(0 as libc::c_int as isize),
            &*((*key).L).as_ptr().offset(2 as libc::c_int as isize),
        );
        i = count >> 1 as libc::c_int;
        while i & 1 as libc::c_int as libc::c_ulong == 0 {
            block16_mulx_be(
                &mut *o.offset(0 as libc::c_int as isize),
                &mut *o.offset(0 as libc::c_int as isize),
            );
            i >>= 1 as libc::c_int;
        }
        block16_xor(&mut *o.offset(0 as libc::c_int as isize), prev);
        block16_xor3(
            &mut *o.offset(1 as libc::c_int as isize),
            &mut *o.offset(0 as libc::c_int as isize),
            &*((*key).L).as_ptr().offset(2 as libc::c_int as isize),
        );
        prev = &mut *o.offset(1 as libc::c_int as isize) as *mut nettle_block16;
        n = (n as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        o = o.offset(2 as libc::c_int as isize);
    }
    block16_set(offset, prev);
    if n > 0 as libc::c_int as libc::c_ulong {
        count = count.wrapping_add(1);
        update_offset(key, offset, count);
        block16_set(o, offset);
    }
}
pub unsafe extern "C" fn nettle_ocb_update(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut block: [nettle_block16; 16] = [nettle_block16 { b: [0; 16] }; 16];
    let mut n: size_t = length.wrapping_div(16 as libc::c_int as libc::c_ulong);
    if (*ctx).message_count == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ctx->message_count == 0\0" as *const u8 as *const libc::c_char,
            b"ocb.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 126],
                &[libc::c_char; 126],
            >(
                b"void nettle_ocb_update(struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2059: {
        if (*ctx).message_count == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ctx->message_count == 0\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                183 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 126],
                    &[libc::c_char; 126],
                >(
                    b"void nettle_ocb_update(struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).data_count == 0 as libc::c_int as libc::c_ulong {
        (*ctx).offset.u64_0[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
        (*ctx)
            .offset
            .u64_0[0 as libc::c_int
            as usize] = (*ctx).offset.u64_0[1 as libc::c_int as usize];
    }
    while n > 0 as libc::c_int as libc::c_ulong {
        let mut size: size_t = 0;
        let mut i: size_t = 0;
        let mut blocks: size_t = if n <= 16 as libc::c_int as libc::c_ulong {
            n
        } else {
            ((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_add((*ctx).data_count & 1 as libc::c_int as libc::c_ulong)
        };
        ocb_fill_n(
            key,
            &mut (*ctx).offset,
            (*ctx).data_count,
            blocks,
            block.as_mut_ptr(),
        );
        (*ctx)
            .data_count = ((*ctx).data_count as libc::c_ulong).wrapping_add(blocks)
            as size_t as size_t;
        size = blocks.wrapping_mul(16 as libc::c_int as libc::c_ulong);
        nettle_memxor(
            (block[0 as libc::c_int as usize].b).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            size,
        );
        f
            .unwrap()(
            cipher,
            size,
            (block[0 as libc::c_int as usize].b).as_mut_ptr(),
            (block[0 as libc::c_int as usize].b).as_mut_ptr(),
        );
        i = 0 as libc::c_int as size_t;
        while i < blocks {
            block16_xor(&mut (*ctx).sum, &mut *block.as_mut_ptr().offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        n = (n as libc::c_ulong).wrapping_sub(blocks) as size_t as size_t;
        data = data.offset(size as isize);
    }
    length &= 15 as libc::c_int as libc::c_ulong;
    if length > 0 as libc::c_int as libc::c_ulong {
        let mut block_0: nettle_block16 = nettle_block16 { b: [0; 16] };
        pad_block(&mut block_0, length, data);
        block16_xor(
            &mut (*ctx).offset,
            &*((*key).L).as_ptr().offset(0 as libc::c_int as isize),
        );
        block16_xor(&mut block_0, &mut (*ctx).offset);
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            (block_0.b).as_mut_ptr(),
            (block_0.b).as_mut_ptr(),
        );
        block16_xor(&mut (*ctx).sum, &mut block_0);
    }
}
unsafe extern "C" fn ocb_crypt_n(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut n: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut o: [nettle_block16; 16] = [nettle_block16 { b: [0; 16] }; 16];
    let mut block: [nettle_block16; 16] = [nettle_block16 { b: [0; 16] }; 16];
    let mut size: size_t = 0;
    while n > 0 as libc::c_int as libc::c_ulong {
        let mut blocks: size_t = if n <= 16 as libc::c_int as libc::c_ulong {
            n
        } else {
            ((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_add((*ctx).message_count & 1 as libc::c_int as libc::c_ulong)
        };
        ocb_fill_n(
            key,
            &mut (*ctx).offset,
            (*ctx).message_count,
            blocks,
            o.as_mut_ptr(),
        );
        (*ctx)
            .message_count = ((*ctx).message_count as libc::c_ulong).wrapping_add(n)
            as size_t as size_t;
        size = blocks.wrapping_mul(16 as libc::c_int as libc::c_ulong);
        nettle_memxor3(
            (block[0 as libc::c_int as usize].b).as_mut_ptr() as *mut libc::c_void,
            (o[0 as libc::c_int as usize].b).as_mut_ptr() as *const libc::c_void,
            src as *const libc::c_void,
            size,
        );
        f
            .unwrap()(
            cipher,
            size,
            (block[0 as libc::c_int as usize].b).as_mut_ptr(),
            (block[0 as libc::c_int as usize].b).as_mut_ptr(),
        );
        nettle_memxor3(
            dst as *mut libc::c_void,
            (block[0 as libc::c_int as usize].b).as_mut_ptr() as *const libc::c_void,
            (o[0 as libc::c_int as usize].b).as_mut_ptr() as *const libc::c_void,
            size,
        );
        n = (n as libc::c_ulong).wrapping_sub(blocks) as size_t as size_t;
        src = src.offset(size as isize);
        dst = dst.offset(-(size as isize));
    }
}
unsafe extern "C" fn ocb_checksum_n(
    mut checksum: *mut nettle_block16,
    mut n: size_t,
    mut src: *const uint8_t,
) {
    let mut initial: libc::c_uint = 0;
    let mut edge_word: uint64_t = 0 as libc::c_int as uint64_t;
    let mut s0: uint64_t = 0;
    let mut s1: uint64_t = 0;
    if n == 1 as libc::c_int as libc::c_ulong {
        nettle_memxor(
            ((*checksum).b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        return;
    }
    initial = ((src as uintptr_t).wrapping_neg() & 7 as libc::c_int as libc::c_ulong)
        as libc::c_uint;
    if initial > 0 as libc::c_int as libc::c_uint {
        let mut i: libc::c_uint = 0;
        i = initial;
        while i > 0 as libc::c_int as libc::c_uint {
            let fresh0 = src;
            src = src.offset(1);
            edge_word = (edge_word << 8 as libc::c_int)
                .wrapping_add(*fresh0 as libc::c_ulong);
            i = i.wrapping_sub(1);
            i;
        }
        n = n.wrapping_sub(1);
        n;
    }
    s1 = 0 as libc::c_int as uint64_t;
    s0 = s1;
    while n > 0 as libc::c_int as libc::c_ulong {
        s0 ^= *(src as *const uint64_t).offset(0 as libc::c_int as isize);
        s1 ^= *(src as *const uint64_t).offset(1 as libc::c_int as isize);
        n = n.wrapping_sub(1);
        n;
        src = src.offset(16 as libc::c_int as isize);
    }
    if initial > 0 as libc::c_int as libc::c_uint {
        let mut i_0: libc::c_uint = 0;
        let mut mask: uint64_t = 0;
        s0 ^= *(src as *const uint64_t).offset(0 as libc::c_int as isize);
        i_0 = (8 as libc::c_int as libc::c_uint).wrapping_sub(initial);
        src = src.offset(8 as libc::c_int as isize);
        while i_0 > 0 as libc::c_int as libc::c_uint {
            let fresh1 = src;
            src = src.offset(1);
            edge_word = (edge_word << 8 as libc::c_int)
                .wrapping_add(*fresh1 as libc::c_ulong);
            i_0 = i_0.wrapping_sub(1);
            i_0;
        }
        let mut __rotate_t: uint64_t = s0
            << (8 as libc::c_int as libc::c_uint).wrapping_mul(initial)
            | s1
                >> (64 as libc::c_int as libc::c_uint)
                    .wrapping_sub(
                        (8 as libc::c_int as libc::c_uint).wrapping_mul(initial),
                    );
        s1 = s1 << (8 as libc::c_int as libc::c_uint).wrapping_mul(initial)
            | s0
                >> (64 as libc::c_int as libc::c_uint)
                    .wrapping_sub(
                        (8 as libc::c_int as libc::c_uint).wrapping_mul(initial),
                    );
        s0 = __rotate_t;
        mask = ((1 as libc::c_int as uint64_t)
            << (8 as libc::c_int as libc::c_uint).wrapping_mul(initial))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        edge_word = edge_word.swap_bytes();
        s0 ^= edge_word & mask;
        s1 ^= edge_word & !mask;
    }
    (*checksum).u64_0[0 as libc::c_int as usize] ^= s0;
    (*checksum).u64_0[1 as libc::c_int as usize] ^= s1;
}
pub unsafe extern "C" fn nettle_ocb_encrypt(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut n: size_t = length.wrapping_div(16 as libc::c_int as libc::c_ulong);
    if (*ctx).message_count == 0 as libc::c_int as libc::c_ulong {
        (*ctx).offset = (*ctx).initial;
    }
    if n > 0 as libc::c_int as libc::c_ulong {
        ocb_checksum_n(&mut (*ctx).checksum, n, src);
        ocb_crypt_n(ctx, key, cipher, f, n, dst, src);
        length &= 15 as libc::c_int as libc::c_ulong;
    }
    if length > 0 as libc::c_int as libc::c_ulong {
        let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
        src = src.offset(n.wrapping_mul(16 as libc::c_int as libc::c_ulong) as isize);
        dst = dst.offset(n.wrapping_mul(16 as libc::c_int as libc::c_ulong) as isize);
        pad_block(&mut block, length, src);
        block16_xor(&mut (*ctx).checksum, &mut block);
        block16_xor(
            &mut (*ctx).offset,
            &*((*key).L).as_ptr().offset(0 as libc::c_int as isize),
        );
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            (block.b).as_mut_ptr(),
            ((*ctx).offset.b).as_mut_ptr(),
        );
        nettle_memxor3(
            dst as *mut libc::c_void,
            (block.b).as_mut_ptr() as *const libc::c_void,
            src as *const libc::c_void,
            length,
        );
        (*ctx).message_count = ((*ctx).message_count).wrapping_add(1);
        (*ctx).message_count;
    }
}
pub unsafe extern "C" fn nettle_ocb_decrypt(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut encrypt_ctx: *const libc::c_void,
    mut encrypt: Option::<nettle_cipher_func>,
    mut decrypt_ctx: *const libc::c_void,
    mut decrypt: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut n: size_t = length.wrapping_div(16 as libc::c_int as libc::c_ulong);
    if (*ctx).message_count == 0 as libc::c_int as libc::c_ulong {
        (*ctx).offset = (*ctx).initial;
    }
    if n > 0 as libc::c_int as libc::c_ulong {
        ocb_crypt_n(ctx, key, decrypt_ctx, decrypt, n, dst, src);
        ocb_checksum_n(&mut (*ctx).checksum, n, dst);
        length &= 15 as libc::c_int as libc::c_ulong;
    }
    if length > 0 as libc::c_int as libc::c_ulong {
        let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
        src = src.offset(n.wrapping_mul(16 as libc::c_int as libc::c_ulong) as isize);
        dst = dst.offset(n.wrapping_mul(16 as libc::c_int as libc::c_ulong) as isize);
        block16_xor(
            &mut (*ctx).offset,
            &*((*key).L).as_ptr().offset(0 as libc::c_int as isize),
        );
        encrypt
            .unwrap()(
            encrypt_ctx,
            16 as libc::c_int as size_t,
            (block.b).as_mut_ptr(),
            ((*ctx).offset.b).as_mut_ptr(),
        );
        nettle_memxor3(
            dst as *mut libc::c_void,
            (block.b).as_mut_ptr() as *const libc::c_void,
            src as *const libc::c_void,
            length,
        );
        pad_block(&mut block, length, dst);
        block16_xor(&mut (*ctx).checksum, &mut block);
        (*ctx).message_count = ((*ctx).message_count).wrapping_add(1);
        (*ctx).message_count;
    }
}
pub unsafe extern "C" fn nettle_ocb_digest(
    mut ctx: *const ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= OCB_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"ocb.c\0" as *const u8 as *const libc::c_char,
            391 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 126],
                &[libc::c_char; 126],
            >(
                b"void nettle_ocb_digest(const struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3032: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= OCB_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                391 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 126],
                    &[libc::c_char; 126],
                >(
                    b"void nettle_ocb_digest(const struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    block16_xor3(
        &mut block,
        &*((*key).L).as_ptr().offset(1 as libc::c_int as isize),
        if (*ctx).message_count > 0 as libc::c_int as libc::c_ulong {
            &(*ctx).offset
        } else {
            &(*ctx).initial
        },
    );
    block16_xor(&mut block, &(*ctx).checksum);
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        (block.b).as_mut_ptr(),
        (block.b).as_mut_ptr(),
    );
    nettle_memxor3(
        digest as *mut libc::c_void,
        (block.b).as_mut_ptr() as *const libc::c_void,
        ((*ctx).sum.b).as_ptr() as *const libc::c_void,
        length,
    );
}
pub unsafe extern "C" fn nettle_ocb_encrypt_message(
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctx: ocb_ctx = ocb_ctx {
        initial: nettle_block16 { b: [0; 16] },
        offset: nettle_block16 { b: [0; 16] },
        sum: nettle_block16 { b: [0; 16] },
        checksum: nettle_block16 { b: [0; 16] },
        data_count: 0,
        message_count: 0,
    };
    if clength >= tlength {} else {
        __assert_fail(
            b"clength >= tlength\0" as *const u8 as *const libc::c_char,
            b"ocb.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 186],
                &[libc::c_char; 186],
            >(
                b"void nettle_ocb_encrypt_message(const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3171: {
        if clength >= tlength {} else {
            __assert_fail(
                b"clength >= tlength\0" as *const u8 as *const libc::c_char,
                b"ocb.c\0" as *const u8 as *const libc::c_char,
                408 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 186],
                    &[libc::c_char; 186],
                >(
                    b"void nettle_ocb_encrypt_message(const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_ocb_set_nonce(&mut ctx, cipher, f, tlength, nlength, nonce);
    nettle_ocb_update(&mut ctx, key, cipher, f, alength, adata);
    nettle_ocb_encrypt(
        &mut ctx,
        key,
        cipher,
        f,
        clength.wrapping_sub(tlength),
        dst,
        src,
    );
    nettle_ocb_digest(
        &mut ctx,
        key,
        cipher,
        f,
        tlength,
        dst.offset(clength as isize).offset(-(tlength as isize)),
    );
}
pub unsafe extern "C" fn nettle_ocb_decrypt_message(
    mut key: *const ocb_key,
    mut encrypt_ctx: *const libc::c_void,
    mut encrypt: Option::<nettle_cipher_func>,
    mut decrypt_ctx: *const libc::c_void,
    mut decrypt: Option::<nettle_cipher_func>,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> libc::c_int {
    let mut ctx: ocb_ctx = ocb_ctx {
        initial: nettle_block16 { b: [0; 16] },
        offset: nettle_block16 { b: [0; 16] },
        sum: nettle_block16 { b: [0; 16] },
        checksum: nettle_block16 { b: [0; 16] },
        data_count: 0,
        message_count: 0,
    };
    let mut digest: nettle_block16 = nettle_block16 { b: [0; 16] };
    nettle_ocb_set_nonce(&mut ctx, encrypt_ctx, encrypt, tlength, nlength, nonce);
    nettle_ocb_update(&mut ctx, key, encrypt_ctx, encrypt, alength, adata);
    nettle_ocb_decrypt(
        &mut ctx,
        key,
        encrypt_ctx,
        encrypt,
        decrypt_ctx,
        decrypt,
        mlength,
        dst,
        src,
    );
    nettle_ocb_digest(
        &mut ctx,
        key,
        encrypt_ctx,
        encrypt,
        tlength,
        (digest.b).as_mut_ptr(),
    );
    return nettle_memeql_sec(
        (digest.b).as_mut_ptr() as *const libc::c_void,
        src.offset(mlength as isize) as *const libc::c_void,
        tlength,
    );
}
