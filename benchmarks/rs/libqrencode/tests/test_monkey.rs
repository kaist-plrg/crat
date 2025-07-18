use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printQRinputInfo(input: *mut QRinput);
    fn printQRinput(input: *mut QRinput);
    fn QRinput_new2(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_free(input: *mut QRinput);
    fn QRinput_Struct_free(s: *mut QRinput_Struct);
    fn QRinput_splitQRinputToStruct(input: *mut QRinput) -> *mut QRinput_Struct;
    fn QRcode_encodeString(
        string: *const libc::c_char,
        version: libc::c_int,
        level: QRecLevel,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> *mut QRcode;
    fn QRcode_encodeData(
        size: libc::c_int,
        data_0: *const libc::c_uchar,
        version: libc::c_int,
        level: QRecLevel,
    ) -> *mut QRcode;
    fn QRcode_free(qrcode: *mut QRcode);
    fn QRcode_encodeInputStructured(s: *mut QRinput_Struct) -> *mut QRcode_List;
    fn QRcode_List_free(qrlist: *mut QRcode_List);
    fn BitStream_new() -> *mut BitStream;
    fn BitStream_free(bstream: *mut BitStream);
    fn QRinput_mergeBitStream(
        input: *mut QRinput,
        bstream: *mut BitStream,
    ) -> libc::c_int;
    fn QRcode_encodeMask(input: *mut QRinput, mask: libc::c_int) -> *mut QRcode;
    static levelChar: [libc::c_char; 4];
    fn testInit(tests: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testEnd(result: libc::c_int);
    fn testReport(tests: libc::c_int);
    fn Split_splitStringToQRinput(
        string: *const libc::c_char,
        input: *mut QRinput,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> libc::c_int;
    fn QRspec_getDataLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getEccSpec(version: libc::c_int, level: QRecLevel, spec: *mut libc::c_int);
    fn QRdata_free(data_0: *mut QRdata);
    fn QRcode_unmask(code: *mut QRcode) -> *mut libc::c_uchar;
    fn QRcode_extractBits(
        code: *mut QRcode,
        dataLength: *mut libc::c_int,
        eccLength: *mut libc::c_int,
    ) -> *mut BitStream;
    fn QRcode_decode(code: *mut QRcode) -> *mut QRdata;
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
pub struct _QRcode_List {
    pub code: *mut QRcode,
    pub next: *mut _QRcode_List,
}
pub type QRcode_List = _QRcode_List;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut data: [libc::c_uchar; 7091] = [0; 7091];
static mut check: [libc::c_uchar; 7091] = [0; 7091];
static mut AN: *const libc::c_char = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn fill8bitString() -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    len = 1 as libc::c_int
        + ((7091 as libc::c_int - 2 as libc::c_int) as libc::c_double
            * rand() as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        data[i
            as usize] = ((255 as libc::c_int as libc::c_double * rand() as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_uchar
            as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
        i += 1;
        i;
    }
    data[len as usize] = '\0' as i32 as libc::c_uchar;
    return len;
}
unsafe extern "C" fn fill8bitData() -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    len = 1 as libc::c_int
        + ((7091 as libc::c_int - 2 as libc::c_int) as libc::c_double
            * rand() as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        data[i
            as usize] = (256 as libc::c_int as libc::c_double * rand() as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_uchar;
        i += 1;
        i;
    }
    data[len as usize] = '\0' as i32 as libc::c_uchar;
    return len;
}
unsafe extern "C" fn fillANData() -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    len = 1 as libc::c_int
        + ((7091 as libc::c_int - 2 as libc::c_int) as libc::c_double
            * rand() as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        data[i
            as usize] = *AN
            .offset(
                (45 as libc::c_int as libc::c_double * rand() as libc::c_double
                    / (2147483647 as libc::c_int as libc::c_double + 1.0f64))
                    as libc::c_int as isize,
            ) as libc::c_uchar;
        i += 1;
        i;
    }
    data[len as usize] = '\0' as i32 as libc::c_uchar;
    return len;
}
unsafe extern "C" fn test_encode_an(mut num: libc::c_int) {
    let mut ret: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = fillANData();
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 256] = [0; 256];
    qrcode = QRcode_encodeString(
        data.as_mut_ptr() as *mut libc::c_char,
        0 as libc::c_int,
        (num % 4 as libc::c_int) as QRecLevel,
        QR_MODE_8,
        num % 2 as libc::c_int,
    );
    if qrcode.is_null() {
        if *__errno_location() == 34 as libc::c_int {
            return;
        }
        perror(
            b"test_encode_an aborted at QRcode_encodeString():\0" as *const u8
                as *const libc::c_char,
        );
        printf(b"Length: %d\n\0" as *const u8 as *const libc::c_char, len);
        printf(
            b"Level: %d\n\0" as *const u8 as *const libc::c_char,
            num % 4 as libc::c_int,
        );
        return;
    }
    qrdata = QRcode_decode(qrcode);
    if qrdata.is_null() {
        printf(
            b"#%d: Failed to decode this code.\n\0" as *const u8 as *const libc::c_char,
            num,
        );
        QRcode_free(qrcode);
        return;
    }
    if (*qrdata).size != len {
        printf(
            b"#%d: length mismatched (orig: %d, decoded: %d)\n\0" as *const u8
                as *const libc::c_char,
            num,
            len,
            (*qrdata).size,
        );
    }
    ret = memcmp(
        (*qrdata).data as *const libc::c_void,
        data.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
    if ret != 0 as libc::c_int {
        let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut x: libc::c_uint = 0;
        let mut y: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut dataLength: libc::c_int = 0;
        let mut eccLength: libc::c_int = 0;
        let mut input: *mut QRinput = 0 as *mut QRinput;
        let mut origcode: *mut QRcode = 0 as *mut QRcode;
        let mut bstream: *mut BitStream = 0 as *mut BitStream;
        let mut spec: [libc::c_int; 5] = [0; 5];
        printf(b"#%d: data mismatched.\n\0" as *const u8 as *const libc::c_char, num);
        printf(
            b"Version: %d\n\0" as *const u8 as *const libc::c_char,
            (*qrcode).version,
        );
        QRspec_getEccSpec(
            (*qrcode).version,
            (num % 4 as libc::c_int) as QRecLevel,
            spec.as_mut_ptr(),
        );
        printf(
            b"DataLength: %d\n\0" as *const u8 as *const libc::c_char,
            spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
                + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize],
        );
        printf(
            b"BlockNum1: %d\n\0" as *const u8 as *const libc::c_char,
            spec[0 as libc::c_int as usize],
        );
        printf(
            b"BlockNum: %d\n\0" as *const u8 as *const libc::c_char,
            spec[0 as libc::c_int as usize] + spec[3 as libc::c_int as usize],
        );
        printf(
            b"DataCodes1: %d\n\0" as *const u8 as *const libc::c_char,
            spec[1 as libc::c_int as usize],
        );
        snprintf(
            buf.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"monkey-orig-%d.dat\0" as *const u8 as *const libc::c_char,
            num,
        );
        fp = fopen(buf.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
        fputs(data.as_mut_ptr() as *mut libc::c_char, fp);
        fclose(fp);
        snprintf(
            buf.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"monkey-result-%d.dat\0" as *const u8 as *const libc::c_char,
            num,
        );
        fp = fopen(buf.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
        fputs((*qrdata).data as *mut libc::c_char, fp);
        fclose(fp);
        snprintf(
            buf.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"monkey-result-unmasked-%d.dat\0" as *const u8 as *const libc::c_char,
            num,
        );
        fp = fopen(buf.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
        frame = QRcode_unmask(qrcode);
        p = frame;
        y = 0 as libc::c_int;
        while y < (*qrcode).width {
            x = 0 as libc::c_int as libc::c_uint;
            while x < (*qrcode).width as libc::c_uint {
                fputc(
                    if *p as libc::c_int & 1 as libc::c_int != 0 {
                        '1' as i32
                    } else {
                        '0' as i32
                    },
                    fp,
                );
                p = p.offset(1);
                p;
                x = x.wrapping_add(1);
                x;
            }
            fputc('\n' as i32, fp);
            y += 1;
            y;
        }
        fclose(fp);
        free(frame as *mut libc::c_void);
        snprintf(
            buf.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"monkey-orig-unmasked-%d.dat\0" as *const u8 as *const libc::c_char,
            num,
        );
        fp = fopen(buf.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
        input = QRinput_new2(0 as libc::c_int, (num % 4 as libc::c_int) as QRecLevel);
        Split_splitStringToQRinput(
            data.as_mut_ptr() as *mut libc::c_char,
            input,
            QR_MODE_8,
            num % 2 as libc::c_int,
        );
        origcode = QRcode_encodeMask(input, -(2 as libc::c_int));
        p = (*origcode).data;
        y = 0 as libc::c_int;
        while y < (*origcode).width {
            x = 0 as libc::c_int as libc::c_uint;
            while x < (*origcode).width as libc::c_uint {
                fputc(
                    if *p as libc::c_int & 1 as libc::c_int != 0 {
                        '1' as i32
                    } else {
                        '0' as i32
                    },
                    fp,
                );
                p = p.offset(1);
                p;
                x = x.wrapping_add(1);
                x;
            }
            fputc('\n' as i32, fp);
            y += 1;
            y;
        }
        fclose(fp);
        QRcode_free(origcode);
        snprintf(
            buf.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"monkey-orig-bits-%d.dat\0" as *const u8 as *const libc::c_char,
            num,
        );
        fp = fopen(buf.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
        bstream = BitStream_new();
        QRinput_mergeBitStream(input, bstream);
        c = 0 as libc::c_int;
        x = 0 as libc::c_int as libc::c_uint;
        while (x as libc::c_ulong) < (*bstream).length {
            fputc(
                if *((*bstream).data).offset(x as isize) as libc::c_int
                    & 1 as libc::c_int != 0
                {
                    '1' as i32
                } else {
                    '0' as i32
                },
                fp,
            );
            if x & 7 as libc::c_int as libc::c_uint == 7 as libc::c_int as libc::c_uint {
                fputc(' ' as i32, fp);
                c += 1;
                c;
            }
            if x & 63 as libc::c_int as libc::c_uint == 63 as libc::c_int as libc::c_uint
            {
                fprintf(fp, b"%d\n\0" as *const u8 as *const libc::c_char, c);
            }
            x = x.wrapping_add(1);
            x;
        }
        fclose(fp);
        QRinput_free(input);
        BitStream_free(bstream);
        snprintf(
            buf.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"monkey-result-bits-%d.dat\0" as *const u8 as *const libc::c_char,
            num,
        );
        fp = fopen(buf.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
        bstream = QRcode_extractBits(qrcode, &mut dataLength, &mut eccLength);
        y = (*bstream).length as libc::c_int;
        p = (*bstream).data;
        c = 0 as libc::c_int;
        x = 0 as libc::c_int as libc::c_uint;
        while x < y as libc::c_uint {
            fputc(
                if *p.offset(x as isize) as libc::c_int & 1 as libc::c_int != 0 {
                    '1' as i32
                } else {
                    '0' as i32
                },
                fp,
            );
            if x & 7 as libc::c_int as libc::c_uint == 7 as libc::c_int as libc::c_uint {
                fputc(' ' as i32, fp);
                c += 1;
                c;
            }
            if x & 63 as libc::c_int as libc::c_uint == 63 as libc::c_int as libc::c_uint
            {
                fprintf(fp, b"%d\n\0" as *const u8 as *const libc::c_char, c);
            }
            x = x.wrapping_add(1);
            x;
        }
        fclose(fp);
        BitStream_free(bstream);
    }
    QRdata_free(qrdata);
    QRcode_free(qrcode);
}
unsafe extern "C" fn monkey_encode_an(mut loop_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"monkey_encode_an\0"))
            .as_ptr(),
        b"Monkey test: QRcode_encodeString() - AlphaNumeric string.\0" as *const u8
            as *const libc::c_char,
    );
    srand(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < loop_0 {
        test_encode_an(i);
        i += 1;
        i;
    }
    testEnd(0 as libc::c_int);
}
unsafe extern "C" fn test_split_an(mut num: libc::c_int) {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    len = fillANData();
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    if input.is_null() {
        perror(
            b"test_split_an aborted at QRinput_new2():\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    ret = Split_splitStringToQRinput(
        data.as_mut_ptr() as *mut libc::c_char,
        input,
        QR_MODE_8,
        1 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        perror(
            b"test_split_an aborted at Split_splitStringToQRinput():\0" as *const u8
                as *const libc::c_char,
        );
        QRinput_free(input);
        return;
    }
    list = (*input).head;
    i = 0 as libc::c_int;
    while !list.is_null() {
        memcpy(
            check.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
            (*list).data as *const libc::c_void,
            (*list).size as libc::c_ulong,
        );
        i += (*list).size;
        list = (*list).next;
    }
    if i != len {
        printf(
            b"#%d: length is not correct. (%d should be %d)\n\0" as *const u8
                as *const libc::c_char,
            num,
            i,
            len,
        );
    }
    check[i as usize] = '\0' as i32 as libc::c_uchar;
    ret = memcmp(
        data.as_mut_ptr() as *const libc::c_void,
        check.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
    if ret != 0 as libc::c_int {
        printf(b"#%d: data mismatched.\n\0" as *const u8 as *const libc::c_char, num);
        list = (*input).head;
        i = 0 as libc::c_int;
        while !list.is_null() {
            ret = memcmp(
                data.as_mut_ptr().offset(i as isize) as *const libc::c_void,
                (*list).data as *const libc::c_void,
                (*list).size as libc::c_ulong,
            );
            printf(b"wrong chunk:\n\0" as *const u8 as *const libc::c_char);
            printf(b" position: %d\n\0" as *const u8 as *const libc::c_char, i);
            printf(
                b" mode    : %d\n\0" as *const u8 as *const libc::c_char,
                (*list).mode as libc::c_int,
            );
            printf(
                b" size    : %d\n\0" as *const u8 as *const libc::c_char,
                (*list).size,
            );
            printf(
                b" data    : %.*s\n\0" as *const u8 as *const libc::c_char,
                (*list).size,
                (*list).data,
            );
            i += (*list).size;
            list = (*list).next;
        }
        exit(1 as libc::c_int);
    }
    QRinput_free(input);
}
unsafe extern "C" fn monkey_split_an(mut loop_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"monkey_split_an\0"))
            .as_ptr(),
        b"Monkey test: Split_splitStringToQRinput() - AlphaNumeric string.\0"
            as *const u8 as *const libc::c_char,
    );
    srand(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < loop_0 {
        test_split_an(i);
        i += 1;
        i;
    }
    testEnd(0 as libc::c_int);
}
unsafe extern "C" fn test_encode_8(mut num: libc::c_int) {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    len = fill8bitData();
    qrcode = QRcode_encodeData(
        len,
        data.as_mut_ptr(),
        0 as libc::c_int,
        (num % 4 as libc::c_int) as QRecLevel,
    );
    if qrcode.is_null() {
        if *__errno_location() == 34 as libc::c_int {
            return;
        }
        perror(
            b"test_encdoe_8 aborted at QRcode_encodeData():\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    qrdata = QRcode_decode(qrcode);
    if qrdata.is_null() {
        printf(
            b"#%d: Failed to decode this code.\n\0" as *const u8 as *const libc::c_char,
            num,
        );
        QRcode_free(qrcode);
        return;
    }
    if (*qrdata).size != len {
        printf(
            b"#%d: length mismatched (orig: %d, decoded: %d)\n\0" as *const u8
                as *const libc::c_char,
            num,
            len,
            (*qrdata).size,
        );
    }
    ret = memcmp(
        (*qrdata).data as *const libc::c_void,
        data.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
    if ret != 0 as libc::c_int {
        printf(b"#%d: data mismatched.\n\0" as *const u8 as *const libc::c_char, num);
    }
    QRdata_free(qrdata);
    QRcode_free(qrcode);
}
unsafe extern "C" fn monkey_encode_8(mut loop_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"monkey_encode_8\0"))
            .as_ptr(),
        b"Monkey test: QRcode_encodeData() - 8bit char string.\0" as *const u8
            as *const libc::c_char,
    );
    srand(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < loop_0 {
        test_encode_8(i);
        i += 1;
        i;
    }
    testEnd(0 as libc::c_int);
}
unsafe extern "C" fn test_split_8(mut num: libc::c_int) {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    len = fill8bitString();
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    if input.is_null() {
        perror(
            b"test_split_8 aborted at QRinput_new2():\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    ret = Split_splitStringToQRinput(
        data.as_mut_ptr() as *mut libc::c_char,
        input,
        QR_MODE_8,
        1 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        perror(
            b"test_split_8 aborted at Split_splitStringToQRinput():\0" as *const u8
                as *const libc::c_char,
        );
        QRinput_free(input);
        return;
    }
    list = (*input).head;
    i = 0 as libc::c_int;
    while !list.is_null() {
        memcpy(
            check.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
            (*list).data as *const libc::c_void,
            (*list).size as libc::c_ulong,
        );
        i += (*list).size;
        list = (*list).next;
    }
    if i != len {
        printf(
            b"#%d: length is not correct. (%d should be %d)\n\0" as *const u8
                as *const libc::c_char,
            num,
            i,
            len,
        );
    }
    check[i as usize] = '\0' as i32 as libc::c_uchar;
    ret = memcmp(
        data.as_mut_ptr() as *const libc::c_void,
        check.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
    if ret != 0 as libc::c_int {
        printf(b"#%d: data mismatched.\n\0" as *const u8 as *const libc::c_char, num);
        list = (*input).head;
        i = 0 as libc::c_int;
        while !list.is_null() {
            ret = memcmp(
                data.as_mut_ptr().offset(i as isize) as *const libc::c_void,
                (*list).data as *const libc::c_void,
                (*list).size as libc::c_ulong,
            );
            printf(b"wrong chunk:\n\0" as *const u8 as *const libc::c_char);
            printf(b" position: %d\n\0" as *const u8 as *const libc::c_char, i);
            printf(
                b" mode    : %d\n\0" as *const u8 as *const libc::c_char,
                (*list).mode as libc::c_int,
            );
            printf(
                b" size    : %d\n\0" as *const u8 as *const libc::c_char,
                (*list).size,
            );
            printf(
                b" data    : %.*s\n\0" as *const u8 as *const libc::c_char,
                (*list).size,
                (*list).data,
            );
            i += (*list).size;
            list = (*list).next;
        }
        exit(1 as libc::c_int);
    }
    QRinput_free(input);
}
unsafe extern "C" fn monkey_split_8(mut loop_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"monkey_split_8\0"))
            .as_ptr(),
        b"Monkey test: Split_splitStringToQRinput() - 8bit char string.\0" as *const u8
            as *const libc::c_char,
    );
    srand(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < loop_0 {
        test_split_8(i);
        i += 1;
        i;
    }
    testEnd(0 as libc::c_int);
}
unsafe extern "C" fn test_encode_kanji(mut num: libc::c_int) {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut qrdata: *mut QRdata = 0 as *mut QRdata;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    len = fill8bitString();
    qrcode = QRcode_encodeString(
        data.as_mut_ptr() as *mut libc::c_char,
        0 as libc::c_int,
        (num % 4 as libc::c_int) as QRecLevel,
        QR_MODE_8,
        1 as libc::c_int,
    );
    if qrcode.is_null() {
        if *__errno_location() == 34 as libc::c_int {
            return;
        }
        perror(
            b"test_encdoe_kanji aborted at QRcode_encodeString():\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    qrdata = QRcode_decode(qrcode);
    if qrdata.is_null() {
        printf(
            b"#%d: Failed to decode this code.\n\0" as *const u8 as *const libc::c_char,
            num,
        );
        QRcode_free(qrcode);
        return;
    }
    if (*qrdata).size != len {
        printf(
            b"#%d: length mismatched (orig: %d, decoded: %d)\n\0" as *const u8
                as *const libc::c_char,
            num,
            len,
            (*qrdata).size,
        );
    }
    ret = memcmp(
        (*qrdata).data as *const libc::c_void,
        data.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
    if ret != 0 as libc::c_int {
        printf(b"#%d: data mismatched.\n\0" as *const u8 as *const libc::c_char, num);
    }
    QRdata_free(qrdata);
    QRcode_free(qrcode);
}
unsafe extern "C" fn monkey_encode_kanji(mut loop_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"monkey_encode_kanji\0"))
            .as_ptr(),
        b"Monkey test: QRcode_encodeString() - kanji string.\0" as *const u8
            as *const libc::c_char,
    );
    srand(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < loop_0 {
        test_encode_kanji(i);
        i += 1;
        i;
    }
    testEnd(0 as libc::c_int);
}
unsafe extern "C" fn test_split_kanji(mut num: libc::c_int) {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    len = fill8bitString();
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    if input.is_null() {
        perror(
            b"test_split_kanji aborted at QRinput_new2():\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    ret = Split_splitStringToQRinput(
        data.as_mut_ptr() as *mut libc::c_char,
        input,
        QR_MODE_KANJI,
        1 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        perror(
            b"test_split_kanji aborted at Split_splitStringToQRinput():\0" as *const u8
                as *const libc::c_char,
        );
        QRinput_free(input);
        return;
    }
    list = (*input).head;
    i = 0 as libc::c_int;
    while !list.is_null() {
        memcpy(
            check.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
            (*list).data as *const libc::c_void,
            (*list).size as libc::c_ulong,
        );
        i += (*list).size;
        list = (*list).next;
    }
    if i != len {
        printf(
            b"#%d: length is not correct. (%d should be %d)\n\0" as *const u8
                as *const libc::c_char,
            num,
            i,
            len,
        );
    }
    check[i as usize] = '\0' as i32 as libc::c_uchar;
    ret = memcmp(
        data.as_mut_ptr() as *const libc::c_void,
        check.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
    if ret != 0 as libc::c_int {
        printf(b"#%d: data mismatched.\n\0" as *const u8 as *const libc::c_char, num);
        list = (*input).head;
        i = 0 as libc::c_int;
        while !list.is_null() {
            ret = memcmp(
                data.as_mut_ptr().offset(i as isize) as *const libc::c_void,
                (*list).data as *const libc::c_void,
                (*list).size as libc::c_ulong,
            );
            printf(b"wrong chunk:\n\0" as *const u8 as *const libc::c_char);
            printf(b" position: %d\n\0" as *const u8 as *const libc::c_char, i);
            printf(
                b" mode    : %d\n\0" as *const u8 as *const libc::c_char,
                (*list).mode as libc::c_int,
            );
            printf(
                b" size    : %d\n\0" as *const u8 as *const libc::c_char,
                (*list).size,
            );
            printf(
                b" data    : %.*s\n\0" as *const u8 as *const libc::c_char,
                (*list).size,
                (*list).data,
            );
            i += (*list).size;
            list = (*list).next;
        }
        exit(1 as libc::c_int);
    }
    QRinput_free(input);
}
unsafe extern "C" fn monkey_split_kanji(mut loop_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"monkey_split_kanji\0"))
            .as_ptr(),
        b"Monkey test: Split_splitStringToQRinput() - kanji string.\0" as *const u8
            as *const libc::c_char,
    );
    srand(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < loop_0 {
        test_split_kanji(i);
        i += 1;
        i;
    }
    testEnd(0 as libc::c_int);
}
unsafe extern "C" fn test_split_structure(mut num: libc::c_int) {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut list: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut il: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut version: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    version = (40 as libc::c_int as libc::c_double * rand() as libc::c_double
        / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as libc::c_int
        + 1 as libc::c_int;
    level = (4 as libc::c_int as libc::c_double * rand() as libc::c_double
        / (2147483647 as libc::c_int as libc::c_double + 1.0f64)) as QRecLevel;
    fill8bitString();
    input = QRinput_new2(version, level);
    if input.is_null() {
        perror(
            b"test_split_structure aborted at QRinput_new2():\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    ret = Split_splitStringToQRinput(
        data.as_mut_ptr() as *mut libc::c_char,
        input,
        QR_MODE_KANJI,
        1 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        perror(
            b"test_split_structure aborted at Split_splitStringToQRinput():\0"
                as *const u8 as *const libc::c_char,
        );
        QRinput_free(input);
        return;
    }
    s = QRinput_splitQRinputToStruct(input);
    if s.is_null() {
        if *__errno_location() != 0 as libc::c_int
            && *__errno_location() != 34 as libc::c_int
        {
            perror(
                b"test_split_structure aborted at QRinput_splitQRinputToStruct():\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        QRinput_free(input);
        return;
    }
    il = (*s).head;
    i = 0 as libc::c_int;
    while !il.is_null() {
        if (*(*il).input).version != version {
            printf(
                b"Test: version %d, level %c\n\0" as *const u8 as *const libc::c_char,
                version,
                levelChar[level as usize] as libc::c_int,
            );
            printf(b"wrong version number.\n\0" as *const u8 as *const libc::c_char);
            printQRinputInfo((*il).input);
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
        il = (*il).next;
    }
    codes = QRcode_encodeInputStructured(s);
    if codes.is_null() {
        perror(
            b"test_split_structure aborted at QRcode_encodeInputStructured():\0"
                as *const u8 as *const libc::c_char,
        );
        QRinput_free(input);
        QRinput_Struct_free(s);
        return;
    }
    list = codes;
    il = (*s).head;
    c = 0 as libc::c_int;
    while !list.is_null() {
        if (*(*list).code).version != version {
            printf(
                b"#%d: data mismatched.\n\0" as *const u8 as *const libc::c_char,
                num,
            );
            printf(
                b"Test: version %d, level %c\n\0" as *const u8 as *const libc::c_char,
                version,
                levelChar[level as usize] as libc::c_int,
            );
            printf(b"code #%d\n\0" as *const u8 as *const libc::c_char, c);
            printf(
                b"Version mismatch: %d should be %d\n\0" as *const u8
                    as *const libc::c_char,
                (*(*list).code).version,
                version,
            );
            printf(
                b"max bits: %d\n\0" as *const u8 as *const libc::c_char,
                QRspec_getDataLength(version, level) * 8 as libc::c_int
                    - 20 as libc::c_int,
            );
            printQRinputInfo((*il).input);
            printQRinput(input);
            exit(1 as libc::c_int);
        }
        list = (*list).next;
        il = (*il).next;
        c += 1;
        c;
    }
    QRinput_free(input);
    QRinput_Struct_free(s);
    QRcode_List_free(codes);
}
unsafe extern "C" fn monkey_split_structure(mut loop_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"monkey_split_structure\0"))
            .as_ptr(),
        b"Monkey test: QRinput_splitQRinputToStruct.\0" as *const u8
            as *const libc::c_char,
    );
    srand(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < loop_0 {
        test_split_structure(i);
        i += 1;
        i;
    }
    testEnd(0 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut loop_0: libc::c_int = 1000 as libc::c_int;
    if argc == 2 as libc::c_int {
        loop_0 = atoi(*argv.offset(1 as libc::c_int as isize));
    }
    let mut tests: libc::c_int = 7 as libc::c_int;
    testInit(tests);
    monkey_split_an(loop_0);
    monkey_encode_an(loop_0);
    monkey_split_8(loop_0);
    monkey_encode_8(loop_0);
    monkey_split_kanji(loop_0);
    monkey_encode_kanji(loop_0);
    monkey_split_structure(loop_0);
    testReport(tests);
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
