use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub type METHOD = libc::c_uint;
pub const __OUT: METHOD = 2;
pub const __LOG: METHOD = 1;
pub unsafe extern "C" fn OPENLOG(mut program: *mut libc::c_char) {
    openlog(program, 0x1 as libc::c_int, (3 as libc::c_int) << 3 as libc::c_int);
}
pub unsafe extern "C" fn CLOSELOG() {
    closelog();
}
unsafe extern "C" fn __message(
    mut M: METHOD,
    mut L: LEVEL,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut buf: [libc::c_char; 40000] = [0; 40000];
    let mut msg: [libc::c_char; 41024] = [0; 41024];
    let mut level: LEVEL = WARNING;
    let mut pmode: [libc::c_char; 64] = [0; 64];
    let mut lmode: [libc::c_char; 64] = [0; 64];
    memset(
        lmode.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        64 as libc::c_int as libc::c_ulong,
    );
    memset(
        pmode.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        64 as libc::c_int as libc::c_ulong,
    );
    vsprintf(buf.as_mut_ptr(), fmt, ap.as_va_list());
    if *__errno_location() == 0 as libc::c_int
        || *__errno_location() == 38 as libc::c_int
        || L as libc::c_uint == DEBUG as libc::c_int as libc::c_uint
    {
        snprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 41024]>() as libc::c_ulong,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
    } else {
        snprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 41024]>() as libc::c_ulong,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
            strerror(*__errno_location()),
        );
    }
    match L as libc::c_uint {
        0 => {
            sprintf(
                pmode.as_mut_ptr(),
                b"[%c[%d;%dmdebug%c[%dm]\0" as *const u8 as *const libc::c_char,
                0x1b as libc::c_int,
                1 as libc::c_int,
                4 as libc::c_int + 30 as libc::c_int,
                0x1b as libc::c_int,
                0 as libc::c_int,
            );
            strcpy(lmode.as_mut_ptr(), b"[debug]\0" as *const u8 as *const libc::c_char);
            level = 4 as LEVEL;
        }
        1 => {
            sprintf(
                pmode.as_mut_ptr(),
                b"[%c[%d;%dmalert%c[%dm]\0" as *const u8 as *const libc::c_char,
                0x1b as libc::c_int,
                1 as libc::c_int,
                2 as libc::c_int + 30 as libc::c_int,
                0x1b as libc::c_int,
                0 as libc::c_int,
            );
            strcpy(
                lmode.as_mut_ptr(),
                b"[alert] \0" as *const u8 as *const libc::c_char,
            );
            level = 4 as LEVEL;
        }
        2 => {
            sprintf(
                pmode.as_mut_ptr(),
                b"[%c[%d;%dmerror%c[%dm]\0" as *const u8 as *const libc::c_char,
                0x1b as libc::c_int,
                1 as libc::c_int,
                3 as libc::c_int + 30 as libc::c_int,
                0x1b as libc::c_int,
                0 as libc::c_int,
            );
            strcpy(lmode.as_mut_ptr(), b"[error]\0" as *const u8 as *const libc::c_char);
            level = FATAL;
        }
        3 => {
            sprintf(
                pmode.as_mut_ptr(),
                b"[%c[%d;%dmfatal%c[%dm]\0" as *const u8 as *const libc::c_char,
                0x1b as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int + 30 as libc::c_int,
                0x1b as libc::c_int,
                0 as libc::c_int,
            );
            strcpy(lmode.as_mut_ptr(), b"[fatal]\0" as *const u8 as *const libc::c_char);
            level = ERROR;
        }
        _ => {}
    }
    if M as libc::c_uint == __LOG as libc::c_int as libc::c_uint {
        syslog(
            level as libc::c_int,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            lmode.as_mut_ptr(),
            msg.as_mut_ptr(),
        );
    } else {
        fflush(stdout);
        fprintf(
            stderr,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            pmode.as_mut_ptr(),
            msg.as_mut_ptr(),
        );
    }
    if L as libc::c_uint == FATAL as libc::c_int as libc::c_uint {
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn SYSLOG(
    mut L: LEVEL,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    __message(__LOG, L, fmt, ap.as_va_list());
}
pub unsafe extern "C" fn NOTIFY(
    mut L: LEVEL,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    __message(__OUT, L, fmt, ap.as_va_list());
}
pub unsafe extern "C" fn __display(
    mut color: libc::c_int,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut buf: [libc::c_char; 40000] = [0; 40000];
    let mut msg: [libc::c_char; 41024] = [0; 41024];
    vsprintf(buf.as_mut_ptr(), fmt, ap.as_va_list());
    if color == -(1 as libc::c_int) {
        snprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 41024]>() as libc::c_ulong,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
    } else {
        snprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 41024]>() as libc::c_ulong,
            b"%c[%d;%dm%s%c[%dm\n\0" as *const u8 as *const libc::c_char,
            0x1b as libc::c_int,
            0 as libc::c_int,
            color + 30 as libc::c_int,
            buf.as_mut_ptr(),
            0x1b as libc::c_int,
            0 as libc::c_int,
        );
    }
    printf(b"%s\0" as *const u8 as *const libc::c_char, msg.as_mut_ptr());
}
pub unsafe extern "C" fn DISPLAY(
    mut color: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    __display(color, fmt, ap.as_va_list());
}
