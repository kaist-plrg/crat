use ::libc;
extern "C" {
    fn _nettle_poly1305_block(
        ctx: *mut poly1305_ctx,
        m: *const uint8_t,
        high: libc::c_uint,
    );
    fn _nettle_poly1305_blocks(
        ctx: *mut poly1305_ctx,
        blocks: size_t,
        m: *const uint8_t,
    ) -> *const uint8_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub struct poly1305_ctx {
    pub r: C2RustUnnamed_0,
    pub s32: [uint32_t; 3],
    pub hh: uint32_t,
    pub h: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub h32: [uint32_t; 4],
    pub h64: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r32: [uint32_t; 6],
    pub r64: [uint64_t; 3],
}
pub unsafe extern "C" fn _nettle_poly1305_update(
    mut ctx: *mut poly1305_ctx,
    mut block: *mut uint8_t,
    mut index: libc::c_uint,
    mut length: size_t,
    mut m: *const uint8_t,
) -> libc::c_uint {
    if index > 0 as libc::c_int as libc::c_uint {
        let mut __md_left: libc::c_uint = (16 as libc::c_int as libc::c_uint)
            .wrapping_sub(index);
        if length < __md_left as libc::c_ulong {
            memcpy(
                block.offset(index as isize) as *mut libc::c_void,
                m as *const libc::c_void,
                length,
            );
            return (index as libc::c_ulong).wrapping_add(length) as libc::c_uint;
        }
        memcpy(
            block.offset(index as isize) as *mut libc::c_void,
            m as *const libc::c_void,
            __md_left as libc::c_ulong,
        );
        m = m.offset(__md_left as isize);
        length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
            as size_t as size_t;
        _nettle_poly1305_block(ctx, block, 1 as libc::c_int as libc::c_uint);
    }
    m = _nettle_poly1305_blocks(ctx, length >> 4 as libc::c_int, m);
    length &= 15 as libc::c_int as libc::c_ulong;
    memcpy(block as *mut libc::c_void, m as *const libc::c_void, length);
    return length as libc::c_uint;
}
