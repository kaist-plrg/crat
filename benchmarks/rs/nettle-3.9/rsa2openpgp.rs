use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn nettle_sha1_init(ctx: *mut sha1_ctx);
    fn nettle_sha1_update(ctx: *mut sha1_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha1_digest(ctx: *mut sha1_ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_pgp_put_public_rsa_key(
        _: *mut nettle_buffer,
        key: *const rsa_public_key,
        timestamp: time_t,
    ) -> libc::c_int;
    fn nettle_pgp_put_rsa_sha1_signature(
        buffer: *mut nettle_buffer,
        key: *const rsa_private_key,
        keyid: *const uint8_t,
        type_0: libc::c_uint,
        hash: *mut sha1_ctx,
    ) -> libc::c_int;
    fn nettle_pgp_put_userid(
        buffer: *mut nettle_buffer,
        length: libc::c_uint,
        name: *const uint8_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
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
pub struct sha1_ctx {
    pub state: [uint32_t; 5],
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
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
}
pub const PGP_SIGN_CERTIFICATION: pgp_signature_type = 16;
pub type pgp_signature_type = libc::c_uint;
pub const PGP_SIGN_TIMESTAMP: pgp_signature_type = 64;
pub const PGP_SIGN_REVOCATION_CERTIFICATE: pgp_signature_type = 48;
pub const PGP_SIGN_REVOCATION_SUBKEY: pgp_signature_type = 40;
pub const PGP_SIGN_REVOCATION: pgp_signature_type = 32;
pub const PGP_SIGN_KEY: pgp_signature_type = 31;
pub const PGP_SIGN_SUBKEY: pgp_signature_type = 24;
pub const PGP_SIGN_CERTIFICATION_POSITIVE: pgp_signature_type = 19;
pub const PGP_SIGN_CERTIFICATION_CASUAL: pgp_signature_type = 18;
pub const PGP_SIGN_CERTIFICATION_PERSONA: pgp_signature_type = 17;
pub const PGP_SIGN_STANDALONE: pgp_signature_type = 2;
pub const PGP_SIGN_TEXT: pgp_signature_type = 1;
pub const PGP_SIGN_BINARY: pgp_signature_type = 0;
pub unsafe extern "C" fn nettle_rsa_keypair_to_openpgp(
    mut buffer: *mut nettle_buffer,
    mut pub_0: *const rsa_public_key,
    mut priv_0: *const rsa_private_key,
    mut userid: *const libc::c_char,
) -> libc::c_int {
    let mut now: time_t = time(0 as *mut time_t);
    let mut key_start: libc::c_uint = 0;
    let mut userid_start: libc::c_uint = 0;
    let mut key_hash: sha1_ctx = sha1_ctx {
        state: [0; 5],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut signature_hash: sha1_ctx = sha1_ctx {
        state: [0; 5],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut fingerprint: [uint8_t; 20] = [0; 20];
    key_start = (*buffer).size as libc::c_uint;
    if nettle_pgp_put_public_rsa_key(buffer, pub_0, now) == 0 {
        return 0 as libc::c_int;
    }
    userid_start = (*buffer).size as libc::c_uint;
    if nettle_pgp_put_userid(
        buffer,
        strlen(userid) as libc::c_uint,
        userid as *const uint8_t,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    nettle_sha1_init(&mut key_hash);
    nettle_sha1_update(
        &mut key_hash,
        userid_start.wrapping_sub(key_start) as size_t,
        ((*buffer).contents).offset(key_start as isize),
    );
    signature_hash = key_hash;
    nettle_sha1_digest(
        &mut key_hash,
        ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong,
        fingerprint.as_mut_ptr(),
    );
    nettle_sha1_update(
        &mut signature_hash,
        ((*buffer).size).wrapping_sub(userid_start as libc::c_ulong),
        ((*buffer).contents).offset(userid_start as isize),
    );
    return nettle_pgp_put_rsa_sha1_signature(
        buffer,
        priv_0,
        fingerprint
            .as_mut_ptr()
            .offset(20 as libc::c_int as isize)
            .offset(-(8 as libc::c_int as isize)),
        PGP_SIGN_CERTIFICATION as libc::c_int as libc::c_uint,
        &mut signature_hash,
    );
}
