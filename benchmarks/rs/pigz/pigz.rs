use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type internal_state;
    pub type thread_s;
    pub type lock_s;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn zlibVersion() -> *const libc::c_char;
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    fn deflateSetDictionary(
        strm: z_streamp,
        dictionary: *const Bytef,
        dictLength: uInt,
    ) -> libc::c_int;
    fn deflateReset(strm: z_streamp) -> libc::c_int;
    fn deflateParams(
        strm: z_streamp,
        level: libc::c_int,
        strategy: libc::c_int,
    ) -> libc::c_int;
    fn deflatePending(
        strm: z_streamp,
        pending: *mut libc::c_uint,
        bits: *mut libc::c_int,
    ) -> libc::c_int;
    fn deflatePrime(
        strm: z_streamp,
        bits: libc::c_int,
        value: libc::c_int,
    ) -> libc::c_int;
    fn inflateBack(
        strm: z_streamp,
        in_0: in_func,
        in_desc: *mut libc::c_void,
        out: out_func,
        out_desc: *mut libc::c_void,
    ) -> libc::c_int;
    fn inflateBackEnd(strm: z_streamp) -> libc::c_int;
    fn adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn get_crc_table() -> *const z_crc_t;
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn deflateInit2_(
        strm: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn inflateBackInit_(
        strm: z_streamp,
        windowBits: libc::c_int,
        window: *mut libc::c_uchar,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    static mut yarn_prefix: *mut libc::c_char;
    static mut yarn_abort: Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn launch_(
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_long,
    ) -> *mut thread;
    fn join_(_: *mut thread, _: *const libc::c_char, _: libc::c_long);
    fn join_all_(_: *const libc::c_char, _: libc::c_long) -> libc::c_int;
    fn new_lock_(_: libc::c_long, _: *const libc::c_char, _: libc::c_long) -> *mut lock;
    fn possess_(_: *mut lock, _: *const libc::c_char, _: libc::c_long);
    fn release_(_: *mut lock, _: *const libc::c_char, _: libc::c_long);
    fn twist_(
        _: *mut lock,
        _: twist_op,
        _: libc::c_long,
        _: *const libc::c_char,
        _: libc::c_long,
    );
    fn wait_for_(
        _: *mut lock,
        _: wait_op,
        _: libc::c_long,
        _: *const libc::c_char,
        _: libc::c_long,
    );
    fn peek_lock(_: *mut lock) -> libc::c_long;
    fn free_lock_(_: *mut lock, _: *const libc::c_char, _: libc::c_long);
    fn ZopfliDeflatePart(
        options: *const ZopfliOptions,
        btype: libc::c_int,
        final_0: libc::c_int,
        in_0: *const libc::c_uchar,
        instart: size_t,
        inend: size_t,
        bp: *mut libc::c_uchar,
        out: *mut *mut libc::c_uchar,
        outsize: *mut size_t,
    );
    fn ZopfliInitOptions(options: *mut ZopfliOptions);
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> libc::c_int;
    static mut try_key_: pthread_key_t;
    fn try_setup_();
    fn try_throw_(code: libc::c_int, fmt: *mut libc::c_char, _: ...) -> !;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint_least16_t = __uint16_t;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
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
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type pthread_key_t = libc::c_uint;
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
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type uint32_t = __uint32_t;
pub type uint_least16_t = __uint_least16_t;
pub type uintmax_t = __uintmax_t;
pub type length_t = uintmax_t;
pub type crc_t = uint32_t;
pub type index_t = uint_least16_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type z_crc_t = libc::c_uint;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
pub type in_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_uchar) -> libc::c_uint,
>;
pub type out_func = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_uchar,
        libc::c_uint,
    ) -> libc::c_int,
