use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn testReport(tests: libc::c_int);
    fn testFinish();
    fn testEnd(result: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testInit(tests: libc::c_int);
    static mut assertionNum: libc::c_int;
    static mut assertionFailed: libc::c_int;
    fn MQRspec_getDataLengthBit(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn MQRspec_getFormatInfo(
        mask: libc::c_int,
        version: libc::c_int,
        level: QRecLevel,
    ) -> libc::c_uint;
    fn MQRspec_newFrame(version: libc::c_int) -> *mut libc::c_uchar;
}
pub type QRecLevel = libc::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
static mut v4frame: [libc::c_uchar; 289] = [
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
    0x90 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn test_newFrame() {
    let mut width: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_newFrame\0"))
            .as_ptr(),
        b"Test empty frames\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i < 4 as libc::c_int {
        frame = MQRspec_newFrame(i);
        width = MQRspec_getWidth(i);
        y = 0 as libc::c_int;
        while y < width {
            assertionNum += 1;
            assertionNum;
            if !(memcmp(
                &mut *frame.offset((y * width) as isize) as *mut libc::c_uchar
                    as *const libc::c_void,
                &mut *v4frame.as_mut_ptr().offset((y * 17 as libc::c_int) as isize)
                    as *mut libc::c_uchar as *const libc::c_void,
                width as libc::c_ulong,
            ) == 0 as libc::c_int)
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Mismatch found in version %d, line %d.\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    y,
                );
            }
            y += 1;
            y;
        }
        free(frame as *mut libc::c_void);
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_newframe_invalid() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"test_newframe_invalid\0"))
            .as_ptr(),
        b"Checking MQRspec_newFrame with invalid version.\0" as *const u8
            as *const libc::c_char,
    );
    frame = MQRspec_newFrame(0 as libc::c_int);
    assertionNum += 1;
    assertionNum;
    if !frame.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"MQRspec_newFrame(0) returns non-NULL.\0" as *const u8
                as *const libc::c_char,
        );
    }
    frame = MQRspec_newFrame(4 as libc::c_int + 1 as libc::c_int);
    assertionNum += 1;
    assertionNum;
    if !frame.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"MQRspec_newFrame(0) returns non-NULL.\0" as *const u8
                as *const libc::c_char,
        );
    }
    testFinish();
}
unsafe extern "C" fn calcFormatInfo(
    mut type_0: libc::c_int,
    mut mask: libc::c_int,
) -> libc::c_uint {
    let mut data: libc::c_uint = 0;
    let mut ecc: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut code: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    data = (type_0 << 12 as libc::c_int | mask << 10 as libc::c_int) as libc::c_uint;
    ecc = data;
    b = ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
    i = 0 as libc::c_int;
    while b != 0 as libc::c_int as libc::c_uint {
        if ecc & b != 0 {
            break;
        }
        b = b >> 1 as libc::c_int;
        i += 1;
        i;
    }
    c = 4 as libc::c_int - i;
    code = ((0x537 as libc::c_int) << c) as libc::c_uint;
    b = ((1 as libc::c_int) << 10 as libc::c_int + c) as libc::c_uint;
    i = 0 as libc::c_int;
    while i <= c {
        if b & ecc != 0 {
            ecc ^= code;
        }
        code = code >> 1 as libc::c_int;
        b = b >> 1 as libc::c_int;
        i += 1;
        i;
    }
    return (data | ecc) ^ 0x4445 as libc::c_int as libc::c_uint;
}
static mut typeTable: [[libc::c_int; 3]; 4] = [
    [0 as libc::c_int, -(1 as libc::c_int), -(1 as libc::c_int)],
    [1 as libc::c_int, 2 as libc::c_int, -(1 as libc::c_int)],
    [3 as libc::c_int, 4 as libc::c_int, -(1 as libc::c_int)],
    [5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int],
];
unsafe extern "C" fn test_format() {
    let mut format: libc::c_uint = 0;
    let mut version: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_format\0"))
            .as_ptr(),
        b"Format info test\0" as *const u8 as *const libc::c_char,
    );
    version = 1 as libc::c_int;
    while version <= 4 as libc::c_int {
        l = QR_ECLEVEL_L as libc::c_int;
        while l <= QR_ECLEVEL_Q as libc::c_int {
            mask = 0 as libc::c_int;
            while mask < 4 as libc::c_int {
                format = MQRspec_getFormatInfo(mask, version, l as QRecLevel);
                type_0 = typeTable[(version - 1 as libc::c_int) as usize][l as usize];
                if type_0 == -(1 as libc::c_int) {
                    if format != 0 as libc::c_int as libc::c_uint {
                        printf(
                            b"Error in version %d, level %d, mask %d\n\0" as *const u8
                                as *const libc::c_char,
                            version,
                            l,
                            mask,
                        );
                        err += 1;
                        err;
                    }
                } else if format != calcFormatInfo(type_0, mask) {
                    printf(
                        b"Error in version %d, level %d, mask %d\n\0" as *const u8
                            as *const libc::c_char,
                        version,
                        l,
                        mask,
                    );
                    err += 1;
                    err;
                }
                mask += 1;
                mask;
            }
            l += 1;
            l;
        }
        version += 1;
        version;
    }
    testEnd(err);
}
unsafe extern "C" fn print_format() {
    let mut format: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    puts(
        b"\nPrinting hex strings of format information.\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            format = calcFormatInfo(j, i);
            printf(b"0x%04x, \0" as *const u8 as *const libc::c_char, format);
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
}
static mut datalen: [[libc::c_int; 3]; 4] = [
    [20 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
    [40 as libc::c_int, 32 as libc::c_int, 0 as libc::c_int],
    [84 as libc::c_int, 68 as libc::c_int, 0 as libc::c_int],
    [128 as libc::c_int, 112 as libc::c_int, 80 as libc::c_int],
];
unsafe extern "C" fn test_dataLength() {
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"test_dataLength\0"))
            .as_ptr(),
        b"Test dataLength\0" as *const u8 as *const libc::c_char,
    );
    v = 0 as libc::c_int;
    while v < 4 as libc::c_int {
        l = 0 as libc::c_int;
        while l < 3 as libc::c_int {
            bits = MQRspec_getDataLengthBit(v + 1 as libc::c_int, l as QRecLevel);
            if bits != datalen[v as usize][l as usize] {
                printf(
                    b"Error in version %d, level %d.\n\0" as *const u8
                        as *const libc::c_char,
                    v,
                    l,
                );
                err += 1;
                err;
            }
            l += 1;
            l;
        }
        v += 1;
        v;
    }
    testEnd(err);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tests: libc::c_int = 4 as libc::c_int;
    testInit(tests);
    test_newFrame();
    test_newframe_invalid();
    test_format();
    test_dataLength();
    testReport(tests);
    if argc > 1 as libc::c_int {
        print_format();
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
