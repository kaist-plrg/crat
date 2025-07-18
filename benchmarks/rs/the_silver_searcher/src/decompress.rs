use ::libc;
extern "C" {
    pub type lzma_internal_s;
    pub type internal_state;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn log_debug(fmt: *const libc::c_char, _: ...);
    fn log_err(fmt: *const libc::c_char, _: ...);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret;
    fn lzma_auto_decoder(
        strm: *mut lzma_stream,
        memlimit: uint64_t,
        flags: uint32_t,
    ) -> lzma_ret;
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type ag_compression_type = libc::c_uint;
pub const AG_XZ: ag_compression_type = 4;
pub const AG_ZIP: ag_compression_type = 3;
pub const AG_COMPRESS: ag_compression_type = 2;
pub const AG_GZIP: ag_compression_type = 1;
pub const AG_NO_COMPRESSION: ag_compression_type = 0;
pub type uint8_t = __uint8_t;
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
pub type lzma_reserved_enum = libc::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub type uint64_t = __uint64_t;
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub opaque: *mut libc::c_void,
}
pub const LZMA_STREAM_END: lzma_ret = 1;
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
pub const LZMA_OK: lzma_ret = 0;
pub type lzma_action = libc::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
pub type uint32_t = __uint32_t;
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *const libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type uLong = libc::c_ulong;
pub type voidpf = *mut libc::c_void;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type uInt = libc::c_uint;
pub type Bytef = Byte;
pub type Byte = libc::c_uchar;
pub type z_streamp = *mut z_stream;
pub static mut XZ_HEADER_MAGIC: [uint8_t; 6] = [
    0xfd as libc::c_int as uint8_t,
    '7' as i32 as uint8_t,
    'z' as i32 as uint8_t,
    'X' as i32 as uint8_t,
    'Z' as i32 as uint8_t,
    0 as libc::c_int as uint8_t,
];
pub static mut LZMA_HEADER_SOMETIMES: [uint8_t; 3] = [
    0x5d as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
unsafe extern "C" fn decompress_zlib(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut result_size: size_t = 0 as libc::c_int as size_t;
    let mut pagesize: size_t = 0 as libc::c_int as size_t;
    let mut stream: z_stream = z_stream {
        next_in: 0 as *const Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *const libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    log_debug(
        b"Decompressing zlib file %s\0" as *const u8 as *const libc::c_char,
        dir_full_path,
    );
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = 0 as voidpf;
    stream.avail_in = 0 as libc::c_int as uInt;
    stream.next_in = 0 as *const Bytef;
    if inflateInit2_(
        &mut stream,
        32 as libc::c_int + 15 as libc::c_int,
        b"1.2.11\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    ) != 0 as libc::c_int
    {
        log_err(
            b"Unable to initialize zlib: %s\0" as *const u8 as *const libc::c_char,
            stream.msg,
        );
    } else {
        stream.avail_in = buf_len as uInt;
        stream.next_in = buf as *mut Bytef;
        pagesize = getpagesize() as size_t;
        result_size = (buf_len as libc::c_ulong)
            .wrapping_add(pagesize)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !pagesize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            let mut tmp_result: *mut libc::c_uchar = result;
            result_size = (result_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            result = realloc(
                result as *mut libc::c_void,
                result_size
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_uchar;
            if result.is_null() {
                free(tmp_result as *mut libc::c_void);
                log_err(
                    b"Unable to allocate %d bytes to decompress file %s\0" as *const u8
                        as *const libc::c_char,
                    result_size
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                    dir_full_path,
                );
                inflateEnd(&mut stream);
                current_block = 17794198092709139953;
                break;
            } else {
                stream
                    .avail_out = result_size
                    .wrapping_div(2 as libc::c_int as libc::c_ulong) as uInt;
                stream
                    .next_out = &mut *result.offset(stream.total_out as isize)
                    as *mut libc::c_uchar;
                ret = inflate(&mut stream, 2 as libc::c_int);
                log_debug(
                    b"inflate ret = %d\0" as *const u8 as *const libc::c_char,
                    ret,
                );
                match ret {
                    -2 => {
                        log_err(
                            b"Found stream error while decompressing zlib stream: %s\0"
                                as *const u8 as *const libc::c_char,
                            stream.msg,
                        );
                        inflateEnd(&mut stream);
                        current_block = 17794198092709139953;
                        break;
                    }
                    2 | -3 | -4 => {
                        log_err(
                            b"Found mem/data error while decompressing zlib stream: %s\0"
                                as *const u8 as *const libc::c_char,
                            stream.msg,
                        );
                        inflateEnd(&mut stream);
                        current_block = 17794198092709139953;
                        break;
                    }
                    _ => {
                        if stream.avail_out == 0 as libc::c_int as libc::c_uint {
                            continue;
                        }
                        if !(ret == 0 as libc::c_int) {
                            current_block = 6669252993407410313;
                            break;
                        }
                    }
                }
            }
        }
        match current_block {
            17794198092709139953 => {}
            _ => {
                *new_buf_len = stream.total_out as libc::c_int;
                inflateEnd(&mut stream);
                if ret == 1 as libc::c_int {
                    return result as *mut libc::c_void;
                }
            }
        }
    }
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn decompress_lzw(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    log_err(
        b"LZW (UNIX compress) files not yet supported: %s\0" as *const u8
            as *const libc::c_char,
        dir_full_path,
    );
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn decompress_zip(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    log_err(
        b"Zip files not yet supported: %s\0" as *const u8 as *const libc::c_char,
        dir_full_path,
    );
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn decompress_lzma(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut stream: lzma_stream = {
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
    let mut lzrt: lzma_ret = LZMA_OK;
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut result_size: size_t = 0 as libc::c_int as size_t;
    let mut pagesize: size_t = 0 as libc::c_int as size_t;
    stream.avail_in = buf_len as size_t;
    stream.next_in = buf as *const uint8_t;
    lzrt = lzma_auto_decoder(
        &mut stream,
        -(1 as libc::c_int) as uint64_t,
        0 as libc::c_int as uint32_t,
    );
    if lzrt as libc::c_uint != LZMA_OK as libc::c_int as libc::c_uint {
        log_err(
            b"Unable to initialize lzma_auto_decoder: %d\0" as *const u8
                as *const libc::c_char,
            lzrt as libc::c_uint,
        );
    } else {
        pagesize = getpagesize() as size_t;
        result_size = (buf_len as libc::c_ulong)
            .wrapping_add(pagesize)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !pagesize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            let mut tmp_result: *mut libc::c_uchar = result;
            result_size = (result_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            result = realloc(
                result as *mut libc::c_void,
                result_size
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_uchar;
            if result.is_null() {
                free(tmp_result as *mut libc::c_void);
                log_err(
                    b"Unable to allocate %d bytes to decompress file %s\0" as *const u8
                        as *const libc::c_char,
                    result_size
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                    dir_full_path,
                );
                current_block = 17426221588567527095;
                break;
            } else {
                stream
                    .avail_out = result_size
                    .wrapping_div(2 as libc::c_int as libc::c_ulong);
                stream
                    .next_out = &mut *result.offset(stream.total_out as isize)
                    as *mut libc::c_uchar;
                lzrt = lzma_code(&mut stream, LZMA_RUN);
                log_debug(
                    b"lzma_code ret = %d\0" as *const u8 as *const libc::c_char,
                    lzrt as libc::c_uint,
                );
                match lzrt as libc::c_uint {
                    0 | 1 => {
                        if stream.avail_out == 0 as libc::c_int as libc::c_ulong {
                            continue;
                        }
                        if !(lzrt as libc::c_uint
                            == LZMA_OK as libc::c_int as libc::c_uint)
                        {
                            current_block = 2838571290723028321;
                            break;
                        }
                    }
                    _ => {
                        log_err(
                            b"Found mem/data error while decompressing xz/lzma stream: %d\0"
                                as *const u8 as *const libc::c_char,
                            lzrt as libc::c_uint,
                        );
                        current_block = 17426221588567527095;
                        break;
                    }
                }
            }
        }
        match current_block {
            17426221588567527095 => {}
            _ => {
                *new_buf_len = stream.total_out as libc::c_int;
                if lzrt as libc::c_uint == LZMA_STREAM_END as libc::c_int as libc::c_uint
                {
                    lzma_end(&mut stream);
                    return result as *mut libc::c_void;
                }
            }
        }
    }
    lzma_end(&mut stream);
    *new_buf_len = 0 as libc::c_int;
    if !result.is_null() {
        free(result as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn decompress(
    zip_type: ag_compression_type,
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    match zip_type as libc::c_uint {
        1 => return decompress_zlib(buf, buf_len, dir_full_path, new_buf_len),
        2 => return decompress_lzw(buf, buf_len, dir_full_path, new_buf_len),
        3 => return decompress_zip(buf, buf_len, dir_full_path, new_buf_len),
        4 => return decompress_lzma(buf, buf_len, dir_full_path, new_buf_len),
        0 => {
            log_err(
                b"File %s is not compressed\0" as *const u8 as *const libc::c_char,
                dir_full_path,
            );
        }
        _ => {
            log_err(
                b"Unsupported compression type: %d\0" as *const u8
                    as *const libc::c_char,
                zip_type as libc::c_uint,
            );
        }
    }
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn is_zipped(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
) -> ag_compression_type {
    let mut buf_c: *const libc::c_uchar = buf as *const libc::c_uchar;
    if buf_len == 0 as libc::c_int {
        return AG_NO_COMPRESSION;
    }
    if buf_len >= 2 as libc::c_int {
        if *buf_c.offset(0 as libc::c_int as isize) as libc::c_int == 0x1f as libc::c_int
        {
            if *buf_c.offset(1 as libc::c_int as isize) as libc::c_int
                == 0x8b as libc::c_int
            {
                log_debug(
                    b"Found gzip-based stream\0" as *const u8 as *const libc::c_char,
                );
                return AG_GZIP;
            } else if *buf_c.offset(1 as libc::c_int as isize) as libc::c_int
                == 0x9b as libc::c_int
            {
                log_debug(
                    b"Found compress-based stream\0" as *const u8 as *const libc::c_char,
                );
                return AG_COMPRESS;
            }
        }
    }
    if buf_len >= 4 as libc::c_int {
        if *buf_c.offset(0 as libc::c_int as isize) as libc::c_int == 0x50 as libc::c_int
            && *buf_c.offset(1 as libc::c_int as isize) as libc::c_int
                == 0x4b as libc::c_int
            && *buf_c.offset(2 as libc::c_int as isize) as libc::c_int
                == 0x3 as libc::c_int
            && *buf_c.offset(3 as libc::c_int as isize) as libc::c_int
                == 0x4 as libc::c_int
        {
            log_debug(b"Found zip-based stream\0" as *const u8 as *const libc::c_char);
            return AG_ZIP;
        }
    }
    if buf_len >= 6 as libc::c_int {
        if memcmp(
            XZ_HEADER_MAGIC.as_ptr() as *const libc::c_void,
            buf_c as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            log_debug(b"Found xz based stream\0" as *const u8 as *const libc::c_char);
            return AG_XZ;
        }
    }
    if buf_len >= 3 as libc::c_int {
        if memcmp(
            LZMA_HEADER_SOMETIMES.as_ptr() as *const libc::c_void,
            buf_c as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            log_debug(b"Found lzma-based stream\0" as *const u8 as *const libc::c_char);
            return AG_XZ;
        }
    }
    return AG_NO_COMPRESSION;
}
