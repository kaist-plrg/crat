use ::libc;
extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cmac128_set_key(
        key: *mut cmac128_key,
        cipher: *const libc::c_void,
        encrypt: Option::<nettle_cipher_func>,
    );
    fn nettle_cmac128_init(ctx: *mut cmac128_ctx);
    fn nettle_cmac128_update(
        ctx: *mut cmac128_ctx,
        cipher: *const libc::c_void,
        encrypt: Option::<nettle_cipher_func>,
        msg_len: size_t,
        msg: *const uint8_t,
    );
    fn nettle_cmac128_digest(
        ctx: *mut cmac128_ctx,
        key: *const cmac128_key,
        cipher: *const libc::c_void,
        encrypt: Option::<nettle_cipher_func>,
        length: libc::c_uint,
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
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_ctx {
    pub X: nettle_block16,
    pub block: nettle_block16,
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac_aes128_ctx {
    pub key: cmac128_key,
    pub ctx: cmac128_ctx,
    pub cipher: aes128_ctx,
}
pub unsafe extern "C" fn nettle_cmac_aes128_set_key(
    mut ctx: *mut cmac_aes128_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes128_set_encrypt_key(&mut (*ctx).cipher, key);
    nettle_cmac128_set_key(
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
    nettle_cmac128_init(&mut (*ctx).ctx);
}
pub unsafe extern "C" fn nettle_cmac_aes128_update(
    mut ctx: *mut cmac_aes128_ctx,
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
        nettle_cmac128_update(
            &mut (*ctx).ctx,
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
pub unsafe extern "C" fn nettle_cmac_aes128_digest(
    mut ctx: *mut cmac_aes128_ctx,
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
        nettle_cmac128_digest(
            &mut (*ctx).ctx,
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
            length as libc::c_uint,
            digest,
        );
    };
}
