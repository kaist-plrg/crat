use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __gmp_randseed(_: *mut __gmp_randstate_struct, _: mpz_srcptr);
    fn __gmpz_add(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_add_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_cmp_ui(_: mpz_srcptr, _: libc::c_ulong) -> libc::c_int;
    fn __gmpz_combit(_: mpz_ptr, _: mp_bitcnt_t);
    fn __gmpz_congruent_p(_: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_fdiv_r(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_init_set_str(
        _: mpz_ptr,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn __gmpz_init_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_mod(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_mul_2exp(_: mpz_ptr, _: mpz_srcptr, _: mp_bitcnt_t);
    fn __gmpz_mul_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_out_str(_: *mut FILE, _: libc::c_int, _: mpz_srcptr) -> size_t;
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_probab_prime_p(_: mpz_srcptr, _: libc::c_int) -> libc::c_int;
    fn __gmpz_set(_: mpz_ptr, _: mpz_srcptr);
    fn __gmpz_set_str(_: mpz_ptr, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn __gmpz_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn __gmpz_sub(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_sub_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_submul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_roinit_n(_: mpz_ptr, _: mp_srcptr, _: mp_size_t) -> mpz_srcptr;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn abort() -> !;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
    fn nettle_sha512_update(ctx: *mut sha512_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md5_init(ctx: *mut md5_ctx);
    fn nettle_md5_update(ctx: *mut md5_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md5_digest(ctx: *mut md5_ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_sha1_init(ctx: *mut sha1_ctx);
    fn nettle_sha1_update(ctx: *mut sha1_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha1_digest(ctx: *mut sha1_ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_sha256_init(ctx: *mut sha256_ctx);
    fn nettle_sha256_update(ctx: *mut sha256_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha256_digest(ctx: *mut sha256_ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_sha512_init(ctx: *mut sha512_ctx);
    fn nettle_rsa_public_key_prepare(key: *mut rsa_public_key) -> libc::c_int;
    fn nettle_rsa_private_key_prepare(key: *mut rsa_private_key) -> libc::c_int;
    fn nettle_rsa_md5_sign(
        key: *const rsa_private_key,
        hash: *mut md5_ctx,
        signature: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_md5_sign_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        hash: *mut md5_ctx,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_md5_verify(
        key: *const rsa_public_key,
        hash: *mut md5_ctx,
        signature: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha1_sign(
        key: *const rsa_private_key,
        hash: *mut sha1_ctx,
        signature: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha1_sign_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        hash: *mut sha1_ctx,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha1_verify(
        key: *const rsa_public_key,
        hash: *mut sha1_ctx,
        signature: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha256_sign(
        key: *const rsa_private_key,
        hash: *mut sha256_ctx,
        signature: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha256_sign_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        hash: *mut sha256_ctx,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha256_verify(
        key: *const rsa_public_key,
        hash: *mut sha256_ctx,
        signature: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha512_sign(
        key: *const rsa_private_key,
        hash: *mut sha512_ctx,
        signature: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha512_sign_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        hash: *mut sha512_ctx,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha512_verify(
        key: *const rsa_public_key,
        hash: *mut sha512_ctx,
        signature: *const __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_md5_sign_digest(
        key: *const rsa_private_key,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_md5_sign_digest_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha1_sign_digest(
        key: *const rsa_private_key,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha1_sign_digest_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
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
    fn nettle_rsa_sha512_sign_digest(
        key: *const rsa_private_key,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_rsa_sha512_sign_digest_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        digest: *const uint8_t,
        s: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_dsa_signature_init(signature: *mut dsa_signature);
    fn nettle_dsa_signature_clear(signature: *mut dsa_signature);
    fn nettle_dsa_verify(
        params: *const dsa_params,
        y: *const __mpz_struct,
        digest_size: size_t,
        digest: *const uint8_t,
        signature: *const dsa_signature,
    ) -> libc::c_int;
    fn nettle_dsa_sha1_sign(
        pub_0: *const dsa_public_key,
        key: *const dsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        hash: *mut sha1_ctx,
        signature: *mut dsa_signature,
    ) -> libc::c_int;
    fn nettle_dsa_sha256_sign(
        pub_0: *const dsa_public_key,
        key: *const dsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        hash: *mut sha256_ctx,
        signature: *mut dsa_signature,
    ) -> libc::c_int;
    fn nettle_dsa_sha1_verify(
        key: *const dsa_public_key,
        hash: *mut sha1_ctx,
        signature: *const dsa_signature,
    ) -> libc::c_int;
    fn nettle_dsa_sha256_verify(
        key: *const dsa_public_key,
        hash: *mut sha256_ctx,
        signature: *const dsa_signature,
    ) -> libc::c_int;
    fn nettle_sha512_digest(ctx: *mut sha512_ctx, length: size_t, digest: *mut uint8_t);
    fn _nettle_mpz_limbs_copy(xp: *mut mp_limb_t, x: mpz_srcptr, n: mp_size_t);
    static _nettle_secp_192r1: ecc_curve;
    static _nettle_secp_224r1: ecc_curve;
    static _nettle_secp_256r1: ecc_curve;
    static _nettle_secp_384r1: ecc_curve;
    static _nettle_secp_521r1: ecc_curve;
    static _nettle_curve25519: ecc_curve;
    static _nettle_curve448: ecc_curve;
    static _nettle_gost_gc256b: ecc_curve;
    static _nettle_gost_gc512a: ecc_curve;
    fn test_main();
    fn nettle_ecc_size_a(ecc: *const ecc_curve) -> mp_size_t;
    fn nettle_base16_decode_init(ctx: *mut base16_decode_ctx);
    fn nettle_base16_decode_update(
        ctx: *mut base16_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_base16_decode_final(ctx: *mut base16_decode_ctx) -> libc::c_int;
    fn nettle_cbc_encrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cbc_decrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb_encrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb_decrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb8_encrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb8_decrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ctr_crypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_knuth_lfib_init(ctx: *mut knuth_lfib_ctx, seed: uint32_t);
    fn nettle_knuth_lfib_random(ctx: *mut knuth_lfib_ctx, n: size_t, dst: *mut uint8_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
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
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type uint64_t = __uint64_t;
pub type nettle_random_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_crypt_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
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
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const libc::c_char,
) -> libc::c_int;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
) -> libc::c_int;
pub type mp_limb_t = libc::c_ulong;
pub type mp_bitcnt_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type gmp_randalg_t = libc::c_uint;
pub const GMP_RAND_ALG_LC: gmp_randalg_t = 0;
pub const GMP_RAND_ALG_DEFAULT: gmp_randalg_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __gmp_randstate_struct {
    pub _mp_seed: mpz_t,
    pub _mp_alg: gmp_randalg_t,
    pub _mp_algdata: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _mp_lc: *mut libc::c_void,
}
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
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
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
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
pub struct dsa_public_key {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
    pub y: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_private_key {
    pub x: mpz_t,
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
pub struct nettle_cipher {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub encrypt: Option::<nettle_cipher_func>,
    pub decrypt: Option::<nettle_cipher_func>,
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
pub struct nettle_mac {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub set_key: Option::<nettle_set_key_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_aead {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub nonce_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub set_nonce: Option::<nettle_set_key_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub encrypt: Option::<nettle_crypt_func>,
    pub decrypt: Option::<nettle_crypt_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const libc::c_char,
    pub encode_context_size: libc::c_uint,
    pub decode_context_size: libc::c_uint,
    pub encode_final_length: libc::c_uint,
    pub encode_init: Option::<nettle_armor_init_func>,
    pub encode_length: Option::<nettle_armor_length_func>,
    pub encode_update: Option::<nettle_armor_encode_update_func>,
    pub encode_final: Option::<nettle_armor_encode_final_func>,
    pub decode_init: Option::<nettle_armor_init_func>,
    pub decode_length: Option::<nettle_armor_length_func>,
    pub decode_update: Option::<nettle_armor_decode_update_func>,
    pub decode_final: Option::<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tstring {
    pub next: *mut tstring,
    pub length: size_t,
    pub data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: libc::c_uchar,
    pub bits: libc::c_uchar,
}
pub type nettle_encrypt_message_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
    size_t,
    *const uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_decrypt_message_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
    size_t,
    *const uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_aead_message {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub supports_inplace: libc::c_int,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub encrypt: Option::<nettle_encrypt_message_func>,
    pub decrypt: Option::<nettle_decrypt_message_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct knuth_lfib_ctx {
    pub x: [uint32_t; 100],
    pub index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_ref_point {
    pub x: *const libc::c_char,
    pub y: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn __gmpn_cmp(
    mut __gmp_xp: mp_srcptr,
    mut __gmp_yp: mp_srcptr,
    mut __gmp_size: mp_size_t,
) -> libc::c_int {
    let mut __gmp_result: libc::c_int = 0;
    let mut __gmp_i: mp_size_t = 0;
    let mut __gmp_x: mp_limb_t = 0;
    let mut __gmp_y: mp_limb_t = 0;
    __gmp_result = 0 as libc::c_int;
    __gmp_i = __gmp_size;
    loop {
        __gmp_i -= 1;
        if !(__gmp_i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        __gmp_x = *__gmp_xp.offset(__gmp_i as isize);
        __gmp_y = *__gmp_yp.offset(__gmp_i as isize);
        if !(__gmp_x != __gmp_y) {
            continue;
        }
        __gmp_result = if __gmp_x > __gmp_y {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        break;
    }
    return __gmp_result;
}
#[inline]
unsafe extern "C" fn __gmpn_zero_p(
    mut __gmp_p: mp_srcptr,
    mut __gmp_n: mp_size_t,
) -> libc::c_int {
    loop {
        __gmp_n -= 1;
        if *__gmp_p.offset(__gmp_n as isize) != 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if !(__gmp_n != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn die(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    abort();
}
pub unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if size != 0 && p.is_null() {
        fprintf(
            stderr,
            b"Virtual memory exhausted.\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return p;
}
static mut tstring_first: *mut tstring = 0 as *const tstring as *mut tstring;
pub unsafe extern "C" fn tstring_alloc(mut length: size_t) -> *mut tstring {
    let mut s: *mut tstring = xalloc(
        (::std::mem::size_of::<tstring>() as libc::c_ulong).wrapping_add(length),
    ) as *mut tstring;
    (*s).length = length;
    (*s).next = tstring_first;
    *((*s).data).as_mut_ptr().offset(length as isize) = '\0' as i32 as uint8_t;
    tstring_first = s;
    return s;
}
pub unsafe extern "C" fn tstring_clear() {
    while !tstring_first.is_null() {
        let mut s: *mut tstring = tstring_first;
        tstring_first = (*s).next;
        free(s as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn tstring_data(
    mut length: size_t,
    mut data: *const uint8_t,
) -> *mut tstring {
    let mut s: *mut tstring = tstring_alloc(length);
    memcpy(
        ((*s).data).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length,
    );
    return s;
}
pub unsafe extern "C" fn tstring_hex(mut hex: *const libc::c_char) -> *mut tstring {
    let mut ctx: base16_decode_ctx = base16_decode_ctx {
        word: 0,
        bits: 0,
    };
    let mut s: *mut tstring = 0 as *mut tstring;
    let mut length: size_t = strlen(hex);
    s = tstring_alloc(
        length
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong),
    );
    nettle_base16_decode_init(&mut ctx);
    if nettle_base16_decode_update(
        &mut ctx,
        &mut (*s).length,
        ((*s).data).as_mut_ptr(),
        length,
        hex,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            b"base16_decode_update (&ctx, &s->length, s->data, length, hex)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if nettle_base16_decode_final(&mut ctx) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            b"base16_decode_final (&ctx)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return s;
}
pub unsafe extern "C" fn tstring_print_hex(mut s: *const tstring) {
    print_hex((*s).length, ((*s).data).as_ptr());
}
pub unsafe extern "C" fn print_hex(mut length: size_t, mut data: *const uint8_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        match i.wrapping_rem(16 as libc::c_int as libc::c_ulong) {
            0 => {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                printf(b" \0" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            *data.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub static mut verbose: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc > 1 as libc::c_int {
        if argc == 2 as libc::c_int
            && strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-v\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            verbose = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"Invalid argument `%s', only accepted option is `-v'.\n\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(1 as libc::c_int as isize),
            );
            return 1 as libc::c_int;
        }
    }
    test_main();
    tstring_clear();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn test_cipher(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = xalloc((*cleartext).length) as *mut uint8_t;
    let mut length: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            b"cleartext->length == ciphertext->length\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            b"key->length == cipher->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    ((*cipher).encrypt).unwrap()(ctx, length, data, ((*cleartext).data).as_ptr());
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"Encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_decrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    ((*cipher).decrypt).unwrap()(ctx, length, data, data);
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"Decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
}
pub unsafe extern "C" fn test_cipher_cbc(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut iiv: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut iv: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            b"cleartext->length == ciphertext->length\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            b"key->length == cipher->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*iiv).length == (*cipher).block_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int,
            b"iiv->length == cipher->block_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    data = xalloc(length) as *mut uint8_t;
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cbc_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CBC encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_decrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cbc_decrypt(
        ctx,
        (*cipher).decrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CBC decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(iv as *mut libc::c_void);
}
pub unsafe extern "C" fn test_cipher_cfb(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut iiv: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut data2: *mut uint8_t = 0 as *mut uint8_t;
    let mut iv: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            b"cleartext->length == ciphertext->length\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int,
            b"key->length == cipher->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*iiv).length == (*cipher).block_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int,
            b"iiv->length == cipher->block_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    data = xalloc(length) as *mut uint8_t;
    data2 = xalloc(length) as *mut uint8_t;
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data2,
        data,
    );
    if memcmp(
        data2 as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data2);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    length = (length as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data2,
        data,
    );
    if memcmp(
        data2 as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data2);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(data2 as *mut libc::c_void);
    free(iv as *mut libc::c_void);
}
pub unsafe extern "C" fn test_cipher_cfb8(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut iiv: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut data2: *mut uint8_t = 0 as *mut uint8_t;
    let mut iv: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    let mut block: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            441 as libc::c_int,
            b"cleartext->length == ciphertext->length\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            444 as libc::c_int,
            b"key->length == cipher->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*iiv).length == (*cipher).block_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            445 as libc::c_int,
            b"iiv->length == cipher->block_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    data = xalloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut uint8_t;
    data2 = xalloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut uint8_t;
    block = 1 as libc::c_int as size_t;
    while block <= length {
        let mut i: size_t = 0;
        ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
        memcpy(
            iv as *mut libc::c_void,
            ((*iiv).data).as_ptr() as *const libc::c_void,
            (*cipher).block_size as libc::c_ulong,
        );
        memset(
            data as *mut libc::c_void,
            0x17 as libc::c_int,
            length.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        i = 0 as libc::c_int as size_t;
        while i.wrapping_add(block) <= length {
            nettle_cfb8_encrypt(
                ctx,
                (*cipher).encrypt,
                (*cipher).block_size as size_t,
                iv,
                block,
                data.offset(i as isize),
                ((*cleartext).data).as_ptr().offset(i as isize),
            );
            i = (i as libc::c_ulong).wrapping_add(block) as size_t as size_t;
        }
        nettle_cfb8_encrypt(
            ctx,
            (*cipher).encrypt,
            (*cipher).block_size as size_t,
            iv,
            length.wrapping_sub(i),
            data.offset(i as isize),
            ((*cleartext).data).as_ptr().offset(i as isize),
        );
        if memcmp(
            data as *const libc::c_void,
            ((*ciphertext).data).as_ptr() as *const libc::c_void,
            length,
        ) != 0
        {
            fprintf(
                stderr,
                b"CFB8 encrypt failed, block size %lu:\nInput:\0" as *const u8
                    as *const libc::c_char,
                block,
            );
            tstring_print_hex(cleartext);
            fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
            print_hex(length, data);
            fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
            tstring_print_hex(ciphertext);
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
        if !(*data.offset(length as isize) as libc::c_int == 0x17 as libc::c_int) {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                480 as libc::c_int,
                b"data[length] == 0x17\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
        ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
        memcpy(
            iv as *mut libc::c_void,
            ((*iiv).data).as_ptr() as *const libc::c_void,
            (*cipher).block_size as libc::c_ulong,
        );
        memset(
            data2 as *mut libc::c_void,
            0x17 as libc::c_int,
            length.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        i = 0 as libc::c_int as size_t;
        while i.wrapping_add(block) <= length {
            nettle_cfb8_decrypt(
                ctx,
                (*cipher).encrypt,
                (*cipher).block_size as size_t,
                iv,
                block,
                data2.offset(i as isize),
                data.offset(i as isize),
            );
            i = (i as libc::c_ulong).wrapping_add(block) as size_t as size_t;
        }
        nettle_cfb8_decrypt(
            ctx,
            (*cipher).encrypt,
            (*cipher).block_size as size_t,
            iv,
            length.wrapping_sub(i),
            data2.offset(i as isize),
            data.offset(i as isize),
        );
        if memcmp(
            data2 as *const libc::c_void,
            ((*cleartext).data).as_ptr() as *const libc::c_void,
            length,
        ) != 0
        {
            fprintf(
                stderr,
                b"CFB8 decrypt failed, block size %lu:\nInput:\0" as *const u8
                    as *const libc::c_char,
                block,
            );
            tstring_print_hex(ciphertext);
            fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
            print_hex(length, data2);
            fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
            tstring_print_hex(cleartext);
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
        if !(*data.offset(length as isize) as libc::c_int == 0x17 as libc::c_int) {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                508 as libc::c_int,
                b"data[length] == 0x17\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
        block = block.wrapping_add(1);
        block;
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb8_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb8_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    length = (length as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb8_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb8_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data2,
        data,
    );
    if memcmp(
        data2 as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data2);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb8_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace encrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_cfb8_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(data2 as *mut libc::c_void);
    free(iv as *mut libc::c_void);
}
pub unsafe extern "C" fn test_cipher_ctr(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut ictr: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut ctr: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut octr: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    let mut nblocks: size_t = 0;
    let mut low: libc::c_uint = 0;
    let mut i: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            647 as libc::c_int,
            b"cleartext->length == ciphertext->length\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            650 as libc::c_int,
            b"key->length == cipher->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*ictr).length == (*cipher).block_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            651 as libc::c_int,
            b"ictr->length == cipher->block_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nblocks = length
        .wrapping_add((*cipher).block_size as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div((*cipher).block_size as libc::c_ulong);
    if !(nblocks < 0x100 as libc::c_int as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            655 as libc::c_int,
            b"nblocks < 0x100\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    memcpy(
        octr as *mut libc::c_void,
        ((*ictr).data).as_ptr() as *const libc::c_void,
        ((*cipher).block_size).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as libc::c_ulong,
    );
    low = (*((*ictr).data)
        .as_ptr()
        .offset(
            ((*cipher).block_size).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as isize,
        ) as libc::c_ulong)
        .wrapping_add(nblocks) as libc::c_uint;
    *octr
        .offset(
            ((*cipher).block_size).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as isize,
        ) = low as uint8_t;
    if low >= 0x100 as libc::c_int as libc::c_uint {
        let mut increment_i: libc::c_uint = ((*cipher).block_size)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        let ref mut fresh0 = *octr.offset(increment_i as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        if *fresh0 as libc::c_int == 0 as libc::c_int {
            while increment_i > 0 as libc::c_int as libc::c_uint
                && {
                    increment_i = increment_i.wrapping_sub(1);
                    let ref mut fresh1 = *octr.offset(increment_i as isize);
                    *fresh1 = (*fresh1).wrapping_add(1);
                    *fresh1 as libc::c_int == 0 as libc::c_int
                }
            {}
        }
    }
    data = xalloc(length) as *mut uint8_t;
    ((*cipher).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    i = 0 as libc::c_int as size_t;
    while i <= length {
        memcpy(
            ctr as *mut libc::c_void,
            ((*ictr).data).as_ptr() as *const libc::c_void,
            (*cipher).block_size as libc::c_ulong,
        );
        memset(data as *mut libc::c_void, 17 as libc::c_int, length);
        nettle_ctr_crypt(
            ctx,
            (*cipher).encrypt,
            (*cipher).block_size as size_t,
            ctr,
            i,
            data,
            ((*cleartext).data).as_ptr(),
        );
        if memcmp(
            data as *const libc::c_void,
            ((*ciphertext).data).as_ptr() as *const libc::c_void,
            i,
        ) != 0
            || i < length && *data.offset(i as isize) as libc::c_int != 17 as libc::c_int
        {
            fprintf(
                stderr,
                b"CTR encrypt failed (length %d of %d):\nInput:\0" as *const u8
                    as *const libc::c_char,
                i as libc::c_int,
                length as libc::c_int,
            );
            tstring_print_hex(cleartext);
            fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
            print_hex(length, data);
            fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
            tstring_print_hex(ciphertext);
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
        i = i.wrapping_add(1);
        i;
    }
    if memcmp(
        ctr as *const libc::c_void,
        octr as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            692 as libc::c_int,
            b"MEMEQ (cipher->block_size, ctr, octr)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    memcpy(
        ctr as *mut libc::c_void,
        ((*ictr).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    );
    nettle_ctr_crypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        ctr,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CTR decrypt failed:\nInput:\0" as *const u8 as *const libc::c_char,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const libc::c_char);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    if memcmp(
        ctr as *const libc::c_void,
        octr as *const libc::c_void,
        (*cipher).block_size as libc::c_ulong,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            712 as libc::c_int,
            b"MEMEQ (cipher->block_size, ctr, octr)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(octr as *mut libc::c_void);
    free(ctr as *mut libc::c_void);
}
pub unsafe extern "C" fn test_aead(
    mut aead: *const nettle_aead,
    mut set_nonce: Option::<nettle_hash_update_func>,
    mut key: *const tstring,
    mut authtext: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut nonce: *const tstring,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*aead).context_size as size_t);
    let mut in_0: *mut uint8_t = 0 as *mut uint8_t;
    let mut out: *mut uint8_t = 0 as *mut uint8_t;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut in_align: libc::c_uint = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            805 as libc::c_int,
            b"cleartext->length == ciphertext->length\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !((*key).length == (*aead).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            806 as libc::c_int,
            b"key->length == aead->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*aead).block_size > 0 as libc::c_int as libc::c_uint) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            807 as libc::c_int,
            b"aead->block_size > 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    buffer = xalloc((*aead).digest_size as size_t) as *mut uint8_t;
    in_0 = xalloc(
        ((*cleartext).length)
            .wrapping_add((*aead).block_size as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut uint8_t;
    out = xalloc(
        ((*cleartext).length)
            .wrapping_add((*aead).block_size as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut uint8_t;
    in_align = 0 as libc::c_int as libc::c_uint;
    while in_align < (*aead).block_size {
        let mut out_align: libc::c_uint = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(in_align)
            .wrapping_rem((*aead).block_size);
        let mut offset: size_t = 0;
        memcpy(
            in_0.offset(in_align as isize) as *mut libc::c_void,
            ((*cleartext).data).as_ptr() as *const libc::c_void,
            (*cleartext).length,
        );
        offset = 0 as libc::c_int as size_t;
        while offset <= (*cleartext).length {
            ((*aead).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
            if set_nonce.is_some() {
                set_nonce.unwrap()(ctx, (*nonce).length, ((*nonce).data).as_ptr());
            } else {
                if (*nonce).length == (*aead).nonce_size as libc::c_ulong {} else {
                    __assert_fail(
                        b"nonce->length == aead->nonce_size\0" as *const u8
                            as *const libc::c_char,
                        b"testutils.c\0" as *const u8 as *const libc::c_char,
                        828 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 214],
                            &[libc::c_char; 214],
                        >(
                            b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_12450: {
                    if (*nonce).length == (*aead).nonce_size as libc::c_ulong {} else {
                        __assert_fail(
                            b"nonce->length == aead->nonce_size\0" as *const u8
                                as *const libc::c_char,
                            b"testutils.c\0" as *const u8 as *const libc::c_char,
                            828 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 214],
                                &[libc::c_char; 214],
                            >(
                                b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                ((*aead).set_nonce).unwrap()(ctx, ((*nonce).data).as_ptr());
            }
            if ((*aead).update).is_some() && (*authtext).length != 0 {
                ((*aead).update)
                    .unwrap()(ctx, (*authtext).length, ((*authtext).data).as_ptr());
            }
            if offset > 0 as libc::c_int as libc::c_ulong {
                ((*aead).encrypt)
                    .unwrap()(
                    ctx,
                    offset,
                    out.offset(out_align as isize),
                    in_0.offset(in_align as isize),
                );
            }
            if offset < (*cleartext).length {
                ((*aead).encrypt)
                    .unwrap()(
                    ctx,
                    ((*cleartext).length).wrapping_sub(offset),
                    out.offset(out_align as isize).offset(offset as isize),
                    in_0.offset(in_align as isize).offset(offset as isize),
                );
            }
            if memcmp(
                out.offset(out_align as isize) as *const libc::c_void,
                ((*ciphertext).data).as_ptr() as *const libc::c_void,
                (*cleartext).length,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"aead->encrypt failed (offset = %u):\nclear: \0" as *const u8
                        as *const libc::c_char,
                    offset as libc::c_uint,
                );
                tstring_print_hex(cleartext);
                fprintf(stderr, b"  got: \0" as *const u8 as *const libc::c_char);
                print_hex((*cleartext).length, out.offset(out_align as isize));
                fprintf(stderr, b"  exp: \0" as *const u8 as *const libc::c_char);
                tstring_print_hex(ciphertext);
                abort();
            }
            if !digest.is_null() {
                if !((*digest).length <= (*aead).digest_size as libc::c_ulong) {
                    fprintf(
                        stderr,
                        b"Assert failed: %s:%d: %s\n\0" as *const u8
                            as *const libc::c_char,
                        b"testutils.c\0" as *const u8 as *const libc::c_char,
                        854 as libc::c_int,
                        b"digest->length <= aead->digest_size\0" as *const u8
                            as *const libc::c_char,
                    );
                    abort();
                }
                memset(
                    buffer as *mut libc::c_void,
                    0 as libc::c_int,
                    (*aead).digest_size as libc::c_ulong,
                );
                ((*aead).digest).unwrap()(ctx, (*digest).length, buffer);
                if memcmp(
                    buffer as *const libc::c_void,
                    ((*digest).data).as_ptr() as *const libc::c_void,
                    (*digest).length,
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"aead->digest failed (offset = %u):\n  got: \0" as *const u8
                            as *const libc::c_char,
                        offset as libc::c_uint,
                    );
                    print_hex((*digest).length, buffer);
                    fprintf(stderr, b"  exp: \0" as *const u8 as *const libc::c_char);
                    tstring_print_hex(digest);
                    abort();
                }
            } else if ((*aead).digest).is_some() {
                fprintf(
                    stderr,
                    b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                    b"testutils.c\0" as *const u8 as *const libc::c_char,
                    868 as libc::c_int,
                    b"!aead->digest\0" as *const u8 as *const libc::c_char,
                );
                abort();
            }
            if ((*aead).set_decrypt_key).is_some() {
                ((*aead).set_decrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
                if set_nonce.is_some() {
                    set_nonce.unwrap()(ctx, (*nonce).length, ((*nonce).data).as_ptr());
                } else {
                    if (*nonce).length == (*aead).nonce_size as libc::c_ulong {} else {
                        __assert_fail(
                            b"nonce->length == aead->nonce_size\0" as *const u8
                                as *const libc::c_char,
                            b"testutils.c\0" as *const u8 as *const libc::c_char,
                            879 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 214],
                                &[libc::c_char; 214],
                            >(
                                b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_12018: {
                        if (*nonce).length == (*aead).nonce_size as libc::c_ulong
                        {} else {
                            __assert_fail(
                                b"nonce->length == aead->nonce_size\0" as *const u8
                                    as *const libc::c_char,
                                b"testutils.c\0" as *const u8 as *const libc::c_char,
                                879 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 214],
                                    &[libc::c_char; 214],
                                >(
                                    b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    ((*aead).set_nonce).unwrap()(ctx, ((*nonce).data).as_ptr());
                }
                if ((*aead).update).is_some() && (*authtext).length != 0 {
                    ((*aead).update)
                        .unwrap()(ctx, (*authtext).length, ((*authtext).data).as_ptr());
                }
                if offset > 0 as libc::c_int as libc::c_ulong {
                    ((*aead).decrypt)
                        .unwrap()(
                        ctx,
                        offset,
                        out.offset(out_align as isize),
                        out.offset(out_align as isize),
                    );
                }
                if offset < (*cleartext).length {
                    ((*aead).decrypt)
                        .unwrap()(
                        ctx,
                        ((*cleartext).length).wrapping_sub(offset),
                        out.offset(out_align as isize).offset(offset as isize),
                        out.offset(out_align as isize).offset(offset as isize),
                    );
                }
                if memcmp(
                    out.offset(out_align as isize) as *const libc::c_void,
                    ((*cleartext).data).as_ptr() as *const libc::c_void,
                    (*cleartext).length,
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"Assert failed: %s:%d: %s\n\0" as *const u8
                            as *const libc::c_char,
                        b"testutils.c\0" as *const u8 as *const libc::c_char,
                        893 as libc::c_int,
                        b"MEMEQ(cleartext->length, out + out_align, cleartext->data)\0"
                            as *const u8 as *const libc::c_char,
                    );
                    abort();
                }
                if !digest.is_null() {
                    memset(
                        buffer as *mut libc::c_void,
                        0 as libc::c_int,
                        (*aead).digest_size as libc::c_ulong,
                    );
                    ((*aead).digest).unwrap()(ctx, (*digest).length, buffer);
                    if memcmp(
                        buffer as *const libc::c_void,
                        ((*digest).data).as_ptr() as *const libc::c_void,
                        (*digest).length,
                    ) != 0
                    {
                        fprintf(
                            stderr,
                            b"Assert failed: %s:%d: %s\n\0" as *const u8
                                as *const libc::c_char,
                            b"testutils.c\0" as *const u8 as *const libc::c_char,
                            899 as libc::c_int,
                            b"MEMEQ(digest->length, buffer, digest->data)\0" as *const u8
                                as *const libc::c_char,
                        );
                        abort();
                    }
                }
            }
            offset = (offset as libc::c_ulong)
                .wrapping_add((*aead).block_size as libc::c_ulong) as size_t as size_t;
        }
        in_align = in_align.wrapping_add(1);
        in_align;
    }
    free(ctx);
    free(in_0 as *mut libc::c_void);
    free(out as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
pub unsafe extern "C" fn test_aead_message(
    mut aead: *const nettle_aead_message,
    mut key: *const tstring,
    mut nonce: *const tstring,
    mut adata: *const tstring,
    mut clear: *const tstring,
    mut cipher: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*aead).context_size as size_t);
    let mut buf: *mut uint8_t = xalloc(
        ((*cipher).length).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut uint8_t;
    let mut copy: *mut uint8_t = xalloc((*cipher).length) as *mut uint8_t;
    static mut nul: uint8_t = 0 as libc::c_int as uint8_t;
    let mut res: libc::c_int = 0;
    if !((*key).length == (*aead).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            925 as libc::c_int,
            b"key->length == aead->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*cipher).length > (*clear).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            926 as libc::c_int,
            b"cipher->length > clear->length\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(((*cipher).length).wrapping_sub((*clear).length)
        == (*aead).digest_size as libc::c_ulong)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            927 as libc::c_int,
            b"cipher->length - clear->length == aead->digest_size\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    ((*aead).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    *buf.offset((*cipher).length as isize) = 0xae as libc::c_int as uint8_t;
    ((*aead).encrypt)
        .unwrap()(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*cipher).length,
        buf,
        ((*clear).data).as_ptr(),
    );
    if memcmp(
        ((*cipher).data).as_ptr() as *const libc::c_void,
        buf as *const libc::c_void,
        (*cipher).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"aead->encrypt (message) failed:\n  got: \0" as *const u8
                as *const libc::c_char,
        );
        print_hex((*cipher).length, buf);
        fprintf(stderr, b"  exp: \0" as *const u8 as *const libc::c_char);
        tstring_print_hex(cipher);
        abort();
    }
    if *buf.offset((*cipher).length as isize) as libc::c_int != 0xae as libc::c_int {
        fprintf(
            stderr,
            b"aead->encrypt (message) wrote too much.\n \0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    ((*aead).set_decrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
    memset(
        buf as *mut libc::c_void,
        0xae as libc::c_int,
        ((*clear).length).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    res = ((*aead).decrypt)
        .unwrap()(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*clear).length,
        buf,
        ((*cipher).data).as_ptr(),
    );
    if res == 0 {
        fprintf(
            stderr,
            b"decrypting valid ciphertext failed:\n  \0" as *const u8
                as *const libc::c_char,
        );
        tstring_print_hex(cipher);
    }
    if memcmp(
        ((*clear).data).as_ptr() as *const libc::c_void,
        buf as *const libc::c_void,
        (*clear).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"aead->decrypt (message) failed:\n  got: \0" as *const u8
                as *const libc::c_char,
        );
        print_hex((*clear).length, buf);
        fprintf(stderr, b"  exp: \0" as *const u8 as *const libc::c_char);
        tstring_print_hex(clear);
        abort();
    }
    if (*clear).length > 0 as libc::c_int as libc::c_ulong
        && ((*aead).decrypt)
            .unwrap()(
            ctx,
            (*nonce).length,
            ((*nonce).data).as_ptr(),
            (*adata).length,
            ((*adata).data).as_ptr(),
            ((*clear).length).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            buf,
            ((*cipher).data).as_ptr(),
        ) != 0
    {
        fprintf(
            stderr,
            b"Invalid message (truncated) not rejected\n\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    memcpy(
        copy as *mut libc::c_void,
        ((*cipher).data).as_ptr() as *const libc::c_void,
        (*cipher).length,
    );
    let ref mut fresh2 = *copy.offset(0 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_int ^ 4 as libc::c_int) as uint8_t;
    if ((*aead).decrypt)
        .unwrap()(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*clear).length,
        buf,
        copy,
    ) != 0
    {
        fprintf(
            stderr,
            b"Invalid message (first byte modified) not rejected\n\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    memcpy(
        copy as *mut libc::c_void,
        ((*cipher).data).as_ptr() as *const libc::c_void,
        (*cipher).length,
    );
    let ref mut fresh3 = *copy
        .offset(
            ((*cipher).length).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    *fresh3 = (*fresh3 as libc::c_int ^ 4 as libc::c_int) as uint8_t;
    if ((*aead).decrypt)
        .unwrap()(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*clear).length,
        buf,
        copy,
    ) != 0
    {
        fprintf(
            stderr,
            b"Invalid message (last byte modified) not rejected\n\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if ((*aead).decrypt)
        .unwrap()(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        if (*adata).length > 0 as libc::c_int as libc::c_ulong {
            ((*adata).length).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            1 as libc::c_int as libc::c_ulong
        },
        if (*adata).length > 0 as libc::c_int as libc::c_ulong {
            ((*adata).data).as_ptr()
        } else {
            &nul
        },
        (*clear).length,
        buf,
        ((*cipher).data).as_ptr(),
    ) != 0
    {
        fprintf(
            stderr,
            b"Invalid adata not rejected\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*aead).supports_inplace != 0 {
        ((*aead).set_encrypt_key).unwrap()(ctx, ((*key).data).as_ptr());
        *buf.offset((*cipher).length as isize) = 0xae as libc::c_int as uint8_t;
        memcpy(
            buf as *mut libc::c_void,
            ((*clear).data).as_ptr() as *const libc::c_void,
            (*clear).length,
        );
        ((*aead).encrypt)
            .unwrap()(
            ctx,
            (*nonce).length,
            ((*nonce).data).as_ptr(),
            (*adata).length,
            ((*adata).data).as_ptr(),
            (*cipher).length,
            buf,
            buf,
        );
        if memcmp(
            ((*cipher).data).as_ptr() as *const libc::c_void,
            buf as *const libc::c_void,
            (*cipher).length,
        ) != 0
        {
            fprintf(
                stderr,
                b"aead->encrypt (in-place message) failed:\n  got: \0" as *const u8
                    as *const libc::c_char,
            );
            print_hex((*cipher).length, buf);
            fprintf(stderr, b"  exp: \0" as *const u8 as *const libc::c_char);
            tstring_print_hex(cipher);
            abort();
        }
        if *buf.offset((*cipher).length as isize) as libc::c_int != 0xae as libc::c_int {
            fprintf(
                stderr,
                b"aead->encrypt (in-place message) wrote too much.\n \0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        res = ((*aead).decrypt)
            .unwrap()(
            ctx,
            (*nonce).length,
            ((*nonce).data).as_ptr(),
            (*adata).length,
            ((*adata).data).as_ptr(),
            (*clear).length,
            buf,
            buf,
        );
        if res == 0 {
            fprintf(
                stderr,
                b"in-place decrypting valid ciphertext failed:\n  \0" as *const u8
                    as *const libc::c_char,
            );
            tstring_print_hex(cipher);
        }
        if memcmp(
            ((*clear).data).as_ptr() as *const libc::c_void,
            buf as *const libc::c_void,
            (*clear).length,
        ) != 0
        {
            fprintf(
                stderr,
                b"aead->decrypt (in-place message) failed:\n  got: \0" as *const u8
                    as *const libc::c_char,
            );
            print_hex((*clear).length, buf);
            fprintf(stderr, b"  exp: \0" as *const u8 as *const libc::c_char);
            tstring_print_hex(clear);
            abort();
        }
    }
    free(ctx);
    free(buf as *mut libc::c_void);
    free(copy as *mut libc::c_void);
}
pub unsafe extern "C" fn test_hash(
    mut hash: *const nettle_hash,
    mut msg: *const tstring,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*hash).context_size as size_t);
    let mut buffer: *mut uint8_t = xalloc((*digest).length) as *mut uint8_t;
    let mut input: *mut uint8_t = 0 as *mut uint8_t;
    let mut offset: libc::c_uint = 0;
    if (*hash).digest_size != 0 {
        if !((*digest).length == (*hash).digest_size as libc::c_ulong) {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                1072 as libc::c_int,
                b"digest->length == hash->digest_size\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
    }
    ((*hash).init).unwrap()(ctx);
    ((*hash).update).unwrap()(ctx, (*msg).length, ((*msg).data).as_ptr());
    ((*hash).digest).unwrap()(ctx, (*digest).length, buffer);
    if (memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        buffer as *const libc::c_void,
        (*digest).length,
    ) == 0) as libc::c_int == 0 as libc::c_int
    {
        fprintf(stdout, b"\nGot:\n\0" as *const u8 as *const libc::c_char);
        print_hex((*digest).length, buffer);
        fprintf(stdout, b"\nExpected:\n\0" as *const u8 as *const libc::c_char);
        print_hex((*digest).length, ((*digest).data).as_ptr());
        abort();
    }
    memset(buffer as *mut libc::c_void, 0 as libc::c_int, (*digest).length);
    ((*hash).update).unwrap()(ctx, (*msg).length, ((*msg).data).as_ptr());
    if !((*digest).length > 0 as libc::c_int as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1090 as libc::c_int,
            b"digest->length > 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    ((*hash).digest)
        .unwrap()(
        ctx,
        ((*digest).length).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        buffer,
    );
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        buffer as *const libc::c_void,
        ((*digest).length).wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1093 as libc::c_int,
            b"MEMEQ(digest->length - 1, digest->data, buffer)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !(*buffer
        .offset(
            ((*digest).length).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1095 as libc::c_int,
            b"buffer[digest->length - 1] == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    input = xalloc(((*msg).length).wrapping_add(16 as libc::c_int as libc::c_ulong))
        as *mut uint8_t;
    offset = 0 as libc::c_int as libc::c_uint;
    while offset < 16 as libc::c_int as libc::c_uint {
        memset(
            input as *mut libc::c_void,
            0 as libc::c_int,
            ((*msg).length).wrapping_add(16 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            input.offset(offset as isize) as *mut libc::c_void,
            ((*msg).data).as_ptr() as *const libc::c_void,
            (*msg).length,
        );
        ((*hash).update).unwrap()(ctx, (*msg).length, input.offset(offset as isize));
        ((*hash).digest).unwrap()(ctx, (*digest).length, buffer);
        if (memcmp(
            ((*digest).data).as_ptr() as *const libc::c_void,
            buffer as *const libc::c_void,
            (*digest).length,
        ) == 0) as libc::c_int == 0 as libc::c_int
        {
            fprintf(
                stdout,
                b"hash input address: %p\nGot:\n\0" as *const u8 as *const libc::c_char,
                input.offset(offset as isize),
            );
            print_hex((*digest).length, buffer);
            fprintf(stdout, b"\nExpected:\n\0" as *const u8 as *const libc::c_char);
            print_hex((*digest).length, ((*digest).data).as_ptr());
            abort();
        }
        offset = offset.wrapping_add(1);
        offset;
    }
    free(ctx);
    free(buffer as *mut libc::c_void);
    free(input as *mut libc::c_void);
}
pub unsafe extern "C" fn test_hash_large(
    mut hash: *const nettle_hash,
    mut count: size_t,
    mut length: size_t,
    mut c: uint8_t,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*hash).context_size as size_t);
    let mut buffer: *mut uint8_t = xalloc((*hash).digest_size as size_t) as *mut uint8_t;
    let mut data: *mut uint8_t = xalloc(length) as *mut uint8_t;
    let mut i: size_t = 0;
    if !((*digest).length == (*hash).digest_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1129 as libc::c_int,
            b"digest->length == hash->digest_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    memset(data as *mut libc::c_void, c as libc::c_int, length);
    ((*hash).init).unwrap()(ctx);
    i = 0 as libc::c_int as size_t;
    while i < count {
        ((*hash).update).unwrap()(ctx, length, data);
        if i.wrapping_rem(count.wrapping_div(50 as libc::c_int as libc::c_ulong))
            == 0 as libc::c_int as libc::c_ulong
        {
            fprintf(stderr, b".\0" as *const u8 as *const libc::c_char);
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    ((*hash).digest).unwrap()(ctx, (*hash).digest_size as size_t, buffer);
    print_hex((*hash).digest_size as size_t, buffer);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        buffer as *const libc::c_void,
        (*hash).digest_size as libc::c_ulong,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1146 as libc::c_int,
            b"MEMEQ(hash->digest_size, digest->data, buffer)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    free(ctx);
    free(buffer as *mut libc::c_void);
    free(data as *mut libc::c_void);
}
pub unsafe extern "C" fn test_mac(
    mut mac: *const nettle_mac,
    mut key: *const tstring,
    mut msg: *const tstring,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*mac).context_size as size_t);
    let mut hash: *mut uint8_t = xalloc((*mac).digest_size as size_t) as *mut uint8_t;
    let mut i: libc::c_uint = 0;
    if !((*digest).length <= (*mac).digest_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1163 as libc::c_int,
            b"digest->length <= mac->digest_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*key).length == (*mac).key_size as libc::c_ulong) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1164 as libc::c_int,
            b"key->length == mac->key_size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    ((*mac).set_key).unwrap()(ctx, ((*key).data).as_ptr());
    ((*mac).update).unwrap()(ctx, (*msg).length, ((*msg).data).as_ptr());
    ((*mac).digest).unwrap()(ctx, (*digest).length, hash);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        hash as *const libc::c_void,
        (*digest).length,
    ) != 0
    {
        fprintf(stderr, b"test_mac failed, msg: \0" as *const u8 as *const libc::c_char);
        print_hex((*msg).length, ((*msg).data).as_ptr());
        fprintf(stderr, b"Output:\0" as *const u8 as *const libc::c_char);
        print_hex((*mac).digest_size as size_t, hash);
        fprintf(stderr, b"Expected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(digest);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*mac).update).unwrap()(ctx, (*msg).length, ((*msg).data).as_ptr());
    ((*mac).digest).unwrap()(ctx, (*digest).length, hash);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        hash as *const libc::c_void,
        (*digest).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"test_mac: failed on re-use, msg: \0" as *const u8 as *const libc::c_char,
        );
        print_hex((*msg).length, ((*msg).data).as_ptr());
        fprintf(stderr, b"Output:\0" as *const u8 as *const libc::c_char);
        print_hex((*mac).digest_size as size_t, hash);
        fprintf(stderr, b"Expected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(digest);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ((*mac).set_key).unwrap()(ctx, ((*key).data).as_ptr());
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*msg).length {
        ((*mac).update)
            .unwrap()(
            ctx,
            1 as libc::c_int as size_t,
            ((*msg).data).as_ptr().offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    ((*mac).digest).unwrap()(ctx, (*digest).length, hash);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        hash as *const libc::c_void,
        (*digest).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"cmac_hash failed on byte-by-byte, msg: \0" as *const u8
                as *const libc::c_char,
        );
        print_hex((*msg).length, ((*msg).data).as_ptr());
        fprintf(stderr, b"Output:\0" as *const u8 as *const libc::c_char);
        print_hex(16 as libc::c_int as size_t, hash);
        fprintf(stderr, b"Expected:\0" as *const u8 as *const libc::c_char);
        tstring_print_hex(digest);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    free(ctx);
    free(hash as *mut libc::c_void);
}
pub unsafe extern "C" fn test_armor(
    mut armor: *const nettle_armor,
    mut data_length: size_t,
    mut data: *const uint8_t,
    mut ascii: *const libc::c_char,
) {
    let mut ascii_length: size_t = strlen(ascii);
    let mut buffer: *mut libc::c_char = xalloc(
        (1 as libc::c_int as libc::c_ulong).wrapping_add(ascii_length),
    ) as *mut libc::c_char;
    let mut check: *mut uint8_t = xalloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(((*armor).decode_length).unwrap()(ascii_length)),
    ) as *mut uint8_t;
    let mut encode: *mut libc::c_void = xalloc((*armor).encode_context_size as size_t);
    let mut decode: *mut libc::c_void = xalloc((*armor).decode_context_size as size_t);
    let mut done: size_t = 0;
    if !(ascii_length
        <= (((*armor).encode_length).unwrap()(data_length))
            .wrapping_add((*armor).encode_final_length as libc::c_ulong))
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1230 as libc::c_int,
            b"ascii_length <= (armor->encode_length(data_length) + armor->encode_final_length)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(data_length <= ((*armor).decode_length).unwrap()(ascii_length)) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1231 as libc::c_int,
            b"data_length <= armor->decode_length(ascii_length)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    memset(
        buffer as *mut libc::c_void,
        0x33 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong).wrapping_add(ascii_length),
    );
    memset(
        check as *mut libc::c_void,
        0x55 as libc::c_int,
        (1 as libc::c_int as libc::c_ulong).wrapping_add(data_length),
    );
    ((*armor).encode_init).unwrap()(encode);
    done = ((*armor).encode_update).unwrap()(encode, buffer, data_length, data);
    done = (done as libc::c_ulong)
        .wrapping_add(
            ((*armor).encode_final).unwrap()(encode, buffer.offset(done as isize)),
        ) as size_t as size_t;
    if !(done == ascii_length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1240 as libc::c_int,
            b"done == ascii_length\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if memcmp(buffer as *const libc::c_void, ascii as *const libc::c_void, ascii_length)
        != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1242 as libc::c_int,
            b"MEMEQ(ascii_length, buffer, ascii)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(0x33 as libc::c_int == *buffer.offset(strlen(ascii) as isize) as libc::c_int) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1243 as libc::c_int,
            b"0x33 == buffer[strlen(ascii)]\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    ((*armor).decode_init).unwrap()(decode);
    done = ((*armor).decode_length).unwrap()(ascii_length);
    if ((*armor).decode_update).unwrap()(decode, &mut done, check, ascii_length, buffer)
        == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1248 as libc::c_int,
            b"armor->decode_update(decode, &done, check, ascii_length, buffer)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(done == data_length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1249 as libc::c_int,
            b"done == data_length\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if ((*armor).decode_final).unwrap()(decode) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1250 as libc::c_int,
            b"armor->decode_final(decode)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if memcmp(check as *const libc::c_void, data as *const libc::c_void, data_length)
        != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1252 as libc::c_int,
            b"MEMEQ(data_length, check, data)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(0x55 as libc::c_int == *check.offset(data_length as isize) as libc::c_int) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1253 as libc::c_int,
            b"0x55 == check[data_length]\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    free(buffer as *mut libc::c_void);
    free(check as *mut libc::c_void);
    free(encode);
    free(decode);
}
pub unsafe extern "C" fn mpn_out_str(
    mut f: *mut FILE,
    mut base: libc::c_int,
    mut xp: *const mp_limb_t,
    mut xn: mp_size_t,
) {
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_out_str(f, base, __gmpz_roinit_n(x.as_mut_ptr(), xp, xn));
}
unsafe extern "C" fn get_random_seed(mut seed: *mut __mpz_struct) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(
        b"/dev/urandom\0" as *const u8 as *const libc::c_char,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if !f.is_null() {
        let mut buf: [uint8_t; 8] = [0; 8];
        let mut res: size_t = 0;
        setbuf(f, 0 as *mut libc::c_char);
        res = fread(
            &mut buf as *mut [uint8_t; 8] as *mut libc::c_void,
            ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        );
        fclose(f);
        if res == 1 as libc::c_int as libc::c_ulong {
            nettle_mpz_set_str_256_u(
                seed,
                ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
                buf.as_mut_ptr(),
            );
            return;
        }
        fprintf(
            stderr,
            b"Read of /dev/urandom failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    __gmpz_set_ui(seed, tv.tv_sec as libc::c_ulong);
    __gmpz_mul_ui(seed, seed as mpz_srcptr, 1000000 as libc::c_ulong);
    __gmpz_add_ui(seed, seed as mpz_srcptr, tv.tv_usec as libc::c_ulong);
}
pub unsafe extern "C" fn test_randomize(
    mut rands: *mut __gmp_randstate_struct,
) -> libc::c_int {
    let mut nettle_test_seed: *const libc::c_char = 0 as *const libc::c_char;
    nettle_test_seed = getenv(b"NETTLE_TEST_SEED\0" as *const u8 as *const libc::c_char);
    if !nettle_test_seed.is_null() && *nettle_test_seed as libc::c_int != 0 {
        let mut seed: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(seed.as_mut_ptr());
        if __gmpz_set_str(seed.as_mut_ptr(), nettle_test_seed, 0 as libc::c_int)
            < 0 as libc::c_int
            || (if (*seed.as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*seed.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            }) < 0 as libc::c_int
        {
            die(
                b"Invalid NETTLE_TEST_SEED: %s\n\0" as *const u8 as *const libc::c_char,
                nettle_test_seed,
            );
        }
        if (if (*seed.as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*seed.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) == 0 as libc::c_int
        {
            get_random_seed(seed.as_mut_ptr());
        }
        fprintf(
            stderr,
            b"Using NETTLE_TEST_SEED=\0" as *const u8 as *const libc::c_char,
        );
        __gmpz_out_str(stderr, 10 as libc::c_int, seed.as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        __gmp_randseed(rands, seed.as_mut_ptr() as mpz_srcptr);
        __gmpz_clear(seed.as_mut_ptr());
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn xalloc_limbs(mut n: mp_size_t) -> *mut mp_limb_t {
    return xalloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as *mut mp_limb_t;
}
pub unsafe extern "C" fn test_rsa_set_key_1(
    mut pub_0: *mut rsa_public_key,
    mut key: *mut rsa_private_key,
) {
    __gmpz_set_str(
        ((*pub_0).n).as_mut_ptr(),
        b"69abd505285af66536ddc7c8f027e6f0ed435d6748b160884fd60842b3a8d7fbbd8a3c98f0cc50ae4f6a9f7dd73122ccec8afa3f77134406f53721973115fc2d8cfbba23b145f28d84f81d3b6ae8ce1e2850580c026e809bcfbb52566ea3a3b3df7edf52971872a7e35c1451b8636d22279a8fb299368238e545fbb4cf\0"
            as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    __gmpz_set_str(
        ((*pub_0).e).as_mut_ptr(),
        b"0db2ad57\0" as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    if nettle_rsa_public_key_prepare(pub_0) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1441 as libc::c_int,
            b"rsa_public_key_prepare(pub)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_set_str(
        ((*key).p).as_mut_ptr(),
        b"0a66399919be4b4de5a78c5ea5c85bf9aba8c013cb4a873214557a12bd67711ebb4073fd39ad9a86f4e80253ad809e5bf2fad3bc37f6f013273c9552c9f489\0"
            as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    __gmpz_set_str(
        ((*key).q).as_mut_ptr(),
        b"0a294f069f118625f5eae2538db9338c776a298eae9533299fd1eed4eba04e82b2593bc98ba8db27de034da7daaea7952d55b07b5f9a5875d1ca5f6dcab897\0"
            as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    __gmpz_set_str(
        ((*key).a).as_mut_ptr(),
        b"011b6c48eb592eeee85d1bb35cfb6e07344ea0b5e5f03a285b405396cbc78c5c868e961db160ba8d4b984250930cf79a1bf8a9f28963de53128aa7d690eb87\0"
            as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    __gmpz_set_str(
        ((*key).b).as_mut_ptr(),
        b"0409ecf3d2557c88214f1af5e1f17853d8b2d63782fa562860cf579b0833b7ff5c0529f2a97c64522fa1a8878a9635abce56debf431bdec270b308fa5bf387\0"
            as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    __gmpz_set_str(
        ((*key).c).as_mut_ptr(),
        b"04e103ee925cb5e66653949fa5e1a462c9e65e1adcd60058e2df9607cee95fa8daec7a389a7d9afc8dd21fef9d83805a40d46f49676a2f6b2926f70c572c00\0"
            as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    if nettle_rsa_private_key_prepare(key) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1479 as libc::c_int,
            b"rsa_private_key_prepare(key)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*pub_0).size == (*key).size) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1480 as libc::c_int,
            b"pub->size == key->size\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
}
pub unsafe extern "C" fn test_rsa_md5(
    mut pub_0: *mut rsa_public_key,
    mut key: *mut rsa_private_key,
    mut expected: *mut __mpz_struct,
) {
    let mut md5: md5_ctx = md5_ctx {
        state: [0; 4],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut rstate: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut digest: [uint8_t; 16] = [0; 16];
    let mut signature: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    nettle_md5_init(&mut md5);
    __gmpz_init(signature.as_mut_ptr());
    nettle_knuth_lfib_init(&mut rstate, 15 as libc::c_int as uint32_t);
    nettle_md5_update(
        &mut md5,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_md5_sign(key, &mut md5, signature.as_mut_ptr()) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"rsa_md5_sign(key, &md5, signature)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"rsa-%s signature: \0" as *const u8 as *const libc::c_char,
            b"md5\0" as *const u8 as *const libc::c_char,
        );
        __gmpz_out_str(stderr, 16 as libc::c_int, signature.as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_md5_update(
        &mut md5,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_md5_sign_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        &mut md5,
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"rsa_md5_sign_tr(pub, key, &rstate, (nettle_random_func *) knuth_lfib_random, &md5, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_md5_update(
        &mut md5,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    nettle_md5_digest(
        &mut md5,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    if nettle_rsa_md5_sign_digest(key, digest.as_mut_ptr(), signature.as_mut_ptr()) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"rsa_md5_sign_digest(key, digest, signature)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if nettle_rsa_md5_sign_digest_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        digest.as_mut_ptr(),
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"rsa_md5_sign_digest_tr(pub, key, &rstate, (nettle_random_func *)knuth_lfib_random, digest, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_md5_update(
        &mut md5,
        (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 41] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_uchar; 41],
                >(b"The magick words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_md5_verify(
        pub_0,
        &mut md5,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1501 as libc::c_int,
            b"!VERIFY(pub, md5, \"The magick words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_md5_update(
        &mut md5,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_md5_verify(
        pub_0,
        &mut md5,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1505 as libc::c_int,
            b"VERIFY(pub, md5, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_combit(signature.as_mut_ptr(), 17 as libc::c_int as mp_bitcnt_t);
    nettle_md5_update(
        &mut md5,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_md5_verify(
        pub_0,
        &mut md5,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1510 as libc::c_int,
            b"!VERIFY(pub, md5, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_clear(signature.as_mut_ptr());
}
pub unsafe extern "C" fn test_rsa_sha1(
    mut pub_0: *mut rsa_public_key,
    mut key: *mut rsa_private_key,
    mut expected: *mut __mpz_struct,
) {
    let mut sha1: sha1_ctx = sha1_ctx {
        state: [0; 5],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut rstate: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut digest: [uint8_t; 20] = [0; 20];
    let mut signature: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    nettle_sha1_init(&mut sha1);
    __gmpz_init(signature.as_mut_ptr());
    nettle_knuth_lfib_init(&mut rstate, 16 as libc::c_int as uint32_t);
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha1_sign(key, &mut sha1, signature.as_mut_ptr()) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"rsa_sha1_sign(key, &sha1, signature)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"rsa-%s signature: \0" as *const u8 as *const libc::c_char,
            b"sha1\0" as *const u8 as *const libc::c_char,
        );
        __gmpz_out_str(stderr, 16 as libc::c_int, signature.as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha1_sign_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        &mut sha1,
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"rsa_sha1_sign_tr(pub, key, &rstate, (nettle_random_func *) knuth_lfib_random, &sha1, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    nettle_sha1_digest(
        &mut sha1,
        ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    if nettle_rsa_sha1_sign_digest(key, digest.as_mut_ptr(), signature.as_mut_ptr()) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"rsa_sha1_sign_digest(key, digest, signature)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if nettle_rsa_sha1_sign_digest_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        digest.as_mut_ptr(),
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"rsa_sha1_sign_digest_tr(pub, key, &rstate, (nettle_random_func *)knuth_lfib_random, digest, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1529 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 41] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_uchar; 41],
                >(b"The magick words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha1_verify(
        pub_0,
        &mut sha1,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1533 as libc::c_int,
            b"!VERIFY(pub, sha1, \"The magick words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha1_verify(
        pub_0,
        &mut sha1,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1537 as libc::c_int,
            b"VERIFY(pub, sha1, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_combit(signature.as_mut_ptr(), 17 as libc::c_int as mp_bitcnt_t);
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha1_verify(
        pub_0,
        &mut sha1,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1542 as libc::c_int,
            b"!VERIFY(pub, sha1, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_clear(signature.as_mut_ptr());
}
pub unsafe extern "C" fn test_rsa_sha256(
    mut pub_0: *mut rsa_public_key,
    mut key: *mut rsa_private_key,
    mut expected: *mut __mpz_struct,
) {
    let mut sha256: sha256_ctx = sha256_ctx {
        state: [0; 8],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut rstate: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut digest: [uint8_t; 32] = [0; 32];
    let mut signature: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    nettle_sha256_init(&mut sha256);
    __gmpz_init(signature.as_mut_ptr());
    nettle_knuth_lfib_init(&mut rstate, 17 as libc::c_int as uint32_t);
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha256_sign(key, &mut sha256, signature.as_mut_ptr()) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"rsa_sha256_sign(key, &sha256, signature)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"rsa-%s signature: \0" as *const u8 as *const libc::c_char,
            b"sha256\0" as *const u8 as *const libc::c_char,
        );
        __gmpz_out_str(stderr, 16 as libc::c_int, signature.as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha256_sign_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        &mut sha256,
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"rsa_sha256_sign_tr(pub, key, &rstate, (nettle_random_func *) knuth_lfib_random, &sha256, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    nettle_sha256_digest(
        &mut sha256,
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    if nettle_rsa_sha256_sign_digest(key, digest.as_mut_ptr(), signature.as_mut_ptr())
        == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"rsa_sha256_sign_digest(key, digest, signature)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if nettle_rsa_sha256_sign_digest_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        digest.as_mut_ptr(),
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"rsa_sha256_sign_digest_tr(pub, key, &rstate, (nettle_random_func *)knuth_lfib_random, digest, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 41] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_uchar; 41],
                >(b"The magick words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha256_verify(
        pub_0,
        &mut sha256,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1565 as libc::c_int,
            b"!VERIFY(pub, sha256, \"The magick words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha256_verify(
        pub_0,
        &mut sha256,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1569 as libc::c_int,
            b"VERIFY(pub, sha256, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_combit(signature.as_mut_ptr(), 17 as libc::c_int as mp_bitcnt_t);
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha256_verify(
        pub_0,
        &mut sha256,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1574 as libc::c_int,
            b"!VERIFY(pub, sha256, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_clear(signature.as_mut_ptr());
}
pub unsafe extern "C" fn test_rsa_sha512(
    mut pub_0: *mut rsa_public_key,
    mut key: *mut rsa_private_key,
    mut expected: *mut __mpz_struct,
) {
    let mut sha512: sha512_ctx = sha512_ctx {
        state: [0; 8],
        count_low: 0,
        count_high: 0,
        index: 0,
        block: [0; 128],
    };
    let mut rstate: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut digest: [uint8_t; 64] = [0; 64];
    let mut signature: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    nettle_sha512_init(&mut sha512);
    __gmpz_init(signature.as_mut_ptr());
    nettle_knuth_lfib_init(&mut rstate, 18 as libc::c_int as uint32_t);
    nettle_sha512_update(
        &mut sha512,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha512_sign(key, &mut sha512, signature.as_mut_ptr()) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"rsa_sha512_sign(key, &sha512, signature)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"rsa-%s signature: \0" as *const u8 as *const libc::c_char,
            b"sha512\0" as *const u8 as *const libc::c_char,
        );
        __gmpz_out_str(stderr, 16 as libc::c_int, signature.as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha512_update(
        &mut sha512,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha512_sign_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        &mut sha512,
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"rsa_sha512_sign_tr(pub, key, &rstate, (nettle_random_func *) knuth_lfib_random, &sha512, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha512_update(
        &mut sha512,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    nettle_sha512_digest(
        &mut sha512,
        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
        digest.as_mut_ptr(),
    );
    if nettle_rsa_sha512_sign_digest(key, digest.as_mut_ptr(), signature.as_mut_ptr())
        == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"rsa_sha512_sign_digest(key, digest, signature)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if nettle_rsa_sha512_sign_digest_tr(
        pub_0,
        key,
        &mut rstate as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        digest.as_mut_ptr(),
        signature.as_mut_ptr(),
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"rsa_sha512_sign_digest_tr(pub, key, &rstate, (nettle_random_func *)knuth_lfib_random, digest, signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_cmp(signature.as_mut_ptr() as mpz_srcptr, expected as mpz_srcptr)
        == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int,
            b"mpz_cmp (signature, expected) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha512_update(
        &mut sha512,
        (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 41] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_uchar; 41],
                >(b"The magick words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha512_verify(
        pub_0,
        &mut sha512,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1597 as libc::c_int,
            b"!VERIFY(pub, sha512, \"The magick words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha512_update(
        &mut sha512,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha512_verify(
        pub_0,
        &mut sha512,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1601 as libc::c_int,
            b"VERIFY(pub, sha512, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_combit(signature.as_mut_ptr(), 17 as libc::c_int as mp_bitcnt_t);
    nettle_sha512_update(
        &mut sha512,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_rsa_sha512_verify(
        pub_0,
        &mut sha512,
        signature.as_mut_ptr() as *const __mpz_struct,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1606 as libc::c_int,
            b"!VERIFY(pub, sha512, \"The magic words are squeamish ossifrage\", signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_clear(signature.as_mut_ptr());
}
pub unsafe extern "C" fn test_rsa_key(
    mut pub_0: *mut rsa_public_key,
    mut key: *mut rsa_private_key,
) {
    let mut tmp: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut phi: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(tmp.as_mut_ptr());
    __gmpz_init(phi.as_mut_ptr());
    if verbose != 0 {
        fprintf(stderr, b"Public key: n=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(
            stderr,
            16 as libc::c_int,
            ((*pub_0).n).as_mut_ptr() as mpz_srcptr,
        );
        fprintf(stderr, b"\n    e=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(
            stderr,
            16 as libc::c_int,
            ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
        );
        fprintf(stderr, b"\n\nPrivate key: d=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(stderr, 16 as libc::c_int, ((*key).d).as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n    p=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(stderr, 16 as libc::c_int, ((*key).p).as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n    q=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(stderr, 16 as libc::c_int, ((*key).q).as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n    a=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(stderr, 16 as libc::c_int, ((*key).a).as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n    b=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(stderr, 16 as libc::c_int, ((*key).b).as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n    c=\0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(stderr, 16 as libc::c_int, ((*key).c).as_mut_ptr() as mpz_srcptr);
        fprintf(stderr, b"\n\n\0" as *const u8 as *const libc::c_char);
    }
    __gmpz_mul(
        tmp.as_mut_ptr(),
        ((*key).p).as_mut_ptr() as mpz_srcptr,
        ((*key).q).as_mut_ptr() as mpz_srcptr,
    );
    if !(__gmpz_cmp(
        tmp.as_mut_ptr() as mpz_srcptr,
        ((*pub_0).n).as_mut_ptr() as mpz_srcptr,
    ) == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1647 as libc::c_int,
            b"mpz_cmp(tmp, pub->n)== 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_mul(
        tmp.as_mut_ptr(),
        ((*key).c).as_mut_ptr() as mpz_srcptr,
        ((*key).q).as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        ((*key).p).as_mut_ptr() as mpz_srcptr,
    );
    if !((if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
        (if (*tmp.as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*tmp.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        })
    } else {
        __gmpz_cmp_ui(tmp.as_mut_ptr() as mpz_srcptr, 1 as libc::c_int as libc::c_ulong)
    }) == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1652 as libc::c_int,
            b"mpz_cmp_ui(tmp, 1) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_sub_ui(
        phi.as_mut_ptr(),
        ((*key).p).as_mut_ptr() as mpz_srcptr,
        1 as libc::c_int as libc::c_ulong,
    );
    __gmpz_sub_ui(
        tmp.as_mut_ptr(),
        ((*key).q).as_mut_ptr() as mpz_srcptr,
        1 as libc::c_int as libc::c_ulong,
    );
    __gmpz_mul(
        phi.as_mut_ptr(),
        phi.as_mut_ptr() as mpz_srcptr,
        tmp.as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_mul(
        tmp.as_mut_ptr(),
        ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
        ((*key).d).as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        phi.as_mut_ptr() as mpz_srcptr,
    );
    if !((if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
        (if (*tmp.as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*tmp.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        })
    } else {
        __gmpz_cmp_ui(tmp.as_mut_ptr() as mpz_srcptr, 1 as libc::c_int as libc::c_ulong)
    }) == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1662 as libc::c_int,
            b"mpz_cmp_ui(tmp, 1) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_sub_ui(
        phi.as_mut_ptr(),
        ((*key).p).as_mut_ptr() as mpz_srcptr,
        1 as libc::c_int as libc::c_ulong,
    );
    __gmpz_mul(
        tmp.as_mut_ptr(),
        ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
        ((*key).a).as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        phi.as_mut_ptr() as mpz_srcptr,
    );
    if !((if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
        (if (*tmp.as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*tmp.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        })
    } else {
        __gmpz_cmp_ui(tmp.as_mut_ptr() as mpz_srcptr, 1 as libc::c_int as libc::c_ulong)
    }) == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1668 as libc::c_int,
            b"mpz_cmp_ui(tmp, 1) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_sub_ui(
        phi.as_mut_ptr(),
        ((*key).q).as_mut_ptr() as mpz_srcptr,
        1 as libc::c_int as libc::c_ulong,
    );
    __gmpz_mul(
        tmp.as_mut_ptr(),
        ((*pub_0).e).as_mut_ptr() as mpz_srcptr,
        ((*key).b).as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        phi.as_mut_ptr() as mpz_srcptr,
    );
    if !((if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
        (if (*tmp.as_mut_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*tmp.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        })
    } else {
        __gmpz_cmp_ui(tmp.as_mut_ptr() as mpz_srcptr, 1 as libc::c_int as libc::c_ulong)
    }) == 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1674 as libc::c_int,
            b"mpz_cmp_ui(tmp, 1) == 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_clear(tmp.as_mut_ptr());
    __gmpz_clear(phi.as_mut_ptr());
}
pub unsafe extern "C" fn test_dsa160(
    mut pub_0: *const dsa_public_key,
    mut key: *const dsa_private_key,
    mut expected: *const dsa_signature,
) {
    let mut sha1: sha1_ctx = sha1_ctx {
        state: [0; 5],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut signature: dsa_signature = dsa_signature {
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
    let mut lfib: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    nettle_sha1_init(&mut sha1);
    nettle_dsa_signature_init(&mut signature);
    nettle_knuth_lfib_init(&mut lfib, 1111 as libc::c_int as uint32_t);
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha1_sign(
        pub_0,
        key,
        &mut lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        &mut sha1,
        &mut signature,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1700 as libc::c_int,
            b"dsa_sha1_sign(pub, key, &lfib, (nettle_random_func *) knuth_lfib_random, &sha1, &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if verbose != 0 {
        fprintf(stderr, b"dsa160 signature: \0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(
            stderr,
            16 as libc::c_int,
            (signature.r).as_mut_ptr() as mpz_srcptr,
        );
        fprintf(stderr, b", \0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(
            stderr,
            16 as libc::c_int,
            (signature.s).as_mut_ptr() as mpz_srcptr,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !expected.is_null() {
        if !(__gmpz_cmp(
            (signature.r).as_mut_ptr() as mpz_srcptr,
            ((*expected).r).as_ptr(),
        ) == 0 as libc::c_int
            && __gmpz_cmp(
                (signature.s).as_mut_ptr() as mpz_srcptr,
                ((*expected).s).as_ptr(),
            ) == 0 as libc::c_int)
        {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                1713 as libc::c_int,
                b"mpz_cmp (signature.r, expected->r) == 0 && mpz_cmp (signature.s, expected->s) == 0\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    }
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 41] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_uchar; 41],
                >(b"The magick words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha1_verify(pub_0, &mut sha1, &mut signature) != 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1718 as libc::c_int,
            b"!DSA_VERIFY(pub, sha1, \"The magick words are squeamish ossifrage\", &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha1_verify(pub_0, &mut sha1, &mut signature) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1723 as libc::c_int,
            b"DSA_VERIFY(pub, sha1, \"The magic words are squeamish ossifrage\", &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_combit((signature.r).as_mut_ptr(), 17 as libc::c_int as mp_bitcnt_t);
    nettle_sha1_update(
        &mut sha1,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha1_verify(pub_0, &mut sha1, &mut signature) != 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1729 as libc::c_int,
            b"!DSA_VERIFY(pub, sha1, \"The magic words are squeamish ossifrage\", &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_dsa_signature_clear(&mut signature);
}
pub unsafe extern "C" fn test_dsa256(
    mut pub_0: *const dsa_public_key,
    mut key: *const dsa_private_key,
    mut expected: *const dsa_signature,
) {
    let mut sha256: sha256_ctx = sha256_ctx {
        state: [0; 8],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut signature: dsa_signature = dsa_signature {
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
    let mut lfib: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    nettle_sha256_init(&mut sha256);
    nettle_dsa_signature_init(&mut signature);
    nettle_knuth_lfib_init(&mut lfib, 1111 as libc::c_int as uint32_t);
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha256_sign(
        pub_0,
        key,
        &mut lfib as *mut knuth_lfib_ctx as *mut libc::c_void,
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
        &mut sha256,
        &mut signature,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1750 as libc::c_int,
            b"dsa_sha256_sign(pub, key, &lfib, (nettle_random_func *) knuth_lfib_random, &sha256, &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if verbose != 0 {
        fprintf(stderr, b"dsa256 signature: \0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(
            stderr,
            16 as libc::c_int,
            (signature.r).as_mut_ptr() as mpz_srcptr,
        );
        fprintf(stderr, b", \0" as *const u8 as *const libc::c_char);
        __gmpz_out_str(
            stderr,
            16 as libc::c_int,
            (signature.s).as_mut_ptr() as mpz_srcptr,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !expected.is_null() {
        if !(__gmpz_cmp(
            (signature.r).as_mut_ptr() as mpz_srcptr,
            ((*expected).r).as_ptr(),
        ) == 0 as libc::c_int
            && __gmpz_cmp(
                (signature.s).as_mut_ptr() as mpz_srcptr,
                ((*expected).s).as_ptr(),
            ) == 0 as libc::c_int)
        {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                1763 as libc::c_int,
                b"mpz_cmp (signature.r, expected->r) == 0 && mpz_cmp (signature.s, expected->s) == 0\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    }
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 41] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_uchar; 41],
                >(b"The magick words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha256_verify(pub_0, &mut sha256, &mut signature) != 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1768 as libc::c_int,
            b"!DSA_VERIFY(pub, sha256, \"The magick words are squeamish ossifrage\", &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha256_verify(pub_0, &mut sha256, &mut signature) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1773 as libc::c_int,
            b"DSA_VERIFY(pub, sha256, \"The magic words are squeamish ossifrage\", &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_combit((signature.r).as_mut_ptr(), 17 as libc::c_int as mp_bitcnt_t);
    nettle_sha256_update(
        &mut sha256,
        (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ({
            static mut us_s: [libc::c_uchar; 40] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_uchar; 40],
                >(b"The magic words are squeamish ossifrage\0")
            };
            us_s.as_ptr()
        }),
    );
    if nettle_dsa_sha256_verify(pub_0, &mut sha256, &mut signature) != 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1779 as libc::c_int,
            b"!DSA_VERIFY(pub, sha256, \"The magic words are squeamish ossifrage\", &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    nettle_dsa_signature_clear(&mut signature);
}
pub unsafe extern "C" fn test_dsa_verify(
    mut params: *const dsa_params,
    mut pub_0: *const __mpz_struct,
    mut hash: *const nettle_hash,
    mut msg: *mut tstring,
    mut ref_0: *const dsa_signature,
) {
    let mut ctx: *mut libc::c_void = xalloc((*hash).context_size as size_t);
    let mut digest: *mut uint8_t = xalloc((*hash).digest_size as size_t) as *mut uint8_t;
    let mut signature: dsa_signature = dsa_signature {
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
    nettle_dsa_signature_init(&mut signature);
    ((*hash).init).unwrap()(ctx);
    ((*hash).update).unwrap()(ctx, (*msg).length, ((*msg).data).as_mut_ptr());
    ((*hash).digest).unwrap()(ctx, (*hash).digest_size as size_t, digest);
    __gmpz_set((signature.r).as_mut_ptr(), ((*ref_0).r).as_ptr());
    __gmpz_set((signature.s).as_mut_ptr(), ((*ref_0).s).as_ptr());
    if nettle_dsa_verify(
        params,
        pub_0,
        (*hash).digest_size as size_t,
        digest,
        &mut signature,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1866 as libc::c_int,
            b"dsa_verify (params, pub, hash->digest_size, digest, &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_combit((signature.r).as_mut_ptr(), 17 as libc::c_int as mp_bitcnt_t);
    if nettle_dsa_verify(
        params,
        pub_0,
        (*hash).digest_size as size_t,
        digest,
        &mut signature,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1872 as libc::c_int,
            b"!dsa_verify (params, pub, hash->digest_size, digest, &signature)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let ref mut fresh4 = *digest
        .offset(
            ((*hash).digest_size)
                .wrapping_div(2 as libc::c_int as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        );
    *fresh4 = (*fresh4 as libc::c_int ^ 8 as libc::c_int) as uint8_t;
    if nettle_dsa_verify(params, pub_0, (*hash).digest_size as size_t, digest, ref_0)
        != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1878 as libc::c_int,
            b"!dsa_verify (params, pub, hash->digest_size, digest, ref)\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    free(ctx);
    free(digest as *mut libc::c_void);
    nettle_dsa_signature_clear(&mut signature);
}
pub unsafe extern "C" fn test_dsa_key(
    mut params: *const dsa_params,
    mut pub_0: *const __mpz_struct,
    mut key: *const __mpz_struct,
    mut q_size: libc::c_uint,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(t.as_mut_ptr());
    if !(__gmpz_sizeinbase(((*params).q).as_ptr(), 2 as libc::c_int)
        == q_size as libc::c_ulong)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1895 as libc::c_int,
            b"mpz_sizeinbase(params->q, 2) == q_size\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if !(__gmpz_sizeinbase(((*params).p).as_ptr(), 2 as libc::c_int)
        >= 512 as libc::c_int as libc::c_ulong)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1896 as libc::c_int,
            b"mpz_sizeinbase(params->p, 2) >= DSA_SHA1_MIN_P_BITS\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if __gmpz_probab_prime_p(((*params).p).as_ptr(), 10 as libc::c_int) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1898 as libc::c_int,
            b"mpz_probab_prime_p(params->p, 10)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if __gmpz_probab_prime_p(((*params).q).as_ptr(), 10 as libc::c_int) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1900 as libc::c_int,
            b"mpz_probab_prime_p(params->q, 10)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_fdiv_r(t.as_mut_ptr(), ((*params).p).as_ptr(), ((*params).q).as_ptr());
    if !(0 as libc::c_int
        == (if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
            (if (*t.as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*t.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            })
        } else {
            __gmpz_cmp_ui(
                t.as_mut_ptr() as mpz_srcptr,
                1 as libc::c_int as libc::c_ulong,
            )
        }))
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1904 as libc::c_int,
            b"0 == mpz_cmp_ui(t, 1)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
        (if (*((*params).g).as_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*params).g).as_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        })
    } else {
        __gmpz_cmp_ui(((*params).g).as_ptr(), 1 as libc::c_int as libc::c_ulong)
    }) > 0 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1906 as libc::c_int,
            b"mpz_cmp_ui(params->g, 1) > 0\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_powm(
        t.as_mut_ptr(),
        ((*params).g).as_ptr(),
        ((*params).q).as_ptr(),
        ((*params).p).as_ptr(),
    );
    if !(0 as libc::c_int
        == (if 0 != 0 && 1 as libc::c_int == 0 as libc::c_int {
            (if (*t.as_mut_ptr())._mp_size < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                ((*t.as_mut_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
            })
        } else {
            __gmpz_cmp_ui(
                t.as_mut_ptr() as mpz_srcptr,
                1 as libc::c_int as libc::c_ulong,
            )
        }))
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1909 as libc::c_int,
            b"0 == mpz_cmp_ui(t, 1)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_powm(t.as_mut_ptr(), ((*params).g).as_ptr(), key, ((*params).p).as_ptr());
    if !(0 as libc::c_int == __gmpz_cmp(t.as_mut_ptr() as mpz_srcptr, pub_0)) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            1912 as libc::c_int,
            b"0 == mpz_cmp(t, pub)\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    __gmpz_clear(t.as_mut_ptr());
}
pub static mut ecc_curves: [*const ecc_curve; 10] = unsafe {
    [
        &_nettle_secp_192r1 as *const ecc_curve,
        &_nettle_secp_224r1 as *const ecc_curve,
        &_nettle_secp_256r1 as *const ecc_curve,
        &_nettle_secp_384r1 as *const ecc_curve,
        &_nettle_secp_521r1 as *const ecc_curve,
        &_nettle_curve25519 as *const ecc_curve,
        &_nettle_curve448 as *const ecc_curve,
        &_nettle_gost_gc256b as *const ecc_curve,
        &_nettle_gost_gc512a as *const ecc_curve,
        0 as *const ecc_curve,
    ]
};
pub unsafe extern "C" fn test_ecc_point_valid_p(
    mut pub_0: *mut ecc_point,
) -> libc::c_int {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
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
    let mut lhs: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut rhs: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    let mut size: mp_size_t = 0;
    size = (*(*pub_0).ecc).p.size as mp_size_t;
    if __gmpn_cmp((*pub_0).p as mp_srcptr, (*(*pub_0).ecc).p.m, size) >= 0 as libc::c_int
        || __gmpn_cmp(
            ((*pub_0).p).offset(size as isize) as mp_srcptr,
            (*(*pub_0).ecc).p.m,
            size,
        ) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(lhs.as_mut_ptr());
    __gmpz_init(rhs.as_mut_ptr());
    __gmpz_roinit_n(x.as_mut_ptr(), (*pub_0).p as mp_srcptr, size);
    __gmpz_roinit_n(
        y.as_mut_ptr(),
        ((*pub_0).p).offset(size as isize) as mp_srcptr,
        size,
    );
    __gmpz_mul(
        lhs.as_mut_ptr(),
        y.as_mut_ptr() as mpz_srcptr,
        y.as_mut_ptr() as mpz_srcptr,
    );
    if (*(*pub_0).ecc).p.bit_size as libc::c_int == 255 as libc::c_int {
        let mut x2: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(x2.as_mut_ptr());
        __gmpz_mul(
            x2.as_mut_ptr(),
            x.as_mut_ptr() as mpz_srcptr,
            x.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_mul(
            rhs.as_mut_ptr(),
            x2.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_sub(
            lhs.as_mut_ptr(),
            x2.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_add_ui(
            lhs.as_mut_ptr(),
            lhs.as_mut_ptr() as mpz_srcptr,
            1 as libc::c_int as libc::c_ulong,
        );
        __gmpz_mul_ui(
            lhs.as_mut_ptr(),
            lhs.as_mut_ptr() as mpz_srcptr,
            121666 as libc::c_int as libc::c_ulong,
        );
        __gmpz_mul_ui(
            rhs.as_mut_ptr(),
            rhs.as_mut_ptr() as mpz_srcptr,
            121665 as libc::c_int as libc::c_ulong,
        );
        __gmpz_clear(x2.as_mut_ptr());
    } else if (*(*pub_0).ecc).p.bit_size as libc::c_int == 448 as libc::c_int {
        let mut x2_0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut d: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(x2_0.as_mut_ptr());
        __gmpz_init_set_ui(d.as_mut_ptr(), 39081 as libc::c_int as libc::c_ulong);
        __gmpz_mul(
            x2_0.as_mut_ptr(),
            x.as_mut_ptr() as mpz_srcptr,
            x.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_mul(
            d.as_mut_ptr(),
            d.as_mut_ptr() as mpz_srcptr,
            x2_0.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_set_ui(rhs.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
        __gmpz_submul(
            rhs.as_mut_ptr(),
            d.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_add(
            lhs.as_mut_ptr(),
            x2_0.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_clear(d.as_mut_ptr());
        __gmpz_clear(x2_0.as_mut_ptr());
    } else {
        __gmpz_mul(
            rhs.as_mut_ptr(),
            x.as_mut_ptr() as mpz_srcptr,
            x.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_sub_ui(
            rhs.as_mut_ptr(),
            rhs.as_mut_ptr() as mpz_srcptr,
            3 as libc::c_int as libc::c_ulong,
        );
        __gmpz_mul(
            rhs.as_mut_ptr(),
            rhs.as_mut_ptr() as mpz_srcptr,
            x.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_add(
            rhs.as_mut_ptr(),
            rhs.as_mut_ptr() as mpz_srcptr,
            __gmpz_roinit_n(t.as_mut_ptr(), (*(*pub_0).ecc).b, size),
        );
    }
    res = __gmpz_congruent_p(
        lhs.as_mut_ptr() as mpz_srcptr,
        rhs.as_mut_ptr() as mpz_srcptr,
        __gmpz_roinit_n(t.as_mut_ptr(), (*(*pub_0).ecc).p.m, size),
    );
    __gmpz_clear(lhs.as_mut_ptr());
    __gmpz_clear(rhs.as_mut_ptr());
    return res;
}
unsafe extern "C" fn test_mpn(
    mut ref_0: *const libc::c_char,
    mut xp: *const mp_limb_t,
    mut n: mp_size_t,
) -> libc::c_int {
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    __gmpz_init_set_str(r.as_mut_ptr(), ref_0, 16 as libc::c_int);
    res = (__gmpz_cmp(
        r.as_mut_ptr() as mpz_srcptr,
        __gmpz_roinit_n(x.as_mut_ptr(), xp, n),
    ) == 0 as libc::c_int) as libc::c_int;
    __gmpz_clear(r.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn write_mpn(
    mut f: *mut FILE,
    mut base: libc::c_int,
    mut xp: *const mp_limb_t,
    mut n: mp_size_t,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_out_str(f, base, __gmpz_roinit_n(t.as_mut_ptr(), xp, n));
}
pub unsafe extern "C" fn test_ecc_point(
    mut ecc: *const ecc_curve,
    mut ref_0: *const ecc_ref_point,
    mut p: *const mp_limb_t,
) {
    if !(test_mpn((*ref_0).x, p, (*ecc).p.size as mp_size_t) != 0
        && test_mpn(
            (*ref_0).y,
            p.offset((*ecc).p.size as libc::c_int as isize),
            (*ecc).p.size as mp_size_t,
        ) != 0)
    {
        fprintf(
            stderr,
            b"Incorrect point, curve bits %d!\ngot: x = \0" as *const u8
                as *const libc::c_char,
            (*ecc).p.bit_size as libc::c_int,
        );
        write_mpn(stderr, 16 as libc::c_int, p, (*ecc).p.size as mp_size_t);
        fprintf(stderr, b"\n     y = \0" as *const u8 as *const libc::c_char);
        write_mpn(
            stderr,
            16 as libc::c_int,
            p.offset((*ecc).p.size as libc::c_int as isize),
            (*ecc).p.size as mp_size_t,
        );
        fprintf(
            stderr,
            b"\nref: x = %s\n     y = %s\n\0" as *const u8 as *const libc::c_char,
            (*ref_0).x,
            (*ref_0).y,
        );
        abort();
    }
}
static mut ecc_ref: [[ecc_ref_point; 4]; 9] = [
    [
        {
            let mut init = ecc_ref_point {
                x: b"188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012\0" as *const u8
                    as *const libc::c_char,
                y: b"07192b95ffc8da78631011ed6b24cdd573f977a11e794811\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"dafebf5828783f2ad35534631588a3f629a70fb16982a888\0" as *const u8
                    as *const libc::c_char,
                y: b"dd6bda0d993da0fa46b27bbc141b868f59331afa5c7e93ab\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"76e32a2557599e6edcd283201fb2b9aadfd0d359cbb263da\0" as *const u8
                    as *const libc::c_char,
                y: b"782c37e372ba4520aa62e0fed121d49ef3b543660cfd05fd\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"35433907297cc378b0015703374729d7a4fe46647084e4ba\0" as *const u8
                    as *const libc::c_char,
                y: b"a2649984f2135c301ea3acb0776cd4f125389b311db3be32\0" as *const u8
                    as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21\0"
                    as *const u8 as *const libc::c_char,
                y: b"bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"706a46dc76dcb76798e60e6d89474788d16dc18032d268fd1a704fa6\0"
                    as *const u8 as *const libc::c_char,
                y: b"1c2b76a7bc25e7702a704fa986892849fca629487acf3709d2e4e8bb\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"df1b1d66a551d0d31eff822558b9d2cc75c2180279fe0d08fd896d04\0"
                    as *const u8 as *const libc::c_char,
                y: b"a3f7f03cadd0be444c0aa56830130ddf77d317344e1af3591981a925\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"ae99feebb5d26945b54892092a8aee02912930fa41cd114e40447301\0"
                    as *const u8 as *const libc::c_char,
                y: b"482580a0ec5bc47e88bc8c378632cd196cb3fa058a7114eb03054c9\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296\0"
                    as *const u8 as *const libc::c_char,
                y: b"4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"7cf27b188d034f7e8a52380304b51ac3c08969e277f21b35a60b48fc47669978\0"
                    as *const u8 as *const libc::c_char,
                y: b"7775510db8ed040293d9ac69f7430dbba7dade63ce982299e04b79d227873d1\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"5ecbe4d1a6330a44c8f7ef951d4bf165e6c6b721efada985fb41661bc6e7fd6c\0"
                    as *const u8 as *const libc::c_char,
                y: b"8734640c4998ff7e374b06ce1a64a2ecd82ab036384fb83d9a79b127a27d5032\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"e2534a3532d08fbba02dde659ee62bd0031fe2db785596ef509302446b030852\0"
                    as *const u8 as *const libc::c_char,
                y: b"e0f1575a4c633cc719dfee5fda862d764efc96c3f30ee0055c42c23f184ed8c6\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7\0"
                    as *const u8 as *const libc::c_char,
                y: b"3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"8d999057ba3d2d969260045c55b97f089025959a6f434d651d207d19fb96e9e4fe0e86ebe0e64f85b96a9c75295df61\0"
                    as *const u8 as *const libc::c_char,
                y: b"8e80f1fa5b1b3cedb7bfe8dffd6dba74b275d875bc6cc43e904e505f256ab4255ffd43e94d39e22d61501e700a940e80\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"77a41d4606ffa1464793c7e5fdc7d98cb9d3910202dcd06bea4f240d3566da6b408bbae5026580d02d7e5c70500c831\0"
                    as *const u8 as *const libc::c_char,
                y: b"c995f7ca0b0c42837d0bbe9602a9fc998520b41c85115aa5f7684c0edc111eacc24abd6be4b5d298b65f28600a2f1df1\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"138251cd52ac9298c1c8aad977321deb97e709bd0b4ca0aca55dc8ad51dcfc9d1589a1597e3a5120e1efd631c63e1835\0"
                    as *const u8 as *const libc::c_char,
                y: b"cacae29869a62e1631e8a28181ab56616dc45d918abc09f3ab0e63cf792aa4dced7387be37bba569549f1c02b270ed67\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66\0"
                    as *const u8 as *const libc::c_char,
                y: b"11839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"433c219024277e7e682fcb288148c282747403279b1ccc06352c6e5505d769be97b3b204da6ef55507aa104a3a35c5af41cf2fa364d60fd967f43e3933ba6d783d\0"
                    as *const u8 as *const libc::c_char,
                y: b"f4bb8cc7f86db26700a7f3eceeeed3f0b5c6b5107c4da97740ab21a29906c42dbbb3e377de9f251f6b93937fa99a3248f4eafcbe95edc0f4f71be356d661f41b02\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"1a73d352443de29195dd91d6a64b5959479b52a6e5b123d9ab9e5ad7a112d7a8dd1ad3f164a3a4832051da6bd16b59fe21baeb490862c32ea05a5919d2ede37ad7d\0"
                    as *const u8 as *const libc::c_char,
                y: b"13e9b03b97dfa62ddd9979f86c6cab814f2f1557fa82a9d0317d2f8ab1fa355ceec2e2dd4cf8dc575b02d5aced1dec3c70cf105c9bc93a590425f588ca1ee86c0e5\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"35b5df64ae2ac204c354b483487c9070cdc61c891c5ff39afc06c5d55541d3ceac8659e24afe3d0750e8b88e9f078af066a1d5025b08e5a5e2fbc87412871902f3\0"
                    as *const u8 as *const libc::c_char,
                y: b"82096f84261279d2b673e0178eb0b4abb65521aef6e6e32e1b5ae63fe2f19907f279f283e54ba385405224f750a95b85eebb7faef04699d1d9e21f47fc346e4d0d\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"216936d3cd6e53fec0a4e231fdd6dc5c692cc7609525a7b2c9562d608f25d51a\0"
                    as *const u8 as *const libc::c_char,
                y: b"6666666666666666666666666666666666666666666666666666666666666658\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"36ab384c9f5a046c3d043b7d1833e7ac080d8e4515d7a45f83c5a14e2843ce0e\0"
                    as *const u8 as *const libc::c_char,
                y: b"2260cdf3092329c21da25ee8c9a21f5697390f51643851560e5f46ae6af8a3c9\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"67ae9c4a22928f491ff4ae743edac83a6343981981624886ac62485fd3f8e25c\0"
                    as *const u8 as *const libc::c_char,
                y: b"1267b1d177ee69aba126a18e60269ef79f16ec176724030402c3684878f5b4d4\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"203da8db56cff1468325d4b87a3520f91a739ec193ce1547493aa657c4c9f870\0"
                    as *const u8 as *const libc::c_char,
                y: b"47d0e827cb1595e1470eb88580d5716c4cf22832ea2f0ff0df38ab61ca32112f\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"4f1970c66bed0ded221d15a622bf36da9e146570470f1767ea6de324a3d3a46412ae1af72ab66511433b80e18b00938e2626a82bc70cc05e\0"
                    as *const u8 as *const libc::c_char,
                y: b"693f46716eb6bc248876203756c9c7624bea73736ca3984087789c1e05a0c2d73ad3ff1ce67c39c4fdbd132c4ed7c8ad9808795bf230fa14\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa955555555555555555555555555555555555555555555555555555555\0"
                    as *const u8 as *const libc::c_char,
                y: b"ae05e9634ad7048db359d6205086c2b0036ed7a035884dd7b7e36d728ad8c4b80d6565833a2a3098bbbcb2bed1cda06bdaeafbcdea9386ed\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"865886b9108af6455bd64316cb6943332241b8b8cda82c7e2ba077a4a3fcfe8daa9cbf7f6271fd6e862b769465da8575728173286ff2f8f\0"
                    as *const u8 as *const libc::c_char,
                y: b"e005a8dbd5125cf706cbda7ad43aa6449a4a8d952356c3b9fce43c82ec4e1d58bb3a331bdb6767f0bffa9a68fed02dafb822ac13588ed6fc\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"49dcbc5c6c0cce2c1419a17226f929ea255a09cf4e0891c693fda4be70c74cc301b7bdf1515dd8ba21aee1798949e120e2ce42ac48ba7f30\0"
                    as *const u8 as *const libc::c_char,
                y: b"d49077e4accde527164b33a5de021b979cb7c02f0457d845c90dc3227b8a5bc1c0d8f97ea1ca9472b5d444285d0d4f5b32e236f86de51839\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"0000000000000000000000000000000000000000000000000000000000000001\0"
                    as *const u8 as *const libc::c_char,
                y: b"8d91e471e0989cda27df505a453f2b7635294f2ddf23e3b122acc99c9e9f1e14\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd95\0"
                    as *const u8 as *const libc::c_char,
                y: b"726e1b8e1f676325d820afa5bac0d489cad6b0d220dc1c4edd5336636160df83\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38d2c\0"
                    as *const u8 as *const libc::c_char,
                y: b"76bcd1ca9a23b041d4d9baf507a6cd821267a94c838768e8486117796b788a51\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"f7063e7063e7063e7063e7063e7063e7063e7063e7063e7063e7063e7063e4b7\0"
                    as *const u8 as *const libc::c_char,
                y: b"83ccf17ba6706d73625cc3534c7a2b9d6ec1ee6a9a7e07c10d84b388de59f741\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
    [
        {
            let mut init = ecc_ref_point {
                x: b"00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003\0"
                    as *const u8 as *const libc::c_char,
                y: b"7503cfe87a836ae3a61b8816e25450e6ce5e1c93acf1abc1778064fdcbefa921df1626be4fd036e93d75e6a50e3a41e98028fe5fc235f5b889a589cb5215f2a4\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"3b89dcfc622996ab97a5869dbff15cf51db00954f43a58a5e5f6b0470a132b2f4434bbcd405d2a9516151d2a6a04f2e4375bf48de1fdb21fb982afd9d2ea137c\0"
                    as *const u8 as *const libc::c_char,
                y: b"c813c4e2e2e0a8a391774c7903da7a6f14686e98e183e670ee6fb784809a3e92ca209dc631d85b1c7534ed3b37fddf64d854d7e01f91f18bb3fd307591afc051\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"a1ff1ab2712a267eb53935ddb5a567f84db156cc096168a1174291d5f488fba543d2840b4d2dd35d764b2f57b308907aec55cfba10544e8416e134687ccb87c3\0"
                    as *const u8 as *const libc::c_char,
                y: b"3cb5c4417ec4637f30374f189bb5b984c41e3a48d7f84fbfa3819e3f333f7eb311d3af7e67c4c16eeacfac2fe94c6dd4c6366f711a4fb6c7125cd7ec518d90d6\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ecc_ref_point {
                x: b"b7bfb80956c8670031ba191929f64e301d681634236d47a60e571a4bedc0ef257452ef78b5b98dbb3d9f3129d9349433ce2a3a35cb519c91e2d633d7b373ae16\0"
                    as *const u8 as *const libc::c_char,
                y: b"3bee95e29eecc5d5ad2beba941abcbf9f1cad478df0fecf614f63aeebef77850da7efdb93de8f3df80bc25eac09239c14175f5c29704ce9a3e383f1b3ec0e929\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ],
];
pub unsafe extern "C" fn test_ecc_ga(mut curve: libc::c_uint, mut p: *const mp_limb_t) {
    return test_ecc_point(
        ecc_curves[curve as usize],
        &*(*ecc_ref.as_ptr().offset(curve as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize),
        p,
    );
}
pub unsafe extern "C" fn test_ecc_mul_a(
    mut curve: libc::c_uint,
    mut n: libc::c_uint,
    mut p: *const mp_limb_t,
) {
    if curve < 9 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"curve < 9\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            2169 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void test_ecc_mul_a(unsigned int, unsigned int, const mp_limb_t *)\0"))
                .as_ptr(),
        );
    }
    'c_22001: {
        if curve < 9 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"curve < 9\0" as *const u8 as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                2169 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"void test_ecc_mul_a(unsigned int, unsigned int, const mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if n <= 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"n <= 4\0" as *const u8 as *const libc::c_char,
            b"testutils.c\0" as *const u8 as *const libc::c_char,
            2170 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void test_ecc_mul_a(unsigned int, unsigned int, const mp_limb_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21963: {
        if n <= 4 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"n <= 4\0" as *const u8 as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                2170 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"void test_ecc_mul_a(unsigned int, unsigned int, const mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if n == 0 as libc::c_int as libc::c_uint {
        let mut ecc: *const ecc_curve = ecc_curves[curve as usize];
        if (*ecc).p.bit_size as libc::c_int == 255 as libc::c_int
            || (*ecc).p.bit_size as libc::c_int == 448 as libc::c_int
        {} else {
            __assert_fail(
                b"ecc->p.bit_size == 255 || ecc->p.bit_size == 448\0" as *const u8
                    as *const libc::c_char,
                b"testutils.c\0" as *const u8 as *const libc::c_char,
                2175 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"void test_ecc_mul_a(unsigned int, unsigned int, const mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_21896: {
            if (*ecc).p.bit_size as libc::c_int == 255 as libc::c_int
                || (*ecc).p.bit_size as libc::c_int == 448 as libc::c_int
            {} else {
                __assert_fail(
                    b"ecc->p.bit_size == 255 || ecc->p.bit_size == 448\0" as *const u8
                        as *const libc::c_char,
                    b"testutils.c\0" as *const u8 as *const libc::c_char,
                    2175 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 67],
                        &[libc::c_char; 67],
                    >(
                        b"void test_ecc_mul_a(unsigned int, unsigned int, const mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if __gmpn_zero_p(p, (*ecc).p.size as mp_size_t) == 0
            || __gmpn_cmp(
                p.offset((*ecc).p.size as libc::c_int as isize),
                (*ecc).unit,
                (*ecc).p.size as mp_size_t,
            ) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"Incorrect point (expected (0, 1))!\ngot: x = \0" as *const u8
                    as *const libc::c_char,
            );
            write_mpn(stderr, 16 as libc::c_int, p, (*ecc).p.size as mp_size_t);
            fprintf(stderr, b"\n     y = \0" as *const u8 as *const libc::c_char);
            write_mpn(
                stderr,
                16 as libc::c_int,
                p.offset((*ecc).p.size as libc::c_int as isize),
                (*ecc).p.size as mp_size_t,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    } else {
        test_ecc_point(
            ecc_curves[curve as usize],
            &*(*ecc_ref.as_ptr().offset(curve as isize))
                .as_ptr()
                .offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize),
            p,
        );
    };
}
pub unsafe extern "C" fn test_ecc_mul_h(
    mut curve: libc::c_uint,
    mut n: libc::c_uint,
    mut p: *const mp_limb_t,
) {
    let mut ecc: *const ecc_curve = ecc_curves[curve as usize];
    let mut np: *mut mp_limb_t = xalloc_limbs(nettle_ecc_size_a(ecc));
    let mut scratch: *mut mp_limb_t = xalloc_limbs((*ecc).h_to_a_itch as mp_size_t);
    ((*ecc).h_to_a).unwrap()(ecc, 0 as libc::c_int, np, p, scratch);
    test_ecc_mul_a(curve, n, np);
    free(np as *mut libc::c_void);
    free(scratch as *mut libc::c_void);
}
pub unsafe extern "C" fn test_ecc_get_g(
    mut curve: libc::c_uint,
    mut rp: *mut mp_limb_t,
) {
    let mut ecc: *const ecc_curve = ecc_curves[curve as usize];
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
    __gmpz_init_set_str(
        x.as_mut_ptr(),
        ecc_ref[curve as usize][0 as libc::c_int as usize].x,
        16 as libc::c_int,
    );
    __gmpz_init_set_str(
        y.as_mut_ptr(),
        ecc_ref[curve as usize][0 as libc::c_int as usize].y,
        16 as libc::c_int,
    );
    if (*ecc).use_redc != 0 {
        let mut t: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_mul_2exp(
            x.as_mut_ptr(),
            x.as_mut_ptr() as mpz_srcptr,
            ((*ecc).p.size as libc::c_int * (64 as libc::c_int - 0 as libc::c_int))
                as mp_bitcnt_t,
        );
        __gmpz_mod(
            x.as_mut_ptr(),
            x.as_mut_ptr() as mpz_srcptr,
            __gmpz_roinit_n(t.as_mut_ptr(), (*ecc).p.m, (*ecc).p.size as mp_size_t),
        );
        __gmpz_mul_2exp(
            y.as_mut_ptr(),
            y.as_mut_ptr() as mpz_srcptr,
            ((*ecc).p.size as libc::c_int * (64 as libc::c_int - 0 as libc::c_int))
                as mp_bitcnt_t,
        );
        __gmpz_mod(
            y.as_mut_ptr(),
            y.as_mut_ptr() as mpz_srcptr,
            __gmpz_roinit_n(t.as_mut_ptr(), (*ecc).p.m, (*ecc).p.size as mp_size_t),
        );
    }
    _nettle_mpz_limbs_copy(rp, x.as_mut_ptr() as mpz_srcptr, (*ecc).p.size as mp_size_t);
    _nettle_mpz_limbs_copy(
        rp.offset((*ecc).p.size as libc::c_int as isize),
        y.as_mut_ptr() as mpz_srcptr,
        (*ecc).p.size as mp_size_t,
    );
    __gmpn_copyi(
        rp.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (*ecc).unit,
        (*ecc).p.size as mp_size_t,
    );
    __gmpz_clear(x.as_mut_ptr());
    __gmpz_clear(y.as_mut_ptr());
}
pub unsafe extern "C" fn test_ecc_get_ga(
    mut curve: libc::c_uint,
    mut rp: *mut mp_limb_t,
) {
    let mut ecc: *const ecc_curve = ecc_curves[curve as usize];
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
    __gmpz_init_set_str(
        x.as_mut_ptr(),
        ecc_ref[curve as usize][0 as libc::c_int as usize].x,
        16 as libc::c_int,
    );
    __gmpz_init_set_str(
        y.as_mut_ptr(),
        ecc_ref[curve as usize][0 as libc::c_int as usize].y,
        16 as libc::c_int,
    );
    _nettle_mpz_limbs_copy(rp, x.as_mut_ptr() as mpz_srcptr, (*ecc).p.size as mp_size_t);
    _nettle_mpz_limbs_copy(
        rp.offset((*ecc).p.size as libc::c_int as isize),
        y.as_mut_ptr() as mpz_srcptr,
        (*ecc).p.size as mp_size_t,
    );
    __gmpz_clear(x.as_mut_ptr());
    __gmpz_clear(y.as_mut_ptr());
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
