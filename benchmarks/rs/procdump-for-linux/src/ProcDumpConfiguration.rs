use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn scandir(
        __dir: *const libc::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: Option::<unsafe extern "C" fn(*const dirent) -> libc::c_int>,
        __cmp: Option::<
            unsafe extern "C" fn(*mut *const dirent, *mut *const dirent) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn alphasort(__e1: *mut *const dirent, __e2: *mut *const dirent) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sem_init(
        __sem: *mut sem_t,
        __pshared: libc::c_int,
        __value: libc::c_uint,
    ) -> libc::c_int;
    fn sem_destroy(__sem: *mut sem_t) -> libc::c_int;
    fn InitNamedEvent(
        Event: *mut Event,
        IsManualReset: bool,
        InitialState: bool,
        Name: *mut libc::c_char,
    );
    fn DestroyEvent(Event: *mut Event);
    fn SetEvent(Event: *mut Event) -> bool;
    fn Log(logLevel: LogLevel, message: *const libc::c_char, _: ...);
    fn DiagTrace(message: *const libc::c_char, _: ...);
    fn WaitForSingleObject(
        Handle: *mut Handle,
        Milliseconds: libc::c_int,
    ) -> libc::c_int;
    fn WaitForMultipleObjects(
        Count: libc::c_int,
        Handles: *mut *mut Handle,
        WaitAll: bool,
        Milliseconds: libc::c_int,
    ) -> libc::c_int;
    fn ptrace(__request: __ptrace_request, _: ...) -> libc::c_long;
    fn CommitMonitoringThread(thread_args: *mut libc::c_void) -> *mut libc::c_void;
    fn CpuMonitoringThread(thread_args: *mut libc::c_void) -> *mut libc::c_void;
    fn ThreadCountMonitoringThread(thread_args: *mut libc::c_void) -> *mut libc::c_void;
    fn FileDescriptorCountMonitoringThread(
        thread_args: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn SignalMonitoringThread(thread_args: *mut libc::c_void) -> *mut libc::c_void;
    fn TimerThread(thread_args: *mut libc::c_void) -> *mut libc::c_void;
    static mut LoggerLock: pthread_mutex_t;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __kernel_long_t = libc::c_long;
pub type __kernel_ulong_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub __domainname: [libc::c_char; 65],
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
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
pub type C2RustUnnamed_5 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_5 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_5 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_5 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_5 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_5 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_5 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_5 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub bTriggered: bool,
    pub bManualReset: bool,
    pub Name: [libc::c_char; 64],
    pub nWaiters: libc::c_int,
}
pub type LogLevel = libc::c_uint;
pub const error: LogLevel = 4;
pub const crit: LogLevel = 3;
pub const warn: LogLevel = 2;
pub const info: LogLevel = 1;
pub const debug: LogLevel = 0;
pub type EHandleType = libc::c_uint;
pub const SEMAPHORE: EHandleType = 1;
pub const EVENT: EHandleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Handle {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub type_0: EHandleType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub event: Event,
    pub semaphore: sem_t,
}
pub type __ptrace_request = libc::c_uint;
pub const PTRACE_GET_SYSCALL_INFO: __ptrace_request = 16910;
pub const PTRACE_SECCOMP_GET_METADATA: __ptrace_request = 16909;
pub const PTRACE_SECCOMP_GET_FILTER: __ptrace_request = 16908;
pub const PTRACE_SETSIGMASK: __ptrace_request = 16907;
pub const PTRACE_GETSIGMASK: __ptrace_request = 16906;
pub const PTRACE_PEEKSIGINFO: __ptrace_request = 16905;
pub const PTRACE_LISTEN: __ptrace_request = 16904;
pub const PTRACE_INTERRUPT: __ptrace_request = 16903;
pub const PTRACE_SEIZE: __ptrace_request = 16902;
pub const PTRACE_SETREGSET: __ptrace_request = 16901;
pub const PTRACE_GETREGSET: __ptrace_request = 16900;
pub const PTRACE_SETSIGINFO: __ptrace_request = 16899;
pub const PTRACE_GETSIGINFO: __ptrace_request = 16898;
pub const PTRACE_GETEVENTMSG: __ptrace_request = 16897;
pub const PTRACE_SETOPTIONS: __ptrace_request = 16896;
pub const PTRACE_SINGLEBLOCK: __ptrace_request = 33;
pub const PTRACE_SYSEMU_SINGLESTEP: __ptrace_request = 32;
pub const PTRACE_SYSEMU: __ptrace_request = 31;
pub const PTRACE_ARCH_PRCTL: __ptrace_request = 30;
pub const PTRACE_SET_THREAD_AREA: __ptrace_request = 26;
pub const PTRACE_GET_THREAD_AREA: __ptrace_request = 25;
pub const PTRACE_SYSCALL: __ptrace_request = 24;
pub const PTRACE_SETFPXREGS: __ptrace_request = 19;
pub const PTRACE_GETFPXREGS: __ptrace_request = 18;
pub const PTRACE_DETACH: __ptrace_request = 17;
pub const PTRACE_ATTACH: __ptrace_request = 16;
pub const PTRACE_SETFPREGS: __ptrace_request = 15;
pub const PTRACE_GETFPREGS: __ptrace_request = 14;
pub const PTRACE_SETREGS: __ptrace_request = 13;
pub const PTRACE_GETREGS: __ptrace_request = 12;
pub const PTRACE_SINGLESTEP: __ptrace_request = 9;
pub const PTRACE_KILL: __ptrace_request = 8;
pub const PTRACE_CONT: __ptrace_request = 7;
pub const PTRACE_POKEUSER: __ptrace_request = 6;
pub const PTRACE_POKEDATA: __ptrace_request = 5;
pub const PTRACE_POKETEXT: __ptrace_request = 4;
pub const PTRACE_PEEKUSER: __ptrace_request = 3;
pub const PTRACE_PEEKDATA: __ptrace_request = 2;
pub const PTRACE_PEEKTEXT: __ptrace_request = 1;
pub const PTRACE_TRACEME: __ptrace_request = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProcDumpConfiguration {
    pub ProcessId: pid_t,
    pub ProcessName: *mut libc::c_char,
    pub SystemInfo: sysinfo,
    pub NumberOfDumpsCollecting: libc::c_int,
    pub NumberOfDumpsCollected: libc::c_int,
    pub bTerminated: bool,
    pub nQuit: libc::c_int,
    pub evtQuit: Handle,
    pub bTriggerThenSnoozeCPU: bool,
    pub bTriggerThenSnoozeMemory: bool,
    pub bTriggerThenSnoozeTimer: bool,
    pub CpuThreshold: libc::c_int,
    pub bCpuTriggerBelowValue: bool,
    pub MemoryThreshold: libc::c_int,
    pub bMemoryTriggerBelowValue: bool,
    pub ThresholdSeconds: libc::c_int,
    pub bTimerThreshold: bool,
    pub NumberOfDumpsToCollect: libc::c_int,
    pub WaitingForProcessName: bool,
    pub DiagnosticsLoggingEnabled: bool,
    pub ThreadThreshold: libc::c_int,
    pub FileDescriptorThreshold: libc::c_int,
    pub SignalNumber: libc::c_int,
    pub PollingInterval: libc::c_int,
    pub CoreDumpPath: *mut libc::c_char,
    pub CoreDumpName: *mut libc::c_char,
    pub nThreads: libc::c_int,
    pub Threads: [pthread_t; 3],
    pub semAvailableDumpSlots: Handle,
    pub evtCtrlHandlerCleanupComplete: Handle,
    pub evtBannerPrinted: Handle,
    pub evtConfigurationPrinted: Handle,
    pub evtDebugThreadInitialized: Handle,
    pub evtStartMonitoring: Handle,
    pub gcorePid: pid_t,
}
pub static mut g_evtConfigurationInitialized: Handle = unsafe {
    {
        let mut init = Handle {
            c2rust_unnamed: C2RustUnnamed_6 {
                event: {
                    let mut init = Event {
                        mutex: pthread_mutex_t {
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
                        },
                        cond: pthread_cond_t {
                            __data: {
                                let mut init = __pthread_cond_s {
                                    c2rust_unnamed: C2RustUnnamed_1 {
                                        __wseq: 0 as libc::c_int as libc::c_ulonglong,
                                    },
                                    c2rust_unnamed_0: C2RustUnnamed {
                                        __g1_start: 0 as libc::c_int as libc::c_ulonglong,
                                    },
                                    __g_refs: [
                                        0 as libc::c_int as libc::c_uint,
                                        0 as libc::c_int as libc::c_uint,
                                    ],
                                    __g_size: [
                                        0 as libc::c_int as libc::c_uint,
                                        0 as libc::c_int as libc::c_uint,
                                    ],
                                    __g1_orig_size: 0 as libc::c_int as libc::c_uint,
                                    __wrefs: 0 as libc::c_int as libc::c_uint,
                                    __g_signals: [
                                        0 as libc::c_int as libc::c_uint,
                                        0 as libc::c_int as libc::c_uint,
                                    ],
                                };
                                init
                            },
                        },
                        bTriggered: 0 as libc::c_int != 0,
                        bManualReset: 1 as libc::c_int != 0,
                        Name: *::std::mem::transmute::<
                            &[u8; 64],
                            &mut [libc::c_char; 64],
                        >(
                            b"ConfigurationInitialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                        ),
                        nWaiters: 0 as libc::c_int,
                    };
                    init
                },
            },
            type_0: EVENT,
        };
        init
    }
};
static mut sig_set: sigset_t = sigset_t { __val: [0; 16] };
static mut sig_thread_id: pthread_t = 0;
static mut sig_monitor_thread_id: pthread_t = 0;
pub static mut HZ: libc::c_long = 0;
pub static mut MAXIMUM_CPU: libc::c_int = 0;
pub static mut g_config: ProcDumpConfiguration = ProcDumpConfiguration {
    ProcessId: 0,
    ProcessName: 0 as *const libc::c_char as *mut libc::c_char,
    SystemInfo: sysinfo {
        uptime: 0,
        loads: [0; 3],
        totalram: 0,
        freeram: 0,
        sharedram: 0,
        bufferram: 0,
        totalswap: 0,
        freeswap: 0,
        procs: 0,
        pad: 0,
        totalhigh: 0,
        freehigh: 0,
        mem_unit: 0,
        _f: [0; 0],
    },
    NumberOfDumpsCollecting: 0,
    NumberOfDumpsCollected: 0,
    bTerminated: false,
    nQuit: 0,
    evtQuit: Handle {
        c2rust_unnamed: C2RustUnnamed_6 {
            event: Event {
                mutex: pthread_mutex_t {
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
                cond: pthread_cond_t {
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
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    bTriggerThenSnoozeCPU: false,
    bTriggerThenSnoozeMemory: false,
    bTriggerThenSnoozeTimer: false,
    CpuThreshold: 0,
    bCpuTriggerBelowValue: false,
    MemoryThreshold: 0,
    bMemoryTriggerBelowValue: false,
    ThresholdSeconds: 0,
    bTimerThreshold: false,
    NumberOfDumpsToCollect: 0,
    WaitingForProcessName: false,
    DiagnosticsLoggingEnabled: false,
    ThreadThreshold: 0,
    FileDescriptorThreshold: 0,
    SignalNumber: 0,
    PollingInterval: 0,
    CoreDumpPath: 0 as *const libc::c_char as *mut libc::c_char,
    CoreDumpName: 0 as *const libc::c_char as *mut libc::c_char,
    nThreads: 0,
    Threads: [0; 3],
    semAvailableDumpSlots: Handle {
        c2rust_unnamed: C2RustUnnamed_6 {
            event: Event {
                mutex: pthread_mutex_t {
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
                cond: pthread_cond_t {
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
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtCtrlHandlerCleanupComplete: Handle {
        c2rust_unnamed: C2RustUnnamed_6 {
            event: Event {
                mutex: pthread_mutex_t {
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
                cond: pthread_cond_t {
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
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtBannerPrinted: Handle {
        c2rust_unnamed: C2RustUnnamed_6 {
            event: Event {
                mutex: pthread_mutex_t {
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
                cond: pthread_cond_t {
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
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtConfigurationPrinted: Handle {
        c2rust_unnamed: C2RustUnnamed_6 {
            event: Event {
                mutex: pthread_mutex_t {
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
                cond: pthread_cond_t {
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
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtDebugThreadInitialized: Handle {
        c2rust_unnamed: C2RustUnnamed_6 {
            event: Event {
                mutex: pthread_mutex_t {
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
                cond: pthread_cond_t {
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
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtStartMonitoring: Handle {
        c2rust_unnamed: C2RustUnnamed_6 {
            event: Event {
                mutex: pthread_mutex_t {
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
                cond: pthread_cond_t {
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
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    gcorePid: 0,
};
pub static mut ptrace_mutex: pthread_mutex_t = pthread_mutex_t {
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
pub unsafe extern "C" fn SignalThread(
    mut input: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = input as *mut ProcDumpConfiguration;
    let mut sig_caught: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    rc = sigwait(&mut sig_set, &mut sig_caught);
    if rc != 0 as libc::c_int {
        Log(error, b"Failed to wait on signal\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    match sig_caught {
        2 => {
            SetQuit(config, 1 as libc::c_int);
            if (*config).gcorePid != 2147483647 as libc::c_int {
                Log(info, b"Shutting down gcore\0" as *const u8 as *const libc::c_char);
                rc = kill(-(*config).gcorePid, 9 as libc::c_int);
                if rc != 0 as libc::c_int {
                    Log(
                        error,
                        b"Failed to shutdown gcore.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            if (*config).SignalNumber != -(1 as libc::c_int) {
                pthread_mutex_lock(&mut ptrace_mutex);
                ptrace(
                    PTRACE_DETACH,
                    (*config).ProcessId,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                pthread_mutex_unlock(&mut ptrace_mutex);
                rc = pthread_cancel(sig_monitor_thread_id);
                if rc != 0 as libc::c_int {
                    Log(
                        error,
                        b"An error occurred while canceling SignalMonitorThread.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(-(1 as libc::c_int));
                }
            }
            Log(info, b"Quit\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            fprintf(
                stderr,
                b"\nUnexpected signal %d\n\0" as *const u8 as *const libc::c_char,
                sig_caught,
            );
        }
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn InitProcDump() {
    openlog(
        b"ProcDump\0" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
        (1 as libc::c_int) << 3 as libc::c_int,
    );
    if CheckKernelVersion() as libc::c_int == 0 as libc::c_int {
        Log(
            error,
            b"Kernel version lower than 3.5+.\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    InitProcDumpConfiguration(&mut g_config);
    pthread_mutex_init(&mut LoggerLock, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut ptrace_mutex, 0 as *const pthread_mutexattr_t);
}
pub unsafe extern "C" fn ExitProcDump() {
    pthread_mutex_destroy(&mut LoggerLock);
    closelog();
    FreeProcDumpConfiguration(&mut g_config);
}
pub unsafe extern "C" fn InitProcDumpConfiguration(
    mut self_0: *mut ProcDumpConfiguration,
) {
    if WaitForSingleObject(&mut g_evtConfigurationInitialized, 0 as libc::c_int)
        == 0 as libc::c_int
    {
        return;
    }
    MAXIMUM_CPU = 100 as libc::c_int
        * sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    HZ = sysconf(_SC_CLK_TCK as libc::c_int);
    sysinfo(&mut (*self_0).SystemInfo);
    InitNamedEvent(
        &mut (*self_0).evtCtrlHandlerCleanupComplete.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"CtrlHandlerCleanupComplete\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).evtCtrlHandlerCleanupComplete.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtBannerPrinted.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"BannerPrinted\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).evtBannerPrinted.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtConfigurationPrinted.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"ConfigurationPrinted\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).evtConfigurationPrinted.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtDebugThreadInitialized.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"DebugThreadInitialized\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).evtDebugThreadInitialized.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtQuit.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"Quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).evtQuit.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtStartMonitoring.c2rust_unnamed.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"StartMonitoring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).evtStartMonitoring.type_0 = EVENT;
    sem_init(
        &mut (*self_0).semAvailableDumpSlots.c2rust_unnamed.semaphore,
        0 as libc::c_int,
        1 as libc::c_int as libc::c_uint,
    );
    (*self_0).semAvailableDumpSlots.type_0 = SEMAPHORE;
    (*self_0).ProcessId = 2147483647 as libc::c_int;
    (*self_0).NumberOfDumpsCollected = 0 as libc::c_int;
    (*self_0).NumberOfDumpsToCollect = 1 as libc::c_int;
    (*self_0).CpuThreshold = -(1 as libc::c_int);
    (*self_0).MemoryThreshold = -(1 as libc::c_int);
    (*self_0).ThreadThreshold = -(1 as libc::c_int);
    (*self_0).FileDescriptorThreshold = -(1 as libc::c_int);
    (*self_0).SignalNumber = -(1 as libc::c_int);
    (*self_0).ThresholdSeconds = 10 as libc::c_int;
    (*self_0).bCpuTriggerBelowValue = 0 as libc::c_int != 0;
    (*self_0).bMemoryTriggerBelowValue = 0 as libc::c_int != 0;
    (*self_0).bTimerThreshold = 0 as libc::c_int != 0;
    (*self_0).WaitingForProcessName = 0 as libc::c_int != 0;
    (*self_0).DiagnosticsLoggingEnabled = 0 as libc::c_int != 0;
    (*self_0).gcorePid = 2147483647 as libc::c_int;
    (*self_0).PollingInterval = 1000 as libc::c_int;
    (*self_0).CoreDumpPath = 0 as *mut libc::c_char;
    (*self_0).CoreDumpName = 0 as *mut libc::c_char;
    SetEvent(&mut g_evtConfigurationInitialized.c2rust_unnamed.event);
}
pub unsafe extern "C" fn FreeProcDumpConfiguration(
    mut self_0: *mut ProcDumpConfiguration,
) {
    DestroyEvent(&mut (*self_0).evtCtrlHandlerCleanupComplete.c2rust_unnamed.event);
    DestroyEvent(&mut (*self_0).evtBannerPrinted.c2rust_unnamed.event);
    DestroyEvent(&mut (*self_0).evtConfigurationPrinted.c2rust_unnamed.event);
    DestroyEvent(&mut (*self_0).evtDebugThreadInitialized.c2rust_unnamed.event);
    DestroyEvent(&mut (*self_0).evtQuit.c2rust_unnamed.event);
    DestroyEvent(&mut (*self_0).evtStartMonitoring.c2rust_unnamed.event);
    sem_destroy(&mut (*self_0).semAvailableDumpSlots.c2rust_unnamed.semaphore);
    if strcmp((*self_0).ProcessName, b"null\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        free((*self_0).ProcessName as *mut libc::c_void);
    }
    free((*self_0).CoreDumpPath as *mut libc::c_void);
    free((*self_0).CoreDumpName as *mut libc::c_void);
}
pub unsafe extern "C" fn GetOptions(
    mut self_0: *mut ProcDumpConfiguration,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if WaitForSingleObject(&mut g_evtConfigurationInitialized, 0 as libc::c_int)
        != 0 as libc::c_int
    {
        DiagTrace(
            b"GetOptions: Configuration not initialized. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 202\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if argc < 2 as libc::c_int {
        DiagTrace(
            b"GetOptions: Invalid number of command line arguments. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 207\0" as *const u8
                as *const libc::c_char,
        );
        return PrintUsage(self_0);
    }
    let mut next_option: libc::c_int = 0;
    let mut option_index: libc::c_int = 0 as libc::c_int;
    let mut short_options: *const libc::c_char = b"+p:C:c:M:m:n:s:w:T:F:G:I:o:dh\0"
        as *const u8 as *const libc::c_char;
    let long_options: [option; 16] = [
        {
            let mut init = option {
                name: b"pid\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"cpu\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lower-cpu\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"memory\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lower-mem\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"number-of-dumps\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"time-between-dumps\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"wait\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'w' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"threads\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"filedescriptors\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"signal\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pollinginterval\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"output-path\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"diag\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut tempOutputPath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statbuf: stat = stat {
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
    loop {
        next_option = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            short_options,
            long_options.as_ptr(),
            &mut option_index,
        );
        if !(next_option != -(1 as libc::c_int)) {
            break;
        }
        match next_option {
            112 => {
                (*self_0).ProcessId = atoi(optarg);
                if !LookupProcessByPid(self_0) {
                    Log(
                        error,
                        b"Invalid PID - failed looking up process name by PID.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            67 => {
                if (*self_0).CpuThreshold != -(1 as libc::c_int)
                    || !IsValidNumberArg(optarg)
                    || {
                        (*self_0).CpuThreshold = atoi(optarg);
                        (*self_0).CpuThreshold < 0 as libc::c_int
                    } || (*self_0).CpuThreshold > MAXIMUM_CPU
                {
                    Log(
                        error,
                        b"Invalid CPU threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            73 => {
                if !IsValidNumberArg(optarg)
                    || {
                        (*self_0).PollingInterval = atoi(optarg);
                        (*self_0).PollingInterval < 0 as libc::c_int
                    } || (*self_0).PollingInterval < 1000 as libc::c_int
                {
                    Log(
                        error,
                        b"Invalid polling interval specified (minimum %d).\0"
                            as *const u8 as *const libc::c_char,
                        1000 as libc::c_int,
                    );
                    return PrintUsage(self_0);
                }
            }
            84 => {
                if (*self_0).ThreadThreshold != -(1 as libc::c_int)
                    || !IsValidNumberArg(optarg)
                    || {
                        (*self_0).ThreadThreshold = atoi(optarg);
                        (*self_0).ThreadThreshold < 0 as libc::c_int
                    }
                {
                    Log(
                        error,
                        b"Invalid threads threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            70 => {
                if (*self_0).FileDescriptorThreshold != -(1 as libc::c_int)
                    || !IsValidNumberArg(optarg)
                    || {
                        (*self_0).FileDescriptorThreshold = atoi(optarg);
                        (*self_0).FileDescriptorThreshold < 0 as libc::c_int
                    }
                {
                    Log(
                        error,
                        b"Invalid file descriptor threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            71 => {
                if (*self_0).SignalNumber != -(1 as libc::c_int)
                    || !IsValidNumberArg(optarg)
                    || {
                        (*self_0).SignalNumber = atoi(optarg);
                        (*self_0).SignalNumber < 0 as libc::c_int
                    }
                {
                    Log(
                        error,
                        b"Invalid signal specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            99 => {
                if (*self_0).CpuThreshold != -(1 as libc::c_int)
                    || !IsValidNumberArg(optarg)
                    || {
                        (*self_0).CpuThreshold = atoi(optarg);
                        (*self_0).CpuThreshold < 0 as libc::c_int
                    } || (*self_0).CpuThreshold > MAXIMUM_CPU
                {
                    Log(
                        error,
                        b"Invalid CPU threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
                (*self_0).bCpuTriggerBelowValue = 1 as libc::c_int != 0;
            }
            77 => {
                if (*self_0).MemoryThreshold != -(1 as libc::c_int)
                    || !IsValidNumberArg(optarg)
                    || {
                        (*self_0).MemoryThreshold = atoi(optarg);
                        (*self_0).MemoryThreshold < 0 as libc::c_int
                    }
                {
                    Log(
                        error,
                        b"Invalid memory threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            109 => {
                if (*self_0).MemoryThreshold != -(1 as libc::c_int)
                    || !IsValidNumberArg(optarg)
                    || {
                        (*self_0).MemoryThreshold = atoi(optarg);
                        (*self_0).MemoryThreshold < 0 as libc::c_int
                    }
                {
                    Log(
                        error,
                        b"Invalid memory threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
                (*self_0).bMemoryTriggerBelowValue = 1 as libc::c_int != 0;
            }
            110 => {
                if !IsValidNumberArg(optarg)
                    || {
                        (*self_0).NumberOfDumpsToCollect = atoi(optarg);
                        (*self_0).NumberOfDumpsToCollect < 0 as libc::c_int
                    }
                {
                    Log(
                        error,
                        b"Invalid dumps threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            115 => {
                if !IsValidNumberArg(optarg)
                    || {
                        (*self_0).ThresholdSeconds = atoi(optarg);
                        (*self_0).ThresholdSeconds == 0 as libc::c_int
                    }
                {
                    Log(
                        error,
                        b"Invalid time threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return PrintUsage(self_0);
                }
            }
            119 => {
                (*self_0).WaitingForProcessName = 1 as libc::c_int != 0;
                (*self_0).ProcessName = strdup(optarg);
            }
            111 => {
                tempOutputPath = strdup(optarg);
                if stat(tempOutputPath, &mut statbuf) == 0 as libc::c_int
                    && statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    || *tempOutputPath
                        .offset(
                            (strlen(tempOutputPath))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '/' as i32
                {
                    (*self_0).CoreDumpPath = tempOutputPath;
                    (*self_0).CoreDumpName = 0 as *mut libc::c_char;
                } else {
                    (*self_0).CoreDumpPath = strdup(dirname(tempOutputPath));
                    free(tempOutputPath as *mut libc::c_void);
                    tempOutputPath = strdup(optarg);
                    (*self_0).CoreDumpName = strdup(__xpg_basename(tempOutputPath));
                    free(tempOutputPath as *mut libc::c_void);
                }
                if stat((*self_0).CoreDumpPath, &mut statbuf) < 0 as libc::c_int
                    || !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                {
                    Log(
                        error,
                        b"Invalid directory (\"%s\") provided for core dump output.\0"
                            as *const u8 as *const libc::c_char,
                        (*self_0).CoreDumpPath,
                    );
                    return PrintUsage(self_0);
                }
            }
            100 => {
                (*self_0).DiagnosticsLoggingEnabled = 1 as libc::c_int != 0;
            }
            104 => return PrintUsage(self_0),
            _ => {
                Log(
                    error,
                    b"Invalid switch specified\0" as *const u8 as *const libc::c_char,
                );
                return PrintUsage(self_0);
            }
        }
    }
    if ((*self_0).CoreDumpPath).is_null() {
        (*self_0).CoreDumpPath = strdup(b".\0" as *const u8 as *const libc::c_char);
    }
    if (*self_0).NumberOfDumpsToCollect != -(1 as libc::c_int)
        && (*self_0).MemoryThreshold == -(1 as libc::c_int)
        && (*self_0).CpuThreshold == -(1 as libc::c_int)
        && (*self_0).ThreadThreshold == -(1 as libc::c_int)
        && (*self_0).FileDescriptorThreshold == -(1 as libc::c_int)
    {
        (*self_0).bTimerThreshold = 1 as libc::c_int != 0;
    }
    if (*self_0).SignalNumber != -(1 as libc::c_int) {
        if (*self_0).CpuThreshold != -(1 as libc::c_int)
            || (*self_0).ThreadThreshold != -(1 as libc::c_int)
            || (*self_0).FileDescriptorThreshold != -(1 as libc::c_int)
            || (*self_0).MemoryThreshold != -(1 as libc::c_int)
        {
            Log(
                error,
                b"Signal trigger must be the only trigger specified.\0" as *const u8
                    as *const libc::c_char,
            );
            return PrintUsage(self_0);
        }
        if (*self_0).PollingInterval != 1000 as libc::c_int {
            Log(
                error,
                b"Polling interval has no meaning during signal monitoring.\0"
                    as *const u8 as *const libc::c_char,
            );
            return PrintUsage(self_0);
        }
        (*self_0).bTimerThreshold = 0 as libc::c_int != 0;
    }
    if (*self_0).ProcessId == 2147483647 as libc::c_int
        && !(*self_0).WaitingForProcessName
    {
        Log(
            error,
            b"A valid PID or process name must be specified\0" as *const u8
                as *const libc::c_char,
        );
        return PrintUsage(self_0);
    }
    if (*self_0).ProcessId != 2147483647 as libc::c_int
        && (*self_0).WaitingForProcessName as libc::c_int != 0
    {
        Log(
            error,
            b"Please only specify one of -p or -w\0" as *const u8 as *const libc::c_char,
        );
        return PrintUsage(self_0);
    }
    if !(*self_0).WaitingForProcessName {
        (*self_0).ProcessName = GetProcessName((*self_0).ProcessId);
    }
    DiagTrace(
        b"GetOpts and initial Configuration finished %s\0" as *const u8
            as *const libc::c_char,
        b"in src/ProcDumpConfiguration.c, at line 429\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn LookupProcessByPid(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    let mut statFilePath: [libc::c_char; 32] = [0; 32];
    sprintf(
        statFilePath.as_mut_ptr(),
        b"/proc/%d/stat\0" as *const u8 as *const libc::c_char,
        (*self_0).ProcessId,
    );
    let mut fd: *mut FILE = fopen(
        statFilePath.as_mut_ptr(),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fd.is_null() {
        Log(
            error,
            b"No process matching the specified PID can be found.\0" as *const u8
                as *const libc::c_char,
        );
        Log(
            error,
            b"Try elevating the command prompt (i.e., `sudo procdump ...`)\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    fclose(fd);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn FilterForPid(mut entry: *const dirent) -> libc::c_int {
    return IsValidNumberArg(((*entry).d_name).as_ptr()) as libc::c_int;
}
pub unsafe extern "C" fn WaitForProcessName(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    Log(
        info,
        b"Waiting for process '%s' to launch...\0" as *const u8 as *const libc::c_char,
        (*self_0).ProcessName,
    );
    loop {
        let mut nameList: *mut *mut dirent = 0 as *mut *mut dirent;
        let mut moreThanOne: bool = 0 as libc::c_int != 0;
        let mut matchingPid: pid_t = 2147483647 as libc::c_int;
        let mut numEntries: libc::c_int = scandir(
            b"/proc/\0" as *const u8 as *const libc::c_char,
            &mut nameList,
            Some(FilterForPid as unsafe extern "C" fn(*const dirent) -> libc::c_int),
            Some(
                alphasort
                    as unsafe extern "C" fn(
                        *mut *const dirent,
                        *mut *const dirent,
                    ) -> libc::c_int,
            ),
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < numEntries {
            let mut procPid: pid_t = atoi(
                ((**nameList.offset(i as isize)).d_name).as_mut_ptr(),
            );
            let mut nameForPid: *mut libc::c_char = GetProcessName(procPid);
            if !(strcmp(nameForPid, b"null\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
            {
                if strcmp(nameForPid, (*self_0).ProcessName) == 0 as libc::c_int {
                    if matchingPid == 2147483647 as libc::c_int {
                        matchingPid = procPid;
                    } else {
                        Log(
                            error,
                            b"More than one matching process found, exiting...\0"
                                as *const u8 as *const libc::c_char,
                        );
                        moreThanOne = 1 as libc::c_int != 0;
                        free(nameForPid as *mut libc::c_void);
                        break;
                    }
                }
                free(nameForPid as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < numEntries {
            free(*nameList.offset(i_0 as isize) as *mut libc::c_void);
            i_0 += 1;
            i_0;
        }
        free(nameList as *mut libc::c_void);
        if moreThanOne {
            (*self_0).bTerminated = 1 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        } else if matchingPid != 2147483647 as libc::c_int {
            (*self_0).ProcessId = matchingPid;
            Log(
                info,
                b"Found process with PID %d\0" as *const u8 as *const libc::c_char,
                matchingPid,
            );
            return 1 as libc::c_int != 0;
        }
    };
}
pub unsafe extern "C" fn GetProcessName(mut pid: pid_t) -> *mut libc::c_char {
    let mut procFilePath: [libc::c_char; 32] = [0; 32];
    let mut fileBuffer: [libc::c_char; 4097] = [0; 4097];
    let mut charactersRead: libc::c_int = 0 as libc::c_int;
    let mut itr: libc::c_int = 0 as libc::c_int;
    let mut stringItr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut processName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut procFile: *mut FILE = 0 as *mut FILE;
    if sprintf(
        procFilePath.as_mut_ptr(),
        b"/proc/%d/cmdline\0" as *const u8 as *const libc::c_char,
        pid,
    ) < 0 as libc::c_int
    {
        return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    procFile = fopen(
        procFilePath.as_mut_ptr(),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !procFile.is_null() {
        if (fgets(
            fileBuffer.as_mut_ptr(),
            4096 as libc::c_int + 1 as libc::c_int,
            procFile,
        ))
            .is_null()
        {
            fclose(procFile);
            if strlen(fileBuffer.as_mut_ptr()) == 0 as libc::c_int as libc::c_ulong {
                Log(debug, b"Empty cmdline.\n\0" as *const u8 as *const libc::c_char);
            } else {
                Log(
                    debug,
                    b"Failed to read from %s.\n\0" as *const u8 as *const libc::c_char,
                    procFilePath.as_mut_ptr(),
                );
            }
            return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        fclose(procFile);
    } else {
        Log(
            debug,
            b"Failed to open %s.\n\0" as *const u8 as *const libc::c_char,
            procFilePath.as_mut_ptr(),
        );
        return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    stringItr = fileBuffer.as_mut_ptr();
    charactersRead = strlen(fileBuffer.as_mut_ptr()) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= charactersRead {
        if fileBuffer[i as usize] as libc::c_int == '\0' as i32 {
            itr = i - itr;
            if strcmp(stringItr, b"sudo\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                processName = strrchr(stringItr, '/' as i32);
                if !processName.is_null() {
                    return strdup(processName.offset(1 as libc::c_int as isize))
                } else {
                    return strdup(stringItr)
                }
            } else {
                stringItr = stringItr.offset((itr + 1 as libc::c_int) as isize);
            }
        }
        i += 1;
        i;
    }
    Log(
        debug,
        b"Failed to extract process name from /proc/PID/cmdline\0" as *const u8
            as *const libc::c_char,
    );
    return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn CreateTriggerThreads(
    mut self_0: *mut ProcDumpConfiguration,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    (*self_0).nThreads = 0 as libc::c_int;
    rc = sigemptyset(&mut sig_set);
    if rc < 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: sigemptyset failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 598\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    rc = sigaddset(&mut sig_set, 2 as libc::c_int);
    if rc < 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: sigaddset failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 603\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    rc = sigaddset(&mut sig_set, 15 as libc::c_int);
    if rc < 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: sigaddset failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 608\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    rc = pthread_sigmask(0 as libc::c_int, &mut sig_set, 0 as *mut __sigset_t);
    if rc != 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: pthread_sigmask failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 614\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    if (*self_0).CpuThreshold != -(1 as libc::c_int) {
        let fresh0 = (*self_0).nThreads;
        (*self_0).nThreads = (*self_0).nThreads + 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(fresh0 as isize),
            0 as *const pthread_attr_t,
            Some(
                CpuMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create CpuThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 621\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).MemoryThreshold != -(1 as libc::c_int) {
        let fresh1 = (*self_0).nThreads;
        (*self_0).nThreads = (*self_0).nThreads + 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(fresh1 as isize),
            0 as *const pthread_attr_t,
            Some(
                CommitMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create CommitThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 628\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).ThreadThreshold != -(1 as libc::c_int) {
        let fresh2 = (*self_0).nThreads;
        (*self_0).nThreads = (*self_0).nThreads + 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(fresh2 as isize),
            0 as *const pthread_attr_t,
            Some(
                ThreadCountMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create ThreadThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 635\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).FileDescriptorThreshold != -(1 as libc::c_int) {
        let fresh3 = (*self_0).nThreads;
        (*self_0).nThreads = (*self_0).nThreads + 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(fresh3 as isize),
            0 as *const pthread_attr_t,
            Some(
                FileDescriptorCountMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create FileDescriptorThread. %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 642\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).SignalNumber != -(1 as libc::c_int) {
        rc = pthread_create(
            &mut sig_monitor_thread_id,
            0 as *const pthread_attr_t,
            Some(
                SignalMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create SignalMonitoringThread. %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 649\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).bTimerThreshold {
        let fresh4 = (*self_0).nThreads;
        (*self_0).nThreads = (*self_0).nThreads + 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(fresh4 as isize),
            0 as *const pthread_attr_t,
            Some(
                TimerThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create TimerThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 656\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    rc = pthread_create(
        &mut sig_thread_id,
        0 as *const pthread_attr_t,
        Some(
            SignalThread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        self_0 as *mut libc::c_void,
    );
    if rc != 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: failed to create SignalThread. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 663\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WaitForQuit(
    mut self_0: *mut ProcDumpConfiguration,
    mut milliseconds: libc::c_int,
) -> libc::c_int {
    if !ContinueMonitoring(self_0) {
        return 0x80 as libc::c_int;
    }
    let mut wait: libc::c_int = WaitForSingleObject(
        &mut (*self_0).evtQuit,
        milliseconds,
    );
    if wait == 110 as libc::c_int && !ContinueMonitoring(self_0) {
        return 0x80 as libc::c_int;
    }
    return wait;
}
pub unsafe extern "C" fn WaitForQuitOrEvent(
    mut self_0: *mut ProcDumpConfiguration,
    mut handle: *mut Handle,
    mut milliseconds: libc::c_int,
) -> libc::c_int {
    let mut waits: [*mut Handle; 2] = [0 as *mut Handle; 2];
    waits[0 as libc::c_int as usize] = &mut (*self_0).evtQuit;
    waits[1 as libc::c_int as usize] = handle;
    if !ContinueMonitoring(self_0) {
        return 0x80 as libc::c_int;
    }
    let mut wait: libc::c_int = WaitForMultipleObjects(
        2 as libc::c_int,
        waits.as_mut_ptr(),
        0 as libc::c_int != 0,
        milliseconds,
    );
    if wait == 110 as libc::c_int && !ContinueMonitoring(self_0) {
        return 0x80 as libc::c_int;
    }
    if wait == 0 as libc::c_int && !ContinueMonitoring(self_0) {
        return 0x80 as libc::c_int;
    }
    return wait;
}
pub unsafe extern "C" fn WaitForAllThreadsToTerminate(
    mut self_0: *mut ProcDumpConfiguration,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (*self_0).SignalNumber != -(1 as libc::c_int) {
        rc = pthread_join(sig_monitor_thread_id, 0 as *mut *mut libc::c_void);
        if rc != 0 as libc::c_int {
            Log(
                error,
                b"An error occurred while joining SignalMonitorThread.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*self_0).nThreads {
            rc = pthread_join(
                (*self_0).Threads[i as usize],
                0 as *mut *mut libc::c_void,
            );
            if rc != 0 as libc::c_int {
                Log(
                    error,
                    b"An error occurred while joining threads\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
            i;
        }
    }
    pthread_cancel(sig_thread_id);
    rc = pthread_join(sig_thread_id, 0 as *mut *mut libc::c_void);
    if rc != 0 as libc::c_int {
        Log(
            error,
            b"An error occurred while joining SignalThread.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return rc;
}
pub unsafe extern "C" fn IsQuit(mut self_0: *mut ProcDumpConfiguration) -> bool {
    return (*self_0).nQuit != 0 as libc::c_int;
}
pub unsafe extern "C" fn SetQuit(
    mut self_0: *mut ProcDumpConfiguration,
    mut quit: libc::c_int,
) -> libc::c_int {
    (*self_0).nQuit = quit;
    SetEvent(&mut (*self_0).evtQuit.c2rust_unnamed.event);
    return (*self_0).nQuit;
}
pub unsafe extern "C" fn PrintConfiguration(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    if WaitForSingleObject(&mut (*self_0).evtConfigurationPrinted, 0 as libc::c_int)
        == 110 as libc::c_int
    {
        if (*self_0).SignalNumber != -(1 as libc::c_int) {
            printf(
                b"** NOTE ** Signal triggers use PTRACE which will impact the performance of the target process\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        printf(
            b"Process:\t\t%s\0" as *const u8 as *const libc::c_char,
            (*self_0).ProcessName,
        );
        if !(*self_0).WaitingForProcessName {
            printf(b" (%d)\0" as *const u8 as *const libc::c_char, (*self_0).ProcessId);
        } else {
            printf(b" (pending)\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        if (*self_0).CpuThreshold != -(1 as libc::c_int) {
            if (*self_0).bCpuTriggerBelowValue {
                printf(
                    b"CPU Threshold:\t\t<%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).CpuThreshold,
                );
            } else {
                printf(
                    b"CPU Threshold:\t\t>=%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).CpuThreshold,
                );
            }
        } else {
            printf(b"CPU Threshold:\t\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        if (*self_0).MemoryThreshold != -(1 as libc::c_int) {
            if (*self_0).bMemoryTriggerBelowValue {
                printf(
                    b"Commit Threshold:\t<%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).MemoryThreshold,
                );
            } else {
                printf(
                    b"Commit Threshold:\t>=%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).MemoryThreshold,
                );
            }
        } else {
            printf(b"Commit Threshold:\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        if (*self_0).ThreadThreshold != -(1 as libc::c_int) {
            printf(
                b"Thread Threshold:\t>=%d\n\0" as *const u8 as *const libc::c_char,
                (*self_0).ThreadThreshold,
            );
        } else {
            printf(b"Thread Threshold:\t\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        if (*self_0).FileDescriptorThreshold != -(1 as libc::c_int) {
            printf(
                b"File descriptor Threshold:\t>=%d\n\0" as *const u8
                    as *const libc::c_char,
                (*self_0).FileDescriptorThreshold,
            );
        } else {
            printf(
                b"File descriptor Threshold:\t\tn/a\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*self_0).SignalNumber != -(1 as libc::c_int) {
            printf(
                b"Signal number:\t%d\n\0" as *const u8 as *const libc::c_char,
                (*self_0).SignalNumber,
            );
        } else {
            printf(b"Signal:\t\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"Polling interval (ms):\t%d\n\0" as *const u8 as *const libc::c_char,
            (*self_0).PollingInterval,
        );
        printf(
            b"Threshold (s):\t%d\n\0" as *const u8 as *const libc::c_char,
            (*self_0).ThresholdSeconds,
        );
        printf(
            b"Number of Dumps:\t%d\n\0" as *const u8 as *const libc::c_char,
            (*self_0).NumberOfDumpsToCollect,
        );
        printf(
            b"Output directory for core dumps:\t%s\n\0" as *const u8
                as *const libc::c_char,
            (*self_0).CoreDumpPath,
        );
        if !((*self_0).CoreDumpName).is_null() {
            printf(
                b"Custom name for core dumps:\t%s_<counter>.<pid>\n\0" as *const u8
                    as *const libc::c_char,
                (*self_0).CoreDumpName,
            );
        }
        SetEvent(&mut (*self_0).evtConfigurationPrinted.c2rust_unnamed.event);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn ContinueMonitoring(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    if (*self_0).NumberOfDumpsCollected >= (*self_0).NumberOfDumpsToCollect {
        return 0 as libc::c_int != 0;
    }
    if (*self_0).bTerminated {
        return 0 as libc::c_int != 0;
    }
    if kill((*self_0).ProcessId, 0 as libc::c_int) != 0 {
        (*self_0).bTerminated = 1 as libc::c_int != 0;
        Log(
            error,
            b"Target process is no longer alive\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn BeginMonitoring(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    return SetEvent(&mut (*self_0).evtStartMonitoring.c2rust_unnamed.event);
}
pub unsafe extern "C" fn IsValidNumberArg(mut arg: *const libc::c_char) -> bool {
    let mut strLen: libc::c_int = strlen(arg) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < strLen {
        if *(*__ctype_b_loc()).offset(*arg.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
            && *(*__ctype_b_loc())
                .offset(*arg.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            return 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn CheckKernelVersion() -> bool {
    let mut kernelInfo: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    if uname(&mut kernelInfo) == 0 as libc::c_int {
        let mut version: libc::c_int = 0;
        let mut patch: libc::c_int = 0 as libc::c_int;
        if sscanf(
            (kernelInfo.release).as_mut_ptr(),
            b"%d.%d\0" as *const u8 as *const libc::c_char,
            &mut version as *mut libc::c_int,
            &mut patch as *mut libc::c_int,
        ) != 2 as libc::c_int
        {
            Log(
                error,
                b"Cannot validate kernel version\0" as *const u8 as *const libc::c_char,
            );
            DiagTrace(
                b"%s %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
                b"in src/ProcDumpConfiguration.c, at line 963\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        if version > 3 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        if version == 3 as libc::c_int && patch >= 5 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
    } else {
        Log(error, strerror(*__errno_location()));
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn PrintBanner() {
    printf(
        b"\nProcDump v1.2 - Sysinternals process dump utility\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Copyright (C) 2020 Microsoft Corporation. All rights reserved. Licensed under the MIT license.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Mark Russinovich, Mario Hewardt, John Salem, Javid Habibi\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Monitors a process and writes a dump file when the process meets the\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"specified criteria.\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn PrintUsage(
    mut self_0: *mut ProcDumpConfiguration,
) -> libc::c_int {
    printf(
        b"\nUsage: procdump [OPTIONS...] TARGET\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"   OPTIONS\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      -h          Prints this help screen\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      -C          Trigger core dump generation when CPU exceeds or equals specified value (0 to 100 * nCPU)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -c          Trigger core dump generation when CPU is less than specified value (0 to 100 * nCPU)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -M          Trigger core dump generation when memory commit exceeds or equals specified value (MB)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -m          Trigger core dump generation when when memory commit is less than specified value (MB)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -T          Trigger when thread count exceeds or equals specified value.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -F          Trigger when file descriptor count exceeds or equals specified value.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -G          Trigger when signal with the specified value (num) is sent (uses PTRACE and will affect performance of target process).\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -I          Polling frequency in milliseconds (default is %d)\n\0"
            as *const u8 as *const libc::c_char,
        1000 as libc::c_int,
    );
    printf(
        b"      -n          Number of core dumps to write before exiting (default is %d)\n\0"
            as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    printf(
        b"      -s          Consecutive seconds before dump is written (default is %d)\n\0"
            as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    printf(
        b"      -o          Path and/or filename prefix where the core dump is written to\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -d          Writes diagnostic logs to syslog\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"   TARGET must be exactly one of these:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      -p          pid of the process\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -w          Name of the process executable\n\n\0" as *const u8
            as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
