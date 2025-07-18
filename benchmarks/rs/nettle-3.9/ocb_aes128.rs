use ::libc;
extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_invert_key(dst: *mut aes128_ctx, src: *const aes128_ctx);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes128_decrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ocb_set_key(
        key: *mut ocb_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
    );
    fn nettle_ocb_set_nonce(
        ctx: *mut ocb_ctx,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        tag_length: size_t,
        nonce_length: size_t,
        nonce: *const uint8_t,
    );
    fn nettle_ocb_update(
        ctx: *mut ocb_ctx,
        key: *const ocb_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_ocb_encrypt(
        ctx: *mut ocb_ctx,
        key: *const ocb_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ocb_decrypt(
        ctx: *mut ocb_ctx,
        key: *const ocb_key,
        encrypt_ctx: *const libc::c_void,
        encrypt: Option::<nettle_cipher_func>,
        decrypt_ctx: *const libc::c_void,
        decrypt: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ocb_digest(
        ctx: *const ocb_ctx,
        key: *const ocb_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_ocb_encrypt_message(
        ocb_key: *const ocb_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ocb_decrypt_message(
        ocb_key: *const ocb_key,
        encrypt_ctx: *const libc::c_void,
        encrypt: Option::<nettle_cipher_func>,
        decrypt_ctx: *const libc::c_void,
        decrypt: Option::<nettle_cipher_func>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ocb_aes128_encrypt_key {
    pub ocb: ocb_key,
    pub encrypt: aes128_ctx,
}
pub unsafe extern "C" fn nettle_ocb_aes128_set_encrypt_key(
    mut ocb_key: *mut ocb_aes128_encrypt_key,
    mut key: *const uint8_t,
) {
    nettle_aes128_set_encrypt_key(&mut (*ocb_key).encrypt, key);
    nettle_ocb_set_key(
        &mut (*ocb_key).ocb,
        &mut (*ocb_key).encrypt as *mut aes128_ctx as *const libc::c_void,
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
pub unsafe extern "C" fn nettle_ocb_aes128_set_decrypt_key(
    mut ocb_key: *mut ocb_aes128_encrypt_key,
    mut decrypt: *mut aes128_ctx,
    mut key: *const uint8_t,
) {
    nettle_ocb_aes128_set_encrypt_key(ocb_key, key);
    nettle_aes128_invert_key(decrypt, &mut (*ocb_key).encrypt);
}
pub unsafe extern "C" fn nettle_ocb_aes128_set_nonce(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_aes128_encrypt_key,
    mut tag_length: size_t,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    nettle_ocb_set_nonce(
        ctx,
        &(*key).encrypt as *const aes128_ctx as *const libc::c_void,
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
        tag_length,
        nonce_length,
        nonce,
    );
}
pub unsafe extern "C" fn nettle_ocb_aes128_update(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_aes128_encrypt_key,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_ocb_update(
        ctx,
        &(*key).ocb,
        &(*key).encrypt as *const aes128_ctx as *const libc::c_void,
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
}
pub unsafe extern "C" fn nettle_ocb_aes128_encrypt(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_aes128_encrypt_key,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ocb_encrypt(
        ctx,
        &(*key).ocb,
        &(*key).encrypt as *const aes128_ctx as *const libc::c_void,
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
}
pub unsafe extern "C" fn nettle_ocb_aes128_decrypt(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_aes128_encrypt_key,
    mut decrypt: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ocb_decrypt(
        ctx,
        &(*key).ocb,
        &(*key).encrypt as *const aes128_ctx as *const libc::c_void,
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
        decrypt as *const libc::c_void,
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
                nettle_aes128_decrypt
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
}
pub unsafe extern "C" fn nettle_ocb_aes128_digest(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_aes128_encrypt_key,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_ocb_digest(
        ctx,
        &(*key).ocb,
        &(*key).encrypt as *const aes128_ctx as *const libc::c_void,
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
}
pub unsafe extern "C" fn nettle_ocb_aes128_encrypt_message(
    mut key: *const ocb_aes128_encrypt_key,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ocb_encrypt_message(
        &(*key).ocb,
        &(*key).encrypt as *const aes128_ctx as *const libc::c_void,
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
        nlength,
        nonce,
        alength,
        adata,
        tlength,
        clength,
        dst,
        src,
    );
}
pub unsafe extern "C" fn nettle_ocb_aes128_decrypt_message(
    mut key: *const ocb_aes128_encrypt_key,
    mut decrypt: *const aes128_ctx,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> libc::c_int {
    return nettle_ocb_decrypt_message(
        &(*key).ocb,
        &(*key).encrypt as *const aes128_ctx as *const libc::c_void,
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
        &mut decrypt as *mut *const aes128_ctx as *const libc::c_void,
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
                nettle_aes128_decrypt
                    as unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        nlength,
        nonce,
        alength,
        adata,
        tlength,
        mlength,
        dst,
        src,
    );
}
