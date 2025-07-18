use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
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
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn nettle_pbkdf2_hmac_sha256(
        key_length: size_t,
        key: *const uint8_t,
        iterations: libc::c_uint,
        salt_length: size_t,
        salt: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
    );
    fn nettle_base16_encode_update(
        dst: *mut libc::c_char,
        length: size_t,
        src: *const uint8_t,
    );
    fn nettle_base16_decode_init(ctx: *mut base16_decode_ctx);
    fn nettle_base16_decode_update(
        ctx: *mut base16_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_base16_decode_final(ctx: *mut base16_decode_ctx) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: libc::c_uchar,
    pub bits: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const OPT_HEX_SALT: C2RustUnnamed = 770;
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
unsafe extern "C" fn usage(mut f: *mut FILE) {
    fprintf(
        f,
        b"Usage: nettle-pbkdf2 [OPTIONS] SALT\nOptions:\n  --help                 Show this help.\n  -V, --version          Show version information.\n  -i, --iterations=COUNT Desired iteration count (default %d).\n  -l, --length=LENGTH    Desired output length (octets, default %d)\n  --raw                  Raw binary output.\n  --hex-salt             Use hex encoding for the salt.\n\0"
            as *const u8 as *const libc::c_char,
        10000 as libc::c_int,
        16 as libc::c_int,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut iterations: libc::c_uint = 10000 as libc::c_int as libc::c_uint;
    let mut output_length: libc::c_uint = 16 as libc::c_int as libc::c_uint;
    let mut password: [libc::c_char; 1024] = [0; 1024];
    let mut password_length: size_t = 0;
    let mut output: *mut uint8_t = 0 as *mut uint8_t;
    let mut salt_length: size_t = 0;
    let mut salt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut raw: libc::c_int = 0 as libc::c_int;
    let mut hex_salt: libc::c_int = 0 as libc::c_int;
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
                name: b"length\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"iterations\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
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
                name: b"hex-salt\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: OPT_HEX_SALT as libc::c_int,
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
            b"Vl:i:\0" as *const u8 as *const libc::c_char,
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
                    b"nettle-pbkdf2 (nettle 3.9)\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
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
                output_length = arg as libc::c_uint;
            }
            105 => {
                let mut arg_0: libc::c_int = 0;
                arg_0 = atoi(optarg);
                if arg_0 <= 0 as libc::c_int {
                    die(
                        b"Invalid iteration count: `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                }
                iterations = arg_0 as libc::c_uint;
            }
            769 => {
                raw = 1 as libc::c_int;
            }
            770 => {
                hex_salt = 1 as libc::c_int;
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
    salt = strdup(*argv.offset(0 as libc::c_int as isize));
    if salt.is_null() {
        die(
            b"strdup failed: Virtual memory exhausted.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    salt_length = strlen(*argv.offset(0 as libc::c_int as isize));
    if hex_salt != 0 {
        let mut base16: base16_decode_ctx = base16_decode_ctx {
            word: 0,
            bits: 0,
        };
        nettle_base16_decode_init(&mut base16);
        if nettle_base16_decode_update(
            &mut base16,
            &mut salt_length,
            salt as *mut uint8_t,
            salt_length,
            salt,
        ) == 0 || nettle_base16_decode_final(&mut base16) == 0
        {
            die(
                b"Invalid salt (expecting hex encoding).\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    password_length = fread(
        password.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        stdin,
    );
    if password_length == ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
    {
        die(
            b"Password input too long. Current limit is %d characters.\n\0" as *const u8
                as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int
                - 1 as libc::c_int,
        );
    }
    if ferror(stdin) != 0 {
        die(
            b"Reading password input failed: %s.\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    output = xalloc(output_length as size_t) as *mut uint8_t;
    nettle_pbkdf2_hmac_sha256(
        password_length,
        password.as_mut_ptr() as *const uint8_t,
        iterations,
        salt_length,
        salt as *const uint8_t,
        output_length as size_t,
        output,
    );
    free(salt as *mut libc::c_void);
    if raw != 0 {
        fwrite(
            output as *const libc::c_void,
            output_length as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            stdout,
        );
    } else {
        let mut i: libc::c_uint = 0;
        let mut hex: [libc::c_char; 17] = [0; 17];
        i = 0 as libc::c_int as libc::c_uint;
        while i.wrapping_add(8 as libc::c_int as libc::c_uint) < output_length {
            nettle_base16_encode_update(
                hex.as_mut_ptr(),
                8 as libc::c_int as size_t,
                output.offset(i as isize),
            );
            hex[(8 as libc::c_int * 2 as libc::c_int)
                as usize] = 0 as libc::c_int as libc::c_char;
            printf(
                b"%s%c\0" as *const u8 as *const libc::c_char,
                hex.as_mut_ptr(),
                if i.wrapping_rem(64 as libc::c_int as libc::c_uint)
                    == 56 as libc::c_int as libc::c_uint
                {
                    '\n' as i32
                } else {
                    ' ' as i32
                },
            );
            i = i.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        nettle_base16_encode_update(
            hex.as_mut_ptr(),
            output_length.wrapping_sub(i) as size_t,
            output.offset(i as isize),
        );
        hex[output_length.wrapping_sub(i).wrapping_mul(2 as libc::c_int as libc::c_uint)
            as usize] = 0 as libc::c_int as libc::c_char;
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, hex.as_mut_ptr());
    }
    free(output as *mut libc::c_void);
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
