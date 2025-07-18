use ::libc;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn time_init();
    static mut time_start: Option::<unsafe extern "C" fn() -> ()>;
    static mut time_end: Option::<unsafe extern "C" fn() -> libc::c_double>;
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
    fn nettle_ctr_crypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn _nettle_ghash_update(
        ctx: *const gcm_key,
        state: *mut nettle_block16,
        blocks: size_t,
        data: *const uint8_t,
    ) -> *const uint8_t;
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn _nettle_salsa20_core(
        dst: *mut uint32_t,
        src: *const uint32_t,
        rounds: libc::c_uint,
    );
    fn nettle_sha1_compress(state: *mut uint32_t, data: *const uint8_t);
    fn nettle_sha3_permute(state: *mut sha3_state);
    fn nettle_umac32_set_key(ctx: *mut umac32_ctx, key: *const uint8_t);
    fn nettle_umac64_set_key(ctx: *mut umac64_ctx, key: *const uint8_t);
    fn nettle_umac96_set_key(ctx: *mut umac96_ctx, key: *const uint8_t);
    fn nettle_umac128_set_key(ctx: *mut umac128_ctx, key: *const uint8_t);
    fn nettle_umac32_update(ctx: *mut umac32_ctx, length: size_t, data: *const uint8_t);
    fn nettle_umac64_update(ctx: *mut umac64_ctx, length: size_t, data: *const uint8_t);
    fn nettle_umac96_update(ctx: *mut umac96_ctx, length: size_t, data: *const uint8_t);
    fn nettle_umac128_update(
        ctx: *mut umac128_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_cmac_aes128_set_key(ctx: *mut cmac_aes128_ctx, key: *const uint8_t);
    fn nettle_cmac_aes128_update(
        ctx: *mut cmac_aes128_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_poly1305_aes_set_key(ctx: *mut poly1305_aes_ctx, key: *const uint8_t);
    fn nettle_poly1305_aes_update(
        ctx: *mut poly1305_aes_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    static nettle_aes128: nettle_cipher;
    static nettle_aes192: nettle_cipher;
    static nettle_aes256: nettle_cipher;
    static nettle_camellia128: nettle_cipher;
    static nettle_camellia192: nettle_cipher;
    static nettle_camellia256: nettle_cipher;
    static nettle_cast128: nettle_cipher;
    static nettle_serpent256: nettle_cipher;
    static nettle_twofish128: nettle_cipher;
    static nettle_twofish192: nettle_cipher;
    static nettle_twofish256: nettle_cipher;
    static nettle_sm4: nettle_cipher;
    static nettle_md2: nettle_hash;
    static nettle_md4: nettle_hash;
    static nettle_md5: nettle_hash;
    static nettle_gosthash94: nettle_hash;
    static nettle_gosthash94cp: nettle_hash;
    static nettle_ripemd160: nettle_hash;
    static nettle_sha1: nettle_hash;
    static nettle_sha224: nettle_hash;
    static nettle_sha256: nettle_hash;
    static nettle_sha384: nettle_hash;
    static nettle_sha512: nettle_hash;
    static nettle_sha512_224: nettle_hash;
    static nettle_sha512_256: nettle_hash;
    static nettle_sha3_224: nettle_hash;
    static nettle_sha3_256: nettle_hash;
    static nettle_sha3_384: nettle_hash;
    static nettle_sha3_512: nettle_hash;
    static nettle_streebog256: nettle_hash;
    static nettle_streebog512: nettle_hash;
    static nettle_sm3: nettle_hash;
    static nettle_gcm_aes128: nettle_aead;
    static nettle_gcm_aes192: nettle_aead;
    static nettle_gcm_aes256: nettle_aead;
    static nettle_gcm_camellia128: nettle_aead;
    static nettle_gcm_camellia256: nettle_aead;
    static nettle_eax_aes128: nettle_aead;
    static nettle_chacha_poly1305: nettle_aead;
    fn nettle_hmac_md5_set_key(
        ctx: *mut hmac_md5_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_md5_update(
        ctx: *mut hmac_md5_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_md5_digest(
        ctx: *mut hmac_md5_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_hmac_sha1_set_key(
        ctx: *mut hmac_sha1_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_sha1_update(
        ctx: *mut hmac_sha1_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_sha1_digest(
        ctx: *mut hmac_sha1_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_hmac_sha256_set_key(
        ctx: *mut hmac_sha256_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_sha256_update(
        ctx: *mut hmac_sha256_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_sha256_digest(
        ctx: *mut hmac_sha256_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_hmac_sha512_set_key(
        ctx: *mut hmac_sha512_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_sha512_update(
        ctx: *mut hmac_sha512_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_sha512_digest(
        ctx: *mut hmac_sha512_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    static nettle_blowfish128: nettle_cipher;
    static nettle_des3: nettle_cipher;
    static nettle_des: nettle_cipher;
    static nettle_openssl_aes192: nettle_cipher;
    static nettle_cbc_aes192: nettle_aead;
    fn nettle_openssl_init();
    static nettle_arcfour128: nettle_aead;
    static nettle_openssl_gcm_aes256: nettle_aead;
    static nettle_chacha: nettle_aead;
    static nettle_salsa20: nettle_aead;
    static nettle_salsa20r12: nettle_aead;
    static nettle_openssl_gcm_aes192: nettle_aead;
    static nettle_cbc_aes128: nettle_aead;
    static nettle_openssl_gcm_aes128: nettle_aead;
    static nettle_openssl_aes128: nettle_cipher;
    static nettle_cbc_aes256: nettle_aead;
    static nettle_openssl_aes256: nettle_cipher;
    static nettle_openssl_blowfish128: nettle_cipher;
    static nettle_openssl_des: nettle_cipher;
    static nettle_openssl_cast128: nettle_cipher;
    static nettle_openssl_md5: nettle_hash;
    static nettle_openssl_sha1: nettle_hash;
    static nettle_ocb_aes128: nettle_aead;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
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
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
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
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct umac32_ctx {
    pub l1_key: [uint32_t; 256],
    pub l2_key: [uint32_t; 6],
    pub l3_key1: [uint64_t; 8],
    pub l3_key2: [uint32_t; 1],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 3],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub nonce_low: libc::c_ushort,
    pub pad_cache: [uint32_t; 4],
    pub index: libc::c_uint,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct umac64_ctx {
    pub l1_key: [uint32_t; 260],
    pub l2_key: [uint32_t; 12],
    pub l3_key1: [uint64_t; 16],
    pub l3_key2: [uint32_t; 2],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 6],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub nonce_low: libc::c_ushort,
    pub pad_cache: [uint32_t; 4],
    pub index: libc::c_uint,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct umac96_ctx {
    pub l1_key: [uint32_t; 264],
    pub l2_key: [uint32_t; 18],
    pub l3_key1: [uint64_t; 24],
    pub l3_key2: [uint32_t; 3],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 9],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub index: libc::c_uint,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct umac128_ctx {
    pub l1_key: [uint32_t; 268],
    pub l2_key: [uint32_t; 24],
    pub l3_key1: [uint64_t; 32],
    pub l3_key2: [uint32_t; 4],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 12],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub index: libc::c_uint,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_ctx {
    pub X: nettle_block16,
    pub block: nettle_block16,
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac_aes128_ctx {
    pub key: cmac128_key,
    pub ctx: cmac128_ctx,
    pub cipher: aes128_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly1305_ctx {
    pub r: C2RustUnnamed_0,
    pub s32: [uint32_t; 3],
    pub hh: uint32_t,
    pub h: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub h32: [uint32_t; 4],
    pub h64: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r32: [uint32_t; 6],
    pub r64: [uint64_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly1305_aes_ctx {
    pub pctx: poly1305_ctx,
    pub block: [uint8_t; 16],
    pub index: libc::c_uint,
    pub nonce: [uint8_t; 16],
    pub aes: aes128_ctx,
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
pub struct md5_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_md5_ctx {
    pub outer: md5_ctx,
    pub inner: md5_ctx,
    pub state: md5_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha1_ctx {
    pub outer: sha1_ctx,
    pub inner: sha1_ctx,
    pub state: sha1_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha256_ctx {
    pub outer: sha256_ctx,
    pub inner: sha256_ctx,
    pub state: sha256_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha512_ctx {
    pub outer: sha512_ctx,
    pub inner: sha512_ctx,
    pub state: sha512_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bench_memxor_info {
    pub dst: *mut libc::c_void,
    pub src: *const libc::c_void,
    pub other: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bench_hash_info {
    pub ctx: *mut libc::c_void,
    pub update: Option::<nettle_hash_update_func>,
    pub data: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bench_cipher_info {
    pub ctx: *mut libc::c_void,
    pub crypt: Option::<nettle_cipher_func>,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bench_cbc_info {
    pub ctx: *mut libc::c_void,
    pub crypt: Option::<nettle_cipher_func>,
    pub src: *const uint8_t,
    pub dst: *mut uint8_t,
    pub block_size: libc::c_uint,
    pub iv: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bench_aead_info {
    pub ctx: *mut libc::c_void,
    pub crypt: Option::<nettle_crypt_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bench_hmac_info {
    pub ctx: *mut libc::c_void,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
    pub length: size_t,
    pub digest_length: size_t,
    pub data: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: size_t,
    pub msg: *const libc::c_char,
}
pub const OPT_HELP: C2RustUnnamed_2 = 300;
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut frequency: libc::c_double = 0.0f64;
unsafe extern "C" fn die(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    exit(1 as libc::c_int);
}
static mut overhead: libc::c_double = 0.0f64;
unsafe extern "C" fn time_function(
    mut f: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> libc::c_double {
    let mut ncalls: libc::c_uint = 0;
    let mut elapsed: libc::c_double = 0.;
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
    return elapsed / ncalls as libc::c_double - overhead;
}
unsafe extern "C" fn bench_memxor(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_memxor_info = arg as *mut bench_memxor_info;
    nettle_memxor((*info).dst, (*info).src, 10240 as libc::c_int as size_t);
}
unsafe extern "C" fn bench_memxor3(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_memxor_info = arg as *mut bench_memxor_info;
    nettle_memxor3(
        (*info).dst,
        (*info).src,
        (*info).other,
        10240 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn bench_hash(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_hash_info = arg as *mut bench_hash_info;
    ((*info).update).unwrap()((*info).ctx, 10240 as libc::c_int as size_t, (*info).data);
}
unsafe extern "C" fn bench_cipher(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_cipher_info = arg as *mut bench_cipher_info;
    ((*info).crypt)
        .unwrap()(
        (*info).ctx,
        10240 as libc::c_int as size_t,
        (*info).data,
        (*info).data,
    );
}
unsafe extern "C" fn bench_cbc_encrypt(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_cbc_info = arg as *mut bench_cbc_info;
    nettle_cbc_encrypt(
        (*info).ctx,
        (*info).crypt,
        (*info).block_size as size_t,
        (*info).iv,
        10240 as libc::c_int as size_t,
        (*info).dst,
        (*info).src,
    );
}
unsafe extern "C" fn bench_cbc_decrypt(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_cbc_info = arg as *mut bench_cbc_info;
    nettle_cbc_decrypt(
        (*info).ctx,
        (*info).crypt,
        (*info).block_size as size_t,
        (*info).iv,
        10240 as libc::c_int as size_t,
        (*info).dst,
        (*info).src,
    );
}
unsafe extern "C" fn bench_ctr(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_cbc_info = arg as *mut bench_cbc_info;
    nettle_ctr_crypt(
        (*info).ctx,
        (*info).crypt,
        (*info).block_size as size_t,
        (*info).iv,
        10240 as libc::c_int as size_t,
        (*info).dst,
        (*info).src,
    );
}
unsafe extern "C" fn bench_aead_crypt(mut arg: *mut libc::c_void) {
    let mut info: *const bench_aead_info = arg as *const bench_aead_info;
    ((*info).crypt)
        .unwrap()(
        (*info).ctx,
        10240 as libc::c_int as size_t,
        (*info).data,
        (*info).data,
    );
}
unsafe extern "C" fn bench_aead_update(mut arg: *mut libc::c_void) {
    let mut info: *const bench_aead_info = arg as *const bench_aead_info;
    ((*info).update).unwrap()((*info).ctx, 10240 as libc::c_int as size_t, (*info).data);
}
unsafe extern "C" fn init_data(mut data: *mut uint8_t) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    j = 0 as libc::c_int as libc::c_uint;
    i = j;
    while i < 10240 as libc::c_int as libc::c_uint {
        if j.wrapping_mul(j) < i {
            j = j.wrapping_add(1);
            j;
        }
        *data.offset(i as isize) = j as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn init_key(mut length: libc::c_uint, mut key: *mut uint8_t) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        *key.offset(i as isize) = i as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn init_nonce(mut length: libc::c_uint, mut nonce: *mut uint8_t) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        *nonce
            .offset(
                i as isize,
            ) = (3 as libc::c_int as libc::c_uint).wrapping_mul(i) as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn header() {
    printf(
        b"%18s %12s Mbyte/s%s\n\0" as *const u8 as *const libc::c_char,
        b"Algorithm\0" as *const u8 as *const libc::c_char,
        b"mode\0" as *const u8 as *const libc::c_char,
        if frequency > 0.0f64 {
            b" cycles/byte cycles/block\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn display(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut block_size: libc::c_uint,
    mut time: libc::c_double,
) {
    printf(
        b"%18s %12s %7.2f\0" as *const u8 as *const libc::c_char,
        name,
        mode,
        10240 as libc::c_int as libc::c_double / (time * 1048576.0f64),
    );
    if frequency > 0.0f64 {
        printf(
            b" %11.2f\0" as *const u8 as *const libc::c_char,
            time * frequency / 10240 as libc::c_int as libc::c_double,
        );
        if block_size > 0 as libc::c_int as libc::c_uint {
            printf(
                b" %12.2f\0" as *const u8 as *const libc::c_char,
                time * frequency * block_size as libc::c_double
                    / 10240 as libc::c_int as libc::c_double,
            );
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if p.is_null() {
        die(b"Virtual memory exhausted.\n\0" as *const u8 as *const libc::c_char);
    }
    return p;
}
unsafe extern "C" fn time_memxor() {
    let mut info: bench_memxor_info = bench_memxor_info {
        dst: 0 as *mut libc::c_void,
        src: 0 as *const libc::c_void,
        other: 0 as *const libc::c_void,
    };
    let mut src: [libc::c_ulong; 1282] = [0; 1282];
    let mut other: [libc::c_ulong; 1282] = [0; 1282];
    let mut dst: [libc::c_ulong; 1281] = [0; 1281];
    info.src = src.as_mut_ptr() as *const libc::c_void;
    info.dst = dst.as_mut_ptr() as *mut libc::c_void;
    display(
        b"memxor\0" as *const u8 as *const libc::c_char,
        b"aligned\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_uint,
        time_function(
            Some(bench_memxor as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_memxor_info as *mut libc::c_void,
        ),
    );
    info
        .src = (src.as_mut_ptr() as *const libc::c_char)
        .offset(1 as libc::c_int as isize) as *const libc::c_void;
    display(
        b"memxor\0" as *const u8 as *const libc::c_char,
        b"unaligned\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_uint,
        time_function(
            Some(bench_memxor as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_memxor_info as *mut libc::c_void,
        ),
    );
    info.src = src.as_mut_ptr() as *const libc::c_void;
    info.other = other.as_mut_ptr() as *const libc::c_void;
    display(
        b"memxor3\0" as *const u8 as *const libc::c_char,
        b"aligned\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_uint,
        time_function(
            Some(bench_memxor3 as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_memxor_info as *mut libc::c_void,
        ),
    );
    info
        .other = (other.as_mut_ptr() as *const libc::c_char)
        .offset(1 as libc::c_int as isize) as *const libc::c_void;
    display(
        b"memxor3\0" as *const u8 as *const libc::c_char,
        b"unaligned01\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_uint,
        time_function(
            Some(bench_memxor3 as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_memxor_info as *mut libc::c_void,
        ),
    );
    info
        .src = (src.as_mut_ptr() as *const libc::c_char)
        .offset(1 as libc::c_int as isize) as *const libc::c_void;
    display(
        b"memxor3\0" as *const u8 as *const libc::c_char,
        b"unaligned11\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_uint,
        time_function(
            Some(bench_memxor3 as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_memxor_info as *mut libc::c_void,
        ),
    );
    info
        .other = (other.as_mut_ptr() as *const libc::c_char)
        .offset(2 as libc::c_int as isize) as *const libc::c_void;
    display(
        b"memxor3\0" as *const u8 as *const libc::c_char,
        b"unaligned12\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_uint,
        time_function(
            Some(bench_memxor3 as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_memxor_info as *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn time_hash(mut hash: *const nettle_hash) {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hash_info = bench_hash_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        data: 0 as *const uint8_t,
    };
    info.ctx = xalloc((*hash).context_size as size_t);
    info.update = (*hash).update;
    info.data = data.as_mut_ptr();
    init_data(data.as_mut_ptr());
    ((*hash).init).unwrap()(info.ctx);
    display(
        (*hash).name,
        b"update\0" as *const u8 as *const libc::c_char,
        (*hash).block_size,
        time_function(
            Some(bench_hash as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_hash_info as *mut libc::c_void,
        ),
    );
    free(info.ctx);
}
unsafe extern "C" fn time_umac() {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hash_info = bench_hash_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        data: 0 as *const uint8_t,
    };
    let mut ctx32: umac32_ctx = umac32_ctx {
        l1_key: [0; 256],
        l2_key: [0; 6],
        l3_key1: [0; 8],
        l3_key2: [0; 1],
        pdf_key: aes128_ctx { keys: [0; 44] },
        l2_state: [0; 3],
        nonce: [0; 16],
        nonce_length: 0,
        nonce_low: 0,
        pad_cache: [0; 4],
        index: 0,
        count: 0,
        block: [0; 1024],
    };
    let mut ctx64: umac64_ctx = umac64_ctx {
        l1_key: [0; 260],
        l2_key: [0; 12],
        l3_key1: [0; 16],
        l3_key2: [0; 2],
        pdf_key: aes128_ctx { keys: [0; 44] },
        l2_state: [0; 6],
        nonce: [0; 16],
        nonce_length: 0,
        nonce_low: 0,
        pad_cache: [0; 4],
        index: 0,
        count: 0,
        block: [0; 1024],
    };
    let mut ctx96: umac96_ctx = umac96_ctx {
        l1_key: [0; 264],
        l2_key: [0; 18],
        l3_key1: [0; 24],
        l3_key2: [0; 3],
        pdf_key: aes128_ctx { keys: [0; 44] },
        l2_state: [0; 9],
        nonce: [0; 16],
        nonce_length: 0,
        index: 0,
        count: 0,
        block: [0; 1024],
    };
    let mut ctx128: umac128_ctx = umac128_ctx {
        l1_key: [0; 268],
        l2_key: [0; 24],
        l3_key1: [0; 32],
        l3_key2: [0; 4],
        pdf_key: aes128_ctx { keys: [0; 44] },
        l2_state: [0; 12],
        nonce: [0; 16],
        nonce_length: 0,
        index: 0,
        count: 0,
        block: [0; 1024],
    };
    let mut key: [uint8_t; 16] = [0; 16];
    init_key(
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong as libc::c_uint,
        key.as_mut_ptr(),
    );
    nettle_umac32_set_key(&mut ctx32, key.as_mut_ptr());
    info.ctx = &mut ctx32 as *mut umac32_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut umac32_ctx, size_t, *const uint8_t) -> ()>,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_umac32_update
                as unsafe extern "C" fn(*mut umac32_ctx, size_t, *const uint8_t) -> (),
        ),
    );
    info.data = data.as_mut_ptr();
    display(
        b"umac32\0" as *const u8 as *const libc::c_char,
        b"update\0" as *const u8 as *const libc::c_char,
        1024 as libc::c_int as libc::c_uint,
        time_function(
            Some(bench_hash as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_hash_info as *mut libc::c_void,
        ),
    );
    nettle_umac64_set_key(&mut ctx64, key.as_mut_ptr());
    info.ctx = &mut ctx64 as *mut umac64_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut umac64_ctx, size_t, *const uint8_t) -> ()>,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_umac64_update
                as unsafe extern "C" fn(*mut umac64_ctx, size_t, *const uint8_t) -> (),
        ),
    );
    info.data = data.as_mut_ptr();
    display(
        b"umac64\0" as *const u8 as *const libc::c_char,
        b"update\0" as *const u8 as *const libc::c_char,
        1024 as libc::c_int as libc::c_uint,
        time_function(
            Some(bench_hash as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_hash_info as *mut libc::c_void,
        ),
    );
    nettle_umac96_set_key(&mut ctx96, key.as_mut_ptr());
    info.ctx = &mut ctx96 as *mut umac96_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut umac96_ctx, size_t, *const uint8_t) -> ()>,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_umac96_update
                as unsafe extern "C" fn(*mut umac96_ctx, size_t, *const uint8_t) -> (),
        ),
    );
    info.data = data.as_mut_ptr();
    display(
        b"umac96\0" as *const u8 as *const libc::c_char,
        b"update\0" as *const u8 as *const libc::c_char,
        1024 as libc::c_int as libc::c_uint,
        time_function(
            Some(bench_hash as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_hash_info as *mut libc::c_void,
        ),
    );
    nettle_umac128_set_key(&mut ctx128, key.as_mut_ptr());
    info.ctx = &mut ctx128 as *mut umac128_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut umac128_ctx, size_t, *const uint8_t) -> ()>,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_umac128_update
                as unsafe extern "C" fn(*mut umac128_ctx, size_t, *const uint8_t) -> (),
        ),
    );
    info.data = data.as_mut_ptr();
    display(
        b"umac128\0" as *const u8 as *const libc::c_char,
        b"update\0" as *const u8 as *const libc::c_char,
        1024 as libc::c_int as libc::c_uint,
        time_function(
            Some(bench_hash as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_hash_info as *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn time_cmac() {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hash_info = bench_hash_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        data: 0 as *const uint8_t,
    };
    let mut ctx: cmac_aes128_ctx = cmac_aes128_ctx {
        key: cmac128_key {
            K1: nettle_block16 { b: [0; 16] },
            K2: nettle_block16 { b: [0; 16] },
        },
        ctx: cmac128_ctx {
            X: nettle_block16 { b: [0; 16] },
            block: nettle_block16 { b: [0; 16] },
            index: 0,
        },
        cipher: aes128_ctx { keys: [0; 44] },
    };
    let mut key: [uint8_t; 16] = [0; 16];
    init_key(
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong as libc::c_uint,
        key.as_mut_ptr(),
    );
    nettle_cmac_aes128_set_key(&mut ctx, key.as_mut_ptr());
    info.ctx = &mut ctx as *mut cmac_aes128_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut cmac_aes128_ctx, size_t, *const uint8_t) -> (),
        >,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_cmac_aes128_update
                as unsafe extern "C" fn(
                    *mut cmac_aes128_ctx,
                    size_t,
                    *const uint8_t,
                ) -> (),
        ),
    );
    info.data = data.as_mut_ptr();
    display(
        b"cmac-aes128\0" as *const u8 as *const libc::c_char,
        b"update\0" as *const u8 as *const libc::c_char,
        16 as libc::c_int as libc::c_uint,
        time_function(
            Some(bench_hash as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_hash_info as *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn time_poly1305_aes() {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hash_info = bench_hash_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        data: 0 as *const uint8_t,
    };
    let mut ctx: poly1305_aes_ctx = poly1305_aes_ctx {
        pctx: poly1305_ctx {
            r: C2RustUnnamed_0 { r32: [0; 6] },
            s32: [0; 3],
            hh: 0,
            h: C2RustUnnamed { h32: [0; 4] },
        },
        block: [0; 16],
        index: 0,
        nonce: [0; 16],
        aes: aes128_ctx { keys: [0; 44] },
    };
    let mut key: [uint8_t; 32] = [0; 32];
    init_key(
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong as libc::c_uint,
        key.as_mut_ptr(),
    );
    nettle_poly1305_aes_set_key(&mut ctx, key.as_mut_ptr());
    info.ctx = &mut ctx as *mut poly1305_aes_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut poly1305_aes_ctx, size_t, *const uint8_t) -> (),
        >,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_poly1305_aes_update
                as unsafe extern "C" fn(
                    *mut poly1305_aes_ctx,
                    size_t,
                    *const uint8_t,
                ) -> (),
        ),
    );
    info.data = data.as_mut_ptr();
    display(
        b"poly1305-aes\0" as *const u8 as *const libc::c_char,
        b"update\0" as *const u8 as *const libc::c_char,
        1024 as libc::c_int as libc::c_uint,
        time_function(
            Some(bench_hash as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_hash_info as *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn bench_hmac(mut arg: *mut libc::c_void) {
    let mut info: *mut bench_hmac_info = arg as *mut bench_hmac_info;
    let mut digest: [uint8_t; 64] = [0; 64];
    let mut pos: size_t = 0;
    let mut length: size_t = 0;
    length = (*info).length;
    pos = 0 as libc::c_int as size_t;
    while pos < 10240 as libc::c_int as libc::c_ulong {
        let mut single: size_t = if pos.wrapping_add(length)
            < 10240 as libc::c_int as libc::c_ulong
        {
            length
        } else {
            (10240 as libc::c_int as libc::c_ulong).wrapping_sub(pos)
        };
        ((*info).update)
            .unwrap()((*info).ctx, single, ((*info).data).offset(pos as isize));
        ((*info).digest)
            .unwrap()((*info).ctx, (*info).digest_length, digest.as_mut_ptr());
        pos = (pos as libc::c_ulong).wrapping_add(length) as size_t as size_t;
    }
}
static mut hmac_tests: [C2RustUnnamed_1; 6] = [
    {
        let mut init = C2RustUnnamed_1 {
            length: 64 as libc::c_int as size_t,
            msg: b"64 bytes\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            length: 256 as libc::c_int as size_t,
            msg: b"256 bytes\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            length: 1024 as libc::c_int as size_t,
            msg: b"1024 bytes\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            length: 4096 as libc::c_int as size_t,
            msg: b"4096 bytes\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            length: 10240 as libc::c_int as size_t,
            msg: b"single msg\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            length: 0 as libc::c_int as size_t,
            msg: 0 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn time_hmac_md5() {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hmac_info = bench_hmac_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        digest: None,
        length: 0,
        digest_length: 0,
        data: 0 as *const uint8_t,
    };
    let mut md5_ctx: hmac_md5_ctx = hmac_md5_ctx {
        outer: md5_ctx {
            state: [0; 4],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        inner: md5_ctx {
            state: [0; 4],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        state: md5_ctx {
            state: [0; 4],
            count: 0,
            index: 0,
            block: [0; 64],
        },
    };
    let mut pos: libc::c_uint = 0;
    init_data(data.as_mut_ptr());
    info.data = data.as_mut_ptr();
    nettle_hmac_md5_set_key(
        &mut md5_ctx,
        64 as libc::c_int as size_t,
        data.as_mut_ptr(),
    );
    info.ctx = &mut md5_ctx as *mut hmac_md5_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut hmac_md5_ctx, size_t, *const uint8_t) -> ()>,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_hmac_md5_update
                as unsafe extern "C" fn(*mut hmac_md5_ctx, size_t, *const uint8_t) -> (),
        ),
    );
    info
        .digest = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut hmac_md5_ctx, size_t, *mut uint8_t) -> ()>,
        Option::<nettle_hash_digest_func>,
    >(
        Some(
            nettle_hmac_md5_digest
                as unsafe extern "C" fn(*mut hmac_md5_ctx, size_t, *mut uint8_t) -> (),
        ),
    );
    info.digest_length = 16 as libc::c_int as size_t;
    pos = 0 as libc::c_int as libc::c_uint;
    while hmac_tests[pos as usize].length != 0 as libc::c_int as libc::c_ulong {
        info.length = hmac_tests[pos as usize].length;
        display(
            b"hmac-md5\0" as *const u8 as *const libc::c_char,
            hmac_tests[pos as usize].msg,
            64 as libc::c_int as libc::c_uint,
            time_function(
                Some(bench_hmac as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info as *mut bench_hmac_info as *mut libc::c_void,
            ),
        );
        pos = pos.wrapping_add(1);
        pos;
    }
}
unsafe extern "C" fn time_hmac_sha1() {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hmac_info = bench_hmac_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        digest: None,
        length: 0,
        digest_length: 0,
        data: 0 as *const uint8_t,
    };
    let mut sha1_ctx: hmac_sha1_ctx = hmac_sha1_ctx {
        outer: sha1_ctx {
            state: [0; 5],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        inner: sha1_ctx {
            state: [0; 5],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        state: sha1_ctx {
            state: [0; 5],
            count: 0,
            index: 0,
            block: [0; 64],
        },
    };
    let mut pos: libc::c_uint = 0;
    init_data(data.as_mut_ptr());
    info.data = data.as_mut_ptr();
    nettle_hmac_sha1_set_key(
        &mut sha1_ctx,
        64 as libc::c_int as size_t,
        data.as_mut_ptr(),
    );
    info.ctx = &mut sha1_ctx as *mut hmac_sha1_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut hmac_sha1_ctx, size_t, *const uint8_t) -> ()>,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_hmac_sha1_update
                as unsafe extern "C" fn(*mut hmac_sha1_ctx, size_t, *const uint8_t) -> (),
        ),
    );
    info
        .digest = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut hmac_sha1_ctx, size_t, *mut uint8_t) -> ()>,
        Option::<nettle_hash_digest_func>,
    >(
        Some(
            nettle_hmac_sha1_digest
                as unsafe extern "C" fn(*mut hmac_sha1_ctx, size_t, *mut uint8_t) -> (),
        ),
    );
    info.digest_length = 20 as libc::c_int as size_t;
    pos = 0 as libc::c_int as libc::c_uint;
    while hmac_tests[pos as usize].length != 0 as libc::c_int as libc::c_ulong {
        info.length = hmac_tests[pos as usize].length;
        display(
            b"hmac-sha1\0" as *const u8 as *const libc::c_char,
            hmac_tests[pos as usize].msg,
            64 as libc::c_int as libc::c_uint,
            time_function(
                Some(bench_hmac as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info as *mut bench_hmac_info as *mut libc::c_void,
            ),
        );
        pos = pos.wrapping_add(1);
        pos;
    }
}
unsafe extern "C" fn time_hmac_sha256() {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hmac_info = bench_hmac_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        digest: None,
        length: 0,
        digest_length: 0,
        data: 0 as *const uint8_t,
    };
    let mut sha256_ctx: hmac_sha256_ctx = hmac_sha256_ctx {
        outer: sha256_ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        inner: sha256_ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        state: sha256_ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        },
    };
    let mut pos: libc::c_uint = 0;
    init_data(data.as_mut_ptr());
    info.data = data.as_mut_ptr();
    nettle_hmac_sha256_set_key(
        &mut sha256_ctx,
        64 as libc::c_int as size_t,
        data.as_mut_ptr(),
    );
    info.ctx = &mut sha256_ctx as *mut hmac_sha256_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut hmac_sha256_ctx, size_t, *const uint8_t) -> (),
        >,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_hmac_sha256_update
                as unsafe extern "C" fn(
                    *mut hmac_sha256_ctx,
                    size_t,
                    *const uint8_t,
                ) -> (),
        ),
    );
    info
        .digest = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut hmac_sha256_ctx, size_t, *mut uint8_t) -> ()>,
        Option::<nettle_hash_digest_func>,
    >(
        Some(
            nettle_hmac_sha256_digest
                as unsafe extern "C" fn(*mut hmac_sha256_ctx, size_t, *mut uint8_t) -> (),
        ),
    );
    info.digest_length = 32 as libc::c_int as size_t;
    pos = 0 as libc::c_int as libc::c_uint;
    while hmac_tests[pos as usize].length != 0 as libc::c_int as libc::c_ulong {
        info.length = hmac_tests[pos as usize].length;
        display(
            b"hmac-sha256\0" as *const u8 as *const libc::c_char,
            hmac_tests[pos as usize].msg,
            64 as libc::c_int as libc::c_uint,
            time_function(
                Some(bench_hmac as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info as *mut bench_hmac_info as *mut libc::c_void,
            ),
        );
        pos = pos.wrapping_add(1);
        pos;
    }
}
unsafe extern "C" fn time_hmac_sha512() {
    static mut data: [uint8_t; 10240] = [0; 10240];
    let mut info: bench_hmac_info = bench_hmac_info {
        ctx: 0 as *mut libc::c_void,
        update: None,
        digest: None,
        length: 0,
        digest_length: 0,
        data: 0 as *const uint8_t,
    };
    let mut sha512_ctx: hmac_sha512_ctx = hmac_sha512_ctx {
        outer: sha512_ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; 128],
        },
        inner: sha512_ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; 128],
        },
        state: sha512_ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; 128],
        },
    };
    let mut pos: libc::c_uint = 0;
    init_data(data.as_mut_ptr());
    info.data = data.as_mut_ptr();
    nettle_hmac_sha512_set_key(
        &mut sha512_ctx,
        128 as libc::c_int as size_t,
        data.as_mut_ptr(),
    );
    info.ctx = &mut sha512_ctx as *mut hmac_sha512_ctx as *mut libc::c_void;
    info
        .update = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut hmac_sha512_ctx, size_t, *const uint8_t) -> (),
        >,
        Option::<nettle_hash_update_func>,
    >(
        Some(
            nettle_hmac_sha512_update
                as unsafe extern "C" fn(
                    *mut hmac_sha512_ctx,
                    size_t,
                    *const uint8_t,
                ) -> (),
        ),
    );
    info
        .digest = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut hmac_sha512_ctx, size_t, *mut uint8_t) -> ()>,
        Option::<nettle_hash_digest_func>,
    >(
        Some(
            nettle_hmac_sha512_digest
                as unsafe extern "C" fn(*mut hmac_sha512_ctx, size_t, *mut uint8_t) -> (),
        ),
    );
    info.digest_length = 64 as libc::c_int as size_t;
    pos = 0 as libc::c_int as libc::c_uint;
    while hmac_tests[pos as usize].length != 0 as libc::c_int as libc::c_ulong {
        info.length = hmac_tests[pos as usize].length;
        display(
            b"hmac-sha512\0" as *const u8 as *const libc::c_char,
            hmac_tests[pos as usize].msg,
            128 as libc::c_int as libc::c_uint,
            time_function(
                Some(bench_hmac as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info as *mut bench_hmac_info as *mut libc::c_void,
            ),
        );
        pos = pos.wrapping_add(1);
        pos;
    }
}
unsafe extern "C" fn prefix_p(
    mut prefix: *const libc::c_char,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while *prefix.offset(i as isize) != 0 {
        if *prefix.offset(i as isize) as libc::c_int
            != *s.offset(i as isize) as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn block_cipher_p(mut cipher: *const nettle_cipher) -> libc::c_int {
    return ((*cipher).block_size > 0 as libc::c_int as libc::c_uint
        && prefix_p(b"openssl\0" as *const u8 as *const libc::c_char, (*cipher).name)
            == 0) as libc::c_int;
}
unsafe extern "C" fn time_cipher(mut cipher: *const nettle_cipher) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut key: *mut uint8_t = xalloc((*cipher).key_size as size_t) as *mut uint8_t;
    static mut src_data: [uint8_t; 10240] = [0; 10240];
    static mut data: [uint8_t; 10240] = [0; 10240];
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    init_data(data.as_mut_ptr());
    init_data(src_data.as_mut_ptr());
    let mut info: bench_cipher_info = bench_cipher_info {
        ctx: 0 as *mut libc::c_void,
        crypt: None,
        data: 0 as *mut uint8_t,
    };
    info.ctx = ctx;
    info.crypt = (*cipher).encrypt;
    info.data = data.as_mut_ptr();
    init_key((*cipher).key_size, key);
    ((*cipher).set_encrypt_key).unwrap()(ctx, key);
    display(
        (*cipher).name,
        b"ECB encrypt\0" as *const u8 as *const libc::c_char,
        (*cipher).block_size,
        time_function(
            Some(bench_cipher as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_cipher_info as *mut libc::c_void,
        ),
    );
    let mut info_0: bench_cipher_info = bench_cipher_info {
        ctx: 0 as *mut libc::c_void,
        crypt: None,
        data: 0 as *mut uint8_t,
    };
    info_0.ctx = ctx;
    info_0.crypt = (*cipher).decrypt;
    info_0.data = data.as_mut_ptr();
    init_key((*cipher).key_size, key);
    ((*cipher).set_decrypt_key).unwrap()(ctx, key);
    display(
        (*cipher).name,
        b"ECB decrypt\0" as *const u8 as *const libc::c_char,
        (*cipher).block_size,
        time_function(
            Some(bench_cipher as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info_0 as *mut bench_cipher_info as *mut libc::c_void,
        ),
    );
    if block_cipher_p(cipher) != 0 {
        let mut iv: *mut uint8_t = xalloc((*cipher).block_size as size_t)
            as *mut uint8_t;
        let mut info_1: bench_cbc_info = bench_cbc_info {
            ctx: 0 as *mut libc::c_void,
            crypt: None,
            src: 0 as *const uint8_t,
            dst: 0 as *mut uint8_t,
            block_size: 0,
            iv: 0 as *mut uint8_t,
        };
        info_1.ctx = ctx;
        info_1.crypt = (*cipher).encrypt;
        info_1.src = src_data.as_mut_ptr();
        info_1.dst = data.as_mut_ptr();
        info_1.block_size = (*cipher).block_size;
        info_1.iv = iv;
        memset(
            iv as *mut libc::c_void,
            0 as libc::c_int,
            (*cipher).block_size as libc::c_ulong,
        );
        ((*cipher).set_encrypt_key).unwrap()(ctx, key);
        display(
            (*cipher).name,
            b"CBC encrypt\0" as *const u8 as *const libc::c_char,
            (*cipher).block_size,
            time_function(
                Some(bench_cbc_encrypt as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info_1 as *mut bench_cbc_info as *mut libc::c_void,
            ),
        );
        let mut info_2: bench_cbc_info = bench_cbc_info {
            ctx: 0 as *mut libc::c_void,
            crypt: None,
            src: 0 as *const uint8_t,
            dst: 0 as *mut uint8_t,
            block_size: 0,
            iv: 0 as *mut uint8_t,
        };
        info_2.ctx = ctx;
        info_2.crypt = (*cipher).decrypt;
        info_2.src = src_data.as_mut_ptr();
        info_2.dst = data.as_mut_ptr();
        info_2.block_size = (*cipher).block_size;
        info_2.iv = iv;
        memset(
            iv as *mut libc::c_void,
            0 as libc::c_int,
            (*cipher).block_size as libc::c_ulong,
        );
        ((*cipher).set_decrypt_key).unwrap()(ctx, key);
        display(
            (*cipher).name,
            b"CBC decrypt\0" as *const u8 as *const libc::c_char,
            (*cipher).block_size,
            time_function(
                Some(bench_cbc_decrypt as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info_2 as *mut bench_cbc_info as *mut libc::c_void,
            ),
        );
        memset(
            iv as *mut libc::c_void,
            0 as libc::c_int,
            (*cipher).block_size as libc::c_ulong,
        );
        info_2.src = data.as_mut_ptr();
        display(
            (*cipher).name,
            b"  (in-place)\0" as *const u8 as *const libc::c_char,
            (*cipher).block_size,
            time_function(
                Some(bench_cbc_decrypt as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info_2 as *mut bench_cbc_info as *mut libc::c_void,
            ),
        );
        let mut info_3: bench_cbc_info = bench_cbc_info {
            ctx: 0 as *mut libc::c_void,
            crypt: None,
            src: 0 as *const uint8_t,
            dst: 0 as *mut uint8_t,
            block_size: 0,
            iv: 0 as *mut uint8_t,
        };
        info_3.ctx = ctx;
        info_3.crypt = (*cipher).encrypt;
        info_3.src = src_data.as_mut_ptr();
        info_3.dst = data.as_mut_ptr();
        info_3.block_size = (*cipher).block_size;
        info_3.iv = iv;
        memset(
            iv as *mut libc::c_void,
            0 as libc::c_int,
            (*cipher).block_size as libc::c_ulong,
        );
        ((*cipher).set_encrypt_key).unwrap()(ctx, key);
        display(
            (*cipher).name,
            b"CTR\0" as *const u8 as *const libc::c_char,
            (*cipher).block_size,
            time_function(
                Some(bench_ctr as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info_3 as *mut bench_cbc_info as *mut libc::c_void,
            ),
        );
        memset(
            iv as *mut libc::c_void,
            0 as libc::c_int,
            (*cipher).block_size as libc::c_ulong,
        );
        info_3.src = data.as_mut_ptr();
        display(
            (*cipher).name,
            b"  (in-place)\0" as *const u8 as *const libc::c_char,
            (*cipher).block_size,
            time_function(
                Some(bench_ctr as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info_3 as *mut bench_cbc_info as *mut libc::c_void,
            ),
        );
        free(iv as *mut libc::c_void);
    }
    free(ctx);
    free(key as *mut libc::c_void);
}
unsafe extern "C" fn time_aead(mut aead: *const nettle_aead) {
    let mut ctx: *mut libc::c_void = xalloc((*aead).context_size as size_t);
    let mut key: *mut uint8_t = xalloc((*aead).key_size as size_t) as *mut uint8_t;
    let mut nonce: *mut uint8_t = xalloc((*aead).nonce_size as size_t) as *mut uint8_t;
    static mut data: [uint8_t; 10240] = [0; 10240];
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    init_data(data.as_mut_ptr());
    if ((*aead).set_nonce).is_some() {
        init_nonce((*aead).nonce_size, nonce);
    }
    let mut info: bench_aead_info = bench_aead_info {
        ctx: 0 as *mut libc::c_void,
        crypt: None,
        update: None,
        data: 0 as *mut uint8_t,
    };
    info.ctx = ctx;
    info.crypt = (*aead).encrypt;
    info.data = data.as_mut_ptr();
    init_key((*aead).key_size, key);
    ((*aead).set_encrypt_key).unwrap()(ctx, key);
    if ((*aead).set_nonce).is_some() {
        ((*aead).set_nonce).unwrap()(ctx, nonce);
    }
    display(
        (*aead).name,
        b"encrypt\0" as *const u8 as *const libc::c_char,
        (*aead).block_size,
        time_function(
            Some(bench_aead_crypt as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut info as *mut bench_aead_info as *mut libc::c_void,
        ),
    );
    if ((*aead).decrypt).is_some() {
        let mut info_0: bench_aead_info = bench_aead_info {
            ctx: 0 as *mut libc::c_void,
            crypt: None,
            update: None,
            data: 0 as *mut uint8_t,
        };
        info_0.ctx = ctx;
        info_0.crypt = (*aead).decrypt;
        info_0.data = data.as_mut_ptr();
        init_key((*aead).key_size, key);
        ((*aead).set_decrypt_key).unwrap()(ctx, key);
        if ((*aead).set_nonce).is_some() {
            ((*aead).set_nonce).unwrap()(ctx, nonce);
        }
        display(
            (*aead).name,
            b"decrypt\0" as *const u8 as *const libc::c_char,
            (*aead).block_size,
            time_function(
                Some(bench_aead_crypt as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info_0 as *mut bench_aead_info as *mut libc::c_void,
            ),
        );
    }
    if ((*aead).update).is_some() {
        let mut info_1: bench_aead_info = bench_aead_info {
            ctx: 0 as *mut libc::c_void,
            crypt: None,
            update: None,
            data: 0 as *mut uint8_t,
        };
        info_1.ctx = ctx;
        info_1.update = (*aead).update;
        info_1.data = data.as_mut_ptr();
        ((*aead).set_encrypt_key).unwrap()(ctx, key);
        if ((*aead).set_nonce).is_some() {
            ((*aead).set_nonce).unwrap()(ctx, nonce);
        }
        display(
            (*aead).name,
            b"update\0" as *const u8 as *const libc::c_char,
            (*aead).block_size,
            time_function(
                Some(bench_aead_update as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                &mut info_1 as *mut bench_aead_info as *mut libc::c_void,
            ),
        );
    }
    free(ctx);
    free(key as *mut libc::c_void);
    free(nonce as *mut libc::c_void);
}
unsafe extern "C" fn compare_double(
    mut ap: *const libc::c_void,
    mut bp: *const libc::c_void,
) -> libc::c_int {
    let mut a: libc::c_double = *(ap as *const libc::c_double);
    let mut b: libc::c_double = *(bp as *const libc::c_double);
    if a < b {
        return -(1 as libc::c_int)
    } else if a > b {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn bench_sha1_compress() {
    let mut state: [uint32_t; 5] = [0; 5];
    let mut data: [uint8_t; 64] = [0; 64];
    let mut t: libc::c_double = 0.;
    let mut tc_count: [libc::c_double; 5] = [0.; 5];
    let mut tc_start_lo: uint32_t = 0;
    let mut tc_start_hi: uint32_t = 0;
    let mut tc_end_lo: uint32_t = 0;
    let mut tc_end_hi: uint32_t = 0;
    let mut tc_i: libc::c_uint = 0;
    let mut tc_j: libc::c_uint = 0;
    tc_j = 0 as libc::c_int as libc::c_uint;
    while tc_j < 5 as libc::c_int as libc::c_uint {
        tc_i = 0 as libc::c_int as libc::c_uint;
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_start_lo, lateout("dx") tc_start_hi, out("r10")   _,
            out("rcx")   _, options(att_syntax)
        );
        while tc_i < 10 as libc::c_int as libc::c_uint {
            nettle_sha1_compress(state.as_mut_ptr(), data.as_mut_ptr());
            tc_i = tc_i.wrapping_add(1);
            tc_i;
        }
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_end_lo, lateout("dx") tc_end_hi, out("r10")   _, out("rcx")
              _, options(att_syntax)
        );
        tc_end_hi = (tc_end_hi as libc::c_uint)
            .wrapping_sub(
                tc_start_hi
                    .wrapping_add(
                        (tc_start_lo > tc_end_lo) as libc::c_int as libc::c_uint,
                    ),
            ) as uint32_t as uint32_t;
        tc_end_lo = (tc_end_lo as libc::c_uint).wrapping_sub(tc_start_lo) as uint32_t
            as uint32_t;
        tc_count[tc_j
            as usize] = ldexp(tc_end_hi as libc::c_double, 32 as libc::c_int)
            + tc_end_lo as libc::c_double;
        tc_j = tc_j.wrapping_add(1);
        tc_j;
    }
    qsort(
        tc_count.as_mut_ptr() as *mut libc::c_void,
        5 as libc::c_int as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        Some(
            compare_double
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    t = tc_count[2 as libc::c_int as usize] / 10 as libc::c_int as libc::c_double;
    printf(b"sha1_compress: %.2f cycles\n\0" as *const u8 as *const libc::c_char, t);
}
unsafe extern "C" fn bench_salsa20_core() {
    let mut state: [uint32_t; 16] = [0; 16];
    let mut t: libc::c_double = 0.;
    let mut tc_count: [libc::c_double; 5] = [0.; 5];
    let mut tc_start_lo: uint32_t = 0;
    let mut tc_start_hi: uint32_t = 0;
    let mut tc_end_lo: uint32_t = 0;
    let mut tc_end_hi: uint32_t = 0;
    let mut tc_i: libc::c_uint = 0;
    let mut tc_j: libc::c_uint = 0;
    tc_j = 0 as libc::c_int as libc::c_uint;
    while tc_j < 5 as libc::c_int as libc::c_uint {
        tc_i = 0 as libc::c_int as libc::c_uint;
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_start_lo, lateout("dx") tc_start_hi, out("r10")   _,
            out("rcx")   _, options(att_syntax)
        );
        while tc_i < 10 as libc::c_int as libc::c_uint {
            _nettle_salsa20_core(
                state.as_mut_ptr(),
                state.as_mut_ptr(),
                20 as libc::c_int as libc::c_uint,
            );
            tc_i = tc_i.wrapping_add(1);
            tc_i;
        }
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_end_lo, lateout("dx") tc_end_hi, out("r10")   _, out("rcx")
              _, options(att_syntax)
        );
        tc_end_hi = (tc_end_hi as libc::c_uint)
            .wrapping_sub(
                tc_start_hi
                    .wrapping_add(
                        (tc_start_lo > tc_end_lo) as libc::c_int as libc::c_uint,
                    ),
            ) as uint32_t as uint32_t;
        tc_end_lo = (tc_end_lo as libc::c_uint).wrapping_sub(tc_start_lo) as uint32_t
            as uint32_t;
        tc_count[tc_j
            as usize] = ldexp(tc_end_hi as libc::c_double, 32 as libc::c_int)
            + tc_end_lo as libc::c_double;
        tc_j = tc_j.wrapping_add(1);
        tc_j;
    }
    qsort(
        tc_count.as_mut_ptr() as *mut libc::c_void,
        5 as libc::c_int as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        Some(
            compare_double
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    t = tc_count[2 as libc::c_int as usize] / 10 as libc::c_int as libc::c_double;
    printf(b"salsa20_core: %.2f cycles\n\0" as *const u8 as *const libc::c_char, t);
}
unsafe extern "C" fn bench_sha3_permute() {
    let mut state: sha3_state = sha3_state { a: [0; 25] };
    let mut t: libc::c_double = 0.;
    let mut tc_count: [libc::c_double; 5] = [0.; 5];
    let mut tc_start_lo: uint32_t = 0;
    let mut tc_start_hi: uint32_t = 0;
    let mut tc_end_lo: uint32_t = 0;
    let mut tc_end_hi: uint32_t = 0;
    let mut tc_i: libc::c_uint = 0;
    let mut tc_j: libc::c_uint = 0;
    tc_j = 0 as libc::c_int as libc::c_uint;
    while tc_j < 5 as libc::c_int as libc::c_uint {
        tc_i = 0 as libc::c_int as libc::c_uint;
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_start_lo, lateout("dx") tc_start_hi, out("r10")   _,
            out("rcx")   _, options(att_syntax)
        );
        while tc_i < 10 as libc::c_int as libc::c_uint {
            nettle_sha3_permute(&mut state);
            tc_i = tc_i.wrapping_add(1);
            tc_i;
        }
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_end_lo, lateout("dx") tc_end_hi, out("r10")   _, out("rcx")
              _, options(att_syntax)
        );
        tc_end_hi = (tc_end_hi as libc::c_uint)
            .wrapping_sub(
                tc_start_hi
                    .wrapping_add(
                        (tc_start_lo > tc_end_lo) as libc::c_int as libc::c_uint,
                    ),
            ) as uint32_t as uint32_t;
        tc_end_lo = (tc_end_lo as libc::c_uint).wrapping_sub(tc_start_lo) as uint32_t
            as uint32_t;
        tc_count[tc_j
            as usize] = ldexp(tc_end_hi as libc::c_double, 32 as libc::c_int)
            + tc_end_lo as libc::c_double;
        tc_j = tc_j.wrapping_add(1);
        tc_j;
    }
    qsort(
        tc_count.as_mut_ptr() as *mut libc::c_void,
        5 as libc::c_int as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        Some(
            compare_double
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    t = tc_count[2 as libc::c_int as usize] / 10 as libc::c_int as libc::c_double;
    printf(
        b"sha3_permute: %.2f cycles (%.2f / round)\n\0" as *const u8
            as *const libc::c_char,
        t,
        t / 24.0f64,
    );
}
unsafe extern "C" fn bench_ghash_update() {
    let mut key: gcm_key = gcm_key {
        h: [nettle_block16 { b: [0; 16] }; 256],
    };
    let mut state: nettle_block16 = nettle_block16 { b: [0; 16] };
    let data: [uint8_t; 160] = [0; 160];
    let mut t: libc::c_double = 0.;
    let mut tc_count: [libc::c_double; 5] = [0.; 5];
    let mut tc_start_lo: uint32_t = 0;
    let mut tc_start_hi: uint32_t = 0;
    let mut tc_end_lo: uint32_t = 0;
    let mut tc_end_hi: uint32_t = 0;
    let mut tc_i: libc::c_uint = 0;
    let mut tc_j: libc::c_uint = 0;
    tc_j = 0 as libc::c_int as libc::c_uint;
    while tc_j < 5 as libc::c_int as libc::c_uint {
        tc_i = 0 as libc::c_int as libc::c_uint;
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_start_lo, lateout("dx") tc_start_hi, out("r10")   _,
            out("rcx")   _, options(att_syntax)
        );
        while tc_i < 10 as libc::c_int as libc::c_uint {
            _nettle_ghash_update(
                &mut key,
                &mut state,
                10 as libc::c_int as size_t,
                data.as_ptr(),
            );
            tc_i = tc_i.wrapping_add(1);
            tc_i;
        }
        asm!(
            "xorl %eax,%eax\nmov %rbx, %r10\ncpuid\nrdtsc\nmov %r10, %rbx\n",
            lateout("ax") tc_end_lo, lateout("dx") tc_end_hi, out("r10")   _, out("rcx")
              _, options(att_syntax)
        );
        tc_end_hi = (tc_end_hi as libc::c_uint)
            .wrapping_sub(
                tc_start_hi
                    .wrapping_add(
                        (tc_start_lo > tc_end_lo) as libc::c_int as libc::c_uint,
                    ),
            ) as uint32_t as uint32_t;
        tc_end_lo = (tc_end_lo as libc::c_uint).wrapping_sub(tc_start_lo) as uint32_t
            as uint32_t;
        tc_count[tc_j
            as usize] = ldexp(tc_end_hi as libc::c_double, 32 as libc::c_int)
            + tc_end_lo as libc::c_double;
        tc_j = tc_j.wrapping_add(1);
        tc_j;
    }
    qsort(
        tc_count.as_mut_ptr() as *mut libc::c_void,
        5 as libc::c_int as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        Some(
            compare_double
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    t = tc_count[2 as libc::c_int as usize] / 10 as libc::c_int as libc::c_double;
    printf(
        b"ghash_update: %.2f cycles / block\n\0" as *const u8 as *const libc::c_char,
        t / 10.0f64,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut c: libc::c_int = 0;
    let mut alg: *const libc::c_char = 0 as *const libc::c_char;
    nettle_openssl_init();
    let mut hashes: [*const nettle_hash; 23] = [
        &nettle_md2,
        &nettle_md4,
        &nettle_md5,
        &nettle_openssl_md5,
        &nettle_sha1,
        &nettle_openssl_sha1,
        &nettle_sha224,
        &nettle_sha256,
        &nettle_sha384,
        &nettle_sha512,
        &nettle_sha512_224,
        &nettle_sha512_256,
        &nettle_sha3_224,
        &nettle_sha3_256,
        &nettle_sha3_384,
        &nettle_sha3_512,
        &nettle_ripemd160,
        &nettle_gosthash94,
        &nettle_gosthash94cp,
        &nettle_streebog256,
        &nettle_streebog512,
        &nettle_sm3,
        0 as *const nettle_hash,
    ];
    let mut ciphers: [*const nettle_cipher; 22] = [
        &nettle_aes128,
        &nettle_aes192,
        &nettle_aes256,
        &nettle_openssl_aes128,
        &nettle_openssl_aes192,
        &nettle_openssl_aes256,
        &nettle_blowfish128,
        &nettle_openssl_blowfish128,
        &nettle_camellia128,
        &nettle_camellia192,
        &nettle_camellia256,
        &nettle_cast128,
        &nettle_openssl_cast128,
        &nettle_des,
        &nettle_openssl_des,
        &nettle_des3,
        &nettle_serpent256,
        &nettle_twofish128,
        &nettle_twofish192,
        &nettle_twofish256,
        &nettle_sm4,
        0 as *const nettle_cipher,
    ];
    let mut aeads: [*const nettle_aead; 19] = [
        &nettle_arcfour128,
        &nettle_salsa20,
        &nettle_salsa20r12,
        &nettle_chacha,
        &nettle_cbc_aes128,
        &nettle_cbc_aes192,
        &nettle_cbc_aes256,
        &nettle_gcm_aes128,
        &nettle_gcm_aes192,
        &nettle_gcm_aes256,
        &nettle_openssl_gcm_aes128,
        &nettle_openssl_gcm_aes192,
        &nettle_openssl_gcm_aes256,
        &nettle_gcm_camellia128,
        &nettle_gcm_camellia256,
        &nettle_eax_aes128,
        &nettle_chacha_poly1305,
        &nettle_ocb_aes128,
        0 as *const nettle_aead,
    ];
    static mut options: [option; 3] = [
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_HELP as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"clock-frequency\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    loop {
        c = getopt_long(
            argc,
            argv,
            b"f:\0" as *const u8 as *const libc::c_char,
            options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        's_53: {
            match c {
                102 => {
                    frequency = atof(optarg);
                    if frequency > 0.0f64 {
                        break 's_53;
                    }
                }
                300 => {}
                63 => return 1 as libc::c_int,
                _ => {
                    abort();
                }
            }
            printf(
                b"Usage: nettle-benchmark [-f clock frequency] [alg...]\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    time_init();
    bench_sha1_compress();
    bench_salsa20_core();
    bench_sha3_permute();
    bench_ghash_update();
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    header();
    loop {
        alg = *argv.offset(optind as isize);
        if alg.is_null()
            || !(strstr(b"memxor\0" as *const u8 as *const libc::c_char, alg)).is_null()
        {
            time_memxor();
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int as libc::c_uint;
        while !(hashes[i as usize]).is_null() {
            if alg.is_null() || !(strstr((*hashes[i as usize]).name, alg)).is_null() {
                time_hash(hashes[i as usize]);
            }
            i = i.wrapping_add(1);
            i;
        }
        if alg.is_null()
            || !(strstr(b"umac\0" as *const u8 as *const libc::c_char, alg)).is_null()
        {
            time_umac();
        }
        if alg.is_null()
            || !(strstr(b"cmac\0" as *const u8 as *const libc::c_char, alg)).is_null()
        {
            time_cmac();
        }
        if alg.is_null()
            || !(strstr(b"poly1305-aes\0" as *const u8 as *const libc::c_char, alg))
                .is_null()
        {
            time_poly1305_aes();
        }
        i = 0 as libc::c_int as libc::c_uint;
        while !(ciphers[i as usize]).is_null() {
            if alg.is_null() || !(strstr((*ciphers[i as usize]).name, alg)).is_null() {
                time_cipher(ciphers[i as usize]);
            }
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as libc::c_uint;
        while !(aeads[i as usize]).is_null() {
            if alg.is_null() || !(strstr((*aeads[i as usize]).name, alg)).is_null() {
                time_aead(aeads[i as usize]);
            }
            i = i.wrapping_add(1);
            i;
        }
        if alg.is_null()
            || !(strstr(b"hmac-md5\0" as *const u8 as *const libc::c_char, alg))
                .is_null()
        {
            time_hmac_md5();
        }
        if alg.is_null()
            || !(strstr(b"hmac-sha1\0" as *const u8 as *const libc::c_char, alg))
                .is_null()
        {
            time_hmac_sha1();
        }
        if alg.is_null()
            || !(strstr(b"hmac-sha256\0" as *const u8 as *const libc::c_char, alg))
                .is_null()
        {
            time_hmac_sha256();
        }
        if alg.is_null()
            || !(strstr(b"hmac-sha512\0" as *const u8 as *const libc::c_char, alg))
                .is_null()
        {
            time_hmac_sha512();
        }
        optind += 1;
        optind;
        if !(!alg.is_null() && !(*argv.offset(optind as isize)).is_null()) {
            break;
        }
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
