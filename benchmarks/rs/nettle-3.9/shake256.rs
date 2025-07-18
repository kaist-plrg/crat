use ::libc;
extern "C" {
    fn nettle_sha3_256_init(ctx: *mut sha3_256_ctx);
    fn nettle_sha3_permute(state: *mut sha3_state);
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
pub unsafe extern "C" fn nettle_sha3_256_shake(
    mut ctx: *mut sha3_256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    _nettle_sha3_pad(
        &mut (*ctx).state,
        136 as libc::c_int as libc::c_uint,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        0x1f as libc::c_int as uint8_t,
    );
    while length > 136 as libc::c_int as libc::c_ulong {
        _nettle_write_le64(
            136 as libc::c_int as size_t,
            dst,
            ((*ctx).state.a).as_mut_ptr(),
        );
        length = (length as libc::c_ulong)
            .wrapping_sub(136 as libc::c_int as libc::c_ulong) as size_t as size_t;
        dst = dst.offset(136 as libc::c_int as isize);
        nettle_sha3_permute(&mut (*ctx).state);
    }
    _nettle_write_le64(length, dst, ((*ctx).state.a).as_mut_ptr());
    nettle_sha3_256_init(ctx);
}
