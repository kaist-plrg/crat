use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn rand() -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn printQRcode(code: *mut QRcode);
    fn printFrame(width: libc::c_int, frame: *mut libc::c_uchar);
    fn testReport(tests: libc::c_int);
    fn testFinish();
    fn testEnd(result: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testInit(tests: libc::c_int);
    static mut modeStr: [*const libc::c_char; 5];
    static mut assertionNum: libc::c_int;
    static mut assertionFailed: libc::c_int;
    fn QRcode_encodeMask(input: *mut QRinput, mask: libc::c_int) -> *mut QRcode;
    fn FrameFiller_testMQR(version: libc::c_int) -> *mut libc::c_uchar;
    fn FrameFiller_test(version: libc::c_int) -> *mut libc::c_uchar;
    fn MQRraw_free(raw: *mut MQRRawCode);
    fn MQRraw_new(input: *mut QRinput) -> *mut MQRRawCode;
    fn QRraw_free(raw: *mut QRRawCode);
    fn QRraw_getCode(raw: *mut QRRawCode) -> libc::c_uchar;
    fn QRraw_new(input: *mut QRinput) -> *mut QRRawCode;
    fn QRinput_new() -> *mut QRinput;
    fn QRinput_new2(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_newMQR(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: libc::c_int,
        data: *const libc::c_uchar,
    ) -> libc::c_int;
    fn QRinput_setVersion(input: *mut QRinput, version: libc::c_int) -> libc::c_int;
    fn QRinput_setErrorCorrectionLevel(
        input: *mut QRinput,
        level: QRecLevel,
    ) -> libc::c_int;
    fn QRinput_free(input: *mut QRinput);
    fn QRcode_encodeString8bitStructured(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
    ) -> *mut QRcode_List;
    fn QRcode_encodeInput(input: *mut QRinput) -> *mut QRcode;
    fn QRcode_encodeString(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> *mut QRcode;
    fn QRcode_encodeString8bit(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
    ) -> *mut QRcode;
    fn QRcode_encodeStringMQR(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> *mut QRcode;
    fn QRcode_encodeString8bitMQR(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
    ) -> *mut QRcode;
    fn QRcode_encodeData(
        size: libc::c_int,
        data: *const libc::c_uchar,
        version: libc::c_int,
        level: QRecLevel,
    ) -> *mut QRcode;
    fn QRcode_free(qrcode: *mut QRcode);
    fn QRcode_encodeStringStructured(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> *mut QRcode_List;
    fn QRcode_List_size(qrlist: *mut QRcode_List) -> libc::c_int;
    fn QRcode_List_free(qrlist: *mut QRcode_List);
    fn QRcode_APIVersion(
        major_version: *mut libc::c_int,
        minor_version: *mut libc::c_int,
        micro_version: *mut libc::c_int,
    );
    fn QRcode_APIVersionString() -> *mut libc::c_char;
    fn QRspec_getDataLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn QRspec_getRemainder(version: libc::c_int) -> libc::c_int;
    fn QRspec_getFormatInfo(mask: libc::c_int, level: QRecLevel) -> libc::c_uint;
    fn QRspec_newFrame(version: libc::c_int) -> *mut libc::c_uchar;
    fn MQRspec_getDataLengthBit(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn MQRspec_newFrame(version: libc::c_int) -> *mut libc::c_uchar;
    fn Mask_writeFormatInformation(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
        level: QRecLevel,
    ) -> libc::c_int;
    fn Split_splitStringToQRinput(
        string: *const libc::c_char,
        input: *mut QRinput,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> libc::c_int;
    static mut MQRformat: [FormatInfo; 0];
    fn QRdata_free(data: *mut QRdata);
    fn QRcode_decodeFormat(
        code: *mut QRcode,
        level: *mut QRecLevel,
        mask: *mut libc::c_int,
    ) -> libc::c_int;
    fn QRcode_decode(code: *mut QRcode) -> *mut QRdata;
    fn QRcode_decodeFormatMQR(
        code: *mut QRcode,
        vesion: *mut libc::c_int,
        level: *mut QRecLevel,
        mask: *mut libc::c_int,
    ) -> libc::c_int;
    fn QRcode_decodeMQR(code: *mut QRcode) -> *mut QRdata;
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
pub struct _QRinput {
    pub version: libc::c_int,
    pub level: QRecLevel,
    pub head: *mut QRinput_List,
    pub tail: *mut QRinput_List,
    pub mqr: libc::c_int,
    pub fnc1: libc::c_int,
    pub appid: libc::c_uchar,
}
pub type QRinput_List = _QRinput_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_List {
    pub mode: QRencodeMode,
    pub size: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub bstream: *mut BitStream,
    pub next: *mut QRinput_List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut libc::c_uchar,
}
pub type QRinput = _QRinput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRcode {
    pub version: libc::c_int,
    pub width: libc::c_int,
    pub data: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRcode_List {
    pub code: *mut QRcode,
    pub next: *mut _QRcode_List,
}
pub type QRcode_List = _QRcode_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RSblock {
    pub dataLength: libc::c_int,
    pub eccLength: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub ecc: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRRawCode {
    pub version: libc::c_int,
    pub dataLength: libc::c_int,
    pub eccLength: libc::c_int,
    pub datacode: *mut libc::c_uchar,
    pub ecccode: *mut libc::c_uchar,
    pub b1: libc::c_int,
    pub blocks: libc::c_int,
    pub rsblock: *mut RSblock,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MQRRawCode {
    pub version: libc::c_int,
    pub dataLength: libc::c_int,
    pub eccLength: libc::c_int,
    pub datacode: *mut libc::c_uchar,
    pub ecccode: *mut libc::c_uchar,
    pub rsblock: *mut RSblock,
    pub oddbits: libc::c_int,
    pub count: libc::c_int,
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
pub struct TestString {
    pub str_0: *mut libc::c_char,
    pub version: libc::c_int,
    pub level: QRecLevel,
    pub hint: QRencodeMode,
    pub casesensitive: libc::c_int,
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
unsafe extern "C" fn test_qrraw_new() {
    let mut i: libc::c_int = 0;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"01234567\0");
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_qrraw_new\0"))
            .as_ptr(),
        b"Test QRraw_new()\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_setVersion(stream, 10 as libc::c_int);
    QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_Q);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    raw = QRraw_new(stream);
    assertionNum += 1;
    assertionNum;
    if raw.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed QRraw_new().\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).count == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRraw.count = %d != 0\n\0" as *const u8 as *const libc::c_char,
            (*raw).count,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).version == 10 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRraw.version was not as expected. (%d)\n\0" as *const u8
                as *const libc::c_char,
            (*raw).version,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).dataLength
        == 19 as libc::c_int * 6 as libc::c_int + 20 as libc::c_int * 2 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRraw.dataLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).eccLength == 24 as libc::c_int * 8 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRraw.eccLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).b1 == 6 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"QRraw.b1 was not as expected.\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).blocks == 8 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRraw.blocks was not as expected.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < (*raw).b1 {
        assertionNum += 1;
        assertionNum;
        if !((*((*raw).rsblock).offset(i as isize)).dataLength == 19 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"QRraw.rsblock[%d].dataLength was not as expected. (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                (*((*raw).rsblock).offset(i as isize)).dataLength,
            );
        }
        i += 1;
        i;
    }
    i = (*raw).b1;
    while i < (*raw).blocks {
        assertionNum += 1;
        assertionNum;
        if !((*((*raw).rsblock).offset(i as isize)).dataLength == 20 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"QRraw.rsblock[%d].dataLength was not as expected. (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                (*((*raw).rsblock).offset(i as isize)).dataLength,
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*raw).blocks {
        assertionNum += 1;
        assertionNum;
        if !((*((*raw).rsblock).offset(i as isize)).eccLength == 24 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"QRraw.rsblock[%d].eccLength was not as expected. (%d)\n\0" as *const u8
                    as *const libc::c_char,
                i,
                (*((*raw).rsblock).offset(i as isize)).eccLength,
            );
        }
        i += 1;
        i;
    }
    QRinput_free(stream);
    QRraw_free(raw);
    testFinish();
}
unsafe extern "C" fn test_iterate() {
    let mut i: libc::c_int = 0;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"01234567\0");
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_iterate\0"))
            .as_ptr(),
        b"Test getCode (1-L)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_setVersion(stream, 1 as libc::c_int);
    QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_L);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    raw = QRraw_new(stream);
    data = (*raw).datacode;
    i = 0 as libc::c_int;
    while i < (*raw).dataLength {
        if *data.offset(i as isize) as libc::c_int != QRraw_getCode(raw) as libc::c_int {
            err += 1;
            err;
        }
        i += 1;
        i;
    }
    QRinput_free(stream);
    QRraw_free(raw);
    testEnd(err);
}
unsafe extern "C" fn test_iterate2() {
    let mut i: libc::c_int = 0;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"01234567\0");
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut correct: [libc::c_uchar; 134] = [
        0x10 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x20 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xc as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x56 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x61 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x80 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0x5c as libc::c_int as libc::c_uchar,
        0xde as libc::c_int as libc::c_uchar,
        0x68 as libc::c_int as libc::c_uchar,
        0x68 as libc::c_int as libc::c_uchar,
        0x4d as libc::c_int as libc::c_uchar,
        0xb3 as libc::c_int as libc::c_uchar,
        0xdb as libc::c_int as libc::c_uchar,
        0xdb as libc::c_int as libc::c_uchar,
        0xd5 as libc::c_int as libc::c_uchar,
        0x14 as libc::c_int as libc::c_uchar,
        0xe1 as libc::c_int as libc::c_uchar,
        0xe1 as libc::c_int as libc::c_uchar,
        0x5b as libc::c_int as libc::c_uchar,
        0x2a as libc::c_int as libc::c_uchar,
        0x1f as libc::c_int as libc::c_uchar,
        0x1f as libc::c_int as libc::c_uchar,
        0x49 as libc::c_int as libc::c_uchar,
        0xc4 as libc::c_int as libc::c_uchar,
        0x78 as libc::c_int as libc::c_uchar,
        0x78 as libc::c_int as libc::c_uchar,
        0xf7 as libc::c_int as libc::c_uchar,
        0xe0 as libc::c_int as libc::c_uchar,
        0x5b as libc::c_int as libc::c_uchar,
        0x5b as libc::c_int as libc::c_uchar,
        0xc3 as libc::c_int as libc::c_uchar,
        0xa7 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0x5d as libc::c_int as libc::c_uchar,
        0x9a as libc::c_int as libc::c_uchar,
        0xea as libc::c_int as libc::c_uchar,
        0xea as libc::c_int as libc::c_uchar,
        0x48 as libc::c_int as libc::c_uchar,
        0xad as libc::c_int as libc::c_uchar,
        0x9d as libc::c_int as libc::c_uchar,
        0x9d as libc::c_int as libc::c_uchar,
        0x58 as libc::c_int as libc::c_uchar,
        0xb3 as libc::c_int as libc::c_uchar,
        0x3f as libc::c_int as libc::c_uchar,
        0x3f as libc::c_int as libc::c_uchar,
        0x10 as libc::c_int as libc::c_uchar,
        0xdb as libc::c_int as libc::c_uchar,
        0xbf as libc::c_int as libc::c_uchar,
        0xbf as libc::c_int as libc::c_uchar,
        0xeb as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x5 as libc::c_int as libc::c_uchar,
        0x5 as libc::c_int as libc::c_uchar,
        0x98 as libc::c_int as libc::c_uchar,
        0x35 as libc::c_int as libc::c_uchar,
        0x83 as libc::c_int as libc::c_uchar,
        0x83 as libc::c_int as libc::c_uchar,
        0xa9 as libc::c_int as libc::c_uchar,
        0x95 as libc::c_int as libc::c_uchar,
        0xa6 as libc::c_int as libc::c_uchar,
        0xa6 as libc::c_int as libc::c_uchar,
        0xea as libc::c_int as libc::c_uchar,
        0x7b as libc::c_int as libc::c_uchar,
        0x8d as libc::c_int as libc::c_uchar,
        0x8d as libc::c_int as libc::c_uchar,
        0x4 as libc::c_int as libc::c_uchar,
        0x3c as libc::c_int as libc::c_uchar,
        0x8 as libc::c_int as libc::c_uchar,
        0x8 as libc::c_int as libc::c_uchar,
        0x64 as libc::c_int as libc::c_uchar,
        0xce as libc::c_int as libc::c_uchar,
        0x3e as libc::c_int as libc::c_uchar,
        0x3e as libc::c_int as libc::c_uchar,
        0x4d as libc::c_int as libc::c_uchar,
        0x9b as libc::c_int as libc::c_uchar,
        0x30 as libc::c_int as libc::c_uchar,
        0x30 as libc::c_int as libc::c_uchar,
        0x4e as libc::c_int as libc::c_uchar,
        0x65 as libc::c_int as libc::c_uchar,
        0xd6 as libc::c_int as libc::c_uchar,
        0xd6 as libc::c_int as libc::c_uchar,
        0xe4 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x2c as libc::c_int as libc::c_uchar,
        0x2c as libc::c_int as libc::c_uchar,
        0x46 as libc::c_int as libc::c_uchar,
        0x1d as libc::c_int as libc::c_uchar,
        0x2e as libc::c_int as libc::c_uchar,
        0x2e as libc::c_int as libc::c_uchar,
        0x29 as libc::c_int as libc::c_uchar,
        0x16 as libc::c_int as libc::c_uchar,
        0x27 as libc::c_int as libc::c_uchar,
        0x27 as libc::c_int as libc::c_uchar,
    ];
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_iterate2\0"))
            .as_ptr(),
        b"Test getCode (5-H)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_setVersion(stream, 5 as libc::c_int);
    QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_H);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    raw = QRraw_new(stream);
    i = 0 as libc::c_int;
    while i < (*raw).dataLength {
        if correct[i as usize] as libc::c_int != QRraw_getCode(raw) as libc::c_int {
            err += 1;
            err;
        }
        i += 1;
        i;
    }
    QRinput_free(stream);
    QRraw_free(raw);
    testEnd(err);
}
unsafe extern "C" fn print_filler() {
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 7 as libc::c_int;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    puts(b"\nPrinting debug info of FrameFiller.\0" as *const u8 as *const libc::c_char);
    width = QRspec_getWidth(version);
    frame = FrameFiller_test(version);
    if frame.is_null() {
        abort();
    }
    printFrame(width, frame);
    free(frame as *mut libc::c_void);
}
unsafe extern "C" fn test_filler() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_filler\0"))
            .as_ptr(),
        b"Frame filler test\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        length = QRspec_getDataLength(i, QR_ECLEVEL_L) * 8 as libc::c_int
            + QRspec_getECCLength(i, QR_ECLEVEL_L) * 8 as libc::c_int
            + QRspec_getRemainder(i);
        frame = FrameFiller_test(i);
        if frame.is_null() {
            assertionNum += 1;
            assertionNum;
            if frame.is_null() {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Something wrong in version %d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
        } else {
            w = QRspec_getWidth(i);
            e = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < w * w {
                if *frame.offset(j as isize) as libc::c_int == 0 as libc::c_int {
                    e += 1;
                    e;
                }
                j += 1;
                j;
            }
            assertionNum += 1;
            assertionNum;
            if !(e == 0 as libc::c_int) {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Not filled bit is found. (%d,%d)\n\0" as *const u8
                        as *const libc::c_char,
                    j % w,
                    j / w,
                );
            }
            e = w
                * (w - 9 as libc::c_int
                    - (if i > 6 as libc::c_int {
                        3 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }));
            assertionNum += 1;
            assertionNum;
            if !(*frame.offset(e as isize) as libc::c_int
                == (length - 1 as libc::c_int & 127 as libc::c_int) as libc::c_uchar
                    as libc::c_int | 0x80 as libc::c_int)
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Number of cell does not match.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            free(frame as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn print_fillerMQR() {
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 3 as libc::c_int;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    puts(
        b"\nPrinting debug info of FrameFiller for Micro QR.\0" as *const u8
            as *const libc::c_char,
    );
    version = 1 as libc::c_int;
    while version <= 4 as libc::c_int {
        width = MQRspec_getWidth(version);
        frame = FrameFiller_testMQR(version);
        if frame.is_null() {
            abort();
        }
        printFrame(width, frame);
        version += 1;
        version;
    }
}
unsafe extern "C" fn test_fillerMQR() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_fillerMQR\0"))
            .as_ptr(),
        b"Micro QR Code Frame filler test\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= 4 as libc::c_int {
        length = MQRspec_getDataLengthBit(i, QR_ECLEVEL_L)
            + MQRspec_getECCLength(i, QR_ECLEVEL_L) * 8 as libc::c_int;
        frame = FrameFiller_testMQR(i);
        if frame.is_null() {
            assertionNum += 1;
            assertionNum;
            if frame.is_null() {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Something wrong in version %d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
        } else {
            w = MQRspec_getWidth(i);
            e = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < w * w {
                if *frame.offset(j as isize) as libc::c_int == 0 as libc::c_int {
                    e += 1;
                    e;
                }
                j += 1;
                j;
            }
            assertionNum += 1;
            assertionNum;
            if !(e == 0 as libc::c_int) {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Not filled bit is found. (%d,%d)\n\0" as *const u8
                        as *const libc::c_char,
                    j % w,
                    j / w,
                );
            }
            if i & 1 as libc::c_int != 0 {
                e = w * 9 as libc::c_int + 1 as libc::c_int;
            } else {
                e = w * (w - 1 as libc::c_int) + 1 as libc::c_int;
            }
            assertionNum += 1;
            assertionNum;
            if !(*frame.offset(e as isize) as libc::c_int
                == (length - 1 as libc::c_int & 127 as libc::c_int) as libc::c_uchar
                    as libc::c_int | 0x80 as libc::c_int)
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Number of cell does not match in version %d.\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            free(frame as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_format() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut format: libc::c_uint = 0;
    let mut width: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut decode: libc::c_uint = 0;
    let mut blacks: libc::c_int = 0;
    let mut b1: libc::c_int = 0 as libc::c_int;
    let mut b2: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_format\0"))
            .as_ptr(),
        b"Test format information(level L,mask 0)\0" as *const u8 as *const libc::c_char,
    );
    width = QRspec_getWidth(1 as libc::c_int);
    frame = QRspec_newFrame(1 as libc::c_int);
    if !frame.is_null() {
        format = QRspec_getFormatInfo(1 as libc::c_int, QR_ECLEVEL_L);
        blacks = Mask_writeFormatInformation(
            width,
            frame,
            1 as libc::c_int,
            QR_ECLEVEL_L,
        );
        decode = 0 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int {
            if ((1 as libc::c_int) << i) as libc::c_uint & format != 0 {
                b2 += 2 as libc::c_int;
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            decode = decode << 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width * 8 as libc::c_int + i
                            + (i > 5 as libc::c_int) as libc::c_int) as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_int as libc::c_uint != 0 {
                b1 += 1;
                b1;
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            decode = decode << 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width
                            * (6 as libc::c_int - i
                                + (i < 1 as libc::c_int) as libc::c_int) + 8 as libc::c_int)
                            as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_int as libc::c_uint != 0 {
                b1 += 1;
                b1;
            }
            i += 1;
            i;
        }
        if decode != format {
            printf(
                b"Upper-left format information is invalid.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"%08x, %08x\n\0" as *const u8 as *const libc::c_char,
                format,
                decode,
            );
            testEnd(1 as libc::c_int);
            return;
        }
        decode = 0 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            decode = decode << 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width * (width - 1 as libc::c_int - i) + 8 as libc::c_int)
                            as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_int as libc::c_uint != 0 {
                b1 += 1;
                b1;
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            decode = decode << 1 as libc::c_int;
            decode
                |= (*frame
                    .offset(
                        (width * 8 as libc::c_int + width - 8 as libc::c_int + i)
                            as isize,
                    ) as libc::c_int & 1 as libc::c_int) as libc::c_uint;
            if decode & 1 as libc::c_int as libc::c_uint != 0 {
                b1 += 1;
                b1;
            }
            i += 1;
            i;
        }
        if decode != format {
            printf(
                b"Bottom and right format information is invalid.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"%08x, %08x\n\0" as *const u8 as *const libc::c_char,
                format,
                decode,
            );
            testEnd(1 as libc::c_int);
            return;
        }
        if b2 != blacks || b1 != b2 {
            printf(
                b"Number of dark modules is incorrect.\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"Return value: %d, dark modules in frame: %d, should be: %d\n\0"
                    as *const u8 as *const libc::c_char,
                blacks,
                b1,
                b2,
            );
            testEnd(1 as libc::c_int);
            return;
        }
        free(frame as *mut libc::c_void);
    }
    testEnd(0 as libc::c_int);
}
pub static mut m1pat: [[libc::c_uint; 21]; 8] = [
    [
        0x1fc77f as libc::c_int as libc::c_uint,
        0x105c41 as libc::c_int as libc::c_uint,
        0x174c5d as libc::c_int as libc::c_uint,
        0x174b5d as libc::c_int as libc::c_uint,
        0x175b5d as libc::c_int as libc::c_uint,
        0x104241 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0x154512 as libc::c_int as libc::c_uint,
        0x1a16a2 as libc::c_int as libc::c_uint,
        0x376ee as libc::c_int as libc::c_uint,
        0x19abb2 as libc::c_int as libc::c_uint,
        0x4eee1 as libc::c_int as libc::c_uint,
        0x1442 as libc::c_int as libc::c_uint,
        0x1fc111 as libc::c_int as libc::c_uint,
        0x10444b as libc::c_int as libc::c_uint,
        0x175d5d as libc::c_int as libc::c_uint,
        0x174aae as libc::c_int as libc::c_uint,
        0x175ae5 as libc::c_int as libc::c_uint,
        0x1043b8 as libc::c_int as libc::c_uint,
        0x1fd2e5 as libc::c_int as libc::c_uint,
    ],
    [
        0x1fdd7f as libc::c_int as libc::c_uint,
        0x104641 as libc::c_int as libc::c_uint,
        0x17565d as libc::c_int as libc::c_uint,
        0x17415d as libc::c_int as libc::c_uint,
        0x17415d as libc::c_int as libc::c_uint,
        0x105841 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0xa00 as libc::c_int as libc::c_uint,
        0x146f25 as libc::c_int as libc::c_uint,
        0x10bc08 as libc::c_int as libc::c_uint,
        0x9dc44 as libc::c_int as libc::c_uint,
        0x130118 as libc::c_int as libc::c_uint,
        0xe444b as libc::c_int as libc::c_uint,
        0x1ee8 as libc::c_int as libc::c_uint,
        0x1fdbbb as libc::c_int as libc::c_uint,
        0x104ee1 as libc::c_int as libc::c_uint,
        0x1747f7 as libc::c_int as libc::c_uint,
        0x174004 as libc::c_int as libc::c_uint,
        0x17504f as libc::c_int as libc::c_uint,
        0x104912 as libc::c_int as libc::c_uint,
        0x1fd84f as libc::c_int as libc::c_uint,
    ],
    [
        0x1fcb7f as libc::c_int as libc::c_uint,
        0x104f41 as libc::c_int as libc::c_uint,
        0x17505d as libc::c_int as libc::c_uint,
        0x17585d as libc::c_int as libc::c_uint,
        0x17575d as libc::c_int as libc::c_uint,
        0x105141 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0x1300 as libc::c_int as libc::c_uint,
        0x17c97c as libc::c_int as libc::c_uint,
        0x2b52c as libc::c_int as libc::c_uint,
        0x46a9f as libc::c_int as libc::c_uint,
        0x1083c as libc::c_int as libc::c_uint,
        0x3f290 as libc::c_int as libc::c_uint,
        0x17cc as libc::c_int as libc::c_uint,
        0x1fcd60 as libc::c_int as libc::c_uint,
        0x1057c5 as libc::c_int as libc::c_uint,
        0x17512c as libc::c_int as libc::c_uint,
        0x175920 as libc::c_int as libc::c_uint,
        0x175694 as libc::c_int as libc::c_uint,
        0x104036 as libc::c_int as libc::c_uint,
        0x1fde94 as libc::c_int as libc::c_uint,
    ],
    [
        0x1fdb7f as libc::c_int as libc::c_uint,
        0x105441 as libc::c_int as libc::c_uint,
        0x174d5d as libc::c_int as libc::c_uint,
        0x17585d as libc::c_int as libc::c_uint,
        0x174c5d as libc::c_int as libc::c_uint,
        0x104c41 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0x1800 as libc::c_int as libc::c_uint,
        0x16e44b as libc::c_int as libc::c_uint,
        0x2b52c as libc::c_int as libc::c_uint,
        0x12f1f2 as libc::c_int as libc::c_uint,
        0x1a258a as libc::c_int as libc::c_uint,
        0x3f290 as libc::c_int as libc::c_uint,
        0x1ca1 as libc::c_int as libc::c_uint,
        0x1fd0d6 as libc::c_int as libc::c_uint,
        0x1057c5 as libc::c_int as libc::c_uint,
        0x174a41 as libc::c_int as libc::c_uint,
        0x175496 as libc::c_int as libc::c_uint,
        0x175694 as libc::c_int as libc::c_uint,
        0x104b5b as libc::c_int as libc::c_uint,
        0x1fd322 as libc::c_int as libc::c_uint,
    ],
    [
        0x1fd37f as libc::c_int as libc::c_uint,
        0x104741 as libc::c_int as libc::c_uint,
        0x17475d as libc::c_int as libc::c_uint,
        0x175f5d as libc::c_int as libc::c_uint,
        0x175f5d as libc::c_int as libc::c_uint,
        0x105941 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0x1400 as libc::c_int as libc::c_uint,
        0x1171f9 as libc::c_int as libc::c_uint,
        0xc8dcf as libc::c_int as libc::c_uint,
        0x15ed83 as libc::c_int as libc::c_uint,
        0x108f20 as libc::c_int as libc::c_uint,
        0xdca73 as libc::c_int as libc::c_uint,
        0x1f2f as libc::c_int as libc::c_uint,
        0x1fda7c as libc::c_int as libc::c_uint,
        0x1040d9 as libc::c_int as libc::c_uint,
        0x1759cf as libc::c_int as libc::c_uint,
        0x1741c3 as libc::c_int as libc::c_uint,
        0x174188 as libc::c_int as libc::c_uint,
        0x10472a as libc::c_int as libc::c_uint,
        0x1fd677 as libc::c_int as libc::c_uint,
    ],
    [
        0x1fcd7f as libc::c_int as libc::c_uint,
        0x105741 as libc::c_int as libc::c_uint,
        0x17505d as libc::c_int as libc::c_uint,
        0x17545d as libc::c_int as libc::c_uint,
        0x17475d as libc::c_int as libc::c_uint,
        0x104941 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0x1b00 as libc::c_int as libc::c_uint,
        0x1059ce as libc::c_int as libc::c_uint,
        0x5a95d as libc::c_int as libc::c_uint,
        0x46a9f as libc::c_int as libc::c_uint,
        0x3001c as libc::c_int as libc::c_uint,
        0xe444b as libc::c_int as libc::c_uint,
        0x1fec as libc::c_int as libc::c_uint,
        0x1fcd60 as libc::c_int as libc::c_uint,
        0x104bb4 as libc::c_int as libc::c_uint,
        0x17412c as libc::c_int as libc::c_uint,
        0x174100 as libc::c_int as libc::c_uint,
        0x17404f as libc::c_int as libc::c_uint,
        0x104816 as libc::c_int as libc::c_uint,
        0x1fde94 as libc::c_int as libc::c_uint,
    ],
    [
        0x1fdd7f as libc::c_int as libc::c_uint,
        0x105741 as libc::c_int as libc::c_uint,
        0x17545d as libc::c_int as libc::c_uint,
        0x17445d as libc::c_int as libc::c_uint,
        0x17555d as libc::c_int as libc::c_uint,
        0x104f41 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0xb00 as libc::c_int as libc::c_uint,
        0x13fd97 as libc::c_int as libc::c_uint,
        0x5a95d as libc::c_int as libc::c_uint,
        0xf8d6 as libc::c_int as libc::c_uint,
        0x28604 as libc::c_int as libc::c_uint,
        0xe444b as libc::c_int as libc::c_uint,
        0x1f2f as libc::c_int as libc::c_uint,
        0x1fd9f2 as libc::c_int as libc::c_uint,
        0x105bb4 as libc::c_int as libc::c_uint,
        0x175365 as libc::c_int as libc::c_uint,
        0x175718 as libc::c_int as libc::c_uint,
        0x17404f as libc::c_int as libc::c_uint,
        0x1048d5 as libc::c_int as libc::c_uint,
        0x1fda06 as libc::c_int as libc::c_uint,
    ],
    [
        0x1fc77f as libc::c_int as libc::c_uint,
        0x104841 as libc::c_int as libc::c_uint,
        0x174e5d as libc::c_int as libc::c_uint,
        0x174b5d as libc::c_int as libc::c_uint,
        0x174f5d as libc::c_int as libc::c_uint,
        0x105041 as libc::c_int as libc::c_uint,
        0x1fd57f as libc::c_int as libc::c_uint,
        0x400 as libc::c_int as libc::c_uint,
        0x12d7a0 as libc::c_int as libc::c_uint,
        0x1a16a2 as libc::c_int as libc::c_uint,
        0xa527c as libc::c_int as libc::c_uint,
        0x1d39fb as libc::c_int as libc::c_uint,
        0x4eee1 as libc::c_int as libc::c_uint,
        0x10d0 as libc::c_int as libc::c_uint,
        0x1fc358 as libc::c_int as libc::c_uint,
        0x10544b as libc::c_int as libc::c_uint,
        0x1749cf as libc::c_int as libc::c_uint,
        0x1758e7 as libc::c_int as libc::c_uint,
        0x174ae5 as libc::c_int as libc::c_uint,
        0x10472a as libc::c_int as libc::c_uint,
        0x1fd0ac as libc::c_int as libc::c_uint,
    ],
];
unsafe extern "C" fn test_encode() {
    let mut current_block: u64;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"01234567\0");
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_encode\0"))
            .as_ptr(),
        b"Test encode (1-M)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    if !stream.is_null() {
        QRinput_append(
            stream,
            QR_MODE_NUM,
            8 as libc::c_int,
            num.as_mut_ptr() as *mut libc::c_uchar,
        );
        mask = 0 as libc::c_int;
        loop {
            if !(mask < 8 as libc::c_int) {
                current_block = 15089075282327824602;
                break;
            }
            QRinput_setVersion(stream, 1 as libc::c_int);
            QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_M);
            qrcode = QRcode_encodeMask(stream, mask);
            if qrcode.is_null() {
                current_block = 6953345254014428093;
                break;
            }
            w = (*qrcode).width;
            frame = (*qrcode).data;
            y = 0 as libc::c_int;
            while y < w {
                x = 0 as libc::c_int;
                while x < w {
                    if m1pat[mask as usize][y as usize] >> 20 as libc::c_int - x
                        & 1 as libc::c_int as libc::c_uint
                        != (*frame.offset((y * w + x) as isize) as libc::c_int
                            & 1 as libc::c_int) as libc::c_uint
                    {
                        printf(
                            b"Diff in mask=%d (%d,%d)\n\0" as *const u8
                                as *const libc::c_char,
                            mask,
                            x,
                            y,
                        );
                        err += 1;
                        err;
                    }
                    x += 1;
                    x;
                }
                y += 1;
                y;
            }
            QRcode_free(qrcode);
            mask += 1;
            mask;
        }
        match current_block {
            6953345254014428093 => {}
            _ => {
                QRinput_free(stream);
            }
        }
    }
    testEnd(err);
}
unsafe extern "C" fn test_encode2() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_encode2\0"))
            .as_ptr(),
        b"Test encode (2-H) (no padding test)\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        b"abcdefghijk123456789012\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        0 as libc::c_int,
    );
    testEnd(!((*qrcode).version == 2 as libc::c_int) as libc::c_int);
    QRcode_free(qrcode);
}
unsafe extern "C" fn test_encode3() {
    let mut code1: *mut QRcode = 0 as *mut QRcode;
    let mut code2: *mut QRcode = 0 as *mut QRcode;
    let mut input: *mut QRinput = 0 as *mut QRinput;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_encode3\0"))
            .as_ptr(),
        b"Compare encodeString and encodeInput\0" as *const u8 as *const libc::c_char,
    );
    code1 = QRcode_encodeString(
        b"0123456\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_NUM,
        7 as libc::c_int,
        b"0123456\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    code2 = QRcode_encodeInput(input);
    testEnd(
        memcmp(
            (*code1).data as *const libc::c_void,
            (*code2).data as *const libc::c_void,
            ((*code1).width * (*code1).width) as libc::c_ulong,
        ),
    );
    QRcode_free(code1);
    QRcode_free(code2);
    QRinput_free(input);
}
unsafe extern "C" fn test_encodeNull() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_encodeNull\0"))
            .as_ptr(),
        b"Test encode NULL.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        0 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    assertionNum;
    if !qrcode.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRcode_encodeString() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeEmpty() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_encodeEmpty\0"))
            .as_ptr(),
        b"Test encode an empty string.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    assertionNum;
    if !qrcode.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRcode_encodeString() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeNull8() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_encodeNull8\0"))
            .as_ptr(),
        b"Test encode NULL.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString8bit(
        0 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    assertionNum;
    if !qrcode.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRcode_encodeString8bit() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeEmpty8() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"test_encodeEmpty8\0"))
            .as_ptr(),
        b"Test encode an empty string.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString8bit(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    assertionNum;
    if !qrcode.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRcode_encodeString8bit() returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
}
unsafe extern "C" fn test_encodeLongData() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut data: [libc::c_uchar; 7090] = [0; 7090];
    let mut maxlength: [[libc::c_int; 4]; 4] = [
        [
            7089 as libc::c_int,
            5596 as libc::c_int,
            3993 as libc::c_int,
            3057 as libc::c_int,
        ],
        [
            4296 as libc::c_int,
            3391 as libc::c_int,
            2420 as libc::c_int,
            1852 as libc::c_int,
        ],
        [
            2953 as libc::c_int,
            2331 as libc::c_int,
            1663 as libc::c_int,
            1273 as libc::c_int,
        ],
        [
            1817 as libc::c_int * 2 as libc::c_int,
            1435 as libc::c_int * 2 as libc::c_int,
            1024 as libc::c_int * 2 as libc::c_int,
            784 as libc::c_int * 2 as libc::c_int,
        ],
    ];
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_encodeLongData\0"))
            .as_ptr(),
        b"Encoding long data.\0" as *const u8 as *const libc::c_char,
    );
    i = QR_MODE_NUM as libc::c_int;
    while i <= QR_MODE_KANJI as libc::c_int {
        if i != QR_MODE_KANJI as libc::c_int {
            memset(
                data.as_mut_ptr() as *mut libc::c_void,
                '0' as i32,
                (maxlength[i as usize][0 as libc::c_int as usize] + 1 as libc::c_int)
                    as libc::c_ulong,
            );
        } else {
            l = 0 as libc::c_int;
            while l
                <= maxlength[i as usize][0 as libc::c_int as usize] / 2 as libc::c_int
                    + 1 as libc::c_int
            {
                data[(l * 2 as libc::c_int)
                    as usize] = 0x93 as libc::c_int as libc::c_uchar;
                data[(l * 2 as libc::c_int + 1 as libc::c_int)
                    as usize] = 0x5f as libc::c_int as libc::c_uchar;
                l += 1;
                l;
            }
        }
        l = QR_ECLEVEL_L as libc::c_int;
        while l <= QR_ECLEVEL_H as libc::c_int {
            stream = QRinput_new2(0 as libc::c_int, l as QRecLevel);
            ret = QRinput_append(
                stream,
                i as QRencodeMode,
                maxlength[i as usize][l as usize],
                data.as_mut_ptr(),
            );
            assertionNum += 1;
            assertionNum;
            if !(ret == 0 as libc::c_int) {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Failed to add %d-byte %s to a QRinput\n\0" as *const u8
                        as *const libc::c_char,
                    maxlength[i as usize][l as usize],
                    modeStr[i as usize],
                );
            }
            qrcode = QRcode_encodeInput(stream);
            assertionNum += 1;
            assertionNum;
            if qrcode.is_null() {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"(QRcode_encodeInput) failed to encode %d-byte %s in level %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    maxlength[i as usize][l as usize],
                    modeStr[i as usize],
                    l,
                );
            }
            if !qrcode.is_null() {
                QRcode_free(qrcode);
            }
            QRinput_free(stream);
            stream = QRinput_new2(0 as libc::c_int, l as QRecLevel);
            len = maxlength[i as usize][l as usize];
            if i == QR_MODE_KANJI as libc::c_int {
                len += 2 as libc::c_int;
            } else {
                len += 1 as libc::c_int;
            }
            ret = QRinput_append(stream, i as QRencodeMode, len, data.as_mut_ptr());
            if ret == 0 as libc::c_int {
                qrcode = QRcode_encodeInput(stream);
                assertionNum += 1;
                assertionNum;
                if !qrcode.is_null() {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(
                        b"(QRcode_encodeInput) incorrectly succeeded to encode %d-byte %s in level %d.\n\0"
                            as *const u8 as *const libc::c_char,
                        len,
                        modeStr[i as usize],
                        l,
                    );
                }
                if !qrcode.is_null() {
                    printf(
                        b"version: %d\n\0" as *const u8 as *const libc::c_char,
                        (*qrcode).version,
                    );
                    QRcode_free(qrcode);
                }
            }
            QRinput_free(stream);
            l += 1;
            l;
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_encodeVer26Num() {
    let mut data: [libc::c_char; 3284] = [0; 3284];
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_encodeVer26Num\0"))
            .as_ptr(),
        b"Encoding 3283 digits number. (issue #160)\0" as *const u8
            as *const libc::c_char,
    );
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        '0' as i32,
        3283 as libc::c_int as libc::c_ulong,
    );
    data[3283 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    qrcode = QRcode_encodeString(
        data.as_mut_ptr(),
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    assertionNum;
    if qrcode.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"(QRcode_encodeString) failed to encode 3283 digits number in level L.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*qrcode).version == 26 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"version number is %d (26 expected)\n\0" as *const u8
                as *const libc::c_char,
            (*qrcode).version,
        );
    }
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(data.as_mut_ptr(), input, QR_MODE_8, 0 as libc::c_int);
    list = (*input).head;
    assertionNum += 1;
    assertionNum;
    if !((*list).size == 3283 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"chunk size is wrong. (%d, 3283 expected)\n\0" as *const u8
                as *const libc::c_char,
            (*list).size,
        );
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_01234567() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"01234567\0");
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut correct: [libc::c_uchar; 441] = [
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0x90 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0x90 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x90 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x90 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x91 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x81 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x84 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0x85 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_01234567\0"))
            .as_ptr(),
        b"Encode 01234567 in 1-M\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_M);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    qrcode = QRcode_encodeInput(stream);
    i = 0 as libc::c_int;
    while i < (*qrcode).width * (*qrcode).width {
        if *((*qrcode).data).offset(i as isize) as libc::c_int
            != correct[i as usize] as libc::c_int
        {
            err += 1;
            err;
        }
        i += 1;
        i;
    }
    testEnd(err);
    QRinput_free(stream);
    QRcode_free(qrcode);
}
unsafe extern "C" fn print_01234567() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"01234567\0");
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    puts(b"\nPrinting QR code of '01234567'.\0" as *const u8 as *const libc::c_char);
    stream = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_M);
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    qrcode = QRcode_encodeInput(stream);
    printQRcode(qrcode);
    QRinput_free(stream);
    QRcode_free(qrcode);
}
unsafe extern "C" fn test_invalid_input() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"test_invalid_input\0"))
            .as_ptr(),
        b"Testing invalid input.\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new();
    QRinput_append(
        input,
        QR_MODE_AN,
        5 as libc::c_int,
        b"TEST1\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    (*input).version = -(1 as libc::c_int);
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid version(-1)  was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    (*input).version = 41 as libc::c_int;
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid version(41) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = (QR_ECLEVEL_H as libc::c_int + 1 as libc::c_int) as QRecLevel;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid level(H+1) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = 4294967295 as QRecLevel;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid level(-1) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_struct_semilong() {
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut str: *const libc::c_char = b"asdfasdfasdfasdfasdfASDFASDASDFASDFAsdfasdfasdfasdASDFASDFADSADadsfasdf\0"
        as *const u8 as *const libc::c_char;
    let mut num: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"test_struct_semilong\0"))
            .as_ptr(),
        b"Testing semi-long structured-append symbols\0" as *const u8
            as *const libc::c_char,
    );
    codes = QRcode_encodeString8bitStructured(str, 1 as libc::c_int, QR_ECLEVEL_L);
    list = codes;
    num = 0 as libc::c_int;
    while !list.is_null() {
        num += 1;
        num;
        assertionNum += 1;
        assertionNum;
        if !((*(*list).code).version == 1 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"version number is %d (1 expected)\n\0" as *const u8
                    as *const libc::c_char,
                (*(*list).code).version,
            );
        }
        list = (*list).next;
    }
    size = QRcode_List_size(codes);
    assertionNum += 1;
    assertionNum;
    if !(num == size) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRcode_List_size returns wrong size?\0" as *const u8 as *const libc::c_char,
        );
    }
    QRcode_List_free(codes);
    codes = QRcode_encodeStringStructured(
        str,
        1 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        1 as libc::c_int,
    );
    list = codes;
    num = 0 as libc::c_int;
    while !list.is_null() {
        num += 1;
        num;
        assertionNum += 1;
        assertionNum;
        if !((*(*list).code).version == 1 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"version number is %d (1 expected)\n\0" as *const u8
                    as *const libc::c_char,
                (*(*list).code).version,
            );
        }
        list = (*list).next;
    }
    size = QRcode_List_size(codes);
    assertionNum += 1;
    assertionNum;
    if !(num == size) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRcode_List_size returns wrong size?\0" as *const u8 as *const libc::c_char,
        );
    }
    QRcode_List_free(codes);
    testFinish();
}
unsafe extern "C" fn test_struct_example() {
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut str: *const libc::c_char = b"an example of four Structured Append symbols,\0"
        as *const u8 as *const libc::c_char;
    let mut num: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_struct_example\0"))
            .as_ptr(),
        b"Testing the example of structured-append symbols\0" as *const u8
            as *const libc::c_char,
    );
    codes = QRcode_encodeString8bitStructured(str, 1 as libc::c_int, QR_ECLEVEL_M);
    list = codes;
    num = 0 as libc::c_int;
    while !list.is_null() {
        num += 1;
        num;
        assertionNum += 1;
        assertionNum;
        if !((*(*list).code).version == 1 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"version number is %d (1 expected)\n\0" as *const u8
                    as *const libc::c_char,
                (*(*list).code).version,
            );
        }
        list = (*list).next;
    }
    assertionNum += 1;
    assertionNum;
    if !(num == 4 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"number of symbols is %d (4 expected).\0" as *const u8
                as *const libc::c_char,
            num,
        );
    }
    testFinish();
    QRcode_List_free(codes);
}
unsafe extern "C" fn test_null_free() {
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_null_free\0"))
            .as_ptr(),
        b"Testing free NULL pointers\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Check QRcode_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRcode_free(0 as *mut QRcode);
    printf(b"Check QRcode_List_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRcode_List_free(0 as *mut QRcode_List);
    printf(b"Check QRraw_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRraw_free(0 as *mut QRRawCode);
    testFinish();
}
unsafe extern "C" fn test_encodeTooLongMQR() {
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut data: [*mut libc::c_char; 4] = [
        b"012345\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ABC0EFG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0123456789\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0123456789ABCDEFG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"test_encodeTooLongMQR\0"))
            .as_ptr(),
        b"Encode too large data for MQR.\0" as *const u8 as *const libc::c_char,
    );
    code = QRcode_encodeStringMQR(
        data[0 as libc::c_int as usize],
        1 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    assertionNum;
    if code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"6 byte length numeric string should be accepted to version 2 or larger.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*code).version == 2 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"6 byte length numeric string should be accepted to version 2.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    code = QRcode_encodeStringMQR(
        data[1 as libc::c_int as usize],
        2 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    assertionNum += 1;
    assertionNum;
    if code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"7 byte length alphanumeric string should be accepted to version 3 or larger.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*code).version == 3 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"7 byte length alphanumeric string should be accepted to version 3.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    code = QRcode_encodeString8bitMQR(
        data[2 as libc::c_int as usize],
        3 as libc::c_int,
        QR_ECLEVEL_L,
    );
    assertionNum += 1;
    assertionNum;
    if code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"9 byte length 8bit string should be accepted to version 4.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*code).version == 4 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"9 byte length 8bit string should be accepted to version 4.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    code = QRcode_encodeString8bitMQR(
        data[3 as libc::c_int as usize],
        4 as libc::c_int,
        QR_ECLEVEL_L,
    );
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"16 byte length 8bit string was accepted to version 4.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(*__errno_location() == 34 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"errno != ERANGE\n\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    if !code.is_null() {
        printQRcode(code);
        QRcode_free(code);
    }
}
unsafe extern "C" fn test_mqrraw_new() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: *mut libc::c_char = b"01234\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut datacode: [libc::c_uchar; 3] = [
        0xa0 as libc::c_int as libc::c_uchar,
        0x62 as libc::c_int as libc::c_uchar,
        0x20 as libc::c_int as libc::c_uchar,
    ];
    let mut raw: *mut MQRRawCode = 0 as *mut MQRRawCode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_mqrraw_new\0"))
            .as_ptr(),
        b"Test MQRRaw_new()\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(stream, QR_MODE_NUM, 5 as libc::c_int, num as *mut libc::c_uchar);
    raw = MQRraw_new(stream);
    assertionNum += 1;
    assertionNum;
    if raw.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed MQRraw_new().\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).count == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"MQRraw.count = %d != 0\n\0" as *const u8 as *const libc::c_char,
            (*raw).count,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).version == 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"MQRraw.version was not as expected. (%d)\n\0" as *const u8
                as *const libc::c_char,
            (*raw).version,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).dataLength == 3 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"MQRraw.dataLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*raw).eccLength == 2 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"MQRraw.eccLength was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(memcmp(
        (*raw).datacode as *const libc::c_void,
        datacode.as_mut_ptr() as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Datacode doesn't match.\n\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(stream);
    MQRraw_free(raw);
    testFinish();
}
unsafe extern "C" fn test_encodeData() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_encodeData\0"))
            .as_ptr(),
        b"Test QRencode_encodeData.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeData(
        0 as libc::c_int,
        0 as *const libc::c_uchar,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    assertionNum;
    if !qrcode.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRcode_encodeData(NULL, 0) returned something.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
    qrcode = QRcode_encodeData(
        10 as libc::c_int,
        b"test\0\0test\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
        0 as libc::c_int,
        QR_ECLEVEL_H,
    );
    assertionNum += 1;
    assertionNum;
    if qrcode.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"QRcode_encodeData() failed.\n\0" as *const u8 as *const libc::c_char);
    }
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_formatInfo() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut mask: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_formatInfo\0"))
            .as_ptr(),
        b"Test format info in QR code.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        b"AC-42\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        1 as libc::c_int,
    );
    ret = QRcode_decodeFormat(qrcode, &mut level, &mut mask);
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(level as libc::c_uint == QR_ECLEVEL_H as libc::c_int as libc::c_uint) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Decoded format is wrong.\n\0" as *const u8 as *const libc::c_char);
    }
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_formatInfoMQR() {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"test_formatInfoMQR\0"))
            .as_ptr(),
        b"Test format info in Micro QR code.\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        qrcode = QRcode_encodeStringMQR(
            b"1\0" as *const u8 as *const libc::c_char,
            (*MQRformat.as_mut_ptr().offset(i as isize)).version,
            (*MQRformat.as_mut_ptr().offset(i as isize)).level,
            QR_MODE_8,
            1 as libc::c_int,
        );
        ret = QRcode_decodeFormatMQR(qrcode, &mut version, &mut level, &mut mask);
        assertionNum += 1;
        assertionNum;
        if !(ret == 0 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
        }
        assertionNum += 1;
        assertionNum;
        if !((*MQRformat.as_mut_ptr().offset(i as isize)).version == version) {
            assertionFailed += 1;
            assertionFailed;
            printf(b"Decoded verion is wrong.\n\0" as *const u8 as *const libc::c_char);
        }
        assertionNum += 1;
        assertionNum;
        if !((*MQRformat.as_mut_ptr().offset(i as isize)).level as libc::c_uint
            == level as libc::c_uint)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(b"Decoded level is wrong.\n\0" as *const u8 as *const libc::c_char);
        }
        QRcode_free(qrcode);
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_decodeSimple() {
    let mut str: *mut libc::c_char = b"AC-42\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"test_decodeSimple\0"))
            .as_ptr(),
        b"Test code words.\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        str,
        1 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        1 as libc::c_int,
    );
    qrdata = QRcode_decode(qrcode);
    assertionNum += 1;
    assertionNum;
    if qrdata.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    if !qrdata.is_null() {
        assertionNum += 1;
        assertionNum;
        if !(strlen(str) == (*qrdata).size as libc::c_ulong) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Lengths of input/output mismatched: %d, expected %d.\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).size,
                strlen(str) as libc::c_int,
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(strncmp(
            str,
            (*qrdata).data as *mut libc::c_char,
            (*qrdata).size as libc::c_ulong,
        ) == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Decoded data %s is different from the original %s\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data,
                str,
            );
        }
    }
    if !qrdata.is_null() {
        QRdata_free(qrdata);
    }
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_decodeLong() {
    let mut str: *mut libc::c_char = b"12345678901234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ?????????????\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_decodeLong\0"))
            .as_ptr(),
        b"Test code words (long, splitted).\0" as *const u8 as *const libc::c_char,
    );
    qrcode = QRcode_encodeString(
        str,
        0 as libc::c_int,
        QR_ECLEVEL_H,
        QR_MODE_8,
        1 as libc::c_int,
    );
    qrdata = QRcode_decode(qrcode);
    assertionNum += 1;
    assertionNum;
    if qrdata.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    if !qrdata.is_null() {
        assertionNum += 1;
        assertionNum;
        if !(strlen(str) == (*qrdata).size as libc::c_ulong) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Lengths of input/output mismatched.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(strncmp(
            str,
            (*qrdata).data as *mut libc::c_char,
            (*qrdata).size as libc::c_ulong,
        ) == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Decoded data %s is different from the original %s\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data,
                str,
            );
        }
    }
    if !qrdata.is_null() {
        QRdata_free(qrdata);
    }
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_decodeVeryLong() {
    let mut str: [libc::c_char; 4000] = [0; 4000];
    let mut i: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_decodeVeryLong\0"))
            .as_ptr(),
        b"Test code words (very long string).\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 3999 as libc::c_int {
        str[i
            as usize] = decodeAnTable[(45 as libc::c_int as libc::c_double
            * rand() as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_int
            as usize];
        i += 1;
        i;
    }
    str[3999 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    qrcode = QRcode_encodeString(
        str.as_mut_ptr(),
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_8,
        0 as libc::c_int,
    );
    qrdata = QRcode_decode(qrcode);
    assertionNum += 1;
    assertionNum;
    if qrdata.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
    }
    if !qrdata.is_null() {
        assertionNum += 1;
        assertionNum;
        if !(strlen(str.as_mut_ptr()) == (*qrdata).size as libc::c_ulong) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Lengths of input/output mismatched.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(strncmp(
            str.as_mut_ptr(),
            (*qrdata).data as *mut libc::c_char,
            (*qrdata).size as libc::c_ulong,
        ) == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Decoded data %s is different from the original %s\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data,
                str.as_mut_ptr(),
            );
        }
    }
    if !qrdata.is_null() {
        QRdata_free(qrdata);
    }
    if !qrcode.is_null() {
        QRcode_free(qrcode);
    }
    testFinish();
}
unsafe extern "C" fn test_decodeShortMQR() {
    let mut str: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"55\0");
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_decodeShortMQR\0"))
            .as_ptr(),
        b"Test code words (MQR).\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        qrcode = QRcode_encodeStringMQR(
            str.as_mut_ptr(),
            (*MQRformat.as_mut_ptr().offset(i as isize)).version,
            (*MQRformat.as_mut_ptr().offset(i as isize)).level,
            QR_MODE_8,
            1 as libc::c_int,
        );
        qrdata = QRcode_decodeMQR(qrcode);
        assertionNum += 1;
        assertionNum;
        if qrdata.is_null() {
            assertionFailed += 1;
            assertionFailed;
            printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
        }
        assertionNum += 1;
        assertionNum;
        if !(strcmp((*qrdata).data as *mut libc::c_char, str.as_mut_ptr())
            == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Decoded data (%s) mismatched (%s)\n\0" as *const u8
                    as *const libc::c_char,
                (*qrdata).data as *mut libc::c_char,
                str.as_mut_ptr(),
            );
        }
        if !qrdata.is_null() {
            QRdata_free(qrdata);
        }
        if !qrcode.is_null() {
            QRcode_free(qrcode);
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_oddBitCalcMQR() {
    let mut tests: [TestString; 2] = [
        {
            let mut init = TestString {
                str_0: b"46194\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                version: 1 as libc::c_int,
                level: QR_ECLEVEL_L,
                hint: QR_MODE_8,
                casesensitive: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = TestString {
                str_0: b"WBA5Y47YPQQ\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                version: 3 as libc::c_int,
                level: QR_ECLEVEL_L,
                hint: QR_MODE_8,
                casesensitive: 1 as libc::c_int,
            };
            init
        },
    ];
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"test_oddBitCalcMQR\0"))
            .as_ptr(),
        b"Odd bits calculation bug checking (MQR).\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[TestString; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<TestString>() as libc::c_ulong)
    {
        qrcode = QRcode_encodeStringMQR(
            tests[i as usize].str_0,
            tests[i as usize].version,
            tests[i as usize].level,
            tests[i as usize].hint,
            tests[i as usize].casesensitive,
        );
        assertionNum += 1;
        assertionNum;
        if qrcode.is_null() {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Failed to encode: %s\n\0" as *const u8 as *const libc::c_char,
                tests[i as usize].str_0,
            );
        }
        if !qrcode.is_null() {
            qrdata = QRcode_decodeMQR(qrcode);
            assertionNum += 1;
            assertionNum;
            if qrdata.is_null() {
                assertionFailed += 1;
                assertionFailed;
                printf(b"Failed to decode.\n\0" as *const u8 as *const libc::c_char);
            }
            assertionNum += 1;
            assertionNum;
            if !(strcmp((*qrdata).data as *mut libc::c_char, tests[i as usize].str_0)
                == 0 as libc::c_int)
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Decoded data (%s) mismatched (%s)\n\0" as *const u8
                        as *const libc::c_char,
                    (*qrdata).data as *mut libc::c_char,
                    tests[i as usize].str_0,
                );
            }
            if !qrdata.is_null() {
                QRdata_free(qrdata);
            }
            QRcode_free(qrcode);
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_invalid_inputMQR() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"test_invalid_inputMQR\0"))
            .as_ptr(),
        b"Testing invalid input (MQR).\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_AN,
        5 as libc::c_int,
        b"TEST1\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    (*input).version = -(1 as libc::c_int);
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid version(-1)  was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    (*input).version = 5 as libc::c_int;
    (*input).level = QR_ECLEVEL_L;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid version(5) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = QR_ECLEVEL_H;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid level(H) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    (*input).version = 1 as libc::c_int;
    (*input).level = 4294967295 as QRecLevel;
    code = QRcode_encodeInput(input);
    assertionNum += 1;
    assertionNum;
    if !code.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"invalid level(-1) access was not checked.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !code.is_null() {
        QRcode_free(code);
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_mqrencode() {
    let mut str: *mut libc::c_char = b"MICROQR\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut pattern: [libc::c_char; 226] = *::std::mem::transmute::<
        &[u8; 226],
        &mut [libc::c_char; 226],
    >(
        b"#######_#_#_#_##_____#_#__#####_###_#_#_####_#_###_#_#__##_##_###_#___#__###_____#____#_#_#######__##_#_#_________#__#__#___#__####_#_#_#######_#_##_###___#_#____#___##_#_####____##__###___#__##__###_#_###_#_#_##____####_###_\0",
    );
    let mut qrcode: QRcode = QRcode {
        version: 0,
        width: 0,
        data: 0 as *mut libc::c_uchar,
    };
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_mqrencode\0"))
            .as_ptr(),
        b"Encoding test (MQR).\0" as *const u8 as *const libc::c_char,
    );
    qrcode.width = 15 as libc::c_int;
    qrcode.version = 3 as libc::c_int;
    frame = MQRspec_newFrame(qrcode.version);
    i = 0 as libc::c_int;
    while i < 225 as libc::c_int {
        let ref mut fresh0 = *frame.offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_int
            ^ if pattern[i as usize] as libc::c_int == '#' as i32 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uchar;
        i += 1;
        i;
    }
    qrcode.data = frame;
    qrdata = QRcode_decodeMQR(&mut qrcode);
    assertionNum += 1;
    assertionNum;
    if !((*qrdata).version == 3 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Format info decoder returns wrong version number: %d (%d expected)\n\0"
                as *const u8 as *const libc::c_char,
            (*qrdata).version,
            3 as libc::c_int,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*qrdata).level as libc::c_uint == 1 as libc::c_int as libc::c_uint) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Format info decoder returns wrong level: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            (*qrdata).level as libc::c_uint,
            1 as libc::c_int,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(strcmp((*qrdata).data as *mut libc::c_char, str) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Decoded data (%s) mismatched (%s)\n\0" as *const u8 as *const libc::c_char,
            (*qrdata).data as *mut libc::c_char,
            str,
        );
    }
    QRdata_free(qrdata);
    free(frame as *mut libc::c_void);
    testFinish();
}
unsafe extern "C" fn test_apiversion() {
    let mut major_version: libc::c_int = 0;
    let mut minor_version: libc::c_int = 0;
    let mut micro_version: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_apiversion\0"))
            .as_ptr(),
        b"API Version check\0" as *const u8 as *const libc::c_char,
    );
    QRcode_APIVersion(&mut major_version, &mut minor_version, &mut micro_version);
    assertionNum += 1;
    assertionNum;
    if !(major_version == 4 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Major version number mismatched: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            major_version,
            4 as libc::c_int,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(minor_version == 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Minor version number mismatched: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            minor_version,
            1 as libc::c_int,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(micro_version == 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Micro version number mismatched: %d (%d expected)\n\0" as *const u8
                as *const libc::c_char,
            micro_version,
            1 as libc::c_int,
        );
    }
    str = QRcode_APIVersionString();
    str2 = QRcode_APIVersionString();
    assertionNum += 1;
    assertionNum;
    if !(strcmp(b"4.1.1\0" as *const u8 as *const libc::c_char, str) == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Version string mismatched: %s (%s expected)\n\0" as *const u8
                as *const libc::c_char,
            str,
            b"4.1.1\0" as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(str == str2) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Version strings are not identical.\0" as *const u8 as *const libc::c_char,
        );
    }
    testFinish();
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tests: libc::c_int = 33 as libc::c_int;
    testInit(tests);
    test_iterate();
    test_iterate2();
    test_filler();
    test_format();
    test_encode();
    test_encode2();
    test_encode3();
    test_encodeNull();
    test_encodeEmpty();
    test_encodeNull8();
    test_encodeEmpty8();
    test_encodeLongData();
    test_encodeVer26Num();
    test_01234567();
    test_invalid_input();
    test_struct_example();
    test_struct_semilong();
    test_null_free();
    test_qrraw_new();
    test_mqrraw_new();
    test_encodeData();
    test_formatInfo();
    test_decodeSimple();
    test_decodeLong();
    test_decodeVeryLong();
    test_fillerMQR();
    test_formatInfoMQR();
    test_encodeTooLongMQR();
    test_decodeShortMQR();
    test_oddBitCalcMQR();
    test_invalid_inputMQR();
    test_mqrencode();
    test_apiversion();
    testReport(tests);
    if argc > 1 as libc::c_int {
        print_filler();
        print_01234567();
        print_fillerMQR();
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
