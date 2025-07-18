use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mbsstr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut environ: *mut *mut libc::c_char;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_ulong,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
}
pub type size_t = libc::c_ulong;
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
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_state {
    pub cmd_argc: size_t,
    pub cmd_argv: *mut *mut libc::c_char,
    pub cmd_argv_alloc: size_t,
    pub argbuf: *mut libc::c_char,
    pub cmd_argv_chars: size_t,
    pub cmd_initial_argv_chars: size_t,
    pub usercontext: *mut libc::c_void,
    pub todo: libc::c_int,
    pub dir_fd: libc::c_int,
    pub largest_successful_arg_count: size_t,
    pub smallest_failed_arg_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_control {
    pub exit_if_size_exceeded: libc::c_int,
    pub posix_arg_size_max: size_t,
    pub posix_arg_size_min: size_t,
    pub arg_max: size_t,
    pub max_arg_count: size_t,
    pub rplen: size_t,
    pub replace_pat: *const libc::c_char,
    pub initial_argc: size_t,
    pub exec_callback: Option::<
        unsafe extern "C" fn(
            *mut buildcmd_control,
            *mut libc::c_void,
            libc::c_int,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub lines_per_exec: libc::c_ulong,
    pub args_per_exec: size_t,
}
pub type BC_INIT_STATUS = libc::c_uint;
pub const BC_INIT_CANNOT_ACCOMODATE_HEADROOM: BC_INIT_STATUS = 2;
pub const BC_INIT_ENV_TOO_BIG: BC_INIT_STATUS = 1;
pub const BC_INIT_OK: BC_INIT_STATUS = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
pub const arg_size: C2RustUnnamed_4 = 131072;
pub type C2RustUnnamed_4 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: libc::c_int,
}
static mut special_terminating_arg: *const libc::c_char = b"do_not_care\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn bc_args_complete(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
) {
    bc_push_arg(
        ctl,
        state,
        special_terminating_arg,
        0 as libc::c_int as size_t,
        0 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn bc_do_insert(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut arg: *mut libc::c_char,
    mut arglen: size_t,
    mut prefix: *const libc::c_char,
    mut pfxlen: size_t,
    mut linebuf: *const libc::c_char,
    mut lblen: size_t,
    mut initial_args: libc::c_int,
) {
    static mut insertbuf: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes_left: size_t = ((*ctl).arg_max)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if insertbuf.is_null() {
        insertbuf = xmalloc(
            ((*ctl).arg_max).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    p = insertbuf;
    loop {
        let mut len: size_t = 0;
        let mut s: *mut libc::c_char = mbsstr(arg, (*ctl).replace_pat);
        if !s.is_null() {
            len = s.offset_from(arg) as libc::c_long as size_t;
        } else {
            len = arglen;
        }
        if bytes_left <= len {
            break;
        }
        bytes_left = (bytes_left as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        strncpy(p, arg, len);
        p = p.offset(len as isize);
        arg = arg.offset(len as isize);
        arglen = (arglen as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        if !s.is_null() {
            if bytes_left <= lblen.wrapping_add(pfxlen) {
                break;
            }
            bytes_left = (bytes_left as libc::c_ulong)
                .wrapping_sub(lblen.wrapping_add(pfxlen)) as size_t as size_t;
            if !prefix.is_null() {
                strcpy(p, prefix);
                p = p.offset(pfxlen as isize);
            }
            strcpy(p, linebuf);
            p = p.offset(lblen as isize);
            arg = arg.offset((*ctl).rplen as isize);
            arglen = (arglen as libc::c_ulong).wrapping_sub((*ctl).rplen) as size_t
                as size_t;
        }
        if !(*arg != 0) {
            break;
        }
    }
    if *arg != 0 {
        if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"command too long\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"command too long\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '\0' as i32 as libc::c_char;
    bc_push_arg(
        ctl,
        state,
        insertbuf,
        p.offset_from(insertbuf) as libc::c_long as size_t,
        0 as *const libc::c_char,
        0 as libc::c_int as size_t,
        initial_args,
    );
}
unsafe extern "C" fn update_limit(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut success: bool,
    mut limit: size_t,
) -> size_t {
    if success {
        if limit > (*state).largest_successful_arg_count {
            (*state).largest_successful_arg_count = limit;
        }
    } else if limit < (*state).smallest_failed_arg_count
        || 0 as libc::c_int as libc::c_ulong == (*state).smallest_failed_arg_count
    {
        (*state).smallest_failed_arg_count = limit;
    }
    if 0 as libc::c_int as libc::c_ulong == (*state).largest_successful_arg_count
        || (*state).smallest_failed_arg_count <= (*state).largest_successful_arg_count
    {
        if success {
            if limit < 18446744073709551615 as libc::c_ulong {
                limit = limit.wrapping_add(1);
                limit;
            }
        } else {
            limit = (limit as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
    } else {
        let shift: size_t = ((*state).smallest_failed_arg_count)
            .wrapping_sub((*state).largest_successful_arg_count)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        if success {
            if shift != 0 {
                limit = (limit as libc::c_ulong).wrapping_add(shift) as size_t as size_t;
            } else {
                limit = limit.wrapping_add(1);
                limit;
            }
        } else if shift != 0 {
            limit = (limit as libc::c_ulong).wrapping_sub(shift) as size_t as size_t;
        } else {
            limit = limit.wrapping_sub(1);
            limit;
        }
    }
    if (*ctl).initial_argc != 0
        && limit
            <= ((*ctl).initial_argc).wrapping_add(1 as libc::c_uint as libc::c_ulong)
    {
        limit = ((*ctl).initial_argc).wrapping_add(1 as libc::c_uint as libc::c_ulong);
    }
    if 0 as libc::c_int as libc::c_ulong == limit {
        limit = 1 as libc::c_uint as size_t;
    }
    return limit;
}
unsafe extern "C" fn copy_args(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut working_args: *mut *mut libc::c_char,
    mut limit: size_t,
    mut done: size_t,
) -> size_t {
    let mut dst_pos: size_t = 0 as libc::c_int as size_t;
    let mut src_pos: size_t = 0 as libc::c_int as size_t;
    while src_pos < (*ctl).initial_argc {
        let fresh1 = src_pos;
        src_pos = src_pos.wrapping_add(1);
        let fresh2 = dst_pos;
        dst_pos = dst_pos.wrapping_add(1);
        let ref mut fresh3 = *working_args.offset(fresh2 as isize);
        *fresh3 = *((*state).cmd_argv).offset(fresh1 as isize);
    }
    src_pos = (src_pos as libc::c_ulong).wrapping_add(done) as size_t as size_t;
    while src_pos < (*state).cmd_argc && dst_pos < limit {
        let fresh4 = src_pos;
        src_pos = src_pos.wrapping_add(1);
        let fresh5 = dst_pos;
        dst_pos = dst_pos.wrapping_add(1);
        let ref mut fresh6 = *working_args.offset(fresh5 as isize);
        *fresh6 = *((*state).cmd_argv).offset(fresh4 as isize);
    }
    if dst_pos >= (*ctl).initial_argc {} else {
        __assert_fail(
            b"dst_pos >= ctl->initial_argc\0" as *const u8 as *const libc::c_char,
            b"buildcmd.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"size_t copy_args(struct buildcmd_control *, struct buildcmd_state *, char **, size_t, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4324: {
        if dst_pos >= (*ctl).initial_argc {} else {
            __assert_fail(
                b"dst_pos >= ctl->initial_argc\0" as *const u8 as *const libc::c_char,
                b"buildcmd.c\0" as *const u8 as *const libc::c_char,
                242 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"size_t copy_args(struct buildcmd_control *, struct buildcmd_state *, char **, size_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let ref mut fresh7 = *working_args.offset(dst_pos as isize);
    *fresh7 = 0 as *mut libc::c_char;
    return dst_pos;
}
pub unsafe extern "C" fn bc_do_exec(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
) {
    let mut working_args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut limit: size_t = 0;
    let mut done: size_t = 0;
    bc_args_complete(ctl, state);
    if (*state).cmd_argc > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"state->cmd_argc > 0\0" as *const u8 as *const libc::c_char,
            b"buildcmd.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0"))
                .as_ptr(),
        );
    }
    'c_4552: {
        if (*state).cmd_argc > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"state->cmd_argc > 0\0" as *const u8 as *const libc::c_char,
                b"buildcmd.c\0" as *const u8 as *const libc::c_char,
                261 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*((*state).cmd_argv)
        .offset(
            ((*state).cmd_argc).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ))
        .is_null()
    {} else {
        __assert_fail(
            b"state->cmd_argv[state->cmd_argc-1] == NULL\0" as *const u8
                as *const libc::c_char,
            b"buildcmd.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0"))
                .as_ptr(),
        );
    }
    'c_4486: {
        if (*((*state).cmd_argv)
            .offset(
                ((*state).cmd_argc).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ))
            .is_null()
        {} else {
            __assert_fail(
                b"state->cmd_argv[state->cmd_argc-1] == NULL\0" as *const u8
                    as *const libc::c_char,
                b"buildcmd.c\0" as *const u8 as *const libc::c_char,
                262 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"void bc_do_exec(struct buildcmd_control *, struct buildcmd_state *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    working_args = xmalloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add((*state).cmd_argc)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    done = 0 as libc::c_int as size_t;
    limit = (*state).cmd_argc;
    loop {
        let dst_pos: size_t = copy_args(ctl, state, working_args, limit, done);
        if ((*ctl).exec_callback)
            .unwrap()(ctl, (*state).usercontext, dst_pos as libc::c_int, working_args)
            != 0
        {
            limit = update_limit(ctl, state, 1 as libc::c_int != 0, limit);
            done = (done as libc::c_ulong)
                .wrapping_add(dst_pos.wrapping_sub((*ctl).initial_argc)) as size_t
                as size_t;
        } else if limit
            <= ((*ctl).initial_argc).wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"can't call exec() due to argument size restrictions\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"can't call exec() due to argument size restrictions\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            limit = update_limit(ctl, state, 0 as libc::c_int != 0, limit);
        }
        if !(done.wrapping_add(1 as libc::c_int as libc::c_ulong)
            < ((*state).cmd_argc).wrapping_sub((*ctl).initial_argc))
        {
            break;
        }
    }
    rpl_free(working_args as *mut libc::c_void);
    bc_clear_args(ctl, state);
}
unsafe extern "C" fn bc_argc_limit_reached(
    mut initial_args: libc::c_int,
    mut ctl: *const buildcmd_control,
    mut state: *mut buildcmd_state,
) -> libc::c_int {
    if initial_args == 0 && (*ctl).args_per_exec != 0
        && ((*state).cmd_argc).wrapping_sub((*ctl).initial_argc) == (*ctl).args_per_exec
    {
        return 1 as libc::c_int;
    }
    return ((*state).cmd_argc == (*ctl).max_arg_count) as libc::c_int;
}
pub unsafe extern "C" fn bc_push_arg(
    mut ctl: *mut buildcmd_control,
    mut state: *mut buildcmd_state,
    mut arg: *const libc::c_char,
    mut len: size_t,
    mut prefix: *const libc::c_char,
    mut pfxlen: size_t,
    mut initial_args: libc::c_int,
) {
    let terminate: libc::c_int = (arg == special_terminating_arg) as libc::c_int;
    if !arg.is_null() {} else {
        __assert_fail(
            b"arg != NULL\0" as *const u8 as *const libc::c_char,
            b"buildcmd.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void bc_push_arg(struct buildcmd_control *, struct buildcmd_state *, const char *, size_t, const char *, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5067: {
        if !arg.is_null() {} else {
            __assert_fail(
                b"arg != NULL\0" as *const u8 as *const libc::c_char,
                b"buildcmd.c\0" as *const u8 as *const libc::c_char,
                341 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 118],
                    &[libc::c_char; 118],
                >(
                    b"void bc_push_arg(struct buildcmd_control *, struct buildcmd_state *, const char *, size_t, const char *, size_t, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if terminate == 0 {
        if ((*state).cmd_argv_chars).wrapping_add(len).wrapping_add(pfxlen)
            > (*ctl).arg_max
        {
            if initial_args != 0 || (*state).cmd_argc == (*ctl).initial_argc {
                if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot fit single argument within argument list size limit\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot fit single argument within argument list size limit\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if !((*ctl).replace_pat).is_null()
                || (*ctl).exit_if_size_exceeded != 0
                    && ((*ctl).lines_per_exec != 0 || (*ctl).args_per_exec != 0)
            {
                if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"argument list too long\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"argument list too long\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            bc_do_exec(ctl, state);
        }
        if bc_argc_limit_reached(initial_args, ctl, state) != 0 {
            bc_do_exec(ctl, state);
        }
    }
    if initial_args == 0 {
        (*state).todo = 1 as libc::c_int;
    }
    if (*state).cmd_argc >= (*state).cmd_argv_alloc {
        if ((*state).cmd_argv).is_null() {
            (*state).cmd_argv_alloc = 64 as libc::c_int as size_t;
            (*state)
                .cmd_argv = xmalloc(
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((*state).cmd_argv_alloc),
            ) as *mut *mut libc::c_char;
        } else {
            (*state)
                .cmd_argv_alloc = ((*state).cmd_argv_alloc as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            (*state)
                .cmd_argv = xrealloc(
                (*state).cmd_argv as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((*state).cmd_argv_alloc),
            ) as *mut *mut libc::c_char;
        }
    }
    if terminate != 0 {
        let fresh8 = (*state).cmd_argc;
        (*state).cmd_argc = ((*state).cmd_argc).wrapping_add(1);
        let ref mut fresh9 = *((*state).cmd_argv).offset(fresh8 as isize);
        *fresh9 = 0 as *mut libc::c_char;
    } else {
        let fresh10 = (*state).cmd_argc;
        (*state).cmd_argc = ((*state).cmd_argc).wrapping_add(1);
        let ref mut fresh11 = *((*state).cmd_argv).offset(fresh10 as isize);
        *fresh11 = ((*state).argbuf).offset((*state).cmd_argv_chars as isize);
        if !prefix.is_null() {
            strcpy(((*state).argbuf).offset((*state).cmd_argv_chars as isize), prefix);
            (*state)
                .cmd_argv_chars = ((*state).cmd_argv_chars as libc::c_ulong)
                .wrapping_add(pfxlen) as size_t as size_t;
        }
        strcpy(((*state).argbuf).offset((*state).cmd_argv_chars as isize), arg);
        (*state)
            .cmd_argv_chars = ((*state).cmd_argv_chars as libc::c_ulong)
            .wrapping_add(len) as size_t as size_t;
        if bc_argc_limit_reached(initial_args, ctl, state) != 0 {
            bc_do_exec(ctl, state);
        }
    }
    if initial_args != 0 {
        (*state).cmd_initial_argv_chars = (*state).cmd_argv_chars;
    }
}
pub unsafe extern "C" fn bc_get_arg_max() -> size_t {
    let mut val: libc::c_long = 0;
    if !(0 as libc::c_int as size_t)
        >= 9223372036854775807 as libc::c_long as libc::c_ulong
    {} else {
        __assert_fail(
            b"(~(size_t)0) >= LONG_MAX\0" as *const u8 as *const libc::c_char,
            b"buildcmd.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"size_t bc_get_arg_max(void)\0"))
                .as_ptr(),
        );
    }
    'c_5749: {
        if !(0 as libc::c_int as size_t)
            >= 9223372036854775807 as libc::c_long as libc::c_ulong
        {} else {
            __assert_fail(
                b"(~(size_t)0) >= LONG_MAX\0" as *const u8 as *const libc::c_char,
                b"buildcmd.c\0" as *const u8 as *const libc::c_char,
                421 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"size_t bc_get_arg_max(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = sysconf(_SC_ARG_MAX as libc::c_int);
    if val > 0 as libc::c_int as libc::c_long {
        return val as size_t;
    }
    return 9223372036854775807 as libc::c_long as size_t;
}
unsafe extern "C" fn cb_exec_noop(
    mut ctl: *mut buildcmd_control,
    mut usercontext: *mut libc::c_void,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bc_size_of_environment() -> size_t {
    let mut len: size_t = 0 as libc::c_uint as size_t;
    let mut envp: *mut *mut libc::c_char = environ;
    while !(*envp).is_null() {
        let fresh12 = envp;
        envp = envp.offset(1);
        len = (len as libc::c_ulong)
            .wrapping_add(
                (strlen(*fresh12)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    return len;
}
pub unsafe extern "C" fn bc_init_controlinfo(
    mut ctl: *mut buildcmd_control,
    mut headroom: size_t,
) -> BC_INIT_STATUS {
    let mut size_of_environment: size_t = bc_size_of_environment();
    (*ctl).posix_arg_size_min = 4096 as libc::c_int as size_t;
    (*ctl).posix_arg_size_max = bc_get_arg_max();
    (*ctl).exit_if_size_exceeded = 0 as libc::c_int;
    if size_of_environment > (*ctl).posix_arg_size_max {
        return BC_INIT_ENV_TOO_BIG
    } else if headroom.wrapping_add(size_of_environment) >= (*ctl).posix_arg_size_max {
        return BC_INIT_CANNOT_ACCOMODATE_HEADROOM
    } else {
        (*ctl)
            .posix_arg_size_max = ((*ctl).posix_arg_size_max as libc::c_ulong)
            .wrapping_sub(size_of_environment) as size_t as size_t;
        (*ctl)
            .posix_arg_size_max = ((*ctl).posix_arg_size_max as libc::c_ulong)
            .wrapping_sub(headroom) as size_t as size_t;
    }
    (*ctl)
        .max_arg_count = ((*ctl).posix_arg_size_max)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_uint as libc::c_ulong);
    if (*ctl).max_arg_count > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ctl->max_arg_count > 0\0" as *const u8 as *const libc::c_char,
            b"buildcmd.c\0" as *const u8 as *const libc::c_char,
            518 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"enum BC_INIT_STATUS bc_init_controlinfo(struct buildcmd_control *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5606: {
        if (*ctl).max_arg_count > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"ctl->max_arg_count > 0\0" as *const u8 as *const libc::c_char,
                b"buildcmd.c\0" as *const u8 as *const libc::c_char,
                518 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"enum BC_INIT_STATUS bc_init_controlinfo(struct buildcmd_control *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*ctl).rplen = 0 as libc::c_uint as size_t;
    (*ctl).replace_pat = 0 as *const libc::c_char;
    (*ctl).initial_argc = 0 as libc::c_int as size_t;
    (*ctl)
        .exec_callback = Some(
        cb_exec_noop
            as unsafe extern "C" fn(
                *mut buildcmd_control,
                *mut libc::c_void,
                libc::c_int,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
    );
    (*ctl).lines_per_exec = 0 as libc::c_int as libc::c_ulong;
    (*ctl).args_per_exec = 0 as libc::c_int as size_t;
    (*ctl).arg_max = (*ctl).posix_arg_size_max;
    return BC_INIT_OK;
}
pub unsafe extern "C" fn bc_use_sensible_arg_max(mut ctl: *mut buildcmd_control) {
    if arg_size as libc::c_int as libc::c_ulong > (*ctl).posix_arg_size_max {
        (*ctl).arg_max = (*ctl).posix_arg_size_max;
    } else if (arg_size as libc::c_int as libc::c_ulong) < (*ctl).posix_arg_size_min {
        (*ctl).arg_max = (*ctl).posix_arg_size_min;
    } else {
        (*ctl).arg_max = arg_size as libc::c_int as size_t;
    };
}
pub unsafe extern "C" fn bc_init_state(
    mut ctl: *const buildcmd_control,
    mut state: *mut buildcmd_state,
    mut context: *mut libc::c_void,
) {
    (*state).cmd_argc = 0 as libc::c_int as size_t;
    (*state).cmd_argv_chars = 0 as libc::c_int as size_t;
    (*state).cmd_argv = 0 as *mut *mut libc::c_char;
    (*state).cmd_argv_alloc = 0 as libc::c_int as size_t;
    (*state).largest_successful_arg_count = 0 as libc::c_int as size_t;
    (*state).smallest_failed_arg_count = 0 as libc::c_int as size_t;
    if (*ctl).arg_max
        <= (9223372036854775807 as libc::c_long - 2048 as libc::c_long) as libc::c_ulong
    {} else {
        __assert_fail(
            b"ctl->arg_max <= (LONG_MAX - 2048L)\0" as *const u8 as *const libc::c_char,
            b"buildcmd.c\0" as *const u8 as *const libc::c_char,
            572 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"void bc_init_state(const struct buildcmd_control *, struct buildcmd_state *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5423: {
        if (*ctl).arg_max
            <= (9223372036854775807 as libc::c_long - 2048 as libc::c_long)
                as libc::c_ulong
        {} else {
            __assert_fail(
                b"ctl->arg_max <= (LONG_MAX - 2048L)\0" as *const u8
                    as *const libc::c_char,
                b"buildcmd.c\0" as *const u8 as *const libc::c_char,
                572 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"void bc_init_state(const struct buildcmd_control *, struct buildcmd_state *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*state)
        .argbuf = xmalloc(
        ((*ctl).arg_max).wrapping_add(1 as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_char;
    (*state).cmd_initial_argv_chars = 0 as libc::c_int as size_t;
    (*state).cmd_argv_chars = (*state).cmd_initial_argv_chars;
    (*state).todo = 0 as libc::c_int;
    (*state).dir_fd = -(1 as libc::c_int);
    (*state).usercontext = context;
}
pub unsafe extern "C" fn bc_clear_args(
    mut ctl: *const buildcmd_control,
    mut state: *mut buildcmd_state,
) {
    (*state).cmd_argc = (*ctl).initial_argc;
    (*state).cmd_argv_chars = (*state).cmd_initial_argv_chars;
    (*state).todo = 0 as libc::c_int;
    (*state).dir_fd = -(1 as libc::c_int);
}
unsafe extern "C" fn exceeds(
    mut env_var_name: *const libc::c_char,
    mut quantity: size_t,
) -> libc::c_int {
    let mut val: *const libc::c_char = getenv(env_var_name);
    if !val.is_null() {
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut limit: libc::c_ulong = 0;
        if xstrtoul(
            val,
            &mut tmp,
            10 as libc::c_int,
            &mut limit,
            0 as *const libc::c_char,
        ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
        {
            if quantity > limit {
                return 1 as libc::c_int;
            }
        } else {
            if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Environment variable %s is not set to a valid decimal number\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    env_var_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Environment variable %s is not set to a valid decimal number\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    env_var_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
            return 0 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bc_args_exceed_testing_limit(
    mut argv: *mut *mut libc::c_char,
) -> bool {
    let mut chars: size_t = 0;
    let mut args: size_t = 0;
    args = 0 as libc::c_int as size_t;
    chars = args;
    while !(*argv).is_null() {
        args = args.wrapping_add(1);
        args;
        chars = (chars as libc::c_ulong).wrapping_add(strlen(*argv)) as size_t as size_t;
        argv = argv.offset(1);
        argv;
    }
    return exceeds(
        b"__GNU_FINDUTILS_EXEC_ARG_COUNT_LIMIT\0" as *const u8 as *const libc::c_char,
        args,
    ) != 0
        || exceeds(
            b"__GNU_FINDUTILS_EXEC_ARG_LENGTH_LIMIT\0" as *const u8
                as *const libc::c_char,
            chars,
        ) != 0;
}
