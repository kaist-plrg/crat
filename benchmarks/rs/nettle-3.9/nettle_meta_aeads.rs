use ::libc;
extern "C" {
    static nettle_chacha_poly1305: nettle_aead;
    static nettle_eax_aes128: nettle_aead;
    static nettle_gcm_sm4: nettle_aead;
    static nettle_gcm_camellia256: nettle_aead;
    static nettle_gcm_camellia128: nettle_aead;
    static nettle_gcm_aes256: nettle_aead;
    static nettle_gcm_aes192: nettle_aead;
    static nettle_gcm_aes128: nettle_aead;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_crypt_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
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
pub struct nettle_aead {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub nonce_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub set_nonce: Option::<nettle_set_key_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub encrypt: Option::<nettle_crypt_func>,
    pub decrypt: Option::<nettle_crypt_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
pub static mut _nettle_aeads: [*const nettle_aead; 9] = unsafe {
    [
        &nettle_gcm_aes128 as *const nettle_aead,
        &nettle_gcm_aes192 as *const nettle_aead,
        &nettle_gcm_aes256 as *const nettle_aead,
        &nettle_gcm_camellia128 as *const nettle_aead,
        &nettle_gcm_camellia256 as *const nettle_aead,
        &nettle_gcm_sm4 as *const nettle_aead,
        &nettle_eax_aes128 as *const nettle_aead,
        &nettle_chacha_poly1305 as *const nettle_aead,
        0 as *const nettle_aead,
    ]
};
pub unsafe extern "C" fn nettle_get_aeads() -> *const *const nettle_aead {
    return _nettle_aeads.as_ptr();
}
