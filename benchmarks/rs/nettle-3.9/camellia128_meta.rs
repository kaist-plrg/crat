use ::libc;
extern "C" {
    fn nettle_camellia128_set_encrypt_key(
        ctx: *mut camellia128_ctx,
        key: *const uint8_t,
    );
    fn nettle_camellia_set_decrypt_key(ctx: *mut camellia128_ctx, key: *const uint8_t);
    fn nettle_camellia128_crypt(
        ctx: *const camellia128_ctx,
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
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
}
pub static mut nettle_camellia128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"camellia128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<camellia128_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut camellia128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_camellia128_set_encrypt_key
                        as unsafe extern "C" fn(
                            *mut camellia128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut camellia128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_camellia_set_decrypt_key
                        as unsafe extern "C" fn(
                            *mut camellia128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            encrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const camellia128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_camellia128_crypt
                        as unsafe extern "C" fn(
                            *const camellia128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const camellia128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_camellia128_crypt
                        as unsafe extern "C" fn(
                            *const camellia128_ctx,
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
