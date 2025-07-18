use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn _nettle_rsa_verify(
        key: *const rsa_public_key,
        m: *const __mpz_struct,
        s: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_pkcs1_rsa_digest_encode(
        m: *mut __mpz_struct,
        key_size: size_t,
        di_length: size_t,
        digest_info: *const uint8_t,
    ) -> libc::c_int;
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
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
pub unsafe extern "C" fn nettle_rsa_pkcs1_verify(
    mut key: *const rsa_public_key,
    mut length: size_t,
    mut digest_info: *const uint8_t,
    mut s: *const __mpz_struct,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(m.as_mut_ptr());
    res = (nettle_pkcs1_rsa_digest_encode(
        m.as_mut_ptr(),
        (*key).size,
        length,
        digest_info,
    ) != 0 && _nettle_rsa_verify(key, m.as_mut_ptr() as *const __mpz_struct, s) != 0)
        as libc::c_int;
    __gmpz_clear(m.as_mut_ptr());
    return res;
}
