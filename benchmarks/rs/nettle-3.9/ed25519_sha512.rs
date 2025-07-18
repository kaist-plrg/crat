use ::libc;
extern "C" {
    fn nettle_sha512_update(ctx: *mut sha512_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha512_digest(ctx: *mut sha512_ctx, length: size_t, digest: *mut uint8_t);
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
pub type mp_limb_t = libc::c_ulong;
pub type nettle_eddsa_dom_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_eddsa {
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
    pub dom: Option::<nettle_eddsa_dom_func>,
    pub low_mask: mp_limb_t,
    pub high_bit: mp_limb_t,
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
unsafe extern "C" fn ed25519_dom(mut ctx: *mut libc::c_void) {}
pub static mut _nettle_ed25519_sha512: ecc_eddsa = unsafe {
    {
        let mut init = ecc_eddsa {
            update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut sha512_ctx, size_t, *const uint8_t) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_sha512_update
                        as unsafe extern "C" fn(
                            *mut sha512_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut sha512_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_sha512_digest
                        as unsafe extern "C" fn(
                            *mut sha512_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
            dom: Some(ed25519_dom as nettle_eddsa_dom_func),
            low_mask: !(7 as libc::c_int as mp_limb_t),
            high_bit: (1 as libc::c_int as mp_limb_t)
                << 254 as libc::c_int % (64 as libc::c_int - 0 as libc::c_int),
        };
        init
    }
};
