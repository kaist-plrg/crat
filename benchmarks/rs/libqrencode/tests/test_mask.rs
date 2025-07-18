use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn testReport(tests: libc::c_int);
    fn testFinish();
    fn testEnd(result: libc::c_int);
    fn QRcode_free(qrcode: *mut QRcode);
    fn QRcode_new(
        version: libc::c_int,
        width: libc::c_int,
        data: *mut libc::c_uchar,
    ) -> *mut QRcode;
    static mut assertionFailed: libc::c_int;
    static mut assertionNum: libc::c_int;
    fn testInit(tests: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn Mask_makeMask(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
        level: QRecLevel,
    ) -> *mut libc::c_uchar;
    fn Mask_calcN2(width: libc::c_int, frame: *mut libc::c_uchar) -> libc::c_int;
    fn Mask_calcN1N3(length: libc::c_int, runLength: *mut libc::c_int) -> libc::c_int;
    fn Mask_calcRunLengthH(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        runLength: *mut libc::c_int,
    ) -> libc::c_int;
    fn Mask_calcRunLengthV(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        runLength: *mut libc::c_int,
    ) -> libc::c_int;
    fn Mask_evaluateSymbol(width: libc::c_int, frame: *mut libc::c_uchar) -> libc::c_int;
    fn Mask_makeMaskedFrame(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
    ) -> *mut libc::c_uchar;
    fn QRspec_getWidth(version: libc::c_int) -> libc::c_int;
    fn QRspec_newFrame(version: libc::c_int) -> *mut libc::c_uchar;
    fn QRcode_decodeFormat(
        code: *mut QRcode,
        level: *mut QRecLevel,
        mask: *mut libc::c_int,
    ) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
static mut dot: [libc::c_char; 2] = [
    '_' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
];
static mut maskPatterns: [*mut libc::c_char; 8] = [
    b"#_#_#__#_#_##_#_#__#_#_##_#_#__#_#_#\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"######______######______######______\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"#__#__#__#__#__#__#__#__#__#__#__#__\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"#__#____#__#_#__#_#__#____#__#_#__#_\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"###___###______###___######___###___\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"#######_____#__#__#_#_#_#__#__#_____\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"#########___##_##_#_#_#_#_##_##___##\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"#_#_#____####___##_#_#_####____###__\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
];
unsafe extern "C" fn print_mask(mut mask: libc::c_int) {
    let w: libc::c_uint = 6 as libc::c_int as libc::c_uint;
    let mut frame: [libc::c_uchar; 36] = [0; 36];
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    memset(
        frame.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        w.wrapping_mul(w) as libc::c_ulong,
    );
    masked = Mask_makeMaskedFrame(w as libc::c_int, frame.as_mut_ptr(), mask);
    p = masked;
    y = 0 as libc::c_int;
    while (y as libc::c_uint) < w {
        x = 0 as libc::c_int;
        while (x as libc::c_uint) < w {
            putchar(dot[(*p as libc::c_int & 1 as libc::c_int) as usize] as libc::c_int);
            p = p.offset(1);
            p;
            x += 1;
            x;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        y += 1;
        y;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(masked as *mut libc::c_void);
}
unsafe extern "C" fn print_masks() {
    let mut i: libc::c_int = 0;
    puts(b"\nPrinting mask patterns.\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        print_mask(i);
        i += 1;
        i;
    }
}
unsafe extern "C" fn test_mask(mut mask: libc::c_int) -> libc::c_int {
    let w: libc::c_int = 6 as libc::c_int;
    let mut frame: [libc::c_uchar; 36] = [0; 36];
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    memset(
        frame.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (w * w) as libc::c_ulong,
    );
    masked = Mask_makeMaskedFrame(w, frame.as_mut_ptr(), mask);
    p = masked;
    q = maskPatterns[mask as usize];
    y = 0 as libc::c_int;
    while y < w {
        x = 0 as libc::c_int;
        while x < w {
            if dot[(*p as libc::c_int & 1 as libc::c_int) as usize] as libc::c_int
                != *q as libc::c_int
            {
                err += 1;
                err;
            }
            p = p.offset(1);
            p;
            q = q.offset(1);
            q;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    free(masked as *mut libc::c_void);
    return err;
}
unsafe extern "C" fn test_masks() {
    let mut i: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"test_masks\0"))
            .as_ptr(),
        b"Mask pattern checks\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        assertionNum += 1;
        assertionNum;
        if !(test_mask(i) == 0 as libc::c_int) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Mask pattern %d incorrect.\n\0" as *const u8 as *const libc::c_char,
                i,
            );
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_eval() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut w: libc::c_uint = 6 as libc::c_int as libc::c_uint;
    let mut demerit: libc::c_int = 0;
    frame = malloc(w.wrapping_mul(w) as libc::c_ulong) as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"test_eval\0"))
            .as_ptr(),
        b"Test mask evaluation (all white)\0" as *const u8 as *const libc::c_char,
    );
    memset(
        frame as *mut libc::c_void,
        0 as libc::c_int,
        w.wrapping_mul(w) as libc::c_ulong,
    );
    demerit = Mask_evaluateSymbol(w as libc::c_int, frame);
    testEnd(
        !(demerit as libc::c_uint
            == ((3 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
                .wrapping_mul(w)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (3 as libc::c_int as libc::c_uint)
                        .wrapping_mul(w.wrapping_sub(1 as libc::c_int as libc::c_uint))
                        .wrapping_mul(w.wrapping_sub(1 as libc::c_int as libc::c_uint)),
                )) as libc::c_int,
    );
    testStartReal(
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"test_eval\0"))
            .as_ptr(),
        b"Test mask evaluation (all black)\0" as *const u8 as *const libc::c_char,
    );
    memset(
        frame as *mut libc::c_void,
        1 as libc::c_int,
        w.wrapping_mul(w) as libc::c_ulong,
    );
    demerit = Mask_evaluateSymbol(w as libc::c_int, frame);
    testEnd(
        !(demerit as libc::c_uint
            == ((3 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
                .wrapping_mul(w)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (3 as libc::c_int as libc::c_uint)
                        .wrapping_mul(w.wrapping_sub(1 as libc::c_int as libc::c_uint))
                        .wrapping_mul(w.wrapping_sub(1 as libc::c_int as libc::c_uint)),
                )) as libc::c_int,
    );
    free(frame as *mut libc::c_void);
}
unsafe extern "C" fn test_eval2() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut w: libc::c_uint = 10 as libc::c_int as libc::c_uint;
    let mut demerit: libc::c_int = 0;
    let mut x: libc::c_uint = 0;
    frame = malloc(w.wrapping_mul(w) as libc::c_ulong) as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"test_eval2\0"))
            .as_ptr(),
        b"Test mask evaluation (run length penalty check)\0" as *const u8
            as *const libc::c_char,
    );
    x = 0 as libc::c_int as libc::c_uint;
    while x < w {
        *frame
            .offset(
                x as isize,
            ) = (x & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_add(x) as isize,
            ) = (x & 1 as libc::c_int as libc::c_uint ^ 1 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(2 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(2 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(3 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(2 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint ^ 1 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(4 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(3 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(5 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(3 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint ^ 1 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(6 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(4 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(7 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(4 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint ^ 1 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(8 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(5 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint) as libc::c_uchar;
        *frame
            .offset(
                w.wrapping_mul(9 as libc::c_int as libc::c_uint).wrapping_add(x) as isize,
            ) = (x.wrapping_div(5 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint ^ 1 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        x = x.wrapping_add(1);
        x;
    }
    demerit = Mask_evaluateSymbol(w as libc::c_int, frame);
    testEnd(
        !(demerit
            == 3 as libc::c_int * 4 as libc::c_int + 3 as libc::c_int * 4 as libc::c_int)
            as libc::c_int,
    );
    free(frame as *mut libc::c_void);
}
unsafe extern "C" fn test_calcN2() {
    let mut frame: [libc::c_uchar; 64] = [0; 64];
    let mut width: libc::c_int = 0;
    let mut demerit: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_calcN2\0"))
            .as_ptr(),
        b"Test mask evaluation (2x2 block check)\0" as *const u8 as *const libc::c_char,
    );
    width = 4 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            frame[(y * width + x)
                as usize] = ((x & 2 as libc::c_int ^ y & 2 as libc::c_int)
                >> 1 as libc::c_int) as libc::c_uchar;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    demerit = Mask_calcN2(width, frame.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(demerit == 3 as libc::c_int * 4 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Calculation of N2 demerit is wrong: %d, expected %d\0" as *const u8
                as *const libc::c_char,
            demerit,
            3 as libc::c_int * 4 as libc::c_int,
        );
    }
    width = 4 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            frame[(y * width + x)
                as usize] = ((x + 1 as libc::c_int & 2 as libc::c_int
                ^ y & 2 as libc::c_int) >> 1 as libc::c_int) as libc::c_uchar;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    demerit = Mask_calcN2(width, frame.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(demerit == 3 as libc::c_int * 2 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Calculation of N2 demerit is wrong: %d, expected %d\0" as *const u8
                as *const libc::c_char,
            demerit,
            3 as libc::c_int * 2 as libc::c_int,
        );
    }
    width = 6 as libc::c_int;
    y = 0 as libc::c_int;
    while y < width {
        x = 0 as libc::c_int;
        while x < width {
            frame[(y * width + x)
                as usize] = (x / 3 as libc::c_int ^ y / 3 as libc::c_int)
                as libc::c_uchar;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    demerit = Mask_calcN2(width, frame.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(demerit == 3 as libc::c_int * 16 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Calculation of N2 demerit is wrong: %d, expected %d\0" as *const u8
                as *const libc::c_char,
            demerit,
            3 as libc::c_int * 16 as libc::c_int,
        );
    }
    testFinish();
}
unsafe extern "C" fn test_eval3() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut w: libc::c_int = 15 as libc::c_int;
    let mut demerit: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    static mut pattern: [[libc::c_uchar; 15]; 7] = [
        [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
    ];
    frame = malloc((w * w) as libc::c_ulong) as *mut libc::c_uchar;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"test_eval3\0"))
            .as_ptr(),
        b"Test mask evaluation (1:1:3:1:1 check)\0" as *const u8 as *const libc::c_char,
    );
    y = 0 as libc::c_int;
    while y < 5 as libc::c_int {
        x = 0 as libc::c_int;
        while x < w {
            *frame
                .offset(
                    (w * y * 2 as libc::c_int + x) as isize,
                ) = pattern[y as usize][x as usize];
            *frame
                .offset(
                    (w * (y * 2 as libc::c_int + 1 as libc::c_int) + x) as isize,
                ) = (pattern[y as usize][x as usize] as libc::c_int ^ 1 as libc::c_int)
                as libc::c_uchar;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    x = 0 as libc::c_int;
    while x < w {
        *frame
            .offset(
                (w * 10 as libc::c_int + x) as isize,
            ) = (x & 1 as libc::c_int) as libc::c_uchar;
        x += 1;
        x;
    }
    y = 5 as libc::c_int;
    while y < 7 as libc::c_int {
        x = 0 as libc::c_int;
        while x < w {
            *frame
                .offset(
                    (w * (y * 2 as libc::c_int + 1 as libc::c_int) + x) as isize,
                ) = pattern[y as usize][x as usize];
            *frame
                .offset(
                    (w * (y * 2 as libc::c_int + 2 as libc::c_int) + x) as isize,
                ) = (pattern[y as usize][x as usize] as libc::c_int ^ 1 as libc::c_int)
                as libc::c_uchar;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    demerit = Mask_evaluateSymbol(w, frame);
    testEnd(
        !(demerit
            == 40 as libc::c_int * 10 as libc::c_int
                + (3 as libc::c_int + 1 as libc::c_int) * 4 as libc::c_int)
            as libc::c_int,
    );
    free(frame as *mut libc::c_void);
}
unsafe extern "C" fn test_format() {
    let mut frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut version: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut dmask: libc::c_int = 0;
    let mut level: QRecLevel = QR_ECLEVEL_L;
    let mut dlevel: QRecLevel = QR_ECLEVEL_L;
    let mut code: *mut QRcode = 0 as *mut QRcode;
    let mut ret: libc::c_int = 0;
    testStartReal(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"test_format\0"))
            .as_ptr(),
        b"Checking format info.\0" as *const u8 as *const libc::c_char,
    );
    version = 1 as libc::c_int;
    while version <= 40 as libc::c_int {
        frame = QRspec_newFrame(version);
        width = QRspec_getWidth(version);
        level = QR_ECLEVEL_L;
        while level as libc::c_uint <= QR_ECLEVEL_H as libc::c_int as libc::c_uint {
            mask = 0 as libc::c_int;
            while mask < 8 as libc::c_int {
                masked = Mask_makeMask(width, frame, mask, level);
                code = QRcode_new(version, width, masked);
                ret = QRcode_decodeFormat(code, &mut dlevel, &mut dmask);
                assertionNum += 1;
                assertionNum;
                if !(ret == 0 as libc::c_int) {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(
                        b"Something wrong in format info.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                assertionNum += 1;
                assertionNum;
                if !(dlevel as libc::c_uint == level as libc::c_uint) {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(
                        b"Decoded level is wrong: %d, expected %d\0" as *const u8
                            as *const libc::c_char,
                        dlevel as libc::c_uint,
                        level as libc::c_uint,
                    );
                }
                assertionNum += 1;
                assertionNum;
                if !(dmask == mask) {
                    assertionFailed += 1;
                    assertionFailed;
                    printf(
                        b"Decoded mask is wrong: %d, expected %d\0" as *const u8
                            as *const libc::c_char,
                        dlevel as libc::c_uint,
                        level as libc::c_uint,
                    );
                }
                QRcode_free(code);
                mask += 1;
                mask;
            }
            level += 1;
            level;
        }
        free(frame as *mut libc::c_void);
        version += 1;
        version;
    }
    testFinish();
}
unsafe extern "C" fn test_calcRunLength() {
    let mut width: libc::c_int = 5 as libc::c_int;
    let vla = (width * width) as usize;
    let mut frame: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let vla_0 = (width + 1 as libc::c_int) as usize;
    let mut runLength: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    static mut pattern: [[libc::c_uchar; 5]; 6] = [
        [
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
        [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
        [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
    ];
    static mut expected: [[libc::c_int; 7]; 6] = [
        [
            1 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            5 as libc::c_int,
        ],
        [
            -(1 as libc::c_int),
            1 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            6 as libc::c_int,
        ],
        [
            5 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        ],
        [
            -(1 as libc::c_int),
            5 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
        ],
        [
            2 as libc::c_int,
            3 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
        ],
        [
            -(1 as libc::c_int),
            2 as libc::c_int,
            3 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            3 as libc::c_int,
        ],
    ];
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"test_calcRunLength\0"))
            .as_ptr(),
        b"Test runlength calc function\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        length = Mask_calcRunLengthH(
            width,
            (pattern[i as usize]).as_mut_ptr(),
            runLength.as_mut_ptr(),
        );
        assertionNum += 1;
        assertionNum;
        if !(expected[i as usize][6 as libc::c_int as usize] == length) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Length incorrect: %d, expected %d.\n\0" as *const u8
                    as *const libc::c_char,
                length,
                expected[i as usize][6 as libc::c_int as usize],
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(memcmp(
            runLength.as_mut_ptr() as *const libc::c_void,
            (expected[i as usize]).as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(
                    expected[i as usize][6 as libc::c_int as usize] as libc::c_ulong,
                ),
        ) == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Run length does not match: pattern %d, horizontal access.\n\0"
                    as *const u8 as *const libc::c_char,
                i,
            );
        }
        j = 0 as libc::c_int;
        while j < width {
            *frame
                .as_mut_ptr()
                .offset((j * width) as isize) = pattern[i as usize][j as usize];
            j += 1;
            j;
        }
        length = Mask_calcRunLengthV(width, frame.as_mut_ptr(), runLength.as_mut_ptr());
        assertionNum += 1;
        assertionNum;
        if !(expected[i as usize][6 as libc::c_int as usize] == length) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Length incorrect: %d, expected %d.\n\0" as *const u8
                    as *const libc::c_char,
                length,
                expected[i as usize][6 as libc::c_int as usize],
            );
        }
        assertionNum += 1;
        assertionNum;
        if !(memcmp(
            runLength.as_mut_ptr() as *const libc::c_void,
            (expected[i as usize]).as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(
                    expected[i as usize][6 as libc::c_int as usize] as libc::c_ulong,
                ),
        ) == 0 as libc::c_int)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"Run length does not match: pattern %d, vertical access.\n\0"
                    as *const u8 as *const libc::c_char,
                i,
            );
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe extern "C" fn test_calcN1N3() {
    let mut runLength: [libc::c_int; 26] = [0; 26];
    let mut length: libc::c_int = 0;
    let mut demerit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    static mut pattern: [[libc::c_uchar; 16]; 6] = [
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
        ],
        [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            (40 as libc::c_int * 2 as libc::c_int) as libc::c_uchar,
        ],
    ];
    static mut pattern2: [[libc::c_uchar; 19]; 5] = [
        [
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            (40 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int) as libc::c_uchar,
        ],
        [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            (40 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int) as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            (3 as libc::c_int + 1 as libc::c_int) as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            (40 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int) as libc::c_uchar,
        ],
        [
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            (40 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int) as libc::c_uchar,
        ],
    ];
    testStartReal(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"test_calcN1N3\0"))
            .as_ptr(),
        b"Test N3 penalty calculation\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        length = Mask_calcRunLengthH(
            15 as libc::c_int,
            (pattern[i as usize]).as_mut_ptr(),
            runLength.as_mut_ptr(),
        );
        demerit = Mask_calcN1N3(length, runLength.as_mut_ptr());
        assertionNum += 1;
        assertionNum;
        if !(pattern[i as usize][15 as libc::c_int as usize] as libc::c_int == demerit) {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"N3 penalty is wrong: %d, expected %d\n\0" as *const u8
                    as *const libc::c_char,
                demerit,
                pattern[i as usize][15 as libc::c_int as usize] as libc::c_int,
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        length = Mask_calcRunLengthH(
            18 as libc::c_int,
            (pattern2[i as usize]).as_mut_ptr(),
            runLength.as_mut_ptr(),
        );
        demerit = Mask_calcN1N3(length, runLength.as_mut_ptr());
        assertionNum += 1;
        assertionNum;
        if !(pattern2[i as usize][18 as libc::c_int as usize] as libc::c_int == demerit)
        {
            assertionFailed += 1;
            assertionFailed;
            printf(
                b"N3 penalty is wrong: %d, expected %d\n\0" as *const u8
                    as *const libc::c_char,
                demerit,
                pattern2[i as usize][18 as libc::c_int as usize] as libc::c_int,
            );
        }
        i += 1;
        i;
    }
    testFinish();
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tests: libc::c_int = 9 as libc::c_int;
    testInit(tests);
    test_masks();
    test_eval();
    test_eval2();
    test_eval3();
    test_format();
    test_calcN2();
    test_calcRunLength();
    test_calcN1N3();
    testReport(tests);
    if argc > 1 as libc::c_int {
        print_masks();
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
