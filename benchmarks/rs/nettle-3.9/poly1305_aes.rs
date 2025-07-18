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
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn _nettle_poly1305_set_key(ctx: *mut poly1305_ctx, key: *const uint8_t);
    fn _nettle_poly1305_digest(ctx: *mut poly1305_ctx, s: *mut nettle_block16);
    fn _nettle_poly1305_block(
        ctx: *mut poly1305_ctx,
        m: *const uint8_t,
        high: libc::c_uint,
    );
    fn _nettle_poly1305_update(
        ctx: *mut poly1305_ctx,
        buffer: *mut uint8_t,
        index: libc::c_uint,
        length: size_t,
        m: *const uint8_t,
    ) -> libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly1305_ctx {
    pub r: C2RustUnnamed_0,
    pub s32: [uint32_t; 3],
    pub hh: uint32_t,
    pub h: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub h32: [uint32_t; 4],
    pub h64: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r32: [uint32_t; 6],
    pub r64: [uint64_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly1305_aes_ctx {
    pub pctx: poly1305_ctx,
    pub block: [uint8_t; 16],
    pub index: libc::c_uint,
    pub nonce: [uint8_t; 16],
    pub aes: aes128_ctx,
}
pub unsafe extern "C" fn nettle_poly1305_aes_set_key(
    mut ctx: *mut poly1305_aes_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes128_set_encrypt_key(&mut (*ctx).aes, key);
    _nettle_poly1305_set_key(&mut (*ctx).pctx, key.offset(16 as libc::c_int as isize));
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_poly1305_aes_set_nonce(
    mut ctx: *mut poly1305_aes_ctx,
    mut nonce: *const uint8_t,
) {
    memcpy(
        ((*ctx).nonce).as_mut_ptr() as *mut libc::c_void,
        nonce as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
}
pub unsafe extern "C" fn nettle_poly1305_aes_update(
    mut ctx: *mut poly1305_aes_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    (*ctx)
        .index = _nettle_poly1305_update(
        &mut (*ctx).pctx,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        data,
    );
}
pub unsafe extern "C" fn nettle_poly1305_aes_digest(
    mut ctx: *mut poly1305_aes_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut s: nettle_block16 = nettle_block16 { b: [0; 16] };
    if (*ctx).index > 0 as libc::c_int as libc::c_uint {
        if (*ctx).index < 16 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"ctx->index < POLY1305_BLOCK_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"poly1305-aes.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void nettle_poly1305_aes_digest(struct poly1305_aes_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_925: {
            if (*ctx).index < 16 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"ctx->index < POLY1305_BLOCK_SIZE\0" as *const u8
                        as *const libc::c_char,
                    b"poly1305-aes.c\0" as *const u8 as *const libc::c_char,
                    75 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void nettle_poly1305_aes_digest(struct poly1305_aes_ctx *, size_t, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*ctx).block[(*ctx).index as usize] = 1 as libc::c_int as uint8_t;
        memset(
            ((*ctx).block)
                .as_mut_ptr()
                .offset((*ctx).index as isize)
                .offset(1 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((16 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                .wrapping_sub((*ctx).index) as libc::c_ulong,
        );
        _nettle_poly1305_block(
            &mut (*ctx).pctx,
            ((*ctx).block).as_mut_ptr(),
            0 as libc::c_int as libc::c_uint,
        );
    }
    nettle_aes128_encrypt(
        &mut (*ctx).aes,
        16 as libc::c_int as size_t,
        (s.b).as_mut_ptr(),
        ((*ctx).nonce).as_mut_ptr(),
    );
    _nettle_poly1305_digest(&mut (*ctx).pctx, &mut s);
    memcpy(
        digest as *mut libc::c_void,
        (s.b).as_mut_ptr() as *const libc::c_void,
        length,
    );
    let mut increment_i: libc::c_uint = (16 as libc::c_int - 1 as libc::c_int)
        as libc::c_uint;
    (*ctx)
        .nonce[increment_i
        as usize] = ((*ctx).nonce[increment_i as usize]).wrapping_add(1);
    if (*ctx).nonce[increment_i as usize] as libc::c_int == 0 as libc::c_int {
        while increment_i > 0 as libc::c_int as libc::c_uint
            && {
                increment_i = increment_i.wrapping_sub(1);
                (*ctx)
                    .nonce[increment_i
                    as usize] = ((*ctx).nonce[increment_i as usize]).wrapping_add(1);
                (*ctx).nonce[increment_i as usize] as libc::c_int == 0 as libc::c_int
            }
        {}
    }
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
