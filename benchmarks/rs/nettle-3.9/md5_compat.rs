use ::libc;
extern "C" {
    fn nettle_md5_init(ctx: *mut md5_ctx);
    fn nettle_md5_update(ctx: *mut md5_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md5_digest(ctx: *mut md5_ctx, length: size_t, digest: *mut uint8_t);
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
pub struct md5_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
pub type MD5_CTX = md5_ctx;
pub unsafe extern "C" fn nettle_MD5Init(mut ctx: *mut MD5_CTX) {
    nettle_md5_init(ctx);
}
pub unsafe extern "C" fn nettle_MD5Update(
    mut ctx: *mut MD5_CTX,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) {
    nettle_md5_update(ctx, length as size_t, data);
}
pub unsafe extern "C" fn nettle_MD5Final(
    mut out: *mut libc::c_uchar,
    mut ctx: *mut MD5_CTX,
) {
    nettle_md5_digest(ctx, 16 as libc::c_int as size_t, out);
}
