use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn nettle_yarrow256_seed(
        ctx: *mut yarrow256_ctx,
        length: size_t,
        seed_file: *const uint8_t,
    );
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub static mut quiet_flag: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if p.is_null() {
        fprintf(
            stderr,
            b"Virtual memory exhausted.\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return p;
}
pub unsafe extern "C" fn werror(mut format: *const libc::c_char, mut args: ...) {
    if quiet_flag == 0 {
        let mut args_0: ::std::ffi::VaListImpl;
        args_0 = args.clone();
        vfprintf(stderr, format, args_0.as_va_list());
    }
}
pub unsafe extern "C" fn read_file(
    mut name: *const libc::c_char,
    mut max_size: size_t,
    mut contents: *mut *mut uint8_t,
) -> size_t {
    let mut size: size_t = 0;
    let mut done: size_t = 0;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(name, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        werror(
            b"Opening `%s' failed: %s\n\0" as *const u8 as *const libc::c_char,
            name,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int as size_t;
    }
    size = 100 as libc::c_int as size_t;
    buffer = 0 as *mut uint8_t;
    done = 0 as libc::c_int as size_t;
    loop {
        let mut p: *mut uint8_t = 0 as *mut uint8_t;
        if max_size != 0 && size > max_size {
            size = max_size;
        }
        p = realloc(
            buffer as *mut libc::c_void,
            size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut uint8_t;
        if !p.is_null() {
            buffer = p;
            done = (done as libc::c_ulong)
                .wrapping_add(
                    fread(
                        buffer.offset(done as isize) as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        size.wrapping_sub(done),
                        f,
                    ),
                ) as size_t as size_t;
            if done < size {
                if ferror(f) != 0 {
                    fprintf(
                        stderr,
                        b"Reading `%s' failed: %s\n\0" as *const u8
                            as *const libc::c_char,
                        name,
                        strerror(*__errno_location()),
                    );
                } else if !(done == 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            } else {
                if size == max_size {
                    break;
                }
                size = (size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                continue;
            }
        }
        fclose(f);
        free(buffer as *mut libc::c_void);
        *contents = 0 as *mut uint8_t;
        return 0 as libc::c_int as size_t;
    }
    fclose(f);
    *buffer.offset(done as isize) = '\0' as i32 as uint8_t;
    *contents = buffer;
    return done;
}
pub unsafe extern "C" fn write_data(
    mut f: *mut FILE,
    mut size: size_t,
    mut buffer: *const libc::c_void,
) -> libc::c_int {
    let mut res: size_t = fwrite(buffer, 1 as libc::c_int as libc::c_ulong, size, f);
    return (res == size) as libc::c_int;
}
pub unsafe extern "C" fn write_file(
    mut name: *const libc::c_char,
    mut size: size_t,
    mut buffer: *const libc::c_void,
) -> libc::c_int {
    let mut f: *mut FILE = fopen(name, b"wb\0" as *const u8 as *const libc::c_char);
    let mut res: libc::c_int = 0;
    if f.is_null() {
        return 0 as libc::c_int;
    }
    res = write_data(f, size, buffer);
    return (fclose(f) == 0 as libc::c_int && res != 0) as libc::c_int;
}
pub unsafe extern "C" fn simple_random(
    mut ctx: *mut yarrow256_ctx,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut length: libc::c_uint = 0;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    if !name.is_null() {
        length = read_file(name, 0 as libc::c_int as size_t, &mut buffer)
            as libc::c_uint;
    } else {
        length = read_file(
            b"/dev/urandom\0" as *const u8 as *const libc::c_char,
            20 as libc::c_int as size_t,
            &mut buffer,
        ) as libc::c_uint;
    }
    if length == 0 {
        return 0 as libc::c_int;
    }
    nettle_yarrow256_seed(ctx, length as size_t, buffer);
    free(buffer as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn hash_file(
    mut hash: *const nettle_hash,
    mut ctx: *mut libc::c_void,
    mut f: *mut FILE,
) -> libc::c_int {
    loop {
        let mut buffer: [uint8_t; 1000] = [0; 1000];
        let mut res: size_t = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[uint8_t; 1000]>() as libc::c_ulong,
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
