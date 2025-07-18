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
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_ctr_crypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
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
pub struct eax_key {
    pub pad_block: nettle_block16,
    pub pad_partial: nettle_block16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eax_ctx {
    pub omac_nonce: nettle_block16,
    pub omac_data: nettle_block16,
    pub omac_message: nettle_block16,
    pub ctr: nettle_block16,
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
unsafe extern "C" fn omac_init(mut state: *mut nettle_block16, mut t: libc::c_uint) {
    memset(
        ((*state).b).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    (*state).b[(16 as libc::c_int - 1 as libc::c_int) as usize] = t as uint8_t;
}
unsafe extern "C" fn omac_update(
    mut state: *mut nettle_block16,
    mut key: *const eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    while length >= 16 as libc::c_int as libc::c_ulong {
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            ((*state).b).as_mut_ptr(),
            ((*state).b).as_mut_ptr(),
        );
        nettle_memxor(
            ((*state).b).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        length = (length as libc::c_ulong)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t as size_t;
        data = data.offset(16 as libc::c_int as isize);
    }
    if length > 0 as libc::c_int as libc::c_ulong {
        f
            .unwrap()(
            cipher,
            16 as libc::c_int as size_t,
            ((*state).b).as_mut_ptr(),
            ((*state).b).as_mut_ptr(),
        );
        nettle_memxor(
            ((*state).b).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            length,
        );
        (*state)
            .b[length
            as usize] = ((*state).b[length as usize] as libc::c_int
            ^ 0x80 as libc::c_int) as uint8_t;
        block16_xor(state, &(*key).pad_partial);
    }
}
unsafe extern "C" fn omac_final(
    mut state: *mut nettle_block16,
    mut key: *const eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
) {
    block16_xor(state, &(*key).pad_block);
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        ((*state).b).as_mut_ptr(),
        ((*state).b).as_mut_ptr(),
    );
}
pub unsafe extern "C" fn nettle_eax_set_key(
    mut key: *mut eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    f
        .unwrap()(
        cipher,
        16 as libc::c_int as size_t,
        ((*key).pad_block.b).as_mut_ptr(),
        (zero_block.b).as_ptr(),
    );
    block16_mulx_be(&mut (*key).pad_block, &mut (*key).pad_block);
    block16_mulx_be(&mut (*key).pad_partial, &mut (*key).pad_block);
    block16_xor(&mut (*key).pad_partial, &mut (*key).pad_block);
}
pub unsafe extern "C" fn nettle_eax_set_nonce(
    mut eax: *mut eax_ctx,
    mut key: *const eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    omac_init(&mut (*eax).omac_nonce, 0 as libc::c_int as libc::c_uint);
    omac_update(&mut (*eax).omac_nonce, key, cipher, f, nonce_length, nonce);
    omac_final(&mut (*eax).omac_nonce, key, cipher, f);
    memcpy(
        ((*eax).ctr.b).as_mut_ptr() as *mut libc::c_void,
        ((*eax).omac_nonce.b).as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    omac_init(&mut (*eax).omac_data, 1 as libc::c_int as libc::c_uint);
    omac_init(&mut (*eax).omac_message, 2 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn nettle_eax_update(
    mut eax: *mut eax_ctx,
    mut key: *const eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut data_length: size_t,
    mut data: *const uint8_t,
) {
    omac_update(&mut (*eax).omac_data, key, cipher, f, data_length, data);
}
pub unsafe extern "C" fn nettle_eax_encrypt(
    mut eax: *mut eax_ctx,
    mut key: *const eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ctr_crypt(
        cipher,
        f,
        16 as libc::c_int as size_t,
        ((*eax).ctr.b).as_mut_ptr(),
        length,
        dst,
        src,
    );
    omac_update(&mut (*eax).omac_message, key, cipher, f, length, dst);
}
pub unsafe extern "C" fn nettle_eax_decrypt(
    mut eax: *mut eax_ctx,
    mut key: *const eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    omac_update(&mut (*eax).omac_message, key, cipher, f, length, src);
    nettle_ctr_crypt(
        cipher,
        f,
        16 as libc::c_int as size_t,
        ((*eax).ctr.b).as_mut_ptr(),
        length,
        dst,
        src,
    );
}
pub unsafe extern "C" fn nettle_eax_digest(
    mut eax: *mut eax_ctx,
    mut key: *const eax_key,
    mut cipher: *const libc::c_void,
    mut f: Option::<nettle_cipher_func>,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if length > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length > 0\0" as *const u8 as *const libc::c_char,
            b"eax.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 120],
                &[libc::c_char; 120],
            >(
                b"void nettle_eax_digest(struct eax_ctx *, const struct eax_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1399: {
        if length > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const libc::c_char,
                b"eax.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 120],
                    &[libc::c_char; 120],
                >(
                    b"void nettle_eax_digest(struct eax_ctx *, const struct eax_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= EAX_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
            b"eax.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 120],
                &[libc::c_char; 120],
            >(
                b"void nettle_eax_digest(struct eax_ctx *, const struct eax_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1356: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= EAX_BLOCK_SIZE\0" as *const u8 as *const libc::c_char,
                b"eax.c\0" as *const u8 as *const libc::c_char,
                141 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 120],
                    &[libc::c_char; 120],
                >(
                    b"void nettle_eax_digest(struct eax_ctx *, const struct eax_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    omac_final(&mut (*eax).omac_data, key, cipher, f);
    omac_final(&mut (*eax).omac_message, key, cipher, f);
    block16_xor(&mut (*eax).omac_nonce, &mut (*eax).omac_data);
    nettle_memxor3(
        digest as *mut libc::c_void,
        ((*eax).omac_nonce.b).as_mut_ptr() as *const libc::c_void,
        ((*eax).omac_message.b).as_mut_ptr() as *const libc::c_void,
        length,
    );
}
