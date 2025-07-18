use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn iconv_open(
        __tocode: *const libc::c_char,
        __fromcode: *const libc::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type iconv_t = *mut libc::c_void;
pub type QRencodeMode = libc::c_int;
pub const QR_MODE_FNC1SECOND: QRencodeMode = 7;
pub const QR_MODE_FNC1FIRST: QRencodeMode = 6;
pub const QR_MODE_ECI: QRencodeMode = 5;
pub const QR_MODE_STRUCTURE: QRencodeMode = 4;
pub const QR_MODE_KANJI: QRencodeMode = 3;
pub const QR_MODE_8: QRencodeMode = 2;
pub const QR_MODE_AN: QRencodeMode = 1;
pub const QR_MODE_NUM: QRencodeMode = 0;
pub const QR_MODE_NUL: QRencodeMode = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DataChunk {
    pub mode: QRencodeMode,
    pub size: libc::c_int,
    pub bits: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub next: *mut _DataChunk,
}
pub type DataChunk = _DataChunk;
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub unsafe extern "C" fn DataChunk_new(mut mode: QRencodeMode) -> *mut DataChunk {
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    chunk = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<DataChunk>() as libc::c_ulong,
    ) as *mut DataChunk;
    if chunk.is_null() {
        return 0 as *mut DataChunk;
    }
    (*chunk).mode = mode;
    return chunk;
}
pub unsafe extern "C" fn DataChunk_free(mut chunk: *mut DataChunk) {
    if !chunk.is_null() {
        if !((*chunk).data).is_null() {
            free((*chunk).data as *mut libc::c_void);
        }
        free(chunk as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn DataChunk_freeList(mut list: *mut DataChunk) {
    let mut next: *mut DataChunk = 0 as *mut DataChunk;
    while !list.is_null() {
        next = (*list).next;
        DataChunk_free(list);
        list = next;
    }
}
unsafe extern "C" fn dumpNum(mut chunk: *mut DataChunk) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*chunk).data);
}
unsafe extern "C" fn dumpAn(mut chunk: *mut DataChunk) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*chunk).data);
}
unsafe extern "C" fn dump8(mut chunk: *mut DataChunk) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_uchar; 16] = [0; 16];
    i = 0 as libc::c_int;
    while i < (*chunk).size {
        buf[count as usize] = *((*chunk).data).offset(i as isize);
        c = *((*chunk).data).offset(i as isize);
        if c as libc::c_int >= ' ' as i32 && c as libc::c_int <= '~' as i32 {
            putchar(c as libc::c_int);
        } else {
            putchar('.' as i32);
        }
        count += 1;
        count;
        if count >= 16 as libc::c_int {
            putchar(' ' as i32);
            j = 0 as libc::c_int;
            while j < 16 as libc::c_int {
                printf(
                    b" %02x\0" as *const u8 as *const libc::c_char,
                    buf[j as usize] as libc::c_int,
                );
                j += 1;
                j;
            }
            count = 0 as libc::c_int;
            putchar('\n' as i32);
        }
        i += 1;
        i;
    }
    if count > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int - count {
            putchar(' ' as i32);
            i += 1;
            i;
        }
        putchar(' ' as i32);
        j = 0 as libc::c_int;
        while j < count {
            printf(
                b" %02x\0" as *const u8 as *const libc::c_char,
                buf[j as usize] as libc::c_int,
            );
            j += 1;
            j;
        }
        count = 0 as libc::c_int;
        putchar('\n' as i32);
    }
}
unsafe extern "C" fn dumpKanji(mut chunk: *mut DataChunk) {
    let mut conv: iconv_t = 0 as *mut libc::c_void;
    let mut inbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inbytes: size_t = 0;
    let mut outbytes: size_t = 0;
    let mut ret: size_t = 0;
    conv = iconv_open(
        b"UTF-8\0" as *const u8 as *const libc::c_char,
        b"SHIFT_JIS\0" as *const u8 as *const libc::c_char,
    );
    inbytes = (*chunk).size as size_t;
    inbuf = (*chunk).data as *mut libc::c_char;
    outbytes = inbytes
        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    outbuf = malloc(
        inbytes
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    outp = outbuf;
    ret = iconv(conv, &mut inbuf, &mut inbytes, &mut outp, &mut outbytes);
    if ret == -(1 as libc::c_int) as size_t {
        perror(0 as *const libc::c_char);
    }
    *outp = '\0' as i32 as libc::c_char;
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, outbuf);
    iconv_close(conv);
    free(outbuf as *mut libc::c_void);
}
unsafe extern "C" fn dumpChunk(mut chunk: *mut DataChunk) {
    match (*chunk).mode as libc::c_int {
        0 => {
            printf(
                b"Numeric: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dumpNum(chunk);
        }
        1 => {
            printf(
                b"AlphaNumeric: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dumpAn(chunk);
        }
        2 => {
            printf(
                b"8-bit data: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dump8(chunk);
        }
        3 => {
            printf(
                b"Kanji: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dumpKanji(chunk);
        }
        _ => {
            printf(
                b"Invalid or reserved: %d bytes\n\0" as *const u8 as *const libc::c_char,
                (*chunk).size,
            );
            dump8(chunk);
        }
    };
}
pub unsafe extern "C" fn DataChunk_dumpChunkList(mut list: *mut DataChunk) {
    while !list.is_null() {
        dumpChunk(list);
        list = (*list).next;
    }
}
pub unsafe extern "C" fn DataChunk_totalSize(mut list: *mut DataChunk) -> libc::c_int {
    let mut size: libc::c_int = 0 as libc::c_int;
    while !list.is_null() {
        size += (*list).size;
        list = (*list).next;
    }
    return size;
}
pub unsafe extern "C" fn DataChunk_concatChunkList(
    mut list: *mut DataChunk,
    mut retsize: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut size: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    size = DataChunk_totalSize(list);
    if size <= 0 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    data = malloc((size + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_uchar;
    idx = 0 as libc::c_int;
    while !list.is_null() {
        memcpy(
            &mut *data.offset(idx as isize) as *mut libc::c_uchar as *mut libc::c_void,
            (*list).data as *const libc::c_void,
            (*list).size as libc::c_ulong,
        );
        idx += (*list).size;
        list = (*list).next;
    }
    *data.offset(size as isize) = '\0' as i32 as libc::c_uchar;
    *retsize = size;
    return data;
}
