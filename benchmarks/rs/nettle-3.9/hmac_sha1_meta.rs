use ::libc;
extern "C" {
    fn nettle_hmac_sha1_update(
        ctx: *mut hmac_sha1_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_sha1_digest(
        ctx: *mut hmac_sha1_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_hmac_sha1_set_key(
        ctx: *mut hmac_sha1_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
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
pub struct nettle_mac {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub set_key: Option::<nettle_set_key_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha1_ctx {
    pub outer: sha1_ctx,
    pub inner: sha1_ctx,
    pub state: sha1_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
unsafe extern "C" fn hmac_sha1_set_key_wrapper(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    nettle_hmac_sha1_set_key(
        ctx as *mut hmac_sha1_ctx,
        20 as libc::c_int as size_t,
        key,
    );
}
pub static mut nettle_hmac_sha1: nettle_mac = unsafe {
    {
        let mut init = nettle_mac {
            name: b"hmac_sha1\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<hmac_sha1_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 20 as libc::c_int as libc::c_uint,
            key_size: 20 as libc::c_int as libc::c_uint,
            set_key: Some(
                hmac_sha1_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut hmac_sha1_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_hmac_sha1_update
                        as unsafe extern "C" fn(
                            *mut hmac_sha1_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut hmac_sha1_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_hmac_sha1_digest
                        as unsafe extern "C" fn(
                            *mut hmac_sha1_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
