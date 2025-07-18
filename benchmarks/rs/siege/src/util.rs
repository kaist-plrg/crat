use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type AUTH_T;
    pub type ARRAY_T;
    pub type COOKIES_T;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut my: CONFIG;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
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
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
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
pub type clock_t = __clock_t;
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_4 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_4 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_4 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_4 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_4 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_4 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_4 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_4 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_4 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_4 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_4 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_4 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_4 = 236;
pub const _SC_IPV6: C2RustUnnamed_4 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_4 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_4 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_4 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_4 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_4 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_4 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_4 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_4 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_4 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_4 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_4 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_4 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_4 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_4 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_4 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_4 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_4 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_4 = 182;
pub const _SC_TRACE: C2RustUnnamed_4 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_4 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_4 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_4 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_4 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_4 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_4 = 175;
pub const _SC_STREAMS: C2RustUnnamed_4 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_4 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_4 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_4 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_4 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_4 = 169;
pub const _SC_2_PBS: C2RustUnnamed_4 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_4 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_4 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_4 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_4 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_4 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_4 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_4 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_4 = 160;
pub const _SC_SPAWN: C2RustUnnamed_4 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_4 = 158;
pub const _SC_SHELL: C2RustUnnamed_4 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_4 = 156;
pub const _SC_REGEXP: C2RustUnnamed_4 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_4 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_4 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_4 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_4 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_4 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_4 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_4 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_4 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_4 = 146;
pub const _SC_PIPE: C2RustUnnamed_4 = 145;
pub const _SC_FIFO: C2RustUnnamed_4 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_4 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_4 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_4 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_4 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_4 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_4 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_4 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_4 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_4 = 135;
pub const _SC_BASE: C2RustUnnamed_4 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_4 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_4 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_4 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_4 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_4 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_4 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_4 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_4 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_4 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_4 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_4 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_4 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_4 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_4 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_4 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_4 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_4 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_4 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_4 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_4 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_4 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_4 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_4 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_4 = 110;
pub const _SC_NZERO: C2RustUnnamed_4 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_4 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_4 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_4 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_4 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_4 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_4 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_4 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_4 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_4 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_4 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_4 = 98;
pub const _SC_2_UPE: C2RustUnnamed_4 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_4 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_4 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_4 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_4 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_4 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_4 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_4 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_4 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_4 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_4 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_4 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_4 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_4 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_4 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_4 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_4 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_4 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_4 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_4 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_4 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_4 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_4 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_4 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_4 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_4 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_4 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_4 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_4 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_4 = 68;
pub const _SC_THREADS: C2RustUnnamed_4 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_4 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_4 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_4 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_4 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_4 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_4 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_4 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_4 = 60;
pub const _SC_SELECT: C2RustUnnamed_4 = 59;
pub const _SC_POLL: C2RustUnnamed_4 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_4 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_4 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_4 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_4 = 54;
pub const _SC_PII: C2RustUnnamed_4 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_4 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_4 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_4 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_4 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_4 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_4 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_4 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_4 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_4 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_4 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_4 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_4 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_4 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_4 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_4 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_4 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_4 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_4 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_4 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_4 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_4 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_4 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_4 = 30;
pub const _SC_VERSION: C2RustUnnamed_4 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_4 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_4 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_4 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_4 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_4 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_4 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_4 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_4 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_4 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_4 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_4 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_4 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_4 = 16;
pub const _SC_FSYNC: C2RustUnnamed_4 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_4 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_4 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_4 = 12;
pub const _SC_TIMERS: C2RustUnnamed_4 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_4 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_4 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_4 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_4 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_4 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_4 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_4 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_4 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_4 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_4 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_4 = 0;
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
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
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
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn parse_time(mut p: *mut libc::c_char) {
    let mut x: size_t = 0 as libc::c_int as size_t;
    my.secs = 0 as libc::c_int;
    my.time = my.secs;
    while *(*__ctype_b_loc())
        .offset(*p.offset(x as isize) as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        x = x.wrapping_add(1);
        x;
    }
    if x == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    my.time = atoi(substring(p, 0 as libc::c_int, x as libc::c_int));
    while x < strlen(p) {
        match ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p.offset(x as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*p.offset(x as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*p.offset(x as isize) as libc::c_int as isize);
            }
            __res
        }) {
            115 => {
                my.secs = my.time;
                my.time = 1 as libc::c_int;
                return;
            }
            109 => {
                my.secs = my.time * 60 as libc::c_int;
                my.time = 1 as libc::c_int;
                return;
            }
            104 => {
                my.secs = my.time * 3600 as libc::c_int;
                my.time = 1 as libc::c_int;
                return;
            }
            _ => {}
        }
        x = x.wrapping_add(1);
        x;
    }
    if my.time > 0 as libc::c_int && my.secs <= 0 as libc::c_int {
        my.secs = my.time * 60 as libc::c_int;
    }
}
pub unsafe extern "C" fn substring(
    mut str: *mut libc::c_char,
    mut start: libc::c_int,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if len < 1 as libc::c_int || start < 0 as libc::c_int
        || start > strlen(str) as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    if start + len > strlen(str) as libc::c_int {
        len = (strlen(str)).wrapping_sub(start as libc::c_ulong) as libc::c_int;
    }
    ret = xmalloc((len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    res = ret;
    ptr = str;
    end = str;
    i = 0 as libc::c_int;
    while i < start {
        i += 1;
        i;
        ptr = ptr.offset(1);
        ptr;
    }
    i = 0 as libc::c_int;
    while i < start + len {
        i += 1;
        i;
        end = end.offset(1);
        end;
    }
    while ptr < end {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        let fresh1 = res;
        res = res.offset(1);
        *fresh1 = *fresh0;
    }
    *res = 0 as libc::c_int as libc::c_char;
    return ret;
}
pub unsafe extern "C" fn okay(mut code: libc::c_int) -> BOOLEAN {
    return (code >= 100 as libc::c_int && code <= 299 as libc::c_int) as libc::c_int
        as BOOLEAN;
}
pub unsafe extern "C" fn strmatch(
    mut option: *mut libc::c_char,
    mut param: *mut libc::c_char,
) -> BOOLEAN {
    if strncasecmp(option, param, strlen(param)) == 0 && strlen(option) == strlen(param)
    {
        return boolean_true
    } else {
        return boolean_false
    };
}
pub unsafe extern "C" fn startswith(
    mut pre: *const libc::c_char,
    mut str: *const libc::c_char,
) -> BOOLEAN {
    let mut lenpre: size_t = strlen(pre);
    let mut lenstr: size_t = strlen(str);
    return (if lenstr < lenpre {
        boolean_false as libc::c_int
    } else {
        (strncmp(pre, str, lenpre) == 0 as libc::c_int) as libc::c_int
    }) as BOOLEAN;
}
pub unsafe extern "C" fn endswith(
    mut suffix: *const libc::c_char,
    mut str: *const libc::c_char,
) -> BOOLEAN {
    if str.is_null() || suffix.is_null() {
        return boolean_false;
    }
    let mut lenstr: size_t = strlen(str);
    let mut lensuffix: size_t = strlen(suffix);
    if lensuffix > lenstr {
        return boolean_false;
    }
    return (strncmp(
        str.offset(lenstr as isize).offset(-(lensuffix as isize)),
        suffix,
        lensuffix,
    ) == 0 as libc::c_int) as libc::c_int as BOOLEAN;
}
pub unsafe extern "C" fn uppercase(
    mut s: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    c = s as *mut libc::c_uchar;
    e = c.offset(len as isize);
    while c < e {
        *c = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *c as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*c as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*c as libc::c_int as isize);
            }
            __res
        }) as libc::c_uchar;
        c = c.offset(1);
        c;
    }
    return s;
}
pub unsafe extern "C" fn lowercase(
    mut s: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    c = s as *mut libc::c_uchar;
    e = c.offset(len as isize);
    while c < e {
        *c = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *c as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*c as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*c as libc::c_int as isize);
            }
            __res
        }) as libc::c_uchar;
        c = c.offset(1);
        c;
    }
    return s;
}
pub unsafe extern "C" fn stristr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    loop {
        let mut h: *const libc::c_char = haystack;
        let mut n: *const libc::c_char = needle;
        while ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *h as libc::c_uchar as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*h as libc::c_uchar as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*h as libc::c_uchar as libc::c_int as isize);
            }
            __res
        })
            == ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *n as libc::c_uchar as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(*n as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*n as libc::c_uchar as libc::c_int as isize);
                }
                __res
            }) && *n as libc::c_int != 0
        {
            h = h.offset(1);
            h;
            n = n.offset(1);
            n;
        }
        if *n as libc::c_int == 0 as libc::c_int {
            return haystack as *mut libc::c_char;
        }
        let fresh2 = haystack;
        haystack = haystack.offset(1);
        if !(*fresh2 != 0) {
            break;
        }
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn pthread_sleep_np(mut secs: libc::c_uint) {
    sleep(secs);
}
pub unsafe extern "C" fn pthread_usleep_np(mut usec: libc::c_ulong) {
    usleep(usec as __useconds_t);
}
pub unsafe extern "C" fn elapsed_time(mut time: clock_t) -> libc::c_float {
    let mut tps: libc::c_long = sysconf(_SC_CLK_TCK as libc::c_int);
    return time as libc::c_float / tps as libc::c_float;
}
pub unsafe extern "C" fn echo(mut fmt: *const libc::c_char, mut args: ...) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ap: ::std::ffi::VaListImpl;
    if my.quiet as u64 != 0 {
        return;
    }
    if my.get as u64 != 0 {
        ap = args.clone();
        vsnprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            fmt,
            ap.as_va_list(),
        );
        printf(b"%s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        fflush(stdout);
        return;
    }
    if my.debug as u64 != 0 {
        ap = args.clone();
        vsnprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            fmt,
            ap.as_va_list(),
        );
        if strlen(buf.as_mut_ptr()) == 1 as libc::c_int as libc::c_ulong {
            printf(b"%s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        } else {
            NOTIFY(DEBUG, buf.as_mut_ptr());
        }
    }
}
pub unsafe extern "C" fn debug(mut fmt: *const libc::c_char, mut args: ...) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ap: ::std::ffi::VaListImpl;
    if my.quiet as u64 != 0 {
        return;
    }
    if my.debug as u64 != 0 {
        ap = args.clone();
        vsnprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            fmt,
            ap.as_va_list(),
        );
        if strlen(buf.as_mut_ptr()) == 1 as libc::c_int as libc::c_ulong {
            printf(b"%s\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        } else {
            NOTIFY(DEBUG, buf.as_mut_ptr());
        }
    }
}
pub unsafe extern "C" fn pthread_rand_np(mut ctx: *mut libc::c_uint) -> libc::c_int {
    return rand_r(ctx);
}
pub unsafe extern "C" fn urandom() -> libc::c_int {
    let mut rand: libc::c_int = -(1 as libc::c_int);
    let mut fd: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    fd = open(b"/dev/urandom\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if fd >= 0 as libc::c_int {
        len = read(
            fd,
            &mut rand as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as libc::c_int;
        if len == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"ERROR: failed to open /dev/urandom\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        close(fd);
    }
    return rand;
}
pub unsafe extern "C" fn strnlen(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut end: *const libc::c_char = memchr(
        str as *const libc::c_void,
        '\0' as i32,
        len,
    ) as *const libc::c_char;
    return if !end.is_null() {
        end.offset_from(str) as libc::c_long as size_t
    } else {
        len
    };
}
pub unsafe extern "C" fn strncasestr(
    mut str1: *const libc::c_char,
    mut str2: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut str1_len: size_t = strnlen(str1, len);
    let mut str2_len: size_t = strlen(str2);
    let mut i: size_t = 0;
    if str1_len < 1 as libc::c_int as libc::c_ulong
        || str2_len < 1 as libc::c_int as libc::c_ulong
    {
        return 0 as *const libc::c_char;
    }
    i = 0 as libc::c_int as size_t;
    while i
        < str1_len.wrapping_sub(str2_len).wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        if strncasecmp(str1, str2, str2_len) == 0 as libc::c_int {
            return str1;
        }
        str1 = str1.offset(1);
        str1;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const libc::c_char;
}
