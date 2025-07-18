use ::libc;
extern "C" {
    static nettle_gosthash94: nettle_hash;
    static nettle_gosthash94cp: nettle_hash;
    fn nettle_gosthash94_update(
        ctx: *mut gosthash94_ctx,
        length: size_t,
        msg: *const uint8_t,
    );
    fn nettle_gosthash94cp_update(
        ctx: *mut gosthash94_ctx,
        length: size_t,
        msg: *const uint8_t,
    );
    fn nettle_hmac_set_key(
        outer: *mut libc::c_void,
        inner: *mut libc::c_void,
        state: *mut libc::c_void,
        hash: *const nettle_hash,
        length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_digest(
        outer: *const libc::c_void,
        inner: *const libc::c_void,
        state: *mut libc::c_void,
        hash: *const nettle_hash,
        length: size_t,
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
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub struct nettle_hash {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gosthash94_ctx {
    pub hash: [uint32_t; 8],
    pub sum: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_gosthash94_ctx {
    pub outer: gosthash94_ctx,
    pub inner: gosthash94_ctx,
    pub state: gosthash94_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_gosthash94cp_ctx {
    pub outer: gosthash94_ctx,
    pub inner: gosthash94_ctx,
    pub state: gosthash94_ctx,
}
pub unsafe extern "C" fn nettle_hmac_gosthash94_set_key(
    mut ctx: *mut hmac_gosthash94_ctx,
    mut key_length: size_t,
    mut key: *const uint8_t,
) {
    nettle_hmac_set_key(
        &mut (*ctx).outer as *mut gosthash94_ctx as *mut libc::c_void,
        &mut (*ctx).inner as *mut gosthash94_ctx as *mut libc::c_void,
        &mut (*ctx).state as *mut gosthash94_ctx as *mut libc::c_void,
        &nettle_gosthash94,
        key_length,
        key,
    );
}
pub unsafe extern "C" fn nettle_hmac_gosthash94_update(
    mut ctx: *mut hmac_gosthash94_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_gosthash94_update(&mut (*ctx).state, length, data);
}
pub unsafe extern "C" fn nettle_hmac_gosthash94_digest(
    mut ctx: *mut hmac_gosthash94_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_hmac_digest(
        &mut (*ctx).outer as *mut gosthash94_ctx as *const libc::c_void,
        &mut (*ctx).inner as *mut gosthash94_ctx as *const libc::c_void,
        &mut (*ctx).state as *mut gosthash94_ctx as *mut libc::c_void,
        &nettle_gosthash94,
        length,
        digest,
    );
}
pub unsafe extern "C" fn nettle_hmac_gosthash94cp_set_key(
    mut ctx: *mut hmac_gosthash94cp_ctx,
    mut key_length: size_t,
    mut key: *const uint8_t,
) {
    nettle_hmac_set_key(
        &mut (*ctx).outer as *mut gosthash94_ctx as *mut libc::c_void,
        &mut (*ctx).inner as *mut gosthash94_ctx as *mut libc::c_void,
        &mut (*ctx).state as *mut gosthash94_ctx as *mut libc::c_void,
        &nettle_gosthash94cp,
        key_length,
        key,
    );
}
pub unsafe extern "C" fn nettle_hmac_gosthash94cp_update(
    mut ctx: *mut hmac_gosthash94cp_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_gosthash94cp_update(&mut (*ctx).state, length, data);
}
pub unsafe extern "C" fn nettle_hmac_gosthash94cp_digest(
    mut ctx: *mut hmac_gosthash94cp_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_hmac_digest(
        &mut (*ctx).outer as *mut gosthash94_ctx as *const libc::c_void,
        &mut (*ctx).inner as *mut gosthash94_ctx as *const libc::c_void,
        &mut (*ctx).state as *mut gosthash94_ctx as *mut libc::c_void,
        &nettle_gosthash94cp,
        length,
        digest,
    );
}
