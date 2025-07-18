use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn nettle_aes256_set_decrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_decrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
    fn nettle_cbc_decrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
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
    fn nettle_memeql_sec(
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> libc::c_int;
    fn nettle_rsa_private_key_init(key: *mut rsa_private_key);
    fn nettle_rsa_private_key_clear(key: *mut rsa_private_key);
    fn nettle_rsa_decrypt(
        key: *const rsa_private_key,
        length: *mut size_t,
        cleartext: *mut uint8_t,
        ciphertext: *const __mpz_struct,
    ) -> libc::c_int;
    fn xalloc(size: size_t) -> *mut libc::c_void;
    fn werror(format: *const libc::c_char, _: ...);
    fn write_data(f: *mut FILE, size: size_t, data: *const libc::c_void) -> libc::c_int;
    fn read_rsa_key(
        name: *const libc::c_char,
        pub_0: *mut rsa_public_key,
        priv_0: *mut rsa_private_key,
    ) -> libc::c_int;
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
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
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
pub struct hmac_sha1_ctx {
    pub outer: sha1_ctx,
    pub inner: sha1_ctx,
    pub state: sha1_ctx,
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
pub type yarrow_pool_id = libc::c_uint;
pub const YARROW_SLOW: yarrow_pool_id = 1;
pub const YARROW_FAST: yarrow_pool_id = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow_source {
    pub estimate: [uint32_t; 2],
    pub next: yarrow_pool_id,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow256_ctx {
    pub pools: [sha256_ctx; 2],
    pub seeded: libc::c_int,
    pub key: aes256_ctx,
    pub counter: [uint8_t; 16],
    pub nsources: libc::c_uint,
    pub sources: *mut yarrow_source,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_session {
    pub aes: C2RustUnnamed,
    pub hmac: hmac_sha1_ctx,
    pub yarrow: yarrow256_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ctx: aes256_ctx,
    pub iv: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_session_info {
    pub key: [uint8_t; 72],
}
pub unsafe extern "C" fn rsa_session_set_decrypt_key(
    mut ctx: *mut rsa_session,
    mut key: *const rsa_session_info,
) {
    let mut aes_key: *const uint8_t = ((*key).key)
        .as_ptr()
        .offset(4 as libc::c_int as isize);
    let mut iv: *const uint8_t = ((*key).key)
        .as_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(32 as libc::c_int as isize);
    let mut hmac_key: *const uint8_t = ((*key).key)
        .as_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(32 as libc::c_int as isize)
        .offset(16 as libc::c_int as isize);
    nettle_aes256_set_decrypt_key(&mut (*ctx).aes.ctx, aes_key);
    memcpy(
        ((*ctx).aes.iv).as_mut_ptr() as *mut libc::c_void,
        iv as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    nettle_hmac_sha1_set_key(&mut (*ctx).hmac, 20 as libc::c_int as size_t, hmac_key);
}
unsafe extern "C" fn read_uint32(mut f: *mut FILE, mut n: *mut uint32_t) -> libc::c_int {
    let mut buf: [uint8_t; 4] = [0; 4];
    if fread(
        buf.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
        f,
    ) != ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    *n = (buf[0 as libc::c_int as usize] as uint32_t) << 24 as libc::c_int
        | (buf[1 as libc::c_int as usize] as uint32_t) << 16 as libc::c_int
        | (buf[2 as libc::c_int as usize] as uint32_t) << 8 as libc::c_int
        | buf[3 as libc::c_int as usize] as uint32_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_version(mut f: *mut FILE) -> libc::c_int {
    let mut version: uint32_t = 0;
    return (read_uint32(f, &mut version) != 0
        && version == 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn read_bignum(
    mut f: *mut FILE,
    mut x: *mut __mpz_struct,
) -> libc::c_int {
    let mut size: uint32_t = 0;
    if read_uint32(f, &mut size) != 0 && size < 1000 as libc::c_int as libc::c_uint {
        let mut p: *mut uint8_t = xalloc(size as size_t) as *mut uint8_t;
        if fread(
            p as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            size as libc::c_ulong,
            f,
        ) != size as libc::c_ulong
        {
            free(p as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        nettle_mpz_set_str_256_u(x, size as size_t, p);
        free(p as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn process_file(
    mut ctx: *mut rsa_session,
    mut in_0: *mut FILE,
    mut out: *mut FILE,
) -> libc::c_int {
    let mut buffer: [uint8_t; 1636] = [0; 1636];
    let mut digest: [uint8_t; 20] = [0; 20];
    let mut size: size_t = 0;
    let mut padding: libc::c_uint = 0;
    size = fread(
        buffer.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (16 as libc::c_int + 20 as libc::c_int) as libc::c_ulong,
        in_0,
    );
    if size < (16 as libc::c_int + 20 as libc::c_int) as libc::c_ulong {
        if ferror(in_0) != 0 {
            werror(
                b"Reading input failed: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        } else {
            werror(b"Unexpected EOF on input.\n\0" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int;
    }
    loop {
        size = fread(
            buffer.as_mut_ptr().offset((16 as libc::c_int + 20 as libc::c_int) as isize)
                as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (100 as libc::c_int * 16 as libc::c_int) as libc::c_ulong,
            in_0,
        );
        if size < (100 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
            && ferror(in_0) != 0
        {
            werror(
                b"Reading input failed: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
        if size.wrapping_rem(16 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            werror(b"Unexpected EOF on input.\n\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        if size != 0 {
            if 0 as libc::c_int != 0 {
                nettle_aes256_decrypt(
                    &mut (*ctx).aes.ctx,
                    !(0 as libc::c_int as size_t),
                    0 as *mut uint8_t,
                    0 as *const uint8_t,
                );
            } else {
                nettle_cbc_decrypt(
                    &mut (*ctx).aes.ctx as *mut aes256_ctx as *mut libc::c_void,
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const aes256_ctx,
                                size_t,
                                *mut uint8_t,
                                *const uint8_t,
                            ) -> (),
                        >,
                        Option::<nettle_cipher_func>,
                    >(
                        Some(
                            nettle_aes256_decrypt
                                as unsafe extern "C" fn(
                                    *const aes256_ctx,
                                    size_t,
                                    *mut uint8_t,
                                    *const uint8_t,
                                ) -> (),
                        ),
                    ),
                    ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
                    ((*ctx).aes.iv).as_mut_ptr(),
                    size,
                    buffer.as_mut_ptr(),
                    buffer.as_mut_ptr(),
                );
            };
            nettle_hmac_sha1_update(&mut (*ctx).hmac, size, buffer.as_mut_ptr());
            if write_data(out, size, buffer.as_mut_ptr() as *const libc::c_void) == 0 {
                werror(
                    b"Writing output failed: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            memmove(
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.as_mut_ptr().offset(size as isize) as *const libc::c_void,
                (16 as libc::c_int + 20 as libc::c_int) as libc::c_ulong,
            );
        }
        if !(size == (100 as libc::c_int * 16 as libc::c_int) as libc::c_ulong) {
            break;
        }
    }
    if 0 as libc::c_int != 0 {
        nettle_aes256_decrypt(
            &mut (*ctx).aes.ctx,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_cbc_decrypt(
            &mut (*ctx).aes.ctx as *mut aes256_ctx as *mut libc::c_void,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes256_decrypt
                        as unsafe extern "C" fn(
                            *const aes256_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
            ((*ctx).aes.iv).as_mut_ptr(),
            16 as libc::c_int as size_t,
            buffer.as_mut_ptr(),
            buffer.as_mut_ptr(),
        );
    };
    padding = buffer[(16 as libc::c_int - 1 as libc::c_int) as usize] as libc::c_uint;
    if padding > 16 as libc::c_int as libc::c_uint {
        werror(
            b"Decryption failed: Invalid padding.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if padding < 16 as libc::c_int as libc::c_uint {
        let mut leftover: libc::c_uint = (16 as libc::c_int as libc::c_uint)
            .wrapping_sub(padding);
        nettle_hmac_sha1_update(
            &mut (*ctx).hmac,
            leftover as size_t,
            buffer.as_mut_ptr(),
        );
        if write_data(
            out,
            leftover as size_t,
            buffer.as_mut_ptr() as *const libc::c_void,
        ) == 0
        {
            werror(
                b"Writing output failed: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    }
    nettle_hmac_sha1_digest(
        &mut (*ctx).hmac,
        20 as libc::c_int as size_t,
        digest.as_mut_ptr(),
    );
    if nettle_memeql_sec(
        digest.as_mut_ptr() as *const libc::c_void,
        buffer.as_mut_ptr().offset(16 as libc::c_int as isize) as *const libc::c_void,
        20 as libc::c_int as size_t,
    ) == 0
    {
        werror(
            b"Decryption failed: Invalid mac.\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut key: rsa_private_key = rsa_private_key {
        size: 0,
        d: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        p: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        q: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        a: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        b: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        c: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut ctx: rsa_session = rsa_session {
        aes: C2RustUnnamed {
            ctx: aes256_ctx { keys: [0; 60] },
            iv: [0; 16],
        },
        hmac: hmac_sha1_ctx {
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
        },
        yarrow: yarrow256_ctx {
            pools: [sha256_ctx {
                state: [0; 8],
                count: 0,
                index: 0,
                block: [0; 64],
            }; 2],
            seeded: 0,
            key: aes256_ctx { keys: [0; 60] },
            counter: [0; 16],
            nsources: 0,
            sources: 0 as *mut yarrow_source,
        },
    };
    let mut session: rsa_session_info = rsa_session_info { key: [0; 72] };
    let mut length: size_t = 0;
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(x.as_mut_ptr());
    if argc != 2 as libc::c_int {
        werror(
            b"Usage: rsa-decrypt PRIVATE-KEY < ciphertext\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    nettle_rsa_private_key_init(&mut key);
    if read_rsa_key(
        *argv.offset(1 as libc::c_int as isize),
        0 as *mut rsa_public_key,
        &mut key,
    ) == 0
    {
        werror(b"Invalid key\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if read_version(stdin) == 0 {
        werror(
            b"Bad version number in input file.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if read_bignum(stdin, x.as_mut_ptr()) == 0 {
        werror(b"Bad rsa header in input file.\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    length = ::std::mem::size_of::<[uint8_t; 72]>() as libc::c_ulong;
    if nettle_rsa_decrypt(
        &mut key,
        &mut length,
        (session.key).as_mut_ptr(),
        x.as_mut_ptr() as *const __mpz_struct,
    ) == 0 || length != ::std::mem::size_of::<[uint8_t; 72]>() as libc::c_ulong
    {
        werror(
            b"Failed to decrypt rsa header in input file.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    __gmpz_clear(x.as_mut_ptr());
    rsa_session_set_decrypt_key(&mut ctx, &mut session);
    if process_file(&mut ctx, stdin, stdout) == 0 {
        return 1 as libc::c_int;
    }
    nettle_rsa_private_key_clear(&mut key);
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
