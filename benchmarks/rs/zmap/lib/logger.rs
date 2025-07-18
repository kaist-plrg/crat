use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn vsyslog(__pri: libc::c_int, __fmt: *const libc::c_char, __ap: ::std::ffi::VaList);
    fn floor(_: libc::c_double) -> libc::c_double;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn lock_file(f: *mut FILE) -> libc::c_int;
    fn unlock_file(f: *mut FILE) -> libc::c_int;
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
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type LogLevel = libc::c_uint;
pub const ZNUM_LOGLEVELS: LogLevel = 6;
pub const ZLOG_TRACE: LogLevel = 5;
pub const ZLOG_DEBUG: LogLevel = 4;
pub const ZLOG_INFO: LogLevel = 3;
pub const ZLOG_WARN: LogLevel = 2;
pub const ZLOG_ERROR: LogLevel = 1;
pub const ZLOG_FATAL: LogLevel = 0;
static mut mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut log_output_level: LogLevel = ZLOG_INFO;
static mut log_output_stream: *mut FILE = 0 as *const FILE as *mut FILE;
static mut color: libc::c_int = 0 as libc::c_int;
static mut log_to_syslog: libc::c_int = 0 as libc::c_int;
static mut log_level_name: [*const libc::c_char; 6] = [
    b"FATAL\0" as *const u8 as *const libc::c_char,
    b"ERROR\0" as *const u8 as *const libc::c_char,
    b"WARN\0" as *const u8 as *const libc::c_char,
    b"INFO\0" as *const u8 as *const libc::c_char,
    b"DEBUG\0" as *const u8 as *const libc::c_char,
    b"TRACE\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn color_for_level(mut level: LogLevel) -> *const libc::c_char {
    match level as libc::c_uint {
        0 => return b"\x1B[31m\0" as *const u8 as *const libc::c_char,
        1 => return b"\x1B[35m\0" as *const u8 as *const libc::c_char,
        2 => return b"\x1B[33m\0" as *const u8 as *const libc::c_char,
        3 => return b"\x1B[32m\0" as *const u8 as *const libc::c_char,
        4 => return b"\x1B[34m\0" as *const u8 as *const libc::c_char,
        5 => return b"\x1B[0m\0" as *const u8 as *const libc::c_char,
        _ => return b"\x1B[0m\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn LogLogVA(
    mut level: LogLevel,
    mut loggerName: *const libc::c_char,
    mut logMessage: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> libc::c_int {
    if level as libc::c_uint <= log_output_level as libc::c_uint {
        if log_output_stream.is_null() {
            log_output_stream = stderr;
        }
        if log_output_stream == stdout || log_output_stream == stderr {
            lock_file(log_output_stream);
        } else {
            pthread_mutex_lock(&mut mutex);
        }
        if color != 0 {
            if color != 0 {
                fprintf(
                    log_output_stream,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    color_for_level(level),
                );
            }
        }
        let mut levelName: *const libc::c_char = log_level_name[level as usize];
        let mut now_0: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut timestamp: [libc::c_char; 256] = [0; 256];
        gettimeofday(&mut now_0, 0 as *mut libc::c_void);
        let mut sec: time_t = now_0.tv_sec;
        let mut ptm: *mut tm = localtime(&mut sec);
        strftime(
            timestamp.as_mut_ptr(),
            20 as libc::c_int as size_t,
            b"%b %d %H:%M:%S\0" as *const u8 as *const libc::c_char,
            ptm,
        );
        fprintf(
            log_output_stream,
            b"%s.%03ld [%s] \0" as *const u8 as *const libc::c_char,
            timestamp.as_mut_ptr(),
            now_0.tv_usec / 1000 as libc::c_int as libc::c_long,
            levelName,
        );
        if !loggerName.is_null() {
            fprintf(
                log_output_stream,
                b"%s: \0" as *const u8 as *const libc::c_char,
                loggerName,
            );
        }
        if !logMessage.is_null() {
            vfprintf(log_output_stream, logMessage, args.as_va_list());
        }
        if !loggerName.is_null() || !logMessage.is_null() {
            fputs(b"\n\0" as *const u8 as *const libc::c_char, log_output_stream);
        }
        if color != 0 {
            if color != 0 {
                fprintf(
                    log_output_stream,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b"\x1B[0m\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        fflush(log_output_stream);
        if log_output_stream == stdout || log_output_stream == stderr {
            unlock_file(log_output_stream);
        } else {
            pthread_mutex_unlock(&mut mutex);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn log_fatal(
    mut name: *const libc::c_char,
    mut message: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut va: ::std::ffi::VaListImpl;
    va = args.clone();
    LogLogVA(ZLOG_FATAL, name, message, va.as_va_list());
    if log_to_syslog != 0 {
        va = args.clone();
        vsyslog(
            (1 as libc::c_int) << 3 as libc::c_int | 2 as libc::c_int,
            message,
            va.as_va_list(),
        );
    }
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn log_error(
    mut name: *const libc::c_char,
    mut message: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut va: ::std::ffi::VaListImpl;
    va = args.clone();
    let mut ret: libc::c_int = LogLogVA(ZLOG_ERROR, name, message, va.as_va_list());
    if log_to_syslog != 0 {
        va = args.clone();
        vsyslog(
            (1 as libc::c_int) << 3 as libc::c_int | 3 as libc::c_int,
            message,
            va.as_va_list(),
        );
    }
    return ret;
}
pub unsafe extern "C" fn log_warn(
    mut name: *const libc::c_char,
    mut message: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut va: ::std::ffi::VaListImpl;
    va = args.clone();
    let mut ret: libc::c_int = LogLogVA(ZLOG_WARN, name, message, va.as_va_list());
    if log_to_syslog != 0 {
        va = args.clone();
        vsyslog(
            (1 as libc::c_int) << 3 as libc::c_int | 4 as libc::c_int,
            message,
            va.as_va_list(),
        );
    }
    return ret;
}
pub unsafe extern "C" fn log_info(
    mut name: *const libc::c_char,
    mut message: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut va: ::std::ffi::VaListImpl;
    va = args.clone();
    let mut ret: libc::c_int = LogLogVA(ZLOG_INFO, name, message, va.as_va_list());
    let mut prefixed: *mut libc::c_char = xmalloc(
        (strlen(name))
            .wrapping_add(strlen(message))
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(prefixed, name);
    strcat(prefixed, b": \0" as *const u8 as *const libc::c_char);
    strcat(prefixed, message);
    if log_to_syslog != 0 {
        va = args.clone();
        vsyslog(
            (1 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
            prefixed,
            va.as_va_list(),
        );
    }
    free(prefixed as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn log_debug(
    mut name: *const libc::c_char,
    mut message: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut va: ::std::ffi::VaListImpl;
    va = args.clone();
    let mut ret: libc::c_int = LogLogVA(ZLOG_DEBUG, name, message, va.as_va_list());
    let mut prefixed: *mut libc::c_char = xmalloc(
        (strlen(name))
            .wrapping_add(strlen(message))
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(prefixed, name);
    strcat(prefixed, b": \0" as *const u8 as *const libc::c_char);
    strcat(prefixed, message);
    if log_to_syslog != 0 {
        va = args.clone();
        vsyslog(
            (1 as libc::c_int) << 3 as libc::c_int | 7 as libc::c_int,
            prefixed,
            va.as_va_list(),
        );
    }
    free(prefixed as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn log_init(
    mut stream: *mut FILE,
    mut level: LogLevel,
    mut syslog_enabled: libc::c_int,
    mut appname: *const libc::c_char,
) -> libc::c_int {
    log_output_stream = stream;
    log_output_level = level;
    if syslog_enabled != 0 {
        log_to_syslog = 1 as libc::c_int;
        openlog(appname, 0 as libc::c_int, (1 as libc::c_int) << 3 as libc::c_int);
    }
    if isatty(fileno(log_output_stream)) != 0 {
        color = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_and_log_file_error(
    mut file: *mut FILE,
    mut name: *const libc::c_char,
) {
    if ferror(file) != 0 {
        log_fatal(
            name,
            b"unable to write to file\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn now() -> libc::c_double {
    let mut now_0: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut now_0, 0 as *mut libc::c_void);
    return now_0.tv_sec as libc::c_double
        + now_0.tv_usec as libc::c_double / 1000000.0f64;
}
pub unsafe extern "C" fn dstrftime(
    mut buf: *mut libc::c_char,
    mut maxsize: size_t,
    mut format: *const libc::c_char,
    mut tm: libc::c_double,
) -> size_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tm_floor: libc::c_double = 0.;
    tm_floor = floor(tm);
    tv.tv_sec = tm_floor as libc::c_long;
    tv
        .tv_usec = (tm - floor(tm)) as libc::c_long
        * 1000000 as libc::c_int as libc::c_long;
    return strftime(
        buf,
        maxsize,
        format,
        localtime(&mut tv as *mut timeval as *const time_t),
    );
}
