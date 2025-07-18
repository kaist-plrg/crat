use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn nettle_rsa_compute_root_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        x: *mut __mpz_struct,
        m: *const __mpz_struct,
    ) -> libc::c_int;
    static nettle_sha256: nettle_hash;
    fn nettle_pss_encode_mgf1(
        m: *mut __mpz_struct,
        bits: size_t,
        hash: *const nettle_hash,
        salt_length: size_t,
        salt: *const uint8_t,
        digest: *const uint8_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_random_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
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
pub struct rsa_private_key {
    pub size: size_t,
    pub d: mpz_t,
    pub p: mpz_t,
    pub q: mpz_t,
    pub a: mpz_t,
    pub b: mpz_t,
    pub c: mpz_t,
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
pub unsafe extern "C" fn nettle_rsa_pss_sha256_sign_digest_tr(
    mut pub_0: *const rsa_public_key,
    mut key: *const rsa_private_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut digest: *const uint8_t,
    mut s: *mut __mpz_struct,
) -> libc::c_int {
    let mut m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    __gmpz_init(m.as_mut_ptr());
    res = (nettle_pss_encode_mgf1(
        m.as_mut_ptr(),
        (__gmpz_sizeinbase(((*pub_0).n).as_ptr(), 2 as libc::c_int))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        &nettle_sha256,
        salt_length,
        salt,
        digest,
    ) != 0
        && nettle_rsa_compute_root_tr(
            pub_0,
            key,
            random_ctx,
            random,
            s,
            m.as_mut_ptr() as *const __mpz_struct,
        ) != 0) as libc::c_int;
    __gmpz_clear(m.as_mut_ptr());
    return res;
}
