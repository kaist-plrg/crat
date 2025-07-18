use ::libc;
extern "C" {
    pub type evp_cipher_st;
    pub type evp_cipher_ctx_st;
    pub type evp_md_st;
    pub type evp_md_ctx_st;
    pub type ossl_init_settings_st;
    pub type engine_st;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn OPENSSL_init_crypto(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> libc::c_int;
    fn CONF_modules_load_file(
        filename: *const libc::c_char,
        appname: *const libc::c_char,
        flags: libc::c_ulong,
    ) -> libc::c_int;
    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    fn EVP_DigestInit(ctx: *mut EVP_MD_CTX, type_0: *const EVP_MD) -> libc::c_int;
    fn EVP_DigestFinal(
        ctx: *mut EVP_MD_CTX,
        md: *mut libc::c_uchar,
        s: *mut libc::c_uint,
    ) -> libc::c_int;
    fn EVP_EncryptUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut libc::c_uchar,
        outl: *mut libc::c_int,
        in_0: *const libc::c_uchar,
        inl: libc::c_int,
    ) -> libc::c_int;
    fn EVP_DecryptUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut libc::c_uchar,
        outl: *mut libc::c_int,
        in_0: *const libc::c_uchar,
        inl: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CipherInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_0: *mut ENGINE,
        key: *const libc::c_uchar,
        iv: *const libc::c_uchar,
        enc: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    fn EVP_CIPHER_CTX_set_padding(
        c: *mut EVP_CIPHER_CTX,
        pad: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CIPHER_CTX_ctrl(
        ctx: *mut EVP_CIPHER_CTX,
        type_0: libc::c_int,
        arg: libc::c_int,
        ptr: *mut libc::c_void,
    ) -> libc::c_int;
    fn EVP_md5() -> *const EVP_MD;
    fn EVP_sha1() -> *const EVP_MD;
    fn EVP_des_ecb() -> *const EVP_CIPHER;
    fn EVP_bf_ecb() -> *const EVP_CIPHER;
    fn EVP_cast5_ecb() -> *const EVP_CIPHER;
    fn EVP_aes_128_ecb() -> *const EVP_CIPHER;
    fn EVP_aes_128_gcm() -> *const EVP_CIPHER;
    fn EVP_aes_192_ecb() -> *const EVP_CIPHER;
    fn EVP_aes_192_gcm() -> *const EVP_CIPHER;
    fn EVP_aes_256_ecb() -> *const EVP_CIPHER;
    fn EVP_aes_256_gcm() -> *const EVP_CIPHER;
    fn EVP_DigestUpdate(
        ctx: *mut EVP_MD_CTX,
        d: *const libc::c_void,
        cnt: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type EVP_CIPHER = evp_cipher_st;
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
pub type EVP_MD = evp_md_st;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
pub type ENGINE = engine_st;
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
pub struct openssl_cipher_ctx {
    pub evp: *mut EVP_CIPHER_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct openssl_hash_ctx {
    pub evp: *mut EVP_MD_CTX,
}
pub unsafe extern "C" fn nettle_openssl_init() {
    OPENSSL_init_crypto(
        0x2 as libc::c_long as uint64_t,
        0 as *const OPENSSL_INIT_SETTINGS,
    );
    OPENSSL_init_crypto(
        (0x4 as libc::c_long | 0x8 as libc::c_long) as uint64_t,
        0 as *const OPENSSL_INIT_SETTINGS,
    );
    CONF_modules_load_file(
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as libc::c_int as libc::c_ulong,
    );
}
unsafe extern "C" fn openssl_evp_set_encrypt_key(
    mut p: *mut libc::c_void,
    mut key: *const uint8_t,
    mut cipher: *const EVP_CIPHER,
) {
    let mut ctx: *mut openssl_cipher_ctx = p as *mut openssl_cipher_ctx;
    let mut ret: libc::c_int = 0;
    (*ctx).evp = EVP_CIPHER_CTX_new();
    ret = EVP_CipherInit_ex(
        (*ctx).evp,
        cipher,
        0 as *mut ENGINE,
        key,
        0 as *const libc::c_uchar,
        1 as libc::c_int,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void openssl_evp_set_encrypt_key(void *, const uint8_t *, const EVP_CIPHER *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15872: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void openssl_evp_set_encrypt_key(void *, const uint8_t *, const EVP_CIPHER *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    EVP_CIPHER_CTX_set_padding((*ctx).evp, 0 as libc::c_int);
}
unsafe extern "C" fn openssl_evp_set_decrypt_key(
    mut p: *mut libc::c_void,
    mut key: *const uint8_t,
    mut cipher: *const EVP_CIPHER,
) {
    let mut ctx: *mut openssl_cipher_ctx = p as *mut openssl_cipher_ctx;
    let mut ret: libc::c_int = 0;
    (*ctx).evp = EVP_CIPHER_CTX_new();
    ret = EVP_CipherInit_ex(
        (*ctx).evp,
        cipher,
        0 as *mut ENGINE,
        key,
        0 as *const libc::c_uchar,
        0 as libc::c_int,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void openssl_evp_set_decrypt_key(void *, const uint8_t *, const EVP_CIPHER *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15765: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void openssl_evp_set_decrypt_key(void *, const uint8_t *, const EVP_CIPHER *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    EVP_CIPHER_CTX_set_padding((*ctx).evp, 0 as libc::c_int);
}
unsafe extern "C" fn openssl_evp_encrypt(
    mut p: *const libc::c_void,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctx: *const openssl_cipher_ctx = p as *const openssl_cipher_ctx;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = EVP_EncryptUpdate(
        (*ctx).evp,
        dst,
        &mut len,
        src,
        length as libc::c_int,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void openssl_evp_encrypt(const void *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_16199: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void openssl_evp_encrypt(const void *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn openssl_evp_decrypt(
    mut p: *const libc::c_void,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctx: *const openssl_cipher_ctx = p as *const openssl_cipher_ctx;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = EVP_DecryptUpdate(
        (*ctx).evp,
        dst,
        &mut len,
        src,
        length as libc::c_int,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void openssl_evp_decrypt(const void *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_16129: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void openssl_evp_decrypt(const void *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn openssl_evp_set_nonce(
    mut p: *mut libc::c_void,
    mut nonce: *const uint8_t,
) {
    let mut ctx: *const openssl_cipher_ctx = p as *const openssl_cipher_ctx;
    let mut ret: libc::c_int = EVP_CipherInit_ex(
        (*ctx).evp,
        0 as *const EVP_CIPHER,
        0 as *mut ENGINE,
        0 as *const libc::c_uchar,
        nonce,
        -(1 as libc::c_int),
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void openssl_evp_set_nonce(void *, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_15655: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                131 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void openssl_evp_set_nonce(void *, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn openssl_evp_update(
    mut p: *mut libc::c_void,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    let mut ctx: *const openssl_cipher_ctx = p as *const openssl_cipher_ctx;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = EVP_EncryptUpdate(
        (*ctx).evp,
        0 as *mut libc::c_uchar,
        &mut len,
        src,
        length as libc::c_int,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void openssl_evp_update(void *, size_t, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_15584: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void openssl_evp_update(void *, size_t, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn openssl_evp_gcm_digest(
    mut p: *mut libc::c_void,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut ctx: *const openssl_cipher_ctx = p as *const openssl_cipher_ctx;
    let mut ret: libc::c_int = EVP_CIPHER_CTX_ctrl(
        (*ctx).evp,
        0x10 as libc::c_int,
        length as libc::c_int,
        dst as *mut libc::c_void,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void openssl_evp_gcm_digest(void *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_15364: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void openssl_evp_gcm_digest(void *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn openssl_evp_aead_encrypt(
    mut p: *mut libc::c_void,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctx: *const openssl_cipher_ctx = p as *const openssl_cipher_ctx;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = EVP_EncryptUpdate(
        (*ctx).evp,
        dst,
        &mut len,
        src,
        length as libc::c_int,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void openssl_evp_aead_encrypt(void *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15515: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                159 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void openssl_evp_aead_encrypt(void *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn openssl_evp_aead_decrypt(
    mut p: *mut libc::c_void,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctx: *const openssl_cipher_ctx = p as *const openssl_cipher_ctx;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = EVP_DecryptUpdate(
        (*ctx).evp,
        dst,
        &mut len,
        src,
        length as libc::c_int,
    );
    if ret == 1 as libc::c_int {} else {
        __assert_fail(
            b"ret == 1\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void openssl_evp_aead_decrypt(void *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15444: {
        if ret == 1 as libc::c_int {} else {
            __assert_fail(
                b"ret == 1\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void openssl_evp_aead_decrypt(void *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn openssl_aes128_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_aes_128_ecb());
}
unsafe extern "C" fn openssl_aes128_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_aes_128_ecb());
}
unsafe extern "C" fn openssl_aes192_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_aes_192_ecb());
}
unsafe extern "C" fn openssl_aes192_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_aes_192_ecb());
}
unsafe extern "C" fn openssl_aes256_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_aes_256_ecb());
}
unsafe extern "C" fn openssl_aes256_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_aes_256_ecb());
}
pub static mut nettle_openssl_aes128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"openssl aes128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(openssl_aes128_set_encrypt_key as nettle_set_key_func),
            set_decrypt_key: Some(openssl_aes128_set_decrypt_key as nettle_set_key_func),
            encrypt: Some(
                openssl_evp_encrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_decrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
pub static mut nettle_openssl_aes192: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"openssl aes192\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 24 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(openssl_aes192_set_encrypt_key as nettle_set_key_func),
            set_decrypt_key: Some(openssl_aes192_set_decrypt_key as nettle_set_key_func),
            encrypt: Some(
                openssl_evp_encrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_decrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
pub static mut nettle_openssl_aes256: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"openssl aes256\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(openssl_aes256_set_encrypt_key as nettle_set_key_func),
            set_decrypt_key: Some(openssl_aes256_set_decrypt_key as nettle_set_key_func),
            encrypt: Some(
                openssl_evp_encrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_decrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn openssl_gcm_aes128_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_aes_128_gcm());
}
unsafe extern "C" fn openssl_gcm_aes128_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_aes_128_gcm());
}
unsafe extern "C" fn openssl_gcm_aes192_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_aes_192_gcm());
}
unsafe extern "C" fn openssl_gcm_aes192_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_aes_192_gcm());
}
unsafe extern "C" fn openssl_gcm_aes256_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_aes_256_gcm());
}
unsafe extern "C" fn openssl_gcm_aes256_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_aes_256_gcm());
}
pub static mut nettle_openssl_gcm_aes128: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"openssl gcm_aes128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            nonce_size: 12 as libc::c_int as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                openssl_gcm_aes128_set_encrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                openssl_gcm_aes128_set_decrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_nonce: Some(
                openssl_evp_set_nonce
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            update: Some(
                openssl_evp_update
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *const uint8_t,
                    ) -> (),
            ),
            encrypt: Some(
                openssl_evp_aead_encrypt
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_aead_decrypt
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            digest: Some(
                openssl_evp_gcm_digest
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
pub static mut nettle_openssl_gcm_aes192: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"openssl gcm_aes192\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 24 as libc::c_int as libc::c_uint,
            nonce_size: 12 as libc::c_int as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                openssl_gcm_aes192_set_encrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                openssl_gcm_aes192_set_decrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_nonce: Some(
                openssl_evp_set_nonce
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            update: Some(
                openssl_evp_update
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *const uint8_t,
                    ) -> (),
            ),
            encrypt: Some(
                openssl_evp_aead_encrypt
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_aead_decrypt
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            digest: Some(
                openssl_evp_gcm_digest
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
pub static mut nettle_openssl_gcm_aes256: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"openssl gcm_aes256\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            nonce_size: 12 as libc::c_int as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                openssl_gcm_aes256_set_encrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                openssl_gcm_aes256_set_decrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_nonce: Some(
                openssl_evp_set_nonce
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            update: Some(
                openssl_evp_update
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *const uint8_t,
                    ) -> (),
            ),
            encrypt: Some(
                openssl_evp_aead_encrypt
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_aead_decrypt
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            digest: Some(
                openssl_evp_gcm_digest
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn openssl_bf128_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_bf_ecb());
}
unsafe extern "C" fn openssl_bf128_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_bf_ecb());
}
pub static mut nettle_openssl_blowfish128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"openssl bf128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                openssl_bf128_set_encrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                openssl_bf128_set_decrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            encrypt: Some(
                openssl_evp_encrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_decrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn openssl_des_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_des_ecb());
}
unsafe extern "C" fn openssl_des_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_des_ecb());
}
pub static mut nettle_openssl_des: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"openssl des\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: 8 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                openssl_des_set_encrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                openssl_des_set_decrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            encrypt: Some(
                openssl_evp_encrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_decrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn openssl_cast128_set_encrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_encrypt_key(ctx, key, EVP_cast5_ecb());
}
unsafe extern "C" fn openssl_cast128_set_decrypt_key(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    openssl_evp_set_decrypt_key(ctx, key, EVP_cast5_ecb());
}
pub static mut nettle_openssl_cast128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"openssl cast128\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_cipher_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 8 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: Some(
                openssl_cast128_set_encrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            set_decrypt_key: Some(
                openssl_cast128_set_decrypt_key
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            encrypt: Some(
                openssl_evp_encrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
            decrypt: Some(
                openssl_evp_decrypt
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn openssl_hash_update(
    mut p: *mut libc::c_void,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    let mut ctx: *mut openssl_hash_ctx = p as *mut openssl_hash_ctx;
    EVP_DigestUpdate((*ctx).evp, src as *const libc::c_void, length);
}
unsafe extern "C" fn openssl_md5_init(mut p: *mut libc::c_void) {
    let mut ctx: *mut openssl_hash_ctx = p as *mut openssl_hash_ctx;
    (*ctx).evp = EVP_MD_CTX_new();
    if ((*ctx).evp).is_null() {
        return;
    }
    EVP_DigestInit((*ctx).evp, EVP_md5());
}
unsafe extern "C" fn openssl_md5_digest(
    mut p: *mut libc::c_void,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut ctx: *mut openssl_hash_ctx = p as *mut openssl_hash_ctx;
    if length == 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length == MD5_DIGEST_LENGTH\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void openssl_md5_digest(void *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_16597: {
        if length == 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length == MD5_DIGEST_LENGTH\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                408 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void openssl_md5_digest(void *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    EVP_DigestFinal((*ctx).evp, dst, 0 as *mut libc::c_uint);
    EVP_DigestInit((*ctx).evp, EVP_md5());
}
pub static mut nettle_openssl_md5: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"openssl md5\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_hash_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            block_size: 64 as libc::c_int as libc::c_uint,
            init: Some(
                openssl_md5_init as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            update: Some(
                openssl_hash_update
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *const uint8_t,
                    ) -> (),
            ),
            digest: Some(
                openssl_md5_digest
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn openssl_sha1_init(mut p: *mut libc::c_void) {
    let mut ctx: *mut openssl_hash_ctx = p as *mut openssl_hash_ctx;
    (*ctx).evp = EVP_MD_CTX_new();
    if ((*ctx).evp).is_null() {
        return;
    }
    EVP_DigestInit((*ctx).evp, EVP_sha1());
}
unsafe extern "C" fn openssl_sha1_digest(
    mut p: *mut libc::c_void,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut ctx: *mut openssl_hash_ctx = p as *mut openssl_hash_ctx;
    if length == 20 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length == SHA_DIGEST_LENGTH\0" as *const u8 as *const libc::c_char,
            b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void openssl_sha1_digest(void *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_16752: {
        if length == 20 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length == SHA_DIGEST_LENGTH\0" as *const u8 as *const libc::c_char,
                b"nettle-openssl.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void openssl_sha1_digest(void *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    EVP_DigestFinal((*ctx).evp, dst, 0 as *mut libc::c_uint);
    EVP_DigestInit((*ctx).evp, EVP_sha1());
}
pub static mut nettle_openssl_sha1: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"openssl sha1\0" as *const u8 as *const libc::c_char,
            context_size: ::std::mem::size_of::<openssl_hash_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 20 as libc::c_int as libc::c_uint,
            block_size: (16 as libc::c_int * 4 as libc::c_int) as libc::c_uint,
            init: Some(
                openssl_sha1_init as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            update: Some(
                openssl_hash_update
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *const uint8_t,
                    ) -> (),
            ),
            digest: Some(
                openssl_sha1_digest
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        };
        init
    }
};
