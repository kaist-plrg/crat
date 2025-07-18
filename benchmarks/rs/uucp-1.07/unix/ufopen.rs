use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn fsuser_perms(_: *mut uid_t, _: *mut gid_t) -> boolean;
    fn fsuucp_perms(_: libc::c_long, _: libc::c_long) -> boolean;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
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
pub type uid_t = __uid_t;
pub type gid_t = __gid_t;
pub type boolean = libc::c_int;
pub type openfile_t = *mut FILE;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn esysdep_user_fopen(
    mut zfile: *const libc::c_char,
    mut frd: boolean,
    mut fbinary: boolean,
) -> openfile_t {
    let mut ieuid: uid_t = 0;
    let mut iegid: gid_t = 0;
    let mut e: openfile_t = 0 as *mut FILE;
    let mut zerr: *const libc::c_char = 0 as *const libc::c_char;
    let mut o: libc::c_int = 0 as libc::c_int;
    if fsuser_perms(&mut ieuid, &mut iegid) == 0 {
        return 0 as *mut libc::c_void as *mut FILE;
    }
    zerr = 0 as *const libc::c_char;
    e = fopen(
        zfile,
        if frd != 0 {
            b"r\0" as *const u8 as *const libc::c_char
        } else {
            b"w\0" as *const u8 as *const libc::c_char
        },
    );
    if e.is_null() {
        zerr = b"fopen\0" as *const u8 as *const libc::c_char;
    } else {
        o = fileno(e);
    }
    if fsuucp_perms(ieuid as libc::c_long, iegid as libc::c_long) == 0 {
        if !e.is_null() {
            fclose(e);
        }
        return 0 as *mut libc::c_void as *mut FILE;
    }
    if !zerr.is_null() {
        ulog(
            LOG_ERROR,
            b"%s (%s): %s\0" as *const u8 as *const libc::c_char,
            zerr,
            zfile,
            strerror(*__errno_location()),
        );
        return 0 as *mut libc::c_void as *mut FILE;
    }
    if fcntl(
        o,
        2 as libc::c_int,
        fcntl(o, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"fcntl (FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        fclose(e);
        return 0 as *mut libc::c_void as *mut FILE;
    }
    return e;
}
