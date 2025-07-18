use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abort() -> !;
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
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_sha256_init(ctx: *mut sha256_ctx);
    fn nettle_sha256_update(ctx: *mut sha256_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha256_digest(ctx: *mut sha256_ctx, length: size_t, digest: *mut uint8_t);
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
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
pub type yarrow_pool_id = libc::c_uint;
pub const YARROW_SLOW: yarrow_pool_id = 1;
pub const YARROW_FAST: yarrow_pool_id = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow_source {
    pub estimate: [uint32_t; 2],
    pub next: yarrow_pool_id,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow256_ctx {
    pub pools: [sha256_ctx; 2],
    pub seeded: libc::c_int,
    pub key: aes256_ctx,
    pub counter: [uint8_t; 16],
    pub nsources: libc::c_uint,
    pub sources: *mut yarrow_source,
}
pub unsafe extern "C" fn nettle_yarrow256_init(
    mut ctx: *mut yarrow256_ctx,
    mut n: libc::c_uint,
    mut s: *mut yarrow_source,
) {
    let mut i: libc::c_uint = 0;
    nettle_sha256_init(
        &mut *((*ctx).pools).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    nettle_sha256_init(
        &mut *((*ctx).pools).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    (*ctx).seeded = 0 as libc::c_int;
    memset(
        ((*ctx).counter).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    (*ctx).nsources = n;
    (*ctx).sources = s;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        (*((*ctx).sources).offset(i as isize))
            .estimate[YARROW_FAST as libc::c_int
            as usize] = 0 as libc::c_int as uint32_t;
        (*((*ctx).sources).offset(i as isize))
            .estimate[YARROW_SLOW as libc::c_int
            as usize] = 0 as libc::c_int as uint32_t;
        (*((*ctx).sources).offset(i as isize)).next = YARROW_FAST;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn nettle_yarrow256_seed(
    mut ctx: *mut yarrow256_ctx,
    mut length: size_t,
    mut seed_file: *const uint8_t,
) {
    if length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length > 0\0" as *const u8 as *const libc::c_char,
            b"yarrow256.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void nettle_yarrow256_seed(struct yarrow256_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2312: {
        if length > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const libc::c_char,
                b"yarrow256.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void nettle_yarrow256_seed(struct yarrow256_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_sha256_update(
        &mut *((*ctx).pools).as_mut_ptr().offset(YARROW_FAST as libc::c_int as isize),
        length,
        seed_file,
    );
    nettle_yarrow256_fast_reseed(ctx);
}
unsafe extern "C" fn yarrow_generate_block(
    mut ctx: *mut yarrow256_ctx,
    mut block: *mut uint8_t,
) {
    let mut i: libc::c_uint = 0;
    nettle_aes256_encrypt(
        &mut (*ctx).key,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
        block,
        ((*ctx).counter).as_mut_ptr(),
    );
    i = ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong as libc::c_uint;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        (*ctx).counter[i as usize] = ((*ctx).counter[i as usize]).wrapping_add(1);
        if (*ctx).counter[i as usize] != 0 {
            break;
        }
    };
}
unsafe extern "C" fn yarrow_iterate(mut digest: *mut uint8_t) {
    let mut v0: [uint8_t; 32] = [0; 32];
    let mut i: libc::c_uint = 0;
    memcpy(
        v0.as_mut_ptr() as *mut libc::c_void,
        digest as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    loop {
        i = i.wrapping_add(1);
        if !(i < 1500 as libc::c_int as libc::c_uint) {
            break;
        }
        let mut count: [uint8_t; 4] = [0; 4];
        let mut hash: sha256_ctx = sha256_ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        };
        nettle_sha256_init(&mut hash);
        count[0 as libc::c_int
            as usize] = (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        count[1 as libc::c_int
            as usize] = (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        count[2 as libc::c_int
            as usize] = (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        count[3 as libc::c_int
            as usize] = (i & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        nettle_sha256_update(&mut hash, 32 as libc::c_int as size_t, digest);
        nettle_sha256_update(
            &mut hash,
            ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
            v0.as_mut_ptr(),
        );
        nettle_sha256_update(
            &mut hash,
            ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
            count.as_mut_ptr(),
        );
        nettle_sha256_digest(&mut hash, 32 as libc::c_int as size_t, digest);
    };
}
pub unsafe extern "C" fn nettle_yarrow256_fast_reseed(mut ctx: *mut yarrow256_ctx) {
    let mut digest: [uint8_t; 32] = [0; 32];
    let mut i: libc::c_uint = 0;
    if (*ctx).seeded != 0 {
        let mut blocks: [uint8_t; 32] = [0; 32];
        yarrow_generate_block(ctx, blocks.as_mut_ptr());
        yarrow_generate_block(
            ctx,
            blocks.as_mut_ptr().offset(16 as libc::c_int as isize),
        );
        nettle_sha256_update(
            &mut *((*ctx).pools)
                .as_mut_ptr()
                .offset(YARROW_FAST as libc::c_int as isize),
            ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
            blocks.as_mut_ptr(),
        );
    }
    nettle_sha256_digest(
        &mut *((*ctx).pools).as_mut_ptr().offset(YARROW_FAST as libc::c_int as isize),
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    yarrow_iterate(digest.as_mut_ptr());
    nettle_aes256_set_encrypt_key(&mut (*ctx).key, digest.as_mut_ptr());
    (*ctx).seeded = 1 as libc::c_int;
    memset(
        ((*ctx).counter).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    nettle_aes256_encrypt(
        &mut (*ctx).key,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
        ((*ctx).counter).as_mut_ptr(),
        ((*ctx).counter).as_mut_ptr(),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*ctx).nsources {
        (*((*ctx).sources).offset(i as isize))
            .estimate[YARROW_FAST as libc::c_int
            as usize] = 0 as libc::c_int as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn nettle_yarrow256_slow_reseed(mut ctx: *mut yarrow256_ctx) {
    let mut digest: [uint8_t; 32] = [0; 32];
    let mut i: libc::c_uint = 0;
    nettle_sha256_digest(
        &mut *((*ctx).pools).as_mut_ptr().offset(YARROW_SLOW as libc::c_int as isize),
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    nettle_sha256_update(
        &mut *((*ctx).pools).as_mut_ptr().offset(YARROW_FAST as libc::c_int as isize),
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    nettle_yarrow256_fast_reseed(ctx);
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*ctx).nsources {
        (*((*ctx).sources).offset(i as isize))
            .estimate[YARROW_SLOW as libc::c_int
            as usize] = 0 as libc::c_int as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn nettle_yarrow256_update(
    mut ctx: *mut yarrow256_ctx,
    mut source_index: libc::c_uint,
    mut entropy: libc::c_uint,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_int {
    let mut current: yarrow_pool_id = YARROW_FAST;
    let mut source: *mut yarrow_source = 0 as *mut yarrow_source;
    if source_index < (*ctx).nsources {} else {
        __assert_fail(
            b"source_index < ctx->nsources\0" as *const u8 as *const libc::c_char,
            b"yarrow256.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 105],
                &[libc::c_char; 105],
            >(
                b"int nettle_yarrow256_update(struct yarrow256_ctx *, unsigned int, unsigned int, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2721: {
        if source_index < (*ctx).nsources {} else {
            __assert_fail(
                b"source_index < ctx->nsources\0" as *const u8 as *const libc::c_char,
                b"yarrow256.c\0" as *const u8 as *const libc::c_char,
                244 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 105],
                    &[libc::c_char; 105],
                >(
                    b"int nettle_yarrow256_update(struct yarrow256_ctx *, unsigned int, unsigned int, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 0 {
        return 0 as libc::c_int;
    }
    source = &mut *((*ctx).sources).offset(source_index as isize) as *mut yarrow_source;
    if (*ctx).seeded == 0 {
        current = YARROW_SLOW;
    } else {
        current = (*source).next;
        (*source).next = ((*source).next as u64 == 0) as libc::c_int as yarrow_pool_id;
    }
    nettle_sha256_update(
        &mut *((*ctx).pools).as_mut_ptr().offset(current as isize),
        length,
        data,
    );
    if (*source).estimate[current as usize] < 0x100000 as libc::c_int as libc::c_uint {
        if entropy > 0x100000 as libc::c_int as libc::c_uint {
            entropy = 0x100000 as libc::c_int as libc::c_uint;
        }
        if length < (0x100000 as libc::c_int / 4 as libc::c_int) as libc::c_ulong
            && entropy as libc::c_ulong
                > (4 as libc::c_int as libc::c_ulong).wrapping_mul(length)
        {
            entropy = (4 as libc::c_int as libc::c_ulong).wrapping_mul(length)
                as libc::c_uint;
        }
        entropy = entropy.wrapping_add((*source).estimate[current as usize]);
        if entropy > 0x100000 as libc::c_int as libc::c_uint {
            entropy = 0x100000 as libc::c_int as libc::c_uint;
        }
        (*source).estimate[current as usize] = entropy;
    }
    match current as libc::c_uint {
        0 => {
            if (*source).estimate[YARROW_FAST as libc::c_int as usize]
                >= 100 as libc::c_int as libc::c_uint
            {
                nettle_yarrow256_fast_reseed(ctx);
                return 1 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        }
        1 => {
            if nettle_yarrow256_needed_sources(ctx) == 0 {
                nettle_yarrow256_slow_reseed(ctx);
                return 1 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn yarrow_gate(mut ctx: *mut yarrow256_ctx) {
    let mut key: [uint8_t; 32] = [0; 32];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong
    {
        yarrow_generate_block(ctx, key.as_mut_ptr().offset(i as isize));
        i = i.wrapping_add(16 as libc::c_int as libc::c_uint);
    }
    nettle_aes256_set_encrypt_key(&mut (*ctx).key, key.as_mut_ptr());
}
pub unsafe extern "C" fn nettle_yarrow256_random(
    mut ctx: *mut yarrow256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    if (*ctx).seeded != 0 {} else {
        __assert_fail(
            b"ctx->seeded\0" as *const u8 as *const libc::c_char,
            b"yarrow256.c\0" as *const u8 as *const libc::c_char,
            328 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2915: {
        if (*ctx).seeded != 0 {} else {
            __assert_fail(
                b"ctx->seeded\0" as *const u8 as *const libc::c_char,
                b"yarrow256.c\0" as *const u8 as *const libc::c_char,
                328 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length >= 16 as libc::c_int as libc::c_ulong {
        yarrow_generate_block(ctx, dst);
        dst = dst.offset(16 as libc::c_int as isize);
        length = (length as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    if length != 0 {
        let mut buffer: [uint8_t; 16] = [0; 16];
        if length < 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length < AES_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                b"yarrow256.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2851: {
            if length < 16 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"length < AES_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                    b"yarrow256.c\0" as *const u8 as *const libc::c_char,
                    340 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        yarrow_generate_block(ctx, buffer.as_mut_ptr());
        memcpy(
            dst as *mut libc::c_void,
            buffer.as_mut_ptr() as *const libc::c_void,
            length,
        );
    }
    yarrow_gate(ctx);
}
pub unsafe extern "C" fn nettle_yarrow256_is_seeded(
    mut ctx: *mut yarrow256_ctx,
) -> libc::c_int {
    return (*ctx).seeded;
}
pub unsafe extern "C" fn nettle_yarrow256_needed_sources(
    mut ctx: *mut yarrow256_ctx,
) -> libc::c_uint {
    let mut k: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    k = 0 as libc::c_int as libc::c_uint;
    i = k;
    while i < (*ctx).nsources {
        if (*((*ctx).sources).offset(i as isize))
            .estimate[YARROW_SLOW as libc::c_int as usize]
            >= 160 as libc::c_int as libc::c_uint
        {
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return if k < 2 as libc::c_int as libc::c_uint {
        (2 as libc::c_int as libc::c_uint).wrapping_sub(k)
    } else {
        0 as libc::c_int as libc::c_uint
    };
}
