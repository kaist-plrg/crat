use ::libc;
extern "C" {
    fn nettle_camellia128_set_encrypt_key(
        ctx: *mut camellia128_ctx,
        key: *const uint8_t,
    );
    fn _nettle_camellia_invert_key(
        nkeys: libc::c_uint,
        dst: *mut uint64_t,
        src: *const uint64_t,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
}
pub unsafe extern "C" fn nettle_camellia128_invert_key(
    mut dst: *mut camellia128_ctx,
    mut src: *const camellia128_ctx,
) {
    _nettle_camellia_invert_key(
        24 as libc::c_int as libc::c_uint,
        ((*dst).keys).as_mut_ptr(),
        ((*src).keys).as_ptr(),
    );
}
pub unsafe extern "C" fn nettle_camellia_set_decrypt_key(
    mut ctx: *mut camellia128_ctx,
    mut key: *const uint8_t,
) {
    nettle_camellia128_set_encrypt_key(ctx, key);
    nettle_camellia128_invert_key(ctx, ctx);
}
