use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn QRinput_isSplittableMode(mode: QRencodeMode) -> libc::c_int;
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
pub struct QRspec_Capacity {
    pub width: libc::c_int,
    pub words: libc::c_int,
    pub remainder: libc::c_int,
    pub ec: [libc::c_int; 4],
}
static mut qrspecCapacity: [QRspec_Capacity; 41] = [
    {
        let mut init = QRspec_Capacity {
            width: 0 as libc::c_int,
            words: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 21 as libc::c_int,
            words: 26 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                7 as libc::c_int,
                10 as libc::c_int,
                13 as libc::c_int,
                17 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 25 as libc::c_int,
            words: 44 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                10 as libc::c_int,
                16 as libc::c_int,
                22 as libc::c_int,
                28 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 29 as libc::c_int,
            words: 70 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                15 as libc::c_int,
                26 as libc::c_int,
                36 as libc::c_int,
                44 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 33 as libc::c_int,
            words: 100 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                20 as libc::c_int,
                36 as libc::c_int,
                52 as libc::c_int,
                64 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 37 as libc::c_int,
            words: 134 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                26 as libc::c_int,
                48 as libc::c_int,
                72 as libc::c_int,
                88 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 41 as libc::c_int,
            words: 172 as libc::c_int,
            remainder: 7 as libc::c_int,
            ec: [
                36 as libc::c_int,
                64 as libc::c_int,
                96 as libc::c_int,
                112 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 45 as libc::c_int,
            words: 196 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                40 as libc::c_int,
                72 as libc::c_int,
                108 as libc::c_int,
                130 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 49 as libc::c_int,
            words: 242 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                48 as libc::c_int,
                88 as libc::c_int,
                132 as libc::c_int,
                156 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 53 as libc::c_int,
            words: 292 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                60 as libc::c_int,
                110 as libc::c_int,
                160 as libc::c_int,
                192 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 57 as libc::c_int,
            words: 346 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                72 as libc::c_int,
                130 as libc::c_int,
                192 as libc::c_int,
                224 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 61 as libc::c_int,
            words: 404 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                80 as libc::c_int,
                150 as libc::c_int,
                224 as libc::c_int,
                264 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 65 as libc::c_int,
            words: 466 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                96 as libc::c_int,
                176 as libc::c_int,
                260 as libc::c_int,
                308 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 69 as libc::c_int,
            words: 532 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                104 as libc::c_int,
                198 as libc::c_int,
                288 as libc::c_int,
                352 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 73 as libc::c_int,
            words: 581 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                120 as libc::c_int,
                216 as libc::c_int,
                320 as libc::c_int,
                384 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 77 as libc::c_int,
            words: 655 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                132 as libc::c_int,
                240 as libc::c_int,
                360 as libc::c_int,
                432 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 81 as libc::c_int,
            words: 733 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                144 as libc::c_int,
                280 as libc::c_int,
                408 as libc::c_int,
                480 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 85 as libc::c_int,
            words: 815 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                168 as libc::c_int,
                308 as libc::c_int,
                448 as libc::c_int,
                532 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 89 as libc::c_int,
            words: 901 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                180 as libc::c_int,
                338 as libc::c_int,
                504 as libc::c_int,
                588 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 93 as libc::c_int,
            words: 991 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                196 as libc::c_int,
                364 as libc::c_int,
                546 as libc::c_int,
                650 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 97 as libc::c_int,
            words: 1085 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                224 as libc::c_int,
                416 as libc::c_int,
                600 as libc::c_int,
                700 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 101 as libc::c_int,
            words: 1156 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                224 as libc::c_int,
                442 as libc::c_int,
                644 as libc::c_int,
                750 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 105 as libc::c_int,
            words: 1258 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                252 as libc::c_int,
                476 as libc::c_int,
                690 as libc::c_int,
                816 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 109 as libc::c_int,
            words: 1364 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                270 as libc::c_int,
                504 as libc::c_int,
                750 as libc::c_int,
                900 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 113 as libc::c_int,
            words: 1474 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                300 as libc::c_int,
                560 as libc::c_int,
                810 as libc::c_int,
                960 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 117 as libc::c_int,
            words: 1588 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                312 as libc::c_int,
                588 as libc::c_int,
                870 as libc::c_int,
                1050 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 121 as libc::c_int,
            words: 1706 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                336 as libc::c_int,
                644 as libc::c_int,
                952 as libc::c_int,
                1110 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 125 as libc::c_int,
            words: 1828 as libc::c_int,
            remainder: 4 as libc::c_int,
            ec: [
                360 as libc::c_int,
                700 as libc::c_int,
                1020 as libc::c_int,
                1200 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 129 as libc::c_int,
            words: 1921 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                390 as libc::c_int,
                728 as libc::c_int,
                1050 as libc::c_int,
                1260 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 133 as libc::c_int,
            words: 2051 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                420 as libc::c_int,
                784 as libc::c_int,
                1140 as libc::c_int,
                1350 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 137 as libc::c_int,
            words: 2185 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                450 as libc::c_int,
                812 as libc::c_int,
                1200 as libc::c_int,
                1440 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 141 as libc::c_int,
            words: 2323 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                480 as libc::c_int,
                868 as libc::c_int,
                1290 as libc::c_int,
                1530 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 145 as libc::c_int,
            words: 2465 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                510 as libc::c_int,
                924 as libc::c_int,
                1350 as libc::c_int,
                1620 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 149 as libc::c_int,
            words: 2611 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                540 as libc::c_int,
                980 as libc::c_int,
                1440 as libc::c_int,
                1710 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 153 as libc::c_int,
            words: 2761 as libc::c_int,
            remainder: 3 as libc::c_int,
            ec: [
                570 as libc::c_int,
                1036 as libc::c_int,
                1530 as libc::c_int,
                1800 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 157 as libc::c_int,
            words: 2876 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                570 as libc::c_int,
                1064 as libc::c_int,
                1590 as libc::c_int,
                1890 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 161 as libc::c_int,
            words: 3034 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                600 as libc::c_int,
                1120 as libc::c_int,
                1680 as libc::c_int,
                1980 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 165 as libc::c_int,
            words: 3196 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                630 as libc::c_int,
                1204 as libc::c_int,
                1770 as libc::c_int,
                2100 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 169 as libc::c_int,
            words: 3362 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                660 as libc::c_int,
                1260 as libc::c_int,
                1860 as libc::c_int,
                2220 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 173 as libc::c_int,
            words: 3532 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                720 as libc::c_int,
                1316 as libc::c_int,
                1950 as libc::c_int,
                2310 as libc::c_int,
            ],
        };
        init
    },
    {
        let mut init = QRspec_Capacity {
            width: 177 as libc::c_int,
            words: 3706 as libc::c_int,
            remainder: 0 as libc::c_int,
            ec: [
                750 as libc::c_int,
                1372 as libc::c_int,
                2040 as libc::c_int,
                2430 as libc::c_int,
            ],
        };
        init
    },
];
pub unsafe extern "C" fn QRspec_getDataLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    return qrspecCapacity[version as usize].words
        - qrspecCapacity[version as usize].ec[level as usize];
}
pub unsafe extern "C" fn QRspec_getECCLength(
    mut version: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    return qrspecCapacity[version as usize].ec[level as usize];
}
pub unsafe extern "C" fn QRspec_getMinimumVersion(
    mut size: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 40 as libc::c_int {
        words = qrspecCapacity[i as usize].words
            - qrspecCapacity[i as usize].ec[level as usize];
        if words >= size {
            return i;
        }
        i += 1;
        i;
    }
    return 40 as libc::c_int;
}
pub unsafe extern "C" fn QRspec_getWidth(mut version: libc::c_int) -> libc::c_int {
    return qrspecCapacity[version as usize].width;
}
pub unsafe extern "C" fn QRspec_getRemainder(mut version: libc::c_int) -> libc::c_int {
    return qrspecCapacity[version as usize].remainder;
}
static mut lengthTableBits: [[libc::c_int; 3]; 4] = [
    [10 as libc::c_int, 12 as libc::c_int, 14 as libc::c_int],
    [9 as libc::c_int, 11 as libc::c_int, 13 as libc::c_int],
    [8 as libc::c_int, 16 as libc::c_int, 16 as libc::c_int],
    [8 as libc::c_int, 10 as libc::c_int, 12 as libc::c_int],
];
pub unsafe extern "C" fn QRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    if QRinput_isSplittableMode(mode) == 0 {
        return 0 as libc::c_int;
    }
    if version <= 9 as libc::c_int {
        l = 0 as libc::c_int;
    } else if version <= 26 as libc::c_int {
        l = 1 as libc::c_int;
    } else {
        l = 2 as libc::c_int;
    }
    return lengthTableBits[mode as usize][l as usize];
}
pub unsafe extern "C" fn QRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut words: libc::c_int = 0;
    if QRinput_isSplittableMode(mode) == 0 {
        return 0 as libc::c_int;
    }
    if version <= 9 as libc::c_int {
        l = 0 as libc::c_int;
    } else if version <= 26 as libc::c_int {
        l = 1 as libc::c_int;
    } else {
        l = 2 as libc::c_int;
    }
    bits = lengthTableBits[mode as usize][l as usize];
    words = ((1 as libc::c_int) << bits) - 1 as libc::c_int;
    if mode as libc::c_int == QR_MODE_KANJI as libc::c_int {
        words *= 2 as libc::c_int;
    }
    return words;
}
static mut eccTable: [[[libc::c_int; 2]; 4]; 41] = [
    [
        [0 as libc::c_int, 0 as libc::c_int],
        [0 as libc::c_int, 0 as libc::c_int],
        [0 as libc::c_int, 0 as libc::c_int],
        [0 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 2 as libc::c_int],
        [2 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 4 as libc::c_int],
        [4 as libc::c_int, 1 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [2 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 0 as libc::c_int],
        [3 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 4 as libc::c_int],
        [4 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 1 as libc::c_int],
        [6 as libc::c_int, 2 as libc::c_int],
        [6 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 0 as libc::c_int],
        [1 as libc::c_int, 4 as libc::c_int],
        [4 as libc::c_int, 4 as libc::c_int],
        [3 as libc::c_int, 8 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 2 as libc::c_int],
        [6 as libc::c_int, 2 as libc::c_int],
        [4 as libc::c_int, 6 as libc::c_int],
        [7 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 0 as libc::c_int],
        [8 as libc::c_int, 1 as libc::c_int],
        [8 as libc::c_int, 4 as libc::c_int],
        [12 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 1 as libc::c_int],
        [4 as libc::c_int, 5 as libc::c_int],
        [11 as libc::c_int, 5 as libc::c_int],
        [11 as libc::c_int, 5 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 1 as libc::c_int],
        [5 as libc::c_int, 5 as libc::c_int],
        [5 as libc::c_int, 7 as libc::c_int],
        [11 as libc::c_int, 7 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 1 as libc::c_int],
        [7 as libc::c_int, 3 as libc::c_int],
        [15 as libc::c_int, 2 as libc::c_int],
        [3 as libc::c_int, 13 as libc::c_int],
    ],
    [
        [1 as libc::c_int, 5 as libc::c_int],
        [10 as libc::c_int, 1 as libc::c_int],
        [1 as libc::c_int, 15 as libc::c_int],
        [2 as libc::c_int, 17 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 1 as libc::c_int],
        [9 as libc::c_int, 4 as libc::c_int],
        [17 as libc::c_int, 1 as libc::c_int],
        [2 as libc::c_int, 19 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 4 as libc::c_int],
        [3 as libc::c_int, 11 as libc::c_int],
        [17 as libc::c_int, 4 as libc::c_int],
        [9 as libc::c_int, 16 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 5 as libc::c_int],
        [3 as libc::c_int, 13 as libc::c_int],
        [15 as libc::c_int, 5 as libc::c_int],
        [15 as libc::c_int, 10 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 4 as libc::c_int],
        [17 as libc::c_int, 0 as libc::c_int],
        [17 as libc::c_int, 6 as libc::c_int],
        [19 as libc::c_int, 6 as libc::c_int],
    ],
    [
        [2 as libc::c_int, 7 as libc::c_int],
        [17 as libc::c_int, 0 as libc::c_int],
        [7 as libc::c_int, 16 as libc::c_int],
        [34 as libc::c_int, 0 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 5 as libc::c_int],
        [4 as libc::c_int, 14 as libc::c_int],
        [11 as libc::c_int, 14 as libc::c_int],
        [16 as libc::c_int, 14 as libc::c_int],
    ],
    [
        [6 as libc::c_int, 4 as libc::c_int],
        [6 as libc::c_int, 14 as libc::c_int],
        [11 as libc::c_int, 16 as libc::c_int],
        [30 as libc::c_int, 2 as libc::c_int],
    ],
    [
        [8 as libc::c_int, 4 as libc::c_int],
        [8 as libc::c_int, 13 as libc::c_int],
        [7 as libc::c_int, 22 as libc::c_int],
        [22 as libc::c_int, 13 as libc::c_int],
    ],
    [
        [10 as libc::c_int, 2 as libc::c_int],
        [19 as libc::c_int, 4 as libc::c_int],
        [28 as libc::c_int, 6 as libc::c_int],
        [33 as libc::c_int, 4 as libc::c_int],
    ],
    [
        [8 as libc::c_int, 4 as libc::c_int],
        [22 as libc::c_int, 3 as libc::c_int],
        [8 as libc::c_int, 26 as libc::c_int],
        [12 as libc::c_int, 28 as libc::c_int],
    ],
    [
        [3 as libc::c_int, 10 as libc::c_int],
        [3 as libc::c_int, 23 as libc::c_int],
        [4 as libc::c_int, 31 as libc::c_int],
        [11 as libc::c_int, 31 as libc::c_int],
    ],
    [
        [7 as libc::c_int, 7 as libc::c_int],
        [21 as libc::c_int, 7 as libc::c_int],
        [1 as libc::c_int, 37 as libc::c_int],
        [19 as libc::c_int, 26 as libc::c_int],
    ],
    [
        [5 as libc::c_int, 10 as libc::c_int],
        [19 as libc::c_int, 10 as libc::c_int],
        [15 as libc::c_int, 25 as libc::c_int],
        [23 as libc::c_int, 25 as libc::c_int],
    ],
    [
        [13 as libc::c_int, 3 as libc::c_int],
        [2 as libc::c_int, 29 as libc::c_int],
        [42 as libc::c_int, 1 as libc::c_int],
        [23 as libc::c_int, 28 as libc::c_int],
    ],
    [
        [17 as libc::c_int, 0 as libc::c_int],
        [10 as libc::c_int, 23 as libc::c_int],
        [10 as libc::c_int, 35 as libc::c_int],
        [19 as libc::c_int, 35 as libc::c_int],
    ],
    [
        [17 as libc::c_int, 1 as libc::c_int],
        [14 as libc::c_int, 21 as libc::c_int],
        [29 as libc::c_int, 19 as libc::c_int],
        [11 as libc::c_int, 46 as libc::c_int],
    ],
    [
        [13 as libc::c_int, 6 as libc::c_int],
        [14 as libc::c_int, 23 as libc::c_int],
        [44 as libc::c_int, 7 as libc::c_int],
        [59 as libc::c_int, 1 as libc::c_int],
    ],
    [
        [12 as libc::c_int, 7 as libc::c_int],
        [12 as libc::c_int, 26 as libc::c_int],
        [39 as libc::c_int, 14 as libc::c_int],
        [22 as libc::c_int, 41 as libc::c_int],
    ],
    [
        [6 as libc::c_int, 14 as libc::c_int],
        [6 as libc::c_int, 34 as libc::c_int],
        [46 as libc::c_int, 10 as libc::c_int],
        [2 as libc::c_int, 64 as libc::c_int],
    ],
    [
        [17 as libc::c_int, 4 as libc::c_int],
        [29 as libc::c_int, 14 as libc::c_int],
        [49 as libc::c_int, 10 as libc::c_int],
        [24 as libc::c_int, 46 as libc::c_int],
    ],
    [
        [4 as libc::c_int, 18 as libc::c_int],
        [13 as libc::c_int, 32 as libc::c_int],
        [48 as libc::c_int, 14 as libc::c_int],
        [42 as libc::c_int, 32 as libc::c_int],
    ],
    [
        [20 as libc::c_int, 4 as libc::c_int],
        [40 as libc::c_int, 7 as libc::c_int],
        [43 as libc::c_int, 22 as libc::c_int],
        [10 as libc::c_int, 67 as libc::c_int],
    ],
    [
        [19 as libc::c_int, 6 as libc::c_int],
        [18 as libc::c_int, 31 as libc::c_int],
        [34 as libc::c_int, 34 as libc::c_int],
        [20 as libc::c_int, 61 as libc::c_int],
    ],
];
pub unsafe extern "C" fn QRspec_getEccSpec(
    mut version: libc::c_int,
    mut level: QRecLevel,
    mut spec: *mut libc::c_int,
) {
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    let mut ecc: libc::c_int = 0;
    b1 = eccTable[version as usize][level as usize][0 as libc::c_int as usize];
    b2 = eccTable[version as usize][level as usize][1 as libc::c_int as usize];
    data = QRspec_getDataLength(version, level);
    ecc = QRspec_getECCLength(version, level);
    if b2 == 0 as libc::c_int {
        *spec.offset(0 as libc::c_int as isize) = b1;
        *spec.offset(1 as libc::c_int as isize) = data / b1;
        *spec.offset(2 as libc::c_int as isize) = ecc / b1;
        let ref mut fresh0 = *spec.offset(4 as libc::c_int as isize);
        *fresh0 = 0 as libc::c_int;
        *spec.offset(3 as libc::c_int as isize) = *fresh0;
    } else {
        *spec.offset(0 as libc::c_int as isize) = b1;
        *spec.offset(1 as libc::c_int as isize) = data / (b1 + b2);
        *spec.offset(2 as libc::c_int as isize) = ecc / (b1 + b2);
        *spec.offset(3 as libc::c_int as isize) = b2;
        *spec
            .offset(
                4 as libc::c_int as isize,
            ) = *spec.offset(1 as libc::c_int as isize) + 1 as libc::c_int;
    };
}
static mut alignmentPattern: [[libc::c_int; 2]; 41] = [
    [0 as libc::c_int, 0 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int],
    [18 as libc::c_int, 0 as libc::c_int],
    [22 as libc::c_int, 0 as libc::c_int],
    [26 as libc::c_int, 0 as libc::c_int],
    [30 as libc::c_int, 0 as libc::c_int],
    [34 as libc::c_int, 0 as libc::c_int],
    [22 as libc::c_int, 38 as libc::c_int],
    [24 as libc::c_int, 42 as libc::c_int],
    [26 as libc::c_int, 46 as libc::c_int],
    [28 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [32 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [26 as libc::c_int, 46 as libc::c_int],
    [26 as libc::c_int, 48 as libc::c_int],
    [26 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [30 as libc::c_int, 56 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [28 as libc::c_int, 50 as libc::c_int],
    [26 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [28 as libc::c_int, 54 as libc::c_int],
    [32 as libc::c_int, 58 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [26 as libc::c_int, 50 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [26 as libc::c_int, 52 as libc::c_int],
    [30 as libc::c_int, 56 as libc::c_int],
    [34 as libc::c_int, 60 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
    [34 as libc::c_int, 62 as libc::c_int],
    [30 as libc::c_int, 54 as libc::c_int],
    [24 as libc::c_int, 50 as libc::c_int],
    [28 as libc::c_int, 54 as libc::c_int],
    [32 as libc::c_int, 58 as libc::c_int],
    [26 as libc::c_int, 54 as libc::c_int],
    [30 as libc::c_int, 58 as libc::c_int],
];
unsafe extern "C" fn QRspec_putAlignmentMarker(
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut ox: libc::c_int,
    mut oy: libc::c_int,
) {
    static mut finder: [libc::c_uchar; 25] = [
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
        0xa1 as libc::c_int as libc::c_uchar,
    ];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
    frame = frame
        .offset(((oy - 2 as libc::c_int) * width + ox - 2 as libc::c_int) as isize);
    s = finder.as_ptr();
    y = 0 as libc::c_int;
    while y < 5 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 5 as libc::c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
            x;
        }
        frame = frame.offset(width as isize);
        s = s.offset(5 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn QRspec_putAlignmentPattern(
    mut version: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut width: libc::c_int,
) {
    let mut d: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    if version < 2 as libc::c_int {
        return;
    }
    d = alignmentPattern[version as usize][1 as libc::c_int as usize]
        - alignmentPattern[version as usize][0 as libc::c_int as usize];
    if d < 0 as libc::c_int {
        w = 2 as libc::c_int;
    } else {
        w = (width - alignmentPattern[version as usize][0 as libc::c_int as usize]) / d
            + 2 as libc::c_int;
    }
    if w * w - 3 as libc::c_int == 1 as libc::c_int {
        x = alignmentPattern[version as usize][0 as libc::c_int as usize];
        y = alignmentPattern[version as usize][0 as libc::c_int as usize];
        QRspec_putAlignmentMarker(frame, width, x, y);
        return;
    }
    cx = alignmentPattern[version as usize][0 as libc::c_int as usize];
    x = 1 as libc::c_int;
    while x < w - 1 as libc::c_int {
        QRspec_putAlignmentMarker(frame, width, 6 as libc::c_int, cx);
        QRspec_putAlignmentMarker(frame, width, cx, 6 as libc::c_int);
        cx += d;
        x += 1;
        x;
    }
    cy = alignmentPattern[version as usize][0 as libc::c_int as usize];
    y = 0 as libc::c_int;
    while y < w - 1 as libc::c_int {
        cx = alignmentPattern[version as usize][0 as libc::c_int as usize];
        x = 0 as libc::c_int;
        while x < w - 1 as libc::c_int {
            QRspec_putAlignmentMarker(frame, width, cx, cy);
            cx += d;
            x += 1;
            x;
        }
        cy += d;
        y += 1;
        y;
    }
}
static mut versionPattern: [libc::c_uint; 34] = [
    0x7c94 as libc::c_int as libc::c_uint,
    0x85bc as libc::c_int as libc::c_uint,
    0x9a99 as libc::c_int as libc::c_uint,
    0xa4d3 as libc::c_int as libc::c_uint,
    0xbbf6 as libc::c_int as libc::c_uint,
    0xc762 as libc::c_int as libc::c_uint,
    0xd847 as libc::c_int as libc::c_uint,
    0xe60d as libc::c_int as libc::c_uint,
    0xf928 as libc::c_int as libc::c_uint,
    0x10b78 as libc::c_int as libc::c_uint,
    0x1145d as libc::c_int as libc::c_uint,
    0x12a17 as libc::c_int as libc::c_uint,
    0x13532 as libc::c_int as libc::c_uint,
    0x149a6 as libc::c_int as libc::c_uint,
    0x15683 as libc::c_int as libc::c_uint,
    0x168c9 as libc::c_int as libc::c_uint,
    0x177ec as libc::c_int as libc::c_uint,
    0x18ec4 as libc::c_int as libc::c_uint,
    0x191e1 as libc::c_int as libc::c_uint,
    0x1afab as libc::c_int as libc::c_uint,
    0x1b08e as libc::c_int as libc::c_uint,
    0x1cc1a as libc::c_int as libc::c_uint,
    0x1d33f as libc::c_int as libc::c_uint,
    0x1ed75 as libc::c_int as libc::c_uint,
    0x1f250 as libc::c_int as libc::c_uint,
    0x209d5 as libc::c_int as libc::c_uint,
    0x216f0 as libc::c_int as libc::c_uint,
    0x228ba as libc::c_int as libc::c_uint,
    0x2379f as libc::c_int as libc::c_uint,
    0x24b0b as libc::c_int as libc::c_uint,
    0x2542e as libc::c_int as libc::c_uint,
    0x26a64 as libc::c_int as libc::c_uint,
    0x27541 as libc::c_int as libc::c_uint,
    0x28c69 as libc::c_int as libc::c_uint,
];
pub unsafe extern "C" fn QRspec_getVersionPattern(
    mut version: libc::c_int,
) -> libc::c_uint {
    if version < 7 as libc::c_int || version > 40 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    return versionPattern[(version - 7 as libc::c_int) as usize];
}
static mut formatInfo: [[libc::c_uint; 8]; 4] = [
    [
        0x77c4 as libc::c_int as libc::c_uint,
        0x72f3 as libc::c_int as libc::c_uint,
        0x7daa as libc::c_int as libc::c_uint,
        0x789d as libc::c_int as libc::c_uint,
        0x662f as libc::c_int as libc::c_uint,
        0x6318 as libc::c_int as libc::c_uint,
        0x6c41 as libc::c_int as libc::c_uint,
        0x6976 as libc::c_int as libc::c_uint,
    ],
    [
        0x5412 as libc::c_int as libc::c_uint,
        0x5125 as libc::c_int as libc::c_uint,
        0x5e7c as libc::c_int as libc::c_uint,
        0x5b4b as libc::c_int as libc::c_uint,
        0x45f9 as libc::c_int as libc::c_uint,
        0x40ce as libc::c_int as libc::c_uint,
        0x4f97 as libc::c_int as libc::c_uint,
        0x4aa0 as libc::c_int as libc::c_uint,
    ],
    [
        0x355f as libc::c_int as libc::c_uint,
        0x3068 as libc::c_int as libc::c_uint,
        0x3f31 as libc::c_int as libc::c_uint,
        0x3a06 as libc::c_int as libc::c_uint,
        0x24b4 as libc::c_int as libc::c_uint,
        0x2183 as libc::c_int as libc::c_uint,
        0x2eda as libc::c_int as libc::c_uint,
        0x2bed as libc::c_int as libc::c_uint,
    ],
    [
        0x1689 as libc::c_int as libc::c_uint,
        0x13be as libc::c_int as libc::c_uint,
        0x1ce7 as libc::c_int as libc::c_uint,
        0x19d0 as libc::c_int as libc::c_uint,
        0x762 as libc::c_int as libc::c_uint,
        0x255 as libc::c_int as libc::c_uint,
        0xd0c as libc::c_int as libc::c_uint,
        0x83b as libc::c_int as libc::c_uint,
    ],
];
pub unsafe extern "C" fn QRspec_getFormatInfo(
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_uint {
    if mask < 0 as libc::c_int || mask > 7 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    return formatInfo[level as usize][mask as usize];
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
unsafe extern "C" fn QRspec_createFrame(mut version: libc::c_int) -> *mut libc::c_uchar {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut verinfo: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;
    width = qrspecCapacity[version as usize].width;
    frame = malloc((width * width) as size_t) as *mut libc::c_uchar;
    if frame.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    memset(frame as *mut libc::c_void, 0 as libc::c_int, (width * width) as size_t);
    putFinderPattern(frame, width, 0 as libc::c_int, 0 as libc::c_int);
    putFinderPattern(frame, width, width - 7 as libc::c_int, 0 as libc::c_int);
    putFinderPattern(frame, width, 0 as libc::c_int, width - 7 as libc::c_int);
    p = frame;
    q = frame.offset((width * (width - 7 as libc::c_int)) as isize);
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p.offset(7 as libc::c_int as isize) = 0xc0 as libc::c_int as libc::c_uchar;
        *p
            .offset(
                (width - 8 as libc::c_int) as isize,
            ) = 0xc0 as libc::c_int as libc::c_uchar;
        *q.offset(7 as libc::c_int as isize) = 0xc0 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        q = q.offset(width as isize);
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
            .offset(-(8 as libc::c_int as isize)) as *mut libc::c_void,
        0xc0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    memset(
        frame.offset((width * (width - 8 as libc::c_int)) as isize) as *mut libc::c_void,
        0xc0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    memset(
        frame.offset((width * 8 as libc::c_int) as isize) as *mut libc::c_void,
        0x84 as libc::c_int,
        9 as libc::c_int as libc::c_ulong,
    );
    memset(
        frame
            .offset((width * 9 as libc::c_int) as isize)
            .offset(-(8 as libc::c_int as isize)) as *mut libc::c_void,
        0x84 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    p = frame.offset(8 as libc::c_int as isize);
    y = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        *p = 0x84 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
        y;
    }
    p = frame
        .offset((width * (width - 7 as libc::c_int)) as isize)
        .offset(8 as libc::c_int as isize);
    y = 0 as libc::c_int;
    while y < 7 as libc::c_int {
        *p = 0x84 as libc::c_int as libc::c_uchar;
        p = p.offset(width as isize);
        y += 1;
        y;
    }
    p = frame
        .offset((width * 6 as libc::c_int) as isize)
        .offset(8 as libc::c_int as isize);
    q = frame
        .offset((width * 8 as libc::c_int) as isize)
        .offset(6 as libc::c_int as isize);
    x = 1 as libc::c_int;
    while x < width - 15 as libc::c_int {
        *p = (0x90 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        *q = (0x90 as libc::c_int | x & 1 as libc::c_int) as libc::c_uchar;
        p = p.offset(1);
        p;
        q = q.offset(width as isize);
        x += 1;
        x;
    }
    QRspec_putAlignmentPattern(version, frame, width);
    if version >= 7 as libc::c_int {
        verinfo = QRspec_getVersionPattern(version);
        p = frame.offset((width * (width - 11 as libc::c_int)) as isize);
        v = verinfo;
        x = 0 as libc::c_int;
        while x < 6 as libc::c_int {
            y = 0 as libc::c_int;
            while y < 3 as libc::c_int {
                *p
                    .offset(
                        (width * y + x) as isize,
                    ) = (0x88 as libc::c_int as libc::c_uint
                    | v & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
                v = v >> 1 as libc::c_int;
                y += 1;
                y;
            }
            x += 1;
            x;
        }
        p = frame.offset(width as isize).offset(-(11 as libc::c_int as isize));
        v = verinfo;
        y = 0 as libc::c_int;
        while y < 6 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 3 as libc::c_int {
                *p
                    .offset(
                        x as isize,
                    ) = (0x88 as libc::c_int as libc::c_uint
                    | v & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
                v = v >> 1 as libc::c_int;
                x += 1;
                x;
            }
            p = p.offset(width as isize);
            y += 1;
            y;
        }
    }
    *frame
        .offset(
            (width * (width - 8 as libc::c_int) + 8 as libc::c_int) as isize,
        ) = 0x81 as libc::c_int as libc::c_uchar;
    return frame;
}
pub unsafe extern "C" fn QRspec_newFrame(
    mut version: libc::c_int,
) -> *mut libc::c_uchar {
    if version < 1 as libc::c_int || version > 40 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    return QRspec_createFrame(version);
}
