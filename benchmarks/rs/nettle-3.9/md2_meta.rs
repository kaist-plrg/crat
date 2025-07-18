use ::libc;
extern "C" {
    fn nettle_md2_init(ctx: *mut md2_ctx);
    fn nettle_md2_update(ctx: *mut md2_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md2_digest(ctx: *mut md2_ctx, length: size_t, digest: *mut uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub struct nettle_hash {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md2_ctx {
    pub C: [uint8_t; 16],
    pub X: [uint8_t; 48],
    pub index: libc::c_uint,
    pub block: [uint8_t; 16],
}
pub static mut nettle_md2: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"md2\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<md2_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            init: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut md2_ctx) -> ()>,
                Option::<nettle_hash_init_func>,
            >(Some(nettle_md2_init as unsafe extern "C" fn(*mut md2_ctx) -> ())),
            update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut md2_ctx, size_t, *const uint8_t) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_md2_update
                        as unsafe extern "C" fn(
                            *mut md2_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut md2_ctx, size_t, *mut uint8_t) -> ()>,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_md2_digest
                        as unsafe extern "C" fn(*mut md2_ctx, size_t, *mut uint8_t) -> (),
                ),
            ),
        };
        init
    }
};
