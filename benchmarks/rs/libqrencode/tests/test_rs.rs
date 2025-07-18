use ::libc;
extern "C" {
    pub type _RS;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn QRinput_new() -> *mut QRinput;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: libc::c_int,
        data: *const libc::c_uchar,
    ) -> libc::c_int;
    fn QRinput_setErrorCorrectionLevel(
        input: *mut QRinput,
        level: QRecLevel,
    ) -> libc::c_int;
    fn QRinput_free(input: *mut QRinput);
    fn QRraw_new(input: *mut QRinput) -> *mut QRRawCode;
    fn QRraw_free(raw: *mut QRRawCode);
    static mut assertionFailed: libc::c_int;
    static mut assertionNum: libc::c_int;
    fn testInit(tests: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testEnd(result: libc::c_int);
    fn testFinish();
    fn testReport(tests: libc::c_int);
    fn QRspec_getEccSpec(version: libc::c_int, level: QRecLevel, spec: *mut libc::c_int);
    fn MQRspec_getDataLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn RSECC_encode(
        data_length: size_t,
        ecc_length: size_t,
        data: *const libc::c_uchar,
        ecc: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn RSECC_decoder_init();
    fn RSECC_decoder_checkSyndrome(
        dl: libc::c_int,
        data: *mut libc::c_uchar,
        el: libc::c_int,
        ecc: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn init_rs(
        symsize: libc::c_int,
        gfpoly: libc::c_int,
        fcr: libc::c_int,
        prim: libc::c_int,
        nroots: libc::c_int,
        pad: libc::c_int,
    ) -> *mut RS;
    fn encode_rs_char(
        rs: *mut RS,
        data: *const libc::c_uchar,
        parity: *mut libc::c_uchar,
    );
    fn free_rs_char(rs: *mut RS);
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
pub type RS = _RS;
pub unsafe extern "C" fn test_rscodeexample() {
    let mut stream: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRRawCode = 0 as *mut QRRawCode;
    static mut str: [libc::c_char; 9] = unsafe {
        *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"01234567\0")
    };
    static mut correct: [libc::c_uchar; 26] = [
        0x10 as libc::c_int as libc::c_uchar,
        0x20 as libc::c_int as libc::c_uchar,
        0xc as libc::c_int as libc::c_uchar,
        0x56 as libc::c_int as libc::c_uchar,
        0x61 as libc::c_int as libc::c_uchar,
        0x80 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xec as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0xa5 as libc::c_int as libc::c_uchar,
        0x24 as libc::c_int as libc::c_uchar,
        0xd4 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xed as libc::c_int as libc::c_uchar,
        0x36 as libc::c_int as libc::c_uchar,
        0xc7 as libc::c_int as libc::c_uchar,
        0x87 as libc::c_int as libc::c_uchar,
        0x2c as libc::c_int as libc::c_uchar,
        0x55 as libc::c_int as libc::c_uchar,
    ];
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"test_rscodeexample\0"))
            .as_ptr(),
        b"RS ecc test\0" as *const u8 as *const libc::c_char,
    );
    stream = QRinput_new();
    QRinput_append(
        stream,
        QR_MODE_NUM,
        8 as libc::c_int,
        str.as_ptr() as *mut libc::c_uchar,
    );
    QRinput_setErrorCorrectionLevel(stream, QR_ECLEVEL_M);
    code = QRraw_new(stream);
    testEnd(
        memcmp(
            correct.as_mut_ptr().offset(16 as libc::c_int as isize)
                as *const libc::c_void,
            (*((*code).rsblock).offset(0 as libc::c_int as isize)).ecc
                as *const libc::c_void,
            10 as libc::c_int as libc::c_ulong,
        ),
    );
    QRinput_free(stream);
    QRraw_free(code);
}
unsafe extern "C" fn compareRS(mut data: *mut libc::c_uchar) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rs: *mut RS = 0 as *mut RS;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let mut dl: libc::c_int = 0;
    let mut el: libc::c_int = 0;
    let mut ecc_expected: [libc::c_uchar; 256] = [0; 256];
    let mut ecc_rscodec: [libc::c_uchar; 256] = [0; 256];
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        j = QR_ECLEVEL_L as libc::c_int;
        while j <= QR_ECLEVEL_H as libc::c_int {
            QRspec_getEccSpec(i, j as QRecLevel, spec.as_mut_ptr());
            dl = spec[1 as libc::c_int as usize];
            el = spec[2 as libc::c_int as usize];
            rs = init_rs(
                8 as libc::c_int,
                0x11d as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
                el,
                255 as libc::c_int - dl - el,
            );
            RSECC_encode(
                dl as size_t,
                el as size_t,
                data as *const libc::c_uchar,
                ecc_rscodec.as_mut_ptr(),
            );
            encode_rs_char(rs, data as *const libc::c_uchar, ecc_expected.as_mut_ptr());
            assertionNum += 1;
            assertionNum;
            if !(memcmp(
                ecc_expected.as_mut_ptr() as *const libc::c_void,
                ecc_rscodec.as_mut_ptr() as *const libc::c_void,
                el as libc::c_ulong,
            ) == 0 as libc::c_int)
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Invalid ECC found: length %d.\n\0" as *const u8
                        as *const libc::c_char,
                    el,
                );
            }
            assertionNum += 1;
            assertionNum;
            if !(RSECC_decoder_checkSyndrome(dl, data, el, ecc_rscodec.as_mut_ptr())
                == 0 as libc::c_int)
            {
                assertionFailed += 1;
                assertionFailed;
                printf(b"ECC error found.\0" as *const u8 as *const libc::c_char);
            }
            free_rs_char(rs);
            dl = spec[4 as libc::c_int as usize];
            el = spec[2 as libc::c_int as usize];
            if dl != 0 as libc::c_int {
                rs = init_rs(
                    8 as libc::c_int,
                    0x11d as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    el,
                    255 as libc::c_int - dl - el,
                );
                RSECC_encode(
                    dl as size_t,
                    el as size_t,
                    data as *const libc::c_uchar,
                    ecc_rscodec.as_mut_ptr(),
                );
                encode_rs_char(
                    rs,
                    data as *const libc::c_uchar,
                    ecc_expected.as_mut_ptr(),
                );
                assertionNum += 1;
                assertionNum;
                if !(memcmp(
                    ecc_expected.as_mut_ptr() as *const libc::c_void,
                    ecc_rscodec.as_mut_ptr() as *const libc::c_void,
                    el as libc::c_ulong,
                ) == 0 as libc::c_int)
                {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(
                        b"Invalid ECC found: length %d.\n\0" as *const u8
                            as *const libc::c_char,
                        el,
                    );
                }
                assertionNum += 1;
                assertionNum;
                if !(RSECC_decoder_checkSyndrome(dl, data, el, ecc_rscodec.as_mut_ptr())
                    == 0 as libc::c_int)
                {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(b"ECC error found.\0" as *const u8 as *const libc::c_char);
                }
                free_rs_char(rs);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn compareRSMQR(mut data: *mut libc::c_uchar) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rs: *mut RS = 0 as *mut RS;
    let mut dl: libc::c_int = 0;
    let mut el: libc::c_int = 0;
    let mut ecc_expected: [libc::c_uchar; 256] = [0; 256];
    let mut ecc_rscodec: [libc::c_uchar; 256] = [0; 256];
    i = 1 as libc::c_int;
    while i <= 4 as libc::c_int {
        j = QR_ECLEVEL_L as libc::c_int;
        while j <= QR_ECLEVEL_Q as libc::c_int {
            dl = MQRspec_getDataLength(i, j as QRecLevel);
            el = MQRspec_getECCLength(i, j as QRecLevel);
            if dl != 0 as libc::c_int {
                rs = init_rs(
                    8 as libc::c_int,
                    0x11d as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    el,
                    255 as libc::c_int - dl - el,
                );
                RSECC_encode(
                    dl as size_t,
                    el as size_t,
                    data as *const libc::c_uchar,
                    ecc_rscodec.as_mut_ptr(),
                );
                encode_rs_char(
                    rs,
                    data as *const libc::c_uchar,
                    ecc_expected.as_mut_ptr(),
                );
                assertionNum += 1;
                assertionNum;
                if !(memcmp(
                    ecc_expected.as_mut_ptr() as *const libc::c_void,
                    ecc_rscodec.as_mut_ptr() as *const libc::c_void,
                    el as libc::c_ulong,
                ) == 0 as libc::c_int)
                {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(
                        b"Invalid ECC found: length %d.\n\0" as *const u8
                            as *const libc::c_char,
                        el,
                    );
                }
                assertionNum += 1;
                assertionNum;
                if !(RSECC_decoder_checkSyndrome(dl, data, el, ecc_rscodec.as_mut_ptr())
                    == 0 as libc::c_int)
                {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(b"ECC error found.\0" as *const u8 as *const libc::c_char);
                }
                free_rs_char(rs);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn test_allQRSizeAndECCLevel() {
    let mut i: libc::c_int = 0;
    let mut data: [libc::c_uchar; 256] = [0; 256];
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"test_allQRSizeAndECCLevel\0"))
            .as_ptr(),
        b"Comparing with KA9Q's code: all QR Code sizes and ECC levels\0" as *const u8
            as *const libc::c_char,
    );
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    compareRS(data.as_mut_ptr());
    compareRSMQR(data.as_mut_ptr());
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        0xaa as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    compareRS(data.as_mut_ptr());
    compareRSMQR(data.as_mut_ptr());
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    compareRS(data.as_mut_ptr());
    compareRSMQR(data.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        data[i as usize] = i as libc::c_uchar;
        i += 1;
        i;
    }
    compareRS(data.as_mut_ptr());
    compareRSMQR(data.as_mut_ptr());
    testFinish();
}
unsafe fn main_0() -> libc::c_int {
    RSECC_decoder_init();
    let mut tests: libc::c_int = 2 as libc::c_int;
    testInit(tests);
    test_rscodeexample();
    test_allQRSizeAndECCLevel();
    testReport(tests);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
