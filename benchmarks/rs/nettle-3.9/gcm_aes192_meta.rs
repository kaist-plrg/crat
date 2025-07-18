use ::libc;
extern "C" {
    fn nettle_gcm_aes192_set_key(ctx: *mut gcm_aes192_ctx, key: *const uint8_t);
    fn nettle_gcm_aes192_update(
        ctx: *mut gcm_aes192_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_gcm_aes192_set_iv(
        ctx: *mut gcm_aes192_ctx,
        length: size_t,
        iv: *const uint8_t,
    );
    fn nettle_gcm_aes192_encrypt(
        ctx: *mut gcm_aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_aes192_decrypt(
        ctx: *mut gcm_aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_aes192_digest(
        ctx: *mut gcm_aes192_ctx,
        length: size_t,
        digest: *mut uint8_t,
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
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_crypt_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_aead {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub nonce_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub set_nonce: Option::<nettle_set_key_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub encrypt: Option::<nettle_crypt_func>,
    pub decrypt: Option::<nettle_crypt_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_aes192_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: aes192_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
unsafe extern "C" fn gcm_aes192_set_nonce_wrapper(
    mut ctx: *mut libc::c_void,
    mut nonce: *const uint8_t,
) {
    nettle_gcm_aes192_set_iv(
        ctx as *mut gcm_aes192_ctx,
        (16 as libc::c_int - 4 as libc::c_int) as size_t,
        nonce,
    );
}
pub static mut nettle_gcm_aes192: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"gcm_aes192\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<gcm_aes192_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 24 as libc::c_int as libc::c_uint,
            nonce_size: (16 as libc::c_int - 4 as libc::c_int) as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut gcm_aes192_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_gcm_aes192_set_key
                        as unsafe extern "C" fn(
                            *mut gcm_aes192_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut gcm_aes192_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_gcm_aes192_set_key
                        as unsafe extern "C" fn(
                            *mut gcm_aes192_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_nonce: Some(gcm_aes192_set_nonce_wrapper as nettle_set_key_func),
            update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gcm_aes192_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_gcm_aes192_update
                        as unsafe extern "C" fn(
                            *mut gcm_aes192_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gcm_aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_gcm_aes192_encrypt
                        as unsafe extern "C" fn(
                            *mut gcm_aes192_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gcm_aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_gcm_aes192_decrypt
                        as unsafe extern "C" fn(
                            *mut gcm_aes192_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut gcm_aes192_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_gcm_aes192_digest
                        as unsafe extern "C" fn(
                            *mut gcm_aes192_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
