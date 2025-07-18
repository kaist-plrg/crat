use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    static mut stdin: *mut FILE;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_set_str(_: mpz_ptr, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn nettle_sha1_init(ctx: *mut sha1_ctx);
    fn nettle_rsa_public_key_init(key: *mut rsa_public_key);
    fn nettle_rsa_public_key_clear(key: *mut rsa_public_key);
    fn nettle_rsa_sha1_verify(
        key: *const rsa_public_key,
        hash: *mut sha1_ctx,
        signature: *const __mpz_struct,
    ) -> libc::c_int;
    static nettle_sha1: nettle_hash;
    fn werror(format: *const libc::c_char, _: ...);
    fn read_file(
        name: *const libc::c_char,
        size: size_t,
        buffer: *mut *mut uint8_t,
    ) -> size_t;
    fn hash_file(
        hash: *const nettle_hash,
        ctx: *mut libc::c_void,
        f: *mut FILE,
    ) -> libc::c_int;
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
pub struct nettle_hash {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
unsafe extern "C" fn read_signature(
    mut name: *const libc::c_char,
    mut s: *mut __mpz_struct,
) -> libc::c_int {
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut length: libc::c_uint = 0;
    let mut res: libc::c_int = 0;
    length = read_file(name, 0 as libc::c_int as size_t, &mut buffer) as libc::c_uint;
    if length == 0 {
        return 0 as libc::c_int;
    }
    res = (__gmpz_set_str(s, buffer as *const libc::c_char, 16 as libc::c_int)
        == 0 as libc::c_int) as libc::c_int;
    free(buffer as *mut libc::c_void);
    return res;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
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
    let mut hash: sha1_ctx = sha1_ctx {
        state: [0; 5],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut s: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if argc != 3 as libc::c_int {
        werror(
            b"Usage: rsa-verify PUBLIC-KEY SIGNATURE-FILE < FILE\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    nettle_rsa_public_key_init(&mut key);
    if read_rsa_key(
        *argv.offset(1 as libc::c_int as isize),
        &mut key,
        0 as *mut rsa_private_key,
    ) == 0
    {
        werror(b"Invalid key\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    __gmpz_init(s.as_mut_ptr());
    if read_signature(*argv.offset(2 as libc::c_int as isize), s.as_mut_ptr()) == 0 {
        werror(
            b"Failed to read signature file `%s'\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(2 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    nettle_sha1_init(&mut hash);
    if hash_file(&nettle_sha1, &mut hash as *mut sha1_ctx as *mut libc::c_void, stdin)
        == 0
    {
        werror(
            b"Failed reading stdin: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    if nettle_rsa_sha1_verify(&mut key, &mut hash, s.as_mut_ptr() as *const __mpz_struct)
        == 0
    {
        werror(b"Invalid signature!\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    __gmpz_clear(s.as_mut_ptr());
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
