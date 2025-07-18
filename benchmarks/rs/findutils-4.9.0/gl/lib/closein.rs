use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: libc::c_int) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn close_stream(stream: *mut FILE) -> libc::c_int;
    fn close_stdout();
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn freadahead(stream: *mut FILE) -> size_t;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
}
pub type FILE = _IO_FILE;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
static mut file_name: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn close_stdin_set_file_name(mut file: *const libc::c_char) {
    file_name = file;
}
pub unsafe extern "C" fn close_stdin() {
    let mut fail: bool = 0 as libc::c_int != 0;
    if freadahead(stdin) > 0 as libc::c_int as libc::c_ulong {
        if rpl_fseeko(stdin, 0 as libc::c_int as off_t, 1 as libc::c_int)
            == 0 as libc::c_int && rpl_fflush(stdin) != 0 as libc::c_int
        {
            fail = 1 as libc::c_int != 0;
        }
    }
    if close_stream(stdin) != 0 as libc::c_int {
        fail = 1 as libc::c_int != 0;
    }
    if fail {
        let mut close_error: *const libc::c_char = dcgettext(
            0 as *const libc::c_char,
            b"error closing file\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        if !file_name.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                quotearg_colon(file_name),
                close_error,
            );
        } else {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                close_error,
            );
        }
    }
    close_stdout();
    if fail {
        _exit(exit_failure);
    }
}
