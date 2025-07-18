use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn QRspec_lengthIndicator(mode: QRencodeMode, version: libc::c_int) -> libc::c_int;
    fn QRspec_getEccSpec(version: libc::c_int, level: QRecLevel, spec: *mut libc::c_int);
    fn BitStream_newWithBits(size: size_t, bits: *mut libc::c_uchar) -> *mut BitStream;
    fn BitStream_free(bstream: *mut BitStream);
    fn Mask_makeMask(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
        level: QRecLevel,
    ) -> *mut libc::c_uchar;
    fn MQRspec_getDataLengthBit(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_lengthIndicator(mode: QRencodeMode, version: libc::c_int) -> libc::c_int;
    fn MMask_makeMask(
        version: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
        level: QRecLevel,
    ) -> *mut libc::c_uchar;
    fn printBinary(data: *mut libc::c_uchar, length: libc::c_int);
    fn DataChunk_new(mode: QRencodeMode) -> *mut DataChunk;
    fn DataChunk_dumpChunkList(list: *mut DataChunk);
    fn DataChunk_concatChunkList(
        list: *mut DataChunk,
        retsize: *mut libc::c_int,
    ) -> *mut libc::c_uchar;
    fn DataChunk_freeList(list: *mut DataChunk);
}
pub type size_t = libc::c_ulong;
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
pub type QRecLevel = libc::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRcode {
    pub version: libc::c_int,
    pub width: libc::c_int,
    pub data: *mut libc::c_uchar,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRdata {
    pub data: *mut libc::c_uchar,
    pub size: libc::c_int,
    pub mqr: libc::c_int,
    pub version: libc::c_int,
    pub level: QRecLevel,
    pub chunks: *mut DataChunk,
    pub last: *mut DataChunk,
    pub eccResult: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormatInfo {
    pub version: libc::c_int,
    pub level: QRecLevel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameFiller {
    pub width: libc::c_int,
    pub frame: *mut libc::c_uchar,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub dir: libc::c_int,
    pub bit: libc::c_int,
    pub mqr: libc::c_int,
}
unsafe extern "C" fn bitToInt(
    mut bits: *mut libc::c_uchar,
    mut length: libc::c_int,
) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < length {
        val = val << 1 as libc::c_int;
        val
            |= (*bits.offset(i as isize) as libc::c_int & 1 as libc::c_int)
                as libc::c_uint;
        i += 1;
        i;
    }
    return val;
}
unsafe extern "C" fn decodeLength(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut mode: QRencodeMode,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut lbits: libc::c_int = 0;
    if mqr != 0 {
        lbits = MQRspec_lengthIndicator(mode, version);
    } else {
        lbits = QRspec_lengthIndicator(mode, version);
    }
    if *bits_length < lbits {
        printf(
            b"Bit length is too short: %d\n\0" as *const u8 as *const libc::c_char,
            *bits_length,
        );
        return 0 as libc::c_int;
    }
    length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < lbits {
        length = length << 1 as libc::c_int;
        length += *(*bits).offset(i as isize) as libc::c_int;
        i += 1;
        i;
    }
    *bits_length -= lbits;
    *bits = (*bits).offset(lbits as isize);
    return length;
}
unsafe extern "C" fn decodeNum(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_uint = 0;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    size = decodeLength(bits_length, bits, QR_MODE_NUM, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut DataChunk;
    }
    words = size / 3 as libc::c_int;
    remain = size - words * 3 as libc::c_int;
    sizeInBit = words * 10 as libc::c_int;
    if remain == 2 as libc::c_int {
        sizeInBit += 7 as libc::c_int;
    } else if remain == 1 as libc::c_int {
        sizeInBit += 4 as libc::c_int;
    }
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut DataChunk;
    }
    buf = malloc((size as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < words {
        val = bitToInt(p, 10 as libc::c_int);
        sprintf(q, b"%03d\0" as *const u8 as *const libc::c_char, val);
        p = p.offset(10 as libc::c_int as isize);
        q = q.offset(3 as libc::c_int as isize);
        i += 1;
        i;
    }
    if remain == 2 as libc::c_int {
        val = bitToInt(p, 7 as libc::c_int);
        sprintf(q, b"%02d\0" as *const u8 as *const libc::c_char, val);
    } else if remain == 1 as libc::c_int {
        val = bitToInt(p, 4 as libc::c_int);
        sprintf(q, b"%1d\0" as *const u8 as *const libc::c_char, val);
    }
    *buf.offset(size as isize) = '\0' as i32 as libc::c_char;
    chunk = DataChunk_new(QR_MODE_NUM);
    (*chunk).size = size;
    (*chunk).data = buf as *mut libc::c_uchar;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
static mut decodeAnTable: [libc::c_char; 45] = [
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
];
unsafe extern "C" fn decodeAn(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut cl: libc::c_int = 0;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    size = decodeLength(bits_length, bits, QR_MODE_AN, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut DataChunk;
    }
    words = size / 2 as libc::c_int;
    remain = size - words * 2 as libc::c_int;
    sizeInBit = words * 11 as libc::c_int + remain * 6 as libc::c_int;
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut DataChunk;
    }
    buf = malloc((size as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < words {
        val = bitToInt(p, 11 as libc::c_int);
        ch = val.wrapping_div(45 as libc::c_int as libc::c_uint) as libc::c_int;
        cl = val.wrapping_rem(45 as libc::c_int as libc::c_uint) as libc::c_int;
        sprintf(
            q,
            b"%c%c\0" as *const u8 as *const libc::c_char,
            decodeAnTable[ch as usize] as libc::c_int,
            decodeAnTable[cl as usize] as libc::c_int,
        );
        p = p.offset(11 as libc::c_int as isize);
        q = q.offset(2 as libc::c_int as isize);
        i += 1;
        i;
    }
    if remain == 1 as libc::c_int {
        val = bitToInt(p, 6 as libc::c_int);
        sprintf(
            q,
            b"%c\0" as *const u8 as *const libc::c_char,
            decodeAnTable[val as usize] as libc::c_int,
        );
    }
    chunk = DataChunk_new(QR_MODE_AN);
    (*chunk).size = size;
    (*chunk).data = buf as *mut libc::c_uchar;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
unsafe extern "C" fn decode8(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    size = decodeLength(bits_length, bits, QR_MODE_8, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut DataChunk;
    }
    sizeInBit = size * 8 as libc::c_int;
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut DataChunk;
    }
    buf = malloc(size as size_t) as *mut libc::c_uchar;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < size {
        *q = bitToInt(p, 8 as libc::c_int) as libc::c_uchar;
        p = p.offset(8 as libc::c_int as isize);
        q = q.offset(1 as libc::c_int as isize);
        i += 1;
        i;
    }
    chunk = DataChunk_new(QR_MODE_8);
    (*chunk).size = size;
    (*chunk).data = buf;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
unsafe extern "C" fn decodeKanji(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> *mut DataChunk {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut sizeInBit: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_uint = 0;
    let mut ch: libc::c_uint = 0;
    let mut cl: libc::c_uint = 0;
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    size = decodeLength(bits_length, bits, QR_MODE_KANJI, version, mqr);
    if size < 0 as libc::c_int {
        return 0 as *mut DataChunk;
    }
    sizeInBit = size * 13 as libc::c_int;
    if *bits_length < sizeInBit {
        printf(
            b"Bit length is too short: %d, expected %d.\n\0" as *const u8
                as *const libc::c_char,
            *bits_length,
            sizeInBit,
        );
        return 0 as *mut DataChunk;
    }
    buf = malloc(
        (size as size_t)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    p = *bits;
    q = buf;
    i = 0 as libc::c_int;
    while i < size {
        val = bitToInt(p, 13 as libc::c_int);
        ch = val.wrapping_div(0xc0 as libc::c_int as libc::c_uint);
        cl = val.wrapping_sub(ch.wrapping_mul(0xc0 as libc::c_int as libc::c_uint));
        val = ch.wrapping_mul(256 as libc::c_int as libc::c_uint).wrapping_add(cl);
        if val >= 0x1f00 as libc::c_int as libc::c_uint {
            val = val.wrapping_add(0xc140 as libc::c_int as libc::c_uint);
        } else {
            val = val.wrapping_add(0x8140 as libc::c_int as libc::c_uint);
        }
        sprintf(
            q,
            b"%c%c\0" as *const u8 as *const libc::c_char,
            val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
            val & 0xff as libc::c_int as libc::c_uint,
        );
        p = p.offset(13 as libc::c_int as isize);
        q = q.offset(2 as libc::c_int as isize);
        i += 1;
        i;
    }
    chunk = DataChunk_new(QR_MODE_KANJI);
    (*chunk).size = size * 2 as libc::c_int;
    (*chunk).data = buf as *mut libc::c_uchar;
    *bits_length -= sizeInBit;
    *bits = (*bits).offset(sizeInBit as isize);
    return chunk;
}
unsafe extern "C" fn decodeChunk(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
) -> *mut DataChunk {
    let mut val: libc::c_uint = 0;
    if *bits_length < 4 as libc::c_int {
        return 0 as *mut DataChunk;
    }
    val = bitToInt(*bits, 4 as libc::c_int);
    *bits_length -= 4 as libc::c_int;
    *bits = (*bits).offset(4 as libc::c_int as isize);
    match val {
        0 => return 0 as *mut DataChunk,
        1 => return decodeNum(bits_length, bits, version, 0 as libc::c_int),
        2 => return decodeAn(bits_length, bits, version, 0 as libc::c_int),
        4 => return decode8(bits_length, bits, version, 0 as libc::c_int),
        8 => return decodeKanji(bits_length, bits, version, 0 as libc::c_int),
        _ => {}
    }
    printf(b"Invalid mode in a chunk: %d\n\0" as *const u8 as *const libc::c_char, val);
    return 0 as *mut DataChunk;
}
unsafe extern "C" fn decodeChunkMQR(
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
    mut version: libc::c_int,
) -> *mut DataChunk {
    let mut modebits: libc::c_int = 0;
    let mut termbits: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    modebits = version - 1 as libc::c_int;
    termbits = version * 2 as libc::c_int + 1 as libc::c_int;
    if *bits_length >= termbits {
        val = bitToInt(*bits, termbits);
        if val == 0 as libc::c_int as libc::c_uint {
            *bits = (*bits).offset(termbits as isize);
            *bits_length -= termbits;
            return 0 as *mut DataChunk;
        }
    } else {
        if *bits_length < modebits {
            val = bitToInt(*bits, *bits_length);
        } else {
            val = bitToInt(*bits, modebits);
        }
        if val == 0 as libc::c_int as libc::c_uint {
            return 0 as *mut DataChunk
        } else {
            printf(
                b"Terminating bits include 1-bit.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut DataChunk;
        }
    }
    val = bitToInt(*bits, modebits);
    if version == 4 as libc::c_int && val > 3 as libc::c_int as libc::c_uint {
        printf(b"Invalid mode number %d.\n\0" as *const u8 as *const libc::c_char, val);
    }
    *bits_length -= modebits;
    *bits = (*bits).offset(modebits as isize);
    match val {
        0 => return decodeNum(bits_length, bits, version, 1 as libc::c_int),
        1 => return decodeAn(bits_length, bits, version, 1 as libc::c_int),
        2 => return decode8(bits_length, bits, version, 1 as libc::c_int),
        3 => return decodeKanji(bits_length, bits, version, 1 as libc::c_int),
        _ => {}
    }
    printf(b"Invalid mode in a chunk: %d\n\0" as *const u8 as *const libc::c_char, val);
    return 0 as *mut DataChunk;
}
unsafe extern "C" fn appendChunk(
    mut qrdata: *mut QRdata,
    mut bits_length: *mut libc::c_int,
    mut bits: *mut *mut libc::c_uchar,
) -> libc::c_int {
    let mut chunk: *mut DataChunk = 0 as *mut DataChunk;
    if (*qrdata).mqr != 0 {
        chunk = decodeChunkMQR(bits_length, bits, (*qrdata).version);
    } else {
        chunk = decodeChunk(bits_length, bits, (*qrdata).version);
    }
    if chunk.is_null() {
        return 1 as libc::c_int;
    }
    if ((*qrdata).last).is_null() {
        (*qrdata).chunks = chunk;
    } else {
        (*(*qrdata).last).next = chunk;
    }
    (*qrdata).last = chunk;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRdata_new() -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    qrdata = calloc(
        ::std::mem::size_of::<QRdata>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut QRdata;
    if qrdata.is_null() {
        return 0 as *mut QRdata;
    }
    (*qrdata).eccResult = 0 as libc::c_int;
    return qrdata;
}
pub unsafe extern "C" fn QRdata_newMQR() -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    qrdata = QRdata_new();
    if qrdata.is_null() {
        return 0 as *mut QRdata;
    }
    (*qrdata).mqr = 1 as libc::c_int;
    return qrdata;
}
pub unsafe extern "C" fn QRdata_free(mut qrdata: *mut QRdata) {
    DataChunk_freeList((*qrdata).chunks);
    if !((*qrdata).data).is_null() {
        free((*qrdata).data as *mut libc::c_void);
    }
    free(qrdata as *mut libc::c_void);
}
unsafe extern "C" fn QRdata_decodeBits(
    mut qrdata: *mut QRdata,
    mut length: libc::c_int,
    mut bits: *mut libc::c_uchar,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    while ret == 0 as libc::c_int {
        ret = appendChunk(qrdata, &mut length, &mut bits);
    }
    return length;
}
pub unsafe extern "C" fn QRdata_decodeBitStream(
    mut qrdata: *mut QRdata,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    return QRdata_decodeBits(qrdata, (*bstream).length as libc::c_int, (*bstream).data);
}
pub unsafe extern "C" fn QRdata_dump(mut data: *mut QRdata) {
    DataChunk_dumpChunkList((*data).chunks);
}
pub unsafe extern "C" fn QRcode_decodeVersion(mut code: *mut QRcode) -> libc::c_int {
    let mut v1: libc::c_uint = 0;
    let mut v2: libc::c_uint = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    width = (*code).width;
    if width < 45 as libc::c_int {
        return (width - 21 as libc::c_int) / 4 as libc::c_int + 1 as libc::c_int;
    }
    v1 = 0 as libc::c_int as libc::c_uint;
    p = ((*code).data)
        .offset((width * (width - 9 as libc::c_int)) as isize)
        .offset(5 as libc::c_int as isize);
    x = 0 as libc::c_int;
    while x < 6 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 3 as libc::c_int {
            v1 = v1 << 1 as libc::c_int;
            v1
                |= (*p.offset(-((y * width) as isize)).offset(-(x as isize))
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    v2 = 0 as libc::c_int as libc::c_uint;
    p = ((*code).data)
        .offset((width * 5 as libc::c_int) as isize)
        .offset(width as isize)
        .offset(-(9 as libc::c_int as isize));
    y = 0 as libc::c_int;
    while y < 6 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 3 as libc::c_int {
            v2 = v2 << 1 as libc::c_int;
            v2
                |= (*p.offset(-((y * width) as isize)).offset(-(x as isize))
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    if v1 != v2 {
        printf(
            b"Two verion patterns are different.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return (v1 >> 12 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn QRcode_decodeFormat(
    mut code: *mut QRcode,
    mut level: *mut QRecLevel,
    mut mask: *mut libc::c_int,
) -> libc::c_int {
    let mut v1: libc::c_uint = 0;
    let mut v2: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    width = (*code).width;
    v1 = 0 as libc::c_int as libc::c_uint;
    p = ((*code).data).offset((width * 8 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v1 = v1 << 1 as libc::c_int;
        if i < 6 as libc::c_int {
            v1
                |= (*p.offset(i as isize) as libc::c_int & 1 as libc::c_int)
                    as libc::c_uint;
        } else {
            v1
                |= (*p.offset(i as isize).offset(1 as libc::c_int as isize)
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        }
        i += 1;
        i;
    }
    p = ((*code).data)
        .offset((width * 7 as libc::c_int) as isize)
        .offset(8 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v1 = v1 << 1 as libc::c_int;
        if i < 1 as libc::c_int {
            v1
                |= (*p.offset(-((width * i) as isize)) as libc::c_int & 1 as libc::c_int)
                    as libc::c_uint;
        } else {
            v1
                |= (*p.offset(-((width * (i + 1 as libc::c_int)) as isize))
                    as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        }
        i += 1;
        i;
    }
    v2 = 0 as libc::c_int as libc::c_uint;
    p = ((*code).data)
        .offset((width * (width - 1 as libc::c_int)) as isize)
        .offset(8 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v2 = v2 << 1 as libc::c_int;
        v2
            |= (*p.offset(-((width * i) as isize)) as libc::c_int & 1 as libc::c_int)
                as libc::c_uint;
        i += 1;
        i;
    }
    p = ((*code).data)
        .offset((width * 8 as libc::c_int) as isize)
        .offset(width as isize)
        .offset(-(8 as libc::c_int as isize));
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v2 = v2 << 1 as libc::c_int;
        v2 |= (*p.offset(i as isize) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        i += 1;
        i;
    }
    if v1 != v2 {
        printf(
            b"Two format infos are different.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    v1 = (v1 ^ 0x5412 as libc::c_int as libc::c_uint) >> 10 as libc::c_int;
    *mask = (v1 & 7 as libc::c_int as libc::c_uint) as libc::c_int;
    match v1 >> 3 as libc::c_int & 3 as libc::c_int as libc::c_uint {
        1 => {
            *level = QR_ECLEVEL_L;
        }
        0 => {
            *level = QR_ECLEVEL_M;
        }
        3 => {
            *level = QR_ECLEVEL_Q;
        }
        2 => {
            *level = QR_ECLEVEL_H;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn unmask(
    mut code: *mut QRcode,
    mut level: QRecLevel,
    mut mask: libc::c_int,
) -> *mut libc::c_uchar {
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    unmasked = Mask_makeMask((*code).width, (*code).data, mask, level);
    return unmasked;
}
pub unsafe extern "C" fn QRcode_unmask(mut code: *mut QRcode) -> *mut libc::c_uchar {
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    version = QRcode_decodeVersion(code);
    if version < 1 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    ret = QRcode_decodeFormat(code, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    return unmask(code, level, mask);
}
unsafe extern "C" fn FrameFiller_new(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mqr: libc::c_int,
) -> *mut FrameFiller {
    let mut filler: *mut FrameFiller = 0 as *mut FrameFiller;
    filler = malloc(::std::mem::size_of::<FrameFiller>() as libc::c_ulong)
        as *mut FrameFiller;
    if filler.is_null() {
        return 0 as *mut FrameFiller;
    }
    (*filler).width = width;
    (*filler).frame = frame;
    (*filler).x = width - 1 as libc::c_int;
    (*filler).y = width - 1 as libc::c_int;
    (*filler).dir = -(1 as libc::c_int);
    (*filler).bit = -(1 as libc::c_int);
    (*filler).mqr = mqr;
    return filler;
}
unsafe extern "C" fn FrameFiller_next(
    mut filler: *mut FrameFiller,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    if (*filler).bit == -(1 as libc::c_int) {
        (*filler).bit = 0 as libc::c_int;
        return ((*filler).frame)
            .offset(((*filler).y * (*filler).width) as isize)
            .offset((*filler).x as isize);
    }
    x = (*filler).x;
    y = (*filler).y;
    p = (*filler).frame;
    w = (*filler).width;
    if (*filler).bit == 0 as libc::c_int {
        x -= 1;
        x;
        (*filler).bit += 1;
        (*filler).bit;
    } else {
        x += 1;
        x;
        y += (*filler).dir;
        (*filler).bit -= 1;
        (*filler).bit;
    }
    if (*filler).dir < 0 as libc::c_int {
        if y < 0 as libc::c_int {
            y = 0 as libc::c_int;
            x -= 2 as libc::c_int;
            (*filler).dir = 1 as libc::c_int;
            if (*filler).mqr == 0 && x == 6 as libc::c_int {
                x -= 1;
                x;
                y = 9 as libc::c_int;
            }
        }
    } else if y == w {
        y = w - 1 as libc::c_int;
        x -= 2 as libc::c_int;
        (*filler).dir = -(1 as libc::c_int);
        if (*filler).mqr == 0 && x == 6 as libc::c_int {
            x -= 1;
            x;
            y -= 8 as libc::c_int;
        }
    }
    if x < 0 as libc::c_int || y < 0 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    (*filler).x = x;
    (*filler).y = y;
    if *p.offset((y * w + x) as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        return FrameFiller_next(filler);
    }
    return &mut *p.offset((y * w + x) as isize) as *mut libc::c_uchar;
}
unsafe extern "C" fn extractBits(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut spec: *mut libc::c_int,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut bits: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut filler: *mut FrameFiller = 0 as *mut FrameFiller;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut d1: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut blocks: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    blocks = *spec.offset(0 as libc::c_int as isize)
        + *spec.offset(3 as libc::c_int as isize);
    words = *spec.offset(0 as libc::c_int as isize)
        * *spec.offset(1 as libc::c_int as isize)
        + *spec.offset(3 as libc::c_int as isize)
            * *spec.offset(4 as libc::c_int as isize);
    d1 = *spec.offset(1 as libc::c_int as isize);
    b1 = *spec.offset(0 as libc::c_int as isize);
    bits = malloc((words as size_t).wrapping_mul(8 as libc::c_int as libc::c_ulong))
        as *mut libc::c_uchar;
    col = 0 as libc::c_int;
    row = col;
    filler = FrameFiller_new(width, frame, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < words {
        col = i / blocks;
        row = i % blocks + (if col >= d1 { b1 } else { 0 as libc::c_int });
        idx = d1 * row + col + (if row > b1 { row - b1 } else { 0 as libc::c_int });
        q = bits.offset((idx * 8 as libc::c_int) as isize);
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            p = FrameFiller_next(filler);
            *q
                .offset(
                    j as isize,
                ) = (*p as libc::c_int & 1 as libc::c_int) as libc::c_uchar;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    free(filler as *mut libc::c_void);
    bstream = BitStream_newWithBits(
        (words as size_t).wrapping_mul(8 as libc::c_int as libc::c_ulong),
        bits,
    );
    free(bits as *mut libc::c_void);
    return bstream;
}
pub unsafe extern "C" fn QRcode_extractBits(
    mut code: *mut QRcode,
    mut dataLength: *mut libc::c_int,
    mut eccLength: *mut libc::c_int,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    version = QRcode_decodeVersion(code);
    if version < 1 as libc::c_int {
        return 0 as *mut BitStream;
    }
    ret = QRcode_decodeFormat(code, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut BitStream;
    }
    QRspec_getEccSpec(version, level, spec.as_mut_ptr());
    unmasked = unmask(code, level, mask);
    if unmasked.is_null() {
        return 0 as *mut BitStream;
    }
    bstream = extractBits((*code).width, unmasked, spec.as_mut_ptr());
    free(unmasked as *mut libc::c_void);
    *dataLength = (spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
        + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize])
        * 8 as libc::c_int;
    *eccLength = (spec[0 as libc::c_int as usize] + spec[3 as libc::c_int as usize])
        * spec[2 as libc::c_int as usize] * 8 as libc::c_int;
    return bstream;
}
unsafe extern "C" fn checkRemainderWords(
    mut length: libc::c_int,
    mut bits: *mut libc::c_uchar,
    mut remainder: libc::c_int,
) -> libc::c_int {
    let mut rbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    words = remainder / 8 as libc::c_int;
    rbits = remainder - words * 8 as libc::c_int;
    bits = bits.offset((length - remainder) as isize);
    i = 0 as libc::c_int;
    while i < rbits {
        if *bits.offset(i as isize) as libc::c_int & 1 as libc::c_int != 0 as libc::c_int
        {
            printf(
                b"Terminating code includes 1-bit.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    p = bits.offset(rbits as isize);
    i = 0 as libc::c_int;
    while i < words {
        v = bitToInt(p, 8 as libc::c_int) as libc::c_uchar;
        if v as libc::c_int
            != (if i & 1 as libc::c_int != 0 {
                0x11 as libc::c_int
            } else {
                0xec as libc::c_int
            })
        {
            printf(
                b"Remainder codewords wrong.\n\0" as *const u8 as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        p = p.offset(8 as libc::c_int as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRcode_decodeBits(mut code: *mut QRcode) -> *mut QRdata {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    version = QRcode_decodeVersion(code);
    if version < 1 as libc::c_int {
        return 0 as *mut QRdata;
    }
    ret = QRcode_decodeFormat(code, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut QRdata;
    }
    QRspec_getEccSpec(version, level, spec.as_mut_ptr());
    length = (spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
        + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize])
        * 8 as libc::c_int;
    unmasked = unmask(code, level, mask);
    if unmasked.is_null() {
        return 0 as *mut QRdata;
    }
    bstream = extractBits((*code).width, unmasked, spec.as_mut_ptr());
    free(unmasked as *mut libc::c_void);
    qrdata = QRdata_new();
    (*qrdata).version = version;
    (*qrdata).level = level;
    ret = QRdata_decodeBitStream(qrdata, bstream);
    if ret > 0 as libc::c_int {
        checkRemainderWords(length, (*bstream).data, ret);
    }
    BitStream_free(bstream);
    return qrdata;
}
pub unsafe extern "C" fn QRdata_concatChunks(mut qrdata: *mut QRdata) {
    (*qrdata).data = DataChunk_concatChunkList((*qrdata).chunks, &mut (*qrdata).size);
}
pub unsafe extern "C" fn QRcode_decode(mut code: *mut QRcode) -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    qrdata = QRcode_decodeBits(code);
    QRdata_concatChunks(qrdata);
    return qrdata;
}
pub static mut MQRformat: [FormatInfo; 8] = [
    {
        let mut init = FormatInfo {
            version: 1 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 2 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 2 as libc::c_int,
            level: QR_ECLEVEL_M,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 3 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 3 as libc::c_int,
            level: QR_ECLEVEL_M,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 4 as libc::c_int,
            level: QR_ECLEVEL_L,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 4 as libc::c_int,
            level: QR_ECLEVEL_M,
        };
        init
    },
    {
        let mut init = FormatInfo {
            version: 4 as libc::c_int,
            level: QR_ECLEVEL_Q,
        };
        init
    },
];
pub unsafe extern "C" fn QRcode_decodeFormatMQR(
    mut code: *mut QRcode,
    mut version: *mut libc::c_int,
    mut level: *mut QRecLevel,
    mut mask: *mut libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_uint = 0;
    let mut t: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    width = (*code).width;
    v = 0 as libc::c_int as libc::c_uint;
    p = ((*code).data)
        .offset((width * 8 as libc::c_int) as isize)
        .offset(1 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v = v << 1 as libc::c_int;
        v |= (*p.offset(i as isize) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
        i += 1;
        i;
    }
    p = ((*code).data)
        .offset((width * 7 as libc::c_int) as isize)
        .offset(8 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v = v << 1 as libc::c_int;
        v
            |= (*p.offset(-((width * i) as isize)) as libc::c_int & 1 as libc::c_int)
                as libc::c_uint;
        i += 1;
        i;
    }
    v ^= 0x4445 as libc::c_int as libc::c_uint;
    *mask = (v >> 10 as libc::c_int & 3 as libc::c_int as libc::c_uint) as libc::c_int;
    t = v >> 12 as libc::c_int & 7 as libc::c_int as libc::c_uint;
    *version = MQRformat[t as usize].version;
    *level = MQRformat[t as usize].level;
    if *version * 2 as libc::c_int + 9 as libc::c_int != width {
        printf(
            b"Decoded version number does not match to the size.\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn unmaskMQR(
    mut code: *mut QRcode,
    mut level: QRecLevel,
    mut mask: libc::c_int,
) -> *mut libc::c_uchar {
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    unmasked = MMask_makeMask((*code).version, (*code).data, mask, level);
    return unmasked;
}
pub unsafe extern "C" fn QRcode_unmaskMQR(mut code: *mut QRcode) -> *mut libc::c_uchar {
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    ret = QRcode_decodeFormatMQR(code, &mut version, &mut level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    return unmaskMQR(code, level, mask);
}
unsafe extern "C" fn extractBitsMQR(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut bits: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut filler: *mut FrameFiller = 0 as *mut FrameFiller;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    size = MQRspec_getDataLengthBit(version, level)
        + MQRspec_getECCLength(version, level) * 8 as libc::c_int;
    bits = malloc(size as size_t) as *mut libc::c_uchar;
    filler = FrameFiller_new(width, frame, 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < size {
        *bits
            .offset(
                i as isize,
            ) = (*FrameFiller_next(filler) as libc::c_int & 1 as libc::c_int)
            as libc::c_uchar;
        i += 1;
        i;
    }
    free(filler as *mut libc::c_void);
    bstream = BitStream_newWithBits(size as size_t, bits);
    free(bits as *mut libc::c_void);
    return bstream;
}
pub unsafe extern "C" fn QRcode_extractBitsMQR(
    mut code: *mut QRcode,
    mut dataLength: *mut libc::c_int,
    mut eccLength: *mut libc::c_int,
    mut version: *mut libc::c_int,
    mut level: *mut QRecLevel,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut unmasked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    ret = QRcode_decodeFormatMQR(code, version, level, &mut mask);
    if ret < 0 as libc::c_int {
        return 0 as *mut BitStream;
    }
    unmasked = unmaskMQR(code, *level, mask);
    if unmasked.is_null() {
        return 0 as *mut BitStream;
    }
    *dataLength = MQRspec_getDataLengthBit(*version, *level);
    *eccLength = MQRspec_getECCLength(*version, *level) * 8 as libc::c_int;
    bstream = extractBitsMQR((*code).width, unmasked, *version, *level);
    free(unmasked as *mut libc::c_void);
    return bstream;
}
unsafe extern "C" fn checkRemainderWordsMQR(
    mut length: libc::c_int,
    mut bits: *mut libc::c_uchar,
    mut remainder: libc::c_int,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut rbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut paddings: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut decoded: libc::c_int = 0;
    decoded = length - remainder;
    bits = bits.offset(decoded as isize);
    words = (decoded + 7 as libc::c_int) / 8 as libc::c_int;
    rbits = words * 8 as libc::c_int - decoded;
    i = 0 as libc::c_int;
    while i < rbits {
        if *bits.offset(i as isize) as libc::c_int & 1 as libc::c_int != 0 as libc::c_int
        {
            printf(
                b"Terminating code includes 1-bit.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    paddings = (length - words * 8 as libc::c_int) / 8 as libc::c_int;
    p = bits.offset(rbits as isize);
    i = 0 as libc::c_int;
    while i < paddings {
        v = bitToInt(p, 8 as libc::c_int) as libc::c_uchar;
        if v as libc::c_int
            != (if i & 1 as libc::c_int != 0 {
                0x11 as libc::c_int
            } else {
                0xec as libc::c_int
            })
        {
            printf(
                b"Remainder codewords wrong.\n\0" as *const u8 as *const libc::c_char,
            );
            printBinary(bits, remainder);
            return -(1 as libc::c_int);
        }
        p = p.offset(8 as libc::c_int as isize);
        i += 1;
        i;
    }
    rbits = length - (paddings + words) * 8 as libc::c_int;
    if rbits > 0 as libc::c_int {
        if (version == 1 as libc::c_int || version == 3 as libc::c_int)
            && rbits == 4 as libc::c_int
        {
            v = bitToInt(p, 4 as libc::c_int) as libc::c_uchar;
            if v as libc::c_int != 0 as libc::c_int {
                printf(
                    b"Last padding bits include 1-bit.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        } else {
            printf(
                b"The length of the last padding bits is %d, not %d.\n\0" as *const u8
                    as *const libc::c_char,
                rbits,
                if version == 1 as libc::c_int || version == 3 as libc::c_int {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                },
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRcode_decodeBitsMQR(mut code: *mut QRcode) -> *mut QRdata {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut ret: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut dataLength: libc::c_int = 0;
    let mut eccLength: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    bstream = QRcode_extractBitsMQR(
        code,
        &mut dataLength,
        &mut eccLength,
        &mut version,
        &mut level,
    );
    if bstream.is_null() {
        return 0 as *mut QRdata;
    }
    qrdata = QRdata_newMQR();
    (*qrdata).version = version;
    (*qrdata).level = level;
    ret = QRdata_decodeBits(qrdata, dataLength, (*bstream).data);
    if ret > 0 as libc::c_int {
        ret = checkRemainderWordsMQR(dataLength, (*bstream).data, ret, version);
        if ret < 0 as libc::c_int {
            QRdata_free(qrdata);
            qrdata = 0 as *mut QRdata;
        }
    }
    BitStream_free(bstream);
    return qrdata;
}
pub unsafe extern "C" fn QRcode_decodeMQR(mut code: *mut QRcode) -> *mut QRdata {
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    qrdata = QRcode_decodeBitsMQR(code);
    if !qrdata.is_null() {
        QRdata_concatChunks(qrdata);
    }
    return qrdata;
}
