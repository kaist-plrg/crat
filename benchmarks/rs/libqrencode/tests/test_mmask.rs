use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn testReport(tests: libc::c_int);
    fn testFinish();
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testInit(tests: libc::c_int);
    static mut assertionNum: libc::c_int;
    static mut assertionFailed: libc::c_int;
    fn MMask_evaluateSymbol(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn MMask_makeMaskedFrame(
        width: libc::c_int,
        frame: *mut libc::c_uchar,
        mask: libc::c_int,
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
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
static mut dot: [libc::c_char; 2] = [
    '_' as i32 as libc::c_char,
    '#' as i32 as libc::c_char,
];
static mut maskPatterns: [*mut libc::c_char; 4] = [
    b"######______######______######______\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"###___###______###___######___###___\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"#########___##_##_#_#_#_#_##_##___##\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"#_#_#____####___##_#_#_####____###__\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
];
unsafe extern "C" fn print_mask(mut mask: libc::c_int) {
    let w: libc::c_int = 6 as libc::c_int;
    let mut frame: [libc::c_uchar; 36] = [0; 36];
    let mut masked: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    memset(
        frame.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (w * w) as libc::c_ulong,
    );
    masked = MMask_makeMaskedFrame(w, frame.as_mut_ptr(), mask);
    p = masked;
    y = 0 as libc::c_int;
    while y < w {
        x = 0 as libc::c_int;
        while x < w {
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
    while i < 4 as libc::c_int {
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
    masked = MMask_makeMaskedFrame(w, frame.as_mut_ptr(), mask);
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
    while i < 4 as libc::c_int {
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
unsafe extern "C" fn test_maskEvaluation() {
    static mut w: libc::c_int = 11 as libc::c_int;
    let mut pattern: [libc::c_uchar; 121] = [0; 121];
    let mut i: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    memset(
        pattern.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (w * w) as libc::c_ulong,
    );
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"test_maskEvaluation\0"))
            .as_ptr(),
        b"Test mask evaluation\0" as *const u8 as *const libc::c_char,
    );
    score = MMask_evaluateSymbol(w, pattern.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(score == 0 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Mask score caluculation is incorrect. (score=%d (%d expected)\n\0"
                as *const u8 as *const libc::c_char,
            score,
            0 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < w {
        pattern[((w - 1 as libc::c_int) * w + i)
            as usize] = 1 as libc::c_int as libc::c_uchar;
        i += 1;
        i;
    }
    score = MMask_evaluateSymbol(w, pattern.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(score == 16 as libc::c_int + w - 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Mask score caluculation is incorrect. (score=%d) (%d expected)\n\0"
                as *const u8 as *const libc::c_char,
            score,
            16 as libc::c_int + w - 1 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < w {
        pattern[((w - 1 as libc::c_int) * w + i)
            as usize] = 0 as libc::c_int as libc::c_uchar;
        pattern[(i * w + w - 1 as libc::c_int)
            as usize] = 1 as libc::c_int as libc::c_uchar;
        i += 1;
        i;
    }
    score = MMask_evaluateSymbol(w, pattern.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(score == 16 as libc::c_int + w - 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Mask score caluculation is incorrect. (score=%d) (%d expected)\n\0"
                as *const u8 as *const libc::c_char,
            score,
            16 as libc::c_int + w - 1 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < w {
        pattern[((w - 1 as libc::c_int) * w + i)
            as usize] = 1 as libc::c_int as libc::c_uchar;
        pattern[(i * w + w - 1 as libc::c_int)
            as usize] = 1 as libc::c_int as libc::c_uchar;
        i += 1;
        i;
    }
    score = MMask_evaluateSymbol(w, pattern.as_mut_ptr());
    assertionNum += 1;
    assertionNum;
    if !(score == 16 as libc::c_int * (w - 1 as libc::c_int) + w - 1 as libc::c_int) {
        assertionFailed += 1;
        assertionFailed;
        printf(
            b"Mask score caluculation is incorrect. (score=%d) (%d expected)\n\0"
                as *const u8 as *const libc::c_char,
            score,
            16 as libc::c_int * (w - 1 as libc::c_int) + w - 1 as libc::c_int,
        );
    }
    testFinish();
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tests: libc::c_int = 2 as libc::c_int;
    testInit(tests);
    test_masks();
    test_maskEvaluation();
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
