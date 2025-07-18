use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn _nettle_rsa_verify_recover(
        key: *const rsa_public_key,
        m: *mut __mpz_struct,
        s: *const __mpz_struct,
    ) -> libc::c_int;
    static nettle_sha256: nettle_hash;
    fn nettle_pss_verify_mgf1(
        m: *const __mpz_struct,
        bits: size_t,
        hash: *const nettle_hash,
        salt_length: size_t,
        digest: *const uint8_t,
    ) -> libc::c_int;
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
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
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
pub unsafe extern "C" fn nettle_rsa_pss_sha256_verify_digest(
    mut key: *const rsa_public_key,
    mut salt_length: size_t,
    mut digest: *const uint8_t,
    mut signature: *const __mpz_struct,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(m.as_mut_ptr());
    res = (_nettle_rsa_verify_recover(key, m.as_mut_ptr(), signature) != 0
        && nettle_pss_verify_mgf1(
            m.as_mut_ptr() as *const __mpz_struct,
            (__gmpz_sizeinbase(((*key).n).as_ptr(), 2 as libc::c_int))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            &nettle_sha256,
            salt_length,
            digest,
        ) != 0) as libc::c_int;
    __gmpz_clear(m.as_mut_ptr());
    return res;
}
