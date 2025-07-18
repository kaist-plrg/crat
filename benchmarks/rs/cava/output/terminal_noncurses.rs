use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn wcscat(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
    fn swprintf(
        __s: *mut wchar_t,
        __n: size_t,
        __format: *const wchar_t,
        _: ...
    ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub static mut frame_buffer: *mut wchar_t = 0 as *const wchar_t as *mut wchar_t;
pub static mut barstring: [*mut wchar_t; 8] = [0 as *const wchar_t as *mut wchar_t; 8];
pub static mut spacestring: *mut wchar_t = 0 as *const wchar_t as *mut wchar_t;
pub static mut buf_length: libc::c_int = 0;
pub static mut ttyframe_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ttybarstring: [*mut libc::c_char; 8] = [0 as *const libc::c_char
    as *mut libc::c_char; 8];
pub static mut ttyspacestring: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ttybuf_length: libc::c_int = 0;
pub unsafe extern "C" fn setecho(
    mut fd: libc::c_int,
    mut onoff: libc::c_int,
) -> libc::c_int {
    let mut t: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    if tcgetattr(fd, &mut t) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if onoff == 0 as libc::c_int {
        t.c_lflag
            &= !(0o10 as libc::c_int | 0o20 as libc::c_int | 0o40 as libc::c_int
                | 0o100 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint;
    } else {
        t.c_lflag
            |= (0o10 as libc::c_int | 0o20 as libc::c_int | 0o40 as libc::c_int
                | 0o100 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint;
    }
    if tcsetattr(fd, 0 as libc::c_int, &mut t) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn free_terminal_noncurses() {
    free(frame_buffer as *mut libc::c_void);
    free(ttyframe_buffer as *mut libc::c_void);
    free(spacestring as *mut libc::c_void);
    free(ttyspacestring as *mut libc::c_void);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        free(barstring[i as usize] as *mut libc::c_void);
        free(ttybarstring[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn init_terminal_noncurses(
    mut tty: libc::c_int,
    mut col: libc::c_int,
    mut bgcol: libc::c_int,
    mut width: libc::c_int,
    mut lines: libc::c_int,
    mut bar_width: libc::c_int,
) -> libc::c_int {
    free_terminal_noncurses();
    if tty != 0 {
        ttybuf_length = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(width as libc::c_ulong)
            .wrapping_mul(lines as libc::c_ulong)
            .wrapping_mul(10 as libc::c_int as libc::c_ulong) as libc::c_int;
        ttyframe_buffer = malloc(ttybuf_length as libc::c_ulong) as *mut libc::c_char;
        ttyspacestring = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut libc::c_char;
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            ttybarstring[n
                as usize] = malloc(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut libc::c_char;
            *(ttybarstring[n as usize])
                .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            n += 1;
            n;
        }
        *ttyspacestring.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        *ttyframe_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        let mut n_0: libc::c_int = 0 as libc::c_int;
        while n_0 < bar_width {
            strcat(
                ttybarstring[0 as libc::c_int as usize],
                b"H\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[1 as libc::c_int as usize],
                b"A\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[2 as libc::c_int as usize],
                b"B\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[3 as libc::c_int as usize],
                b"C\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[4 as libc::c_int as usize],
                b"D\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[5 as libc::c_int as usize],
                b"E\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[6 as libc::c_int as usize],
                b"F\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[7 as libc::c_int as usize],
                b"G\0" as *const u8 as *const libc::c_char,
            );
            strcat(ttyspacestring, b" \0" as *const u8 as *const libc::c_char);
            n_0 += 1;
            n_0;
        }
    } else if tty == 0 {
        buf_length = (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(width as libc::c_ulong)
            .wrapping_mul(lines as libc::c_ulong)
            .wrapping_mul(10 as libc::c_int as libc::c_ulong) as libc::c_int;
        frame_buffer = malloc(buf_length as libc::c_ulong) as *mut wchar_t;
        spacestring = malloc(
            (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut wchar_t;
        let mut n_1: libc::c_int = 0 as libc::c_int;
        while n_1 < 8 as libc::c_int {
            barstring[n_1
                as usize] = malloc(
                (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                    .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut wchar_t;
            *(barstring[n_1 as usize]).offset(0 as libc::c_int as isize) = '\0' as i32;
            n_1 += 1;
            n_1;
        }
        *spacestring.offset(0 as libc::c_int as isize) = '\0' as i32;
        *frame_buffer.offset(0 as libc::c_int as isize) = '\0' as i32;
        let mut n_2: libc::c_int = 0 as libc::c_int;
        while n_2 < bar_width {
            wcscat(
                barstring[0 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x88%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                barstring[1 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x81%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                barstring[2 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x82%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                barstring[3 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x83%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                barstring[4 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x84%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                barstring[5 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x85%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                barstring[6 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x86%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                barstring[7 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b"\x87%\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            wcscat(
                spacestring,
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b" \0\0\0\0\0\0\0"))
                    .as_ptr(),
            );
            n_2 += 1;
            n_2;
        }
    }
    col += 30 as libc::c_int;
    system(b"setterm -cursor off\0" as *const u8 as *const libc::c_char);
    system(b"setterm -blank 0\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[0m\n\0" as *const u8 as *const libc::c_char);
    system(b"clear\0" as *const u8 as *const libc::c_char);
    if col != 0 {
        printf(b"\x1B[%dm\0" as *const u8 as *const libc::c_char, col);
    }
    if bgcol != 0 as libc::c_int {
        bgcol += 40 as libc::c_int;
        printf(b"\x1B[%dm\0" as *const u8 as *const libc::c_char, bgcol);
        let mut n_3: libc::c_int = lines;
        while n_3 >= 0 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < width {
                printf(b" \0" as *const u8 as *const libc::c_char);
                i += 1;
                i;
            }
            if n_3 != 0 as libc::c_int {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            } else {
                printf(b"\r\0" as *const u8 as *const libc::c_char);
            }
            n_3 -= 1;
            n_3;
        }
        printf(b"\x1B[%dA\0" as *const u8 as *const libc::c_char, lines);
    }
    setecho(0 as libc::c_int, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_terminal_dim_noncurses(
    mut width: *mut libc::c_int,
    mut lines: *mut libc::c_int,
) {
    let mut dim: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    ioctl(
        1 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut dim as *mut winsize,
    );
    *lines = dim.ws_row as libc::c_int;
    *width = dim.ws_col as libc::c_int;
    system(b"clear\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn draw_terminal_noncurses(
    mut tty: libc::c_int,
    mut lines: libc::c_int,
    mut width: libc::c_int,
    mut number_of_bars: libc::c_int,
    mut bar_width: libc::c_int,
    mut bar_spacing: libc::c_int,
    mut rest: libc::c_int,
    mut bars: *mut libc::c_int,
    mut previous_frame: *mut libc::c_int,
    mut x_axis_info: libc::c_int,
) -> libc::c_int {
    let mut current_cell: libc::c_int = 0;
    let mut prev_cell: libc::c_int = 0;
    let mut same_line: libc::c_int = 0;
    let mut new_line: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut dim: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    same_line = 0 as libc::c_int;
    new_line = 0 as libc::c_int;
    cx = 0 as libc::c_int;
    if tty == 0 {
        ioctl(
            1 as libc::c_int,
            0x5413 as libc::c_int as libc::c_ulong,
            &mut dim as *mut winsize,
        );
        if x_axis_info != 0 {
            lines += 1;
            lines;
        }
        if dim.ws_row as libc::c_int != lines || dim.ws_col as libc::c_int != width {
            return -(1 as libc::c_int);
        }
        if x_axis_info != 0 {
            lines -= 1;
            lines;
        }
    }
    if tty != 0 {
        *ttyframe_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else if tty == 0 {
        *frame_buffer.offset(0 as libc::c_int as isize) = '\0' as i32;
    }
    let mut current_line: libc::c_int = lines - 1 as libc::c_int;
    while current_line >= 0 as libc::c_int {
        let mut same_bar: libc::c_int = 0 as libc::c_int;
        let mut center_adjusted: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < number_of_bars {
            current_cell = *bars.offset(i as isize) - current_line * 8 as libc::c_int;
            prev_cell = *previous_frame.offset(i as isize)
                - current_line * 8 as libc::c_int;
            if current_cell < 1 as libc::c_int && prev_cell < 1 as libc::c_int
                || current_cell > 7 as libc::c_int && prev_cell > 7 as libc::c_int
                || current_cell == prev_cell
            {
                same_bar += 1;
                same_bar;
            } else if tty != 0 {
                if same_line > 0 as libc::c_int {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"\x1B[%dB\0" as *const u8 as *const libc::c_char,
                            same_line,
                        );
                    new_line += same_line;
                    same_line = 0 as libc::c_int;
                }
                if same_bar > 0 as libc::c_int {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                            (bar_width + bar_spacing) * same_bar,
                        );
                    same_bar = 0 as libc::c_int;
                }
                if center_adjusted == 0 && rest != 0 {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                            rest,
                        );
                    center_adjusted = 1 as libc::c_int;
                }
                if current_cell < 1 as libc::c_int {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            ttyspacestring,
                        );
                } else if current_cell > 7 as libc::c_int {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            ttybarstring[0 as libc::c_int as usize],
                        );
                } else {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            ttybarstring[current_cell as usize],
                        );
                }
                if bar_spacing != 0 {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                            bar_spacing,
                        );
                }
            } else if tty == 0 {
                if same_line > 0 as libc::c_int {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            (*::std::mem::transmute::<
                                &[u8; 24],
                                &[libc::c_int; 6],
                            >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0B\0\0\0\0\0\0\0"))
                                .as_ptr(),
                            same_line,
                        );
                    new_line += same_line;
                    same_line = 0 as libc::c_int;
                }
                if same_bar > 0 as libc::c_int {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            (*::std::mem::transmute::<
                                &[u8; 24],
                                &[libc::c_int; 6],
                            >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0C\0\0\0\0\0\0\0"))
                                .as_ptr(),
                            (bar_width + bar_spacing) * same_bar,
                        );
                    same_bar = 0 as libc::c_int;
                }
                if center_adjusted == 0 && rest != 0 {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            (*::std::mem::transmute::<
                                &[u8; 24],
                                &[libc::c_int; 6],
                            >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0C\0\0\0\0\0\0\0"))
                                .as_ptr(),
                            rest,
                        );
                    center_adjusted = 1 as libc::c_int;
                }
                if current_cell < 1 as libc::c_int {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            spacestring,
                        );
                } else if current_cell > 7 as libc::c_int {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            barstring[0 as libc::c_int as usize],
                        );
                } else {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            barstring[current_cell as usize],
                        );
                }
                if bar_spacing != 0 {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            (*::std::mem::transmute::<
                                &[u8; 24],
                                &[libc::c_int; 6],
                            >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0C\0\0\0\0\0\0\0"))
                                .as_ptr(),
                            bar_spacing,
                        );
                }
            }
            i += 1;
            i;
        }
        if same_bar != number_of_bars {
            if current_line != 0 as libc::c_int {
                if tty != 0 {
                    cx
                        += snprintf(
                            ttyframe_buffer.offset(cx as isize),
                            (ttybuf_length - cx) as libc::c_ulong,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                } else if tty == 0 {
                    cx
                        += swprintf(
                            frame_buffer.offset(cx as isize),
                            (buf_length - cx) as size_t,
                            (*::std::mem::transmute::<
                                &[u8; 8],
                                &[libc::c_int; 2],
                            >(b"\n\0\0\0\0\0\0\0"))
                                .as_ptr(),
                        );
                }
                new_line += 1;
                new_line;
            }
        } else {
            same_line += 1;
            same_line;
        }
        current_line -= 1;
        current_line;
    }
    if same_line != lines {
        if tty != 0 {
            printf(
                b"%s\r\x1B[%dA\0" as *const u8 as *const libc::c_char,
                ttyframe_buffer,
                new_line,
            );
        } else if tty == 0 {
            printf(
                b"%ls\r\x1B[%dA\0" as *const u8 as *const libc::c_char,
                frame_buffer,
                new_line,
            );
        }
        fflush(stdout);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cleanup_terminal_noncurses() {
    setecho(0 as libc::c_int, 1 as libc::c_int);
    printf(b"\x1B[0m\n\0" as *const u8 as *const libc::c_char);
    system(b"setfont  >/dev/null 2>&1\0" as *const u8 as *const libc::c_char);
    system(
        b"setfont /usr/share/consolefonts/Lat2-Fixed16.psf.gz  >/dev/null 2>&1\0"
            as *const u8 as *const libc::c_char,
    );
    system(b"setterm -cursor on\0" as *const u8 as *const libc::c_char);
    system(b"setterm -blank 10\0" as *const u8 as *const libc::c_char);
    system(b"clear\0" as *const u8 as *const libc::c_char);
}
