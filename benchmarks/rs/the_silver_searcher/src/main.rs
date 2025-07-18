use ::libc;
extern "C" {
    pub type real_pcre;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn free(__ptr: *mut libc::c_void);
    fn pcre_config(_: libc::c_int, _: *mut libc::c_void) -> libc::c_int;
    fn pcre_version() -> *const libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    static mut print_mtx: pthread_mutex_t;
    fn set_log_level(threshold: log_level);
    fn log_debug(fmt: *const libc::c_char, _: ...);
    fn log_err(fmt: *const libc::c_char, _: ...);
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut opts: cli_options;
    fn parse_options(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        base_paths: *mut *mut *mut libc::c_char,
        paths: *mut *mut *mut libc::c_char,
    );
    fn cleanup_options();
    static mut root_ignores: *mut ignores;
    fn init_ignore(
        parent: *mut ignores,
        dirname: *const libc::c_char,
        dirname_len: size_t,
    ) -> *mut ignores;
    fn cleanup_ignore(ig: *mut ignores);
    static mut out_fd: *mut FILE;
    fn ag_calloc(nelem: size_t, elsize: size_t) -> *mut libc::c_void;
    static mut stats: ag_stats;
    fn generate_alpha_skip(
        find: *const libc::c_char,
        f_len: size_t,
        skip_lookup: *mut size_t,
        case_sensitive: libc::c_int,
    );
    fn generate_find_skip(
        find: *const libc::c_char,
        f_len: size_t,
        skip_lookup: *mut *mut size_t,
        case_sensitive: libc::c_int,
    );
    fn generate_hash(
        find: *const libc::c_char,
        f_len: size_t,
        H: *mut uint8_t,
        case_sensitive: libc::c_int,
    );
    fn compile_study(
        re: *mut *mut pcre,
        re_extra: *mut *mut pcre_extra,
        q: *mut libc::c_char,
        pcre_opts: libc::c_int,
        study_opts: libc::c_int,
    );
    fn init_wordchar_table();
    fn is_wordchar(ch: libc::c_char) -> libc::c_int;
    fn is_lowercase(s: *const libc::c_char) -> libc::c_int;
    fn die(fmt: *const libc::c_char, _: ...);
    fn ag_asprintf(ret: *mut *mut libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut alpha_skip_lookup: [size_t; 256];
    static mut find_skip_lookup: *mut size_t;
    static mut h_table: [uint8_t; 65536];
    static mut work_queue: *mut work_queue_t;
    static mut work_queue_tail: *mut work_queue_t;
    static mut done_adding_files: libc::c_int;
    static mut files_ready: pthread_cond_t;
    static mut stats_mtx: pthread_mutex_t;
    static mut work_queue_mtx: pthread_mutex_t;
    static mut symhash: *mut symdir_t;
    fn search_stream(stream: *mut FILE, path: *const libc::c_char) -> ssize_t;
    fn search_file_worker(i: *mut libc::c_void) -> *mut libc::c_void;
    fn search_dir(
        ig: *mut ignores,
        base_path: *const libc::c_char,
        path: *const libc::c_char,
        depth: libc::c_int,
        original_dev: dev_t,
    );
}
pub type __uint8_t = libc::c_uchar;
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
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type ssize_t = __ssize_t;
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
pub type pcre = real_pcre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void,
}
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_3 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_3 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_3 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_3 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_3 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_3 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_3 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_3 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_3 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_3 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_3 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_3 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_3 = 236;
pub const _SC_IPV6: C2RustUnnamed_3 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_3 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_3 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_3 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_3 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_3 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_3 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_3 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_3 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_3 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_3 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_3 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_3 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_3 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_3 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_3 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_3 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_3 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_3 = 182;
pub const _SC_TRACE: C2RustUnnamed_3 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_3 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_3 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_3 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_3 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_3 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_3 = 175;
pub const _SC_STREAMS: C2RustUnnamed_3 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_3 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_3 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_3 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_3 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_3 = 169;
pub const _SC_2_PBS: C2RustUnnamed_3 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_3 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_3 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_3 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_3 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_3 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_3 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_3 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_3 = 160;
pub const _SC_SPAWN: C2RustUnnamed_3 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_3 = 158;
pub const _SC_SHELL: C2RustUnnamed_3 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_3 = 156;
pub const _SC_REGEXP: C2RustUnnamed_3 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_3 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_3 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_3 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_3 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_3 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_3 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_3 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_3 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_3 = 146;
pub const _SC_PIPE: C2RustUnnamed_3 = 145;
pub const _SC_FIFO: C2RustUnnamed_3 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_3 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_3 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_3 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_3 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_3 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_3 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_3 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_3 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_3 = 135;
pub const _SC_BASE: C2RustUnnamed_3 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_3 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_3 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_3 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_3 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_3 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_3 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_3 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_3 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_3 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_3 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_3 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_3 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_3 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_3 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_3 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_3 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_3 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_3 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_3 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_3 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_3 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_3 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_3 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_3 = 110;
pub const _SC_NZERO: C2RustUnnamed_3 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_3 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_3 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_3 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_3 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_3 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_3 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_3 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_3 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_3 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_3 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_3 = 98;
pub const _SC_2_UPE: C2RustUnnamed_3 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_3 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_3 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_3 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_3 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_3 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_3 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_3 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_3 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_3 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_3 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_3 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_3 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_3 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_3 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_3 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_3 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_3 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_3 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_3 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_3 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_3 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_3 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_3 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_3 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_3 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_3 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_3 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_3 = 68;
pub const _SC_THREADS: C2RustUnnamed_3 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_3 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_3 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_3 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_3 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_3 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_3 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_3 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_3 = 60;
pub const _SC_SELECT: C2RustUnnamed_3 = 59;
pub const _SC_POLL: C2RustUnnamed_3 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_3 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_3 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_3 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_3 = 54;
pub const _SC_PII: C2RustUnnamed_3 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_3 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_3 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_3 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_3 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_3 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_3 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_3 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_3 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_3 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_3 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_3 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_3 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_3 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_3 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_3 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_3 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_3 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_3 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_3 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_3 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_3 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_3 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_3 = 30;
pub const _SC_VERSION: C2RustUnnamed_3 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_3 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_3 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_3 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_3 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_3 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_3 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_3 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_3 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_3 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_3 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_3 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_3 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_3 = 16;
pub const _SC_FSYNC: C2RustUnnamed_3 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_3 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_3 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_3 = 12;
pub const _SC_TIMERS: C2RustUnnamed_3 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_3 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_3 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_3 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_3 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_3 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_3 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_3 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_3 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_3 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_3 = 0;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type log_level = libc::c_uint;
pub const LOG_LEVEL_NONE: log_level = 100;
pub const LOG_LEVEL_ERR: log_level = 40;
pub const LOG_LEVEL_WARN: log_level = 30;
pub const LOG_LEVEL_MSG: log_level = 20;
pub const LOG_LEVEL_DEBUG: log_level = 10;
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
pub type case_behavior = libc::c_uint;
pub const CASE_SENSITIVE_RETRY_INSENSITIVE: case_behavior = 4;
pub const CASE_SMART: case_behavior = 3;
pub const CASE_INSENSITIVE: case_behavior = 2;
pub const CASE_SENSITIVE: case_behavior = 1;
pub const CASE_DEFAULT: case_behavior = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_options {
    pub ackmate: libc::c_int,
    pub ackmate_dir_filter: *mut pcre,
    pub ackmate_dir_filter_extra: *mut pcre_extra,
    pub after: size_t,
    pub before: size_t,
    pub casing: case_behavior,
    pub file_search_string: *const libc::c_char,
    pub match_files: libc::c_int,
    pub file_search_regex: *mut pcre,
    pub file_search_regex_extra: *mut pcre_extra,
    pub color: libc::c_int,
    pub color_line_number: *mut libc::c_char,
    pub color_match: *mut libc::c_char,
    pub color_path: *mut libc::c_char,
    pub color_win_ansi: libc::c_int,
    pub column: libc::c_int,
    pub context: libc::c_int,
    pub follow_symlinks: libc::c_int,
    pub invert_match: libc::c_int,
    pub literal: libc::c_int,
    pub literal_starts_wordchar: libc::c_int,
    pub literal_ends_wordchar: libc::c_int,
    pub max_matches_per_file: size_t,
    pub max_search_depth: libc::c_int,
    pub mmap: libc::c_int,
    pub multiline: libc::c_int,
    pub one_dev: libc::c_int,
    pub only_matching: libc::c_int,
    pub path_sep: libc::c_char,
    pub path_to_ignore: libc::c_int,
    pub print_break: libc::c_int,
    pub print_count: libc::c_int,
    pub print_filename_only: libc::c_int,
    pub print_nonmatching_files: libc::c_int,
    pub print_path: libc::c_int,
    pub print_all_paths: libc::c_int,
    pub print_line_numbers: libc::c_int,
    pub print_long_lines: libc::c_int,
    pub passthrough: libc::c_int,
    pub re: *mut pcre,
    pub re_extra: *mut pcre_extra,
    pub recurse_dirs: libc::c_int,
    pub search_all_files: libc::c_int,
    pub skip_vcs_ignores: libc::c_int,
    pub search_binary_files: libc::c_int,
    pub search_zip_files: libc::c_int,
    pub search_hidden_files: libc::c_int,
    pub search_stream: libc::c_int,
    pub stats: libc::c_int,
    pub stream_line_num: size_t,
    pub match_found: libc::c_int,
    pub stdout_inode: ino_t,
    pub query: *mut libc::c_char,
    pub query_len: libc::c_int,
    pub pager: *mut libc::c_char,
    pub paths_len: libc::c_int,
    pub parallel: libc::c_int,
    pub use_thread_affinity: libc::c_int,
    pub vimgrep: libc::c_int,
    pub width: size_t,
    pub word_regexp: libc::c_int,
    pub workers: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignores {
    pub extensions: *mut *mut libc::c_char,
    pub extensions_len: size_t,
    pub names: *mut *mut libc::c_char,
    pub names_len: size_t,
    pub slash_names: *mut *mut libc::c_char,
    pub slash_names_len: size_t,
    pub regexes: *mut *mut libc::c_char,
    pub regexes_len: size_t,
    pub invert_regexes: *mut *mut libc::c_char,
    pub invert_regexes_len: size_t,
    pub slash_regexes: *mut *mut libc::c_char,
    pub slash_regexes_len: size_t,
    pub dirname: *const libc::c_char,
    pub dirname_len: size_t,
    pub abs_path: *mut libc::c_char,
    pub abs_path_len: size_t,
    pub parent: *mut ignores,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ag_stats {
    pub total_bytes: size_t,
    pub total_files: size_t,
    pub total_matches: size_t,
    pub total_file_matches: size_t,
    pub time_start: timeval,
    pub time_end: timeval,
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work_queue_t {
    pub path: *mut libc::c_char,
    pub next: *mut work_queue_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirkey_t {
    pub dev: dev_t,
    pub ino: ino_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symdir_t {
    pub key: dirkey_t,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_t {
    pub thread: pthread_t,
    pub id: libc::c_int,
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut base_paths: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut paths: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut pcre_opts: libc::c_int = 0x2 as libc::c_int;
    let mut study_opts: libc::c_int = 0 as libc::c_int;
    let mut workers: *mut worker_t = 0 as *mut worker_t;
    let mut workers_len: libc::c_int = 0;
    let mut num_cores: libc::c_int = 0;
    set_log_level(LOG_LEVEL_WARN);
    work_queue = 0 as *mut work_queue_t;
    work_queue_tail = 0 as *mut work_queue_t;
    root_ignores = init_ignore(
        0 as *mut ignores,
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    out_fd = stdout;
    parse_options(argc, argv, &mut base_paths, &mut paths);
    log_debug(b"PCRE Version: %s\0" as *const u8 as *const libc::c_char, pcre_version());
    if opts.stats != 0 {
        memset(
            &mut stats as *mut ag_stats as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<ag_stats>() as libc::c_ulong,
        );
        gettimeofday(&mut stats.time_start, 0 as *mut libc::c_void);
    }
    let mut has_jit: libc::c_int = 0 as libc::c_int;
    pcre_config(9 as libc::c_int, &mut has_jit as *mut libc::c_int as *mut libc::c_void);
    if has_jit != 0 {
        study_opts |= 0x1 as libc::c_int;
    }
    num_cores = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    workers_len = if num_cores < 8 as libc::c_int {
        num_cores
    } else {
        8 as libc::c_int
    };
    if opts.literal != 0 {
        workers_len -= 1;
        workers_len;
    }
    if opts.workers != 0 {
        workers_len = opts.workers;
    }
    if workers_len < 1 as libc::c_int {
        workers_len = 1 as libc::c_int;
    }
    log_debug(b"Using %i workers\0" as *const u8 as *const libc::c_char, workers_len);
    done_adding_files = 0 as libc::c_int;
    workers = ag_calloc(
        workers_len as size_t,
        ::std::mem::size_of::<worker_t>() as libc::c_ulong,
    ) as *mut worker_t;
    if pthread_cond_init(&mut files_ready, 0 as *const pthread_condattr_t) != 0 {
        die(b"pthread_cond_init failed!\0" as *const u8 as *const libc::c_char);
    }
    if pthread_mutex_init(&mut print_mtx, 0 as *const pthread_mutexattr_t) != 0 {
        die(b"pthread_mutex_init failed!\0" as *const u8 as *const libc::c_char);
    }
    if opts.stats != 0
        && pthread_mutex_init(&mut stats_mtx, 0 as *const pthread_mutexattr_t) != 0
    {
        die(b"pthread_mutex_init failed!\0" as *const u8 as *const libc::c_char);
    }
    if pthread_mutex_init(&mut work_queue_mtx, 0 as *const pthread_mutexattr_t) != 0 {
        die(b"pthread_mutex_init failed!\0" as *const u8 as *const libc::c_char);
    }
    if opts.casing as libc::c_uint == CASE_SMART as libc::c_int as libc::c_uint {
        opts
            .casing = (if is_lowercase(opts.query) != 0 {
            CASE_INSENSITIVE as libc::c_int
        } else {
            CASE_SENSITIVE as libc::c_int
        }) as case_behavior;
    }
    if opts.literal != 0 {
        if opts.casing as libc::c_uint == CASE_INSENSITIVE as libc::c_int as libc::c_uint
        {
            let mut c: *mut libc::c_char = opts.query;
            while *c as libc::c_int != '\0' as i32 {
                *c = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *c as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*c as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*c as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
                c = c.offset(1);
                c;
            }
        }
        generate_alpha_skip(
            opts.query,
            opts.query_len as size_t,
            alpha_skip_lookup.as_mut_ptr(),
            (opts.casing as libc::c_uint
                == CASE_SENSITIVE as libc::c_int as libc::c_uint) as libc::c_int,
        );
        find_skip_lookup = 0 as *mut size_t;
        generate_find_skip(
            opts.query,
            opts.query_len as size_t,
            &mut find_skip_lookup,
            (opts.casing as libc::c_uint
                == CASE_SENSITIVE as libc::c_int as libc::c_uint) as libc::c_int,
        );
        generate_hash(
            opts.query,
            opts.query_len as size_t,
            h_table.as_mut_ptr(),
            (opts.casing as libc::c_uint
                == CASE_SENSITIVE as libc::c_int as libc::c_uint) as libc::c_int,
        );
        if opts.word_regexp != 0 {
            init_wordchar_table();
            opts
                .literal_starts_wordchar = is_wordchar(
                *(opts.query).offset(0 as libc::c_int as isize),
            );
            opts
                .literal_ends_wordchar = is_wordchar(
                *(opts.query).offset((opts.query_len - 1 as libc::c_int) as isize),
            );
        }
    } else {
        if opts.casing as libc::c_uint == CASE_INSENSITIVE as libc::c_int as libc::c_uint
        {
            pcre_opts |= 0x1 as libc::c_int;
        }
        if opts.word_regexp != 0 {
            let mut word_regexp_query: *mut libc::c_char = 0 as *mut libc::c_char;
            ag_asprintf(
                &mut word_regexp_query as *mut *mut libc::c_char,
                b"\\b(?:%s)\\b\0" as *const u8 as *const libc::c_char,
                opts.query,
            );
            free(opts.query as *mut libc::c_void);
            opts.query = word_regexp_query;
            opts.query_len = strlen(opts.query) as libc::c_int;
        }
        compile_study(
            &mut opts.re,
            &mut opts.re_extra,
            opts.query,
            pcre_opts,
            study_opts,
        );
    }
    if opts.search_stream != 0 {
        search_stream(stdin, b"\0" as *const u8 as *const libc::c_char);
    } else {
        i = 0 as libc::c_int;
        while i < workers_len {
            (*workers.offset(i as isize)).id = i;
            let mut rv: libc::c_int = pthread_create(
                &mut (*workers.offset(i as isize)).thread,
                0 as *const pthread_attr_t,
                Some(
                    search_file_worker
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                &mut (*workers.offset(i as isize)).id as *mut libc::c_int
                    as *mut libc::c_void,
            );
            if rv != 0 as libc::c_int {
                die(
                    b"Error in pthread_create(): %s\0" as *const u8
                        as *const libc::c_char,
                    strerror(rv),
                );
            }
            if opts.use_thread_affinity != 0 {
                let mut cpu_set: cpu_set_t = cpu_set_t { __bits: [0; 16] };
                libc::memset(
                    &mut cpu_set as *mut cpu_set_t as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
                );
                let mut __cpu: size_t = (i % num_cores) as size_t;
                if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong)
                    < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
                {
                    let ref mut fresh0 = *(cpu_set.__bits)
                        .as_mut_ptr()
                        .offset(
                            __cpu
                                .wrapping_div(
                                    (8 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                        ),
                                ) as isize,
                        );
                    *fresh0
                        |= (1 as libc::c_int as __cpu_mask)
                            << __cpu
                                .wrapping_rem(
                                    (8 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                        ),
                                );
                } else {};
                rv = pthread_setaffinity_np(
                    (*workers.offset(i as isize)).thread,
                    ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
                    &mut cpu_set,
                );
                if rv != 0 {
                    log_err(
                        b"Error in pthread_setaffinity_np(): %s\0" as *const u8
                            as *const libc::c_char,
                        strerror(rv),
                    );
                    log_err(
                        b"Performance may be affected. Use --noaffinity to suppress this message.\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    log_debug(
                        b"Thread %i set to CPU %i\0" as *const u8 as *const libc::c_char,
                        i,
                        i,
                    );
                }
            } else {
                log_debug(
                    b"Thread affinity disabled.\0" as *const u8 as *const libc::c_char,
                );
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while !(*paths.offset(i as isize)).is_null() {
            log_debug(
                b"searching path %s for %s\0" as *const u8 as *const libc::c_char,
                *paths.offset(i as isize),
                opts.query,
            );
            symhash = 0 as *mut symdir_t;
            let mut ig: *mut ignores = init_ignore(
                root_ignores,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
            let mut s: stat = {
                let mut init = stat {
                    st_dev: 0 as libc::c_int as __dev_t,
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
                init
            };
            if opts.one_dev != 0
                && lstat(*paths.offset(i as isize), &mut s) == -(1 as libc::c_int)
            {
                log_err(
                    b"Failed to get device information for path %s. Skipping...\0"
                        as *const u8 as *const libc::c_char,
                    *paths.offset(i as isize),
                );
            }
            search_dir(
                ig,
                *base_paths.offset(i as isize),
                *paths.offset(i as isize),
                0 as libc::c_int,
                s.st_dev,
            );
            cleanup_ignore(ig);
            i += 1;
            i;
        }
        pthread_mutex_lock(&mut work_queue_mtx);
        done_adding_files = 1 as libc::c_int;
        pthread_cond_broadcast(&mut files_ready);
        pthread_mutex_unlock(&mut work_queue_mtx);
        i = 0 as libc::c_int;
        while i < workers_len {
            if pthread_join(
                (*workers.offset(i as isize)).thread,
                0 as *mut *mut libc::c_void,
            ) != 0
            {
                die(b"pthread_join failed!\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            i;
        }
    }
    if opts.stats != 0 {
        gettimeofday(&mut stats.time_end, 0 as *mut libc::c_void);
        let mut time_diff: libc::c_double = (stats.time_end.tv_sec
            * 1000000 as libc::c_int as libc::c_long + stats.time_end.tv_usec
            - (stats.time_start.tv_sec * 1000000 as libc::c_int as libc::c_long
                + stats.time_start.tv_usec)) as libc::c_double;
        time_diff /= 1000000 as libc::c_int as libc::c_double;
        printf(
            b"%zu matches\n%zu files contained matches\n%zu files searched\n%zu bytes searched\n%f seconds\n\0"
                as *const u8 as *const libc::c_char,
            stats.total_matches,
            stats.total_file_matches,
            stats.total_files,
            stats.total_bytes,
            time_diff,
        );
        pthread_mutex_destroy(&mut stats_mtx);
    }
    if !(opts.pager).is_null() {
        pclose(out_fd);
    }
    cleanup_options();
    pthread_cond_destroy(&mut files_ready);
    pthread_mutex_destroy(&mut work_queue_mtx);
    pthread_mutex_destroy(&mut print_mtx);
    cleanup_ignore(root_ignores);
    free(workers as *mut libc::c_void);
    i = 0 as libc::c_int;
    while !(*paths.offset(i as isize)).is_null() {
        free(*paths.offset(i as isize) as *mut libc::c_void);
        free(*base_paths.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(base_paths as *mut libc::c_void);
    free(paths as *mut libc::c_void);
    if !find_skip_lookup.is_null() {
        free(find_skip_lookup as *mut libc::c_void);
    }
    return (opts.match_found == 0) as libc::c_int;
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
