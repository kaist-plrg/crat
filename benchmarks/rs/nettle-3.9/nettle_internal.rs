use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes192_set_encrypt_key(ctx: *mut aes192_ctx, key: *const uint8_t);
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_ocb_aes128_set_encrypt_key(
        ocb: *mut ocb_aes128_encrypt_key,
        key: *const uint8_t,
    );
    fn nettle_ocb_aes128_set_decrypt_key(
        ocb: *mut ocb_aes128_encrypt_key,
        decrypt: *mut aes128_ctx,
        key: *const uint8_t,
    );
    fn nettle_ocb_aes128_set_nonce(
        ctx: *mut ocb_ctx,
        key: *const ocb_aes128_encrypt_key,
        tag_length: size_t,
        nonce_length: size_t,
        nonce: *const uint8_t,
    );
    fn nettle_ocb_aes128_update(
        ctx: *mut ocb_ctx,
        key: *const ocb_aes128_encrypt_key,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_ocb_aes128_encrypt(
        ctx: *mut ocb_ctx,
        key: *const ocb_aes128_encrypt_key,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ocb_aes128_decrypt(
        ctx: *mut ocb_ctx,
        key: *const ocb_aes128_encrypt_key,
        decrypt: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ocb_aes128_digest(
        ctx: *mut ocb_ctx,
        key: *const ocb_aes128_encrypt_key,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_arcfour128_set_key(ctx: *mut arcfour_ctx, key: *const uint8_t);
    fn nettle_arcfour_crypt(
        ctx: *mut arcfour_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_blowfish128_set_key(
        ctx: *mut blowfish_ctx,
        key: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_blowfish_encrypt(
        ctx: *const blowfish_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_blowfish_decrypt(
        ctx: *const blowfish_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cbc_aes128_encrypt(
        ctx: *const aes128_ctx,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cbc_aes192_encrypt(
        ctx: *const aes192_ctx,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cbc_aes256_encrypt(
        ctx: *const aes256_ctx,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_chacha_set_key(ctx: *mut chacha_ctx, key: *const uint8_t);
    fn nettle_chacha_set_nonce(ctx: *mut chacha_ctx, nonce: *const uint8_t);
    fn nettle_chacha_crypt(
        ctx: *mut chacha_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_des_set_key(ctx: *mut des_ctx, key: *const uint8_t) -> libc::c_int;
    fn nettle_des_encrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_des_decrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_des3_set_key(ctx: *mut des3_ctx, key: *const uint8_t) -> libc::c_int;
    fn nettle_des3_encrypt(
        ctx: *const des3_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_des3_decrypt(
        ctx: *const des3_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_salsa20_256_set_key(ctx: *mut salsa20_ctx, key: *const uint8_t);
    fn nettle_salsa20_set_nonce(ctx: *mut salsa20_ctx, nonce: *const uint8_t);
    fn nettle_salsa20_crypt(
        ctx: *mut salsa20_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_salsa20r12_crypt(
        ctx: *mut salsa20_ctx,
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
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
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
pub struct nettle_cipher {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub encrypt: Option::<nettle_cipher_func>,
    pub decrypt: Option::<nettle_cipher_func>,
}
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
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des3_ctx {
    pub des: [des_ctx; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blowfish_ctx {
    pub s: [[uint32_t; 256]; 4],
    pub p: [uint32_t; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arcfour_ctx {
    pub S: [uint8_t; 256],
    pub i: uint8_t,
    pub j: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct salsa20_ctx {
    pub input: [uint32_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbc_aes128_ctx {
    pub ctx: aes128_ctx,
    pub iv: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbc_aes192_ctx {
    pub ctx: aes192_ctx,
    pub iv: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbc_aes256_ctx {
    pub ctx: aes256_ctx,
    pub iv: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ocb_aes128_ctx {
    pub ocb: ocb_ctx,
    pub key: ocb_aes128_encrypt_key,
    pub decrypt: aes128_ctx,
}
unsafe extern "C" fn des_set_key_wrapper(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    nettle_des_set_key(ctx as *mut des_ctx, key);
}
unsafe extern "C" fn des3_set_key_wrapper(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    nettle_des3_set_key(ctx as *mut des3_ctx, key);
}
unsafe extern "C" fn blowfish128_set_key_wrapper(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    nettle_blowfish128_set_key(ctx as *mut blowfish_ctx, key);
}
pub static mut nettle_des: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"des\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<des_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: 8 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                des_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                des_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const des_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_des_encrypt
                        as unsafe extern "C" fn(
                            *const des_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const des_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_des_decrypt
                        as unsafe extern "C" fn(
                            *const des_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub static mut nettle_des3: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"des3\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<des3_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: 24 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                des3_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                des3_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const des3_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_des3_encrypt
                        as unsafe extern "C" fn(
                            *const des3_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const des3_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_des3_decrypt
                        as unsafe extern "C" fn(
                            *const des3_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub static mut nettle_blowfish128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"blowfish128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<blowfish_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                blowfish128_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                blowfish128_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const blowfish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_blowfish_encrypt
                        as unsafe extern "C" fn(
                            *const blowfish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const blowfish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_blowfish_decrypt
                        as unsafe extern "C" fn(
                            *const blowfish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub static mut nettle_arcfour128: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"arcfour128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<arcfour_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 1 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            nonce_size: 0 as libc::c_int as libc::c_uint,
            digest_size: 0 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arcfour_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arcfour128_set_key
                        as unsafe extern "C" fn(*mut arcfour_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arcfour_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arcfour128_set_key
                        as unsafe extern "C" fn(*mut arcfour_ctx, *const uint8_t) -> (),
                ),
            ),
            set_nonce: None,
            update: None,
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arcfour_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_arcfour_crypt
                        as unsafe extern "C" fn(
                            *mut arcfour_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arcfour_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_arcfour_crypt
                        as unsafe extern "C" fn(
                            *mut arcfour_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: None,
        };
        init
    }
};
pub static mut nettle_chacha: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"chacha\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<chacha_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 64 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            nonce_size: 8 as libc::c_int as libc::c_uint,
            digest_size: 0 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut chacha_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_chacha_set_key
                        as unsafe extern "C" fn(*mut chacha_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut chacha_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_chacha_set_key
                        as unsafe extern "C" fn(*mut chacha_ctx, *const uint8_t) -> (),
                ),
            ),
            set_nonce: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut chacha_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_chacha_set_nonce
                        as unsafe extern "C" fn(*mut chacha_ctx, *const uint8_t) -> (),
                ),
            ),
            update: None,
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut chacha_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_chacha_crypt
                        as unsafe extern "C" fn(
                            *mut chacha_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut chacha_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_chacha_crypt
                        as unsafe extern "C" fn(
                            *mut chacha_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: None,
        };
        init
    }
};
pub static mut nettle_salsa20: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"salsa20\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<salsa20_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 64 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            nonce_size: 8 as libc::c_int as libc::c_uint,
            digest_size: 0 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_salsa20_256_set_key
                        as unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_salsa20_256_set_key
                        as unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> (),
                ),
            ),
            set_nonce: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_salsa20_set_nonce
                        as unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> (),
                ),
            ),
            update: None,
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut salsa20_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_salsa20_crypt
                        as unsafe extern "C" fn(
                            *mut salsa20_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut salsa20_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_salsa20_crypt
                        as unsafe extern "C" fn(
                            *mut salsa20_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: None,
        };
        init
    }
};
pub static mut nettle_salsa20r12: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"salsa20r12\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<salsa20_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 64 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            nonce_size: 8 as libc::c_int as libc::c_uint,
            digest_size: 0 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_salsa20_256_set_key
                        as unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_salsa20_256_set_key
                        as unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> (),
                ),
            ),
            set_nonce: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_salsa20_set_nonce
                        as unsafe extern "C" fn(*mut salsa20_ctx, *const uint8_t) -> (),
                ),
            ),
            update: None,
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut salsa20_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_salsa20r12_crypt
                        as unsafe extern "C" fn(
                            *mut salsa20_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut salsa20_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_salsa20r12_crypt
                        as unsafe extern "C" fn(
                            *mut salsa20_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: None,
        };
        init
    }
};
unsafe extern "C" fn cbc_aes128_set_encrypt_key(
    mut ctx: *mut cbc_aes128_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes128_set_encrypt_key(&mut (*ctx).ctx, key);
}
unsafe extern "C" fn cbc_aes128_set_iv(
    mut ctx: *mut cbc_aes128_ctx,
    mut iv: *const uint8_t,
) {
    memcpy(
        ((*ctx).iv).as_mut_ptr() as *mut libc::c_void,
        iv as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn cbc_aes128_encrypt_wrapper(
    mut ctx: *mut cbc_aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_cbc_aes128_encrypt(
        &mut (*ctx).ctx,
        ((*ctx).iv).as_mut_ptr(),
        length,
        dst,
        src,
    );
}
pub static mut nettle_cbc_aes128: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"cbc_aes128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<cbc_aes128_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            nonce_size: 16 as libc::c_int as libc::c_uint,
            digest_size: 0 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut cbc_aes128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    cbc_aes128_set_encrypt_key
                        as unsafe extern "C" fn(
                            *mut cbc_aes128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: None,
            set_nonce: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut cbc_aes128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    cbc_aes128_set_iv
                        as unsafe extern "C" fn(
                            *mut cbc_aes128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            update: None,
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cbc_aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    cbc_aes128_encrypt_wrapper
                        as unsafe extern "C" fn(
                            *mut cbc_aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: None,
            digest: None,
        };
        init
    }
};
unsafe extern "C" fn cbc_aes192_set_encrypt_key(
    mut ctx: *mut cbc_aes192_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes192_set_encrypt_key(&mut (*ctx).ctx, key);
}
unsafe extern "C" fn cbc_aes192_set_iv(
    mut ctx: *mut cbc_aes192_ctx,
    mut iv: *const uint8_t,
) {
    memcpy(
        ((*ctx).iv).as_mut_ptr() as *mut libc::c_void,
        iv as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn cbc_aes192_encrypt_wrapper(
    mut ctx: *mut cbc_aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_cbc_aes192_encrypt(
        &mut (*ctx).ctx,
        ((*ctx).iv).as_mut_ptr(),
        length,
        dst,
        src,
    );
}
pub static mut nettle_cbc_aes192: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"cbc_aes192\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<cbc_aes192_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 24 as libc::c_int as libc::c_uint,
            nonce_size: 16 as libc::c_int as libc::c_uint,
            digest_size: 0 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut cbc_aes192_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    cbc_aes192_set_encrypt_key
                        as unsafe extern "C" fn(
                            *mut cbc_aes192_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: None,
            set_nonce: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut cbc_aes192_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    cbc_aes192_set_iv
                        as unsafe extern "C" fn(
                            *mut cbc_aes192_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            update: None,
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cbc_aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    cbc_aes192_encrypt_wrapper
                        as unsafe extern "C" fn(
                            *mut cbc_aes192_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: None,
            digest: None,
        };
        init
    }
};
unsafe extern "C" fn cbc_aes256_set_encrypt_key(
    mut ctx: *mut cbc_aes256_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes256_set_encrypt_key(&mut (*ctx).ctx, key);
}
unsafe extern "C" fn cbc_aes256_set_iv(
    mut ctx: *mut cbc_aes256_ctx,
    mut iv: *const uint8_t,
) {
    memcpy(
        ((*ctx).iv).as_mut_ptr() as *mut libc::c_void,
        iv as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn cbc_aes256_encrypt_wrapper(
    mut ctx: *mut cbc_aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_cbc_aes256_encrypt(
        &mut (*ctx).ctx,
        ((*ctx).iv).as_mut_ptr(),
        length,
        dst,
        src,
    );
}
pub static mut nettle_cbc_aes256: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"cbc_aes256\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<cbc_aes256_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            nonce_size: 16 as libc::c_int as libc::c_uint,
            digest_size: 0 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut cbc_aes256_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    cbc_aes256_set_encrypt_key
                        as unsafe extern "C" fn(
                            *mut cbc_aes256_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: None,
            set_nonce: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut cbc_aes256_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    cbc_aes256_set_iv
                        as unsafe extern "C" fn(
                            *mut cbc_aes256_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            update: None,
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cbc_aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    cbc_aes256_encrypt_wrapper
                        as unsafe extern "C" fn(
                            *mut cbc_aes256_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: None,
            digest: None,
        };
        init
    }
};
unsafe extern "C" fn ocb_aes128_set_encrypt_key_wrapper(
    mut ctx: *mut ocb_aes128_ctx,
    mut key: *const uint8_t,
) {
    nettle_ocb_aes128_set_encrypt_key(&mut (*ctx).key, key);
}
unsafe extern "C" fn ocb_aes128_set_decrypt_key_wrapper(
    mut ctx: *mut ocb_aes128_ctx,
    mut key: *const uint8_t,
) {
    nettle_ocb_aes128_set_decrypt_key(&mut (*ctx).key, &mut (*ctx).decrypt, key);
}
unsafe extern "C" fn ocb_aes128_set_nonce_wrapper(
    mut ctx: *mut ocb_aes128_ctx,
    mut nonce: *const uint8_t,
) {
    nettle_ocb_aes128_set_nonce(
        &mut (*ctx).ocb,
        &mut (*ctx).key,
        16 as libc::c_int as size_t,
        12 as libc::c_int as size_t,
        nonce,
    );
}
unsafe extern "C" fn ocb_aes128_update_wrapper(
    mut ctx: *mut ocb_aes128_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_ocb_aes128_update(&mut (*ctx).ocb, &mut (*ctx).key, length, data);
}
unsafe extern "C" fn ocb_aes128_encrypt_wrapper(
    mut ctx: *mut ocb_aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ocb_aes128_encrypt(&mut (*ctx).ocb, &mut (*ctx).key, length, dst, src);
}
unsafe extern "C" fn ocb_aes128_decrypt_wrapper(
    mut ctx: *mut ocb_aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ocb_aes128_decrypt(
        &mut (*ctx).ocb,
        &mut (*ctx).key,
        &mut (*ctx).decrypt,
        length,
        dst,
        src,
    );
}
unsafe extern "C" fn ocb_aes128_digest_wrapper(
    mut ctx: *mut ocb_aes128_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_ocb_aes128_digest(&mut (*ctx).ocb, &mut (*ctx).key, length, digest);
}
pub static mut nettle_ocb_aes128: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"ocb_aes128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<ocb_aes128_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            nonce_size: 12 as libc::c_int as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut ocb_aes128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    ocb_aes128_set_encrypt_key_wrapper
                        as unsafe extern "C" fn(
                            *mut ocb_aes128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut ocb_aes128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    ocb_aes128_set_decrypt_key_wrapper
                        as unsafe extern "C" fn(
                            *mut ocb_aes128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_nonce: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut ocb_aes128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    ocb_aes128_set_nonce_wrapper
                        as unsafe extern "C" fn(
                            *mut ocb_aes128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut ocb_aes128_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    ocb_aes128_update_wrapper
                        as unsafe extern "C" fn(
                            *mut ocb_aes128_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut ocb_aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    ocb_aes128_encrypt_wrapper
                        as unsafe extern "C" fn(
                            *mut ocb_aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut ocb_aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    ocb_aes128_decrypt_wrapper
                        as unsafe extern "C" fn(
                            *mut ocb_aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut ocb_aes128_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    ocb_aes128_digest_wrapper
                        as unsafe extern "C" fn(
                            *mut ocb_aes128_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
