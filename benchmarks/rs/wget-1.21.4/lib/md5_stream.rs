use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn md5_init_ctx(ctx: *mut md5_ctx);
    fn md5_process_block(buffer: *const libc::c_void, len: size_t, ctx: *mut md5_ctx);
    fn md5_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut md5_ctx);
    fn md5_finish_ctx(ctx: *mut md5_ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn afalg_stream(
    mut stream: *mut FILE,
    mut alg: *const libc::c_char,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
) -> libc::c_int {
    return -(97 as libc::c_int);
}
pub unsafe extern "C" fn md5_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    match afalg_stream(
        stream,
        b"md5\0" as *const u8 as *const libc::c_char,
        resblock,
        16 as libc::c_int as ssize_t,
    ) {
        0 => return 0 as libc::c_int,
        -5 => return 1 as libc::c_int,
        _ => {}
    }
    let mut buffer: *mut libc::c_char = malloc(
        (32768 as libc::c_int + 72 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if buffer.is_null() {
        return 1 as libc::c_int;
    }
    let mut ctx: md5_ctx = md5_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    md5_init_ctx(&mut ctx);
    let mut sum: size_t = 0;
    's_34: loop {
        let mut n: size_t = 0;
        sum = 0 as libc::c_int as size_t;
        loop {
            if feof_unlocked(stream) != 0 {
                break 's_34;
            }
            n = if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t)
                    .wrapping_mul(
                        (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                    ) <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                {
                    let mut __ptr: *mut libc::c_char = buffer.offset(sum as isize);
                    let mut __stream: *mut FILE = stream;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                        );
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let mut __c: libc::c_int = getc_unlocked(__stream);
                        if __c == -(1 as libc::c_int) {
                            break;
                        }
                        let fresh1 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh1 = __c as libc::c_char;
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                        )
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as libc::c_int as size_t)
                }
            } else if 0 != 0
                && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum)
                        == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fread_unlocked(
                    buffer.offset(sum as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                    stream,
                )
            };
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if sum == 32768 as libc::c_int as libc::c_ulong {
                break;
            }
            if !(n == 0 as libc::c_int as libc::c_ulong) {
                continue;
            }
            if ferror_unlocked(stream) != 0 {
                rpl_free(buffer as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            break 's_34;
        }
        md5_process_block(
            buffer as *const libc::c_void,
            32768 as libc::c_int as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as libc::c_int as libc::c_ulong {
        md5_process_bytes(buffer as *const libc::c_void, sum, &mut ctx);
    }
    md5_finish_ctx(&mut ctx, resblock);
    rpl_free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
