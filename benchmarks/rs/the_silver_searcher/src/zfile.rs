use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type internal_state;
    pub type lzma_internal_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fopencookie(
        __magic_cookie: *mut libc::c_void,
        __modes: *const libc::c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn warn(__format: *const libc::c_char, _: ...);
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn zError(_: libc::c_int) -> *const libc::c_char;
    fn lzma_auto_decoder(
        strm: *mut lzma_stream,
        memlimit: uint64_t,
        flags: uint32_t,
    ) -> lzma_ret;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret;
    fn log_err(fmt: *const libc::c_char, _: ...);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off64_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type cookie_read_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
    size_t,
) -> __ssize_t;
pub type cookie_write_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> __ssize_t;
pub type cookie_seek_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut __off64_t,
    libc::c_int,
) -> libc::c_int;
pub type cookie_close_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_cookie_io_functions_t {
    pub read: Option::<cookie_read_function_t>,
    pub write: Option::<cookie_write_function_t>,
    pub seek: Option::<cookie_seek_function_t>,
    pub close: Option::<cookie_close_function_t>,
}
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
pub type lzma_reserved_enum = libc::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub type lzma_ret = libc::c_uint;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
pub type lzma_action = libc::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub opaque: *mut libc::c_void,
}
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
    pub total_in: uint64_t,
    pub next_out: *mut uint8_t,
    pub avail_out: size_t,
    pub total_out: uint64_t,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut libc::c_void,
    pub reserved_ptr2: *mut libc::c_void,
    pub reserved_ptr3: *mut libc::c_void,
    pub reserved_ptr4: *mut libc::c_void,
    pub reserved_int1: uint64_t,
    pub reserved_int2: uint64_t,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
}
pub type ag_compression_type = libc::c_uint;
pub const AG_XZ: ag_compression_type = 4;
pub const AG_ZIP: ag_compression_type = 3;
pub const AG_COMPRESS: ag_compression_type = 2;
pub const AG_GZIP: ag_compression_type = 1;
pub const AG_NO_COMPRESSION: ag_compression_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zfile {
    pub in_0: *mut FILE,
    pub logic_offset: uint64_t,
    pub decode_offset: uint64_t,
    pub actual_len: uint64_t,
    pub outbuf_start: uint32_t,
    pub ctype: ag_compression_type,
    pub stream: C2RustUnnamed,
    pub inbuf: [uint8_t; 32768],
    pub outbuf: [uint8_t; 262144],
    pub eof: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub gz: z_stream,
    pub lzma: lzma_stream,
}
static mut zfile_io: cookie_io_functions_t = unsafe {
    {
        let mut init = _IO_cookie_io_functions_t {
            read: Some(zfile_read as cookie_read_function_t),
            write: None,
            seek: Some(zfile_seek as cookie_seek_function_t),
            close: Some(zfile_close as cookie_close_function_t),
        };
        init
    }
};
unsafe extern "C" fn zfile_cookie_init(mut cookie: *mut zfile) -> libc::c_int {
    let mut lzrc: lzma_ret = LZMA_OK;
    let mut rc: libc::c_int = 0;
    if (*cookie).logic_offset == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cookie->logic_offset == 0\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int zfile_cookie_init(struct zfile *)\0"))
                .as_ptr(),
        );
    }
    'c_8449: {
        if (*cookie).logic_offset == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cookie->logic_offset == 0\0" as *const u8 as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int zfile_cookie_init(struct zfile *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*cookie).decode_offset == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cookie->decode_offset == 0\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int zfile_cookie_init(struct zfile *)\0"))
                .as_ptr(),
        );
    }
    'c_8405: {
        if (*cookie).decode_offset == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cookie->decode_offset == 0\0" as *const u8 as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int zfile_cookie_init(struct zfile *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*cookie).actual_len = 0 as libc::c_int as uint64_t;
    match (*cookie).ctype as libc::c_uint {
        1 => {
            memset(
                &mut (*cookie).stream.gz as *mut z_stream as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong,
            );
            rc = inflateInit2_(
                &mut (*cookie).stream.gz,
                32 as libc::c_int + 15 as libc::c_int,
                b"1.2.11\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            );
            if rc != 0 as libc::c_int {
                log_err(
                    b"Unable to initialize zlib: %s\0" as *const u8
                        as *const libc::c_char,
                    zError(rc),
                );
                return 5 as libc::c_int;
            }
            (*cookie).stream.gz.next_in = 0 as *mut Bytef;
            (*cookie).stream.gz.avail_in = 0 as libc::c_int as uInt;
            (*cookie).stream.gz.next_out = ((*cookie).outbuf).as_mut_ptr();
            (*cookie)
                .stream
                .gz
                .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>() as libc::c_ulong
                as uInt;
        }
        4 => {
            (*cookie)
                .stream
                .lzma = {
                let mut init = lzma_stream {
                    next_in: 0 as *const uint8_t,
                    avail_in: 0 as libc::c_int as size_t,
                    total_in: 0 as libc::c_int as uint64_t,
                    next_out: 0 as *mut uint8_t,
                    avail_out: 0 as libc::c_int as size_t,
                    total_out: 0 as libc::c_int as uint64_t,
                    allocator: 0 as *const lzma_allocator,
                    internal: 0 as *mut lzma_internal,
                    reserved_ptr1: 0 as *mut libc::c_void,
                    reserved_ptr2: 0 as *mut libc::c_void,
                    reserved_ptr3: 0 as *mut libc::c_void,
                    reserved_ptr4: 0 as *mut libc::c_void,
                    reserved_int1: 0 as libc::c_int as uint64_t,
                    reserved_int2: 0 as libc::c_int as uint64_t,
                    reserved_int3: 0 as libc::c_int as size_t,
                    reserved_int4: 0 as libc::c_int as size_t,
                    reserved_enum1: LZMA_RESERVED_ENUM,
                    reserved_enum2: LZMA_RESERVED_ENUM,
                };
                init
            };
            lzrc = lzma_auto_decoder(
                &mut (*cookie).stream.lzma,
                -(1 as libc::c_int) as uint64_t,
                0 as libc::c_int as uint32_t,
            );
            if lzrc as libc::c_uint != LZMA_OK as libc::c_int as libc::c_uint {
                log_err(
                    b"Unable to initialize lzma_auto_decoder: %d\0" as *const u8
                        as *const libc::c_char,
                    lzrc as libc::c_uint,
                );
                return 5 as libc::c_int;
            }
            (*cookie).stream.lzma.next_in = 0 as *const uint8_t;
            (*cookie).stream.lzma.avail_in = 0 as libc::c_int as size_t;
            (*cookie).stream.lzma.next_out = ((*cookie).outbuf).as_mut_ptr();
            (*cookie)
                .stream
                .lzma
                .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>() as libc::c_ulong;
        }
        _ => {
            log_err(
                b"Unsupported compression type: %d\0" as *const u8
                    as *const libc::c_char,
                (*cookie).ctype as libc::c_uint,
            );
            return 22 as libc::c_int;
        }
    }
    (*cookie).outbuf_start = 0 as libc::c_int as uint32_t;
    (*cookie).eof = 0 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn zfile_cookie_cleanup(mut cookie: *mut zfile) {
    match (*cookie).ctype as libc::c_uint {
        1 => {
            inflateEnd(&mut (*cookie).stream.gz);
        }
        4 => {
            lzma_end(&mut (*cookie).stream.lzma);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn decompress_open(
    mut fd: libc::c_int,
    mut mode: *const libc::c_char,
    mut ctype: ag_compression_type,
) -> *mut FILE {
    let mut cookie: *mut zfile = 0 as *mut zfile;
    let mut res: *mut FILE = 0 as *mut FILE;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut error: libc::c_int = 0;
    cookie = 0 as *mut zfile;
    res = 0 as *mut FILE;
    in_0 = res;
    if !(strstr(mode, b"w\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(mode, b"a\0" as *const u8 as *const libc::c_char)).is_null()
    {
        *__errno_location() = 22 as libc::c_int;
    } else {
        in_0 = fdopen(fd, mode);
        if !in_0.is_null() {
            cookie = malloc(::std::mem::size_of::<zfile>() as libc::c_ulong)
                as *mut zfile;
            if cookie.is_null() {
                *__errno_location() = 12 as libc::c_int;
            } else {
                (*cookie).in_0 = in_0;
                (*cookie).logic_offset = 0 as libc::c_int as uint64_t;
                (*cookie).decode_offset = 0 as libc::c_int as uint64_t;
                (*cookie).ctype = ctype;
                error = zfile_cookie_init(cookie);
                if error != 0 as libc::c_int {
                    *__errno_location() = error;
                } else {
                    res = fopencookie(cookie as *mut libc::c_void, mode, zfile_io);
                }
            }
        }
    }
    if res.is_null() {
        if !in_0.is_null() {
            fclose(in_0);
        }
        if !cookie.is_null() {
            free(cookie as *mut libc::c_void);
        }
    }
    return res;
}
unsafe extern "C" fn zfile_read(
    mut cookie_: *mut libc::c_void,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> __ssize_t {
    let mut cookie: *mut zfile = cookie_ as *mut zfile;
    let mut nb: size_t = 0;
    let mut ignorebytes: size_t = 0;
    let mut total: ssize_t = 0 as libc::c_int as ssize_t;
    let mut lzret: lzma_ret = LZMA_OK;
    let mut ret: libc::c_int = 0;
    if size <= 9223372036854775807 as libc::c_long as libc::c_ulong {} else {
        __assert_fail(
            b"size <= SSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8007: {
        if size <= 9223372036854775807 as libc::c_long as libc::c_ulong {} else {
            __assert_fail(
                b"size <= SSIZE_MAX\0" as *const u8 as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                213 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as __ssize_t;
    }
    if (*cookie).eof {
        return 0 as libc::c_int as __ssize_t;
    }
    ret = 0 as libc::c_int;
    lzret = LZMA_OK;
    ignorebytes = ((*cookie).logic_offset).wrapping_sub((*cookie).decode_offset);
    if ignorebytes == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ignorebytes == 0\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_7935: {
        if ignorebytes == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ignorebytes == 0\0" as *const u8 as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    loop {
        let mut inflated: size_t = 0;
        while (if (*cookie).ctype as libc::c_uint
            == AG_GZIP as libc::c_int as libc::c_uint
        {
            (*cookie).stream.gz.next_out
        } else {
            (*cookie).stream.lzma.next_out
        })
            > &mut *((*cookie).outbuf)
                .as_mut_ptr()
                .offset((*cookie).outbuf_start as isize) as *mut uint8_t
        {
            let mut left: size_t = (if (*cookie).ctype as libc::c_uint
                == AG_GZIP as libc::c_int as libc::c_uint
            {
                (*cookie).stream.gz.next_out
            } else {
                (*cookie).stream.lzma.next_out
            })
                .offset_from(
                    &mut *((*cookie).outbuf)
                        .as_mut_ptr()
                        .offset((*cookie).outbuf_start as isize) as *mut uint8_t,
                ) as libc::c_long as size_t;
            let mut ignoreskip: size_t = ({
                let mut _a: size_t = ignorebytes;
                let mut _b: size_t = left;
                if _a < _b { _a } else { _b }
            });
            let mut toread: size_t = 0;
            if ignoreskip > 0 as libc::c_int as libc::c_ulong {
                ignorebytes = (ignorebytes as libc::c_ulong).wrapping_sub(ignoreskip)
                    as size_t as size_t;
                left = (left as libc::c_ulong).wrapping_sub(ignoreskip) as size_t
                    as size_t;
                (*cookie)
                    .outbuf_start = ((*cookie).outbuf_start as libc::c_ulong)
                    .wrapping_add(ignoreskip) as uint32_t as uint32_t;
                (*cookie)
                    .decode_offset = ((*cookie).decode_offset as libc::c_ulong)
                    .wrapping_add(ignoreskip) as uint64_t as uint64_t;
            }
            if ignorebytes > 0 as libc::c_int as libc::c_ulong {
                break;
            }
            toread = ({
                let mut _a: size_t = left;
                let mut _b: size_t = size;
                if _a < _b { _a } else { _b }
            });
            memcpy(
                buf as *mut libc::c_void,
                &mut *((*cookie).outbuf)
                    .as_mut_ptr()
                    .offset((*cookie).outbuf_start as isize) as *mut uint8_t
                    as *const libc::c_void,
                toread,
            );
            buf = buf.offset(toread as isize);
            size = (size as libc::c_ulong).wrapping_sub(toread) as size_t as size_t;
            left = (left as libc::c_ulong).wrapping_sub(toread) as size_t as size_t;
            (*cookie)
                .outbuf_start = ((*cookie).outbuf_start as libc::c_ulong)
                .wrapping_add(toread) as uint32_t as uint32_t;
            (*cookie)
                .decode_offset = ((*cookie).decode_offset as libc::c_ulong)
                .wrapping_add(toread) as uint64_t as uint64_t;
            (*cookie)
                .logic_offset = ((*cookie).logic_offset as libc::c_ulong)
                .wrapping_add(toread) as uint64_t as uint64_t;
            total = (total as libc::c_ulong).wrapping_add(toread) as ssize_t as ssize_t;
            if size == 0 as libc::c_int as libc::c_ulong {
                break;
            }
        }
        if size == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if (*cookie).stream.gz.next_out
            == &mut *((*cookie).outbuf)
                .as_mut_ptr()
                .offset((*cookie).outbuf_start as isize) as *mut uint8_t
        {} else {
            __assert_fail(
                b"cookie->stream.gz.next_out == &cookie->outbuf[cookie->outbuf_start]\0"
                    as *const u8 as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                273 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                    .as_ptr(),
            );
        }
        'c_7625: {
            if (*cookie).stream.gz.next_out
                == &mut *((*cookie).outbuf)
                    .as_mut_ptr()
                    .offset((*cookie).outbuf_start as isize) as *mut uint8_t
            {} else {
                __assert_fail(
                    b"cookie->stream.gz.next_out == &cookie->outbuf[cookie->outbuf_start]\0"
                        as *const u8 as *const libc::c_char,
                    b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                    273 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 45],
                        &[libc::c_char; 45],
                    >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*cookie).ctype as libc::c_uint == AG_XZ as libc::c_int as libc::c_uint
            && lzret as libc::c_uint == LZMA_STREAM_END as libc::c_int as libc::c_uint
            || (*cookie).ctype as libc::c_uint == AG_GZIP as libc::c_int as libc::c_uint
                && ret == 1 as libc::c_int
        {
            (*cookie).eof = 1 as libc::c_int != 0;
            break;
        } else {
            if (if (*cookie).ctype as libc::c_uint
                == AG_GZIP as libc::c_int as libc::c_uint
            {
                (*cookie).stream.gz.avail_in as libc::c_ulong
            } else {
                (*cookie).stream.lzma.avail_in
            }) == 0 as libc::c_int as libc::c_ulong
            {
                nb = fread(
                    ((*cookie).inbuf).as_mut_ptr() as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<[uint8_t; 32768]>() as libc::c_ulong,
                    (*cookie).in_0,
                );
                if ferror((*cookie).in_0) != 0 {
                    warn(b"error read core\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
                if nb == 0 as libc::c_int as libc::c_ulong && feof((*cookie).in_0) != 0 {
                    warn(b"truncated file\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
                if (*cookie).ctype as libc::c_uint
                    == AG_XZ as libc::c_int as libc::c_uint
                {
                    (*cookie).stream.lzma.avail_in = nb;
                    (*cookie).stream.lzma.next_in = ((*cookie).inbuf).as_mut_ptr();
                } else {
                    (*cookie).stream.gz.avail_in = nb as uInt;
                    (*cookie).stream.gz.next_in = ((*cookie).inbuf).as_mut_ptr();
                }
            }
            if (*cookie).ctype as libc::c_uint == AG_XZ as libc::c_int as libc::c_uint {
                (*cookie).stream.lzma.next_out = ((*cookie).outbuf).as_mut_ptr();
                (*cookie)
                    .stream
                    .lzma
                    .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>()
                    as libc::c_ulong;
            } else {
                (*cookie).stream.gz.next_out = ((*cookie).outbuf).as_mut_ptr();
                (*cookie)
                    .stream
                    .gz
                    .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>()
                    as libc::c_ulong as uInt;
            }
            (*cookie).outbuf_start = 0 as libc::c_int as uint32_t;
            if (*cookie).ctype as libc::c_uint == AG_GZIP as libc::c_int as libc::c_uint
            {
                ret = inflate(&mut (*cookie).stream.gz, 0 as libc::c_int);
                if ret != 0 as libc::c_int && ret != 1 as libc::c_int {
                    log_err(
                        b"Found mem/data error while decompressing zlib stream: %s\0"
                            as *const u8 as *const libc::c_char,
                        zError(ret),
                    );
                    return -(1 as libc::c_int) as __ssize_t;
                }
            } else {
                lzret = lzma_code(&mut (*cookie).stream.lzma, LZMA_RUN);
                if lzret as libc::c_uint != LZMA_OK as libc::c_int as libc::c_uint
                    && lzret as libc::c_uint
                        != LZMA_STREAM_END as libc::c_int as libc::c_uint
                {
                    log_err(
                        b"Found mem/data error while decompressing xz/lzma stream: %d\0"
                            as *const u8 as *const libc::c_char,
                        lzret as libc::c_uint,
                    );
                    return -(1 as libc::c_int) as __ssize_t;
                }
            }
            inflated = (if (*cookie).ctype as libc::c_uint
                == AG_GZIP as libc::c_int as libc::c_uint
            {
                (*cookie).stream.gz.next_out
            } else {
                (*cookie).stream.lzma.next_out
            })
                .offset_from(
                    &mut *((*cookie).outbuf)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t,
                ) as libc::c_long as size_t;
            (*cookie)
                .actual_len = ((*cookie).actual_len as libc::c_ulong)
                .wrapping_add(inflated) as uint64_t as uint64_t;
            if !(ferror((*cookie).in_0) == 0 && size > 0 as libc::c_int as libc::c_ulong)
            {
                break;
            }
        }
    }
    if total <= 9223372036854775807 as libc::c_long {} else {
        __assert_fail(
            b"total <= SSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_7156: {
        if total <= 9223372036854775807 as libc::c_long {} else {
            __assert_fail(
                b"total <= SSIZE_MAX\0" as *const u8 as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                329 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"__ssize_t zfile_read(void *, char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return total;
}
unsafe extern "C" fn zfile_seek(
    mut cookie_: *mut libc::c_void,
    mut offset_: *mut off64_t,
    mut whence: libc::c_int,
) -> libc::c_int {
    let mut cookie: *mut zfile = cookie_ as *mut zfile;
    let mut new_offset: off64_t = 0 as libc::c_int as off64_t;
    let mut offset: off64_t = *offset_;
    if whence == 0 as libc::c_int {
        new_offset = offset;
    } else if whence == 1 as libc::c_int {
        new_offset = (*cookie).logic_offset as off64_t + offset;
    } else {
        return -(1 as libc::c_int)
    }
    if new_offset < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    if new_offset < (*cookie).logic_offset as off64_t
        && new_offset != 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if new_offset == 0 as libc::c_int as libc::c_long {
        (*cookie).decode_offset = 0 as libc::c_int as uint64_t;
        (*cookie).logic_offset = 0 as libc::c_int as uint64_t;
        zfile_cookie_cleanup(cookie);
        zfile_cookie_init(cookie);
    } else if new_offset as uint64_t > (*cookie).logic_offset {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        let bsz: size_t = (32 as libc::c_int * 1024 as libc::c_int) as size_t;
        buf = malloc(bsz) as *mut libc::c_char;
        while new_offset as uint64_t > (*cookie).logic_offset {
            let mut diff: size_t = ({
                let _a: size_t = bsz;
                let mut _b: libc::c_ulong = (new_offset as uint64_t)
                    .wrapping_sub((*cookie).logic_offset);
                if _a < _b { _a } else { _b }
            });
            let mut err: ssize_t = zfile_read(cookie_, buf, diff);
            if err < 0 as libc::c_int as libc::c_long {
                free(buf as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            if !(err == 0 as libc::c_int as libc::c_long) {
                continue;
            }
            if (*cookie).eof {} else {
                __assert_fail(
                    b"cookie->eof\0" as *const u8 as *const libc::c_char,
                    b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                    378 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"int zfile_seek(void *, off64_t *, int)\0"))
                        .as_ptr(),
                );
            }
            'c_7042: {
                if (*cookie).eof {} else {
                    __assert_fail(
                        b"cookie->eof\0" as *const u8 as *const libc::c_char,
                        b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                        378 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 39],
                            &[libc::c_char; 39],
                        >(b"int zfile_seek(void *, off64_t *, int)\0"))
                            .as_ptr(),
                    );
                }
            };
            new_offset = (*cookie).logic_offset as off64_t;
            break;
        }
        free(buf as *mut libc::c_void);
    }
    if (*cookie).logic_offset == new_offset as uint64_t {} else {
        __assert_fail(
            b"cookie->logic_offset == (uint64_t)new_offset\0" as *const u8
                as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            386 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int zfile_seek(void *, off64_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_6954: {
        if (*cookie).logic_offset == new_offset as uint64_t {} else {
            __assert_fail(
                b"cookie->logic_offset == (uint64_t)new_offset\0" as *const u8
                    as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                386 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"int zfile_seek(void *, off64_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    *offset_ = new_offset;
    return 0 as libc::c_int;
}
unsafe extern "C" fn zfile_close(mut cookie_: *mut libc::c_void) -> libc::c_int {
    let mut cookie: *mut zfile = cookie_ as *mut zfile;
    zfile_cookie_cleanup(cookie);
    fclose((*cookie).in_0);
    free(cookie as *mut libc::c_void);
    return 0 as libc::c_int;
}
