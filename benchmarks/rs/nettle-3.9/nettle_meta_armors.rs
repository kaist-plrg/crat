use ::libc;
extern "C" {
    static nettle_base16: nettle_armor;
    static nettle_base64url: nettle_armor;
    static nettle_base64: nettle_armor;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const libc::c_char,
) -> libc::c_int;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const libc::c_char,
    pub encode_context_size: libc::c_uint,
    pub decode_context_size: libc::c_uint,
    pub encode_final_length: libc::c_uint,
    pub encode_init: Option::<nettle_armor_init_func>,
    pub encode_length: Option::<nettle_armor_length_func>,
    pub encode_update: Option::<nettle_armor_encode_update_func>,
    pub encode_final: Option::<nettle_armor_encode_final_func>,
    pub decode_init: Option::<nettle_armor_init_func>,
    pub decode_length: Option::<nettle_armor_length_func>,
    pub decode_update: Option::<nettle_armor_decode_update_func>,
    pub decode_final: Option::<nettle_armor_decode_final_func>,
}
pub static mut _nettle_armors: [*const nettle_armor; 4] = unsafe {
    [
        &nettle_base64 as *const nettle_armor,
        &nettle_base64url as *const nettle_armor,
        &nettle_base16 as *const nettle_armor,
        0 as *const nettle_armor,
    ]
};
pub unsafe extern "C" fn nettle_get_armors() -> *const *const nettle_armor {
    return _nettle_armors.as_ptr();
}
