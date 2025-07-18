use ::libc;
extern "C" {
    fn _nettle_salsa20_crypt(
        ctx: *mut salsa20_ctx,
        rounds: libc::c_uint,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct salsa20_ctx {
    pub input: [uint32_t; 16],
}
pub unsafe extern "C" fn nettle_salsa20r12_crypt(
    mut ctx: *mut salsa20_ctx,
    mut length: size_t,
    mut c: *mut uint8_t,
    mut m: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    _nettle_salsa20_crypt(ctx, 12 as libc::c_int as libc::c_uint, length, c, m);
}
