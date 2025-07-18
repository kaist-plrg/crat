use ::libc;
extern "C" {
    fn nettle_arctwo40_set_key(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo64_set_key(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo128_set_key(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo128_set_key_gutmann(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo_encrypt(
        ctx: *mut arctwo_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_arctwo_decrypt(
        ctx: *mut arctwo_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub struct arctwo_ctx {
    pub S: [uint16_t; 64],
}
pub static mut nettle_arctwo40: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo40\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<arctwo_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: (40 as libc::c_int / 8 as libc::c_int) as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo40_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo40_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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
pub static mut nettle_arctwo64: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo64\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<arctwo_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: (64 as libc::c_int / 8 as libc::c_int) as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo64_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo64_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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
pub static mut nettle_arctwo128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<arctwo_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: (128 as libc::c_int / 8 as libc::c_int) as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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
pub static mut nettle_arctwo_gutmann128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo_gutmann128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<arctwo_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key_gutmann
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key_gutmann
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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
