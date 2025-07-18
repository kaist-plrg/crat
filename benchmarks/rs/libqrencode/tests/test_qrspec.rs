use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn abort() -> !;
    fn testReport(tests: libc::c_int);
    fn testFinish();
    fn testEnd(result: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testInit(tests: libc::c_int);
    static mut assertionNum: libc::c_int;
    static mut assertionFailed: libc::c_int;
    fn QRcode_free(qrcode: *mut QRcode);
    fn QRcode_new(
        version: libc::c_int,
        width: libc::c_int,
        data: *mut libc::c_uchar,
    ) -> *mut QRcode;
    fn QRspec_getDataLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn QRspec_getEccSpec(version: libc::c_int, level: QRecLevel, spec: *mut libc::c_int);
    fn QRspec_getVersionPattern(version: libc::c_int) -> libc::c_uint;
    fn QRspec_getFormatInfo(mask: libc::c_int, level: QRecLevel) -> libc::c_uint;
    fn QRspec_newFrame(version: libc::c_int) -> *mut libc::c_uchar;
    fn QRcode_decodeVersion(code: *mut QRcode) -> libc::c_int;
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
pub type QRecLevel = libc::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRcode {
    pub version: libc::c_int,
    pub width: libc::c_int,
    pub data: *mut libc::c_uchar,
}
unsafe extern "C" fn print_eccTable() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    let mut spec: [libc::c_int; 5] = [0; 5];
    puts(b"\nPrinting ECC table.\n\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        printf(b"Version %2d\n\0" as *const u8 as *const libc::c_char, i);
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            QRspec_getEccSpec(i, j as QRecLevel, spec.as_mut_ptr());
            data = spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
                + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize];
            ecc = spec[0 as libc::c_int as usize] * spec[2 as libc::c_int as usize]
                + spec[3 as libc::c_int as usize] * spec[2 as libc::c_int as usize];
            printf(b"%3d\t\0" as *const u8 as *const libc::c_char, data);
            printf(b"%3d\t\0" as *const u8 as *const libc::c_char, ecc);
            printf(
                b"%2d\t\0" as *const u8 as *const libc::c_char,
                spec[0 as libc::c_int as usize],
            );
            printf(
                b"(%3d, %3d, %3d)\n\0" as *const u8 as *const libc::c_char,
                spec[1 as libc::c_int as usize] + spec[2 as libc::c_int as usize],
                spec[1 as libc::c_int as usize],
                spec[2 as libc::c_int as usize],
            );
            if spec[3 as libc::c_int as usize] > 0 as libc::c_int {
                printf(
                    b"\t%2d\t\0" as *const u8 as *const libc::c_char,
                    spec[3 as libc::c_int as usize],
                );
                printf(
                    b"(%3d, %3d, %3d)\n\0" as *const u8 as *const libc::c_char,
                    spec[4 as libc::c_int as usize] + spec[2 as libc::c_int as usize],
                    spec[4 as libc::c_int as usize],
                    spec[2 as libc::c_int as usize],
                );
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn test_eccTable() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut spec: [libc::c_int; 5] = [0; 5];
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_eccTable\0"))
            .as_ptr(),
        b"Checking ECC table.\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            QRspec_getEccSpec(i, j as QRecLevel, spec.as_mut_ptr());
            data = spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
                + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize];
            ecc = spec[0 as libc::c_int as usize] * spec[2 as libc::c_int as usize]
                + spec[3 as libc::c_int as usize] * spec[2 as libc::c_int as usize];
            if data + ecc
                != QRspec_getDataLength(i, j as QRecLevel)
                    + QRspec_getECCLength(i, j as QRecLevel)
            {
                printf(
                    b"Error in version %d, level %d: invalid size\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    j,
                );
                printf(
                    b"%d %d %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
                    spec[0 as libc::c_int as usize],
                    spec[1 as libc::c_int as usize],
                    spec[2 as libc::c_int as usize],
                    spec[3 as libc::c_int as usize],
                    spec[4 as libc::c_int as usize],
                    spec[2 as libc::c_int as usize],
                );
                err += 1;
                err;
            }
            if ecc != QRspec_getECCLength(i, j as QRecLevel) {
                printf(
                    b"Error in version %d, level %d: invalid data\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    j,
                );
                printf(
                    b"%d %d %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
                    spec[0 as libc::c_int as usize],
                    spec[1 as libc::c_int as usize],
                    spec[2 as libc::c_int as usize],
                    spec[3 as libc::c_int as usize],
                    spec[4 as libc::c_int as usize],
                    spec[2 as libc::c_int as usize],
                );
                err += 1;
                err;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    testEnd(err);
}
unsafe extern "C" fn test_eccTable2() {
    let mut i: libc::c_int = 0;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let correct: [[libc::c_int; 6]; 7] = [
        [
            8 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
            60 as libc::c_int,
            38 as libc::c_int,
        ],
        [
            8 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            2 as libc::c_int,
            61 as libc::c_int,
            39 as libc::c_int,
        ],
        [
            24 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
            11 as libc::c_int,
            54 as libc::c_int,
            24 as libc::c_int,
        ],
        [
            24 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
            16 as libc::c_int,
            55 as libc::c_int,
            25 as libc::c_int,
        ],
        [
            32 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            17 as libc::c_int,
            145 as libc::c_int,
            115 as libc::c_int,
        ],
        [
            40 as libc::c_int,
            3 as libc::c_int,
            0 as libc::c_int,
            20 as libc::c_int,
            45 as libc::c_int,
            15 as libc::c_int,
        ],
        [
            40 as libc::c_int,
            3 as libc::c_int,
            1 as libc::c_int,
            61 as libc::c_int,
            46 as libc::c_int,
            16 as libc::c_int,
        ],
    ];
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"test_eccTable2\0"))
            .as_ptr(),
        b"Checking ECC table(2)\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        QRspec_getEccSpec(
            correct[i as usize][0 as libc::c_int as usize],
            correct[i as usize][1 as libc::c_int as usize] as QRecLevel,
            spec.as_mut_ptr(),
        );
        if correct[i as usize][2 as libc::c_int as usize] == 0 as libc::c_int {
            assertionNum += 1;
            assertionNum;
            if !(spec[0 as libc::c_int as usize]
                == correct[i as usize][3 as libc::c_int as usize])
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Error in version %d, level %d. rsBlockNum1 was %d, expected %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    correct[i as usize][0 as libc::c_int as usize],
                    correct[i as usize][1 as libc::c_int as usize],
                    spec[0 as libc::c_int as usize],
                    correct[i as usize][3 as libc::c_int as usize],
                );
            }
            assertionNum += 1;
            assertionNum;
            if !(spec[1 as libc::c_int as usize] + spec[2 as libc::c_int as usize]
                == correct[i as usize][4 as libc::c_int as usize])
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Error in version %d, level %d. rsDataCodes1 + rsEccCodes1 was %d, expected %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    correct[i as usize][0 as libc::c_int as usize],
                    correct[i as usize][1 as libc::c_int as usize],
                    spec[1 as libc::c_int as usize] + spec[2 as libc::c_int as usize],
                    correct[i as usize][4 as libc::c_int as usize],
                );
            }
            assertionNum += 1;
            assertionNum;
            if !(spec[1 as libc::c_int as usize]
                == correct[i as usize][5 as libc::c_int as usize])
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Error in version %d, level %d. rsDataCodes1 was %d, expected %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    correct[i as usize][0 as libc::c_int as usize],
                    correct[i as usize][1 as libc::c_int as usize],
                    spec[1 as libc::c_int as usize],
                    correct[i as usize][5 as libc::c_int as usize],
                );
            }
        } else {
            assertionNum += 1;
            assertionNum;
            if !(spec[3 as libc::c_int as usize]
                == correct[i as usize][3 as libc::c_int as usize])
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Error in version %d, level %d. rsBlockNum2 was %d, expected %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    correct[i as usize][0 as libc::c_int as usize],
                    correct[i as usize][1 as libc::c_int as usize],
                    spec[3 as libc::c_int as usize],
                    correct[i as usize][3 as libc::c_int as usize],
                );
            }
            assertionNum += 1;
            assertionNum;
            if !(spec[4 as libc::c_int as usize] + spec[2 as libc::c_int as usize]
                == correct[i as usize][4 as libc::c_int as usize])
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Error in version %d, level %d. rsDataCodes2 + rsEccCodes2 was %d, expected %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    correct[i as usize][0 as libc::c_int as usize],
                    correct[i as usize][1 as libc::c_int as usize],
                    spec[4 as libc::c_int as usize] + spec[2 as libc::c_int as usize],
                    correct[i as usize][4 as libc::c_int as usize],
                );
            }
            assertionNum += 1;
            assertionNum;
            if !(spec[4 as libc::c_int as usize]
                == correct[i as usize][5 as libc::c_int as usize])
            {
                assertionFailed += 1;
                assertionFailed;
                printf(
                    b"Error in version %d, level %d. rsDataCodes2 was %d, expected %d.\n\0"
                        as *const u8 as *const libc::c_char,
                    correct[i as usize][0 as libc::c_int as usize],
                    correct[i as usize][1 as libc::c_int as usize],
                    spec[4 as libc::c_int as usize],
                    correct[i as usize][5 as libc::c_int as usize],
                );
            }
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_newframe() {
    let mut buf: [libc::c_uchar; 31329] = [0; 31329];
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut version: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_newframe\0"))
            .as_ptr(),
        b"Checking newly created frame.\0" as *const u8 as *const libc::c_char,
    );
    fp = fopen(
        b"./frame\0" as *const u8 as *const libc::c_char,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        perror(b"Failed to open \"./frame\":\0" as *const u8 as *const libc::c_char);
        abort();
    }
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        frame = QRspec_newFrame(i);
        width = QRspec_getWidth(i);
        len = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (width * width) as libc::c_ulong,
            fp,
        );
        if len as libc::c_int != width * width {
            perror(
                b"Failed to read the pattern file:\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
        assertionNum += 1;
        assertionNum;
        if !(memcmp(
            frame as *const libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            len,
        ) == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"frame pattern mismatch (version %d)\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
        }
        qrcode = QRcode_new(i, width, frame);
        version = QRcode_decodeVersion(qrcode);
        assertionNum += 1;
        assertionNum;
        if !(version == i) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Decoded version number is wrong: %d, expected %d.\n\0" as *const u8
                    as *const libc::c_char,
                version,
                i,
            );
        }
        QRcode_free(qrcode);
        i += 1;
        i;
    }
    testFinish();
    fclose(fp);
}
unsafe extern "C" fn test_newframe_invalid() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"test_newframe_invalid\0"))
            .as_ptr(),
        b"Checking QRspec_newFrame with invalid version.\0" as *const u8
            as *const libc::c_char,
    );
    frame = QRspec_newFrame(0 as libc::c_int);
    assertionNum += 1;
    assertionNum;
    if !frame.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRspec_newFrame(0) returns non-NULL.\0" as *const u8 as *const libc::c_char,
        );
    }
    frame = QRspec_newFrame(40 as libc::c_int + 1 as libc::c_int);
    assertionNum += 1;
    assertionNum;
    if !frame.is_null() {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"QRspec_newFrame(0) returns non-NULL.\0" as *const u8 as *const libc::c_char,
        );
    }
    testFinish();
}
unsafe extern "C" fn test_verpat() {
    let mut version: libc::c_int = 0;
    let mut pattern: libc::c_uint = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut data: libc::c_uint = 0;
    let mut code: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_verpat\0"))
            .as_ptr(),
        b"Checking version pattern.\0" as *const u8 as *const libc::c_char,
    );
    version = 7 as libc::c_int;
    while version <= 40 as libc::c_int {
        pattern = QRspec_getVersionPattern(version);
        if pattern >> 12 as libc::c_int != version as libc::c_uint {
            printf(
                b"Error in version %d.\n\0" as *const u8 as *const libc::c_char,
                version,
            );
            err += 1;
            err;
        } else {
            mask = 0x40 as libc::c_int as libc::c_uint;
            i = 0 as libc::c_int;
            while mask != 0 as libc::c_int as libc::c_uint {
                if version as libc::c_uint & mask != 0 {
                    break;
                }
                mask = mask >> 1 as libc::c_int;
                i += 1;
                i;
            }
            c = 6 as libc::c_int - i;
            data = (version << 12 as libc::c_int) as libc::c_uint;
            code = ((0x1f25 as libc::c_int) << c) as libc::c_uint;
            mask = (0x40000 as libc::c_int >> 6 as libc::c_int - c) as libc::c_uint;
            i = 0 as libc::c_int;
            while i <= c {
                if mask & data != 0 {
                    data ^= code;
                }
                code = code >> 1 as libc::c_int;
                mask = mask >> 1 as libc::c_int;
                i += 1;
                i;
            }
            data = (version << 12 as libc::c_int) as libc::c_uint
                | data & 0xfff as libc::c_int as libc::c_uint;
            if data != pattern {
                printf(
                    b"Error in version %d\n\0" as *const u8 as *const libc::c_char,
                    version,
                );
                err += 1;
                err;
            }
        }
        version += 1;
        version;
    }
    testEnd(err);
}
static mut levelIndicator: [libc::c_uint; 4] = [
    1 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    3 as libc::c_int as libc::c_uint,
    2 as libc::c_int as libc::c_uint,
];
unsafe extern "C" fn calcFormatInfo(
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_uint {
    let mut data: libc::c_uint = 0;
    let mut ecc: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut code: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    data = levelIndicator[level as usize] << 13 as libc::c_int
        | (mask << 10 as libc::c_int) as libc::c_uint;
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
    return (data | ecc) ^ 0x5412 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn test_format() {
    let mut format: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_format\0"))
            .as_ptr(),
        b"Format info test\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            format = calcFormatInfo(j, i as QRecLevel);
            if format != QRspec_getFormatInfo(j, i as QRecLevel) {
                printf(
                    b"Level %d, mask %x\n\0" as *const u8 as *const libc::c_char,
                    i,
                    j,
                );
                err += 1;
                err;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    testEnd(err);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tests: libc::c_int = 6 as libc::c_int;
    testInit(tests);
    test_eccTable();
    test_eccTable2();
    test_newframe();
    test_newframe_invalid();
    test_verpat();
    test_format();
    testReport(tests);
    if argc > 1 as libc::c_int {
        print_eccTable();
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
