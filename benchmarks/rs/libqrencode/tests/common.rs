use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn BitStream_new() -> *mut BitStream;
    fn BitStream_free(bstream: *mut BitStream);
    fn QRinput_mergeBitStream(
        input: *mut QRinput,
        bstream: *mut BitStream,
    ) -> libc::c_int;
    fn QRraw_new(input: *mut QRinput) -> *mut QRRawCode;
    fn QRraw_free(raw: *mut QRRawCode);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRcode {
    pub version: libc::c_int,
    pub width: libc::c_int,
    pub data: *mut libc::c_uchar,
}
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
static mut tests: libc::c_int = 0 as libc::c_int;
static mut failed: libc::c_int = 0 as libc::c_int;
pub static mut assertionFailed: libc::c_int = 0 as libc::c_int;
pub static mut assertionNum: libc::c_int = 0 as libc::c_int;
static mut testName: *const libc::c_char = 0 as *const libc::c_char;
static mut testFunc: *const libc::c_char = 0 as *const libc::c_char;
pub static mut levelChar: [libc::c_char; 4] = [
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
];
pub static mut modeStr: [*const libc::c_char; 5] = [
    b"nm\0" as *const u8 as *const libc::c_char,
    b"an\0" as *const u8 as *const libc::c_char,
    b"8\0" as *const u8 as *const libc::c_char,
    b"kj\0" as *const u8 as *const libc::c_char,
    b"st\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn ncmpBin(
    mut correct: *mut libc::c_char,
    mut bstream: *mut BitStream,
    mut len: size_t,
) -> libc::c_int {
    let mut bit: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if len != (*bstream).length {
        printf(
            b"Length is not match: %zu, %zu expected.\n\0" as *const u8
                as *const libc::c_char,
            (*bstream).length,
            len,
        );
        return -(1 as libc::c_int);
    }
    p = correct;
    i = 0 as libc::c_int as size_t;
    while *p as libc::c_int != '\0' as i32 {
        while *p as libc::c_int == ' ' as i32 {
            p = p.offset(1);
            p;
        }
        bit = if *p as libc::c_int == '1' as i32 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if *((*bstream).data).offset(i as isize) as libc::c_int != bit {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
        p = p.offset(1);
        p;
        if i == len {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cmpBin(
    mut correct: *mut libc::c_char,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = correct;
    while *p as libc::c_int != '\0' as i32 {
        if *p as libc::c_int != ' ' as i32 {
            len = len.wrapping_add(1);
            len;
        }
        p = p.offset(1);
        p;
    }
    return ncmpBin(correct, bstream, len);
}
pub unsafe extern "C" fn testInit(mut tests_0: libc::c_int) {
    printf(b"1..%d\n\0" as *const u8 as *const libc::c_char, tests_0);
}
pub unsafe extern "C" fn testStartReal(
    mut func: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    tests += 1;
    tests;
    testName = name;
    testFunc = func;
    assertionFailed = 0 as libc::c_int;
    assertionNum = 0 as libc::c_int;
}
pub unsafe extern "C" fn testEnd(mut result: libc::c_int) {
    if result != 0 {
        printf(
            b"not ok %d %s: %s\n\0" as *const u8 as *const libc::c_char,
            tests,
            testFunc,
            testName,
        );
        failed += 1;
        failed;
    } else {
        printf(
            b"ok %d %s: %s\n\0" as *const u8 as *const libc::c_char,
            tests,
            testFunc,
            testName,
        );
    };
}
pub unsafe extern "C" fn testFinish() {
    if assertionFailed != 0 {
        printf(
            b"not ok %d %s: %s (%d assertions failed.)\n\0" as *const u8
                as *const libc::c_char,
            tests,
            testFunc,
            testName,
            assertionFailed,
        );
        failed += 1;
        failed;
    } else {
        printf(
            b"ok %d %s: %s (%d assertions passed.)\n\0" as *const u8
                as *const libc::c_char,
            tests,
            testFunc,
            testName,
            assertionNum,
        );
    };
}
pub unsafe extern "C" fn testReport(mut expectedTests: libc::c_int) {
    printf(
        b"Total %d tests, %d fails.\n\0" as *const u8 as *const libc::c_char,
        tests,
        failed,
    );
    if failed != 0 {
        exit(-(1 as libc::c_int));
    }
    if expectedTests != tests {
        printf(
            b"WARNING: the number of the executed tests (%d) is not equal to the expecetd (%d).\n\0"
                as *const u8 as *const libc::c_char,
            tests,
            expectedTests,
        );
    }
}
pub unsafe extern "C" fn testNum() -> libc::c_int {
    return tests;
}
pub unsafe extern "C" fn printBinary(
    mut data: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        printf(
            if *data.offset(i as isize) as libc::c_int != 0 {
                b"1\0" as *const u8 as *const libc::c_char
            } else {
                b"0\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn printBstream(mut bstream: *mut BitStream) {
    printBinary((*bstream).data, (*bstream).length as libc::c_int);
}
pub unsafe extern "C" fn printQRinput(mut input: *mut QRinput) {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut i: libc::c_int = 0;
    list = (*input).head;
    while !list.is_null() {
        i = 0 as libc::c_int;
        while i < (*list).size {
            printf(
                b"0x%02x,\0" as *const u8 as *const libc::c_char,
                *((*list).data).offset(i as isize) as libc::c_int,
            );
            i += 1;
            i;
        }
        list = (*list).next;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn printQRinputInfo(mut input: *mut QRinput) {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut b: *mut BitStream = 0 as *mut BitStream;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    printf(b"QRinput info:\n\0" as *const u8 as *const libc::c_char);
    printf(b" version: %d\n\0" as *const u8 as *const libc::c_char, (*input).version);
    printf(
        b" level  : %c\n\0" as *const u8 as *const libc::c_char,
        levelChar[(*input).level as usize] as libc::c_int,
    );
    list = (*input).head;
    i = 0 as libc::c_int;
    while !list.is_null() {
        i += 1;
        i;
        list = (*list).next;
    }
    printf(b"  chunks: %d\n\0" as *const u8 as *const libc::c_char, i);
    b = BitStream_new();
    ret = QRinput_mergeBitStream(input, b);
    if ret == 0 as libc::c_int {
        printf(
            b"  bitstream-size: %zu\n\0" as *const u8 as *const libc::c_char,
            (*b).length,
        );
        BitStream_free(b);
    }
    list = (*input).head;
    i = 0 as libc::c_int;
    while !list.is_null() {
        printf(
            b"\t#%d: mode = %s, size = %d\n\0" as *const u8 as *const libc::c_char,
            i,
            modeStr[(*list).mode as usize],
            (*list).size,
        );
        i += 1;
        i;
        list = (*list).next;
    }
}
pub unsafe extern "C" fn printQRinputStruct(mut s: *mut QRinput_Struct) {
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut i: libc::c_int = 1 as libc::c_int;
    printf(b"Struct size: %d\n\0" as *const u8 as *const libc::c_char, (*s).size);
    printf(b"Struct parity: %08x\n\0" as *const u8 as *const libc::c_char, (*s).parity);
    list = (*s).head;
    while !list.is_null() {
        printf(b"Symbol %d - \0" as *const u8 as *const libc::c_char, i);
        printQRinputInfo((*list).input);
        i += 1;
        i;
        list = (*list).next;
    }
}
pub unsafe extern "C" fn printFrame(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            let fresh0 = frame;
            frame = frame.offset(1);
            printf(
                b"%02x \0" as *const u8 as *const libc::c_char,
                *fresh0 as libc::c_int,
            );
            x += 1;
            x;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        y += 1;
        y;
    }
}
pub unsafe extern "C" fn printQRcode(mut code: *mut QRcode) {
    printFrame((*code).width, (*code).data);
}
pub unsafe extern "C" fn printQRRawCodeFromQRinput(mut input: *mut QRinput) {
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut i: libc::c_int = 0;
    puts(b"QRRawCode dump image:\0" as *const u8 as *const libc::c_char);
    raw = QRraw_new(input);
    if raw.is_null() {
        puts(
            b"Failed to generate QRRawCode from this input.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < (*raw).dataLength {
        printf(
            b" %02x\0" as *const u8 as *const libc::c_char,
            *((*raw).datacode).offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*raw).eccLength {
        printf(
            b" %02x\0" as *const u8 as *const libc::c_char,
            *((*raw).ecccode).offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    QRraw_free(raw);
}
pub unsafe extern "C" fn show_QRcode(mut qrcode: *mut QRcode) {}
