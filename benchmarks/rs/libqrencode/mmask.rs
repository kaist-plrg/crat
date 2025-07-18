use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn MQRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn MQRspec_getFormatInfo(
        mask: libc::c_int,
        version: libc::c_int,
        level: QRecLevel,
    ) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type QRecLevel = libc::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
pub type MaskMaker = unsafe extern "C" fn(
    libc::c_int,
    *const libc::c_uchar,
    *mut libc::c_uchar,
) -> ();
pub unsafe extern "C" fn MMask_writeFormatInformation(
    mut version: libc::c_int,
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) {
    let mut format: libc::c_uint = 0;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    format = MQRspec_getFormatInfo(mask, version, level);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v = (0x84 as libc::c_int as libc::c_uint
            | format & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
        *frame.offset((width * (i + 1 as libc::c_int) + 8 as libc::c_int) as isize) = v;
        format = format >> 1 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        v = (0x84 as libc::c_int as libc::c_uint
            | format & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
        *frame.offset((width * 8 as libc::c_int + 7 as libc::c_int - i) as isize) = v;
        format = format >> 1 as libc::c_int;
        i += 1;
        i;
    }
}
unsafe extern "C" fn Mask_mask0(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (y & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            s = s.offset(1);
            s;
            d = d.offset(1);
            d;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
}
unsafe extern "C" fn Mask_mask1(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (y / 2 as libc::c_int + x / 3 as libc::c_int & 1 as libc::c_int
                        == 0 as libc::c_int) as libc::c_int) as libc::c_uchar;
            }
            s = s.offset(1);
            s;
            d = d.offset(1);
            d;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
}
unsafe extern "C" fn Mask_mask2(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x * y & 1 as libc::c_int) + x * y % 3 as libc::c_int
                        & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            s = s.offset(1);
            s;
            d = d.offset(1);
            d;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
}
unsafe extern "C" fn Mask_mask3(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x + y & 1 as libc::c_int) + x * y % 3 as libc::c_int
                        & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            s = s.offset(1);
            s;
            d = d.offset(1);
            d;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
}
static mut maskMakers: [Option::<MaskMaker>; 4] = unsafe {
    [
        Some(
            Mask_mask0
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask1
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask2
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> (),
        ),
        Some(
            Mask_mask3
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> (),
        ),
    ]
};
pub unsafe extern "C" fn MMask_makeMaskedFrame(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
) -> *mut libc::c_uchar {
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    masked = malloc((width * width) as size_t) as *mut libc::c_uchar;
    if masked.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    (maskMakers[mask as usize]).unwrap()(width, frame, masked);
    return masked;
}
pub unsafe extern "C" fn MMask_makeMask(
    mut version: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut width: libc::c_int = 0;
    if mask < 0 as libc::c_int || mask >= 4 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_uchar;
    }
    width = MQRspec_getWidth(version);
    masked = malloc((width * width) as size_t) as *mut libc::c_uchar;
    if masked.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    (maskMakers[mask as usize]).unwrap()(width, frame, masked);
    MMask_writeFormatInformation(version, width, masked, mask, level);
    return masked;
}
pub unsafe extern "C" fn MMask_evaluateSymbol(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sum1: libc::c_int = 0 as libc::c_int;
    let mut sum2: libc::c_int = 0 as libc::c_int;
    p = frame.offset((width * (width - 1 as libc::c_int)) as isize);
    x = 1 as libc::c_int;
    while x < width {
        sum1 += *p.offset(x as isize) as libc::c_int & 1 as libc::c_int;
        x += 1;
        x;
    }
    p = frame
        .offset((width * 2 as libc::c_int) as isize)
        .offset(-(1 as libc::c_int as isize));
    y = 1 as libc::c_int;
    while y < width {
        sum2 += *p as libc::c_int & 1 as libc::c_int;
        p = p.offset(width as isize);
        y += 1;
        y;
    }
    return if sum1 <= sum2 {
        sum1 * 16 as libc::c_int + sum2
    } else {
        sum2 * 16 as libc::c_int + sum1
    };
}
pub unsafe extern "C" fn MMask_mask(
    mut version: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0;
    let mut mask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bestMask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut maxScore: libc::c_int = 0 as libc::c_int;
    let mut score: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    width = MQRspec_getWidth(version);
    mask = malloc((width * width) as size_t) as *mut libc::c_uchar;
    if mask.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    bestMask = 0 as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        score = 0 as libc::c_int;
        (maskMakers[i as usize]).unwrap()(width, frame, mask);
        MMask_writeFormatInformation(version, width, mask, i, level);
        score = MMask_evaluateSymbol(width, mask);
        if score > maxScore {
            maxScore = score;
            free(bestMask as *mut libc::c_void);
            bestMask = mask;
            mask = malloc((width * width) as size_t) as *mut libc::c_uchar;
            if mask.is_null() {
                break;
            }
        }
        i += 1;
        i;
    }
    free(mask as *mut libc::c_void);
    return bestMask;
}
