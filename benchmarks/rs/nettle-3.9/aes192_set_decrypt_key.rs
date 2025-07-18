use ::libc;
extern "C" {
    fn nettle_aes192_set_encrypt_key(ctx: *mut aes192_ctx, key: *const uint8_t);
    fn _nettle_aes_invert(
        rounds: libc::c_uint,
        dst: *mut uint32_t,
        src: *const uint32_t,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
pub unsafe extern "C" fn nettle_aes192_invert_key(
    mut dst: *mut aes192_ctx,
    mut src: *const aes192_ctx,
) {
    _nettle_aes_invert(
        12 as libc::c_int as libc::c_uint,
        ((*dst).keys).as_mut_ptr(),
        ((*src).keys).as_ptr(),
    );
}
pub unsafe extern "C" fn nettle_aes192_set_decrypt_key(
    mut ctx: *mut aes192_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes192_set_encrypt_key(ctx, key);
    nettle_aes192_invert_key(ctx, ctx);
}
