use ::libc;
extern "C" {
    static nettle_aes128: nettle_cipher;
    fn nettle_siv_cmac_set_key(
        cmac_key: *mut cmac128_key,
        cmac_cipher: *mut libc::c_void,
        ctr_cipher: *mut libc::c_void,
        nc: *const nettle_cipher,
        key: *const uint8_t,
    );
    fn nettle_siv_cmac_encrypt_message(
        cmac_key: *const cmac128_key,
        cmac_cipher_ctx: *const libc::c_void,
        nc: *const nettle_cipher,
        ctr_ctx: *const libc::c_void,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_siv_cmac_decrypt_message(
        cmac_key: *const cmac128_key,
        cmac_cipher: *const libc::c_void,
        nc: *const nettle_cipher,
        ctr_cipher: *const libc::c_void,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
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
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siv_cmac_aes128_ctx {
    pub cmac_key: cmac128_key,
    pub cmac_cipher: aes128_ctx,
    pub ctr_cipher: aes128_ctx,
}
pub unsafe extern "C" fn nettle_siv_cmac_aes128_set_key(
    mut ctx: *mut siv_cmac_aes128_ctx,
    mut key: *const uint8_t,
) {
    nettle_siv_cmac_set_key(
        &mut (*ctx).cmac_key,
        &mut (*ctx).cmac_cipher as *mut aes128_ctx as *mut libc::c_void,
        &mut (*ctx).ctr_cipher as *mut aes128_ctx as *mut libc::c_void,
        &nettle_aes128,
        key,
    );
}
pub unsafe extern "C" fn nettle_siv_cmac_aes128_encrypt_message(
    mut ctx: *const siv_cmac_aes128_ctx,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_siv_cmac_encrypt_message(
        &(*ctx).cmac_key,
        &(*ctx).cmac_cipher as *const aes128_ctx as *const libc::c_void,
        &nettle_aes128,
        &(*ctx).ctr_cipher as *const aes128_ctx as *const libc::c_void,
        nlength,
        nonce,
        alength,
        adata,
        clength,
        dst,
        src,
    );
}
pub unsafe extern "C" fn nettle_siv_cmac_aes128_decrypt_message(
    mut ctx: *const siv_cmac_aes128_ctx,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> libc::c_int {
    return nettle_siv_cmac_decrypt_message(
        &(*ctx).cmac_key,
        &(*ctx).cmac_cipher as *const aes128_ctx as *const libc::c_void,
        &nettle_aes128,
        &(*ctx).ctr_cipher as *const aes128_ctx as *const libc::c_void,
        nlength,
        nonce,
        alength,
        adata,
        mlength,
        dst,
        src,
    );
}
