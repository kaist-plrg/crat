use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn QRinput_new() -> *mut QRinput;
    fn QRinput_new2(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_newMQR(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: libc::c_int,
        data: *const libc::c_uchar,
    ) -> libc::c_int;
    fn QRinput_appendECIheader(input: *mut QRinput, ecinum: libc::c_uint) -> libc::c_int;
    fn QRinput_getVersion(input: *mut QRinput) -> libc::c_int;
    fn QRinput_setVersion(input: *mut QRinput, version: libc::c_int) -> libc::c_int;
    fn QRinput_getErrorCorrectionLevel(input: *mut QRinput) -> QRecLevel;
    fn QRinput_setErrorCorrectionLevel(
        input: *mut QRinput,
        level: QRecLevel,
    ) -> libc::c_int;
    fn QRinput_free(input: *mut QRinput);
    fn QRinput_Struct_new() -> *mut QRinput_Struct;
    fn QRinput_Struct_setParity(s: *mut QRinput_Struct, parity: libc::c_uchar);
    fn QRinput_Struct_appendInput(
        s: *mut QRinput_Struct,
        input: *mut QRinput,
    ) -> libc::c_int;
    fn QRinput_Struct_free(s: *mut QRinput_Struct);
    fn QRinput_splitQRinputToStruct(input: *mut QRinput) -> *mut QRinput_Struct;
    fn QRinput_Struct_insertStructuredAppendHeaders(
        s: *mut QRinput_Struct,
    ) -> libc::c_int;
    fn BitStream_new() -> *mut BitStream;
    fn BitStream_free(bstream: *mut BitStream);
    fn QRinput_dup(input: *mut QRinput) -> *mut QRinput;
    fn QRinput_mergeBitStream(
        input: *mut QRinput,
        bstream: *mut BitStream,
    ) -> libc::c_int;
    fn QRinput_getBitStream(input: *mut QRinput, bstream: *mut BitStream) -> libc::c_int;
    fn QRinput_splitEntry(entry: *mut QRinput_List, bytes: libc::c_int) -> libc::c_int;
    fn QRinput_estimateVersion(input: *mut QRinput) -> libc::c_int;
    fn QRinput_lengthOfCode(
        mode: QRencodeMode,
        version: libc::c_int,
        bits: libc::c_int,
    ) -> libc::c_int;
    fn QRinput_insertStructuredAppendHeader(
        input: *mut QRinput,
        size: libc::c_int,
        index: libc::c_int,
        parity: libc::c_uchar,
    ) -> libc::c_int;
    static mut assertionFailed: libc::c_int;
    static mut assertionNum: libc::c_int;
    fn testInit(tests: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testEnd(result: libc::c_int);
    fn testFinish();
    fn testReport(tests: libc::c_int);
    fn ncmpBin(
        correct: *mut libc::c_char,
        bstream: *mut BitStream,
        len: size_t,
    ) -> libc::c_int;
    fn cmpBin(correct: *mut libc::c_char, bstream: *mut BitStream) -> libc::c_int;
    fn printQRinputStruct(s: *mut QRinput_Struct);
    fn printBstream(bstream: *mut BitStream);
    fn Split_splitStringToQRinput(
        string: *const libc::c_char,
        input: *mut QRinput,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_Struct {
    pub size: libc::c_int,
    pub parity: libc::c_int,
    pub head: *mut QRinput_InputList,
    pub tail: *mut QRinput_InputList,
}
pub type QRinput_InputList = _QRinput_InputList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_InputList {
    pub input: *mut QRinput,
    pub next: *mut QRinput_InputList,
}
pub type QRinput_Struct = _QRinput_Struct;
static mut maxCharacterLengths: [[libc::c_int; 4]; 40] = [
    [41 as libc::c_int, 25 as libc::c_int, 17 as libc::c_int, 10 as libc::c_int],
    [77 as libc::c_int, 47 as libc::c_int, 32 as libc::c_int, 20 as libc::c_int],
    [127 as libc::c_int, 77 as libc::c_int, 53 as libc::c_int, 32 as libc::c_int],
    [187 as libc::c_int, 114 as libc::c_int, 78 as libc::c_int, 48 as libc::c_int],
    [255 as libc::c_int, 154 as libc::c_int, 106 as libc::c_int, 65 as libc::c_int],
    [322 as libc::c_int, 195 as libc::c_int, 134 as libc::c_int, 82 as libc::c_int],
    [370 as libc::c_int, 224 as libc::c_int, 154 as libc::c_int, 95 as libc::c_int],
    [461 as libc::c_int, 279 as libc::c_int, 192 as libc::c_int, 118 as libc::c_int],
    [552 as libc::c_int, 335 as libc::c_int, 230 as libc::c_int, 141 as libc::c_int],
    [652 as libc::c_int, 395 as libc::c_int, 271 as libc::c_int, 167 as libc::c_int],
    [772 as libc::c_int, 468 as libc::c_int, 321 as libc::c_int, 198 as libc::c_int],
    [883 as libc::c_int, 535 as libc::c_int, 367 as libc::c_int, 226 as libc::c_int],
    [1022 as libc::c_int, 619 as libc::c_int, 425 as libc::c_int, 262 as libc::c_int],
    [1101 as libc::c_int, 667 as libc::c_int, 458 as libc::c_int, 282 as libc::c_int],
    [1250 as libc::c_int, 758 as libc::c_int, 520 as libc::c_int, 320 as libc::c_int],
    [1408 as libc::c_int, 854 as libc::c_int, 586 as libc::c_int, 361 as libc::c_int],
    [1548 as libc::c_int, 938 as libc::c_int, 644 as libc::c_int, 397 as libc::c_int],
    [1725 as libc::c_int, 1046 as libc::c_int, 718 as libc::c_int, 442 as libc::c_int],
    [1903 as libc::c_int, 1153 as libc::c_int, 792 as libc::c_int, 488 as libc::c_int],
    [2061 as libc::c_int, 1249 as libc::c_int, 858 as libc::c_int, 528 as libc::c_int],
    [2232 as libc::c_int, 1352 as libc::c_int, 929 as libc::c_int, 572 as libc::c_int],
    [2409 as libc::c_int, 1460 as libc::c_int, 1003 as libc::c_int, 618 as libc::c_int],
    [2620 as libc::c_int, 1588 as libc::c_int, 1091 as libc::c_int, 672 as libc::c_int],
    [2812 as libc::c_int, 1704 as libc::c_int, 1171 as libc::c_int, 721 as libc::c_int],
    [3057 as libc::c_int, 1853 as libc::c_int, 1273 as libc::c_int, 784 as libc::c_int],
    [3283 as libc::c_int, 1990 as libc::c_int, 1367 as libc::c_int, 842 as libc::c_int],
    [3517 as libc::c_int, 2132 as libc::c_int, 1465 as libc::c_int, 902 as libc::c_int],
    [3669 as libc::c_int, 2223 as libc::c_int, 1528 as libc::c_int, 940 as libc::c_int],
    [3909 as libc::c_int, 2369 as libc::c_int, 1628 as libc::c_int, 1002 as libc::c_int],
    [4158 as libc::c_int, 2520 as libc::c_int, 1732 as libc::c_int, 1066 as libc::c_int],
    [4417 as libc::c_int, 2677 as libc::c_int, 1840 as libc::c_int, 1132 as libc::c_int],
    [4686 as libc::c_int, 2840 as libc::c_int, 1952 as libc::c_int, 1201 as libc::c_int],
    [4965 as libc::c_int, 3009 as libc::c_int, 2068 as libc::c_int, 1273 as libc::c_int],
    [5253 as libc::c_int, 3183 as libc::c_int, 2188 as libc::c_int, 1347 as libc::c_int],
    [5529 as libc::c_int, 3351 as libc::c_int, 2303 as libc::c_int, 1417 as libc::c_int],
    [5836 as libc::c_int, 3537 as libc::c_int, 2431 as libc::c_int, 1496 as libc::c_int],
    [6153 as libc::c_int, 3729 as libc::c_int, 2563 as libc::c_int, 1577 as libc::c_int],
    [6479 as libc::c_int, 3927 as libc::c_int, 2699 as libc::c_int, 1661 as libc::c_int],
    [6743 as libc::c_int, 4087 as libc::c_int, 2809 as libc::c_int, 1729 as libc::c_int],
    [7089 as libc::c_int, 4296 as libc::c_int, 2953 as libc::c_int, 1817 as libc::c_int],
];
unsafe extern "C" fn encodeAndCheckBStream(
    mut mqr: libc::c_int,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut mode: QRencodeMode,
    mut data: *mut libc::c_char,
    mut correct: *mut libc::c_char,
) -> libc::c_int {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut ret: libc::c_int = 0;
    if mqr != 0 {
        input = QRinput_newMQR(version, level);
    } else {
        input = QRinput_new2(version, level);
    }
    QRinput_append(input, mode, strlen(data) as libc::c_int, data as *mut libc::c_uchar);
    bstream = BitStream_new();
    QRinput_getBitStream(input, bstream);
    ret = cmpBin(correct, bstream);
    if ret != 0 {
        printf(b"result : \0" as *const u8 as *const libc::c_char);
        printBstream(bstream);
        printf(b"correct: %s\n\0" as *const u8 as *const libc::c_char, correct);
    }
    QRinput_free(input);
    BitStream_free(bstream);
    return ret;
}
unsafe extern "C" fn mergeAndCheckBStream(
    mut mqr: libc::c_int,
    mut mode: QRencodeMode,
    mut data: *mut libc::c_char,
    mut correct: *mut libc::c_char,
) -> libc::c_int {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut ret: libc::c_int = 0;
    if mqr != 0 {
        input = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    } else {
        input = QRinput_new();
    }
    QRinput_append(input, mode, strlen(data) as libc::c_int, data as *mut libc::c_uchar);
    bstream = BitStream_new();
    QRinput_mergeBitStream(input, bstream);
    ret = cmpBin(correct, bstream);
    QRinput_free(input);
    BitStream_free(bstream);
    return ret;
}
unsafe extern "C" fn test_encodeKanji() {
    let mut str: [libc::c_char; 5] = [
        0x93 as libc::c_int as libc::c_char,
        0x5f as libc::c_int as libc::c_char,
        0xe4 as libc::c_int as libc::c_char,
        0xaa as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let mut correct: *mut libc::c_char = b"10000000001001101100111111101010101010\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_encodeKanji\0"))
            .as_ptr(),
        b"Encoding kanji stream.\0" as *const u8 as *const libc::c_char,
    );
    testEnd(
        mergeAndCheckBStream(0 as libc::c_int, QR_MODE_KANJI, str.as_mut_ptr(), correct),
    );
}
unsafe extern "C" fn test_encode8() {
    let mut str: [libc::c_char; 6] = *::std::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"AC-42\0");
    let mut correct: [libc::c_char; 53] = *::std::mem::transmute::<
        &[u8; 53],
        &mut [libc::c_char; 53],
    >(b"0100000001010100000101000011001011010011010000110010\0");
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_encode8\0"))
            .as_ptr(),
        b"Encoding 8bit stream.\0" as *const u8 as *const libc::c_char,
    );
    testEnd(
        mergeAndCheckBStream(
            0 as libc::c_int,
            QR_MODE_8,
            str.as_mut_ptr(),
            correct.as_mut_ptr(),
        ),
    );
}
unsafe extern "C" fn test_encode8_versionup() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut version: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"test_encode8_versionup\0"))
            .as_ptr(),
        b"Encoding 8bit stream. (auto-version up test)\0" as *const u8
            as *const libc::c_char,
    );
    str = malloc(2900 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    memset(
        str as *mut libc::c_void,
        0xff as libc::c_int,
        2900 as libc::c_int as libc::c_ulong,
    );
    stream = QRinput_new();
    bstream = BitStream_new();
    QRinput_append(stream, QR_MODE_8, 2900 as libc::c_int, str as *mut libc::c_uchar);
    QRinput_mergeBitStream(stream, bstream);
    version = QRinput_getVersion(stream);
    assertionNum += 1;
    assertionNum;
    if !(version == 40 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Version is %d (40 expected).\n\0" as *const u8 as *const libc::c_char,
            version,
        );
    }
    testFinish();
    QRinput_free(stream);
    BitStream_free(bstream);
    free(str as *mut libc::c_void);
}
unsafe extern "C" fn test_encodeAn() {
    let mut str: *mut libc::c_char = b"AC-42\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut correct: [libc::c_char; 42] = *::std::mem::transmute::<
        &[u8; 42],
        &mut [libc::c_char; 42],
    >(b"00100000001010011100111011100111001000010\0");
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_encodeAn\0"))
            .as_ptr(),
        b"Encoding alphabet-numeric stream.\0" as *const u8 as *const libc::c_char,
    );
    testEnd(
        mergeAndCheckBStream(0 as libc::c_int, QR_MODE_AN, str, correct.as_mut_ptr()),
    );
}
unsafe extern "C" fn test_encodeAn2() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut str: [libc::c_char; 6] = *::std::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"!,;$%\0");
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_encodeAn2\0"))
            .as_ptr(),
        b"Encoding INVALID alphabet-numeric stream.\0" as *const u8
            as *const libc::c_char,
    );
    stream = QRinput_new();
    ret = QRinput_append(
        stream,
        QR_MODE_AN,
        5 as libc::c_int,
        str.as_mut_ptr() as *mut libc::c_uchar,
    );
    testEnd((ret == 0) as libc::c_int);
    QRinput_free(stream);
}
unsafe extern "C" fn test_encodeNumeric() {
    let mut str: *mut libc::c_char = b"01234567\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut correct: [libc::c_char; 42] = *::std::mem::transmute::<
        &[u8; 42],
        &mut [libc::c_char; 42],
    >(b"00010000001000000000110001010110011000011\0");
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"test_encodeNumeric\0"))
            .as_ptr(),
        b"Encoding numeric stream. (8 digits)\0" as *const u8 as *const libc::c_char,
    );
    testEnd(
        mergeAndCheckBStream(0 as libc::c_int, QR_MODE_NUM, str, correct.as_mut_ptr()),
    );
}
unsafe extern "C" fn test_encodeNumeric_versionup() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut version: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 29],
            &[libc::c_char; 29],
        >(b"test_encodeNumeric_versionup\0"))
            .as_ptr(),
        b"Encoding numeric stream. (auto-version up test)\0" as *const u8
            as *const libc::c_char,
    );
    str = malloc(1050 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    memset(str as *mut libc::c_void, '1' as i32, 1050 as libc::c_int as libc::c_ulong);
    stream = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    bstream = BitStream_new();
    QRinput_append(stream, QR_MODE_NUM, 1050 as libc::c_int, str as *mut libc::c_uchar);
    QRinput_mergeBitStream(stream, bstream);
    version = QRinput_getVersion(stream);
    assertionNum += 1;
    assertionNum;
    if !(version == 14 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Version is %d (14 expected).\0" as *const u8 as *const libc::c_char,
            version,
        );
    }
    testFinish();
    QRinput_free(stream);
    BitStream_free(bstream);
    free(str as *mut libc::c_void);
}
unsafe extern "C" fn test_encodeNumericPadded() {
    let mut str: *mut libc::c_char = b"01234567\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut correct: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut correctHead: *mut libc::c_char = b"000100000010000000001100010101100110000110000000\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"test_encodeNumericPadded\0"))
            .as_ptr(),
        b"Encoding numeric stream. (8 digits)(padded)\0" as *const u8
            as *const libc::c_char,
    );
    correct = malloc(
        (19 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    *correct.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    strcat(correct, correctHead);
    i = 0 as libc::c_int;
    while i < 13 as libc::c_int {
        strcat(
            correct,
            if i & 1 as libc::c_int != 0 {
                b"00010001\0" as *const u8 as *const libc::c_char
            } else {
                b"11101100\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
        i;
    }
    ret = encodeAndCheckBStream(
        0 as libc::c_int,
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_NUM,
        str,
        correct,
    );
    testEnd(ret);
    free(correct as *mut libc::c_void);
}
unsafe extern "C" fn test_encodeNumericPadded2() {
    let mut str: *mut libc::c_char = b"0123456\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut correct: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut correctHead: *mut libc::c_char = b"000100000001110000001100010101100101100000000000\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"test_encodeNumericPadded2\0"))
            .as_ptr(),
        b"Encoding numeric stream. (7 digits)(padded)\0" as *const u8
            as *const libc::c_char,
    );
    correct = malloc(
        (19 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    *correct.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    strcat(correct, correctHead);
    i = 0 as libc::c_int;
    while i < 13 as libc::c_int {
        strcat(
            correct,
            if i & 1 as libc::c_int != 0 {
                b"00010001\0" as *const u8 as *const libc::c_char
            } else {
                b"11101100\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
        i;
    }
    ret = encodeAndCheckBStream(
        0 as libc::c_int,
        0 as libc::c_int,
        QR_ECLEVEL_L,
        QR_MODE_NUM,
        str,
        correct,
    );
    testEnd(ret);
    free(correct as *mut libc::c_void);
}
unsafe extern "C" fn test_padding() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut data: [libc::c_char; 18] = *::std::mem::transmute::<
        &[u8; 18],
        &mut [libc::c_char; 18],
    >(b"0123456789ABCDeFG\0");
    let mut c: libc::c_uchar = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_padding\0"))
            .as_ptr(),
        b"Padding bit check. (less than 5 bits)\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_8,
        17 as libc::c_int,
        data.as_mut_ptr() as *mut libc::c_uchar,
    );
    bstream = BitStream_new();
    QRinput_getBitStream(input, bstream);
    size = (*bstream).length as libc::c_int;
    assertionNum += 1;
    assertionNum;
    if !(size == 152 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"# of bit is incorrect (%d != 152).\n\0" as *const u8
                as *const libc::c_char,
            size,
        );
    }
    c = 0 as libc::c_int as libc::c_uchar;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        c = (c as libc::c_int
            + *((*bstream).data).offset((size - i - 1 as libc::c_int) as isize)
                as libc::c_int) as libc::c_uchar;
        i += 1;
        i;
    }
    assertionNum += 1;
    assertionNum;
    if !(c as libc::c_int == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Padding bits are not zero.\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    QRinput_free(input);
    BitStream_free(bstream);
}
unsafe extern "C" fn test_padding2() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut data: [libc::c_char; 17] = *::std::mem::transmute::<
        &[u8; 17],
        &mut [libc::c_char; 17],
    >(b"0123456789ABCDeF\0");
    let mut correct: [libc::c_char; 153] = [0; 153];
    let mut c: libc::c_uchar = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_padding2\0"))
            .as_ptr(),
        b"Padding bit check. (1 or 2 padding bytes)\0" as *const u8
            as *const libc::c_char,
    );
    memset(
        correct.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        153 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        correct.as_mut_ptr() as *mut libc::c_void,
        b"010000010000\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        12 as libc::c_int as libc::c_ulong,
    );
    size = 0 as libc::c_int;
    while size < 16 as libc::c_int {
        c = 0x80 as libc::c_int as libc::c_uchar;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            correct[(size * 8 as libc::c_int + i + 12 as libc::c_int)
                as usize] = (if data[size as usize] as libc::c_int & c as libc::c_int
                != 0
            {
                '1' as i32
            } else {
                '0' as i32
            }) as libc::c_char;
            c = (c as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            i += 1;
            i;
        }
        size += 1;
        size;
    }
    memcpy(
        correct.as_mut_ptr().offset(140 as libc::c_int as isize) as *mut libc::c_void,
        b"000011101100\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        12 as libc::c_int as libc::c_ulong,
    );
    input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_8,
        16 as libc::c_int,
        data.as_mut_ptr() as *mut libc::c_uchar,
    );
    bstream = BitStream_new();
    QRinput_getBitStream(input, bstream);
    size = (*bstream).length as libc::c_int;
    assertionNum += 1;
    assertionNum;
    if !(size == 152 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"16byte: # of bit is incorrect (%d != 152).\n\0" as *const u8
                as *const libc::c_char,
            size,
        );
    }
    ret = ncmpBin(correct.as_mut_ptr(), bstream, 152 as libc::c_int as size_t);
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Padding bits incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(input);
    BitStream_free(bstream);
    memcpy(
        correct.as_mut_ptr() as *mut libc::c_void,
        b"010000001111\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        12 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        correct.as_mut_ptr().offset(132 as libc::c_int as isize) as *mut libc::c_void,
        b"00001110110000010001\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_8,
        15 as libc::c_int,
        data.as_mut_ptr() as *mut libc::c_uchar,
    );
    bstream = BitStream_new();
    QRinput_getBitStream(input, bstream);
    size = (*bstream).length as libc::c_int;
    assertionNum += 1;
    assertionNum;
    if !(size == 152 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"15byte: # of bit is incorrect (%d != 152).\n\0" as *const u8
                as *const libc::c_char,
            size,
        );
    }
    ret = ncmpBin(correct.as_mut_ptr(), bstream, 152 as libc::c_int as size_t);
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Padding bits incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    QRinput_free(input);
    BitStream_free(bstream);
}
unsafe extern "C" fn test_encodeNumeric2() {
    let mut str: *mut libc::c_char = b"0123456789012345\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut correct: *mut libc::c_char = b"00010000010000000000110001010110011010100110111000010100111010100101\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_encodeNumeric2\0"))
            .as_ptr(),
        b"Encoding numeric stream. (16 digits)\0" as *const u8 as *const libc::c_char,
    );
    testEnd(mergeAndCheckBStream(0 as libc::c_int, QR_MODE_NUM, str, correct));
}
unsafe extern "C" fn test_encodeNumeric3() {
    let mut str: *mut libc::c_char = b"0123456\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut correct: *mut libc::c_char = b"0001 0000000111 0000001100 0101011001 0110\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_encodeNumeric3\0"))
            .as_ptr(),
        b"Encoding numeric stream. (7 digits)\0" as *const u8 as *const libc::c_char,
    );
    testEnd(mergeAndCheckBStream(0 as libc::c_int, QR_MODE_NUM, str, correct));
}
unsafe extern "C" fn test_encodeAnNum() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_encodeAnNum\0"))
            .as_ptr(),
        b"Bit length check of alpha-numeric stream. (11 + 12)\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new();
    QRinput_append(
        input,
        QR_MODE_AN,
        11 as libc::c_int,
        b"ABCDEFGHIJK\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    QRinput_append(
        input,
        QR_MODE_NUM,
        12 as libc::c_int,
        b"123456789012\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    bstream = BitStream_new();
    QRinput_mergeBitStream(input, bstream);
    testEnd(!((*bstream).length == 128 as libc::c_int as libc::c_ulong) as libc::c_int);
    QRinput_free(input);
    BitStream_free(bstream);
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_encodeAnNum\0"))
            .as_ptr(),
        b"Bit length check of alphabet stream. (23)\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new();
    QRinput_append(
        input,
        QR_MODE_AN,
        23 as libc::c_int,
        b"ABCDEFGHIJK123456789012\0" as *const u8 as *const libc::c_char
            as *mut libc::c_uchar,
    );
    bstream = BitStream_new();
    QRinput_mergeBitStream(input, bstream);
    testEnd(!((*bstream).length == 140 as libc::c_int as libc::c_ulong) as libc::c_int);
    QRinput_free(input);
    BitStream_free(bstream);
}
unsafe extern "C" fn test_struct_listop() {
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut inputs: [*mut QRinput; 5] = [0 as *mut QRinput; 5];
    let mut l: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"test_struct_listop\0"))
            .as_ptr(),
        b"QRinput_Struct list operation test.\0" as *const u8 as *const libc::c_char,
    );
    s = QRinput_Struct_new();
    QRinput_Struct_setParity(s, 10 as libc::c_int as libc::c_uchar);
    assertionNum += 1;
    assertionNum;
    if s.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"QRinput_Struct_new() failed.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*s).parity == 10 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_Struct_setParity() failed.\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        inputs[i as usize] = QRinput_new();
        QRinput_append(
            inputs[i as usize],
            QR_MODE_AN,
            5 as libc::c_int,
            b"ABCDE\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
        );
        ret = QRinput_Struct_appendInput(s, inputs[i as usize]);
        i += 1;
        i;
    }
    assertionNum += 1;
    assertionNum;
    if !(ret == 5 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_Struct_appendInput() returns wrong num?\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*s).size == 5 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRiput_Struct.size counts wrong number.\0" as *const u8
                as *const libc::c_char,
        );
    }
    l = (*s).head;
    i = 0 as libc::c_int;
    while !l.is_null() {
        assertionNum += 1;
        assertionNum;
        if !((*l).input == inputs[i as usize]) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"QRinput_Struct input list order would be wrong?\0" as *const u8
                    as *const libc::c_char,
            );
        }
        l = (*l).next;
        i += 1;
        i;
    }
    QRinput_Struct_free(s);
    testFinish();
}
unsafe extern "C" fn test_insertStructuredAppendHeader() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut correct: [libc::c_char; 41] = *::std::mem::transmute::<
        &[u8; 41],
        &mut [libc::c_char; 41],
    >(b"0011000011111010010101000000000101000001\0");
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 34],
            &[libc::c_char; 34],
        >(b"test_insertStructuredAppendHeader\0"))
            .as_ptr(),
        b"Insert a structured-append header\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    bstream = BitStream_new();
    QRinput_append(
        stream,
        QR_MODE_8,
        1 as libc::c_int,
        b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    ret = QRinput_insertStructuredAppendHeader(
        stream,
        16 as libc::c_int,
        1 as libc::c_int,
        0xa5 as libc::c_int as libc::c_uchar,
    );
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_insertStructuredAppendHeader() returns nonzero.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    QRinput_mergeBitStream(stream, bstream);
    assertionNum += 1;
    assertionNum;
    if ((*bstream).data).is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Bstream->data is null.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(correct.as_mut_ptr(), bstream) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"bitstream is wrong.\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    QRinput_free(stream);
    BitStream_free(bstream);
}
unsafe extern "C" fn test_insertStructuredAppendHeader_error() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 40],
            &[libc::c_char; 40],
        >(b"test_insertStructuredAppendHeader_error\0"))
            .as_ptr(),
        b"Insert a structured-append header (errors expected)\0" as *const u8
            as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_append(
        stream,
        QR_MODE_8,
        1 as libc::c_int,
        b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    ret = QRinput_insertStructuredAppendHeader(
        stream,
        17 as libc::c_int,
        1 as libc::c_int,
        0xa5 as libc::c_int as libc::c_uchar,
    );
    assertionNum += 1;
    assertionNum;
    if !(-(1 as libc::c_int) == ret) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_insertStructuredAppendHeader() returns 0.\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(22 as libc::c_int == *__errno_location()) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"errno is not set correctly (%d returned).\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
    }
    ret = QRinput_insertStructuredAppendHeader(
        stream,
        16 as libc::c_int,
        17 as libc::c_int,
        0xa5 as libc::c_int as libc::c_uchar,
    );
    assertionNum += 1;
    assertionNum;
    if !(-(1 as libc::c_int) == ret) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_insertStructuredAppendHeader() returns 0.\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(22 as libc::c_int == *__errno_location()) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"errno is not set correctly (%d returned).\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
    }
    ret = QRinput_insertStructuredAppendHeader(
        stream,
        16 as libc::c_int,
        0 as libc::c_int,
        0xa5 as libc::c_int as libc::c_uchar,
    );
    assertionNum += 1;
    assertionNum;
    if !(-(1 as libc::c_int) == ret) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_insertStructuredAppendHeader() returns 0.\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(22 as libc::c_int == *__errno_location()) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"errno is not set correctly (%d returned).\0" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
    }
    testFinish();
    QRinput_free(stream);
}
unsafe extern "C" fn test_struct_insertStructuredAppendHeaders() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut p: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"test_struct_insertStructuredAppendHeaders\0"))
            .as_ptr(),
        b"Insert structured-append headers to a QRinput_Struct.\0" as *const u8
            as *const libc::c_char,
    );
    s = QRinput_Struct_new();
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        input = QRinput_new();
        QRinput_append(
            input,
            QR_MODE_8,
            1 as libc::c_int,
            b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
        );
        QRinput_Struct_appendInput(s, input);
        i += 1;
        i;
    }
    QRinput_Struct_insertStructuredAppendHeaders(s);
    p = (*s).head;
    i = 1 as libc::c_int;
    while !p.is_null() {
        assertionNum += 1;
        assertionNum;
        if !((*(*(*p).input).head).mode as libc::c_int
            == QR_MODE_STRUCTURE as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"a structured-append header is not inserted.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(*((*(*(*p).input).head).data).offset(0 as libc::c_int as isize)
            as libc::c_int == 10 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"size of the structured-header is wrong: #%d, %d should be %d\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                *((*(*(*p).input).head).data).offset(0 as libc::c_int as isize)
                    as libc::c_int,
                10 as libc::c_int,
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(*((*(*(*p).input).head).data).offset(1 as libc::c_int as isize)
            as libc::c_int == i)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"index of the structured-header is wrong: #%d, %d should be %d\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                *((*(*(*p).input).head).data).offset(1 as libc::c_int as isize)
                    as libc::c_int,
                i,
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(*((*(*(*p).input).head).data).offset(2 as libc::c_int as isize)
            as libc::c_int == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"parity of the structured-header is wrong: #%d\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
        }
        p = (*p).next;
        i += 1;
        i;
    }
    testFinish();
    QRinput_Struct_free(s);
}
unsafe extern "C" fn check_lengthOfCode(
    mut mode: QRencodeMode,
    mut data: *mut libc::c_char,
    mut size: libc::c_int,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut b: *mut BitStream = 0 as *mut BitStream;
    let mut bits: size_t = 0;
    let mut bytes: libc::c_int = 0;
    input = QRinput_new();
    QRinput_setVersion(input, version);
    QRinput_append(input, mode, size, data as *mut libc::c_uchar);
    b = BitStream_new();
    QRinput_mergeBitStream(input, b);
    bits = (*b).length;
    bytes = QRinput_lengthOfCode(mode, version, bits as libc::c_int);
    QRinput_free(input);
    BitStream_free(b);
    return bytes;
}
unsafe extern "C" fn test_lengthOfCode_num() {
    let mut i: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = malloc(8000 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 8000 as libc::c_int {
        *data.offset(i as isize) = ('0' as i32 + i % 10 as libc::c_int) as libc::c_char;
        i += 1;
        i;
    }
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"test_lengthOfCode_num\0"))
            .as_ptr(),
        b"Checking length of code (numeric)\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= 9 as libc::c_int {
        bytes = check_lengthOfCode(QR_MODE_NUM, data, i, 1 as libc::c_int);
        assertionNum += 1;
        assertionNum;
        if !(i == bytes) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"lengthOfCode failed. (QR_MODE_NUM, version:1, size:%d)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
            );
        }
        i += 1;
        i;
    }
    i = 1023 as libc::c_int;
    while i <= 1025 as libc::c_int {
        bytes = check_lengthOfCode(QR_MODE_NUM, data, i, 1 as libc::c_int);
        assertionNum += 1;
        assertionNum;
        if !(1023 as libc::c_int == bytes) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"lengthOfCode failed. (QR_MODE_NUM, version:1, size:%d)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
            );
        }
        i += 1;
        i;
    }
    testFinish();
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn test_lengthOfCode_kanji() {
    let mut i: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut str: [libc::c_uchar; 12] = [
        0x93 as libc::c_int as libc::c_uchar,
        0x5f as libc::c_int as libc::c_uchar,
        0xe4 as libc::c_int as libc::c_uchar,
        0xaa as libc::c_int as libc::c_uchar,
        0x81 as libc::c_int as libc::c_uchar,
        0x40 as libc::c_int as libc::c_uchar,
        0x9f as libc::c_int as libc::c_uchar,
        0xfc as libc::c_int as libc::c_uchar,
        0xe0 as libc::c_int as libc::c_uchar,
        0x40 as libc::c_int as libc::c_uchar,
        0xeb as libc::c_int as libc::c_uchar,
        0xbf as libc::c_int as libc::c_uchar,
    ];
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"test_lengthOfCode_kanji\0"))
            .as_ptr(),
        b"Checking length of code (kanji)\0" as *const u8 as *const libc::c_char,
    );
    i = 2 as libc::c_int;
    while i <= 12 as libc::c_int {
        bytes = check_lengthOfCode(
            QR_MODE_KANJI,
            str.as_mut_ptr() as *mut libc::c_char,
            i,
            1 as libc::c_int,
        );
        assertionNum += 1;
        assertionNum;
        if !(i == bytes) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"lengthOfCode failed. (QR_MODE_KANJI, version:1, size:%d)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
            );
        }
        i += 2 as libc::c_int;
    }
    testFinish();
}
unsafe extern "C" fn test_struct_split_example() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut e: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut l: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut str: [*const libc::c_char; 4] = [
        b"an example \0" as *const u8 as *const libc::c_char,
        b"of four Str\0" as *const u8 as *const libc::c_char,
        b"uctured Appe\0" as *const u8 as *const libc::c_char,
        b"nd symbols,\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"test_struct_split_example\0"))
            .as_ptr(),
        b"Testing the example of structured-append symbols\0" as *const u8
            as *const libc::c_char,
    );
    s = QRinput_Struct_new();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_M);
        QRinput_append(
            input,
            QR_MODE_8,
            strlen(str[i as usize]) as libc::c_int,
            str[i as usize] as *mut libc::c_uchar,
        );
        QRinput_Struct_appendInput(s, input);
        i += 1;
        i;
    }
    QRinput_Struct_insertStructuredAppendHeaders(s);
    e = (*s).head;
    i = 0 as libc::c_int;
    while !e.is_null() {
        bstream = BitStream_new();
        QRinput_mergeBitStream((*e).input, bstream);
        BitStream_free(bstream);
        l = (*(*(*e).input).head).next;
        assertionNum += 1;
        assertionNum;
        if !((*l).mode as libc::c_int == QR_MODE_8 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"#%d: wrong mode (%d).\n\0" as *const u8 as *const libc::c_char,
                i,
                (*l).mode as libc::c_int,
            );
        }
        assertionNum += 1;
        assertionNum;
        if !((*(*e).input).level as libc::c_uint
            == QR_ECLEVEL_M as libc::c_int as libc::c_uint)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"#%d: wrong level (%d).\n\0" as *const u8 as *const libc::c_char,
                i,
                (*(*e).input).level as libc::c_uint,
            );
        }
        e = (*e).next;
        i += 1;
        i;
    }
    testFinish();
    QRinput_Struct_free(s);
}
unsafe extern "C" fn test_struct_split_tooLarge() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errsv: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 27],
            &[libc::c_char; 27],
        >(b"test_struct_split_tooLarge\0"))
            .as_ptr(),
        b"Testing structured-append symbols. (too large data)\0" as *const u8
            as *const libc::c_char,
    );
    str = malloc(128 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    memset(str as *mut libc::c_void, 'a' as i32, 128 as libc::c_int as libc::c_ulong);
    input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_H);
    QRinput_append(input, QR_MODE_8, 128 as libc::c_int, str as *mut libc::c_uchar);
    s = QRinput_splitQRinputToStruct(input);
    errsv = *__errno_location();
    assertionNum += 1;
    assertionNum;
    if !s.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"returns non-null.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(errsv == 34 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"did not return ERANGE.\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    if !s.is_null() {
        QRinput_Struct_free(s);
    }
    QRinput_free(input);
    free(str as *mut libc::c_void);
}
unsafe extern "C" fn test_struct_split_invalidVersion() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errsv: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 33],
            &[libc::c_char; 33],
        >(b"test_struct_split_invalidVersion\0"))
            .as_ptr(),
        b"Testing structured-append symbols. (invalid version 0)\0" as *const u8
            as *const libc::c_char,
    );
    str = malloc(128 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    memset(str as *mut libc::c_void, 'a' as i32, 128 as libc::c_int as libc::c_ulong);
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_H);
    QRinput_append(input, QR_MODE_8, 128 as libc::c_int, str as *mut libc::c_uchar);
    s = QRinput_splitQRinputToStruct(input);
    errsv = *__errno_location();
    assertionNum += 1;
    assertionNum;
    if !s.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"returns non-null.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(errsv == 34 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"did not return ERANGE.\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    if !s.is_null() {
        QRinput_Struct_free(s);
    }
    QRinput_free(input);
    free(str as *mut libc::c_void);
}
unsafe extern "C" fn test_struct_singlestructure() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut str: *mut libc::c_char = b"TEST\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 28],
            &[libc::c_char; 28],
        >(b"test_struct_singlestructure\0"))
            .as_ptr(),
        b"Testing structured-append symbols. (single structure)\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new2(10 as libc::c_int, QR_ECLEVEL_H);
    QRinput_append(
        input,
        QR_MODE_AN,
        strlen(str) as libc::c_int,
        str as *mut libc::c_uchar,
    );
    s = QRinput_splitQRinputToStruct(input);
    assertionNum += 1;
    assertionNum;
    if s.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"must return a code.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*s).size == 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"size must be 1, but %d returned.\0" as *const u8 as *const libc::c_char,
            (*s).size,
        );
    }
    if (*s).size != 1 as libc::c_int {
        printQRinputStruct(s);
    }
    testFinish();
    if !s.is_null() {
        QRinput_Struct_free(s);
    }
    QRinput_free(input);
}
unsafe extern "C" fn test_splitentry() {
    let mut i1: *mut QRinput = 0 as *mut QRinput;
    let mut i2: *mut QRinput = 0 as *mut QRinput;
    let mut e: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut str: *const libc::c_char = b"abcdefghij\0" as *const u8
        as *const libc::c_char;
    let mut size1: libc::c_int = 0;
    let mut size2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut d1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut d2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_splitentry\0"))
            .as_ptr(),
        b"Testing QRinput_splitEntry. (next == NULL)\0" as *const u8
            as *const libc::c_char,
    );
    i1 = QRinput_new();
    QRinput_append(i1, QR_MODE_8, strlen(str) as libc::c_int, str as *mut libc::c_uchar);
    i2 = QRinput_dup(i1);
    e = (*i2).head;
    QRinput_splitEntry(e, 4 as libc::c_int);
    size2 = 0 as libc::c_int;
    size1 = size2;
    e = (*i1).head;
    while !e.is_null() {
        size1 += (*e).size;
        e = (*e).next;
    }
    e = (*i2).head;
    while !e.is_null() {
        size2 += (*e).size;
        e = (*e).next;
    }
    d1 = malloc(size1 as libc::c_ulong) as *mut libc::c_uchar;
    e = (*i1).head;
    i = 0 as libc::c_int;
    while !e.is_null() {
        memcpy(
            &mut *d1.offset(i as isize) as *mut libc::c_uchar as *mut libc::c_void,
            (*e).data as *const libc::c_void,
            (*e).size as libc::c_ulong,
        );
        i += (*e).size;
        e = (*e).next;
    }
    d2 = malloc(size2 as libc::c_ulong) as *mut libc::c_uchar;
    e = (*i2).head;
    i = 0 as libc::c_int;
    while !e.is_null() {
        memcpy(
            &mut *d2.offset(i as isize) as *mut libc::c_uchar as *mut libc::c_void,
            (*e).data as *const libc::c_void,
            (*e).size as libc::c_ulong,
        );
        i += (*e).size;
        e = (*e).next;
    }
    assertionNum += 1;
    assertionNum;
    if !(size1 == size2) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"sizes are different. (%d:%d)\n\0" as *const u8 as *const libc::c_char,
            size1,
            size2,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*(*i2).head).size == 4 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"split failed (first half)\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*(*(*i2).head).next).size == 6 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"split failed(second half)\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(memcmp(
        d1 as *const libc::c_void,
        d2 as *const libc::c_void,
        size1 as libc::c_ulong,
    ) == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"strings are different.\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(i1);
    QRinput_free(i2);
    free(d1 as *mut libc::c_void);
    free(d2 as *mut libc::c_void);
    testFinish();
}
unsafe extern "C" fn test_splitentry2() {
    let mut i1: *mut QRinput = 0 as *mut QRinput;
    let mut i2: *mut QRinput = 0 as *mut QRinput;
    let mut e: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut str: *const libc::c_char = b"abcdefghij\0" as *const u8
        as *const libc::c_char;
    let mut size1: libc::c_int = 0;
    let mut size2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut d1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut d2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_splitentry2\0"))
            .as_ptr(),
        b"Testing QRinput_splitEntry. (next != NULL)\0" as *const u8
            as *const libc::c_char,
    );
    i1 = QRinput_new();
    QRinput_append(i1, QR_MODE_8, strlen(str) as libc::c_int, str as *mut libc::c_uchar);
    QRinput_append(i1, QR_MODE_8, strlen(str) as libc::c_int, str as *mut libc::c_uchar);
    i2 = QRinput_dup(i1);
    e = (*i2).head;
    QRinput_splitEntry(e, 4 as libc::c_int);
    size2 = 0 as libc::c_int;
    size1 = size2;
    e = (*i1).head;
    while !e.is_null() {
        size1 += (*e).size;
        e = (*e).next;
    }
    e = (*i2).head;
    while !e.is_null() {
        size2 += (*e).size;
        e = (*e).next;
    }
    d1 = malloc(size1 as libc::c_ulong) as *mut libc::c_uchar;
    e = (*i1).head;
    i = 0 as libc::c_int;
    while !e.is_null() {
        memcpy(
            &mut *d1.offset(i as isize) as *mut libc::c_uchar as *mut libc::c_void,
            (*e).data as *const libc::c_void,
            (*e).size as libc::c_ulong,
        );
        i += (*e).size;
        e = (*e).next;
    }
    d2 = malloc(size2 as libc::c_ulong) as *mut libc::c_uchar;
    e = (*i2).head;
    i = 0 as libc::c_int;
    while !e.is_null() {
        memcpy(
            &mut *d2.offset(i as isize) as *mut libc::c_uchar as *mut libc::c_void,
            (*e).data as *const libc::c_void,
            (*e).size as libc::c_ulong,
        );
        i += (*e).size;
        e = (*e).next;
    }
    assertionNum += 1;
    assertionNum;
    if !(size1 == size2) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"sizes are different. (%d:%d)\n\0" as *const u8 as *const libc::c_char,
            size1,
            size2,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*(*i2).head).size == 4 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"split failed (first half)\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*(*(*i2).head).next).size == 6 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"split failed(second half)\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(memcmp(
        d1 as *const libc::c_void,
        d2 as *const libc::c_void,
        size1 as libc::c_ulong,
    ) == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"strings are different.\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(i1);
    QRinput_free(i2);
    free(d1 as *mut libc::c_void);
    free(d2 as *mut libc::c_void);
    testFinish();
}
unsafe extern "C" fn test_splitentry3() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut e00: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut e01: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut e10: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut e11: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut str: *const libc::c_char = b"abcdefghijklmno\0" as *const u8
        as *const libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_splitentry3\0"))
            .as_ptr(),
        b"Testing QRinput_splitEntry. (does not split an entry)\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_8,
        strlen(str) as libc::c_int,
        str as *mut libc::c_uchar,
    );
    QRinput_append(
        input,
        QR_MODE_8,
        strlen(str) as libc::c_int,
        str as *mut libc::c_uchar,
    );
    s = QRinput_splitQRinputToStruct(input);
    list = (*s).head;
    e00 = (*(*list).input).head;
    e01 = (*e00).next;
    list = (*list).next;
    e10 = (*(*list).input).head;
    e11 = (*e10).next;
    assertionNum += 1;
    assertionNum;
    if !((*e00).mode as libc::c_int == QR_MODE_STRUCTURE as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Structure header is missing?\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*e01).mode as libc::c_int == QR_MODE_8 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"no data?!\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*e01).next).is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Input list is not terminated!\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*e10).mode as libc::c_int == QR_MODE_STRUCTURE as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Structure header is missing?\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*e11).mode as libc::c_int == QR_MODE_8 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"no data?!\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !((*e11).next).is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Input list is not terminated!\n\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(input);
    QRinput_Struct_free(s);
    testFinish();
}
unsafe extern "C" fn test_parity() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut text: *const libc::c_char = b"an example of four Structured Append symbols,\0"
        as *const u8 as *const libc::c_char;
    let mut str: [*const libc::c_char; 4] = [
        b"an example \0" as *const u8 as *const libc::c_char,
        b"of four Str\0" as *const u8 as *const libc::c_char,
        b"uctured Appe\0" as *const u8 as *const libc::c_char,
        b"nd symbols,\0" as *const u8 as *const libc::c_char,
    ];
    let mut p1: libc::c_uchar = 0;
    let mut p2: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_parity\0"))
            .as_ptr(),
        b"Testing parity calc.\0" as *const u8 as *const libc::c_char,
    );
    s = QRinput_Struct_new();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_M);
        QRinput_append(
            input,
            QR_MODE_8,
            strlen(str[i as usize]) as libc::c_int,
            str[i as usize] as *mut libc::c_uchar,
        );
        QRinput_Struct_appendInput(s, input);
        i += 1;
        i;
    }
    QRinput_Struct_insertStructuredAppendHeaders(s);
    p1 = (*s).parity as libc::c_uchar;
    p2 = 0 as libc::c_int as libc::c_uchar;
    len = strlen(text) as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        p2 = (p2 as libc::c_int ^ *text.offset(i as isize) as libc::c_int)
            as libc::c_uchar;
        i += 1;
        i;
    }
    assertionNum += 1;
    assertionNum;
    if !(p1 as libc::c_int == p2 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Parity numbers didn't match. (%02x should be %02x).\n\0" as *const u8
                as *const libc::c_char,
            p1 as libc::c_int,
            p2 as libc::c_int,
        );
    }
    testFinish();
    QRinput_Struct_free(s);
}
unsafe extern "C" fn test_parity2() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut text: *const libc::c_char = b"an example of four Structured Append symbols,\0"
        as *const u8 as *const libc::c_char;
    let mut p1: libc::c_uchar = 0;
    let mut p2: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_parity2\0"))
            .as_ptr(),
        b"Testing parity calc.(split)\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_L);
    QRinput_append(
        input,
        QR_MODE_8,
        strlen(text) as libc::c_int,
        text as *mut libc::c_uchar,
    );
    s = QRinput_splitQRinputToStruct(input);
    p1 = (*s).parity as libc::c_uchar;
    p2 = 0 as libc::c_int as libc::c_uchar;
    len = strlen(text) as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        p2 = (p2 as libc::c_int ^ *text.offset(i as isize) as libc::c_int)
            as libc::c_uchar;
        i += 1;
        i;
    }
    assertionNum += 1;
    assertionNum;
    if !(p1 as libc::c_int == p2 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Parity numbers didn't match. (%02x should be %02x).\n\0" as *const u8
                as *const libc::c_char,
            p1 as libc::c_int,
            p2 as libc::c_int,
        );
    }
    testFinish();
    QRinput_free(input);
    QRinput_Struct_free(s);
}
unsafe extern "C" fn test_null_free() {
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_null_free\0"))
            .as_ptr(),
        b"Testing free NULL pointers\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Check QRinput_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRinput_free(0 as *mut QRinput);
    printf(b"Check QRinput_Struct_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    QRinput_Struct_free(0 as *mut QRinput_Struct);
    testFinish();
}
unsafe extern "C" fn fillCharacter(
    mut dest: *mut libc::c_char,
    mut ch: libc::c_char,
    mut size: libc::c_int,
) {
    memset(dest as *mut libc::c_void, ch as libc::c_int, size as libc::c_ulong);
    *dest.offset(size as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn checkEstimatedVersion(mut ver: libc::c_int, mut mode: libc::c_int) {
    let mut estimatedVersion: libc::c_int = 0;
    let mut data: [libc::c_char; 7200] = [0; 7200];
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut hint: QRencodeMode = QR_MODE_NUM;
    let mut size1: libc::c_int = 0;
    let mut size2: libc::c_int = 0;
    static mut modeStr: [*mut libc::c_char; 4] = [
        b"numeric\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"alphanumeric\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"8 bit data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"kanji\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    static mut ch: [libc::c_char; 4] = [
        '0' as i32 as libc::c_char,
        'A' as i32 as libc::c_char,
        'a' as i32 as libc::c_char,
        -110i32 as libc::c_char,
    ];
    if mode == QR_MODE_KANJI as libc::c_int {
        hint = QR_MODE_KANJI;
        size1 = maxCharacterLengths[(ver - 1 as libc::c_int) as usize][mode as usize]
            * 2 as libc::c_int;
        size2 = size1 + 2 as libc::c_int;
    } else {
        hint = QR_MODE_8;
        size1 = maxCharacterLengths[(ver - 1 as libc::c_int) as usize][mode as usize];
        size2 = size1 + 1 as libc::c_int;
    }
    fillCharacter(data.as_mut_ptr(), ch[mode as usize], size1);
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(data.as_mut_ptr(), input, hint, 1 as libc::c_int);
    estimatedVersion = QRinput_estimateVersion(input);
    assertionNum += 1;
    assertionNum;
    if !(estimatedVersion == ver) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Estimated version %d is not equal to the expected version %d for %d %s sequence.\n\0"
                as *const u8 as *const libc::c_char,
            estimatedVersion,
            ver,
            maxCharacterLengths[(ver - 1 as libc::c_int) as usize][mode as usize],
            modeStr[mode as usize],
        );
    }
    QRinput_free(input);
    fillCharacter(data.as_mut_ptr(), ch[mode as usize], size2);
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(data.as_mut_ptr(), input, hint, 1 as libc::c_int);
    estimatedVersion = QRinput_estimateVersion(input);
    assertionNum += 1;
    assertionNum;
    if !(estimatedVersion == ver + 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Estimated version %d is not equal to the expected version %d for %d %s sequence.\n\0"
                as *const u8 as *const libc::c_char,
            estimatedVersion,
            ver,
            maxCharacterLengths[(ver - 1 as libc::c_int) as usize][mode as usize]
                + 1 as libc::c_int,
            modeStr[mode as usize],
        );
    }
    QRinput_free(input);
}
unsafe extern "C" fn test_estimateVersionBoundaryCheck() {
    let mut ver: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 34],
            &[libc::c_char; 34],
        >(b"test_estimateVersionBoundaryCheck\0"))
            .as_ptr(),
        b"Boundary check of estimateVersion\0" as *const u8 as *const libc::c_char,
    );
    ver = 1 as libc::c_int;
    while ver < 40 as libc::c_int {
        checkEstimatedVersion(ver, QR_MODE_NUM as libc::c_int);
        checkEstimatedVersion(ver, QR_MODE_AN as libc::c_int);
        checkEstimatedVersion(ver, QR_MODE_8 as libc::c_int);
        checkEstimatedVersion(ver, QR_MODE_KANJI as libc::c_int);
        ver += 1;
        ver;
    }
    testFinish();
}
unsafe extern "C" fn test_QRinput_new_invalid() {
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"test_QRinput_new_invalid\0"))
            .as_ptr(),
        b"Invalid input to QRinput_new2()\0" as *const u8 as *const libc::c_char,
    );
    let mut input: *mut QRinput = 0 as *mut QRinput;
    input = QRinput_new2(-(1 as libc::c_int), QR_ECLEVEL_H);
    assertionNum += 1;
    assertionNum;
    if !input.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_new2() returns non-null for invalid version (-1).\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(*__errno_location() == 22 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Error code is not EINVAL.\n\0" as *const u8 as *const libc::c_char);
    }
    input = QRinput_new2(41 as libc::c_int, QR_ECLEVEL_H);
    assertionNum += 1;
    assertionNum;
    if !input.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_new2() returns non-null for invalid version (41).\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(*__errno_location() == 22 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Error code is not EINVAL.\n\0" as *const u8 as *const libc::c_char);
    }
    input = QRinput_new2(1 as libc::c_int, 4294967295 as QRecLevel);
    assertionNum += 1;
    assertionNum;
    if !input.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_new2() returns non-null for invalid level (-1).\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(*__errno_location() == 22 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Error code is not EINVAL.\n\0" as *const u8 as *const libc::c_char);
    }
    input = QRinput_new2(1 as libc::c_int, 5 as QRecLevel);
    assertionNum += 1;
    assertionNum;
    if !input.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_new2() returns non-null for invalid level (5).\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(*__errno_location() == 22 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Error code is not EINVAL.\n\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
}
unsafe extern "C" fn test_QRinput_getErrorCorrectionLevel() {
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 37],
            &[libc::c_char; 37],
        >(b"test_QRinput_getErrorCorrectionLevel\0"))
            .as_ptr(),
        b"Invalid input to QRinput_getErrorCorrectionLevel()\0" as *const u8
            as *const libc::c_char,
    );
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    input = QRinput_new2(1 as libc::c_int, QR_ECLEVEL_H);
    level = QRinput_getErrorCorrectionLevel(input);
    assertionNum += 1;
    assertionNum;
    if !(level as libc::c_uint == QR_ECLEVEL_H as libc::c_int as libc::c_uint) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_getErrorCorrectionLevel() fails to return expected level.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    testFinish();
}
unsafe extern "C" fn test_mqr_new() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_mqr_new\0"))
            .as_ptr(),
        b"Testing QRinput_newMQR().\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_newMQR(0 as libc::c_int, QR_ECLEVEL_L);
    assertionNum += 1;
    assertionNum;
    if !input.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Version 0 passed.\n\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(input);
    input = QRinput_newMQR(5 as libc::c_int, QR_ECLEVEL_L);
    assertionNum += 1;
    assertionNum;
    if !input.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Version 5 passed.\n\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(input);
    input = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_M);
    assertionNum += 1;
    assertionNum;
    if !input.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Invalid ECLEVEL passed.\n\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(input);
    input = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    assertionNum += 1;
    assertionNum;
    if !((*input).version == 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput.version was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*input).level as libc::c_uint == QR_ECLEVEL_L as libc::c_int as libc::c_uint) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput.version was not as expected.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_mqr_setversion() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_mqr_setversion\0"))
            .as_ptr(),
        b"Testing QRinput_setVersion() for MQR.\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    ret = QRinput_setVersion(input, 2 as libc::c_int);
    assertionNum += 1;
    assertionNum;
    if !(ret < 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_setVersion should be denied.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_mqr_setlevel() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"test_mqr_setlevel\0"))
            .as_ptr(),
        b"Testing QRinput_setErrorCorrectionLevel() for MQR.\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_newMQR(1 as libc::c_int, QR_ECLEVEL_L);
    ret = QRinput_setErrorCorrectionLevel(input, QR_ECLEVEL_M);
    assertionNum += 1;
    assertionNum;
    if !(ret < 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRinput_setErrorCorrectionLevel should be denied.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    QRinput_free(input);
    testFinish();
}
unsafe extern "C" fn test_paddingMQR() {
    let mut dataM1: [*mut libc::c_char; 4] = [
        b"65\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"513\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"5139\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"51365\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut correctM1: [*mut libc::c_char; 4] = [
        b"01010000010000000000\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"01110000000010000000\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"10010000000011001000\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"10110000000011000001\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ];
    let mut dataM2: [*mut libc::c_char; 2] = [
        b"513513\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"51351365\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut correctM2: [*mut libc::c_char; 2] = [
        b"0 0110 1000000001 1000000001 0000000\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"0 1000 1000000001 1000000001 1000001\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_paddingMQR\0"))
            .as_ptr(),
        b"Padding bit check of MQR. (only 0 padding)\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        ret = encodeAndCheckBStream(
            1 as libc::c_int,
            1 as libc::c_int,
            QR_ECLEVEL_L,
            QR_MODE_NUM,
            dataM1[i as usize],
            correctM1[i as usize],
        );
        assertionNum += 1;
        assertionNum;
        if !(ret == 0 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Number %s incorrectly encoded.\n\0" as *const u8
                    as *const libc::c_char,
                dataM1[i as usize],
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        ret = encodeAndCheckBStream(
            1 as libc::c_int,
            2 as libc::c_int,
            QR_ECLEVEL_M,
            QR_MODE_NUM,
            dataM2[i as usize],
            correctM2[i as usize],
        );
        assertionNum += 1;
        assertionNum;
        if !(ret == 0 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Number %s incorrectly encoded.\n\0" as *const u8
                    as *const libc::c_char,
                dataM2[i as usize],
            );
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_padding2MQR() {
    let mut data: [*mut libc::c_char; 4] = [
        b"9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"513513\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"513\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"513\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut ver: [libc::c_int; 4] = [
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    let mut correct: [*mut libc::c_char; 4] = [
        b"00110010 00000000 0000\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"0 0110 1000000001 1000000001 0000000 11101100\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"0 0011 1000000001 000000000 11101100 00010001\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"00 00011 1000000001 0000000 11101100 00010001 11101100 00010001 11101100 00010001 11101100 0000\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_padding2MQR\0"))
            .as_ptr(),
        b"Padding bit check. (1 or 2 padding bytes)\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        ret = encodeAndCheckBStream(
            1 as libc::c_int,
            ver[i as usize],
            QR_ECLEVEL_L,
            QR_MODE_NUM,
            data[i as usize],
            correct[i as usize],
        );
        assertionNum += 1;
        assertionNum;
        if !(ret == 0 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Number %s incorrectly encoded.\n\0" as *const u8
                    as *const libc::c_char,
                data[i as usize],
            );
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_textMQR() {
    let mut version: libc::c_int = 3 as libc::c_int;
    let mut level: QRecLevel = QR_ECLEVEL_M;
    let mut str: *mut libc::c_char = b"MICROQR\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut correct: *mut libc::c_char = b"01 0111 01111110000 01000110111 10001010010 011011 0000000 0000 11101100 0000\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_textMQR\0"))
            .as_ptr(),
        b"Text encoding (Micro QR)\0" as *const u8 as *const libc::c_char,
    );
    ret = encodeAndCheckBStream(
        1 as libc::c_int,
        version,
        level,
        QR_MODE_AN,
        str,
        correct,
    );
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"AlphaNumeric string '%s' incorrectly encoded.\n\0" as *const u8
                as *const libc::c_char,
            str,
        );
    }
    testFinish();
}
unsafe extern "C" fn test_ECIinvalid() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_ECIinvalid\0"))
            .as_ptr(),
        b"Appending invalid ECI header\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    ret = QRinput_appendECIheader(stream, 999999 as libc::c_int as libc::c_uint);
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Valid ECI header rejected.\0" as *const u8 as *const libc::c_char);
    }
    ret = QRinput_appendECIheader(stream, 1000000 as libc::c_int as libc::c_uint);
    assertionNum += 1;
    assertionNum;
    if !(ret != 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Invalid ECI header accepted.\0" as *const u8 as *const libc::c_char);
    }
    QRinput_free(stream);
    testFinish();
}
unsafe extern "C" fn test_encodeECI() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut str: [libc::c_uchar; 5] = [
        0xa1 as libc::c_int as libc::c_uchar,
        0xa2 as libc::c_int as libc::c_uchar,
        0xa3 as libc::c_int as libc::c_uchar,
        0xa4 as libc::c_int as libc::c_uchar,
        0xa5 as libc::c_int as libc::c_uchar,
    ];
    let mut correct: *mut libc::c_char = b"0111 00001001 0100 00000101 10100001 10100010 10100011 10100100 10100101\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_encodeECI\0"))
            .as_ptr(),
        b"Encoding characters with ECI header.\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new();
    ret = QRinput_appendECIheader(input, 9 as libc::c_int as libc::c_uint);
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Valid ECI header rejected.\n\0" as *const u8 as *const libc::c_char);
    }
    ret = QRinput_append(input, QR_MODE_8, 5 as libc::c_int, str.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to append characters.\n\0" as *const u8 as *const libc::c_char);
    }
    bstream = BitStream_new();
    QRinput_mergeBitStream(input, bstream);
    assertionNum += 1;
    assertionNum;
    if bstream.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to merge.\n\0" as *const u8 as *const libc::c_char);
    }
    if !bstream.is_null() {
        ret = ncmpBin(correct, bstream, 64 as libc::c_int as size_t);
        assertionNum += 1;
        assertionNum;
        if !(ret == 0 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Encodation of ECI header was invalid.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        BitStream_free(bstream);
    }
    QRinput_free(input);
    testFinish();
}
unsafe fn main_0() -> libc::c_int {
    let mut tests: libc::c_int = 42 as libc::c_int;
    testInit(tests);
    test_encodeNumeric();
    test_encodeNumeric2();
    test_encodeNumeric3();
    test_encodeNumeric_versionup();
    test_encode8();
    test_encode8_versionup();
    test_encodeAn();
    test_encodeAn2();
    test_encodeKanji();
    test_encodeNumericPadded();
    test_encodeNumericPadded2();
    test_encodeAnNum();
    test_padding();
    test_padding2();
    test_struct_listop();
    test_insertStructuredAppendHeader();
    test_insertStructuredAppendHeader_error();
    test_struct_insertStructuredAppendHeaders();
    test_lengthOfCode_num();
    test_lengthOfCode_kanji();
    test_splitentry();
    test_splitentry2();
    test_splitentry3();
    test_struct_split_example();
    test_struct_split_tooLarge();
    test_struct_split_invalidVersion();
    test_struct_singlestructure();
    test_parity();
    test_parity2();
    test_null_free();
    test_estimateVersionBoundaryCheck();
    test_QRinput_new_invalid();
    test_QRinput_getErrorCorrectionLevel();
    test_mqr_new();
    test_mqr_setversion();
    test_mqr_setlevel();
    test_paddingMQR();
    test_padding2MQR();
    test_textMQR();
    test_ECIinvalid();
    test_encodeECI();
    testReport(tests);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
