use ::libc;
extern "C" {
    fn nettle_pbkdf2(
        mac_ctx: *mut libc::c_void,
        update: Option::<nettle_hash_update_func>,
        digest: Option::<nettle_hash_digest_func>,
        digest_size: size_t,
        iterations: libc::c_uint,
        salt_length: size_t,
        salt: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
    );
    fn nettle_hmac_sha512_digest(
        ctx: *mut hmac_sha512_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_hmac_sha512_set_key(
        ctx: *mut hmac_sha512_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_sha512_update(
        ctx: *mut hmac_sha512_ctx,
        length: size_t,
        data: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
pub struct hmac_sha512_ctx {
    pub outer: sha512_ctx,
    pub inner: sha512_ctx,
    pub state: sha512_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 128],
}
pub unsafe extern "C" fn nettle_pbkdf2_hmac_sha512(
    mut key_length: size_t,
    mut key: *const uint8_t,
    mut iterations: libc::c_uint,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut sha512ctx: hmac_sha512_ctx = hmac_sha512_ctx {
        outer: sha512_ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; 128],
        },
        inner: sha512_ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; 128],
        },
        state: sha512_ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; 128],
        },
    };
    nettle_hmac_sha512_set_key(&mut sha512ctx, key_length, key);
    if 0 as libc::c_int != 0 {
        nettle_hmac_sha512_update(
            &mut sha512ctx,
            0 as libc::c_int as size_t,
            0 as *mut uint8_t,
        );
        nettle_hmac_sha512_digest(
            &mut sha512ctx,
            0 as libc::c_int as size_t,
            0 as *mut uint8_t,
        );
    } else {
        nettle_pbkdf2(
            &mut sha512ctx as *mut hmac_sha512_ctx as *mut libc::c_void,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut hmac_sha512_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_hmac_sha512_update
                        as unsafe extern "C" fn(
                            *mut hmac_sha512_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut hmac_sha512_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_hmac_sha512_digest
                        as unsafe extern "C" fn(
                            *mut hmac_sha512_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
            64 as libc::c_int as size_t,
            iterations,
            salt_length,
            salt,
            length,
            dst,
        );
    };
}
