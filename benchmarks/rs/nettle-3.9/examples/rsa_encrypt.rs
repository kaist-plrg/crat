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
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn nettle_mpz_sizeinbase_256_u(x: *const __mpz_struct) -> size_t;
    fn nettle_mpz_get_str_256(length: size_t, s: *mut uint8_t, x: *const __mpz_struct);
    fn nettle_rsa_public_key_init(key: *mut rsa_public_key);
    fn nettle_rsa_public_key_clear(key: *mut rsa_public_key);
    fn nettle_rsa_encrypt(
        key: *const rsa_public_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        length: size_t,
        cleartext: *const uint8_t,
        cipher: *mut __mpz_struct,
    ) -> libc::c_int;
    fn nettle_yarrow256_init(
        ctx: *mut yarrow256_ctx,
        nsources: libc::c_uint,
        sources: *mut yarrow_source,
    );
    fn nettle_yarrow256_random(
        ctx: *mut yarrow256_ctx,
        length: size_t,
        dst: *mut uint8_t,
    );
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn xalloc(size: size_t) -> *mut libc::c_void;
    fn werror(format: *const libc::c_char, _: ...);
    fn write_data(f: *mut FILE, size: size_t, data: *const libc::c_void) -> libc::c_int;
    fn simple_random(ctx: *mut yarrow256_ctx, name: *const libc::c_char) -> libc::c_int;
    fn read_rsa_key(
        name: *const libc::c_char,
        pub_0: *mut rsa_public_key,
        priv_0: *mut rsa_private_key,
    ) -> libc::c_int;
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
    fn nettle_cbc_encrypt(
        ctx: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
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
pub type nettle_random_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
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
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
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
pub struct hmac_sha1_ctx {
    pub outer: sha1_ctx,
    pub inner: sha1_ctx,
    pub state: sha1_ctx,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const OPT_HELP: C2RustUnnamed_0 = 300;
pub type C2RustUnnamed_0 = libc::c_uint;
pub unsafe extern "C" fn rsa_session_set_encrypt_key(
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
    nettle_aes256_set_encrypt_key(&mut (*ctx).aes.ctx, aes_key);
    memcpy(
        ((*ctx).aes.iv).as_mut_ptr() as *mut libc::c_void,
        iv as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    nettle_hmac_sha1_set_key(&mut (*ctx).hmac, 20 as libc::c_int as size_t, hmac_key);
}
unsafe extern "C" fn write_uint32(mut f: *mut FILE, mut n: uint32_t) -> libc::c_int {
    let mut buffer: [uint8_t; 4] = [0; 4];
    buffer[0 as libc::c_int
        as usize] = (n >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    buffer[1 as libc::c_int
        as usize] = (n >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    buffer[2 as libc::c_int
        as usize] = (n >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    buffer[3 as libc::c_int
        as usize] = (n & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    return write_data(
        f,
        ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
        buffer.as_mut_ptr() as *const libc::c_void,
    );
}
unsafe extern "C" fn write_version(mut f: *mut FILE) -> libc::c_int {
    return write_uint32(f, 1 as libc::c_int as uint32_t);
}
unsafe extern "C" fn write_bignum(
    mut f: *mut FILE,
    mut x: *mut __mpz_struct,
) -> libc::c_int {
    let mut size: libc::c_uint = nettle_mpz_sizeinbase_256_u(x as *const __mpz_struct)
        as libc::c_uint;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut res: libc::c_int = 0;
    if write_uint32(f, size) == 0 {
        return 0 as libc::c_int;
    }
    p = xalloc(size as size_t) as *mut uint8_t;
    nettle_mpz_get_str_256(size as size_t, p, x as *const __mpz_struct);
    res = write_data(f, size as size_t, p as *const libc::c_void);
    free(p as *mut libc::c_void);
    return res;
}
unsafe extern "C" fn process_file(
    mut ctx: *mut rsa_session,
    mut in_0: *mut FILE,
    mut out: *mut FILE,
) -> libc::c_int {
    let mut buffer: [uint8_t; 1620] = [0; 1620];
    loop {
        let mut size: size_t = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (16 as libc::c_int * 100 as libc::c_int) as libc::c_ulong,
            in_0,
        );
        nettle_hmac_sha1_update(&mut (*ctx).hmac, size, buffer.as_mut_ptr());
        if size < (16 as libc::c_int * 100 as libc::c_int) as libc::c_ulong {
            let mut leftover: libc::c_uint = 0;
            let mut padding: libc::c_uint = 0;
            if ferror(in_0) != 0 {
                werror(
                    b"Reading input failed: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            leftover = size.wrapping_rem(16 as libc::c_int as libc::c_ulong)
                as libc::c_uint;
            padding = (16 as libc::c_int as libc::c_uint).wrapping_sub(leftover);
            if size.wrapping_add(padding as libc::c_ulong)
                <= (16 as libc::c_int * 100 as libc::c_int) as libc::c_ulong
            {} else {
                __assert_fail(
                    b"size + padding <= BLOCK_SIZE\0" as *const u8
                        as *const libc::c_char,
                    b"rsa-encrypt.c\0" as *const u8 as *const libc::c_char,
                    131 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int process_file(struct rsa_session *, FILE *, FILE *)\0"))
                        .as_ptr(),
                );
            }
            'c_7626: {
                if size.wrapping_add(padding as libc::c_ulong)
                    <= (16 as libc::c_int * 100 as libc::c_int) as libc::c_ulong
                {} else {
                    __assert_fail(
                        b"size + padding <= BLOCK_SIZE\0" as *const u8
                            as *const libc::c_char,
                        b"rsa-encrypt.c\0" as *const u8 as *const libc::c_char,
                        131 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"int process_file(struct rsa_session *, FILE *, FILE *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if padding > 1 as libc::c_int as libc::c_uint {
                nettle_yarrow256_random(
                    &mut (*ctx).yarrow,
                    padding.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    buffer.as_mut_ptr().offset(size as isize),
                );
            }
            size = (size as libc::c_ulong).wrapping_add(padding as libc::c_ulong)
                as size_t as size_t;
            buffer[size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = padding as uint8_t;
            if 0 as libc::c_int != 0 {
                nettle_aes256_encrypt(
                    &mut (*ctx).aes.ctx,
                    !(0 as libc::c_int as size_t),
                    0 as *mut uint8_t,
                    0 as *const uint8_t,
                );
            } else {
                nettle_cbc_encrypt(
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
                            nettle_aes256_encrypt
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
            if size.wrapping_add(20 as libc::c_int as libc::c_ulong)
                <= ::std::mem::size_of::<[uint8_t; 1620]>() as libc::c_ulong
            {} else {
                __assert_fail(
                    b"size + SHA1_DIGEST_SIZE <= sizeof(buffer)\0" as *const u8
                        as *const libc::c_char,
                    b"rsa-encrypt.c\0" as *const u8 as *const libc::c_char,
                    141 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int process_file(struct rsa_session *, FILE *, FILE *)\0"))
                        .as_ptr(),
                );
            }
            'c_7467: {
                if size.wrapping_add(20 as libc::c_int as libc::c_ulong)
                    <= ::std::mem::size_of::<[uint8_t; 1620]>() as libc::c_ulong
                {} else {
                    __assert_fail(
                        b"size + SHA1_DIGEST_SIZE <= sizeof(buffer)\0" as *const u8
                            as *const libc::c_char,
                        b"rsa-encrypt.c\0" as *const u8 as *const libc::c_char,
                        141 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"int process_file(struct rsa_session *, FILE *, FILE *)\0"))
                            .as_ptr(),
                    );
                }
            };
            nettle_hmac_sha1_digest(
                &mut (*ctx).hmac,
                20 as libc::c_int as size_t,
                buffer.as_mut_ptr().offset(size as isize),
            );
            size = (size as libc::c_ulong)
                .wrapping_add(20 as libc::c_int as libc::c_ulong) as size_t as size_t;
            if write_data(out, size, buffer.as_mut_ptr() as *const libc::c_void) == 0 {
                werror(
                    b"Writing output failed: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        if 0 as libc::c_int != 0 {
            nettle_aes256_encrypt(
                &mut (*ctx).aes.ctx,
                !(0 as libc::c_int as size_t),
                0 as *mut uint8_t,
                0 as *const uint8_t,
            );
        } else {
            nettle_cbc_encrypt(
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
                        nettle_aes256_encrypt
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
        if write_data(out, size, buffer.as_mut_ptr() as *const libc::c_void) == 0 {
            werror(
                b"Writing output failed: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn usage(mut out: *mut FILE) {
    fprintf(
        out,
        b"Usage: rsa-encrypt [OPTIONS] PUBLIC-KEY < cleartext\nOptions:\n   -r, --random=FILE   seed file for randomness generator\n       --help          display this help\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
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
    let mut info: rsa_session_info = rsa_session_info { key: [0; 72] };
    let mut key: rsa_public_key = rsa_public_key {
        size: 0,
        n: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        e: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut c: libc::c_int = 0;
    let mut random_name: *const libc::c_char = 0 as *const libc::c_char;
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
                name: b"random\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
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
            b"o:r:\0" as *const u8 as *const libc::c_char,
            options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            114 => {
                random_name = optarg;
            }
            63 => return 1 as libc::c_int,
            300 => {
                usage(stdout);
                return 0 as libc::c_int;
            }
            _ => {
                abort();
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc != 1 as libc::c_int {
        usage(stderr);
        return 1 as libc::c_int;
    }
    nettle_rsa_public_key_init(&mut key);
    if read_rsa_key(
        *argv.offset(0 as libc::c_int as isize),
        &mut key,
        0 as *mut rsa_private_key,
    ) == 0
    {
        werror(b"Invalid key\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    nettle_yarrow256_init(
        &mut ctx.yarrow,
        0 as libc::c_int as libc::c_uint,
        0 as *mut yarrow_source,
    );
    if simple_random(&mut ctx.yarrow, random_name) == 0 {
        werror(
            b"Initialization of randomness generator failed.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    info
        .key[0 as libc::c_int
        as usize] = (1 as libc::c_int >> 24 as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    info
        .key[1 as libc::c_int
        as usize] = (1 as libc::c_int >> 16 as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    info
        .key[2 as libc::c_int
        as usize] = (1 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    info
        .key[3 as libc::c_int
        as usize] = (1 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    nettle_yarrow256_random(
        &mut ctx.yarrow,
        (::std::mem::size_of::<[uint8_t; 72]>() as libc::c_ulong)
            .wrapping_sub(4 as libc::c_int as libc::c_ulong),
        (info.key).as_mut_ptr().offset(4 as libc::c_int as isize),
    );
    rsa_session_set_encrypt_key(&mut ctx, &mut info);
    write_version(stdout);
    __gmpz_init(x.as_mut_ptr());
    if nettle_rsa_encrypt(
        &mut key,
        &mut ctx.yarrow as *mut yarrow256_ctx as *mut libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut yarrow256_ctx, size_t, *mut uint8_t) -> (),
            >,
            Option::<nettle_random_func>,
        >(
            Some(
                nettle_yarrow256_random
                    as unsafe extern "C" fn(
                        *mut yarrow256_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
            ),
        ),
        ::std::mem::size_of::<[uint8_t; 72]>() as libc::c_ulong,
        (info.key).as_mut_ptr(),
        x.as_mut_ptr(),
    ) == 0
    {
        werror(b"RSA encryption failed.\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    write_bignum(stdout, x.as_mut_ptr());
    __gmpz_clear(x.as_mut_ptr());
    if process_file(&mut ctx, stdin, stdout) == 0 {
        return 1 as libc::c_int;
    }
    nettle_rsa_public_key_clear(&mut key);
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
