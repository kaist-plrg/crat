use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static _nettle_hashes: [*const nettle_hash; 0];
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
pub unsafe extern "C" fn nettle_lookup_hash(
    mut name: *const libc::c_char,
) -> *const nettle_hash {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while !(*_nettle_hashes.as_ptr().offset(i as isize)).is_null() {
        if strcmp(name, (**_nettle_hashes.as_ptr().offset(i as isize)).name) == 0 {
            return *_nettle_hashes.as_ptr().offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const nettle_hash;
}
