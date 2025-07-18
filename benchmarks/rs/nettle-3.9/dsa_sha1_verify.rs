use ::libc;
extern "C" {
    fn nettle_dsa_verify(
        params: *const dsa_params,
        y: *const __mpz_struct,
        digest_size: size_t,
        digest: *const uint8_t,
        signature: *const dsa_signature,
    ) -> libc::c_int;
    fn nettle_sha1_digest(ctx: *mut sha1_ctx, length: size_t, digest: *mut uint8_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_params {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_signature {
    pub r: mpz_t,
    pub s: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_public_key {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
    pub y: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_sha1_verify_digest(
    mut key: *const dsa_public_key,
    mut digest: *const uint8_t,
    mut signature: *const dsa_signature,
) -> libc::c_int {
    return nettle_dsa_verify(
        key as *const dsa_params,
        ((*key).y).as_ptr(),
        20 as libc::c_int as size_t,
        digest,
        signature,
    );
}
pub unsafe extern "C" fn nettle_dsa_sha1_verify(
    mut key: *const dsa_public_key,
    mut hash: *mut sha1_ctx,
    mut signature: *const dsa_signature,
) -> libc::c_int {
    let mut digest: [uint8_t; 20] = [0; 20];
    nettle_sha1_digest(
        hash,
        ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    return nettle_dsa_verify(
        key as *const dsa_params,
        ((*key).y).as_ptr(),
        ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong,
        digest.as_mut_ptr(),
        signature,
    );
}
