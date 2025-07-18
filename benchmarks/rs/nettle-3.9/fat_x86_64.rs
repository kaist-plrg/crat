use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn secure_getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn _nettle_cpuid(input: uint32_t, regs: *mut uint32_t);
    fn _nettle_aes128_encrypt_c(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes128_encrypt_aesni(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes128_decrypt_c(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes128_decrypt_aesni(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_encrypt_c(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_encrypt_aesni(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_decrypt_c(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_decrypt_aesni(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_encrypt_c(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_encrypt_aesni(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_decrypt_c(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_decrypt_aesni(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes128_encrypt_c(
        _: *const aes128_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes128_encrypt_aesni(
        _: *const aes128_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes192_encrypt_c(
        _: *const aes192_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes192_encrypt_aesni(
        _: *const aes192_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes256_encrypt_c(
        _: *const aes256_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes256_encrypt_aesni(
        _: *const aes256_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_memxor_x86_64(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
    fn _nettle_memxor_sse2(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
    fn _nettle_sha1_compress_x86_64(_: *mut uint32_t, _: *const uint8_t);
    fn _nettle_sha1_compress_sha_ni(_: *mut uint32_t, _: *const uint8_t);
    fn _nettle_sha256_compress_n_x86_64(
        _: *mut uint32_t,
        _: *const uint32_t,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_sha256_compress_n_sha_ni(
        _: *mut uint32_t,
        _: *const uint32_t,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_ghash_set_key_c(_: *mut gcm_key, _: *const nettle_block16);
    fn _nettle_ghash_set_key_pclmul(_: *mut gcm_key, _: *const nettle_block16);
    fn _nettle_ghash_update_table(
        _: *const gcm_key,
        _: *mut nettle_block16,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_ghash_update_pclmul(
        _: *const gcm_key,
        _: *mut nettle_block16,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
pub type aes128_crypt_func = unsafe extern "C" fn(
    *const aes128_ctx,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type memxor_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub const X86_INTEL: x86_vendor = 1;
pub type x86_vendor = libc::c_uint;
pub const X86_AMD: x86_vendor = 2;
pub const X86_OTHER: x86_vendor = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x86_features {
    pub vendor: x86_vendor,
    pub have_aesni: libc::c_int,
    pub have_sha_ni: libc::c_int,
    pub have_pclmul: libc::c_int,
}
pub type ghash_update_func = unsafe extern "C" fn(
    *const gcm_key,
    *mut nettle_block16,
    size_t,
    *const uint8_t,
) -> *const uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
pub type ghash_set_key_func = unsafe extern "C" fn(
    *mut gcm_key,
    *const nettle_block16,
) -> ();
pub type sha256_compress_n_func = unsafe extern "C" fn(
    *mut uint32_t,
    *const uint32_t,
    size_t,
    *const uint8_t,
) -> *const uint8_t;
pub type sha1_compress_func = unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> ();
pub type cbc_aes256_encrypt_func = unsafe extern "C" fn(
    *const aes256_ctx,
    *mut uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
pub type cbc_aes192_encrypt_func = unsafe extern "C" fn(
    *const aes192_ctx,
    *mut uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
pub type cbc_aes128_encrypt_func = unsafe extern "C" fn(
    *const aes128_ctx,
    *mut uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type aes256_crypt_func = unsafe extern "C" fn(
    *const aes256_ctx,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type aes192_crypt_func = unsafe extern "C" fn(
    *const aes192_ctx,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
unsafe extern "C" fn get_x86_features(mut features: *mut x86_features) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    (*features).vendor = X86_OTHER;
    (*features).have_aesni = 0 as libc::c_int;
    (*features).have_sha_ni = 0 as libc::c_int;
    (*features).have_pclmul = 0 as libc::c_int;
    s = secure_getenv(b"NETTLE_FAT_OVERRIDE\0" as *const u8 as *const libc::c_char);
    if !s.is_null() {
        loop {
            let mut sep: *const libc::c_char = strchr(s, ',' as i32);
            let mut length: size_t = if !sep.is_null() {
                sep.offset_from(s) as libc::c_long as size_t
            } else {
                strlen(s)
            };
            if if length >= 7 as libc::c_int as libc::c_ulong
                && memcmp(
                    s as *const libc::c_void,
                    b"vendor:\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                length = (length as libc::c_ulong)
                    .wrapping_sub(7 as libc::c_int as libc::c_ulong) as size_t as size_t;
                s = s.offset(7 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                if length == 5 as libc::c_int as libc::c_ulong
                    && memcmp(
                        s as *const libc::c_void,
                        b"intel\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    (*features).vendor = X86_INTEL;
                } else if length == 3 as libc::c_int as libc::c_ulong
                    && memcmp(
                        s as *const libc::c_void,
                        b"amd\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    (*features).vendor = X86_AMD;
                }
            } else if length == 5 as libc::c_int as libc::c_ulong
                && memcmp(
                    s as *const libc::c_void,
                    b"aesni\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*features).have_aesni = 1 as libc::c_int;
            } else if length == 6 as libc::c_int as libc::c_ulong
                && memcmp(
                    s as *const libc::c_void,
                    b"sha_ni\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*features).have_sha_ni = 1 as libc::c_int;
            } else if length == 6 as libc::c_int as libc::c_ulong
                && memcmp(
                    s as *const libc::c_void,
                    b"pclmul\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*features).have_pclmul = 1 as libc::c_int;
            }
            if sep.is_null() {
                break;
            }
            s = sep.offset(1 as libc::c_int as isize);
        }
    } else {
        let mut cpuid_data: [uint32_t; 4] = [0; 4];
        _nettle_cpuid(0 as libc::c_int as uint32_t, cpuid_data.as_mut_ptr());
        if memcmp(
            cpuid_data.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *const libc::c_void,
            b"GenuntelineI\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            12 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*features).vendor = X86_INTEL;
        } else if memcmp(
            cpuid_data.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *const libc::c_void,
            b"AuthcAMDenti\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            12 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*features).vendor = X86_AMD;
        }
        _nettle_cpuid(1 as libc::c_int as uint32_t, cpuid_data.as_mut_ptr());
        if cpuid_data[2 as libc::c_int as usize] & 0x2 as libc::c_int as libc::c_uint
            != 0
        {
            (*features).have_pclmul = 1 as libc::c_int;
        }
        if cpuid_data[2 as libc::c_int as usize]
            & 0x2000000 as libc::c_int as libc::c_uint != 0
        {
            (*features).have_aesni = 1 as libc::c_int;
        }
        _nettle_cpuid(7 as libc::c_int as uint32_t, cpuid_data.as_mut_ptr());
        if cpuid_data[1 as libc::c_int as usize]
            & 0x20000000 as libc::c_int as libc::c_uint != 0
        {
            (*features).have_sha_ni = 1 as libc::c_int;
        }
    };
}
static mut nettle_aes128_encrypt_vec: Option::<aes128_crypt_func> = unsafe {
    Some(nettle_aes128_encrypt_init as aes128_crypt_func)
};
static mut nettle_aes128_decrypt_vec: Option::<aes128_crypt_func> = unsafe {
    Some(nettle_aes128_decrypt_init as aes128_crypt_func)
};
static mut nettle_aes192_encrypt_vec: Option::<aes192_crypt_func> = unsafe {
    Some(nettle_aes192_encrypt_init as aes192_crypt_func)
};
static mut nettle_aes192_decrypt_vec: Option::<aes192_crypt_func> = unsafe {
    Some(nettle_aes192_decrypt_init as aes192_crypt_func)
};
static mut nettle_aes256_encrypt_vec: Option::<aes256_crypt_func> = unsafe {
    Some(nettle_aes256_encrypt_init as aes256_crypt_func)
};
static mut nettle_aes256_decrypt_vec: Option::<aes256_crypt_func> = unsafe {
    Some(nettle_aes256_decrypt_init as aes256_crypt_func)
};
static mut nettle_cbc_aes128_encrypt_vec: Option::<cbc_aes128_encrypt_func> = unsafe {
    Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
};
static mut nettle_cbc_aes192_encrypt_vec: Option::<cbc_aes192_encrypt_func> = unsafe {
    Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
};
static mut nettle_cbc_aes256_encrypt_vec: Option::<cbc_aes256_encrypt_func> = unsafe {
    Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
};
static mut nettle_memxor_vec: Option::<memxor_func> = unsafe {
    Some(nettle_memxor_init as memxor_func)
};
static mut nettle_sha1_compress_vec: Option::<sha1_compress_func> = unsafe {
    Some(nettle_sha1_compress_init as sha1_compress_func)
};
static mut _nettle_sha256_compress_n_vec: Option::<sha256_compress_n_func> = unsafe {
    Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
};
static mut _nettle_ghash_set_key_vec: Option::<ghash_set_key_func> = unsafe {
    Some(_nettle_ghash_set_key_init as ghash_set_key_func)
};
static mut _nettle_ghash_update_vec: Option::<ghash_update_func> = unsafe {
    Some(_nettle_ghash_update_init as ghash_update_func)
};
unsafe extern "C" fn fat_init() {
    let mut features: x86_features = x86_features {
        vendor: X86_OTHER,
        have_aesni: 0,
        have_sha_ni: 0,
        have_pclmul: 0,
    };
    let mut verbose: libc::c_int = 0;
    verbose = (getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    if verbose != 0 {
        fprintf(
            stderr,
            b"libnettle: fat library initialization.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    get_x86_features(&mut features);
    if verbose != 0 {
        let vendor_names: [*const libc::c_char; 3] = [
            b"other\0" as *const u8 as *const libc::c_char,
            b"intel\0" as *const u8 as *const libc::c_char,
            b"amd\0" as *const u8 as *const libc::c_char,
        ];
        fprintf(
            stderr,
            b"libnettle: cpu features: vendor:%s%s%s%s\n\0" as *const u8
                as *const libc::c_char,
            vendor_names[features.vendor as usize],
            if features.have_aesni != 0 {
                b",aesni\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if features.have_sha_ni != 0 {
                b",sha_ni\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if features.have_pclmul != 0 {
                b",pclmul\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if features.have_aesni != 0 {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: using aes instructions.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nettle_aes128_encrypt_vec = Some(
            _nettle_aes128_encrypt_aesni as aes128_crypt_func,
        );
        nettle_aes128_decrypt_vec = Some(
            _nettle_aes128_decrypt_aesni as aes128_crypt_func,
        );
        nettle_aes192_encrypt_vec = Some(
            _nettle_aes192_encrypt_aesni as aes192_crypt_func,
        );
        nettle_aes192_decrypt_vec = Some(
            _nettle_aes192_decrypt_aesni as aes192_crypt_func,
        );
        nettle_aes256_encrypt_vec = Some(
            _nettle_aes256_encrypt_aesni as aes256_crypt_func,
        );
        nettle_aes256_decrypt_vec = Some(
            _nettle_aes256_decrypt_aesni as aes256_crypt_func,
        );
        nettle_cbc_aes128_encrypt_vec = Some(
            _nettle_cbc_aes128_encrypt_aesni as cbc_aes128_encrypt_func,
        );
        nettle_cbc_aes192_encrypt_vec = Some(
            _nettle_cbc_aes192_encrypt_aesni as cbc_aes192_encrypt_func,
        );
        nettle_cbc_aes256_encrypt_vec = Some(
            _nettle_cbc_aes256_encrypt_aesni as cbc_aes256_encrypt_func,
        );
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: not using aes instructions.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nettle_aes128_encrypt_vec = Some(_nettle_aes128_encrypt_c as aes128_crypt_func);
        nettle_aes128_decrypt_vec = Some(_nettle_aes128_decrypt_c as aes128_crypt_func);
        nettle_aes192_encrypt_vec = Some(_nettle_aes192_encrypt_c as aes192_crypt_func);
        nettle_aes192_decrypt_vec = Some(_nettle_aes192_decrypt_c as aes192_crypt_func);
        nettle_aes256_encrypt_vec = Some(_nettle_aes256_encrypt_c as aes256_crypt_func);
        nettle_aes256_decrypt_vec = Some(_nettle_aes256_decrypt_c as aes256_crypt_func);
        nettle_cbc_aes128_encrypt_vec = Some(
            _nettle_cbc_aes128_encrypt_c as cbc_aes128_encrypt_func,
        );
        nettle_cbc_aes192_encrypt_vec = Some(
            _nettle_cbc_aes192_encrypt_c as cbc_aes192_encrypt_func,
        );
        nettle_cbc_aes256_encrypt_vec = Some(
            _nettle_cbc_aes256_encrypt_c as cbc_aes256_encrypt_func,
        );
    }
    if features.have_sha_ni != 0 {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: using sha_ni instructions.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nettle_sha1_compress_vec = Some(
            _nettle_sha1_compress_sha_ni as sha1_compress_func,
        );
        _nettle_sha256_compress_n_vec = Some(
            _nettle_sha256_compress_n_sha_ni as sha256_compress_n_func,
        );
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: not using sha_ni instructions.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nettle_sha1_compress_vec = Some(
            _nettle_sha1_compress_x86_64 as sha1_compress_func,
        );
        _nettle_sha256_compress_n_vec = Some(
            _nettle_sha256_compress_n_x86_64 as sha256_compress_n_func,
        );
    }
    if features.have_pclmul != 0 {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: using pclmulqdq instructions.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _nettle_ghash_set_key_vec = Some(
            _nettle_ghash_set_key_pclmul as ghash_set_key_func,
        );
        _nettle_ghash_update_vec = Some(
            _nettle_ghash_update_pclmul as ghash_update_func,
        );
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: not using pclmulqdq instructions.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _nettle_ghash_set_key_vec = Some(_nettle_ghash_set_key_c as ghash_set_key_func);
        _nettle_ghash_update_vec = Some(_nettle_ghash_update_table as ghash_update_func);
    }
    if features.vendor as libc::c_uint == X86_INTEL as libc::c_int as libc::c_uint {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: intel SSE2 will be used for memxor.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nettle_memxor_vec = Some(_nettle_memxor_sse2 as memxor_func);
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: intel SSE2 will not be used for memxor.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nettle_memxor_vec = Some(_nettle_memxor_x86_64 as memxor_func);
    };
}
unsafe extern "C" fn nettle_aes128_encrypt_init(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes128_encrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_aes128_encrypt_vec == Some(nettle_aes128_encrypt_init as aes128_crypt_func)
    {
        fat_init();
    }
    if nettle_aes128_encrypt_vec != Some(nettle_aes128_encrypt_init as aes128_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes128_encrypt_vec != nettle_aes128_encrypt_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void nettle_aes128_encrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2695: {
        if nettle_aes128_encrypt_vec
            != Some(nettle_aes128_encrypt_init as aes128_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes128_encrypt_vec != nettle_aes128_encrypt_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                273 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"void nettle_aes128_encrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes128_encrypt_vec.unwrap()(ctx, length, dst, src);
}
pub unsafe extern "C" fn nettle_aes128_encrypt(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes128_encrypt_vec.unwrap()(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes128_decrypt_init(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes128_decrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_aes128_decrypt_vec == Some(nettle_aes128_decrypt_init as aes128_crypt_func)
    {
        fat_init();
    }
    if nettle_aes128_decrypt_vec != Some(nettle_aes128_decrypt_init as aes128_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes128_decrypt_vec != nettle_aes128_decrypt_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void nettle_aes128_decrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4148: {
        if nettle_aes128_decrypt_vec
            != Some(nettle_aes128_decrypt_init as aes128_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes128_decrypt_vec != nettle_aes128_decrypt_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                277 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"void nettle_aes128_decrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes128_decrypt_vec.unwrap()(ctx, length, dst, src);
}
pub unsafe extern "C" fn nettle_aes128_decrypt(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes128_decrypt_vec.unwrap()(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes192_encrypt_init(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes192_encrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_aes192_encrypt_vec == Some(nettle_aes192_encrypt_init as aes192_crypt_func)
    {
        fat_init();
    }
    if nettle_aes192_encrypt_vec != Some(nettle_aes192_encrypt_init as aes192_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes192_encrypt_vec != nettle_aes192_encrypt_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void nettle_aes192_encrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4055: {
        if nettle_aes192_encrypt_vec
            != Some(nettle_aes192_encrypt_init as aes192_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes192_encrypt_vec != nettle_aes192_encrypt_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"void nettle_aes192_encrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes192_encrypt_vec.unwrap()(ctx, length, dst, src);
}
pub unsafe extern "C" fn nettle_aes192_encrypt(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes192_encrypt_vec.unwrap()(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes192_decrypt_init(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes192_decrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_aes192_decrypt_vec == Some(nettle_aes192_decrypt_init as aes192_crypt_func)
    {
        fat_init();
    }
    if nettle_aes192_decrypt_vec != Some(nettle_aes192_decrypt_init as aes192_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes192_decrypt_vec != nettle_aes192_decrypt_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void nettle_aes192_decrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3962: {
        if nettle_aes192_decrypt_vec
            != Some(nettle_aes192_decrypt_init as aes192_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes192_decrypt_vec != nettle_aes192_decrypt_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"void nettle_aes192_decrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes192_decrypt_vec.unwrap()(ctx, length, dst, src);
}
pub unsafe extern "C" fn nettle_aes192_decrypt(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes192_decrypt_vec.unwrap()(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes256_encrypt_init(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes256_encrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_aes256_encrypt_vec == Some(nettle_aes256_encrypt_init as aes256_crypt_func)
    {
        fat_init();
    }
    if nettle_aes256_encrypt_vec != Some(nettle_aes256_encrypt_init as aes256_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes256_encrypt_vec != nettle_aes256_encrypt_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void nettle_aes256_encrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3865: {
        if nettle_aes256_encrypt_vec
            != Some(nettle_aes256_encrypt_init as aes256_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes256_encrypt_vec != nettle_aes256_encrypt_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                291 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"void nettle_aes256_encrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes256_encrypt_vec.unwrap()(ctx, length, dst, src);
}
pub unsafe extern "C" fn nettle_aes256_encrypt(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes256_encrypt_vec.unwrap()(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes256_decrypt_init(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes256_decrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_aes256_decrypt_vec == Some(nettle_aes256_decrypt_init as aes256_crypt_func)
    {
        fat_init();
    }
    if nettle_aes256_decrypt_vec != Some(nettle_aes256_decrypt_init as aes256_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes256_decrypt_vec != nettle_aes256_decrypt_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void nettle_aes256_decrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3772: {
        if nettle_aes256_decrypt_vec
            != Some(nettle_aes256_decrypt_init as aes256_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes256_decrypt_vec != nettle_aes256_decrypt_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                295 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"void nettle_aes256_decrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes256_decrypt_vec.unwrap()(ctx, length, dst, src);
}
pub unsafe extern "C" fn nettle_aes256_decrypt(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes256_decrypt_vec.unwrap()(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_cbc_aes128_encrypt_init(
    mut ctx: *const aes128_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_cbc_aes128_encrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_cbc_aes128_encrypt_vec
        == Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
    {
        fat_init();
    }
    if nettle_cbc_aes128_encrypt_vec
        != Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
    {} else {
        __assert_fail(
            b"nettle_cbc_aes128_encrypt_vec != nettle_cbc_aes128_encrypt_init\0"
                as *const u8 as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void nettle_cbc_aes128_encrypt_init(const struct aes128_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3675: {
        if nettle_cbc_aes128_encrypt_vec
            != Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
        {} else {
            __assert_fail(
                b"nettle_cbc_aes128_encrypt_vec != nettle_cbc_aes128_encrypt_init\0"
                    as *const u8 as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                300 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"void nettle_cbc_aes128_encrypt_init(const struct aes128_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_cbc_aes128_encrypt_vec.unwrap()(ctx, iv, length, dst, src);
}
pub unsafe extern "C" fn nettle_cbc_aes128_encrypt(
    mut ctx: *const aes128_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_cbc_aes128_encrypt_vec.unwrap()(ctx, iv, length, dst, src);
}
unsafe extern "C" fn nettle_cbc_aes192_encrypt_init(
    mut ctx: *const aes192_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_cbc_aes192_encrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_cbc_aes192_encrypt_vec
        == Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
    {
        fat_init();
    }
    if nettle_cbc_aes192_encrypt_vec
        != Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
    {} else {
        __assert_fail(
            b"nettle_cbc_aes192_encrypt_vec != nettle_cbc_aes192_encrypt_init\0"
                as *const u8 as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void nettle_cbc_aes192_encrypt_init(const struct aes192_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3574: {
        if nettle_cbc_aes192_encrypt_vec
            != Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
        {} else {
            __assert_fail(
                b"nettle_cbc_aes192_encrypt_vec != nettle_cbc_aes192_encrypt_init\0"
                    as *const u8 as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                304 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"void nettle_cbc_aes192_encrypt_init(const struct aes192_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_cbc_aes192_encrypt_vec.unwrap()(ctx, iv, length, dst, src);
}
pub unsafe extern "C" fn nettle_cbc_aes192_encrypt(
    mut ctx: *const aes192_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_cbc_aes192_encrypt_vec.unwrap()(ctx, iv, length, dst, src);
}
unsafe extern "C" fn nettle_cbc_aes256_encrypt_init(
    mut ctx: *const aes256_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_cbc_aes256_encrypt_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_cbc_aes256_encrypt_vec
        == Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
    {
        fat_init();
    }
    if nettle_cbc_aes256_encrypt_vec
        != Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
    {} else {
        __assert_fail(
            b"nettle_cbc_aes256_encrypt_vec != nettle_cbc_aes256_encrypt_init\0"
                as *const u8 as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void nettle_cbc_aes256_encrypt_init(const struct aes256_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3466: {
        if nettle_cbc_aes256_encrypt_vec
            != Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
        {} else {
            __assert_fail(
                b"nettle_cbc_aes256_encrypt_vec != nettle_cbc_aes256_encrypt_init\0"
                    as *const u8 as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"void nettle_cbc_aes256_encrypt_init(const struct aes256_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_cbc_aes256_encrypt_vec.unwrap()(ctx, iv, length, dst, src);
}
pub unsafe extern "C" fn nettle_cbc_aes256_encrypt(
    mut ctx: *const aes256_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_cbc_aes256_encrypt_vec.unwrap()(ctx, iv, length, dst, src);
}
unsafe extern "C" fn nettle_memxor_init(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_memxor_init\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if nettle_memxor_vec == Some(nettle_memxor_init as memxor_func) {
        fat_init();
    }
    if nettle_memxor_vec != Some(nettle_memxor_init as memxor_func) {} else {
        __assert_fail(
            b"nettle_memxor_vec != nettle_memxor_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void *nettle_memxor_init(void *, const void *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_2795: {
        if nettle_memxor_vec != Some(nettle_memxor_init as memxor_func) {} else {
            __assert_fail(
                b"nettle_memxor_vec != nettle_memxor_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                312 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void *nettle_memxor_init(void *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return nettle_memxor_vec.unwrap()(dst, src, n);
}
pub unsafe extern "C" fn nettle_memxor(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    return nettle_memxor_vec.unwrap()(dst, src, n);
}
unsafe extern "C" fn nettle_sha1_compress_init(
    mut state: *mut uint32_t,
    mut input: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_sha1_compress_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nettle_sha1_compress_vec == Some(nettle_sha1_compress_init as sha1_compress_func)
    {
        fat_init();
    }
    if nettle_sha1_compress_vec != Some(nettle_sha1_compress_init as sha1_compress_func)
    {} else {
        __assert_fail(
            b"nettle_sha1_compress_vec != nettle_sha1_compress_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void nettle_sha1_compress_init(uint32_t *, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3304: {
        if nettle_sha1_compress_vec
            != Some(nettle_sha1_compress_init as sha1_compress_func)
        {} else {
            __assert_fail(
                b"nettle_sha1_compress_vec != nettle_sha1_compress_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                316 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void nettle_sha1_compress_init(uint32_t *, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return nettle_sha1_compress_vec.unwrap()(state, input);
}
pub unsafe extern "C" fn nettle_sha1_compress(
    mut state: *mut uint32_t,
    mut input: *const uint8_t,
) {
    return nettle_sha1_compress_vec.unwrap()(state, input);
}
unsafe extern "C" fn _nettle_sha256_compress_n_init(
    mut state: *mut uint32_t,
    mut k: *const uint32_t,
    mut blocks: size_t,
    mut input: *const uint8_t,
) -> *const uint8_t {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: _nettle_sha256_compress_n_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _nettle_sha256_compress_n_vec
        == Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
    {
        fat_init();
    }
    if _nettle_sha256_compress_n_vec
        != Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
    {} else {
        __assert_fail(
            b"_nettle_sha256_compress_n_vec != _nettle_sha256_compress_n_init\0"
                as *const u8 as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"const uint8_t *_nettle_sha256_compress_n_init(uint32_t *, const uint32_t *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3212: {
        if _nettle_sha256_compress_n_vec
            != Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
        {} else {
            __assert_fail(
                b"_nettle_sha256_compress_n_vec != _nettle_sha256_compress_n_init\0"
                    as *const u8 as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                321 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"const uint8_t *_nettle_sha256_compress_n_init(uint32_t *, const uint32_t *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return _nettle_sha256_compress_n_vec.unwrap()(state, k, blocks, input);
}
pub unsafe extern "C" fn _nettle_sha256_compress_n(
    mut state: *mut uint32_t,
    mut k: *const uint32_t,
    mut blocks: size_t,
    mut input: *const uint8_t,
) -> *const uint8_t {
    return _nettle_sha256_compress_n_vec.unwrap()(state, k, blocks, input);
}
unsafe extern "C" fn _nettle_ghash_set_key_init(
    mut ctx: *mut gcm_key,
    mut key: *const nettle_block16,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: _nettle_ghash_set_key_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _nettle_ghash_set_key_vec
        == Some(_nettle_ghash_set_key_init as ghash_set_key_func)
    {
        fat_init();
    }
    if _nettle_ghash_set_key_vec
        != Some(_nettle_ghash_set_key_init as ghash_set_key_func)
    {} else {
        __assert_fail(
            b"_nettle_ghash_set_key_vec != _nettle_ghash_set_key_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            325 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void _nettle_ghash_set_key_init(struct gcm_key *, const union nettle_block16 *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3062: {
        if _nettle_ghash_set_key_vec
            != Some(_nettle_ghash_set_key_init as ghash_set_key_func)
        {} else {
            __assert_fail(
                b"_nettle_ghash_set_key_vec != _nettle_ghash_set_key_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                325 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"void _nettle_ghash_set_key_init(struct gcm_key *, const union nettle_block16 *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return _nettle_ghash_set_key_vec.unwrap()(ctx, key);
}
pub unsafe extern "C" fn _nettle_ghash_set_key(
    mut ctx: *mut gcm_key,
    mut key: *const nettle_block16,
) {
    return _nettle_ghash_set_key_vec.unwrap()(ctx, key);
}
unsafe extern "C" fn _nettle_ghash_update_init(
    mut ctx: *const gcm_key,
    mut state: *mut nettle_block16,
    mut blocks: size_t,
    mut data: *const uint8_t,
) -> *const uint8_t {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libnettle: _nettle_ghash_update_init\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _nettle_ghash_update_vec == Some(_nettle_ghash_update_init as ghash_update_func) {
        fat_init();
    }
    if _nettle_ghash_update_vec != Some(_nettle_ghash_update_init as ghash_update_func)
    {} else {
        __assert_fail(
            b"_nettle_ghash_update_vec != _nettle_ghash_update_init\0" as *const u8
                as *const libc::c_char,
            b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"const uint8_t *_nettle_ghash_update_init(const struct gcm_key *, union nettle_block16 *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2968: {
        if _nettle_ghash_update_vec
            != Some(_nettle_ghash_update_init as ghash_update_func)
        {} else {
            __assert_fail(
                b"_nettle_ghash_update_vec != _nettle_ghash_update_init\0" as *const u8
                    as *const libc::c_char,
                b"fat-x86_64.c\0" as *const u8 as *const libc::c_char,
                329 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 114],
                    &[libc::c_char; 114],
                >(
                    b"const uint8_t *_nettle_ghash_update_init(const struct gcm_key *, union nettle_block16 *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return _nettle_ghash_update_vec.unwrap()(ctx, state, blocks, data);
}
pub unsafe extern "C" fn _nettle_ghash_update(
    mut ctx: *const gcm_key,
    mut state: *mut nettle_block16,
    mut blocks: size_t,
    mut data: *const uint8_t,
) -> *const uint8_t {
    return _nettle_ghash_update_vec.unwrap()(ctx, state, blocks, data);
}
