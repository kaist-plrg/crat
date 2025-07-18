use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rtlsdr_dev;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn rtlsdr_get_device_count() -> uint32_t;
    fn rtlsdr_get_device_usb_strings(
        index: uint32_t,
        manufact: *mut libc::c_char,
        product: *mut libc::c_char,
        serial: *mut libc::c_char,
    ) -> libc::c_int;
    fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: uint32_t) -> libc::c_int;
    fn rtlsdr_close(dev: *mut rtlsdr_dev_t) -> libc::c_int;
    fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev_t, freq: uint32_t) -> libc::c_int;
    fn rtlsdr_set_freq_correction(
        dev: *mut rtlsdr_dev_t,
        ppm: libc::c_int,
    ) -> libc::c_int;
    fn rtlsdr_get_tuner_gains(
        dev: *mut rtlsdr_dev_t,
        gains: *mut libc::c_int,
    ) -> libc::c_int;
    fn rtlsdr_set_tuner_gain(dev: *mut rtlsdr_dev_t, gain: libc::c_int) -> libc::c_int;
    fn rtlsdr_get_tuner_gain(dev: *mut rtlsdr_dev_t) -> libc::c_int;
    fn rtlsdr_set_tuner_gain_mode(
        dev: *mut rtlsdr_dev_t,
        manual: libc::c_int,
    ) -> libc::c_int;
    fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev_t, rate: uint32_t) -> libc::c_int;
    fn rtlsdr_set_agc_mode(dev: *mut rtlsdr_dev_t, on: libc::c_int) -> libc::c_int;
    fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev_t) -> libc::c_int;
    fn rtlsdr_read_async(
        dev: *mut rtlsdr_dev_t,
        cb: rtlsdr_read_async_cb_t,
        ctx: *mut libc::c_void,
        buf_num: uint32_t,
        buf_len: uint32_t,
    ) -> libc::c_int;
    fn anetTcpServer(
        err: *mut libc::c_char,
        port: libc::c_int,
        bindaddr: *mut libc::c_char,
    ) -> libc::c_int;
    fn anetTcpAccept(
        err: *mut libc::c_char,
        serversock: libc::c_int,
        ip: *mut libc::c_char,
        port: *mut libc::c_int,
    ) -> libc::c_int;
    fn anetNonBlock(err: *mut libc::c_char, fd: libc::c_int) -> libc::c_int;
    fn anetSetSendBuffer(
        err: *mut libc::c_char,
        fd: libc::c_int,
        buffsize: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
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
    pub __fds_bits: [__fd_mask; 16],
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type rtlsdr_dev_t = rtlsdr_dev;
pub type rtlsdr_read_async_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, uint32_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub fd: libc::c_int,
    pub service: libc::c_int,
    pub buf: [libc::c_char; 1025],
    pub buflen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aircraft {
    pub addr: uint32_t,
    pub hexaddr: [libc::c_char; 7],
    pub flight: [libc::c_char; 9],
    pub altitude: libc::c_int,
    pub speed: libc::c_int,
    pub track: libc::c_int,
    pub seen: time_t,
    pub messages: libc::c_long,
    pub odd_cprlat: libc::c_int,
    pub odd_cprlon: libc::c_int,
    pub even_cprlat: libc::c_int,
    pub even_cprlon: libc::c_int,
    pub lat: libc::c_double,
    pub lon: libc::c_double,
    pub odd_cprtime: libc::c_longlong,
    pub even_cprtime: libc::c_longlong,
    pub next: *mut aircraft,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub reader_thread: pthread_t,
    pub data_mutex: pthread_mutex_t,
    pub data_cond: pthread_cond_t,
    pub data: *mut libc::c_uchar,
    pub magnitude: *mut uint16_t,
    pub data_len: uint32_t,
    pub fd: libc::c_int,
    pub data_ready: libc::c_int,
    pub icao_cache: *mut uint32_t,
    pub maglut: *mut uint16_t,
    pub exit: libc::c_int,
    pub dev_index: libc::c_int,
    pub gain: libc::c_int,
    pub enable_agc: libc::c_int,
    pub dev: *mut rtlsdr_dev_t,
    pub freq: libc::c_int,
    pub aneterr: [libc::c_char; 256],
    pub clients: [*mut client; 1024],
    pub maxfd: libc::c_int,
    pub sbsos: libc::c_int,
    pub ros: libc::c_int,
    pub ris: libc::c_int,
    pub https: libc::c_int,
    pub filename: *mut libc::c_char,
    pub loop_0: libc::c_int,
    pub fix_errors: libc::c_int,
    pub check_crc: libc::c_int,
    pub raw: libc::c_int,
    pub debug: libc::c_int,
    pub net: libc::c_int,
    pub net_only: libc::c_int,
    pub interactive: libc::c_int,
    pub interactive_rows: libc::c_int,
    pub interactive_ttl: libc::c_int,
    pub stats: libc::c_int,
    pub onlyaddr: libc::c_int,
    pub metric: libc::c_int,
    pub aggressive: libc::c_int,
    pub aircrafts: *mut aircraft,
    pub interactive_last_update: libc::c_longlong,
    pub stat_valid_preamble: libc::c_longlong,
    pub stat_demodulated: libc::c_longlong,
    pub stat_goodcrc: libc::c_longlong,
    pub stat_badcrc: libc::c_longlong,
    pub stat_fixed: libc::c_longlong,
    pub stat_single_bit_fix: libc::c_longlong,
    pub stat_two_bits_fix: libc::c_longlong,
    pub stat_http_requests: libc::c_longlong,
    pub stat_sbs_connections: libc::c_longlong,
    pub stat_out_of_phase: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modesMessage {
    pub msg: [libc::c_uchar; 14],
    pub msgbits: libc::c_int,
    pub msgtype: libc::c_int,
    pub crcok: libc::c_int,
    pub crc: uint32_t,
    pub errorbit: libc::c_int,
    pub aa1: libc::c_int,
    pub aa2: libc::c_int,
    pub aa3: libc::c_int,
    pub phase_corrected: libc::c_int,
    pub ca: libc::c_int,
    pub metype: libc::c_int,
    pub mesub: libc::c_int,
    pub heading_is_valid: libc::c_int,
    pub heading: libc::c_int,
    pub aircraft_type: libc::c_int,
    pub fflag: libc::c_int,
    pub tflag: libc::c_int,
    pub raw_latitude: libc::c_int,
    pub raw_longitude: libc::c_int,
    pub flight: [libc::c_char; 9],
    pub ew_dir: libc::c_int,
    pub ew_velocity: libc::c_int,
    pub ns_dir: libc::c_int,
    pub ns_velocity: libc::c_int,
    pub vert_rate_source: libc::c_int,
    pub vert_rate_sign: libc::c_int,
    pub vert_rate: libc::c_int,
    pub velocity: libc::c_int,
    pub fs: libc::c_int,
    pub dr: libc::c_int,
    pub um: libc::c_int,
    pub identity: libc::c_int,
    pub altitude: libc::c_int,
    pub unit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub descr: *mut libc::c_char,
    pub socket: *mut libc::c_int,
    pub port: libc::c_int,
}
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut Modes: C2RustUnnamed_4 = C2RustUnnamed_4 {
    reader_thread: 0,
    data_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
    data_cond: pthread_cond_t {
        __data: __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 { __wseq: 0 },
            c2rust_unnamed_0: C2RustUnnamed { __g1_start: 0 },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        },
    },
    data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    magnitude: 0 as *const uint16_t as *mut uint16_t,
    data_len: 0,
    fd: 0,
    data_ready: 0,
    icao_cache: 0 as *const uint32_t as *mut uint32_t,
    maglut: 0 as *const uint16_t as *mut uint16_t,
    exit: 0,
    dev_index: 0,
    gain: 0,
    enable_agc: 0,
    dev: 0 as *const rtlsdr_dev_t as *mut rtlsdr_dev_t,
    freq: 0,
    aneterr: [0; 256],
    clients: [0 as *const client as *mut client; 1024],
    maxfd: 0,
    sbsos: 0,
    ros: 0,
    ris: 0,
    https: 0,
    filename: 0 as *const libc::c_char as *mut libc::c_char,
    loop_0: 0,
    fix_errors: 0,
    check_crc: 0,
    raw: 0,
    debug: 0,
    net: 0,
    net_only: 0,
    interactive: 0,
    interactive_rows: 0,
    interactive_ttl: 0,
    stats: 0,
    onlyaddr: 0,
    metric: 0,
    aggressive: 0,
    aircrafts: 0 as *const aircraft as *mut aircraft,
    interactive_last_update: 0,
    stat_valid_preamble: 0,
    stat_demodulated: 0,
    stat_goodcrc: 0,
    stat_badcrc: 0,
    stat_fixed: 0,
    stat_single_bit_fix: 0,
    stat_two_bits_fix: 0,
    stat_http_requests: 0,
    stat_sbs_connections: 0,
    stat_out_of_phase: 0,
};
unsafe extern "C" fn mstime() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut mst: libc::c_longlong = 0;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    mst = tv.tv_sec as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong;
    mst += (tv.tv_usec / 1000 as libc::c_int as libc::c_long) as libc::c_longlong;
    return mst;
}
pub unsafe extern "C" fn modesInitConfig() {
    Modes.gain = 999999 as libc::c_int;
    Modes.dev_index = 0 as libc::c_int;
    Modes.enable_agc = 0 as libc::c_int;
    Modes.freq = 1090000000 as libc::c_int;
    Modes.filename = 0 as *mut libc::c_char;
    Modes.fix_errors = 1 as libc::c_int;
    Modes.check_crc = 1 as libc::c_int;
    Modes.raw = 0 as libc::c_int;
    Modes.net = 0 as libc::c_int;
    Modes.net_only = 0 as libc::c_int;
    Modes.onlyaddr = 0 as libc::c_int;
    Modes.debug = 0 as libc::c_int;
    Modes.interactive = 0 as libc::c_int;
    Modes.interactive_rows = 15 as libc::c_int;
    Modes.interactive_ttl = 60 as libc::c_int;
    Modes.aggressive = 0 as libc::c_int;
    Modes.interactive_rows = getTermRows();
    Modes.loop_0 = 0 as libc::c_int;
}
pub unsafe extern "C" fn modesInit() {
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    pthread_mutex_init(&mut Modes.data_mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut Modes.data_cond, 0 as *const pthread_condattr_t);
    Modes
        .data_len = (16 as libc::c_int * 16384 as libc::c_int
        + (8 as libc::c_int + 112 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int)
        as uint32_t;
    Modes.data_ready = 0 as libc::c_int;
    Modes
        .icao_cache = malloc(
        (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut uint32_t;
    memset(
        Modes.icao_cache as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    );
    Modes.aircrafts = 0 as *mut aircraft;
    Modes.interactive_last_update = 0 as libc::c_int as libc::c_longlong;
    Modes.data = malloc(Modes.data_len as libc::c_ulong) as *mut libc::c_uchar;
    if (Modes.data).is_null()
        || {
            Modes
                .magnitude = malloc(
                (Modes.data_len).wrapping_mul(2 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            ) as *mut uint16_t;
            (Modes.magnitude).is_null()
        }
    {
        fprintf(
            stderr,
            b"Out of memory allocating data buffer.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    memset(
        Modes.data as *mut libc::c_void,
        127 as libc::c_int,
        Modes.data_len as libc::c_ulong,
    );
    Modes
        .maglut = malloc(
        (129 as libc::c_int * 129 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
    ) as *mut uint16_t;
    i = 0 as libc::c_int;
    while i <= 128 as libc::c_int {
        q = 0 as libc::c_int;
        while q <= 128 as libc::c_int {
            *(Modes.maglut)
                .offset(
                    (i * 129 as libc::c_int + q) as isize,
                ) = round(
                sqrt((i * i + q * q) as libc::c_double)
                    * 360 as libc::c_int as libc::c_double,
            ) as uint16_t;
            q += 1;
            q;
        }
        i += 1;
        i;
    }
    Modes.stat_valid_preamble = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_demodulated = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_goodcrc = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_badcrc = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_fixed = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_single_bit_fix = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_two_bits_fix = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_http_requests = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_sbs_connections = 0 as libc::c_int as libc::c_longlong;
    Modes.stat_out_of_phase = 0 as libc::c_int as libc::c_longlong;
    Modes.exit = 0 as libc::c_int;
}
pub unsafe extern "C" fn modesInitRTLSDR() {
    let mut j: libc::c_int = 0;
    let mut device_count: libc::c_int = 0;
    let mut ppm_error: libc::c_int = 0 as libc::c_int;
    let mut vendor: [libc::c_char; 256] = [0; 256];
    let mut product: [libc::c_char; 256] = [0; 256];
    let mut serial: [libc::c_char; 256] = [0; 256];
    device_count = rtlsdr_get_device_count() as libc::c_int;
    if device_count == 0 {
        fprintf(
            stderr,
            b"No supported RTLSDR devices found.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"Found %d device(s):\n\0" as *const u8 as *const libc::c_char,
        device_count,
    );
    j = 0 as libc::c_int;
    while j < device_count {
        rtlsdr_get_device_usb_strings(
            j as uint32_t,
            vendor.as_mut_ptr(),
            product.as_mut_ptr(),
            serial.as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"%d: %s, %s, SN: %s %s\n\0" as *const u8 as *const libc::c_char,
            j,
            vendor.as_mut_ptr(),
            product.as_mut_ptr(),
            serial.as_mut_ptr(),
            if j == Modes.dev_index {
                b"(currently selected)\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        j += 1;
        j;
    }
    if rtlsdr_open(&mut Modes.dev, Modes.dev_index as uint32_t) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Error opening the RTLSDR device: %s\n\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    rtlsdr_set_tuner_gain_mode(
        Modes.dev,
        if Modes.gain == -(100 as libc::c_int) {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        },
    );
    if Modes.gain != -(100 as libc::c_int) {
        if Modes.gain == 999999 as libc::c_int {
            let mut numgains: libc::c_int = 0;
            let mut gains: [libc::c_int; 100] = [0; 100];
            numgains = rtlsdr_get_tuner_gains(Modes.dev, gains.as_mut_ptr());
            Modes.gain = gains[(numgains - 1 as libc::c_int) as usize];
            fprintf(
                stderr,
                b"Max available gain is: %.2f\n\0" as *const u8 as *const libc::c_char,
                Modes.gain as libc::c_double / 10.0f64,
            );
        }
        rtlsdr_set_tuner_gain(Modes.dev, Modes.gain);
        fprintf(
            stderr,
            b"Setting gain to: %.2f\n\0" as *const u8 as *const libc::c_char,
            Modes.gain as libc::c_double / 10.0f64,
        );
    } else {
        fprintf(
            stderr,
            b"Using automatic gain control.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    rtlsdr_set_freq_correction(Modes.dev, ppm_error);
    if Modes.enable_agc != 0 {
        rtlsdr_set_agc_mode(Modes.dev, 1 as libc::c_int);
    }
    rtlsdr_set_center_freq(Modes.dev, Modes.freq as uint32_t);
    rtlsdr_set_sample_rate(Modes.dev, 2000000 as libc::c_int as uint32_t);
    rtlsdr_reset_buffer(Modes.dev);
    fprintf(
        stderr,
        b"Gain reported by device: %.2f\n\0" as *const u8 as *const libc::c_char,
        rtlsdr_get_tuner_gain(Modes.dev) as libc::c_double / 10.0f64,
    );
}
pub unsafe extern "C" fn rtlsdrCallback(
    mut buf: *mut libc::c_uchar,
    mut len: uint32_t,
    mut ctx: *mut libc::c_void,
) {
    pthread_mutex_lock(&mut Modes.data_mutex);
    if len > (16 as libc::c_int * 16384 as libc::c_int) as libc::c_uint {
        len = (16 as libc::c_int * 16384 as libc::c_int) as uint32_t;
    }
    memcpy(
        Modes.data as *mut libc::c_void,
        (Modes.data).offset((16 as libc::c_int * 16384 as libc::c_int) as isize)
            as *const libc::c_void,
        ((8 as libc::c_int + 112 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int)
            as libc::c_ulong,
    );
    memcpy(
        (Modes.data)
            .offset(
                ((8 as libc::c_int + 112 as libc::c_int - 1 as libc::c_int)
                    * 4 as libc::c_int) as isize,
            ) as *mut libc::c_void,
        buf as *const libc::c_void,
        len as libc::c_ulong,
    );
    Modes.data_ready = 1 as libc::c_int;
    pthread_cond_signal(&mut Modes.data_cond);
    pthread_mutex_unlock(&mut Modes.data_mutex);
}
pub unsafe extern "C" fn readDataFromFile() {
    pthread_mutex_lock(&mut Modes.data_mutex);
    loop {
        let mut nread: ssize_t = 0;
        let mut toread: ssize_t = 0;
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if Modes.data_ready != 0 {
            pthread_cond_wait(&mut Modes.data_cond, &mut Modes.data_mutex);
        } else {
            if Modes.interactive != 0 {
                pthread_mutex_unlock(&mut Modes.data_mutex);
                usleep(5000 as libc::c_int as __useconds_t);
                pthread_mutex_lock(&mut Modes.data_mutex);
            }
            memcpy(
                Modes.data as *mut libc::c_void,
                (Modes.data).offset((16 as libc::c_int * 16384 as libc::c_int) as isize)
                    as *const libc::c_void,
                ((8 as libc::c_int + 112 as libc::c_int - 1 as libc::c_int)
                    * 4 as libc::c_int) as libc::c_ulong,
            );
            toread = (16 as libc::c_int * 16384 as libc::c_int) as ssize_t;
            p = (Modes.data)
                .offset(
                    ((8 as libc::c_int + 112 as libc::c_int - 1 as libc::c_int)
                        * 4 as libc::c_int) as isize,
                );
            while toread != 0 {
                nread = read(Modes.fd, p as *mut libc::c_void, toread as size_t);
                if nread == 0 as libc::c_int as libc::c_long
                    && !(Modes.filename).is_null() && Modes.fd != 0 as libc::c_int
                    && Modes.loop_0 != 0
                {
                    if lseek(Modes.fd, 0 as libc::c_int as __off_t, 0 as libc::c_int)
                        != -(1 as libc::c_int) as libc::c_long
                    {
                        continue;
                    }
                }
                if nread <= 0 as libc::c_int as libc::c_long {
                    Modes.exit = 1 as libc::c_int;
                    break;
                } else {
                    p = p.offset(nread as isize);
                    toread -= nread;
                }
            }
            if toread != 0 {
                memset(
                    p as *mut libc::c_void,
                    127 as libc::c_int,
                    toread as libc::c_ulong,
                );
            }
            Modes.data_ready = 1 as libc::c_int;
            pthread_cond_signal(&mut Modes.data_cond);
        }
    };
}
pub unsafe extern "C" fn readerThreadEntryPoint(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    if (Modes.filename).is_null() {
        rtlsdr_read_async(
            Modes.dev,
            Some(
                rtlsdrCallback
                    as unsafe extern "C" fn(
                        *mut libc::c_uchar,
                        uint32_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
            12 as libc::c_int as uint32_t,
            (16 as libc::c_int * 16384 as libc::c_int) as uint32_t,
        );
    } else {
        readDataFromFile();
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn dumpMagnitudeBar(
    mut index: libc::c_int,
    mut magnitude: libc::c_int,
) {
    let mut set: *mut libc::c_char = b" .-o\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut div: libc::c_int = magnitude / 256 as libc::c_int / 4 as libc::c_int;
    let mut rem: libc::c_int = magnitude / 256 as libc::c_int % 4 as libc::c_int;
    memset(buf.as_mut_ptr() as *mut libc::c_void, 'O' as i32, div as libc::c_ulong);
    buf[div as usize] = *set.offset(rem as isize);
    buf[(div + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    if index >= 0 as libc::c_int {
        let mut markchar: libc::c_int = ']' as i32;
        if index == 0 as libc::c_int || index == 2 as libc::c_int
            || index == 7 as libc::c_int || index == 9 as libc::c_int
        {
            markchar = '>' as i32;
        }
        if index >= 16 as libc::c_int {
            markchar = if (index - 16 as libc::c_int) / 2 as libc::c_int
                & 1 as libc::c_int != 0
            {
                '|' as i32
            } else {
                ')' as i32
            };
        }
        printf(
            b"[%.3d%c |%-66s %d\n\0" as *const u8 as *const libc::c_char,
            index,
            markchar,
            buf.as_mut_ptr(),
            magnitude,
        );
    } else {
        printf(
            b"[%.2d] |%-66s %d\n\0" as *const u8 as *const libc::c_char,
            index,
            buf.as_mut_ptr(),
            magnitude,
        );
    };
}
pub unsafe extern "C" fn dumpMagnitudeVector(
    mut m: *mut uint16_t,
    mut offset: uint32_t,
) {
    let mut padding: uint32_t = 5 as libc::c_int as uint32_t;
    let mut start: uint32_t = if offset < padding {
        0 as libc::c_int as libc::c_uint
    } else {
        offset.wrapping_sub(padding)
    };
    let mut end: uint32_t = offset
        .wrapping_add((8 as libc::c_int * 2 as libc::c_int) as libc::c_uint)
        .wrapping_add((56 as libc::c_int * 2 as libc::c_int) as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut j: uint32_t = 0;
    j = start;
    while j <= end {
        dumpMagnitudeBar(
            j.wrapping_sub(offset) as libc::c_int,
            *m.offset(j as isize) as libc::c_int,
        );
        j = j.wrapping_add(1);
        j;
    }
}
pub unsafe extern "C" fn dumpRawMessageJS(
    mut descr: *mut libc::c_char,
    mut msg: *mut libc::c_uchar,
    mut m: *mut uint16_t,
    mut offset: uint32_t,
    mut fixable: libc::c_int,
) {
    let mut padding: libc::c_int = 5 as libc::c_int;
    let mut start: libc::c_int = offset.wrapping_sub(padding as libc::c_uint)
        as libc::c_int;
    let mut end: libc::c_int = offset
        .wrapping_add((8 as libc::c_int * 2 as libc::c_int) as libc::c_uint)
        .wrapping_add((112 as libc::c_int * 2 as libc::c_int) as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut j: libc::c_int = 0;
    let mut fix1: libc::c_int = -(1 as libc::c_int);
    let mut fix2: libc::c_int = -(1 as libc::c_int);
    if fixable != -(1 as libc::c_int) {
        fix1 = fixable & 0xff as libc::c_int;
        if fixable > 255 as libc::c_int {
            fix2 = fixable >> 8 as libc::c_int;
        }
    }
    fp = fopen(
        b"frames.js\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        fprintf(
            stderr,
            b"Error opening frames.js: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        fp,
        b"frames.push({\"descr\": \"%s\", \"mag\": [\0" as *const u8
            as *const libc::c_char,
        descr,
    );
    j = start;
    while j <= end {
        fprintf(
            fp,
            b"%d\0" as *const u8 as *const libc::c_char,
            if j < 0 as libc::c_int {
                0 as libc::c_int
            } else {
                *m.offset(j as isize) as libc::c_int
            },
        );
        if j != end {
            fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
        }
        j += 1;
        j;
    }
    fprintf(
        fp,
        b"], \"fix1\": %d, \"fix2\": %d, \"bits\": %d, \"hex\": \"\0" as *const u8
            as *const libc::c_char,
        fix1,
        fix2,
        modesMessageLenByType(
            *msg.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int,
        ),
    );
    j = 0 as libc::c_int;
    while j < 112 as libc::c_int / 8 as libc::c_int {
        fprintf(
            fp,
            b"\\x%02x\0" as *const u8 as *const libc::c_char,
            *msg.offset(j as isize) as libc::c_int,
        );
        j += 1;
        j;
    }
    fprintf(fp, b"\"});\n\0" as *const u8 as *const libc::c_char);
    fclose(fp);
}
pub unsafe extern "C" fn dumpRawMessage(
    mut descr: *mut libc::c_char,
    mut msg: *mut libc::c_uchar,
    mut m: *mut uint16_t,
    mut offset: uint32_t,
) {
    let mut j: libc::c_int = 0;
    let mut msgtype: libc::c_int = *msg.offset(0 as libc::c_int as isize) as libc::c_int
        >> 3 as libc::c_int;
    let mut fixable: libc::c_int = -(1 as libc::c_int);
    if msgtype == 11 as libc::c_int || msgtype == 17 as libc::c_int {
        let mut msgbits: libc::c_int = if msgtype == 11 as libc::c_int {
            56 as libc::c_int
        } else {
            112 as libc::c_int
        };
        fixable = fixSingleBitErrors(msg, msgbits);
        if fixable == -(1 as libc::c_int) {
            fixable = fixTwoBitsErrors(msg, msgbits);
        }
    }
    if Modes.debug & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        dumpRawMessageJS(descr, msg, m, offset, fixable);
        return;
    }
    printf(b"\n--- %s\n    \0" as *const u8 as *const libc::c_char, descr);
    j = 0 as libc::c_int;
    while j < 112 as libc::c_int / 8 as libc::c_int {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            *msg.offset(j as isize) as libc::c_int,
        );
        if j == 56 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int {
            printf(b" ... \0" as *const u8 as *const libc::c_char);
        }
        j += 1;
        j;
    }
    printf(
        b" (DF %d, Fixable: %d)\n\0" as *const u8 as *const libc::c_char,
        msgtype,
        fixable,
    );
    dumpMagnitudeVector(m, offset);
    printf(b"---\n\n\0" as *const u8 as *const libc::c_char);
}
pub static mut modes_checksum_table: [uint32_t; 112] = [
    0x3935ea as libc::c_int as uint32_t,
    0x1c9af5 as libc::c_int as uint32_t,
    0xf1b77e as libc::c_int as uint32_t,
    0x78dbbf as libc::c_int as uint32_t,
    0xc397db as libc::c_int as uint32_t,
    0x9e31e9 as libc::c_int as uint32_t,
    0xb0e2f0 as libc::c_int as uint32_t,
    0x587178 as libc::c_int as uint32_t,
    0x2c38bc as libc::c_int as uint32_t,
    0x161c5e as libc::c_int as uint32_t,
    0xb0e2f as libc::c_int as uint32_t,
    0xfa7d13 as libc::c_int as uint32_t,
    0x82c48d as libc::c_int as uint32_t,
    0xbe9842 as libc::c_int as uint32_t,
    0x5f4c21 as libc::c_int as uint32_t,
    0xd05c14 as libc::c_int as uint32_t,
    0x682e0a as libc::c_int as uint32_t,
    0x341705 as libc::c_int as uint32_t,
    0xe5f186 as libc::c_int as uint32_t,
    0x72f8c3 as libc::c_int as uint32_t,
    0xc68665 as libc::c_int as uint32_t,
    0x9cb936 as libc::c_int as uint32_t,
    0x4e5c9b as libc::c_int as uint32_t,
    0xd8d449 as libc::c_int as uint32_t,
    0x939020 as libc::c_int as uint32_t,
    0x49c810 as libc::c_int as uint32_t,
    0x24e408 as libc::c_int as uint32_t,
    0x127204 as libc::c_int as uint32_t,
    0x93902 as libc::c_int as uint32_t,
    0x49c81 as libc::c_int as uint32_t,
    0xfdb444 as libc::c_int as uint32_t,
    0x7eda22 as libc::c_int as uint32_t,
    0x3f6d11 as libc::c_int as uint32_t,
    0xe04c8c as libc::c_int as uint32_t,
    0x702646 as libc::c_int as uint32_t,
    0x381323 as libc::c_int as uint32_t,
    0xe3f395 as libc::c_int as uint32_t,
    0x8e03ce as libc::c_int as uint32_t,
    0x4701e7 as libc::c_int as uint32_t,
    0xdc7af7 as libc::c_int as uint32_t,
    0x91c77f as libc::c_int as uint32_t,
    0xb719bb as libc::c_int as uint32_t,
    0xa476d9 as libc::c_int as uint32_t,
    0xadc168 as libc::c_int as uint32_t,
    0x56e0b4 as libc::c_int as uint32_t,
    0x2b705a as libc::c_int as uint32_t,
    0x15b82d as libc::c_int as uint32_t,
    0xf52612 as libc::c_int as uint32_t,
    0x7a9309 as libc::c_int as uint32_t,
    0xc2b380 as libc::c_int as uint32_t,
    0x6159c0 as libc::c_int as uint32_t,
    0x30ace0 as libc::c_int as uint32_t,
    0x185670 as libc::c_int as uint32_t,
    0xc2b38 as libc::c_int as uint32_t,
    0x6159c as libc::c_int as uint32_t,
    0x30ace as libc::c_int as uint32_t,
    0x18567 as libc::c_int as uint32_t,
    0xff38b7 as libc::c_int as uint32_t,
    0x80665f as libc::c_int as uint32_t,
    0xbfc92b as libc::c_int as uint32_t,
    0xa01e91 as libc::c_int as uint32_t,
    0xaff54c as libc::c_int as uint32_t,
    0x57faa6 as libc::c_int as uint32_t,
    0x2bfd53 as libc::c_int as uint32_t,
    0xea04ad as libc::c_int as uint32_t,
    0x8af852 as libc::c_int as uint32_t,
    0x457c29 as libc::c_int as uint32_t,
    0xdd4410 as libc::c_int as uint32_t,
    0x6ea208 as libc::c_int as uint32_t,
    0x375104 as libc::c_int as uint32_t,
    0x1ba882 as libc::c_int as uint32_t,
    0xdd441 as libc::c_int as uint32_t,
    0xf91024 as libc::c_int as uint32_t,
    0x7c8812 as libc::c_int as uint32_t,
    0x3e4409 as libc::c_int as uint32_t,
    0xe0d800 as libc::c_int as uint32_t,
    0x706c00 as libc::c_int as uint32_t,
    0x383600 as libc::c_int as uint32_t,
    0x1c1b00 as libc::c_int as uint32_t,
    0xe0d80 as libc::c_int as uint32_t,
    0x706c0 as libc::c_int as uint32_t,
    0x38360 as libc::c_int as uint32_t,
    0x1c1b0 as libc::c_int as uint32_t,
    0xe0d8 as libc::c_int as uint32_t,
    0x706c as libc::c_int as uint32_t,
    0x3836 as libc::c_int as uint32_t,
    0x1c1b as libc::c_int as uint32_t,
    0xfff409 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
];
pub unsafe extern "C" fn modesChecksum(
    mut msg: *mut libc::c_uchar,
    mut bits: libc::c_int,
) -> uint32_t {
    let mut crc: uint32_t = 0 as libc::c_int as uint32_t;
    let mut offset: libc::c_int = if bits == 112 as libc::c_int {
        0 as libc::c_int
    } else {
        112 as libc::c_int - 56 as libc::c_int
    };
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < bits {
        let mut byte: libc::c_int = j / 8 as libc::c_int;
        let mut bit: libc::c_int = j % 8 as libc::c_int;
        let mut bitmask: libc::c_int = (1 as libc::c_int) << 7 as libc::c_int - bit;
        if *msg.offset(byte as isize) as libc::c_int & bitmask != 0 {
            crc ^= modes_checksum_table[(j + offset) as usize];
        }
        j += 1;
        j;
    }
    return crc;
}
pub unsafe extern "C" fn modesMessageLenByType(mut type_0: libc::c_int) -> libc::c_int {
    if type_0 == 16 as libc::c_int || type_0 == 17 as libc::c_int
        || type_0 == 19 as libc::c_int || type_0 == 20 as libc::c_int
        || type_0 == 21 as libc::c_int
    {
        return 112 as libc::c_int
    } else {
        return 56 as libc::c_int
    };
}
pub unsafe extern "C" fn fixSingleBitErrors(
    mut msg: *mut libc::c_uchar,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut aux: [libc::c_uchar; 14] = [0; 14];
    j = 0 as libc::c_int;
    while j < bits {
        let mut byte: libc::c_int = j / 8 as libc::c_int;
        let mut bitmask: libc::c_int = (1 as libc::c_int)
            << 7 as libc::c_int - j % 8 as libc::c_int;
        let mut crc1: uint32_t = 0;
        let mut crc2: uint32_t = 0;
        memcpy(
            aux.as_mut_ptr() as *mut libc::c_void,
            msg as *const libc::c_void,
            (bits / 8 as libc::c_int) as libc::c_ulong,
        );
        aux[byte
            as usize] = (aux[byte as usize] as libc::c_int ^ bitmask) as libc::c_uchar;
        crc1 = (aux[(bits / 8 as libc::c_int - 3 as libc::c_int) as usize] as uint32_t)
            << 16 as libc::c_int
            | (aux[(bits / 8 as libc::c_int - 2 as libc::c_int) as usize] as uint32_t)
                << 8 as libc::c_int
            | aux[(bits / 8 as libc::c_int - 1 as libc::c_int) as usize] as uint32_t;
        crc2 = modesChecksum(aux.as_mut_ptr(), bits);
        if crc1 == crc2 {
            memcpy(
                msg as *mut libc::c_void,
                aux.as_mut_ptr() as *const libc::c_void,
                (bits / 8 as libc::c_int) as libc::c_ulong,
            );
            return j;
        }
        j += 1;
        j;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn fixTwoBitsErrors(
    mut msg: *mut libc::c_uchar,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut aux: [libc::c_uchar; 14] = [0; 14];
    j = 0 as libc::c_int;
    while j < bits {
        let mut byte1: libc::c_int = j / 8 as libc::c_int;
        let mut bitmask1: libc::c_int = (1 as libc::c_int)
            << 7 as libc::c_int - j % 8 as libc::c_int;
        i = j + 1 as libc::c_int;
        while i < bits {
            let mut byte2: libc::c_int = i / 8 as libc::c_int;
            let mut bitmask2: libc::c_int = (1 as libc::c_int)
                << 7 as libc::c_int - i % 8 as libc::c_int;
            let mut crc1: uint32_t = 0;
            let mut crc2: uint32_t = 0;
            memcpy(
                aux.as_mut_ptr() as *mut libc::c_void,
                msg as *const libc::c_void,
                (bits / 8 as libc::c_int) as libc::c_ulong,
            );
            aux[byte1
                as usize] = (aux[byte1 as usize] as libc::c_int ^ bitmask1)
                as libc::c_uchar;
            aux[byte2
                as usize] = (aux[byte2 as usize] as libc::c_int ^ bitmask2)
                as libc::c_uchar;
            crc1 = (aux[(bits / 8 as libc::c_int - 3 as libc::c_int) as usize]
                as uint32_t) << 16 as libc::c_int
                | (aux[(bits / 8 as libc::c_int - 2 as libc::c_int) as usize]
                    as uint32_t) << 8 as libc::c_int
                | aux[(bits / 8 as libc::c_int - 1 as libc::c_int) as usize] as uint32_t;
            crc2 = modesChecksum(aux.as_mut_ptr(), bits);
            if crc1 == crc2 {
                memcpy(
                    msg as *mut libc::c_void,
                    aux.as_mut_ptr() as *const libc::c_void,
                    (bits / 8 as libc::c_int) as libc::c_ulong,
                );
                return j | i << 8 as libc::c_int;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn ICAOCacheHashAddress(mut a: uint32_t) -> uint32_t {
    a = (a >> 16 as libc::c_int ^ a)
        .wrapping_mul(0x45d9f3b as libc::c_int as libc::c_uint);
    a = (a >> 16 as libc::c_int ^ a)
        .wrapping_mul(0x45d9f3b as libc::c_int as libc::c_uint);
    a = a >> 16 as libc::c_int ^ a;
    return a & (1024 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
}
pub unsafe extern "C" fn addRecentlySeenICAOAddr(mut addr: uint32_t) {
    let mut h: uint32_t = ICAOCacheHashAddress(addr);
    *(Modes.icao_cache)
        .offset(h.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize) = addr;
    *(Modes.icao_cache)
        .offset(
            h
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = time(0 as *mut time_t) as uint32_t;
}
pub unsafe extern "C" fn ICAOAddressWasRecentlySeen(mut addr: uint32_t) -> libc::c_int {
    let mut h: uint32_t = ICAOCacheHashAddress(addr);
    let mut a: uint32_t = *(Modes.icao_cache)
        .offset(h.wrapping_mul(2 as libc::c_int as libc::c_uint) as isize);
    let mut t: uint32_t = *(Modes.icao_cache)
        .offset(
            h
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    return (a != 0 && a == addr
        && time(0 as *mut time_t) - t as libc::c_long
            <= 60 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn bruteForceAP(
    mut msg: *mut libc::c_uchar,
    mut mm: *mut modesMessage,
) -> libc::c_int {
    let mut aux: [libc::c_uchar; 14] = [0; 14];
    let mut msgtype: libc::c_int = (*mm).msgtype;
    let mut msgbits: libc::c_int = (*mm).msgbits;
    if msgtype == 0 as libc::c_int || msgtype == 4 as libc::c_int
        || msgtype == 5 as libc::c_int || msgtype == 16 as libc::c_int
        || msgtype == 20 as libc::c_int || msgtype == 21 as libc::c_int
        || msgtype == 24 as libc::c_int
    {
        let mut addr: uint32_t = 0;
        let mut crc: uint32_t = 0;
        let mut lastbyte: libc::c_int = msgbits / 8 as libc::c_int - 1 as libc::c_int;
        memcpy(
            aux.as_mut_ptr() as *mut libc::c_void,
            msg as *const libc::c_void,
            (msgbits / 8 as libc::c_int) as libc::c_ulong,
        );
        crc = modesChecksum(aux.as_mut_ptr(), msgbits);
        aux[lastbyte
            as usize] = (aux[lastbyte as usize] as libc::c_uint
            ^ crc & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        aux[(lastbyte - 1 as libc::c_int)
            as usize] = (aux[(lastbyte - 1 as libc::c_int) as usize] as libc::c_uint
            ^ crc >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        aux[(lastbyte - 2 as libc::c_int)
            as usize] = (aux[(lastbyte - 2 as libc::c_int) as usize] as libc::c_uint
            ^ crc >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        addr = (aux[lastbyte as usize] as libc::c_int
            | (aux[(lastbyte - 1 as libc::c_int) as usize] as libc::c_int)
                << 8 as libc::c_int
            | (aux[(lastbyte - 2 as libc::c_int) as usize] as libc::c_int)
                << 16 as libc::c_int) as uint32_t;
        if ICAOAddressWasRecentlySeen(addr) != 0 {
            (*mm).aa1 = aux[(lastbyte - 2 as libc::c_int) as usize] as libc::c_int;
            (*mm).aa2 = aux[(lastbyte - 1 as libc::c_int) as usize] as libc::c_int;
            (*mm).aa3 = aux[lastbyte as usize] as libc::c_int;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn decodeAC13Field(
    mut msg: *mut libc::c_uchar,
    mut unit: *mut libc::c_int,
) -> libc::c_int {
    let mut m_bit: libc::c_int = *msg.offset(3 as libc::c_int as isize) as libc::c_int
        & (1 as libc::c_int) << 6 as libc::c_int;
    let mut q_bit: libc::c_int = *msg.offset(3 as libc::c_int as isize) as libc::c_int
        & (1 as libc::c_int) << 4 as libc::c_int;
    if m_bit == 0 {
        *unit = 0 as libc::c_int;
        if q_bit != 0 {
            let mut n: libc::c_int = (*msg.offset(2 as libc::c_int as isize)
                as libc::c_int & 31 as libc::c_int) << 6 as libc::c_int
                | (*msg.offset(3 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int) >> 2 as libc::c_int
                | (*msg.offset(3 as libc::c_int as isize) as libc::c_int
                    & 0x20 as libc::c_int) >> 1 as libc::c_int
                | *msg.offset(3 as libc::c_int as isize) as libc::c_int
                    & 15 as libc::c_int;
            return n * 25 as libc::c_int - 1000 as libc::c_int;
        }
    } else {
        *unit = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn decodeAC12Field(
    mut msg: *mut libc::c_uchar,
    mut unit: *mut libc::c_int,
) -> libc::c_int {
    let mut q_bit: libc::c_int = *msg.offset(5 as libc::c_int as isize) as libc::c_int
        & 1 as libc::c_int;
    if q_bit != 0 {
        *unit = 0 as libc::c_int;
        let mut n: libc::c_int = (*msg.offset(5 as libc::c_int as isize) as libc::c_int
            >> 1 as libc::c_int) << 4 as libc::c_int
            | (*msg.offset(6 as libc::c_int as isize) as libc::c_int
                & 0xf0 as libc::c_int) >> 4 as libc::c_int;
        return n * 25 as libc::c_int - 1000 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub static mut ca_str: [*mut libc::c_char; 8] = [
    b"Level 1 (Survillance Only)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 2 (DF0,4,5,11)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Level 3 (DF0,4,5,11,20,21)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 4 (DF0,4,5,11,20,21,24)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 2+3+4 (DF0,4,5,11,20,21,24,code7 - is on ground)\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Level 2+3+4 (DF0,4,5,11,20,21,24,code7 - is on airborne)\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Level 2+3+4 (DF0,4,5,11,20,21,24,code7)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Level 7 ???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut fs_str: [*mut libc::c_char; 8] = [
    b"Normal, Airborne\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Normal, On the ground\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALERT,  Airborne\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALERT,  On the ground\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ALERT & Special Position Identification. Airborne or Ground\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Special Position Identification. Airborne or Ground\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Value 6 is not assigned\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Value 7 is not assigned\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut me_str: [*mut libc::c_char; 0] = [];
pub unsafe extern "C" fn getMEDescription(
    mut metype: libc::c_int,
    mut mesub: libc::c_int,
) -> *mut libc::c_char {
    let mut mename: *mut libc::c_char = b"Unknown\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    if metype >= 1 as libc::c_int && metype <= 4 as libc::c_int {
        mename = b"Aircraft Identification and Category\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
    } else if metype >= 5 as libc::c_int && metype <= 8 as libc::c_int {
        mename = b"Surface Position\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if metype >= 9 as libc::c_int && metype <= 18 as libc::c_int {
        mename = b"Airborne Position (Baro Altitude)\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
    } else if metype == 19 as libc::c_int && mesub >= 1 as libc::c_int
        && mesub <= 4 as libc::c_int
    {
        mename = b"Airborne Velocity\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if metype >= 20 as libc::c_int && metype <= 22 as libc::c_int {
        mename = b"Airborne Position (GNSS Height)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if metype == 23 as libc::c_int && mesub == 0 as libc::c_int {
        mename = b"Test Message\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if metype == 24 as libc::c_int && mesub == 1 as libc::c_int {
        mename = b"Surface System Status\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if metype == 28 as libc::c_int && mesub == 1 as libc::c_int {
        mename = b"Extended Squitter Aircraft Status (Emergency)\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
    } else if metype == 28 as libc::c_int && mesub == 2 as libc::c_int {
        mename = b"Extended Squitter Aircraft Status (1090ES TCAS RA)\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
    } else if metype == 29 as libc::c_int
        && (mesub == 0 as libc::c_int || mesub == 1 as libc::c_int)
    {
        mename = b"Target State and Status Message\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if metype == 31 as libc::c_int
        && (mesub == 0 as libc::c_int || mesub == 1 as libc::c_int)
    {
        mename = b"Aircraft Operational Status Message\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
    }
    return mename;
}
pub unsafe extern "C" fn decodeModesMessage(
    mut mm: *mut modesMessage,
    mut msg: *mut libc::c_uchar,
) {
    let mut crc2: uint32_t = 0;
    let mut ais_charset: *mut libc::c_char = b"?ABCDEFGHIJKLMNOPQRSTUVWXYZ????? ???????????????0123456789??????\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    memcpy(
        ((*mm).msg).as_mut_ptr() as *mut libc::c_void,
        msg as *const libc::c_void,
        (112 as libc::c_int / 8 as libc::c_int) as libc::c_ulong,
    );
    msg = ((*mm).msg).as_mut_ptr();
    (*mm)
        .msgtype = *msg.offset(0 as libc::c_int as isize) as libc::c_int
        >> 3 as libc::c_int;
    (*mm).msgbits = modesMessageLenByType((*mm).msgtype);
    (*mm)
        .crc = (*msg
        .offset(((*mm).msgbits / 8 as libc::c_int - 3 as libc::c_int) as isize)
        as uint32_t) << 16 as libc::c_int
        | (*msg.offset(((*mm).msgbits / 8 as libc::c_int - 2 as libc::c_int) as isize)
            as uint32_t) << 8 as libc::c_int
        | *msg.offset(((*mm).msgbits / 8 as libc::c_int - 1 as libc::c_int) as isize)
            as uint32_t;
    crc2 = modesChecksum(msg, (*mm).msgbits);
    (*mm).errorbit = -(1 as libc::c_int);
    (*mm).crcok = ((*mm).crc == crc2) as libc::c_int;
    if (*mm).crcok == 0 && Modes.fix_errors != 0
        && ((*mm).msgtype == 11 as libc::c_int || (*mm).msgtype == 17 as libc::c_int)
    {
        (*mm).errorbit = fixSingleBitErrors(msg, (*mm).msgbits);
        if (*mm).errorbit != -(1 as libc::c_int) {
            (*mm).crc = modesChecksum(msg, (*mm).msgbits);
            (*mm).crcok = 1 as libc::c_int;
        } else if Modes.aggressive != 0 && (*mm).msgtype == 17 as libc::c_int
            && {
                (*mm).errorbit = fixTwoBitsErrors(msg, (*mm).msgbits);
                (*mm).errorbit != -(1 as libc::c_int)
            }
        {
            (*mm).crc = modesChecksum(msg, (*mm).msgbits);
            (*mm).crcok = 1 as libc::c_int;
        }
    }
    (*mm).ca = *msg.offset(0 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int;
    (*mm).aa1 = *msg.offset(1 as libc::c_int as isize) as libc::c_int;
    (*mm).aa2 = *msg.offset(2 as libc::c_int as isize) as libc::c_int;
    (*mm).aa3 = *msg.offset(3 as libc::c_int as isize) as libc::c_int;
    (*mm)
        .metype = *msg.offset(4 as libc::c_int as isize) as libc::c_int
        >> 3 as libc::c_int;
    (*mm)
        .mesub = *msg.offset(4 as libc::c_int as isize) as libc::c_int
        & 7 as libc::c_int;
    (*mm).fs = *msg.offset(0 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int;
    (*mm)
        .dr = *msg.offset(1 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
        & 31 as libc::c_int;
    (*mm)
        .um = (*msg.offset(1 as libc::c_int as isize) as libc::c_int & 7 as libc::c_int)
        << 3 as libc::c_int
        | *msg.offset(2 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    a = (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int)
        >> 5 as libc::c_int
        | (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 0x2 as libc::c_int)
            >> 0 as libc::c_int
        | (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 0x8 as libc::c_int)
            >> 3 as libc::c_int;
    b = (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 0x2 as libc::c_int)
        << 1 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 0x8 as libc::c_int)
            >> 2 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 0x20 as libc::c_int)
            >> 5 as libc::c_int;
    c = (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 0x1 as libc::c_int)
        << 2 as libc::c_int
        | (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 0x4 as libc::c_int)
            >> 1 as libc::c_int
        | (*msg.offset(2 as libc::c_int as isize) as libc::c_int & 0x10 as libc::c_int)
            >> 4 as libc::c_int;
    d = (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 0x1 as libc::c_int)
        << 2 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 0x4 as libc::c_int)
            >> 1 as libc::c_int
        | (*msg.offset(3 as libc::c_int as isize) as libc::c_int & 0x10 as libc::c_int)
            >> 4 as libc::c_int;
    (*mm)
        .identity = a * 1000 as libc::c_int + b * 100 as libc::c_int
        + c * 10 as libc::c_int + d;
    if (*mm).msgtype != 11 as libc::c_int && (*mm).msgtype != 17 as libc::c_int {
        if bruteForceAP(msg, mm) != 0 {
            (*mm).crcok = 1 as libc::c_int;
        } else {
            (*mm).crcok = 0 as libc::c_int;
        }
    } else if (*mm).crcok != 0 && (*mm).errorbit == -(1 as libc::c_int) {
        let mut addr: uint32_t = ((*mm).aa1 << 16 as libc::c_int
            | (*mm).aa2 << 8 as libc::c_int | (*mm).aa3) as uint32_t;
        addRecentlySeenICAOAddr(addr);
    }
    if (*mm).msgtype == 0 as libc::c_int || (*mm).msgtype == 4 as libc::c_int
        || (*mm).msgtype == 16 as libc::c_int || (*mm).msgtype == 20 as libc::c_int
    {
        (*mm).altitude = decodeAC13Field(msg, &mut (*mm).unit);
    }
    if (*mm).msgtype == 17 as libc::c_int {
        if (*mm).metype >= 1 as libc::c_int && (*mm).metype <= 4 as libc::c_int {
            (*mm).aircraft_type = (*mm).metype - 1 as libc::c_int;
            (*mm)
                .flight[0 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    (*msg.offset(5 as libc::c_int as isize) as libc::c_int
                        >> 2 as libc::c_int) as isize,
                );
            (*mm)
                .flight[1 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    ((*msg.offset(5 as libc::c_int as isize) as libc::c_int
                        & 3 as libc::c_int) << 4 as libc::c_int
                        | *msg.offset(6 as libc::c_int as isize) as libc::c_int
                            >> 4 as libc::c_int) as isize,
                );
            (*mm)
                .flight[2 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    ((*msg.offset(6 as libc::c_int as isize) as libc::c_int
                        & 15 as libc::c_int) << 2 as libc::c_int
                        | *msg.offset(7 as libc::c_int as isize) as libc::c_int
                            >> 6 as libc::c_int) as isize,
                );
            (*mm)
                .flight[3 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    (*msg.offset(7 as libc::c_int as isize) as libc::c_int
                        & 63 as libc::c_int) as isize,
                );
            (*mm)
                .flight[4 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    (*msg.offset(8 as libc::c_int as isize) as libc::c_int
                        >> 2 as libc::c_int) as isize,
                );
            (*mm)
                .flight[5 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    ((*msg.offset(8 as libc::c_int as isize) as libc::c_int
                        & 3 as libc::c_int) << 4 as libc::c_int
                        | *msg.offset(9 as libc::c_int as isize) as libc::c_int
                            >> 4 as libc::c_int) as isize,
                );
            (*mm)
                .flight[6 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    ((*msg.offset(9 as libc::c_int as isize) as libc::c_int
                        & 15 as libc::c_int) << 2 as libc::c_int
                        | *msg.offset(10 as libc::c_int as isize) as libc::c_int
                            >> 6 as libc::c_int) as isize,
                );
            (*mm)
                .flight[7 as libc::c_int
                as usize] = *ais_charset
                .offset(
                    (*msg.offset(10 as libc::c_int as isize) as libc::c_int
                        & 63 as libc::c_int) as isize,
                );
            (*mm).flight[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        } else if (*mm).metype >= 9 as libc::c_int && (*mm).metype <= 18 as libc::c_int {
            (*mm)
                .fflag = *msg.offset(6 as libc::c_int as isize) as libc::c_int
                & (1 as libc::c_int) << 2 as libc::c_int;
            (*mm)
                .tflag = *msg.offset(6 as libc::c_int as isize) as libc::c_int
                & (1 as libc::c_int) << 3 as libc::c_int;
            (*mm).altitude = decodeAC12Field(msg, &mut (*mm).unit);
            (*mm)
                .raw_latitude = (*msg.offset(6 as libc::c_int as isize) as libc::c_int
                & 3 as libc::c_int) << 15 as libc::c_int
                | (*msg.offset(7 as libc::c_int as isize) as libc::c_int)
                    << 7 as libc::c_int
                | *msg.offset(8 as libc::c_int as isize) as libc::c_int
                    >> 1 as libc::c_int;
            (*mm)
                .raw_longitude = (*msg.offset(8 as libc::c_int as isize) as libc::c_int
                & 1 as libc::c_int) << 16 as libc::c_int
                | (*msg.offset(9 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int
                | *msg.offset(10 as libc::c_int as isize) as libc::c_int;
        } else if (*mm).metype == 19 as libc::c_int && (*mm).mesub >= 1 as libc::c_int
            && (*mm).mesub <= 4 as libc::c_int
        {
            if (*mm).mesub == 1 as libc::c_int || (*mm).mesub == 2 as libc::c_int {
                (*mm)
                    .ew_dir = (*msg.offset(5 as libc::c_int as isize) as libc::c_int
                    & 4 as libc::c_int) >> 2 as libc::c_int;
                (*mm)
                    .ew_velocity = (*msg.offset(5 as libc::c_int as isize) as libc::c_int
                    & 3 as libc::c_int) << 8 as libc::c_int
                    | *msg.offset(6 as libc::c_int as isize) as libc::c_int;
                (*mm)
                    .ns_dir = (*msg.offset(7 as libc::c_int as isize) as libc::c_int
                    & 0x80 as libc::c_int) >> 7 as libc::c_int;
                (*mm)
                    .ns_velocity = (*msg.offset(7 as libc::c_int as isize) as libc::c_int
                    & 0x7f as libc::c_int) << 3 as libc::c_int
                    | (*msg.offset(8 as libc::c_int as isize) as libc::c_int
                        & 0xe0 as libc::c_int) >> 5 as libc::c_int;
                (*mm)
                    .vert_rate_source = (*msg.offset(8 as libc::c_int as isize)
                    as libc::c_int & 0x10 as libc::c_int) >> 4 as libc::c_int;
                (*mm)
                    .vert_rate_sign = (*msg.offset(8 as libc::c_int as isize)
                    as libc::c_int & 0x8 as libc::c_int) >> 3 as libc::c_int;
                (*mm)
                    .vert_rate = (*msg.offset(8 as libc::c_int as isize) as libc::c_int
                    & 7 as libc::c_int) << 6 as libc::c_int
                    | (*msg.offset(9 as libc::c_int as isize) as libc::c_int
                        & 0xfc as libc::c_int) >> 2 as libc::c_int;
                (*mm)
                    .velocity = sqrt(
                    ((*mm).ns_velocity * (*mm).ns_velocity
                        + (*mm).ew_velocity * (*mm).ew_velocity) as libc::c_double,
                ) as libc::c_int;
                if (*mm).velocity != 0 {
                    let mut ewv: libc::c_int = (*mm).ew_velocity;
                    let mut nsv: libc::c_int = (*mm).ns_velocity;
                    let mut heading: libc::c_double = 0.;
                    if (*mm).ew_dir != 0 {
                        ewv *= -(1 as libc::c_int);
                    }
                    if (*mm).ns_dir != 0 {
                        nsv *= -(1 as libc::c_int);
                    }
                    heading = atan2(ewv as libc::c_double, nsv as libc::c_double);
                    (*mm)
                        .heading = (heading * 360 as libc::c_int as libc::c_double
                        / (3.14159265358979323846f64
                            * 2 as libc::c_int as libc::c_double)) as libc::c_int;
                    if (*mm).heading < 0 as libc::c_int {
                        (*mm).heading += 360 as libc::c_int;
                    }
                } else {
                    (*mm).heading = 0 as libc::c_int;
                }
            } else if (*mm).mesub == 3 as libc::c_int || (*mm).mesub == 4 as libc::c_int
            {
                (*mm)
                    .heading_is_valid = *msg.offset(5 as libc::c_int as isize)
                    as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int;
                (*mm)
                    .heading = (360.0f64 / 128 as libc::c_int as libc::c_double
                    * ((*msg.offset(5 as libc::c_int as isize) as libc::c_int
                        & 3 as libc::c_int) << 5 as libc::c_int
                        | *msg.offset(6 as libc::c_int as isize) as libc::c_int
                            >> 3 as libc::c_int) as libc::c_double) as libc::c_int;
            }
        }
    }
    (*mm).phase_corrected = 0 as libc::c_int;
}
pub unsafe extern "C" fn displayModesMessage(mut mm: *mut modesMessage) {
    let mut j: libc::c_int = 0;
    if Modes.onlyaddr != 0 {
        printf(
            b"%02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
        return;
    }
    printf(b"*\0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int;
    while j < (*mm).msgbits / 8 as libc::c_int {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            (*mm).msg[j as usize] as libc::c_int,
        );
        j += 1;
        j;
    }
    printf(b";\n\0" as *const u8 as *const libc::c_char);
    if Modes.raw != 0 {
        fflush(stdout);
        return;
    }
    printf(
        b"CRC: %06x (%s)\n\0" as *const u8 as *const libc::c_char,
        (*mm).crc as libc::c_int,
        if (*mm).crcok != 0 {
            b"ok\0" as *const u8 as *const libc::c_char
        } else {
            b"wrong\0" as *const u8 as *const libc::c_char
        },
    );
    if (*mm).errorbit != -(1 as libc::c_int) {
        printf(
            b"Single bit error fixed, bit %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).errorbit,
        );
    }
    if (*mm).msgtype == 0 as libc::c_int {
        printf(
            b"DF 0: Short Air-Air Surveillance.\n\0" as *const u8 as *const libc::c_char,
        );
        printf(
            b"  Altitude       : %d %s\n\0" as *const u8 as *const libc::c_char,
            (*mm).altitude,
            if (*mm).unit == 1 as libc::c_int {
                b"meters\0" as *const u8 as *const libc::c_char
            } else {
                b"feet\0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
    } else if (*mm).msgtype == 4 as libc::c_int || (*mm).msgtype == 20 as libc::c_int {
        printf(
            b"DF %d: %s, Altitude Reply.\n\0" as *const u8 as *const libc::c_char,
            (*mm).msgtype,
            if (*mm).msgtype == 4 as libc::c_int {
                b"Surveillance\0" as *const u8 as *const libc::c_char
            } else {
                b"Comm-B\0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"  Flight Status  : %s\n\0" as *const u8 as *const libc::c_char,
            fs_str[(*mm).fs as usize],
        );
        printf(
            b"  DR             : %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).dr,
        );
        printf(
            b"  UM             : %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).um,
        );
        printf(
            b"  Altitude       : %d %s\n\0" as *const u8 as *const libc::c_char,
            (*mm).altitude,
            if (*mm).unit == 1 as libc::c_int {
                b"meters\0" as *const u8 as *const libc::c_char
            } else {
                b"feet\0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
        (*mm).msgtype == 20 as libc::c_int;
    } else if (*mm).msgtype == 5 as libc::c_int || (*mm).msgtype == 21 as libc::c_int {
        printf(
            b"DF %d: %s, Identity Reply.\n\0" as *const u8 as *const libc::c_char,
            (*mm).msgtype,
            if (*mm).msgtype == 5 as libc::c_int {
                b"Surveillance\0" as *const u8 as *const libc::c_char
            } else {
                b"Comm-B\0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"  Flight Status  : %s\n\0" as *const u8 as *const libc::c_char,
            fs_str[(*mm).fs as usize],
        );
        printf(
            b"  DR             : %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).dr,
        );
        printf(
            b"  UM             : %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).um,
        );
        printf(
            b"  Squawk         : %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).identity,
        );
        printf(
            b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
        (*mm).msgtype == 21 as libc::c_int;
    } else if (*mm).msgtype == 11 as libc::c_int {
        printf(b"DF 11: All Call Reply.\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"  Capability  : %s\n\0" as *const u8 as *const libc::c_char,
            ca_str[(*mm).ca as usize],
        );
        printf(
            b"  ICAO Address: %02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
    } else if (*mm).msgtype == 17 as libc::c_int {
        printf(b"DF 17: ADS-B message.\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"  Capability     : %d (%s)\n\0" as *const u8 as *const libc::c_char,
            (*mm).ca,
            ca_str[(*mm).ca as usize],
        );
        printf(
            b"  ICAO Address   : %02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            (*mm).aa1,
            (*mm).aa2,
            (*mm).aa3,
        );
        printf(
            b"  Extended Squitter  Type: %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).metype,
        );
        printf(
            b"  Extended Squitter  Sub : %d\n\0" as *const u8 as *const libc::c_char,
            (*mm).mesub,
        );
        printf(
            b"  Extended Squitter  Name: %s\n\0" as *const u8 as *const libc::c_char,
            getMEDescription((*mm).metype, (*mm).mesub),
        );
        if (*mm).metype >= 1 as libc::c_int && (*mm).metype <= 4 as libc::c_int {
            let mut ac_type_str: [*mut libc::c_char; 4] = [
                b"Aircraft Type D\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"Aircraft Type C\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"Aircraft Type B\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"Aircraft Type A\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ];
            printf(
                b"    Aircraft Type  : %s\n\0" as *const u8 as *const libc::c_char,
                ac_type_str[(*mm).aircraft_type as usize],
            );
            printf(
                b"    Identification : %s\n\0" as *const u8 as *const libc::c_char,
                ((*mm).flight).as_mut_ptr(),
            );
        } else if (*mm).metype >= 9 as libc::c_int && (*mm).metype <= 18 as libc::c_int {
            printf(
                b"    F flag   : %s\n\0" as *const u8 as *const libc::c_char,
                if (*mm).fflag != 0 {
                    b"odd\0" as *const u8 as *const libc::c_char
                } else {
                    b"even\0" as *const u8 as *const libc::c_char
                },
            );
            printf(
                b"    T flag   : %s\n\0" as *const u8 as *const libc::c_char,
                if (*mm).tflag != 0 {
                    b"UTC\0" as *const u8 as *const libc::c_char
                } else {
                    b"non-UTC\0" as *const u8 as *const libc::c_char
                },
            );
            printf(
                b"    Altitude : %d feet\n\0" as *const u8 as *const libc::c_char,
                (*mm).altitude,
            );
            printf(
                b"    Latitude : %d (not decoded)\n\0" as *const u8
                    as *const libc::c_char,
                (*mm).raw_latitude,
            );
            printf(
                b"    Longitude: %d (not decoded)\n\0" as *const u8
                    as *const libc::c_char,
                (*mm).raw_longitude,
            );
        } else if (*mm).metype == 19 as libc::c_int && (*mm).mesub >= 1 as libc::c_int
            && (*mm).mesub <= 4 as libc::c_int
        {
            if (*mm).mesub == 1 as libc::c_int || (*mm).mesub == 2 as libc::c_int {
                printf(
                    b"    EW direction      : %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).ew_dir,
                );
                printf(
                    b"    EW velocity       : %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).ew_velocity,
                );
                printf(
                    b"    NS direction      : %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).ns_dir,
                );
                printf(
                    b"    NS velocity       : %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).ns_velocity,
                );
                printf(
                    b"    Vertical rate src : %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).vert_rate_source,
                );
                printf(
                    b"    Vertical rate sign: %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).vert_rate_sign,
                );
                printf(
                    b"    Vertical rate     : %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*mm).vert_rate,
                );
            } else if (*mm).mesub == 3 as libc::c_int || (*mm).mesub == 4 as libc::c_int
            {
                printf(
                    b"    Heading status: %d\0" as *const u8 as *const libc::c_char,
                    (*mm).heading_is_valid,
                );
                printf(
                    b"    Heading: %d\0" as *const u8 as *const libc::c_char,
                    (*mm).heading,
                );
            }
        } else {
            printf(
                b"    Unrecognized ME type: %d subtype: %d\n\0" as *const u8
                    as *const libc::c_char,
                (*mm).metype,
                (*mm).mesub,
            );
        }
    } else if Modes.check_crc != 0 {
        printf(
            b"DF %d with good CRC received (decoding still not implemented).\n\0"
                as *const u8 as *const libc::c_char,
            (*mm).msgtype,
        );
    }
}
pub unsafe extern "C" fn computeMagnitudeVector() {
    let mut m: *mut uint16_t = Modes.magnitude;
    let mut p: *mut libc::c_uchar = Modes.data;
    let mut j: uint32_t = 0;
    j = 0 as libc::c_int as uint32_t;
    while j < Modes.data_len {
        let mut i: libc::c_int = *p.offset(j as isize) as libc::c_int
            - 127 as libc::c_int;
        let mut q: libc::c_int = *p
            .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int - 127 as libc::c_int;
        if i < 0 as libc::c_int {
            i = -i;
        }
        if q < 0 as libc::c_int {
            q = -q;
        }
        *m
            .offset(
                j.wrapping_div(2 as libc::c_int as libc::c_uint) as isize,
            ) = *(Modes.maglut).offset((i * 129 as libc::c_int + q) as isize);
        j = (j as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    }
}
pub unsafe extern "C" fn detectOutOfPhase(mut m: *mut uint16_t) -> libc::c_int {
    if *m.offset(3 as libc::c_int as isize) as libc::c_int
        > *m.offset(2 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if *m.offset(10 as libc::c_int as isize) as libc::c_int
        > *m.offset(9 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if *m.offset(6 as libc::c_int as isize) as libc::c_int
        > *m.offset(7 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if *m.offset(-(1 as libc::c_int) as isize) as libc::c_int
        > *m.offset(1 as libc::c_int as isize) as libc::c_int / 3 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn applyPhaseCorrection(mut m: *mut uint16_t) {
    let mut j: libc::c_int = 0;
    m = m.offset(16 as libc::c_int as isize);
    j = 0 as libc::c_int;
    while j < (112 as libc::c_int - 1 as libc::c_int) * 2 as libc::c_int {
        if *m.offset(j as isize) as libc::c_int
            > *m.offset((j + 1 as libc::c_int) as isize) as libc::c_int
        {
            *m
                .offset(
                    (j + 2 as libc::c_int) as isize,
                ) = (*m.offset((j + 2 as libc::c_int) as isize) as libc::c_int
                * 5 as libc::c_int / 4 as libc::c_int) as uint16_t;
        } else {
            *m
                .offset(
                    (j + 2 as libc::c_int) as isize,
                ) = (*m.offset((j + 2 as libc::c_int) as isize) as libc::c_int
                * 4 as libc::c_int / 5 as libc::c_int) as uint16_t;
        }
        j += 2 as libc::c_int;
    }
}
pub unsafe extern "C" fn detectModeS(mut m: *mut uint16_t, mut mlen: uint32_t) {
    let mut bits: [libc::c_uchar; 112] = [0; 112];
    let mut msg: [libc::c_uchar; 56] = [0; 56];
    let mut aux: [uint16_t; 224] = [0; 224];
    let mut j: uint32_t = 0;
    let mut use_correction: libc::c_int = 0 as libc::c_int;
    let mut current_block_94: u64;
    j = 0 as libc::c_int as uint32_t;
    while j
        < mlen
            .wrapping_sub(
                ((8 as libc::c_int + 112 as libc::c_int) * 2 as libc::c_int)
                    as libc::c_uint,
            )
    {
        let mut low: libc::c_int = 0;
        let mut high: libc::c_int = 0;
        let mut delta: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut errors: libc::c_int = 0;
        let mut good_message: libc::c_int = 0 as libc::c_int;
        if use_correction != 0 {
            current_block_94 = 6549934102840725649;
        } else if !(*m.offset(j as isize) as libc::c_int
            > *m.offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int
            && (*m.offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int)
                < *m.offset(j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
            && *m.offset(j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int
                > *m.offset(j.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
            && (*m.offset(j.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int) < *m.offset(j as isize) as libc::c_int
            && (*m.offset(j.wrapping_add(4 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int) < *m.offset(j as isize) as libc::c_int
            && (*m.offset(j.wrapping_add(5 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int) < *m.offset(j as isize) as libc::c_int
            && (*m.offset(j.wrapping_add(6 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int) < *m.offset(j as isize) as libc::c_int
            && *m.offset(j.wrapping_add(7 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int
                > *m.offset(j.wrapping_add(8 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
            && (*m.offset(j.wrapping_add(8 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int)
                < *m.offset(j.wrapping_add(9 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
            && *m.offset(j.wrapping_add(9 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int
                > *m.offset(j.wrapping_add(6 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int)
        {
            if Modes.debug & (1 as libc::c_int) << 4 as libc::c_int != 0
                && *m.offset(j as isize) as libc::c_int > 25 as libc::c_int
            {
                dumpRawMessage(
                    b"Unexpected ratio among first 10 samples\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    msg.as_mut_ptr(),
                    m,
                    j,
                );
            }
            current_block_94 = 14916268686031723178;
        } else {
            high = (*m.offset(j as isize) as libc::c_int
                + *m.offset(j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
                + *m.offset(j.wrapping_add(7 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
                + *m.offset(j.wrapping_add(9 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) / 6 as libc::c_int;
            if *m.offset(j.wrapping_add(4 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int >= high
                || *m.offset(j.wrapping_add(5 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int >= high
            {
                if Modes.debug & (1 as libc::c_int) << 4 as libc::c_int != 0
                    && *m.offset(j as isize) as libc::c_int > 25 as libc::c_int
                {
                    dumpRawMessage(
                        b"Too high level in samples between 3 and 6\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        msg.as_mut_ptr(),
                        m,
                        j,
                    );
                }
                current_block_94 = 14916268686031723178;
            } else if *m
                .offset(j.wrapping_add(11 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int >= high
                || *m.offset(j.wrapping_add(12 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int >= high
                || *m.offset(j.wrapping_add(13 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int >= high
                || *m.offset(j.wrapping_add(14 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int >= high
            {
                if Modes.debug & (1 as libc::c_int) << 4 as libc::c_int != 0
                    && *m.offset(j as isize) as libc::c_int > 25 as libc::c_int
                {
                    dumpRawMessage(
                        b"Too high level in samples between 10 and 15\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        msg.as_mut_ptr(),
                        m,
                        j,
                    );
                }
                current_block_94 = 14916268686031723178;
            } else {
                Modes.stat_valid_preamble += 1;
                Modes.stat_valid_preamble;
                current_block_94 = 6549934102840725649;
            }
        }
        match current_block_94 {
            6549934102840725649 => {
                if use_correction != 0 {
                    memcpy(
                        aux.as_mut_ptr() as *mut libc::c_void,
                        m
                            .offset(j as isize)
                            .offset((8 as libc::c_int * 2 as libc::c_int) as isize)
                            as *const libc::c_void,
                        ::std::mem::size_of::<[uint16_t; 224]>() as libc::c_ulong,
                    );
                    if j != 0 && detectOutOfPhase(m.offset(j as isize)) != 0 {
                        applyPhaseCorrection(m.offset(j as isize));
                        Modes.stat_out_of_phase += 1;
                        Modes.stat_out_of_phase;
                    }
                }
                errors = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < 112 as libc::c_int * 2 as libc::c_int {
                    low = *m
                        .offset(
                            j
                                .wrapping_add(i as libc::c_uint)
                                .wrapping_add(
                                    (8 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                                ) as isize,
                        ) as libc::c_int;
                    high = *m
                        .offset(
                            j
                                .wrapping_add(i as libc::c_uint)
                                .wrapping_add(
                                    (8 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                    delta = low - high;
                    if delta < 0 as libc::c_int {
                        delta = -delta;
                    }
                    if i > 0 as libc::c_int && delta < 256 as libc::c_int {
                        bits[(i / 2 as libc::c_int)
                            as usize] = bits[(i / 2 as libc::c_int - 1 as libc::c_int)
                            as usize];
                    } else if low == high {
                        bits[(i / 2 as libc::c_int)
                            as usize] = 2 as libc::c_int as libc::c_uchar;
                        if i < 56 as libc::c_int * 2 as libc::c_int {
                            errors += 1;
                            errors;
                        }
                    } else if low > high {
                        bits[(i / 2 as libc::c_int)
                            as usize] = 1 as libc::c_int as libc::c_uchar;
                    } else {
                        bits[(i / 2 as libc::c_int)
                            as usize] = 0 as libc::c_int as libc::c_uchar;
                    }
                    i += 2 as libc::c_int;
                }
                if use_correction != 0 {
                    memcpy(
                        m
                            .offset(j as isize)
                            .offset((8 as libc::c_int * 2 as libc::c_int) as isize)
                            as *mut libc::c_void,
                        aux.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[uint16_t; 224]>() as libc::c_ulong,
                    );
                }
                i = 0 as libc::c_int;
                while i < 112 as libc::c_int {
                    msg[(i / 8 as libc::c_int)
                        as usize] = ((bits[i as usize] as libc::c_int)
                        << 7 as libc::c_int
                        | (bits[(i + 1 as libc::c_int) as usize] as libc::c_int)
                            << 6 as libc::c_int
                        | (bits[(i + 2 as libc::c_int) as usize] as libc::c_int)
                            << 5 as libc::c_int
                        | (bits[(i + 3 as libc::c_int) as usize] as libc::c_int)
                            << 4 as libc::c_int
                        | (bits[(i + 4 as libc::c_int) as usize] as libc::c_int)
                            << 3 as libc::c_int
                        | (bits[(i + 5 as libc::c_int) as usize] as libc::c_int)
                            << 2 as libc::c_int
                        | (bits[(i + 6 as libc::c_int) as usize] as libc::c_int)
                            << 1 as libc::c_int
                        | bits[(i + 7 as libc::c_int) as usize] as libc::c_int)
                        as libc::c_uchar;
                    i += 8 as libc::c_int;
                }
                let mut msgtype: libc::c_int = msg[0 as libc::c_int as usize]
                    as libc::c_int >> 3 as libc::c_int;
                let mut msglen: libc::c_int = modesMessageLenByType(msgtype)
                    / 8 as libc::c_int;
                delta = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < msglen * 8 as libc::c_int * 2 as libc::c_int {
                    delta
                        += abs(
                            *m
                                .offset(
                                    j
                                        .wrapping_add(i as libc::c_uint)
                                        .wrapping_add(
                                            (8 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                                        ) as isize,
                                ) as libc::c_int
                                - *m
                                    .offset(
                                        j
                                            .wrapping_add(i as libc::c_uint)
                                            .wrapping_add(
                                                (8 as libc::c_int * 2 as libc::c_int) as libc::c_uint,
                                            )
                                            .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                    ) as libc::c_int,
                        );
                    i += 2 as libc::c_int;
                }
                delta /= msglen * 4 as libc::c_int;
                if delta < 10 as libc::c_int * 255 as libc::c_int {
                    use_correction = 0 as libc::c_int;
                } else {
                    if errors == 0 as libc::c_int
                        || Modes.aggressive != 0 && errors < 3 as libc::c_int
                    {
                        let mut mm: modesMessage = modesMessage {
                            msg: [0; 14],
                            msgbits: 0,
                            msgtype: 0,
                            crcok: 0,
                            crc: 0,
                            errorbit: 0,
                            aa1: 0,
                            aa2: 0,
                            aa3: 0,
                            phase_corrected: 0,
                            ca: 0,
                            metype: 0,
                            mesub: 0,
                            heading_is_valid: 0,
                            heading: 0,
                            aircraft_type: 0,
                            fflag: 0,
                            tflag: 0,
                            raw_latitude: 0,
                            raw_longitude: 0,
                            flight: [0; 9],
                            ew_dir: 0,
                            ew_velocity: 0,
                            ns_dir: 0,
                            ns_velocity: 0,
                            vert_rate_source: 0,
                            vert_rate_sign: 0,
                            vert_rate: 0,
                            velocity: 0,
                            fs: 0,
                            dr: 0,
                            um: 0,
                            identity: 0,
                            altitude: 0,
                            unit: 0,
                        };
                        decodeModesMessage(&mut mm, msg.as_mut_ptr());
                        if mm.crcok != 0 || use_correction != 0 {
                            if errors == 0 as libc::c_int {
                                Modes.stat_demodulated += 1;
                                Modes.stat_demodulated;
                            }
                            if mm.errorbit == -(1 as libc::c_int) {
                                if mm.crcok != 0 {
                                    Modes.stat_goodcrc += 1;
                                    Modes.stat_goodcrc;
                                } else {
                                    Modes.stat_badcrc += 1;
                                    Modes.stat_badcrc;
                                }
                            } else {
                                Modes.stat_badcrc += 1;
                                Modes.stat_badcrc;
                                Modes.stat_fixed += 1;
                                Modes.stat_fixed;
                                if mm.errorbit < 112 as libc::c_int {
                                    Modes.stat_single_bit_fix += 1;
                                    Modes.stat_single_bit_fix;
                                } else {
                                    Modes.stat_two_bits_fix += 1;
                                    Modes.stat_two_bits_fix;
                                }
                            }
                        }
                        if use_correction == 0 as libc::c_int {
                            if Modes.debug & (1 as libc::c_int) << 0 as libc::c_int != 0
                            {
                                dumpRawMessage(
                                    b"Demodulated with 0 errors\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    msg.as_mut_ptr(),
                                    m,
                                    j,
                                );
                            } else if Modes.debug
                                & (1 as libc::c_int) << 2 as libc::c_int != 0
                                && mm.msgtype == 17 as libc::c_int
                                && (mm.crcok == 0 || mm.errorbit != -(1 as libc::c_int))
                            {
                                dumpRawMessage(
                                    b"Decoded with bad CRC\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    msg.as_mut_ptr(),
                                    m,
                                    j,
                                );
                            } else if Modes.debug
                                & (1 as libc::c_int) << 3 as libc::c_int != 0
                                && mm.crcok != 0 && mm.errorbit == -(1 as libc::c_int)
                            {
                                dumpRawMessage(
                                    b"Decoded with good CRC\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    msg.as_mut_ptr(),
                                    m,
                                    j,
                                );
                            }
                        }
                        if mm.crcok != 0 {
                            j = (j as libc::c_uint)
                                .wrapping_add(
                                    ((8 as libc::c_int + msglen * 8 as libc::c_int)
                                        * 2 as libc::c_int) as libc::c_uint,
                                ) as uint32_t as uint32_t;
                            good_message = 1 as libc::c_int;
                            if use_correction != 0 {
                                mm.phase_corrected = 1 as libc::c_int;
                            }
                        }
                        useModesMessage(&mut mm);
                    } else if Modes.debug & (1 as libc::c_int) << 1 as libc::c_int != 0
                        && use_correction != 0
                    {
                        printf(
                            b"The following message has %d demod errors\n\0" as *const u8
                                as *const libc::c_char,
                            errors,
                        );
                        dumpRawMessage(
                            b"Demodulated with errors\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            msg.as_mut_ptr(),
                            m,
                            j,
                        );
                    }
                    if good_message == 0 && use_correction == 0 {
                        j = j.wrapping_sub(1);
                        j;
                        use_correction = 1 as libc::c_int;
                    } else {
                        use_correction = 0 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        j = j.wrapping_add(1);
        j;
    }
}
pub unsafe extern "C" fn useModesMessage(mut mm: *mut modesMessage) {
    if Modes.stats == 0 && (Modes.check_crc == 0 as libc::c_int || (*mm).crcok != 0) {
        if Modes.interactive != 0
            || Modes.stat_http_requests > 0 as libc::c_int as libc::c_longlong
            || Modes.stat_sbs_connections > 0 as libc::c_int as libc::c_longlong
        {
            let mut a: *mut aircraft = interactiveReceiveData(mm);
            if !a.is_null()
                && Modes.stat_sbs_connections > 0 as libc::c_int as libc::c_longlong
            {
                modesSendSBSOutput(mm, a);
            }
        }
        if Modes.interactive == 0 {
            displayModesMessage(mm);
            if Modes.raw == 0 && Modes.onlyaddr == 0 {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        if Modes.net != 0 {
            modesSendRawOutput(mm);
        }
    }
}
pub unsafe extern "C" fn interactiveCreateAircraft(mut addr: uint32_t) -> *mut aircraft {
    let mut a: *mut aircraft = malloc(::std::mem::size_of::<aircraft>() as libc::c_ulong)
        as *mut aircraft;
    (*a).addr = addr;
    snprintf(
        ((*a).hexaddr).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
        b"%06x\0" as *const u8 as *const libc::c_char,
        addr as libc::c_int,
    );
    (*a).flight[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    (*a).altitude = 0 as libc::c_int;
    (*a).speed = 0 as libc::c_int;
    (*a).track = 0 as libc::c_int;
    (*a).odd_cprlat = 0 as libc::c_int;
    (*a).odd_cprlon = 0 as libc::c_int;
    (*a).odd_cprtime = 0 as libc::c_int as libc::c_longlong;
    (*a).even_cprlat = 0 as libc::c_int;
    (*a).even_cprlon = 0 as libc::c_int;
    (*a).even_cprtime = 0 as libc::c_int as libc::c_longlong;
    (*a).lat = 0 as libc::c_int as libc::c_double;
    (*a).lon = 0 as libc::c_int as libc::c_double;
    (*a).seen = time(0 as *mut time_t);
    (*a).messages = 0 as libc::c_int as libc::c_long;
    (*a).next = 0 as *mut aircraft;
    return a;
}
pub unsafe extern "C" fn interactiveFindAircraft(mut addr: uint32_t) -> *mut aircraft {
    let mut a: *mut aircraft = Modes.aircrafts;
    while !a.is_null() {
        if (*a).addr == addr {
            return a;
        }
        a = (*a).next;
    }
    return 0 as *mut aircraft;
}
pub unsafe extern "C" fn cprModFunction(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = a % b;
    if res < 0 as libc::c_int {
        res += b;
    }
    return res;
}
pub unsafe extern "C" fn cprNLFunction(mut lat: libc::c_double) -> libc::c_int {
    if lat < 0 as libc::c_int as libc::c_double {
        lat = -lat;
    }
    if lat < 10.47047130f64 {
        return 59 as libc::c_int;
    }
    if lat < 14.82817437f64 {
        return 58 as libc::c_int;
    }
    if lat < 18.18626357f64 {
        return 57 as libc::c_int;
    }
    if lat < 21.02939493f64 {
        return 56 as libc::c_int;
    }
    if lat < 23.54504487f64 {
        return 55 as libc::c_int;
    }
    if lat < 25.82924707f64 {
        return 54 as libc::c_int;
    }
    if lat < 27.93898710f64 {
        return 53 as libc::c_int;
    }
    if lat < 29.91135686f64 {
        return 52 as libc::c_int;
    }
    if lat < 31.77209708f64 {
        return 51 as libc::c_int;
    }
    if lat < 33.53993436f64 {
        return 50 as libc::c_int;
    }
    if lat < 35.22899598f64 {
        return 49 as libc::c_int;
    }
    if lat < 36.85025108f64 {
        return 48 as libc::c_int;
    }
    if lat < 38.41241892f64 {
        return 47 as libc::c_int;
    }
    if lat < 39.92256684f64 {
        return 46 as libc::c_int;
    }
    if lat < 41.38651832f64 {
        return 45 as libc::c_int;
    }
    if lat < 42.80914012f64 {
        return 44 as libc::c_int;
    }
    if lat < 44.19454951f64 {
        return 43 as libc::c_int;
    }
    if lat < 45.54626723f64 {
        return 42 as libc::c_int;
    }
    if lat < 46.86733252f64 {
        return 41 as libc::c_int;
    }
    if lat < 48.16039128f64 {
        return 40 as libc::c_int;
    }
    if lat < 49.42776439f64 {
        return 39 as libc::c_int;
    }
    if lat < 50.67150166f64 {
        return 38 as libc::c_int;
    }
    if lat < 51.89342469f64 {
        return 37 as libc::c_int;
    }
    if lat < 53.09516153f64 {
        return 36 as libc::c_int;
    }
    if lat < 54.27817472f64 {
        return 35 as libc::c_int;
    }
    if lat < 55.44378444f64 {
        return 34 as libc::c_int;
    }
    if lat < 56.59318756f64 {
        return 33 as libc::c_int;
    }
    if lat < 57.72747354f64 {
        return 32 as libc::c_int;
    }
    if lat < 58.84763776f64 {
        return 31 as libc::c_int;
    }
    if lat < 59.95459277f64 {
        return 30 as libc::c_int;
    }
    if lat < 61.04917774f64 {
        return 29 as libc::c_int;
    }
    if lat < 62.13216659f64 {
        return 28 as libc::c_int;
    }
    if lat < 63.20427479f64 {
        return 27 as libc::c_int;
    }
    if lat < 64.26616523f64 {
        return 26 as libc::c_int;
    }
    if lat < 65.31845310f64 {
        return 25 as libc::c_int;
    }
    if lat < 66.36171008f64 {
        return 24 as libc::c_int;
    }
    if lat < 67.39646774f64 {
        return 23 as libc::c_int;
    }
    if lat < 68.42322022f64 {
        return 22 as libc::c_int;
    }
    if lat < 69.44242631f64 {
        return 21 as libc::c_int;
    }
    if lat < 70.45451075f64 {
        return 20 as libc::c_int;
    }
    if lat < 71.45986473f64 {
        return 19 as libc::c_int;
    }
    if lat < 72.45884545f64 {
        return 18 as libc::c_int;
    }
    if lat < 73.45177442f64 {
        return 17 as libc::c_int;
    }
    if lat < 74.43893416f64 {
        return 16 as libc::c_int;
    }
    if lat < 75.42056257f64 {
        return 15 as libc::c_int;
    }
    if lat < 76.39684391f64 {
        return 14 as libc::c_int;
    }
    if lat < 77.36789461f64 {
        return 13 as libc::c_int;
    }
    if lat < 78.33374083f64 {
        return 12 as libc::c_int;
    }
    if lat < 79.29428225f64 {
        return 11 as libc::c_int;
    }
    if lat < 80.24923213f64 {
        return 10 as libc::c_int;
    }
    if lat < 81.19801349f64 {
        return 9 as libc::c_int;
    }
    if lat < 82.13956981f64 {
        return 8 as libc::c_int;
    }
    if lat < 83.07199445f64 {
        return 7 as libc::c_int;
    }
    if lat < 83.99173563f64 {
        return 6 as libc::c_int;
    }
    if lat < 84.89166191f64 {
        return 5 as libc::c_int;
    }
    if lat < 85.75541621f64 {
        return 4 as libc::c_int;
    }
    if lat < 86.53536998f64 {
        return 3 as libc::c_int;
    }
    if lat < 87.00000000f64 { return 2 as libc::c_int } else { return 1 as libc::c_int };
}
pub unsafe extern "C" fn cprNFunction(
    mut lat: libc::c_double,
    mut isodd: libc::c_int,
) -> libc::c_int {
    let mut nl: libc::c_int = cprNLFunction(lat) - isodd;
    if nl < 1 as libc::c_int {
        nl = 1 as libc::c_int;
    }
    return nl;
}
pub unsafe extern "C" fn cprDlonFunction(
    mut lat: libc::c_double,
    mut isodd: libc::c_int,
) -> libc::c_double {
    return 360.0f64 / cprNFunction(lat, isodd) as libc::c_double;
}
pub unsafe extern "C" fn decodeCPR(mut a: *mut aircraft) {
    let AirDlat0: libc::c_double = 360.0f64 / 60 as libc::c_int as libc::c_double;
    let AirDlat1: libc::c_double = 360.0f64 / 59 as libc::c_int as libc::c_double;
    let mut lat0: libc::c_double = (*a).even_cprlat as libc::c_double;
    let mut lat1: libc::c_double = (*a).odd_cprlat as libc::c_double;
    let mut lon0: libc::c_double = (*a).even_cprlon as libc::c_double;
    let mut lon1: libc::c_double = (*a).odd_cprlon as libc::c_double;
    let mut j: libc::c_int = floor(
        (59 as libc::c_int as libc::c_double * lat0
            - 60 as libc::c_int as libc::c_double * lat1)
            / 131072 as libc::c_int as libc::c_double + 0.5f64,
    ) as libc::c_int;
    let mut rlat0: libc::c_double = AirDlat0
        * (cprModFunction(j, 60 as libc::c_int) as libc::c_double
            + lat0 / 131072 as libc::c_int as libc::c_double);
    let mut rlat1: libc::c_double = AirDlat1
        * (cprModFunction(j, 59 as libc::c_int) as libc::c_double
            + lat1 / 131072 as libc::c_int as libc::c_double);
    if rlat0 >= 270 as libc::c_int as libc::c_double {
        rlat0 -= 360 as libc::c_int as libc::c_double;
    }
    if rlat1 >= 270 as libc::c_int as libc::c_double {
        rlat1 -= 360 as libc::c_int as libc::c_double;
    }
    if cprNLFunction(rlat0) != cprNLFunction(rlat1) {
        return;
    }
    if (*a).even_cprtime > (*a).odd_cprtime {
        let mut ni: libc::c_int = cprNFunction(rlat0, 0 as libc::c_int);
        let mut m: libc::c_int = floor(
            (lon0 * (cprNLFunction(rlat0) - 1 as libc::c_int) as libc::c_double
                - lon1 * cprNLFunction(rlat0) as libc::c_double)
                / 131072 as libc::c_int as libc::c_double + 0.5f64,
        ) as libc::c_int;
        (*a)
            .lon = cprDlonFunction(rlat0, 0 as libc::c_int)
            * (cprModFunction(m, ni) as libc::c_double
                + lon0 / 131072 as libc::c_int as libc::c_double);
        (*a).lat = rlat0;
    } else {
        let mut ni_0: libc::c_int = cprNFunction(rlat1, 1 as libc::c_int);
        let mut m_0: libc::c_int = floor(
            (lon0 * (cprNLFunction(rlat1) - 1 as libc::c_int) as libc::c_double
                - lon1 * cprNLFunction(rlat1) as libc::c_double) / 131072.0f64 + 0.5f64,
        ) as libc::c_int;
        (*a)
            .lon = cprDlonFunction(rlat1, 1 as libc::c_int)
            * (cprModFunction(m_0, ni_0) as libc::c_double
                + lon1 / 131072 as libc::c_int as libc::c_double);
        (*a).lat = rlat1;
    }
    if (*a).lon > 180 as libc::c_int as libc::c_double {
        (*a).lon -= 360 as libc::c_int as libc::c_double;
    }
}
pub unsafe extern "C" fn interactiveReceiveData(
    mut mm: *mut modesMessage,
) -> *mut aircraft {
    let mut addr: uint32_t = 0;
    let mut a: *mut aircraft = 0 as *mut aircraft;
    let mut aux: *mut aircraft = 0 as *mut aircraft;
    if Modes.check_crc != 0 && (*mm).crcok == 0 as libc::c_int {
        return 0 as *mut aircraft;
    }
    addr = ((*mm).aa1 << 16 as libc::c_int | (*mm).aa2 << 8 as libc::c_int | (*mm).aa3)
        as uint32_t;
    a = interactiveFindAircraft(addr);
    if a.is_null() {
        a = interactiveCreateAircraft(addr);
        (*a).next = Modes.aircrafts;
        Modes.aircrafts = a;
    } else if 0 as libc::c_int != 0 && Modes.aircrafts != a
        && time(0 as *mut time_t) - (*a).seen >= 1 as libc::c_int as libc::c_long
    {
        aux = Modes.aircrafts;
        while (*aux).next != a {
            aux = (*aux).next;
        }
        (*aux).next = (*(*aux).next).next;
        (*a).next = Modes.aircrafts;
        Modes.aircrafts = a;
    }
    (*a).seen = time(0 as *mut time_t);
    (*a).messages += 1;
    (*a).messages;
    if (*mm).msgtype == 0 as libc::c_int || (*mm).msgtype == 4 as libc::c_int
        || (*mm).msgtype == 20 as libc::c_int
    {
        (*a).altitude = (*mm).altitude;
    } else if (*mm).msgtype == 17 as libc::c_int {
        if (*mm).metype >= 1 as libc::c_int && (*mm).metype <= 4 as libc::c_int {
            memcpy(
                ((*a).flight).as_mut_ptr() as *mut libc::c_void,
                ((*mm).flight).as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            );
        } else if (*mm).metype >= 9 as libc::c_int && (*mm).metype <= 18 as libc::c_int {
            (*a).altitude = (*mm).altitude;
            if (*mm).fflag != 0 {
                (*a).odd_cprlat = (*mm).raw_latitude;
                (*a).odd_cprlon = (*mm).raw_longitude;
                (*a).odd_cprtime = mstime();
            } else {
                (*a).even_cprlat = (*mm).raw_latitude;
                (*a).even_cprlon = (*mm).raw_longitude;
                (*a).even_cprtime = mstime();
            }
            if abs(((*a).even_cprtime - (*a).odd_cprtime) as libc::c_int)
                <= 10000 as libc::c_int
            {
                decodeCPR(a);
            }
        } else if (*mm).metype == 19 as libc::c_int {
            if (*mm).mesub == 1 as libc::c_int || (*mm).mesub == 2 as libc::c_int {
                (*a).speed = (*mm).velocity;
                (*a).track = (*mm).heading;
            }
        }
    }
    return a;
}
pub unsafe extern "C" fn interactiveShowData() {
    let mut a: *mut aircraft = Modes.aircrafts;
    let mut now: time_t = time(0 as *mut time_t);
    let mut progress: [libc::c_char; 4] = [0; 4];
    let mut count: libc::c_int = 0 as libc::c_int;
    memset(
        progress.as_mut_ptr() as *mut libc::c_void,
        ' ' as i32,
        3 as libc::c_int as libc::c_ulong,
    );
    progress[(time(0 as *mut time_t) % 3 as libc::c_int as libc::c_long)
        as usize] = '.' as i32 as libc::c_char;
    progress[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    printf(b"\x1B[H\x1B[2J\0" as *const u8 as *const libc::c_char);
    printf(
        b"Hex    Flight   Altitude  Speed   Lat       Lon       Track  Messages Seen %s\n--------------------------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
        progress.as_mut_ptr(),
    );
    while !a.is_null() && count < Modes.interactive_rows {
        let mut altitude: libc::c_int = (*a).altitude;
        let mut speed: libc::c_int = (*a).speed;
        if Modes.metric != 0 {
            altitude = (altitude as libc::c_double / 3.2828f64) as libc::c_int;
            speed = (speed as libc::c_double * 1.852f64) as libc::c_int;
        }
        printf(
            b"%-6s %-8s %-9d %-7d %-7.03f   %-7.03f   %-3d   %-9ld %d sec\n\0"
                as *const u8 as *const libc::c_char,
            ((*a).hexaddr).as_mut_ptr(),
            ((*a).flight).as_mut_ptr(),
            altitude,
            speed,
            (*a).lat,
            (*a).lon,
            (*a).track,
            (*a).messages,
            (now - (*a).seen) as libc::c_int,
        );
        a = (*a).next;
        count += 1;
        count;
    }
}
pub unsafe extern "C" fn interactiveRemoveStaleAircrafts() {
    let mut a: *mut aircraft = Modes.aircrafts;
    let mut prev: *mut aircraft = 0 as *mut aircraft;
    let mut now: time_t = time(0 as *mut time_t);
    while !a.is_null() {
        if now - (*a).seen > Modes.interactive_ttl as libc::c_long {
            let mut next: *mut aircraft = (*a).next;
            free(a as *mut libc::c_void);
            if prev.is_null() {
                Modes.aircrafts = next;
            } else {
                (*prev).next = next;
            }
            a = next;
        } else {
            prev = a;
            a = (*a).next;
        }
    }
}
pub unsafe extern "C" fn snipMode(mut level: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut c: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    loop {
        i = getchar();
        if !(i != -(1 as libc::c_int)
            && {
                q = getchar();
                q != -(1 as libc::c_int)
            })
        {
            break;
        }
        if abs(i - 127 as libc::c_int) < level && abs(q - 127 as libc::c_int) < level {
            c += 1;
            c;
            if c > (8 as libc::c_int * 4 as libc::c_int) as libc::c_longlong {
                continue;
            }
        } else {
            c = 0 as libc::c_int as libc::c_longlong;
        }
        putchar(i);
        putchar(q);
    };
}
pub static mut modesNetServices: [C2RustUnnamed_5; 4] = [C2RustUnnamed_5 {
    descr: 0 as *mut libc::c_char,
    socket: 0 as *mut libc::c_int,
    port: 0,
}; 4];
pub unsafe extern "C" fn modesInitNet() {
    let mut j: libc::c_int = 0;
    memset(
        (Modes.clients).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut client; 1024]>() as libc::c_ulong,
    );
    Modes.maxfd = -(1 as libc::c_int);
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        let mut s: libc::c_int = anetTcpServer(
            (Modes.aneterr).as_mut_ptr(),
            modesNetServices[j as usize].port,
            0 as *mut libc::c_char,
        );
        if s == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Error opening the listening port %d (%s): %s\n\0" as *const u8
                    as *const libc::c_char,
                modesNetServices[j as usize].port,
                modesNetServices[j as usize].descr,
                strerror(*__errno_location()),
            );
            exit(1 as libc::c_int);
        }
        anetNonBlock((Modes.aneterr).as_mut_ptr(), s);
        *modesNetServices[j as usize].socket = s;
        j += 1;
        j;
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
}
pub unsafe extern "C" fn modesAcceptClients() {
    let mut fd: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    let mut c: *mut client = 0 as *mut client;
    j = 0 as libc::c_int as libc::c_uint;
    while j < 4 as libc::c_int as libc::c_uint {
        fd = anetTcpAccept(
            (Modes.aneterr).as_mut_ptr(),
            *modesNetServices[j as usize].socket,
            0 as *mut libc::c_char,
            &mut port,
        );
        if fd == -(1 as libc::c_int) {
            if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0
                && *__errno_location() != 11 as libc::c_int
            {
                printf(
                    b"Accept %d: %s\n\0" as *const u8 as *const libc::c_char,
                    *modesNetServices[j as usize].socket,
                    strerror(*__errno_location()),
                );
            }
        } else {
            if fd >= 1024 as libc::c_int {
                close(fd);
                return;
            }
            anetNonBlock((Modes.aneterr).as_mut_ptr(), fd);
            c = malloc(::std::mem::size_of::<client>() as libc::c_ulong) as *mut client;
            (*c).service = *modesNetServices[j as usize].socket;
            (*c).fd = fd;
            (*c).buflen = 0 as libc::c_int;
            Modes.clients[fd as usize] = c;
            anetSetSendBuffer(
                (Modes.aneterr).as_mut_ptr(),
                fd,
                1024 as libc::c_int * 64 as libc::c_int,
            );
            if Modes.maxfd < fd {
                Modes.maxfd = fd;
            }
            if *modesNetServices[j as usize].socket == Modes.sbsos {
                Modes.stat_sbs_connections += 1;
                Modes.stat_sbs_connections;
            }
            j = j.wrapping_sub(1);
            j;
            if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                printf(
                    b"Created new client %d\n\0" as *const u8 as *const libc::c_char,
                    fd,
                );
            }
        }
        j = j.wrapping_add(1);
        j;
    }
}
pub unsafe extern "C" fn modesFreeClient(mut fd: libc::c_int) {
    close(fd);
    free(Modes.clients[fd as usize] as *mut libc::c_void);
    Modes.clients[fd as usize] = 0 as *mut client;
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(b"Closing client %d\n\0" as *const u8 as *const libc::c_char, fd);
    }
    if Modes.maxfd == fd {
        let mut j: libc::c_int = 0;
        Modes.maxfd = -(1 as libc::c_int);
        j = fd - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            if !(Modes.clients[j as usize]).is_null() {
                Modes.maxfd = j;
                break;
            } else {
                j -= 1;
                j;
            }
        }
    }
}
pub unsafe extern "C" fn modesSendAllClients(
    mut service: libc::c_int,
    mut msg: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut c: *mut client = 0 as *mut client;
    j = 0 as libc::c_int;
    while j <= Modes.maxfd {
        c = Modes.clients[j as usize];
        if !c.is_null() && (*c).service == service {
            let mut nwritten: libc::c_int = write(j, msg, len as size_t) as libc::c_int;
            if nwritten != len {
                modesFreeClient(j);
            }
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn modesSendRawOutput(mut mm: *mut modesMessage) {
    let mut msg: [libc::c_char; 128] = [0; 128];
    let mut p: *mut libc::c_char = msg.as_mut_ptr();
    let mut j: libc::c_int = 0;
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '*' as i32 as libc::c_char;
    j = 0 as libc::c_int;
    while j < (*mm).msgbits / 8 as libc::c_int {
        sprintf(
            p,
            b"%02X\0" as *const u8 as *const libc::c_char,
            (*mm).msg[j as usize] as libc::c_int,
        );
        p = p.offset(2 as libc::c_int as isize);
        j += 1;
        j;
    }
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = ';' as i32 as libc::c_char;
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = '\n' as i32 as libc::c_char;
    modesSendAllClients(
        Modes.ros,
        msg.as_mut_ptr() as *mut libc::c_void,
        p.offset_from(msg.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
}
pub unsafe extern "C" fn modesSendSBSOutput(
    mut mm: *mut modesMessage,
    mut a: *mut aircraft,
) {
    let mut msg: [libc::c_char; 256] = [0; 256];
    let mut p: *mut libc::c_char = msg.as_mut_ptr();
    let mut emergency: libc::c_int = 0 as libc::c_int;
    let mut ground: libc::c_int = 0 as libc::c_int;
    let mut alert: libc::c_int = 0 as libc::c_int;
    let mut spi: libc::c_int = 0 as libc::c_int;
    if (*mm).msgtype == 4 as libc::c_int || (*mm).msgtype == 5 as libc::c_int
        || (*mm).msgtype == 21 as libc::c_int
    {
        if (*mm).identity == 7500 as libc::c_int || (*mm).identity == 7600 as libc::c_int
            || (*mm).identity == 7700 as libc::c_int
        {
            emergency = -(1 as libc::c_int);
        }
        if (*mm).fs == 1 as libc::c_int || (*mm).fs == 3 as libc::c_int {
            ground = -(1 as libc::c_int);
        }
        if (*mm).fs == 2 as libc::c_int || (*mm).fs == 3 as libc::c_int
            || (*mm).fs == 4 as libc::c_int
        {
            alert = -(1 as libc::c_int);
        }
        if (*mm).fs == 4 as libc::c_int || (*mm).fs == 5 as libc::c_int {
            spi = -(1 as libc::c_int);
        }
    }
    if (*mm).msgtype == 0 as libc::c_int {
        p = p
            .offset(
                sprintf(
                    p,
                    b"MSG,5,,,%02X%02X%02X,,,,,,,%d,,,,,,,,,,\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                    (*mm).altitude,
                ) as isize,
            );
    } else if (*mm).msgtype == 4 as libc::c_int {
        p = p
            .offset(
                sprintf(
                    p,
                    b"MSG,5,,,%02X%02X%02X,,,,,,,%d,,,,,,,%d,%d,%d,%d\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                    (*mm).altitude,
                    alert,
                    emergency,
                    spi,
                    ground,
                ) as isize,
            );
    } else if (*mm).msgtype == 5 as libc::c_int {
        p = p
            .offset(
                sprintf(
                    p,
                    b"MSG,6,,,%02X%02X%02X,,,,,,,,,,,,,%d,%d,%d,%d,%d\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                    (*mm).identity,
                    alert,
                    emergency,
                    spi,
                    ground,
                ) as isize,
            );
    } else if (*mm).msgtype == 11 as libc::c_int {
        p = p
            .offset(
                sprintf(
                    p,
                    b"MSG,8,,,%02X%02X%02X,,,,,,,,,,,,,,,,,\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                ) as isize,
            );
    } else if (*mm).msgtype == 17 as libc::c_int && (*mm).metype == 4 as libc::c_int {
        p = p
            .offset(
                sprintf(
                    p,
                    b"MSG,1,,,%02X%02X%02X,,,,,,%s,,,,,,,,0,0,0,0\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                    ((*mm).flight).as_mut_ptr(),
                ) as isize,
            );
    } else if (*mm).msgtype == 17 as libc::c_int && (*mm).metype >= 9 as libc::c_int
        && (*mm).metype <= 18 as libc::c_int
    {
        if (*a).lat == 0 as libc::c_int as libc::c_double
            && (*a).lon == 0 as libc::c_int as libc::c_double
        {
            p = p
                .offset(
                    sprintf(
                        p,
                        b"MSG,3,,,%02X%02X%02X,,,,,,,%d,,,,,,,0,0,0,0\0" as *const u8
                            as *const libc::c_char,
                        (*mm).aa1,
                        (*mm).aa2,
                        (*mm).aa3,
                        (*mm).altitude,
                    ) as isize,
                );
        } else {
            p = p
                .offset(
                    sprintf(
                        p,
                        b"MSG,3,,,%02X%02X%02X,,,,,,,%d,,,%1.5f,%1.5f,,,0,0,0,0\0"
                            as *const u8 as *const libc::c_char,
                        (*mm).aa1,
                        (*mm).aa2,
                        (*mm).aa3,
                        (*mm).altitude,
                        (*a).lat,
                        (*a).lon,
                    ) as isize,
                );
        }
    } else if (*mm).msgtype == 17 as libc::c_int && (*mm).metype == 19 as libc::c_int
        && (*mm).mesub == 1 as libc::c_int
    {
        let mut vr: libc::c_int = (if (*mm).vert_rate_sign == 0 as libc::c_int {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) * ((*mm).vert_rate - 1 as libc::c_int) * 64 as libc::c_int;
        p = p
            .offset(
                sprintf(
                    p,
                    b"MSG,4,,,%02X%02X%02X,,,,,,,,%d,%d,,,%i,,0,0,0,0\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                    (*a).speed,
                    (*a).track,
                    vr,
                ) as isize,
            );
    } else if (*mm).msgtype == 21 as libc::c_int {
        p = p
            .offset(
                sprintf(
                    p,
                    b"MSG,6,,,%02X%02X%02X,,,,,,,,,,,,,%d,%d,%d,%d,%d\0" as *const u8
                        as *const libc::c_char,
                    (*mm).aa1,
                    (*mm).aa2,
                    (*mm).aa3,
                    (*mm).identity,
                    alert,
                    emergency,
                    spi,
                    ground,
                ) as isize,
            );
    } else {
        return
    }
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = '\n' as i32 as libc::c_char;
    modesSendAllClients(
        Modes.sbsos,
        msg.as_mut_ptr() as *mut libc::c_void,
        p.offset_from(msg.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
}
pub unsafe extern "C" fn hexDigitVal(mut c: libc::c_int) -> libc::c_int {
    c = ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = c;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(c);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(c as isize);
        }
        __res
    });
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn decodeHexMessage(mut c: *mut client) -> libc::c_int {
    let mut hex: *mut libc::c_char = ((*c).buf).as_mut_ptr();
    let mut l: libc::c_int = strlen(hex) as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut msg: [libc::c_uchar; 14] = [0; 14];
    let mut mm: modesMessage = modesMessage {
        msg: [0; 14],
        msgbits: 0,
        msgtype: 0,
        crcok: 0,
        crc: 0,
        errorbit: 0,
        aa1: 0,
        aa2: 0,
        aa3: 0,
        phase_corrected: 0,
        ca: 0,
        metype: 0,
        mesub: 0,
        heading_is_valid: 0,
        heading: 0,
        aircraft_type: 0,
        fflag: 0,
        tflag: 0,
        raw_latitude: 0,
        raw_longitude: 0,
        flight: [0; 9],
        ew_dir: 0,
        ew_velocity: 0,
        ns_dir: 0,
        ns_velocity: 0,
        vert_rate_source: 0,
        vert_rate_sign: 0,
        vert_rate: 0,
        velocity: 0,
        fs: 0,
        dr: 0,
        um: 0,
        identity: 0,
        altitude: 0,
        unit: 0,
    };
    while l != 0
        && *(*__ctype_b_loc())
            .offset(*hex.offset((l - 1 as libc::c_int) as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        *hex.offset((l - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        l -= 1;
        l;
    }
    while *(*__ctype_b_loc()).offset(*hex as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        hex = hex.offset(1);
        hex;
        l -= 1;
        l;
    }
    if l < 2 as libc::c_int
        || *hex.offset(0 as libc::c_int as isize) as libc::c_int != '*' as i32
        || *hex.offset((l - 1 as libc::c_int) as isize) as libc::c_int != ';' as i32
    {
        return 0 as libc::c_int;
    }
    hex = hex.offset(1);
    hex;
    l -= 2 as libc::c_int;
    if l > 112 as libc::c_int / 8 as libc::c_int * 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < l {
        let mut high: libc::c_int = hexDigitVal(*hex.offset(j as isize) as libc::c_int);
        let mut low: libc::c_int = hexDigitVal(
            *hex.offset((j + 1 as libc::c_int) as isize) as libc::c_int,
        );
        if high == -(1 as libc::c_int) || low == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
        msg[(j / 2 as libc::c_int)
            as usize] = (high << 4 as libc::c_int | low) as libc::c_uchar;
        j += 2 as libc::c_int;
    }
    decodeModesMessage(&mut mm, msg.as_mut_ptr());
    useModesMessage(&mut mm);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn aircraftsToJson(
    mut len: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut a: *mut aircraft = Modes.aircrafts;
    let mut buflen: libc::c_int = 1024 as libc::c_int;
    let mut buf: *mut libc::c_char = malloc(buflen as libc::c_ulong)
        as *mut libc::c_char;
    let mut p: *mut libc::c_char = buf;
    let mut l: libc::c_int = 0;
    l = snprintf(
        p,
        buflen as libc::c_ulong,
        b"[\n\0" as *const u8 as *const libc::c_char,
    );
    p = p.offset(l as isize);
    buflen -= l;
    while !a.is_null() {
        let mut altitude: libc::c_int = (*a).altitude;
        let mut speed: libc::c_int = (*a).speed;
        if Modes.metric != 0 {
            altitude = (altitude as libc::c_double / 3.2828f64) as libc::c_int;
            speed = (speed as libc::c_double * 1.852f64) as libc::c_int;
        }
        if (*a).lat != 0 as libc::c_int as libc::c_double
            && (*a).lon != 0 as libc::c_int as libc::c_double
        {
            l = snprintf(
                p,
                buflen as libc::c_ulong,
                b"{\"hex\":\"%s\", \"flight\":\"%s\", \"lat\":%f, \"lon\":%f, \"altitude\":%d, \"track\":%d, \"speed\":%d},\n\0"
                    as *const u8 as *const libc::c_char,
                ((*a).hexaddr).as_mut_ptr(),
                ((*a).flight).as_mut_ptr(),
                (*a).lat,
                (*a).lon,
                (*a).altitude,
                (*a).track,
                (*a).speed,
            );
            p = p.offset(l as isize);
            buflen -= l;
            if buflen < 256 as libc::c_int {
                let mut used: libc::c_int = p.offset_from(buf) as libc::c_long
                    as libc::c_int;
                buflen += 1024 as libc::c_int;
                buf = realloc(buf as *mut libc::c_void, (used + buflen) as libc::c_ulong)
                    as *mut libc::c_char;
                p = buf.offset(used as isize);
            }
        }
        a = (*a).next;
    }
    if *p.offset(-(2 as libc::c_int as isize)) as libc::c_int == ',' as i32 {
        *p.offset(-(2 as libc::c_int as isize)) = '\n' as i32 as libc::c_char;
        p = p.offset(-1);
        p;
        buflen += 1;
        buflen;
    }
    l = snprintf(
        p,
        buflen as libc::c_ulong,
        b"]\n\0" as *const u8 as *const libc::c_char,
    );
    p = p.offset(l as isize);
    buflen -= l;
    *len = p.offset_from(buf) as libc::c_long as libc::c_int;
    return buf;
}
pub unsafe extern "C" fn handleHTTPRequest(mut c: *mut client) -> libc::c_int {
    let mut hdr: [libc::c_char; 512] = [0; 512];
    let mut clen: libc::c_int = 0;
    let mut hdrlen: libc::c_int = 0;
    let mut httpver: libc::c_int = 0;
    let mut keepalive: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut content: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ctype: *mut libc::c_char = 0 as *mut libc::c_char;
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(
            b"\nHTTP request: %s\n\0" as *const u8 as *const libc::c_char,
            ((*c).buf).as_mut_ptr(),
        );
    }
    httpver = if !(strstr(
        ((*c).buf).as_mut_ptr(),
        b"HTTP/1.1\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        11 as libc::c_int
    } else {
        10 as libc::c_int
    };
    if httpver == 10 as libc::c_int {
        keepalive = (strstr(
            ((*c).buf).as_mut_ptr(),
            b"Connection: keep-alive\0" as *const u8 as *const libc::c_char,
        ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    } else if httpver == 11 as libc::c_int {
        keepalive = (strstr(
            ((*c).buf).as_mut_ptr(),
            b"Connection: close\0" as *const u8 as *const libc::c_char,
        ) == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    }
    p = strchr(((*c).buf).as_mut_ptr(), ' ' as i32);
    if p.is_null() {
        return 1 as libc::c_int;
    }
    p = p.offset(1);
    url = p;
    p = strchr(p, ' ' as i32);
    if p.is_null() {
        return 1 as libc::c_int;
    }
    *p = '\0' as i32 as libc::c_char;
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(
            b"\nHTTP keep alive: %d\n\0" as *const u8 as *const libc::c_char,
            keepalive,
        );
        printf(b"HTTP requested URL: %s\n\n\0" as *const u8 as *const libc::c_char, url);
    }
    if !(strstr(url, b"/data.json\0" as *const u8 as *const libc::c_char)).is_null() {
        content = aircraftsToJson(&mut clen);
        ctype = b"application/json;charset=utf-8\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        let mut sbuf: stat = stat {
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
        let mut fd: libc::c_int = -(1 as libc::c_int);
        if stat(b"gmap.html\0" as *const u8 as *const libc::c_char, &mut sbuf)
            != -(1 as libc::c_int)
            && {
                fd = open(
                    b"gmap.html\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                fd != -(1 as libc::c_int)
            }
        {
            content = malloc(sbuf.st_size as libc::c_ulong) as *mut libc::c_char;
            if read(fd, content as *mut libc::c_void, sbuf.st_size as size_t)
                == -(1 as libc::c_int) as libc::c_long
            {
                snprintf(
                    content,
                    sbuf.st_size as libc::c_ulong,
                    b"Error reading from file: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            clen = sbuf.st_size as libc::c_int;
        } else {
            let mut buf: [libc::c_char; 128] = [0; 128];
            clen = snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"Error opening HTML file: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            content = strdup(buf.as_mut_ptr());
        }
        if fd != -(1 as libc::c_int) {
            close(fd);
        }
        ctype = b"text/html;charset=utf-8\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    hdrlen = snprintf(
        hdr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"HTTP/1.1 200 OK\r\nServer: Dump1090\r\nContent-Type: %s\r\nConnection: %s\r\nContent-Length: %d\r\nAccess-Control-Allow-Origin: *\r\n\r\n\0"
            as *const u8 as *const libc::c_char,
        ctype,
        if keepalive != 0 {
            b"keep-alive\0" as *const u8 as *const libc::c_char
        } else {
            b"close\0" as *const u8 as *const libc::c_char
        },
        clen,
    );
    if Modes.debug & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        printf(
            b"HTTP Reply header:\n%s\0" as *const u8 as *const libc::c_char,
            hdr.as_mut_ptr(),
        );
    }
    if write((*c).fd, hdr.as_mut_ptr() as *const libc::c_void, hdrlen as size_t)
        != hdrlen as libc::c_long
        || write((*c).fd, content as *const libc::c_void, clen as size_t)
            != clen as libc::c_long
    {
        free(content as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    free(content as *mut libc::c_void);
    Modes.stat_http_requests += 1;
    Modes.stat_http_requests;
    return (keepalive == 0) as libc::c_int;
}
pub unsafe extern "C" fn modesReadFromClient(
    mut c: *mut client,
    mut sep: *mut libc::c_char,
    mut handler: Option::<unsafe extern "C" fn(*mut client) -> libc::c_int>,
) {
    loop {
        let mut left: libc::c_int = 1024 as libc::c_int - (*c).buflen;
        let mut nread: libc::c_int = read(
            (*c).fd,
            ((*c).buf).as_mut_ptr().offset((*c).buflen as isize) as *mut libc::c_void,
            left as size_t,
        ) as libc::c_int;
        let mut fullmsg: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if nread <= 0 as libc::c_int {
            if nread == 0 as libc::c_int || *__errno_location() != 11 as libc::c_int {
                modesFreeClient((*c).fd);
            }
            break;
        } else {
            (*c).buflen += nread;
            (*c).buf[(*c).buflen as usize] = '\0' as i32 as libc::c_char;
            loop {
                p = strstr(((*c).buf).as_mut_ptr(), sep);
                if p.is_null() {
                    break;
                }
                i = p.offset_from(((*c).buf).as_mut_ptr()) as libc::c_long
                    as libc::c_int;
                (*c).buf[i as usize] = '\0' as i32 as libc::c_char;
                if handler.unwrap()(c) != 0 {
                    modesFreeClient((*c).fd);
                    return;
                }
                i = (i as libc::c_ulong).wrapping_add(strlen(sep)) as libc::c_int
                    as libc::c_int;
                memmove(
                    ((*c).buf).as_mut_ptr() as *mut libc::c_void,
                    ((*c).buf).as_mut_ptr().offset(i as isize) as *const libc::c_void,
                    ((*c).buflen - i) as libc::c_ulong,
                );
                (*c).buflen -= i;
                (*c).buf[(*c).buflen as usize] = '\0' as i32 as libc::c_char;
                fullmsg = 1 as libc::c_int;
            }
            if (*c).buflen == 1024 as libc::c_int {
                (*c).buflen = 0 as libc::c_int;
            } else if fullmsg == 0 {
                break;
            }
        }
    };
}
pub unsafe extern "C" fn modesReadFromClients() {
    let mut j: libc::c_int = 0;
    let mut c: *mut client = 0 as *mut client;
    j = 0 as libc::c_int;
    while j <= Modes.maxfd {
        c = Modes.clients[j as usize];
        if !c.is_null() {
            if (*c).service == Modes.ris {
                modesReadFromClient(
                    c,
                    b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    Some(
                        decodeHexMessage
                            as unsafe extern "C" fn(*mut client) -> libc::c_int,
                    ),
                );
            } else if (*c).service == Modes.https {
                modesReadFromClient(
                    c,
                    b"\r\n\r\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    Some(
                        handleHTTPRequest
                            as unsafe extern "C" fn(*mut client) -> libc::c_int,
                    ),
                );
            }
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn modesWaitReadableClients(mut timeout_ms: libc::c_int) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut j: libc::c_int = 0;
    let mut maxfd: libc::c_int = Modes.maxfd;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh4 = &mut __d0;
    let fresh5;
    let fresh6 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh7 = &mut __d1;
    let fresh8;
    let fresh9 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh4,
        fresh6) => fresh5, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh7,
        fresh9) => fresh8, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
    c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
    j = 0 as libc::c_int;
    while j <= Modes.maxfd {
        if !(Modes.clients[j as usize]).is_null() {
            fds
                .__fds_bits[(j
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << j
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
        }
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        let mut s: libc::c_int = *modesNetServices[j as usize].socket;
        fds
            .__fds_bits[(s
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << s
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        if s > maxfd {
            maxfd = s;
        }
        j += 1;
        j;
    }
    tv.tv_sec = (timeout_ms / 1000 as libc::c_int) as __time_t;
    tv
        .tv_usec = (timeout_ms % 1000 as libc::c_int * 1000 as libc::c_int)
        as __suseconds_t;
    select(
        maxfd + 1 as libc::c_int,
        &mut fds,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut tv,
    );
}
pub unsafe extern "C" fn sigWinchCallback() {
    signal(
        28 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    Modes.interactive_rows = getTermRows();
    interactiveShowData();
    signal(
        28 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            __sighandler_t,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(sigWinchCallback),
            ),
        ),
    );
}
pub unsafe extern "C" fn getTermRows() -> libc::c_int {
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    ioctl(
        1 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut w as *mut winsize,
    );
    return w.ws_row as libc::c_int;
}
pub unsafe extern "C" fn showHelp() {
    printf(
        b"--device-index <index>   Select RTL device (default: 0).\n--gain <db>              Set gain (default: max gain. Use -100 for auto-gain).\n--enable-agc             Enable the Automatic Gain Control (default: off).\n--freq <hz>              Set frequency (default: 1090 Mhz).\n--ifile <filename>       Read data from file (use '-' for stdin).\n--loop                   With --ifile, read the same file in a loop.\n--interactive            Interactive mode refreshing data on screen.\n--interactive-rows <num> Max number of rows in interactive mode (default: 15).\n--interactive-ttl <sec>  Remove from list if idle for <sec> (default: 60).\n--raw                    Show only messages hex values.\n--net                    Enable networking.\n--net-only               Enable just networking, no RTL device or file used.\n--net-ro-port <port>     TCP listening port for raw output (default: 30002).\n--net-ri-port <port>     TCP listening port for raw input (default: 30001).\n--net-http-port <port>   HTTP server port (default: 8080).\n--net-sbs-port <port>    TCP listening port for BaseStation format output (default: 30003).\n--no-fix                 Disable single-bits error correction using CRC.\n--no-crc-check           Disable messages with broken CRC (discouraged).\n--aggressive             More CPU for more messages (two bits fixes, ...).\n--stats                  With --ifile print stats at exit. No other output.\n--onlyaddr               Show only ICAO addresses (testing purposes).\n--metric                 Use metric units (meters, km/h, ...).\n--snip <level>           Strip IQ file removing samples < level.\n--debug <flags>          Debug mode (verbose), see README for details.\n--help                   Show this help.\n\nDebug mode flags: d = Log frames decoded with errors\n                  D = Log frames decoded with zero errors\n                  c = Log frames with bad CRC\n                  C = Log frames with good CRC\n                  p = Log frames with bad preamble\n                  n = Log network debugging info\n                  j = Log frames to frames.js, loadable by debug.html.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn backgroundTasks() {
    if Modes.net != 0 {
        modesAcceptClients();
        modesReadFromClients();
        interactiveRemoveStaleAircrafts();
    }
    if Modes.interactive != 0
        && mstime() - Modes.interactive_last_update
            > 250 as libc::c_int as libc::c_longlong
    {
        interactiveRemoveStaleAircrafts();
        interactiveShowData();
        Modes.interactive_last_update = mstime();
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    modesInitConfig();
    j = 1 as libc::c_int;
    while j < argc {
        let mut more: libc::c_int = ((j + 1 as libc::c_int) < argc) as libc::c_int;
        if strcmp(
            *argv.offset(j as isize),
            b"--device-index\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            Modes.dev_index = atoi(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--gain\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            Modes
                .gain = (atof(*argv.offset(j as isize))
                * 10 as libc::c_int as libc::c_double) as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--enable-agc\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.enable_agc += 1;
            Modes.enable_agc;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--freq\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            Modes
                .freq = strtoll(
                *argv.offset(j as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            ) as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--ifile\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            Modes.filename = strdup(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--loop\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.loop_0 = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--no-fix\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.fix_errors = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--no-crc-check\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.check_crc = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--raw\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.raw = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--net\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.net = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--net-only\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.net = 1 as libc::c_int;
            Modes.net_only = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--net-ro-port\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            modesNetServices[0 as libc::c_int as usize]
                .port = atoi(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--net-ri-port\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            modesNetServices[1 as libc::c_int as usize]
                .port = atoi(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--net-http-port\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            modesNetServices[2 as libc::c_int as usize]
                .port = atoi(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--net-sbs-port\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            modesNetServices[3 as libc::c_int as usize]
                .port = atoi(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--onlyaddr\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.onlyaddr = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--metric\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.metric = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--aggressive\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.aggressive += 1;
            Modes.aggressive;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--interactive\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.interactive = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--interactive-rows\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            j += 1;
            Modes.interactive_rows = atoi(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--interactive-ttl\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            j += 1;
            Modes.interactive_ttl = atoi(*argv.offset(j as isize));
        } else if strcmp(
            *argv.offset(j as isize),
            b"--debug\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            let mut f: *mut libc::c_char = *argv.offset(j as isize);
            while *f != 0 {
                match *f as libc::c_int {
                    68 => {
                        Modes.debug |= (1 as libc::c_int) << 0 as libc::c_int;
                    }
                    100 => {
                        Modes.debug |= (1 as libc::c_int) << 1 as libc::c_int;
                    }
                    67 => {
                        Modes.debug |= (1 as libc::c_int) << 3 as libc::c_int;
                    }
                    99 => {
                        Modes.debug |= (1 as libc::c_int) << 2 as libc::c_int;
                    }
                    112 => {
                        Modes.debug |= (1 as libc::c_int) << 4 as libc::c_int;
                    }
                    110 => {
                        Modes.debug |= (1 as libc::c_int) << 5 as libc::c_int;
                    }
                    106 => {
                        Modes.debug |= (1 as libc::c_int) << 6 as libc::c_int;
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"Unknown debugging flag: %c\n\0" as *const u8
                                as *const libc::c_char,
                            *f as libc::c_int,
                        );
                        exit(1 as libc::c_int);
                    }
                }
                f = f.offset(1);
                f;
            }
        } else if strcmp(
            *argv.offset(j as isize),
            b"--stats\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            Modes.stats = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(j as isize),
            b"--snip\0" as *const u8 as *const libc::c_char,
        ) == 0 && more != 0
        {
            j += 1;
            snipMode(atoi(*argv.offset(j as isize)));
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(j as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            showHelp();
            exit(0 as libc::c_int);
        } else {
            fprintf(
                stderr,
                b"Unknown or not enough arguments for option '%s'.\n\n\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(j as isize),
            );
            showHelp();
            exit(1 as libc::c_int);
        }
        j += 1;
        j;
    }
    if Modes.interactive == 1 as libc::c_int {
        signal(
            28 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                __sighandler_t,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(sigWinchCallback),
                ),
            ),
        );
    }
    modesInit();
    if Modes.net_only != 0 {
        fprintf(
            stderr,
            b"Net-only mode, no RTL device or file open.\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if (Modes.filename).is_null() {
        modesInitRTLSDR();
    } else if *(Modes.filename).offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32
        && *(Modes.filename).offset(1 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        Modes.fd = 0 as libc::c_int;
    } else {
        Modes.fd = open(Modes.filename, 0 as libc::c_int);
        if Modes.fd == -(1 as libc::c_int) {
            perror(b"Opening data file\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    if Modes.net != 0 {
        modesInitNet();
    }
    while Modes.net_only != 0 {
        backgroundTasks();
        modesWaitReadableClients(100 as libc::c_int);
    }
    pthread_create(
        &mut Modes.reader_thread,
        0 as *const pthread_attr_t,
        Some(
            readerThreadEntryPoint
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    pthread_mutex_lock(&mut Modes.data_mutex);
    loop {
        if Modes.data_ready == 0 {
            pthread_cond_wait(&mut Modes.data_cond, &mut Modes.data_mutex);
        } else {
            computeMagnitudeVector();
            Modes.data_ready = 0 as libc::c_int;
            pthread_cond_signal(&mut Modes.data_cond);
            pthread_mutex_unlock(&mut Modes.data_mutex);
            detectModeS(
                Modes.magnitude,
                (Modes.data_len).wrapping_div(2 as libc::c_int as libc::c_uint),
            );
            backgroundTasks();
            pthread_mutex_lock(&mut Modes.data_mutex);
            if Modes.exit != 0 {
                break;
            }
        }
    }
    if Modes.stats != 0 && !(Modes.filename).is_null() {
        printf(
            b"%lld valid preambles\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_valid_preamble,
        );
        printf(
            b"%lld demodulated again after phase correction\n\0" as *const u8
                as *const libc::c_char,
            Modes.stat_out_of_phase,
        );
        printf(
            b"%lld demodulated with zero errors\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_demodulated,
        );
        printf(
            b"%lld with good crc\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_goodcrc,
        );
        printf(
            b"%lld with bad crc\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_badcrc,
        );
        printf(
            b"%lld errors corrected\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_fixed,
        );
        printf(
            b"%lld single bit errors\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_single_bit_fix,
        );
        printf(
            b"%lld two bits errors\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_two_bits_fix,
        );
        printf(
            b"%lld total usable messages\n\0" as *const u8 as *const libc::c_char,
            Modes.stat_goodcrc + Modes.stat_fixed,
        );
    }
    rtlsdr_close(Modes.dev);
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
unsafe extern "C" fn run_static_initializers() {
    modesNetServices = [
        {
            let mut init = C2RustUnnamed_5 {
                descr: b"Raw TCP output\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.ros,
                port: 30002 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                descr: b"Raw TCP input\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.ris,
                port: 30001 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                descr: b"HTTP server\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.https,
                port: 8080 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                descr: b"Basestation TCP output\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                socket: &mut Modes.sbsos,
                port: 30003 as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
