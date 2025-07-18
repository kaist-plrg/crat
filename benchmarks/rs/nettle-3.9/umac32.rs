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
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn _nettle_umac_set_key(
        l1_key: *mut uint32_t,
        l2_key: *mut uint32_t,
        l3_key1: *mut uint64_t,
        l3_key2: *mut uint32_t,
        pad: *mut aes128_ctx,
        key: *const uint8_t,
        n: libc::c_uint,
    );
    fn _nettle_umac_nh(
        key: *const uint32_t,
        length: libc::c_uint,
        msg: *const uint8_t,
    ) -> uint64_t;
    fn _nettle_umac_l2(
        key: *const uint32_t,
        state: *mut uint64_t,
        n: libc::c_uint,
        count: uint64_t,
        m: *const uint64_t,
    );
    fn _nettle_umac_l2_final(
        key: *const uint32_t,
        state: *mut uint64_t,
        n: libc::c_uint,
        count: uint64_t,
    );
    fn _nettle_umac_l3(key: *const uint64_t, m: *const uint64_t) -> uint32_t;
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
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct umac32_ctx {
    pub l1_key: [uint32_t; 256],
    pub l2_key: [uint32_t; 6],
    pub l3_key1: [uint64_t; 8],
    pub l3_key2: [uint32_t; 1],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 3],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub nonce_low: libc::c_ushort,
    pub pad_cache: [uint32_t; 4],
    pub index: libc::c_uint,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
