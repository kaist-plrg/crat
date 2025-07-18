use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nettle_sha3_update(
        state: *mut sha3_state,
        block_size: libc::c_uint,
        block: *mut uint8_t,
        pos: libc::c_uint,
        length: size_t,
        data: *const uint8_t,
    ) -> libc::c_uint;
    fn _nettle_sha3_pad(
        state: *mut sha3_state,
        block_size: libc::c_uint,
        block: *mut uint8_t,
        pos: libc::c_uint,
        magic: uint8_t,
    );
    fn _nettle_write_le64(length: size_t, dst: *mut uint8_t, src: *const uint64_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_256_ctx {
    pub state: sha3_state,
    pub index: libc::c_uint,
    pub block: [uint8_t; 136],
}
pub unsafe extern "C" fn nettle_sha3_256_init(mut ctx: *mut sha3_256_ctx) {
    memset(ctx as *mut libc::c_void, 0 as libc::c_int, 204 as libc::c_ulong);
}
pub unsafe extern "C" fn nettle_sha3_256_update(
    mut ctx: *mut sha3_256_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    (*ctx)
        .index = _nettle_sha3_update(
        &mut (*ctx).state,
        136 as libc::c_int as libc::c_uint,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        data,
    );
}
pub unsafe extern "C" fn nettle_sha3_256_digest(
    mut ctx: *mut sha3_256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    _nettle_sha3_pad(
        &mut (*ctx).state,
        136 as libc::c_int as libc::c_uint,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        6 as libc::c_int as uint8_t,
    );
    _nettle_write_le64(length, digest, ((*ctx).state.a).as_mut_ptr());
    nettle_sha3_256_init(ctx);
}
