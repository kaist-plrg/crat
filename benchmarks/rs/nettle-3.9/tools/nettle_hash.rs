use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn nettle_get_hashes() -> *const *const nettle_hash;
    fn nettle_lookup_hash(name: *const libc::c_char) -> *const nettle_hash;
    fn nettle_base16_encode_update(
        dst: *mut libc::c_char,
        length: size_t,
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
    fn die(format: *const libc::c_char, _: ...) -> !;
    fn xalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const OPT_LIST: C2RustUnnamed = 770;
pub const OPT_RAW: C2RustUnnamed = 769;
pub const OPT_HELP: C2RustUnnamed = 768;
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn list_algorithms() {
    let mut i: libc::c_uint = 0;
    let mut alg: *const nettle_hash = 0 as *const nettle_hash;
    printf(
        b"%10s digestsize (internal block size, context size), in units of octets\n\0"
            as *const u8 as *const libc::c_char,
        b"name\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    loop {
        alg = *(nettle_get_hashes()).offset(i as isize);
        if alg.is_null() {
            break;
        }
        printf(
            b"%10s %d (%d, %d)\n\0" as *const u8 as *const libc::c_char,
            (*alg).name,
            (*alg).digest_size,
            (*alg).block_size,
            (*alg).context_size,
        );
        i = i.wrapping_add(1);
        i;
    };
}
unsafe extern "C" fn hash_file(
    mut hash: *const nettle_hash,
    mut ctx: *mut libc::c_void,
    mut f: *mut FILE,
) -> libc::c_int {
    loop {
        let mut buffer: [uint8_t; 16384] = [0; 16384];
        let mut res: size_t = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[uint8_t; 16384]>() as libc::c_ulong,
            f,
        );
        if ferror(f) != 0 {
            return 0 as libc::c_int;
        }
        ((*hash).update).unwrap()(ctx, res, buffer.as_mut_ptr());
        if feof(f) != 0 {
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn digest_file(
    mut alg: *const nettle_hash,
    mut digest_length: libc::c_uint,
    mut raw: libc::c_int,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut ctx: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut digest: *mut uint8_t = 0 as *mut uint8_t;
    ctx = xalloc((*alg).context_size as size_t);
    ((*alg).init).unwrap()(ctx);
    if hash_file(alg, ctx, f) == 0 {
        free(ctx);
        return 0 as libc::c_int;
    }
    digest = xalloc(digest_length as size_t) as *mut uint8_t;
    ((*alg).digest).unwrap()(ctx, digest_length as size_t, digest);
    free(ctx);
    if raw != 0 {
        fwrite(
            digest as *const libc::c_void,
            digest_length as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            stdout,
        );
    } else {
        let mut i: libc::c_uint = 0;
        let mut hex: [libc::c_char; 17] = [0; 17];
        i = 0 as libc::c_int as libc::c_uint;
        while i.wrapping_add(8 as libc::c_int as libc::c_uint) < digest_length {
            nettle_base16_encode_update(
                hex.as_mut_ptr(),
                8 as libc::c_int as size_t,
                digest.offset(i as isize),
            );
            hex[(8 as libc::c_int * 2 as libc::c_int)
                as usize] = 0 as libc::c_int as libc::c_char;
            printf(b"%s \0" as *const u8 as *const libc::c_char, hex.as_mut_ptr());
            i = i.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        nettle_base16_encode_update(
            hex.as_mut_ptr(),
            digest_length.wrapping_sub(i) as size_t,
            digest.offset(i as isize),
        );
        hex[digest_length.wrapping_sub(i).wrapping_mul(2 as libc::c_int as libc::c_uint)
            as usize] = 0 as libc::c_int as libc::c_char;
        printf(
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            hex.as_mut_ptr(),
            (*alg).name,
        );
    }
    free(digest as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn usage(mut f: *mut FILE) {
    fprintf(
        f,
        b"Usage: nettle-hash -a ALGORITHM [OPTIONS] [FILE ...]\nOptions:\n  --help              Show this help.\n  -V, --version       Show version information.\n  --list              List supported hash algorithms.\n  -a, --algorithm=ALG Hash algorithm to use.\n  -l, --length=LENGTH Desired digest length (octets)\n  --raw               Raw binary output.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut alg_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut alg: *const nettle_hash = 0 as *const nettle_hash;
    let mut length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut raw: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    static mut options: [option; 7] = [
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
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"algorithm\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"length\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"list\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_LIST as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"raw\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_RAW as libc::c_int,
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
            b"Va:l:\0" as *const u8 as *const libc::c_char,
            options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            63 => {
                usage(stderr);
                return 1 as libc::c_int;
            }
            768 => {
                usage(stdout);
                return 0 as libc::c_int;
            }
            86 => {
                printf(
                    b"nettle-hash (nettle 3.9)\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            97 => {
                alg_name = optarg;
            }
            108 => {
                let mut arg: libc::c_int = 0;
                arg = atoi(optarg);
                if arg <= 0 as libc::c_int {
                    die(
                        b"Invalid length argument: `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                }
                length = arg as libc::c_uint;
            }
            769 => {
                raw = 1 as libc::c_int;
            }
            770 => {
                list_algorithms();
                return 0 as libc::c_int;
            }
            _ => {
                abort();
            }
        }
    }
    if alg_name.is_null() {
        die(
            b"Algorithm argument (-a option) is mandatory.\nSee nettle-hash --help for further information.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    alg = nettle_lookup_hash(alg_name);
    if alg.is_null() {
        die(
            b"Hash algorithm `%s' not supported or .\nUse nettle-hash --list to list available algorithms.\n\0"
                as *const u8 as *const libc::c_char,
            alg_name,
        );
    }
    if length == 0 as libc::c_int as libc::c_uint {
        length = (*alg).digest_size;
    } else if length > (*alg).digest_size {
        die(
            b"Length argument %d too large for selected algorithm.\n\0" as *const u8
                as *const libc::c_char,
            length,
        );
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc == 0 as libc::c_int {
        digest_file(alg, length, raw, stdin);
    } else {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < argc {
            let mut f: *mut FILE = fopen(
                *argv.offset(i as isize),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            if f.is_null() {
                die(
                    b"Cannot open `%s': %s\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                    strerror(*__errno_location()),
                );
            }
            printf(
                b"%s: \0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            );
            if digest_file(alg, length, raw, f) == 0 {
                die(
                    b"Reading `%s' failed: %s\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                    strerror(*__errno_location()),
                );
            }
            fclose(f);
            i += 1;
            i;
        }
    }
    if fflush(stdout) != 0 as libc::c_int {
        die(
            b"Write failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
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
