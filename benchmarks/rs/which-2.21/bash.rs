use ::libc;
extern "C" {
    fn eaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn getgroups(__size: libc::c_int, __list: *mut __gid_t) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn endpwent();
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub struct user_info {
    pub uid: uid_t,
    pub euid: uid_t,
    pub gid: gid_t,
    pub egid: gid_t,
    pub user_name: *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub home_dir: *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut current_user: user_info = {
    let mut init = user_info {
        uid: -(1 as libc::c_int) as uid_t,
        euid: -(1 as libc::c_int) as uid_t,
        gid: -(1 as libc::c_int) as gid_t,
        egid: -(1 as libc::c_int) as gid_t,
        user_name: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        shell: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        home_dir: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
    };
    init
};
pub unsafe extern "C" fn uidget() -> libc::c_int {
    let mut u: uid_t = 0;
    u = getuid();
    if current_user.uid != u {
        if !(current_user.user_name).is_null() {
            free(current_user.user_name as *mut libc::c_void);
        }
        if !(current_user.shell).is_null() {
            free(current_user.shell as *mut libc::c_void);
        }
        if !(current_user.home_dir).is_null() {
            free(current_user.home_dir as *mut libc::c_void);
        }
        current_user.home_dir = 0 as *mut libc::c_void as *mut libc::c_char;
        current_user.shell = current_user.home_dir;
        current_user.user_name = current_user.shell;
    }
    current_user.uid = u;
    current_user.gid = getgid();
    current_user.euid = geteuid();
    current_user.egid = getegid();
    return (current_user.uid != current_user.euid
        || current_user.gid != current_user.egid) as libc::c_int;
}
static mut ngroups: libc::c_int = 0;
static mut maxgroups: libc::c_int = 0;
static mut group_array: *mut gid_t = 0 as *const libc::c_void as *mut libc::c_void
    as *mut gid_t;
pub unsafe extern "C" fn getmaxgroups() -> libc::c_int {
    static mut maxgroups_0: libc::c_int = -(1 as libc::c_int);
    if maxgroups_0 > 0 as libc::c_int {
        return maxgroups_0;
    }
    maxgroups_0 = sysconf(_SC_NGROUPS_MAX as libc::c_int) as libc::c_int;
    if maxgroups_0 <= 0 as libc::c_int {
        maxgroups_0 = 64 as libc::c_int;
    }
    return maxgroups_0;
}
unsafe extern "C" fn initialize_group_array() {
    let mut i: libc::c_int = 0;
    if maxgroups == 0 as libc::c_int {
        maxgroups = getmaxgroups();
    }
    ngroups = 0 as libc::c_int;
    group_array = xrealloc(
        group_array as *mut libc::c_void,
        (maxgroups as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong),
    ) as *mut gid_t;
    ngroups = getgroups(maxgroups, group_array);
    if ngroups == 0 as libc::c_int {
        *group_array.offset(0 as libc::c_int as isize) = current_user.gid;
        ngroups = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < ngroups {
        if current_user.gid == *group_array.offset(i as isize) {
            break;
        }
        i += 1;
        i;
    }
    if i == ngroups && ngroups < maxgroups {
        i = ngroups;
        while i > 0 as libc::c_int {
            *group_array
                .offset(
                    i as isize,
                ) = *group_array.offset((i - 1 as libc::c_int) as isize);
            i -= 1;
            i;
        }
        *group_array.offset(0 as libc::c_int as isize) = current_user.gid;
        ngroups += 1;
        ngroups;
    }
    if *group_array.offset(0 as libc::c_int as isize) != current_user.gid {
        i = 0 as libc::c_int;
        while i < ngroups {
            if *group_array.offset(i as isize) == current_user.gid {
                break;
            }
            i += 1;
            i;
        }
        if i < ngroups {
            *group_array
                .offset(i as isize) = *group_array.offset(0 as libc::c_int as isize);
            *group_array.offset(0 as libc::c_int as isize) = current_user.gid;
        }
    }
}
pub unsafe extern "C" fn group_member(mut gid: gid_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if gid == current_user.gid || gid == current_user.egid {
        return 1 as libc::c_int;
    }
    if ngroups == 0 as libc::c_int {
        initialize_group_array();
    }
    if ngroups <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < ngroups {
        if gid == *group_array.offset(i as isize) {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn file_status(mut name: *const libc::c_char) -> libc::c_int {
    let mut finfo: stat = stat {
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
    let mut r: libc::c_int = 0;
    if stat(name, &mut finfo) < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if finfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        return 0x1 as libc::c_int | 0x10 as libc::c_int;
    }
    r = 0x1 as libc::c_int;
    if eaccess(name, 1 as libc::c_int) == 0 as libc::c_int {
        r |= 0x2 as libc::c_int;
    }
    if eaccess(name, 4 as libc::c_int) == 0 as libc::c_int {
        r |= 0x40 as libc::c_int;
    }
    return r;
}
pub unsafe extern "C" fn absolute_program(
    mut string: *const libc::c_char,
) -> libc::c_int {
    return (strchr(string, '/' as i32) != 0 as *mut libc::c_void as *mut libc::c_char)
        as libc::c_int;
}
pub unsafe extern "C" fn substring(
    mut string: *const libc::c_char,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    len = end - start;
    result = xmalloc((len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        result as *mut libc::c_void,
        string.offset(start as isize) as *const libc::c_void,
        len as libc::c_ulong,
    );
    *result.offset(len as isize) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn extract_colon_unit(
    mut string: *const libc::c_char,
    mut p_index: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if string.is_null() {
        return 0 as *mut libc::c_char;
    }
    len = strlen(string) as libc::c_int;
    if *p_index >= len {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    i = *p_index;
    if i != 0 && *string.offset(i as isize) as libc::c_int == ':' as i32 {
        i += 1;
        i;
    }
    start = i;
    while *string.offset(i as isize) as libc::c_int != 0
        && *string.offset(i as isize) as libc::c_int != ':' as i32
    {
        i += 1;
        i;
    }
    *p_index = i;
    if i == start {
        if *string.offset(i as isize) != 0 {
            *p_index += 1;
            *p_index;
        }
        value = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
        *value.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        value = substring(string, start, i);
    }
    return value;
}
pub unsafe extern "C" fn get_next_path_element(
    mut path_list: *const libc::c_char,
    mut path_index_pointer: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    path = extract_colon_unit(path_list, path_index_pointer);
    if path.is_null() {
        return path;
    }
    if *path as libc::c_int == '\0' as i32 {
        free(path as *mut libc::c_void);
        path = strcpy(
            xmalloc(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(strlen(b".\0" as *const u8 as *const libc::c_char)),
            ) as *mut libc::c_char,
            b".\0" as *const u8 as *const libc::c_char,
        );
    }
    return path;
}
pub unsafe extern "C" fn make_full_pathname(
    mut path: *const libc::c_char,
    mut name: *const libc::c_char,
    mut name_len: libc::c_int,
) -> *mut libc::c_char {
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_len: libc::c_int = 0;
    path_len = strlen(path) as libc::c_int;
    full_path = xmalloc((2 as libc::c_int + path_len + name_len) as size_t)
        as *mut libc::c_char;
    strcpy(full_path, path);
    *full_path.offset(path_len as isize) = '/' as i32 as libc::c_char;
    strcpy(full_path.offset(path_len as isize).offset(1 as libc::c_int as isize), name);
    return full_path;
}
pub unsafe extern "C" fn get_current_user_info() {
    let mut entry: *mut passwd = 0 as *mut passwd;
    if (current_user.user_name).is_null() {
        entry = getpwuid(current_user.uid);
        if !entry.is_null() {
            current_user
                .user_name = strcpy(
                xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*entry).pw_name)),
                ) as *mut libc::c_char,
                (*entry).pw_name,
            );
            current_user
                .shell = if !((*entry).pw_shell).is_null()
                && *((*entry).pw_shell).offset(0 as libc::c_int as isize) as libc::c_int
                    != 0
            {
                strcpy(
                    xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen((*entry).pw_shell)),
                    ) as *mut libc::c_char,
                    (*entry).pw_shell,
                )
            } else {
                strcpy(
                    xmalloc(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                strlen(b"/bin/sh\0" as *const u8 as *const libc::c_char),
                            ),
                    ) as *mut libc::c_char,
                    b"/bin/sh\0" as *const u8 as *const libc::c_char,
                )
            };
            current_user
                .home_dir = strcpy(
                xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*entry).pw_dir)),
                ) as *mut libc::c_char,
                (*entry).pw_dir,
            );
        } else {
            current_user
                .user_name = b"I have no name!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            current_user
                .user_name = strcpy(
                xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(current_user.user_name)),
                ) as *mut libc::c_char,
                current_user.user_name,
            );
            current_user
                .shell = strcpy(
                xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            strlen(b"/bin/sh\0" as *const u8 as *const libc::c_char),
                        ),
                ) as *mut libc::c_char,
                b"/bin/sh\0" as *const u8 as *const libc::c_char,
            );
            current_user
                .home_dir = strcpy(
                xmalloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(b"/\0" as *const u8 as *const libc::c_char)),
                ) as *mut libc::c_char,
                b"/\0" as *const u8 as *const libc::c_char,
            );
        }
        endpwent();
    }
}
pub unsafe extern "C" fn sh_get_env_value(
    mut v: *const libc::c_char,
) -> *mut libc::c_char {
    return getenv(v);
}
pub unsafe extern "C" fn sh_get_home_dir() -> *mut libc::c_char {
    if (current_user.home_dir).is_null() {
        get_current_user_info();
    }
    return current_user.home_dir;
}
