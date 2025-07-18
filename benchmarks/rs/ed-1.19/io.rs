use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn clearerr(__stream: *mut FILE);
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn current_addr() -> libc::c_int;
    fn get_sbuf_line(lp: *const line_t) -> *mut libc::c_char;
    fn inc_addr(addr: libc::c_int) -> libc::c_int;
    fn isbinary() -> bool_0;
    fn last_addr() -> libc::c_int;
    fn put_sbuf_line(buf: *const libc::c_char, size: libc::c_int) -> *const libc::c_char;
    fn search_line_node(addr: libc::c_int) -> *mut line_t;
    fn set_binary();
    fn set_current_addr(addr: libc::c_int);
    fn push_undo_atom(
        type_0: libc::c_int,
        from: libc::c_int,
        to: libc::c_int,
    ) -> *mut undo_t;
    fn scripted() -> bool_0;
    fn show_strerror(filename: *const libc::c_char, errcode: libc::c_int);
    fn strip_cr() -> bool_0;
    fn traditional() -> bool_0;
    fn invalid_address();
    fn set_error_msg(msg: *const libc::c_char);
    fn disable_interrupts();
    fn enable_interrupts();
    fn resize_buffer(
        buf: *mut *mut libc::c_char,
        size: *mut libc::c_int,
        min_size: libc::c_uint,
    ) -> bool_0;
    fn window_columns() -> libc::c_int;
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
pub type Bool = libc::c_uint;
pub const true_0: Bool = 1;
pub const false_0: Bool = 0;
pub type bool_0 = Bool;
pub type Pflags = libc::c_uint;
pub const pf_p: Pflags = 4;
pub const pf_n: Pflags = 2;
pub const pf_l: Pflags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub q_forw: *mut line,
    pub q_back: *mut line,
    pub pos: libc::c_long,
    pub len: libc::c_int,
}
pub type line_t = line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct undo_t {
    pub type_0: C2RustUnnamed,
    pub head: *mut line_t,
    pub tail: *mut line_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const VMOV: C2RustUnnamed = 3;
pub const UMOV: C2RustUnnamed = 2;
pub const UDEL: C2RustUnnamed = 1;
pub const UADD: C2RustUnnamed = 0;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
static mut unterminated_line: *const line_t = 0 as *const line_t;
static mut linenum_: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn linenum() -> libc::c_int {
    return linenum_;
}
pub unsafe extern "C" fn reset_unterminated_line() {
    unterminated_line = 0 as *const line_t;
}
pub unsafe extern "C" fn unmark_unterminated_line(lp: *const line_t) {
    if unterminated_line == lp {
        unterminated_line = 0 as *const line_t;
    }
}
unsafe extern "C" fn unterminated_last_line() -> bool_0 {
    return (!unterminated_line.is_null()
        && unterminated_line == search_line_node(last_addr()) as *const line_t)
        as libc::c_int as bool_0;
}
unsafe extern "C" fn print_line(
    mut p: *const libc::c_char,
    mut len: libc::c_int,
    pflags: libc::c_int,
) {
    let escapes: [libc::c_char; 8] = *::std::mem::transmute::<
        &[u8; 8],
        &[libc::c_char; 8],
    >(b"\x07\x08\x0C\n\r\t\x0B\0");
    let escchars: [libc::c_char; 8] = *::std::mem::transmute::<
        &[u8; 8],
        &[libc::c_char; 8],
    >(b"abfnrtv\0");
    let mut col: libc::c_int = 0 as libc::c_int;
    if pflags & pf_n as libc::c_int != 0 {
        printf(b"%d\t\0" as *const u8 as *const libc::c_char, current_addr());
        col = 8 as libc::c_int;
    }
    loop {
        len -= 1;
        if !(len >= 0 as libc::c_int) {
            break;
        }
        let fresh0 = p;
        p = p.offset(1);
        let ch: libc::c_uchar = *fresh0 as libc::c_uchar;
        if pflags & pf_l as libc::c_int == 0 {
            putchar(ch as libc::c_int);
        } else {
            col += 1;
            if col > window_columns() {
                col = 1 as libc::c_int;
                fputs(b"\\\n\0" as *const u8 as *const libc::c_char, stdout);
            }
            if ch as libc::c_int >= 32 as libc::c_int
                && ch as libc::c_int <= 126 as libc::c_int
            {
                if ch as libc::c_int == '$' as i32 || ch as libc::c_int == '\\' as i32 {
                    col += 1;
                    col;
                    putchar('\\' as i32);
                }
                putchar(ch as libc::c_int);
            } else {
                let p_0: *mut libc::c_char = strchr(escapes.as_ptr(), ch as libc::c_int);
                col += 1;
                col;
                putchar('\\' as i32);
                if ch as libc::c_int != 0 && !p_0.is_null() {
                    putchar(
                        escchars[p_0.offset_from(escapes.as_ptr()) as libc::c_long
                            as usize] as libc::c_int,
                    );
                } else {
                    col += 2 as libc::c_int;
                    putchar(
                        (ch as libc::c_int >> 6 as libc::c_int & 7 as libc::c_int)
                            + '0' as i32,
                    );
                    putchar(
                        (ch as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int)
                            + '0' as i32,
                    );
                    putchar((ch as libc::c_int & 7 as libc::c_int) + '0' as i32);
                }
            }
        }
    }
    if traditional() as u64 == 0 && pflags & pf_l as libc::c_int != 0 {
        putchar('$' as i32);
    }
    putchar('\n' as i32);
}
pub unsafe extern "C" fn print_lines(
    mut from: libc::c_int,
    to: libc::c_int,
    pflags: libc::c_int,
) -> bool_0 {
    let ep: *mut line_t = search_line_node(inc_addr(to));
    let mut bp: *mut line_t = search_line_node(from);
    if from == 0 {
        invalid_address();
        return false_0;
    }
    while bp != ep {
        let s: *const libc::c_char = get_sbuf_line(bp);
        if s.is_null() {
            return false_0;
        }
        let fresh1 = from;
        from = from + 1;
        set_current_addr(fresh1);
        print_line(s, (*bp).len, pflags);
        bp = (*bp).q_forw;
    }
    return true_0;
}
unsafe extern "C" fn trailing_escape(
    s: *const libc::c_char,
    mut len: libc::c_int,
) -> bool_0 {
    let mut odd_escape: bool_0 = false_0;
    loop {
        len -= 1;
        if !(len >= 0 as libc::c_int
            && *s.offset(len as isize) as libc::c_int == '\\' as i32)
        {
            break;
        }
        odd_escape = (odd_escape as u64 == 0) as libc::c_int as bool_0;
    }
    return odd_escape;
}
pub unsafe extern "C" fn get_extended_line(
    ibufpp: *mut *const libc::c_char,
    lenp: *mut libc::c_int,
    strip_escaped_newlines: bool_0,
) -> bool_0 {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    len = 0 as libc::c_int;
    loop {
        let fresh2 = len;
        len = len + 1;
        if !(*(*ibufpp).offset(fresh2 as isize) as libc::c_int != '\n' as i32) {
            break;
        }
    }
    if len < 2 as libc::c_int
        || trailing_escape(*ibufpp, len - 1 as libc::c_int) as u64 == 0
    {
        if !lenp.is_null() {
            *lenp = len;
        }
        return true_0;
    }
    if resize_buffer(&mut buf, &mut bufsz, (len + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return false_0;
    }
    memcpy(
        buf as *mut libc::c_void,
        *ibufpp as *const libc::c_void,
        len as libc::c_ulong,
    );
    len -= 1;
    len;
    *buf.offset((len - 1 as libc::c_int) as isize) = '\n' as i32 as libc::c_char;
    if strip_escaped_newlines as u64 != 0 {
        len -= 1;
        len;
    }
    while true_0 as libc::c_int != 0 {
        let mut len2: libc::c_int = 0;
        let s: *const libc::c_char = get_stdin_line(&mut len2);
        if s.is_null() {
            return false_0;
        }
        if len2 <= 0 as libc::c_int {
            return false_0;
        }
        if resize_buffer(
            &mut buf,
            &mut bufsz,
            (len + len2 + 1 as libc::c_int) as libc::c_uint,
        ) as u64 == 0
        {
            return false_0;
        }
        memcpy(
            buf.offset(len as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len2 as libc::c_ulong,
        );
        len += len2;
        if len2 < 2 as libc::c_int
            || trailing_escape(buf, len - 1 as libc::c_int) as u64 == 0
        {
            break;
        }
        len -= 1;
        len;
        *buf.offset((len - 1 as libc::c_int) as isize) = '\n' as i32 as libc::c_char;
        if strip_escaped_newlines as u64 != 0 {
            len -= 1;
            len;
        }
    }
    *buf.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    *ibufpp = buf;
    if !lenp.is_null() {
        *lenp = len;
    }
    return true_0;
}
pub unsafe extern "C" fn get_stdin_line(sizep: *mut libc::c_int) -> *const libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while true_0 as libc::c_int != 0 {
        let c: libc::c_int = getchar();
        if resize_buffer(&mut buf, &mut bufsz, (i + 2 as libc::c_int) as libc::c_uint)
            as u64 == 0
        {
            *sizep = 0 as libc::c_int;
            return 0 as *const libc::c_char;
        }
        if c == -(1 as libc::c_int) {
            if ferror(stdin) != 0 {
                show_strerror(
                    b"stdin\0" as *const u8 as *const libc::c_char,
                    *__errno_location(),
                );
                set_error_msg(
                    b"Cannot read stdin\0" as *const u8 as *const libc::c_char,
                );
                clearerr(stdin);
                *sizep = 0 as libc::c_int;
                return 0 as *const libc::c_char;
            }
            if feof(stdin) != 0 {
                set_error_msg(
                    b"Unexpected end-of-file\0" as *const u8 as *const libc::c_char,
                );
                clearerr(stdin);
                *buf
                    .offset(
                        0 as libc::c_int as isize,
                    ) = 0 as libc::c_int as libc::c_char;
                *sizep = 0 as libc::c_int;
                if i > 0 as libc::c_int {
                    linenum_ += 1;
                    linenum_;
                }
                return buf;
            }
        } else {
            let fresh3 = i;
            i = i + 1;
            *buf.offset(fresh3 as isize) = c as libc::c_char;
            if c == 0 as libc::c_int {
                set_binary();
            }
            if c != '\n' as i32 {
                continue;
            }
            linenum_ += 1;
            linenum_;
            *buf.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            *sizep = i;
            return buf;
        }
    }
    panic!();
}
unsafe extern "C" fn read_stream_line(
    filename: *const libc::c_char,
    fp: *mut FILE,
    sizep: *mut libc::c_int,
    newline_addedp: *mut bool_0,
) -> *const libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while true_0 as libc::c_int != 0 {
        if resize_buffer(&mut buf, &mut bufsz, (i + 2 as libc::c_int) as libc::c_uint)
            as u64 == 0
        {
            return 0 as *const libc::c_char;
        }
        c = getc(fp);
        if c == -(1 as libc::c_int) {
            break;
        }
        let fresh4 = i;
        i = i + 1;
        *buf.offset(fresh4 as isize) = c as libc::c_char;
        if c == 0 as libc::c_int {
            set_binary();
        } else {
            if !(c == '\n' as i32) {
                continue;
            }
            if strip_cr() as libc::c_uint != 0 && i > 1 as libc::c_int
                && *buf.offset((i - 2 as libc::c_int) as isize) as libc::c_int
                    == '\r' as i32
            {
                *buf
                    .offset(
                        (i - 2 as libc::c_int) as isize,
                    ) = '\n' as i32 as libc::c_char;
                i -= 1;
                i;
            }
            break;
        }
    }
    *buf.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    if c == -(1 as libc::c_int) {
        if ferror(fp) != 0 {
            show_strerror(filename, *__errno_location());
            set_error_msg(
                b"Cannot read input file\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *const libc::c_char;
        } else if i != 0 {
            *buf.offset(i as isize) = '\n' as i32 as libc::c_char;
            *buf
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_char;
            *newline_addedp = true_0;
            if isbinary() as u64 == 0 {
                i += 1;
                i;
            }
        }
    }
    *sizep = i;
    return buf;
}
unsafe extern "C" fn read_stream(
    filename: *const libc::c_char,
    fp: *mut FILE,
    addr: libc::c_int,
) -> libc::c_long {
    let mut lp: *mut line_t = search_line_node(addr);
    let mut up: *mut undo_t = 0 as *mut undo_t;
    let mut total_size: libc::c_long = 0 as libc::c_int as libc::c_long;
    let o_isbinary: bool_0 = isbinary();
    let appended: bool_0 = (addr == last_addr()) as libc::c_int as bool_0;
    let o_unterminated_last_line: bool_0 = unterminated_last_line();
    let mut newline_added: bool_0 = false_0;
    set_current_addr(addr);
    while true_0 as libc::c_int != 0 {
        let mut size: libc::c_int = 0 as libc::c_int;
        let s: *const libc::c_char = read_stream_line(
            filename,
            fp,
            &mut size,
            &mut newline_added,
        );
        if s.is_null() {
            return -(1 as libc::c_int) as libc::c_long;
        }
        if size <= 0 as libc::c_int {
            break;
        }
        total_size += size as libc::c_long;
        disable_interrupts();
        if (put_sbuf_line(
            s,
            (size as libc::c_uint).wrapping_add(newline_added as libc::c_uint)
                as libc::c_int,
        ))
            .is_null()
        {
            enable_interrupts();
            return -(1 as libc::c_int) as libc::c_long;
        }
        lp = (*lp).q_forw;
        if !up.is_null() {
            (*up).tail = lp;
        } else {
            up = push_undo_atom(UADD as libc::c_int, current_addr(), current_addr());
            if up.is_null() {
                enable_interrupts();
                return -(1 as libc::c_int) as libc::c_long;
            }
        }
        enable_interrupts();
    }
    if addr != 0 && appended as libc::c_uint != 0 && total_size != 0
        && o_unterminated_last_line as libc::c_uint != 0
    {
        fputs(b"Newline inserted\n\0" as *const u8 as *const libc::c_char, stdout);
    } else if newline_added as libc::c_uint != 0
        && (appended as u64 == 0 || isbinary() as u64 == 0)
    {
        fputs(b"Newline appended\n\0" as *const u8 as *const libc::c_char, stdout);
    }
    if appended as u64 == 0 && isbinary() as libc::c_uint != 0 && o_isbinary as u64 == 0
        && newline_added as libc::c_uint != 0
    {
        total_size += 1;
        total_size;
    }
    if appended as libc::c_uint != 0 && isbinary() as libc::c_uint != 0
        && (newline_added as libc::c_uint != 0
            || total_size == 0 as libc::c_int as libc::c_long)
    {
        unterminated_line = search_line_node(last_addr());
    }
    return total_size;
}
pub unsafe extern "C" fn read_file(
    filename: *const libc::c_char,
    addr: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut size: libc::c_long = 0;
    let mut ret: libc::c_int = 0;
    if *filename as libc::c_int == '!' as i32 {
        fp = popen(
            filename.offset(1 as libc::c_int as isize),
            b"r\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    }
    if fp.is_null() {
        show_strerror(filename, *__errno_location());
        set_error_msg(b"Cannot open input file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    size = read_stream(filename, fp, addr);
    if *filename as libc::c_int == '!' as i32 {
        ret = pclose(fp);
    } else {
        ret = fclose(fp);
    }
    if size < 0 as libc::c_int as libc::c_long {
        return -(2 as libc::c_int);
    }
    if ret != 0 as libc::c_int {
        show_strerror(filename, *__errno_location());
        set_error_msg(b"Cannot close input file\0" as *const u8 as *const libc::c_char);
        return -(2 as libc::c_int);
    }
    if scripted() as u64 == 0 {
        printf(b"%lu\n\0" as *const u8 as *const libc::c_char, size);
    }
    return current_addr() - addr;
}
unsafe extern "C" fn write_stream(
    filename: *const libc::c_char,
    fp: *mut FILE,
    mut from: libc::c_int,
    to: libc::c_int,
) -> libc::c_long {
    let mut lp: *mut line_t = search_line_node(from);
    let mut size: libc::c_long = 0 as libc::c_int as libc::c_long;
    while from != 0 && from <= to {
        let mut len: libc::c_int = 0;
        let mut p: *mut libc::c_char = get_sbuf_line(lp);
        if p.is_null() {
            return -(1 as libc::c_int) as libc::c_long;
        }
        len = (*lp).len;
        if from != last_addr() || isbinary() as u64 == 0
            || unterminated_last_line() as u64 == 0
        {
            let fresh5 = len;
            len = len + 1;
            *p.offset(fresh5 as isize) = '\n' as i32 as libc::c_char;
        }
        size += len as libc::c_long;
        loop {
            len -= 1;
            if !(len >= 0 as libc::c_int) {
                break;
            }
            let fresh6 = p;
            p = p.offset(1);
            if fputc(*fresh6 as libc::c_int, fp) == -(1 as libc::c_int) {
                show_strerror(filename, *__errno_location());
                set_error_msg(
                    b"Cannot write file\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int) as libc::c_long;
            }
        }
        from += 1;
        from;
        lp = (*lp).q_forw;
    }
    return size;
}
pub unsafe extern "C" fn write_file(
    filename: *const libc::c_char,
    mode: *const libc::c_char,
    from: libc::c_int,
    to: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut size: libc::c_long = 0;
    let mut ret: libc::c_int = 0;
    if *filename as libc::c_int == '!' as i32 {
        fp = popen(
            filename.offset(1 as libc::c_int as isize),
            b"w\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fp = fopen(filename, mode);
    }
    if fp.is_null() {
        show_strerror(filename, *__errno_location());
        set_error_msg(b"Cannot open output file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    size = write_stream(filename, fp, from, to);
    if *filename as libc::c_int == '!' as i32 {
        ret = pclose(fp);
    } else {
        ret = fclose(fp);
    }
    if size < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    if ret != 0 as libc::c_int {
        show_strerror(filename, *__errno_location());
        set_error_msg(b"Cannot close output file\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if scripted() as u64 == 0 {
        printf(b"%lu\n\0" as *const u8 as *const libc::c_char, size);
    }
    return if from != 0 && from <= to {
        to - from + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
