use ::libc;
extern "C" {
    static nettle_cmac_aes128: nettle_mac;
    static nettle_cmac_aes256: nettle_mac;
    static nettle_cmac_des3: nettle_mac;
    static nettle_hmac_md5: nettle_mac;
    static nettle_hmac_ripemd160: nettle_mac;
    static nettle_hmac_sha1: nettle_mac;
    static nettle_hmac_sha224: nettle_mac;
    static nettle_hmac_sm3: nettle_mac;
    static nettle_hmac_streebog512: nettle_mac;
    static nettle_hmac_streebog256: nettle_mac;
    static nettle_hmac_sha512: nettle_mac;
    static nettle_hmac_sha384: nettle_mac;
    static nettle_hmac_sha256: nettle_mac;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
pub static mut _nettle_macs: [*const nettle_mac; 14] = unsafe {
    [
        &nettle_cmac_aes128 as *const nettle_mac,
        &nettle_cmac_aes256 as *const nettle_mac,
        &nettle_cmac_des3 as *const nettle_mac,
        &nettle_hmac_md5 as *const nettle_mac,
        &nettle_hmac_ripemd160 as *const nettle_mac,
        &nettle_hmac_sha1 as *const nettle_mac,
        &nettle_hmac_sha224 as *const nettle_mac,
        &nettle_hmac_sha256 as *const nettle_mac,
        &nettle_hmac_sha384 as *const nettle_mac,
        &nettle_hmac_sha512 as *const nettle_mac,
        &nettle_hmac_streebog256 as *const nettle_mac,
        &nettle_hmac_streebog512 as *const nettle_mac,
        &nettle_hmac_sm3 as *const nettle_mac,
        0 as *const nettle_mac,
    ]
};
pub unsafe extern "C" fn nettle_get_macs() -> *const *const nettle_mac {
    return _nettle_macs.as_ptr();
}
