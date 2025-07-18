use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
    fn nettle_sha1_digest(ctx: *mut sha1_ctx, length: size_t, digest: *mut uint8_t);
    fn _nettle_pkcs1_signature_prefix(
        key_size: libc::c_uint,
        buffer: *mut uint8_t,
        id_size: libc::c_uint,
        id: *const uint8_t,
        digest_size: libc::c_uint,
    ) -> *mut uint8_t;
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
static mut sha1_prefix: [uint8_t; 15] = [
    0x30 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
];
pub unsafe extern "C" fn nettle_pkcs1_rsa_sha1_encode(
    mut m: *mut __mpz_struct,
    mut key_size: size_t,
    mut hash: *mut sha1_ctx,
) -> libc::c_int {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    tmp_em_size = key_size;
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(key_size),
    ) as *mut uint8_t;
    p = _nettle_pkcs1_signature_prefix(
        key_size as libc::c_uint,
        em,
        ::std::mem::size_of::<[uint8_t; 15]>() as libc::c_ulong as libc::c_uint,
        sha1_prefix.as_ptr(),
        20 as libc::c_int as libc::c_uint,
    );
    if !p.is_null() {
        nettle_sha1_digest(hash, 20 as libc::c_int as size_t, p);
        nettle_mpz_set_str_256_u(m, key_size, em);
        _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
        return 1 as libc::c_int;
    } else {
        _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn nettle_pkcs1_rsa_sha1_encode_digest(
    mut m: *mut __mpz_struct,
    mut key_size: size_t,
    mut digest: *const uint8_t,
) -> libc::c_int {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    tmp_em_size = key_size;
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul(key_size),
    ) as *mut uint8_t;
    p = _nettle_pkcs1_signature_prefix(
        key_size as libc::c_uint,
        em,
        ::std::mem::size_of::<[uint8_t; 15]>() as libc::c_ulong as libc::c_uint,
        sha1_prefix.as_ptr(),
        20 as libc::c_int as libc::c_uint,
    );
    if !p.is_null() {
        memcpy(
            p as *mut libc::c_void,
            digest as *const libc::c_void,
            20 as libc::c_int as libc::c_ulong,
        );
        nettle_mpz_set_str_256_u(m, key_size, em);
        _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
        return 1 as libc::c_int;
    } else {
        _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
        return 0 as libc::c_int;
    };
}
