use ::libc;
extern "C" {
    pub type __dirstream;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn set_cloexec_flag(desc: libc::c_int, value: bool) -> libc::c_int;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir_safer(name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn extendbuf(
        existing: *mut libc::c_void,
        wanted: size_t,
        allocated: *mut size_t,
    ) -> *mut libc::c_void;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn safe_atoi(s: *const libc::c_char, style: quoting_style) -> libc::c_int;
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
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type va_list = __builtin_va_list;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
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
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remember_fd_context {
    pub buf: *mut libc::c_int,
    pub used: size_t,
    pub allocated: size_t,
}
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub const MAX_POLL: C2RustUnnamed_0 = 64;
pub type C2RustUnnamed_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_leak_context {
    pub prev_buf: *const libc::c_int,
    pub used: size_t,
    pub lookup_pos: size_t,
    pub leaked_fd: libc::c_int,
}
static mut non_cloexec_fds: *mut libc::c_int = 0 as *const libc::c_int
    as *mut libc::c_int;
static mut num_cloexec_fds: size_t = 0;
unsafe extern "C" fn get_proc_max_fd() -> libc::c_int {
    let mut path: *const libc::c_char = b"/proc/self/fd\0" as *const u8
        as *const libc::c_char;
    let mut maxfd: libc::c_int = -(1 as libc::c_int);
    let mut dir: *mut DIR = opendir_safer(path);
    if !dir.is_null() {
        let mut good: libc::c_int = 0 as libc::c_int;
        let mut dent: *mut dirent = 0 as *mut dirent;
        loop {
            *__errno_location() = 0 as libc::c_int;
            dent = readdir(dir);
            if dent.is_null() {
                if *__errno_location() != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style(0 as libc::c_int, locale_quoting_style, path),
                    );
                    good = 0 as libc::c_int;
                }
                break;
            } else if (*dent).d_name[0 as libc::c_int as usize] as libc::c_int
                != '.' as i32
                || (*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                    != 0 as libc::c_int
                    && ((*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                        != '.' as i32
                        || (*dent).d_name[2 as libc::c_int as usize] as libc::c_int
                            != 0 as libc::c_int)
            {
                let fd: libc::c_int = safe_atoi(
                    ((*dent).d_name).as_mut_ptr(),
                    literal_quoting_style,
                );
                if fd > maxfd {
                    maxfd = fd;
                }
                good = 1 as libc::c_int;
            }
        }
        closedir(dir);
        if good != 0 {
            return maxfd;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn get_max_fd() -> libc::c_int {
    let mut open_max: libc::c_long = 0;
    open_max = get_proc_max_fd() as libc::c_long;
    if open_max >= 0 as libc::c_int as libc::c_long {
        return open_max as libc::c_int;
    }
    open_max = sysconf(_SC_OPEN_MAX as libc::c_int);
    if open_max == -(1 as libc::c_int) as libc::c_long {
        open_max = 20 as libc::c_int as libc::c_long;
    }
    let mut fd_limit: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    if 0 as libc::c_int == getrlimit(RLIMIT_NOFILE, &mut fd_limit) {
        if fd_limit.rlim_cur == -(1 as libc::c_int) as __rlim_t {
            return open_max as libc::c_int
        } else {
            return fd_limit.rlim_cur as libc::c_int
        }
    }
    return open_max as libc::c_int;
}
unsafe extern "C" fn visit_open_fds(
    mut fd_min: libc::c_int,
    mut fd_max: libc::c_int,
    mut callback: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int,
    >,
    mut cb_context: *mut libc::c_void,
) -> libc::c_int {
    let mut pf: [pollfd; 64] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 64];
    let mut rv: libc::c_int = 0 as libc::c_int;
    while fd_min < fd_max {
        let mut i: libc::c_int = 0;
        let mut limit: libc::c_int = fd_max - fd_min;
        if limit > MAX_POLL as libc::c_int {
            limit = MAX_POLL as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < limit {
            pf[i as usize]
                .events = (0x1 as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
            pf[i as usize].revents = 0 as libc::c_int as libc::c_short;
            pf[i as usize].fd = fd_min + i;
            i += 1;
            i;
        }
        rv = poll(pf.as_mut_ptr(), limit as nfds_t, 0 as libc::c_int);
        if -(1 as libc::c_int) == rv {
            return -(1 as libc::c_int)
        } else {
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < limit {
                if pf[j as usize].revents as libc::c_int != 0x20 as libc::c_int {
                    rv = callback.unwrap()(pf[j as usize].fd, cb_context);
                    if 0 as libc::c_int != rv {
                        return rv;
                    }
                }
                j += 1;
                j;
            }
        }
        fd_min += limit;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fd_is_cloexec(mut fd: libc::c_int) -> libc::c_int {
    let flags: libc::c_int = rpl_fcntl(fd, 1 as libc::c_int);
    return flags & 1 as libc::c_int;
}
unsafe extern "C" fn remember_fd_if_non_cloexec(
    mut fd: libc::c_int,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    if fd_is_cloexec(fd) != 0 {
        return 0 as libc::c_int
    } else {
        let p: *mut remember_fd_context = context as *mut remember_fd_context;
        let mut newbuf: *mut libc::c_void = extendbuf(
            (*p).buf as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(
                    ((*p).used).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
            &mut (*p).allocated,
        );
        if !newbuf.is_null() {
            (*p).buf = newbuf as *mut libc::c_int;
            *((*p).buf).offset((*p).used as isize) = fd;
            (*p).used = ((*p).used).wrapping_add(1);
            (*p).used;
            return 0 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    };
}
pub unsafe extern "C" fn remember_non_cloexec_fds() {
    let mut max_fd: libc::c_int = get_max_fd();
    let mut cb_data: remember_fd_context = remember_fd_context {
        buf: 0 as *mut libc::c_int,
        used: 0,
        allocated: 0,
    };
    cb_data.buf = 0 as *mut libc::c_int;
    cb_data.allocated = 0 as libc::c_int as size_t;
    cb_data.used = cb_data.allocated;
    if max_fd < 2147483647 as libc::c_int {
        max_fd += 1;
        max_fd;
    }
    visit_open_fds(
        0 as libc::c_int,
        max_fd,
        Some(
            remember_fd_if_non_cloexec
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int,
        ),
        &mut cb_data as *mut remember_fd_context as *mut libc::c_void,
    );
    non_cloexec_fds = cb_data.buf;
    num_cloexec_fds = cb_data.used;
}
unsafe extern "C" fn find_first_leak_callback(
    mut fd: libc::c_int,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    if fd_is_cloexec(fd) == 0 {
        let mut p: *mut fd_leak_context = context as *mut fd_leak_context;
        while (*p).lookup_pos < (*p).used {
            if *((*p).prev_buf).offset((*p).lookup_pos as isize) < fd {
                (*p).lookup_pos = ((*p).lookup_pos).wrapping_add(1);
                (*p).lookup_pos;
            } else {
                if !(*((*p).prev_buf).offset((*p).lookup_pos as isize) == fd) {
                    break;
                }
                return 0 as libc::c_int;
            }
        }
        (*p).leaked_fd = fd;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_first_leaked_fd(
    mut prev_non_cloexec_fds: *const libc::c_int,
    mut n: size_t,
) -> libc::c_int {
    let mut context: fd_leak_context = fd_leak_context {
        prev_buf: 0 as *const libc::c_int,
        used: 0,
        lookup_pos: 0,
        leaked_fd: 0,
    };
    let mut max_fd: libc::c_int = get_max_fd();
    if max_fd < 2147483647 as libc::c_int {
        max_fd += 1;
        max_fd;
    }
    context.prev_buf = prev_non_cloexec_fds;
    context.used = n;
    context.lookup_pos = 0 as libc::c_int as size_t;
    context.leaked_fd = -(1 as libc::c_int);
    visit_open_fds(
        0 as libc::c_int,
        max_fd,
        Some(
            find_first_leak_callback
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int,
        ),
        &mut context as *mut fd_leak_context as *mut libc::c_void,
    );
    return context.leaked_fd;
}
unsafe extern "C" fn o_cloexec_works() -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    let mut fd: libc::c_int = open_safer(
        b"/\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int | 0o2000000 as libc::c_int,
    );
    if fd >= 0 as libc::c_int {
        result = fd_is_cloexec(fd) != 0;
        close(fd);
    }
    return result;
}
pub unsafe extern "C" fn open_cloexec(
    mut path: *const libc::c_char,
    mut flags: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut mode: mode_t = 0 as libc::c_int as mode_t;
    static mut cloexec_works: bool = 0 as libc::c_int != 0;
    static mut cloexec_status_known: bool = 0 as libc::c_int != 0;
    if flags & 0o100 as libc::c_int != 0 {
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        mode = ap.arg::<mode_t>();
    }
    if !cloexec_status_known {
        cloexec_works = o_cloexec_works();
        cloexec_status_known = 1 as libc::c_int != 0;
    }
    fd = open_safer(path, flags | 0o2000000 as libc::c_int, mode);
    if fd >= 0 as libc::c_int
        && !(0o2000000 as libc::c_int != 0 && cloexec_works as libc::c_int != 0)
    {
        set_cloexec_flag(fd, 1 as libc::c_int != 0);
    }
    return fd;
}
pub unsafe extern "C" fn forget_non_cloexec_fds() {
    rpl_free(non_cloexec_fds as *mut libc::c_void);
    non_cloexec_fds = 0 as *mut libc::c_int;
    num_cloexec_fds = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn fd_leak_check_is_enabled() -> bool {
    if !(getenv(b"GNU_FINDUTILS_FD_LEAK_CHECK\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn complain_about_leaky_fds() {
    let mut no_leaks: libc::c_int = 1 as libc::c_int;
    let leaking_fd: libc::c_int = find_first_leaked_fd(non_cloexec_fds, num_cloexec_fds);
    if leaking_fd >= 0 as libc::c_int {
        no_leaks = 0 as libc::c_int;
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"File descriptor %d will leak; please report this as a bug, remembering to include a detailed description of the simplest way to reproduce this problem.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            leaking_fd,
        );
    }
    if no_leaks != 0 {} else {
        __assert_fail(
            b"no_leaks\0" as *const u8 as *const libc::c_char,
            b"fdleak.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void complain_about_leaky_fds(void)\0"))
                .as_ptr(),
        );
    }
    'c_5006: {
        if no_leaks != 0 {} else {
            __assert_fail(
                b"no_leaks\0" as *const u8 as *const libc::c_char,
                b"fdleak.c\0" as *const u8 as *const libc::c_char,
                396 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void complain_about_leaky_fds(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
