use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn clock() -> clock_t;
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_out_str(_: *mut FILE, _: libc::c_int, _: mpz_srcptr) -> size_t;
    fn nettle_random_prime(
        p: *mut __mpz_struct,
        bits: libc::c_uint,
        top_bits_set: libc::c_int,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        progress_ctx: *mut libc::c_void,
        progress: Option::<nettle_progress_func>,
    );
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
    fn simple_random(ctx: *mut yarrow256_ctx, name: *const libc::c_char) -> libc::c_int;
    fn werror(format: *const libc::c_char, _: ...);
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
pub type __clock_t = libc::c_long;
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
pub type clock_t = __clock_t;
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
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
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
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"Usage: random-prime [OPTIONS] bits\n\nOptions:\n      --help         Display this message.\n  -v, --verbose      Display timing information.\n  -r, --random FILE  Random data to use for seeding.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut bits: libc::c_long = 0;
    let mut p: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
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
    let mut verbose: libc::c_int = 0 as libc::c_int;
    let mut random_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    let mut arg_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: clock_t = 0;
    let mut end: clock_t = 0;
    static mut options: [option; 4] = [
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
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
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
            b"vr:\0" as *const u8 as *const libc::c_char,
            options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            118 => {
                verbose = 1 as libc::c_int;
            }
            114 => {
                random_file = optarg;
            }
            300 => {
                usage();
                return 0 as libc::c_int;
            }
            63 => return 1 as libc::c_int,
            _ => {
                abort();
            }
        }
    }
    argc -= optind;
    argv = argv.offset(optind as isize);
    if argc != 1 as libc::c_int {
        usage();
        return 1 as libc::c_int;
    }
    bits = strtol(
        *argv.offset(0 as libc::c_int as isize),
        &mut arg_end,
        0 as libc::c_int,
    );
    if *arg_end as libc::c_int != 0 || bits < 0 as libc::c_int as libc::c_long {
        fprintf(stderr, b"Invalid number.\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if bits < 3 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            b"Bitsize must be at least 3.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    nettle_yarrow256_init(
        &mut yarrow,
        0 as libc::c_int as libc::c_uint,
        0 as *mut yarrow_source,
    );
    if simple_random(&mut yarrow, random_file) == 0 {
        werror(
            b"Initialization of randomness generator failed.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    __gmpz_init(p.as_mut_ptr());
    start = clock();
    nettle_random_prime(
        p.as_mut_ptr(),
        bits as libc::c_uint,
        0 as libc::c_int,
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
        None,
    );
    end = clock();
    __gmpz_out_str(stdout, 10 as libc::c_int, p.as_mut_ptr() as mpz_srcptr);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if verbose != 0 {
        fprintf(
            stderr,
            b"time: %.3g s\n\0" as *const u8 as *const libc::c_char,
            (end - start) as libc::c_double
                / 1000000 as libc::c_int as __clock_t as libc::c_double,
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
