use ::libc;
extern "C" {
    pub type AUTH_T;
    pub type ARRAY_T;
    pub type COOKIES_T;
    pub type DATA_T;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    static mut my: CONFIG;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn data_get_total(this: DATA) -> libc::c_float;
    fn data_get_megabytes(this: DATA) -> libc::c_float;
    fn data_get_elapsed(this: DATA) -> libc::c_float;
    fn data_get_count(this: DATA) -> libc::c_uint;
    fn data_get_code(this: DATA) -> libc::c_uint;
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
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
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type METHOD = libc::c_uint;
pub const PATCH: METHOD = 9;
pub const CONNECT: METHOD = 8;
pub const OPTIONS: METHOD = 7;
pub const TRACE: METHOD = 6;
pub const DELETE: METHOD = 5;
pub const PUT: METHOD = 4;
pub const POST: METHOD = 3;
pub const GET: METHOD = 2;
pub const HEAD: METHOD = 1;
pub const NOMETHOD: METHOD = 0;
pub type AUTH = *mut AUTH_T;
pub type ARRAY = *mut ARRAY_T;
pub type COOKIES = *mut COOKIES_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONFIG {
    pub logging: BOOLEAN,
    pub shlog: BOOLEAN,
    pub limit: libc::c_int,
    pub url: *mut libc::c_char,
    pub logfile: [libc::c_char; 128],
    pub verbose: BOOLEAN,
    pub quiet: BOOLEAN,
    pub parser: BOOLEAN,
    pub csv: BOOLEAN,
    pub fullurl: BOOLEAN,
    pub display: BOOLEAN,
    pub config: BOOLEAN,
    pub color: BOOLEAN,
    pub cusers: libc::c_int,
    pub delay: libc::c_float,
    pub timeout: libc::c_int,
    pub bench: BOOLEAN,
    pub internet: BOOLEAN,
    pub timestamp: BOOLEAN,
    pub time: libc::c_int,
    pub secs: libc::c_int,
    pub reps: libc::c_int,
    pub file: [libc::c_char; 128],
    pub length: libc::c_int,
    pub nomap: *mut LINES,
    pub debug: BOOLEAN,
    pub chunked: BOOLEAN,
    pub unique: BOOLEAN,
    pub get: BOOLEAN,
    pub print: BOOLEAN,
    pub mark: BOOLEAN,
    pub markstr: *mut libc::c_char,
    pub protocol: libc::c_int,
    pub cookies: COOKIES,
    pub uagent: [libc::c_char; 256],
    pub encoding: [libc::c_char; 256],
    pub conttype: [libc::c_char; 256],
    pub bids: libc::c_int,
    pub auth: AUTH,
    pub keepalive: BOOLEAN,
    pub signaled: libc::c_int,
    pub extra: [libc::c_char; 2048],
    pub login: BOOLEAN,
    pub loginurl: *mut libc::c_char,
    pub lurl: ARRAY,
    pub failures: libc::c_int,
    pub failed: libc::c_int,
    pub escape: BOOLEAN,
    pub expire: BOOLEAN,
    pub follow: BOOLEAN,
    pub zero_ok: BOOLEAN,
    pub spinner: BOOLEAN,
    pub cache: BOOLEAN,
    pub rc: [libc::c_char; 256],
    pub ssl_timeout: libc::c_int,
    pub ssl_cert: *mut libc::c_char,
    pub ssl_key: *mut libc::c_char,
    pub ssl_ciphers: *mut libc::c_char,
    pub method: METHOD,
    pub json_output: BOOLEAN,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
}
pub type DATA = *mut DATA_T;
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub unsafe extern "C" fn log_transaction(mut D: DATA) {
    write_to_log(
        data_get_count(D) as libc::c_int,
        data_get_elapsed(D),
        data_get_megabytes(D) as libc::c_int,
        data_get_total(D),
        data_get_code(D) as libc::c_int,
        my.failed,
    );
}
pub unsafe extern "C" fn write_to_log(
    mut count: libc::c_int,
    mut elapsed: libc::c_float,
    mut bytes: libc::c_int,
    mut ttime: libc::c_float,
    mut code: libc::c_int,
    mut failed: libc::c_int,
) {
    let mut fd: libc::c_int = 0;
    let mut entry: [libc::c_char; 512] = [0; 512];
    let mut keepsake: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tmp: *mut tm = 0 as *mut tm;
    let mut now: time_t = 0;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut date: [libc::c_char; 65] = [0; 65];
    now = time(0 as *mut time_t);
    tmp = localtime_r(&mut now, &mut keepsake);
    setlocale(2 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    len = strftime(
        date.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
        b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
        tmp,
    );
    if my.shlog as u64 != 0 {
        printf(
            b"LOG FILE: %s\n\0" as *const u8 as *const libc::c_char,
            (my.logfile).as_mut_ptr(),
        );
        printf(
            b"You can disable this log file notification by editing\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"%s/.siege/siege.conf \0" as *const u8 as *const libc::c_char,
            getenv(b"HOME\0" as *const u8 as *const libc::c_char),
        );
        puts(
            b"and changing 'show-logfile' to false.\0" as *const u8
                as *const libc::c_char,
        );
    }
    if file_exists((my.logfile).as_mut_ptr()) as u64 == 0 {
        if create_logfile((my.logfile).as_mut_ptr()) as u64 == 0 {
            NOTIFY(
                ERROR,
                b"unable to create log file: %s\0" as *const u8 as *const libc::c_char,
                (my.logfile).as_mut_ptr(),
            );
            return;
        }
    }
    snprintf(
        entry.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"%s,%7d,%11.2f,%12u,%11.2f,%12.2f,%12.2f,%12.2f,%8d,%8d\n\0" as *const u8
            as *const libc::c_char,
        date.as_mut_ptr(),
        count,
        elapsed as libc::c_double,
        bytes,
        (ttime / count as libc::c_float) as libc::c_double,
        (count as libc::c_float / elapsed) as libc::c_double,
        (bytes as libc::c_float / elapsed) as libc::c_double,
        (ttime / elapsed) as libc::c_double,
        code,
        failed,
    );
    fd = open(
        (my.logfile).as_mut_ptr(),
        0o1 as libc::c_int | 0o2000 as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"Unable to open file: %s\0" as *const u8 as *const libc::c_char,
            (my.logfile).as_mut_ptr(),
        );
        return;
    }
    len = write(
        fd,
        entry.as_mut_ptr() as *const libc::c_void,
        strlen(entry.as_mut_ptr()),
    ) as size_t;
    if len == -(1 as libc::c_int) as libc::c_uint as libc::c_ulong {
        match *__errno_location() {
            9 => {
                NOTIFY(
                    ERROR,
                    b"Unable to write to log file (bad file descriptor): %s\0"
                        as *const u8 as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
            4 => {
                NOTIFY(
                    ERROR,
                    b"Unable to write to log file (system interrupt): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
            _ => {
                NOTIFY(
                    ERROR,
                    b"Unable to write to log file (unknown error): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
        }
    }
    close(fd);
}
pub unsafe extern "C" fn mark_log_file(mut message: *mut libc::c_char) {
    let mut fd: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut entry: [libc::c_char; 512] = [0; 512];
    if file_exists((my.logfile).as_mut_ptr()) as u64 == 0 {
        if create_logfile((my.logfile).as_mut_ptr()) as u64 == 0 {
            NOTIFY(
                ERROR,
                b"unable to create log file: %s\0" as *const u8 as *const libc::c_char,
                (my.logfile).as_mut_ptr(),
            );
            return;
        }
    }
    snprintf(
        entry.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"**** %s ****\n\0" as *const u8 as *const libc::c_char,
        message,
    );
    fd = open(
        (my.logfile).as_mut_ptr(),
        0o1 as libc::c_int | 0o2000 as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"Unable to write to file: %s\0" as *const u8 as *const libc::c_char,
            (my.logfile).as_mut_ptr(),
        );
    }
    len = write(
        fd,
        entry.as_mut_ptr() as *const libc::c_void,
        strlen(entry.as_mut_ptr()),
    ) as size_t;
    if len == -(1 as libc::c_int) as libc::c_uint as libc::c_ulong {
        match *__errno_location() {
            9 => {
                NOTIFY(
                    ERROR,
                    b"Unable to mark log file (bad file descriptor): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
            4 => {
                NOTIFY(
                    ERROR,
                    b"Unable to mark log file (system interrupt): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
            _ => {
                NOTIFY(
                    ERROR,
                    b"Unable to mark log file (unknown error): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
        }
    }
    close(fd);
}
pub unsafe extern "C" fn file_exists(mut file: *mut libc::c_char) -> BOOLEAN {
    let mut fd: libc::c_int = 0;
    fd = open(file, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        close(fd);
        return boolean_false;
    } else {
        close(fd);
        return boolean_true;
    };
}
pub unsafe extern "C" fn create_logfile(mut file: *const libc::c_char) -> BOOLEAN {
    let mut fd: libc::c_int = 0;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut ret: BOOLEAN = boolean_true;
    let mut head: *mut libc::c_char = b"      Date & Time,  Trans,  Elap Time,  Data Trans,  Resp Time,  Trans Rate,  Throughput,  Concurrent,    OKAY,   Failed\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    fd = open(file, 0o100 as libc::c_int | 0o1 as libc::c_int, 0o644 as libc::c_int);
    if fd < 0 as libc::c_int {
        return boolean_false;
    }
    len = write(fd, head as *const libc::c_void, strlen(head)) as size_t;
    if len == -(1 as libc::c_int) as libc::c_uint as libc::c_ulong {
        ret = boolean_false;
        match *__errno_location() {
            9 => {
                NOTIFY(
                    ERROR,
                    b"Unable to create log file (bad file descriptor): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
            4 => {
                NOTIFY(
                    ERROR,
                    b"Unable to create log file (system interrupt): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
            _ => {
                NOTIFY(
                    ERROR,
                    b"Unable to create log file (unknown error): %s\0" as *const u8
                        as *const libc::c_char,
                    (my.logfile).as_mut_ptr(),
                );
            }
        }
    }
    close(fd);
    return ret;
}
