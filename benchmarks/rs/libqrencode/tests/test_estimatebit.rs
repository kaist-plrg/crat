use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn QRinput_new() -> *mut QRinput;
    fn QRinput_newMQR(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: libc::c_int,
        data: *const libc::c_uchar,
    ) -> libc::c_int;
    fn QRinput_getVersion(input: *mut QRinput) -> libc::c_int;
    fn QRinput_free(input: *mut QRinput);
    static mut assertionFailed: libc::c_int;
    static mut assertionNum: libc::c_int;
    fn testInit(tests: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testEnd(result: libc::c_int);
    fn testFinish();
    fn testReport(tests: libc::c_int);
    fn QRinput_estimateBitStreamSize(
        input: *mut QRinput,
        version: libc::c_int,
    ) -> libc::c_int;
    fn QRinput_insertStructuredAppendHeader(
        input: *mut QRinput,
        size: libc::c_int,
        index: libc::c_int,
        parity: libc::c_uchar,
    ) -> libc::c_int;
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
static mut gstream: *mut QRinput = 0 as *const QRinput as *mut QRinput;
unsafe extern "C" fn test_numbit() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"01234567\0");
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_numbit\0"))
            .as_ptr(),
        b"Estimation of Numeric stream (8 digits)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    bits = QRinput_estimateBitStreamSize(stream, 0 as libc::c_int);
    testEnd(!(bits == 41 as libc::c_int) as libc::c_int);
    QRinput_append(
        gstream,
        QR_MODE_NUM,
        8 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    QRinput_free(stream);
}
unsafe extern "C" fn test_numbit2() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: [libc::c_char; 17] = *::std::mem::transmute::<
        &[u8; 17],
        &mut [libc::c_char; 17],
    >(b"0123456789012345\0");
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_numbit2\0"))
            .as_ptr(),
        b"Estimation of Numeric stream (16 digits)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_append(
        stream,
        QR_MODE_NUM,
        16 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    bits = QRinput_estimateBitStreamSize(stream, 0 as libc::c_int);
    testEnd(!(bits == 68 as libc::c_int) as libc::c_int);
    QRinput_append(
        gstream,
        QR_MODE_NUM,
        16 as libc::c_int,
        num.as_mut_ptr() as *mut libc::c_uchar,
    );
    QRinput_free(stream);
}
unsafe extern "C" fn test_numbit3() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut num: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_numbit3\0"))
            .as_ptr(),
        b"Estimation of Numeric stream (400 digits)\0" as *const u8
            as *const libc::c_char,
    );
    stream = QRinput_new();
    num = malloc(401 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    memset(num as *mut libc::c_void, '1' as i32, 400 as libc::c_int as libc::c_ulong);
    *num.offset(400 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    QRinput_append(stream, QR_MODE_NUM, 400 as libc::c_int, num as *mut libc::c_uchar);
    bits = QRinput_estimateBitStreamSize(stream, 0 as libc::c_int);
    testEnd(!(bits == 1348 as libc::c_int) as libc::c_int);
    QRinput_append(gstream, QR_MODE_NUM, 400 as libc::c_int, num as *mut libc::c_uchar);
    QRinput_free(stream);
    free(num as *mut libc::c_void);
}
unsafe extern "C" fn test_an() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut str: [libc::c_char; 6] = *::std::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"AC-42\0");
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"test_an\0")).as_ptr(),
        b"Estimation of Alphabet-Numeric stream (5 chars)\0" as *const u8
            as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_append(
        stream,
        QR_MODE_AN,
        5 as libc::c_int,
        str.as_mut_ptr() as *mut libc::c_uchar,
    );
    bits = QRinput_estimateBitStreamSize(stream, 0 as libc::c_int);
    testEnd(!(bits == 41 as libc::c_int) as libc::c_int);
    QRinput_append(
        gstream,
        QR_MODE_AN,
        5 as libc::c_int,
        str.as_mut_ptr() as *mut libc::c_uchar,
    );
    QRinput_free(stream);
}
unsafe extern "C" fn test_8() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut str: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"12345678\0");
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"test_8\0")).as_ptr(),
        b"Estimation of 8 bit data stream (8 bytes)\0" as *const u8
            as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_append(
        stream,
        QR_MODE_8,
        8 as libc::c_int,
        str.as_mut_ptr() as *mut libc::c_uchar,
    );
    bits = QRinput_estimateBitStreamSize(stream, 0 as libc::c_int);
    testEnd(!(bits == 76 as libc::c_int) as libc::c_int);
    QRinput_append(
        gstream,
        QR_MODE_8,
        8 as libc::c_int,
        str.as_mut_ptr() as *mut libc::c_uchar,
    );
    QRinput_free(stream);
}
unsafe extern "C" fn test_structure() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_structure\0"))
            .as_ptr(),
        b"Estimation of a structure-append header\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_insertStructuredAppendHeader(
        stream,
        10 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
    );
    bits = QRinput_estimateBitStreamSize(stream, 1 as libc::c_int);
    testEnd(!(bits == 20 as libc::c_int) as libc::c_int);
    QRinput_insertStructuredAppendHeader(
        gstream,
        10 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
    );
    QRinput_free(stream);
}
unsafe extern "C" fn test_kanji() {
    let mut res: libc::c_int = 0;
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut str: [libc::c_uchar; 4] = [
        0x93 as libc::c_int as libc::c_uchar,
        0x5f as libc::c_int as libc::c_uchar,
        0xe4 as libc::c_int as libc::c_uchar,
        0xaa as libc::c_int as libc::c_uchar,
    ];
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"test_kanji\0"))
            .as_ptr(),
        b"Estimation of Kanji stream (2 chars)\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    res = QRinput_append(stream, QR_MODE_KANJI, 4 as libc::c_int, str.as_mut_ptr());
    if res < 0 as libc::c_int {
        printf(b"Failed to add.\n\0" as *const u8 as *const libc::c_char);
        testEnd(1 as libc::c_int);
    } else {
        bits = QRinput_estimateBitStreamSize(stream, 0 as libc::c_int);
        testEnd(!(bits == 38 as libc::c_int) as libc::c_int);
        QRinput_append(gstream, QR_MODE_KANJI, 4 as libc::c_int, str.as_mut_ptr());
    }
    QRinput_free(stream);
}
unsafe extern "C" fn test_mix() {
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"test_mix\0")).as_ptr(),
        b"Estimation of Mixed stream\0" as *const u8 as *const libc::c_char,
    );
    bits = QRinput_estimateBitStreamSize(gstream, 0 as libc::c_int);
    testEnd(
        !(bits
            == 41 as libc::c_int + 68 as libc::c_int + 1348 as libc::c_int
                + 41 as libc::c_int + 76 as libc::c_int + 38 as libc::c_int
                + 20 as libc::c_int) as libc::c_int,
    );
    QRinput_free(gstream);
}
unsafe extern "C" fn test_numbit1_mqr() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut str: *mut libc::c_char = b"0123456789012345\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut bits: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_numbit1_mqr\0"))
            .as_ptr(),
        b"Estimation of Numeric stream for Micro QR Code (16 digits)\0" as *const u8
            as *const libc::c_char,
    );
    stream = QRinput_newMQR(3 as libc::c_int, QR_ECLEVEL_M);
    QRinput_append(stream, QR_MODE_NUM, 16 as libc::c_int, str as *const libc::c_uchar);
    bits = QRinput_estimateBitStreamSize(stream, QRinput_getVersion(stream));
    assertionNum += 1;
    assertionNum;
    if !(bits == 61 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Estimated bit length is wrong: %d, expected: %d.\n\0" as *const u8
                as *const libc::c_char,
            bits,
            61 as libc::c_int,
        );
    }
    QRinput_free(stream);
    stream = QRinput_newMQR(4 as libc::c_int, QR_ECLEVEL_M);
    QRinput_append(stream, QR_MODE_NUM, 16 as libc::c_int, str as *const libc::c_uchar);
    bits = QRinput_estimateBitStreamSize(stream, QRinput_getVersion(stream));
    assertionNum += 1;
    assertionNum;
    if !(bits == 63 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Estimated bit length is wrong: %d, expected: %d.\n\0" as *const u8
                as *const libc::c_char,
            bits,
            63 as libc::c_int,
        );
    }
    QRinput_free(stream);
    testFinish();
}
unsafe fn main_0() -> libc::c_int {
    gstream = QRinput_new();
    let mut tests: libc::c_int = 9 as libc::c_int;
    testInit(tests);
    test_numbit();
    test_numbit2();
    test_numbit3();
    test_an();
    test_8();
    test_kanji();
    test_structure();
    test_mix();
    test_numbit1_mqr();
    testReport(tests);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
