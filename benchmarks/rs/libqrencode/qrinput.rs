use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn QRspec_getDataLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_getMinimumVersion(size: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn QRspec_lengthIndicator(mode: QRencodeMode, version: libc::c_int) -> libc::c_int;
    fn QRspec_maximumWords(mode: QRencodeMode, version: libc::c_int) -> libc::c_int;
    fn MQRspec_getDataLengthBit(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_getECCLength(version: libc::c_int, level: QRecLevel) -> libc::c_int;
    fn MQRspec_lengthIndicator(mode: QRencodeMode, version: libc::c_int) -> libc::c_int;
    fn MQRspec_maximumWords(mode: QRencodeMode, version: libc::c_int) -> libc::c_int;
    fn BitStream_new() -> *mut BitStream;
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
pub unsafe extern "C" fn QRinput_isSplittableMode(
    mut mode: QRencodeMode,
) -> libc::c_int {
    return (mode as libc::c_int >= QR_MODE_NUM as libc::c_int
        && mode as libc::c_int <= QR_MODE_KANJI as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn QRinput_List_newEntry(
    mut mode: QRencodeMode,
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> *mut QRinput_List {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    if QRinput_check(mode, size, data) != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRinput_List;
    }
    entry = malloc(::std::mem::size_of::<QRinput_List>() as libc::c_ulong)
        as *mut QRinput_List;
    if entry.is_null() {
        return 0 as *mut QRinput_List;
    }
    (*entry).mode = mode;
    (*entry).size = size;
    (*entry).data = 0 as *mut libc::c_uchar;
    if size > 0 as libc::c_int {
        (*entry).data = malloc(size as size_t) as *mut libc::c_uchar;
        if ((*entry).data).is_null() {
            free(entry as *mut libc::c_void);
            return 0 as *mut QRinput_List;
        }
        memcpy(
            (*entry).data as *mut libc::c_void,
            data as *const libc::c_void,
            size as size_t,
        );
    }
    (*entry).bstream = 0 as *mut BitStream;
    (*entry).next = 0 as *mut QRinput_List;
    return entry;
}
unsafe extern "C" fn QRinput_List_freeEntry(mut entry: *mut QRinput_List) {
    if !entry.is_null() {
        free((*entry).data as *mut libc::c_void);
        BitStream_free((*entry).bstream);
        free(entry as *mut libc::c_void);
    }
}
unsafe extern "C" fn QRinput_List_dup(
    mut entry: *mut QRinput_List,
) -> *mut QRinput_List {
    let mut n: *mut QRinput_List = 0 as *mut QRinput_List;
    n = malloc(::std::mem::size_of::<QRinput_List>() as libc::c_ulong)
        as *mut QRinput_List;
    if n.is_null() {
        return 0 as *mut QRinput_List;
    }
    (*n).mode = (*entry).mode;
    (*n).size = (*entry).size;
    (*n).data = malloc((*n).size as size_t) as *mut libc::c_uchar;
    if ((*n).data).is_null() {
        free(n as *mut libc::c_void);
        return 0 as *mut QRinput_List;
    }
    memcpy(
        (*n).data as *mut libc::c_void,
        (*entry).data as *const libc::c_void,
        (*entry).size as size_t,
    );
    (*n).bstream = 0 as *mut BitStream;
    (*n).next = 0 as *mut QRinput_List;
    return n;
}
pub unsafe extern "C" fn QRinput_new() -> *mut QRinput {
    return QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
}
pub unsafe extern "C" fn QRinput_new2(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    if version < 0 as libc::c_int || version > 40 as libc::c_int
        || (level as libc::c_uint) < 0 as libc::c_int as libc::c_uint
        || level as libc::c_uint > QR_ECLEVEL_H as libc::c_int as libc::c_uint
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRinput;
    }
    input = malloc(::std::mem::size_of::<QRinput>() as libc::c_ulong) as *mut QRinput;
    if input.is_null() {
        return 0 as *mut QRinput;
    }
    (*input).head = 0 as *mut QRinput_List;
    (*input).tail = 0 as *mut QRinput_List;
    (*input).version = version;
    (*input).level = level;
    (*input).mqr = 0 as libc::c_int;
    (*input).fnc1 = 0 as libc::c_int;
    return input;
}
pub unsafe extern "C" fn QRinput_newMQR(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    if !(version <= 0 as libc::c_int || version > 4 as libc::c_int) {
        if !(MQRspec_getECCLength(version, level) == 0 as libc::c_int) {
            input = QRinput_new2(version, level);
            if input.is_null() {
                return 0 as *mut QRinput;
            }
            (*input).mqr = 1 as libc::c_int;
            return input;
        }
    }
    *__errno_location() = 22 as libc::c_int;
    return 0 as *mut QRinput;
}
pub unsafe extern "C" fn QRinput_getVersion(mut input: *mut QRinput) -> libc::c_int {
    return (*input).version;
}
pub unsafe extern "C" fn QRinput_setVersion(
    mut input: *mut QRinput,
    mut version: libc::c_int,
) -> libc::c_int {
    if (*input).mqr != 0 || version < 0 as libc::c_int || version > 40 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*input).version = version;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_getErrorCorrectionLevel(
    mut input: *mut QRinput,
) -> QRecLevel {
    return (*input).level;
}
pub unsafe extern "C" fn QRinput_setErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut level: QRecLevel,
) -> libc::c_int {
    if (*input).mqr != 0
        || level as libc::c_uint > QR_ECLEVEL_H as libc::c_int as libc::c_uint
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*input).level = level;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_setVersionAndErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut current_block: u64;
    if (*input).mqr != 0 {
        if version <= 0 as libc::c_int || version > 4 as libc::c_int {
            current_block = 17185570528830662309;
        } else if MQRspec_getECCLength(version, level) == 0 as libc::c_int {
            current_block = 17185570528830662309;
        } else {
            current_block = 11875828834189669668;
        }
    } else if version < 0 as libc::c_int || version > 40 as libc::c_int {
        current_block = 17185570528830662309;
    } else if level as libc::c_uint > QR_ECLEVEL_H as libc::c_int as libc::c_uint {
        current_block = 17185570528830662309;
    } else {
        current_block = 11875828834189669668;
    }
    match current_block {
        11875828834189669668 => {
            (*input).version = version;
            (*input).level = level;
            return 0 as libc::c_int;
        }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
    };
}
unsafe extern "C" fn QRinput_appendEntry(
    mut input: *mut QRinput,
    mut entry: *mut QRinput_List,
) {
    if ((*input).tail).is_null() {
        (*input).head = entry;
        (*input).tail = entry;
    } else {
        (*(*input).tail).next = entry;
        (*input).tail = entry;
    }
    (*entry).next = 0 as *mut QRinput_List;
}
pub unsafe extern "C" fn QRinput_append(
    mut input: *mut QRinput,
    mut mode: QRencodeMode,
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    entry = QRinput_List_newEntry(mode, size, data);
    if entry.is_null() {
        return -(1 as libc::c_int);
    }
    QRinput_appendEntry(input, entry);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_insertStructuredAppendHeader(
    mut input: *mut QRinput,
    mut size: libc::c_int,
    mut number: libc::c_int,
    mut parity: libc::c_uchar,
) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut buf: [libc::c_uchar; 3] = [0; 3];
    if size > 16 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if number <= 0 as libc::c_int || number > size {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    buf[0 as libc::c_int as usize] = size as libc::c_uchar;
    buf[1 as libc::c_int as usize] = number as libc::c_uchar;
    buf[2 as libc::c_int as usize] = parity;
    entry = QRinput_List_newEntry(QR_MODE_STRUCTURE, 3 as libc::c_int, buf.as_mut_ptr());
    if entry.is_null() {
        return -(1 as libc::c_int);
    }
    (*entry).next = (*input).head;
    (*input).head = entry;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_appendECIheader(
    mut input: *mut QRinput,
    mut ecinum: libc::c_uint,
) -> libc::c_int {
    let mut data: [libc::c_uchar; 4] = [0; 4];
    if ecinum > 999999 as libc::c_int as libc::c_uint {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    data[0 as libc::c_int
        as usize] = (ecinum & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    data[1 as libc::c_int
        as usize] = (ecinum >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    data[2 as libc::c_int
        as usize] = (ecinum >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    data[3 as libc::c_int
        as usize] = (ecinum >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    return QRinput_append(input, QR_MODE_ECI, 4 as libc::c_int, data.as_mut_ptr());
}
pub unsafe extern "C" fn QRinput_free(mut input: *mut QRinput) {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut next: *mut QRinput_List = 0 as *mut QRinput_List;
    if !input.is_null() {
        list = (*input).head;
        while !list.is_null() {
            next = (*list).next;
            QRinput_List_freeEntry(list);
            list = next;
        }
        free(input as *mut libc::c_void);
    }
}
unsafe extern "C" fn QRinput_calcParity(mut input: *mut QRinput) -> libc::c_uchar {
    let mut parity: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut i: libc::c_int = 0;
    list = (*input).head;
    while !list.is_null() {
        if (*list).mode as libc::c_int != QR_MODE_STRUCTURE as libc::c_int {
            i = (*list).size - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                parity = (parity as libc::c_int
                    ^ *((*list).data).offset(i as isize) as libc::c_int)
                    as libc::c_uchar;
                i -= 1;
                i;
            }
        }
        list = (*list).next;
    }
    return parity;
}
pub unsafe extern "C" fn QRinput_dup(mut input: *mut QRinput) -> *mut QRinput {
    let mut n: *mut QRinput = 0 as *mut QRinput;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut e: *mut QRinput_List = 0 as *mut QRinput_List;
    if (*input).mqr != 0 {
        n = QRinput_newMQR((*input).version, (*input).level);
    } else {
        n = QRinput_new2((*input).version, (*input).level);
    }
    if n.is_null() {
        return 0 as *mut QRinput;
    }
    list = (*input).head;
    while !list.is_null() {
        e = QRinput_List_dup(list);
        if e.is_null() {
            QRinput_free(n);
            return 0 as *mut QRinput;
        }
        QRinput_appendEntry(n, e);
        list = (*list).next;
    }
    return n;
}
unsafe extern "C" fn QRinput_checkModeNum(
    mut size: libc::c_int,
    mut data: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if (*data.offset(i as isize) as libc::c_int) < '0' as i32
            || *data.offset(i as isize) as libc::c_int > '9' as i32
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsModeNum(
    mut size: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    w = size / 3 as libc::c_int;
    bits = w * 10 as libc::c_int;
    match size - w * 3 as libc::c_int {
        1 => {
            bits += 4 as libc::c_int;
        }
        2 => {
            bits += 7 as libc::c_int;
        }
        _ => {}
    }
    return bits;
}
unsafe extern "C" fn QRinput_encodeModeNum(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut words: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    if mqr != 0 {
        if version > 1 as libc::c_int {
            ret = BitStream_appendNum(
                bstream,
                (version - 1 as libc::c_int) as size_t,
                0 as libc::c_int as libc::c_uint,
            );
            if ret < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_NUM, version) as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            1 as libc::c_int as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_NUM, version) as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    words = (*entry).size / 3 as libc::c_int;
    i = 0 as libc::c_int;
    while i < words {
        val = ((*((*entry).data).offset((i * 3 as libc::c_int) as isize) as libc::c_int
            - '0' as i32) as libc::c_uint)
            .wrapping_mul(100 as libc::c_int as libc::c_uint);
        val = val
            .wrapping_add(
                ((*((*entry).data)
                    .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int - '0' as i32) as libc::c_uint)
                    .wrapping_mul(10 as libc::c_int as libc::c_uint),
            );
        val = val
            .wrapping_add(
                (*((*entry).data)
                    .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_int - '0' as i32) as libc::c_uint,
            );
        ret = BitStream_appendNum(bstream, 10 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    if (*entry).size - words * 3 as libc::c_int == 1 as libc::c_int {
        val = (*((*entry).data).offset((words * 3 as libc::c_int) as isize)
            as libc::c_int - '0' as i32) as libc::c_uint;
        ret = BitStream_appendNum(bstream, 4 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else if (*entry).size - words * 3 as libc::c_int == 2 as libc::c_int {
        val = ((*((*entry).data).offset((words * 3 as libc::c_int) as isize)
            as libc::c_int - '0' as i32) as libc::c_uint)
            .wrapping_mul(10 as libc::c_int as libc::c_uint);
        val = val
            .wrapping_add(
                (*((*entry).data)
                    .offset((words * 3 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int - '0' as i32) as libc::c_uint,
            );
        ret = BitStream_appendNum(bstream, 7 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub static mut QRinput_anTable: [libc::c_schar; 128] = [
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
];
unsafe extern "C" fn QRinput_checkModeAn(
    mut size: libc::c_int,
    mut data: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if (if *data.offset(i as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
            -(1 as libc::c_int)
        } else {
            QRinput_anTable[*data.offset(i as isize) as libc::c_int as usize]
                as libc::c_int
        }) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsModeAn(
    mut size: libc::c_int,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    w = size / 2 as libc::c_int;
    bits = w * 11 as libc::c_int;
    if size & 1 as libc::c_int != 0 {
        bits += 6 as libc::c_int;
    }
    return bits;
}
unsafe extern "C" fn QRinput_encodeModeAn(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut words: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    if mqr != 0 {
        if version < 2 as libc::c_int {
            *__errno_location() = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as libc::c_int) as size_t,
            1 as libc::c_int as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_AN, version) as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            2 as libc::c_int as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_AN, version) as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    words = (*entry).size / 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < words {
        val = ((if *((*entry).data).offset((i * 2 as libc::c_int) as isize)
            as libc::c_int & 0x80 as libc::c_int != 0
        {
            -(1 as libc::c_int)
        } else {
            QRinput_anTable[*((*entry).data).offset((i * 2 as libc::c_int) as isize)
                as libc::c_int as usize] as libc::c_int
        }) as libc::c_uint)
            .wrapping_mul(45 as libc::c_int as libc::c_uint);
        val = val
            .wrapping_add(
                (if *((*entry).data)
                    .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int & 0x80 as libc::c_int != 0
                {
                    -(1 as libc::c_int)
                } else {
                    QRinput_anTable[*((*entry).data)
                        .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int as usize] as libc::c_int
                }) as libc::c_uint,
            );
        ret = BitStream_appendNum(bstream, 11 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    if (*entry).size & 1 as libc::c_int != 0 {
        val = (if *((*entry).data).offset((words * 2 as libc::c_int) as isize)
            as libc::c_int & 0x80 as libc::c_int != 0
        {
            -(1 as libc::c_int)
        } else {
            QRinput_anTable[*((*entry).data).offset((words * 2 as libc::c_int) as isize)
                as libc::c_int as usize] as libc::c_int
        }) as libc::c_uint;
        ret = BitStream_appendNum(bstream, 6 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsMode8(
    mut size: libc::c_int,
) -> libc::c_int {
    return size * 8 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeMode8(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if mqr != 0 {
        if version < 3 as libc::c_int {
            *__errno_location() = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as libc::c_int) as size_t,
            2 as libc::c_int as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_8, version) as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            4 as libc::c_int as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_8, version) as size_t,
            (*entry).size as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    ret = BitStream_appendBytes(bstream, (*entry).size as size_t, (*entry).data);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_estimateBitsModeKanji(
    mut size: libc::c_int,
) -> libc::c_int {
    return size / 2 as libc::c_int * 13 as libc::c_int;
}
unsafe extern "C" fn QRinput_checkModeKanji(
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    if size & 1 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < size {
        val = (*data.offset(i as isize) as libc::c_uint) << 8 as libc::c_int
            | *data.offset((i + 1 as libc::c_int) as isize) as libc::c_uint;
        if val < 0x8140 as libc::c_int as libc::c_uint
            || val > 0x9ffc as libc::c_int as libc::c_uint
                && val < 0xe040 as libc::c_int as libc::c_uint
            || val > 0xebbf as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int);
        }
        i += 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeModeKanji(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    if mqr != 0 {
        if version < 2 as libc::c_int {
            *__errno_location() = 34 as libc::c_int;
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as libc::c_int) as size_t,
            3 as libc::c_int as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_KANJI, version) as size_t,
            ((*entry).size as libc::c_uint)
                .wrapping_div(2 as libc::c_int as libc::c_uint),
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as libc::c_int as size_t,
            8 as libc::c_int as libc::c_uint,
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_KANJI, version) as size_t,
            ((*entry).size as libc::c_uint)
                .wrapping_div(2 as libc::c_int as libc::c_uint),
        );
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    while i < (*entry).size {
        val = (*((*entry).data).offset(i as isize) as libc::c_uint) << 8 as libc::c_int
            | *((*entry).data).offset((i + 1 as libc::c_int) as isize) as libc::c_uint;
        if val <= 0x9ffc as libc::c_int as libc::c_uint {
            val = val.wrapping_sub(0x8140 as libc::c_int as libc::c_uint);
        } else {
            val = val.wrapping_sub(0xc140 as libc::c_int as libc::c_uint);
        }
        h = (val >> 8 as libc::c_int).wrapping_mul(0xc0 as libc::c_int as libc::c_uint);
        val = (val & 0xff as libc::c_int as libc::c_uint).wrapping_add(h);
        ret = BitStream_appendNum(bstream, 13 as libc::c_int as size_t, val);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeModeStructure(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if mqr != 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        (*((*entry).data).offset(1 as libc::c_int as isize) as libc::c_uint)
            .wrapping_sub(1 as libc::c_uint),
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        (*((*entry).data).offset(0 as libc::c_int as isize) as libc::c_uint)
            .wrapping_sub(1 as libc::c_uint),
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        8 as libc::c_int as size_t,
        *((*entry).data).offset(2 as libc::c_int as isize) as libc::c_uint,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_checkModeFNC1Second(mut size: libc::c_int) -> libc::c_int {
    if size != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_encodeModeFNC1Second(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        9 as libc::c_int as libc::c_uint,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendBytes(bstream, 1 as libc::c_int as size_t, (*entry).data);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_decodeECIfromByteArray(
    mut data: *mut libc::c_uchar,
) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut ecinum: libc::c_uint = 0;
    ecinum = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        ecinum = ecinum << 8 as libc::c_int;
        ecinum |= *data.offset((3 as libc::c_int - i) as isize) as libc::c_uint;
        i += 1;
        i;
    }
    return ecinum;
}
unsafe extern "C" fn QRinput_estimateBitsModeECI(
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut ecinum: libc::c_uint = 0;
    ecinum = QRinput_decodeECIfromByteArray(data);
    if ecinum < 128 as libc::c_int as libc::c_uint {
        return 4 as libc::c_int + 8 as libc::c_int
    } else if ecinum < 16384 as libc::c_int as libc::c_uint {
        return 4 as libc::c_int + 16 as libc::c_int
    } else {
        return 4 as libc::c_int + 24 as libc::c_int
    };
}
unsafe extern "C" fn QRinput_encodeModeECI(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut ecinum: libc::c_uint = 0;
    let mut code: libc::c_uint = 0;
    ecinum = QRinput_decodeECIfromByteArray((*entry).data);
    if ecinum < 128 as libc::c_int as libc::c_uint {
        words = 1 as libc::c_int;
        code = ecinum;
    } else if ecinum < 16384 as libc::c_int as libc::c_uint {
        words = 2 as libc::c_int;
        code = (0x8000 as libc::c_int as libc::c_uint).wrapping_add(ecinum);
    } else {
        words = 3 as libc::c_int;
        code = (0xc0000 as libc::c_int as libc::c_uint).wrapping_add(ecinum);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as libc::c_int as size_t,
        7 as libc::c_int as libc::c_uint,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        (words as size_t).wrapping_mul(8 as libc::c_int as libc::c_ulong),
        code,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_check(
    mut mode: QRencodeMode,
    mut size: libc::c_int,
    mut data: *const libc::c_uchar,
) -> libc::c_int {
    if mode as libc::c_int == QR_MODE_FNC1FIRST as libc::c_int && size < 0 as libc::c_int
        || size <= 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    match mode as libc::c_int {
        0 => return QRinput_checkModeNum(size, data as *const libc::c_char),
        1 => return QRinput_checkModeAn(size, data as *const libc::c_char),
        3 => return QRinput_checkModeKanji(size, data),
        2 => return 0 as libc::c_int,
        4 => return 0 as libc::c_int,
        5 => return 0 as libc::c_int,
        6 => return 0 as libc::c_int,
        7 => return QRinput_checkModeFNC1Second(size),
        -1 | _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn QRinput_estimateBitStreamSizeOfEntry(
    mut entry: *mut QRinput_List,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if version == 0 as libc::c_int {
        version = 1 as libc::c_int;
    }
    match (*entry).mode as libc::c_int {
        0 => {
            bits = QRinput_estimateBitsModeNum((*entry).size);
        }
        1 => {
            bits = QRinput_estimateBitsModeAn((*entry).size);
        }
        2 => {
            bits = QRinput_estimateBitsMode8((*entry).size);
        }
        3 => {
            bits = QRinput_estimateBitsModeKanji((*entry).size);
        }
        4 => return 20 as libc::c_int,
        5 => {
            bits = QRinput_estimateBitsModeECI((*entry).data);
        }
        6 => return 4 as libc::c_int,
        7 => return 4 as libc::c_int + 8 as libc::c_int,
        _ => return 0 as libc::c_int,
    }
    if mqr != 0 {
        l = MQRspec_lengthIndicator((*entry).mode, version);
        m = version - 1 as libc::c_int;
        bits += l + m;
    } else {
        l = QRspec_lengthIndicator((*entry).mode, version);
        m = (1 as libc::c_int) << l;
        if (*entry).mode as libc::c_int == QR_MODE_KANJI as libc::c_int {
            num = ((*entry).size / 2 as libc::c_int + m - 1 as libc::c_int) / m;
        } else {
            num = ((*entry).size + m - 1 as libc::c_int) / m;
        }
        bits += num * (4 as libc::c_int + l);
    }
    return bits;
}
pub unsafe extern "C" fn QRinput_estimateBitStreamSize(
    mut input: *mut QRinput,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut bits: libc::c_int = 0 as libc::c_int;
    list = (*input).head;
    while !list.is_null() {
        bits += QRinput_estimateBitStreamSizeOfEntry(list, version, (*input).mqr);
        list = (*list).next;
    }
    return bits;
}
pub unsafe extern "C" fn QRinput_estimateVersion(
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    version = 0 as libc::c_int;
    loop {
        prev = version;
        bits = QRinput_estimateBitStreamSize(input, prev);
        version = QRspec_getMinimumVersion(
            (bits + 7 as libc::c_int) / 8 as libc::c_int,
            (*input).level,
        );
        if prev == 0 as libc::c_int && version > 1 as libc::c_int {
            version -= 1;
            version;
        }
        if !(version > prev) {
            break;
        }
    }
    return version;
}
pub unsafe extern "C" fn QRinput_lengthOfCode(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut payload: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut chunks: libc::c_int = 0;
    let mut remain: libc::c_int = 0;
    let mut maxsize: libc::c_int = 0;
    payload = bits - 4 as libc::c_int - QRspec_lengthIndicator(mode, version);
    match mode as libc::c_int {
        0 => {
            chunks = payload / 10 as libc::c_int;
            remain = payload - chunks * 10 as libc::c_int;
            size = chunks * 3 as libc::c_int;
            if remain >= 7 as libc::c_int {
                size += 2 as libc::c_int;
            } else if remain >= 4 as libc::c_int {
                size += 1 as libc::c_int;
            }
        }
        1 => {
            chunks = payload / 11 as libc::c_int;
            remain = payload - chunks * 11 as libc::c_int;
            size = chunks * 2 as libc::c_int;
            if remain >= 6 as libc::c_int {
                size += 1;
                size;
            }
        }
        2 => {
            size = payload / 8 as libc::c_int;
        }
        3 => {
            size = payload / 13 as libc::c_int * 2 as libc::c_int;
        }
        4 => {
            size = payload / 8 as libc::c_int;
        }
        _ => {
            size = 0 as libc::c_int;
        }
    }
    maxsize = QRspec_maximumWords(mode, version);
    if size < 0 as libc::c_int {
        size = 0 as libc::c_int;
    }
    if maxsize > 0 as libc::c_int && size > maxsize {
        size = maxsize;
    }
    return size;
}
unsafe extern "C" fn QRinput_encodeBitStream(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: libc::c_int,
    mut mqr: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut words: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut st1: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut st2: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut prevsize: libc::c_int = 0;
    prevsize = (*bstream).length as libc::c_int;
    if mqr != 0 {
        words = MQRspec_maximumWords((*entry).mode, version);
    } else {
        words = QRspec_maximumWords((*entry).mode, version);
    }
    if words != 0 as libc::c_int && (*entry).size > words {
        st1 = QRinput_List_newEntry((*entry).mode, words, (*entry).data);
        if st1.is_null() {
            current_block = 11469998461051720438;
        } else {
            st2 = QRinput_List_newEntry(
                (*entry).mode,
                (*entry).size - words,
                &mut *((*entry).data).offset(words as isize),
            );
            if st2.is_null() {
                current_block = 11469998461051720438;
            } else {
                ret = QRinput_encodeBitStream(st1, bstream, version, mqr);
                if ret < 0 as libc::c_int {
                    current_block = 11469998461051720438;
                } else {
                    ret = QRinput_encodeBitStream(st2, bstream, version, mqr);
                    if ret < 0 as libc::c_int {
                        current_block = 11469998461051720438;
                    } else {
                        QRinput_List_freeEntry(st1);
                        QRinput_List_freeEntry(st2);
                        current_block = 4775909272756257391;
                    }
                }
            }
        }
        match current_block {
            4775909272756257391 => {}
            _ => {
                QRinput_List_freeEntry(st1);
                QRinput_List_freeEntry(st2);
                return -(1 as libc::c_int);
            }
        }
    } else {
        ret = 0 as libc::c_int;
        match (*entry).mode as libc::c_int {
            0 => {
                ret = QRinput_encodeModeNum(entry, bstream, version, mqr);
            }
            1 => {
                ret = QRinput_encodeModeAn(entry, bstream, version, mqr);
            }
            2 => {
                ret = QRinput_encodeMode8(entry, bstream, version, mqr);
            }
            3 => {
                ret = QRinput_encodeModeKanji(entry, bstream, version, mqr);
            }
            4 => {
                ret = QRinput_encodeModeStructure(entry, bstream, mqr);
            }
            5 => {
                ret = QRinput_encodeModeECI(entry, bstream);
            }
            7 => {
                ret = QRinput_encodeModeFNC1Second(entry, bstream);
            }
            _ => {}
        }
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return (*bstream).length as libc::c_int - prevsize;
}
unsafe extern "C" fn QRinput_createBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut bits: libc::c_int = 0;
    let mut total: libc::c_int = 0 as libc::c_int;
    list = (*input).head;
    while !list.is_null() {
        bits = QRinput_encodeBitStream(list, bstream, (*input).version, (*input).mqr);
        if bits < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        total += bits;
        list = (*list).next;
    }
    return total;
}
unsafe extern "C" fn QRinput_convertData(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut ver: libc::c_int = 0;
    ver = QRinput_estimateVersion(input);
    if ver > QRinput_getVersion(input) {
        QRinput_setVersion(input, ver);
    }
    loop {
        (*bstream).length = 0 as libc::c_int as size_t;
        bits = QRinput_createBitStream(input, bstream);
        if bits < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ver = QRspec_getMinimumVersion(
            (bits + 7 as libc::c_int) / 8 as libc::c_int,
            (*input).level,
        );
        if !(ver > QRinput_getVersion(input)) {
            break;
        }
        QRinput_setVersion(input, ver);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_appendPaddingBit(
    mut bstream: *mut BitStream,
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut maxwords: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut padlen: libc::c_int = 0;
    bits = (*bstream).length as libc::c_int;
    maxwords = QRspec_getDataLength((*input).version, (*input).level);
    maxbits = maxwords * 8 as libc::c_int;
    if maxbits < bits {
        *__errno_location() = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if maxbits == bits {
        return 0 as libc::c_int;
    }
    if maxbits - bits <= 4 as libc::c_int {
        return BitStream_appendNum(
            bstream,
            (maxbits - bits) as size_t,
            0 as libc::c_int as libc::c_uint,
        );
    }
    words = (bits + 4 as libc::c_int + 7 as libc::c_int) / 8 as libc::c_int;
    ret = BitStream_appendNum(
        bstream,
        (words * 8 as libc::c_int - bits) as size_t,
        0 as libc::c_int as libc::c_uint,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    padlen = maxwords - words;
    if padlen > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < padlen {
            ret = BitStream_appendNum(
                bstream,
                8 as libc::c_int as size_t,
                (if i & 1 as libc::c_int != 0 {
                    0x11 as libc::c_int
                } else {
                    0xec as libc::c_int
                }) as libc::c_uint,
            );
            if ret < 0 as libc::c_int {
                return ret;
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_appendPaddingBitMQR(
    mut bstream: *mut BitStream,
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    let mut maxwords: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut termbits: libc::c_int = 0;
    let mut padlen: libc::c_int = 0;
    bits = (*bstream).length as libc::c_int;
    maxbits = MQRspec_getDataLengthBit((*input).version, (*input).level);
    maxwords = maxbits / 8 as libc::c_int;
    if maxbits < bits {
        *__errno_location() = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if maxbits == bits {
        return 0 as libc::c_int;
    }
    termbits = (*input).version * 2 as libc::c_int + 1 as libc::c_int;
    if maxbits - bits <= termbits {
        return BitStream_appendNum(
            bstream,
            (maxbits - bits) as size_t,
            0 as libc::c_int as libc::c_uint,
        );
    }
    bits += termbits;
    words = (bits + 7 as libc::c_int) / 8 as libc::c_int;
    if maxbits - words * 8 as libc::c_int > 0 as libc::c_int {
        termbits += words * 8 as libc::c_int - bits;
        if words == maxwords {
            termbits += maxbits - words * 8 as libc::c_int;
        }
    } else {
        termbits += words * 8 as libc::c_int - bits;
    }
    ret = BitStream_appendNum(
        bstream,
        termbits as size_t,
        0 as libc::c_int as libc::c_uint,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    padlen = maxwords - words;
    if padlen > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < padlen {
            ret = BitStream_appendNum(
                bstream,
                8 as libc::c_int as size_t,
                (if i & 1 as libc::c_int != 0 {
                    0x11 as libc::c_int
                } else {
                    0xec as libc::c_int
                }) as libc::c_uint,
            );
            if ret < 0 as libc::c_int {
                return ret;
            }
            i += 1;
            i;
        }
        termbits = maxbits - maxwords * 8 as libc::c_int;
        if termbits > 0 as libc::c_int {
            ret = BitStream_appendNum(
                bstream,
                termbits as size_t,
                0 as libc::c_int as libc::c_uint,
            );
            if ret < 0 as libc::c_int {
                return ret;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn QRinput_insertFNC1Header(mut input: *mut QRinput) -> libc::c_int {
    let mut entry: *mut QRinput_List = 0 as *mut QRinput_List;
    if (*input).fnc1 == 1 as libc::c_int {
        entry = QRinput_List_newEntry(
            QR_MODE_FNC1FIRST,
            0 as libc::c_int,
            0 as *const libc::c_uchar,
        );
    } else if (*input).fnc1 == 2 as libc::c_int {
        entry = QRinput_List_newEntry(
            QR_MODE_FNC1SECOND,
            1 as libc::c_int,
            &mut (*input).appid,
        );
    }
    if entry.is_null() {
        return -(1 as libc::c_int);
    }
    if (*(*input).head).mode as libc::c_int != QR_MODE_STRUCTURE as libc::c_int
        && (*(*input).head).mode as libc::c_int != QR_MODE_ECI as libc::c_int
    {
        (*entry).next = (*input).head;
        (*input).head = entry;
    } else {
        (*entry).next = (*(*input).head).next;
        (*(*input).head).next = entry;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_mergeBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    if (*input).mqr != 0 {
        if QRinput_createBitStream(input, bstream) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        if (*input).fnc1 != 0 {
            if QRinput_insertFNC1Header(input) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        if QRinput_convertData(input, bstream) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_getBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = QRinput_mergeBitStream(input, bstream);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*input).mqr != 0 {
        ret = QRinput_appendPaddingBitMQR(bstream, input);
    } else {
        ret = QRinput_appendPaddingBit(bstream, input);
    }
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_getByteStream(
    mut input: *mut QRinput,
) -> *mut libc::c_uchar {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut array: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    bstream = BitStream_new();
    if bstream.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    ret = QRinput_getBitStream(input, bstream);
    if ret < 0 as libc::c_int {
        BitStream_free(bstream);
        return 0 as *mut libc::c_uchar;
    }
    array = BitStream_toByte(bstream);
    BitStream_free(bstream);
    return array;
}
unsafe extern "C" fn QRinput_InputList_newEntry(
    mut input: *mut QRinput,
) -> *mut QRinput_InputList {
    let mut entry: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    entry = malloc(::std::mem::size_of::<QRinput_InputList>() as libc::c_ulong)
        as *mut QRinput_InputList;
    if entry.is_null() {
        return 0 as *mut QRinput_InputList;
    }
    (*entry).input = input;
    (*entry).next = 0 as *mut QRinput_InputList;
    return entry;
}
unsafe extern "C" fn QRinput_InputList_freeEntry(mut entry: *mut QRinput_InputList) {
    if !entry.is_null() {
        QRinput_free((*entry).input);
        free(entry as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn QRinput_Struct_new() -> *mut QRinput_Struct {
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    s = malloc(::std::mem::size_of::<QRinput_Struct>() as libc::c_ulong)
        as *mut QRinput_Struct;
    if s.is_null() {
        return 0 as *mut QRinput_Struct;
    }
    (*s).size = 0 as libc::c_int;
    (*s).parity = -(1 as libc::c_int);
    (*s).head = 0 as *mut QRinput_InputList;
    (*s).tail = 0 as *mut QRinput_InputList;
    return s;
}
pub unsafe extern "C" fn QRinput_Struct_setParity(
    mut s: *mut QRinput_Struct,
    mut parity: libc::c_uchar,
) {
    (*s).parity = parity as libc::c_int;
}
pub unsafe extern "C" fn QRinput_Struct_appendInput(
    mut s: *mut QRinput_Struct,
    mut input: *mut QRinput,
) -> libc::c_int {
    let mut e: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    if (*input).mqr != 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    e = QRinput_InputList_newEntry(input);
    if e.is_null() {
        return -(1 as libc::c_int);
    }
    (*s).size += 1;
    (*s).size;
    if ((*s).tail).is_null() {
        (*s).head = e;
        (*s).tail = e;
    } else {
        (*(*s).tail).next = e;
        (*s).tail = e;
    }
    return (*s).size;
}
pub unsafe extern "C" fn QRinput_Struct_free(mut s: *mut QRinput_Struct) {
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut next: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    if !s.is_null() {
        list = (*s).head;
        while !list.is_null() {
            next = (*list).next;
            QRinput_InputList_freeEntry(list);
            list = next;
        }
        free(s as *mut libc::c_void);
    }
}
unsafe extern "C" fn QRinput_Struct_calcParity(
    mut s: *mut QRinput_Struct,
) -> libc::c_uchar {
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    let mut parity: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    list = (*s).head;
    while !list.is_null() {
        parity = (parity as libc::c_int
            ^ QRinput_calcParity((*list).input) as libc::c_int) as libc::c_uchar;
        list = (*list).next;
    }
    QRinput_Struct_setParity(s, parity);
    return parity;
}
unsafe extern "C" fn QRinput_List_shrinkEntry(
    mut entry: *mut QRinput_List,
    mut bytes: libc::c_int,
) -> libc::c_int {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    data = malloc(bytes as size_t) as *mut libc::c_uchar;
    if data.is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        data as *mut libc::c_void,
        (*entry).data as *const libc::c_void,
        bytes as size_t,
    );
    free((*entry).data as *mut libc::c_void);
    (*entry).data = data;
    (*entry).size = bytes;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_splitEntry(
    mut entry: *mut QRinput_List,
    mut bytes: libc::c_int,
) -> libc::c_int {
    let mut e: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut ret: libc::c_int = 0;
    e = QRinput_List_newEntry(
        (*entry).mode,
        (*entry).size - bytes,
        ((*entry).data).offset(bytes as isize),
    );
    if e.is_null() {
        return -(1 as libc::c_int);
    }
    ret = QRinput_List_shrinkEntry(entry, bytes);
    if ret < 0 as libc::c_int {
        QRinput_List_freeEntry(e);
        return -(1 as libc::c_int);
    }
    (*e).next = (*entry).next;
    (*entry).next = e;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_splitQRinputToStruct(
    mut input: *mut QRinput,
) -> *mut QRinput_Struct {
    let mut current_block: u64;
    let mut p: *mut QRinput = 0 as *mut QRinput;
    let mut s: *mut QRinput_Struct = 0 as *mut QRinput_Struct;
    let mut bits: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0;
    let mut nextbits: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut list: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut next: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut prev: *mut QRinput_List = 0 as *mut QRinput_List;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    if (*input).mqr != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut QRinput_Struct;
    }
    s = QRinput_Struct_new();
    if s.is_null() {
        return 0 as *mut QRinput_Struct;
    }
    input = QRinput_dup(input);
    if input.is_null() {
        QRinput_Struct_free(s);
        return 0 as *mut QRinput_Struct;
    }
    QRinput_Struct_setParity(s, QRinput_calcParity(input));
    maxbits = QRspec_getDataLength((*input).version, (*input).level) * 8 as libc::c_int
        - 20 as libc::c_int;
    if !(maxbits <= 0 as libc::c_int) {
        bstream = BitStream_new();
        if !bstream.is_null() {
            bits = 0 as libc::c_int;
            list = (*input).head;
            prev = 0 as *mut QRinput_List;
            loop {
                if list.is_null() {
                    current_block = 572715077006366937;
                    break;
                }
                nextbits = QRinput_estimateBitStreamSizeOfEntry(
                    list,
                    (*input).version,
                    (*input).mqr,
                );
                if bits + nextbits <= maxbits {
                    (*bstream).length = 0 as libc::c_int as size_t;
                    ret = QRinput_encodeBitStream(
                        list,
                        bstream,
                        (*input).version,
                        (*input).mqr,
                    );
                    if ret < 0 as libc::c_int {
                        current_block = 9503129933615575477;
                        break;
                    }
                    bits += ret;
                    prev = list;
                    list = (*list).next;
                } else {
                    bytes = QRinput_lengthOfCode(
                        (*list).mode,
                        (*input).version,
                        maxbits - bits,
                    );
                    p = QRinput_new2((*input).version, (*input).level);
                    if p.is_null() {
                        current_block = 9503129933615575477;
                        break;
                    }
                    if bytes > 0 as libc::c_int {
                        ret = QRinput_splitEntry(list, bytes);
                        if ret < 0 as libc::c_int {
                            QRinput_free(p);
                            current_block = 9503129933615575477;
                            break;
                        } else {
                            next = (*list).next;
                            (*list).next = 0 as *mut QRinput_List;
                            (*p).head = next;
                            (*p).tail = (*input).tail;
                            (*input).tail = list;
                            prev = list;
                            list = next;
                        }
                    } else {
                        (*prev).next = 0 as *mut QRinput_List;
                        (*p).head = list;
                        (*p).tail = (*input).tail;
                        (*input).tail = prev;
                    }
                    ret = QRinput_Struct_appendInput(s, input);
                    if ret < 0 as libc::c_int {
                        QRinput_free(p);
                        current_block = 9503129933615575477;
                        break;
                    } else {
                        input = p;
                        bits = 0 as libc::c_int;
                    }
                }
            }
            match current_block {
                9503129933615575477 => {}
                _ => {
                    ret = QRinput_Struct_appendInput(s, input);
                    if !(ret < 0 as libc::c_int) {
                        if (*s).size > 16 as libc::c_int {
                            *__errno_location() = 34 as libc::c_int;
                            QRinput_Struct_free(s);
                            BitStream_free(bstream);
                            return 0 as *mut QRinput_Struct;
                        }
                        ret = QRinput_Struct_insertStructuredAppendHeaders(s);
                        if ret < 0 as libc::c_int {
                            QRinput_Struct_free(s);
                            BitStream_free(bstream);
                            return 0 as *mut QRinput_Struct;
                        }
                        BitStream_free(bstream);
                        return s;
                    }
                }
            }
        }
    }
    BitStream_free(bstream);
    QRinput_free(input);
    QRinput_Struct_free(s);
    return 0 as *mut QRinput_Struct;
}
pub unsafe extern "C" fn QRinput_Struct_insertStructuredAppendHeaders(
    mut s: *mut QRinput_Struct,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut list: *mut QRinput_InputList = 0 as *mut QRinput_InputList;
    if (*s).size == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*s).parity < 0 as libc::c_int {
        QRinput_Struct_calcParity(s);
    }
    i = 1 as libc::c_int;
    list = (*s).head;
    while !list.is_null() {
        if QRinput_insertStructuredAppendHeader(
            (*list).input,
            (*s).size,
            i,
            (*s).parity as libc::c_uchar,
        ) != 0
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
        list = (*list).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_setFNC1First(mut input: *mut QRinput) -> libc::c_int {
    if (*input).mqr != 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*input).fnc1 = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn QRinput_setFNC1Second(
    mut input: *mut QRinput,
    mut appid: libc::c_uchar,
) -> libc::c_int {
    if (*input).mqr != 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*input).fnc1 = 2 as libc::c_int;
    (*input).appid = appid;
    return 0 as libc::c_int;
}
