use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
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
pub type openfile_t = *mut FILE;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn esysdep_truncate(
    mut e: openfile_t,
    mut zname: *const libc::c_char,
) -> openfile_t {
    let mut o: libc::c_int = 0;
    let mut itrunc: libc::c_int = 0;
    if !(fseek(e, 0 as libc::c_int as libc::c_long, 0 as libc::c_int)
        == 0 as libc::c_int)
    {
        ulog(
            LOG_ERROR,
            b"rewind: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fclose(e);
        remove(zname);
        return 0 as *mut libc::c_void as *mut FILE;
    }
    o = fileno(e);
    itrunc = ftruncate(o, 0 as libc::c_int as __off_t);
    if itrunc != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"ftruncate: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        ulog(
            LOG_ERROR,
            b"ltrunc: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fclose(e);
        remove(zname);
        return 0 as *mut libc::c_void as *mut FILE;
    }
    return e;
}
