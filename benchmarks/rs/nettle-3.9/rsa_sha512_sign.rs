use ::libc;
extern "C" {
    fn __gmpz_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn nettle_rsa_compute_root(
        key: *const rsa_private_key,
        x: *mut __mpz_struct,
        m: *const __mpz_struct,
    );
    fn nettle_pkcs1_rsa_sha512_encode(
        m: *mut __mpz_struct,
        length: size_t,
        hash: *mut sha512_ctx,
    ) -> libc::c_int;
    fn nettle_pkcs1_rsa_sha512_encode_digest(
        m: *mut __mpz_struct,
        length: size_t,
        digest: *const uint8_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
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
pub struct sha512_ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 128],
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
pub unsafe extern "C" fn nettle_rsa_sha512_sign(
    mut key: *const rsa_private_key,
    mut hash: *mut sha512_ctx,
    mut s: *mut __mpz_struct,
) -> libc::c_int {
    if nettle_pkcs1_rsa_sha512_encode(s, (*key).size, hash) != 0 {
        nettle_rsa_compute_root(key, s, s as *const __mpz_struct);
        return 1 as libc::c_int;
    } else {
        __gmpz_set_ui(s, 0 as libc::c_int as libc::c_ulong);
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn nettle_rsa_sha512_sign_digest(
    mut key: *const rsa_private_key,
    mut digest: *const uint8_t,
    mut s: *mut __mpz_struct,
) -> libc::c_int {
    if nettle_pkcs1_rsa_sha512_encode_digest(s, (*key).size, digest) != 0 {
        nettle_rsa_compute_root(key, s, s as *const __mpz_struct);
        return 1 as libc::c_int;
    } else {
        __gmpz_set_ui(s, 0 as libc::c_int as libc::c_ulong);
        return 0 as libc::c_int;
    };
}
