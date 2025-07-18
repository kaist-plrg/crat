use ::libc;
extern "C" {
    static nettle_sm3: nettle_hash;
    static nettle_streebog512: nettle_hash;
    static nettle_streebog256: nettle_hash;
    static nettle_sha3_512: nettle_hash;
    static nettle_sha3_384: nettle_hash;
    static nettle_sha3_256: nettle_hash;
    static nettle_sha3_224: nettle_hash;
    static nettle_sha512: nettle_hash;
    static nettle_sha384: nettle_hash;
    static nettle_sha256: nettle_hash;
    static nettle_sha224: nettle_hash;
    static nettle_sha1: nettle_hash;
    static nettle_ripemd160: nettle_hash;
    static nettle_md5: nettle_hash;
    static nettle_md4: nettle_hash;
    static nettle_md2: nettle_hash;
    static nettle_gosthash94cp: nettle_hash;
    static nettle_gosthash94: nettle_hash;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
pub static mut _nettle_hashes: [*const nettle_hash; 19] = unsafe {
    [
        &nettle_gosthash94 as *const nettle_hash,
        &nettle_gosthash94cp as *const nettle_hash,
        &nettle_md2 as *const nettle_hash,
        &nettle_md4 as *const nettle_hash,
        &nettle_md5 as *const nettle_hash,
        &nettle_ripemd160 as *const nettle_hash,
        &nettle_sha1 as *const nettle_hash,
        &nettle_sha224 as *const nettle_hash,
        &nettle_sha256 as *const nettle_hash,
        &nettle_sha384 as *const nettle_hash,
        &nettle_sha512 as *const nettle_hash,
        &nettle_sha3_224 as *const nettle_hash,
        &nettle_sha3_256 as *const nettle_hash,
        &nettle_sha3_384 as *const nettle_hash,
        &nettle_sha3_512 as *const nettle_hash,
        &nettle_streebog256 as *const nettle_hash,
        &nettle_streebog512 as *const nettle_hash,
        &nettle_sm3 as *const nettle_hash,
        0 as *const nettle_hash,
    ]
};
pub unsafe extern "C" fn nettle_get_hashes() -> *const *const nettle_hash {
    return _nettle_hashes.as_ptr();
}
