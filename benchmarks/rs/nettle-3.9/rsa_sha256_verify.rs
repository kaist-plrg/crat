use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn _nettle_rsa_verify(
        key: *const rsa_public_key,
        m: *const __mpz_struct,
        s: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_pkcs1_rsa_sha256_encode(
        m: *mut __mpz_struct,
        length: size_t,
        hash: *mut sha256_ctx,
    ) -> libc::c_int;
    fn nettle_pkcs1_rsa_sha256_encode_digest(
        m: *mut __mpz_struct,
        length: size_t,
        digest: *const uint8_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
pub unsafe extern "C" fn nettle_rsa_sha256_verify(
    mut key: *const rsa_public_key,
    mut hash: *mut sha256_ctx,
    mut s: *const __mpz_struct,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(m.as_mut_ptr());
    res = (nettle_pkcs1_rsa_sha256_encode(m.as_mut_ptr(), (*key).size, hash) != 0
        && _nettle_rsa_verify(key, m.as_mut_ptr() as *const __mpz_struct, s) != 0)
        as libc::c_int;
    __gmpz_clear(m.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn nettle_rsa_sha256_verify_digest(
    mut key: *const rsa_public_key,
    mut digest: *const uint8_t,
    mut s: *const __mpz_struct,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(m.as_mut_ptr());
    res = (nettle_pkcs1_rsa_sha256_encode_digest(m.as_mut_ptr(), (*key).size, digest)
        != 0 && _nettle_rsa_verify(key, m.as_mut_ptr() as *const __mpz_struct, s) != 0)
        as libc::c_int;
    __gmpz_clear(m.as_mut_ptr());
    return res;
}
