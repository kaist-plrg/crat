use ::libc;
extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_set_decrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_decrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_xts_encrypt_message(
        enc_ctx: *const libc::c_void,
        twk_ctx: *const libc::c_void,
        encf: Option::<nettle_cipher_func>,
        tweak: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_xts_decrypt_message(
        dec_ctx: *const libc::c_void,
        twk_ctx: *const libc::c_void,
        decf: Option::<nettle_cipher_func>,
        encf: Option::<nettle_cipher_func>,
        tweak: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xts_aes256_key {
    pub cipher: aes256_ctx,
    pub tweak_cipher: aes256_ctx,
}
pub unsafe extern "C" fn nettle_xts_aes256_set_encrypt_key(
    mut xts_key: *mut xts_aes256_key,
    mut key: *const uint8_t,
) {
    nettle_aes256_set_encrypt_key(&mut (*xts_key).cipher, key);
    nettle_aes256_set_encrypt_key(
        &mut (*xts_key).tweak_cipher,
        &*key.offset(32 as libc::c_int as isize),
    );
}
pub unsafe extern "C" fn nettle_xts_aes256_set_decrypt_key(
    mut xts_key: *mut xts_aes256_key,
    mut key: *const uint8_t,
) {
    nettle_aes256_set_decrypt_key(&mut (*xts_key).cipher, key);
    nettle_aes256_set_encrypt_key(
        &mut (*xts_key).tweak_cipher,
        &*key.offset(32 as libc::c_int as isize),
    );
}
pub unsafe extern "C" fn nettle_xts_aes256_encrypt_message(
    mut xts_key: *const xts_aes256_key,
    mut tweak: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_xts_encrypt_message(
        &(*xts_key).cipher as *const aes256_ctx as *const libc::c_void,
        &(*xts_key).tweak_cipher as *const aes256_ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        tweak,
        length,
        dst,
        src,
    );
}
pub unsafe extern "C" fn nettle_xts_aes256_decrypt_message(
    mut xts_key: *const xts_aes256_key,
    mut tweak: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_xts_decrypt_message(
        &(*xts_key).cipher as *const aes256_ctx as *const libc::c_void,
        &(*xts_key).tweak_cipher as *const aes256_ctx as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_decrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        tweak,
        length,
        dst,
        src,
    );
}
