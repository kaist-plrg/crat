use ::libc;
extern "C" {
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
    fn _nettle_pkcs1_signature_prefix(
        key_size: libc::c_uint,
        buffer: *mut uint8_t,
        id_size: libc::c_uint,
        id: *const uint8_t,
        digest_size: libc::c_uint,
    ) -> *mut uint8_t;
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
pub unsafe extern "C" fn nettle_pkcs1_rsa_digest_encode(
    mut m: *mut __mpz_struct,
    mut key_size: size_t,
    mut di_length: size_t,
    mut digest_info: *const uint8_t,
) -> libc::c_int {
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    tmp_em_size = key_size;
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(key_size),
    ) as *mut uint8_t;
    if !(_nettle_pkcs1_signature_prefix(
        key_size as libc::c_uint,
        em,
        di_length as libc::c_uint,
        digest_info,
        0 as libc::c_int as libc::c_uint,
    ))
        .is_null()
    {
        nettle_mpz_set_str_256_u(m, key_size, em);
        _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
        return 1 as libc::c_int;
    } else {
        _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
        return 0 as libc::c_int;
    };
}
