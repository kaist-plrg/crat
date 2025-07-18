use ::libc;
extern "C" {
    fn nettle_mpz_get_str_256(length: size_t, s: *mut uint8_t, x: *const __mpz_struct);
    fn _nettle_pkcs1_sec_decrypt_variable(
        length: *mut size_t,
        message: *mut uint8_t,
        padded_message_length: size_t,
        padded_message: *const uint8_t,
    ) -> libc::c_int;
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub unsafe extern "C" fn nettle_pkcs1_decrypt(
    mut key_size: size_t,
    mut m: *const __mpz_struct,
    mut length: *mut size_t,
    mut message: *mut uint8_t,
) -> libc::c_int {
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    let mut ret: libc::c_int = 0;
    tmp_em_size = key_size;
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(key_size),
    ) as *mut uint8_t;
    nettle_mpz_get_str_256(key_size, em, m);
    ret = _nettle_pkcs1_sec_decrypt_variable(length, message, key_size, em);
    _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
    return ret;
}
