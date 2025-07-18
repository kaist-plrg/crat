use ::libc;
extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_eax_digest(
        eax: *mut eax_ctx,
        key: *const eax_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_eax_decrypt(
        eax: *mut eax_ctx,
        key: *const eax_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_eax_encrypt(
        eax: *mut eax_ctx,
        key: *const eax_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_eax_set_key(
        key: *mut eax_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
    );
    fn nettle_eax_set_nonce(
        eax: *mut eax_ctx,
        key: *const eax_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        nonce_length: size_t,
        nonce: *const uint8_t,
    );
    fn nettle_eax_update(
        eax: *mut eax_ctx,
        key: *const eax_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        data_length: size_t,
        data: *const uint8_t,
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
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eax_aes128_ctx {
    pub key: eax_key,
    pub eax: eax_ctx,
    pub cipher: aes128_ctx,
}
pub unsafe extern "C" fn nettle_eax_aes128_set_key(
    mut ctx: *mut eax_aes128_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes128_set_encrypt_key(&mut (*ctx).cipher, key);
    nettle_eax_set_key(
        &mut (*ctx).key,
        &mut (*ctx).cipher as *mut aes128_ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes128_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes128_encrypt
                    as unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
    );
}
pub unsafe extern "C" fn nettle_eax_aes128_set_nonce(
    mut ctx: *mut eax_aes128_ctx,
    mut length: size_t,
    mut iv: *const uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes128_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_eax_set_nonce(
            &mut (*ctx).eax,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes128_ctx as *const libc::c_void,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes128_encrypt
                        as unsafe extern "C" fn(
                            *const aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            iv,
        );
    };
}
pub unsafe extern "C" fn nettle_eax_aes128_update(
    mut ctx: *mut eax_aes128_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes128_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_eax_update(
            &mut (*ctx).eax,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes128_ctx as *const libc::c_void,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes128_encrypt
                        as unsafe extern "C" fn(
                            *const aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            data,
        );
    };
}
pub unsafe extern "C" fn nettle_eax_aes128_encrypt(
    mut ctx: *mut eax_aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes128_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_eax_encrypt(
            &mut (*ctx).eax,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes128_ctx as *const libc::c_void,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes128_encrypt
                        as unsafe extern "C" fn(
                            *const aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            dst,
            src,
        );
    };
}
pub unsafe extern "C" fn nettle_eax_aes128_decrypt(
    mut ctx: *mut eax_aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes128_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_eax_decrypt(
            &mut (*ctx).eax,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes128_ctx as *const libc::c_void,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes128_encrypt
                        as unsafe extern "C" fn(
                            *const aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            dst,
            src,
        );
    };
}
pub unsafe extern "C" fn nettle_eax_aes128_digest(
    mut ctx: *mut eax_aes128_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes128_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_eax_digest(
            &mut (*ctx).eax,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes128_ctx as *const libc::c_void,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes128_encrypt
                        as unsafe extern "C" fn(
                            *const aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            digest,
        );
    };
}
