use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
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
pub type mode_t = __mode_t;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn esysdep_fopen(
    mut zfile: *const libc::c_char,
    mut fpublic: boolean,
    mut fappend: boolean,
    mut fmkdirs: boolean,
) -> *mut FILE {
    let mut imode: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut e: *mut FILE = 0 as *mut FILE;
    if fpublic != 0 {
        imode = 0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int;
    } else {
        imode = 0o400 as libc::c_int | 0o200 as libc::c_int;
    }
    if fappend == 0 {
        o = creat(zfile as *mut libc::c_char, imode as mode_t);
    } else {
        o = open(
            zfile as *mut libc::c_char,
            0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int
                | 0o400 as libc::c_int,
            imode,
        );
    }
    if o < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int && fmkdirs != 0 {
            if fsysdep_make_dirs(zfile, fpublic) == 0 {
                return 0 as *mut FILE;
            }
            if fappend == 0 {
                o = creat(zfile as *mut libc::c_char, imode as mode_t);
            } else {
                o = open(
                    zfile as *mut libc::c_char,
                    0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int
                        | 0o400 as libc::c_int,
                    imode,
                );
            }
        }
        if o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"open (%s): %s\0" as *const u8 as *const libc::c_char,
                zfile,
                strerror(*__errno_location()),
            );
            return 0 as *mut FILE;
        }
    }
    if fcntl(
        o,
        2 as libc::c_int,
        fcntl(o, 1 as libc::c_int, 0 as libc::c_int) | 1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        ulog(
            LOG_ERROR,
            b"fcntl (%s, FD_CLOEXEC): %s\0" as *const u8 as *const libc::c_char,
            zfile,
            strerror(*__errno_location()),
        );
        close(o);
        return 0 as *mut FILE;
    }
    if fappend != 0 {
        e = fdopen(o, b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else {
        e = fdopen(o, b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    if e.is_null() {
        ulog(
            LOG_ERROR,
            b"fdopen: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(o);
    }
    return e;
}
