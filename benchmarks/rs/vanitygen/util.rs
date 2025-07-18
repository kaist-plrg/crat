use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type bn_blinding_st;
    pub type engine_st;
    pub type ASN1_VALUE_st;
    pub type evp_pkey_ctx_st;
    pub type ec_key_st;
    pub type evp_pkey_asn1_method_st;
    pub type ec_group_st;
    pub type ec_point_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn BN_num_bits(a: *const BIGNUM) -> libc::c_int;
    fn BN_init(_: *mut BIGNUM);
    fn BN_clear_free(a: *mut BIGNUM);
    fn BN_bin2bn(
        s: *const libc::c_uchar,
        len: libc::c_int,
        ret: *mut BIGNUM,
    ) -> *mut BIGNUM;
    fn BN_bn2bin(a: *const BIGNUM, to: *mut libc::c_uchar) -> libc::c_int;
    fn BN_add(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_mul(
        r: *mut BIGNUM,
        a: *const BIGNUM,
        b: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_div(
        dv: *mut BIGNUM,
        rem: *mut BIGNUM,
        m: *const BIGNUM,
        d: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_set_word(a: *mut BIGNUM, w: libc::c_ulong) -> libc::c_int;
    fn BN_get_word(a: *const BIGNUM) -> libc::c_ulong;
    fn BN_clear(a: *mut BIGNUM);
    fn BN_bn2hex(a: *const BIGNUM) -> *mut libc::c_char;
    fn OPENSSL_cleanse(ptr: *mut libc::c_void, len: size_t);
    fn CRYPTO_free(ptr: *mut libc::c_void);
    fn SHA256(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn RIPEMD160(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn HMAC(
        evp_md: *const EVP_MD,
        key: *const libc::c_void,
        key_len: libc::c_int,
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
        md_len: *mut libc::c_uint,
    ) -> *mut libc::c_uchar;
    fn BIO_new(type_0: *mut BIO_METHOD) -> *mut BIO;
    fn BIO_free(a: *mut BIO) -> libc::c_int;
    fn BIO_ctrl(
        bp: *mut BIO,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn BIO_s_mem() -> *mut BIO_METHOD;
    fn BIO_new_mem_buf(buf: *mut libc::c_void, len: libc::c_int) -> *mut BIO;
    fn EVP_read_pw_string(
        buf: *mut libc::c_char,
        length: libc::c_int,
        prompt: *const libc::c_char,
        verify: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CipherInit(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        key: *const libc::c_uchar,
        iv: *const libc::c_uchar,
        enc: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CipherUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut libc::c_uchar,
        outl: *mut libc::c_int,
        in_0: *const libc::c_uchar,
        inl: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CipherFinal(
        ctx: *mut EVP_CIPHER_CTX,
        outm: *mut libc::c_uchar,
        outl: *mut libc::c_int,
    ) -> libc::c_int;
    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    fn EVP_CIPHER_CTX_free(a: *mut EVP_CIPHER_CTX);
    fn EVP_CIPHER_CTX_set_padding(
        c: *mut EVP_CIPHER_CTX,
        pad: libc::c_int,
    ) -> libc::c_int;
    fn EVP_sha256() -> *const EVP_MD;
    fn EVP_aes_256_cbc() -> *const EVP_CIPHER;
    fn EVP_PKEY_set1_EC_KEY(pkey: *mut EVP_PKEY, key: *mut ec_key_st) -> libc::c_int;
    fn EVP_PKEY_get1_EC_KEY(pkey: *mut EVP_PKEY) -> *mut ec_key_st;
    fn EVP_PKEY_new() -> *mut EVP_PKEY;
    fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
    fn PKCS5_PBKDF2_HMAC(
        pass: *const libc::c_char,
        passlen: libc::c_int,
        salt: *const libc::c_uchar,
        saltlen: libc::c_int,
        iter: libc::c_int,
        digest: *const EVP_MD,
        keylen: libc::c_int,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn RAND_bytes(buf: *mut libc::c_uchar, num: libc::c_int) -> libc::c_int;
    fn EC_GROUP_cmp(
        a: *const EC_GROUP,
        b: *const EC_GROUP,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINT_new(group: *const EC_GROUP) -> *mut EC_POINT;
    fn EC_POINT_free(point: *mut EC_POINT);
    fn EC_POINT_point2oct(
        group: *const EC_GROUP,
        p: *const EC_POINT,
        form: point_conversion_form_t,
        buf: *mut libc::c_uchar,
        len: size_t,
        ctx: *mut BN_CTX,
    ) -> size_t;
    fn EC_POINT_mul(
        group: *const EC_GROUP,
        r: *mut EC_POINT,
        n: *const BIGNUM,
        q: *const EC_POINT,
        m: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn EC_KEY_copy(dst: *mut EC_KEY, src: *const EC_KEY) -> *mut EC_KEY;
    fn EC_KEY_dup(src: *const EC_KEY) -> *mut EC_KEY;
    fn EC_KEY_get0_group(key: *const EC_KEY) -> *const EC_GROUP;
    fn EC_KEY_get0_private_key(key: *const EC_KEY) -> *const BIGNUM;
    fn EC_KEY_set_private_key(key: *mut EC_KEY, prv: *const BIGNUM) -> libc::c_int;
    fn EC_KEY_set_public_key(key: *mut EC_KEY, pub_0: *const EC_POINT) -> libc::c_int;
    fn EC_KEY_check_key(key: *const EC_KEY) -> libc::c_int;
    fn X509_SIG_free(a: *mut X509_SIG);
    fn EVP_PKEY2PKCS8(pkey: *mut EVP_PKEY) -> *mut PKCS8_PRIV_KEY_INFO;
    fn PKCS8_PRIV_KEY_INFO_free(a: *mut PKCS8_PRIV_KEY_INFO);
    fn EVP_PKCS82PKEY(p8: *mut PKCS8_PRIV_KEY_INFO) -> *mut EVP_PKEY;
    fn PEM_write_bio_PKCS8(bp: *mut BIO, x: *mut X509_SIG) -> libc::c_int;
    fn PEM_read_bio_PKCS8(
        bp: *mut BIO,
        x: *mut *mut X509_SIG,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut X509_SIG;
    fn PEM_write_bio_PKCS8_PRIV_KEY_INFO(
        bp: *mut BIO,
        x: *mut PKCS8_PRIV_KEY_INFO,
    ) -> libc::c_int;
    fn PEM_read_bio_PKCS8_PRIV_KEY_INFO(
        bp: *mut BIO,
        x: *mut *mut PKCS8_PRIV_KEY_INFO,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut PKCS8_PRIV_KEY_INFO;
    fn PKCS8_decrypt(
        p8: *mut X509_SIG,
        pass: *const libc::c_char,
        passlen: libc::c_int,
    ) -> *mut PKCS8_PRIV_KEY_INFO;
    fn PKCS8_encrypt(
        pbe_nid: libc::c_int,
        cipher: *const EVP_CIPHER,
        pass: *const libc::c_char,
        passlen: libc::c_int,
        salt: *mut libc::c_uchar,
        saltlen: libc::c_int,
        iter: libc::c_int,
        p8: *mut PKCS8_PRIV_KEY_INFO,
    ) -> *mut X509_SIG;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_string_st {
    pub length: libc::c_int,
    pub type_0: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub flags: libc::c_long,
}
pub type ASN1_INTEGER = asn1_string_st;
pub type ASN1_ENUMERATED = asn1_string_st;
pub type ASN1_BIT_STRING = asn1_string_st;
pub type ASN1_OCTET_STRING = asn1_string_st;
pub type ASN1_PRINTABLESTRING = asn1_string_st;
pub type ASN1_T61STRING = asn1_string_st;
pub type ASN1_IA5STRING = asn1_string_st;
pub type ASN1_GENERALSTRING = asn1_string_st;
pub type ASN1_UNIVERSALSTRING = asn1_string_st;
pub type ASN1_BMPSTRING = asn1_string_st;
pub type ASN1_UTCTIME = asn1_string_st;
pub type ASN1_GENERALIZEDTIME = asn1_string_st;
pub type ASN1_VISIBLESTRING = asn1_string_st;
pub type ASN1_UTF8STRING = asn1_string_st;
pub type ASN1_STRING = asn1_string_st;
pub type ASN1_BOOLEAN = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type BIGNUM = bignum_st;
pub type BN_CTX = bignum_ctx;
pub type BN_BLINDING = bn_blinding_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_mont_ctx_st {
    pub ri: libc::c_int,
    pub RR: BIGNUM,
    pub N: BIGNUM,
    pub Ni: BIGNUM,
    pub n0: [libc::c_ulong; 2],
    pub flags: libc::c_int,
}
pub type BN_MONT_CTX = bn_mont_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_gencb_st {
    pub ver: libc::c_uint,
    pub arg: *mut libc::c_void,
    pub cb: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub cb_1: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub cb_2: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut BN_GENCB) -> libc::c_int,
    >,
}
pub type BN_GENCB = bn_gencb_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
}
pub type BUF_MEM = buf_mem_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_st {
    pub nid: libc::c_int,
    pub block_size: libc::c_int,
    pub key_len: libc::c_int,
    pub iv_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *const libc::c_uchar,
            *const libc::c_uchar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub do_cipher: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *mut libc::c_uchar,
            *const libc::c_uchar,
            size_t,
        ) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_CIPHER_CTX) -> libc::c_int>,
    pub ctx_size: libc::c_int,
    pub set_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub get_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub app_data: *mut libc::c_void,
}
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_ctx_st {
    pub cipher: *const EVP_CIPHER,
    pub engine: *mut ENGINE,
    pub encrypt: libc::c_int,
    pub buf_len: libc::c_int,
    pub oiv: [libc::c_uchar; 16],
    pub iv: [libc::c_uchar; 16],
    pub buf: [libc::c_uchar; 32],
    pub num: libc::c_int,
    pub app_data: *mut libc::c_void,
    pub key_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub cipher_data: *mut libc::c_void,
    pub final_used: libc::c_int,
    pub block_mask: libc::c_int,
    pub final_0: [libc::c_uchar; 32],
}
pub type ENGINE = engine_st;
pub type EVP_CIPHER = evp_cipher_st;
pub type ASN1_TYPE = asn1_type_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_type_st {
    pub type_0: libc::c_int,
    pub value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ptr: *mut libc::c_char,
    pub boolean: ASN1_BOOLEAN,
    pub asn1_string: *mut ASN1_STRING,
    pub object: *mut ASN1_OBJECT,
    pub integer: *mut ASN1_INTEGER,
    pub enumerated: *mut ASN1_ENUMERATED,
    pub bit_string: *mut ASN1_BIT_STRING,
    pub octet_string: *mut ASN1_OCTET_STRING,
    pub printablestring: *mut ASN1_PRINTABLESTRING,
    pub t61string: *mut ASN1_T61STRING,
    pub ia5string: *mut ASN1_IA5STRING,
    pub generalstring: *mut ASN1_GENERALSTRING,
    pub bmpstring: *mut ASN1_BMPSTRING,
    pub universalstring: *mut ASN1_UNIVERSALSTRING,
    pub utctime: *mut ASN1_UTCTIME,
    pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
    pub visiblestring: *mut ASN1_VISIBLESTRING,
    pub utf8string: *mut ASN1_UTF8STRING,
    pub set: *mut ASN1_STRING,
    pub sequence: *mut ASN1_STRING,
    pub asn1_value: *mut ASN1_VALUE,
}
pub type ASN1_VALUE = ASN1_VALUE_st;
pub type ASN1_OBJECT = asn1_object_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_object_st {
    pub sn: *const libc::c_char,
    pub ln: *const libc::c_char,
    pub nid: libc::c_int,
    pub length: libc::c_int,
    pub data: *const libc::c_uchar,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_st {
    pub type_0: libc::c_int,
    pub pkey_type: libc::c_int,
    pub md_size: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub final_0: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *mut libc::c_uchar) -> libc::c_int,
    >,
    pub copy: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const EVP_MD_CTX) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub required_pkey_type: [libc::c_int; 5],
    pub block_size: libc::c_int,
    pub ctx_size: libc::c_int,
    pub md_ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_MD_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
}
pub type EVP_MD_CTX = env_md_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_ctx_st {
    pub digest: *const EVP_MD,
    pub engine: *mut ENGINE,
    pub flags: libc::c_ulong,
    pub md_data: *mut libc::c_void,
    pub pctx: *mut EVP_PKEY_CTX,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
}
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type EVP_MD = env_md_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_pkey_st {
    pub type_0: libc::c_int,
    pub save_type: libc::c_int,
    pub references: libc::c_int,
    pub ameth: *const EVP_PKEY_ASN1_METHOD,
    pub engine: *mut ENGINE,
    pub pkey: C2RustUnnamed_1,
    pub save_parameters: libc::c_int,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_ATTRIBUTE {
    pub stack: _STACK,
}
pub type _STACK = stack_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st {
    pub num: libc::c_int,
    pub data: *mut *mut libc::c_char,
    pub sorted: libc::c_int,
    pub num_alloc: libc::c_int,
    pub comp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ptr: *mut libc::c_char,
    pub rsa: *mut rsa_st,
    pub dsa: *mut dsa_st,
    pub dh: *mut dh_st,
    pub ec: *mut ec_key_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_st {
    pub pad: libc::c_int,
    pub version: libc::c_int,
    pub p: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub length: libc::c_long,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub q: *mut BIGNUM,
    pub j: *mut BIGNUM,
    pub seed: *mut libc::c_uchar,
    pub seedlen: libc::c_int,
    pub counter: *mut BIGNUM,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DH_METHOD,
    pub engine: *mut ENGINE,
}
pub type DH_METHOD = dh_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_method {
    pub name: *const libc::c_char,
    pub generate_key: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub compute_key: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, *const BIGNUM, *mut DH) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *const DH,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub generate_params: Option::<
        unsafe extern "C" fn(
            *mut DH,
            libc::c_int,
            libc::c_int,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type DH = dh_st;
pub type CRYPTO_EX_DATA = crypto_ex_data_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crypto_ex_data_st {
    pub sk: *mut stack_st_void,
    pub dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_void {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub write_params: libc::c_int,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub kinv: *mut BIGNUM,
    pub r: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DSA_METHOD,
    pub engine: *mut ENGINE,
}
pub type DSA_METHOD = dsa_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_method {
    pub name: *const libc::c_char,
    pub dsa_do_sign: Option::<
        unsafe extern "C" fn(*const libc::c_uchar, libc::c_int, *mut DSA) -> *mut DSA_SIG,
    >,
    pub dsa_sign_setup: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BN_CTX,
            *mut *mut BIGNUM,
            *mut *mut BIGNUM,
        ) -> libc::c_int,
    >,
    pub dsa_do_verify: Option::<
        unsafe extern "C" fn(
            *const libc::c_uchar,
            libc::c_int,
            *mut DSA_SIG,
            *mut DSA,
        ) -> libc::c_int,
    >,
    pub dsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub dsa_paramgen: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            libc::c_int,
            *const libc::c_uchar,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_ulong,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
    pub dsa_keygen: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
}
pub type DSA = dsa_st;
pub type DSA_SIG = DSA_SIG_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSA_SIG_st {
    pub r: *mut BIGNUM,
    pub s: *mut BIGNUM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub meth: *const RSA_METHOD,
    pub engine: *mut ENGINE,
    pub n: *mut BIGNUM,
    pub e: *mut BIGNUM,
    pub d: *mut BIGNUM,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub dmp1: *mut BIGNUM,
    pub dmq1: *mut BIGNUM,
    pub iqmp: *mut BIGNUM,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub _method_mod_n: *mut BN_MONT_CTX,
    pub _method_mod_p: *mut BN_MONT_CTX,
    pub _method_mod_q: *mut BN_MONT_CTX,
    pub bignum_data: *mut libc::c_char,
    pub blinding: *mut BN_BLINDING,
    pub mt_blinding: *mut BN_BLINDING,
}
pub type RSA_METHOD = rsa_meth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_meth_st {
    pub name: *const libc::c_char,
    pub rsa_pub_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_pub_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *mut RSA,
            *mut BN_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub rsa_sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_keygen: Option::<
        unsafe extern "C" fn(
            *mut RSA,
            libc::c_int,
            *mut BIGNUM,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type RSA = rsa_st;
pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
pub type EVP_PKEY = evp_pkey_st;
pub type X509_ALGOR = X509_algor_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_algor_st {
    pub algorithm: *mut ASN1_OBJECT,
    pub parameter: *mut ASN1_TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkcs8_priv_key_info_st {
    pub broken: libc::c_int,
    pub version: *mut ASN1_INTEGER,
    pub pkeyalg: *mut X509_ALGOR,
    pub pkey: *mut ASN1_TYPE,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
pub type PKCS8_PRIV_KEY_INFO = pkcs8_priv_key_info_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_st {
    pub method: *mut BIO_METHOD,
    pub callback: Option::<
        unsafe extern "C" fn(
            *mut bio_st,
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
            libc::c_long,
            libc::c_long,
        ) -> libc::c_long,
    >,
    pub cb_arg: *mut libc::c_char,
    pub init: libc::c_int,
    pub shutdown: libc::c_int,
    pub flags: libc::c_int,
    pub retry_reason: libc::c_int,
    pub num: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub next_bio: *mut bio_st,
    pub prev_bio: *mut bio_st,
    pub references: libc::c_int,
    pub num_read: libc::c_ulong,
    pub num_write: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type BIO_METHOD = bio_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub bwrite: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bread: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bputs: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char) -> libc::c_int,
    >,
    pub bgets: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub create: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub destroy: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            Option::<bio_info_cb>,
        ) -> libc::c_long,
    >,
}
pub type bio_info_cb = unsafe extern "C" fn(
    *mut bio_st,
    libc::c_int,
    *const libc::c_char,
    libc::c_int,
    libc::c_long,
    libc::c_long,
) -> ();
pub type BIO = bio_st;
pub type point_conversion_form_t = libc::c_uint;
pub const POINT_CONVERSION_HYBRID: point_conversion_form_t = 6;
pub const POINT_CONVERSION_UNCOMPRESSED: point_conversion_form_t = 4;
pub const POINT_CONVERSION_COMPRESSED: point_conversion_form_t = 2;
pub type EC_GROUP = ec_group_st;
pub type EC_POINT = ec_point_st;
pub type EC_KEY = ec_key_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_sig_st {
    pub algor: *mut X509_ALGOR,
    pub digest: *mut ASN1_OCTET_STRING,
}
pub type X509_SIG = X509_sig_st;
pub type pem_password_cb = unsafe extern "C" fn(
    *mut libc::c_char,
    libc::c_int,
    libc::c_int,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vg_protkey_parameters_t {
    pub mode: libc::c_int,
    pub iterations: libc::c_int,
    pub pbkdf_hash_getter: Option::<unsafe extern "C" fn() -> *const EVP_MD>,
    pub cipher_getter: Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
}
pub static mut vg_b58_alphabet: *const libc::c_char = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz\0"
    as *const u8 as *const libc::c_char;
pub static mut vg_b58_reverse_map: [libc::c_schar; 256] = [
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    45 as libc::c_int as libc::c_schar,
    46 as libc::c_int as libc::c_schar,
    47 as libc::c_int as libc::c_schar,
    48 as libc::c_int as libc::c_schar,
    49 as libc::c_int as libc::c_schar,
    50 as libc::c_int as libc::c_schar,
    51 as libc::c_int as libc::c_schar,
    52 as libc::c_int as libc::c_schar,
    53 as libc::c_int as libc::c_schar,
    54 as libc::c_int as libc::c_schar,
    55 as libc::c_int as libc::c_schar,
    56 as libc::c_int as libc::c_schar,
    57 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn fdumphex(
    mut fp: *mut FILE,
    mut src: *const libc::c_uchar,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        fprintf(
            fp,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *src.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn fdumpbn(mut fp: *mut FILE, mut bn: *const BIGNUM) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = BN_bn2hex(bn);
    fprintf(
        fp,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if !buf.is_null() {
            buf as *const libc::c_char
        } else {
            b"0\0" as *const u8 as *const libc::c_char
        },
    );
    if !buf.is_null() {
        CRYPTO_free(buf as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn dumphex(mut src: *const libc::c_uchar, mut len: size_t) {
    fdumphex(stdout, src, len);
}
pub unsafe extern "C" fn dumpbn(mut bn: *const BIGNUM) {
    fdumpbn(stdout, bn);
}
pub unsafe extern "C" fn vg_b58_encode_check(
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut result: *mut libc::c_char,
) {
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 32] = [0; 32];
    let mut d: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut bn: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bndiv: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bntmp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bna: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnb: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnbase: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnrem: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut binres: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut brlen: libc::c_int = 0;
    let mut zpfx: libc::c_int = 0;
    bnctx = BN_CTX_new();
    BN_init(&mut bna);
    BN_init(&mut bnb);
    BN_init(&mut bnbase);
    BN_init(&mut bnrem);
    BN_set_word(&mut bnbase, 58 as libc::c_int as libc::c_ulong);
    bn = &mut bna;
    bndiv = &mut bnb;
    brlen = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(len)
        .wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_int;
    binres = malloc(brlen as libc::c_ulong) as *mut libc::c_uchar;
    memcpy(binres as *mut libc::c_void, buf, len);
    SHA256(binres, len, hash1.as_mut_ptr());
    SHA256(
        hash1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        hash2.as_mut_ptr(),
    );
    memcpy(
        &mut *binres.offset(len as isize) as *mut libc::c_uchar as *mut libc::c_void,
        hash2.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    BN_bin2bn(
        binres,
        len.wrapping_add(4 as libc::c_int as libc::c_ulong) as libc::c_int,
        bn,
    );
    zpfx = 0 as libc::c_int;
    while (zpfx as libc::c_ulong) < len.wrapping_add(4 as libc::c_int as libc::c_ulong)
        && *binres.offset(zpfx as isize) as libc::c_int == 0 as libc::c_int
    {
        zpfx += 1;
        zpfx;
    }
    p = brlen;
    while !((*bn).top == 0 as libc::c_int) {
        BN_div(bndiv, &mut bnrem, bn, &mut bnbase, bnctx);
        bntmp = bn;
        bn = bndiv;
        bndiv = bntmp;
        d = BN_get_word(&mut bnrem) as libc::c_int;
        p -= 1;
        *binres
            .offset(p as isize) = *vg_b58_alphabet.offset(d as isize) as libc::c_uchar;
    }
    loop {
        let fresh0 = zpfx;
        zpfx = zpfx - 1;
        if !(fresh0 != 0) {
            break;
        }
        p -= 1;
        *binres
            .offset(
                p as isize,
            ) = *vg_b58_alphabet.offset(0 as libc::c_int as isize) as libc::c_uchar;
    }
    memcpy(
        result as *mut libc::c_void,
        &mut *binres.offset(p as isize) as *mut libc::c_uchar as *const libc::c_void,
        (brlen - p) as libc::c_ulong,
    );
    *result.offset((brlen - p) as isize) = '\0' as i32 as libc::c_char;
    free(binres as *mut libc::c_void);
    BN_clear_free(&mut bna);
    BN_clear_free(&mut bnb);
    BN_clear_free(&mut bnbase);
    BN_clear_free(&mut bnrem);
    BN_CTX_free(bnctx);
}
pub unsafe extern "C" fn vg_b58_decode_check(
    mut input: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut xbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bn: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnw: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnbase: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 32] = [0; 32];
    let mut zpfx: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    BN_init(&mut bn);
    BN_init(&mut bnw);
    BN_init(&mut bnbase);
    BN_set_word(&mut bnbase, 58 as libc::c_int as libc::c_ulong);
    bnctx = BN_CTX_new();
    l = strlen(input) as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        if !(i < l) {
            current_block = 4166486009154926805;
            break;
        }
        if !(*input.offset(i as isize) as libc::c_int == '\r' as i32
            || *input.offset(i as isize) as libc::c_int == '\n' as i32
            || *input.offset(i as isize) as libc::c_int == ' ' as i32
            || *input.offset(i as isize) as libc::c_int == '\t' as i32)
        {
            c = vg_b58_reverse_map[*input.offset(i as isize) as libc::c_int as usize]
                as libc::c_int;
            if c < 0 as libc::c_int {
                current_block = 15276083285793946289;
                break;
            }
            BN_clear(&mut bnw);
            BN_set_word(&mut bnw, c as libc::c_ulong);
            BN_mul(&mut bn, &mut bn, &mut bnbase, bnctx);
            BN_add(&mut bn, &mut bn, &mut bnw);
        }
        i += 1;
        i;
    }
    match current_block {
        4166486009154926805 => {
            i = 0 as libc::c_int;
            zpfx = 0 as libc::c_int;
            while *input.offset(i as isize) != 0 {
                if !(*input.offset(i as isize) as libc::c_int == '\r' as i32
                    || *input.offset(i as isize) as libc::c_int == '\n' as i32
                    || *input.offset(i as isize) as libc::c_int == ' ' as i32
                    || *input.offset(i as isize) as libc::c_int == '\t' as i32)
                {
                    if *input.offset(i as isize) as libc::c_int
                        != *vg_b58_alphabet.offset(0 as libc::c_int as isize)
                            as libc::c_int
                    {
                        break;
                    }
                    zpfx += 1;
                    zpfx;
                }
                i += 1;
                i;
            }
            c = (BN_num_bits(&mut bn) + 7 as libc::c_int) / 8 as libc::c_int;
            l = zpfx + c;
            if !(l < 5 as libc::c_int) {
                xbuf = malloc(l as libc::c_ulong) as *mut libc::c_uchar;
                if !xbuf.is_null() {
                    if zpfx != 0 {
                        memset(
                            xbuf as *mut libc::c_void,
                            0 as libc::c_int,
                            zpfx as libc::c_ulong,
                        );
                    }
                    if c != 0 {
                        BN_bn2bin(&mut bn, xbuf.offset(zpfx as isize));
                    }
                    l -= 4 as libc::c_int;
                    SHA256(xbuf, l as size_t, hash1.as_mut_ptr());
                    SHA256(
                        hash1.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
                        hash2.as_mut_ptr(),
                    );
                    if !(memcmp(
                        hash2.as_mut_ptr() as *const libc::c_void,
                        xbuf.offset(l as isize) as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    ) != 0)
                    {
                        if len != 0 {
                            if len > l as libc::c_ulong {
                                len = l as size_t;
                            }
                            memcpy(buf, xbuf as *const libc::c_void, len);
                        }
                        res = l;
                    }
                }
            }
        }
        _ => {}
    }
    if !xbuf.is_null() {
        free(xbuf as *mut libc::c_void);
    }
    BN_clear_free(&mut bn);
    BN_clear_free(&mut bnw);
    BN_clear_free(&mut bnbase);
    BN_CTX_free(bnctx);
    return res;
}
pub unsafe extern "C" fn vg_encode_address(
    mut ppoint: *const EC_POINT,
    mut pgroup: *const EC_GROUP,
    mut addrtype: libc::c_int,
    mut result: *mut libc::c_char,
) {
    let mut eckey_buf: [libc::c_uchar; 128] = [0; 128];
    let mut pend: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut binres: [libc::c_uchar; 21] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    pend = eckey_buf.as_mut_ptr();
    EC_POINT_point2oct(
        pgroup,
        ppoint,
        POINT_CONVERSION_UNCOMPRESSED,
        eckey_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong,
        0 as *mut BN_CTX,
    );
    pend = eckey_buf.as_mut_ptr().offset(0x41 as libc::c_int as isize);
    binres[0 as libc::c_int as usize] = addrtype as libc::c_uchar;
    SHA256(
        eckey_buf.as_mut_ptr(),
        pend.offset_from(eckey_buf.as_mut_ptr()) as libc::c_long as size_t,
        hash1.as_mut_ptr(),
    );
    RIPEMD160(
        hash1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        &mut *binres.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    vg_b58_encode_check(
        binres.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 21]>() as libc::c_ulong,
        result,
    );
}
pub unsafe extern "C" fn vg_encode_script_address(
    mut ppoint: *const EC_POINT,
    mut pgroup: *const EC_GROUP,
    mut addrtype: libc::c_int,
    mut result: *mut libc::c_char,
) {
    let mut script_buf: [libc::c_uchar; 69] = [0; 69];
    let mut eckey_buf: *mut libc::c_uchar = script_buf
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut binres: [libc::c_uchar; 21] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    script_buf[0 as libc::c_int as usize] = 0x51 as libc::c_int as libc::c_uchar;
    script_buf[1 as libc::c_int as usize] = 0x41 as libc::c_int as libc::c_uchar;
    script_buf[67 as libc::c_int as usize] = 0x51 as libc::c_int as libc::c_uchar;
    script_buf[68 as libc::c_int as usize] = 0xae as libc::c_int as libc::c_uchar;
    EC_POINT_point2oct(
        pgroup,
        ppoint,
        POINT_CONVERSION_UNCOMPRESSED,
        eckey_buf,
        65 as libc::c_int as size_t,
        0 as *mut BN_CTX,
    );
    binres[0 as libc::c_int as usize] = addrtype as libc::c_uchar;
    SHA256(script_buf.as_mut_ptr(), 69 as libc::c_int as size_t, hash1.as_mut_ptr());
    RIPEMD160(
        hash1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        &mut *binres.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    vg_b58_encode_check(
        binres.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 21]>() as libc::c_ulong,
        result,
    );
}
pub unsafe extern "C" fn vg_encode_privkey(
    mut pkey: *const EC_KEY,
    mut addrtype: libc::c_int,
    mut result: *mut libc::c_char,
) {
    let mut eckey_buf: [libc::c_uchar; 128] = [0; 128];
    let mut bn: *const BIGNUM = 0 as *const BIGNUM;
    let mut nbytes: libc::c_int = 0;
    bn = EC_KEY_get0_private_key(pkey);
    eckey_buf[0 as libc::c_int as usize] = addrtype as libc::c_uchar;
    nbytes = (BN_num_bits(bn) + 7 as libc::c_int) / 8 as libc::c_int;
    if nbytes <= 32 as libc::c_int {} else {
        __assert_fail(
            b"nbytes <= 32\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void vg_encode_privkey(const EC_KEY *, int, char *)\0"))
                .as_ptr(),
        );
    }
    'c_18221: {
        if nbytes <= 32 as libc::c_int {} else {
            __assert_fail(
                b"nbytes <= 32\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                301 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void vg_encode_privkey(const EC_KEY *, int, char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if nbytes < 32 as libc::c_int {
        memset(
            eckey_buf.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int - nbytes) as libc::c_ulong,
        );
    }
    BN_bn2bin(
        bn,
        &mut *eckey_buf.as_mut_ptr().offset((33 as libc::c_int - nbytes) as isize),
    );
    vg_b58_encode_check(
        eckey_buf.as_mut_ptr() as *mut libc::c_void,
        33 as libc::c_int as size_t,
        result,
    );
}
pub unsafe extern "C" fn vg_set_privkey(
    mut bnpriv: *const BIGNUM,
    mut pkey: *mut EC_KEY,
) -> libc::c_int {
    let mut pgroup: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut ppnt: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut res: libc::c_int = 0;
    pgroup = EC_KEY_get0_group(pkey);
    ppnt = EC_POINT_new(pgroup);
    res = (!ppnt.is_null() && EC_KEY_set_private_key(pkey, bnpriv) != 0
        && EC_POINT_mul(
            pgroup,
            ppnt,
            bnpriv,
            0 as *const EC_POINT,
            0 as *const BIGNUM,
            0 as *mut BN_CTX,
        ) != 0 && EC_KEY_set_public_key(pkey, ppnt) != 0) as libc::c_int;
    if !ppnt.is_null() {
        EC_POINT_free(ppnt);
    }
    if res == 0 {
        return 0 as libc::c_int;
    }
    if EC_KEY_check_key(pkey) != 0 {} else {
        __assert_fail(
            b"EC_KEY_check_key(pkey)\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"int vg_set_privkey(const BIGNUM *, EC_KEY *)\0"))
                .as_ptr(),
        );
    }
    'c_18307: {
        if EC_KEY_check_key(pkey) != 0 {} else {
            __assert_fail(
                b"EC_KEY_check_key(pkey)\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"int vg_set_privkey(const BIGNUM *, EC_KEY *)\0"))
                    .as_ptr(),
            );
        }
    };
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_decode_privkey(
    mut b58encoded: *const libc::c_char,
    mut pkey: *mut EC_KEY,
    mut addrtype: *mut libc::c_int,
) -> libc::c_int {
    let mut bnpriv: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut ecpriv: [libc::c_uchar; 48] = [0; 48];
    let mut res: libc::c_int = 0;
    res = vg_b58_decode_check(
        b58encoded,
        ecpriv.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 48]>() as libc::c_ulong,
    );
    if res != 33 as libc::c_int {
        return 0 as libc::c_int;
    }
    BN_init(&mut bnpriv);
    BN_bin2bn(
        ecpriv.as_mut_ptr().offset(1 as libc::c_int as isize),
        res - 1 as libc::c_int,
        &mut bnpriv,
    );
    res = vg_set_privkey(&mut bnpriv, pkey);
    BN_clear_free(&mut bnpriv);
    *addrtype = ecpriv[0 as libc::c_int as usize] as libc::c_int;
    return 1 as libc::c_int;
}
static mut protkey_parameters: [vg_protkey_parameters_t; 17] = unsafe {
    [
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 4096 as libc::c_int,
                pbkdf_hash_getter: Some(
                    EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD,
                ),
                cipher_getter: Some(
                    EVP_aes_256_cbc as unsafe extern "C" fn() -> *const EVP_CIPHER,
                ),
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: None,
                cipher_getter: None,
            };
            init
        },
        {
            let mut init = vg_protkey_parameters_t {
                mode: 1 as libc::c_int,
                iterations: 4096 as libc::c_int,
                pbkdf_hash_getter: Some(
                    EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD,
                ),
                cipher_getter: Some(
                    EVP_aes_256_cbc as unsafe extern "C" fn() -> *const EVP_CIPHER,
                ),
            };
            init
        },
    ]
};
unsafe extern "C" fn vg_protect_crypt(
    mut parameter_group: libc::c_int,
    mut data_in: *mut libc::c_uchar,
    mut data_in_len: libc::c_int,
    mut data_out: *mut libc::c_uchar,
    mut pass: *const libc::c_char,
    mut enc: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ctx: *mut EVP_CIPHER_CTX = 0 as *mut EVP_CIPHER_CTX;
    let mut salt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut keymaterial: [libc::c_uchar; 144] = [0; 144];
    let mut hmac: [libc::c_uchar; 64] = [0; 64];
    let mut hmac_len: libc::c_int = 0 as libc::c_int;
    let mut hmac_keylen: libc::c_int = 0 as libc::c_int;
    let mut salt_len: libc::c_int = 0;
    let mut plaintext_len: libc::c_int = 32 as libc::c_int;
    let mut ciphertext_len: libc::c_int = 0;
    let mut pkcs7_padding: libc::c_int = 1 as libc::c_int;
    let mut params: *const vg_protkey_parameters_t = 0 as *const vg_protkey_parameters_t;
    let mut cipher: *const EVP_CIPHER = 0 as *const EVP_CIPHER;
    let mut pbkdf_digest: *const EVP_MD = 0 as *const EVP_MD;
    let mut hmac_digest: *const EVP_MD = 0 as *const EVP_MD;
    let mut hlen: libc::c_uint = 0;
    let mut opos: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut oincr: libc::c_int = 0;
    let mut nbytes: libc::c_int = 0;
    let mut ipos: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    ctx = EVP_CIPHER_CTX_new();
    if !ctx.is_null() {
        if parameter_group < 0 as libc::c_int {
            if enc != 0 {
                parameter_group = 0 as libc::c_int;
            } else {
                parameter_group = *data_in.offset(0 as libc::c_int as isize)
                    as libc::c_int;
            }
            current_block = 3512920355445576850;
        } else if enc == 0
            && parameter_group
                != *data_in.offset(0 as libc::c_int as isize) as libc::c_int
        {
            current_block = 15409601725402290189;
        } else {
            current_block = 3512920355445576850;
        }
        match current_block {
            15409601725402290189 => {}
            _ => {
                if !(parameter_group as libc::c_ulong
                    > (::std::mem::size_of::<[vg_protkey_parameters_t; 17]>()
                        as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<vg_protkey_parameters_t>()
                                as libc::c_ulong,
                        ))
                {
                    params = &*protkey_parameters
                        .as_ptr()
                        .offset(parameter_group as isize)
                        as *const vg_protkey_parameters_t;
                    if !((*params).iterations == 0
                        || ((*params).pbkdf_hash_getter).is_none())
                    {
                        pbkdf_digest = ((*params).pbkdf_hash_getter).unwrap()();
                        cipher = ((*params).cipher_getter).unwrap()();
                        if (*params).mode == 0 as libc::c_int {
                            salt_len = 4 as libc::c_int;
                            hmac_len = 8 as libc::c_int;
                            hmac_keylen = 16 as libc::c_int;
                            ciphertext_len = (plaintext_len + (*cipher).block_size
                                - 1 as libc::c_int) / (*cipher).block_size
                                * (*cipher).block_size;
                            pkcs7_padding = 0 as libc::c_int;
                            hmac_digest = EVP_sha256();
                        } else {
                            salt_len = 8 as libc::c_int;
                            ciphertext_len = (plaintext_len + (*cipher).block_size)
                                / (*cipher).block_size * (*cipher).block_size;
                            hmac_digest = 0 as *const EVP_MD;
                        }
                        if !(enc == 0
                            && data_in_len
                                != 1 as libc::c_int + ciphertext_len + hmac_len + salt_len)
                        {
                            if pass.is_null() || data_out.is_null() {
                                ret = plaintext_len;
                            } else {
                                if enc == 0 {
                                    salt = data_in
                                        .offset(1 as libc::c_int as isize)
                                        .offset(ciphertext_len as isize)
                                        .offset(hmac_len as isize);
                                } else if salt_len != 0 {
                                    salt = data_out
                                        .offset(1 as libc::c_int as isize)
                                        .offset(ciphertext_len as isize)
                                        .offset(hmac_len as isize);
                                    RAND_bytes(salt, salt_len);
                                } else {
                                    salt = 0 as *mut libc::c_uchar;
                                }
                                PKCS5_PBKDF2_HMAC(
                                    pass,
                                    (strlen(pass))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int,
                                    salt,
                                    salt_len,
                                    (*params).iterations,
                                    pbkdf_digest,
                                    (*cipher).key_len + (*cipher).iv_len + hmac_keylen,
                                    keymaterial.as_mut_ptr(),
                                );
                                if EVP_CipherInit(
                                    ctx,
                                    cipher,
                                    keymaterial.as_mut_ptr(),
                                    keymaterial.as_mut_ptr().offset((*cipher).key_len as isize),
                                    enc,
                                ) == 0
                                {
                                    fprintf(
                                        stderr,
                                        b"ERROR: could not configure cipher\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    if pkcs7_padding == 0 {
                                        EVP_CIPHER_CTX_set_padding(ctx, 0 as libc::c_int);
                                    }
                                    if enc == 0 {
                                        opos = 0 as libc::c_int;
                                        olen = plaintext_len;
                                        nbytes = ciphertext_len;
                                        ipos = 1 as libc::c_int;
                                    } else {
                                        *data_out
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = parameter_group as libc::c_uchar;
                                        opos = 1 as libc::c_int;
                                        olen = 1 as libc::c_int + ciphertext_len + hmac_len
                                            + salt_len - opos;
                                        nbytes = plaintext_len;
                                        ipos = 0 as libc::c_int;
                                    }
                                    oincr = olen;
                                    if EVP_CipherUpdate(
                                        ctx,
                                        data_out.offset(opos as isize),
                                        &mut oincr,
                                        data_in.offset(ipos as isize),
                                        nbytes,
                                    ) == 0
                                    {
                                        current_block = 5688892970823152787;
                                    } else {
                                        opos += oincr;
                                        olen -= oincr;
                                        oincr = olen;
                                        if EVP_CipherFinal(
                                            ctx,
                                            data_out.offset(opos as isize),
                                            &mut oincr,
                                        ) == 0
                                        {
                                            current_block = 5688892970823152787;
                                        } else {
                                            opos += oincr;
                                            if hmac_len != 0 {
                                                hlen = ::std::mem::size_of::<[libc::c_uchar; 64]>()
                                                    as libc::c_ulong as libc::c_uint;
                                                HMAC(
                                                    hmac_digest,
                                                    keymaterial
                                                        .as_mut_ptr()
                                                        .offset((*cipher).key_len as isize)
                                                        .offset((*cipher).iv_len as isize) as *const libc::c_void,
                                                    hmac_keylen,
                                                    if enc != 0 { data_in } else { data_out },
                                                    plaintext_len as size_t,
                                                    hmac.as_mut_ptr(),
                                                    &mut hlen,
                                                );
                                                if enc != 0 {
                                                    memcpy(
                                                        data_out
                                                            .offset(1 as libc::c_int as isize)
                                                            .offset(ciphertext_len as isize) as *mut libc::c_void,
                                                        hmac.as_mut_ptr() as *const libc::c_void,
                                                        hmac_len as libc::c_ulong,
                                                    );
                                                    current_block = 4741994311446740739;
                                                } else if memcmp(
                                                    hmac.as_mut_ptr() as *const libc::c_void,
                                                    data_in
                                                        .offset(1 as libc::c_int as isize)
                                                        .offset(ciphertext_len as isize) as *const libc::c_void,
                                                    hmac_len as libc::c_ulong,
                                                ) != 0
                                                {
                                                    current_block = 5688892970823152787;
                                                } else {
                                                    current_block = 4741994311446740739;
                                                }
                                            } else {
                                                current_block = 4741994311446740739;
                                            }
                                            match current_block {
                                                5688892970823152787 => {}
                                                _ => {
                                                    if enc != 0 {
                                                        if opos != 1 as libc::c_int + ciphertext_len {
                                                            fprintf(
                                                                stderr,
                                                                b"ERROR: plaintext size mismatch\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                            );
                                                            current_block = 15409601725402290189;
                                                        } else {
                                                            opos += hmac_len + salt_len;
                                                            current_block = 6243635450180130569;
                                                        }
                                                    } else if opos != plaintext_len {
                                                        fprintf(
                                                            stderr,
                                                            b"ERROR: plaintext size mismatch\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        current_block = 15409601725402290189;
                                                    } else {
                                                        current_block = 6243635450180130569;
                                                    }
                                                    match current_block {
                                                        15409601725402290189 => {}
                                                        _ => {
                                                            ret = opos;
                                                            current_block = 15409601725402290189;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    match current_block {
                                        15409601725402290189 => {}
                                        _ => {
                                            fprintf(
                                                stderr,
                                                b"ERROR: Invalid password\n\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    OPENSSL_cleanse(
        hmac.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
    );
    OPENSSL_cleanse(
        keymaterial.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 144]>() as libc::c_ulong,
    );
    if !ctx.is_null() {
        EVP_CIPHER_CTX_free(ctx);
    }
    return ret;
}
pub unsafe extern "C" fn vg_protect_encode_privkey(
    mut out: *mut libc::c_char,
    mut pkey: *const EC_KEY,
    mut keytype: libc::c_int,
    mut parameter_group: libc::c_int,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut ecpriv: [libc::c_uchar; 64] = [0; 64];
    let mut ecenc: [libc::c_uchar; 128] = [0; 128];
    let mut privkey: *const BIGNUM = 0 as *const BIGNUM;
    let mut nbytes: libc::c_int = 0;
    let mut restype: libc::c_int = 0;
    restype = if keytype & 1 as libc::c_int != 0 {
        79 as libc::c_int
    } else {
        32 as libc::c_int
    };
    privkey = EC_KEY_get0_private_key(pkey);
    nbytes = (BN_num_bits(privkey) + 7 as libc::c_int) / 8 as libc::c_int;
    if nbytes < 32 as libc::c_int {
        memset(
            ecpriv.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int - nbytes) as libc::c_ulong,
        );
    }
    BN_bn2bin(
        privkey,
        ecpriv.as_mut_ptr().offset(32 as libc::c_int as isize).offset(-(nbytes as isize)),
    );
    nbytes = vg_protect_crypt(
        parameter_group,
        ecpriv.as_mut_ptr(),
        32 as libc::c_int,
        &mut *ecenc.as_mut_ptr().offset(1 as libc::c_int as isize),
        pass,
        1 as libc::c_int,
    );
    if nbytes <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    OPENSSL_cleanse(
        ecpriv.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
    );
    ecenc[0 as libc::c_int as usize] = restype as libc::c_uchar;
    vg_b58_encode_check(
        ecenc.as_mut_ptr() as *mut libc::c_void,
        (nbytes + 1 as libc::c_int) as size_t,
        out,
    );
    nbytes = strlen(out) as libc::c_int;
    return nbytes;
}
pub unsafe extern "C" fn vg_protect_decode_privkey(
    mut pkey: *mut EC_KEY,
    mut keytype: *mut libc::c_int,
    mut encoded: *const libc::c_char,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut ecpriv: [libc::c_uchar; 64] = [0; 64];
    let mut ecenc: [libc::c_uchar; 128] = [0; 128];
    let mut bn: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut restype: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    res = vg_b58_decode_check(
        encoded,
        ecenc.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong,
    );
    if res < 2 as libc::c_int
        || res as libc::c_ulong
            > ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    match ecenc[0 as libc::c_int as usize] as libc::c_int {
        32 => {
            restype = 128 as libc::c_int;
        }
        79 => {
            restype = 239 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    }
    if vg_protect_crypt(
        -(1 as libc::c_int),
        ecenc.as_mut_ptr().offset(1 as libc::c_int as isize),
        res - 1 as libc::c_int,
        if !pkey.is_null() { ecpriv.as_mut_ptr() } else { 0 as *mut libc::c_uchar },
        pass,
        0 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    res = 1 as libc::c_int;
    if !pkey.is_null() {
        BN_init(&mut bn);
        BN_bin2bn(ecpriv.as_mut_ptr(), 32 as libc::c_int, &mut bn);
        res = vg_set_privkey(&mut bn, pkey);
        BN_clear_free(&mut bn);
        OPENSSL_cleanse(
            ecpriv.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
        );
    }
    *keytype = restype;
    return res;
}
pub unsafe extern "C" fn vg_pkcs8_encode_privkey(
    mut out: *mut libc::c_char,
    mut outlen: libc::c_int,
    mut pkey: *const EC_KEY,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pkey_copy: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut evp_key: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut pkcs8: *mut PKCS8_PRIV_KEY_INFO = 0 as *mut PKCS8_PRIV_KEY_INFO;
    let mut pkcs8_enc: *mut X509_SIG = 0 as *mut X509_SIG;
    let mut memptr: *mut BUF_MEM = 0 as *mut BUF_MEM;
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut res: libc::c_int = 0 as libc::c_int;
    pkey_copy = EC_KEY_dup(pkey);
    if !pkey_copy.is_null() {
        evp_key = EVP_PKEY_new();
        if !(evp_key.is_null() || EVP_PKEY_set1_EC_KEY(evp_key, pkey_copy) == 0) {
            pkcs8 = EVP_PKEY2PKCS8(evp_key);
            if !pkcs8.is_null() {
                bio = BIO_new(BIO_s_mem());
                if !bio.is_null() {
                    if pass.is_null() {
                        res = PEM_write_bio_PKCS8_PRIV_KEY_INFO(bio, pkcs8);
                        current_block = 12599329904712511516;
                    } else {
                        pkcs8_enc = PKCS8_encrypt(
                            -(1 as libc::c_int),
                            EVP_aes_256_cbc(),
                            pass,
                            strlen(pass) as libc::c_int,
                            0 as *mut libc::c_uchar,
                            0 as libc::c_int,
                            4096 as libc::c_int,
                            pkcs8,
                        );
                        if pkcs8_enc.is_null() {
                            current_block = 1087353680672421626;
                        } else {
                            res = PEM_write_bio_PKCS8(bio, pkcs8_enc);
                            current_block = 12599329904712511516;
                        }
                    }
                    match current_block {
                        1087353680672421626 => {}
                        _ => {
                            BIO_ctrl(
                                bio,
                                115 as libc::c_int,
                                0 as libc::c_int as libc::c_long,
                                &mut memptr as *mut *mut BUF_MEM as *mut libc::c_char
                                    as *mut libc::c_void,
                            );
                            res = (*memptr).length as libc::c_int;
                            if res < outlen {
                                memcpy(
                                    out as *mut libc::c_void,
                                    (*memptr).data as *const libc::c_void,
                                    res as libc::c_ulong,
                                );
                                *out.offset(res as isize) = '\0' as i32 as libc::c_char;
                            } else {
                                memcpy(
                                    out as *mut libc::c_void,
                                    (*memptr).data as *const libc::c_void,
                                    (outlen - 1 as libc::c_int) as libc::c_ulong,
                                );
                                *out
                                    .offset(
                                        (outlen - 1 as libc::c_int) as isize,
                                    ) = '\0' as i32 as libc::c_char;
                            }
                        }
                    }
                }
            }
        }
    }
    if !bio.is_null() {
        BIO_free(bio);
    }
    if !pkey_copy.is_null() {
        EC_KEY_free(pkey_copy);
    }
    if !evp_key.is_null() {
        EVP_PKEY_free(evp_key);
    }
    if !pkcs8.is_null() {
        PKCS8_PRIV_KEY_INFO_free(pkcs8);
    }
    if !pkcs8_enc.is_null() {
        X509_SIG_free(pkcs8_enc);
    }
    return res;
}
pub unsafe extern "C" fn vg_pkcs8_decode_privkey(
    mut pkey: *mut EC_KEY,
    mut pem_in: *const libc::c_char,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut pkey_in: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut test_key: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut evp_key: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut pkcs8: *mut PKCS8_PRIV_KEY_INFO = 0 as *mut PKCS8_PRIV_KEY_INFO;
    let mut pkcs8_enc: *mut X509_SIG = 0 as *mut X509_SIG;
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut res: libc::c_int = 0 as libc::c_int;
    bio = BIO_new_mem_buf(
        pem_in as *mut libc::c_char as *mut libc::c_void,
        strlen(pem_in) as libc::c_int,
    );
    if !bio.is_null() {
        pkcs8_enc = PEM_read_bio_PKCS8(
            bio,
            0 as *mut *mut X509_SIG,
            None,
            0 as *mut libc::c_void,
        );
        if !pkcs8_enc.is_null() {
            if pass.is_null() {
                return -(1 as libc::c_int);
            }
            pkcs8 = PKCS8_decrypt(pkcs8_enc, pass, strlen(pass) as libc::c_int);
        } else {
            BIO_ctrl(
                bio,
                1 as libc::c_int,
                0 as libc::c_int as libc::c_long,
                0 as *mut libc::c_void,
            );
            pkcs8 = PEM_read_bio_PKCS8_PRIV_KEY_INFO(
                bio,
                0 as *mut *mut PKCS8_PRIV_KEY_INFO,
                None,
                0 as *mut libc::c_void,
            );
        }
        if !pkcs8.is_null() {
            evp_key = EVP_PKCS82PKEY(pkcs8);
            if !evp_key.is_null() {
                pkey_in = EVP_PKEY_get1_EC_KEY(evp_key);
                if !pkey_in.is_null() {
                    test_key = EC_KEY_new_by_curve_name(714 as libc::c_int);
                    if !(test_key.is_null()
                        || EC_GROUP_cmp(
                            EC_KEY_get0_group(pkey_in),
                            EC_KEY_get0_group(test_key),
                            0 as *mut BN_CTX,
                        ) != 0)
                    {
                        if !(EC_KEY_copy(pkey, pkey_in)).is_null() {
                            res = 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    if !bio.is_null() {
        BIO_free(bio);
    }
    if !test_key.is_null() {
        EC_KEY_free(pkey_in);
    }
    if !evp_key.is_null() {
        EVP_PKEY_free(evp_key);
    }
    if !pkcs8.is_null() {
        PKCS8_PRIV_KEY_INFO_free(pkcs8);
    }
    if !pkcs8_enc.is_null() {
        X509_SIG_free(pkcs8_enc);
    }
    return res;
}
pub unsafe extern "C" fn vg_decode_privkey_any(
    mut pkey: *mut EC_KEY,
    mut addrtype: *mut libc::c_int,
    mut input: *const libc::c_char,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    if vg_decode_privkey(input, pkey, addrtype) != 0 {
        return 1 as libc::c_int;
    }
    if vg_protect_decode_privkey(pkey, addrtype, input, 0 as *const libc::c_char) != 0 {
        if pass.is_null() {
            return -(1 as libc::c_int);
        }
        return vg_protect_decode_privkey(pkey, addrtype, input, pass);
    }
    res = vg_pkcs8_decode_privkey(pkey, input, pass);
    if res > 0 as libc::c_int {
        *addrtype = 128 as libc::c_int;
    }
    return res;
}
pub unsafe extern "C" fn vg_read_password(
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    return (EVP_read_pw_string(
        buf,
        size as libc::c_int,
        b"Enter new password:\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) == 0) as libc::c_int;
}
static mut ascii_class: [libc::c_uchar; 128] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn vg_check_password_complexity(
    mut pass: *const libc::c_char,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut classes: [libc::c_int; 6] = [0 as libc::c_int, 0, 0, 0, 0, 0];
    let mut crackunit: *const libc::c_char = b"seconds\0" as *const u8
        as *const libc::c_char;
    let mut char_complexity: libc::c_int = 0 as libc::c_int;
    let mut crackops: libc::c_double = 0.;
    let mut cracktime: libc::c_double = 0.;
    let mut weak: libc::c_int = 0;
    let rate: libc::c_int = 250000000 as libc::c_int;
    let weak_threshold: libc::c_int = 60 as libc::c_int * 60 as libc::c_int
        * 24 as libc::c_int * 365 as libc::c_int;
    len = strlen(pass) as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        if *pass.offset(i as isize) as libc::c_ulong
            > ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong
        {
            classes[5 as libc::c_int as usize] += 1;
            classes[5 as libc::c_int as usize];
        } else if !(ascii_class[*pass.offset(i as isize) as libc::c_int as usize] == 0) {
            classes[(ascii_class[*pass.offset(i as isize) as libc::c_int as usize]
                as libc::c_int - 1 as libc::c_int) as usize] += 1;
            classes[(ascii_class[*pass.offset(i as isize) as libc::c_int as usize]
                as libc::c_int - 1 as libc::c_int) as usize];
        }
        i += 1;
        i;
    }
    if classes[0 as libc::c_int as usize] != 0 {
        char_complexity += 26 as libc::c_int;
    }
    if classes[1 as libc::c_int as usize] != 0 {
        char_complexity += 26 as libc::c_int;
    }
    if classes[2 as libc::c_int as usize] != 0 {
        char_complexity += 10 as libc::c_int;
    }
    if classes[3 as libc::c_int as usize] != 0 {
        char_complexity += 14 as libc::c_int;
    }
    if classes[4 as libc::c_int as usize] != 0 {
        char_complexity += 19 as libc::c_int;
    }
    if classes[5 as libc::c_int as usize] != 0 {
        char_complexity += 32 as libc::c_int;
    }
    crackops = pow(char_complexity as libc::c_double, len as libc::c_double);
    cracktime = crackops
        * (1 as libc::c_int as libc::c_double
            - 1 as libc::c_int as libc::c_double / 2.7182818284590452354f64)
        / rate as libc::c_double;
    weak = (cracktime < weak_threshold as libc::c_double) as libc::c_int;
    if cracktime > 60.0f64 {
        cracktime /= 60.0f64;
        crackunit = b"minutes\0" as *const u8 as *const libc::c_char;
        if cracktime > 60.0f64 {
            cracktime /= 60.0f64;
            crackunit = b"hours\0" as *const u8 as *const libc::c_char;
            if cracktime > 24.0f64 {
                cracktime /= 24 as libc::c_int as libc::c_double;
                crackunit = b"days\0" as *const u8 as *const libc::c_char;
                if cracktime > 365.0f64 {
                    cracktime /= 365.0f64;
                    crackunit = b"years\0" as *const u8 as *const libc::c_char;
                }
            }
        }
    }
    if weak != 0 && verbose > 0 as libc::c_int || verbose > 1 as libc::c_int {
        if cracktime < 1.0f64 {
            fprintf(
                stderr,
                b"Estimated password crack time: >1 %s\n\0" as *const u8
                    as *const libc::c_char,
                crackunit,
            );
        } else if cracktime < 1000000 as libc::c_int as libc::c_double {
            fprintf(
                stderr,
                b"Estimated password crack time: %.1f %s\n\0" as *const u8
                    as *const libc::c_char,
                cracktime,
                crackunit,
            );
        } else {
            fprintf(
                stderr,
                b"Estimated password crack time: %e %s\n\0" as *const u8
                    as *const libc::c_char,
                cracktime,
                crackunit,
            );
        }
        if classes[0 as libc::c_int as usize] == 0
            && classes[1 as libc::c_int as usize] == 0
            && classes[2 as libc::c_int as usize] != 0
            && classes[3 as libc::c_int as usize] == 0
            && classes[4 as libc::c_int as usize] == 0
            && classes[5 as libc::c_int as usize] == 0
        {
            fprintf(
                stderr,
                b"WARNING: Password contains only numbers\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if classes[2 as libc::c_int as usize] == 0
            && classes[3 as libc::c_int as usize] == 0
            && classes[4 as libc::c_int as usize] == 0
            && classes[5 as libc::c_int as usize] == 0
        {
            if classes[0 as libc::c_int as usize] == 0
                || classes[1 as libc::c_int as usize] == 0
            {
                fprintf(
                    stderr,
                    b"WARNING: Password contains only %scase letters\n\0" as *const u8
                        as *const libc::c_char,
                    if classes[0 as libc::c_int as usize] != 0 {
                        b"lower\0" as *const u8 as *const libc::c_char
                    } else {
                        b"upper\0" as *const u8 as *const libc::c_char
                    },
                );
            } else {
                fprintf(
                    stderr,
                    b"WARNING: Password contains only letters\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    return (weak == 0) as libc::c_int;
}
pub unsafe extern "C" fn vg_read_file(
    mut fp: *mut FILE,
    mut result: *mut *mut *mut libc::c_char,
    mut rescount: *mut libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut patterns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pat: *mut libc::c_char = 0 as *mut libc::c_char;
    let blksize: libc::c_int = 16 as libc::c_int * 1024 as libc::c_int;
    let mut nalloc: libc::c_int = 16 as libc::c_int;
    let mut npatterns: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    patterns = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nalloc as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    count = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    loop {
        obuf = buf;
        buf = malloc(blksize as libc::c_ulong) as *mut libc::c_char;
        if buf.is_null() {
            ret = 0 as libc::c_int;
            break;
        } else {
            if pos < count {
                memcpy(
                    buf as *mut libc::c_void,
                    &mut *obuf.offset(pos as isize) as *mut libc::c_char
                        as *const libc::c_void,
                    (count - pos) as libc::c_ulong,
                );
            }
            pos = count - pos;
            count = fread(
                &mut *buf.offset(pos as isize) as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                (blksize - pos) as libc::c_ulong,
                fp,
            ) as libc::c_int;
            if count < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Error reading file: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                ret = 0 as libc::c_int;
            }
            if count <= 0 as libc::c_int {
                break;
            }
            count += pos;
            pat = buf;
            while pos < count {
                if *buf.offset(pos as isize) as libc::c_int == '\r' as i32
                    || *buf.offset(pos as isize) as libc::c_int == '\n' as i32
                {
                    *buf.offset(pos as isize) = '\0' as i32 as libc::c_char;
                    if !pat.is_null() {
                        if npatterns == nalloc {
                            nalloc *= 2 as libc::c_int;
                            patterns = realloc(
                                patterns as *mut libc::c_void,
                                (::std::mem::size_of::<*mut libc::c_char>()
                                    as libc::c_ulong)
                                    .wrapping_mul(nalloc as libc::c_ulong),
                            ) as *mut *mut libc::c_char;
                        }
                        let ref mut fresh1 = *patterns.offset(npatterns as isize);
                        *fresh1 = pat;
                        npatterns += 1;
                        npatterns;
                        pat = 0 as *mut libc::c_char;
                    }
                } else if pat.is_null() {
                    pat = &mut *buf.offset(pos as isize) as *mut libc::c_char;
                }
                pos += 1;
                pos;
            }
            pos = (if !pat.is_null() {
                pat.offset_from(buf) as libc::c_long
            } else {
                count as libc::c_long
            }) as libc::c_int;
        }
    }
    *result = patterns;
    *rescount = npatterns;
    return ret;
}
