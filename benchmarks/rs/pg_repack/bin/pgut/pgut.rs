use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pg_conn;
    pub type pg_result;
    pub type pg_cancel;
    fn simple_prompt(
        prompt: *const libc::c_char,
        destination: *mut libc::c_char,
        destlen: size_t,
        echo: bool,
    );
    fn pg_strerror(errnum: libc::c_int) -> *mut libc::c_char;
    fn pg_fprintf(stream: *mut FILE, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn pg_snprintf(
        str: *mut libc::c_char,
        count: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pg_vsnprintf(
        str: *mut libc::c_char,
        count: size_t,
        fmt: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn pg_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn pg_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn set_pglocale_pgservice(argv0: *const libc::c_char, app: *const libc::c_char);
    fn get_parent_directory(path: *mut libc::c_char);
    fn get_progname(argv0: *const libc::c_char) -> *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn abort() -> !;
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn pqsignal(signo: libc::c_int, func: pqsigfunc) -> pqsigfunc;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn mktime(__tp: *mut tm) -> time_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn PQconnectdb(conninfo: *const libc::c_char) -> *mut PGconn;
    fn PQfinish(conn: *mut PGconn);
    fn PQgetCancel(conn: *mut PGconn) -> *mut PGcancel;
    fn PQfreeCancel(cancel: *mut PGcancel);
    fn PQcancel(
        cancel: *mut PGcancel,
        errbuf: *mut libc::c_char,
        errbufsize: libc::c_int,
    ) -> libc::c_int;
    fn PQstatus(conn: *const PGconn) -> ConnStatusType;
    fn PQtransactionStatus(conn: *const PGconn) -> PGTransactionStatusType;
    fn PQerrorMessage(conn: *const PGconn) -> *mut libc::c_char;
    fn PQsocket(conn: *const PGconn) -> libc::c_int;
    fn PQconnectionNeedsPassword(conn: *const PGconn) -> libc::c_int;
    fn PQexec(conn: *mut PGconn, query: *const libc::c_char) -> *mut PGresult;
    fn PQexecParams(
        conn: *mut PGconn,
        command: *const libc::c_char,
        nParams: libc::c_int,
        paramTypes: *const Oid,
        paramValues: *const *const libc::c_char,
        paramLengths: *const libc::c_int,
        paramFormats: *const libc::c_int,
        resultFormat: libc::c_int,
    ) -> *mut PGresult;
    fn PQsendQuery(conn: *mut PGconn, query: *const libc::c_char) -> libc::c_int;
    fn PQsendQueryParams(
        conn: *mut PGconn,
        command: *const libc::c_char,
        nParams: libc::c_int,
        paramTypes: *const Oid,
        paramValues: *const *const libc::c_char,
        paramLengths: *const libc::c_int,
        paramFormats: *const libc::c_int,
        resultFormat: libc::c_int,
    ) -> libc::c_int;
    fn PQisBusy(conn: *mut PGconn) -> libc::c_int;
    fn PQconsumeInput(conn: *mut PGconn) -> libc::c_int;
    fn PQresultStatus(res: *const PGresult) -> ExecStatusType;
    fn PQclear(res: *mut PGresult);
    fn initPQExpBuffer(str: PQExpBuffer);
    fn termPQExpBuffer(str: PQExpBuffer);
    fn resetPQExpBuffer(str: PQExpBuffer);
    fn enlargePQExpBuffer(str: PQExpBuffer, needed: size_t) -> libc::c_int;
    fn appendPQExpBuffer(str: PQExpBuffer, fmt: *const libc::c_char, _: ...);
    fn appendPQExpBufferStr(str: PQExpBuffer, data: *const libc::c_char);
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
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
pub type Oid = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
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
pub type int32 = libc::c_int;
pub type uint32 = libc::c_uint;
pub type int64 = libc::c_long;
pub type uint64 = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type pqsigfunc = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub type ConnStatusType = libc::c_uint;
pub const CONNECTION_GSS_STARTUP: ConnStatusType = 11;
pub const CONNECTION_CONSUME: ConnStatusType = 10;
pub const CONNECTION_CHECK_WRITABLE: ConnStatusType = 9;
pub const CONNECTION_NEEDED: ConnStatusType = 8;
pub const CONNECTION_SSL_STARTUP: ConnStatusType = 7;
pub const CONNECTION_SETENV: ConnStatusType = 6;
pub const CONNECTION_AUTH_OK: ConnStatusType = 5;
pub const CONNECTION_AWAITING_RESPONSE: ConnStatusType = 4;
pub const CONNECTION_MADE: ConnStatusType = 3;
pub const CONNECTION_STARTED: ConnStatusType = 2;
pub const CONNECTION_BAD: ConnStatusType = 1;
pub const CONNECTION_OK: ConnStatusType = 0;
pub type ExecStatusType = libc::c_uint;
pub const PGRES_SINGLE_TUPLE: ExecStatusType = 9;
pub const PGRES_COPY_BOTH: ExecStatusType = 8;
pub const PGRES_FATAL_ERROR: ExecStatusType = 7;
pub const PGRES_NONFATAL_ERROR: ExecStatusType = 6;
pub const PGRES_BAD_RESPONSE: ExecStatusType = 5;
pub const PGRES_COPY_IN: ExecStatusType = 4;
pub const PGRES_COPY_OUT: ExecStatusType = 3;
pub const PGRES_TUPLES_OK: ExecStatusType = 2;
pub const PGRES_COMMAND_OK: ExecStatusType = 1;
pub const PGRES_EMPTY_QUERY: ExecStatusType = 0;
pub type PGTransactionStatusType = libc::c_uint;
pub const PQTRANS_UNKNOWN: PGTransactionStatusType = 4;
pub const PQTRANS_INERROR: PGTransactionStatusType = 3;
pub const PQTRANS_INTRANS: PGTransactionStatusType = 2;
pub const PQTRANS_ACTIVE: PGTransactionStatusType = 1;
pub const PQTRANS_IDLE: PGTransactionStatusType = 0;
pub type PGconn = pg_conn;
pub type PGresult = pg_result;
pub type PGcancel = pg_cancel;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PQExpBufferData {
    pub data: *mut libc::c_char,
    pub len: size_t,
    pub maxlen: size_t,
}
pub type PQExpBuffer = *mut PQExpBufferData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgutErrorData {
    pub elevel: libc::c_int,
    pub save_errno: libc::c_int,
    pub code: libc::c_int,
    pub msg: PQExpBufferData,
    pub detail: PQExpBufferData,
}
pub type YesNo = libc::c_uint;
pub const YES: YesNo = 2;
pub const NO: YesNo = 1;
pub const DEFAULT: YesNo = 0;
pub type pgut_atexit_callback = Option::<
    unsafe extern "C" fn(bool, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgutConn {
    pub conn: *mut PGconn,
    pub cancel: *mut PGcancel,
    pub next: *mut pgutConn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pgut_atexit_item {
    pub callback: pgut_atexit_callback,
    pub userdata: *mut libc::c_void,
    pub next: *mut pgut_atexit_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleStringListCell {
    pub next: *mut SimpleStringListCell,
    pub val: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleStringList {
    pub head: *mut SimpleStringListCell,
    pub tail: *mut SimpleStringListCell,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut pgut_conn_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub static mut PROGRAM_NAME: *const libc::c_char = 0 as *const libc::c_char;
pub static mut interrupted: bool = 0 as libc::c_int != 0;
static mut in_cleanup: bool = 0 as libc::c_int != 0;
pub static mut pgut_log_level: libc::c_int = 17 as libc::c_int;
pub static mut pgut_abort_level: libc::c_int = 20 as libc::c_int;
pub static mut pgut_echo: bool = 0 as libc::c_int != 0;
static mut pgut_connections: *mut pgutConn = 0 as *const pgutConn as *mut pgutConn;
pub unsafe extern "C" fn pgut_init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    if PROGRAM_NAME.is_null() {
        PROGRAM_NAME = get_progname(*argv.offset(0 as libc::c_int as isize));
        set_pglocale_pgservice(
            *argv.offset(0 as libc::c_int as isize),
            b"pgscripts\0" as *const u8 as *const libc::c_char,
        );
        if (getenv(b"PGAPPNAME\0" as *const u8 as *const libc::c_char)).is_null() {
            pgut_putenv(
                b"PGAPPNAME\0" as *const u8 as *const libc::c_char,
                PROGRAM_NAME,
            );
        }
        init_cancel_handler();
        atexit(Some(on_cleanup as unsafe extern "C" fn() -> ()));
    }
}
pub unsafe extern "C" fn pgut_putenv(
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    pg_snprintf(
        buf.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        b"%s=%s\0" as *const u8 as *const libc::c_char,
        key,
        value,
    );
    putenv(pgut_strdup(buf.as_mut_ptr()));
}
pub unsafe extern "C" fn parse_bool(
    mut value: *const libc::c_char,
    mut result: *mut bool,
) -> bool {
    return parse_bool_with_len(value, strlen(value), result);
}
pub unsafe extern "C" fn parse_bool_with_len(
    mut value: *const libc::c_char,
    mut len: size_t,
    mut result: *mut bool,
) -> bool {
    match *value as libc::c_int {
        116 | 84 => {
            if pg_strncasecmp(value, b"true\0" as *const u8 as *const libc::c_char, len)
                == 0 as libc::c_int
            {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        102 | 70 => {
            if pg_strncasecmp(value, b"false\0" as *const u8 as *const libc::c_char, len)
                == 0 as libc::c_int
            {
                if !result.is_null() {
                    *result = 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        121 | 89 => {
            if pg_strncasecmp(value, b"yes\0" as *const u8 as *const libc::c_char, len)
                == 0 as libc::c_int
            {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        110 | 78 => {
            if pg_strncasecmp(value, b"no\0" as *const u8 as *const libc::c_char, len)
                == 0 as libc::c_int
            {
                if !result.is_null() {
                    *result = 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        111 | 79 => {
            if pg_strncasecmp(
                value,
                b"on\0" as *const u8 as *const libc::c_char,
                (if len > 2 as libc::c_int as libc::c_ulong {
                    len
                } else {
                    2 as libc::c_int as libc::c_ulong
                }),
            ) == 0 as libc::c_int
            {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            } else if pg_strncasecmp(
                value,
                b"off\0" as *const u8 as *const libc::c_char,
                (if len > 2 as libc::c_int as libc::c_ulong {
                    len
                } else {
                    2 as libc::c_int as libc::c_ulong
                }),
            ) == 0 as libc::c_int
            {
                if !result.is_null() {
                    *result = 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        49 => {
            if len == 1 as libc::c_int as libc::c_ulong {
                if !result.is_null() {
                    *result = 1 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        48 => {
            if len == 1 as libc::c_int as libc::c_ulong {
                if !result.is_null() {
                    *result = 0 as libc::c_int != 0;
                }
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    if !result.is_null() {
        *result = 0 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_int32(
    mut value: *const libc::c_char,
    mut result: *mut int32,
) -> bool {
    let mut val: int64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *result = 2147483647 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    *__errno_location() = 0 as libc::c_int;
    val = strtol(value, &mut endptr, 0 as libc::c_int);
    if endptr == value as *mut libc::c_char || *endptr as libc::c_int != 0 {
        return 0 as libc::c_int != 0;
    }
    if *__errno_location() == 34 as libc::c_int || val != val as int32 as int64 {
        return 0 as libc::c_int != 0;
    }
    *result = val as int32;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_uint32(
    mut value: *const libc::c_char,
    mut result: *mut uint32,
) -> bool {
    let mut val: uint64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *result = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint);
        return 1 as libc::c_int != 0;
    }
    *__errno_location() = 0 as libc::c_int;
    val = strtoul(value, &mut endptr, 0 as libc::c_int);
    if endptr == value as *mut libc::c_char || *endptr as libc::c_int != 0 {
        return 0 as libc::c_int != 0;
    }
    if *__errno_location() == 34 as libc::c_int || val != val as uint32 as uint64 {
        return 0 as libc::c_int != 0;
    }
    *result = val as uint32;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_int64(
    mut value: *const libc::c_char,
    mut result: *mut int64,
) -> bool {
    let mut val: int64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *result = 9223372036854775807 as libc::c_longlong as int64;
        return 1 as libc::c_int != 0;
    }
    *__errno_location() = 0 as libc::c_int;
    val = strtol(value, &mut endptr, 0 as libc::c_int);
    if endptr == value as *mut libc::c_char || *endptr as libc::c_int != 0 {
        return 0 as libc::c_int != 0;
    }
    if *__errno_location() == 34 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    *result = val;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_uint64(
    mut value: *const libc::c_char,
    mut result: *mut uint64,
) -> bool {
    let mut val: uint64 = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(value, b"INFINITE\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *result = (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong);
        return 1 as libc::c_int != 0;
    }
    *__errno_location() = 0 as libc::c_int;
    val = strtoul(value, &mut endptr, 0 as libc::c_int);
    if endptr == value as *mut libc::c_char || *endptr as libc::c_int != 0 {
        return 0 as libc::c_int != 0;
    }
    if *__errno_location() == 34 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    *result = val;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_time(
    mut value: *const libc::c_char,
    mut time: *mut time_t,
) -> bool {
    let mut len: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tm: tm = tm {
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
    let mut junk: [libc::c_char; 2] = [0; 2];
    tmp = pgut_malloc((strlen(value)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    len = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while *value.offset(i as isize) != 0 {
        let fresh0 = len;
        len = len.wrapping_add(1);
        *tmp
            .offset(
                fresh0 as isize,
            ) = (if *(*__ctype_b_loc())
            .offset(*value.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *value.offset(i as isize) as libc::c_int
        } else {
            ' ' as i32
        }) as libc::c_char;
        i += 1;
        i;
    }
    *tmp.offset(len as isize) = '\0' as i32 as libc::c_char;
    memset(
        &mut tm as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tm>() as libc::c_ulong,
    );
    tm.tm_year = 0 as libc::c_int;
    tm.tm_mon = 0 as libc::c_int;
    tm.tm_mday = 1 as libc::c_int;
    tm.tm_hour = 0 as libc::c_int;
    tm.tm_min = 0 as libc::c_int;
    tm.tm_sec = 0 as libc::c_int;
    i = sscanf(
        tmp,
        b"%04d %02d %02d %02d %02d %02d%1s\0" as *const u8 as *const libc::c_char,
        &mut tm.tm_year as *mut libc::c_int,
        &mut tm.tm_mon as *mut libc::c_int,
        &mut tm.tm_mday as *mut libc::c_int,
        &mut tm.tm_hour as *mut libc::c_int,
        &mut tm.tm_min as *mut libc::c_int,
        &mut tm.tm_sec as *mut libc::c_int,
        junk.as_mut_ptr(),
    );
    free(tmp as *mut libc::c_void);
    if i < 1 as libc::c_int || (6 as libc::c_int) < i {
        return 0 as libc::c_int != 0;
    }
    if tm.tm_year < 100 as libc::c_int {
        tm.tm_year += 2000 as libc::c_int - 1900 as libc::c_int;
    } else if tm.tm_year >= 1900 as libc::c_int {
        tm.tm_year -= 1900 as libc::c_int;
    }
    if i > 1 as libc::c_int {
        tm.tm_mon -= 1 as libc::c_int;
    }
    tm.tm_isdst = -(1 as libc::c_int);
    *time = mktime(&mut tm);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn simple_string_list_append(
    mut list: *mut SimpleStringList,
    mut val: *const libc::c_char,
) {
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    cell = pgut_malloc(
        (::std::mem::size_of::<SimpleStringListCell>() as libc::c_ulong)
            .wrapping_add(strlen(val)),
    ) as *mut SimpleStringListCell;
    (*cell).next = 0 as *mut SimpleStringListCell;
    strcpy(((*cell).val).as_mut_ptr(), val);
    if !((*list).tail).is_null() {
        (*(*list).tail).next = cell;
    } else {
        (*list).head = cell;
    }
    (*list).tail = cell;
}
pub unsafe extern "C" fn simple_string_list_member(
    mut list: *mut SimpleStringList,
    mut val: *const libc::c_char,
) -> bool {
    let mut cell: *mut SimpleStringListCell = 0 as *mut SimpleStringListCell;
    cell = (*list).head;
    while !cell.is_null() {
        if strcmp(((*cell).val).as_mut_ptr(), val) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        cell = (*cell).next;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn simple_string_list_size(mut list: SimpleStringList) -> size_t {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut cell: *mut SimpleStringListCell = list.head;
    while !cell.is_null() {
        cell = (*cell).next;
        i = i.wrapping_add(1);
        i;
    }
    return i;
}
unsafe extern "C" fn prompt_for_password() -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut passwdbuf: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut have_passwd: bool = 0 as libc::c_int != 0;
    buf = pgut_malloc(100 as libc::c_int as size_t) as *mut libc::c_char;
    if have_passwd {
        memcpy(
            buf as *mut libc::c_void,
            passwdbuf as *const libc::c_void,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(100 as libc::c_int as libc::c_ulong),
        );
    } else {
        if !buf.is_null() {
            simple_prompt(
                b"Password: \0" as *const u8 as *const libc::c_char,
                buf,
                100 as libc::c_int as size_t,
                0 as libc::c_int != 0,
            );
        }
        have_passwd = 1 as libc::c_int != 0;
        passwdbuf = pgut_malloc(100 as libc::c_int as size_t) as *mut libc::c_char;
        memcpy(
            passwdbuf as *mut libc::c_void,
            buf as *const libc::c_void,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(100 as libc::c_int as libc::c_ulong),
        );
    }
    if buf.is_null() {
        if pgut_errstart(21 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode_errno(),
                errmsg(
                    b"could not allocate memory (%lu bytes): \0" as *const u8
                        as *const libc::c_char,
                    100 as libc::c_int as uint64,
                ),
            );
        } else {};
    }
    return buf;
}
pub unsafe extern "C" fn pgut_connect(
    mut info: *const libc::c_char,
    mut prompt: YesNo,
    mut elevel: libc::c_int,
) -> *mut PGconn {
    let mut passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut add_pass: PQExpBufferData = PQExpBufferData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
    };
    if prompt as libc::c_uint == YES as libc::c_int as libc::c_uint {
        passwd = prompt_for_password();
        initPQExpBuffer(&mut add_pass);
        appendPQExpBufferStr(&mut add_pass, info);
        appendPQExpBuffer(
            &mut add_pass as *mut PQExpBufferData,
            b" password=%s \0" as *const u8 as *const libc::c_char,
            passwd,
        );
    } else {
        passwd = 0 as *mut libc::c_char;
        add_pass.data = 0 as *mut libc::c_char;
    }
    loop {
        let mut conn: *mut PGconn = 0 as *mut PGconn;
        CHECK_FOR_INTERRUPTS();
        if passwd.is_null() {
            conn = PQconnectdb(info);
        } else {
            conn = PQconnectdb(add_pass.data);
        }
        if PQstatus(conn) as libc::c_uint == CONNECTION_OK as libc::c_int as libc::c_uint
        {
            let mut c: *mut pgutConn = 0 as *mut pgutConn;
            c = pgut_malloc(::std::mem::size_of::<pgutConn>() as libc::c_ulong)
                as *mut pgutConn;
            (*c).conn = conn;
            (*c).cancel = 0 as *mut PGcancel;
            pthread_mutex_lock(&mut pgut_conn_mutex);
            (*c).next = pgut_connections;
            pgut_connections = c;
            pthread_mutex_unlock(&mut pgut_conn_mutex);
            if !(add_pass.data).is_null() {
                termPQExpBuffer(&mut add_pass);
            }
            free(passwd as *mut libc::c_void);
            pgut_command(
                conn,
                b"SET search_path TO pg_catalog, pg_temp, public\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut *const libc::c_char,
            );
            return conn;
        }
        if !conn.is_null() && PQconnectionNeedsPassword(conn) != 0
            && prompt as libc::c_uint != NO as libc::c_int as libc::c_uint
        {
            PQfinish(conn);
            free(passwd as *mut libc::c_void);
            passwd = prompt_for_password();
            if !(add_pass.data).is_null() {
                resetPQExpBuffer(&mut add_pass);
            } else {
                initPQExpBuffer(&mut add_pass);
            }
            appendPQExpBufferStr(&mut add_pass, info);
            appendPQExpBuffer(
                &mut add_pass as *mut PQExpBufferData,
                b" password=%s \0" as *const u8 as *const libc::c_char,
                passwd,
            );
        } else {
            if !(add_pass.data).is_null() {
                termPQExpBuffer(&mut add_pass);
            }
            free(passwd as *mut libc::c_void);
            if pgut_errstart(elevel) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(-(1 as libc::c_int)),
                    errmsg(
                        b"could not connect to database: %s\0" as *const u8
                            as *const libc::c_char,
                        PQerrorMessage(conn),
                    ),
                );
            } else {};
            PQfinish(conn);
            return 0 as *mut PGconn;
        }
    };
}
pub unsafe extern "C" fn pgut_disconnect(mut conn: *mut PGconn) {
    if !conn.is_null() {
        let mut c: *mut pgutConn = 0 as *mut pgutConn;
        let mut prev: *mut *mut pgutConn = 0 as *mut *mut pgutConn;
        pthread_mutex_lock(&mut pgut_conn_mutex);
        prev = &mut pgut_connections;
        c = pgut_connections;
        while !c.is_null() {
            if (*c).conn == conn {
                *prev = (*c).next;
                break;
            } else {
                prev = &mut (*c).next;
                c = (*c).next;
            }
        }
        pthread_mutex_unlock(&mut pgut_conn_mutex);
        PQfinish(conn);
    }
}
pub unsafe extern "C" fn pgut_disconnect_all() {
    pthread_mutex_lock(&mut pgut_conn_mutex);
    while !pgut_connections.is_null() {
        PQfinish((*pgut_connections).conn);
        pgut_connections = (*pgut_connections).next;
    }
    pthread_mutex_unlock(&mut pgut_conn_mutex);
}
unsafe extern "C" fn echo_query(
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if !(strchr(query, '\n' as i32)).is_null() {
        elog(
            15 as libc::c_int,
            b"(query)\n%s\0" as *const u8 as *const libc::c_char,
            query,
        );
    } else {
        elog(
            15 as libc::c_int,
            b"(query) %s\0" as *const u8 as *const libc::c_char,
            query,
        );
    }
    i = 0 as libc::c_int;
    while i < nParams {
        elog(
            15 as libc::c_int,
            b"\t(param:%d) = %s\0" as *const u8 as *const libc::c_char,
            i,
            if !(*params.offset(i as isize)).is_null() {
                *params.offset(i as isize)
            } else {
                b"(null)\0" as *const u8 as *const libc::c_char
            },
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn pgut_execute(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> *mut PGresult {
    return pgut_execute_elevel(conn, query, nParams, params, 20 as libc::c_int);
}
pub unsafe extern "C" fn pgut_execute_elevel(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
    mut elevel: libc::c_int,
) -> *mut PGresult {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut c: *mut pgutConn = 0 as *mut pgutConn;
    CHECK_FOR_INTERRUPTS();
    if pgut_echo {
        echo_query(query, nParams, params);
    }
    if conn.is_null() {
        if pgut_errstart(elevel) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(-(2 as libc::c_int)),
                errmsg(b"not connected\0" as *const u8 as *const libc::c_char),
            );
        } else {};
        return 0 as *mut PGresult;
    }
    pthread_mutex_lock(&mut pgut_conn_mutex);
    c = pgut_connections;
    while !c.is_null() {
        if (*c).conn == conn {
            break;
        }
        c = (*c).next;
    }
    pthread_mutex_unlock(&mut pgut_conn_mutex);
    if !c.is_null() {
        on_before_exec(c);
    }
    if nParams == 0 as libc::c_int {
        res = PQexec(conn, query);
    } else {
        res = PQexecParams(
            conn,
            query,
            nParams,
            0 as *const Oid,
            params,
            0 as *const libc::c_int,
            0 as *const libc::c_int,
            0 as libc::c_int,
        );
    }
    if !c.is_null() {
        on_after_exec(c);
    }
    match PQresultStatus(res) as libc::c_uint {
        2 | 1 | 4 => {}
        _ => {
            if pgut_errstart(elevel) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(-(2 as libc::c_int)),
                    errmsg(
                        b"query failed: %s\0" as *const u8 as *const libc::c_char,
                        PQerrorMessage(conn),
                    ),
                    errdetail(
                        b"query was: %s\0" as *const u8 as *const libc::c_char,
                        query,
                    ),
                );
            } else {};
        }
    }
    return res;
}
pub unsafe extern "C" fn pgut_command(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> ExecStatusType {
    let mut res: *mut PGresult = 0 as *mut PGresult;
    let mut code: ExecStatusType = PGRES_EMPTY_QUERY;
    res = pgut_execute(conn, query, nParams, params);
    code = PQresultStatus(res);
    PQclear(res);
    return code;
}
pub unsafe extern "C" fn pgut_commit(mut conn: *mut PGconn) -> bool {
    if !conn.is_null()
        && PQtransactionStatus(conn) as libc::c_uint
            != PQTRANS_IDLE as libc::c_int as libc::c_uint
    {
        return pgut_command(
            conn,
            b"COMMIT\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut *const libc::c_char,
        ) as libc::c_uint == PGRES_COMMAND_OK as libc::c_int as libc::c_uint;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pgut_rollback(mut conn: *mut PGconn) {
    if !conn.is_null()
        && PQtransactionStatus(conn) as libc::c_uint
            != PQTRANS_IDLE as libc::c_int as libc::c_uint
    {
        pgut_command(
            conn,
            b"ROLLBACK\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn pgut_send(
    mut conn: *mut PGconn,
    mut query: *const libc::c_char,
    mut nParams: libc::c_int,
    mut params: *mut *const libc::c_char,
) -> bool {
    let mut res: libc::c_int = 0;
    CHECK_FOR_INTERRUPTS();
    if pgut_echo {
        echo_query(query, nParams, params);
    }
    if conn.is_null() {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(-(2 as libc::c_int)),
                errmsg(b"not connected\0" as *const u8 as *const libc::c_char),
            );
        } else {};
        return 0 as libc::c_int != 0;
    }
    if nParams == 0 as libc::c_int {
        res = PQsendQuery(conn, query);
    } else {
        res = PQsendQueryParams(
            conn,
            query,
            nParams,
            0 as *const Oid,
            params,
            0 as *const libc::c_int,
            0 as *const libc::c_int,
            0 as libc::c_int,
        );
    }
    if res != 1 as libc::c_int {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(-(2 as libc::c_int)),
                errmsg(
                    b"query failed: %s\0" as *const u8 as *const libc::c_char,
                    PQerrorMessage(conn),
                ),
                errdetail(b"query was: %s\0" as *const u8 as *const libc::c_char, query),
            );
        } else {};
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pgut_wait(
    mut num: libc::c_int,
    mut connections: *mut *mut PGconn,
    mut timeout: *mut timeval,
) -> libc::c_int {
    while !interrupted {
        let mut i: libc::c_int = 0;
        let mut mask: fd_set = fd_set { fds_bits: [0; 16] };
        let mut maxsock: libc::c_int = 0;
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh1 = &mut __d0;
        let fresh2;
        let fresh3 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh4 = &mut __d1;
        let fresh5;
        let fresh6 = &mut *(mask.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh1,
            fresh3) => fresh2, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh4,
            fresh6) => fresh5, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
        c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
        maxsock = -(1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < num {
            let mut sock: libc::c_int = 0;
            if !(*connections.offset(i as isize)).is_null() {
                sock = PQsocket(*connections.offset(i as isize));
                if sock >= 0 as libc::c_int {
                    mask
                        .fds_bits[(sock
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << sock
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    if maxsock < sock {
                        maxsock = sock;
                    }
                }
            }
            i += 1;
            i;
        }
        if maxsock == -(1 as libc::c_int) {
            *__errno_location() = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
        i = wait_for_sockets(maxsock + 1 as libc::c_int, &mut mask, timeout);
        if i == 0 as libc::c_int {
            break;
        }
        i = 0 as libc::c_int;
        while i < num {
            if !(*connections.offset(i as isize)).is_null()
                && mask
                    .fds_bits[(PQsocket(*connections.offset(i as isize))
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << PQsocket(*connections.offset(i as isize))
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                PQconsumeInput(*connections.offset(i as isize));
                if !(PQisBusy(*connections.offset(i as isize)) != 0) {
                    return i;
                }
            }
            i += 1;
            i;
        }
    }
    *__errno_location() = 4 as libc::c_int;
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn CHECK_FOR_INTERRUPTS() {
    if interrupted as libc::c_int != 0 && !in_cleanup {
        if pgut_errstart(21 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode(4 as libc::c_int),
                errmsg(b"interrupted\0" as *const u8 as *const libc::c_char),
            );
        } else {};
    }
}
unsafe extern "C" fn getErrorData() -> *mut pgutErrorData {
    static mut edata: pgutErrorData = pgutErrorData {
        elevel: 0,
        save_errno: 0,
        code: 0,
        msg: PQExpBufferData {
            data: 0 as *mut libc::c_char,
            len: 0,
            maxlen: 0,
        },
        detail: PQExpBufferData {
            data: 0 as *mut libc::c_char,
            len: 0,
            maxlen: 0,
        },
    };
    return &mut edata;
}
unsafe extern "C" fn pgut_errinit(mut elevel: libc::c_int) -> *mut pgutErrorData {
    let mut save_errno: libc::c_int = *__errno_location();
    let mut edata: *mut pgutErrorData = getErrorData();
    (*edata).elevel = elevel;
    (*edata).save_errno = save_errno;
    (*edata)
        .code = if elevel >= 20 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if !((*edata).msg.data).is_null() {
        resetPQExpBuffer(&mut (*edata).msg);
    } else {
        initPQExpBuffer(&mut (*edata).msg);
    }
    if !((*edata).detail.data).is_null() {
        resetPQExpBuffer(&mut (*edata).detail);
    } else {
        initPQExpBuffer(&mut (*edata).detail);
    }
    return edata;
}
unsafe extern "C" fn trimStringBuffer(mut str: PQExpBuffer) {
    while (*str).len > 0 as libc::c_int as libc::c_ulong
        && *(*__ctype_b_loc())
            .offset(
                *((*str).data)
                    .offset(
                        ((*str).len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_uchar as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*str).len = ((*str).len).wrapping_sub(1);
        *((*str).data).offset((*str).len as isize) = '\0' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn elog(
    mut elevel: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut ok: bool = false;
    let mut len: size_t = 0;
    let mut edata: *mut pgutErrorData = 0 as *mut pgutErrorData;
    if elevel < pgut_abort_level && !log_required(elevel, pgut_log_level) {
        return;
    }
    edata = pgut_errinit(elevel);
    loop {
        args_0 = args.clone();
        ok = pgut_appendStringInfoVA(&mut (*edata).msg, fmt, args_0.as_va_list());
        if ok {
            break;
        }
    }
    len = strlen(fmt);
    if len > 2 as libc::c_int as libc::c_ulong
        && strcmp(
            fmt.offset(len as isize).offset(-(2 as libc::c_int as isize)),
            b": \0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        appendPQExpBufferStr(&mut (*edata).msg, pg_strerror((*edata).save_errno));
    }
    trimStringBuffer(&mut (*edata).msg);
    pgut_errfinish(1 as libc::c_int);
}
pub unsafe extern "C" fn pgut_errstart(mut elevel: libc::c_int) -> bool {
    if elevel < pgut_abort_level && !log_required(elevel, pgut_log_level) {
        return 0 as libc::c_int != 0;
    }
    pgut_errinit(elevel);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pgut_errfinish(mut dummy: libc::c_int, mut args: ...) {
    let mut edata: *mut pgutErrorData = getErrorData();
    if log_required((*edata).elevel, pgut_log_level) {
        pgut_error(
            (*edata).elevel,
            (*edata).code,
            if !((*edata).msg.data).is_null() {
                (*edata).msg.data as *const libc::c_char
            } else {
                b"unknown\0" as *const u8 as *const libc::c_char
            },
            (*edata).detail.data,
        );
    }
    if pgut_abort_level <= (*edata).elevel && (*edata).elevel <= 22 as libc::c_int {
        in_cleanup = 1 as libc::c_int != 0;
        exit_or_abort((*edata).code, (*edata).elevel);
    }
}
pub unsafe extern "C" fn pgut_error(
    mut elevel: libc::c_int,
    mut code: libc::c_int,
    mut msg: *const libc::c_char,
    mut detail: *const libc::c_char,
) {
    let mut tag: *const libc::c_char = format_elevel(elevel);
    if !detail.is_null() && *detail.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        pg_fprintf(
            stderr,
            b"%s: %s\nDETAIL: %s\n\0" as *const u8 as *const libc::c_char,
            tag,
            msg,
            detail,
        );
    } else {
        pg_fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const libc::c_char, tag, msg);
    }
    fflush(stderr);
}
pub unsafe extern "C" fn log_required(
    mut elevel: libc::c_int,
    mut log_min_level: libc::c_int,
) -> bool {
    if elevel == 15 as libc::c_int || elevel == 16 as libc::c_int {
        if log_min_level == 15 as libc::c_int || log_min_level <= 20 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
    } else if log_min_level == 15 as libc::c_int {
        if elevel >= 21 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
    } else if elevel >= log_min_level {
        return 1 as libc::c_int != 0
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn format_elevel(mut elevel: libc::c_int) -> *const libc::c_char {
    match elevel {
        10 | 11 | 12 | 13 | 14 => return b"DEBUG\0" as *const u8 as *const libc::c_char,
        15 => return b"LOG\0" as *const u8 as *const libc::c_char,
        17 => return b"INFO\0" as *const u8 as *const libc::c_char,
        18 => return b"NOTICE\0" as *const u8 as *const libc::c_char,
        19 => return b"WARNING\0" as *const u8 as *const libc::c_char,
        16 | 20 => return b"ERROR\0" as *const u8 as *const libc::c_char,
        21 => return b"FATAL\0" as *const u8 as *const libc::c_char,
        22 => return b"PANIC\0" as *const u8 as *const libc::c_char,
        _ => {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode(22 as libc::c_int),
                    errmsg(
                        b"invalid elevel: %d\0" as *const u8 as *const libc::c_char,
                        elevel,
                    ),
                );
            } else {};
            return b"\0" as *const u8 as *const libc::c_char;
        }
    };
}
pub unsafe extern "C" fn parse_elevel(mut value: *const libc::c_char) -> libc::c_int {
    if pg_strcasecmp(value, b"DEBUG\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 13 as libc::c_int
    } else if pg_strcasecmp(value, b"INFO\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 17 as libc::c_int
    } else if pg_strcasecmp(value, b"NOTICE\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 18 as libc::c_int
    } else if pg_strcasecmp(value, b"LOG\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 15 as libc::c_int
    } else if pg_strcasecmp(value, b"WARNING\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 19 as libc::c_int
    } else if pg_strcasecmp(value, b"ERROR\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 20 as libc::c_int
    } else if pg_strcasecmp(value, b"FATAL\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 21 as libc::c_int
    } else if pg_strcasecmp(value, b"PANIC\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 22 as libc::c_int
    }
    if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
        pgut_errfinish(
            errcode(22 as libc::c_int),
            errmsg(b"invalid elevel: %s\0" as *const u8 as *const libc::c_char, value),
        );
    } else {};
    return 20 as libc::c_int;
}
pub unsafe extern "C" fn errcode(mut sqlerrcode: libc::c_int) -> libc::c_int {
    let mut edata: *mut pgutErrorData = getErrorData();
    (*edata).code = sqlerrcode;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn errcode_errno() -> libc::c_int {
    let mut edata: *mut pgutErrorData = getErrorData();
    (*edata).code = (*edata).save_errno;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn errmsg(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut edata: *mut pgutErrorData = getErrorData();
    let mut args_0: ::std::ffi::VaListImpl;
    let mut len: size_t = 0;
    let mut ok: bool = false;
    loop {
        args_0 = args.clone();
        ok = pgut_appendStringInfoVA(&mut (*edata).msg, fmt, args_0.as_va_list());
        if ok {
            break;
        }
    }
    len = strlen(fmt);
    if len > 2 as libc::c_int as libc::c_ulong
        && strcmp(
            fmt.offset(len as isize).offset(-(2 as libc::c_int as isize)),
            b": \0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        appendPQExpBufferStr(&mut (*edata).msg, pg_strerror((*edata).save_errno));
    }
    trimStringBuffer(&mut (*edata).msg);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn errdetail(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut edata: *mut pgutErrorData = getErrorData();
    let mut args_0: ::std::ffi::VaListImpl;
    let mut ok: bool = false;
    loop {
        args_0 = args.clone();
        ok = pgut_appendStringInfoVA(&mut (*edata).detail, fmt, args_0.as_va_list());
        if ok {
            break;
        }
    }
    trimStringBuffer(&mut (*edata).detail);
    return 0 as libc::c_int;
}
unsafe extern "C" fn on_before_exec(mut conn: *mut pgutConn) {
    let mut old: *mut PGcancel = 0 as *mut PGcancel;
    if in_cleanup {
        return;
    }
    old = (*conn).cancel;
    (*conn).cancel = 0 as *mut PGcancel;
    if !old.is_null() {
        PQfreeCancel(old);
    }
    (*conn).cancel = PQgetCancel((*conn).conn);
}
unsafe extern "C" fn on_after_exec(mut conn: *mut pgutConn) {
    let mut old: *mut PGcancel = 0 as *mut PGcancel;
    if in_cleanup {
        return;
    }
    old = (*conn).cancel;
    (*conn).cancel = 0 as *mut PGcancel;
    if !old.is_null() {
        PQfreeCancel(old);
    }
}
unsafe extern "C" fn on_interrupt() {
    let mut c: *mut pgutConn = 0 as *mut pgutConn;
    let mut save_errno: libc::c_int = *__errno_location();
    interrupted = 1 as libc::c_int != 0;
    if in_cleanup {
        return;
    }
    pthread_mutex_lock(&mut pgut_conn_mutex);
    c = pgut_connections;
    while !c.is_null() {
        let mut buf: [libc::c_char; 256] = [0; 256];
        if !((*c).cancel).is_null()
            && PQcancel(
                (*c).cancel,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    as libc::c_int,
            ) != 0
        {
            elog(
                19 as libc::c_int,
                b"Cancel request sent\0" as *const u8 as *const libc::c_char,
            );
        }
        c = (*c).next;
    }
    pthread_mutex_unlock(&mut pgut_conn_mutex);
    *__errno_location() = save_errno;
}
static mut pgut_atexit_stack: *mut pgut_atexit_item = 0 as *const pgut_atexit_item
    as *mut pgut_atexit_item;
pub unsafe extern "C" fn pgut_atexit_push(
    mut callback: pgut_atexit_callback,
    mut userdata: *mut libc::c_void,
) {
    let mut item: *mut pgut_atexit_item = 0 as *mut pgut_atexit_item;
    item = pgut_malloc(::std::mem::size_of::<pgut_atexit_item>() as libc::c_ulong)
        as *mut pgut_atexit_item;
    (*item).callback = callback;
    (*item).userdata = userdata;
    (*item).next = pgut_atexit_stack;
    pgut_atexit_stack = item;
}
pub unsafe extern "C" fn pgut_atexit_pop(
    mut callback: pgut_atexit_callback,
    mut userdata: *mut libc::c_void,
) {
    let mut item: *mut pgut_atexit_item = 0 as *mut pgut_atexit_item;
    let mut prev: *mut *mut pgut_atexit_item = 0 as *mut *mut pgut_atexit_item;
    item = pgut_atexit_stack;
    prev = &mut pgut_atexit_stack;
    while !item.is_null() {
        if (*item).callback == callback && (*item).userdata == userdata {
            *prev = (*item).next;
            free(item as *mut libc::c_void);
            break;
        } else {
            prev = &mut (*item).next;
            item = (*item).next;
        }
    }
}
unsafe extern "C" fn call_atexit_callbacks(mut fatal: bool) {
    let mut item: *mut pgut_atexit_item = 0 as *mut pgut_atexit_item;
    item = pgut_atexit_stack;
    while !item.is_null() {
        ((*item).callback).unwrap()(fatal, (*item).userdata);
        item = (*item).next;
    }
}
unsafe extern "C" fn on_cleanup() {
    in_cleanup = 1 as libc::c_int != 0;
    interrupted = 0 as libc::c_int != 0;
    call_atexit_callbacks(0 as libc::c_int != 0);
    pgut_disconnect_all();
}
unsafe extern "C" fn exit_or_abort(mut exitcode: libc::c_int, mut elevel: libc::c_int) {
    if in_cleanup as libc::c_int != 0 && 21 as libc::c_int > elevel {
        call_atexit_callbacks(1 as libc::c_int != 0);
        exit(exitcode);
    } else if elevel >= 21 as libc::c_int && elevel <= 22 as libc::c_int {
        call_atexit_callbacks(1 as libc::c_int != 0);
        abort();
    } else {
        exit(exitcode);
    };
}
pub unsafe extern "C" fn pgut_appendStringInfoVA(
    mut str: PQExpBuffer,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> bool {
    let mut avail: size_t = 0;
    let mut nprinted: libc::c_int = 0;
    avail = ((*str).maxlen)
        .wrapping_sub((*str).len)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    nprinted = pg_vsnprintf(
        ((*str).data).offset((*str).len as isize),
        avail,
        fmt,
        args.as_va_list(),
    );
    if nprinted >= 0 as libc::c_int && nprinted < avail as libc::c_int - 1 as libc::c_int
    {
        (*str)
            .len = ((*str).len as libc::c_ulong).wrapping_add(nprinted as libc::c_ulong)
            as size_t as size_t;
        return 1 as libc::c_int != 0;
    }
    enlargePQExpBuffer(str, (*str).maxlen);
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn appendStringInfoFile(
    mut str: PQExpBuffer,
    mut fp: *mut FILE,
) -> libc::c_int {
    loop {
        let mut rc: libc::c_int = 0;
        if ((*str).maxlen).wrapping_sub((*str).len) < 2 as libc::c_int as libc::c_ulong
            && enlargePQExpBuffer(str, 1024 as libc::c_int as size_t) == 0 as libc::c_int
        {
            let ref mut fresh7 = *__errno_location();
            *fresh7 = 12 as libc::c_int;
            return *fresh7;
        }
        rc = fread(
            ((*str).data).offset((*str).len as isize) as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ((*str).maxlen)
                .wrapping_sub((*str).len)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            fp,
        ) as libc::c_int;
        if rc == 0 as libc::c_int {
            break;
        }
        if rc > 0 as libc::c_int {
            (*str)
                .len = ((*str).len as libc::c_ulong).wrapping_add(rc as libc::c_ulong)
                as size_t as size_t;
            *((*str).data).offset((*str).len as isize) = '\0' as i32 as libc::c_char;
        } else if ferror(fp) != 0 && *__errno_location() != 4 as libc::c_int {
            return *__errno_location()
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn appendStringInfoFd(
    mut str: PQExpBuffer,
    mut fd: libc::c_int,
) -> libc::c_int {
    loop {
        let mut rc: libc::c_int = 0;
        if ((*str).maxlen).wrapping_sub((*str).len) < 2 as libc::c_int as libc::c_ulong
            && enlargePQExpBuffer(str, 1024 as libc::c_int as size_t) == 0 as libc::c_int
        {
            let ref mut fresh8 = *__errno_location();
            *fresh8 = 12 as libc::c_int;
            return *fresh8;
        }
        rc = read(
            fd,
            ((*str).data).offset((*str).len as isize) as *mut libc::c_void,
            ((*str).maxlen)
                .wrapping_sub((*str).len)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
        if rc == 0 as libc::c_int {
            break;
        }
        if rc > 0 as libc::c_int {
            (*str)
                .len = ((*str).len as libc::c_ulong).wrapping_add(rc as libc::c_ulong)
                as size_t as size_t;
            *((*str).data).offset((*str).len as isize) = '\0' as i32 as libc::c_char;
        } else if *__errno_location() != 4 as libc::c_int {
            return *__errno_location()
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pgut_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = malloc(size) as *mut libc::c_char;
    if ret.is_null() {
        if pgut_errstart(21 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode_errno(),
                errmsg(
                    b"could not allocate memory (%lu bytes): \0" as *const u8
                        as *const libc::c_char,
                    size,
                ),
            );
        } else {};
    }
    return ret as *mut libc::c_void;
}
pub unsafe extern "C" fn pgut_realloc(
    mut p: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = realloc(p, size) as *mut libc::c_char;
    if ret.is_null() {
        if pgut_errstart(21 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode_errno(),
                errmsg(
                    b"could not re-allocate memory (%lu bytes): \0" as *const u8
                        as *const libc::c_char,
                    size,
                ),
            );
        } else {};
    }
    return ret as *mut libc::c_void;
}
pub unsafe extern "C" fn pgut_strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    ret = strdup(str);
    if ret.is_null() {
        if pgut_errstart(21 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode_errno(),
                errmsg(
                    b"could not duplicate string \"%s\": \0" as *const u8
                        as *const libc::c_char,
                    str,
                ),
            );
        } else {};
    }
    return ret;
}
pub unsafe extern "C" fn strdup_with_len(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    r = pgut_malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memcpy(r as *mut libc::c_void, str as *const libc::c_void, len);
    *r.offset(len as isize) = '\0' as i32 as libc::c_char;
    return r;
}
pub unsafe extern "C" fn strdup_trim(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut len: size_t = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    while *(*__ctype_b_loc())
        .offset(
            *str.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                as isize,
        ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        str = str.offset(1);
        str;
    }
    len = strlen(str);
    while len > 0 as libc::c_int as libc::c_ulong
        && *(*__ctype_b_loc())
            .offset(
                *str.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_uchar as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        len = len.wrapping_sub(1);
        len;
    }
    return strdup_with_len(str, len);
}
pub unsafe extern "C" fn pgut_fopen(
    mut path: *const libc::c_char,
    mut omode: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut missing_ok: bool = 0 as libc::c_int != 0;
    let mut mode: [libc::c_char; 16] = [0; 16];
    strlcpy(
        mode.as_mut_ptr(),
        omode,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    if mode[0 as libc::c_int as usize] as libc::c_int == 'R' as i32 {
        mode[0 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
        missing_ok = 1 as libc::c_int != 0;
    }
    let mut current_block_10: u64;
    loop {
        fp = fopen(path, mode.as_mut_ptr());
        if !fp.is_null() {
            current_block_10 = 13586036798005543211;
            break;
        }
        if !(*__errno_location() == 2 as libc::c_int) {
            current_block_10 = 2968425633554183086;
            break;
        }
        if missing_ok {
            return 0 as *mut FILE;
        }
        if !(mode[0 as libc::c_int as usize] as libc::c_int == 'w' as i32
            || mode[0 as libc::c_int as usize] as libc::c_int == 'a' as i32)
        {
            current_block_10 = 2968425633554183086;
            break;
        }
        let mut dir: [libc::c_char; 1024] = [0; 1024];
        strlcpy(dir.as_mut_ptr(), path, 1024 as libc::c_int as size_t);
        get_parent_directory(dir.as_mut_ptr());
        pgut_mkdir(dir.as_mut_ptr());
    }
    match current_block_10 {
        2968425633554183086 => {
            if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                pgut_errfinish(
                    errcode_errno(),
                    errmsg(
                        b"could not open file \"%s\": \0" as *const u8
                            as *const libc::c_char,
                        path,
                    ),
                );
            } else {};
        }
        _ => {}
    }
    return fp;
}
pub unsafe extern "C" fn pgut_mkdir(mut dirpath: *const libc::c_char) -> bool {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut first: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    path = pgut_strdup(dirpath);
    p = path;
    retval = 0 as libc::c_int;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        p = p.offset(1);
        p;
    }
    let mut current_block_18: u64;
    first = 1 as libc::c_int;
    last = 0 as libc::c_int;
    's_27: while last == 0 {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            last = 1 as libc::c_int;
            current_block_18 = 3276175668257526147;
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
            current_block_18 = 15427931788582360902;
        } else {
            current_block_18 = 3276175668257526147;
        }
        match current_block_18 {
            3276175668257526147 => {
                *p = '\0' as i32 as libc::c_char;
                if last == 0
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    last = 1 as libc::c_int;
                }
                if first != 0 {
                    first = 0 as libc::c_int;
                }
                loop {
                    if stat(path, &mut sb) == 0 as libc::c_int {
                        if !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint)
                        {
                            current_block_18 = 12349973810996921269;
                            break;
                        } else {
                            current_block_18 = 18317007320854588510;
                            break;
                        }
                    } else {
                        if !(mkdir(
                            path,
                            (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) as __mode_t,
                        ) < 0 as libc::c_int)
                        {
                            current_block_18 = 18317007320854588510;
                            break;
                        }
                        if *__errno_location() == 17 as libc::c_int {
                            continue;
                        }
                        retval = 1 as libc::c_int;
                        break 's_27;
                    }
                }
                match current_block_18 {
                    18317007320854588510 => {
                        if last == 0 {
                            *p = '/' as i32 as libc::c_char;
                        }
                    }
                    _ => {
                        if last != 0 {
                            *__errno_location() = 17 as libc::c_int;
                        } else {
                            *__errno_location() = 20 as libc::c_int;
                        }
                        retval = 1 as libc::c_int;
                        break;
                    }
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    free(path as *mut libc::c_void);
    if retval == 0 as libc::c_int {
        if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
            pgut_errfinish(
                errcode_errno(),
                errmsg(
                    b"could not create directory \"%s\": \0" as *const u8
                        as *const libc::c_char,
                    dirpath,
                ),
            );
        } else {};
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn wait_for_socket(
    mut sock: libc::c_int,
    mut timeout: *mut timeval,
) -> libc::c_int {
    let mut fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh9 = &mut __d0;
    let fresh10;
    let fresh11 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh12 = &mut __d1;
    let fresh13;
    let fresh14 = &mut *(fds.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh9,
        fresh11) => fresh10, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh12,
        fresh14) => fresh13, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
    fds
        .fds_bits[(sock
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << sock
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    return wait_for_sockets(sock + 1 as libc::c_int, &mut fds, timeout);
}
pub unsafe extern "C" fn wait_for_sockets(
    mut nfds: libc::c_int,
    mut fds: *mut fd_set,
    mut timeout: *mut timeval,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    loop {
        i = select(nfds, fds, 0 as *mut fd_set, 0 as *mut fd_set, timeout);
        if i < 0 as libc::c_int {
            CHECK_FOR_INTERRUPTS();
            if *__errno_location() != 4 as libc::c_int {
                if pgut_errstart(20 as libc::c_int) as libc::c_int != 0 {
                    pgut_errfinish(
                        errcode_errno(),
                        errmsg(b"select failed: \0" as *const u8 as *const libc::c_char),
                    );
                } else {};
                return -(1 as libc::c_int);
            }
        } else {
            return i
        }
    };
}
unsafe extern "C" fn handle_sigint(mut postgres_signal_arg: libc::c_int) {
    on_interrupt();
}
unsafe extern "C" fn init_cancel_handler() {
    pqsignal(
        2 as libc::c_int,
        Some(handle_sigint as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
