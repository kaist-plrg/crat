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
    fn _nettle_umac_nh_n(
        out: *mut uint64_t,
        n: libc::c_uint,
        key: *const uint32_t,
        length: libc::c_uint,
        msg: *const uint8_t,
    );
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
pub struct umac64_ctx {
    pub l1_key: [uint32_t; 260],
    pub l2_key: [uint32_t; 12],
    pub l3_key1: [uint64_t; 16],
    pub l3_key2: [uint32_t; 2],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 6],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub nonce_low: libc::c_ushort,
    pub pad_cache: [uint32_t; 4],
    pub index: libc::c_uint,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
pub unsafe extern "C" fn nettle_umac64_set_key(
    mut ctx: *mut umac64_ctx,
    mut key: *const uint8_t,
) {
    _nettle_umac_set_key(
        ((*ctx).l1_key).as_mut_ptr(),
        ((*ctx).l2_key).as_mut_ptr(),
        ((*ctx).l3_key1).as_mut_ptr(),
        ((*ctx).l3_key2).as_mut_ptr(),
        &mut (*ctx).pdf_key,
        key,
        2 as libc::c_int as libc::c_uint,
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
pub unsafe extern "C" fn nettle_umac64_set_nonce(
    mut ctx: *mut umac64_ctx,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    if nonce_length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nonce_length > 0\0" as *const u8 as *const libc::c_char,
            b"umac64.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_934: {
        if nonce_length > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"nonce_length > 0\0" as *const u8 as *const libc::c_char,
                b"umac64.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nonce_length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
            b"umac64.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_891: {
        if nonce_length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                b"umac64.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
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
        as libc::c_int & 1 as libc::c_int) as libc::c_ushort;
    (*ctx)
        .nonce[nonce_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = ((*ctx)
        .nonce[nonce_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_int & !(1 as libc::c_int)) as uint8_t;
    (*ctx).nonce_length = nonce_length as libc::c_ushort;
}
pub unsafe extern "C" fn nettle_umac64_update(
    mut ctx: *mut umac64_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block_26: u64;
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
            current_block_26 = 10043043949733653460;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            let mut __umac64_y: [uint64_t; 2] = [0; 2];
            _nettle_umac_nh_n(
                __umac64_y.as_mut_ptr(),
                2 as libc::c_int as libc::c_uint,
                ((*ctx).l1_key).as_mut_ptr(),
                1024 as libc::c_int as libc::c_uint,
                ((*ctx).block).as_mut_ptr(),
            );
            __umac64_y[0 as libc::c_int
                as usize] = (__umac64_y[0 as libc::c_int as usize] as libc::c_ulong)
                .wrapping_add((8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                as uint64_t as uint64_t;
            __umac64_y[1 as libc::c_int
                as usize] = (__umac64_y[1 as libc::c_int as usize] as libc::c_ulong)
                .wrapping_add((8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                as uint64_t as uint64_t;
            let fresh0 = (*ctx).count;
            (*ctx).count = ((*ctx).count).wrapping_add(1);
            _nettle_umac_l2(
                ((*ctx).l2_key).as_mut_ptr(),
                ((*ctx).l2_state).as_mut_ptr(),
                2 as libc::c_int as libc::c_uint,
                fresh0,
                __umac64_y.as_mut_ptr(),
            );
            data = data.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_26 = 13586036798005543211;
        }
    } else {
        current_block_26 = 13586036798005543211;
    }
    match current_block_26 {
        13586036798005543211 => {
            while length >= ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong {
                let mut __umac64_y_0: [uint64_t; 2] = [0; 2];
                _nettle_umac_nh_n(
                    __umac64_y_0.as_mut_ptr(),
                    2 as libc::c_int as libc::c_uint,
                    ((*ctx).l1_key).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_uint,
                    data,
                );
                __umac64_y_0[0 as libc::c_int
                    as usize] = (__umac64_y_0[0 as libc::c_int as usize]
                    as libc::c_ulong)
                    .wrapping_add(
                        (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
                    ) as uint64_t as uint64_t;
                __umac64_y_0[1 as libc::c_int
                    as usize] = (__umac64_y_0[1 as libc::c_int as usize]
                    as libc::c_ulong)
                    .wrapping_add(
                        (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
                    ) as uint64_t as uint64_t;
                let fresh1 = (*ctx).count;
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                _nettle_umac_l2(
                    ((*ctx).l2_key).as_mut_ptr(),
                    ((*ctx).l2_state).as_mut_ptr(),
                    2 as libc::c_int as libc::c_uint,
                    fresh1,
                    __umac64_y_0.as_mut_ptr(),
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
pub unsafe extern "C" fn nettle_umac64_digest(
    mut ctx: *mut umac64_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut tag: [uint32_t; 2] = [0; 2];
    let mut pad: *mut uint32_t = 0 as *mut uint32_t;
    if length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length > 0\0" as *const u8 as *const libc::c_char,
            b"umac64.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1857: {
        if length > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const libc::c_char,
                b"umac64.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 8 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= 8\0" as *const u8 as *const libc::c_char,
            b"umac64.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1818: {
        if length <= 8 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= 8\0" as *const u8 as *const libc::c_char,
                b"umac64.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).index > 0 as libc::c_int as libc::c_uint
        || (*ctx).count == 0 as libc::c_int as libc::c_ulong
    {
        let mut y: [uint64_t; 2] = [0; 2];
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
        _nettle_umac_nh_n(
            y.as_mut_ptr(),
            2 as libc::c_int as libc::c_uint,
            ((*ctx).l1_key).as_mut_ptr(),
            ((*ctx).index).wrapping_add(pad_0),
            ((*ctx).block).as_mut_ptr(),
        );
        y[0 as libc::c_int
            as usize] = (y[0 as libc::c_int as usize] as libc::c_ulong)
            .wrapping_add(
                (8 as libc::c_int as libc::c_uint).wrapping_mul((*ctx).index)
                    as libc::c_ulong,
            ) as uint64_t as uint64_t;
        y[1 as libc::c_int
            as usize] = (y[1 as libc::c_int as usize] as libc::c_ulong)
            .wrapping_add(
                (8 as libc::c_int as libc::c_uint).wrapping_mul((*ctx).index)
                    as libc::c_ulong,
            ) as uint64_t as uint64_t;
        let fresh2 = (*ctx).count;
        (*ctx).count = ((*ctx).count).wrapping_add(1);
        _nettle_umac_l2(
            ((*ctx).l2_key).as_mut_ptr(),
            ((*ctx).l2_state).as_mut_ptr(),
            2 as libc::c_int as libc::c_uint,
            fresh2,
            y.as_mut_ptr(),
        );
    }
    if (*ctx).count > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ctx->count > 0\0" as *const u8 as *const libc::c_char,
            b"umac64.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1642: {
        if (*ctx).count > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ctx->count > 0\0" as *const u8 as *const libc::c_char,
                b"umac64.c\0" as *const u8 as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0",
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
    pad = ((*ctx).pad_cache)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int * ((*ctx).nonce_low as libc::c_int & 1 as libc::c_int))
                as isize,
        );
    (*ctx).nonce_low = ((*ctx).nonce_low).wrapping_add(1);
    (*ctx).nonce_low;
    if (*ctx).nonce_low as libc::c_int & 1 as libc::c_int == 0 {
        let mut i: libc::c_uint = ((*ctx).nonce_length as libc::c_int - 1 as libc::c_int)
            as libc::c_uint;
        (*ctx).nonce_low = 0 as libc::c_int as libc::c_ushort;
        (*ctx)
            .nonce[i
            as usize] = ((*ctx).nonce[i as usize] as libc::c_int + 2 as libc::c_int)
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
        2 as libc::c_int as libc::c_uint,
        (*ctx).count,
    );
    tag[0 as libc::c_int
        as usize] = *pad.offset(0 as libc::c_int as isize)
        ^ (*ctx).l3_key2[0 as libc::c_int as usize]
        ^ _nettle_umac_l3(((*ctx).l3_key1).as_mut_ptr(), ((*ctx).l2_state).as_mut_ptr());
    tag[1 as libc::c_int
        as usize] = *pad.offset(1 as libc::c_int as isize)
        ^ (*ctx).l3_key2[1 as libc::c_int as usize]
        ^ _nettle_umac_l3(
            ((*ctx).l3_key1).as_mut_ptr().offset(8 as libc::c_int as isize),
            ((*ctx).l2_state).as_mut_ptr().offset(2 as libc::c_int as isize),
        );
    memcpy(digest as *mut libc::c_void, tag.as_mut_ptr() as *const libc::c_void, length);
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
    (*ctx).count = (*ctx).index as uint64_t;
}
