use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn QRspec_getFormatInfo(mask: libc::c_int, level: QRecLevel) -> libc::c_uint;
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
) -> libc::c_int;
pub unsafe extern "C" fn Mask_writeFormatInformation(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> libc::c_int {
    let mut format: libc::c_uint = 0;
    let mut v: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut blacks: libc::c_int = 0 as libc::c_int;
    format = QRspec_getFormatInfo(mask, level);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if format & 1 as libc::c_int as libc::c_uint != 0 {
            blacks += 2 as libc::c_int;
            v = 0x85 as libc::c_int as libc::c_uchar;
        } else {
            v = 0x84 as libc::c_int as libc::c_uchar;
        }
        *frame
            .offset(
                (width * 8 as libc::c_int + width - 1 as libc::c_int - i) as isize,
            ) = v;
        if i < 6 as libc::c_int {
            *frame.offset((width * i + 8 as libc::c_int) as isize) = v;
        } else {
            *frame
                .offset(
                    (width * (i + 1 as libc::c_int) + 8 as libc::c_int) as isize,
                ) = v;
        }
        format = format >> 1 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if format & 1 as libc::c_int as libc::c_uint != 0 {
            blacks += 2 as libc::c_int;
            v = 0x85 as libc::c_int as libc::c_uchar;
        } else {
            v = 0x84 as libc::c_int as libc::c_uchar;
        }
        *frame
            .offset(
                (width * (width - 7 as libc::c_int + i) + 8 as libc::c_int) as isize,
            ) = v;
        if i == 0 as libc::c_int {
            *frame.offset((width * 8 as libc::c_int + 7 as libc::c_int) as isize) = v;
        } else {
            *frame
                .offset((width * 8 as libc::c_int + 6 as libc::c_int - i) as isize) = v;
        }
        format = format >> 1 as libc::c_int;
        i += 1;
        i;
    }
    return blacks;
}
unsafe extern "C" fn Mask_mask0(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (x + y & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
unsafe extern "C" fn Mask_mask1(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
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
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
unsafe extern "C" fn Mask_mask2(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (x % 3 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
unsafe extern "C" fn Mask_mask3(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x + y) % 3 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
unsafe extern "C" fn Mask_mask4(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
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
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
unsafe extern "C" fn Mask_mask5(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ ((x * y & 1 as libc::c_int) + x * y % 3 as libc::c_int
                        == 0 as libc::c_int) as libc::c_int) as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
unsafe extern "C" fn Mask_mask6(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
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
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
unsafe extern "C" fn Mask_mask7(
    mut width: libc::c_int,
    mut s: *const libc::c_uchar,
    mut d: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b: libc::c_int = 0 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            if *s as libc::c_int & 0x80 as libc::c_int != 0 {
                *d = *s;
            } else {
                *d = (*s as libc::c_int
                    ^ (x * y % 3 as libc::c_int + (x + y & 1 as libc::c_int)
                        & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int)
                    as libc::c_uchar;
            }
            b += *d as libc::c_int & 1 as libc::c_int;
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
    return b;
}
static mut maskMakers: [Option::<MaskMaker>; 8] = unsafe {
    [
        Some(
            Mask_mask0
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
        Some(
            Mask_mask1
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
        Some(
            Mask_mask2
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
        Some(
            Mask_mask3
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
        Some(
            Mask_mask4
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
        Some(
            Mask_mask5
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
        Some(
            Mask_mask6
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
        Some(
            Mask_mask7
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_uchar,
                    *mut libc::c_uchar,
                ) -> libc::c_int,
        ),
    ]
};
pub unsafe extern "C" fn Mask_makeMaskedFrame(
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
pub unsafe extern "C" fn Mask_makeMask(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut mask: libc::c_int,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if mask < 0 as libc::c_int || mask >= 8 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_uchar;
    }
    masked = malloc((width * width) as size_t) as *mut libc::c_uchar;
    if masked.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    (maskMakers[mask as usize]).unwrap()(width, frame, masked);
    Mask_writeFormatInformation(width, masked, mask, level);
    return masked;
}
pub unsafe extern "C" fn Mask_calcN1N3(
    mut length: libc::c_int,
    mut runLength: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut demerit: libc::c_int = 0 as libc::c_int;
    let mut fact: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        if *runLength.offset(i as isize) >= 5 as libc::c_int {
            demerit
                += 3 as libc::c_int + (*runLength.offset(i as isize) - 5 as libc::c_int);
        }
        if i & 1 as libc::c_int != 0 {
            if i >= 3 as libc::c_int && i < length - 2 as libc::c_int
                && *runLength.offset(i as isize) % 3 as libc::c_int == 0 as libc::c_int
            {
                fact = *runLength.offset(i as isize) / 3 as libc::c_int;
                if *runLength.offset((i - 2 as libc::c_int) as isize) == fact
                    && *runLength.offset((i - 1 as libc::c_int) as isize) == fact
                    && *runLength.offset((i + 1 as libc::c_int) as isize) == fact
                    && *runLength.offset((i + 2 as libc::c_int) as isize) == fact
                {
                    if i == 3 as libc::c_int
                        || *runLength.offset((i - 3 as libc::c_int) as isize)
                            >= 4 as libc::c_int * fact
                    {
                        demerit += 40 as libc::c_int;
                    } else if i + 4 as libc::c_int >= length
                        || *runLength.offset((i + 3 as libc::c_int) as isize)
                            >= 4 as libc::c_int * fact
                    {
                        demerit += 40 as libc::c_int;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    return demerit;
}
pub unsafe extern "C" fn Mask_calcN2(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut b22: libc::c_uchar = 0;
    let mut w22: libc::c_uchar = 0;
    let mut demerit: libc::c_int = 0 as libc::c_int;
    p = frame.offset(width as isize).offset(1 as libc::c_int as isize);
    y = 1 as libc::c_int;
    while y < width {
        x = 1 as libc::c_int;
        while x < width {
            b22 = (*p.offset(0 as libc::c_int as isize) as libc::c_int
                & *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                & *p.offset(-width as isize) as libc::c_int
                & *p.offset((-width - 1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_uchar;
            w22 = (*p.offset(0 as libc::c_int as isize) as libc::c_int
                | *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                | *p.offset(-width as isize) as libc::c_int
                | *p.offset((-width - 1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_uchar;
            if (b22 as libc::c_int | w22 as libc::c_int ^ 1 as libc::c_int)
                & 1 as libc::c_int != 0
            {
                demerit += 3 as libc::c_int;
            }
            p = p.offset(1);
            p;
            x += 1;
            x;
        }
        p = p.offset(1);
        p;
        y += 1;
        y;
    }
    return demerit;
}
pub unsafe extern "C" fn Mask_calcRunLengthH(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut runLength: *mut libc::c_int,
) -> libc::c_int {
    let mut head: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prev: libc::c_uchar = 0;
    if *frame.offset(0 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int != 0 {
        *runLength.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        head = 1 as libc::c_int;
    } else {
        head = 0 as libc::c_int;
    }
    *runLength.offset(head as isize) = 1 as libc::c_int;
    prev = *frame.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < width {
        if (*frame.offset(i as isize) as libc::c_int ^ prev as libc::c_int)
            & 1 as libc::c_int != 0
        {
            head += 1;
            head;
            *runLength.offset(head as isize) = 1 as libc::c_int;
            prev = *frame.offset(i as isize);
        } else {
            let ref mut fresh0 = *runLength.offset(head as isize);
            *fresh0 += 1;
            *fresh0;
        }
        i += 1;
        i;
    }
    return head + 1 as libc::c_int;
}
pub unsafe extern "C" fn Mask_calcRunLengthV(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut runLength: *mut libc::c_int,
) -> libc::c_int {
    let mut head: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prev: libc::c_uchar = 0;
    if *frame.offset(0 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int != 0 {
        *runLength.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        head = 1 as libc::c_int;
    } else {
        head = 0 as libc::c_int;
    }
    *runLength.offset(head as isize) = 1 as libc::c_int;
    prev = *frame.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < width {
        if (*frame.offset((i * width) as isize) as libc::c_int ^ prev as libc::c_int)
            & 1 as libc::c_int != 0
        {
            head += 1;
            head;
            *runLength.offset(head as isize) = 1 as libc::c_int;
            prev = *frame.offset((i * width) as isize);
        } else {
            let ref mut fresh1 = *runLength.offset(head as isize);
            *fresh1 += 1;
            *fresh1;
        }
        i += 1;
        i;
    }
    return head + 1 as libc::c_int;
}
pub unsafe extern "C" fn Mask_evaluateSymbol(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut demerit: libc::c_int = 0 as libc::c_int;
    let mut runLength: [libc::c_int; 178] = [0; 178];
    let mut length: libc::c_int = 0;
    demerit += Mask_calcN2(width, frame);
    y = 0 as libc::c_int;
    while y < width {
        length = Mask_calcRunLengthH(
            width,
            frame.offset((y * width) as isize),
            runLength.as_mut_ptr(),
        );
        demerit += Mask_calcN1N3(length, runLength.as_mut_ptr());
        y += 1;
        y;
    }
    x = 0 as libc::c_int;
    while x < width {
        length = Mask_calcRunLengthV(
            width,
            frame.offset(x as isize),
            runLength.as_mut_ptr(),
        );
        demerit += Mask_calcN1N3(length, runLength.as_mut_ptr());
        x += 1;
        x;
    }
    return demerit;
}
pub unsafe extern "C" fn Mask_mask(
    mut width: libc::c_int,
    mut frame: *mut libc::c_uchar,
    mut level: QRecLevel,
) -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0;
    let mut mask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bestMask: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut minDemerit: libc::c_int = 2147483647 as libc::c_int;
    let mut blacks: libc::c_int = 0;
    let mut bratio: libc::c_int = 0;
    let mut demerit: libc::c_int = 0;
    let mut w2: libc::c_int = width * width;
    mask = malloc(w2 as size_t) as *mut libc::c_uchar;
    if mask.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    bestMask = malloc(w2 as size_t) as *mut libc::c_uchar;
    if bestMask.is_null() {
        free(mask as *mut libc::c_void);
        return 0 as *mut libc::c_uchar;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        demerit = 0 as libc::c_int;
        blacks = (maskMakers[i as usize]).unwrap()(width, frame, mask);
        blacks += Mask_writeFormatInformation(width, mask, i, level);
        bratio = (200 as libc::c_int * blacks + w2) / w2 / 2 as libc::c_int;
        demerit = abs(bratio - 50 as libc::c_int) / 5 as libc::c_int * 10 as libc::c_int;
        demerit += Mask_evaluateSymbol(width, mask);
        if demerit < minDemerit {
            minDemerit = demerit;
            memcpy(
                bestMask as *mut libc::c_void,
                mask as *const libc::c_void,
                w2 as size_t,
            );
        }
        i += 1;
        i;
    }
    free(mask as *mut libc::c_void);
    return bestMask;
}
