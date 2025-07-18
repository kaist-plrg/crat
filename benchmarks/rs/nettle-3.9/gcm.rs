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
    fn _nettle_ghash_set_key(ctx: *mut gcm_key, key: *const nettle_block16);
    fn _nettle_ghash_update(
        ctx: *const gcm_key,
        state: *mut nettle_block16,
        blocks: size_t,
        data: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_ctr_crypt16(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        fill: Option::<nettle_fill16_func>,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
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
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_ctx {
    pub iv: nettle_block16,
    pub ctr: nettle_block16,
    pub x: nettle_block16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
}
pub type nettle_fill16_func = unsafe extern "C" fn(
    *mut uint8_t,
    size_t,
    *mut nettle_block16,
) -> ();
#[inline]
unsafe extern "C" fn block16_zero(mut r: *mut nettle_block16) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    *r = zero_block;
}
#[inline]
unsafe extern "C" fn block16_xor(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as libc::c_int as usize] ^= (*x).u64_0[0 as libc::c_int as usize];
    (*r).u64_0[1 as libc::c_int as usize] ^= (*x).u64_0[1 as libc::c_int as usize];
}
pub unsafe extern "C" fn nettle_gcm_set_key(
    mut key: *mut gcm_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut key_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        (key_block.b).as_mut_ptr(),
        (zero_block.b).as_ptr(),
    );
    _nettle_ghash_set_key(key, &mut key_block);
}
unsafe extern "C" fn gcm_hash(
    mut key: *const gcm_key,
    mut x: *mut nettle_block16,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    data = _nettle_ghash_update(
        key,
        x,
        length.wrapping_div(16 as libc::c_int as libc::c_ulong),
        data,
    );
    length &= (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    if length > 0 as libc::c_int as libc::c_ulong {
        let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
        block16_zero(&mut block);
        memcpy(
            (block.b).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            length,
        );
        _nettle_ghash_update(key, x, 1 as libc::c_int as size_t, (block.b).as_mut_ptr());
    }
}
unsafe extern "C" fn gcm_hash_sizes(
    mut key: *const gcm_key,
    mut x: *mut nettle_block16,
    mut auth_size: uint64_t,
    mut data_size: uint64_t,
) {
    let mut buffer: nettle_block16 = nettle_block16 { b: [0; 16] };
    data_size = (data_size as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
    auth_size = (auth_size as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
    buffer.u64_0[0 as libc::c_int as usize] = auth_size.swap_bytes();
    buffer.u64_0[1 as libc::c_int as usize] = data_size.swap_bytes();
    _nettle_ghash_update(key, x, 1 as libc::c_int as size_t, (buffer.b).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_gcm_set_iv(
    mut ctx: *mut gcm_ctx,
    mut key: *const gcm_key,
    mut length: size_t,
    mut iv: *const uint8_t,
) {
    if length == (16 as libc::c_int - 4 as libc::c_int) as libc::c_ulong {
        memcpy(
            ((*ctx).iv.b).as_mut_ptr() as *mut libc::c_void,
            iv as *const libc::c_void,
            (16 as libc::c_int - 4 as libc::c_int) as libc::c_ulong,
        );
        (*ctx)
            .iv
            .b[(16 as libc::c_int - 4 as libc::c_int)
            as usize] = 0 as libc::c_int as uint8_t;
        (*ctx)
            .iv
            .b[(16 as libc::c_int - 3 as libc::c_int)
            as usize] = 0 as libc::c_int as uint8_t;
        (*ctx)
            .iv
            .b[(16 as libc::c_int - 2 as libc::c_int)
            as usize] = 0 as libc::c_int as uint8_t;
        (*ctx)
            .iv
            .b[(16 as libc::c_int - 1 as libc::c_int)
            as usize] = 1 as libc::c_int as uint8_t;
    } else {
        block16_zero(&mut (*ctx).iv);
        gcm_hash(key, &mut (*ctx).iv, length, iv);
        gcm_hash_sizes(key, &mut (*ctx).iv, 0 as libc::c_int as uint64_t, length);
    }
    (*ctx).ctr = (*ctx).iv;
    let mut increment_i: libc::c_uint = (4 as libc::c_int - 1 as libc::c_int)
        as libc::c_uint;
    let ref mut fresh0 = *((*ctx).ctr.b)
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(-(4 as libc::c_int as isize))
        .offset(increment_i as isize);
    *fresh0 = (*fresh0).wrapping_add(1);
    if *fresh0 as libc::c_int == 0 as libc::c_int {
        while increment_i > 0 as libc::c_int as libc::c_uint
            && {
                increment_i = increment_i.wrapping_sub(1);
                let ref mut fresh1 = *((*ctx).ctr.b)
                    .as_mut_ptr()
                    .offset(16 as libc::c_int as isize)
                    .offset(-(4 as libc::c_int as isize))
                    .offset(increment_i as isize);
                *fresh1 = (*fresh1).wrapping_add(1);
                *fresh1 as libc::c_int == 0 as libc::c_int
            }
        {}
    }
    block16_zero(&mut (*ctx).x);
    (*ctx).data_size = 0 as libc::c_int as uint64_t;
    (*ctx).auth_size = (*ctx).data_size;
}
pub unsafe extern "C" fn nettle_gcm_update(
    mut ctx: *mut gcm_ctx,
    mut key: *const gcm_key,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    if ((*ctx).auth_size).wrapping_rem(16 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"ctx->auth_size % GCM_BLOCK_SIZE == 0\0" as *const u8
                as *const libc::c_char,
            b"gcm.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void nettle_gcm_update(struct gcm_ctx *, const struct gcm_key *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2255: {
        if ((*ctx).auth_size).wrapping_rem(16 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"ctx->auth_size % GCM_BLOCK_SIZE == 0\0" as *const u8
                    as *const libc::c_char,
                b"gcm.c\0" as *const u8 as *const libc::c_char,
                139 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"void nettle_gcm_update(struct gcm_ctx *, const struct gcm_key *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).data_size == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ctx->data_size == 0\0" as *const u8 as *const libc::c_char,
            b"gcm.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void nettle_gcm_update(struct gcm_ctx *, const struct gcm_key *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2208: {
        if (*ctx).data_size == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ctx->data_size == 0\0" as *const u8 as *const libc::c_char,
                b"gcm.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"void nettle_gcm_update(struct gcm_ctx *, const struct gcm_key *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    gcm_hash(key, &mut (*ctx).x, length, data);
    (*ctx)
        .auth_size = ((*ctx).auth_size as libc::c_ulong).wrapping_add(length) as uint64_t
        as uint64_t;
}
unsafe extern "C" fn gcm_fill(
    mut ctr: *mut uint8_t,
    mut blocks: size_t,
    mut buffer: *mut nettle_block16,
) {
    let mut hi: uint64_t = 0;
    let mut mid: uint64_t = 0;
    let mut lo: uint32_t = 0;
    let mut i: size_t = 0;
    hi = (*ctr.offset(7 as libc::c_int as isize) as uint64_t) << 56 as libc::c_int
        | (*ctr.offset(6 as libc::c_int as isize) as uint64_t) << 48 as libc::c_int
        | (*ctr.offset(5 as libc::c_int as isize) as uint64_t) << 40 as libc::c_int
        | (*ctr.offset(4 as libc::c_int as isize) as uint64_t) << 32 as libc::c_int
        | (*ctr.offset(3 as libc::c_int as isize) as uint64_t) << 24 as libc::c_int
        | (*ctr.offset(2 as libc::c_int as isize) as uint64_t) << 16 as libc::c_int
        | (*ctr.offset(1 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
        | *ctr.offset(0 as libc::c_int as isize) as uint64_t;
    mid = ((*ctr.offset(8 as libc::c_int as isize).offset(3 as libc::c_int as isize)
        as uint32_t) << 24 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*ctr.offset(8 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *ctr.offset(8 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t) as uint64_t;
    lo = (*ctr.offset(12 as libc::c_int as isize).offset(0 as libc::c_int as isize)
        as uint32_t) << 24 as libc::c_int
        | (*ctr.offset(12 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as uint32_t) << 16 as libc::c_int
        | (*ctr.offset(12 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as uint32_t) << 8 as libc::c_int
        | *ctr.offset(12 as libc::c_int as isize).offset(3 as libc::c_int as isize)
            as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < blocks {
        (*buffer.offset(i as isize)).u64_0[0 as libc::c_int as usize] = hi;
        (*buffer.offset(i as isize))
            .u64_0[1 as libc::c_int
            as usize] = mid
            .wrapping_add((lo.swap_bytes() as uint64_t) << 32 as libc::c_int);
        lo = lo.wrapping_add(1);
        lo;
        i = i.wrapping_add(1);
        i;
    }
    *ctr
        .offset(12 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (lo >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *ctr
        .offset(12 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (lo >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *ctr
        .offset(12 as libc::c_int as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (lo >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    *ctr
        .offset(12 as libc::c_int as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (lo & 0xff as libc::c_int as libc::c_uint) as uint8_t;
}
pub unsafe extern "C" fn nettle_gcm_encrypt(
    mut ctx: *mut gcm_ctx,
    mut key: *const gcm_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if ((*ctx).data_size).wrapping_rem(16 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"ctx->data_size % GCM_BLOCK_SIZE == 0\0" as *const u8
                as *const libc::c_char,
            b"gcm.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"void nettle_gcm_encrypt(struct gcm_ctx *, const struct gcm_key *, const void *, nettle_cipher_func *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2729: {
        if ((*ctx).data_size).wrapping_rem(16 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"ctx->data_size % GCM_BLOCK_SIZE == 0\0" as *const u8
                    as *const libc::c_char,
                b"gcm.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 138],
                    &[libc::c_char; 138],
                >(
                    b"void nettle_gcm_encrypt(struct gcm_ctx *, const struct gcm_key *, const void *, nettle_cipher_func *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    _nettle_ctr_crypt16(
        cipher,
        f,
        Some(gcm_fill as nettle_fill16_func),
        ((*ctx).ctr.b).as_mut_ptr(),
        length,
        dst,
        src,
    );
    gcm_hash(key, &mut (*ctx).x, length, dst);
    (*ctx)
        .data_size = ((*ctx).data_size as libc::c_ulong).wrapping_add(length) as uint64_t
        as uint64_t;
}
pub unsafe extern "C" fn nettle_gcm_decrypt(
    mut ctx: *mut gcm_ctx,
    mut key: *const gcm_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if ((*ctx).data_size).wrapping_rem(16 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"ctx->data_size % GCM_BLOCK_SIZE == 0\0" as *const u8
                as *const libc::c_char,
            b"gcm.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"void nettle_gcm_decrypt(struct gcm_ctx *, const struct gcm_key *, const void *, nettle_cipher_func *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2830: {
        if ((*ctx).data_size).wrapping_rem(16 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"ctx->data_size % GCM_BLOCK_SIZE == 0\0" as *const u8
                    as *const libc::c_char,
                b"gcm.c\0" as *const u8 as *const libc::c_char,
                223 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 138],
                    &[libc::c_char; 138],
                >(
                    b"void nettle_gcm_decrypt(struct gcm_ctx *, const struct gcm_key *, const void *, nettle_cipher_func *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    gcm_hash(key, &mut (*ctx).x, length, src);
    _nettle_ctr_crypt16(
        cipher,
        f,
        Some(gcm_fill as nettle_fill16_func),
        ((*ctx).ctr.b).as_mut_ptr(),
        length,
        dst,
        src,
    );
    (*ctx)
        .data_size = ((*ctx).data_size as libc::c_ulong).wrapping_add(length) as uint64_t
        as uint64_t;
}
pub unsafe extern "C" fn nettle_gcm_digest(
    mut ctx: *mut gcm_ctx,
    mut key: *const gcm_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut buffer: nettle_block16 = nettle_block16 { b: [0; 16] };
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= GCM_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
            b"gcm.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 120],
                &[libc::c_char; 120],
            >(
                b"void nettle_gcm_digest(struct gcm_ctx *, const struct gcm_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2987: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= GCM_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                b"gcm.c\0" as *const u8 as *const libc::c_char,
                238 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 120],
                    &[libc::c_char; 120],
                >(
                    b"void nettle_gcm_digest(struct gcm_ctx *, const struct gcm_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    gcm_hash_sizes(key, &mut (*ctx).x, (*ctx).auth_size, (*ctx).data_size);
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        (buffer.b).as_mut_ptr(),
        ((*ctx).iv.b).as_mut_ptr(),
    );
    block16_xor(&mut buffer, &mut (*ctx).x);
    memcpy(
        digest as *mut libc::c_void,
        (buffer.b).as_mut_ptr() as *const libc::c_void,
        length,
    );
}
