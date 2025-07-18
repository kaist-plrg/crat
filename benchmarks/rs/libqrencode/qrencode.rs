use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn QRinput_new2(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_newMQR(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: libc::c_int,
        data: *const libc::c_uchar,
    ) -> libc::c_int;
    fn QRinput_free(input: *mut QRinput);
    fn QRinput_Struct_free(s: *mut QRinput_Struct);
    fn QRinput_splitQRinputToStruct(input: *mut QRinput) -> *mut QRinput_Struct;
    fn QRspec_getDataLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn QRspec_getRemainder(version: libc::c_int) -> libc::c_int;
    fn QRspec_getEccSpec(version: libc::c_int, level: QRecLevel, spec: *mut libc::c_int);
    fn QRspec_newFrame(version: libc::c_int) -> *mut libc::c_uchar;
    fn MQRspec_getDataLengthBit(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getDataLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn MQRspec_newFrame(version: libc::c_int) -> *mut libc::c_uchar;
    fn QRinput_getByteStream(input: *mut QRinput) -> *mut libc::c_uchar;
    fn RSECC_encode(
        data_length: size_t,
        ecc_length: size_t,
        data: *const libc::c_uchar,
        ecc: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn Split_splitStringToQRinput(
        string: *const libc::c_char,
        input: *mut QRinput,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> libc::c_int;
    fn Mask_makeMask(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
        level: QRecLevel,
    ) -> *mut libc::c_uchar;
    fn Mask_mask(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        level: QRecLevel,
    ) -> *mut libc::c_uchar;
    fn MMask_makeMask(
        version: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
        level: QRecLevel,
    ) -> *mut libc::c_uchar;
    fn MMask_mask(
        version: libc::c_int,
        frame: *mut libc::c_uchar,
        level: QRecLevel,
    ) -> *mut libc::c_uchar;
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
pub struct RSblock {
    pub dataLength: libc::c_int,
    pub eccLength: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub ecc: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameFiller {
    pub width: libc::c_int,
    pub frame: *mut libc::c_uchar,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub dir: libc::c_int,
    pub bit: libc::c_int,
    pub mqr: libc::c_int,
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
unsafe extern "C" fn RSblock_initBlock(
    mut block: *mut RSblock,
    mut dl: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut el: libc::c_int,
    mut ecc: *mut libc::c_uchar,
) {
    (*block).dataLength = dl;
    (*block).data = data;
    (*block).eccLength = el;
    (*block).ecc = ecc;
    RSECC_encode(dl as size_t, el as size_t, data, ecc);
}
unsafe extern "C" fn RSblock_init(
    mut blocks: *mut RSblock,
    mut spec: *mut libc::c_int,
    mut data: *mut libc::c_uchar,
    mut ecc: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut block: *mut RSblock = 0 as *mut RSblock;
    let mut dp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut el: libc::c_int = 0;
    let mut dl: libc::c_int = 0;
    dl = *spec.offset(1 as libc::c_int as isize);
    el = *spec.offset(2 as libc::c_int as isize);
    block = blocks;
    dp = data;
    ep = ecc;
    i = 0 as libc::c_int;
    while i < *spec.offset(0 as libc::c_int as isize) {
        RSblock_initBlock(block, dl, dp, el, ep);
        dp = dp.offset(dl as isize);
        ep = ep.offset(el as isize);
        block = block.offset(1);
        block;
        i += 1;
        i;
    }
    if *spec.offset(3 as libc::c_int as isize) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    dl = *spec.offset(4 as libc::c_int as isize);
    el = *spec.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < *spec.offset(3 as libc::c_int as isize) {
        RSblock_initBlock(block, dl, dp, el, ep);
        dp = dp.offset(dl as isize);
        ep = ep.offset(el as isize);
        block = block.offset(1);
        block;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRraw_new(mut input: *mut QRinput) -> *mut QRRawCode {
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut spec: [libc::c_int; 5] = [0; 5];
    let mut ret: libc::c_int = 0;
    raw = malloc(::std::mem::size_of::<QRRawCode>() as libc::c_ulong) as *mut QRRawCode;
    if raw.is_null() {
        return 0 as *mut QRRawCode;
    }
    (*raw).datacode = QRinput_getByteStream(input);
    if ((*raw).datacode).is_null() {
        free(raw as *mut libc::c_void);
        return 0 as *mut QRRawCode;
    }
    QRspec_getEccSpec((*input).version, (*input).level, spec.as_mut_ptr());
    (*raw).version = (*input).version;
    (*raw).b1 = spec[0 as libc::c_int as usize];
    (*raw)
        .dataLength = spec[0 as libc::c_int as usize] * spec[1 as libc::c_int as usize]
        + spec[3 as libc::c_int as usize] * spec[4 as libc::c_int as usize];
    (*raw)
        .eccLength = (spec[0 as libc::c_int as usize] + spec[3 as libc::c_int as usize])
        * spec[2 as libc::c_int as usize];
    (*raw).ecccode = malloc((*raw).eccLength as size_t) as *mut libc::c_uchar;
    if ((*raw).ecccode).is_null() {
        free((*raw).datacode as *mut libc::c_void);
        free(raw as *mut libc::c_void);
        return 0 as *mut QRRawCode;
    }
    (*raw).blocks = spec[0 as libc::c_int as usize] + spec[3 as libc::c_int as usize];
    (*raw)
        .rsblock = calloc(
        (*raw).blocks as size_t,
        ::std::mem::size_of::<RSblock>() as libc::c_ulong,
    ) as *mut RSblock;
    if ((*raw).rsblock).is_null() {
        QRraw_free(raw);
        return 0 as *mut QRRawCode;
    }
    ret = RSblock_init(
        (*raw).rsblock,
        spec.as_mut_ptr(),
        (*raw).datacode,
        (*raw).ecccode,
    );
    if ret < 0 as libc::c_int {
        QRraw_free(raw);
        return 0 as *mut QRRawCode;
    }
    (*raw).count = 0 as libc::c_int;
    return raw;
}
pub unsafe extern "C" fn QRraw_getCode(mut raw: *mut QRRawCode) -> libc::c_uchar {
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut ret: libc::c_uchar = 0;
    if (*raw).count < (*raw).dataLength {
        row = (*raw).count % (*raw).blocks;
        col = (*raw).count / (*raw).blocks;
        if col >= (*((*raw).rsblock).offset(0 as libc::c_int as isize)).dataLength {
            row += (*raw).b1;
        }
        ret = *((*((*raw).rsblock).offset(row as isize)).data).offset(col as isize);
    } else if (*raw).count < (*raw).dataLength + (*raw).eccLength {
        row = ((*raw).count - (*raw).dataLength) % (*raw).blocks;
        col = ((*raw).count - (*raw).dataLength) / (*raw).blocks;
        ret = *((*((*raw).rsblock).offset(row as isize)).ecc).offset(col as isize);
    } else {
        return 0 as libc::c_int as libc::c_uchar
    }
    (*raw).count += 1;
    (*raw).count;
    return ret;
}
pub unsafe extern "C" fn QRraw_free(mut raw: *mut QRRawCode) {
    if !raw.is_null() {
        free((*raw).datacode as *mut libc::c_void);
        free((*raw).ecccode as *mut libc::c_void);
        free((*raw).rsblock as *mut libc::c_void);
        free(raw as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn MQRraw_new(mut input: *mut QRinput) -> *mut MQRRawCode {
    let mut raw: *mut MQRRawCode = 0 as *mut MQRRawCode;
    raw = malloc(::std::mem::size_of::<MQRRawCode>() as libc::c_ulong)
        as *mut MQRRawCode;
    if raw.is_null() {
        return 0 as *mut MQRRawCode;
    }
    (*raw).version = (*input).version;
    (*raw).dataLength = MQRspec_getDataLength((*input).version, (*input).level);
    (*raw).eccLength = MQRspec_getECCLength((*input).version, (*input).level);
    (*raw)
        .oddbits = (*raw).dataLength * 8 as libc::c_int
        - MQRspec_getDataLengthBit((*input).version, (*input).level);
    (*raw).datacode = QRinput_getByteStream(input);
    if ((*raw).datacode).is_null() {
        free(raw as *mut libc::c_void);
        return 0 as *mut MQRRawCode;
    }
    (*raw).ecccode = malloc((*raw).eccLength as size_t) as *mut libc::c_uchar;
    if ((*raw).ecccode).is_null() {
        free((*raw).datacode as *mut libc::c_void);
        free(raw as *mut libc::c_void);
        return 0 as *mut MQRRawCode;
    }
    (*raw)
        .rsblock = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<RSblock>() as libc::c_ulong,
    ) as *mut RSblock;
    if ((*raw).rsblock).is_null() {
        MQRraw_free(raw);
        return 0 as *mut MQRRawCode;
    }
    RSblock_initBlock(
        (*raw).rsblock,
        (*raw).dataLength,
        (*raw).datacode,
        (*raw).eccLength,
        (*raw).ecccode,
    );
    (*raw).count = 0 as libc::c_int;
    return raw;
}
pub unsafe extern "C" fn MQRraw_getCode(mut raw: *mut MQRRawCode) -> libc::c_uchar {
    let mut ret: libc::c_uchar = 0;
    if (*raw).count < (*raw).dataLength {
        ret = *((*raw).datacode).offset((*raw).count as isize);
    } else if (*raw).count < (*raw).dataLength + (*raw).eccLength {
        ret = *((*raw).ecccode).offset(((*raw).count - (*raw).dataLength) as isize);
    } else {
        return 0 as libc::c_int as libc::c_uchar
    }
    (*raw).count += 1;
    (*raw).count;
    return ret;
}
pub unsafe extern "C" fn MQRraw_free(mut raw: *mut MQRRawCode) {
    if !raw.is_null() {
        free((*raw).datacode as *mut libc::c_void);
        free((*raw).ecccode as *mut libc::c_void);
        free((*raw).rsblock as *mut libc::c_void);
        free(raw as *mut libc::c_void);
    }
}
unsafe extern "C" fn FrameFiller_set(
    mut filler: *mut FrameFiller,
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mqr: libc::c_int,
) {
    (*filler).width = width;
    (*filler).frame = frame;
    (*filler).x = width - 1 as libc::c_int;
    (*filler).y = width - 1 as libc::c_int;
    (*filler).dir = -(1 as libc::c_int);
    (*filler).bit = -(1 as libc::c_int);
    (*filler).mqr = mqr;
}
unsafe extern "C" fn FrameFiller_next(
    mut filler: *mut FrameFiller,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    if (*filler).bit == -(1 as libc::c_int) {
        (*filler).bit = 0 as libc::c_int;
        return ((*filler).frame)
            .offset(((*filler).y * (*filler).width) as isize)
            .offset((*filler).x as isize);
    }
    x = (*filler).x;
    y = (*filler).y;
    p = (*filler).frame;
    w = (*filler).width;
    if (*filler).bit == 0 as libc::c_int {
        x -= 1;
        x;
        (*filler).bit += 1;
        (*filler).bit;
    } else {
        x += 1;
        x;
        y += (*filler).dir;
        (*filler).bit -= 1;
        (*filler).bit;
    }
    if (*filler).dir < 0 as libc::c_int {
        if y < 0 as libc::c_int {
            y = 0 as libc::c_int;
            x -= 2 as libc::c_int;
            (*filler).dir = 1 as libc::c_int;
            if (*filler).mqr == 0 && x == 6 as libc::c_int {
                x -= 1;
                x;
                y = 9 as libc::c_int;
            }
        }
    } else if y == w {
        y = w - 1 as libc::c_int;
        x -= 2 as libc::c_int;
        (*filler).dir = -(1 as libc::c_int);
        if (*filler).mqr == 0 && x == 6 as libc::c_int {
            x -= 1;
            x;
            y -= 8 as libc::c_int;
        }
    }
    if x < 0 as libc::c_int || y < 0 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    (*filler).x = x;
    (*filler).y = y;
    if *p.offset((y * w + x) as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        return FrameFiller_next(filler);
    }
    return &mut *p.offset((y * w + x) as isize) as *mut libc::c_uchar;
}
pub unsafe extern "C" fn FrameFiller_test(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut width: libc::c_int = 0;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    width = QRspec_getWidth(version);
    frame = QRspec_newFrame(version);
    if frame.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    FrameFiller_set(&mut filler, width, frame, 0 as libc::c_int);
    length = QRspec_getDataLength(version, QR_ECLEVEL_L) * 8 as libc::c_int
        + QRspec_getECCLength(version, QR_ECLEVEL_L) * 8 as libc::c_int
        + QRspec_getRemainder(version);
    i = 0 as libc::c_int;
    while i < length {
        p = FrameFiller_next(&mut filler);
        if p.is_null() {
            free(frame as *mut libc::c_void);
            return 0 as *mut libc::c_uchar;
        }
        *p = ((i & 0x7f as libc::c_int) as libc::c_uchar as libc::c_int
            | 0x80 as libc::c_int) as libc::c_uchar;
        i += 1;
        i;
    }
    return frame;
}
pub unsafe extern "C" fn FrameFiller_testMQR(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut width: libc::c_int = 0;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    width = MQRspec_getWidth(version);
    frame = MQRspec_newFrame(version);
    if frame.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    FrameFiller_set(&mut filler, width, frame, 1 as libc::c_int);
    length = MQRspec_getDataLengthBit(version, QR_ECLEVEL_L)
        + MQRspec_getECCLength(version, QR_ECLEVEL_L) * 8 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        p = FrameFiller_next(&mut filler);
        if p.is_null() {
            fprintf(
                stderr,
                b"Frame filler run over the frame!\n\0" as *const u8
                    as *const libc::c_char,
            );
            return frame;
        }
        *p = ((i & 0x7f as libc::c_int) as libc::c_uchar as libc::c_int
            | 0x80 as libc::c_int) as libc::c_uchar;
        i += 1;
        i;
    }
    return frame;
}
pub unsafe extern "C" fn QRcode_new(
    mut version: libc::c_int,
    mut width: libc::c_int,
    mut data: *mut libc::c_uchar,
) -> *mut QRcode {
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    qrcode = malloc(::std::mem::size_of::<QRcode>() as libc::c_ulong) as *mut QRcode;
    if qrcode.is_null() {
        return 0 as *mut QRcode;
    }
    (*qrcode).version = version;
    (*qrcode).width = width;
    (*qrcode).data = data;
    return qrcode;
}
pub unsafe extern "C" fn QRcode_free(mut qrcode: *mut QRcode) {
    if !qrcode.is_null() {
        free((*qrcode).data as *mut libc::c_void);
        free(qrcode as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn QRcode_encodeMask(
    mut input: *mut QRinput,
    mut mask: libc::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut raw: *mut QRRawCode = 0 as *mut QRRawCode;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut code: libc::c_uchar = 0;
    let mut bit: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    if (*input).mqr != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if (*input).version < 0 as libc::c_int || (*input).version > 40 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if !((*input).level as libc::c_uint >= QR_ECLEVEL_L as libc::c_int as libc::c_uint
        && (*input).level as libc::c_uint <= QR_ECLEVEL_H as libc::c_int as libc::c_uint)
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    raw = QRraw_new(input);
    if raw.is_null() {
        return 0 as *mut QRcode;
    }
    version = (*raw).version;
    width = QRspec_getWidth(version);
    frame = QRspec_newFrame(version);
    if frame.is_null() {
        QRraw_free(raw);
        return 0 as *mut QRcode;
    }
    FrameFiller_set(&mut filler, width, frame, 0 as libc::c_int);
    i = 0 as libc::c_int;
    's_88: loop {
        if !(i < (*raw).dataLength) {
            current_block = 4068382217303356765;
            break;
        }
        code = QRraw_getCode(raw);
        bit = 0x80 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            p = FrameFiller_next(&mut filler);
            if p.is_null() {
                current_block = 16955815368617111662;
                break 's_88;
            }
            *p = (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                as libc::c_int as libc::c_uchar;
            bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    match current_block {
        4068382217303356765 => {
            i = 0 as libc::c_int;
            's_134: loop {
                if !(i < (*raw).eccLength) {
                    current_block = 7245201122033322888;
                    break;
                }
                code = QRraw_getCode(raw);
                bit = 0x80 as libc::c_int as libc::c_uchar;
                j = 0 as libc::c_int;
                while j < 8 as libc::c_int {
                    p = FrameFiller_next(&mut filler);
                    if p.is_null() {
                        current_block = 16955815368617111662;
                        break 's_134;
                    }
                    *p = (0x2 as libc::c_int
                        | (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                            as libc::c_int) as libc::c_uchar;
                    bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            match current_block {
                16955815368617111662 => {}
                _ => {
                    QRraw_free(raw);
                    raw = 0 as *mut QRRawCode;
                    j = QRspec_getRemainder(version);
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < j) {
                            current_block = 4567019141635105728;
                            break;
                        }
                        p = FrameFiller_next(&mut filler);
                        if p.is_null() {
                            current_block = 16955815368617111662;
                            break;
                        }
                        *p = 0x2 as libc::c_int as libc::c_uchar;
                        i += 1;
                        i;
                    }
                    match current_block {
                        16955815368617111662 => {}
                        _ => {
                            if mask == -(2 as libc::c_int) {
                                masked = malloc((width * width) as size_t)
                                    as *mut libc::c_uchar;
                                memcpy(
                                    masked as *mut libc::c_void,
                                    frame as *const libc::c_void,
                                    (width * width) as size_t,
                                );
                            } else if mask < 0 as libc::c_int {
                                masked = Mask_mask(width, frame, (*input).level);
                            } else {
                                masked = Mask_makeMask(width, frame, mask, (*input).level);
                            }
                            if !masked.is_null() {
                                qrcode = QRcode_new(version, width, masked);
                                if qrcode.is_null() {
                                    free(masked as *mut libc::c_void);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    QRraw_free(raw);
    free(frame as *mut libc::c_void);
    return qrcode;
}
pub unsafe extern "C" fn QRcode_encodeMaskMQR(
    mut input: *mut QRinput,
    mut mask: libc::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut raw: *mut MQRRawCode = 0 as *mut MQRRawCode;
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut code: libc::c_uchar = 0;
    let mut bit: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut qrcode: *mut QRcode = 0 as *mut QRcode;
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: 0 as *mut libc::c_uchar,
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    if (*input).mqr == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if (*input).version <= 0 as libc::c_int || (*input).version > 4 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if !((*input).level as libc::c_uint >= QR_ECLEVEL_L as libc::c_int as libc::c_uint
        && (*input).level as libc::c_uint <= QR_ECLEVEL_Q as libc::c_int as libc::c_uint)
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    raw = MQRraw_new(input);
    if raw.is_null() {
        return 0 as *mut QRcode;
    }
    version = (*raw).version;
    width = MQRspec_getWidth(version);
    frame = MQRspec_newFrame(version);
    if frame.is_null() {
        MQRraw_free(raw);
        return 0 as *mut QRcode;
    }
    FrameFiller_set(&mut filler, width, frame, 1 as libc::c_int);
    i = 0 as libc::c_int;
    's_88: loop {
        if !(i < (*raw).dataLength) {
            current_block = 17788412896529399552;
            break;
        }
        code = MQRraw_getCode(raw);
        bit = 0x80 as libc::c_int as libc::c_uchar;
        if (*raw).oddbits != 0 && i == (*raw).dataLength - 1 as libc::c_int {
            length = (*raw).oddbits;
        } else {
            length = 8 as libc::c_int;
        }
        j = 0 as libc::c_int;
        while j < length {
            p = FrameFiller_next(&mut filler);
            if p.is_null() {
                current_block = 14631690749086992643;
                break 's_88;
            }
            *p = (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                as libc::c_int as libc::c_uchar;
            bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    match current_block {
        17788412896529399552 => {
            i = 0 as libc::c_int;
            's_154: loop {
                if !(i < (*raw).eccLength) {
                    current_block = 15090052786889560393;
                    break;
                }
                code = MQRraw_getCode(raw);
                bit = 0x80 as libc::c_int as libc::c_uchar;
                length = 8 as libc::c_int;
                j = 0 as libc::c_int;
                while j < length {
                    p = FrameFiller_next(&mut filler);
                    if p.is_null() {
                        current_block = 14631690749086992643;
                        break 's_154;
                    }
                    *p = (0x2 as libc::c_int
                        | (bit as libc::c_int & code as libc::c_int != 0 as libc::c_int)
                            as libc::c_int) as libc::c_uchar;
                    bit = (bit as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            match current_block {
                14631690749086992643 => {}
                _ => {
                    MQRraw_free(raw);
                    raw = 0 as *mut MQRRawCode;
                    if mask == -(2 as libc::c_int) {
                        masked = malloc((width * width) as size_t) as *mut libc::c_uchar;
                        memcpy(
                            masked as *mut libc::c_void,
                            frame as *const libc::c_void,
                            (width * width) as size_t,
                        );
                    } else if mask < 0 as libc::c_int {
                        masked = MMask_mask(version, frame, (*input).level);
                    } else {
                        masked = MMask_makeMask(version, frame, mask, (*input).level);
                    }
                    if !masked.is_null() {
                        qrcode = QRcode_new(version, width, masked);
                        if qrcode.is_null() {
                            free(masked as *mut libc::c_void);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    MQRraw_free(raw);
    free(frame as *mut libc::c_void);
    return qrcode;
}
pub unsafe extern "C" fn QRcode_encodeInput(mut input: *mut QRinput) -> *mut QRcode {
    if (*input).mqr != 0 {
        return QRcode_encodeMaskMQR(input, -(1 as libc::c_int))
    } else {
        return QRcode_encodeMask(input, -(1 as libc::c_int))
    };
}
unsafe extern "C" fn QRcode_encodeStringReal(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut mqr: libc::c_int,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut ret: libc::c_int = 0;
    if string.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if hint as libc::c_int != QR_MODE_8 as libc::c_int
        && hint as libc::c_int != QR_MODE_KANJI as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if mqr != 0 {
        input = QRinput_newMQR(version, level);
    } else {
        input = QRinput_new2(version, level);
    }
    if input.is_null() {
        return 0 as *mut QRcode;
    }
    ret = Split_splitStringToQRinput(string, input, hint, casesensitive);
    if ret < 0 as libc::c_int {
        QRinput_free(input);
        return 0 as *mut QRcode;
    }
    code = QRcode_encodeInput(input);
    QRinput_free(input);
    return code;
}
pub unsafe extern "C" fn QRcode_encodeString(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode {
    return QRcode_encodeStringReal(
        string,
        version,
        level,
        0 as libc::c_int,
        hint,
        casesensitive,
    );
}
pub unsafe extern "C" fn QRcode_encodeStringMQR(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    i = version;
    while i <= 4 as libc::c_int {
        let mut code: *mut QRcode = QRcode_encodeStringReal(
            string,
            i,
            level,
            1 as libc::c_int,
            hint,
            casesensitive,
        );
        if !code.is_null() {
            return code;
        }
        i += 1;
        i;
    }
    return 0 as *mut QRcode;
}
unsafe extern "C" fn QRcode_encodeDataReal(
    mut data: *const libc::c_uchar,
    mut length: libc::c_int,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut mqr: libc::c_int,
) -> *mut QRcode {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut ret: libc::c_int = 0;
    if data.is_null() || length == 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if mqr != 0 {
        input = QRinput_newMQR(version, level);
    } else {
        input = QRinput_new2(version, level);
    }
    if input.is_null() {
        return 0 as *mut QRcode;
    }
    ret = QRinput_append(input, QR_MODE_8, length, data);
    if ret < 0 as libc::c_int {
        QRinput_free(input);
        return 0 as *mut QRcode;
    }
    code = QRcode_encodeInput(input);
    QRinput_free(input);
    return code;
}
pub unsafe extern "C" fn QRcode_encodeData(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    return QRcode_encodeDataReal(data, size, version, level, 0 as libc::c_int);
}
pub unsafe extern "C" fn QRcode_encodeString8bit(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    if string.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    return QRcode_encodeDataReal(
        string as *mut libc::c_uchar,
        strlen(string) as libc::c_int,
        version,
        level,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn QRcode_encodeDataMQR(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    i = version;
    while i <= 4 as libc::c_int {
        let mut code: *mut QRcode = QRcode_encodeDataReal(
            data,
            size,
            i,
            level,
            1 as libc::c_int,
        );
        if !code.is_null() {
            return code;
        }
        i += 1;
        i;
    }
    return 0 as *mut QRcode;
}
pub unsafe extern "C" fn QRcode_encodeString8bitMQR(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut i: libc::c_int = 0;
    if string.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode;
    }
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    i = version;
    while i <= 4 as libc::c_int {
        let mut code: *mut QRcode = QRcode_encodeDataReal(
            string as *mut libc::c_uchar,
            strlen(string) as libc::c_int,
            i,
            level,
            1 as libc::c_int,
        );
        if !code.is_null() {
            return code;
        }
        i += 1;
        i;
    }
    return 0 as *mut QRcode;
}
unsafe extern "C" fn QRcode_List_newEntry() -> *mut QRcode_List {
    let mut entry: *mut QRcode_List = 0 as *mut QRcode_List;
    entry = malloc(::std::mem::size_of::<QRcode_List>() as libc::c_ulong)
        as *mut QRcode_List;
    if entry.is_null() {
        return 0 as *mut QRcode_List;
    }
    (*entry).next = 0 as *mut _QRcode_List;
    (*entry).code = 0 as *mut QRcode;
    return entry;
}
unsafe extern "C" fn QRcode_List_freeEntry(mut entry: *mut QRcode_List) {
    if !entry.is_null() {
        QRcode_free((*entry).code);
        free(entry as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn QRcode_List_free(mut qrlist: *mut QRcode_List) {
    let mut list: *mut QRcode_List = qrlist;
    let mut next: *mut QRcode_List = 0 as *mut QRcode_List;
    while !list.is_null() {
        next = (*list).next;
        QRcode_List_freeEntry(list);
        list = next;
    }
}
pub unsafe extern "C" fn QRcode_List_size(mut qrlist: *mut QRcode_List) -> libc::c_int {
    let mut list: *mut QRcode_List = qrlist;
    let mut size: libc::c_int = 0 as libc::c_int;
    while !list.is_null() {
        size += 1;
        size;
        list = (*list).next;
    }
    return size;
}
pub unsafe extern "C" fn QRcode_encodeInputStructured(
    mut s: *mut QRinput_Struct,
) -> *mut QRcode_List {
    let mut current_block: u64;
    let mut head: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut tail: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut entry: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut list: *mut QRinput_InputList = (*s).head;
    loop {
        if list.is_null() {
            current_block = 12800627514080957624;
            break;
        }
        if head.is_null() {
            entry = QRcode_List_newEntry();
            if entry.is_null() {
                current_block = 11636989796256057796;
                break;
            }
            head = entry;
            tail = head;
        } else {
            entry = QRcode_List_newEntry();
            if entry.is_null() {
                current_block = 11636989796256057796;
                break;
            }
            (*tail).next = entry;
            tail = (*tail).next;
        }
        (*tail).code = QRcode_encodeInput((*list).input);
        if ((*tail).code).is_null() {
            current_block = 11636989796256057796;
            break;
        }
        list = (*list).next;
    }
    match current_block {
        12800627514080957624 => return head,
        _ => {
            QRcode_List_free(head);
            return 0 as *mut QRcode_List;
        }
    };
}
unsafe extern "C" fn QRcode_encodeInputToStructured(
    mut input: *mut QRinput,
) -> *mut QRcode_List {
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    s = QRinput_splitQRinputToStruct(input);
    if s.is_null() {
        return 0 as *mut QRcode_List;
    }
    codes = QRcode_encodeInputStructured(s);
    QRinput_Struct_free(s);
    return codes;
}
unsafe extern "C" fn QRcode_encodeDataStructuredReal(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut eightbit: libc::c_int,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode_List {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut codes: *mut QRcode_List = 0 as *mut QRcode_List;
    let mut ret: libc::c_int = 0;
    if version <= 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode_List;
    }
    if eightbit == 0
        && (hint as libc::c_int != QR_MODE_8 as libc::c_int
            && hint as libc::c_int != QR_MODE_KANJI as libc::c_int)
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode_List;
    }
    input = QRinput_new2(version, level);
    if input.is_null() {
        return 0 as *mut QRcode_List;
    }
    if eightbit != 0 {
        ret = QRinput_append(input, QR_MODE_8, size, data);
    } else {
        ret = Split_splitStringToQRinput(
            data as *mut libc::c_char,
            input,
            hint,
            casesensitive,
        );
    }
    if ret < 0 as libc::c_int {
        QRinput_free(input);
        return 0 as *mut QRcode_List;
    }
    codes = QRcode_encodeInputToStructured(input);
    QRinput_free(input);
    return codes;
}
pub unsafe extern "C" fn QRcode_encodeDataStructured(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode_List {
    return QRcode_encodeDataStructuredReal(
        size,
        data,
        version,
        level,
        1 as libc::c_int,
        QR_MODE_NUL,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn QRcode_encodeString8bitStructured(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRcode_List {
    if string.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode_List;
    }
    return QRcode_encodeDataStructured(
        strlen(string) as libc::c_int,
        string as *mut libc::c_uchar,
        version,
        level,
    );
}
pub unsafe extern "C" fn QRcode_encodeStringStructured(
    mut string: *const libc::c_char,
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: libc::c_int,
) -> *mut QRcode_List {
    if string.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRcode_List;
    }
    return QRcode_encodeDataStructuredReal(
        strlen(string) as libc::c_int,
        string as *mut libc::c_uchar,
        version,
        level,
        0 as libc::c_int,
        hint,
        casesensitive,
    );
}
pub unsafe extern "C" fn QRcode_APIVersion(
    mut major_version: *mut libc::c_int,
    mut minor_version: *mut libc::c_int,
    mut micro_version: *mut libc::c_int,
) {
    if !major_version.is_null() {
        *major_version = 4 as libc::c_int;
    }
    if !minor_version.is_null() {
        *minor_version = 1 as libc::c_int;
    }
    if !micro_version.is_null() {
        *micro_version = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn QRcode_APIVersionString() -> *mut libc::c_char {
    return b"4.1.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn QRcode_clearCache() {}
