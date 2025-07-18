use ::libc;
extern "C" {
    fn nettle_des3_set_key(ctx: *mut des3_ctx, key: *const uint8_t) -> libc::c_int;
    fn nettle_des3_encrypt(
        ctx: *const des3_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cmac64_set_key(
        key: *mut cmac64_key,
        cipher: *const libc::c_void,
        encrypt: Option::<nettle_cipher_func>,
    );
    fn nettle_cmac64_init(ctx: *mut cmac64_ctx);
    fn nettle_cmac64_update(
        ctx: *mut cmac64_ctx,
        cipher: *const libc::c_void,
        encrypt: Option::<nettle_cipher_func>,
        msg_len: size_t,
        msg: *const uint8_t,
    );
    fn nettle_cmac64_digest(
        ctx: *mut cmac64_ctx,
        key: *const cmac64_key,
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
pub union nettle_block8 {
    pub b: [uint8_t; 8],
    pub u64_0: uint64_t,
}
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
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
pub struct cmac64_key {
    pub K1: nettle_block8,
    pub K2: nettle_block8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac64_ctx {
    pub X: nettle_block8,
    pub block: nettle_block8,
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac_des3_ctx {
    pub key: cmac64_key,
    pub ctx: cmac64_ctx,
    pub cipher: des3_ctx,
}
pub unsafe extern "C" fn nettle_cmac_des3_set_key(
    mut ctx: *mut cmac_des3_ctx,
    mut key: *const uint8_t,
) {
    nettle_des3_set_key(&mut (*ctx).cipher, key);
    nettle_cmac64_set_key(
        &mut (*ctx).key,
        &mut (*ctx).cipher as *mut des3_ctx as *const libc::c_void,
        ::std::mem::transmute::<
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
    );
    nettle_cmac64_init(&mut (*ctx).ctx);
}
pub unsafe extern "C" fn nettle_cmac_des3_update(
    mut ctx: *mut cmac_des3_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_des3_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_cmac64_update(
            &mut (*ctx).ctx,
            &mut (*ctx).cipher as *mut des3_ctx as *const libc::c_void,
            ::std::mem::transmute::<
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
            length,
            data,
        );
    };
}
pub unsafe extern "C" fn nettle_cmac_des3_digest(
    mut ctx: *mut cmac_des3_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_des3_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_cmac64_digest(
            &mut (*ctx).ctx,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut des3_ctx as *const libc::c_void,
            ::std::mem::transmute::<
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
            length as libc::c_uint,
            digest,
        );
    };
}
