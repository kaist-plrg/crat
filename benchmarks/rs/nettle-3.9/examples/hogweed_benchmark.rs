use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_st;
    pub type bn_gencb_st;
    pub type rsa_st;
    pub type ec_key_st;
    pub type ECDSA_SIG_st;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time_init();
    static mut time_start: Option::<unsafe extern "C" fn() -> ()>;
    static mut time_end: Option::<unsafe extern "C" fn() -> libc::c_double>;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_init_set_str(
        _: mpz_ptr,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn nettle_dsa_verify(
        params: *const dsa_params,
        y: *const __mpz_struct,
        digest_size: size_t,
        digest: *const uint8_t,
        signature: *const dsa_signature,
    ) -> libc::c_int;
    fn nettle_dsa_sign(
        params: *const dsa_params,
        x: *const __mpz_struct,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        digest_size: size_t,
        digest: *const uint8_t,
        signature: *mut dsa_signature,
    ) -> libc::c_int;
    fn nettle_dsa_signature_clear(signature: *mut dsa_signature);
    fn nettle_dsa_signature_init(signature: *mut dsa_signature);
    fn nettle_dsa_params_clear(params: *mut dsa_params);
    fn nettle_dsa_params_init(params: *mut dsa_params);
    fn nettle_dsa_keypair_from_sexp_alist(
        params: *mut dsa_params,
        pub_0: *mut __mpz_struct,
        priv_0: *mut __mpz_struct,
        p_max_bits: libc::c_uint,
        q_bits: libc::c_uint,
        i: *mut sexp_iterator,
    ) -> libc::c_int;
    fn nettle_rsa_public_key_init(key: *mut rsa_public_key);
    fn nettle_rsa_public_key_clear(key: *mut rsa_public_key);
    fn nettle_rsa_private_key_init(key: *mut rsa_private_key);
    fn nettle_rsa_private_key_clear(key: *mut rsa_private_key);
    fn nettle_rsa_sha256_sign_digest(
        key: *const rsa_private_key,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha256_sign_digest_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha256_verify_digest(
        key: *const rsa_public_key,
        digest: *const uint8_t,
        signature: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_keypair_from_sexp_alist(
        pub_0: *mut rsa_public_key,
        priv_0: *mut rsa_private_key,
        limit: libc::c_uint,
        i: *mut sexp_iterator,
    ) -> libc::c_int;
    fn nettle_ed25519_sha512_public_key(pub_0: *mut uint8_t, priv_0: *const uint8_t);
    fn nettle_ed25519_sha512_sign(
        pub_0: *const uint8_t,
        priv_0: *const uint8_t,
        length: size_t,
        msg: *const uint8_t,
        signature: *mut uint8_t,
    );
    fn nettle_ed25519_sha512_verify(
        pub_0: *const uint8_t,
        length: size_t,
        msg: *const uint8_t,
        signature: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_ed448_shake256_public_key(pub_0: *mut uint8_t, priv_0: *const uint8_t);
    fn nettle_ed448_shake256_sign(
        pub_0: *const uint8_t,
        priv_0: *const uint8_t,
        length: size_t,
        msg: *const uint8_t,
        signature: *mut uint8_t,
    );
    fn nettle_ed448_shake256_verify(
        pub_0: *const uint8_t,
        length: size_t,
        msg: *const uint8_t,
        signature: *const uint8_t,
    ) -> libc::c_int;
    fn nettle_ecc_point_init(p: *mut ecc_point, ecc: *const ecc_curve);
    fn nettle_ecc_point_clear(p: *mut ecc_point);
    fn nettle_ecc_point_set(
        p: *mut ecc_point,
        x: *const __mpz_struct,
        y: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_ecc_scalar_init(s: *mut ecc_scalar, ecc: *const ecc_curve);
    fn nettle_ecc_scalar_clear(s: *mut ecc_scalar);
    fn nettle_ecc_scalar_set(s: *mut ecc_scalar, z: *const __mpz_struct) -> libc::c_int;
    fn nettle_ecdsa_sign(
        key: *const ecc_scalar,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        digest_length: size_t,
        digest: *const uint8_t,
        signature: *mut dsa_signature,
    );
    fn nettle_ecdsa_verify(
        pub_0: *const ecc_point,
        length: size_t,
        digest: *const uint8_t,
        signature: *const dsa_signature,
    ) -> libc::c_int;
    fn nettle_gostdsa_verify(
        pub_0: *const ecc_point,
        length: size_t,
        digest: *const uint8_t,
        signature: *const dsa_signature,
    ) -> libc::c_int;
    fn nettle_gostdsa_sign(
        key: *const ecc_scalar,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        digest_length: size_t,
        digest: *const uint8_t,
        signature: *mut dsa_signature,
    );
    fn nettle_curve25519_mul_g(q: *mut uint8_t, n: *const uint8_t);
    fn nettle_curve25519_mul(q: *mut uint8_t, n: *const uint8_t, p: *const uint8_t);
    fn nettle_curve448_mul_g(q: *mut uint8_t, n: *const uint8_t);
    fn nettle_curve448_mul(q: *mut uint8_t, n: *const uint8_t, p: *const uint8_t);
    static nettle_sha1: nettle_hash;
    static nettle_sha224: nettle_hash;
    static nettle_sha256: nettle_hash;
    static nettle_sha384: nettle_hash;
    static nettle_sha512: nettle_hash;
    fn nettle_sexp_transport_iterator_first(
        iterator: *mut sexp_iterator,
        length: size_t,
        input: *mut uint8_t,
    ) -> libc::c_int;
    fn nettle_sexp_iterator_check_type(
        iterator: *mut sexp_iterator,
        type_0: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_knuth_lfib_init(ctx: *mut knuth_lfib_ctx, seed: uint32_t);
    fn nettle_knuth_lfib_random(ctx: *mut knuth_lfib_ctx, n: size_t, dst: *mut uint8_t);
    static _nettle_secp_192r1: ecc_curve;
    static _nettle_secp_224r1: ecc_curve;
    static _nettle_secp_256r1: ecc_curve;
    static _nettle_secp_384r1: ecc_curve;
    static _nettle_secp_521r1: ecc_curve;
    static _nettle_gost_gc256b: ecc_curve;
    static _nettle_gost_gc512a: ecc_curve;
    fn BN_new() -> *mut BIGNUM;
    fn BN_set_word(a: *mut BIGNUM, w: libc::c_ulong) -> libc::c_int;
    fn BN_free(a: *mut BIGNUM);
    fn RSA_blinding_off(rsa: *mut RSA);
    fn RSA_verify(
        type_0: libc::c_int,
        m: *const libc::c_uchar,
        m_length: libc::c_uint,
        sigbuf: *const libc::c_uchar,
        siglen: libc::c_uint,
        rsa: *mut RSA,
    ) -> libc::c_int;
    fn RSA_sign(
        type_0: libc::c_int,
        m: *const libc::c_uchar,
        m_length: libc::c_uint,
        sigret: *mut libc::c_uchar,
        siglen: *mut libc::c_uint,
        rsa: *mut RSA,
    ) -> libc::c_int;
    fn RSA_free(r: *mut RSA);
    fn RSA_generate_key_ex(
        rsa: *mut RSA,
        bits: libc::c_int,
        e: *mut BIGNUM,
        cb: *mut BN_GENCB,
    ) -> libc::c_int;
    fn RSA_size(rsa: *const RSA) -> libc::c_int;
    fn RSA_new() -> *mut RSA;
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn EC_KEY_generate_key(key: *mut EC_KEY) -> libc::c_int;
    fn ECDSA_SIG_free(sig: *mut ECDSA_SIG);
    fn ECDSA_do_sign(
        dgst: *const libc::c_uchar,
        dgst_len: libc::c_int,
        eckey: *mut EC_KEY,
    ) -> *mut ECDSA_SIG;
    fn ECDSA_do_verify(
        dgst: *const libc::c_uchar,
        dgst_len: libc::c_int,
        sig: *const ECDSA_SIG,
        eckey: *mut EC_KEY,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_iterator {
    pub length: size_t,
    pub buffer: *const uint8_t,
    pub start: size_t,
    pub pos: size_t,
    pub level: libc::c_uint,
    pub type_0: sexp_type,
    pub display_length: size_t,
    pub display: *const uint8_t,
    pub atom_length: size_t,
    pub atom: *const uint8_t,
}
pub type sexp_type = libc::c_uint;
pub const SEXP_END: sexp_type = 2;
pub const SEXP_LIST: sexp_type = 1;
pub const SEXP_ATOM: sexp_type = 0;
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
pub struct ecc_curve {
    pub p: ecc_modulo,
    pub q: ecc_modulo,
    pub use_redc: libc::c_ushort,
    pub pippenger_k: libc::c_ushort,
    pub pippenger_c: libc::c_ushort,
    pub add_hh_itch: libc::c_ushort,
    pub add_hhh_itch: libc::c_ushort,
    pub dup_itch: libc::c_ushort,
    pub mul_itch: libc::c_ushort,
    pub mul_g_itch: libc::c_ushort,
    pub h_to_a_itch: libc::c_ushort,
    pub add_hh: Option::<ecc_add_func>,
    pub add_hhh: Option::<ecc_add_func>,
    pub dup: Option::<ecc_dup_func>,
    pub mul: Option::<ecc_mul_func>,
    pub mul_g: Option::<ecc_mul_g_func>,
    pub h_to_a: Option::<ecc_h_to_a_func>,
    pub b: *const mp_limb_t,
    pub unit: *const mp_limb_t,
    pub pippenger_table: *const mp_limb_t,
}
pub type ecc_h_to_a_func = unsafe extern "C" fn(
    *const ecc_curve,
    libc::c_int,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mul_g_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mul_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_dup_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_add_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_modulo {
    pub bit_size: libc::c_ushort,
    pub size: libc::c_ushort,
    pub B_size: libc::c_ushort,
    pub redc_size: libc::c_ushort,
    pub invert_itch: libc::c_ushort,
    pub sqrt_itch: libc::c_ushort,
    pub sqrt_ratio_itch: libc::c_ushort,
    pub m: *const mp_limb_t,
    pub B: *const mp_limb_t,
    pub B_shifted: *const mp_limb_t,
    pub Bm2m: *const mp_limb_t,
    pub redc_mpm1: *const mp_limb_t,
    pub mp1h: *const mp_limb_t,
    pub mod_0: Option::<ecc_mod_func>,
    pub reduce: Option::<ecc_mod_func>,
    pub invert: Option::<ecc_mod_inv_func>,
    pub sqrt: Option::<ecc_mod_sqrt_func>,
    pub sqrt_ratio: Option::<ecc_mod_sqrt_ratio_func>,
}
pub type ecc_mod_sqrt_ratio_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> libc::c_int;
pub type ecc_mod_sqrt_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> libc::c_int;
pub type ecc_mod_inv_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mod_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *mut mp_limb_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_point {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_scalar {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct knuth_lfib_ctx {
    pub x: [uint32_t; 100],
    pub index: libc::c_uint,
}
pub type BIGNUM = bignum_st;
pub type BN_GENCB = bn_gencb_st;
pub type RSA = rsa_st;
pub type EC_KEY = ec_key_st;
pub type ECDSA_SIG = ECDSA_SIG_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alg {
    pub name: *const libc::c_char,
    pub size: libc::c_uint,
    pub init: Option::<unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void>,
    pub sign: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub verify: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub clear: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_ctx {
    pub pub_0: rsa_public_key,
    pub key: rsa_private_key,
    pub lfib: knuth_lfib_ctx,
    pub digest: *mut uint8_t,
    pub s: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_ctx {
    pub params: dsa_params,
    pub pub_0: mpz_t,
    pub key: mpz_t,
    pub lfib: knuth_lfib_ctx,
    pub s: dsa_signature,
    pub digest: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecdsa_ctx {
    pub pub_0: ecc_point,
    pub key: ecc_scalar,
    pub lfib: knuth_lfib_ctx,
    pub digest_size: libc::c_uint,
    pub digest: *mut uint8_t,
    pub s: dsa_signature,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eddsa_ctx {
    pub pub_0: [uint8_t; 57],
    pub key: [uint8_t; 57],
    pub signature: [uint8_t; 114],
    pub sign: Option::<
        unsafe extern "C" fn(
            *const uint8_t,
            *const uint8_t,
            size_t,
            *const uint8_t,
            *mut uint8_t,
        ) -> (),
    >,
    pub verify: Option::<
        unsafe extern "C" fn(
            *const uint8_t,
            size_t,
            *const uint8_t,
            *const uint8_t,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct openssl_rsa_ctx {
    pub key: *mut RSA,
    pub ref_0: *mut libc::c_uchar,
    pub signature: *mut libc::c_uchar,
    pub siglen: libc::c_uint,
    pub digest: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct openssl_ecdsa_ctx {
    pub key: *mut EC_KEY,
    pub signature: *mut ECDSA_SIG,
    pub digest_length: libc::c_uint,
    pub digest: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curve_ctx {
    pub x: [uint8_t; 56],
    pub s: [uint8_t; 56],
    pub mul_g: Option::<unsafe extern "C" fn(*mut uint8_t, *const uint8_t) -> ()>,
    pub mul: Option::<
        unsafe extern "C" fn(*mut uint8_t, *const uint8_t, *const uint8_t) -> (),
    >,
}
unsafe extern "C" fn die(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    exit(1 as libc::c_int);
}
unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if p.is_null() {
        fprintf(
            stderr,
            b"Virtual memory exhausted\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return p;
}
unsafe extern "C" fn hash_string(
    mut hash: *const nettle_hash,
    mut s: *const libc::c_char,
) -> *mut uint8_t {
    let mut ctx: *mut libc::c_void = xalloc((*hash).context_size as size_t);
    let mut digest: *mut uint8_t = xalloc((*hash).digest_size as size_t) as *mut uint8_t;
    ((*hash).init).unwrap()(ctx);
    ((*hash).update).unwrap()(ctx, strlen(s), s as *const uint8_t);
    ((*hash).digest).unwrap()(ctx, (*hash).digest_size as size_t, digest);
    free(ctx);
    return digest;
}
unsafe extern "C" fn time_function(
    mut f: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> libc::c_double {
    let mut ncalls: libc::c_uint = 0;
    let mut elapsed: libc::c_double = 0.;
    f.unwrap()(arg);
    ncalls = 10 as libc::c_int as libc::c_uint;
    loop {
        let mut i: libc::c_uint = 0;
        time_start.unwrap()();
        i = 0 as libc::c_int as libc::c_uint;
        while i < ncalls {
            f.unwrap()(arg);
            i = i.wrapping_add(1);
            i;
        }
        elapsed = time_end.unwrap()();
        if elapsed > 0.1f64 {
            break;
        }
        if elapsed < 0.1f64 / 10 as libc::c_int as libc::c_double {
            ncalls = ncalls.wrapping_mul(10 as libc::c_int as libc::c_uint);
        } else {
            ncalls = ncalls.wrapping_mul(2 as libc::c_int as libc::c_uint);
        }
    }
    return elapsed / ncalls as libc::c_double;
}
unsafe extern "C" fn bench_alg(mut alg: *const alg) {
    let mut sign: libc::c_double = 0.;
    let mut verify: libc::c_double = 0.;
    let mut ctx: *mut libc::c_void = 0 as *mut libc::c_void;
    ctx = ((*alg).init).unwrap()((*alg).size);
    if ctx.is_null() {
        printf(
            b"%16s %4d N/A\n\0" as *const u8 as *const libc::c_char,
            (*alg).name,
            (*alg).size,
        );
        return;
    }
    sign = time_function((*alg).sign, ctx);
    verify = time_function((*alg).verify, ctx);
    ((*alg).clear).unwrap()(ctx);
    printf(
        b"%16s %4d %9.4f %9.4f\n\0" as *const u8 as *const libc::c_char,
        (*alg).name,
        (*alg).size,
        1e-3f64 / sign,
        1e-3f64 / verify,
    );
}
unsafe extern "C" fn bench_rsa_init(mut size: libc::c_uint) -> *mut libc::c_void {
    let mut rsa1024: [libc::c_uchar; 927] = *::std::mem::transmute::<
        &[u8; 927],
        &mut [libc::c_uchar; 927],
    >(
        b"{KDExOnByaXZhdGUta2V5KDE0OnJzYS1wa2NzMS1zaGExKDE6bjEyOToA90+K5EmjbFJBeJD xP2KD2Df+0Twc9425uB+vhqTrVijtd2PnwEQDfR2VoducgkKcXJzYYyCNILQJbFAi2Km/sD jImERBqDtaI217Ze+tOKEmImexYTAgFuqEptp2F3M4DqgRQ7s/3nJQ/bPE5Hfi1OZhJSShu I80ATTU4fUgrPspKDE6ZTM6AQABKSgxOmQxMjk6APAhKckzvxxkWfHJOpXDACWnaSKcbbvo vtWK3pGr/F2ya7CrLtE+uOx5F1sLs9G+/7flCy5k4uNILIYg4VTirZ1zQ8fNKPrjK1VMRls JiRRU/0VAs9d7HdncJfs6rbvRQbCRSRYURo4hWir3Lq8V3UUQVBprc4dO+uWmplvwQ5qxKS gxOnA2NToA+8aIVkdbk8Jg8dJOuzc7m/hZnwkKSs6eVDw4N/2T0CJKGJYT+B3Ct+fPkxhUR ggd2DQ9OpkTra7SXHTNkzdPVSkoMTpxNjU6APt11P8vXNnGYF0OC/cPsR8zhSYuFmtRuX6G ES+DdG0VCU07UeNQkok1UoW5sXqY0IGr1jkJq8AMSgKoNbLQ6w8pKDE6YTY0Ohzkxsan/8F wQDHgQbrIduXKVXaj0fONzKu8EXOTfUAYf0pdBsOlnq/+QVsPIrS6v7oNHK253YFEG84SdX kcktUpKDE6YjY1OgCR+cRtY3RWY+f6/TWK9gwPndv03xpasLWrMm71ky1aSbT9pasS9+opR tAiGzthfSbFsBiLQgb3VOr+AeIybT+XKSgxOmM2NDojigqARWN5u1CVDVuD2L2ManpoGiM6 kQ6FaJjqRjxeRRKFrQxGJa9tM1hqStxokC1oJidgaOLGnn60iwzToug9KSkp}\0",
    );
    let mut rsa2048: [libc::c_uchar; 1714] = *::std::mem::transmute::<
        &[u8; 1714],
        &mut [libc::c_uchar; 1714],
    >(
        b"{KDExOnByaXZhdGUta2V5KDE0OnJzYS1wa2NzMS1zaGExKDE6bjI1NzoAtxWXiglIdunDK48 8I0vW0wTqnh/riW9pLk8n1F8MUPBFdhvkkl0bDQqSJPUvSHy+w4fLVwcEzeI4qFyo3b2Avz JK20MFbt/WfHD1TbxuK8rNqXyqmqjJ9vgjtV9nPzAz7CM9ogs3/RJHpcfZPQF15ifizleUZ aQT0GAXHZL7cePj10yGI2u3hgTkokVzdNC/1T34guKYpErg0pt0B/KejWpsFTb84z3tkR+B YVx07p/OoByZwoABgncS/uALl31fRS8jyJ2JqUiZOqe7XoO9hkDHYNCWUGUfNGQ7ZgVp9+e NQpracSjrp6Jnrj7r/oxJUx5ZDVNi18AzQadE/oKOrSkoMTplMzoBAAEpKDE6ZDI1NjogBT C5vaHk2kF+LtDvw2XRBj0aZq7FHK0ioklvBSicR0l+vKYfSxVeFIk22YLphJfAjtFraRjYA Uaze3E1Rt1rkxoweupKV++lWAQvElOaaR/LErirz/Vysjdck1D1ZjLOi+NNofSq2DWbsvY1 iznZhQRP3lVf6XBls0iXrYs4gb0pBZqXLQW+j9Ihx6eantf1/6ZMKPgCkzcAZ0ABsCfaFSg ouNCzilblsgFEspEbb8QqrNQoStS3F/gMwWgDsr3+vQzBqt+7ykWoCJ9wctbYy9kEPq+hX3 GP0hG6HdS81r9E8pgdf3wloNNMzYUHwn7poXGpOi8tG0pmR56TqD/BKSgxOnAxMjk6AN4AJ TiGPm9We2ga3Y0jpTfA3mWpUbhYgaXYLWA1/riniwq16fqxRIkWQT/O2KKpBVe6WSvNYq9u lM8N6bdPtDytJs6AOXy0X5vtJ953ZYVMhHbhmUxhIL9I+s0O1+LxMF8b9U4CrFyaTxd8Un/ FXP1BvYJRrkoup6HYvOlGx36lKSgxOnExMjk6ANMfrfH6z/3o7K56aW6kSiloDDbKZQ0+W5 8LzP2ZOBLf6LX6jLhN3olU1Z0KGTM0S/1AxvwGjuRqhu+LcOJ7oUCUH3uusR5c5nSnArYPq +0wbco4BQngot/HmGN7U0EDsIWqPt/qoa/b8bCk+TOwJlknNq/PnZU26SPj48XS05lpKSgx OmExMjk6AJM2n3gLNW3ZeH5BindkktQU9qWNkV5geqDCaNyrEZ3bpI1WsrEGSj9p3Zz1ipz a3msdbLJqQS26c72WKUzg8tFltR0s1HJInjolGtIgdNbfNdwrn9+RbQjL2VyPokOg0wXO4W 14wlmqDhax33dRJmfe50964MvaglkGA8fhorrtKSgxOmIxMjk6AKMe+vrX2xRHf3dfxU5jS ZmsdqNuxZzx7UB5kazvUU/kCJ1yNH/CSoq5LULkpovVgFDwV84qEwWQ+SjkCBg1hWWsDJc3 ZkobZUQENigM+72LiYiQt/PlyHI2eRuEEdNN0nm0DFhdpQeHXLoq/RBerYJ8tdgpBYxgnMn KLhaOykbhKSgxOmMxMjg6MVlKj2bjb7qFQVkLO1OPg28jSrtRpnQCR+qegN4ZmNam/qbest 8yn0JQ6gxX7PvP382+jx7uHHWHYYqPq/Flf8gqtOOcjqS5TJgVHz3F3xHWquo1ZofGtCMro Dd2c0xjRjIVGvLV6Ngs+HRdljRav40vRpTyEoEdlzHBQiILesopKSk=}\0",
    );
    let mut ctx: *mut rsa_ctx = 0 as *mut rsa_ctx;
    let mut i: sexp_iterator = sexp_iterator {
        length: 0,
        buffer: 0 as *const uint8_t,
        start: 0,
        pos: 0,
        level: 0,
        type_0: SEXP_ATOM,
        display_length: 0,
        display: 0 as *const uint8_t,
        atom_length: 0,
        atom: 0 as *const uint8_t,
    };
    let mut res: libc::c_int = 0;
    ctx = xalloc(::std::mem::size_of::<rsa_ctx>() as libc::c_ulong) as *mut rsa_ctx;
    nettle_rsa_public_key_init(&mut (*ctx).pub_0);
    nettle_rsa_private_key_init(&mut (*ctx).key);
    __gmpz_init(((*ctx).s).as_mut_ptr());
    nettle_knuth_lfib_init(&mut (*ctx).lfib, 1 as libc::c_int as uint32_t);
    if size == 1024 as libc::c_int as libc::c_uint {
        res = nettle_sexp_transport_iterator_first(
            &mut i,
            (::std::mem::size_of::<[libc::c_uchar; 927]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            rsa1024.as_mut_ptr(),
        );
    } else if size == 2048 as libc::c_int as libc::c_uint {
        res = nettle_sexp_transport_iterator_first(
            &mut i,
            (::std::mem::size_of::<[libc::c_uchar; 1714]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            rsa2048.as_mut_ptr(),
        );
    } else {
        die(b"Internal error.\n\0" as *const u8 as *const libc::c_char);
    }
    if !(res != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            b"private-key\0" as *const u8 as *const libc::c_char,
        ) != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            b"rsa-pkcs1-sha1\0" as *const u8 as *const libc::c_char,
        ) != 0
        && nettle_rsa_keypair_from_sexp_alist(
            &mut (*ctx).pub_0,
            &mut (*ctx).key,
            0 as libc::c_int as libc::c_uint,
            &mut i,
        ) != 0)
    {
        die(b"Internal error.\n\0" as *const u8 as *const libc::c_char);
    }
    (*ctx)
        .digest = hash_string(
        &nettle_sha256,
        b"foo\0" as *const u8 as *const libc::c_char,
    );
    nettle_rsa_sha256_sign_digest(
        &mut (*ctx).key,
        (*ctx).digest,
        ((*ctx).s).as_mut_ptr(),
    );
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_rsa_sign(mut p: *mut libc::c_void) {
    let mut ctx: *mut rsa_ctx = p as *mut rsa_ctx;
    let mut s: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(s.as_mut_ptr());
    nettle_rsa_sha256_sign_digest(&mut (*ctx).key, (*ctx).digest, s.as_mut_ptr());
    __gmpz_clear(s.as_mut_ptr());
}
unsafe extern "C" fn bench_rsa_sign_tr(mut p: *mut libc::c_void) {
    let mut ctx: *mut rsa_ctx = p as *mut rsa_ctx;
    let mut s: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(s.as_mut_ptr());
    nettle_rsa_sha256_sign_digest_tr(
        &mut (*ctx).pub_0,
        &mut (*ctx).key,
        &mut (*ctx).lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut knuth_lfib_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_knuth_lfib_random
                    as unsafe extern "C" fn(
                        *mut knuth_lfib_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        (*ctx).digest,
        s.as_mut_ptr(),
    );
    __gmpz_clear(s.as_mut_ptr());
}
unsafe extern "C" fn bench_rsa_verify(mut p: *mut libc::c_void) {
    let mut ctx: *mut rsa_ctx = p as *mut rsa_ctx;
    if nettle_rsa_sha256_verify_digest(
        &mut (*ctx).pub_0,
        (*ctx).digest,
        ((*ctx).s).as_mut_ptr() as *const __mpz_struct,
    ) == 0
    {
        die(
            b"Internal error, rsa_sha256_verify_digest failed.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn bench_rsa_clear(mut p: *mut libc::c_void) {
    let mut ctx: *mut rsa_ctx = p as *mut rsa_ctx;
    nettle_rsa_public_key_clear(&mut (*ctx).pub_0);
    nettle_rsa_private_key_clear(&mut (*ctx).key);
    __gmpz_clear(((*ctx).s).as_mut_ptr());
    free((*ctx).digest as *mut libc::c_void);
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn bench_dsa_init(mut size: libc::c_uint) -> *mut libc::c_void {
    let mut ctx: *mut dsa_ctx = 0 as *mut dsa_ctx;
    let mut i: sexp_iterator = sexp_iterator {
        length: 0,
        buffer: 0 as *const uint8_t,
        start: 0,
        pos: 0,
        level: 0,
        type_0: SEXP_ATOM,
        display_length: 0,
        display: 0 as *const uint8_t,
        atom_length: 0,
        atom: 0 as *const uint8_t,
    };
    let mut dsa1024: [libc::c_uchar; 672] = *::std::mem::transmute::<
        &[u8; 672],
        &mut [libc::c_uchar; 672],
    >(
        b"{KDExOnByaXZhdGUta2V5KDM6ZHNhKDE6cDEyOToA2q4hOXEClLMXXMOl9xaPcGC/GeGmCMv VCaaW0uWc50DvvmJDPQPdCehyfZr/1dv2UDbx06TC7ls/IFd+BsDzGBRxqIQ44J20cn+0gt NMIXAocE1QhCCFaT5gXrk8zMlqBEGaP3RdpgxNanEXkTj2Wma8r1GtrLX3HPafio62jicpK DE6cTIxOgDN9pcW3exdVAesC9WsxwCGoJK24ykoMTpnMTI5OgCJr9DmKdiE0WJZB7HACESv Tpg1qZgc8E15byQ+OsHUyOTRrJRTcrgKZJW7dFRJ9cXmyi7XYCd3bJtu/2HRHLY1vd4qMvU 7Y8x08ooCoABGV7nGqcmdQfEho1OY6TZh2NikmPKZLeur3PZFpcZ8Dl+KVWtwC55plNC7Om iAQy8MaCkoMTp5MTI5OgDakk0LOUQwzOKt9FHOBmBKrWsvTm7549eScTUqH4OMm3btjUsXz MmlqEe+imwQCOW/AE3Xw9pXZeETWK0jlLe8k5vnKcNskerFwZ1eQKtOPPQty8IqQ9PEfF6B 0oVQiJg2maHUDWFnDkIBd7ZR1z8FnZMUxH9mH4kEUo6YQgtCdykoMTp4MjA6cOl3ijiiMjI pesFD8jxESWb2mn8pKSk=}\0",
    );
    ctx = xalloc(::std::mem::size_of::<dsa_ctx>() as libc::c_ulong) as *mut dsa_ctx;
    nettle_dsa_params_init(&mut (*ctx).params);
    __gmpz_init(((*ctx).pub_0).as_mut_ptr());
    __gmpz_init(((*ctx).key).as_mut_ptr());
    nettle_dsa_signature_init(&mut (*ctx).s);
    nettle_knuth_lfib_init(&mut (*ctx).lfib, 1 as libc::c_int as uint32_t);
    if size != 1024 as libc::c_int as libc::c_uint {
        die(b"Internal error.\n\0" as *const u8 as *const libc::c_char);
    }
    if !(nettle_sexp_transport_iterator_first(
        &mut i,
        (::std::mem::size_of::<[libc::c_uchar; 672]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        dsa1024.as_mut_ptr(),
    ) != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            b"private-key\0" as *const u8 as *const libc::c_char,
        ) != 0
        && nettle_sexp_iterator_check_type(
            &mut i,
            b"dsa\0" as *const u8 as *const libc::c_char,
        ) != 0
        && nettle_dsa_keypair_from_sexp_alist(
            &mut (*ctx).params,
            ((*ctx).pub_0).as_mut_ptr(),
            ((*ctx).key).as_mut_ptr(),
            0 as libc::c_int as libc::c_uint,
            160 as libc::c_int as libc::c_uint,
            &mut i,
        ) != 0)
    {
        die(b"Internal error.\n\0" as *const u8 as *const libc::c_char);
    }
    (*ctx)
        .digest = hash_string(
        &nettle_sha1,
        b"foo\0" as *const u8 as *const libc::c_char,
    );
    nettle_dsa_sign(
        &mut (*ctx).params,
        ((*ctx).key).as_mut_ptr() as *const __mpz_struct,
        &mut (*ctx).lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut knuth_lfib_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_knuth_lfib_random
                    as unsafe extern "C" fn(
                        *mut knuth_lfib_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        20 as libc::c_int as size_t,
        (*ctx).digest,
        &mut (*ctx).s,
    );
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_dsa_sign(mut p: *mut libc::c_void) {
    let mut ctx: *mut dsa_ctx = p as *mut dsa_ctx;
    let mut s: dsa_signature = dsa_signature {
        r: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        s: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    nettle_dsa_signature_init(&mut s);
    nettle_dsa_sign(
        &mut (*ctx).params,
        ((*ctx).key).as_mut_ptr() as *const __mpz_struct,
        &mut (*ctx).lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut knuth_lfib_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_knuth_lfib_random
                    as unsafe extern "C" fn(
                        *mut knuth_lfib_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        20 as libc::c_int as size_t,
        (*ctx).digest,
        &mut s,
    );
    nettle_dsa_signature_clear(&mut s);
}
unsafe extern "C" fn bench_dsa_verify(mut p: *mut libc::c_void) {
    let mut ctx: *mut dsa_ctx = p as *mut dsa_ctx;
    if nettle_dsa_verify(
        &mut (*ctx).params,
        ((*ctx).pub_0).as_mut_ptr() as *const __mpz_struct,
        20 as libc::c_int as size_t,
        (*ctx).digest,
        &mut (*ctx).s,
    ) == 0
    {
        die(
            b"Internal error, dsa_sha1_verify_digest failed.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn bench_dsa_clear(mut p: *mut libc::c_void) {
    let mut ctx: *mut dsa_ctx = p as *mut dsa_ctx;
    nettle_dsa_params_clear(&mut (*ctx).params);
    __gmpz_clear(((*ctx).pub_0).as_mut_ptr());
    __gmpz_clear(((*ctx).key).as_mut_ptr());
    nettle_dsa_signature_clear(&mut (*ctx).s);
    free((*ctx).digest as *mut libc::c_void);
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn bench_ecdsa_init(mut size: libc::c_uint) -> *mut libc::c_void {
    let mut ctx: *mut ecdsa_ctx = 0 as *mut ecdsa_ctx;
    let mut ecc: *const ecc_curve = 0 as *const ecc_curve;
    let mut xs: *const libc::c_char = 0 as *const libc::c_char;
    let mut ys: *const libc::c_char = 0 as *const libc::c_char;
    let mut zs: *const libc::c_char = 0 as *const libc::c_char;
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut y: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut z: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    ctx = xalloc(::std::mem::size_of::<ecdsa_ctx>() as libc::c_ulong) as *mut ecdsa_ctx;
    nettle_dsa_signature_init(&mut (*ctx).s);
    nettle_knuth_lfib_init(&mut (*ctx).lfib, 17 as libc::c_int as uint32_t);
    match size {
        192 => {
            ecc = &_nettle_secp_192r1;
            xs = b"8e8e07360350fb6b7ad8370cfd32fa8c6bba785e6e200599\0" as *const u8
                as *const libc::c_char;
            ys = b"7f82ddb58a43d59ff8dc66053002b918b99bd01bd68d6736\0" as *const u8
                as *const libc::c_char;
            zs = b"f2e620e086d658b4b507996988480917640e4dc107808bdd\0" as *const u8
                as *const libc::c_char;
            (*ctx)
                .digest = hash_string(
                &nettle_sha1,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
            (*ctx).digest_size = 20 as libc::c_int as libc::c_uint;
        }
        224 => {
            ecc = &_nettle_secp_224r1;
            xs = b"993bf363f4f2bc0f255f22563980449164e9c894d9efd088d7b77334\0"
                as *const u8 as *const libc::c_char;
            ys = b"b75fff9849997d02d135140e4d0030944589586e22df1fc4b629082a\0"
                as *const u8 as *const libc::c_char;
            zs = b"cdfd01838247f5de3cc70b688418046f10a2bfaca6de9ec836d48c27\0"
                as *const u8 as *const libc::c_char;
            (*ctx)
                .digest = hash_string(
                &nettle_sha224,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
            (*ctx).digest_size = 28 as libc::c_int as libc::c_uint;
        }
        256 => {
            ecc = &_nettle_secp_256r1;
            xs = b"2442A5CC 0ECD015F A3CA31DC 8E2BBC70 BF42D60C BCA20085 E0822CB0 4235E970\0"
                as *const u8 as *const libc::c_char;
            ys = b"6FC98BD7 E50211A4 A27102FA 3549DF79 EBCB4BF2 46B80945 CDDFE7D5 09BBFD7D\0"
                as *const u8 as *const libc::c_char;
            zs = b"DC51D386 6A15BACD E33D96F9 92FCA99D A7E6EF09 34E70975 59C27F16 14C88A7F\0"
                as *const u8 as *const libc::c_char;
            (*ctx)
                .digest = hash_string(
                &nettle_sha256,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
            (*ctx).digest_size = 32 as libc::c_int as libc::c_uint;
        }
        384 => {
            ecc = &_nettle_secp_384r1;
            xs = b"96281BF8 DD5E0525 CA049C04 8D345D30 82968D10 FEDF5C5A CA0C64E6 465A97EA5CE10C9D FEC21797 41571072 1F437922\0"
                as *const u8 as *const libc::c_char;
            ys = b"447688BA 94708EB6 E2E4D59F 6AB6D7ED FF9301D2 49FE49C3 3096655F 5D502FAD3D383B91 C5E7EDAA 2B714CC9 9D5743CA\0"
                as *const u8 as *const libc::c_char;
            zs = b"0BEB6466 34BA8773 5D77AE48 09A0EBEA 865535DE 4C1E1DCB 692E8470 8E81A5AF62E528C3 8B2A81B3 5309668D 73524D9F\0"
                as *const u8 as *const libc::c_char;
            (*ctx)
                .digest = hash_string(
                &nettle_sha384,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
            (*ctx).digest_size = 48 as libc::c_int as libc::c_uint;
        }
        521 => {
            ecc = &_nettle_secp_521r1;
            xs = b"0151518F 1AF0F563 517EDD54 85190DF9 5A4BF57B 5CBA4CF2 A9A3F647 4725A35F7AFE0A6D DEB8BEDB CD6A197E 592D4018 8901CECD 650699C9 B5E456AE A5ADD19052A8\0"
                as *const u8 as *const libc::c_char;
            ys = b"006F3B14 2EA1BFFF 7E2837AD 44C9E4FF 6D2D34C7 3184BBAD 90026DD5 E6E85317D9DF45CA D7803C6C 20035B2F 3FF63AFF 4E1BA64D 1C077577 DA3F4286 C58F0AEAE643\0"
                as *const u8 as *const libc::c_char;
            zs = b"0065FDA3 409451DC AB0A0EAD 45495112 A3D813C1 7BFD34BD F8C1209D 7DF5849120597779 060A7FF9 D704ADF7 8B570FFA D6F062E9 5C7E0C5D 5481C5B1 53B48B375FA1\0"
                as *const u8 as *const libc::c_char;
            (*ctx)
                .digest = hash_string(
                &nettle_sha512,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
            (*ctx).digest_size = 64 as libc::c_int as libc::c_uint;
        }
        _ => {
            die(b"Internal error.\n\0" as *const u8 as *const libc::c_char);
        }
    }
    nettle_ecc_point_init(&mut (*ctx).pub_0, ecc);
    nettle_ecc_scalar_init(&mut (*ctx).key, ecc);
    __gmpz_init_set_str(x.as_mut_ptr(), xs, 16 as libc::c_int);
    __gmpz_init_set_str(y.as_mut_ptr(), ys, 16 as libc::c_int);
    __gmpz_init_set_str(z.as_mut_ptr(), zs, 16 as libc::c_int);
    nettle_ecc_point_set(
        &mut (*ctx).pub_0,
        x.as_mut_ptr() as *const __mpz_struct,
        y.as_mut_ptr() as *const __mpz_struct,
    );
    nettle_ecc_scalar_set(&mut (*ctx).key, z.as_mut_ptr() as *const __mpz_struct);
    __gmpz_clear(x.as_mut_ptr());
    __gmpz_clear(y.as_mut_ptr());
    __gmpz_clear(z.as_mut_ptr());
    nettle_ecdsa_sign(
        &mut (*ctx).key,
        &mut (*ctx).lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut knuth_lfib_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_knuth_lfib_random
                    as unsafe extern "C" fn(
                        *mut knuth_lfib_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        (*ctx).digest_size as size_t,
        (*ctx).digest,
        &mut (*ctx).s,
    );
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_ecdsa_sign(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecdsa_ctx = p as *mut ecdsa_ctx;
    let mut s: dsa_signature = dsa_signature {
        r: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        s: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    nettle_dsa_signature_init(&mut s);
    nettle_ecdsa_sign(
        &mut (*ctx).key,
        &mut (*ctx).lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut knuth_lfib_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_knuth_lfib_random
                    as unsafe extern "C" fn(
                        *mut knuth_lfib_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        (*ctx).digest_size as size_t,
        (*ctx).digest,
        &mut s,
    );
    nettle_dsa_signature_clear(&mut s);
}
unsafe extern "C" fn bench_ecdsa_verify(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecdsa_ctx = p as *mut ecdsa_ctx;
    if nettle_ecdsa_verify(
        &mut (*ctx).pub_0,
        (*ctx).digest_size as size_t,
        (*ctx).digest,
        &mut (*ctx).s,
    ) == 0
    {
        die(
            b"Internal error, ecdsa_verify failed.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn bench_ecdsa_clear(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecdsa_ctx = p as *mut ecdsa_ctx;
    nettle_ecc_point_clear(&mut (*ctx).pub_0);
    nettle_ecc_scalar_clear(&mut (*ctx).key);
    nettle_dsa_signature_clear(&mut (*ctx).s);
    free((*ctx).digest as *mut libc::c_void);
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn bench_eddsa_init(mut size: libc::c_uint) -> *mut libc::c_void {
    let mut lfib: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut ctx: *mut eddsa_ctx = 0 as *mut eddsa_ctx;
    nettle_knuth_lfib_init(&mut lfib, 17 as libc::c_int as uint32_t);
    ctx = xalloc(::std::mem::size_of::<eddsa_ctx>() as libc::c_ulong) as *mut eddsa_ctx;
    match size {
        255 => {
            (*ctx)
                .sign = Some(
                nettle_ed25519_sha512_sign
                    as unsafe extern "C" fn(
                        *const uint8_t,
                        *const uint8_t,
                        size_t,
                        *const uint8_t,
                        *mut uint8_t,
                    ) -> (),
            );
            (*ctx)
                .verify = Some(
                nettle_ed25519_sha512_verify
                    as unsafe extern "C" fn(
                        *const uint8_t,
                        size_t,
                        *const uint8_t,
                        *const uint8_t,
                    ) -> libc::c_int,
            );
            nettle_knuth_lfib_random(
                &mut lfib,
                32 as libc::c_int as size_t,
                ((*ctx).key).as_mut_ptr(),
            );
            nettle_ed25519_sha512_public_key(
                ((*ctx).pub_0).as_mut_ptr(),
                ((*ctx).key).as_mut_ptr(),
            );
        }
        448 => {
            (*ctx)
                .sign = Some(
                nettle_ed448_shake256_sign
                    as unsafe extern "C" fn(
                        *const uint8_t,
                        *const uint8_t,
                        size_t,
                        *const uint8_t,
                        *mut uint8_t,
                    ) -> (),
            );
            (*ctx)
                .verify = Some(
                nettle_ed448_shake256_verify
                    as unsafe extern "C" fn(
                        *const uint8_t,
                        size_t,
                        *const uint8_t,
                        *const uint8_t,
                    ) -> libc::c_int,
            );
            nettle_knuth_lfib_random(
                &mut lfib,
                57 as libc::c_int as size_t,
                ((*ctx).key).as_mut_ptr(),
            );
            nettle_ed448_shake256_public_key(
                ((*ctx).pub_0).as_mut_ptr(),
                ((*ctx).key).as_mut_ptr(),
            );
        }
        _ => {
            abort();
        }
    }
    ((*ctx).sign)
        .unwrap()(
        ((*ctx).pub_0).as_mut_ptr(),
        ((*ctx).key).as_mut_ptr(),
        3 as libc::c_int as size_t,
        b"abc\0" as *const u8 as *const libc::c_char as *const uint8_t,
        ((*ctx).signature).as_mut_ptr(),
    );
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_eddsa_sign(mut p: *mut libc::c_void) {
    let mut ctx: *mut eddsa_ctx = p as *mut eddsa_ctx;
    ((*ctx).sign)
        .unwrap()(
        ((*ctx).pub_0).as_mut_ptr(),
        ((*ctx).key).as_mut_ptr(),
        3 as libc::c_int as size_t,
        b"abc\0" as *const u8 as *const libc::c_char as *const uint8_t,
        ((*ctx).signature).as_mut_ptr(),
    );
}
unsafe extern "C" fn bench_eddsa_verify(mut p: *mut libc::c_void) {
    let mut ctx: *mut eddsa_ctx = p as *mut eddsa_ctx;
    if ((*ctx).verify)
        .unwrap()(
        ((*ctx).pub_0).as_mut_ptr(),
        3 as libc::c_int as size_t,
        b"abc\0" as *const u8 as *const libc::c_char as *const uint8_t,
        ((*ctx).signature).as_mut_ptr(),
    ) == 0
    {
        die(
            b"Internal error, eddsa_verify failed.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn bench_eddsa_clear(mut p: *mut libc::c_void) {
    free(p);
}
unsafe extern "C" fn bench_gostdsa_init(mut size: libc::c_uint) -> *mut libc::c_void {
    let mut ctx: *mut ecdsa_ctx = 0 as *mut ecdsa_ctx;
    let mut ecc: *const ecc_curve = 0 as *const ecc_curve;
    let mut xs: *const libc::c_char = 0 as *const libc::c_char;
    let mut ys: *const libc::c_char = 0 as *const libc::c_char;
    let mut zs: *const libc::c_char = 0 as *const libc::c_char;
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut y: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut z: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    ctx = xalloc(::std::mem::size_of::<ecdsa_ctx>() as libc::c_ulong) as *mut ecdsa_ctx;
    nettle_dsa_signature_init(&mut (*ctx).s);
    nettle_knuth_lfib_init(&mut (*ctx).lfib, 17 as libc::c_int as uint32_t);
    match size {
        256 => {
            ecc = &_nettle_gost_gc256b;
            xs = b"971566ceda436ee7678f7e07e84ebb7217406c0b4747aa8fd2ab1453c3d0dfba\0"
                as *const u8 as *const libc::c_char;
            ys = b"ad58736965949f8e59830f8de20fc6c0d177f6ab599874f1e2e24ff71f9ce643\0"
                as *const u8 as *const libc::c_char;
            zs = b"bfcf1d623e5cdd3032a7c6eabb4a923c46e43d640ffeaaf2c3ed39a8fa399924\0"
                as *const u8 as *const libc::c_char;
            (*ctx)
                .digest = hash_string(
                &nettle_sha256,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
            (*ctx).digest_size = 32 as libc::c_int as libc::c_uint;
        }
        512 => {
            ecc = &_nettle_gost_gc512a;
            xs = b"03A36340A95BB5F93D131961B5B1C1B3213DF7FF3B5A30376407E2A65C441BC6D1B34662317083243F007B15A8512B526606D3B172B606DCE86DBD6F82DA3D40\0"
                as *const u8 as *const libc::c_char;
            ys = b"DEAD76318012FED79507809C89CC44848743640EAC9A3C847DA9082E050760A10679F4B707ABC1872640AD20D7441F66C7A8B3BFF1B8E11B4A076F0A86749F73\0"
                as *const u8 as *const libc::c_char;
            zs = b"3FC01CDCD4EC5F972EB482774C41E66DB7F380528DFE9E67992BA05AEE462435757530E641077CE587B976C8EEB48C48FD33FD175F0C7DE6A44E014E6BCB074B\0"
                as *const u8 as *const libc::c_char;
            (*ctx)
                .digest = hash_string(
                &nettle_sha512,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
            (*ctx).digest_size = 64 as libc::c_int as libc::c_uint;
        }
        _ => {
            die(b"Internal error.\n\0" as *const u8 as *const libc::c_char);
        }
    }
    nettle_ecc_point_init(&mut (*ctx).pub_0, ecc);
    nettle_ecc_scalar_init(&mut (*ctx).key, ecc);
    __gmpz_init_set_str(x.as_mut_ptr(), xs, 16 as libc::c_int);
    __gmpz_init_set_str(y.as_mut_ptr(), ys, 16 as libc::c_int);
    __gmpz_init_set_str(z.as_mut_ptr(), zs, 16 as libc::c_int);
    nettle_ecc_point_set(
        &mut (*ctx).pub_0,
        x.as_mut_ptr() as *const __mpz_struct,
        y.as_mut_ptr() as *const __mpz_struct,
    );
    nettle_ecc_scalar_set(&mut (*ctx).key, z.as_mut_ptr() as *const __mpz_struct);
    __gmpz_clear(x.as_mut_ptr());
    __gmpz_clear(y.as_mut_ptr());
    __gmpz_clear(z.as_mut_ptr());
    nettle_gostdsa_sign(
        &mut (*ctx).key,
        &mut (*ctx).lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut knuth_lfib_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_knuth_lfib_random
                    as unsafe extern "C" fn(
                        *mut knuth_lfib_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        (*ctx).digest_size as size_t,
        (*ctx).digest,
        &mut (*ctx).s,
    );
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_gostdsa_sign(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecdsa_ctx = p as *mut ecdsa_ctx;
    let mut s: dsa_signature = dsa_signature {
        r: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        s: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    nettle_dsa_signature_init(&mut s);
    nettle_gostdsa_sign(
        &mut (*ctx).key,
        &mut (*ctx).lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut knuth_lfib_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_knuth_lfib_random
                    as unsafe extern "C" fn(
                        *mut knuth_lfib_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        (*ctx).digest_size as size_t,
        (*ctx).digest,
        &mut s,
    );
    nettle_dsa_signature_clear(&mut s);
}
unsafe extern "C" fn bench_gostdsa_verify(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecdsa_ctx = p as *mut ecdsa_ctx;
    if nettle_gostdsa_verify(
        &mut (*ctx).pub_0,
        (*ctx).digest_size as size_t,
        (*ctx).digest,
        &mut (*ctx).s,
    ) == 0
    {
        die(
            b"Internal error, _gostdsa_verify failed.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn bench_gostdsa_clear(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecdsa_ctx = p as *mut ecdsa_ctx;
    nettle_ecc_point_clear(&mut (*ctx).pub_0);
    nettle_ecc_scalar_clear(&mut (*ctx).key);
    nettle_dsa_signature_clear(&mut (*ctx).s);
    free((*ctx).digest as *mut libc::c_void);
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn make_openssl_rsa_ctx(
    mut size: libc::c_uint,
) -> *mut openssl_rsa_ctx {
    let mut ctx: *mut openssl_rsa_ctx = xalloc(
        ::std::mem::size_of::<openssl_rsa_ctx>() as libc::c_ulong,
    ) as *mut openssl_rsa_ctx;
    let mut e: *mut BIGNUM = BN_new();
    BN_set_word(e, 65537 as libc::c_int as libc::c_ulong);
    (*ctx).key = RSA_new();
    RSA_generate_key_ex((*ctx).key, size as libc::c_int, e, 0 as *mut BN_GENCB);
    (*ctx).ref_0 = xalloc(RSA_size((*ctx).key) as size_t) as *mut libc::c_uchar;
    (*ctx).signature = xalloc(RSA_size((*ctx).key) as size_t) as *mut libc::c_uchar;
    (*ctx)
        .digest = hash_string(
        &nettle_sha1,
        b"foo\0" as *const u8 as *const libc::c_char,
    );
    if RSA_sign(
        64 as libc::c_int,
        (*ctx).digest,
        20 as libc::c_int as libc::c_uint,
        (*ctx).ref_0,
        &mut (*ctx).siglen,
        (*ctx).key,
    ) == 0
    {
        die(b"OpenSSL RSA_sign failed.\n\0" as *const u8 as *const libc::c_char);
    }
    BN_free(e);
    return ctx;
}
unsafe extern "C" fn bench_openssl_rsa_init(
    mut size: libc::c_uint,
) -> *mut libc::c_void {
    let mut ctx: *mut openssl_rsa_ctx = make_openssl_rsa_ctx(size);
    RSA_blinding_off((*ctx).key);
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_openssl_rsa_tr_init(
    mut size: libc::c_uint,
) -> *mut libc::c_void {
    return make_openssl_rsa_ctx(size) as *mut libc::c_void;
}
unsafe extern "C" fn bench_openssl_rsa_sign(mut p: *mut libc::c_void) {
    let mut ctx: *const openssl_rsa_ctx = p as *const openssl_rsa_ctx;
    let mut siglen: libc::c_uint = 0;
    if RSA_sign(
        64 as libc::c_int,
        (*ctx).digest,
        20 as libc::c_int as libc::c_uint,
        (*ctx).signature,
        &mut siglen,
        (*ctx).key,
    ) == 0
    {
        die(b"OpenSSL RSA_sign failed.\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn bench_openssl_rsa_verify(mut p: *mut libc::c_void) {
    let mut ctx: *const openssl_rsa_ctx = p as *const openssl_rsa_ctx;
    if RSA_verify(
        64 as libc::c_int,
        (*ctx).digest,
        20 as libc::c_int as libc::c_uint,
        (*ctx).ref_0,
        (*ctx).siglen,
        (*ctx).key,
    ) == 0
    {
        die(b"OpenSSL RSA_verify failed.\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn bench_openssl_rsa_clear(mut p: *mut libc::c_void) {
    let mut ctx: *mut openssl_rsa_ctx = p as *mut openssl_rsa_ctx;
    RSA_free((*ctx).key);
    free((*ctx).ref_0 as *mut libc::c_void);
    free((*ctx).signature as *mut libc::c_void);
    free((*ctx).digest as *mut libc::c_void);
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn bench_openssl_ecdsa_init(
    mut size: libc::c_uint,
) -> *mut libc::c_void {
    let mut ctx: *mut openssl_ecdsa_ctx = xalloc(
        ::std::mem::size_of::<openssl_ecdsa_ctx>() as libc::c_ulong,
    ) as *mut openssl_ecdsa_ctx;
    match size {
        192 => {
            (*ctx).key = EC_KEY_new_by_curve_name(409 as libc::c_int);
            (*ctx).digest_length = 24 as libc::c_int as libc::c_uint;
            (*ctx)
                .digest = hash_string(
                &nettle_sha224,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
        }
        224 => {
            (*ctx).key = EC_KEY_new_by_curve_name(713 as libc::c_int);
            (*ctx).digest_length = 28 as libc::c_int as libc::c_uint;
            (*ctx)
                .digest = hash_string(
                &nettle_sha224,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
        }
        256 => {
            (*ctx).key = EC_KEY_new_by_curve_name(415 as libc::c_int);
            (*ctx).digest_length = 32 as libc::c_int as libc::c_uint;
            (*ctx)
                .digest = hash_string(
                &nettle_sha256,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
        }
        384 => {
            (*ctx).key = EC_KEY_new_by_curve_name(715 as libc::c_int);
            (*ctx).digest_length = 48 as libc::c_int as libc::c_uint;
            (*ctx)
                .digest = hash_string(
                &nettle_sha384,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
        }
        521 => {
            (*ctx).key = EC_KEY_new_by_curve_name(716 as libc::c_int);
            (*ctx).digest_length = 64 as libc::c_int as libc::c_uint;
            (*ctx)
                .digest = hash_string(
                &nettle_sha512,
                b"abc\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            die(b"Internal error.\n\0" as *const u8 as *const libc::c_char);
        }
    }
    if ((*ctx).key).is_null() {
        free(ctx as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    if EC_KEY_generate_key((*ctx).key) == 0 {
        die(
            b"Openssl EC_KEY_generate_key failed.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*ctx)
        .signature = ECDSA_do_sign(
        (*ctx).digest,
        (*ctx).digest_length as libc::c_int,
        (*ctx).key,
    );
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_openssl_ecdsa_sign(mut p: *mut libc::c_void) {
    let mut ctx: *const openssl_ecdsa_ctx = p as *const openssl_ecdsa_ctx;
    let mut sig: *mut ECDSA_SIG = ECDSA_do_sign(
        (*ctx).digest,
        (*ctx).digest_length as libc::c_int,
        (*ctx).key,
    );
    ECDSA_SIG_free(sig);
}
unsafe extern "C" fn bench_openssl_ecdsa_verify(mut p: *mut libc::c_void) {
    let mut ctx: *const openssl_ecdsa_ctx = p as *const openssl_ecdsa_ctx;
    if ECDSA_do_verify(
        (*ctx).digest,
        (*ctx).digest_length as libc::c_int,
        (*ctx).signature,
        (*ctx).key,
    ) != 1 as libc::c_int
    {
        die(b"Openssl ECDSA_do_verify failed.\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn bench_openssl_ecdsa_clear(mut p: *mut libc::c_void) {
    let mut ctx: *mut openssl_ecdsa_ctx = p as *mut openssl_ecdsa_ctx;
    ECDSA_SIG_free((*ctx).signature);
    EC_KEY_free((*ctx).key);
    free((*ctx).digest as *mut libc::c_void);
    free(ctx as *mut libc::c_void);
}
unsafe extern "C" fn bench_curve_init(mut size: libc::c_uint) -> *mut libc::c_void {
    let mut lfib: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut ctx: *mut curve_ctx = xalloc(
        ::std::mem::size_of::<curve_ctx>() as libc::c_ulong,
    ) as *mut curve_ctx;
    nettle_knuth_lfib_init(&mut lfib, 17 as libc::c_int as uint32_t);
    match size {
        255 => {
            (*ctx)
                .mul = Some(
                nettle_curve25519_mul
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        *const uint8_t,
                    ) -> (),
            );
            (*ctx)
                .mul_g = Some(
                nettle_curve25519_mul_g
                    as unsafe extern "C" fn(*mut uint8_t, *const uint8_t) -> (),
            );
            nettle_knuth_lfib_random(
                &mut lfib,
                32 as libc::c_int as size_t,
                ((*ctx).s).as_mut_ptr(),
            );
        }
        448 => {
            (*ctx)
                .mul = Some(
                nettle_curve448_mul
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        *const uint8_t,
                    ) -> (),
            );
            (*ctx)
                .mul_g = Some(
                nettle_curve448_mul_g
                    as unsafe extern "C" fn(*mut uint8_t, *const uint8_t) -> (),
            );
            nettle_knuth_lfib_random(
                &mut lfib,
                56 as libc::c_int as size_t,
                ((*ctx).s).as_mut_ptr(),
            );
        }
        _ => {
            abort();
        }
    }
    ((*ctx).mul_g).unwrap()(((*ctx).x).as_mut_ptr(), ((*ctx).s).as_mut_ptr());
    return ctx as *mut libc::c_void;
}
unsafe extern "C" fn bench_curve_mul_g(mut p: *mut libc::c_void) {
    let mut ctx: *mut curve_ctx = p as *mut curve_ctx;
    let mut q: [uint8_t; 56] = [0; 56];
    ((*ctx).mul_g).unwrap()(q.as_mut_ptr(), ((*ctx).s).as_mut_ptr());
}
unsafe extern "C" fn bench_curve_mul(mut p: *mut libc::c_void) {
    let mut ctx: *mut curve_ctx = p as *mut curve_ctx;
    let mut q: [uint8_t; 56] = [0; 56];
    ((*ctx).mul)
        .unwrap()(q.as_mut_ptr(), ((*ctx).s).as_mut_ptr(), ((*ctx).x).as_mut_ptr());
}
unsafe extern "C" fn bench_curve_clear(mut p: *mut libc::c_void) {
    free(p);
}
pub static mut alg_list: [alg; 25] = unsafe {
    [
        {
            let mut init = alg {
                name: b"rsa\0" as *const u8 as *const libc::c_char,
                size: 1024 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_rsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_rsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_rsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_rsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"rsa\0" as *const u8 as *const libc::c_char,
                size: 2048 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_rsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_rsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_rsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_rsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"rsa-tr\0" as *const u8 as *const libc::c_char,
                size: 1024 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_rsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_rsa_sign_tr as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_rsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_rsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"rsa-tr\0" as *const u8 as *const libc::c_char,
                size: 2048 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_rsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_rsa_sign_tr as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_rsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_rsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"rsa (openssl)\0" as *const u8 as *const libc::c_char,
                size: 1024 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_rsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_rsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_rsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_rsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"rsa (openssl)\0" as *const u8 as *const libc::c_char,
                size: 2048 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_rsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_rsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_rsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_rsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"rsa-tr (openssl)\0" as *const u8 as *const libc::c_char,
                size: 1024 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_rsa_tr_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_rsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_rsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_rsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"rsa-tr (openssl)\0" as *const u8 as *const libc::c_char,
                size: 2048 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_rsa_tr_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_rsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_rsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_rsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"dsa\0" as *const u8 as *const libc::c_char,
                size: 1024 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_dsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_dsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_dsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_dsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa\0" as *const u8 as *const libc::c_char,
                size: 192 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_ecdsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_ecdsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_ecdsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa\0" as *const u8 as *const libc::c_char,
                size: 224 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_ecdsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_ecdsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_ecdsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa\0" as *const u8 as *const libc::c_char,
                size: 256 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_ecdsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_ecdsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_ecdsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa\0" as *const u8 as *const libc::c_char,
                size: 384 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_ecdsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_ecdsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_ecdsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa\0" as *const u8 as *const libc::c_char,
                size: 521 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_ecdsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_ecdsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_ecdsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa (openssl)\0" as *const u8 as *const libc::c_char,
                size: 192 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_ecdsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_ecdsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_ecdsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa (openssl)\0" as *const u8 as *const libc::c_char,
                size: 224 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_ecdsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_ecdsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_ecdsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa (openssl)\0" as *const u8 as *const libc::c_char,
                size: 256 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_ecdsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_ecdsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_ecdsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa (openssl)\0" as *const u8 as *const libc::c_char,
                size: 384 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_ecdsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_ecdsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_ecdsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"ecdsa (openssl)\0" as *const u8 as *const libc::c_char,
                size: 521 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_openssl_ecdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_openssl_ecdsa_sign
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_openssl_ecdsa_verify
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_openssl_ecdsa_clear
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"eddsa\0" as *const u8 as *const libc::c_char,
                size: 255 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_eddsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_eddsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_eddsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_eddsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"eddsa\0" as *const u8 as *const libc::c_char,
                size: 448 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_eddsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_eddsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_eddsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_eddsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"curve\0" as *const u8 as *const libc::c_char,
                size: 255 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_curve_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_curve_mul_g as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_curve_mul as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_curve_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"curve\0" as *const u8 as *const libc::c_char,
                size: 448 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_curve_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_curve_mul_g as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_curve_mul as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_curve_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"gostdsa\0" as *const u8 as *const libc::c_char,
                size: 256 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_gostdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_gostdsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_gostdsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_gostdsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
        {
            let mut init = alg {
                name: b"gostdsa\0" as *const u8 as *const libc::c_char,
                size: 512 as libc::c_int as libc::c_uint,
                init: Some(
                    bench_gostdsa_init
                        as unsafe extern "C" fn(libc::c_uint) -> *mut libc::c_void,
                ),
                sign: Some(
                    bench_gostdsa_sign as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                verify: Some(
                    bench_gostdsa_verify as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                clear: Some(
                    bench_gostdsa_clear as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            };
            init
        },
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut filter: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    if argc > 1 as libc::c_int {
        filter = *argv.offset(1 as libc::c_int as isize);
    }
    time_init();
    printf(
        b"%16s %4s %9s %9s\n\0" as *const u8 as *const libc::c_char,
        b"name\0" as *const u8 as *const libc::c_char,
        b"size\0" as *const u8 as *const libc::c_char,
        b"sign/ms\0" as *const u8 as *const libc::c_char,
        b"verify/ms\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[alg; 25]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<alg>() as libc::c_ulong)
    {
        if filter.is_null() || !(strstr(alg_list[i as usize].name, filter)).is_null() {
            bench_alg(&mut *alg_list.as_mut_ptr().offset(i as isize));
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
