use ::libc;
extern "C" {
    fn nettle_serpent128_set_key(ctx: *mut serpent_ctx, key: *const uint8_t);
    fn nettle_serpent192_set_key(ctx: *mut serpent_ctx, key: *const uint8_t);
    fn nettle_serpent256_set_key(ctx: *mut serpent_ctx, key: *const uint8_t);
    fn nettle_serpent_encrypt(
        ctx: *const serpent_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_serpent_decrypt(
        ctx: *const serpent_ctx,
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
pub struct serpent_ctx {
    pub keys: [[uint32_t; 4]; 33],
}
pub static mut nettle_serpent128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"serpent128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<serpent_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_serpent128_set_key
                        as unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_serpent128_set_key
                        as unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const serpent_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_serpent_encrypt
                        as unsafe extern "C" fn(
                            *const serpent_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const serpent_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_serpent_decrypt
                        as unsafe extern "C" fn(
                            *const serpent_ctx,
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
pub static mut nettle_serpent192: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"serpent192\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<serpent_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 24 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_serpent192_set_key
                        as unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_serpent192_set_key
                        as unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const serpent_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_serpent_encrypt
                        as unsafe extern "C" fn(
                            *const serpent_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const serpent_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_serpent_decrypt
                        as unsafe extern "C" fn(
                            *const serpent_ctx,
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
pub static mut nettle_serpent256: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"serpent256\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<serpent_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_serpent256_set_key
                        as unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_serpent256_set_key
                        as unsafe extern "C" fn(*mut serpent_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const serpent_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_serpent_encrypt
                        as unsafe extern "C" fn(
                            *const serpent_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const serpent_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_serpent_decrypt
                        as unsafe extern "C" fn(
                            *const serpent_ctx,
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
