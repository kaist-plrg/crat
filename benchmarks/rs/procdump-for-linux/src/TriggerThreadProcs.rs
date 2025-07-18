use ::libc;
extern "C" {
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn NewCoreDumpWriter(
        type_0: ECoreDumpType,
        config: *mut ProcDumpConfiguration,
    ) -> *mut CoreDumpWriter;
    fn WaitForQuitOrEvent(
        self_0: *mut ProcDumpConfiguration,
        handle: *mut Handle,
        milliseconds: libc::c_int,
    ) -> libc::c_int;
    fn WaitForQuit(
        self_0: *mut ProcDumpConfiguration,
        milliseconds: libc::c_int,
    ) -> libc::c_int;
    fn GetProcessStat(pid: pid_t, proc_0: *mut ProcessStat) -> bool;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn WriteCoreDump(self_0: *mut CoreDumpWriter) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
    fn ptrace(__request: __ptrace_request, _: ...) -> libc::c_long;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn Log(logLevel: LogLevel, message: *const libc::c_char, _: ...);
    fn DiagTrace(message: *const libc::c_char, _: ...);
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut HZ: libc::c_long;
    static mut ptrace_mutex: pthread_mutex_t;
}
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
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
pub type gid_t = __gid_t;
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
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub type_0: EHandleType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub event: Event,
    pub semaphore: sem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProcessStat {
    pub pid: pid_t,
    pub comm: *mut libc::c_char,
    pub state: libc::c_char,
    pub ppid: pid_t,
    pub pgrp: gid_t,
    pub session: libc::c_int,
    pub tty_nr: libc::c_int,
    pub tpgid: gid_t,
    pub flags: libc::c_uint,
    pub minflt: libc::c_ulong,
    pub cminflt: libc::c_ulong,
    pub majflt: libc::c_ulong,
    pub cmajflt: libc::c_ulong,
    pub utime: libc::c_ulong,
    pub stime: libc::c_ulong,
    pub cutime: libc::c_ulong,
    pub cstime: libc::c_ulong,
    pub priority: libc::c_long,
    pub nice: libc::c_long,
    pub num_threads: libc::c_long,
    pub itrealvalue: libc::c_long,
    pub starttime: libc::c_ulonglong,
    pub vsize: libc::c_ulong,
    pub rss: libc::c_long,
    pub rsslim: libc::c_ulong,
    pub startcode: libc::c_ulong,
    pub endcode: libc::c_ulong,
    pub startstack: libc::c_ulong,
    pub kstkesp: libc::c_ulong,
    pub kstkeip: libc::c_ulong,
    pub signal: libc::c_ulong,
    pub blocked: libc::c_ulong,
    pub sigignore: libc::c_ulong,
    pub sigcatch: libc::c_ulong,
    pub wchan: libc::c_ulong,
    pub nswap: libc::c_ulong,
    pub cnswap: libc::c_ulong,
    pub exit_signal: libc::c_int,
    pub processor: libc::c_int,
    pub rt_priority: libc::c_uint,
    pub policy: libc::c_uint,
    pub delayacct_blkio_ticks: libc::c_ulonglong,
    pub guest_time: libc::c_ulong,
    pub cguest_time: libc::c_long,
    pub start_data: libc::c_ulong,
    pub end_data: libc::c_ulong,
    pub start_brk: libc::c_ulong,
    pub arg_start: libc::c_ulong,
    pub arg_end: libc::c_ulong,
    pub env_start: libc::c_ulong,
    pub env_end: libc::c_ulong,
    pub exit_code: libc::c_int,
    pub num_filedescriptors: libc::c_int,
}
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
pub type ECoreDumpType = libc::c_uint;
pub const MANUAL: ECoreDumpType = 6;
pub const TIME: ECoreDumpType = 5;
pub const SIGNAL: ECoreDumpType = 4;
pub const FILEDESC: ECoreDumpType = 3;
pub const THREAD: ECoreDumpType = 2;
pub const CPU: ECoreDumpType = 1;
pub const COMMIT: ECoreDumpType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoreDumpWriter {
    pub Config: *mut ProcDumpConfiguration,
    pub Type: ECoreDumpType,
}
pub unsafe extern "C" fn CommitMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    DiagTrace(
        b"CommitMonitoringThread: Starting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 16\0" as *const u8 as *const libc::c_char,
    );
    let mut config: *mut ProcDumpConfiguration = thread_args
        as *mut ProcDumpConfiguration;
    let mut pageSize_kb: libc::c_long = 0;
    let mut memUsage: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut proc_0: ProcessStat = {
        let mut init = ProcessStat {
            pid: 0 as libc::c_int,
            comm: 0 as *mut libc::c_char,
            state: 0,
            ppid: 0,
            pgrp: 0,
            session: 0,
            tty_nr: 0,
            tpgid: 0,
            flags: 0,
            minflt: 0,
            cminflt: 0,
            majflt: 0,
            cmajflt: 0,
            utime: 0,
            stime: 0,
            cutime: 0,
            cstime: 0,
            priority: 0,
            nice: 0,
            num_threads: 0,
            itrealvalue: 0,
            starttime: 0,
            vsize: 0,
            rss: 0,
            rsslim: 0,
            startcode: 0,
            endcode: 0,
            startstack: 0,
            kstkesp: 0,
            kstkeip: 0,
            signal: 0,
            blocked: 0,
            sigignore: 0,
            sigcatch: 0,
            wchan: 0,
            nswap: 0,
            cnswap: 0,
            exit_signal: 0,
            processor: 0,
            rt_priority: 0,
            policy: 0,
            delayacct_blkio_ticks: 0,
            guest_time: 0,
            cguest_time: 0,
            start_data: 0,
            end_data: 0,
            start_brk: 0,
            arg_start: 0,
            arg_end: 0,
            env_start: 0,
            env_end: 0,
            exit_code: 0,
            num_filedescriptors: 0,
        };
        init
    };
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut writer: *mut CoreDumpWriter = NewCoreDumpWriter(COMMIT, config);
    pageSize_kb = sysconf(_SC_PAGESIZE as libc::c_int) >> 10 as libc::c_int;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int + 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            if GetProcessStat((*config).ProcessId, &mut proc_0) {
                memUsage = (proc_0.rss * pageSize_kb >> 10 as libc::c_int)
                    as libc::c_ulong;
                memUsage = memUsage
                    .wrapping_add(
                        (proc_0.nswap).wrapping_mul(pageSize_kb as libc::c_ulong)
                            >> 10 as libc::c_int,
                    );
                if !((*config).bMemoryTriggerBelowValue as libc::c_int != 0
                    && memUsage < (*config).MemoryThreshold as libc::c_ulong
                    || !(*config).bMemoryTriggerBelowValue
                        && memUsage >= (*config).MemoryThreshold as libc::c_ulong)
                {
                    continue;
                }
                Log(
                    info,
                    b"Commit: %ld MB\0" as *const u8 as *const libc::c_char,
                    memUsage,
                );
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"CommitMonitoringThread: Exiting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 59\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn ThreadCountMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    DiagTrace(
        b"ThreadCountMonitoringThread: Starting Thread Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 65\0" as *const u8 as *const libc::c_char,
    );
    let mut config: *mut ProcDumpConfiguration = thread_args
        as *mut ProcDumpConfiguration;
    let mut proc_0: ProcessStat = {
        let mut init = ProcessStat {
            pid: 0 as libc::c_int,
            comm: 0 as *mut libc::c_char,
            state: 0,
            ppid: 0,
            pgrp: 0,
            session: 0,
            tty_nr: 0,
            tpgid: 0,
            flags: 0,
            minflt: 0,
            cminflt: 0,
            majflt: 0,
            cmajflt: 0,
            utime: 0,
            stime: 0,
            cutime: 0,
            cstime: 0,
            priority: 0,
            nice: 0,
            num_threads: 0,
            itrealvalue: 0,
            starttime: 0,
            vsize: 0,
            rss: 0,
            rsslim: 0,
            startcode: 0,
            endcode: 0,
            startstack: 0,
            kstkesp: 0,
            kstkeip: 0,
            signal: 0,
            blocked: 0,
            sigignore: 0,
            sigcatch: 0,
            wchan: 0,
            nswap: 0,
            cnswap: 0,
            exit_signal: 0,
            processor: 0,
            rt_priority: 0,
            policy: 0,
            delayacct_blkio_ticks: 0,
            guest_time: 0,
            cguest_time: 0,
            start_data: 0,
            end_data: 0,
            start_brk: 0,
            arg_start: 0,
            arg_end: 0,
            env_start: 0,
            env_end: 0,
            exit_code: 0,
            num_filedescriptors: 0,
        };
        init
    };
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut writer: *mut CoreDumpWriter = NewCoreDumpWriter(THREAD, config);
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int + 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            if GetProcessStat((*config).ProcessId, &mut proc_0) {
                if !(proc_0.num_threads >= (*config).ThreadThreshold as libc::c_long) {
                    continue;
                }
                Log(
                    info,
                    b"Threads: %ld\0" as *const u8 as *const libc::c_char,
                    proc_0.num_threads,
                );
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"ThreadCountMonitoringThread: Exiting Thread trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 98\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn FileDescriptorCountMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    DiagTrace(
        b"FileDescriptorCountMonitoringThread: Starting Filedescriptor Thread %s\0"
            as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 105\0" as *const u8 as *const libc::c_char,
    );
    let mut config: *mut ProcDumpConfiguration = thread_args
        as *mut ProcDumpConfiguration;
    let mut proc_0: ProcessStat = {
        let mut init = ProcessStat {
            pid: 0 as libc::c_int,
            comm: 0 as *mut libc::c_char,
            state: 0,
            ppid: 0,
            pgrp: 0,
            session: 0,
            tty_nr: 0,
            tpgid: 0,
            flags: 0,
            minflt: 0,
            cminflt: 0,
            majflt: 0,
            cmajflt: 0,
            utime: 0,
            stime: 0,
            cutime: 0,
            cstime: 0,
            priority: 0,
            nice: 0,
            num_threads: 0,
            itrealvalue: 0,
            starttime: 0,
            vsize: 0,
            rss: 0,
            rsslim: 0,
            startcode: 0,
            endcode: 0,
            startstack: 0,
            kstkesp: 0,
            kstkeip: 0,
            signal: 0,
            blocked: 0,
            sigignore: 0,
            sigcatch: 0,
            wchan: 0,
            nswap: 0,
            cnswap: 0,
            exit_signal: 0,
            processor: 0,
            rt_priority: 0,
            policy: 0,
            delayacct_blkio_ticks: 0,
            guest_time: 0,
            cguest_time: 0,
            start_data: 0,
            end_data: 0,
            start_brk: 0,
            arg_start: 0,
            arg_end: 0,
            env_start: 0,
            env_end: 0,
            exit_code: 0,
            num_filedescriptors: 0,
        };
        init
    };
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut writer: *mut CoreDumpWriter = NewCoreDumpWriter(FILEDESC, config);
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int + 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            if GetProcessStat((*config).ProcessId, &mut proc_0) {
                if !(proc_0.num_filedescriptors >= (*config).FileDescriptorThreshold) {
                    continue;
                }
                Log(
                    info,
                    b"File descriptors: %ld\0" as *const u8 as *const libc::c_char,
                    proc_0.num_filedescriptors,
                );
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"FileDescriptorCountMonitoringThread: Exiting Filedescriptor trigger Thread %s\0"
            as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 138\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn SignalMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    DiagTrace(
        b"SignalMonitoringThread: Starting SignalMonitoring Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 154\0" as *const u8 as *const libc::c_char,
    );
    let mut config: *mut ProcDumpConfiguration = thread_args
        as *mut ProcDumpConfiguration;
    let mut wstatus: libc::c_int = 0;
    let mut signum: libc::c_int = -(1 as libc::c_int);
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut writer: *mut CoreDumpWriter = NewCoreDumpWriter(SIGNAL, config);
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int + 1 as libc::c_int {
        if ptrace(
            PTRACE_SEIZE,
            (*config).ProcessId,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            Log(
                error,
                b"Unable to PTRACE the target process\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            loop {
                waitpid((*config).ProcessId, &mut wstatus, 0 as libc::c_int);
                if wstatus & 0x7f as libc::c_int == 0 as libc::c_int
                    || ((wstatus & 0x7f as libc::c_int) + 1 as libc::c_int)
                        as libc::c_schar as libc::c_int >> 1 as libc::c_int
                        > 0 as libc::c_int
                {
                    ptrace(
                        PTRACE_DETACH,
                        (*config).ProcessId,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    break;
                } else {
                    pthread_mutex_lock(&mut ptrace_mutex);
                    signum = (wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int;
                    if signum == (*config).SignalNumber {
                        if ptrace(
                            PTRACE_DETACH,
                            (*config).ProcessId,
                            0 as libc::c_int,
                            19 as libc::c_int,
                        ) == -(1 as libc::c_int) as libc::c_long
                        {
                            Log(
                                error,
                                b"Unable to PTRACE (DETACH) the target process\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            pthread_mutex_unlock(&mut ptrace_mutex);
                            break;
                        } else {
                            Log(
                                info,
                                b"Signal intercepted: %d\0" as *const u8
                                    as *const libc::c_char,
                                signum,
                            );
                            rc = WriteCoreDump(writer);
                            kill((*config).ProcessId, 18 as libc::c_int);
                            if (*config).NumberOfDumpsCollected
                                >= (*config).NumberOfDumpsToCollect
                            {
                                kill((*config).ProcessId, signum);
                                pthread_mutex_unlock(&mut ptrace_mutex);
                                break;
                            } else {
                                ptrace(
                                    PTRACE_CONT,
                                    (*config).ProcessId,
                                    0 as *mut libc::c_void,
                                    signum,
                                );
                                if ptrace(
                                    PTRACE_SEIZE,
                                    (*config).ProcessId,
                                    0 as *mut libc::c_void,
                                    0 as *mut libc::c_void,
                                ) == -(1 as libc::c_int) as libc::c_long
                                {
                                    Log(
                                        error,
                                        b"Unable to PTRACE the target process\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    pthread_mutex_unlock(&mut ptrace_mutex);
                                    break;
                                } else {
                                    pthread_mutex_unlock(&mut ptrace_mutex);
                                }
                            }
                        }
                    } else {
                        ptrace(
                            PTRACE_CONT,
                            (*config).ProcessId,
                            0 as *mut libc::c_void,
                            signum,
                        );
                        pthread_mutex_unlock(&mut ptrace_mutex);
                    }
                }
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"SignalMonitoringThread: Exiting SignalMonitoring Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 232\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn CpuMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    DiagTrace(
        b"CpuMonitoringThread: Starting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 238\0" as *const u8 as *const libc::c_char,
    );
    let mut config: *mut ProcDumpConfiguration = thread_args
        as *mut ProcDumpConfiguration;
    let mut totalTime: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut elapsedTime: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut sysInfo: sysinfo = sysinfo {
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
    };
    let mut cpuUsage: libc::c_int = 0;
    let mut writer: *mut CoreDumpWriter = NewCoreDumpWriter(CPU, config);
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut proc_0: ProcessStat = {
        let mut init = ProcessStat {
            pid: 0 as libc::c_int,
            comm: 0 as *mut libc::c_char,
            state: 0,
            ppid: 0,
            pgrp: 0,
            session: 0,
            tty_nr: 0,
            tpgid: 0,
            flags: 0,
            minflt: 0,
            cminflt: 0,
            majflt: 0,
            cmajflt: 0,
            utime: 0,
            stime: 0,
            cutime: 0,
            cstime: 0,
            priority: 0,
            nice: 0,
            num_threads: 0,
            itrealvalue: 0,
            starttime: 0,
            vsize: 0,
            rss: 0,
            rsslim: 0,
            startcode: 0,
            endcode: 0,
            startstack: 0,
            kstkesp: 0,
            kstkeip: 0,
            signal: 0,
            blocked: 0,
            sigignore: 0,
            sigcatch: 0,
            wchan: 0,
            nswap: 0,
            cnswap: 0,
            exit_signal: 0,
            processor: 0,
            rt_priority: 0,
            policy: 0,
            delayacct_blkio_ticks: 0,
            guest_time: 0,
            cguest_time: 0,
            start_data: 0,
            end_data: 0,
            start_brk: 0,
            arg_start: 0,
            arg_end: 0,
            env_start: 0,
            env_end: 0,
            exit_code: 0,
            num_filedescriptors: 0,
        };
        init
    };
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int + 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            sysinfo(&mut sysInfo);
            if GetProcessStat((*config).ProcessId, &mut proc_0) {
                totalTime = (proc_0.utime)
                    .wrapping_add(proc_0.stime)
                    .wrapping_div(HZ as libc::c_ulong);
                elapsedTime = (sysInfo.uptime
                    - (proc_0.starttime).wrapping_div(HZ as libc::c_ulonglong)
                        as libc::c_long) as libc::c_ulong;
                cpuUsage = (100 as libc::c_int as libc::c_double
                    * (totalTime as libc::c_double / elapsedTime as libc::c_double))
                    as libc::c_int;
                if !((*config).bCpuTriggerBelowValue as libc::c_int != 0
                    && cpuUsage < (*config).CpuThreshold
                    || !(*config).bCpuTriggerBelowValue
                        && cpuUsage >= (*config).CpuThreshold)
                {
                    continue;
                }
                Log(info, b"CPU:\t%d%%\0" as *const u8 as *const libc::c_char, cpuUsage);
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"CpuTCpuMonitoringThread: Exiting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 285\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn TimerThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    DiagTrace(
        b"TimerThread: Starting Trigger Thread %s\0" as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 291\0" as *const u8 as *const libc::c_char,
    );
    let mut config: *mut ProcDumpConfiguration = thread_args
        as *mut ProcDumpConfiguration;
    let mut writer: *mut CoreDumpWriter = NewCoreDumpWriter(TIME, config);
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int + 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, 0 as libc::c_int);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            Log(info, b"Timed:\0" as *const u8 as *const libc::c_char);
            rc = WriteCoreDump(writer);
            rc = WaitForQuit(config, (*config).ThresholdSeconds * 1000 as libc::c_int);
            if rc != 110 as libc::c_int {
                break;
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"TimerThread: Exiting Trigger Thread %s\0" as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 312\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
