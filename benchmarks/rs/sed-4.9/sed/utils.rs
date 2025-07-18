use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn __errno_location() -> *mut libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rpl_free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static mut program_name: *const libc::c_char;
    fn __fwriting(__fp: *mut FILE) -> libc::c_int;
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
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type ptrdiff_t = libc::c_long;
pub type mode_t = __mode_t;
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
pub type idx_t = ptrdiff_t;
pub type exit_codes = libc::c_uint;
pub const EXIT_PANIC: exit_codes = 4;
pub const EXIT_BAD_INPUT: exit_codes = 2;
pub const EXIT_BAD_USAGE: exit_codes = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_file {
    pub fp: *mut FILE,
    pub name: *mut libc::c_char,
    pub link: *mut open_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub allocated: size_t,
    pub length: size_t,
    pub b: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __eloop_threshold() -> libc::c_uint {
    static mut sysconf_symloop_max: libc::c_long = 0;
    if sysconf_symloop_max == 0 as libc::c_int as libc::c_long {
        sysconf_symloop_max = sysconf(_SC_SYMLOOP_MAX as libc::c_int);
    }
    let symloop_max: libc::c_uint = (if sysconf_symloop_max
        <= 0 as libc::c_int as libc::c_long
    {
        8 as libc::c_int as libc::c_long
    } else {
        sysconf_symloop_max
    }) as libc::c_uint;
    return if symloop_max > 40 as libc::c_int as libc::c_uint {
        symloop_max
    } else {
        40 as libc::c_int as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
}
static mut open_files: *mut open_file = 0 as *const open_file as *mut open_file;
pub unsafe extern "C" fn panic(mut str: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program_name);
    ap = args.clone();
    vfprintf(stderr, str, ap.as_va_list());
    putc_unlocked('\n' as i32, stderr);
    exit(EXIT_PANIC as libc::c_int);
}
unsafe extern "C" fn utils_fp_name(mut fp: *mut FILE) -> *const libc::c_char {
    let mut p: *mut open_file = 0 as *mut open_file;
    p = open_files;
    while !p.is_null() {
        if (*p).fp == fp {
            return (*p).name;
        }
        p = (*p).link;
    }
    if fp == stdin {
        return b"stdin\0" as *const u8 as *const libc::c_char
    } else if fp == stdout {
        return b"stdout\0" as *const u8 as *const libc::c_char
    } else if fp == stderr {
        return b"stderr\0" as *const u8 as *const libc::c_char
    }
    return b"<unknown>\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn register_open_file(
    mut fp: *mut FILE,
    mut name: *const libc::c_char,
) {
    let mut p: *mut open_file = 0 as *mut open_file;
    p = xmalloc(::std::mem::size_of::<open_file>() as libc::c_ulong) as *mut open_file;
    (*p).link = open_files;
    open_files = p;
    (*p).name = xstrdup(name);
    (*p).fp = fp;
}
pub unsafe extern "C" fn ck_fopen(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut fail: libc::c_int,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, mode);
    if fp.is_null() {
        if fail != 0 {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't open file %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
                strerror(*__errno_location()),
            );
        }
        return 0 as *mut FILE;
    }
    register_open_file(fp, name);
    return fp;
}
pub unsafe extern "C" fn ck_fdopen(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut fail: libc::c_int,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fdopen(fd, mode);
    if fp.is_null() {
        if fail != 0 {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't attach to %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
                strerror(*__errno_location()),
            );
        }
        return 0 as *mut FILE;
    }
    register_open_file(fp, name);
    return fp;
}
static mut G_file_to_unlink: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn remove_cleanup_file() {
    if !G_file_to_unlink.is_null() {
        unlink(G_file_to_unlink);
    }
}
unsafe extern "C" fn register_cleanup_file(mut file: *const libc::c_char) {
    G_file_to_unlink = file;
}
pub unsafe extern "C" fn cancel_cleanup() {
    G_file_to_unlink = 0 as *const libc::c_char;
}
pub unsafe extern "C" fn ck_mkstemp(
    mut p_filename: *mut *mut libc::c_char,
    mut tmpdir: *const libc::c_char,
    mut base: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut template: *mut libc::c_char = xmalloc(
        (strlen(tmpdir))
            .wrapping_add(strlen(base))
            .wrapping_add(8 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        template,
        b"%s/%sXXXXXX\0" as *const u8 as *const libc::c_char,
        tmpdir,
        base,
    );
    let mut save_umask: mode_t = umask(0o77 as libc::c_int as __mode_t);
    let mut fd: libc::c_int = mkostemp(template, 0 as libc::c_int);
    let mut err: libc::c_int = *__errno_location();
    umask(save_umask);
    let mut fp: *mut FILE = 0 as *mut FILE;
    if 0 as libc::c_int <= fd {
        *p_filename = template;
        register_cleanup_file(template);
        fp = fdopen(fd, mode);
        err = *__errno_location();
    }
    if fp.is_null() {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't open temporary file %s: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            template,
            strerror(err),
        );
    }
    register_open_file(fp, template);
    return fp;
}
pub unsafe extern "C" fn ck_fwrite(
    mut ptr: *const libc::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut FILE,
) {
    clearerr_unlocked(stream);
    if size != 0
        && (if 0 != 0 && 0 != 0
            && size.wrapping_mul(nmemb) <= 8 as libc::c_int as libc::c_ulong
            && size != 0 as libc::c_int as libc::c_ulong
        {
            {
                let mut __ptr: *const libc::c_char = ptr as *const libc::c_char;
                let mut __stream: *mut FILE = stream;
                let mut __cnt: size_t = 0;
                __cnt = size.wrapping_mul(nmemb);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh2 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh2 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                size.wrapping_mul(nmemb).wrapping_sub(__cnt).wrapping_div(size)
            }
        } else {
            if 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
                || 0 != 0 && nmemb == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fwrite_unlocked(ptr, size, nmemb, stream)
            }
        }) != nmemb
    {
        panic(
            dcngettext(
                0 as *const libc::c_char,
                b"couldn't write %llu item to %s: %s\0" as *const u8
                    as *const libc::c_char,
                b"couldn't write %llu items to %s: %s\0" as *const u8
                    as *const libc::c_char,
                nmemb,
                5 as libc::c_int,
            ),
            nmemb as libc::c_ulonglong,
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
}
pub unsafe extern "C" fn ck_fread(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut FILE,
) -> size_t {
    clearerr_unlocked(stream);
    if size != 0
        && {
            nmemb = if 0 != 0 && 0 != 0
                && size.wrapping_mul(nmemb) <= 8 as libc::c_int as libc::c_ulong
                && size != 0 as libc::c_int as libc::c_ulong
            {
                {
                    let mut __ptr: *mut libc::c_char = ptr as *mut libc::c_char;
                    let mut __stream: *mut FILE = stream;
                    let mut __cnt: size_t = 0;
                    __cnt = size.wrapping_mul(nmemb);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let mut __c: libc::c_int = getc_unlocked(__stream);
                        if __c == -(1 as libc::c_int) {
                            break;
                        }
                        let fresh3 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh3 = __c as libc::c_char;
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    size.wrapping_mul(nmemb).wrapping_sub(__cnt).wrapping_div(size)
                }
            } else {
                if 0 != 0 && size == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0 && nmemb == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int as size_t
                } else {
                    fread_unlocked(ptr, size, nmemb, stream)
                }
            };
            nmemb <= 0 as libc::c_int as libc::c_ulong
        } && ferror_unlocked(stream) != 0
    {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"read error on %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
    return nmemb;
}
pub unsafe extern "C" fn ck_getdelim(
    mut text: *mut *mut libc::c_char,
    mut buflen: *mut size_t,
    mut delim: libc::c_char,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: ssize_t = 0;
    let mut error: bool = false;
    error = ferror_unlocked(stream) != 0;
    if !error {
        result = getdelim(text, buflen, delim as libc::c_int, stream);
        error = ferror_unlocked(stream) != 0;
    }
    if error {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"read error on %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
    return result as size_t;
}
pub unsafe extern "C" fn ck_fflush(mut stream: *mut FILE) {
    if !(__fwriting(stream) != 0 as libc::c_int) {
        return;
    }
    clearerr_unlocked(stream);
    if fflush_unlocked(stream) == -(1 as libc::c_int)
        && *__errno_location() != 9 as libc::c_int
    {
        panic(
            b"couldn't flush %s: %s\0" as *const u8 as *const libc::c_char,
            utils_fp_name(stream),
            strerror(*__errno_location()),
        );
    }
}
pub unsafe extern "C" fn ck_fclose(mut stream: *mut FILE) {
    let mut prev: *mut *mut open_file = &mut open_files;
    let mut cur: *mut open_file = 0 as *mut open_file;
    loop {
        cur = *prev;
        if cur.is_null() {
            break;
        }
        if stream.is_null() || stream == (*cur).fp {
            let mut fp: *mut FILE = (*cur).fp;
            *prev = (*cur).link;
            do_ck_fclose(fp, (*cur).name);
            rpl_free((*cur).name as *mut libc::c_void);
            rpl_free(cur as *mut libc::c_void);
        } else {
            prev = &mut (*cur).link;
        }
    }
    if stream.is_null() {
        do_ck_fclose(stdout, b"stdout\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn do_ck_fclose(mut fp: *mut FILE, mut name: *const libc::c_char) {
    ck_fflush(fp);
    clearerr_unlocked(fp);
    if fclose(fp) == -(1 as libc::c_int) {
        panic(
            b"couldn't close %s: %s\0" as *const u8 as *const libc::c_char,
            name,
            strerror(*__errno_location()),
        );
    }
}
pub unsafe extern "C" fn follow_symlink(
    mut fname: *const libc::c_char,
) -> *const libc::c_char {
    let mut fn_0: *const libc::c_char = fname;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buf_size: idx_t = 0;
    let mut buf_used: idx_t = 0 as libc::c_int as idx_t;
    let mut num_links: idx_t = 0 as libc::c_int as idx_t;
    loop {
        let mut linklen: ssize_t = 0;
        let mut newlen: idx_t = 0;
        let mut c: *const libc::c_char = 0 as *const libc::c_char;
        loop {
            linklen = if buf_used < buf_size {
                readlink(
                    fn_0,
                    buf.offset(buf_used as isize),
                    (buf_size - buf_used) as size_t,
                )
            } else {
                0 as libc::c_int as libc::c_long
            };
            if !(linklen == buf_size) {
                break;
            }
            buf = xpalloc(
                buf as *mut libc::c_void,
                &mut buf_size,
                1 as libc::c_int as idx_t,
                if (9223372036854775807 as libc::c_long)
                    < 9223372036854775807 as libc::c_long
                {
                    9223372036854775807 as libc::c_long
                } else {
                    9223372036854775807 as libc::c_long
                },
                1 as libc::c_int as idx_t,
            ) as *mut libc::c_char;
            if num_links != 0 {
                fn_0 = buf;
            }
        }
        if linklen < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 22 as libc::c_int {
                break;
            }
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't readlink %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fn_0,
                strerror(*__errno_location()),
            );
        }
        if __eloop_threshold() as libc::c_long <= num_links {
            panic(
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't follow symlink %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                strerror(40 as libc::c_int),
            );
        }
        if (linklen == 0 as libc::c_int as libc::c_long
            || *buf.offset(buf_used as isize) as libc::c_int != '/' as i32)
            && {
                c = strrchr(fn_0, '/' as i32);
                !c.is_null()
            }
        {
            let mut dirlen: idx_t = c.offset_from(fn_0) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            newlen = dirlen + linklen;
            if buf_size <= newlen {
                buf = xpalloc(
                    buf as *mut libc::c_void,
                    &mut buf_size,
                    newlen + 1 as libc::c_int as libc::c_long - buf_size,
                    if (9223372036854775807 as libc::c_long)
                        < 9223372036854775807 as libc::c_long
                    {
                        9223372036854775807 as libc::c_long
                    } else {
                        9223372036854775807 as libc::c_long
                    },
                    1 as libc::c_int as idx_t,
                ) as *mut libc::c_char;
                if num_links != 0 {
                    fn_0 = buf;
                }
            }
            memmove(
                buf.offset(dirlen as isize) as *mut libc::c_void,
                buf.offset(buf_used as isize) as *const libc::c_void,
                linklen as libc::c_ulong,
            );
            if fn_0 != buf as *const libc::c_char {
                memcpy(
                    buf as *mut libc::c_void,
                    fn_0 as *const libc::c_void,
                    dirlen as libc::c_ulong,
                );
            }
        } else {
            memmove(
                buf as *mut libc::c_void,
                buf.offset(buf_used as isize) as *const libc::c_void,
                linklen as libc::c_ulong,
            );
            newlen = linklen;
        }
        *buf.offset(newlen as isize) = '\0' as i32 as libc::c_char;
        buf_used = newlen + 1 as libc::c_int as libc::c_long;
        fn_0 = buf;
        num_links += 1;
        num_links;
    }
    return fn_0;
}
pub unsafe extern "C" fn ck_rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    let mut rd: libc::c_int = rename(from, to);
    if rd != -(1 as libc::c_int) {
        return;
    }
    panic(
        dcgettext(
            0 as *const libc::c_char,
            b"cannot rename %s: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        from,
        strerror(*__errno_location()),
    );
}
pub unsafe extern "C" fn init_buffer() -> *mut buffer {
    let mut b: *mut buffer = (if ::std::mem::size_of::<buffer>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xzalloc(1 as libc::c_int as size_t)
    } else {
        xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<buffer>() as libc::c_ulong,
        )
    }) as *mut buffer;
    (*b)
        .b = (if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xzalloc(50 as libc::c_int as size_t)
    } else {
        xcalloc(
            50 as libc::c_int as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        )
    }) as *mut libc::c_char;
    (*b).allocated = 50 as libc::c_int as size_t;
    (*b).length = 0 as libc::c_int as size_t;
    return b;
}
pub unsafe extern "C" fn get_buffer(mut b: *const buffer) -> *mut libc::c_char {
    return (*b).b;
}
pub unsafe extern "C" fn size_buffer(mut b: *const buffer) -> size_t {
    return (*b).length;
}
unsafe extern "C" fn resize_buffer(mut b: *mut buffer, mut newlen: size_t) {
    let mut try_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alen: size_t = (*b).allocated;
    if newlen <= alen {
        return;
    }
    alen = (alen as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    if newlen < alen {
        try_0 = realloc((*b).b as *mut libc::c_void, alen) as *mut libc::c_char;
    }
    if try_0.is_null() {
        alen = newlen;
        try_0 = xnrealloc(
            (*b).b as *mut libc::c_void,
            alen,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    (*b).allocated = alen;
    (*b).b = try_0;
}
pub unsafe extern "C" fn add_buffer(
    mut b: *mut buffer,
    mut p: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*b).allocated).wrapping_sub((*b).length) < n {
        resize_buffer(b, ((*b).length).wrapping_add(n));
    }
    result = memcpy(
        ((*b).b).offset((*b).length as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        n,
    ) as *mut libc::c_char;
    (*b).length = ((*b).length as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    return result;
}
pub unsafe extern "C" fn add1_buffer(
    mut b: *mut buffer,
    mut c: libc::c_int,
) -> *mut libc::c_char {
    if c != -(1 as libc::c_int) {
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        if ((*b).allocated).wrapping_sub((*b).length) < 1 as libc::c_int as libc::c_ulong
        {
            resize_buffer(
                b,
                ((*b).length).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        let fresh4 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        result = ((*b).b).offset(fresh4 as isize);
        *result = c as libc::c_char;
        return result;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn free_buffer(mut b: *mut buffer) {
    if !b.is_null() {
        rpl_free((*b).b as *mut libc::c_void);
    }
    rpl_free(b as *mut libc::c_void);
}
