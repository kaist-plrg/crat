use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn has_match(
        needle: *const libc::c_char,
        haystack: *const libc::c_char,
    ) -> libc::c_int;
    #[link_name = "match"]
    fn match_0(needle: *const libc::c_char, haystack: *const libc::c_char) -> score_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct options_t {
    pub benchmark: libc::c_int,
    pub filter: *const libc::c_char,
    pub init_search: *const libc::c_char,
    pub tty_filename: *const libc::c_char,
    pub show_scores: libc::c_int,
    pub num_lines: libc::c_uint,
    pub scrolloff: libc::c_uint,
    pub prompt: *const libc::c_char,
    pub workers: libc::c_uint,
    pub input_delimiter: libc::c_char,
    pub show_info: libc::c_int,
}
pub type score_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scored_result {
    pub score: score_t,
    pub str_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct choices_t {
    pub buffer: *mut libc::c_char,
    pub buffer_size: size_t,
    pub capacity: size_t,
    pub size: size_t,
    pub strings: *mut *const libc::c_char,
    pub results: *mut scored_result,
    pub available: size_t,
    pub selection: size_t,
    pub worker_count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_job {
    pub lock: pthread_mutex_t,
    pub choices: *mut choices_t,
    pub search: *const libc::c_char,
    pub processed: size_t,
    pub workers: *mut worker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker {
    pub thread_id: pthread_t,
    pub job: *mut search_job,
    pub worker_num: libc::c_uint,
    pub result: result_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct result_list {
    pub list: *mut scored_result,
    pub size: size_t,
}
unsafe extern "C" fn cmpchoice(
    mut _idx1: *const libc::c_void,
    mut _idx2: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const scored_result = _idx1 as *const scored_result;
    let mut b: *const scored_result = _idx2 as *const scored_result;
    if (*a).score == (*b).score {
        if (*a).str_0 < (*b).str_0 {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    } else if (*a).score < (*b).score {
        return 1 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn safe_realloc(
    mut buffer: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    buffer = realloc(buffer, size);
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Error: Can't allocate memory (%zu bytes)\n\0" as *const u8
                as *const libc::c_char,
            size,
        );
        abort();
    }
    return buffer;
}
pub unsafe extern "C" fn choices_fread(
    mut c: *mut choices_t,
    mut file: *mut FILE,
    mut input_delimiter: libc::c_char,
) {
    let mut buffer_start: size_t = (*c).buffer_size;
    let mut capacity: size_t = 4096 as libc::c_int as size_t;
    while capacity <= (*c).buffer_size {
        capacity = (capacity as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    (*c)
        .buffer = safe_realloc((*c).buffer as *mut libc::c_void, capacity)
        as *mut libc::c_char;
    loop {
        (*c)
            .buffer_size = ((*c).buffer_size as libc::c_ulong)
            .wrapping_add(
                fread(
                    ((*c).buffer).offset((*c).buffer_size as isize) as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    capacity.wrapping_sub((*c).buffer_size),
                    file,
                ),
            ) as size_t as size_t;
        if !((*c).buffer_size == capacity) {
            break;
        }
        capacity = (capacity as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*c)
            .buffer = safe_realloc((*c).buffer as *mut libc::c_void, capacity)
            as *mut libc::c_char;
    }
    (*c)
        .buffer = safe_realloc(
        (*c).buffer as *mut libc::c_void,
        ((*c).buffer_size).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let fresh0 = (*c).buffer_size;
    (*c).buffer_size = ((*c).buffer_size).wrapping_add(1);
    *((*c).buffer).offset(fresh0 as isize) = '\0' as i32 as libc::c_char;
    let mut line_end: *const libc::c_char = ((*c).buffer)
        .offset((*c).buffer_size as isize);
    let mut line: *mut libc::c_char = ((*c).buffer).offset(buffer_start as isize);
    loop {
        let mut nl: *mut libc::c_char = strchr(line, input_delimiter as libc::c_int);
        if !nl.is_null() {
            let fresh1 = nl;
            nl = nl.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
        }
        if *line != 0 {
            choices_add(c, line);
        }
        line = nl;
        if !(!line.is_null() && line < line_end as *mut libc::c_char) {
            break;
        }
    };
}
unsafe extern "C" fn choices_resize(mut c: *mut choices_t, mut new_capacity: size_t) {
    (*c)
        .strings = safe_realloc(
        (*c).strings as *mut libc::c_void,
        new_capacity
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    (*c).capacity = new_capacity;
}
unsafe extern "C" fn choices_reset_search(mut c: *mut choices_t) {
    free((*c).results as *mut libc::c_void);
    (*c).available = 0 as libc::c_int as size_t;
    (*c).selection = (*c).available;
    (*c).results = 0 as *mut scored_result;
}
pub unsafe extern "C" fn choices_init(
    mut c: *mut choices_t,
    mut options: *mut options_t,
) {
    (*c).strings = 0 as *mut *const libc::c_char;
    (*c).results = 0 as *mut scored_result;
    (*c).buffer_size = 0 as libc::c_int as size_t;
    (*c).buffer = 0 as *mut libc::c_char;
    (*c).size = 0 as libc::c_int as size_t;
    (*c).capacity = (*c).size;
    choices_resize(c, 128 as libc::c_int as size_t);
    if (*options).workers != 0 {
        (*c).worker_count = (*options).workers;
    } else {
        (*c)
            .worker_count = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int
            as libc::c_uint;
    }
    choices_reset_search(c);
}
pub unsafe extern "C" fn choices_destroy(mut c: *mut choices_t) {
    free((*c).buffer as *mut libc::c_void);
    (*c).buffer = 0 as *mut libc::c_char;
    (*c).buffer_size = 0 as libc::c_int as size_t;
    free((*c).strings as *mut libc::c_void);
    (*c).strings = 0 as *mut *const libc::c_char;
    (*c).size = 0 as libc::c_int as size_t;
    (*c).capacity = (*c).size;
    free((*c).results as *mut libc::c_void);
    (*c).results = 0 as *mut scored_result;
    (*c).selection = 0 as libc::c_int as size_t;
    (*c).available = (*c).selection;
}
pub unsafe extern "C" fn choices_add(
    mut c: *mut choices_t,
    mut choice: *const libc::c_char,
) {
    choices_reset_search(c);
    if (*c).size == (*c).capacity {
        choices_resize(
            c,
            ((*c).capacity).wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
    }
    let fresh2 = (*c).size;
    (*c).size = ((*c).size).wrapping_add(1);
    let ref mut fresh3 = *((*c).strings).offset(fresh2 as isize);
    *fresh3 = choice;
}
pub unsafe extern "C" fn choices_available(mut c: *mut choices_t) -> size_t {
    return (*c).available;
}
unsafe extern "C" fn worker_get_next_batch(
    mut job: *mut search_job,
    mut start: *mut size_t,
    mut end: *mut size_t,
) {
    pthread_mutex_lock(&mut (*job).lock);
    *start = (*job).processed;
    (*job)
        .processed = ((*job).processed as libc::c_ulong)
        .wrapping_add(512 as libc::c_int as libc::c_ulong) as size_t as size_t;
    if (*job).processed > (*(*job).choices).size {
        (*job).processed = (*(*job).choices).size;
    }
    *end = (*job).processed;
    pthread_mutex_unlock(&mut (*job).lock);
}
unsafe extern "C" fn merge2(
    mut list1: result_list,
    mut list2: result_list,
) -> result_list {
    let mut result_index: size_t = 0 as libc::c_int as size_t;
    let mut index1: size_t = 0 as libc::c_int as size_t;
    let mut index2: size_t = 0 as libc::c_int as size_t;
    let mut result: result_list = result_list {
        list: 0 as *mut scored_result,
        size: 0,
    };
    result.size = (list1.size).wrapping_add(list2.size);
    result
        .list = malloc(
        (result.size)
            .wrapping_mul(::std::mem::size_of::<scored_result>() as libc::c_ulong),
    ) as *mut scored_result;
    if (result.list).is_null() {
        fprintf(
            stderr,
            b"Error: Can't allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    while index1 < list1.size && index2 < list2.size {
        if cmpchoice(
            &mut *(list1.list).offset(index1 as isize) as *mut scored_result
                as *const libc::c_void,
            &mut *(list2.list).offset(index2 as isize) as *mut scored_result
                as *const libc::c_void,
        ) < 0 as libc::c_int
        {
            let fresh4 = index1;
            index1 = index1.wrapping_add(1);
            let fresh5 = result_index;
            result_index = result_index.wrapping_add(1);
            *(result.list)
                .offset(fresh5 as isize) = *(list1.list).offset(fresh4 as isize);
        } else {
            let fresh6 = index2;
            index2 = index2.wrapping_add(1);
            let fresh7 = result_index;
            result_index = result_index.wrapping_add(1);
            *(result.list)
                .offset(fresh7 as isize) = *(list2.list).offset(fresh6 as isize);
        }
    }
    while index1 < list1.size {
        let fresh8 = index1;
        index1 = index1.wrapping_add(1);
        let fresh9 = result_index;
        result_index = result_index.wrapping_add(1);
        *(result.list).offset(fresh9 as isize) = *(list1.list).offset(fresh8 as isize);
    }
    while index2 < list2.size {
        let fresh10 = index2;
        index2 = index2.wrapping_add(1);
        let fresh11 = result_index;
        result_index = result_index.wrapping_add(1);
        *(result.list).offset(fresh11 as isize) = *(list2.list).offset(fresh10 as isize);
    }
    free(list1.list as *mut libc::c_void);
    free(list2.list as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn choices_search_worker(
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut w: *mut worker = data as *mut worker;
    let mut job: *mut search_job = (*w).job;
    let mut c: *const choices_t = (*job).choices;
    let mut result: *mut result_list = &mut (*w).result;
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    loop {
        worker_get_next_batch(job, &mut start, &mut end);
        if start == end {
            break;
        }
        let mut i: size_t = start;
        while i < end {
            if has_match((*job).search, *((*c).strings).offset(i as isize)) != 0 {
                let ref mut fresh12 = (*((*result).list).offset((*result).size as isize))
                    .str_0;
                *fresh12 = *((*c).strings).offset(i as isize);
                (*((*result).list).offset((*result).size as isize))
                    .score = match_0((*job).search, *((*c).strings).offset(i as isize));
                (*result).size = ((*result).size).wrapping_add(1);
                (*result).size;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    qsort(
        (*result).list as *mut libc::c_void,
        (*result).size,
        ::std::mem::size_of::<scored_result>() as libc::c_ulong,
        Some(
            cmpchoice
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut step: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !(((*w).worker_num).wrapping_rem(((2 as libc::c_int) << step) as libc::c_uint)
        != 0)
    {
        let mut next_worker: libc::c_uint = (*w).worker_num
            | ((1 as libc::c_int) << step) as libc::c_uint;
        if next_worker >= (*c).worker_count {
            break;
        }
        let ref mut fresh13 = *__errno_location();
        *fresh13 = pthread_join(
            (*((*job).workers).offset(next_worker as isize)).thread_id,
            0 as *mut *mut libc::c_void,
        );
        if *fresh13 != 0 {
            perror(b"pthread_join\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        (*w)
            .result = merge2(
            (*w).result,
            (*((*job).workers).offset(next_worker as isize)).result,
        );
        step = step.wrapping_add(1);
        step;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn choices_search(
    mut c: *mut choices_t,
    mut search: *const libc::c_char,
) {
    choices_reset_search(c);
    let mut job: *mut search_job = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<search_job>() as libc::c_ulong,
    ) as *mut search_job;
    (*job).search = search;
    (*job).choices = c;
    if pthread_mutex_init(&mut (*job).lock, 0 as *const pthread_mutexattr_t)
        != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error: pthread_mutex_init failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    (*job)
        .workers = calloc(
        (*c).worker_count as libc::c_ulong,
        ::std::mem::size_of::<worker>() as libc::c_ulong,
    ) as *mut worker;
    let mut workers: *mut worker = (*job).workers;
    let mut i: libc::c_int = ((*c).worker_count)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    while i >= 0 as libc::c_int {
        let ref mut fresh14 = (*workers.offset(i as isize)).job;
        *fresh14 = job;
        (*workers.offset(i as isize)).worker_num = i as libc::c_uint;
        (*workers.offset(i as isize)).result.size = 0 as libc::c_int as size_t;
        let ref mut fresh15 = (*workers.offset(i as isize)).result.list;
        *fresh15 = malloc(
            ((*c).size)
                .wrapping_mul(::std::mem::size_of::<scored_result>() as libc::c_ulong),
        ) as *mut scored_result;
        let ref mut fresh16 = *__errno_location();
        *fresh16 = pthread_create(
            &mut (*workers.offset(i as isize)).thread_id,
            0 as *const pthread_attr_t,
            Some(
                choices_search_worker
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut *workers.offset(i as isize) as *mut worker as *mut libc::c_void,
        );
        if *fresh16 != 0 {
            perror(b"pthread_create\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        i -= 1;
        i;
    }
    if pthread_join(
        (*workers.offset(0 as libc::c_int as isize)).thread_id,
        0 as *mut *mut libc::c_void,
    ) != 0
    {
        perror(b"pthread_join\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    (*c).results = (*workers.offset(0 as libc::c_int as isize)).result.list;
    (*c).available = (*workers.offset(0 as libc::c_int as isize)).result.size;
    free(workers as *mut libc::c_void);
    pthread_mutex_destroy(&mut (*job).lock);
    free(job as *mut libc::c_void);
}
pub unsafe extern "C" fn choices_get(
    mut c: *mut choices_t,
    mut n: size_t,
) -> *const libc::c_char {
    if n < (*c).available {
        return (*((*c).results).offset(n as isize)).str_0
    } else {
        return 0 as *const libc::c_char
    };
}
pub unsafe extern "C" fn choices_getscore(
    mut c: *mut choices_t,
    mut n: size_t,
) -> score_t {
    return (*((*c).results).offset(n as isize)).score;
}
pub unsafe extern "C" fn choices_prev(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c)
            .selection = ((*c).selection)
            .wrapping_add((*c).available)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_rem((*c).available);
    }
}
pub unsafe extern "C" fn choices_next(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c)
            .selection = ((*c).selection)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_rem((*c).available);
    }
}
