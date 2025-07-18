use ::libc;
extern "C" {
    fn test_hash_large(
        hash: *const nettle_hash,
        count: size_t,
        length: size_t,
        c: uint8_t,
        digest: *const tstring,
    );
    fn tstring_hex(hex: *const libc::c_char) -> *mut tstring;
    static nettle_sha1: nettle_hash;
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
pub struct tstring {
    pub next: *mut tstring,
    pub length: size_t,
    pub data: [uint8_t; 1],
}
pub unsafe extern "C" fn test_main() {
    test_hash_large(
        &nettle_sha1,
        10000000 as libc::c_int as size_t,
        30000 as libc::c_int as size_t,
        'a' as i32 as uint8_t,
        tstring_hex(
            b"0ba79364dc64648f 2074fb4bc5c28bcfb7a787b0\0" as *const u8
                as *const libc::c_char,
        ),
    );
}
