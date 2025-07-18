use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub struct MQRspec_Capacity {
    pub width: libc::c_int,
    pub ec: [libc::c_int; 4],
}
static mut mqrspecCapacity: [MQRspec_Capacity; 5] = [
    {
        let mut init = MQRspec_Capacity {
            width: 0 as libc::c_int,
            ec: [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = MQRspec_Capacity {
            width: 11 as libc::c_int,
            ec: [2 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = MQRspec_Capacity {
            width: 13 as libc::c_int,
            ec: [5 as libc::c_int, 6 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = MQRspec_Capacity {
            width: 15 as libc::c_int,
            ec: [6 as libc::c_int, 8 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = MQRspec_Capacity {
            width: 17 as libc::c_int,
            ec: [
                8 as libc::c_int,
                10 as libc::c_int,
                14 as libc::c_int,
                0 as libc::c_int,
            ],
        };
        init
    },
];
pub unsafe extern "C" fn MQRspec_getDataLengthBit(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    w = mqrspecCapacity[version as usize].width - 1 as libc::c_int;
    ecc = mqrspecCapacity[version as usize].ec[level as usize];
    if ecc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return w * w - 64 as libc::c_int - ecc * 8 as libc::c_int;
}
pub unsafe extern "C" fn MQRspec_getDataLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    return (MQRspec_getDataLengthBit(version, level) + 4 as libc::c_int)
        / 8 as libc::c_int;
}
pub unsafe extern "C" fn MQRspec_getECCLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    return mqrspecCapacity[version as usize].ec[level as usize];
}
pub unsafe extern "C" fn MQRspec_getWidth(mut version: libc::c_int) -> libc::c_int {
    return mqrspecCapacity[version as usize].width;
}
static mut lengthTableBits: [[libc::c_int; 4]; 4] = [
    [3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int],
    [0 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int],
];
pub unsafe extern "C" fn MQRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    return lengthTableBits[mode as usize][(version - 1 as libc::c_int) as usize];
}
pub unsafe extern "C" fn MQRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    bits = lengthTableBits[mode as usize][(version - 1 as libc::c_int) as usize];
    words = ((1 as libc::c_int) << bits) - 1 as libc::c_int;
    if mode as libc::c_int == QR_MODE_KANJI as libc::c_int {
        words *= 2 as libc::c_int;
    }
    return words;
}
static mut formatInfo: [[libc::c_uint; 8]; 4] = [
    [
        0x4445 as libc::c_int as libc::c_uint,
        0x55ae as libc::c_int as libc::c_uint,
        0x6793 as libc::c_int as libc::c_uint,
        0x7678 as libc::c_int as libc::c_uint,
        0x6de as libc::c_int as libc::c_uint,
        0x1735 as libc::c_int as libc::c_uint,
        0x2508 as libc::c_int as libc::c_uint,
        0x34e3 as libc::c_int as libc::c_uint,
    ],
    [
        0x4172 as libc::c_int as libc::c_uint,
        0x5099 as libc::c_int as libc::c_uint,
        0x62a4 as libc::c_int as libc::c_uint,
        0x734f as libc::c_int as libc::c_uint,
        0x3e9 as libc::c_int as libc::c_uint,
        0x1202 as libc::c_int as libc::c_uint,
        0x203f as libc::c_int as libc::c_uint,
        0x31d4 as libc::c_int as libc::c_uint,
    ],
    [
        0x4e2b as libc::c_int as libc::c_uint,
        0x5fc0 as libc::c_int as libc::c_uint,
        0x6dfd as libc::c_int as libc::c_uint,
        0x7c16 as libc::c_int as libc::c_uint,
        0xcb0 as libc::c_int as libc::c_uint,
        0x1d5b as libc::c_int as libc::c_uint,
        0x2f66 as libc::c_int as libc::c_uint,
        0x3e8d as libc::c_int as libc::c_uint,
    ],
    [
        0x4b1c as libc::c_int as libc::c_uint,
        0x5af7 as libc::c_int as libc::c_uint,
        0x68ca as libc::c_int as libc::c_uint,
        0x7921 as libc::c_int as libc::c_uint,
        0x987 as libc::c_int as libc::c_uint,
        0x186c as libc::c_int as libc::c_uint,
        0x2a51 as libc::c_int as libc::c_uint,
        0x3bba as libc::c_int as libc::c_uint,
    ],
];
static mut typeTable: [[libc::c_int; 3]; 5] = [
    [-(1 as libc::c_int), -(1 as libc::c_int), -(1 as libc::c_int)],
    [0 as libc::c_int, -(1 as libc::c_int), -(1 as libc::c_int)],
    [1 as libc::c_int, 2 as libc::c_int, -(1 as libc::c_int)],
    [3 as libc::c_int, 4 as libc::c_int, -(1 as libc::c_int)],
    [5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int],
];
pub unsafe extern "C" fn MQRspec_getFormatInfo(
    mut mask: libc::c_int,
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_uint {
    let mut type_0: libc::c_int = 0;
    if mask < 0 as libc::c_int || mask > 3 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    if version <= 0 as libc::c_int || version > 4 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    if level as libc::c_uint == QR_ECLEVEL_H as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    type_0 = typeTable[version as usize][level as usize];
    if type_0 < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    return formatInfo[mask as usize][type_0 as usize];
}
unsafe extern "C" fn putFinderPattern(
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut ox: libc::c_int,
    mut oy: libc::c_int,
) {
    static mut finder: [libc::c_uchar; 49] = [
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
        0xc1 as libc::c_int as libc::c_uchar,
    ];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
    frame = frame.offset((oy * width + ox) as isize);
    s = finder.as_ptr();
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 7 as libc::c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
            x;
        }
        frame = frame.offset(width as isize);
        s = s.offset(7 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn MQRspec_createFrame(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    width = mqrspecCapacity[version as usize].width;
    frame = malloc((width * width) as size_t) as *mut libc::c_uchar;
    if frame.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    memset(frame as *mut libc::c_void, 0 as libc::c_int, (width * width) as size_t);
    putFinderPattern(frame, width, 0 as libc::c_int, 0 as libc::c_int);
    p = frame;
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p.offset(7 as libc::c_int as isize) = 0xc0 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
        y;
    }
    memset(
        frame.offset((width * 7 as libc::c_int) as isize) as *mut libc::c_void,
        0xc0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    memset(
        frame
            .offset((width * 8 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
        0x84 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    p = frame.offset(width as isize).offset(8 as libc::c_int as isize);
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p = 0x84 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
        y;
    }
    p = frame.offset(8 as libc::c_int as isize);
    q = frame.offset((width * 8 as libc::c_int) as isize);
    x = 1 as libc::c_int;
    while x < width - 7 as libc::c_int {
        *p = (0x90 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        *q = (0x90 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        p = p.offset(1);
        p;
        q = q.offset(width as isize);
        x += 1;
        x;
    }
    return frame;
}
pub unsafe extern "C" fn MQRspec_newFrame(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    if version < 1 as libc::c_int || version > 4 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    return MQRspec_createFrame(version);
}
