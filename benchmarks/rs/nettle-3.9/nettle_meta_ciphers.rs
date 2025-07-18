use ::libc;
extern "C" {
    static nettle_sm4: nettle_cipher;
    static nettle_arctwo_gutmann128: nettle_cipher;
    static nettle_arctwo128: nettle_cipher;
    static nettle_arctwo64: nettle_cipher;
    static nettle_arctwo40: nettle_cipher;
    static nettle_twofish256: nettle_cipher;
    static nettle_twofish192: nettle_cipher;
    static nettle_twofish128: nettle_cipher;
    static nettle_serpent256: nettle_cipher;
    static nettle_serpent192: nettle_cipher;
    static nettle_serpent128: nettle_cipher;
    static nettle_cast128: nettle_cipher;
    static nettle_camellia256: nettle_cipher;
    static nettle_camellia192: nettle_cipher;
    static nettle_camellia128: nettle_cipher;
    static nettle_aes256: nettle_cipher;
    static nettle_aes192: nettle_cipher;
    static nettle_aes128: nettle_cipher;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub encrypt: Option::<nettle_cipher_func>,
    pub decrypt: Option::<nettle_cipher_func>,
}
pub static mut _nettle_ciphers: [*const nettle_cipher; 19] = unsafe {
    [
        &nettle_aes128 as *const nettle_cipher,
        &nettle_aes192 as *const nettle_cipher,
        &nettle_aes256 as *const nettle_cipher,
        &nettle_camellia128 as *const nettle_cipher,
        &nettle_camellia192 as *const nettle_cipher,
        &nettle_camellia256 as *const nettle_cipher,
        &nettle_cast128 as *const nettle_cipher,
        &nettle_serpent128 as *const nettle_cipher,
        &nettle_serpent192 as *const nettle_cipher,
        &nettle_serpent256 as *const nettle_cipher,
        &nettle_twofish128 as *const nettle_cipher,
        &nettle_twofish192 as *const nettle_cipher,
        &nettle_twofish256 as *const nettle_cipher,
        &nettle_arctwo40 as *const nettle_cipher,
        &nettle_arctwo64 as *const nettle_cipher,
        &nettle_arctwo128 as *const nettle_cipher,
        &nettle_arctwo_gutmann128 as *const nettle_cipher,
        &nettle_sm4 as *const nettle_cipher,
        0 as *const nettle_cipher,
    ]
};
pub unsafe extern "C" fn nettle_get_ciphers() -> *const *const nettle_cipher {
    return _nettle_ciphers.as_ptr();
}
