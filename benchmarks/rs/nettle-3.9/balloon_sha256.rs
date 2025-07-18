use ::libc;
extern "C" {
    fn nettle_balloon(
        hash_ctx: *mut libc::c_void,
        update: Option::<nettle_hash_update_func>,
        digest: Option::<nettle_hash_digest_func>,
        digest_size: size_t,
        s_cost: size_t,
        t_cost: size_t,
        passwd_length: size_t,
        passwd: *const uint8_t,
        salt_length: size_t,
        salt: *const uint8_t,
        scratch: *mut uint8_t,
        dst: *mut uint8_t,
    );
    fn nettle_sha256_init(ctx: *mut sha256_ctx);
    fn nettle_sha256_update(ctx: *mut sha256_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha256_digest(ctx: *mut sha256_ctx, length: size_t, digest: *mut uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
pub unsafe extern "C" fn nettle_balloon_sha256(
    mut s_cost: size_t,
    mut t_cost: size_t,
    mut passwd_length: size_t,
    mut passwd: *const uint8_t,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut scratch: *mut uint8_t,
    mut dst: *mut uint8_t,
) {
    let mut ctx: sha256_ctx = sha256_ctx {
        state: [0; 8],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    nettle_sha256_init(&mut ctx);
    nettle_balloon(
        &mut ctx as *mut sha256_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut sha256_ctx, size_t, *const uint8_t) -> (),
            >,
            Option::<nettle_hash_update_func>,
        >(
            Some(
                nettle_sha256_update
                    as unsafe extern "C" fn(
                        *mut sha256_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut sha256_ctx, size_t, *mut uint8_t) -> ()>,
            Option::<nettle_hash_digest_func>,
        >(
            Some(
                nettle_sha256_digest
                    as unsafe extern "C" fn(*mut sha256_ctx, size_t, *mut uint8_t) -> (),
            ),
        ),
        32 as libc::c_int as size_t,
        s_cost,
        t_cost,
        passwd_length,
        passwd,
        salt_length,
        salt,
        scratch,
        dst,
    );
}