>;
pub type thread = thread_s;
pub type lock = lock_s;
pub type twist_op = libc::c_uint;
pub const BY: twist_op = 1;
pub const TO: twist_op = 0;
pub type wait_op = libc::c_uint;
pub const TO_BE_LESS_THAN: wait_op = 3;
pub const TO_BE_MORE_THAN: wait_op = 2;
pub const NOT_TO_BE: wait_op = 1;
pub const TO_BE: wait_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: libc::c_int,
    pub verbose_more: libc::c_int,
    pub numiterations: libc::c_int,
    pub blocksplitting: libc::c_int,
    pub blocksplittinglast: libc::c_int,
    pub blocksplittingmax: libc::c_int,
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct try_ball_t_ {
    pub ret: libc::c_int,
    pub code: libc::c_int,
    pub free: libc::c_int,
    pub why: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct try_s_ {
    pub env: jmp_buf,
    pub ball: try_ball_t_,
    pub next: *mut try_t_,
}
pub type try_t_ = try_s_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ret: libc::c_int,
    pub prog: *mut libc::c_char,
    pub ind: libc::c_int,
    pub outd: libc::c_int,
    pub inf: *mut libc::c_char,
    pub inz: size_t,
    pub outf: *mut libc::c_char,
    pub verbosity: libc::c_int,
    pub headis: libc::c_int,
    pub pipeout: libc::c_int,
    pub keep: libc::c_int,
    pub force: libc::c_int,
    pub sync: libc::c_int,
    pub form: libc::c_int,
    pub magic1: libc::c_int,
    pub recurse: libc::c_int,
    pub sufx: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub mtime: time_t,
    pub list: libc::c_int,
    pub first: libc::c_int,
    pub decode: libc::c_int,
    pub level: libc::c_int,
    pub strategy: libc::c_int,
    pub zopts: ZopfliOptions,
    pub rsync: libc::c_int,
    pub procs: libc::c_int,
    pub setdict: libc::c_int,
    pub block: size_t,
    pub shift: crc_t,
    pub stamp: time_t,
    pub hname: *mut libc::c_char,
    pub hcomm: *mut libc::c_char,
    pub zip_crc: libc::c_ulong,
    pub zip_clen: length_t,
    pub zip_ulen: length_t,
    pub zip64: libc::c_int,
    pub in_buf: [libc::c_uchar; 32810],
    pub in_next: *mut libc::c_uchar,
    pub in_left: size_t,
    pub in_eof: libc::c_int,
    pub in_short: libc::c_int,
    pub in_tot: length_t,
    pub out_tot: length_t,
    pub out_check: libc::c_ulong,
    pub in_buf2: [libc::c_uchar; 32810],
    pub in_len: size_t,
    pub in_which: libc::c_int,
    pub load_state: *mut lock,
    pub load_thread: *mut thread,
}
pub type val_t = length_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct space {
    pub use_0: *mut lock,
    pub buf: *mut libc::c_uchar,
    pub size: size_t,
    pub len: size_t,
    pub pool: *mut pool,
    pub next: *mut space,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool {
    pub have: *mut lock,
    pub head: *mut space,
    pub size: size_t,
    pub limit: libc::c_int,
    pub made: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct job {
    pub seq: libc::c_long,
    pub more: libc::c_int,
    pub in_0: *mut space,
    pub out: *mut space,
    pub lens: *mut space,
    pub check: libc::c_ulong,
    pub calc: *mut lock,
    pub next: *mut job,
}
pub type bits_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
static mut g: C2RustUnnamed_0 = C2RustUnnamed_0 {
    ret: 0,
    prog: 0 as *const libc::c_char as *mut libc::c_char,
    ind: 0,
    outd: 0,
    inf: 0 as *const libc::c_char as *mut libc::c_char,
    inz: 0,
    outf: 0 as *const libc::c_char as *mut libc::c_char,
    verbosity: 0,
    headis: 0,
    pipeout: 0,
    keep: 0,
    force: 0,
    sync: 0,
    form: 0,
    magic1: 0,
    recurse: 0,
    sufx: 0 as *const libc::c_char as *mut libc::c_char,
    name: 0 as *const libc::c_char as *mut libc::c_char,
    alias: 0 as *const libc::c_char as *mut libc::c_char,
    comment: 0 as *const libc::c_char as *mut libc::c_char,
    mtime: 0,
    list: 0,
    first: 0,
    decode: 0,
    level: 0,
    strategy: 0,
    zopts: ZopfliOptions {
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    },
    rsync: 0,
    procs: 0,
    setdict: 0,
    block: 0,
    shift: 0,
    stamp: 0,
    hname: 0 as *const libc::c_char as *mut libc::c_char,
    hcomm: 0 as *const libc::c_char as *mut libc::c_char,
    zip_crc: 0,
    zip_clen: 0,
    zip_ulen: 0,
    zip64: 0,
    in_buf: [0; 32810],
    in_next: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    in_left: 0,
    in_eof: 0,
    in_short: 0,
    in_tot: 0,
    out_tot: 0,
    out_check: 0,
    in_buf2: [0; 32810],
    in_len: 0,
    in_which: 0,
    load_state: 0 as *const lock as *mut lock,
    load_thread: 0 as *const thread as *mut thread,
};
unsafe extern "C" fn message(mut fmt: *mut libc::c_char, mut ap: ::std::ffi::VaList) {
    if g.verbosity > 0 as libc::c_int {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, g.prog);
        vfprintf(stderr, fmt, ap.as_va_list());
        putc('\n' as i32, stderr);
        fflush(stderr);
    }
}
unsafe extern "C" fn complain(mut fmt: *mut libc::c_char, mut args: ...) -> libc::c_int {
    g.ret = 1 as libc::c_int;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    message(fmt, ap.as_va_list());
    return 0 as libc::c_int;
}
unsafe extern "C" fn grumble(mut fmt: *mut libc::c_char, mut args: ...) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    message(fmt, ap.as_va_list());
    return 0 as libc::c_int;
}
unsafe extern "C" fn alloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    ptr = realloc(ptr, size);
    if ptr.is_null() {
        try_throw_(
            12 as libc::c_int,
            b"not enough memory\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    return ptr;
}
unsafe extern "C" fn cut_short(mut sig: libc::c_int) {
    sig == 2 as libc::c_int;
    if g.outd != -(1 as libc::c_int) && g.outd != 1 as libc::c_int {
        unlink(g.outf);
        if !(g.outf).is_null() {
            free(g.outf as *mut libc::c_void);
            g.outf = 0 as *mut libc::c_char;
        }
        g.outd = -(1 as libc::c_int);
    }
    _exit(if sig < 0 as libc::c_int { -sig } else { 4 as libc::c_int });
}
#[inline]
unsafe extern "C" fn grow(mut size: size_t) -> size_t {
    let mut was: size_t = 0;
    let mut top: size_t = 0;
    let mut shift: libc::c_int = 0;
    was = size;
    size = (size as libc::c_ulong).wrapping_add(size >> 2 as libc::c_int) as size_t
        as size_t;
    top = size;
    shift = 0 as libc::c_int;
    while top > 7 as libc::c_int as libc::c_ulong {
        top >>= 1 as libc::c_int;
        shift += 1;
        shift;
    }
    if top == 7 as libc::c_int as libc::c_ulong {
        size = (1 as libc::c_int as size_t) << shift + 3 as libc::c_int;
    }
    if size < 16 as libc::c_int as libc::c_ulong {
        size = 16 as libc::c_int as size_t;
    }
    if size <= was {
        size = (0 as libc::c_int as size_t)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    return size;
}
#[inline]
unsafe extern "C" fn vmemcpy(
    mut mem: *mut *mut libc::c_char,
    mut size: *mut size_t,
    mut off: size_t,
    mut cpy: *mut libc::c_void,
    mut len: size_t,
) -> size_t {
    let mut need: size_t = 0;
    need = off.wrapping_add(len);
    if need < off {
        try_throw_(
            34 as libc::c_int,
            b"overflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    if need > *size {
        need = grow(need);
        if off == 0 as libc::c_int as libc::c_ulong {
            if !(*mem).is_null() {
                free(*mem as *mut libc::c_void);
                *mem = 0 as *mut libc::c_char;
            }
            *size = 0 as libc::c_int as size_t;
        }
        *mem = alloc(*mem as *mut libc::c_void, need) as *mut libc::c_char;
        *size = need;
    }
    memcpy((*mem).offset(off as isize) as *mut libc::c_void, cpy, len);
    return off.wrapping_add(len);
}
#[inline]
unsafe extern "C" fn vstrcpy(
    mut str: *mut *mut libc::c_char,
    mut size: *mut size_t,
    mut off: size_t,
    mut cpy: *mut libc::c_void,
) -> size_t {
    return vmemcpy(
        str,
        size,
        off,
        cpy,
        (strlen(cpy as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn readn(
    mut desc: libc::c_int,
    mut buf: *mut libc::c_uchar,
    mut len: size_t,
) -> size_t {
    let mut ret: ssize_t = 0;
    let mut got: size_t = 0;
    got = 0 as libc::c_int as size_t;
    while len != 0 {
        ret = read(desc, buf as *mut libc::c_void, len);
        if ret < 0 as libc::c_int as libc::c_long {
            try_throw_(
                *__errno_location(),
                b"read error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                strerror(*__errno_location()),
                0 as *mut libc::c_void,
            );
        }
        if ret == 0 as libc::c_int as libc::c_long {
            break;
        }
        buf = buf.offset(ret as isize);
        len = (len as libc::c_ulong).wrapping_sub(ret as size_t) as size_t as size_t;
        got = (got as libc::c_ulong).wrapping_add(ret as size_t) as size_t as size_t;
    }
    return got;
}
unsafe extern "C" fn writen(
    mut desc: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    let mut next: *const libc::c_char = buf as *const libc::c_char;
    let mut left: size_t = len;
    while left != 0 {
        let max: size_t = 9223372036854775807 as libc::c_long as size_t;
        let mut ret: ssize_t = write(
            desc,
            next as *const libc::c_void,
            if left > max { max } else { left },
        );
        if ret < 1 as libc::c_int as libc::c_long {
            try_throw_(
                *__errno_location(),
                b"write error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.outf,
                strerror(*__errno_location()),
                0 as *mut libc::c_void,
            );
        }
        next = next.offset(ret as isize);
        left = (left as libc::c_ulong).wrapping_sub(ret as size_t) as size_t as size_t;
    }
    return len;
}
unsafe extern "C" fn time2dos(mut t: time_t) -> libc::c_ulong {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut dos: libc::c_ulong = 0;
    if t == 0 as libc::c_int as libc::c_long {
        t = time(0 as *mut time_t);
    }
    tm = localtime(&mut t);
    if (*tm).tm_year < 80 as libc::c_int || (*tm).tm_year > 207 as libc::c_int {
        return 0 as libc::c_int as libc::c_ulong;
    }
    dos = (((*tm).tm_year - 80 as libc::c_int) as libc::c_ulong) << 25 as libc::c_int;
    dos = dos
        .wrapping_add(
            (((*tm).tm_mon + 1 as libc::c_int) as libc::c_ulong) << 21 as libc::c_int,
        );
    dos = dos.wrapping_add(((*tm).tm_mday as libc::c_ulong) << 16 as libc::c_int);
    dos = dos.wrapping_add(((*tm).tm_hour as libc::c_ulong) << 11 as libc::c_int);
    dos = dos.wrapping_add(((*tm).tm_min as libc::c_ulong) << 5 as libc::c_int);
    dos = dos
        .wrapping_add(
            ((*tm).tm_sec + 1 as libc::c_int) as libc::c_ulong >> 1 as libc::c_int,
        );
    return dos;
}
unsafe extern "C" fn put(mut out: libc::c_int, mut args: ...) -> libc::c_uint {
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    loop {
        n = ap.arg::<libc::c_int>();
        if !(n != 0 as libc::c_int) {
            break;
        }
        ap.arg::<val_t>();
        count = count.wrapping_add(abs(n) as libc::c_uint);
    }
    let mut wrap: *mut libc::c_uchar = alloc(0 as *mut libc::c_void, count as size_t)
        as *mut libc::c_uchar;
    let mut next: *mut libc::c_uchar = wrap;
    ap = args.clone();
    loop {
        n = ap.arg::<libc::c_int>();
        if !(n != 0 as libc::c_int) {
            break;
        }
        let mut val: val_t = ap.arg::<val_t>();
        if n < 0 as libc::c_int {
            n = -n << 3 as libc::c_int;
            loop {
                n -= 8 as libc::c_int;
                let fresh0 = next;
                next = next.offset(1);
                *fresh0 = (val >> n) as libc::c_uchar;
                if !(n != 0) {
                    break;
                }
            }
        } else {
            loop {
                let fresh1 = next;
                next = next.offset(1);
                *fresh1 = val as libc::c_uchar;
                val >>= 8 as libc::c_int;
                n -= 1;
                if !(n != 0) {
                    break;
                }
            }
        }
    }
    writen(out, wrap as *const libc::c_void, count as size_t);
    free(wrap as *mut libc::c_void);
    return count;
}
unsafe extern "C" fn put_header() -> length_t {
    let mut len: length_t = 0;
    if g.form > 1 as libc::c_int {
        len = put(
            g.outd,
            4 as libc::c_int,
            0x4034b50 as libc::c_int as val_t,
            2 as libc::c_int,
            45 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            4 as libc::c_int,
            time2dos(g.mtime),
            4 as libc::c_int,
            0 as libc::c_int as val_t,
            4 as libc::c_int,
            0xffffffff as libc::c_uint as val_t,
            4 as libc::c_int,
            0xffffffff as libc::c_uint as val_t,
            2 as libc::c_int,
            strlen(if (g.name).is_null() { g.alias } else { g.name }),
            2 as libc::c_int,
            29 as libc::c_int as val_t,
            0 as libc::c_int,
        ) as length_t;
        len = (len as libc::c_ulong)
            .wrapping_add(
                writen(
                    g.outd,
                    (if (g.name).is_null() { g.alias } else { g.name })
                        as *const libc::c_void,
                    strlen(if (g.name).is_null() { g.alias } else { g.name }),
                ),
            ) as length_t as length_t;
        len = (len as libc::c_ulong)
            .wrapping_add(
                put(
                    g.outd,
                    2 as libc::c_int,
                    0x1 as libc::c_int as val_t,
                    2 as libc::c_int,
                    16 as libc::c_int as val_t,
                    8 as libc::c_int,
                    0 as libc::c_int as val_t,
                    8 as libc::c_int,
                    0 as libc::c_int as val_t,
                    2 as libc::c_int,
                    0x5455 as libc::c_int as val_t,
                    2 as libc::c_int,
                    5 as libc::c_int as val_t,
                    1 as libc::c_int,
                    1 as libc::c_int as val_t,
                    4 as libc::c_int,
                    g.mtime as val_t,
                    0 as libc::c_int,
                ) as libc::c_ulong,
            ) as length_t as length_t;
    } else if g.form != 0 {
        if !(g.comment).is_null() {
            complain(
                b"can't store comment in zlib format -- ignoring\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        let mut head: libc::c_uint = 0;
        head = (((0x78 as libc::c_int) << 8 as libc::c_int)
            + (if g.level >= 9 as libc::c_int {
                (3 as libc::c_int) << 6 as libc::c_int
            } else {
                (if g.level == 1 as libc::c_int {
                    (0 as libc::c_int) << 6 as libc::c_int
                } else {
                    (if g.level >= 6 as libc::c_int || g.level == -(1 as libc::c_int) {
                        (1 as libc::c_int) << 6 as libc::c_int
                    } else {
                        (2 as libc::c_int) << 6 as libc::c_int
                    })
                })
            })) as libc::c_uint;
        head = head
            .wrapping_add(
                (31 as libc::c_int as libc::c_uint)
                    .wrapping_sub(head.wrapping_rem(31 as libc::c_int as libc::c_uint)),
            );
        len = put(g.outd, -(2 as libc::c_int), head as val_t, 0 as libc::c_int)
            as length_t;
    } else {
        len = put(
            g.outd,
            1 as libc::c_int,
            31 as libc::c_int as val_t,
            1 as libc::c_int,
            139 as libc::c_int as val_t,
            1 as libc::c_int,
            8 as libc::c_int as val_t,
            1 as libc::c_int,
            ((if !(g.name).is_null() { 8 as libc::c_int } else { 0 as libc::c_int })
                + (if !(g.comment).is_null() {
                    16 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as val_t,
            4 as libc::c_int,
            g.mtime as val_t,
            1 as libc::c_int,
            (if g.level >= 9 as libc::c_int {
                2 as libc::c_int
            } else if g.level == 1 as libc::c_int {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            }) as val_t,
            1 as libc::c_int,
            3 as libc::c_int as val_t,
            0 as libc::c_int,
        ) as length_t;
        if !(g.name).is_null() {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    writen(
                        g.outd,
                        g.name as *const libc::c_void,
                        (strlen(g.name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
                ) as length_t as length_t;
        }
        if !(g.comment).is_null() {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    writen(
                        g.outd,
                        g.comment as *const libc::c_void,
                        (strlen(g.comment))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
                ) as length_t as length_t;
        }
    }
    return len;
}
unsafe extern "C" fn put_trailer(
    mut ulen: length_t,
    mut clen: length_t,
    mut check: libc::c_ulong,
    mut head: length_t,
) {
    if g.form > 1 as libc::c_int {
        let mut desc: length_t = put(
            g.outd,
            4 as libc::c_int,
            0x8074b50 as libc::c_int as val_t,
            4 as libc::c_int,
            check,
            8 as libc::c_int,
            clen,
            8 as libc::c_int,
            ulen,
            0 as libc::c_int,
        ) as length_t;
        let mut zip64: libc::c_int = (ulen >= 0xffffffff as libc::c_uint as libc::c_ulong
            || clen >= 0xffffffff as libc::c_uint as libc::c_ulong) as libc::c_int;
        let mut cent: length_t = put(
            g.outd,
            4 as libc::c_int,
            0x2014b50 as libc::c_int as val_t,
            1 as libc::c_int,
            45 as libc::c_int as val_t,
            1 as libc::c_int,
            255 as libc::c_int as val_t,
            2 as libc::c_int,
            45 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            2 as libc::c_int,
            8 as libc::c_int as val_t,
            4 as libc::c_int,
            time2dos(g.mtime),
            4 as libc::c_int,
            check,
            4 as libc::c_int,
            if zip64 != 0 { 0xffffffff as libc::c_uint as libc::c_ulong } else { clen },
            4 as libc::c_int,
            if zip64 != 0 { 0xffffffff as libc::c_uint as libc::c_ulong } else { ulen },
            2 as libc::c_int,
            strlen(if (g.name).is_null() { g.alias } else { g.name }),
            2 as libc::c_int,
            (if zip64 != 0 { 29 as libc::c_int } else { 9 as libc::c_int }) as val_t,
            2 as libc::c_int,
            if (g.comment).is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(g.comment)
            },
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            4 as libc::c_int,
            0 as libc::c_int as val_t,
            4 as libc::c_int,
            0 as libc::c_int as val_t,
            0 as libc::c_int,
        ) as length_t;
        cent = (cent as libc::c_ulong)
            .wrapping_add(
                writen(
                    g.outd,
                    (if (g.name).is_null() { g.alias } else { g.name })
                        as *const libc::c_void,
                    strlen(if (g.name).is_null() { g.alias } else { g.name }),
                ),
            ) as length_t as length_t;
        if zip64 != 0 {
            cent = (cent as libc::c_ulong)
                .wrapping_add(
                    put(
                        g.outd,
                        2 as libc::c_int,
                        0x1 as libc::c_int as val_t,
                        2 as libc::c_int,
                        16 as libc::c_int as val_t,
                        8 as libc::c_int,
                        ulen,
                        8 as libc::c_int,
                        clen,
                        0 as libc::c_int,
                    ) as libc::c_ulong,
                ) as length_t as length_t;
        }
        cent = (cent as libc::c_ulong)
            .wrapping_add(
                put(
                    g.outd,
                    2 as libc::c_int,
                    0x5455 as libc::c_int as val_t,
                    2 as libc::c_int,
                    5 as libc::c_int as val_t,
                    1 as libc::c_int,
                    1 as libc::c_int as val_t,
                    4 as libc::c_int,
                    g.mtime as val_t,
                    0 as libc::c_int,
                ) as libc::c_ulong,
            ) as length_t as length_t;
        if !(g.comment).is_null() {
            cent = (cent as libc::c_ulong)
                .wrapping_add(
                    writen(g.outd, g.comment as *const libc::c_void, strlen(g.comment)),
                ) as length_t as length_t;
        }
        zip64 = (head.wrapping_add(clen).wrapping_add(desc)
            >= 0xffffffff as libc::c_uint as libc::c_ulong) as libc::c_int;
        if zip64 != 0 {
            put(
                g.outd,
                4 as libc::c_int,
                0x6064b50 as libc::c_int as val_t,
                8 as libc::c_int,
                44 as libc::c_int as val_t,
                2 as libc::c_int,
                45 as libc::c_int as val_t,
                2 as libc::c_int,
                45 as libc::c_int as val_t,
                4 as libc::c_int,
                0 as libc::c_int as val_t,
                4 as libc::c_int,
                0 as libc::c_int as val_t,
                8 as libc::c_int,
                1 as libc::c_int as val_t,
                8 as libc::c_int,
                1 as libc::c_int as val_t,
                8 as libc::c_int,
                cent,
                8 as libc::c_int,
                head.wrapping_add(clen).wrapping_add(desc),
                4 as libc::c_int,
                0x7064b50 as libc::c_int as val_t,
                4 as libc::c_int,
                0 as libc::c_int as val_t,
                8 as libc::c_int,
                head.wrapping_add(clen).wrapping_add(desc).wrapping_add(cent),
                4 as libc::c_int,
                1 as libc::c_int as val_t,
                0 as libc::c_int,
            );
        }
        put(
            g.outd,
            4 as libc::c_int,
            0x6054b50 as libc::c_int as val_t,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            2 as libc::c_int,
            (if zip64 != 0 { 0xffff as libc::c_int } else { 1 as libc::c_int }) as val_t,
            2 as libc::c_int,
            (if zip64 != 0 { 0xffff as libc::c_int } else { 1 as libc::c_int }) as val_t,
            4 as libc::c_int,
            if zip64 != 0 { 0xffffffff as libc::c_uint as libc::c_ulong } else { cent },
            4 as libc::c_int,
            if zip64 != 0 {
                0xffffffff as libc::c_uint as libc::c_ulong
            } else {
                head.wrapping_add(clen).wrapping_add(desc)
            },
            2 as libc::c_int,
            0 as libc::c_int as val_t,
            0 as libc::c_int,
        );
    } else if g.form != 0 {
        put(g.outd, -(4 as libc::c_int), check, 0 as libc::c_int);
    } else {
        put(g.outd, 4 as libc::c_int, check, 4 as libc::c_int, ulen, 0 as libc::c_int);
    };
}
unsafe extern "C" fn adler32z(
    mut adler: libc::c_ulong,
    mut buf: *const libc::c_uchar,
    mut len: size_t,
) -> libc::c_ulong {
    while len
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong && !buf.is_null()
    {
        adler = adler32(
            adler,
            buf,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
        );
        buf = buf
            .offset(
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint) as isize,
            );
        len = (len as libc::c_ulong)
            .wrapping_sub(
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint) as libc::c_ulong,
            ) as size_t as size_t;
    }
    return adler32(adler, buf, len as libc::c_uint);
}
unsafe extern "C" fn crc32z(
    mut crc: libc::c_ulong,
    mut buf: *const libc::c_uchar,
    mut len: size_t,
) -> libc::c_ulong {
    while len
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong && !buf.is_null()
    {
        crc = crc32(
            crc,
            buf,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
        );
        buf = buf
            .offset(
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint) as isize,
            );
        len = (len as libc::c_ulong)
            .wrapping_sub(
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint) as libc::c_ulong,
            ) as size_t as size_t;
    }
    return crc32(crc, buf, len as libc::c_uint);
}
unsafe extern "C" fn zlib_vernum() -> libc::c_long {
    let mut ver: *const libc::c_char = zlibVersion();
    let mut num_0: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut left: libc::c_int = 4 as libc::c_int;
    let mut comp: libc::c_int = 0 as libc::c_int;
    loop {
        if *ver as libc::c_int >= '0' as i32 && *ver as libc::c_int <= '9' as i32 {
            comp = 10 as libc::c_int * comp + *ver as libc::c_int - '0' as i32;
        } else {
            num_0 = (num_0 << 4 as libc::c_int)
                + (if comp > 0xf as libc::c_int { 0xf as libc::c_int } else { comp })
                    as libc::c_long;
            left -= 1;
            left;
            if *ver as libc::c_int != '.' as i32 {
                break;
            }
            comp = 0 as libc::c_int;
        }
        ver = ver.offset(1);
        ver;
        if !(left != 0) {
            break;
        }
    }
    return if left < 2 as libc::c_int {
        num_0 << (left << 2 as libc::c_int)
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
unsafe extern "C" fn multmodp(mut a: crc_t, mut b: crc_t) -> crc_t {
    let mut m: crc_t = (1 as libc::c_int as crc_t) << 31 as libc::c_int;
    let mut p: crc_t = 0 as libc::c_int as crc_t;
    loop {
        if a & m != 0 {
            p ^= b;
            if a & m.wrapping_sub(1 as libc::c_int as libc::c_uint)
                == 0 as libc::c_int as libc::c_uint
            {
                break;
            }
        }
        m >>= 1 as libc::c_int;
        b = if b & 1 as libc::c_int as libc::c_uint != 0 {
            b >> 1 as libc::c_int ^ 0xedb88320 as libc::c_uint
        } else {
            b >> 1 as libc::c_int
        };
    }
    return p;
}
static mut x2n_table: [crc_t; 32] = [
    0x40000000 as libc::c_int as crc_t,
    0x20000000 as libc::c_int as crc_t,
    0x8000000 as libc::c_int as crc_t,
    0x800000 as libc::c_int as crc_t,
    0x8000 as libc::c_int as crc_t,
    0xedb88320 as libc::c_uint,
    0xb1e6b092 as libc::c_uint,
    0xa06a2517 as libc::c_uint,
    0xed627dae as libc::c_uint,
    0x88d14467 as libc::c_uint,
    0xd7bbfe6a as libc::c_uint,
    0xec447f11 as libc::c_uint,
    0x8e7ea170 as libc::c_uint,
    0x6427800e as libc::c_int as crc_t,
    0x4d47bae0 as libc::c_int as crc_t,
    0x9fe548f as libc::c_int as crc_t,
    0x83852d0f as libc::c_uint,
    0x30362f1a as libc::c_int as crc_t,
    0x7b5a9cc3 as libc::c_int as crc_t,
    0x31fec169 as libc::c_int as crc_t,
    0x9fec022a as libc::c_uint,
    0x6c8dedc4 as libc::c_int as crc_t,
    0x15d6874d as libc::c_int as crc_t,
    0x5fde7a4e as libc::c_int as crc_t,
    0xbad90e37 as libc::c_uint,
    0x2e4e5eef as libc::c_int as crc_t,
    0x4eaba214 as libc::c_int as crc_t,
    0xa8a472c0 as libc::c_uint,
    0x429a969e as libc::c_int as crc_t,
    0x148d302a as libc::c_int as crc_t,
    0xc40ba6d0 as libc::c_uint,
    0xc4e22c3c as libc::c_uint,
];
unsafe extern "C" fn x2nmodp(mut n: size_t, mut k: libc::c_uint) -> crc_t {
    let mut p: crc_t = (1 as libc::c_int as crc_t) << 31 as libc::c_int;
    while n != 0 {
        if n & 1 as libc::c_int as libc::c_ulong != 0 {
            p = multmodp(x2n_table[(k & 31 as libc::c_int as libc::c_uint) as usize], p);
        }
        n >>= 1 as libc::c_int;
        k = k.wrapping_add(1);
        k;
    }
    return p;
}
unsafe extern "C" fn crc32_comb(
    mut crc1: libc::c_ulong,
    mut crc2: libc::c_ulong,
    mut len2: size_t,
) -> libc::c_ulong {
    return multmodp(
        (if len2 == g.block {
            g.shift
        } else {
            x2nmodp(len2, 3 as libc::c_int as libc::c_uint)
        }),
        crc1 as crc_t,
    ) as libc::c_ulong ^ crc2;
}
unsafe extern "C" fn adler32_comb(
    mut adler1: libc::c_ulong,
    mut adler2: libc::c_ulong,
    mut len2: size_t,
) -> libc::c_ulong {
    let mut sum1: libc::c_ulong = 0;
    let mut sum2: libc::c_ulong = 0;
    let mut rem: libc::c_uint = 0;
    rem = len2.wrapping_rem(65521 as libc::c_uint as libc::c_ulong) as libc::c_uint;
    sum1 = adler1 & 0xffff as libc::c_int as libc::c_ulong;
    sum2 = (rem as libc::c_ulong)
        .wrapping_mul(sum1)
        .wrapping_rem(65521 as libc::c_uint as libc::c_ulong);
    sum1 = sum1
        .wrapping_add(
            (adler2 & 0xffff as libc::c_int as libc::c_ulong)
                .wrapping_add(65521 as libc::c_uint as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    sum2 = sum2
        .wrapping_add(
            (adler1 >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    adler2 >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong,
                )
                .wrapping_add(65521 as libc::c_uint as libc::c_ulong)
                .wrapping_sub(rem as libc::c_ulong),
        );
    if sum1 >= 65521 as libc::c_uint as libc::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as libc::c_uint as libc::c_ulong);
    }
    if sum1 >= 65521 as libc::c_uint as libc::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as libc::c_uint as libc::c_ulong);
    }
    if sum2 >= ((65521 as libc::c_uint) << 1 as libc::c_int) as libc::c_ulong {
        sum2 = sum2
            .wrapping_sub(
                ((65521 as libc::c_uint) << 1 as libc::c_int) as libc::c_ulong,
            );
    }
    if sum2 >= 65521 as libc::c_uint as libc::c_ulong {
        sum2 = sum2.wrapping_sub(65521 as libc::c_uint as libc::c_ulong);
    }
    return sum1 | sum2 << 16 as libc::c_int;
}
unsafe extern "C" fn new_pool(
    mut pool: *mut pool,
    mut size: size_t,
    mut limit: libc::c_int,
) {
    (*pool)
        .have = new_lock_(
        0 as libc::c_int as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1457 as libc::c_int as libc::c_long,
    );
    (*pool).head = 0 as *mut space;
    (*pool).size = size;
    (*pool).limit = limit;
    (*pool).made = 0 as libc::c_int;
}
unsafe extern "C" fn get_space(mut pool: *mut pool) -> *mut space {
    let mut space: *mut space = 0 as *mut space;
    possess_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1470 as libc::c_int as libc::c_long,
    );
    if (*pool).limit == 0 as libc::c_int {
        wait_for_(
            (*pool).have,
            NOT_TO_BE,
            0 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1472 as libc::c_int as libc::c_long,
        );
    }
    if !((*pool).head).is_null() {
        space = (*pool).head;
        (*pool).head = (*space).next;
        twist_(
            (*pool).have,
            BY,
            -(1 as libc::c_int) as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1478 as libc::c_int as libc::c_long,
        );
        possess_(
            (*space).use_0,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1479 as libc::c_int as libc::c_long,
        );
        twist_(
            (*space).use_0,
            TO,
            1 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1480 as libc::c_int as libc::c_long,
        );
        (*space).len = 0 as libc::c_int as size_t;
        return space;
    }
    if (*pool).limit != 0 as libc::c_int {} else {
        __assert_fail(
            b"pool->limit != 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1486 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"struct space *get_space(struct pool *)\0"))
                .as_ptr(),
        );
    }
    'c_7272: {
        if (*pool).limit != 0 as libc::c_int {} else {
            __assert_fail(
                b"pool->limit != 0\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"struct space *get_space(struct pool *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*pool).limit > 0 as libc::c_int {
        (*pool).limit -= 1;
        (*pool).limit;
    }
    (*pool).made += 1;
    (*pool).made;
    release_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1490 as libc::c_int as libc::c_long,
    );
    space = alloc(
        0 as *mut libc::c_void,
        ::std::mem::size_of::<space>() as libc::c_ulong,
    ) as *mut space;
    (*space)
        .use_0 = new_lock_(
        1 as libc::c_int as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1492 as libc::c_int as libc::c_long,
    );
    (*space).buf = alloc(0 as *mut libc::c_void, (*pool).size) as *mut libc::c_uchar;
    (*space).size = (*pool).size;
    (*space).len = 0 as libc::c_int as size_t;
    (*space).pool = pool;
    return space;
}
unsafe extern "C" fn grow_space(mut space: *mut space) {
    let mut more: size_t = 0;
    more = grow((*space).size);
    if more == (*space).size {
        try_throw_(
            34 as libc::c_int,
            b"overflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    (*space).buf = alloc((*space).buf as *mut libc::c_void, more) as *mut libc::c_uchar;
    (*space).size = more;
}
unsafe extern "C" fn use_space(mut space: *mut space) {
    let mut use_0: libc::c_long = 0;
    possess_(
        (*space).use_0,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1519 as libc::c_int as libc::c_long,
    );
    use_0 = peek_lock((*space).use_0);
    if use_0 != 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"use != 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1521 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void use_space(struct space *)\0"))
                .as_ptr(),
        );
    }
    'c_7514: {
        if use_0 != 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"use != 0\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1521 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void use_space(struct space *)\0"))
                    .as_ptr(),
            );
        }
    };
    twist_(
        (*space).use_0,
        BY,
        1 as libc::c_int as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1522 as libc::c_int as libc::c_long,
    );
}
unsafe extern "C" fn drop_space(mut space: *mut space) {
    let mut use_0: libc::c_long = 0;
    let mut pool: *mut pool = 0 as *mut pool;
    if space.is_null() {
        return;
    }
    possess_(
        (*space).use_0,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1532 as libc::c_int as libc::c_long,
    );
    use_0 = peek_lock((*space).use_0);
    if use_0 != 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"use != 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1534 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void drop_space(struct space *)\0"))
                .as_ptr(),
        );
    }
    'c_7662: {
        if use_0 != 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"use != 0\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1534 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void drop_space(struct space *)\0"))
                    .as_ptr(),
            );
        }
    };
    twist_(
        (*space).use_0,
        BY,
        -(1 as libc::c_int) as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1535 as libc::c_int as libc::c_long,
    );
    if use_0 == 1 as libc::c_int as libc::c_long {
        pool = (*space).pool;
        possess_(
            (*pool).have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1538 as libc::c_int as libc::c_long,
        );
        (*space).next = (*pool).head;
        (*pool).head = space;
        twist_(
            (*pool).have,
            BY,
            1 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1541 as libc::c_int as libc::c_long,
        );
    }
}
unsafe extern "C" fn free_pool(mut pool: *mut pool) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut space: *mut space = 0 as *mut space;
    possess_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1551 as libc::c_int as libc::c_long,
    );
    count = 0 as libc::c_int;
    loop {
        space = (*pool).head;
        if space.is_null() {
            break;
        }
        (*pool).head = (*space).next;
        free((*space).buf as *mut libc::c_void);
        free_lock_(
            (*space).use_0,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1556 as libc::c_int as libc::c_long,
        );
        free(space as *mut libc::c_void);
        count += 1;
        count;
    }
    if count == (*pool).made {} else {
        __assert_fail(
            b"count == pool->made\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1560 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"int free_pool(struct pool *)\0"))
                .as_ptr(),
        );
    }
    'c_7767: {
        if count == (*pool).made {} else {
            __assert_fail(
                b"count == pool->made\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1560 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"int free_pool(struct pool *)\0"))
                    .as_ptr(),
            );
        }
    };
    release_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1561 as libc::c_int as libc::c_long,
    );
    free_lock_(
        (*pool).have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1562 as libc::c_int as libc::c_long,
    );
    return count;
}
static mut in_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut out_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut dict_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut lens_pool: pool = pool {
    have: 0 as *const lock as *mut lock,
    head: 0 as *const space as *mut space,
    size: 0,
    limit: 0,
    made: 0,
};
static mut compress_have: *mut lock = 0 as *const lock as *mut lock;
static mut compress_head: *mut job = 0 as *const job as *mut job;
static mut compress_tail: *mut *mut job = 0 as *const *mut job as *mut *mut job;
static mut write_first: *mut lock = 0 as *const lock as *mut lock;
static mut write_head: *mut job = 0 as *const job as *mut job;
static mut cthreads: libc::c_int = 0 as libc::c_int;
static mut writeth: *mut thread = 0 as *const thread as *mut thread;
unsafe extern "C" fn setup_jobs() {
    if !compress_have.is_null() {
        return;
    }
    compress_have = new_lock_(
        0 as libc::c_int as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1609 as libc::c_int as libc::c_long,
    );
    compress_head = 0 as *mut job;
    compress_tail = &mut compress_head;
    write_first = new_lock_(
        -(1 as libc::c_int) as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1612 as libc::c_int as libc::c_long,
    );
    write_head = 0 as *mut job;
    new_pool(&mut in_pool, g.block, (g.procs << 1 as libc::c_int) + 3 as libc::c_int);
    new_pool(
        &mut out_pool,
        (g.block)
            .wrapping_add(g.block >> 4 as libc::c_int)
            .wrapping_add(32768 as libc::c_uint as libc::c_ulong),
        -(1 as libc::c_int),
    );
    new_pool(&mut dict_pool, 32768 as libc::c_uint as size_t, -(1 as libc::c_int));
    new_pool(
        &mut lens_pool,
        g.block >> 12 as libc::c_int - 1 as libc::c_int,
        -(1 as libc::c_int),
    );
}
unsafe extern "C" fn finish_jobs() {
    let mut job: job = job {
        seq: 0,
        more: 0,
        in_0: 0 as *mut space,
        out: 0 as *mut space,
        lens: 0 as *mut space,
        check: 0,
        calc: 0 as *mut lock,
        next: 0 as *mut job,
    };
    let mut caught: libc::c_int = 0;
    if compress_have.is_null() {
        return;
    }
    possess_(
        compress_have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1635 as libc::c_int as libc::c_long,
    );
    job.seq = -(1 as libc::c_int) as libc::c_long;
    job.next = 0 as *mut job;
    compress_head = &mut job;
    compress_tail = &mut job.next;
    twist_(
        compress_have,
        BY,
        1 as libc::c_int as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1640 as libc::c_int as libc::c_long,
    );
    caught = join_all_(
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1643 as libc::c_int as libc::c_long,
    );
    if caught == cthreads {} else {
        __assert_fail(
            b"caught == cthreads\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1645 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void finish_jobs(void)\0"))
                .as_ptr(),
        );
    }
    'c_8124: {
        if caught == cthreads {} else {
            __assert_fail(
                b"caught == cthreads\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1645 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void finish_jobs(void)\0"))
                    .as_ptr(),
            );
        }
    };
    cthreads = 0 as libc::c_int;
    caught = free_pool(&mut lens_pool);
    caught = free_pool(&mut dict_pool);
    caught = free_pool(&mut out_pool);
    caught = free_pool(&mut in_pool);
    free_lock_(
        write_first,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1657 as libc::c_int as libc::c_long,
    );
    free_lock_(
        compress_have,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        1658 as libc::c_int as libc::c_long,
    );
    compress_have = 0 as *mut lock;
}
unsafe extern "C" fn deflate_engine(
    mut strm: *mut z_stream,
    mut out: *mut space,
    mut flush: libc::c_int,
) {
    let mut room: size_t = 0;
    loop {
        room = ((*out).size).wrapping_sub((*out).len);
        if room == 0 as libc::c_int as libc::c_ulong {
            grow_space(out);
            room = ((*out).size).wrapping_sub((*out).len);
        }
        (*strm).next_out = ((*out).buf).offset((*out).len as isize);
        (*strm)
            .avail_out = if room
            < (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong
        {
            room as libc::c_uint
        } else {
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
        };
        deflate(strm, flush);
        (*out)
            .len = ((*strm).next_out).offset_from((*out).buf) as libc::c_long as size_t;
        if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1680 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void deflate_engine(z_stream *, struct space *, int)\0"))
                .as_ptr(),
        );
    }
    'c_8238: {
        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1680 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void deflate_engine(z_stream *, struct space *, int)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn compress_thread(mut dummy: *mut libc::c_void) {
    let mut job: *mut job = 0 as *mut job;
    let mut here: *mut job = 0 as *mut job;
    let mut prior: *mut *mut job = 0 as *mut *mut job;
    let mut check: libc::c_ulong = 0;
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut left: size_t = 0;
    let mut len: size_t = 0;
    let mut bits: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_char;
    try_setup_();
    try_this_.next = pthread_getspecific(try_key_) as *mut try_t_;
    if pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    ) == 0 as libc::c_int
        && !(b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1703 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void compress_thread(void *)\0"))
                .as_ptr(),
        );
    }
    'c_10359: {
        if pthread_setspecific(
            try_key_,
            &mut try_this_ as *mut try_t_ as *const libc::c_void,
        ) == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1703 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void compress_thread(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if _setjmp((try_this_.env).as_mut_ptr()) == 0 as libc::c_int {
        let mut strm: z_stream = z_stream {
            next_in: 0 as *mut Bytef,
            avail_in: 0,
            total_in: 0,
            next_out: 0 as *mut Bytef,
            avail_out: 0,
            total_out: 0,
            msg: 0 as *mut libc::c_char,
            state: 0 as *mut internal_state,
            zalloc: None,
            zfree: None,
            opaque: 0 as *mut libc::c_void,
            data_type: 0,
            adler: 0,
            reserved: 0,
        };
        let mut temp: *mut space = 0 as *mut space;
        if g.level > 9 as libc::c_int {
            temp = get_space(&mut out_pool);
        } else {
            strm.zfree = None;
            strm.zalloc = None;
            strm.opaque = 0 as voidpf;
            ret = deflateInit2_(
                &mut strm,
                6 as libc::c_int,
                8 as libc::c_int,
                -(15 as libc::c_int),
                8 as libc::c_int,
                g.strategy,
                b"1.2.11\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            );
            if ret == -(4 as libc::c_int) {
                try_throw_(
                    12 as libc::c_int,
                    b"not enough memory\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
            if ret != 0 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"internal error\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
        }
        loop {
            possess_(
                compress_have,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1727 as libc::c_int as libc::c_long,
            );
            wait_for_(
                compress_have,
                NOT_TO_BE,
                0 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1728 as libc::c_int as libc::c_long,
            );
            job = compress_head;
            if !job.is_null() {} else {
                __assert_fail(
                    b"job != NULL\0" as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    1730 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void compress_thread(void *)\0"))
                        .as_ptr(),
                );
            }
            'c_10176: {
                if !job.is_null() {} else {
                    __assert_fail(
                        b"job != NULL\0" as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        1730 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 29],
                            &[libc::c_char; 29],
                        >(b"void compress_thread(void *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if (*job).seq == -(1 as libc::c_int) as libc::c_long {
                break;
            }
            compress_head = (*job).next;
            if ((*job).next).is_null() {
                compress_tail = &mut compress_head;
            }
            twist_(
                compress_have,
                BY,
                -(1 as libc::c_int) as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1736 as libc::c_int as libc::c_long,
            );
            if g.level <= 9 as libc::c_int {
                deflateReset(&mut strm);
                deflateParams(&mut strm, g.level, g.strategy);
            } else {
                (*temp).len = 0 as libc::c_int as size_t;
            }
            if !((*job).out).is_null() {
                len = (*(*job).out).len;
                left = if len < 32768 as libc::c_uint as libc::c_ulong {
                    len
                } else {
                    32768 as libc::c_uint as libc::c_ulong
                };
                if g.level <= 9 as libc::c_int {
                    deflateSetDictionary(
                        &mut strm,
                        ((*(*job).out).buf).offset(len.wrapping_sub(left) as isize),
                        left as libc::c_uint,
                    );
                } else {
                    memcpy(
                        (*temp).buf as *mut libc::c_void,
                        ((*(*job).out).buf).offset(len.wrapping_sub(left) as isize)
                            as *const libc::c_void,
                        left,
                    );
                    (*temp).len = left;
                }
                drop_space((*job).out);
            }
            (*job).out = get_space(&mut out_pool);
            if g.level <= 9 as libc::c_int {
                strm.next_in = (*(*job).in_0).buf;
                strm.next_out = (*(*job).out).buf;
            } else {
                memcpy(
                    ((*temp).buf).offset((*temp).len as isize) as *mut libc::c_void,
                    (*(*job).in_0).buf as *const libc::c_void,
                    (*(*job).in_0).len,
                );
            }
            next = if ((*job).lens).is_null() {
                0 as *mut libc::c_uchar
            } else {
                (*(*job).lens).buf
            };
            left = (*(*job).in_0).len;
            (*(*job).out).len = 0 as libc::c_int as size_t;
            loop {
                len = (if next.is_null() {
                    128 as libc::c_int
                } else {
                    let fresh2 = next;
                    next = next.offset(1);
                    *fresh2 as libc::c_int
                }) as size_t;
                if len < 128 as libc::c_int as libc::c_ulong {
                    let fresh3 = next;
                    next = next.offset(1);
                    len = (len << 8 as libc::c_int)
                        .wrapping_add(*fresh3 as libc::c_ulong)
                        .wrapping_add(64 as libc::c_int as libc::c_ulong);
                } else if len == 128 as libc::c_int as libc::c_ulong {
                    len = left;
                } else if len < 192 as libc::c_int as libc::c_ulong {
                    len &= 0x3f as libc::c_int as libc::c_ulong;
                } else if len < 224 as libc::c_int as libc::c_ulong {
                    let fresh4 = next;
                    next = next.offset(1);
                    len = ((len & 0x1f as libc::c_int as libc::c_ulong)
                        << 16 as libc::c_int)
                        .wrapping_add((*fresh4 as size_t) << 8 as libc::c_int);
                    let fresh5 = next;
                    next = next.offset(1);
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            (*fresh5 as libc::c_uint).wrapping_add(32832 as libc::c_uint)
                                as libc::c_ulong,
                        ) as size_t as size_t;
                } else {
                    let fresh6 = next;
                    next = next.offset(1);
                    len = ((len & 0x1f as libc::c_int as libc::c_ulong)
                        << 24 as libc::c_int)
                        .wrapping_add((*fresh6 as size_t) << 16 as libc::c_int);
                    let fresh7 = next;
                    next = next.offset(1);
                    len = (len as libc::c_ulong)
                        .wrapping_add((*fresh7 as size_t) << 8 as libc::c_int) as size_t
                        as size_t;
                    let fresh8 = next;
                    next = next.offset(1);
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            (*fresh8 as size_t).wrapping_add(2129984 as libc::c_ulong),
                        ) as size_t as size_t;
                }
                left = (left as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
                if g.level <= 9 as libc::c_int {
                    while len
                        > (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as libc::c_ulong
                    {
                        strm
                            .avail_in = (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            );
                        deflate_engine(&mut strm, (*job).out, 0 as libc::c_int);
                        len = (len as libc::c_ulong)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint)
                                    .wrapping_sub(
                                        (2147483647 as libc::c_int as libc::c_uint)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                                    ) as libc::c_ulong,
                            ) as size_t as size_t;
                    }
                    strm.avail_in = len as libc::c_uint;
                    if left != 0 || (*job).more != 0 {
                        if zlib_vernum() >= 0x1260 as libc::c_int as libc::c_long {
                            deflate_engine(&mut strm, (*job).out, 5 as libc::c_int);
                            deflatePending(&mut strm, 0 as *mut libc::c_uint, &mut bits);
                            if bits & 1 as libc::c_int != 0 || g.setdict == 0 {
                                deflate_engine(&mut strm, (*job).out, 2 as libc::c_int);
                            } else if bits & 7 as libc::c_int != 0 {
                                loop {
                                    bits = deflatePrime(
                                        &mut strm,
                                        10 as libc::c_int,
                                        2 as libc::c_int,
                                    );
                                    if bits == 0 as libc::c_int {} else {
                                        __assert_fail(
                                            b"bits == Z_OK\0" as *const u8 as *const libc::c_char,
                                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                                            1840 as libc::c_int as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 29],
                                                &[libc::c_char; 29],
                                            >(b"void compress_thread(void *)\0"))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_9556: {
                                        if bits == 0 as libc::c_int {} else {
                                            __assert_fail(
                                                b"bits == Z_OK\0" as *const u8 as *const libc::c_char,
                                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                                1840 as libc::c_int as libc::c_uint,
                                                (*::std::mem::transmute::<
                                                    &[u8; 29],
                                                    &[libc::c_char; 29],
                                                >(b"void compress_thread(void *)\0"))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    deflatePending(
                                        &mut strm,
                                        0 as *mut libc::c_uint,
                                        &mut bits,
                                    );
                                    if !(bits & 7 as libc::c_int != 0) {
                                        break;
                                    }
                                }
                                deflate_engine(&mut strm, (*job).out, 5 as libc::c_int);
                            }
                        } else {
                            deflate_engine(&mut strm, (*job).out, 2 as libc::c_int);
                        }
                        if g.setdict == 0 {
                            deflate_engine(&mut strm, (*job).out, 3 as libc::c_int);
                        }
                    } else {
                        deflate_engine(&mut strm, (*job).out, 4 as libc::c_int);
                    }
                } else {
                    let mut bits_0: libc::c_uchar = 0;
                    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut outsize: size_t = 0;
                    out = 0 as *mut libc::c_uchar;
                    outsize = 0 as libc::c_int as size_t;
                    bits_0 = 0 as libc::c_int as libc::c_uchar;
                    ZopfliDeflatePart(
                        &mut g.zopts,
                        2 as libc::c_int,
                        !(left != 0 || (*job).more != 0) as libc::c_int,
                        (*temp).buf,
                        (*temp).len,
                        ((*temp).len).wrapping_add(len),
                        &mut bits_0,
                        &mut out,
                        &mut outsize,
                    );
                    if ((*(*job).out).len)
                        .wrapping_add(outsize)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong)
                        <= (*(*job).out).size
                    {} else {
                        __assert_fail(
                            b"job->out->len + outsize + 5 <= job->out->size\0"
                                as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            1869 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 29],
                                &[libc::c_char; 29],
                            >(b"void compress_thread(void *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_9342: {
                        if ((*(*job).out).len)
                            .wrapping_add(outsize)
                            .wrapping_add(5 as libc::c_int as libc::c_ulong)
                            <= (*(*job).out).size
                        {} else {
                            __assert_fail(
                                b"job->out->len + outsize + 5 <= job->out->size\0"
                                    as *const u8 as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                1869 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 29],
                                    &[libc::c_char; 29],
                                >(b"void compress_thread(void *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    memcpy(
                        ((*(*job).out).buf).offset((*(*job).out).len as isize)
                            as *mut libc::c_void,
                        out as *const libc::c_void,
                        outsize,
                    );
                    free(out as *mut libc::c_void);
                    (*(*job).out)
                        .len = ((*(*job).out).len as libc::c_ulong).wrapping_add(outsize)
                        as size_t as size_t;
                    if left != 0 || (*job).more != 0 {
                        bits_0 = (bits_0 as libc::c_int & 7 as libc::c_int)
                            as libc::c_uchar;
                        if bits_0 as libc::c_int & 1 as libc::c_int != 0
                            || g.setdict == 0
                        {
                            if bits_0 as libc::c_int == 0 as libc::c_int
                                || bits_0 as libc::c_int > 5 as libc::c_int
                            {
                                let fresh9 = (*(*job).out).len;
                                (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                *((*(*job).out).buf)
                                    .offset(
                                        fresh9 as isize,
                                    ) = 0 as libc::c_int as libc::c_uchar;
                            }
                            let fresh10 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh10 as isize,
                                ) = 0 as libc::c_int as libc::c_uchar;
                            let fresh11 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh11 as isize,
                                ) = 0 as libc::c_int as libc::c_uchar;
                            let fresh12 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh12 as isize,
                                ) = 0xff as libc::c_int as libc::c_uchar;
                            let fresh13 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh13 as isize,
                                ) = 0xff as libc::c_int as libc::c_uchar;
                        } else if bits_0 != 0 {
                            loop {
                                let ref mut fresh14 = *((*(*job).out).buf)
                                    .offset(
                                        ((*(*job).out).len)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    );
                                *fresh14 = (*fresh14 as libc::c_int
                                    + ((2 as libc::c_int) << bits_0 as libc::c_int))
                                    as libc::c_uchar;
                                let fresh15 = (*(*job).out).len;
                                (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                                *((*(*job).out).buf)
                                    .offset(
                                        fresh15 as isize,
                                    ) = 0 as libc::c_int as libc::c_uchar;
                                bits_0 = (bits_0 as libc::c_int + 2 as libc::c_int)
                                    as libc::c_uchar;
                                if !((bits_0 as libc::c_int) < 8 as libc::c_int) {
                                    break;
                                }
                            }
                        }
                        if g.setdict == 0 {
                            let fresh16 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh16 as isize,
                                ) = 0 as libc::c_int as libc::c_uchar;
                            let fresh17 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh17 as isize,
                                ) = 0 as libc::c_int as libc::c_uchar;
                            let fresh18 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh18 as isize,
                                ) = 0 as libc::c_int as libc::c_uchar;
                            let fresh19 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh19 as isize,
                                ) = 0xff as libc::c_int as libc::c_uchar;
                            let fresh20 = (*(*job).out).len;
                            (*(*job).out).len = ((*(*job).out).len).wrapping_add(1);
                            *((*(*job).out).buf)
                                .offset(
                                    fresh20 as isize,
                                ) = 0xff as libc::c_int as libc::c_uchar;
                        }
                    }
                    (*temp)
                        .len = ((*temp).len as libc::c_ulong).wrapping_add(len) as size_t
                        as size_t;
                }
                if !(left != 0) {
                    break;
                }
            }
            drop_space((*job).lens);
            (*job).lens = 0 as *mut space;
            use_space((*job).in_0);
            possess_(
                write_first,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1911 as libc::c_int as libc::c_long,
            );
            prior = &mut write_head;
            loop {
                here = *prior;
                if here.is_null() {
                    break;
                }
                if (*here).seq > (*job).seq {
                    break;
                }
                prior = &mut (*here).next;
            }
            (*job).next = here;
            *prior = job;
            twist_(
                write_first,
                TO,
                (*write_head).seq,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1920 as libc::c_int as libc::c_long,
            );
            len = (*(*job).in_0).len;
            next = (*(*job).in_0).buf;
            check = if g.form == 1 as libc::c_int {
                adler32z(
                    0 as libc::c_long as libc::c_ulong,
                    0 as *const libc::c_uchar,
                    0 as libc::c_int as size_t,
                )
            } else {
                crc32z(
                    0 as libc::c_long as libc::c_ulong,
                    0 as *const libc::c_uchar,
                    0 as libc::c_int as size_t,
                )
            };
            while len
                > (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_sub(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                    ) as libc::c_ulong
            {
                check = if g.form == 1 as libc::c_int {
                    adler32z(
                        check,
                        next,
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as size_t,
                    )
                } else {
                    crc32z(
                        check,
                        next,
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as size_t,
                    )
                };
                len = (len as libc::c_ulong)
                    .wrapping_sub(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as libc::c_ulong,
                    ) as size_t as size_t;
                next = next
                    .offset(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as isize,
                    );
            }
            check = if g.form == 1 as libc::c_int {
                adler32z(check, next, len as libc::c_uint as size_t)
            } else {
                crc32z(check, next, len as libc::c_uint as size_t)
            };
            drop_space((*job).in_0);
            (*job).check = check;
            possess_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1937 as libc::c_int as libc::c_long,
            );
            twist_(
                (*job).calc,
                TO,
                1 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1938 as libc::c_int as libc::c_long,
            );
        }
        release_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1944 as libc::c_int as libc::c_long,
        );
        if g.level > 9 as libc::c_int {
            drop_space(temp);
        } else {
            deflateEnd(&mut strm);
        }
    }
    if try_pushed_ != 0 {
        if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
            == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1954 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void compress_thread(void *)\0"))
                    .as_ptr(),
            );
        }
        'c_8509: {
            if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    1954 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void compress_thread(void *)\0"))
                        .as_ptr(),
                );
            }
        };
        ::std::ptr::write_volatile(
            &mut try_pushed_ as *mut libc::c_int,
            0 as libc::c_int,
        );
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn write_thread(mut dummy: *mut libc::c_void) {
    let mut seq: libc::c_long = 0;
    let mut job: *mut job = 0 as *mut job;
    let mut len: size_t = 0;
    let mut more: libc::c_int = 0;
    let mut head: length_t = 0;
    let mut ulen: length_t = 0;
    let mut clen: length_t = 0;
    let mut check: libc::c_ulong = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_char;
    try_setup_();
    try_this_.next = pthread_getspecific(try_key_) as *mut try_t_;
    if pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    ) == 0 as libc::c_int
        && !(b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            1975 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void write_thread(void *)\0"))
                .as_ptr(),
        );
    }
    'c_11091: {
        if pthread_setspecific(
            try_key_,
            &mut try_this_ as *mut try_t_ as *const libc::c_void,
        ) == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1975 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void write_thread(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if _setjmp((try_this_.env).as_mut_ptr()) == 0 as libc::c_int {
        head = put_header();
        clen = 0 as libc::c_int as length_t;
        ulen = clen;
        check = if g.form == 1 as libc::c_int {
            adler32z(
                0 as libc::c_long as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            )
        } else {
            crc32z(
                0 as libc::c_long as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            )
        };
        seq = 0 as libc::c_int as libc::c_long;
        loop {
            possess_(
                write_first,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1986 as libc::c_int as libc::c_long,
            );
            wait_for_(
                write_first,
                TO_BE,
                seq,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1987 as libc::c_int as libc::c_long,
            );
            job = write_head;
            write_head = (*job).next;
            twist_(
                write_first,
                TO,
                if write_head.is_null() {
                    -(1 as libc::c_int) as libc::c_long
                } else {
                    (*write_head).seq
                },
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                1990 as libc::c_int as libc::c_long,
            );
            more = (*job).more;
            len = (*(*job).in_0).len;
            drop_space((*job).in_0);
            ulen = (ulen as libc::c_ulong).wrapping_add(len) as length_t as length_t;
            clen = (clen as libc::c_ulong).wrapping_add((*(*job).out).len) as length_t
                as length_t;
            writen(g.outd, (*(*job).out).buf as *const libc::c_void, (*(*job).out).len);
            drop_space((*job).out);
            possess_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2007 as libc::c_int as libc::c_long,
            );
            wait_for_(
                (*job).calc,
                TO_BE,
                1 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2008 as libc::c_int as libc::c_long,
            );
            release_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2009 as libc::c_int as libc::c_long,
            );
            check = if g.form == 1 as libc::c_int {
                adler32_comb(check, (*job).check, len)
            } else {
                crc32_comb(check, (*job).check, len)
            };
            free_lock_(
                (*job).calc,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2014 as libc::c_int as libc::c_long,
            );
            free(job as *mut libc::c_void);
            seq += 1;
            seq;
            if !(more != 0) {
                break;
            }
        }
        put_trailer(ulen, clen, check, head);
        possess_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2025 as libc::c_int as libc::c_long,
        );
        if compress_head.is_null()
            && peek_lock(compress_have) == 0 as libc::c_int as libc::c_long
        {} else {
            __assert_fail(
                b"compress_head == NULL && peek_lock(compress_have) == 0\0" as *const u8
                    as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2026 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void write_thread(void *)\0"))
                    .as_ptr(),
            );
        }
        'c_10717: {
            if compress_head.is_null()
                && peek_lock(compress_have) == 0 as libc::c_int as libc::c_long
            {} else {
                __assert_fail(
                    b"compress_head == NULL && peek_lock(compress_have) == 0\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2026 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void write_thread(void *)\0"))
                        .as_ptr(),
                );
            }
        };
        release_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2027 as libc::c_int as libc::c_long,
        );
        possess_(
            write_first,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2028 as libc::c_int as libc::c_long,
        );
        if write_head.is_null() {} else {
            __assert_fail(
                b"write_head == NULL\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2029 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void write_thread(void *)\0"))
                    .as_ptr(),
            );
        }
        'c_10657: {
            if write_head.is_null() {} else {
                __assert_fail(
                    b"write_head == NULL\0" as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2029 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void write_thread(void *)\0"))
                        .as_ptr(),
                );
            }
        };
        twist_(
            write_first,
            TO,
            -(1 as libc::c_int) as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2030 as libc::c_int as libc::c_long,
        );
    }
    if try_pushed_ != 0 {
        if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
            == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2032 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void write_thread(void *)\0"))
                    .as_ptr(),
            );
        }
        'c_10567: {
            if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2032 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void write_thread(void *)\0"))
                        .as_ptr(),
                );
            }
        };
        ::std::ptr::write_volatile(
            &mut try_pushed_ as *mut libc::c_int,
            0 as libc::c_int,
        );
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn append_len(mut job: *mut job, mut len: size_t) {
    let mut lens: *mut space = 0 as *mut space;
    if len < 539000896 as libc::c_ulong {} else {
        __assert_fail(
            b"len < 539000896UL\0" as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2041 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void append_len(struct job *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_11442: {
        if len < 539000896 as libc::c_ulong {} else {
            __assert_fail(
                b"len < 539000896UL\0" as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2041 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void append_len(struct job *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*job).lens).is_null() {
        (*job).lens = get_space(&mut lens_pool);
    }
    lens = (*job).lens;
    if (*lens).size < ((*lens).len).wrapping_add(3 as libc::c_int as libc::c_ulong) {
        grow_space(lens);
    }
    if len < 64 as libc::c_int as libc::c_ulong {
        let fresh21 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(
                fresh21 as isize,
            ) = len.wrapping_add(128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
    } else if len < 32832 as libc::c_uint as libc::c_ulong {
        len = (len as libc::c_ulong).wrapping_sub(64 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        let fresh22 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(fresh22 as isize) = (len >> 8 as libc::c_int) as libc::c_uchar;
        let fresh23 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf).offset(fresh23 as isize) = len as libc::c_uchar;
    } else if len < 2129984 as libc::c_ulong {
        len = (len as libc::c_ulong).wrapping_sub(32832 as libc::c_uint as libc::c_ulong)
            as size_t as size_t;
        let fresh24 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(
                fresh24 as isize,
            ) = (len >> 16 as libc::c_int)
            .wrapping_add(192 as libc::c_int as libc::c_ulong) as libc::c_uchar;
        let fresh25 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(fresh25 as isize) = (len >> 8 as libc::c_int) as libc::c_uchar;
        let fresh26 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf).offset(fresh26 as isize) = len as libc::c_uchar;
    } else {
        len = (len as libc::c_ulong).wrapping_sub(2129984 as libc::c_ulong) as size_t
            as size_t;
        let fresh27 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(
                fresh27 as isize,
            ) = (len >> 24 as libc::c_int)
            .wrapping_add(224 as libc::c_int as libc::c_ulong) as libc::c_uchar;
        let fresh28 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(fresh28 as isize) = (len >> 16 as libc::c_int) as libc::c_uchar;
        let fresh29 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf)
            .offset(fresh29 as isize) = (len >> 8 as libc::c_int) as libc::c_uchar;
        let fresh30 = (*lens).len;
        (*lens).len = ((*lens).len).wrapping_add(1);
        *((*lens).buf).offset(fresh30 as isize) = len as libc::c_uchar;
    };
}
unsafe extern "C" fn parallel_compress() {
    let mut seq: libc::c_long = 0;
    let mut curr: *mut space = 0 as *mut space;
    let mut next: *mut space = 0 as *mut space;
    let mut hold: *mut space = 0 as *mut space;
    let mut dict: *mut space = 0 as *mut space;
    let mut job: *mut job = 0 as *mut job;
    let mut more: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut scan: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut last: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut left: size_t = 0;
    let mut len: size_t = 0;
    setup_jobs();
    writeth = launch_(
        Some(write_thread as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2092 as libc::c_int as libc::c_long,
    );
    seq = 0 as libc::c_int as libc::c_long;
    next = get_space(&mut in_pool);
    (*next).len = readn(g.ind, (*next).buf, (*next).size);
    hold = 0 as *mut space;
    dict = 0 as *mut space;
    scan = (*next).buf;
    hash = ((1 as libc::c_uint) << 12 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int;
    left = 0 as libc::c_int as size_t;
    loop {
        job = alloc(
            0 as *mut libc::c_void,
            ::std::mem::size_of::<job>() as libc::c_ulong,
        ) as *mut job;
        (*job)
            .calc = new_lock_(
            0 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2107 as libc::c_int as libc::c_long,
        );
        curr = next;
        next = hold;
        hold = 0 as *mut space;
        if next.is_null() {
            next = get_space(&mut in_pool);
            (*next).len = readn(g.ind, (*next).buf, (*next).size);
        }
        (*job).lens = 0 as *mut space;
        if g.rsync != 0 && (*curr).len != 0 {
            if left == 0 as libc::c_int as libc::c_ulong {
                last = (*curr).buf;
                end = ((*curr).buf).offset((*curr).len as isize);
                while scan < end {
                    let fresh31 = scan;
                    scan = scan.offset(1);
                    hash = (hash << 1 as libc::c_int ^ *fresh31 as libc::c_uint)
                        & ((1 as libc::c_uint) << 12 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint);
                    if hash
                        == ((1 as libc::c_uint) << 12 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            >> 1 as libc::c_int
                    {
                        len = scan.offset_from(last) as libc::c_long as size_t;
                        append_len(job, len);
                        last = scan;
                    }
                }
                left = scan.offset_from(last) as libc::c_long as size_t;
                scan = (*next).buf;
            }
            last = (*next).buf;
            len = ((*curr).size).wrapping_sub((*curr).len);
            if len > (*next).len {
                len = (*next).len;
            }
            end = ((*next).buf).offset(len as isize);
            while scan < end {
                let fresh32 = scan;
                scan = scan.offset(1);
                hash = (hash << 1 as libc::c_int ^ *fresh32 as libc::c_uint)
                    & ((1 as libc::c_uint) << 12 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                if hash
                    == ((1 as libc::c_uint) << 12 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        >> 1 as libc::c_int
                {
                    len = (scan.offset_from(last) as libc::c_long as size_t)
                        .wrapping_add(left);
                    left = 0 as libc::c_int as size_t;
                    append_len(job, len);
                    last = scan;
                }
            }
            append_len(job, 0 as libc::c_int as size_t);
            len = (if (*(*job).lens).len == 1 as libc::c_int as libc::c_ulong {
                scan
            } else {
                last
            })
                .offset_from((*next).buf) as libc::c_long as size_t;
            if len != 0 {
                memcpy(
                    ((*curr).buf).offset((*curr).len as isize) as *mut libc::c_void,
                    (*next).buf as *const libc::c_void,
                    len,
                );
                (*curr)
                    .len = ((*curr).len as libc::c_ulong).wrapping_add(len) as size_t
                    as size_t;
                memmove(
                    (*next).buf as *mut libc::c_void,
                    ((*next).buf).offset(len as isize) as *const libc::c_void,
                    ((*next).len).wrapping_sub(len),
                );
                (*next)
                    .len = ((*next).len as libc::c_ulong).wrapping_sub(len) as size_t
                    as size_t;
                scan = scan.offset(-(len as isize));
                left = 0 as libc::c_int as size_t;
            } else if (*(*job).lens).len != 1 as libc::c_int as libc::c_ulong
                && left != 0 && (*next).len != 0
            {
                hold = next;
                next = get_space(&mut in_pool);
                memcpy(
                    (*next).buf as *mut libc::c_void,
                    ((*curr).buf).offset(((*curr).len).wrapping_sub(left) as isize)
                        as *const libc::c_void,
                    left,
                );
                (*next).len = left;
                (*curr)
                    .len = ((*curr).len as libc::c_ulong).wrapping_sub(left) as size_t
                    as size_t;
            } else {
                left = 0 as libc::c_int as size_t;
            }
        }
        (*job).in_0 = curr;
        more = ((*next).len != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
        (*job).more = more;
        (*job).out = dict;
        if more != 0 && g.setdict != 0 {
            if (*curr).len >= 32768 as libc::c_uint as libc::c_ulong
                || ((*job).out).is_null()
            {
                dict = curr;
                use_space(dict);
            } else {
                dict = get_space(&mut dict_pool);
                len = (32768 as libc::c_uint as libc::c_ulong).wrapping_sub((*curr).len);
                memcpy(
                    (*dict).buf as *mut libc::c_void,
                    ((*(*job).out).buf)
                        .offset(((*(*job).out).len).wrapping_sub(len) as isize)
                        as *const libc::c_void,
                    len,
                );
                memcpy(
                    ((*dict).buf).offset(len as isize) as *mut libc::c_void,
                    (*curr).buf as *const libc::c_void,
                    (*curr).len,
                );
                (*dict).len = 32768 as libc::c_uint as size_t;
            }
        }
        (*job).seq = seq;
        seq += 1;
        if seq < 1 as libc::c_int as libc::c_long {
            try_throw_(
                34 as libc::c_int,
                b"overflow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        if (cthreads as libc::c_long) < seq && cthreads < g.procs {
            launch_(
                Some(compress_thread as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2228 as libc::c_int as libc::c_long,
            );
            cthreads += 1;
            cthreads;
        }
        possess_(
            compress_have,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2233 as libc::c_int as libc::c_long,
        );
        (*job).next = 0 as *mut job;
        *compress_tail = job;
        compress_tail = &mut (*job).next;
        twist_(
            compress_have,
            BY,
            1 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2237 as libc::c_int as libc::c_long,
        );
        if !(more != 0) {
            break;
        }
    }
    drop_space(next);
    join_(
        writeth,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2243 as libc::c_int as libc::c_long,
    );
    writeth = 0 as *mut thread;
}
unsafe extern "C" fn single_compress(mut reset: libc::c_int) {
    let mut got: size_t = 0;
    let mut more: size_t = 0;
    let mut start: size_t = 0;
    let mut have: size_t = 0;
    let mut hist: size_t = 0;
    let mut fresh: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut scan: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut left: size_t = 0;
    let mut head: libc::c_ulong = 0;
    let mut ulen: length_t = 0;
    let mut clen: length_t = 0;
    let mut check: libc::c_ulong = 0;
    static mut out_size: libc::c_uint = 0;
    static mut in_0: *mut libc::c_uchar = 0 as *const libc::c_uchar
        as *mut libc::c_uchar;
    static mut next: *mut libc::c_uchar = 0 as *const libc::c_uchar
        as *mut libc::c_uchar;
    static mut out: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
    static mut strm: *mut z_stream = 0 as *const z_stream as *mut z_stream;
    if reset != 0 {
        if !strm.is_null() {
            deflateEnd(strm);
            free(strm as *mut libc::c_void);
            free(out as *mut libc::c_void);
            free(next as *mut libc::c_void);
            free(in_0 as *mut libc::c_void);
            strm = 0 as *mut z_stream;
        }
        return;
    }
    if strm.is_null() {
        let mut ret: libc::c_int = 0;
        out_size = if g.block
            > (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_sub(
                    (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                ) as libc::c_ulong
        {
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_sub(
                    (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                )
        } else {
            g.block as libc::c_uint
        };
        in_0 = alloc(
            0 as *mut libc::c_void,
            (g.block).wrapping_add(32768 as libc::c_uint as libc::c_ulong),
        ) as *mut libc::c_uchar;
        next = alloc(
            0 as *mut libc::c_void,
            (g.block).wrapping_add(32768 as libc::c_uint as libc::c_ulong),
        ) as *mut libc::c_uchar;
        out = alloc(0 as *mut libc::c_void, out_size as size_t) as *mut libc::c_uchar;
        strm = alloc(
            0 as *mut libc::c_void,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong,
        ) as *mut z_stream;
        (*strm).zfree = None;
        (*strm).zalloc = None;
        (*strm).opaque = 0 as voidpf;
        ret = deflateInit2_(
            strm,
            6 as libc::c_int,
            8 as libc::c_int,
            -(15 as libc::c_int),
            8 as libc::c_int,
            g.strategy,
            b"1.2.11\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        );
        if ret == -(4 as libc::c_int) {
            try_throw_(
                12 as libc::c_int,
                b"not enough memory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        if ret != 0 as libc::c_int {
            try_throw_(
                22 as libc::c_int,
                b"internal error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
    }
    head = put_header();
    if g.level <= 9 as libc::c_int {
        deflateReset(strm);
        deflateParams(strm, g.level, g.strategy);
    }
    got = 0 as libc::c_int as size_t;
    more = readn(g.ind, next, g.block);
    ulen = more;
    start = 0 as libc::c_int as size_t;
    hist = 0 as libc::c_int as size_t;
    clen = 0 as libc::c_int as length_t;
    have = 0 as libc::c_int as size_t;
    check = if g.form == 1 as libc::c_int {
        adler32z(
            0 as libc::c_long as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        )
    } else {
        crc32z(
            0 as libc::c_long as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        )
    };
    hash = ((1 as libc::c_uint) << 12 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int;
    loop {
        if got == 0 as libc::c_int as libc::c_ulong {
            scan = in_0;
            in_0 = next;
            next = scan;
            (*strm).next_in = in_0.offset(start as isize);
            got = more;
            if g.level > 9 as libc::c_int {
                left = start.wrapping_add(more).wrapping_sub(hist);
                if left > 32768 as libc::c_uint as libc::c_ulong {
                    left = 32768 as libc::c_uint as size_t;
                }
                memcpy(
                    next as *mut libc::c_void,
                    in_0.offset(start.wrapping_add(more).wrapping_sub(left) as isize)
                        as *const libc::c_void,
                    left,
                );
                start = left;
                hist = 0 as libc::c_int as size_t;
            } else {
                start = 0 as libc::c_int as size_t;
            }
            more = readn(g.ind, next.offset(start as isize), g.block);
            ulen = (ulen as libc::c_ulong).wrapping_add(more) as length_t as length_t;
        }
        left = 0 as libc::c_int as size_t;
        if g.rsync != 0 && got != 0 {
            scan = (*strm).next_in;
            left = got;
            loop {
                if left == 0 as libc::c_int as libc::c_ulong {
                    if more == 0 as libc::c_int as libc::c_ulong || got == g.block {
                        break;
                    }
                    if g.level > 9 as libc::c_int {
                        left = (((*strm).next_in).offset_from(in_0) as libc::c_long
                            as size_t)
                            .wrapping_sub(hist);
                        if left > 32768 as libc::c_uint as libc::c_ulong {
                            left = 32768 as libc::c_uint as size_t;
                        }
                    }
                    memmove(
                        in_0 as *mut libc::c_void,
                        ((*strm).next_in).offset(-(left as isize))
                            as *const libc::c_void,
                        left.wrapping_add(got),
                    );
                    hist = 0 as libc::c_int as size_t;
                    (*strm).next_in = in_0.offset(left as isize);
                    scan = in_0.offset(left as isize).offset(got as isize);
                    left = if more > (g.block).wrapping_sub(got) {
                        (g.block).wrapping_sub(got)
                    } else {
                        more
                    };
                    memcpy(
                        scan as *mut libc::c_void,
                        next.offset(start as isize) as *const libc::c_void,
                        left,
                    );
                    got = (got as libc::c_ulong).wrapping_add(left) as size_t as size_t;
                    more = (more as libc::c_ulong).wrapping_sub(left) as size_t
                        as size_t;
                    start = (start as libc::c_ulong).wrapping_add(left) as size_t
                        as size_t;
                    if more == 0 as libc::c_int as libc::c_ulong {
                        more = readn(g.ind, next, g.block);
                        ulen = (ulen as libc::c_ulong).wrapping_add(more) as length_t
                            as length_t;
                        start = 0 as libc::c_int as size_t;
                    }
                }
                left = left.wrapping_sub(1);
                left;
                let fresh33 = scan;
                scan = scan.offset(1);
                hash = (hash << 1 as libc::c_int ^ *fresh33 as libc::c_uint)
                    & ((1 as libc::c_uint) << 12 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                if !(hash
                    != ((1 as libc::c_uint) << 12 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        >> 1 as libc::c_int)
                {
                    break;
                }
            }
            got = (got as libc::c_ulong).wrapping_sub(left) as size_t as size_t;
        }
        fresh = 0 as libc::c_int;
        if g.setdict == 0 {
            have = (have as libc::c_ulong).wrapping_add(got) as size_t as size_t;
            if have > g.block {
                fresh = 1 as libc::c_int;
                have = got;
            }
        }
        if g.level <= 9 as libc::c_int {
            if fresh != 0 {
                deflateReset(strm);
            }
            while got
                > (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_sub(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                    ) as libc::c_ulong
            {
                (*strm)
                    .avail_in = (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_sub(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                    );
                check = if g.form == 1 as libc::c_int {
                    adler32z(check, (*strm).next_in, (*strm).avail_in as size_t)
                } else {
                    crc32z(check, (*strm).next_in, (*strm).avail_in as size_t)
                };
                loop {
                    (*strm).avail_out = out_size;
                    (*strm).next_out = out;
                    deflate(strm, 0 as libc::c_int);
                    clen = (clen as libc::c_ulong)
                        .wrapping_add(
                            writen(
                                g.outd,
                                out as *const libc::c_void,
                                out_size.wrapping_sub((*strm).avail_out) as size_t,
                            ),
                        ) as length_t as length_t;
                    if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        2421 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 26],
                            &[libc::c_char; 26],
                        >(b"void single_compress(int)\0"))
                            .as_ptr(),
                    );
                }
                'c_13779: {
                    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            2421 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void single_compress(int)\0"))
                                .as_ptr(),
                        );
                    }
                };
                got = (got as libc::c_ulong)
                    .wrapping_sub(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as libc::c_ulong,
                    ) as size_t as size_t;
            }
            (*strm).avail_in = got as libc::c_uint;
            got = left;
            check = if g.form == 1 as libc::c_int {
                adler32z(check, (*strm).next_in, (*strm).avail_in as size_t)
            } else {
                crc32z(check, (*strm).next_in, (*strm).avail_in as size_t)
            };
            if more != 0 || got != 0 {
                if zlib_vernum() >= 0x1260 as libc::c_int as libc::c_long {
                    let mut bits: libc::c_int = 0;
                    loop {
                        (*strm).avail_out = out_size;
                        (*strm).next_out = out;
                        deflate(strm, 5 as libc::c_int);
                        clen = (clen as libc::c_ulong)
                            .wrapping_add(
                                writen(
                                    g.outd,
                                    out as *const libc::c_void,
                                    out_size.wrapping_sub((*strm).avail_out) as size_t,
                                ),
                            ) as length_t as length_t;
                        if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
                            break;
                        }
                    }
                    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            2434 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void single_compress(int)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_13596: {
                        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2434 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 26],
                                    &[libc::c_char; 26],
                                >(b"void single_compress(int)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    deflatePending(strm, 0 as *mut libc::c_uint, &mut bits);
                    if bits & 1 as libc::c_int != 0 || g.setdict == 0 {
                        loop {
                            (*strm).avail_out = out_size;
                            (*strm).next_out = out;
                            deflate(strm, 2 as libc::c_int);
                            clen = (clen as libc::c_ulong)
                                .wrapping_add(
                                    writen(
                                        g.outd,
                                        out as *const libc::c_void,
                                        out_size.wrapping_sub((*strm).avail_out) as size_t,
                                    ),
                                ) as length_t as length_t;
                            if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
                                break;
                            }
                        }
                        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2437 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 26],
                                    &[libc::c_char; 26],
                                >(b"void single_compress(int)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_13481: {
                            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                            {} else {
                                __assert_fail(
                                    b"strm->avail_in == 0\0" as *const u8
                                        as *const libc::c_char,
                                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                                    2437 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 26],
                                        &[libc::c_char; 26],
                                    >(b"void single_compress(int)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                    } else if bits & 7 as libc::c_int != 0 {
                        loop {
                            bits = deflatePrime(
                                strm,
                                10 as libc::c_int,
                                2 as libc::c_int,
                            );
                            if bits == 0 as libc::c_int {} else {
                                __assert_fail(
                                    b"bits == Z_OK\0" as *const u8 as *const libc::c_char,
                                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                                    2441 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 26],
                                        &[libc::c_char; 26],
                                    >(b"void single_compress(int)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_13429: {
                                if bits == 0 as libc::c_int {} else {
                                    __assert_fail(
                                        b"bits == Z_OK\0" as *const u8 as *const libc::c_char,
                                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                                        2441 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 26],
                                            &[libc::c_char; 26],
                                        >(b"void single_compress(int)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            deflatePending(strm, 0 as *mut libc::c_uint, &mut bits);
                            if !(bits & 7 as libc::c_int != 0) {
                                break;
                            }
                        }
                        loop {
                            (*strm).avail_out = out_size;
                            (*strm).next_out = out;
                            deflate(strm, 0 as libc::c_int);
                            clen = (clen as libc::c_ulong)
                                .wrapping_add(
                                    writen(
                                        g.outd,
                                        out as *const libc::c_void,
                                        out_size.wrapping_sub((*strm).avail_out) as size_t,
                                    ),
                                ) as length_t as length_t;
                            if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
                                break;
                            }
                        }
                        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2444 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 26],
                                    &[libc::c_char; 26],
                                >(b"void single_compress(int)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_13318: {
                            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                            {} else {
                                __assert_fail(
                                    b"strm->avail_in == 0\0" as *const u8
                                        as *const libc::c_char,
                                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                                    2444 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 26],
                                        &[libc::c_char; 26],
                                    >(b"void single_compress(int)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                    }
                } else {
                    loop {
                        (*strm).avail_out = out_size;
                        (*strm).next_out = out;
                        deflate(strm, 2 as libc::c_int);
                        clen = (clen as libc::c_ulong)
                            .wrapping_add(
                                writen(
                                    g.outd,
                                    out as *const libc::c_void,
                                    out_size.wrapping_sub((*strm).avail_out) as size_t,
                                ),
                            ) as length_t as length_t;
                        if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
                            break;
                        }
                    }
                    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            2448 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void single_compress(int)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_13211: {
                        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2448 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 26],
                                    &[libc::c_char; 26],
                                >(b"void single_compress(int)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                if g.setdict == 0 {
                    loop {
                        (*strm).avail_out = out_size;
                        (*strm).next_out = out;
                        deflate(strm, 3 as libc::c_int);
                        clen = (clen as libc::c_ulong)
                            .wrapping_add(
                                writen(
                                    g.outd,
                                    out as *const libc::c_void,
                                    out_size.wrapping_sub((*strm).avail_out) as size_t,
                                ),
                            ) as length_t as length_t;
                        if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
                            break;
                        }
                    }
                    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            2453 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void single_compress(int)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_13109: {
                        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                            __assert_fail(
                                b"strm->avail_in == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2453 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 26],
                                    &[libc::c_char; 26],
                                >(b"void single_compress(int)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
            } else {
                loop {
                    (*strm).avail_out = out_size;
                    (*strm).next_out = out;
                    deflate(strm, 4 as libc::c_int);
                    clen = (clen as libc::c_ulong)
                        .wrapping_add(
                            writen(
                                g.outd,
                                out as *const libc::c_void,
                                out_size.wrapping_sub((*strm).avail_out) as size_t,
                            ),
                        ) as length_t as length_t;
                    if !((*strm).avail_out == 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        2456 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 26],
                            &[libc::c_char; 26],
                        >(b"void single_compress(int)\0"))
                            .as_ptr(),
                    );
                }
                'c_13007: {
                    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"strm->avail_in == 0\0" as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            2456 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void single_compress(int)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
        } else {
            let mut bits_0: libc::c_uchar = 0;
            let mut def: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut size: size_t = 0;
            let mut off: size_t = 0;
            off = ((*strm).next_in).offset_from(in_0) as libc::c_long as size_t;
            if fresh != 0 {
                hist = off;
            }
            def = 0 as *mut libc::c_uchar;
            size = 0 as libc::c_int as size_t;
            bits_0 = 0 as libc::c_int as libc::c_uchar;
            ZopfliDeflatePart(
                &mut g.zopts,
                2 as libc::c_int,
                !(more != 0 || left != 0) as libc::c_int,
                in_0.offset(hist as isize),
                off.wrapping_sub(hist),
                off.wrapping_sub(hist).wrapping_add(got),
                &mut bits_0,
                &mut def,
                &mut size,
            );
            bits_0 = (bits_0 as libc::c_int & 7 as libc::c_int) as libc::c_uchar;
            if more != 0 || left != 0 {
                if bits_0 as libc::c_int & 1 as libc::c_int != 0 || g.setdict == 0 {
                    writen(g.outd, def as *const libc::c_void, size);
                    if bits_0 as libc::c_int == 0 as libc::c_int
                        || bits_0 as libc::c_int > 5 as libc::c_int
                    {
                        writen(
                            g.outd,
                            b"\0\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_uchar as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                    }
                    writen(
                        g.outd,
                        b"\0\0\xFF\xFF\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_uchar as *const libc::c_void,
                        4 as libc::c_int as size_t,
                    );
                } else {
                    if size > 0 as libc::c_int as libc::c_ulong {} else {
                        __assert_fail(
                            b"size > 0\0" as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            2484 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void single_compress(int)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_12811: {
                        if size > 0 as libc::c_int as libc::c_ulong {} else {
                            __assert_fail(
                                b"size > 0\0" as *const u8 as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                2484 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 26],
                                    &[libc::c_char; 26],
                                >(b"void single_compress(int)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    writen(
                        g.outd,
                        def as *const libc::c_void,
                        size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    if bits_0 != 0 {
                        loop {
                            let ref mut fresh34 = *def
                                .offset(
                                    size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                );
                            *fresh34 = (*fresh34 as libc::c_int
                                + ((2 as libc::c_int) << bits_0 as libc::c_int))
                                as libc::c_uchar;
                            writen(
                                g.outd,
                                def
                                    .offset(size as isize)
                                    .offset(-(1 as libc::c_int as isize))
                                    as *const libc::c_void,
                                1 as libc::c_int as size_t,
                            );
                            *def
                                .offset(
                                    size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) = 0 as libc::c_int as libc::c_uchar;
                            bits_0 = (bits_0 as libc::c_int + 2 as libc::c_int)
                                as libc::c_uchar;
                            if !((bits_0 as libc::c_int) < 8 as libc::c_int) {
                                break;
                            }
                        }
                    }
                    writen(
                        g.outd,
                        def.offset(size as isize).offset(-(1 as libc::c_int as isize))
                            as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                }
                if g.setdict == 0 {
                    writen(
                        g.outd,
                        b"\0\0\0\xFF\xFF\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_uchar as *const libc::c_void,
                        5 as libc::c_int as size_t,
                    );
                }
            } else {
                writen(g.outd, def as *const libc::c_void, size);
            }
            free(def as *mut libc::c_void);
            while got
                > (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_sub(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                    ) as libc::c_ulong
            {
                check = if g.form == 1 as libc::c_int {
                    adler32z(
                        check,
                        (*strm).next_in,
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as size_t,
                    )
                } else {
                    crc32z(
                        check,
                        (*strm).next_in,
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as size_t,
                    )
                };
                (*strm)
                    .next_in = ((*strm).next_in)
                    .offset(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as isize,
                    );
                got = (got as libc::c_ulong)
                    .wrapping_sub(
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int,
                            ) as libc::c_ulong,
                    ) as size_t as size_t;
            }
            check = if g.form == 1 as libc::c_int {
                adler32z(check, (*strm).next_in, got as libc::c_uint as size_t)
            } else {
                crc32z(check, (*strm).next_in, got as libc::c_uint as size_t)
            };
            (*strm).next_in = ((*strm).next_in).offset(got as isize);
            got = left;
        }
        if !(more != 0 || got != 0) {
            break;
        }
    }
    put_trailer(ulen, clen, check, head);
}
unsafe extern "C" fn load_read(mut dummy: *mut libc::c_void) {
    let mut len: size_t = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_char;
    try_setup_();
    try_this_.next = pthread_getspecific(try_key_) as *mut try_t_;
    if pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    ) == 0 as libc::c_int
        && !(b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2531 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void load_read(void *)\0"))
                .as_ptr(),
        );
    }
    'c_15018: {
        if pthread_setspecific(
            try_key_,
            &mut try_this_ as *mut try_t_ as *const libc::c_void,
        ) == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2531 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void load_read(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if _setjmp((try_this_.env).as_mut_ptr()) == 0 as libc::c_int {
        loop {
            possess_(
                g.load_state,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2533 as libc::c_int as libc::c_long,
            );
            wait_for_(
                g.load_state,
                NOT_TO_BE,
                0 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2534 as libc::c_int as libc::c_long,
            );
            if peek_lock(g.load_state) > 1 as libc::c_int as libc::c_long {
                release_(
                    g.load_state,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2536 as libc::c_int as libc::c_long,
                );
                break;
            } else {
                len = readn(
                    g.ind,
                    if g.in_which != 0 {
                        (g.in_buf).as_mut_ptr()
                    } else {
                        (g.in_buf2).as_mut_ptr()
                    },
                    32768 as libc::c_int as size_t,
                );
                g.in_len = len;
                twist_(
                    g.load_state,
                    TO,
                    0 as libc::c_int as libc::c_long,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2542 as libc::c_int as libc::c_long,
                );
                if !(len == 32768 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
        }
    }
    if try_pushed_ != 0 {
        if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
            == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2545 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void load_read(void *)\0"))
                    .as_ptr(),
            );
        }
        'c_14848: {
            if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    2545 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"void load_read(void *)\0"))
                        .as_ptr(),
                );
            }
        };
        ::std::ptr::write_volatile(
            &mut try_pushed_ as *mut libc::c_int,
            0 as libc::c_int,
        );
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn load_wait() {
    if g.in_which == -(1 as libc::c_int) {
        return;
    }
    possess_(
        g.load_state,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2556 as libc::c_int as libc::c_long,
    );
    wait_for_(
        g.load_state,
        TO_BE,
        0 as libc::c_int as libc::c_long,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2557 as libc::c_int as libc::c_long,
    );
    release_(
        g.load_state,
        b"pigz.c\0" as *const u8 as *const libc::c_char,
        2558 as libc::c_int as libc::c_long,
    );
}
unsafe extern "C" fn load() -> size_t {
    if g.in_short != 0 {
        g.in_eof = 1 as libc::c_int;
        g.in_left = 0 as libc::c_int as size_t;
        return 0 as libc::c_int as size_t;
    }
    if g.procs > 1 as libc::c_int {
        if g.in_which == -(1 as libc::c_int) {
            g.in_which = 1 as libc::c_int;
            g
                .load_state = new_lock_(
                1 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2582 as libc::c_int as libc::c_long,
            );
            g
                .load_thread = launch_(
                Some(load_read as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2583 as libc::c_int as libc::c_long,
            );
        }
        load_wait();
        g
            .in_next = if g.in_which != 0 {
            (g.in_buf).as_mut_ptr()
        } else {
            (g.in_buf2).as_mut_ptr()
        };
        g.in_left = g.in_len;
        if g.in_len == 32768 as libc::c_int as libc::c_ulong {
            g.in_which = 1 as libc::c_int - g.in_which;
            possess_(
                g.load_state,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2597 as libc::c_int as libc::c_long,
            );
            twist_(
                g.load_state,
                TO,
                1 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2598 as libc::c_int as libc::c_long,
            );
        } else {
            join_(
                g.load_thread,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2603 as libc::c_int as libc::c_long,
            );
            free_lock_(
                g.load_state,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                2604 as libc::c_int as libc::c_long,
            );
            g.in_which = -(1 as libc::c_int);
        }
    } else {
        g.in_next = (g.in_buf).as_mut_ptr();
        g.in_left = readn(g.ind, g.in_next, 32768 as libc::c_int as size_t);
    }
    if g.in_left < 32768 as libc::c_int as libc::c_ulong {
        g.in_short = 1 as libc::c_int;
        if g.in_left == 0 as libc::c_int as libc::c_ulong {
            g.in_eof = 1 as libc::c_int;
        }
    }
    g
        .in_tot = (g.in_tot as libc::c_ulong).wrapping_add(g.in_left) as length_t
        as length_t;
    return g.in_left;
}
unsafe extern "C" fn load_end() {
    if g.in_which != -(1 as libc::c_int) {
        possess_(
            g.load_state,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2637 as libc::c_int as libc::c_long,
        );
        wait_for_(
            g.load_state,
            TO_BE,
            0 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2638 as libc::c_int as libc::c_long,
        );
        twist_(
            g.load_state,
            TO,
            2 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2639 as libc::c_int as libc::c_long,
        );
        join_(
            g.load_thread,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2642 as libc::c_int as libc::c_long,
        );
        free_lock_(
            g.load_state,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            2643 as libc::c_int as libc::c_long,
        );
        g.in_which = -(1 as libc::c_int);
    }
    g.in_left = 0 as libc::c_int as size_t;
    g.in_short = 1 as libc::c_int;
    g.in_eof = 1 as libc::c_int;
    if g.ind != 0 as libc::c_int {
        close(g.ind);
    }
    if !(g.hname).is_null() {
        free(g.hname as *mut libc::c_void);
        g.hname = 0 as *mut libc::c_char;
    }
    if !(g.hcomm).is_null() {
        free(g.hcomm as *mut libc::c_void);
        g.hcomm = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn in_init() {
    g.in_left = 0 as libc::c_int as size_t;
    g.in_eof = 0 as libc::c_int;
    g.in_short = 0 as libc::c_int;
    g.in_tot = 0 as libc::c_int as length_t;
    g.in_which = -(1 as libc::c_int);
}
unsafe extern "C" fn dos2time(mut dos: libc::c_ulong) -> time_t {
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
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    if dos == 0 as libc::c_int as libc::c_ulong {
        return time(0 as *mut time_t);
    }
    tm
        .tm_year = ((dos >> 25 as libc::c_int) as libc::c_int & 0x7f as libc::c_int)
        + 80 as libc::c_int;
    tm
        .tm_mon = ((dos >> 21 as libc::c_int) as libc::c_int & 0xf as libc::c_int)
        - 1 as libc::c_int;
    tm.tm_mday = (dos >> 16 as libc::c_int) as libc::c_int & 0x1f as libc::c_int;
    tm.tm_hour = (dos >> 11 as libc::c_int) as libc::c_int & 0x1f as libc::c_int;
    tm.tm_min = (dos >> 5 as libc::c_int) as libc::c_int & 0x3f as libc::c_int;
    tm.tm_sec = (dos << 1 as libc::c_int) as libc::c_int & 0x3e as libc::c_int;
    tm.tm_isdst = -(1 as libc::c_int);
    return mktime(&mut tm);
}
unsafe extern "C" fn tolong(mut val: libc::c_ulong) -> libc::c_long {
    return (val & 0x7fffffff as libc::c_ulong) as libc::c_long
        - (val & 0x80000000 as libc::c_ulong) as libc::c_long;
}
unsafe extern "C" fn read_extra(
    mut len: libc::c_uint,
    mut save: libc::c_int,
) -> libc::c_int {
    let mut id: libc::c_uint = 0;
    let mut size: libc::c_uint = 0;
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    while len >= 4 as libc::c_int as libc::c_uint {
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh35 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh35 as libc::c_int
        }) as libc::c_uint;
        id = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh36 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh36 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            );
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh37 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh37 as libc::c_int
        }) as libc::c_uint;
        size = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh38 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh38 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            );
        if g.in_eof != 0 {
            return -(1 as libc::c_int);
        }
        len = len.wrapping_sub(4 as libc::c_int as libc::c_uint);
        if size > len {
            break;
        }
        len = len.wrapping_sub(size);
        if id == 0x1 as libc::c_int as libc::c_uint {
            g.zip64 = 1 as libc::c_int;
            if g.zip_ulen == 0xffffffff as libc::c_uint as libc::c_ulong
                && size >= 8 as libc::c_int as libc::c_uint
            {
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh39 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh39 as libc::c_int
                }) as libc::c_uint;
                tmp4 = tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh40 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh40 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong;
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh41 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh41 as libc::c_int
                }) as libc::c_uint;
                g
                    .zip_ulen = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add(
                                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                    && (g.in_eof != 0
                                        || load() == 0 as libc::c_int as libc::c_ulong)
                                {
                                    0 as libc::c_int
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    g.in_left;
                                    let fresh42 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    *fresh42 as libc::c_int
                                }) as libc::c_uint) << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    );
                let mut togo: size_t = 4 as libc::c_int as size_t;
                while togo > g.in_left {
                    togo = (togo as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                        as size_t;
                    if load() == 0 as libc::c_int as libc::c_ulong {
                        return -(3 as libc::c_int);
                    }
                }
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo) as size_t
                    as size_t;
                g.in_next = (g.in_next).offset(togo as isize);
                size = size.wrapping_sub(8 as libc::c_int as libc::c_uint);
            }
            if g.zip_clen == 0xffffffff as libc::c_uint as libc::c_ulong
                && size >= 8 as libc::c_int as libc::c_uint
            {
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh43 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh43 as libc::c_int
                }) as libc::c_uint;
                tmp4 = tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh44 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh44 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong;
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh45 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh45 as libc::c_int
                }) as libc::c_uint;
                g
                    .zip_clen = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add(
                                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                    && (g.in_eof != 0
                                        || load() == 0 as libc::c_int as libc::c_ulong)
                                {
                                    0 as libc::c_int
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    g.in_left;
                                    let fresh46 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    *fresh46 as libc::c_int
                                }) as libc::c_uint) << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    );
                let mut togo_0: size_t = 4 as libc::c_int as size_t;
                while togo_0 > g.in_left {
                    togo_0 = (togo_0 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                        as size_t;
                    if load() == 0 as libc::c_int as libc::c_ulong {
                        return -(3 as libc::c_int);
                    }
                }
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_0)
                    as size_t as size_t;
                g.in_next = (g.in_next).offset(togo_0 as isize);
                size = size.wrapping_sub(8 as libc::c_int as libc::c_uint);
            }
        }
        if save != 0 {
            if (id == 0xd as libc::c_int as libc::c_uint
                || id == 0x5855 as libc::c_int as libc::c_uint)
                && size >= 8 as libc::c_int as libc::c_uint
            {
                let mut togo_1: size_t = 4 as libc::c_int as size_t;
                while togo_1 > g.in_left {
                    togo_1 = (togo_1 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                        as size_t;
                    if load() == 0 as libc::c_int as libc::c_ulong {
                        return -(3 as libc::c_int);
                    }
                }
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_1)
                    as size_t as size_t;
                g.in_next = (g.in_next).offset(togo_1 as isize);
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh47 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh47 as libc::c_int
                }) as libc::c_uint;
                tmp4 = tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh48 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh48 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong;
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh49 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh49 as libc::c_int
                }) as libc::c_uint;
                g
                    .stamp = tolong(
                    tmp4
                        .wrapping_add(
                            (tmp2
                                .wrapping_add(
                                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                        && (g.in_eof != 0
                                            || load() == 0 as libc::c_int as libc::c_ulong)
                                    {
                                        0 as libc::c_int
                                    } else {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        g.in_left;
                                        let fresh50 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                        *fresh50 as libc::c_int
                                    }) as libc::c_uint) << 8 as libc::c_int,
                                ) as libc::c_ulong) << 16 as libc::c_int,
                        ),
                );
                size = size.wrapping_sub(8 as libc::c_int as libc::c_uint);
            }
            if id == 0x5455 as libc::c_int as libc::c_uint
                && size >= 5 as libc::c_int as libc::c_uint
            {
                size = size.wrapping_sub(1);
                size;
                if (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh51 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh51 as libc::c_int
                }) & 1 as libc::c_int != 0
                {
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh52 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh52 as libc::c_int
                    }) as libc::c_uint;
                    tmp4 = tmp2
                        .wrapping_add(
                            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                && (g.in_eof != 0
                                    || load() == 0 as libc::c_int as libc::c_ulong)
                            {
                                0 as libc::c_int
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                g.in_left;
                                let fresh53 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                *fresh53 as libc::c_int
                            }) as libc::c_uint) << 8 as libc::c_int,
                        ) as libc::c_ulong;
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh54 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh54 as libc::c_int
                    }) as libc::c_uint;
                    g
                        .stamp = tolong(
                        tmp4
                            .wrapping_add(
                                (tmp2
                                    .wrapping_add(
                                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                            && (g.in_eof != 0
                                                || load() == 0 as libc::c_int as libc::c_ulong)
                                        {
                                            0 as libc::c_int
                                        } else {
                                            g.in_left = (g.in_left).wrapping_sub(1);
                                            g.in_left;
                                            let fresh55 = g.in_next;
                                            g.in_next = (g.in_next).offset(1);
                                            *fresh55 as libc::c_int
                                        }) as libc::c_uint) << 8 as libc::c_int,
                                    ) as libc::c_ulong) << 16 as libc::c_int,
                            ),
                    );
                    size = size.wrapping_sub(4 as libc::c_int as libc::c_uint);
                }
            }
        }
        let mut togo_2: size_t = size as size_t;
        while togo_2 > g.in_left {
            togo_2 = (togo_2 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                as size_t;
            if load() == 0 as libc::c_int as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        g
            .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_2) as size_t
            as size_t;
        g.in_next = (g.in_next).offset(togo_2 as isize);
    }
    let mut togo_3: size_t = len as size_t;
    while togo_3 > g.in_left {
        togo_3 = (togo_3 as libc::c_ulong).wrapping_sub(g.in_left) as size_t as size_t;
        if load() == 0 as libc::c_int as libc::c_ulong {
            return -(3 as libc::c_int);
        }
    }
    g.in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_3) as size_t as size_t;
    g.in_next = (g.in_next).offset(togo_3 as isize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_header(mut save: libc::c_int) -> libc::c_int {
    let mut magic: libc::c_uint = 0;
    let mut method: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut fname: libc::c_uint = 0;
    let mut extra: libc::c_uint = 0;
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    let mut crc: libc::c_ulong = 0;
    if save != 0 {
        g.stamp = 0 as libc::c_int as time_t;
        if !(g.hname).is_null() {
            free(g.hname as *mut libc::c_void);
            g.hname = 0 as *mut libc::c_char;
        }
        if !(g.hcomm).is_null() {
            free(g.hcomm as *mut libc::c_void);
            g.hcomm = 0 as *mut libc::c_char;
        }
    }
    g
        .magic1 = if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        0 as libc::c_int
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        g.in_left;
        let fresh56 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        *fresh56 as libc::c_int
    };
    if g.in_eof != 0 {
        g.magic1 = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    magic = (g.magic1 as libc::c_uint) << 8 as libc::c_int;
    magic = magic
        .wrapping_add(
            (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh57 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh57 as libc::c_int
            }) as libc::c_uint,
        );
    if g.in_eof != 0 {
        return -(2 as libc::c_int);
    }
    if magic.wrapping_rem(31 as libc::c_int as libc::c_uint)
        == 0 as libc::c_int as libc::c_uint
        && magic & 0x8f20 as libc::c_int as libc::c_uint
            == 0x800 as libc::c_int as libc::c_uint
    {
        g.form = 1 as libc::c_int;
        return 8 as libc::c_int;
    }
    if magic == 0x1f9d as libc::c_int as libc::c_uint {
        g.form = -(1 as libc::c_int);
        return 257 as libc::c_int;
    }
    if magic == 0x504b as libc::c_int as libc::c_uint {
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh58 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh58 as libc::c_int
        }) as libc::c_uint;
        magic = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh59 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh59 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            );
        if g.in_eof != 0 {
            return -(3 as libc::c_int);
        }
        if magic == 0x201 as libc::c_int as libc::c_uint
            || magic == 0x806 as libc::c_int as libc::c_uint
        {
            return -(5 as libc::c_int);
        }
        if magic != 0x403 as libc::c_int as libc::c_uint {
            return -(4 as libc::c_int);
        }
        g.zip64 = 0 as libc::c_int;
        let mut togo: size_t = 2 as libc::c_int as size_t;
        while togo > g.in_left {
            togo = (togo as libc::c_ulong).wrapping_sub(g.in_left) as size_t as size_t;
            if load() == 0 as libc::c_int as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        g.in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo) as size_t as size_t;
        g.in_next = (g.in_next).offset(togo as isize);
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh60 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh60 as libc::c_int
        }) as libc::c_uint;
        flags = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh61 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh61 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            );
        if flags & 0xf7f0 as libc::c_int as libc::c_uint != 0 {
            return -(4 as libc::c_int);
        }
        method = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh62 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh62 as libc::c_int
        }) as libc::c_uint;
        if (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh63 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh63 as libc::c_int
        }) != 0 as libc::c_int || flags & 1 as libc::c_int as libc::c_uint != 0
        {
            method = 256 as libc::c_int as libc::c_uint;
        }
        if save != 0 {
            tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh64 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh64 as libc::c_int
            }) as libc::c_uint;
            tmp4 = tmp2
                .wrapping_add(
                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh65 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh65 as libc::c_int
                    }) as libc::c_uint) << 8 as libc::c_int,
                ) as libc::c_ulong;
            tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh66 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh66 as libc::c_int
            }) as libc::c_uint;
            g
                .stamp = dos2time(
                tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add(
                                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                    && (g.in_eof != 0
                                        || load() == 0 as libc::c_int as libc::c_ulong)
                                {
                                    0 as libc::c_int
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    g.in_left;
                                    let fresh67 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    *fresh67 as libc::c_int
                                }) as libc::c_uint) << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    ),
            );
        } else {
            let mut togo_0: size_t = 4 as libc::c_int as size_t;
            while togo_0 > g.in_left {
                togo_0 = (togo_0 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                    as size_t;
                if load() == 0 as libc::c_int as libc::c_ulong {
                    return -(3 as libc::c_int);
                }
            }
            g
                .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_0) as size_t
                as size_t;
            g.in_next = (g.in_next).offset(togo_0 as isize);
        }
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh68 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh68 as libc::c_int
        }) as libc::c_uint;
        tmp4 = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh69 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh69 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            ) as libc::c_ulong;
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh70 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh70 as libc::c_int
        }) as libc::c_uint;
        g
            .zip_crc = tmp4
            .wrapping_add(
                (tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh71 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh71 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong) << 16 as libc::c_int,
            );
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh72 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh72 as libc::c_int
        }) as libc::c_uint;
        tmp4 = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh73 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh73 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            ) as libc::c_ulong;
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh74 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh74 as libc::c_int
        }) as libc::c_uint;
        g
            .zip_clen = tmp4
            .wrapping_add(
                (tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh75 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh75 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong) << 16 as libc::c_int,
            );
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh76 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh76 as libc::c_int
        }) as libc::c_uint;
        tmp4 = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh77 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh77 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            ) as libc::c_ulong;
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh78 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh78 as libc::c_int
        }) as libc::c_uint;
        g
            .zip_ulen = tmp4
            .wrapping_add(
                (tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh79 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh79 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong) << 16 as libc::c_int,
            );
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh80 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh80 as libc::c_int
        }) as libc::c_uint;
        fname = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh81 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh81 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            );
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh82 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh82 as libc::c_int
        }) as libc::c_uint;
        extra = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh83 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh83 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            );
        if save != 0 {
            let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
            if g.in_eof != 0 {
                return -(3 as libc::c_int);
            }
            g
                .hname = alloc(
                0 as *mut libc::c_void,
                fname.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
            ) as *mut libc::c_char;
            next = g.hname;
            while fname as libc::c_ulong > g.in_left {
                memcpy(
                    next as *mut libc::c_void,
                    g.in_next as *const libc::c_void,
                    g.in_left,
                );
                fname = (fname as libc::c_ulong).wrapping_sub(g.in_left) as libc::c_uint
                    as libc::c_uint;
                next = next.offset(g.in_left as isize);
                if load() == 0 as libc::c_int as libc::c_ulong {
                    return -(3 as libc::c_int);
                }
            }
            memcpy(
                next as *mut libc::c_void,
                g.in_next as *const libc::c_void,
                fname as libc::c_ulong,
            );
            g
                .in_left = (g.in_left as libc::c_ulong)
                .wrapping_sub(fname as libc::c_ulong) as size_t as size_t;
            g.in_next = (g.in_next).offset(fname as isize);
            next = next.offset(fname as isize);
            *next = 0 as libc::c_int as libc::c_char;
        } else {
            let mut togo_1: size_t = fname as size_t;
            while togo_1 > g.in_left {
                togo_1 = (togo_1 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                    as size_t;
                if load() == 0 as libc::c_int as libc::c_ulong {
                    return -(3 as libc::c_int);
                }
            }
            g
                .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_1) as size_t
                as size_t;
            g.in_next = (g.in_next).offset(togo_1 as isize);
        }
        read_extra(extra, save);
        g
            .form = (2 as libc::c_int as libc::c_uint)
            .wrapping_add((flags & 8 as libc::c_int as libc::c_uint) >> 3 as libc::c_int)
            as libc::c_int;
        return if g.in_eof != 0 { -(3 as libc::c_int) } else { method as libc::c_int };
    }
    if magic != 0x1f8b as libc::c_int as libc::c_uint {
        g.in_left = (g.in_left).wrapping_add(1);
        g.in_left;
        g.in_next = (g.in_next).offset(-1);
        g.in_next;
        return -(2 as libc::c_int);
    }
    crc = 0xf6e946c9 as libc::c_uint as libc::c_ulong;
    method = (if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        0 as libc::c_int
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        g.in_left;
        crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
        let fresh84 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        *fresh84 as libc::c_int
    }) as libc::c_uint;
    flags = (if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        0 as libc::c_int
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        g.in_left;
        crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
        let fresh85 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        *fresh85 as libc::c_int
    }) as libc::c_uint;
    if flags & 0xe0 as libc::c_int as libc::c_uint != 0 {
        return -(4 as libc::c_int);
    }
    if save != 0 {
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
            let fresh86 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh86 as libc::c_int
        }) as libc::c_uint;
        tmp4 = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
                    let fresh87 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh87 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            ) as libc::c_ulong;
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
            let fresh88 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh88 as libc::c_int
        }) as libc::c_uint;
        g
            .stamp = tolong(
            tmp4
                .wrapping_add(
                    (tmp2
                        .wrapping_add(
                            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                && (g.in_eof != 0
                                    || load() == 0 as libc::c_int as libc::c_ulong)
                            {
                                0 as libc::c_int
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                g.in_left;
                                crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
                                let fresh89 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                *fresh89 as libc::c_int
                            }) as libc::c_uint) << 8 as libc::c_int,
                        ) as libc::c_ulong) << 16 as libc::c_int,
                ),
        );
    } else {
        let mut togo_2: size_t = 4 as libc::c_int as size_t;
        while togo_2 > g.in_left {
            crc = crc32z(crc, g.in_next, g.in_left);
            togo_2 = (togo_2 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                as size_t;
            if load() == 0 as libc::c_int as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        crc = crc32z(crc, g.in_next, togo_2);
        g
            .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_2) as size_t
            as size_t;
        g.in_next = (g.in_next).offset(togo_2 as isize);
    }
    let mut togo_3: size_t = 2 as libc::c_int as size_t;
    while togo_3 > g.in_left {
        crc = crc32z(crc, g.in_next, g.in_left);
        togo_3 = (togo_3 as libc::c_ulong).wrapping_sub(g.in_left) as size_t as size_t;
        if load() == 0 as libc::c_int as libc::c_ulong {
            return -(3 as libc::c_int);
        }
    }
    crc = crc32z(crc, g.in_next, togo_3);
    g.in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_3) as size_t as size_t;
    g.in_next = (g.in_next).offset(togo_3 as isize);
    if flags & 4 as libc::c_int as libc::c_uint != 0 {
        tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            0 as libc::c_int
        } else {
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
            let fresh90 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            *fresh90 as libc::c_int
        }) as libc::c_uint;
        let mut togo_4: size_t = tmp2
            .wrapping_add(
                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
                    let fresh91 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh91 as libc::c_int
                }) as libc::c_uint) << 8 as libc::c_int,
            ) as size_t;
        while togo_4 > g.in_left {
            crc = crc32z(crc, g.in_next, g.in_left);
            togo_4 = (togo_4 as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                as size_t;
            if load() == 0 as libc::c_int as libc::c_ulong {
                return -(3 as libc::c_int);
            }
        }
        crc = crc32z(crc, g.in_next, togo_4);
        g
            .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo_4) as size_t
            as size_t;
        g.in_next = (g.in_next).offset(togo_4 as isize);
    }
    if flags & 8 as libc::c_int as libc::c_uint != 0 {
        if save != 0 {
            let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut copy: size_t = 0;
            let mut have: size_t = 0;
            let mut size: size_t = 0 as libc::c_int as size_t;
            have = 0 as libc::c_int as size_t;
            loop {
                if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && load() == 0 as libc::c_int as libc::c_ulong
                {
                    return -(3 as libc::c_int);
                }
                end = memchr(
                    g.in_next as *const libc::c_void,
                    0 as libc::c_int,
                    g.in_left,
                ) as *mut libc::c_uchar;
                copy = if end.is_null() {
                    g.in_left
                } else {
                    (end.offset_from(g.in_next) as libc::c_long as size_t)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                have = vmemcpy(
                    &mut g.hname,
                    &mut size,
                    have,
                    g.in_next as *mut libc::c_void,
                    copy,
                );
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(copy) as size_t
                    as size_t;
                g.in_next = (g.in_next).offset(copy as isize);
                if !end.is_null() {
                    break;
                }
            }
            crc = crc32z(crc, g.hname as *mut libc::c_uchar, have);
        } else {
            while (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
                let fresh92 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh92 as libc::c_int
            }) != 0 as libc::c_int
            {}
        }
    }
    if flags & 16 as libc::c_int as libc::c_uint != 0 {
        if save != 0 {
            let mut end_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut copy_0: size_t = 0;
            let mut have_0: size_t = 0;
            let mut size_0: size_t = 0 as libc::c_int as size_t;
            have_0 = 0 as libc::c_int as size_t;
            loop {
                if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && load() == 0 as libc::c_int as libc::c_ulong
                {
                    return -(3 as libc::c_int);
                }
                end_0 = memchr(
                    g.in_next as *const libc::c_void,
                    0 as libc::c_int,
                    g.in_left,
                ) as *mut libc::c_uchar;
                copy_0 = if end_0.is_null() {
                    g.in_left
                } else {
                    (end_0.offset_from(g.in_next) as libc::c_long as size_t)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                };
                have_0 = vmemcpy(
                    &mut g.hcomm,
                    &mut size_0,
                    have_0,
                    g.in_next as *mut libc::c_void,
                    copy_0,
                );
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(copy_0)
                    as size_t as size_t;
                g.in_next = (g.in_next).offset(copy_0 as isize);
                if !end_0.is_null() {
                    break;
                }
            }
            crc = crc32z(crc, g.hcomm as *mut libc::c_uchar, have_0);
        } else {
            while (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                crc = crc32z(crc, g.in_next, 1 as libc::c_int as size_t);
                let fresh93 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh93 as libc::c_int
            }) != 0 as libc::c_int
            {}
        }
    }
    if flags & 2 as libc::c_int as libc::c_uint != 0
        && {
            tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh94 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh94 as libc::c_int
            }) as libc::c_uint;
            tmp2
                .wrapping_add(
                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh95 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh95 as libc::c_int
                    }) as libc::c_uint) << 8 as libc::c_int,
                ) as libc::c_ulong != crc & 0xffff as libc::c_int as libc::c_ulong
        }
    {
        return -(6 as libc::c_int);
    }
    g.form = 0 as libc::c_int;
    return if g.in_eof != 0 { -(3 as libc::c_int) } else { method as libc::c_int };
}
unsafe extern "C" fn more_zip_entries() -> libc::c_int {
    let mut sig: libc::c_ulong = 0;
    let mut ret: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut first: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    let central: [libc::c_uchar; 4] = [
        0x50 as libc::c_int as libc::c_uchar,
        0x4b as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
    ];
    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        0 as libc::c_int
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        g.in_left;
        let fresh96 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        *fresh96 as libc::c_int
    }) as libc::c_uint;
    tmp4 = tmp2
        .wrapping_add(
            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh97 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh97 as libc::c_int
            }) as libc::c_uint) << 8 as libc::c_int,
        ) as libc::c_ulong;
    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        0 as libc::c_int
    } else {
        g.in_left = (g.in_left).wrapping_sub(1);
        g.in_left;
        let fresh98 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        *fresh98 as libc::c_int
    }) as libc::c_uint;
    sig = tmp4
        .wrapping_add(
            (tmp2
                .wrapping_add(
                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh99 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh99 as libc::c_int
                    }) as libc::c_uint) << 8 as libc::c_int,
                ) as libc::c_ulong) << 16 as libc::c_int,
        );
    ret = (g.in_eof == 0 && sig == 0x4034b50 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    if g.list == 0 || g.verbosity < 2 as libc::c_int {
        return ret;
    }
    n = if sig == 0x2014b50 as libc::c_int as libc::c_ulong {
        4 as libc::c_int
    } else {
        0 as libc::c_int
    };
    loop {
        if g.in_left == 0 as libc::c_int as libc::c_ulong
            && load() == 0 as libc::c_int as libc::c_ulong
        {
            return ret;
        }
        if n == 0 as libc::c_int {
            first = memchr(
                g.in_next as *const libc::c_void,
                central[0 as libc::c_int as usize] as libc::c_int,
                g.in_left,
            ) as *mut libc::c_uchar;
            if first.is_null() {
                g.in_left = 0 as libc::c_int as size_t;
            } else {
                n += 1;
                n;
                g
                    .in_left = (g.in_left as libc::c_ulong)
                    .wrapping_sub(
                        (first.offset_from(g.in_next) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                    ) as size_t as size_t;
                g.in_next = first.offset(1 as libc::c_int as isize);
            }
        } else if n < 4 as libc::c_int {
            if *(g.in_next).offset(0 as libc::c_int as isize) as libc::c_int
                == central[n as usize] as libc::c_int
            {
                n += 1;
                n;
                g.in_next = (g.in_next).offset(1);
                g.in_next;
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
            } else {
                n = 0 as libc::c_int;
            }
        } else {
            let mut head: [libc::c_uchar; 42] = [0; 42];
            let mut need: size_t = 42 as libc::c_int as size_t;
            let mut part: size_t = 0 as libc::c_int as size_t;
            let mut len: size_t = 0;
            let mut i: size_t = 0;
            if need > g.in_left {
                part = g.in_left;
                memcpy(
                    head
                        .as_mut_ptr()
                        .offset(42 as libc::c_int as isize)
                        .offset(-(need as isize)) as *mut libc::c_void,
                    g.in_next as *const libc::c_void,
                    part,
                );
                need = (need as libc::c_ulong).wrapping_sub(part) as size_t as size_t;
                g.in_left = 0 as libc::c_int as size_t;
                if load() == 0 as libc::c_int as libc::c_ulong {
                    return ret;
                }
            }
            memcpy(
                head
                    .as_mut_ptr()
                    .offset(42 as libc::c_int as isize)
                    .offset(-(need as isize)) as *mut libc::c_void,
                g.in_next as *const libc::c_void,
                need,
            );
            if ((*head
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*head
                        .as_mut_ptr()
                        .offset(12 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                ) as libc::c_ulong)
                .wrapping_add(
                    ((*head
                        .as_mut_ptr()
                        .offset(12 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*head
                                .as_mut_ptr()
                                .offset(12 as libc::c_int as isize)
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        ) as libc::c_ulong) << 16 as libc::c_int,
                ) == g.out_check
                && ((*head
                    .as_mut_ptr()
                    .offset(38 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*head
                            .as_mut_ptr()
                            .offset(38 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    ) as libc::c_ulong)
                    .wrapping_add(
                        ((*head
                            .as_mut_ptr()
                            .offset(38 as libc::c_int as isize)
                            .offset(2 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*head
                                    .as_mut_ptr()
                                    .offset(38 as libc::c_int as isize)
                                    .offset(2 as libc::c_int as isize)
                                    .offset(1 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    ) == 0 as libc::c_int as libc::c_ulong
            {
                g.in_next = (g.in_next).offset(need as isize);
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(need) as size_t
                    as size_t;
                len = (*head
                    .as_mut_ptr()
                    .offset(28 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*head
                            .as_mut_ptr()
                            .offset(28 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    ) as size_t;
                if len == 0 as libc::c_int as libc::c_ulong {
                    return ret;
                }
                let mut togo: size_t = ((*head
                    .as_mut_ptr()
                    .offset(24 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*head
                            .as_mut_ptr()
                            .offset(24 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    ) as libc::c_ulong)
                    .wrapping_add(
                        (*head
                            .as_mut_ptr()
                            .offset(26 as libc::c_int as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*head
                                    .as_mut_ptr()
                                    .offset(26 as libc::c_int as isize)
                                    .offset(1 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            ) as libc::c_ulong,
                    );
                while togo > g.in_left {
                    togo = (togo as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                        as size_t;
                    if load() == 0 as libc::c_int as libc::c_ulong {
                        return -(3 as libc::c_int);
                    }
                }
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(togo) as size_t
                    as size_t;
                g.in_next = (g.in_next).offset(togo as isize);
                need = len;
                g
                    .hcomm = alloc(
                    0 as *mut libc::c_void,
                    len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                while need > g.in_left {
                    memcpy(
                        (g.hcomm).offset(len as isize).offset(-(need as isize))
                            as *mut libc::c_void,
                        g.in_next as *const libc::c_void,
                        g.in_left,
                    );
                    need = (need as libc::c_ulong).wrapping_sub(g.in_left) as size_t
                        as size_t;
                    g.in_left = 0 as libc::c_int as size_t;
                    if load() == 0 as libc::c_int as libc::c_ulong {
                        if !(g.hcomm).is_null() {
                            free(g.hcomm as *mut libc::c_void);
                            g.hcomm = 0 as *mut libc::c_char;
                        }
                        return ret;
                    }
                }
                memcpy(
                    (g.hcomm).offset(len as isize).offset(-(need as isize))
                        as *mut libc::c_void,
                    g.in_next as *const libc::c_void,
                    need,
                );
                g.in_next = (g.in_next).offset(need as isize);
                g
                    .in_left = (g.in_left as libc::c_ulong).wrapping_sub(need) as size_t
                    as size_t;
                i = 0 as libc::c_int as size_t;
                while i < len {
                    if *(g.hcomm).offset(i as isize) as libc::c_int == 0 as libc::c_int {
                        *(g.hcomm).offset(i as isize) = ' ' as i32 as libc::c_char;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                *(g.hcomm).offset(len as isize) = 0 as libc::c_int as libc::c_char;
                return ret;
            } else {
                if part != 0 {
                    memmove(
                        (g.in_next).offset(part as isize) as *mut libc::c_void,
                        g.in_next as *const libc::c_void,
                        g.in_left,
                    );
                    memcpy(
                        g.in_next as *mut libc::c_void,
                        head.as_mut_ptr() as *const libc::c_void,
                        part,
                    );
                    g
                        .in_left = (g.in_left as libc::c_ulong).wrapping_add(part)
                        as size_t as size_t;
                }
                n = 0 as libc::c_int;
            }
        }
    };
}
unsafe extern "C" fn compressed_suffix(mut nm: *mut libc::c_char) -> size_t {
    let mut len: size_t = 0;
    len = strlen(nm);
    if len > 4 as libc::c_int as libc::c_ulong {
        nm = nm.offset(len.wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize);
        len = 4 as libc::c_int as size_t;
        if strcmp(nm, b".zip\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(nm, b".ZIP\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(nm, b".tgz\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            return 4 as libc::c_int as size_t;
        }
    }
    if len > 3 as libc::c_int as libc::c_ulong {
        nm = nm.offset(len.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize);
        len = 3 as libc::c_int as size_t;
        if strcmp(nm, b".gz\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(nm, b"-gz\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(nm, b".zz\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(nm, b"-zz\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            return 3 as libc::c_int as size_t;
        }
    }
    if len > 2 as libc::c_int as libc::c_ulong {
        nm = nm.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
        if strcmp(nm, b".z\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(nm, b"-z\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(nm, b"_z\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(nm, b".Z\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            return 2 as libc::c_int as size_t;
        }
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn show_info(
    mut method: libc::c_int,
    mut check: libc::c_ulong,
    mut len: length_t,
    mut cont: libc::c_int,
) {
    let mut max: size_t = 0;
    let mut n: size_t = 0;
    let mut now: time_t = 0;
    let mut mod_0: [libc::c_char; 26] = [0; 26];
    let mut tag: [libc::c_char; 49] = [0; 49];
    max = (if g.verbosity > 1 as libc::c_int {
        16 as libc::c_int
    } else {
        48 as libc::c_int
    }) as size_t;
    memset(
        tag.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        max.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if cont != 0 {
        strncpy(
            tag.as_mut_ptr(),
            b"<...>\0" as *const u8 as *const libc::c_char,
            max.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else if (g.hname).is_null() {
        n = (strlen(g.inf)).wrapping_sub(compressed_suffix(g.inf));
        memcpy(
            tag.as_mut_ptr() as *mut libc::c_void,
            g.inf as *const libc::c_void,
            if n > max.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                max.wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                n
            },
        );
        if strcmp(
            (g.inf).offset(n as isize),
            b".tgz\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int && n < max.wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            strncpy(
                tag.as_mut_ptr().offset(n as isize),
                b".tar\0" as *const u8 as *const libc::c_char,
                max.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_sub(n),
            );
        }
    } else {
        strncpy(
            tag.as_mut_ptr(),
            g.hname,
            max.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    if tag[max as usize] != 0 {
        strcpy(
            tag.as_mut_ptr().offset(max as isize).offset(-(3 as libc::c_int as isize)),
            b"...\0" as *const u8 as *const libc::c_char,
        );
    }
    if g.stamp != 0 && cont == 0 {
        strcpy(mod_0.as_mut_ptr(), ctime(&mut g.stamp));
        now = time(0 as *mut time_t);
        if strcmp(
            mod_0.as_mut_ptr().offset(20 as libc::c_int as isize),
            (ctime(&mut now)).offset(20 as libc::c_int as isize),
        ) != 0 as libc::c_int
        {
            strcpy(
                mod_0.as_mut_ptr().offset(11 as libc::c_int as isize),
                mod_0.as_mut_ptr().offset(19 as libc::c_int as isize),
            );
        }
    } else {
        strcpy(
            mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
            b"------ -----\0" as *const u8 as *const libc::c_char,
        );
    }
    mod_0[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if g.first != 0 {
        if g.verbosity > 1 as libc::c_int {
            fputs(
                b"method    check    timestamp    \0" as *const u8
                    as *const libc::c_char,
                stdout,
            );
        }
        if g.verbosity > 0 as libc::c_int {
            puts(
                b"compressed   original reduced  name\0" as *const u8
                    as *const libc::c_char,
            );
        }
        g.first = 0 as libc::c_int;
    }
    if g.verbosity > 1 as libc::c_int {
        if g.form == 3 as libc::c_int && g.decode == 0 {
            printf(
                b"zip%3d  --------  %s  \0" as *const u8 as *const libc::c_char,
                method,
                mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
            );
        } else if g.form > 1 as libc::c_int {
            printf(
                b"zip%3d  %08lx  %s  \0" as *const u8 as *const libc::c_char,
                method,
                check,
                mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
            );
        } else if g.form == 1 as libc::c_int {
            printf(
                b"zlib%2d  %08lx  %s  \0" as *const u8 as *const libc::c_char,
                method,
                check,
                mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
            );
        } else if method == 257 as libc::c_int {
            printf(
                b"lzw     --------  %s  \0" as *const u8 as *const libc::c_char,
                mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
            );
        } else {
            printf(
                b"gzip%2d  %08lx  %s  \0" as *const u8 as *const libc::c_char,
                method,
                check,
                mod_0.as_mut_ptr().offset(4 as libc::c_int as isize),
            );
        }
    }
    if g.verbosity > 0 as libc::c_int {
        let mut red: libc::c_double = 100.0f64
            * (len as libc::c_double - g.in_tot as libc::c_double)
            / len as libc::c_double;
        if g.form == 3 as libc::c_int && g.decode == 0
            || method == 8 as libc::c_int
                && g.in_tot
                    > len
                        .wrapping_add(len >> 10 as libc::c_int)
                        .wrapping_add(12 as libc::c_int as libc::c_ulong)
            || method == 257 as libc::c_int
                && g.in_tot
                    > len
                        .wrapping_add(len >> 1 as libc::c_int)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        {
            printf(
                b"%10ju %10ju?  unk    %s\n\0" as *const u8 as *const libc::c_char,
                g.in_tot,
                len,
                tag.as_mut_ptr(),
            );
        } else {
            printf(
                b"%10ju %10ju %6.1f%%  %s\n\0" as *const u8 as *const libc::c_char,
                g.in_tot,
                len,
                red,
                tag.as_mut_ptr(),
            );
        }
    }
    if g.verbosity > 1 as libc::c_int && !(g.hcomm).is_null() {
        puts(g.hcomm);
    }
}
unsafe extern "C" fn list_info() {
    let mut method: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut at: off_t = 0;
    let mut tail: [libc::c_uchar; 8] = [0; 8];
    let mut check: libc::c_ulong = 0;
    let mut len: length_t = 0;
    in_init();
    method = get_header(1 as libc::c_int);
    if method < 0 as libc::c_int {
        complain(
            (if method == -(6 as libc::c_int) {
                b"skipping: %s corrupt: header crc error\0" as *const u8
                    as *const libc::c_char
            } else if method == -(1 as libc::c_int) {
                b"skipping: %s empty\0" as *const u8 as *const libc::c_char
            } else {
                b"skipping: %s unrecognized format\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            g.inf,
        );
        return;
    }
    load_wait();
    if g.form > 1 as libc::c_int {
        more_zip_entries();
        g.in_tot = g.zip_clen;
        show_info(method, g.zip_crc, g.zip_ulen, 0 as libc::c_int);
        return;
    }
    if g.form == 1 as libc::c_int {
        at = lseek(g.ind, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
        if at == -(1 as libc::c_int) as libc::c_long {
            check = 0 as libc::c_int as libc::c_ulong;
            loop {
                len = if g.in_left < 4 as libc::c_int as libc::c_ulong {
                    g.in_left
                } else {
                    4 as libc::c_int as libc::c_ulong
                };
                g.in_next = (g.in_next).offset((g.in_left).wrapping_sub(len) as isize);
                loop {
                    let fresh100 = len;
                    len = len.wrapping_sub(1);
                    if !(fresh100 != 0) {
                        break;
                    }
                    let fresh101 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    check = (check << 8 as libc::c_int)
                        .wrapping_add(*fresh101 as libc::c_ulong);
                }
                if !(load() != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
            check &= 0xffffffff as libc::c_uint as libc::c_ulong;
        } else {
            g.in_tot = at as length_t;
            lseek(g.ind, -(4 as libc::c_int) as __off64_t, 2 as libc::c_int);
            readn(g.ind, tail.as_mut_ptr(), 4 as libc::c_int as size_t);
            check = ((((tail[0 as libc::c_int as usize] as libc::c_uint)
                << 8 as libc::c_int)
                .wrapping_add(tail[1 as libc::c_int as usize] as libc::c_uint)
                as libc::c_ulong) << 16 as libc::c_int)
                .wrapping_add(
                    ((*tail
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int)
                        .wrapping_add(
                            *tail
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(1 as libc::c_int as isize) as libc::c_uint,
                        ) as libc::c_ulong,
                );
        }
        g
            .in_tot = (g.in_tot as libc::c_ulong)
            .wrapping_sub(6 as libc::c_int as libc::c_ulong) as length_t as length_t;
        show_info(method, check, 0 as libc::c_int as length_t, 0 as libc::c_int);
        return;
    }
    if method == 257 as libc::c_int {
        at = lseek(g.ind, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
        if at == -(1 as libc::c_int) as libc::c_long {
            while load() != 0 as libc::c_int as libc::c_ulong {}
        } else {
            g.in_tot = at as length_t;
        }
        g
            .in_tot = (g.in_tot as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as length_t as length_t;
        show_info(
            method,
            0 as libc::c_int as libc::c_ulong,
            0 as libc::c_int as length_t,
            0 as libc::c_int,
        );
        return;
    }
    if g.in_short != 0 {
        if g.in_left < 8 as libc::c_int as libc::c_ulong {
            complain(
                b"skipping: %s not a valid gzip file\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                g.inf,
            );
            return;
        }
        g.in_tot = (g.in_left).wrapping_sub(8 as libc::c_int as libc::c_ulong);
        memcpy(
            tail.as_mut_ptr() as *mut libc::c_void,
            (g.in_next)
                .offset(
                    (g.in_left).wrapping_sub(8 as libc::c_int as libc::c_ulong) as isize,
                ) as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
    } else {
        at = lseek(g.ind, -(8 as libc::c_int) as __off64_t, 2 as libc::c_int);
        if at != -(1 as libc::c_int) as libc::c_long {
            g.in_tot = (at as length_t).wrapping_sub(g.in_tot).wrapping_add(g.in_left);
            readn(g.ind, tail.as_mut_ptr(), 8 as libc::c_int as size_t);
        } else {
            len = (g.in_tot).wrapping_sub(g.in_left);
            loop {
                n = if g.in_left < 8 as libc::c_int as libc::c_ulong {
                    g.in_left
                } else {
                    8 as libc::c_int as libc::c_ulong
                };
                memcpy(
                    tail.as_mut_ptr() as *mut libc::c_void,
                    (g.in_next).offset((g.in_left).wrapping_sub(n) as isize)
                        as *const libc::c_void,
                    n,
                );
                load();
                if !(g.in_left == 32768 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
            if g.in_left < 8 as libc::c_int as libc::c_ulong {
                if n.wrapping_add(g.in_left) < 8 as libc::c_int as libc::c_ulong {
                    complain(
                        b"skipping: %s not a valid gzip file\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                    );
                    return;
                }
                if g.in_left != 0 {
                    if n.wrapping_add(g.in_left) > 8 as libc::c_int as libc::c_ulong {
                        memcpy(
                            tail.as_mut_ptr() as *mut libc::c_void,
                            tail
                                .as_mut_ptr()
                                .offset(n as isize)
                                .offset(
                                    -((8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(g.in_left) as isize),
                                ) as *const libc::c_void,
                            (8 as libc::c_int as libc::c_ulong).wrapping_sub(g.in_left),
                        );
                    }
                    memcpy(
                        tail
                            .as_mut_ptr()
                            .offset(8 as libc::c_int as isize)
                            .offset(-(g.in_left as isize)) as *mut libc::c_void,
                        g.in_next as *const libc::c_void,
                        g.in_left,
                    );
                }
            } else {
                memcpy(
                    tail.as_mut_ptr() as *mut libc::c_void,
                    (g.in_next)
                        .offset(
                            (g.in_left).wrapping_sub(8 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as *const libc::c_void,
                    8 as libc::c_int as libc::c_ulong,
                );
            }
            g
                .in_tot = (g.in_tot as libc::c_ulong)
                .wrapping_sub(len.wrapping_add(8 as libc::c_int as libc::c_ulong))
                as length_t as length_t;
        }
    }
    if g.in_tot < 2 as libc::c_int as libc::c_ulong {
        complain(
            b"skipping: %s not a valid gzip file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
        );
        return;
    }
    check = ((tail[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(
            (tail[1 as libc::c_int as usize] as libc::c_uint) << 8 as libc::c_int,
        ) as libc::c_ulong)
        .wrapping_add(
            ((*tail
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*tail
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                ) as libc::c_ulong) << 16 as libc::c_int,
        );
    len = ((*tail
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_add(
            (*tail
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int,
        ) as libc::c_ulong)
        .wrapping_add(
            ((*tail
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*tail
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                ) as libc::c_ulong) << 16 as libc::c_int,
        );
    show_info(method, check, len, 0 as libc::c_int);
}
unsafe extern "C" fn cat() {
    if g.magic1 != -(1 as libc::c_int) {
        let mut buf: [libc::c_uchar; 1] = [g.magic1 as libc::c_uchar];
        g
            .out_tot = (g.out_tot as libc::c_ulong)
            .wrapping_add(
                writen(
                    g.outd,
                    buf.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as size_t,
                ),
            ) as length_t as length_t;
    }
    while g.in_left != 0 {
        g
            .out_tot = (g.out_tot as libc::c_ulong)
            .wrapping_add(writen(g.outd, g.in_next as *const libc::c_void, g.in_left))
            as length_t as length_t;
        g.in_left = 0 as libc::c_int as size_t;
        load();
    }
}
unsafe extern "C" fn inb(
    mut desc: *mut libc::c_void,
    mut buf: *mut *mut libc::c_uchar,
) -> libc::c_uint {
    if g.in_left == 0 as libc::c_int as libc::c_ulong {
        load();
    }
    *buf = g.in_next;
    let mut len: libc::c_uint = if g.in_left
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
    {
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
    } else {
        g.in_left as libc::c_uint
    };
    g.in_next = (g.in_next).offset(len as isize);
    g
        .in_left = (g.in_left as libc::c_ulong).wrapping_sub(len as libc::c_ulong)
        as size_t as size_t;
    return len;
}
static mut out_buf: [libc::c_uchar; 32768] = [0; 32768];
static mut out_copy: [libc::c_uchar; 32768] = [0; 32768];
static mut out_len: size_t = 0;
static mut outb_write_more: *mut lock = 0 as *const lock as *mut lock;
static mut outb_check_more: *mut lock = 0 as *const lock as *mut lock;
unsafe extern "C" fn outb_write(mut dummy: *mut libc::c_void) {
    let mut len: size_t = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_char;
    try_setup_();
    try_this_.next = pthread_getspecific(try_key_) as *mut try_t_;
    if pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    ) == 0 as libc::c_int
        && !(b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3350 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"void outb_write(void *)\0"))
                .as_ptr(),
        );
    }
    'c_22702: {
        if pthread_setspecific(
            try_key_,
            &mut try_this_ as *mut try_t_ as *const libc::c_void,
        ) == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3350 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void outb_write(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if _setjmp((try_this_.env).as_mut_ptr()) == 0 as libc::c_int {
        loop {
            possess_(
                outb_write_more,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3352 as libc::c_int as libc::c_long,
            );
            wait_for_(
                outb_write_more,
                TO_BE,
                1 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3353 as libc::c_int as libc::c_long,
            );
            len = out_len;
            if len != 0 && g.decode == 1 as libc::c_int {
                writen(g.outd, out_copy.as_mut_ptr() as *const libc::c_void, len);
            }
            twist_(
                outb_write_more,
                TO,
                0 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3358 as libc::c_int as libc::c_long,
            );
            if !(len != 0) {
                break;
            }
        }
    }
    if try_pushed_ != 0 {
        if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
            == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3361 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void outb_write(void *)\0"))
                    .as_ptr(),
            );
        }
        'c_22559: {
            if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3361 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"void outb_write(void *)\0"))
                        .as_ptr(),
                );
            }
        };
        ::std::ptr::write_volatile(
            &mut try_pushed_ as *mut libc::c_int,
            0 as libc::c_int,
        );
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn outb_check(mut dummy: *mut libc::c_void) {
    let mut len: size_t = 0;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_char;
    try_setup_();
    try_this_.next = pthread_getspecific(try_key_) as *mut try_t_;
    if pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    ) == 0 as libc::c_int
        && !(b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3375 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"void outb_check(void *)\0"))
                .as_ptr(),
        );
    }
    'c_23055: {
        if pthread_setspecific(
            try_key_,
            &mut try_this_ as *mut try_t_ as *const libc::c_void,
        ) == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3375 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void outb_check(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if _setjmp((try_this_.env).as_mut_ptr()) == 0 as libc::c_int {
        loop {
            possess_(
                outb_check_more,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3377 as libc::c_int as libc::c_long,
            );
            wait_for_(
                outb_check_more,
                TO_BE,
                1 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3378 as libc::c_int as libc::c_long,
            );
            len = out_len;
            g
                .out_check = if g.form == 1 as libc::c_int {
                adler32z(g.out_check, out_copy.as_mut_ptr(), len)
            } else {
                crc32z(g.out_check, out_copy.as_mut_ptr(), len)
            };
            twist_(
                outb_check_more,
                TO,
                0 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3382 as libc::c_int as libc::c_long,
            );
            if !(len != 0) {
                break;
            }
        }
    }
    if try_pushed_ != 0 {
        if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
            == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3385 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void outb_check(void *)\0"))
                    .as_ptr(),
            );
        }
        'c_22901: {
            if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    3385 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"void outb_check(void *)\0"))
                        .as_ptr(),
                );
            }
        };
        ::std::ptr::write_volatile(
            &mut try_pushed_ as *mut libc::c_int,
            0 as libc::c_int,
        );
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
}
unsafe extern "C" fn outb(
    mut desc: *mut libc::c_void,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_int {
    static mut wr: *mut thread = 0 as *const thread as *mut thread;
    static mut ch: *mut thread = 0 as *const thread as *mut thread;
    if g.procs > 1 as libc::c_int {
        if outb_write_more.is_null() {
            outb_write_more = new_lock_(
                0 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3405 as libc::c_int as libc::c_long,
            );
            outb_check_more = new_lock_(
                0 as libc::c_int as libc::c_long,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3406 as libc::c_int as libc::c_long,
            );
            wr = launch_(
                Some(outb_write as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3407 as libc::c_int as libc::c_long,
            );
            ch = launch_(
                Some(outb_check as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3408 as libc::c_int as libc::c_long,
            );
        }
        possess_(
            outb_check_more,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3412 as libc::c_int as libc::c_long,
        );
        wait_for_(
            outb_check_more,
            TO_BE,
            0 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3413 as libc::c_int as libc::c_long,
        );
        possess_(
            outb_write_more,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3414 as libc::c_int as libc::c_long,
        );
        wait_for_(
            outb_write_more,
            TO_BE,
            0 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3415 as libc::c_int as libc::c_long,
        );
        out_len = len as size_t;
        g
            .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(len as libc::c_ulong)
            as length_t as length_t;
        memcpy(
            out_copy.as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            len as libc::c_ulong,
        );
        twist_(
            outb_write_more,
            TO,
            1 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3421 as libc::c_int as libc::c_long,
        );
        twist_(
            outb_check_more,
            TO,
            1 as libc::c_int as libc::c_long,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            3422 as libc::c_int as libc::c_long,
        );
        if len == 0 as libc::c_int as libc::c_uint && !outb_write_more.is_null() {
            join_(
                ch,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3427 as libc::c_int as libc::c_long,
            );
            join_(
                wr,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3428 as libc::c_int as libc::c_long,
            );
            free_lock_(
                outb_check_more,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3429 as libc::c_int as libc::c_long,
            );
            free_lock_(
                outb_write_more,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                3430 as libc::c_int as libc::c_long,
            );
            outb_write_more = 0 as *mut lock;
        }
        return 0 as libc::c_int;
    }
    if len != 0 {
        if g.decode == 1 as libc::c_int {
            writen(g.outd, buf as *const libc::c_void, len as size_t);
        }
        g
            .out_check = if g.form == 1 as libc::c_int {
            adler32z(g.out_check, buf, len as size_t)
        } else {
            crc32z(g.out_check, buf, len as size_t)
        };
        g
            .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(len as libc::c_ulong)
            as length_t as length_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn infchk() {
    let mut ret: libc::c_int = 0;
    let mut cont: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    let mut check: libc::c_ulong = 0;
    let mut len: libc::c_ulong = 0;
    let mut ktot: libc::c_ulong = 0;
    let mut strm: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut tmp2: libc::c_uint = 0;
    let mut tmp4: libc::c_ulong = 0;
    let mut clen: length_t = 0;
    let mut ctot: length_t = 0;
    let mut utot: length_t = 0;
    utot = 0 as libc::c_int as length_t;
    ctot = utot;
    ktot = if g.form == 1 as libc::c_int {
        adler32z(
            0 as libc::c_long as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        )
    } else {
        crc32z(
            0 as libc::c_long as libc::c_ulong,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        )
    };
    more = 0 as libc::c_int;
    cont = more;
    loop {
        g.in_tot = g.in_left;
        g.out_tot = 0 as libc::c_int as length_t;
        g
            .out_check = if g.form == 1 as libc::c_int {
            adler32z(
                0 as libc::c_long as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            )
        } else {
            crc32z(
                0 as libc::c_long as libc::c_ulong,
                0 as *const libc::c_uchar,
                0 as libc::c_int as size_t,
            )
        };
        strm.zalloc = None;
        strm.zfree = None;
        strm.opaque = 0 as voidpf;
        ret = inflateBackInit_(
            &mut strm,
            15 as libc::c_int,
            out_buf.as_mut_ptr(),
            b"1.2.11\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        );
        if ret == -(4 as libc::c_int) {
            try_throw_(
                12 as libc::c_int,
                b"not enough memory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        if ret != 0 as libc::c_int {
            try_throw_(
                22 as libc::c_int,
                b"internal error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        strm.avail_in = 0 as libc::c_int as uInt;
        strm.next_in = 0 as *mut Bytef;
        ret = inflateBack(
            &mut strm,
            Some(
                inb
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_uchar,
                    ) -> libc::c_uint,
            ),
            0 as *mut libc::c_void,
            Some(
                outb
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_uchar,
                        libc::c_uint,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        inflateBackEnd(&mut strm);
        if ret == -(3 as libc::c_int) {
            try_throw_(
                33 as libc::c_int,
                b"%s: corrupted -- invalid deflate data (%s)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                g.inf,
                strm.msg,
                0 as *mut libc::c_void,
            );
        }
        if ret == -(5 as libc::c_int) {
            try_throw_(
                33 as libc::c_int,
                b"%s: corrupted -- incomplete deflate data\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                g.inf,
                0 as *mut libc::c_void,
            );
        }
        if ret != 1 as libc::c_int {
            try_throw_(
                22 as libc::c_int,
                b"internal error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        g
            .in_left = (g.in_left as libc::c_ulong)
            .wrapping_add(strm.avail_in as libc::c_ulong) as size_t as size_t;
        g.in_next = strm.next_in;
        outb(
            0 as *mut libc::c_void,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as libc::c_uint,
        );
        clen = (g.in_tot).wrapping_sub(g.in_left);
        if g.form > 1 as libc::c_int {
            if g.form == 3 as libc::c_int {
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh102 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh102 as libc::c_int
                }) as libc::c_uint;
                tmp4 = tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh103 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh103 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong;
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh104 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh104 as libc::c_int
                }) as libc::c_uint;
                g
                    .zip_crc = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add(
                                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                    && (g.in_eof != 0
                                        || load() == 0 as libc::c_int as libc::c_ulong)
                                {
                                    0 as libc::c_int
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    g.in_left;
                                    let fresh105 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    *fresh105 as libc::c_int
                                }) as libc::c_uint) << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    );
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh106 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh106 as libc::c_int
                }) as libc::c_uint;
                tmp4 = tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh107 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh107 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong;
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh108 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh108 as libc::c_int
                }) as libc::c_uint;
                g
                    .zip_clen = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add(
                                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                    && (g.in_eof != 0
                                        || load() == 0 as libc::c_int as libc::c_ulong)
                                {
                                    0 as libc::c_int
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    g.in_left;
                                    let fresh109 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    *fresh109 as libc::c_int
                                }) as libc::c_uint) << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    );
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh110 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh110 as libc::c_int
                }) as libc::c_uint;
                tmp4 = tmp2
                    .wrapping_add(
                        ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                            && (g.in_eof != 0
                                || load() == 0 as libc::c_int as libc::c_ulong)
                        {
                            0 as libc::c_int
                        } else {
                            g.in_left = (g.in_left).wrapping_sub(1);
                            g.in_left;
                            let fresh111 = g.in_next;
                            g.in_next = (g.in_next).offset(1);
                            *fresh111 as libc::c_int
                        }) as libc::c_uint) << 8 as libc::c_int,
                    ) as libc::c_ulong;
                tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int
                } else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh112 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh112 as libc::c_int
                }) as libc::c_uint;
                g
                    .zip_ulen = tmp4
                    .wrapping_add(
                        (tmp2
                            .wrapping_add(
                                ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                    && (g.in_eof != 0
                                        || load() == 0 as libc::c_int as libc::c_ulong)
                                {
                                    0 as libc::c_int
                                } else {
                                    g.in_left = (g.in_left).wrapping_sub(1);
                                    g.in_left;
                                    let fresh113 = g.in_next;
                                    g.in_next = (g.in_next).offset(1);
                                    *fresh113 as libc::c_int
                                }) as libc::c_uint) << 8 as libc::c_int,
                            ) as libc::c_ulong) << 16 as libc::c_int,
                    );
                if g.zip_crc == 0x8074b50 as libc::c_int as libc::c_ulong
                    && (g.out_check != 0x8074b50 as libc::c_int as libc::c_ulong
                        || g.zip_clen == 0x8074b50 as libc::c_int as libc::c_ulong
                            && (clen & 0xffffffff as libc::c_uint as libc::c_ulong
                                != 0x8074b50 as libc::c_int as libc::c_ulong
                                || g.zip_ulen == 0x8074b50 as libc::c_int as libc::c_ulong
                                    && (if g.zip64 != 0 {
                                        clen >> 32 as libc::c_int
                                    } else {
                                        g.out_tot
                                    }) != 0x8074b50 as libc::c_int as libc::c_ulong))
                {
                    g.zip_crc = g.zip_clen;
                    g.zip_clen = g.zip_ulen;
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh114 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh114 as libc::c_int
                    }) as libc::c_uint;
                    tmp4 = tmp2
                        .wrapping_add(
                            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                && (g.in_eof != 0
                                    || load() == 0 as libc::c_int as libc::c_ulong)
                            {
                                0 as libc::c_int
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                g.in_left;
                                let fresh115 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                *fresh115 as libc::c_int
                            }) as libc::c_uint) << 8 as libc::c_int,
                        ) as libc::c_ulong;
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh116 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh116 as libc::c_int
                    }) as libc::c_uint;
                    g
                        .zip_ulen = tmp4
                        .wrapping_add(
                            (tmp2
                                .wrapping_add(
                                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                        && (g.in_eof != 0
                                            || load() == 0 as libc::c_int as libc::c_ulong)
                                    {
                                        0 as libc::c_int
                                    } else {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        g.in_left;
                                        let fresh117 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                        *fresh117 as libc::c_int
                                    }) as libc::c_uint) << 8 as libc::c_int,
                                ) as libc::c_ulong) << 16 as libc::c_int,
                        );
                }
                if g.zip64 != 0 {
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh118 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh118 as libc::c_int
                    }) as libc::c_uint;
                    tmp4 = tmp2
                        .wrapping_add(
                            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                && (g.in_eof != 0
                                    || load() == 0 as libc::c_int as libc::c_ulong)
                            {
                                0 as libc::c_int
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                g.in_left;
                                let fresh119 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                *fresh119 as libc::c_int
                            }) as libc::c_uint) << 8 as libc::c_int,
                        ) as libc::c_ulong;
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh120 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh120 as libc::c_int
                    }) as libc::c_uint;
                    g
                        .zip_ulen = tmp4
                        .wrapping_add(
                            (tmp2
                                .wrapping_add(
                                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                        && (g.in_eof != 0
                                            || load() == 0 as libc::c_int as libc::c_ulong)
                                    {
                                        0 as libc::c_int
                                    } else {
                                        g.in_left = (g.in_left).wrapping_sub(1);
                                        g.in_left;
                                        let fresh121 = g.in_next;
                                        g.in_next = (g.in_next).offset(1);
                                        *fresh121 as libc::c_int
                                    }) as libc::c_uint) << 8 as libc::c_int,
                                ) as libc::c_ulong) << 16 as libc::c_int,
                        );
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh122 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh122 as libc::c_int
                    }) as libc::c_uint;
                    tmp4 = tmp2
                        .wrapping_add(
                            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                && (g.in_eof != 0
                                    || load() == 0 as libc::c_int as libc::c_ulong)
                            {
                                0 as libc::c_int
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                g.in_left;
                                let fresh123 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                *fresh123 as libc::c_int
                            }) as libc::c_uint) << 8 as libc::c_int,
                        ) as libc::c_ulong;
                    tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh124 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh124 as libc::c_int
                    }) as libc::c_uint;
                    if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {} else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh125 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh125;
                    };
                }
                if g.in_eof != 0 {
                    try_throw_(
                        33 as libc::c_int,
                        b"%s: corrupted entry -- missing trailer\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                        0 as *mut libc::c_void,
                    );
                }
            }
            check = g.zip_crc;
            if check != g.out_check {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted entry -- crc32 mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if g.zip_clen != clen & 0xffffffff as libc::c_uint as libc::c_ulong
                || g.zip_ulen != g.out_tot & 0xffffffff as libc::c_uint as libc::c_ulong
            {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted entry -- length mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            more = more_zip_entries();
        } else if g.form == 1 as libc::c_int {
            check = ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh126 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh126 as libc::c_int
            }) as libc::c_ulong) << 24 as libc::c_int;
            check = check
                .wrapping_add(
                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh127 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh127 as libc::c_int
                    }) as libc::c_ulong) << 16 as libc::c_int,
                );
            check = check
                .wrapping_add(
                    (((if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh128 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh128 as libc::c_int
                    }) as libc::c_uint) << 8 as libc::c_int) as libc::c_ulong,
                );
            check = check
                .wrapping_add(
                    (if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh129 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh129 as libc::c_int
                    }) as libc::c_ulong,
                );
            if g.in_eof != 0 {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- missing trailer\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if check != g.out_check {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- adler32 mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
        } else {
            tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh130 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh130 as libc::c_int
            }) as libc::c_uint;
            tmp4 = tmp2
                .wrapping_add(
                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh131 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh131 as libc::c_int
                    }) as libc::c_uint) << 8 as libc::c_int,
                ) as libc::c_ulong;
            tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh132 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh132 as libc::c_int
            }) as libc::c_uint;
            check = tmp4
                .wrapping_add(
                    (tmp2
                        .wrapping_add(
                            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                && (g.in_eof != 0
                                    || load() == 0 as libc::c_int as libc::c_ulong)
                            {
                                0 as libc::c_int
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                g.in_left;
                                let fresh133 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                *fresh133 as libc::c_int
                            }) as libc::c_uint) << 8 as libc::c_int,
                        ) as libc::c_ulong) << 16 as libc::c_int,
                );
            tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh134 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh134 as libc::c_int
            }) as libc::c_uint;
            tmp4 = tmp2
                .wrapping_add(
                    ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int
                    } else {
                        g.in_left = (g.in_left).wrapping_sub(1);
                        g.in_left;
                        let fresh135 = g.in_next;
                        g.in_next = (g.in_next).offset(1);
                        *fresh135 as libc::c_int
                    }) as libc::c_uint) << 8 as libc::c_int,
                ) as libc::c_ulong;
            tmp2 = (if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                g.in_left = (g.in_left).wrapping_sub(1);
                g.in_left;
                let fresh136 = g.in_next;
                g.in_next = (g.in_next).offset(1);
                *fresh136 as libc::c_int
            }) as libc::c_uint;
            len = tmp4
                .wrapping_add(
                    (tmp2
                        .wrapping_add(
                            ((if g.in_left == 0 as libc::c_int as libc::c_ulong
                                && (g.in_eof != 0
                                    || load() == 0 as libc::c_int as libc::c_ulong)
                            {
                                0 as libc::c_int
                            } else {
                                g.in_left = (g.in_left).wrapping_sub(1);
                                g.in_left;
                                let fresh137 = g.in_next;
                                g.in_next = (g.in_next).offset(1);
                                *fresh137 as libc::c_int
                            }) as libc::c_uint) << 8 as libc::c_int,
                        ) as libc::c_ulong) << 16 as libc::c_int,
                );
            if g.in_eof != 0 {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- missing trailer\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if check != g.out_check {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- crc32 mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if len != g.out_tot & 0xffffffff as libc::c_uint as libc::c_ulong {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: corrupted -- length mismatch\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
        }
        if g.list != 0 {
            ctot = (ctot as libc::c_ulong).wrapping_add(clen) as length_t as length_t;
            utot = (utot as libc::c_ulong).wrapping_add(g.out_tot) as length_t
                as length_t;
            ktot = if g.form == 1 as libc::c_int {
                adler32_comb(ktot, check, g.out_tot)
            } else {
                crc32_comb(ktot, check, g.out_tot)
            };
            g.in_tot = clen;
            show_info(8 as libc::c_int, check, g.out_tot, cont);
            cont = if cont != 0 { 2 as libc::c_int } else { 1 as libc::c_int };
        }
        if !(g.form == 0 as libc::c_int
            && {
                ret = get_header(0 as libc::c_int);
                ret == 8 as libc::c_int
            })
        {
            break;
        }
    }
    if cont > 1 as libc::c_int && g.verbosity > 0 as libc::c_int {
        if g.verbosity > 1 as libc::c_int {
            printf(
                b"        %08lx                \0" as *const u8 as *const libc::c_char,
                ktot,
            );
        }
        printf(
            b"%10ju %10ju %6.1f%%  (total)\n\0" as *const u8 as *const libc::c_char,
            ctot,
            utot,
            100.0f64 * (utot as libc::c_double - ctot as libc::c_double)
                / utot as libc::c_double,
        );
    }
    if g.form == 0 as libc::c_int && ret == -(2 as libc::c_int) && g.force != 0
        && g.pipeout != 0 && g.decode != 2 as libc::c_int && g.list == 0
    {
        cat();
    } else if more != 0 {
        complain(
            b"warning: %s: entries after the first were ignored\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            g.inf,
        );
        g.keep = 1 as libc::c_int;
    } else if g.verbosity > 1 as libc::c_int && g.form == 0 as libc::c_int
        && ret != -(1 as libc::c_int)
        || g.form == 1 as libc::c_int
            && {
                if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {} else {
                    g.in_left = (g.in_left).wrapping_sub(1);
                    g.in_left;
                    let fresh138 = g.in_next;
                    g.in_next = (g.in_next).offset(1);
                    *fresh138;
                };
                g.in_eof == 0
            }
    {
        complain(
            b"warning: %s: trailing junk was ignored\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            g.inf,
        );
    }
}
unsafe extern "C" fn unlzw() {
    let mut bits: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0;
    let mut buf: bits_t = 0;
    let mut left: libc::c_uint = 0;
    let mut mark: length_t = 0;
    let mut code: libc::c_uint = 0;
    let mut max: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut prev: libc::c_uint = 0;
    let mut final_0: libc::c_uint = 0;
    let mut stack: libc::c_uint = 0;
    let mut outcnt: libc::c_uint = 0;
    let mut prefix: [index_t; 65536] = [0; 65536];
    let mut suffix: [libc::c_uchar; 65536] = [0; 65536];
    let mut match_0: [libc::c_uchar; 65282] = [0; 65282];
    g.out_tot = 0 as libc::c_int as length_t;
    if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        try_throw_(
            33 as libc::c_int,
            b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    }
    g.in_left = (g.in_left).wrapping_sub(1);
    g.in_left;
    let fresh139 = g.in_next;
    g.in_next = (g.in_next).offset(1);
    flags = *fresh139 as libc::c_uint;
    if flags & 0x60 as libc::c_int as libc::c_uint != 0 {
        try_throw_(
            33 as libc::c_int,
            b"%s: unknown lzw flags set\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    }
    max = flags & 0x1f as libc::c_int as libc::c_uint;
    if max < 9 as libc::c_int as libc::c_uint || max > 16 as libc::c_int as libc::c_uint
    {
        try_throw_(
            33 as libc::c_int,
            b"%s: lzw bits out of range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    }
    if max == 9 as libc::c_int as libc::c_uint {
        max = 10 as libc::c_int as libc::c_uint;
    }
    flags &= 0x80 as libc::c_int as libc::c_uint;
    mark = (g.in_tot).wrapping_sub(g.in_left);
    bits = 9 as libc::c_int as libc::c_uint;
    mask = 0x1ff as libc::c_int as libc::c_uint;
    end = (if flags != 0 { 256 as libc::c_int } else { 255 as libc::c_int })
        as libc::c_uint;
    if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        return;
    }
    g.in_left = (g.in_left).wrapping_sub(1);
    g.in_left;
    let fresh140 = g.in_next;
    g.in_next = (g.in_next).offset(1);
    buf = *fresh140 as libc::c_uint as bits_t;
    if g.in_left == 0 as libc::c_int as libc::c_ulong
        && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
    {
        try_throw_(
            33 as libc::c_int,
            b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    }
    g.in_left = (g.in_left).wrapping_sub(1);
    g.in_left;
    let fresh141 = g.in_next;
    g.in_next = (g.in_next).offset(1);
    buf = (buf as libc::c_ulong)
        .wrapping_add(((*fresh141 as libc::c_uint) << 8 as libc::c_int) as libc::c_ulong)
        as bits_t as bits_t;
    prev = (buf & mask as libc::c_ulong) as libc::c_uint;
    final_0 = prev;
    buf >>= bits;
    left = (16 as libc::c_int as libc::c_uint).wrapping_sub(bits);
    if prev > 255 as libc::c_int as libc::c_uint {
        try_throw_(
            33 as libc::c_int,
            b"%s: invalid lzw code\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.inf,
            0 as *mut libc::c_void,
        );
    }
    out_buf[0 as libc::c_int as usize] = final_0 as libc::c_uchar;
    outcnt = 1 as libc::c_int as libc::c_uint;
    stack = 0 as libc::c_int as libc::c_uint;
    loop {
        if end >= mask && bits < max {
            let mut rem: libc::c_uint = (g.in_tot)
                .wrapping_sub(g.in_left)
                .wrapping_sub(mark)
                .wrapping_rem(bits as libc::c_ulong) as libc::c_uint;
            if rem != 0 {
                rem = bits.wrapping_sub(rem);
                if g.in_left == 0 as libc::c_int as libc::c_ulong
                    && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
                {
                    break;
                }
                while rem as libc::c_ulong > g.in_left {
                    rem = (rem as libc::c_ulong).wrapping_sub(g.in_left) as libc::c_uint
                        as libc::c_uint;
                    if load() == 0 as libc::c_int as libc::c_ulong {
                        try_throw_(
                            33 as libc::c_int,
                            b"%s: lzw premature end\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            g.inf,
                            0 as *mut libc::c_void,
                        );
                    }
                }
                g
                    .in_left = (g.in_left as libc::c_ulong)
                    .wrapping_sub(rem as libc::c_ulong) as size_t as size_t;
                g.in_next = (g.in_next).offset(rem as isize);
            }
            buf = 0 as libc::c_int as bits_t;
            left = 0 as libc::c_int as libc::c_uint;
            mark = (g.in_tot).wrapping_sub(g.in_left);
            bits = bits.wrapping_add(1);
            bits;
            mask <<= 1 as libc::c_int;
            mask = mask.wrapping_add(1);
            mask;
        }
        if g.in_left == 0 as libc::c_int as libc::c_ulong
            && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
        {
            break;
        }
        g.in_left = (g.in_left).wrapping_sub(1);
        g.in_left;
        let fresh142 = g.in_next;
        g.in_next = (g.in_next).offset(1);
        buf = (buf as libc::c_ulong)
            .wrapping_add((*fresh142 as libc::c_uint as bits_t) << left) as bits_t
            as bits_t;
        left = left.wrapping_add(8 as libc::c_int as libc::c_uint);
        if left < bits {
            if g.in_left == 0 as libc::c_int as libc::c_ulong
                && (g.in_eof != 0 || load() == 0 as libc::c_int as libc::c_ulong)
            {
                try_throw_(
                    33 as libc::c_int,
                    b"%s: lzw premature end\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            g.in_left = (g.in_left).wrapping_sub(1);
            g.in_left;
            let fresh143 = g.in_next;
            g.in_next = (g.in_next).offset(1);
            buf = (buf as libc::c_ulong)
                .wrapping_add((*fresh143 as libc::c_uint as bits_t) << left) as bits_t
                as bits_t;
            left = left.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        code = (buf & mask as libc::c_ulong) as libc::c_uint;
        buf >>= bits;
        left = left.wrapping_sub(bits);
        if code == 256 as libc::c_int as libc::c_uint && flags != 0 {
            let mut rem_0: libc::c_uint = (g.in_tot)
                .wrapping_sub(g.in_left)
                .wrapping_sub(mark)
                .wrapping_rem(bits as libc::c_ulong) as libc::c_uint;
            if rem_0 != 0 {
                rem_0 = bits.wrapping_sub(rem_0);
                while rem_0 as libc::c_ulong > g.in_left {
                    rem_0 = (rem_0 as libc::c_ulong).wrapping_sub(g.in_left)
                        as libc::c_uint as libc::c_uint;
                    if load() == 0 as libc::c_int as libc::c_ulong {
                        try_throw_(
                            33 as libc::c_int,
                            b"%s: lzw premature end\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            g.inf,
                            0 as *mut libc::c_void,
                        );
                    }
                }
                g
                    .in_left = (g.in_left as libc::c_ulong)
                    .wrapping_sub(rem_0 as libc::c_ulong) as size_t as size_t;
                g.in_next = (g.in_next).offset(rem_0 as isize);
            }
            buf = 0 as libc::c_int as bits_t;
            left = 0 as libc::c_int as libc::c_uint;
            mark = (g.in_tot).wrapping_sub(g.in_left);
            bits = 9 as libc::c_int as libc::c_uint;
            mask = 0x1ff as libc::c_int as libc::c_uint;
            end = 255 as libc::c_int as libc::c_uint;
        } else {
            let mut temp: libc::c_uint = code;
            if code > end {
                if code != end.wrapping_add(1 as libc::c_int as libc::c_uint)
                    || prev > end
                {
                    try_throw_(
                        33 as libc::c_int,
                        b"%s: invalid lzw code\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        g.inf,
                        0 as *mut libc::c_void,
                    );
                }
                let fresh144 = stack;
                stack = stack.wrapping_add(1);
                match_0[fresh144 as usize] = final_0 as libc::c_uchar;
                code = prev;
            }
            while code >= 256 as libc::c_int as libc::c_uint {
                let fresh145 = stack;
                stack = stack.wrapping_add(1);
                match_0[fresh145 as usize] = suffix[code as usize];
                code = prefix[code as usize] as libc::c_uint;
            }
            let fresh146 = stack;
            stack = stack.wrapping_add(1);
            match_0[fresh146 as usize] = code as libc::c_uchar;
            final_0 = code;
            if end < mask {
                end = end.wrapping_add(1);
                end;
                prefix[end as usize] = prev as index_t;
                suffix[end as usize] = final_0 as libc::c_uchar;
            }
            prev = temp;
            while stack > (32768 as libc::c_uint).wrapping_sub(outcnt) {
                while outcnt < 32768 as libc::c_uint {
                    stack = stack.wrapping_sub(1);
                    let fresh147 = outcnt;
                    outcnt = outcnt.wrapping_add(1);
                    out_buf[fresh147 as usize] = match_0[stack as usize];
                }
                g
                    .out_tot = (g.out_tot as libc::c_ulong)
                    .wrapping_add(outcnt as libc::c_ulong) as length_t as length_t;
                if g.decode == 1 as libc::c_int {
                    writen(
                        g.outd,
                        out_buf.as_mut_ptr() as *const libc::c_void,
                        outcnt as size_t,
                    );
                }
                outcnt = 0 as libc::c_int as libc::c_uint;
            }
            loop {
                stack = stack.wrapping_sub(1);
                let fresh148 = outcnt;
                outcnt = outcnt.wrapping_add(1);
                out_buf[fresh148 as usize] = match_0[stack as usize];
                if !(stack != 0) {
                    break;
                }
            }
        }
    }
    g
        .out_tot = (g.out_tot as libc::c_ulong).wrapping_add(outcnt as libc::c_ulong)
        as length_t as length_t;
    if outcnt != 0 && g.decode == 1 as libc::c_int {
        writen(g.outd, out_buf.as_mut_ptr() as *const libc::c_void, outcnt as size_t);
    }
}
unsafe extern "C" fn justname(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strrchr(path, '/' as i32);
    return if p.is_null() { path } else { p.offset(1 as libc::c_int as isize) };
}
unsafe extern "C" fn copymeta(
    mut from: *mut libc::c_char,
    mut to: *mut libc::c_char,
) -> libc::c_int {
    let mut st: stat = stat {
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
    let mut times: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    if stat(from, &mut st) != 0 as libc::c_int
        || st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            != 0o100000 as libc::c_int as libc::c_uint
    {
        return -(4 as libc::c_int);
    }
    let mut ret: libc::c_int = chmod(
        to,
        st.st_mode & 0o7777 as libc::c_int as libc::c_uint,
    );
    ret += chown(to, st.st_uid, st.st_gid);
    times[0 as libc::c_int as usize].tv_sec = st.st_atim.tv_sec;
    times[0 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    times[1 as libc::c_int as usize].tv_sec = st.st_mtim.tv_sec;
    times[1 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    ret += utimes(to, times.as_mut_ptr() as *const timeval);
    return ret;
}
unsafe extern "C" fn touch(mut path: *mut libc::c_char, mut t: time_t) {
    let mut times: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    times[0 as libc::c_int as usize].tv_sec = t;
    times[0 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    times[1 as libc::c_int as usize].tv_sec = t;
    times[1 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
    utimes(path, times.as_mut_ptr() as *const timeval);
}
unsafe extern "C" fn out_push() {
    if g.outd == -(1 as libc::c_int) {
        return;
    }
    let mut ret: libc::c_int = fsync(g.outd);
    if ret == -(1 as libc::c_int) {
        try_throw_(
            *__errno_location(),
            b"sync error on %s (%s)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            g.outf,
            strerror(*__errno_location()),
            0 as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn process(mut path: *mut libc::c_char) {
    let mut method: libc::c_int = -(1 as libc::c_int);
    let mut len: size_t = 0;
    let mut st: stat = stat {
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
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    static mut sufs: [*mut libc::c_char; 12] = [
        b".z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"_z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".gz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-gz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".zz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-zz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".zip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".ZIP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".tgz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    if path.is_null() {
        vstrcpy(
            &mut g.inf,
            &mut g.inz,
            0 as libc::c_int as size_t,
            b"<stdin>\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        );
        g.ind = 0 as libc::c_int;
        g.name = 0 as *mut libc::c_char;
        g
            .mtime = if g.headis & 2 as libc::c_int != 0 {
            if fstat(g.ind, &mut st) != 0 {
                time(0 as *mut time_t)
            } else {
                st.st_mtim.tv_sec
            }
        } else {
            0 as libc::c_int as libc::c_long
        };
        len = 0 as libc::c_int as size_t;
    } else {
        if path != g.inf {
            vstrcpy(
                &mut g.inf,
                &mut g.inz,
                0 as libc::c_int as size_t,
                path as *mut libc::c_void,
            );
        }
        len = strlen(g.inf);
        if lstat(g.inf, &mut st) != 0 {
            if *__errno_location() == 2 as libc::c_int && (g.list != 0 || g.decode != 0)
            {
                let mut sufx: *mut *mut libc::c_char = sufs.as_mut_ptr();
                while !(*sufx).is_null() {
                    let fresh149 = sufx;
                    sufx = sufx.offset(1);
                    vstrcpy(&mut g.inf, &mut g.inz, len, *fresh149 as *mut libc::c_void);
                    *__errno_location() = 0 as libc::c_int;
                    if !(lstat(g.inf, &mut st) != 0
                        && *__errno_location() == 2 as libc::c_int)
                    {
                        break;
                    }
                }
            }
            if *__errno_location() == 75 as libc::c_int
                || *__errno_location() == 27 as libc::c_int
            {
                try_throw_(
                    33 as libc::c_int,
                    b"%s too large -- not compiled with large file support\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                    0 as *mut libc::c_void,
                );
            }
            if *__errno_location() != 0 {
                *(g.inf).offset(len as isize) = 0 as libc::c_int as libc::c_char;
                complain(
                    b"skipping: %s does not exist\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.inf,
                );
                return;
            }
            len = strlen(g.inf);
        }
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            != 0o100000 as libc::c_int as libc::c_uint
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                != 0o10000 as libc::c_int as libc::c_uint
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                != 0o120000 as libc::c_int as libc::c_uint
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                != 0o40000 as libc::c_int as libc::c_uint
        {
            complain(
                b"skipping: %s is a special file or device\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                g.inf,
            );
            return;
        }
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint && g.force == 0 && g.pipeout == 0
        {
            complain(
                b"skipping: %s is a symbolic link\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
            );
            return;
        }
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint && g.recurse == 0
        {
            complain(
                b"skipping: %s is a directory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
            );
            return;
        }
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            let mut roll: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut size: size_t = 0 as libc::c_int as size_t;
            let mut off: size_t = 0 as libc::c_int as size_t;
            let mut base: size_t = 0;
            let mut here: *mut DIR = 0 as *mut DIR;
            let mut next: *mut dirent = 0 as *mut dirent;
            here = opendir(g.inf);
            if here.is_null() {
                return;
            }
            loop {
                next = readdir(here);
                if next.is_null() {
                    break;
                }
                if (*next).d_name[0 as libc::c_int as usize] as libc::c_int
                    == 0 as libc::c_int
                    || (*next).d_name[0 as libc::c_int as usize] as libc::c_int
                        == '.' as i32
                        && ((*next).d_name[1 as libc::c_int as usize] as libc::c_int
                            == 0 as libc::c_int
                            || (*next).d_name[1 as libc::c_int as usize] as libc::c_int
                                == '.' as i32
                                && (*next).d_name[2 as libc::c_int as usize] as libc::c_int
                                    == 0 as libc::c_int)
                {
                    continue;
                }
                off = vstrcpy(
                    &mut roll,
                    &mut size,
                    off,
                    ((*next).d_name).as_mut_ptr() as *mut libc::c_void,
                );
            }
            closedir(here);
            vstrcpy(
                &mut roll,
                &mut size,
                off,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            );
            base = if len != 0
                && *(g.inf)
                    .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int != '/' as i32 as libc::c_uchar as libc::c_int
            {
                (vstrcpy(
                    &mut g.inf,
                    &mut g.inz,
                    len,
                    b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                ))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                len
            };
            off = 0 as libc::c_int as size_t;
            while *roll.offset(off as isize) != 0 {
                vstrcpy(
                    &mut g.inf,
                    &mut g.inz,
                    base,
                    roll.offset(off as isize) as *mut libc::c_void,
                );
                process(g.inf);
                off = (off as libc::c_ulong)
                    .wrapping_add(
                        (strlen(roll.offset(off as isize)))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
            *(g.inf).offset(len as isize) = 0 as libc::c_int as libc::c_char;
            free(roll as *mut libc::c_void);
            return;
        }
        if !(g.force != 0 || g.list != 0 || g.decode != 0) && len >= strlen(g.sufx)
            && strcmp(
                (g.inf).offset(len as isize).offset(-(strlen(g.sufx) as isize)),
                g.sufx,
            ) == 0 as libc::c_int
        {
            grumble(
                b"skipping: %s ends with %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                g.sufx,
            );
            return;
        }
        if g.decode == 1 as libc::c_int && g.pipeout == 0 && g.list == 0 {
            let mut suf: size_t = compressed_suffix(g.inf);
            if suf == 0 as libc::c_int as libc::c_ulong {
                complain(
                    b"skipping: %s does not have compressed suffix\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    g.inf,
                );
                return;
            }
            len = (len as libc::c_ulong).wrapping_sub(suf) as size_t as size_t;
        }
        g.ind = open(g.inf, 0 as libc::c_int, 0 as libc::c_int);
        if g.ind < 0 as libc::c_int {
            try_throw_(
                *__errno_location(),
                b"read error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.inf,
                strerror(*__errno_location()),
                0 as *mut libc::c_void,
            );
        }
        g
            .name = if g.headis & 1 as libc::c_int != 0 {
            justname(g.inf)
        } else {
            0 as *mut libc::c_char
        };
        g
            .mtime = if g.headis & 2 as libc::c_int != 0 {
            st.st_mtim.tv_sec
        } else {
            0 as libc::c_int as libc::c_long
        };
    }
    if g.list != 0 && g.decode != 2 as libc::c_int {
        list_info();
        load_end();
        return;
    }
    if g.decode != 0 {
        in_init();
        ::std::ptr::write_volatile(
            &mut method as *mut libc::c_int,
            get_header(1 as libc::c_int),
        );
        if method != 8 as libc::c_int && method != 257 as libc::c_int
            && !((method == -(1 as libc::c_int) || method == -(2 as libc::c_int))
                && g.force != 0 && g.pipeout != 0 && g.decode != 2 as libc::c_int
                && g.list == 0)
        {
            load_end();
            complain(
                (if method == -(6 as libc::c_int) {
                    b"skipping: %s corrupt: header crc error\0" as *const u8
                        as *const libc::c_char
                } else if method == -(1 as libc::c_int) {
                    b"skipping: %s empty\0" as *const u8 as *const libc::c_char
                } else if method < 0 as libc::c_int {
                    b"skipping: %s unrecognized format\0" as *const u8
                        as *const libc::c_char
                } else {
                    b"skipping: %s unknown compression method\0" as *const u8
                        as *const libc::c_char
                }) as *mut libc::c_char,
                g.inf,
            );
            return;
        }
        if g.decode == 2 as libc::c_int {
            let mut try_this_: try_t_ = try_t_ {
                env: [__jmp_buf_tag {
                    __jmpbuf: [0; 8],
                    __mask_was_saved: 0,
                    __saved_mask: __sigset_t { __val: [0; 16] },
                }; 1],
                ball: try_ball_t_ {
                    ret: 0,
                    code: 0,
                    free: 0,
                    why: 0 as *mut libc::c_char,
                },
                next: 0 as *mut try_t_,
            };
            let mut try_pushed_: libc::c_int = 1 as libc::c_int;
            try_this_.ball.ret = 0 as libc::c_int;
            try_this_.ball.code = 0 as libc::c_int;
            try_this_.ball.free = 0 as libc::c_int;
            try_this_.ball.why = 0 as *mut libc::c_char;
            try_setup_();
            try_this_.next = pthread_getspecific(try_key_) as *mut try_t_;
            if pthread_setspecific(
                try_key_,
                &mut try_this_ as *mut try_t_ as *const libc::c_void,
            ) == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4068 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"void process(char *)\0"))
                        .as_ptr(),
                );
            }
            'c_28616: {
                if pthread_setspecific(
                    try_key_,
                    &mut try_this_ as *mut try_t_ as *const libc::c_void,
                ) == 0 as libc::c_int
                    && !(b"try: pthread_setspecific() failed\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                            as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4068 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"void process(char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if _setjmp((try_this_.env).as_mut_ptr()) == 0 as libc::c_int {
                if method == 8 as libc::c_int {
                    infchk();
                } else {
                    unlzw();
                    if g.list != 0 {
                        g
                            .in_tot = (g.in_tot as libc::c_ulong)
                            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as length_t
                            as length_t;
                        show_info(
                            method,
                            0 as libc::c_int as libc::c_ulong,
                            g.out_tot,
                            0 as libc::c_int,
                        );
                    }
                }
            }
            if try_pushed_ != 0 {
                if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                    == 0 as libc::c_int
                    && !(b"try: pthread_setspecific() failed\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                            as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4079 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"void process(char *)\0"))
                            .as_ptr(),
                    );
                }
                'c_28507: {
                    if pthread_setspecific(
                        try_key_,
                        try_this_.next as *const libc::c_void,
                    ) == 0 as libc::c_int
                        && !(b"try: pthread_setspecific() failed\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                    {} else {
                        __assert_fail(
                            b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                                as *const u8 as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            4079 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"void process(char *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                ::std::ptr::write_volatile(
                    &mut try_pushed_ as *mut libc::c_int,
                    0 as libc::c_int,
                );
            }
            err = try_this_.ball;
            if err.code != 0 {
                if err.code != 33 as libc::c_int {
                    try_setup_();
                    if !(pthread_getspecific(try_key_) as *mut try_t_).is_null()
                        && !(b"try: naked punt\0" as *const u8 as *const libc::c_char)
                            .is_null()
                    {} else {
                        __assert_fail(
                            b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                                as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            4081 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"void process(char *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_28408: {
                        if !(pthread_getspecific(try_key_) as *mut try_t_).is_null()
                            && !(b"try: naked punt\0" as *const u8
                                as *const libc::c_char)
                                .is_null()
                        {} else {
                            __assert_fail(
                                b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                                    as *const libc::c_char,
                                b"pigz.c\0" as *const u8 as *const libc::c_char,
                                4081 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 21],
                                    &[libc::c_char; 21],
                                >(b"void process(char *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    (*(pthread_getspecific(try_key_) as *mut try_t_)).ball = err;
                    longjmp(
                        ((*(pthread_getspecific(try_key_) as *mut try_t_)).env)
                            .as_mut_ptr(),
                        1 as libc::c_int,
                    );
                }
                complain(
                    b"skipping: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    err.why,
                );
                if err.free != 0 {
                    free(err.why as *mut libc::c_void);
                    err.free = 0 as libc::c_int;
                    err.why = 0 as *mut libc::c_char;
                }
                outb(
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_uchar,
                    0 as libc::c_int as libc::c_uint,
                );
            }
            load_end();
            return;
        }
    }
    if path.is_null() || g.pipeout != 0 {
        g
            .outf = alloc(
            0 as *mut libc::c_void,
            (strlen(b"<stdout>\0" as *const u8 as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(g.outf, b"<stdout>\0" as *const u8 as *const libc::c_char);
        g.outd = 1 as libc::c_int;
        if g.decode == 0 && g.force == 0 && isatty(g.outd) != 0 {
            try_throw_(
                22 as libc::c_int,
                b"trying to write compressed data to a terminal (use -f to force)\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
    } else {
        let mut to: *mut libc::c_char = g.inf;
        let mut sufx_0: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        let mut pre: size_t = 0 as libc::c_int as size_t;
        if g.decode != 0 {
            if g.headis & 1 as libc::c_int != 0 as libc::c_int && !(g.hname).is_null() {
                pre = (justname(g.inf)).offset_from(g.inf) as libc::c_long as size_t;
                to = justname(g.hname);
                len = strlen(to);
            } else if strcmp(
                to.offset(len as isize),
                b".tgz\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                sufx_0 = b".tar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        } else {
            sufx_0 = g.sufx;
        }
        g
            .outf = alloc(
            0 as *mut libc::c_void,
            pre
                .wrapping_add(len)
                .wrapping_add(strlen(sufx_0))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(g.outf as *mut libc::c_void, g.inf as *const libc::c_void, pre);
        memcpy(
            (g.outf).offset(pre as isize) as *mut libc::c_void,
            to as *const libc::c_void,
            len,
        );
        strcpy((g.outf).offset(pre as isize).offset(len as isize), sufx_0);
        g
            .outd = open(
            g.outf,
            0o100 as libc::c_int | 0o1000 as libc::c_int | 0o1 as libc::c_int
                | (if g.force != 0 { 0 as libc::c_int } else { 0o200 as libc::c_int }),
            0o600 as libc::c_int,
        );
        if g.outd < 0 as libc::c_int && *__errno_location() == 17 as libc::c_int {
            let mut overwrite: libc::c_int = 0 as libc::c_int;
            if isatty(0 as libc::c_int) != 0 && g.verbosity != 0 {
                fprintf(
                    stderr,
                    b"%s exists -- overwrite (y/n)? \0" as *const u8
                        as *const libc::c_char,
                    g.outf,
                );
                fflush(stderr);
                let mut ch: libc::c_int = 0;
                let mut first: libc::c_int = 1 as libc::c_int;
                loop {
                    ch = getchar();
                    if first == 1 as libc::c_int {
                        if !(ch == ' ' as i32 || ch == '\t' as i32) {
                            if ch == 'y' as i32 || ch == 'Y' as i32 {
                                overwrite = 1 as libc::c_int;
                            }
                            first = 0 as libc::c_int;
                        }
                    }
                    if !(ch != -(1 as libc::c_int) && ch != '\n' as i32
                        && ch != '\r' as i32)
                    {
                        break;
                    }
                }
            }
            if overwrite == 0 {
                complain(
                    b"skipping: %s exists\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g.outf,
                );
                if !(g.outf).is_null() {
                    free(g.outf as *mut libc::c_void);
                    g.outf = 0 as *mut libc::c_char;
                }
                load_end();
                return;
            }
            g
                .outd = open(
                g.outf,
                0o100 as libc::c_int | 0o1000 as libc::c_int | 0o1 as libc::c_int,
                0o600 as libc::c_int,
            );
        }
        if g.outd < 0 as libc::c_int {
            try_throw_(
                *__errno_location(),
                b"write error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.outf,
                strerror(*__errno_location()),
                0 as *mut libc::c_void,
            );
        }
    }
    if g.verbosity > 1 as libc::c_int {
        fprintf(
            stderr,
            b"%s to %s \0" as *const u8 as *const libc::c_char,
            g.inf,
            g.outf,
        );
    }
    if g.decode != 0 {
        let mut try_this__0: try_t_ = try_t_ {
            env: [__jmp_buf_tag {
                __jmpbuf: [0; 8],
                __mask_was_saved: 0,
                __saved_mask: __sigset_t { __val: [0; 16] },
            }; 1],
            ball: try_ball_t_ {
                ret: 0,
                code: 0,
                free: 0,
                why: 0 as *mut libc::c_char,
            },
            next: 0 as *mut try_t_,
        };
        let mut try_pushed__0: libc::c_int = 1 as libc::c_int;
        try_this__0.ball.ret = 0 as libc::c_int;
        try_this__0.ball.code = 0 as libc::c_int;
        try_this__0.ball.free = 0 as libc::c_int;
        try_this__0.ball.why = 0 as *mut libc::c_char;
        try_setup_();
        try_this__0.next = pthread_getspecific(try_key_) as *mut try_t_;
        if pthread_setspecific(
            try_key_,
            &mut try_this__0 as *mut try_t_ as *const libc::c_void,
        ) == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4170 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void process(char *)\0"))
                    .as_ptr(),
            );
        }
        'c_27706: {
            if pthread_setspecific(
                try_key_,
                &mut try_this__0 as *mut try_t_ as *const libc::c_void,
            ) == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4170 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"void process(char *)\0"))
                        .as_ptr(),
                );
            }
        };
        if _setjmp((try_this__0.env).as_mut_ptr()) == 0 as libc::c_int {
            if method == 8 as libc::c_int {
                infchk();
            } else if method == 257 as libc::c_int {
                unlzw();
            } else {
                cat();
            }
        }
        if try_pushed__0 != 0 {
            if pthread_setspecific(try_key_, try_this__0.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4178 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"void process(char *)\0"))
                        .as_ptr(),
                );
            }
            'c_27609: {
                if pthread_setspecific(try_key_, try_this__0.next as *const libc::c_void)
                    == 0 as libc::c_int
                    && !(b"try: pthread_setspecific() failed\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                            as *const u8 as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4178 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"void process(char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            ::std::ptr::write_volatile(
                &mut try_pushed__0 as *mut libc::c_int,
                0 as libc::c_int,
            );
        }
        err = try_this__0.ball;
        if err.code != 0 {
            if err.code != 33 as libc::c_int {
                try_setup_();
                if !(pthread_getspecific(try_key_) as *mut try_t_).is_null()
                    && !(b"try: naked punt\0" as *const u8 as *const libc::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                            as *const libc::c_char,
                        b"pigz.c\0" as *const u8 as *const libc::c_char,
                        4180 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"void process(char *)\0"))
                            .as_ptr(),
                    );
                }
                'c_27507: {
                    if !(pthread_getspecific(try_key_) as *mut try_t_).is_null()
                        && !(b"try: naked punt\0" as *const u8 as *const libc::c_char)
                            .is_null()
                    {} else {
                        __assert_fail(
                            b"try_stack_ != NULL && \"try: naked punt\"\0" as *const u8
                                as *const libc::c_char,
                            b"pigz.c\0" as *const u8 as *const libc::c_char,
                            4180 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"void process(char *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                (*(pthread_getspecific(try_key_) as *mut try_t_)).ball = err;
                longjmp(
                    ((*(pthread_getspecific(try_key_) as *mut try_t_)).env).as_mut_ptr(),
                    1 as libc::c_int,
                );
            }
            complain(
                b"skipping: %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                err.why,
            );
            if err.free != 0 {
                free(err.why as *mut libc::c_void);
                err.free = 0 as libc::c_int;
                err.why = 0 as *mut libc::c_char;
            }
            outb(
                0 as *mut libc::c_void,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as libc::c_uint,
            );
            if g.outd != -(1 as libc::c_int) && g.outd != 1 as libc::c_int {
                close(g.outd);
                g.outd = -(1 as libc::c_int);
                unlink(g.outf);
                if !(g.outf).is_null() {
                    free(g.outf as *mut libc::c_void);
                    g.outf = 0 as *mut libc::c_char;
                }
            }
        }
    } else if g.procs > 1 as libc::c_int {
        parallel_compress();
    } else {
        single_compress(0 as libc::c_int);
    }
    if g.verbosity > 1 as libc::c_int {
        putc('\n' as i32, stderr);
        fflush(stderr);
    }
    load_end();
    if g.outd != -(1 as libc::c_int) && g.outd != 1 as libc::c_int {
        if g.sync != 0 {
            out_push();
        }
        if close(g.outd) != 0 {
            try_throw_(
                *__errno_location(),
                b"write error on %s (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g.outf,
                strerror(*__errno_location()),
                0 as *mut libc::c_void,
            );
        }
        g.outd = -(1 as libc::c_int);
        if g.ind != 0 as libc::c_int {
            copymeta(g.inf, g.outf);
            if g.keep == 0 {
                if st.st_nlink > 1 as libc::c_int as libc::c_ulong && g.force == 0 {
                    complain(
                        b"%s has hard links -- not unlinking\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        g.inf,
                    );
                } else {
                    unlink(g.inf);
                }
            }
        }
        if g.decode != 0 && g.headis & 2 as libc::c_int != 0 as libc::c_int
            && g.stamp != 0
        {
            touch(g.outf, g.stamp);
        }
    }
    if !(g.outf).is_null() {
        free(g.outf as *mut libc::c_void);
        g.outf = 0 as *mut libc::c_char;
    }
}
static mut helptext: [*mut libc::c_char; 42] = [
    b"Usage: pigz [options] [files ...]\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"  will compress files in place, adding the suffix '.gz'. If no files are\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  specified, stdin will be compressed to stdout. pigz does what gzip does,\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  but spreads the work over multiple processors and cores when compressing.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Options:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -0 to -9, -11        Compression level (level 11, zopfli, is much slower)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  --fast, --best       Compression levels 1 and 9 respectively\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -A, --alias xxx      Use xxx as the name for any --zip entry from stdin\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -b, --blocksize mmm  Set compression block size to mmmK (default 128K)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -c, --stdout         Write all processed output to stdout (won't delete)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -C, --comment ccc    Put comment ccc in the gzip or zip header\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -d, --decompress     Decompress the compressed input\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -f, --force          Force overwrite, compress .gz, links, and to terminal\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -F  --first          Do iterations first, before block split for -11\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -h, --help           Display a help screen and quit\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -H, --huffman        Use only Huffman coding for compression\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -i, --independent    Compress blocks independently for damage recovery\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -I, --iterations n   Number of iterations for -11 optimization\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -J, --maxsplits n    Maximum number of split blocks for -11\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -k, --keep           Do not delete original file after processing\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -K, --zip            Compress to PKWare zip (.zip) single entry format\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -l, --list           List the contents of the compressed input\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -L, --license        Display the pigz license and quit\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -m, --no-time        Do not store or restore mod time\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -M, --time           Store or restore mod time\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -n, --no-name        Do not store or restore file name or mod time\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -N, --name           Store or restore file name and mod time\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -O  --oneblock       Do not split into smaller blocks for -11\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -p, --processes n    Allow up to n compression threads (default is the\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"                       number of online processors, or 8 if unknown)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -q, --quiet          Print no messages, even on error\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -r, --recursive      Process the contents of all subdirectories\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -R, --rsyncable      Input-determined block locations for rsync\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -S, --suffix .sss    Use suffix .sss instead of .gz (for compression)\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  -t, --test           Test the integrity of the compressed input\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -U, --rle            Use run-length encoding for compression\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -v, --verbose        Provide more verbose output\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -V  --version        Show the version of pigz\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -Y  --synchronous    Force output file write to permanent storage\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"  -z, --zlib           Compress to zlib (.zz) instead of gzip format\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"  --                   All arguments after \"--\" are treated as files\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn help() {
    let mut n: libc::c_int = 0;
    if g.verbosity == 0 as libc::c_int {
        return;
    }
    n = 0 as libc::c_int;
    while n
        < (::std::mem::size_of::<[*mut libc::c_char; 42]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int
    {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            helptext[n as usize],
        );
        n += 1;
        n;
    }
    fflush(stderr);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn nprocs(mut n: libc::c_int) -> libc::c_int {
    n = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    return n;
}
unsafe extern "C" fn defaults() {
    g.level = -(1 as libc::c_int);
    g.strategy = 0 as libc::c_int;
    ZopfliInitOptions(&mut g.zopts);
    g.block = 131072 as libc::c_ulong;
    g.procs = nprocs(8 as libc::c_int);
    g.shift = x2nmodp(g.block, 3 as libc::c_int as libc::c_uint);
    g.rsync = 0 as libc::c_int;
    g.setdict = 1 as libc::c_int;
    g.verbosity = 1 as libc::c_int;
    g.headis = 3 as libc::c_int;
    g.pipeout = 0 as libc::c_int;
    g.sufx = b".gz\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g.comment = 0 as *mut libc::c_char;
    g.decode = 0 as libc::c_int;
    g.list = 0 as libc::c_int;
    g.keep = 0 as libc::c_int;
    g.force = 0 as libc::c_int;
    g.sync = 0 as libc::c_int;
    g.recurse = 0 as libc::c_int;
    g.form = 0 as libc::c_int;
}
static mut longopts: [[*mut libc::c_char; 2]; 41] = [
    [
        b"LZW\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"lzw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"alias\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"ascii\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"best\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"bits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"blocksize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"decompress\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"fast\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"force\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"first\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"iterations\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"maxsplits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"J\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"oneblock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"O\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"independent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"keep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"k\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"license\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"l\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"no-name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"no-time\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"processes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"quiet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"recursive\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"rsyncable\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"silent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"suffix\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"synchronous\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"test\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"time\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"to-stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"uncompress\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"verbose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"v\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"V\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"zip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"zlib\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"huffman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"rle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"U\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
];
unsafe extern "C" fn new_opts() {
    single_compress(1 as libc::c_int);
    finish_jobs();
}
unsafe extern "C" fn num(mut arg: *mut libc::c_char) -> size_t {
    let mut str: *mut libc::c_char = arg;
    let mut val: size_t = 0 as libc::c_int as size_t;
    if *str as libc::c_int == 0 as libc::c_int {
        try_throw_(
            22 as libc::c_int,
            b"internal error: empty parameter\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    loop {
        if (*str as libc::c_int) < '0' as i32 || *str as libc::c_int > '9' as i32
            || val != 0
                && (!(0 as libc::c_int as size_t))
                    .wrapping_sub((*str as libc::c_int - '0' as i32) as size_t)
                    .wrapping_div(val) < 10 as libc::c_int as libc::c_ulong
        {
            try_throw_(
                22 as libc::c_int,
                b"invalid numeric parameter: %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                arg,
                0 as *mut libc::c_void,
            );
        }
        val = val
            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
            .wrapping_add((*str as libc::c_int - '0' as i32) as size_t);
        str = str.offset(1);
        if !(*str != 0) {
            break;
        }
    }
    return val;
}
unsafe extern "C" fn option(mut arg: *mut libc::c_char) -> libc::c_int {
    static mut get: libc::c_int = 0 as libc::c_int;
    let mut bad: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"-X\0");
    if get != 0 && (arg.is_null() || *arg as libc::c_int == '-' as i32) {
        bad[1 as libc::c_int
            as usize] = (*::std::mem::transmute::<
            &[u8; 8],
            &[libc::c_char; 8],
        >(b"bpSIJAC\0"))[(get - 1 as libc::c_int) as usize];
        try_throw_(
            22 as libc::c_int,
            b"missing parameter after %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            bad.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
    }
    if arg.is_null() {
        return 1 as libc::c_int;
    }
    if *arg as libc::c_int == '-' as i32 {
        arg = arg.offset(1);
        if *arg as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if *arg as libc::c_int == '-' as i32 {
            let mut j: libc::c_int = 0;
            arg = arg.offset(1);
            arg;
            j = (::std::mem::size_of::<[[*mut libc::c_char; 2]; 41]>() as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        << 1 as libc::c_int,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            while j >= 0 as libc::c_int {
                if strcmp(arg, longopts[j as usize][0 as libc::c_int as usize])
                    == 0 as libc::c_int
                {
                    arg = longopts[j as usize][1 as libc::c_int as usize];
                    break;
                } else {
                    j -= 1;
                    j;
                }
            }
            if j < 0 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"invalid option: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg.offset(-(2 as libc::c_int as isize)),
                    0 as *mut libc::c_void,
                );
            }
        }
        loop {
            if get != 0 {
                if get == 3 as libc::c_int {
                    try_throw_(
                        22 as libc::c_int,
                        b"invalid usage: -S must be followed by space\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                if get == 7 as libc::c_int {
                    try_throw_(
                        22 as libc::c_int,
                        b"invalid usage: -C must be followed by space\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                break;
            } else {
                bad[1 as libc::c_int as usize] = *arg;
                match *arg as libc::c_int {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        g.level = *arg as libc::c_int - '0' as i32;
                        while *arg.offset(1 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *arg.offset(1 as libc::c_int as isize) as libc::c_int
                                <= '9' as i32
                        {
                            if g.level != 0
                                && (2147483647 as libc::c_int
                                    - (*arg.offset(1 as libc::c_int as isize) as libc::c_int
                                        - '0' as i32)) / g.level < 10 as libc::c_int
                            {
                                try_throw_(
                                    22 as libc::c_int,
                                    b"only levels 0..9 and 11 are allowed\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_void,
                                );
                            }
                            arg = arg.offset(1);
                            g
                                .level = g.level * 10 as libc::c_int + *arg as libc::c_int
                                - '0' as i32;
                        }
                        if g.level == 10 as libc::c_int || g.level > 11 as libc::c_int {
                            try_throw_(
                                22 as libc::c_int,
                                b"only levels 0..9 and 11 are allowed\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                0 as *mut libc::c_void,
                            );
                        }
                    }
                    65 => {
                        get = 6 as libc::c_int;
                    }
                    67 => {
                        get = 7 as libc::c_int;
                    }
                    70 => {
                        g.zopts.blocksplittinglast = 1 as libc::c_int;
                    }
                    72 => {
                        g.strategy = 2 as libc::c_int;
                    }
                    73 => {
                        get = 4 as libc::c_int;
                    }
                    74 => {
                        get = 5 as libc::c_int;
                    }
                    75 => {
                        g.form = 2 as libc::c_int;
                        g
                            .sufx = b".zip\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    76 => {
                        puts(b"pigz 2.7\0" as *const u8 as *const libc::c_char);
                        puts(
                            b"Copyright (C) 2007-2022 Mark Adler\0" as *const u8
                                as *const libc::c_char,
                        );
                        puts(
                            b"Subject to the terms of the zlib license.\0" as *const u8
                                as *const libc::c_char,
                        );
                        puts(
                            b"No warranty is provided or implied.\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(0 as libc::c_int);
                    }
                    77 => {
                        g.headis |= 0xa as libc::c_int;
                    }
                    78 => {
                        g.headis = 0xf as libc::c_int;
                    }
                    79 => {
                        g.zopts.blocksplitting = 0 as libc::c_int;
                    }
                    82 => {
                        g.rsync = 1 as libc::c_int;
                    }
                    83 => {
                        get = 3 as libc::c_int;
                    }
                    86 => {
                        puts(b"pigz 2.7\0" as *const u8 as *const libc::c_char);
                        if g.verbosity > 1 as libc::c_int {
                            printf(
                                b"zlib %s\n\0" as *const u8 as *const libc::c_char,
                                zlibVersion(),
                            );
                        }
                        exit(0 as libc::c_int);
                    }
                    89 => {
                        g.sync = 1 as libc::c_int;
                    }
                    90 => {
                        try_throw_(
                            22 as libc::c_int,
                            b"invalid option: LZW output not supported: %s\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            bad.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                    }
                    97 => {
                        try_throw_(
                            22 as libc::c_int,
                            b"invalid option: no ascii conversion: %s\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            bad.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                    }
                    98 => {
                        get = 1 as libc::c_int;
                    }
                    99 => {
                        g.pipeout = 1 as libc::c_int;
                    }
                    100 => {
                        if g.decode == 0 {
                            g.headis >>= 2 as libc::c_int;
                        }
                        g.decode = 1 as libc::c_int;
                    }
                    102 => {
                        g.force = 1 as libc::c_int;
                    }
                    104 => {
                        help();
                    }
                    105 => {
                        g.setdict = 0 as libc::c_int;
                    }
                    107 => {
                        g.keep = 1 as libc::c_int;
                    }
                    108 => {
                        g.list = 1 as libc::c_int;
                    }
                    110 => {
                        g.headis = 0 as libc::c_int;
                    }
                    84 | 109 => {
                        g.headis &= !(0xa as libc::c_int);
                    }
                    112 => {
                        get = 2 as libc::c_int;
                    }
                    113 => {
                        g.verbosity = 0 as libc::c_int;
                    }
                    114 => {
                        g.recurse = 1 as libc::c_int;
                    }
                    116 => {
                        g.decode = 2 as libc::c_int;
                    }
                    85 => {
                        g.strategy = 3 as libc::c_int;
                    }
                    118 => {
                        g.verbosity += 1;
                        g.verbosity;
                    }
                    122 => {
                        g.form = 1 as libc::c_int;
                        g
                            .sufx = b".zz\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {
                        try_throw_(
                            22 as libc::c_int,
                            b"invalid option: %s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            bad.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                    }
                }
                arg = arg.offset(1);
                if !(*arg != 0) {
                    break;
                }
            }
        }
        if *arg as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if get != 0 {
        let mut n: size_t = 0;
        if get == 1 as libc::c_int {
            n = num(arg);
            g.block = n << 10 as libc::c_int;
            g.shift = x2nmodp(g.block, 3 as libc::c_int as libc::c_uint);
            if g.block < 32768 as libc::c_uint as libc::c_ulong {
                try_throw_(
                    22 as libc::c_int,
                    b"block size too small (must be >= 32K)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
            if n != g.block >> 10 as libc::c_int
                || (g.block)
                    .wrapping_add(g.block >> 4 as libc::c_int)
                    .wrapping_add(32768 as libc::c_uint as libc::c_ulong) < g.block
                || ((g.block)
                    .wrapping_add(g.block >> 4 as libc::c_int)
                    .wrapping_add(32768 as libc::c_uint as libc::c_ulong) as ssize_t)
                    < 0 as libc::c_int as libc::c_long
                || g.block > (1 as libc::c_ulong) << 29 as libc::c_int
            {
                try_throw_(
                    22 as libc::c_int,
                    b"block size too large: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg,
                    0 as *mut libc::c_void,
                );
            }
        } else if get == 2 as libc::c_int {
            n = num(arg);
            g.procs = n as libc::c_int;
            if g.procs < 1 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"invalid number of processes: %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    arg,
                    0 as *mut libc::c_void,
                );
            }
            if g.procs as size_t != n
                || ((g.procs << 1 as libc::c_int) + 3 as libc::c_int) < 1 as libc::c_int
            {
                try_throw_(
                    22 as libc::c_int,
                    b"too many processes: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg,
                    0 as *mut libc::c_void,
                );
            }
        } else if get == 3 as libc::c_int {
            if *arg as libc::c_int == 0 as libc::c_int {
                try_throw_(
                    22 as libc::c_int,
                    b"suffix cannot be empty\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
            }
            g.sufx = arg;
        } else if get == 4 as libc::c_int {
            g.zopts.numiterations = num(arg) as libc::c_int;
        } else if get == 5 as libc::c_int {
            g.zopts.blocksplittingmax = num(arg) as libc::c_int;
        } else if get == 6 as libc::c_int {
            g.alias = arg;
        } else if get == 7 as libc::c_int {
            g.comment = arg;
        }
        get = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cut_yarn(mut err: libc::c_int) {
    try_throw_(
        err,
        b"internal threads error\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as *mut libc::c_void,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut nop: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut k: size_t = 0;
    let mut opts: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: try_ball_t_ = try_ball_t_ {
        ret: 0,
        code: 0,
        free: 0,
        why: 0 as *mut libc::c_char,
    };
    g.ret = 0 as libc::c_int;
    let mut try_this_: try_t_ = try_t_ {
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        ball: try_ball_t_ {
            ret: 0,
            code: 0,
            free: 0,
            why: 0 as *mut libc::c_char,
        },
        next: 0 as *mut try_t_,
    };
    let mut try_pushed_: libc::c_int = 1 as libc::c_int;
    try_this_.ball.ret = 0 as libc::c_int;
    try_this_.ball.code = 0 as libc::c_int;
    try_this_.ball.free = 0 as libc::c_int;
    try_this_.ball.why = 0 as *mut libc::c_char;
    try_setup_();
    try_this_.next = pthread_getspecific(try_key_) as *mut try_t_;
    if pthread_setspecific(
        try_key_,
        &mut try_this_ as *mut try_t_ as *const libc::c_void,
    ) == 0 as libc::c_int
        && !(b"try: pthread_setspecific() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                as *const u8 as *const libc::c_char,
            b"pigz.c\0" as *const u8 as *const libc::c_char,
            4605 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_32652: {
        if pthread_setspecific(
            try_key_,
            &mut try_this_ as *mut try_t_ as *const libc::c_void,
        ) == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, &try_this_) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4605 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    if _setjmp((try_this_.env).as_mut_ptr()) == 0 as libc::c_int {
        g.inf = 0 as *mut libc::c_char;
        g.inz = 0 as libc::c_int as size_t;
        g.in_which = -(1 as libc::c_int);
        g.alias = b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        g.outf = 0 as *mut libc::c_char;
        g.first = 1 as libc::c_int;
        g.hname = 0 as *mut libc::c_char;
        g.hcomm = 0 as *mut libc::c_char;
        p = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
        p = if p.is_null() {
            *argv.offset(0 as libc::c_int as isize)
        } else {
            p.offset(1 as libc::c_int as isize)
        };
        g
            .prog = (if *p as libc::c_int != 0 {
            p as *const libc::c_char
        } else {
            b"pigz\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        signal(
            2 as libc::c_int,
            Some(cut_short as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        yarn_prefix = g.prog;
        yarn_abort = Some(cut_yarn as unsafe extern "C" fn(libc::c_int) -> ());
        defaults();
        if zlib_vernum() < 0x1230 as libc::c_int as libc::c_long {
            try_throw_(
                22 as libc::c_int,
                b"zlib version less than 1.2.3\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        get_crc_table();
        opts = getenv(b"GZIP\0" as *const u8 as *const libc::c_char);
        if !opts.is_null() {
            while *opts != 0 {
                while *opts as libc::c_int == ' ' as i32
                    || *opts as libc::c_int == '\t' as i32
                {
                    opts = opts.offset(1);
                    opts;
                }
                p = opts;
                while *p as libc::c_int != 0 && *p as libc::c_int != ' ' as i32
                    && *p as libc::c_int != '\t' as i32
                {
                    p = p.offset(1);
                    p;
                }
                n = *p as libc::c_int;
                *p = 0 as libc::c_int as libc::c_char;
                if option(opts) == 0 {
                    try_throw_(
                        22 as libc::c_int,
                        b"cannot provide files in GZIP environment variable\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                opts = p
                    .offset(
                        (if n != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
                            as isize,
                    );
            }
            option(0 as *mut libc::c_char);
        }
        opts = getenv(b"PIGZ\0" as *const u8 as *const libc::c_char);
        if !opts.is_null() {
            while *opts != 0 {
                while *opts as libc::c_int == ' ' as i32
                    || *opts as libc::c_int == '\t' as i32
                {
                    opts = opts.offset(1);
                    opts;
                }
                p = opts;
                while *p as libc::c_int != 0 && *p as libc::c_int != ' ' as i32
                    && *p as libc::c_int != '\t' as i32
                {
                    p = p.offset(1);
                    p;
                }
                n = *p as libc::c_int;
                *p = 0 as libc::c_int as libc::c_char;
                if option(opts) == 0 {
                    try_throw_(
                        22 as libc::c_int,
                        b"cannot provide files in PIGZ environment variable\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                }
                opts = p
                    .offset(
                        (if n != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
                            as isize,
                    );
            }
            option(0 as *mut libc::c_char);
        }
        if strcmp(g.prog, b"unpigz\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(g.prog, b"gunzip\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            if g.decode == 0 {
                g.headis >>= 2 as libc::c_int;
            }
            g.decode = 1 as libc::c_int;
        }
        k = strlen(g.prog);
        if k > 2 as libc::c_int as libc::c_ulong
            && strcmp(
                (g.prog).offset(k as isize).offset(-(3 as libc::c_int as isize)),
                b"cat\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            if g.decode == 0 {
                g.headis >>= 2 as libc::c_int;
            }
            g.decode = 1 as libc::c_int;
            g.pipeout = 1 as libc::c_int;
        }
        if argc < 2 as libc::c_int
            && isatty((if g.decode != 0 { 0 as libc::c_int } else { 1 as libc::c_int }))
                != 0
        {
            help();
        }
        nop = argc;
        n = 1 as libc::c_int;
        while n < argc {
            if strcmp(
                *argv.offset(n as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                nop = n;
                let ref mut fresh150 = *argv.offset(n as isize);
                *fresh150 = 0 as *mut libc::c_char;
                break;
            } else {
                if option(*argv.offset(n as isize)) != 0 {
                    let ref mut fresh151 = *argv.offset(n as isize);
                    *fresh151 = 0 as *mut libc::c_char;
                }
                n += 1;
                n;
            }
        }
        option(0 as *mut libc::c_char);
        done = 0 as libc::c_int;
        n = 1 as libc::c_int;
        while n < argc {
            if !(*argv.offset(n as isize)).is_null() {
                if done == 1 as libc::c_int && g.pipeout != 0 && g.decode == 0
                    && g.list == 0 && g.form > 1 as libc::c_int
                {
                    complain(
                        b"warning: output will be concatenated zip files -- %s will not be able to extract\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        g.prog,
                    );
                }
                process(
                    if n < nop
                        && strcmp(
                            *argv.offset(n as isize),
                            b"-\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                    {
                        0 as *mut libc::c_char
                    } else {
                        *argv.offset(n as isize)
                    },
                );
                done += 1;
                done;
            }
            n += 1;
            n;
        }
        if done == 0 as libc::c_int {
            process(0 as *mut libc::c_char);
        }
    }
    if try_pushed_ != 0 {
        if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
            == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4727 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_31865: {
            if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4727 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        ::std::ptr::write_volatile(
            &mut try_pushed_ as *mut libc::c_int,
            0 as libc::c_int,
        );
    }
    if !(g.inf).is_null() {
        free(g.inf as *mut libc::c_void);
        g.inf = 0 as *mut libc::c_char;
    }
    g.inz = 0 as libc::c_int as size_t;
    new_opts();
    if try_pushed_ != 0 {
        if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
            == 0 as libc::c_int
            && !(b"try: pthread_setspecific() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"pigz.c\0" as *const u8 as *const libc::c_char,
                4733 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_31753: {
            if pthread_setspecific(try_key_, try_this_.next as *const libc::c_void)
                == 0 as libc::c_int
                && !(b"try: pthread_setspecific() failed\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"pthread_setspecific(try_key_, try_this_.next) == 0 && \"try: pthread_setspecific() failed\"\0"
                        as *const u8 as *const libc::c_char,
                    b"pigz.c\0" as *const u8 as *const libc::c_char,
                    4733 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        ::std::ptr::write_volatile(
            &mut try_pushed_ as *mut libc::c_int,
            0 as libc::c_int,
        );
    }
    err = try_this_.ball;
    if err.code != 0 {
        if err.code != 32 as libc::c_int {
            complain(
                b"abort: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                err.why,
            );
        }
        if err.free != 0 {
            free(err.why as *mut libc::c_void);
            err.free = 0 as libc::c_int;
            err.why = 0 as *mut libc::c_char;
        }
        cut_short(-err.code);
    }
    return g.ret;
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
