use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn printQRinputInfo(input: *mut QRinput);
    fn testReport(tests: libc::c_int);
    fn testFinish();
    fn testEnd(result: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testInit(tests: libc::c_int);
    static mut assertionNum: libc::c_int;
    static mut assertionFailed: libc::c_int;
    fn QRinput_new() -> *mut QRinput;
    fn QRinput_new2(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: libc::c_int,
        data: *const libc::c_uchar,
    ) -> libc::c_int;
    fn QRinput_free(input: *mut QRinput);
    fn BitStream_new() -> *mut BitStream;
    fn BitStream_free(bstream: *mut BitStream);
    fn QRinput_mergeBitStream(
        input: *mut QRinput,
        bstream: *mut BitStream,
    ) -> libc::c_int;
    fn Split_splitStringToQRinput(
        string: *const libc::c_char,
        input: *mut QRinput,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> libc::c_int;
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
unsafe extern "C" fn inputTest(
    mut list: *mut QRinput_List,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut size: libc::c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    ap = args.clone();
    i = 1 as libc::c_int;
    while *fmt != 0 {
        if list.is_null() {
            err = 1 as libc::c_int;
            break;
        } else {
            size = ap.arg::<libc::c_int>();
            if (*list).size != size {
                err = 1 as libc::c_int;
                break;
            } else {
                let fresh0 = fmt;
                fmt = fmt.offset(1);
                match *fresh0 as libc::c_int {
                    110 => {
                        mode = QR_MODE_NUM;
                    }
                    97 => {
                        mode = QR_MODE_AN;
                    }
                    107 => {
                        mode = QR_MODE_KANJI;
                    }
                    56 => {
                        mode = QR_MODE_8;
                    }
                    _ => return -(1 as libc::c_int),
                }
                if (*list).mode as libc::c_int != mode as libc::c_int {
                    err = 1 as libc::c_int;
                    break;
                } else {
                    list = (*list).next;
                    i += 1;
                    i;
                }
            }
        }
    }
    if !list.is_null() {
        err = 1 as libc::c_int;
    }
    if err != 0 {
        return -i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn inputSize(mut input: *mut QRinput) -> libc::c_int {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut size: libc::c_int = 0;
    bstream = BitStream_new();
    QRinput_mergeBitStream(input, bstream);
    size = (*bstream).length as libc::c_int;
    BitStream_free(bstream);
    return size;
}
unsafe extern "C" fn test_split1() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split1\0"))
            .as_ptr(),
        b"Split test: null string\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    bstream = BitStream_new();
    QRinput_mergeBitStream(input, bstream);
    testEnd(!((*bstream).length == 0 as libc::c_int as libc::c_ulong) as libc::c_int);
    QRinput_free(input);
    BitStream_free(bstream);
}
unsafe extern "C" fn test_split2() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split2\0"))
            .as_ptr(),
        b"Split test: single typed strings (num)\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"0123\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"n\0" as *const u8 as *const libc::c_char, 4 as libc::c_int) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
    err = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split2\0"))
            .as_ptr(),
        b"Split test: single typed strings (num2)\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"12345678901234567890\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"n\0" as *const u8 as *const libc::c_char, 20 as libc::c_int)
        != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_split3() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split3\0"))
            .as_ptr(),
        b"Split test: single typed strings (an)\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"ab:-E\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"a\0" as *const u8 as *const libc::c_char, 5 as libc::c_int) != 0
    {
        printQRinputInfo(input);
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
    err = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split3\0"))
            .as_ptr(),
        b"Split test: num + an\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"0123abcde\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"a\0" as *const u8 as *const libc::c_char, 9 as libc::c_int) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
    err = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split3\0"))
            .as_ptr(),
        b"Split test: an + num + an\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"Ab345fg\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"a\0" as *const u8 as *const libc::c_char, 7 as libc::c_int) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_split4() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut i1: *mut QRinput = 0 as *mut QRinput;
    let mut i2: *mut QRinput = 0 as *mut QRinput;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split4\0"))
            .as_ptr(),
        b"Split test: an and num entries\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"ABCDEFGHIJK123456\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    i1 = QRinput_new();
    QRinput_append(
        i1,
        QR_MODE_AN,
        17 as libc::c_int,
        b"ABCDEFGHIJK123456\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    i2 = QRinput_new();
    QRinput_append(
        i2,
        QR_MODE_AN,
        11 as libc::c_int,
        b"ABCDEFGHIJK\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    QRinput_append(
        i2,
        QR_MODE_NUM,
        6 as libc::c_int,
        b"123456\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    size = inputSize(input);
    s1 = inputSize(i1);
    s2 = inputSize(i2);
    testEnd(!(size == (if s1 < s2 { s1 } else { s2 })) as libc::c_int);
    QRinput_free(input);
    QRinput_free(i1);
    QRinput_free(i2);
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split4\0"))
            .as_ptr(),
        b"Split test: num and an entries\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"123456ABCDEFGHIJK\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    i1 = QRinput_new();
    QRinput_append(
        i1,
        QR_MODE_AN,
        17 as libc::c_int,
        b"123456ABCDEFGHIJK\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    i2 = QRinput_new();
    QRinput_append(
        i2,
        QR_MODE_NUM,
        6 as libc::c_int,
        b"123456\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    QRinput_append(
        i2,
        QR_MODE_AN,
        11 as libc::c_int,
        b"ABCDEFGHIJK\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    size = inputSize(input);
    s1 = inputSize(i1);
    s2 = inputSize(i2);
    testEnd(!(size == (if s1 < s2 { s1 } else { s2 })) as libc::c_int);
    QRinput_free(input);
    QRinput_free(i1);
    QRinput_free(i2);
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split4\0"))
            .as_ptr(),
        b"Split test: num and an entries (should be splitted)\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"1234567ABCDEFGHIJK\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    i1 = QRinput_new();
    QRinput_append(
        i1,
        QR_MODE_AN,
        18 as libc::c_int,
        b"1234567ABCDEFGHIJK\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    i2 = QRinput_new();
    QRinput_append(
        i2,
        QR_MODE_NUM,
        7 as libc::c_int,
        b"1234567\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    QRinput_append(
        i2,
        QR_MODE_AN,
        11 as libc::c_int,
        b"ABCDEFGHIJK\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    size = inputSize(input);
    s1 = inputSize(i1);
    s2 = inputSize(i2);
    testEnd(!(size == (if s1 < s2 { s1 } else { s2 })) as libc::c_int);
    QRinput_free(input);
    QRinput_free(i1);
    QRinput_free(i2);
}
unsafe extern "C" fn test_split5() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split5\0"))
            .as_ptr(),
        b"Split test: bit, an, bit, num\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"\x82\xD9abcdeabcdea\x82\xB0123456\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(
        list,
        b"8a8n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        11 as libc::c_int,
        2 as libc::c_int,
        6 as libc::c_int,
    ) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_split6() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split6\0"))
            .as_ptr(),
        b"Split test: kanji, an, kanji, num\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"\x82\xD9abcdeabcdea\x82\xB0123456\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(
        list,
        b"kakn\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        11 as libc::c_int,
        2 as libc::c_int,
        6 as libc::c_int,
    ) != 0
    {
        printQRinputInfo(input);
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_split7() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split7\0"))
            .as_ptr(),
        b"Split test: an and num as bits\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"\x82\xD9abcde\x82\xB012345\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(
        list,
        b"8n\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int,
        5 as libc::c_int,
    ) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_split8() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_split8\0"))
            .as_ptr(),
        b"Split test: terminated with a half of kanji code\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"\x82\xD9abcdefgh\x82\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(
        list,
        b"ka8\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
    ) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_split3c() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_split3c\0"))
            .as_ptr(),
        b"Split test: single typed strings (an, case-sensitive)\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"ab:-E\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        1 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"8\0" as *const u8 as *const libc::c_char, 5 as libc::c_int) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
    err = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_split3c\0"))
            .as_ptr(),
        b"Split test: num + an\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"0123abcde\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        1 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(
        list,
        b"n8\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int,
        5 as libc::c_int,
    ) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
    err = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_split3c\0"))
            .as_ptr(),
        b"Split test: an + num + an\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"Ab345fg\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        1 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"8\0" as *const u8 as *const libc::c_char, 7 as libc::c_int) != 0
    {
        printQRinputInfo(input);
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_toupper() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_toupper\0"))
            .as_ptr(),
        b"Split test: check dupAndToUpper (lower->upper)\0" as *const u8
            as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"abcde\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"a\0" as *const u8 as *const libc::c_char, 5 as libc::c_int) != 0
    {
        err += 1;
        err;
    }
    if strncmp(
        (*list).data as *mut libc::c_char,
        b"ABCDE\0" as *const u8 as *const libc::c_char,
        (*list).size as libc::c_ulong,
    ) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
    err = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_toupper\0"))
            .as_ptr(),
        b"Split test: check dupAndToUpper (kanji)\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"\x83n\x83q\x83t\x83w\x83z\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_KANJI,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"k\0" as *const u8 as *const libc::c_char, 10 as libc::c_int)
        != 0
    {
        printQRinputInfo(input);
        err += 1;
        err;
    }
    if strncmp(
        (*list).data as *mut libc::c_char,
        b"\x83n\x83q\x83t\x83w\x83z\0" as *const u8 as *const libc::c_char,
        (*list).size as libc::c_ulong,
    ) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
    err = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"test_toupper\0"))
            .as_ptr(),
        b"Split test: check dupAndToUpper (8bit)\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"\x83n\x83q\x83t\x83w\x83z\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        0 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"8\0" as *const u8 as *const libc::c_char, 10 as libc::c_int)
        != 0
    {
        printQRinputInfo(input);
        err += 1;
        err;
    }
    if strncmp(
        (*list).data as *mut libc::c_char,
        b"\x83N\x83Q\x83T\x83W\x83Z\0" as *const u8 as *const libc::c_char,
        (*list).size as libc::c_ulong,
    ) != 0
    {
        err += 1;
        err;
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_splitNum8() {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_splitNum8\0"))
            .as_ptr(),
        b"Split test: num and 8bit to 8bit\0" as *const u8 as *const libc::c_char,
    );
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(
        b"1abcdefg\0" as *const u8 as *const libc::c_char,
        input,
        QR_MODE_8,
        1 as libc::c_int,
    );
    list = (*input).head;
    if inputTest(list, b"8\0" as *const u8 as *const libc::c_char, 8 as libc::c_int) != 0
    {
        err += 1;
        err;
        printQRinputInfo(input);
    }
    testEnd(err);
    QRinput_free(input);
}
unsafe extern "C" fn test_splitAnNAn() {
    let mut input1: *mut QRinput = 0 as *mut QRinput;
    let mut input2: *mut QRinput = 0 as *mut QRinput;
    let mut input3: *mut QRinput = 0 as *mut QRinput;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut s3: libc::c_int = 0;
    let mut strall: *mut libc::c_char = b"326A80A9C5004C0875571F8B71C311F2F86\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut str1: *mut libc::c_char = b"326A80A9C5004C\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut str2: *mut libc::c_char = b"0875571\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut str3: *mut libc::c_char = b"F8B71C311F2F86\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_splitAnNAn\0"))
            .as_ptr(),
        b"Split test: An-N-An switching cost test\0" as *const u8 as *const libc::c_char,
    );
    input1 = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(strall, input1, QR_MODE_8, 0 as libc::c_int);
    input2 = QRinput_new();
    QRinput_append(input2, QR_MODE_AN, 35 as libc::c_int, strall as *mut libc::c_uchar);
    input3 = QRinput_new();
    QRinput_append(input3, QR_MODE_AN, 14 as libc::c_int, str1 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_NUM, 7 as libc::c_int, str2 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_AN, 14 as libc::c_int, str3 as *mut libc::c_uchar);
    s1 = inputSize(input1);
    s2 = inputSize(input2);
    s3 = inputSize(input3);
    assertionNum += 1;
    assertionNum;
    if !(s1 == s2) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect split\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(s2 < s3) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect estimation\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    QRinput_free(input1);
    QRinput_free(input2);
    QRinput_free(input3);
}
unsafe extern "C" fn test_splitAn8An() {
    let mut input1: *mut QRinput = 0 as *mut QRinput;
    let mut input2: *mut QRinput = 0 as *mut QRinput;
    let mut input3: *mut QRinput = 0 as *mut QRinput;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut s3: libc::c_int = 0;
    let mut strall: *mut libc::c_char = b"ABCDabcdefABCD\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut str1: *mut libc::c_char = b"ABCD\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut str2: *mut libc::c_char = b"abcdef\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut str3: *mut libc::c_char = b"ABCD\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_splitAn8An\0"))
            .as_ptr(),
        b"Split test: An-8-An switching cost test\0" as *const u8 as *const libc::c_char,
    );
    input1 = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(strall, input1, QR_MODE_8, 1 as libc::c_int);
    input2 = QRinput_new();
    QRinput_append(input2, QR_MODE_8, 14 as libc::c_int, strall as *mut libc::c_uchar);
    input3 = QRinput_new();
    QRinput_append(input3, QR_MODE_AN, 4 as libc::c_int, str1 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_8, 6 as libc::c_int, str2 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_AN, 4 as libc::c_int, str3 as *mut libc::c_uchar);
    s1 = inputSize(input1);
    s2 = inputSize(input2);
    s3 = inputSize(input3);
    assertionNum += 1;
    assertionNum;
    if !(s1 == s2) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect split\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(s2 < s3) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect estimation\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    QRinput_free(input1);
    QRinput_free(input2);
    QRinput_free(input3);
}
unsafe extern "C" fn test_split8An8() {
    let mut input1: *mut QRinput = 0 as *mut QRinput;
    let mut input2: *mut QRinput = 0 as *mut QRinput;
    let mut input3: *mut QRinput = 0 as *mut QRinput;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut s3: libc::c_int = 0;
    let mut strall: *mut libc::c_char = b"abcABCDEFGHabc\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut str1: *mut libc::c_char = b"abc\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut str2: *mut libc::c_char = b"ABCDEFGH\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut str3: *mut libc::c_char = b"abc\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_split8An8\0"))
            .as_ptr(),
        b"Split test: 8-An-8 switching cost test\0" as *const u8 as *const libc::c_char,
    );
    input1 = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(strall, input1, QR_MODE_8, 1 as libc::c_int);
    input2 = QRinput_new();
    QRinput_append(input2, QR_MODE_8, 14 as libc::c_int, strall as *mut libc::c_uchar);
    input3 = QRinput_new();
    QRinput_append(input3, QR_MODE_8, 3 as libc::c_int, str1 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_AN, 8 as libc::c_int, str2 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_8, 3 as libc::c_int, str3 as *mut libc::c_uchar);
    s1 = inputSize(input1);
    s2 = inputSize(input2);
    s3 = inputSize(input3);
    assertionNum += 1;
    assertionNum;
    if !(s1 == s2) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect split\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(s2 < s3) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect estimation\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    QRinput_free(input1);
    QRinput_free(input2);
    QRinput_free(input3);
}
unsafe extern "C" fn test_split8N8() {
    let mut input1: *mut QRinput = 0 as *mut QRinput;
    let mut input2: *mut QRinput = 0 as *mut QRinput;
    let mut input3: *mut QRinput = 0 as *mut QRinput;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut s3: libc::c_int = 0;
    let mut strall: *mut libc::c_char = b"abc1234abc\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut str1: *mut libc::c_char = b"abc\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut str2: *mut libc::c_char = b"1234\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut str3: *mut libc::c_char = b"abc\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_split8N8\0"))
            .as_ptr(),
        b"Split test: 8-N-8 switching cost test\0" as *const u8 as *const libc::c_char,
    );
    input1 = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(strall, input1, QR_MODE_8, 1 as libc::c_int);
    input2 = QRinput_new();
    QRinput_append(input2, QR_MODE_8, 10 as libc::c_int, strall as *mut libc::c_uchar);
    input3 = QRinput_new();
    QRinput_append(input3, QR_MODE_8, 3 as libc::c_int, str1 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_NUM, 4 as libc::c_int, str2 as *mut libc::c_uchar);
    QRinput_append(input3, QR_MODE_8, 3 as libc::c_int, str3 as *mut libc::c_uchar);
    s1 = inputSize(input1);
    s2 = inputSize(input2);
    s3 = inputSize(input3);
    assertionNum += 1;
    assertionNum;
    if !(s1 == s2) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect split\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(s2 < s3) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Incorrect estimation\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    QRinput_free(input1);
    QRinput_free(input2);
    QRinput_free(input3);
}
unsafe fn main_0() -> libc::c_int {
    let mut tests: libc::c_int = 24 as libc::c_int;
    testInit(tests);
    test_split1();
    test_split2();
    test_split3();
    test_split4();
    test_split5();
    test_split6();
    test_split7();
    test_split8();
    test_split3c();
    test_toupper();
    test_splitNum8();
    test_splitAnNAn();
    test_splitAn8An();
    test_split8An8();
    test_split8N8();
    testReport(tests);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