pub unsafe extern "C" fn nettle_umac32_set_key(
    mut ctx: *mut umac32_ctx,
    mut key: *const uint8_t,
) {
    _nettle_umac_set_key(
        ((*ctx).l1_key).as_mut_ptr(),
        ((*ctx).l2_key).as_mut_ptr(),
        ((*ctx).l3_key1).as_mut_ptr(),
        ((*ctx).l3_key2).as_mut_ptr(),
        &mut (*ctx).pdf_key,
        key,
        1 as libc::c_int as libc::c_uint,
    );
    memset(
        ((*ctx).nonce).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    (*ctx).nonce_low = 0 as libc::c_int as libc::c_ushort;
    (*ctx)
        .nonce_length = ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong
        as libc::c_ushort;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
    (*ctx).count = (*ctx).index as uint64_t;
}
pub unsafe extern "C" fn nettle_umac32_set_nonce(
    mut ctx: *mut umac32_ctx,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    if nonce_length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nonce_length > 0\0" as *const u8 as *const libc::c_char,
            b"umac32.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_umac32_set_nonce(struct umac32_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_929: {
        if nonce_length > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"nonce_length > 0\0" as *const u8 as *const libc::c_char,
                b"umac32.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_umac32_set_nonce(struct umac32_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nonce_length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
            b"umac32.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_umac32_set_nonce(struct umac32_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_886: {
        if nonce_length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                b"umac32.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_umac32_set_nonce(struct umac32_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memcpy(
        ((*ctx).nonce).as_mut_ptr() as *mut libc::c_void,
        nonce as *const libc::c_void,
        nonce_length,
    );
    memset(
        ((*ctx).nonce).as_mut_ptr().offset(nonce_length as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (16 as libc::c_int as libc::c_ulong).wrapping_sub(nonce_length),
    );
    (*ctx)
        .nonce_low = ((*ctx)
        .nonce[nonce_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_int & 3 as libc::c_int) as libc::c_ushort;
    (*ctx)
        .nonce[nonce_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = ((*ctx)
        .nonce[nonce_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_int & !(3 as libc::c_int)) as uint8_t;
    (*ctx).nonce_length = nonce_length as libc::c_ushort;
}
pub unsafe extern "C" fn nettle_umac32_update(
    mut ctx: *mut umac32_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block_20: u64;
    if (*ctx).index != 0 {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 1024]>()
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
            current_block_20 = 11584701595673473500;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            let mut __umac32_y: uint64_t = (_nettle_umac_nh(
                ((*ctx).l1_key).as_mut_ptr(),
                1024 as libc::c_int as libc::c_uint,
                ((*ctx).block).as_mut_ptr(),
            ))
                .wrapping_add((8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong);
            let fresh0 = (*ctx).count;
            (*ctx).count = ((*ctx).count).wrapping_add(1);
            _nettle_umac_l2(
                ((*ctx).l2_key).as_mut_ptr(),
                ((*ctx).l2_state).as_mut_ptr(),
                1 as libc::c_int as libc::c_uint,
                fresh0,
                &mut __umac32_y,
            );
            data = data.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_20 = 7746791466490516765;
        }
    } else {
        current_block_20 = 7746791466490516765;
    }
    match current_block_20 {
        7746791466490516765 => {
            while length >= ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong {
                let mut __umac32_y_0: uint64_t = (_nettle_umac_nh(
                    ((*ctx).l1_key).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_uint,
                    data,
                ))
                    .wrapping_add(
                        (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
                    );
                let fresh1 = (*ctx).count;
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                _nettle_umac_l2(
                    ((*ctx).l2_key).as_mut_ptr(),
                    ((*ctx).l2_state).as_mut_ptr(),
                    1 as libc::c_int as libc::c_uint,
                    fresh1,
                    &mut __umac32_y_0,
                );
                data = data
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong
                            as isize,
                    );
                length = (length as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong,
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
pub unsafe extern "C" fn nettle_umac32_digest(
    mut ctx: *mut umac32_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut pad: uint32_t = 0;
    if length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length > 0\0" as *const u8 as *const libc::c_char,
            b"umac32.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_umac32_digest(struct umac32_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1739: {
        if length > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const libc::c_char,
                b"umac32.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_umac32_digest(struct umac32_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 4 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= 4\0" as *const u8 as *const libc::c_char,
            b"umac32.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_umac32_digest(struct umac32_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1700: {
        if length <= 4 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= 4\0" as *const u8 as *const libc::c_char,
                b"umac32.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_umac32_digest(struct umac32_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).index > 0 as libc::c_int as libc::c_uint
        || (*ctx).count == 0 as libc::c_int as libc::c_ulong
    {
        let mut y: uint64_t = 0;
        let mut pad_0: libc::c_uint = if (*ctx).index > 0 as libc::c_int as libc::c_uint
        {
            31 as libc::c_int as libc::c_uint & ((*ctx).index).wrapping_neg()
        } else {
            32 as libc::c_int as libc::c_uint
        };
        memset(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            pad_0 as libc::c_ulong,
        );
        y = (_nettle_umac_nh(
            ((*ctx).l1_key).as_mut_ptr(),
            ((*ctx).index).wrapping_add(pad_0),
            ((*ctx).block).as_mut_ptr(),
        ))
            .wrapping_add(
                (8 as libc::c_int as libc::c_uint).wrapping_mul((*ctx).index)
                    as libc::c_ulong,
            );
        let fresh2 = (*ctx).count;
        (*ctx).count = ((*ctx).count).wrapping_add(1);
        _nettle_umac_l2(
            ((*ctx).l2_key).as_mut_ptr(),
            ((*ctx).l2_state).as_mut_ptr(),
            1 as libc::c_int as libc::c_uint,
            fresh2,
            &mut y,
        );
    }
    if (*ctx).count > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ctx->count > 0\0" as *const u8 as *const libc::c_char,
            b"umac32.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_umac32_digest(struct umac32_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1545: {
        if (*ctx).count > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ctx->count > 0\0" as *const u8 as *const libc::c_char,
                b"umac32.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_umac32_digest(struct umac32_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).nonce_low as libc::c_int & 0x80 as libc::c_int == 0 {
        nettle_aes128_encrypt(
            &mut (*ctx).pdf_key,
            16 as libc::c_int as size_t,
            ((*ctx).pad_cache).as_mut_ptr() as *mut uint8_t,
            ((*ctx).nonce).as_mut_ptr(),
        );
        (*ctx)
            .nonce_low = ((*ctx).nonce_low as libc::c_int | 0x80 as libc::c_int)
            as libc::c_ushort;
    }
    pad = (*ctx)
        .pad_cache[((*ctx).nonce_low as libc::c_int & 3 as libc::c_int) as usize];
    (*ctx).nonce_low = ((*ctx).nonce_low).wrapping_add(1);
    (*ctx).nonce_low;
    if (*ctx).nonce_low as libc::c_int & 3 as libc::c_int == 0 {
        let mut i: libc::c_uint = ((*ctx).nonce_length as libc::c_int - 1 as libc::c_int)
            as libc::c_uint;
        (*ctx).nonce_low = 0 as libc::c_int as libc::c_ushort;
        (*ctx)
            .nonce[i
            as usize] = ((*ctx).nonce[i as usize] as libc::c_int + 4 as libc::c_int)
            as uint8_t;
        if (*ctx).nonce[i as usize] as libc::c_int == 0 as libc::c_int
            && i > 0 as libc::c_int as libc::c_uint
        {
            let mut increment_i: libc::c_uint = i
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
            (*ctx)
                .nonce[increment_i
                as usize] = ((*ctx).nonce[increment_i as usize]).wrapping_add(1);
            if (*ctx).nonce[increment_i as usize] as libc::c_int == 0 as libc::c_int {
                while increment_i > 0 as libc::c_int as libc::c_uint
                    && {
                        increment_i = increment_i.wrapping_sub(1);
                        (*ctx)
                            .nonce[increment_i
                            as usize] = ((*ctx).nonce[increment_i as usize])
                            .wrapping_add(1);
                        (*ctx).nonce[increment_i as usize] as libc::c_int
                            == 0 as libc::c_int
                    }
                {}
            }
        }
    }
    _nettle_umac_l2_final(
        ((*ctx).l2_key).as_mut_ptr(),
        ((*ctx).l2_state).as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        (*ctx).count,
    );
    pad
        ^= (*ctx).l3_key2[0 as libc::c_int as usize]
            ^ _nettle_umac_l3(
                ((*ctx).l3_key1).as_mut_ptr(),
                ((*ctx).l2_state).as_mut_ptr(),
            );
    memcpy(
        digest as *mut libc::c_void,
        &mut pad as *mut uint32_t as *const libc::c_void,
        length,
    );
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
    (*ctx).count = (*ctx).index as uint64_t;
}
