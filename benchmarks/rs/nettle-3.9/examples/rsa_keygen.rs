use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn nettle_buffer_init(buffer: *mut nettle_buffer);
    fn nettle_buffer_clear(buffer: *mut nettle_buffer);
    fn __gmpz_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn nettle_rsa_public_key_init(key: *mut rsa_public_key);
    fn nettle_rsa_public_key_clear(key: *mut rsa_public_key);
    fn nettle_rsa_private_key_init(key: *mut rsa_private_key);
    fn nettle_rsa_private_key_clear(key: *mut rsa_private_key);
    fn nettle_rsa_generate_keypair(
        pub_0: *mut rsa_public_key,
        key: *mut rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        progress_ctx: *mut libc::c_void,
        progress_0: Option::<nettle_progress_func>,
        n_size: libc::c_uint,
        e_size: libc::c_uint,
    ) -> libc::c_int;
    fn nettle_rsa_keypair_to_sexp(
        buffer: *mut nettle_buffer,
        algorithm_name: *const libc::c_char,
        pub_0: *const rsa_public_key,
        priv_0: *const rsa_private_key,
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
    fn xalloc(size: size_t) -> *mut libc::c_void;
    fn werror(format: *const libc::c_char, _: ...);
    fn write_file(
        name: *const libc::c_char,
        size: size_t,
        data: *const libc::c_void,
    ) -> libc::c_int;
    fn simple_random(ctx: *mut yarrow256_ctx, name: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
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
pub type nettle_progress_func = unsafe extern "C" fn(
    *mut libc::c_void,
    libc::c_int,
) -> ();
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option::<nettle_realloc_func>,
    pub size: size_t,
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const OPT_HELP: C2RustUnnamed = 300;
pub type C2RustUnnamed = libc::c_uint;
unsafe extern "C" fn progress(mut ctx: *mut libc::c_void, mut c: libc::c_int) {
    fputc(c, stderr);
}
unsafe extern "C" fn uint_arg(
    mut c: libc::c_char,
    mut arg: *const libc::c_char,
) -> libc::c_ulong {
    let mut val: libc::c_ulong = 0;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    val = strtoul(arg, &mut end, 0 as libc::c_int);
    if *arg as libc::c_int == '\0' as i32 || *end as libc::c_int != '\0' as i32 {
        werror(
            b"Invalid integer argument for -%c option.\n\0" as *const u8
                as *const libc::c_char,
            c as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    return val;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut yarrow: yarrow256_ctx = yarrow256_ctx {
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
    };
    let mut pub_0: rsa_public_key = rsa_public_key {
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
    let mut priv_0: rsa_private_key = rsa_private_key {
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
    let mut c: libc::c_int = 0;
    let mut pub_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut priv_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut random_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut pub_buffer: nettle_buffer = nettle_buffer {
        contents: 0 as *mut uint8_t,
        alloc: 0,
        realloc_ctx: 0 as *mut libc::c_void,
        realloc: None,
        size: 0,
    };
    let mut priv_buffer: nettle_buffer = nettle_buffer {
        contents: 0 as *mut uint8_t,
        alloc: 0,
        realloc_ctx: 0 as *mut libc::c_void,
        realloc: None,
        size: 0,
    };
    let mut key_size: libc::c_ulong = 2048 as libc::c_int as libc::c_ulong;
    let mut key_e: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
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
            b"o:r:e:s:\0" as *const u8 as *const libc::c_char,
            options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            111 => {
                priv_name = optarg;
            }
            114 => {
                random_name = optarg;
            }
            115 => {
                key_size = uint_arg('s' as i32 as libc::c_char, optarg);
            }
            101 => {
                key_e = uint_arg('e' as i32 as libc::c_char, optarg);
            }
            300 => {
                printf(
                    b"FIXME: Usage information.\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            63 => return 1 as libc::c_int,
            _ => {
                abort();
            }
        }
    }
    if priv_name.is_null() {
        werror(b"No filename provided.\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    pub_name = xalloc(
        (strlen(priv_name)).wrapping_add(5 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(pub_name, b"%s.pub\0" as *const u8 as *const libc::c_char, priv_name);
    nettle_yarrow256_init(
        &mut yarrow,
        0 as libc::c_int as libc::c_uint,
        0 as *mut yarrow_source,
    );
    if simple_random(&mut yarrow, random_name) == 0 {
        werror(
            b"Initialization of randomness generator failed.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    nettle_rsa_public_key_init(&mut pub_0);
    nettle_rsa_private_key_init(&mut priv_0);
    if key_e != 0 {
        __gmpz_set_ui((pub_0.e).as_mut_ptr(), key_e);
    }
    if nettle_rsa_generate_keypair(
        &mut pub_0,
        &mut priv_0,
        &mut yarrow as *mut yarrow256_ctx as *mut libc::c_void,
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
        0 as *mut libc::c_void,
        Some(progress as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
        key_size as libc::c_uint,
        (if key_e == 0 as libc::c_int as libc::c_ulong {
            30 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint,
    ) == 0
    {
        werror(b"Key generation failed.\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    nettle_buffer_init(&mut priv_buffer);
    nettle_buffer_init(&mut pub_buffer);
    if nettle_rsa_keypair_to_sexp(
        &mut pub_buffer,
        b"rsa-pkcs1-sha1\0" as *const u8 as *const libc::c_char,
        &mut pub_0,
        0 as *const rsa_private_key,
    ) == 0
    {
        werror(b"Formatting public key failed.\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if nettle_rsa_keypair_to_sexp(
        &mut priv_buffer,
        b"rsa-pkcs1-sha1\0" as *const u8 as *const libc::c_char,
        &mut pub_0,
        &mut priv_0,
    ) == 0
    {
        werror(
            b"Formatting private key failed.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if write_file(pub_name, pub_buffer.size, pub_buffer.contents as *const libc::c_void)
        == 0
    {
        werror(
            b"Failed to write public key: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 1 as libc::c_int;
    }
    if write_file(
        priv_name,
        priv_buffer.size,
        priv_buffer.contents as *const libc::c_void,
    ) == 0
    {
        werror(
            b"Failed to write private key: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 1 as libc::c_int;
    }
    nettle_buffer_clear(&mut priv_buffer);
    nettle_buffer_clear(&mut pub_buffer);
    nettle_rsa_public_key_clear(&mut pub_0);
    nettle_rsa_private_key_clear(&mut priv_0);
    free(pub_name as *mut libc::c_void);
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
