use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn toupper(_: libc::c_int) -> libc::c_int;
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
pub static mut at_start_of_line: libc::c_int = 1 as libc::c_int;
pub static mut at_start_of_line_for_dot: libc::c_int = 1 as libc::c_int;
pub static mut to_upper: libc::c_int = 0 as libc::c_int;
pub static mut ignore: libc::c_int = 0 as libc::c_int;
pub static mut disabled: libc::c_int = 0 as libc::c_int;
pub static mut in_section: libc::c_int = 0 as libc::c_int;
pub static mut in_item: libc::c_int = 0 as libc::c_int;
pub static mut in_table: libc::c_int = 0 as libc::c_int;
pub static mut no_break: libc::c_int = 0 as libc::c_int;
pub static mut indentation: libc::c_int = 0 as libc::c_int;
pub static mut bold: libc::c_int = 0 as libc::c_int;
pub static mut underlined: libc::c_int = 0 as libc::c_int;
pub static mut texinfoname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn where_0(mut line_no: libc::c_int) -> *mut libc::c_char {
    static mut b: [libc::c_char; 256] = [0; 256];
    sprintf(
        b.as_mut_ptr(),
        b"%s line %-3d\0" as *const u8 as *const libc::c_char,
        texinfoname,
        line_no,
    );
    return b.as_mut_ptr();
}
unsafe extern "C" fn start_bold() {
    if bold == 0 && underlined == 0 {
        if at_start_of_line != 0 {
            printf(b".B \0" as *const u8 as *const libc::c_char);
            at_start_of_line_for_dot = 0 as libc::c_int;
        } else {
            printf(b"\\fB\0" as *const u8 as *const libc::c_char);
            at_start_of_line_for_dot = at_start_of_line;
        }
        at_start_of_line = 0 as libc::c_int;
    }
    bold += 1;
    bold;
}
unsafe extern "C" fn stop_bold() {
    bold -= 1;
    if bold == 0 as libc::c_int && underlined == 0 {
        printf(b"\\fR\0" as *const u8 as *const libc::c_char);
        at_start_of_line_for_dot = at_start_of_line;
        at_start_of_line = 0 as libc::c_int;
    }
}
unsafe extern "C" fn start_I() {
    if underlined == 0 {
        if at_start_of_line != 0 {
            printf(b".I \0" as *const u8 as *const libc::c_char);
            at_start_of_line_for_dot = 0 as libc::c_int;
        } else {
            printf(b"\\fI\0" as *const u8 as *const libc::c_char);
            at_start_of_line_for_dot = at_start_of_line;
        }
        at_start_of_line = 0 as libc::c_int;
    }
    underlined += 1;
    underlined;
}
unsafe extern "C" fn stop_I() {
    underlined -= 1;
    if underlined == 0 as libc::c_int {
        if bold != 0 {
            if at_start_of_line != 0 {
                printf(b".B \0" as *const u8 as *const libc::c_char);
                at_start_of_line_for_dot = 0 as libc::c_int;
            } else {
                printf(b"\\fB\0" as *const u8 as *const libc::c_char);
                at_start_of_line_for_dot = at_start_of_line;
            }
        } else {
            printf(b"\\fR\0" as *const u8 as *const libc::c_char);
            at_start_of_line_for_dot = at_start_of_line;
        }
    }
    at_start_of_line = 0 as libc::c_int;
}
unsafe extern "C" fn putone(mut c: libc::c_char) {
    if disabled != 0 || in_item != 0 && c as libc::c_int == ' ' as i32 {
        return;
    }
    if at_start_of_line_for_dot != 0 && c as libc::c_int == '.' as i32 {
        if at_start_of_line == 0 {
            printf(b" \x08.\0" as *const u8 as *const libc::c_char);
        } else if underlined != 0 {
            printf(b".I .\x0CI\0" as *const u8 as *const libc::c_char);
        } else if bold != 0 {
            printf(b".B .\x0CB\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b" \x08.\0" as *const u8 as *const libc::c_char);
        }
        return;
    }
    if c as libc::c_int == '\'' as i32 {
        printf(b"\\(cq\0" as *const u8 as *const libc::c_char);
        return;
    }
    if c as libc::c_int == '`' as i32 {
        printf(b"\\(oq\0" as *const u8 as *const libc::c_char);
        return;
    }
    if no_break != 0 && c as libc::c_int == '\n' as i32 {
        c = ' ' as i32 as libc::c_char;
    }
    if no_break != 0 && c as libc::c_int == ' ' as i32 {
        putchar('\\' as i32);
    }
    if to_upper != 0 {
        c = toupper(c as libc::c_int) as libc::c_char;
    }
    if c as libc::c_int == '\\' as i32 {
        putchar('\\' as i32);
    }
    putchar(c as libc::c_int);
    at_start_of_line_for_dot = (c as libc::c_int == '\n' as i32) as libc::c_int;
    at_start_of_line = at_start_of_line_for_dot;
}
pub static mut nr: libc::c_int = 0 as libc::c_int;
pub static mut vars: [*mut libc::c_char; 256] = [
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut replacement: [*mut libc::c_char; 256] = [
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut start_line: [libc::c_int; 256] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut value_updated: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &mut [libc::c_char; 64],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut value_edition: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &mut [libc::c_char; 64],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut value_version: [libc::c_char; 64] = unsafe {
    *::std::mem::transmute::<
        &[u8; 64],
        &mut [libc::c_char; 64],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
unsafe extern "C" fn process_texi(mut in_0: *mut FILE) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut in_block: libc::c_int = 0 as libc::c_int;
    let mut line_no: libc::c_int = 0 as libc::c_int;
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        in_0,
    ))
        .is_null()
    {
        line_no += 1;
        line_no;
        if strncmp(
            buf.as_mut_ptr(),
            b"@include \0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut in2: *mut FILE = 0 as *mut FILE;
            let mut p: *mut libc::c_char = strchr(buf.as_mut_ptr(), '\n' as i32);
            if p.is_null() {
                continue;
            }
            *p = 0 as libc::c_int as libc::c_char;
            in2 = fopen(
                buf.as_mut_ptr().offset(9 as libc::c_int as isize),
                b"r\0" as *const u8 as *const libc::c_char,
            );
            *p = '\n' as i32 as libc::c_char;
            if !in2.is_null() {
                process_texi(in2);
                fclose(in2);
            }
        } else if strncmp(
            buf.as_mut_ptr(),
            b"@set \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut p_0: *mut libc::c_char = strchr(buf.as_mut_ptr(), '\n' as i32);
            if p_0.is_null() {
                continue;
            }
            *p_0 = 0 as libc::c_int as libc::c_char;
            if strncmp(
                buf.as_mut_ptr().offset(5 as libc::c_int as isize),
                b"UPDATED \0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                memset(
                    value_updated.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                );
                strncpy(
                    value_updated.as_mut_ptr(),
                    buf.as_mut_ptr().offset(13 as libc::c_int as isize),
                    (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            }
            if strncmp(
                buf.as_mut_ptr().offset(5 as libc::c_int as isize),
                b"EDITION \0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                memset(
                    value_updated.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                );
                strncpy(
                    value_edition.as_mut_ptr(),
                    buf.as_mut_ptr().offset(13 as libc::c_int as isize),
                    (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            }
            if strncmp(
                buf.as_mut_ptr().offset(5 as libc::c_int as isize),
                b"VERSION \0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                memset(
                    value_updated.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                );
                strncpy(
                    value_version.as_mut_ptr(),
                    buf.as_mut_ptr().offset(13 as libc::c_int as isize),
                    (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            }
            *p_0 = '\n' as i32 as libc::c_char;
        }
        if strncmp(
            buf.as_mut_ptr(),
            b"@c !BEGIN \0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut p_1: *mut libc::c_char = buf
                .as_mut_ptr()
                .offset(10 as libc::c_int as isize);
            let mut len: size_t = strlen(p_1);
            if nr == 256 as libc::c_int {
                exit(-(1 as libc::c_int));
            }
            vars[nr
                as usize] = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            replacement[nr
                as usize] = malloc(131072 as libc::c_int as libc::c_ulong)
                as *mut libc::c_char;
            start_line[nr as usize] = line_no + 1 as libc::c_int;
            strcpy(vars[nr as usize], p_1);
            p_1 = strchr(vars[nr as usize], '\n' as i32);
            if !p_1.is_null() {
                *p_1 = 0 as libc::c_int as libc::c_char;
            }
            p_1 = strchr(vars[nr as usize], ' ' as i32);
            if !p_1.is_null() {
                *p_1 = 0 as libc::c_int as libc::c_char;
            }
            in_block = 1 as libc::c_int;
        } else if strncmp(
            buf.as_mut_ptr(),
            b"@c !END\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut len_0: size_t = strlen(replacement[nr as usize]);
            if *(replacement[nr as usize])
                .offset(len_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\n' as i32
            {
                *(replacement[nr as usize])
                    .offset(
                        len_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = 0 as libc::c_int as libc::c_char;
                len_0 = len_0.wrapping_sub(1);
                len_0;
            }
            in_block = 0 as libc::c_int;
            replacement[nr
                as usize] = realloc(
                replacement[nr as usize] as *mut libc::c_void,
                len_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            nr += 1;
            nr;
        } else if in_block != 0 {
            if in_block == 1 as libc::c_int {
                strcpy(replacement[nr as usize], buf.as_mut_ptr());
                in_block = 2 as libc::c_int;
            } else {
                strcat(replacement[nr as usize], buf.as_mut_ptr());
            }
        }
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut line_no: libc::c_int = 0 as libc::c_int;
    texinfoname = *argv.offset(2 as libc::c_int as isize);
    in_0 = fopen(
        *argv.offset(2 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    process_texi(in_0);
    fclose(in_0);
    in_0 = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        in_0,
    ))
        .is_null()
    {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        p = buf.as_mut_ptr();
        while *p != 0 {
            if *p as libc::c_int != '@' as i32 {
                putchar(*p as libc::c_int);
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < nr {
                    let mut len: size_t = strlen(vars[i as usize]);
                    if strncmp(
                        p.offset(1 as libc::c_int as isize),
                        vars[i as usize],
                        len,
                    ) == 0 as libc::c_int
                        && *p
                            .offset(
                                len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '@' as i32
                    {
                        let mut stack: [libc::c_int; 16] = [0; 16];
                        let mut what: *mut libc::c_int = stack.as_mut_ptr();
                        let mut at_start_of_input_line: libc::c_int = 1 as libc::c_int;
                        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
                        *what = 0 as libc::c_int;
                        p = p
                            .offset(
                                (strlen(vars[i as usize]))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                        line_no = start_line[i as usize];
                        let mut current_block_236: u64;
                        q = replacement[i as usize];
                        while !q.is_null() && *q as libc::c_int != 0 {
                            if *q as libc::c_int != '@' as i32 {
                                let mut was_at_start_of_input_line: libc::c_int = at_start_of_input_line;
                                at_start_of_input_line = (*q as libc::c_int == '\n' as i32)
                                    as libc::c_int;
                                if at_start_of_input_line != 0 {
                                    line_no += 1;
                                    line_no;
                                }
                                if !(ignore != 0) {
                                    if in_section != 0 && *q as libc::c_int == '\n' as i32 {
                                        in_section = 0 as libc::c_int;
                                        to_upper = 0 as libc::c_int;
                                        putone('"' as i32 as libc::c_char);
                                    }
                                    if in_item != 0 && *q as libc::c_int == '\n' as i32 {
                                        in_item = 0 as libc::c_int;
                                        stop_bold();
                                    }
                                    if !(in_table != 0 && *q as libc::c_int == '\n' as i32) {
                                        if *what != 0 && *q as libc::c_int == '}' as i32 {
                                            let mut current_block_34: u64;
                                            let fresh0 = what;
                                            what = what.offset(-1);
                                            match *fresh0 {
                                                6 => {
                                                    to_upper = 0 as libc::c_int;
                                                    no_break -= 1;
                                                    no_break;
                                                    current_block_34 = 14644295595285902404;
                                                }
                                                1 => {
                                                    current_block_34 = 14644295595285902404;
                                                }
                                                2 => {
                                                    putone('"' as i32 as libc::c_char);
                                                    current_block_34 = 9441801433784995173;
                                                }
                                                3 => {
                                                    stop_I();
                                                    current_block_34 = 9441801433784995173;
                                                }
                                                4 => {
                                                    putone(']' as i32 as libc::c_char);
                                                    current_block_34 = 9441801433784995173;
                                                }
                                                5 => {
                                                    stop_I();
                                                    current_block_34 = 9441801433784995173;
                                                }
                                                7 => {
                                                    putone('\'' as i32 as libc::c_char);
                                                    current_block_34 = 9441801433784995173;
                                                }
                                                8 => {
                                                    no_break -= 1;
                                                    no_break;
                                                    current_block_34 = 9441801433784995173;
                                                }
                                                9 | _ => {
                                                    current_block_34 = 9441801433784995173;
                                                }
                                            }
                                            match current_block_34 {
                                                14644295595285902404 => {
                                                    if in_section == 0 {
                                                        stop_bold();
                                                    }
                                                }
                                                _ => {}
                                            }
                                        } else {
                                            if in_table != 0 && was_at_start_of_input_line != 0
                                                && in_item == 0
                                            {
                                                putone('\n' as i32 as libc::c_char);
                                            }
                                            putone(*q);
                                        }
                                    }
                                }
                            } else {
                                if at_start_of_input_line != 0 {
                                    if strncmp(
                                        q,
                                        b"@end ignore\0" as *const u8 as *const libc::c_char,
                                        11 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                        || strncmp(
                                            q,
                                            b"@end menu\0" as *const u8 as *const libc::c_char,
                                            9 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@end iftex\0" as *const u8 as *const libc::c_char,
                                            10 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@end tex\0" as *const u8 as *const libc::c_char,
                                            8 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@end direntry\0" as *const u8 as *const libc::c_char,
                                            13 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                    {
                                        ignore -= 1;
                                        ignore;
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@ignore\n\0" as *const u8 as *const libc::c_char,
                                        8 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                        || strncmp(
                                            q,
                                            b"@menu\n\0" as *const u8 as *const libc::c_char,
                                            6 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@iftex\n\0" as *const u8 as *const libc::c_char,
                                            7 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@tex\n\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@direntry\n\0" as *const u8 as *const libc::c_char,
                                            10 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                    {
                                        ignore += 1;
                                        ignore;
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if ignore != 0 {
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@author \0" as *const u8 as *const libc::c_char,
                                        8 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                    {
                                        printf(b".br\n\0" as *const u8 as *const libc::c_char);
                                        q = q.offset(7 as libc::c_int as isize);
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@example\n\0" as *const u8 as *const libc::c_char,
                                        9 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                        || strncmp(
                                            q,
                                            b"@smallexample\n\0" as *const u8 as *const libc::c_char,
                                            14 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                    {
                                        if at_start_of_line == 0 {
                                            putchar('\n' as i32);
                                        }
                                        printf(b".in +5\n\0" as *const u8 as *const libc::c_char);
                                        indentation += 5 as libc::c_int;
                                        printf(b".nf\n.na\n\0" as *const u8 as *const libc::c_char);
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@end example\0" as *const u8 as *const libc::c_char,
                                        12 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                        || strncmp(
                                            q,
                                            b"@end smallexample\0" as *const u8 as *const libc::c_char,
                                            17 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                    {
                                        if at_start_of_line == 0 {
                                            putchar('\n' as i32);
                                        }
                                        printf(b".in -5\n\0" as *const u8 as *const libc::c_char);
                                        indentation -= 5 as libc::c_int;
                                        printf(b".ad\n.fi\n\0" as *const u8 as *const libc::c_char);
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@noindent\0" as *const u8 as *const libc::c_char,
                                        9 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                    {
                                        if indentation != 0 {
                                            if at_start_of_line == 0 {
                                                putchar('\n' as i32);
                                            }
                                            printf(
                                                b".in -%d\n\0" as *const u8 as *const libc::c_char,
                                                indentation,
                                            );
                                            at_start_of_line_for_dot = 1 as libc::c_int;
                                            at_start_of_line = at_start_of_line_for_dot;
                                            indentation = 0 as libc::c_int;
                                        }
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@display\n\0" as *const u8 as *const libc::c_char,
                                        9 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                    {
                                        if at_start_of_line == 0 {
                                            putchar('\n' as i32);
                                        }
                                        printf(b".nf\n.na\n\0" as *const u8 as *const libc::c_char);
                                        q = q.offset(8 as libc::c_int as isize);
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@end display\0" as *const u8 as *const libc::c_char,
                                        12 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                    {
                                        if at_start_of_line == 0 {
                                            putchar('\n' as i32);
                                        }
                                        printf(b".ad\n.fi\n\0" as *const u8 as *const libc::c_char);
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@section \0" as *const u8 as *const libc::c_char,
                                        9 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                        || strncmp(
                                            q,
                                            b"@appendix \0" as *const u8 as *const libc::c_char,
                                            10 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@subsection \0" as *const u8 as *const libc::c_char,
                                            12 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        || strncmp(
                                            q,
                                            b"@subheading \0" as *const u8 as *const libc::c_char,
                                            12 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                    {
                                        if at_start_of_line == 0 {
                                            putchar('\n' as i32);
                                        }
                                        printf(b".SH \"\0" as *const u8 as *const libc::c_char);
                                        at_start_of_line_for_dot = 0 as libc::c_int;
                                        at_start_of_line = at_start_of_line_for_dot;
                                        in_section = 1 as libc::c_int;
                                        to_upper = 1 as libc::c_int;
                                        if *q.offset(2 as libc::c_int as isize) as libc::c_int
                                            == 'e' as i32
                                        {
                                            q = q.offset(8 as libc::c_int as isize);
                                        } else if *q.offset(2 as libc::c_int as isize)
                                            as libc::c_int == 'p' as i32
                                        {
                                            q = q.offset(9 as libc::c_int as isize);
                                        } else {
                                            q = q.offset(11 as libc::c_int as isize);
                                            to_upper = 0 as libc::c_int;
                                        }
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@table\0" as *const u8 as *const libc::c_char,
                                        6 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                    {
                                        in_table = 1 as libc::c_int;
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@end table\0" as *const u8 as *const libc::c_char,
                                        10 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                    {
                                        disabled = 0 as libc::c_int;
                                        in_table = 0 as libc::c_int;
                                        q = strchr(q, '\n' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        line_no += 1;
                                        line_no;
                                        current_block_236 = 7149356873433890176;
                                    } else if strncmp(
                                        q,
                                        b"@item \0" as *const u8 as *const libc::c_char,
                                        6 as libc::c_int as libc::c_ulong,
                                    ) == 0
                                        || strncmp(
                                            q,
                                            b"@itemx \0" as *const u8 as *const libc::c_char,
                                            7 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                    {
                                        disabled = 0 as libc::c_int;
                                        if in_table != 0
                                            && *q.offset(5 as libc::c_int as isize) as libc::c_int
                                                == ' ' as i32
                                        {
                                            if at_start_of_line == 0 {
                                                putchar('\n' as i32);
                                            }
                                            if in_table == 1 as libc::c_int {
                                                in_table = 2 as libc::c_int;
                                                printf(b".TP 4\n\0" as *const u8 as *const libc::c_char);
                                                at_start_of_line_for_dot = 1 as libc::c_int;
                                                at_start_of_line = at_start_of_line_for_dot;
                                            } else {
                                                printf(b".TP\n\0" as *const u8 as *const libc::c_char);
                                                at_start_of_line_for_dot = 1 as libc::c_int;
                                                at_start_of_line = at_start_of_line_for_dot;
                                            }
                                        }
                                        if in_table != 0
                                            && *q.offset(5 as libc::c_int as isize) as libc::c_int
                                                == 'x' as i32
                                        {
                                            printf(b", \0" as *const u8 as *const libc::c_char);
                                            at_start_of_line_for_dot = 0 as libc::c_int;
                                            at_start_of_line = at_start_of_line_for_dot;
                                        }
                                        in_item = 1 as libc::c_int;
                                        q = strchr(q, ' ' as i32);
                                        if q.is_null() {
                                            break;
                                        }
                                        start_bold();
                                        current_block_236 = 7149356873433890176;
                                    } else {
                                        current_block_236 = 1830138855519935310;
                                    }
                                } else if ignore != 0 {
                                    current_block_236 = 7149356873433890176;
                                } else {
                                    current_block_236 = 1830138855519935310;
                                }
                                match current_block_236 {
                                    7149356873433890176 => {}
                                    _ => {
                                        if strncmp(
                                            q,
                                            b"@code{\0" as *const u8 as *const libc::c_char,
                                            6 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                            || strncmp(
                                                q,
                                                b"@env{\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int as libc::c_ulong,
                                            ) == 0
                                            || strncmp(
                                                q,
                                                b"@command{\0" as *const u8 as *const libc::c_char,
                                                9 as libc::c_int as libc::c_ulong,
                                            ) == 0
                                        {
                                            if in_section == 0 {
                                                start_bold();
                                            }
                                            what = what.offset(1);
                                            *what = 1 as libc::c_int;
                                            if *q.offset(1 as libc::c_int as isize) as libc::c_int
                                                == 'e' as i32
                                            {
                                                q = q.offset(4 as libc::c_int as isize);
                                            } else if *q.offset(3 as libc::c_int as isize)
                                                as libc::c_int == 'd' as i32
                                            {
                                                q = q.offset(5 as libc::c_int as isize);
                                            } else {
                                                q = q.offset(8 as libc::c_int as isize);
                                            }
                                        } else if strncmp(
                                            q,
                                            b"@samp{\0" as *const u8 as *const libc::c_char,
                                            6 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                            || strncmp(
                                                q,
                                                b"@option{\0" as *const u8 as *const libc::c_char,
                                                8 as libc::c_int as libc::c_ulong,
                                            ) == 0
                                            || strncmp(
                                                q,
                                                b"@file{\0" as *const u8 as *const libc::c_char,
                                                6 as libc::c_int as libc::c_ulong,
                                            ) == 0
                                        {
                                            putone('`' as i32 as libc::c_char);
                                            what = what.offset(1);
                                            *what = 7 as libc::c_int;
                                            if *q.offset(1 as libc::c_int as isize) as libc::c_int
                                                == 'o' as i32
                                            {
                                                q = q.offset(7 as libc::c_int as isize);
                                            } else {
                                                q = q.offset(5 as libc::c_int as isize);
                                            }
                                        } else if strncmp(
                                            q,
                                            b"@cite{\0" as *const u8 as *const libc::c_char,
                                            6 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            putone('"' as i32 as libc::c_char);
                                            what = what.offset(1);
                                            *what = 2 as libc::c_int;
                                            q = q.offset(5 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@var{\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                            || strncmp(
                                                q,
                                                b"@dfn{\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int as libc::c_ulong,
                                            ) == 0
                                        {
                                            start_I();
                                            what = what.offset(1);
                                            *what = 3 as libc::c_int;
                                            q = q.offset(4 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@copyright{}\0" as *const u8 as *const libc::c_char,
                                            12 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            printf(b"(c)\0" as *const u8 as *const libc::c_char);
                                            q = q.offset(11 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@value{UPDATED}\0" as *const u8 as *const libc::c_char,
                                            15 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            printf(
                                                b"%s\0" as *const u8 as *const libc::c_char,
                                                value_updated.as_mut_ptr(),
                                            );
                                            q = q.offset(14 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@value{VERSION}\0" as *const u8 as *const libc::c_char,
                                            15 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            printf(
                                                b"%s\0" as *const u8 as *const libc::c_char,
                                                value_version.as_mut_ptr(),
                                            );
                                            q = q.offset(14 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@value{EDITION}\0" as *const u8 as *const libc::c_char,
                                            15 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            printf(
                                                b"%s\0" as *const u8 as *const libc::c_char,
                                                value_edition.as_mut_ptr(),
                                            );
                                            q = q.offset(14 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@refill\0" as *const u8 as *const libc::c_char,
                                            7 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            q = q.offset(6 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@footnote{\0" as *const u8 as *const libc::c_char,
                                            10 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            if at_start_of_line == 0 {
                                                putone(' ' as i32 as libc::c_char);
                                            }
                                            putone('[' as i32 as libc::c_char);
                                            what = what.offset(1);
                                            *what = 4 as libc::c_int;
                                            q = q.offset(9 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@emph{\0" as *const u8 as *const libc::c_char,
                                            6 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            start_I();
                                            what = what.offset(1);
                                            *what = 5 as libc::c_int;
                                            q = q.offset(5 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@xref{\0" as *const u8 as *const libc::c_char,
                                            6 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                            || strncmp(
                                                q,
                                                b"@pxref{\0" as *const u8 as *const libc::c_char,
                                                7 as libc::c_int as libc::c_ulong,
                                            ) == 0
                                        {
                                            if disabled == 0 {
                                                printf(b"See \0" as *const u8 as *const libc::c_char);
                                                at_start_of_line_for_dot = 0 as libc::c_int;
                                                at_start_of_line = at_start_of_line_for_dot;
                                                start_bold();
                                                to_upper = 1 as libc::c_int;
                                                no_break += 1;
                                                no_break;
                                                what = what.offset(1);
                                                *what = 6 as libc::c_int;
                                            }
                                            if *q.offset(1 as libc::c_int as isize) as libc::c_int
                                                == 'p' as i32
                                            {
                                                q = q.offset(6 as libc::c_int as isize);
                                            } else {
                                                q = q.offset(5 as libc::c_int as isize);
                                            }
                                        } else if strncmp(
                                            q,
                                            b"@w{\0" as *const u8 as *const libc::c_char,
                                            3 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            no_break += 1;
                                            no_break;
                                            what = what.offset(1);
                                            *what = 8 as libc::c_int;
                                            q = q.offset(2 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@email{\0" as *const u8 as *const libc::c_char,
                                            7 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            what = what.offset(1);
                                            *what = 9 as libc::c_int;
                                            q = q.offset(6 as libc::c_int as isize);
                                        } else if strncmp(
                                            q,
                                            b"@{\0" as *const u8 as *const libc::c_char,
                                            2 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            putone('{' as i32 as libc::c_char);
                                            q = q.offset(1);
                                            q;
                                        } else if strncmp(
                                            q,
                                            b"@}\0" as *const u8 as *const libc::c_char,
                                            2 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            putone('}' as i32 as libc::c_char);
                                            q = q.offset(1);
                                            q;
                                        } else if strncmp(
                                            q,
                                            b"@*\0" as *const u8 as *const libc::c_char,
                                            2 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            if at_start_of_line_for_dot == 0 {
                                                putchar('\n' as i32);
                                            }
                                            printf(b".br\n\0" as *const u8 as *const libc::c_char);
                                            at_start_of_line_for_dot = 1 as libc::c_int;
                                            at_start_of_line = at_start_of_line_for_dot;
                                            q = q.offset(1);
                                            q;
                                        } else if strncmp(
                                            q,
                                            b"@@\0" as *const u8 as *const libc::c_char,
                                            2 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            putone('@' as i32 as libc::c_char);
                                            q = q.offset(1);
                                            q;
                                        } else {
                                            let mut s: *mut libc::c_char = strchr(q, ' ' as i32);
                                            let mut a: *mut libc::c_char = strchr(q, '{' as i32);
                                            if s.is_null() || a.is_null() || s < a {
                                                let mut q2: *mut libc::c_char = strchr(q, '\n' as i32);
                                                if strncmp(
                                                    q,
                                                    b"@ifinfo\0" as *const u8 as *const libc::c_char,
                                                    7 as libc::c_int as libc::c_ulong,
                                                ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@end ifinfo\0" as *const u8 as *const libc::c_char,
                                                        11 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@cindex\0" as *const u8 as *const libc::c_char,
                                                        7 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@kindex\0" as *const u8 as *const libc::c_char,
                                                        7 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@refill\0" as *const u8 as *const libc::c_char,
                                                        7 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@include\0" as *const u8 as *const libc::c_char,
                                                        8 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@set\0" as *const u8 as *const libc::c_char,
                                                        4 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@group\0" as *const u8 as *const libc::c_char,
                                                        6 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@end group\0" as *const u8 as *const libc::c_char,
                                                        10 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@node\0" as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@comment\0" as *const u8 as *const libc::c_char,
                                                        8 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@c\n\0" as *const u8 as *const libc::c_char,
                                                        3 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@c \0" as *const u8 as *const libc::c_char,
                                                        3 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@need\0" as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                    && strncmp(
                                                        q,
                                                        b"@dircategory\0" as *const u8 as *const libc::c_char,
                                                        12 as libc::c_int as libc::c_ulong,
                                                    ) != 0
                                                {
                                                    if !a.is_null() {
                                                        *a = 0 as libc::c_int as libc::c_char;
                                                    }
                                                    if !q2.is_null() {
                                                        *q2 = 0 as libc::c_int as libc::c_char;
                                                    }
                                                    fprintf(
                                                        stderr,
                                                        b"%s: Unknown texinfo command %s\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        where_0(line_no),
                                                        q,
                                                    );
                                                    if !a.is_null() {
                                                        *a = '{' as i32 as libc::c_char;
                                                    }
                                                }
                                                q = q2;
                                                if q.is_null() {
                                                    break;
                                                }
                                                line_no += 1;
                                                line_no;
                                            }
                                        }
                                    }
                                }
                            }
                            q = q.offset(1);
                            q;
                        }
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                if i == nr {
                    putone('@' as i32 as libc::c_char);
                }
            }
            p = p.offset(1);
            p;
        }
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
