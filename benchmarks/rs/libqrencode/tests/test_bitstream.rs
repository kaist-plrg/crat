use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn cmpBin(correct: *mut libc::c_char, bstream: *mut BitStream) -> libc::c_int;
    fn ncmpBin(
        correct: *mut libc::c_char,
        bstream: *mut BitStream,
        len: size_t,
    ) -> libc::c_int;
    fn testReport(tests: libc::c_int);
    fn testFinish();
    fn testEnd(result: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testInit(tests: libc::c_int);
    static mut assertionNum: libc::c_int;
    static mut assertionFailed: libc::c_int;
    fn BitStream_new() -> *mut BitStream;
    fn BitStream_newWithBits(size: size_t, bits: *mut libc::c_uchar) -> *mut BitStream;
    fn BitStream_append(bstream: *mut BitStream, arg: *mut BitStream) -> libc::c_int;
    fn BitStream_appendNum(
        bstream: *mut BitStream,
        bits: size_t,
        num: libc::c_uint,
    ) -> libc::c_int;
    fn BitStream_appendBytes(
        bstream: *mut BitStream,
        size: size_t,
        data: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn BitStream_toByte(bstream: *mut BitStream) -> *mut libc::c_uchar;
    fn BitStream_free(bstream: *mut BitStream);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut libc::c_uchar,
}
unsafe extern "C" fn test_null() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"test_null\0"))
            .as_ptr(),
        b"Empty stream\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    assertionNum += 1;
    assertionNum;
    if !((*bstream).length == 0 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Size of empty BitStream is not 0.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(BitStream_toByte(bstream)).is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"BitStream_toByte returned non-NULL.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    printf(b"Checking BitStream_free(NULL).\n\0" as *const u8 as *const libc::c_char);
    BitStream_free(0 as *mut BitStream);
    testFinish();
    BitStream_free(bstream);
}
unsafe extern "C" fn test_num() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut data: libc::c_uint = 0x13579bdf as libc::c_int as libc::c_uint;
    let mut correct: [libc::c_char; 32] = *::std::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"0010011010101111001101111011111\0");
    testStartReal(
        (*::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"test_num\0")).as_ptr(),
        b"New from num\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    BitStream_appendNum(bstream, 31 as libc::c_int as size_t, data);
    testEnd(cmpBin(correct.as_mut_ptr(), bstream));
    BitStream_free(bstream);
}
unsafe extern "C" fn test_bytes() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut data: [libc::c_uchar; 1] = [0x3a as libc::c_int as libc::c_uchar];
    let mut correct: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"00111010\0");
    testStartReal(
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"test_bytes\0"))
            .as_ptr(),
        b"New from bytes\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    BitStream_appendBytes(bstream, 1 as libc::c_int as size_t, data.as_mut_ptr());
    testEnd(cmpBin(correct.as_mut_ptr(), bstream));
    BitStream_free(bstream);
}
unsafe extern "C" fn test_appendNum() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut correct: [libc::c_char; 60] = *::std::mem::transmute::<
        &[u8; 60],
        &mut [libc::c_char; 60],
    >(b"10001010 11111111 11111111 00010010001101000101011001111000\0");
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_appendNum\0"))
            .as_ptr(),
        b"Append Num\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    BitStream_appendNum(
        bstream,
        8 as libc::c_int as size_t,
        0x8a as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !(ncmpBin(correct.as_mut_ptr(), bstream, 8 as libc::c_int as size_t)
        == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    BitStream_appendNum(
        bstream,
        16 as libc::c_int as size_t,
        0xffff as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !(ncmpBin(correct.as_mut_ptr(), bstream, 24 as libc::c_int as size_t)
        == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    BitStream_appendNum(
        bstream,
        32 as libc::c_int as size_t,
        0x12345678 as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(correct.as_mut_ptr(), bstream) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    BitStream_free(bstream);
}
unsafe extern "C" fn test_appendBytes() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut data: [libc::c_uchar; 8] = [0; 8];
    let mut correct: [libc::c_char; 57] = *::std::mem::transmute::<
        &[u8; 57],
        &mut [libc::c_char; 57],
    >(b"10001010111111111111111100010010001101000101011001111000\0");
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_appendBytes\0"))
            .as_ptr(),
        b"Append Bytes\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    data[0 as libc::c_int as usize] = 0x8a as libc::c_int as libc::c_uchar;
    BitStream_appendBytes(bstream, 1 as libc::c_int as size_t, data.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(ncmpBin(correct.as_mut_ptr(), bstream, 8 as libc::c_int as size_t)
        == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\0" as *const u8 as *const libc::c_char);
    }
    data[0 as libc::c_int as usize] = 0xff as libc::c_int as libc::c_uchar;
    data[1 as libc::c_int as usize] = 0xff as libc::c_int as libc::c_uchar;
    BitStream_appendBytes(bstream, 2 as libc::c_int as size_t, data.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(ncmpBin(correct.as_mut_ptr(), bstream, 24 as libc::c_int as size_t)
        == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    data[0 as libc::c_int as usize] = 0x12 as libc::c_int as libc::c_uchar;
    data[1 as libc::c_int as usize] = 0x34 as libc::c_int as libc::c_uchar;
    data[2 as libc::c_int as usize] = 0x56 as libc::c_int as libc::c_uchar;
    data[3 as libc::c_int as usize] = 0x78 as libc::c_int as libc::c_uchar;
    BitStream_appendBytes(bstream, 4 as libc::c_int as size_t, data.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(correct.as_mut_ptr(), bstream) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    BitStream_free(bstream);
}
unsafe extern "C" fn test_toByte() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut correct: [libc::c_uchar; 7] = [
        0x8a as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0x12 as libc::c_int as libc::c_uchar,
        0x34 as libc::c_int as libc::c_uchar,
        0x56 as libc::c_int as libc::c_uchar,
        0x78 as libc::c_int as libc::c_uchar,
    ];
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_toByte\0"))
            .as_ptr(),
        b"Convert to a byte array\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    BitStream_appendBytes(
        bstream,
        1 as libc::c_int as size_t,
        &mut *correct.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    BitStream_appendBytes(
        bstream,
        2 as libc::c_int as size_t,
        &mut *correct.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    BitStream_appendBytes(
        bstream,
        4 as libc::c_int as size_t,
        &mut *correct.as_mut_ptr().offset(3 as libc::c_int as isize),
    );
    result = BitStream_toByte(bstream);
    testEnd(
        memcmp(
            correct.as_mut_ptr() as *const libc::c_void,
            result as *const libc::c_void,
            7 as libc::c_int as libc::c_ulong,
        ),
    );
    BitStream_free(bstream);
    free(result as *mut libc::c_void);
}
unsafe extern "C" fn test_toByte_4bitpadding() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"test_toByte_4bitpadding\0"))
            .as_ptr(),
        b"Convert to a byte array\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        0xb as libc::c_int as libc::c_uint,
    );
    result = BitStream_toByte(bstream);
    assertionNum += 1;
    assertionNum;
    if !(*result.offset(0 as libc::c_int as isize) as libc::c_int == 0xb0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"incorrect paddings\n\0" as *const u8 as *const libc::c_char);
    }
    BitStream_free(bstream);
    free(result as *mut libc::c_void);
    bstream = BitStream_new();
    BitStream_appendNum(
        bstream,
        12 as libc::c_int as size_t,
        0x335 as libc::c_int as libc::c_uint,
    );
    result = BitStream_toByte(bstream);
    assertionNum += 1;
    assertionNum;
    if !(*result.offset(0 as libc::c_int as isize) as libc::c_int == 0x33 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"incorrect paddings\n\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(*result.offset(1 as libc::c_int as isize) as libc::c_int == 0x50 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"incorrect paddings\n\0" as *const u8 as *const libc::c_char);
    }
    BitStream_free(bstream);
    free(result as *mut libc::c_void);
    testFinish();
}
unsafe extern "C" fn test_size() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"test_size\0"))
            .as_ptr(),
        b"size check\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_new();
    assertionNum += 1;
    assertionNum;
    if !((*bstream).length == 0 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Initialized BitStream is not 0 length\0" as *const u8
                as *const libc::c_char,
        );
    }
    BitStream_appendNum(
        bstream,
        1 as libc::c_int as size_t,
        0 as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !((*bstream).length == 1 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Size incorrect. (first append)\0" as *const u8 as *const libc::c_char);
    }
    BitStream_appendNum(
        bstream,
        2 as libc::c_int as size_t,
        0 as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !((*bstream).length == 3 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Size incorrect. (second append)\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    BitStream_free(bstream);
}
unsafe extern "C" fn test_append() {
    let mut bs1: *mut BitStream = 0 as *mut BitStream;
    let mut bs2: *mut BitStream = 0 as *mut BitStream;
    let mut c1: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"00\0");
    let mut c2: [libc::c_char; 5] = *::std::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"0011\0");
    let mut c3: [libc::c_char; 18] = *::std::mem::transmute::<
        &[u8; 18],
        &mut [libc::c_char; 18],
    >(b"01111111111111111\0");
    let mut c4: [libc::c_char; 22] = *::std::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"001101111111111111111\0");
    let mut c5: [libc::c_char; 35] = *::std::mem::transmute::<
        &[u8; 35],
        &mut [libc::c_char; 35],
    >(b"0011011111111111111111111111111111\0");
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_append\0"))
            .as_ptr(),
        b"Append two BitStreams\0" as *const u8 as *const libc::c_char,
    );
    bs1 = BitStream_new();
    bs2 = BitStream_new();
    ret = BitStream_appendNum(
        bs1,
        1 as libc::c_int as size_t,
        0 as libc::c_int as libc::c_uint,
    );
    ret = BitStream_appendNum(
        bs2,
        1 as libc::c_int as size_t,
        0 as libc::c_int as libc::c_uint,
    );
    ret = BitStream_append(bs1, bs2);
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to append.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(c1.as_mut_ptr(), bs1) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\0" as *const u8 as *const libc::c_char);
    }
    ret = BitStream_appendNum(
        bs1,
        2 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to append.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(c2.as_mut_ptr(), bs1) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\0" as *const u8 as *const libc::c_char);
    }
    ret = BitStream_appendNum(
        bs2,
        16 as libc::c_int as size_t,
        65535 as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to append.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(c3.as_mut_ptr(), bs2) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\0" as *const u8 as *const libc::c_char);
    }
    ret = BitStream_append(bs1, bs2);
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to append.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(c4.as_mut_ptr(), bs1) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\0" as *const u8 as *const libc::c_char);
    }
    ret = BitStream_appendNum(
        bs1,
        13 as libc::c_int as size_t,
        16383 as libc::c_int as libc::c_uint,
    );
    assertionNum += 1;
    assertionNum;
    if !(ret == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Failed to append.\0" as *const u8 as *const libc::c_char);
    }
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(c5.as_mut_ptr(), bs1) == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    BitStream_free(bs1);
    BitStream_free(bs2);
}
unsafe extern "C" fn test_newWithBits() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut data: [libc::c_uchar; 4] = [
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
    ];
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"test_newWithBits\0"))
            .as_ptr(),
        b"New with bits\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_newWithBits(4 as libc::c_int as size_t, data.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !((*bstream).length == 4 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Internal bit length is incorrect.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*bstream).datasize == 4 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Internal buffer size is incorrect.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !(cmpBin(
        b"0101\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        bstream,
    ) == 0 as libc::c_int)
    {
        assertionFailed += 1;
        assertionFailed;
        printf(b"Internal data is incorrect.\n\0" as *const u8 as *const libc::c_char);
    }
    testFinish();
    BitStream_free(bstream);
}
unsafe extern "C" fn test_newWithBits_size0() {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"test_newWithBits_size0\0"))
            .as_ptr(),
        b"New with bits (size = 0)\0" as *const u8 as *const libc::c_char,
    );
    bstream = BitStream_newWithBits(0 as libc::c_int as size_t, 0 as *mut libc::c_uchar);
    assertionNum += 1;
    assertionNum;
    if !((*bstream).length == 0 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Internal bit length is incorrect.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if !((*bstream).datasize != 0 as libc::c_int as libc::c_ulong) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Internal buffer size is incorrect.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    assertionNum += 1;
    assertionNum;
    if ((*bstream).data).is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Internal buffer not allocated.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    testFinish();
    BitStream_free(bstream);
}
unsafe fn main_0() -> libc::c_int {
    let mut tests: libc::c_int = 11 as libc::c_int;
    testInit(tests);
    test_null();
    test_num();
    test_bytes();
    test_appendNum();
    test_appendBytes();
    test_toByte();
    test_toByte_4bitpadding();
    test_size();
    test_append();
    test_newWithBits();
    test_newWithBits_size0();
    testReport(tests);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
