use ::libc;
extern "C" {
    fn nettle_sha3_256_update(
        ctx: *mut sha3_256_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_sha3_256_shake(
        ctx: *mut sha3_256_ctx,
        length: size_t,
        digest: *mut uint8_t,
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
pub struct sha3_256_ctx {
    pub state: sha3_state,
    pub index: libc::c_uint,
    pub block: [uint8_t; 136],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
unsafe extern "C" fn ed448_dom(mut ctx: *mut libc::c_void) {
    static mut dom: [uint8_t; 10] = [
        'S' as i32 as uint8_t,
        'i' as i32 as uint8_t,
        'g' as i32 as uint8_t,
        'E' as i32 as uint8_t,
        'd' as i32 as uint8_t,
        '4' as i32 as uint8_t,
        '4' as i32 as uint8_t,
        '8' as i32 as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    nettle_sha3_256_update(
        ctx as *mut sha3_256_ctx,
        10 as libc::c_int as size_t,
        dom.as_ptr(),
    );
}
pub static mut _nettle_ed448_shake256: ecc_eddsa = unsafe {
    {
        let mut init = ecc_eddsa {
            update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut sha3_256_ctx, size_t, *const uint8_t) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_sha3_256_update
                        as unsafe extern "C" fn(
                            *mut sha3_256_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut sha3_256_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_sha3_256_shake
                        as unsafe extern "C" fn(
                            *mut sha3_256_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
            dom: Some(ed448_dom as nettle_eddsa_dom_func),
            low_mask: !(3 as libc::c_int as mp_limb_t),
            high_bit: (1 as libc::c_int as mp_limb_t)
                << 447 as libc::c_int % (64 as libc::c_int - 0 as libc::c_int),
        };
        init
    }
};
