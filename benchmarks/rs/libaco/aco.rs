use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn aco_funcp_protector_asm();
    fn acosw(from_co: *mut aco_t, to_co: *mut aco_t) -> *mut libc::c_void;
    fn aco_save_fpucw_mxcsr(p: *mut libc::c_void);
    fn mprotect(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
    ) -> libc::c_int;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint128_t = u128;
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
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
pub struct aco_save_stack_t {
    pub ptr: *mut libc::c_void,
    pub sz: size_t,
    pub valid_sz: size_t,
    pub max_cpsz: size_t,
    pub ct_save: size_t,
    pub ct_restore: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aco_s {
    pub reg: [*mut libc::c_void; 9],
    pub main_co: *mut aco_t,
    pub arg: *mut libc::c_void,
    pub is_end: libc::c_char,
    pub fp: aco_cofuncp_t,
    pub save_stack: aco_save_stack_t,
    pub share_stack: *mut aco_share_stack_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aco_share_stack_t {
    pub ptr: *mut libc::c_void,
    pub sz: size_t,
    pub align_highptr: *mut libc::c_void,
    pub align_retptr: *mut libc::c_void,
    pub align_validsz: size_t,
    pub align_limit: size_t,
    pub owner: *mut aco_t,
    pub guard_page_enabled: libc::c_char,
    pub real_ptr: *mut libc::c_void,
    pub real_sz: size_t,
}
pub type aco_t = aco_s;
pub type aco_cofuncp_t = Option::<unsafe extern "C" fn() -> ()>;
pub unsafe extern "C" fn aco_runtime_test() {
    if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= 4 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    if (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        <= ::std::mem::size_of::<size_t>() as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
}
unsafe extern "C" fn aco_default_protector_last_word() {
    let mut co: *mut aco_t = ({ aco_gtls_co });
    fprintf(
        stderr,
        b"error: aco_default_protector_last_word triggered\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"error: co:%p should call `aco_exit()` instead of direct `return` in co_fp:%p to finish its execution\n\0"
            as *const u8 as *const libc::c_char,
        co,
        ::std::mem::transmute::<aco_cofuncp_t, *mut libc::c_void>((*co).fp),
    );
    if (0 as libc::c_int != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
}
#[thread_local]
pub static mut aco_gtls_co: *mut aco_t = 0 as *const aco_t as *mut aco_t;
#[thread_local]
static mut aco_gtls_last_word_fp: aco_cofuncp_t = unsafe {
    Some(aco_default_protector_last_word as unsafe extern "C" fn() -> ())
};
#[thread_local]
static mut aco_gtls_fpucw_mxcsr: [*mut libc::c_void; 1] = [0 as *const libc::c_void
    as *mut libc::c_void; 1];
pub unsafe extern "C" fn aco_thread_init(mut last_word_co_fp: aco_cofuncp_t) {
    aco_save_fpucw_mxcsr(aco_gtls_fpucw_mxcsr.as_mut_ptr() as *mut libc::c_void);
    if !(::std::mem::transmute::<aco_cofuncp_t, *mut libc::c_void>(last_word_co_fp))
        .is_null()
    {
        aco_gtls_last_word_fp = last_word_co_fp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn aco_funcp_protector() {
    if !(::std::mem::transmute::<
        aco_cofuncp_t,
        *mut libc::c_void,
    >(aco_gtls_last_word_fp))
        .is_null()
    {
        aco_gtls_last_word_fp.unwrap()();
    } else {
        aco_default_protector_last_word();
    }
    if (0 as libc::c_int != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
}
pub unsafe extern "C" fn aco_share_stack_new(mut sz: size_t) -> *mut aco_share_stack_t {
    return aco_share_stack_new2(sz, 1 as libc::c_int as libc::c_char);
}
pub unsafe extern "C" fn aco_share_stack_new2(
    mut sz: size_t,
    mut guard_page_enabled: libc::c_char,
) -> *mut aco_share_stack_t {
    if sz == 0 as libc::c_int as libc::c_ulong {
        sz = (1024 as libc::c_int * 1024 as libc::c_int * 2 as libc::c_int) as size_t;
    }
    if sz < 4096 as libc::c_int as libc::c_ulong {
        sz = 4096 as libc::c_int as size_t;
    }
    if (sz > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    let mut u_pgsz: size_t = 0 as libc::c_int as size_t;
    if guard_page_enabled as libc::c_int != 0 as libc::c_int {
        let mut pgsz: libc::c_long = sysconf(_SC_PAGESIZE as libc::c_int);
        if (pgsz > 0 as libc::c_int as libc::c_long
            && pgsz - 1 as libc::c_int as libc::c_long & pgsz
                == 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        u_pgsz = pgsz as libc::c_ulong;
        if (u_pgsz == pgsz as libc::c_ulong
            && u_pgsz << 1 as libc::c_int >> 1 as libc::c_int == u_pgsz) as libc::c_int
            as libc::c_long != 0
        {} else {
            abort();
        };
        if sz <= u_pgsz {
            sz = u_pgsz << 1 as libc::c_int;
        } else {
            let mut new_sz: size_t = 0;
            if sz & u_pgsz.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
            {
                new_sz = sz & !u_pgsz.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                if (new_sz >= u_pgsz) as libc::c_int as libc::c_long != 0 {} else {
                    abort();
                };
                if (new_sz.wrapping_add(u_pgsz << 1 as libc::c_int) >= new_sz)
                    as libc::c_int as libc::c_long != 0
                {} else {
                    abort();
                };
                new_sz = new_sz.wrapping_add(u_pgsz << 1 as libc::c_int);
                if (sz
                    .wrapping_div(u_pgsz)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    == new_sz.wrapping_div(u_pgsz)) as libc::c_int as libc::c_long != 0
                {} else {
                    abort();
                };
            } else {
                if (sz.wrapping_add(u_pgsz) >= sz) as libc::c_int as libc::c_long != 0
                {} else {
                    abort();
                };
                new_sz = sz.wrapping_add(u_pgsz);
                if (sz
                    .wrapping_div(u_pgsz)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    == new_sz.wrapping_div(u_pgsz)) as libc::c_int as libc::c_long != 0
                {} else {
                    abort();
                };
            }
            sz = new_sz;
            if (sz.wrapping_div(u_pgsz) > 1 as libc::c_int as libc::c_ulong
                && sz & u_pgsz.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
                != 0
            {} else {
                abort();
            };
        }
    }
    let mut p: *mut aco_share_stack_t = malloc(
        ::std::mem::size_of::<aco_share_stack_t>() as libc::c_ulong,
    ) as *mut aco_share_stack_t;
    if p.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                as *const libc::c_char,
            b"aco.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"aco_share_stack_t *aco_share_stack_new2(size_t, char)\0"))
                .as_ptr(),
        );
        abort();
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<aco_share_stack_t>() as libc::c_ulong,
    );
    if guard_page_enabled as libc::c_int != 0 as libc::c_int {
        (*p)
            .real_ptr = mmap(
            0 as *mut libc::c_void,
            sz,
            0x1 as libc::c_int | 0x2 as libc::c_int,
            0x2 as libc::c_int | 0x20 as libc::c_int,
            -(1 as libc::c_int),
            0 as libc::c_int as __off_t,
        );
        if !((*p).real_ptr != -(1 as libc::c_int) as *mut libc::c_void) as libc::c_int
            as libc::c_long != 0
        {
            fprintf(
                stderr,
                b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                    as *const libc::c_char,
                b"aco.c\0" as *const u8 as *const libc::c_char,
                255 as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"aco_share_stack_t *aco_share_stack_new2(size_t, char)\0"))
                    .as_ptr(),
            );
            abort();
        }
        (*p).guard_page_enabled = 1 as libc::c_int as libc::c_char;
        if (0 as libc::c_int == mprotect((*p).real_ptr, u_pgsz, 0x1 as libc::c_int))
            as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        (*p)
            .ptr = ((*p).real_ptr as uintptr_t).wrapping_add(u_pgsz)
            as *mut libc::c_void;
        (*p).real_sz = sz;
        if (sz >= u_pgsz << 1 as libc::c_int) as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        (*p).sz = sz.wrapping_sub(u_pgsz);
    } else {
        (*p).sz = sz;
        (*p).ptr = malloc(sz);
        if ((*p).ptr).is_null() as libc::c_int as libc::c_long != 0 {
            fprintf(
                stderr,
                b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                    as *const libc::c_char,
                b"aco.c\0" as *const u8 as *const libc::c_char,
                267 as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"aco_share_stack_t *aco_share_stack_new2(size_t, char)\0"))
                    .as_ptr(),
            );
            abort();
        }
    }
    (*p).owner = 0 as *mut aco_t;
    let mut u_p: uintptr_t = ((*p).sz)
        .wrapping_sub(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                << 1 as libc::c_int,
        )
        .wrapping_add((*p).ptr as uintptr_t);
    u_p = (u_p >> 4 as libc::c_int) << 4 as libc::c_int;
    (*p).align_highptr = u_p as *mut libc::c_void;
    (*p)
        .align_retptr = u_p
        .wrapping_sub(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        as *mut libc::c_void;
    let ref mut fresh0 = *((*p).align_retptr as *mut *mut libc::c_void);
    *fresh0 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        *mut libc::c_void,
    >(Some(aco_funcp_protector_asm as unsafe extern "C" fn() -> ()));
    if ((*p).sz
        > (16 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    << 1 as libc::c_int,
            )
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong))
        as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    (*p)
        .align_limit = ((*p).sz)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong)
        .wrapping_sub(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                << 1 as libc::c_int,
        );
    return p;
}
pub unsafe extern "C" fn aco_share_stack_destroy(mut sstk: *mut aco_share_stack_t) {
    if (!sstk.is_null() && !((*sstk).ptr).is_null()) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    if (*sstk).guard_page_enabled != 0 {
        if (0 as libc::c_int == munmap((*sstk).real_ptr, (*sstk).real_sz)) as libc::c_int
            as libc::c_long != 0
        {} else {
            abort();
        };
        (*sstk).real_ptr = 0 as *mut libc::c_void;
        (*sstk).ptr = 0 as *mut libc::c_void;
    } else {
        free((*sstk).ptr);
        (*sstk).ptr = 0 as *mut libc::c_void;
    }
    free(sstk as *mut libc::c_void);
}
pub unsafe extern "C" fn aco_create(
    mut main_co: *mut aco_t,
    mut share_stack: *mut aco_share_stack_t,
    mut save_stack_sz: size_t,
    mut fp: aco_cofuncp_t,
    mut arg: *mut libc::c_void,
) -> *mut aco_t {
    let mut p: *mut aco_t = malloc(::std::mem::size_of::<aco_t>() as libc::c_ulong)
        as *mut aco_t;
    if p.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                as *const libc::c_char,
            b"aco.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"aco_t *aco_create(aco_t *, aco_share_stack_t *, size_t, aco_cofuncp_t, void *)\0",
            ))
                .as_ptr(),
        );
        abort();
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<aco_t>() as libc::c_ulong,
    );
    if !main_co.is_null() {
        if !share_stack.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        (*p).share_stack = share_stack;
        (*p)
            .reg[4 as libc::c_int
            as usize] = ::std::mem::transmute::<aco_cofuncp_t, *mut libc::c_void>(fp);
        (*p).reg[5 as libc::c_int as usize] = (*(*p).share_stack).align_retptr;
        (*p)
            .reg[8 as libc::c_int
            as usize] = aco_gtls_fpucw_mxcsr[0 as libc::c_int as usize];
        (*p).main_co = main_co;
        (*p).arg = arg;
        (*p).fp = fp;
        if save_stack_sz == 0 as libc::c_int as libc::c_ulong {
            save_stack_sz = 64 as libc::c_int as size_t;
        }
        (*p).save_stack.ptr = malloc(save_stack_sz);
        if ((*p).save_stack.ptr).is_null() as libc::c_int as libc::c_long != 0 {
            fprintf(
                stderr,
                b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                    as *const libc::c_char,
                b"aco.c\0" as *const u8 as *const libc::c_char,
                344 as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"aco_t *aco_create(aco_t *, aco_share_stack_t *, size_t, aco_cofuncp_t, void *)\0",
                ))
                    .as_ptr(),
            );
            abort();
        }
        (*p).save_stack.sz = save_stack_sz;
        (*p).save_stack.valid_sz = 0 as libc::c_int as size_t;
        return p;
    } else {
        (*p).main_co = 0 as *mut aco_t;
        (*p).arg = arg;
        (*p).fp = fp;
        (*p).share_stack = 0 as *mut aco_share_stack_t;
        (*p).save_stack.ptr = 0 as *mut libc::c_void;
        return p;
    };
}
pub unsafe extern "C" fn aco_resume(mut resume_co: *mut aco_t) {
    if (!resume_co.is_null() && !((*resume_co).main_co).is_null()
        && (*resume_co).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    if (*(*resume_co).share_stack).owner != resume_co {
        if !((*(*resume_co).share_stack).owner).is_null() {
            let mut owner_co: *mut aco_t = (*(*resume_co).share_stack).owner;
            if ((*owner_co).share_stack == (*resume_co).share_stack) as libc::c_int
                as libc::c_long != 0
            {} else {
                abort();
            };
            if ((*(*owner_co).share_stack).align_retptr as uintptr_t
                >= (*owner_co).reg[5 as libc::c_int as usize] as uintptr_t
                && ((*(*owner_co).share_stack).align_highptr as uintptr_t)
                    .wrapping_sub((*(*owner_co).share_stack).align_limit)
                    <= (*owner_co).reg[5 as libc::c_int as usize] as uintptr_t)
                as libc::c_int as libc::c_long != 0
            {} else {
                abort();
            };
            (*owner_co)
                .save_stack
                .valid_sz = ((*(*owner_co).share_stack).align_retptr as uintptr_t)
                .wrapping_sub((*owner_co).reg[5 as libc::c_int as usize] as uintptr_t);
            if (*owner_co).save_stack.sz < (*owner_co).save_stack.valid_sz {
                free((*owner_co).save_stack.ptr);
                (*owner_co).save_stack.ptr = 0 as *mut libc::c_void;
                loop {
                    (*owner_co)
                        .save_stack
                        .sz = (*owner_co).save_stack.sz << 1 as libc::c_int;
                    if ((*owner_co).save_stack.sz > 0 as libc::c_int as libc::c_ulong)
                        as libc::c_int as libc::c_long != 0
                    {} else {
                        abort();
                    };
                    if (*owner_co).save_stack.sz >= (*owner_co).save_stack.valid_sz {
                        break;
                    }
                }
                (*owner_co).save_stack.ptr = malloc((*owner_co).save_stack.sz);
                if ((*owner_co).save_stack.ptr).is_null() as libc::c_int as libc::c_long
                    != 0
                {
                    fprintf(
                        stderr,
                        b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                            as *const libc::c_char,
                        b"aco.c\0" as *const u8 as *const libc::c_char,
                        403 as libc::c_int,
                        (*::std::mem::transmute::<
                            &[u8; 25],
                            &[libc::c_char; 25],
                        >(b"void aco_resume(aco_t *)\0"))
                            .as_ptr(),
                    );
                    abort();
                }
            }
            if (*owner_co).save_stack.valid_sz > 0 as libc::c_int as libc::c_ulong {
                if (*owner_co).reg[5 as libc::c_int as usize] as uintptr_t
                    & 0xf as libc::c_int as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
                    && (*owner_co).save_stack.ptr as uintptr_t
                        & 0xf as libc::c_int as libc::c_ulong
                        == 0 as libc::c_int as libc::c_ulong
                    && (*owner_co).save_stack.valid_sz
                        & 0xf as libc::c_int as libc::c_ulong
                        == 0x8 as libc::c_int as libc::c_ulong
                    && (*owner_co).save_stack.valid_sz >> 4 as libc::c_int
                        >= 0 as libc::c_int as libc::c_ulong
                    && (*owner_co).save_stack.valid_sz >> 4 as libc::c_int
                        <= 8 as libc::c_int as libc::c_ulong
                {
                    let mut xmm0: __uint128_t = 0;
                    let mut xmm1: __uint128_t = 0;
                    let mut xmm2: __uint128_t = 0;
                    let mut xmm3: __uint128_t = 0;
                    let mut xmm4: __uint128_t = 0;
                    let mut xmm5: __uint128_t = 0;
                    let mut xmm6: __uint128_t = 0;
                    let mut xmm7: __uint128_t = 0;
                    match (*owner_co).save_stack.valid_sz >> 4 as libc::c_int {
                        1 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                        }
                        2 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize) = xmm1;
                        }
                        3 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize);
                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize) = xmm1;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize) = xmm2;
                        }
                        4 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize);
                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize);
                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize) = xmm1;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize) = xmm2;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize) = xmm3;
                        }
                        5 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize);
                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize);
                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize);
                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize) = xmm1;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize) = xmm2;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize) = xmm3;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize) = xmm4;
                        }
                        6 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize);
                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize);
                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize);
                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize);
                            xmm5 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(5 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize) = xmm1;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize) = xmm2;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize) = xmm3;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize) = xmm4;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(5 as libc::c_int as isize) = xmm5;
                        }
                        7 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize);
                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize);
                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize);
                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize);
                            xmm5 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(5 as libc::c_int as isize);
                            xmm6 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(6 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize) = xmm1;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize) = xmm2;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize) = xmm3;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize) = xmm4;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(5 as libc::c_int as isize) = xmm5;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(6 as libc::c_int as isize) = xmm6;
                        }
                        8 => {
                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize);
                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize);
                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize);
                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize);
                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize);
                            xmm5 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(5 as libc::c_int as isize);
                            xmm6 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(6 as libc::c_int as isize);
                            xmm7 = *((*owner_co).reg[5 as libc::c_int as usize]
                                as *mut __uint128_t)
                                .offset(7 as libc::c_int as isize);
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(0 as libc::c_int as isize) = xmm0;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(1 as libc::c_int as isize) = xmm1;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(2 as libc::c_int as isize) = xmm2;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(3 as libc::c_int as isize) = xmm3;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(4 as libc::c_int as isize) = xmm4;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(5 as libc::c_int as isize) = xmm5;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(6 as libc::c_int as isize) = xmm6;
                            *((*owner_co).save_stack.ptr as *mut __uint128_t)
                                .offset(7 as libc::c_int as isize) = xmm7;
                        }
                        0 | _ => {}
                    }
                    *(((*owner_co).save_stack.ptr as uintptr_t)
                        .wrapping_add((*owner_co).save_stack.valid_sz)
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                        as *mut uint64_t) = *(((*owner_co).reg[5 as libc::c_int as usize]
                        as uintptr_t)
                        .wrapping_add((*owner_co).save_stack.valid_sz)
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                        as *mut uint64_t);
                } else {
                    memcpy(
                        (*owner_co).save_stack.ptr,
                        (*owner_co).reg[5 as libc::c_int as usize],
                        (*owner_co).save_stack.valid_sz,
                    );
                }
                (*owner_co)
                    .save_stack
                    .ct_save = ((*owner_co).save_stack.ct_save).wrapping_add(1);
                (*owner_co).save_stack.ct_save;
            }
            if (*owner_co).save_stack.valid_sz > (*owner_co).save_stack.max_cpsz {
                (*owner_co).save_stack.max_cpsz = (*owner_co).save_stack.valid_sz;
            }
            (*(*owner_co).share_stack).owner = 0 as *mut aco_t;
            (*(*owner_co).share_stack).align_validsz = 0 as libc::c_int as size_t;
        }
        if ((*(*resume_co).share_stack).owner).is_null() as libc::c_int as libc::c_long
            != 0
        {} else {
            abort();
        };
        if ((*resume_co).save_stack.valid_sz
            <= ((*(*resume_co).share_stack).align_limit)
                .wrapping_sub(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                )) as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        if (*resume_co).save_stack.valid_sz > 0 as libc::c_int as libc::c_ulong {
            if (*resume_co).save_stack.ptr as uintptr_t
                & 0xf as libc::c_int as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
                && ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                    .wrapping_sub((*resume_co).save_stack.valid_sz) as *mut libc::c_void
                    as uintptr_t & 0xf as libc::c_int as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
                && (*resume_co).save_stack.valid_sz & 0xf as libc::c_int as libc::c_ulong
                    == 0x8 as libc::c_int as libc::c_ulong
                && (*resume_co).save_stack.valid_sz >> 4 as libc::c_int
                    >= 0 as libc::c_int as libc::c_ulong
                && (*resume_co).save_stack.valid_sz >> 4 as libc::c_int
                    <= 8 as libc::c_int as libc::c_ulong
            {
                let mut xmm0_0: __uint128_t = 0;
                let mut xmm1_0: __uint128_t = 0;
                let mut xmm2_0: __uint128_t = 0;
                let mut xmm3_0: __uint128_t = 0;
                let mut xmm4_0: __uint128_t = 0;
                let mut xmm5_0: __uint128_t = 0;
                let mut xmm6_0: __uint128_t = 0;
                let mut xmm7_0: __uint128_t = 0;
                match (*resume_co).save_stack.valid_sz >> 4 as libc::c_int {
                    1 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                    }
                    2 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        xmm1_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize) = xmm1_0;
                    }
                    3 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        xmm1_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize);
                        xmm2_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize) = xmm1_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize) = xmm2_0;
                    }
                    4 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        xmm1_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize);
                        xmm2_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize);
                        xmm3_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize) = xmm1_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize) = xmm2_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize) = xmm3_0;
                    }
                    5 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        xmm1_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize);
                        xmm2_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize);
                        xmm3_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize);
                        xmm4_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize) = xmm1_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize) = xmm2_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize) = xmm3_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize) = xmm4_0;
                    }
                    6 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        xmm1_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize);
                        xmm2_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize);
                        xmm3_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize);
                        xmm4_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize);
                        xmm5_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(5 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize) = xmm1_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize) = xmm2_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize) = xmm3_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize) = xmm4_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(5 as libc::c_int as isize) = xmm5_0;
                    }
                    7 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        xmm1_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize);
                        xmm2_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize);
                        xmm3_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize);
                        xmm4_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize);
                        xmm5_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(5 as libc::c_int as isize);
                        xmm6_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(6 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize) = xmm1_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize) = xmm2_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize) = xmm3_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize) = xmm4_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(5 as libc::c_int as isize) = xmm5_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(6 as libc::c_int as isize) = xmm6_0;
                    }
                    8 => {
                        xmm0_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize);
                        xmm1_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize);
                        xmm2_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize);
                        xmm3_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize);
                        xmm4_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize);
                        xmm5_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(5 as libc::c_int as isize);
                        xmm6_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(6 as libc::c_int as isize);
                        xmm7_0 = *((*resume_co).save_stack.ptr as *mut __uint128_t)
                            .offset(7 as libc::c_int as isize);
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(0 as libc::c_int as isize) = xmm0_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(1 as libc::c_int as isize) = xmm1_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(2 as libc::c_int as isize) = xmm2_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(3 as libc::c_int as isize) = xmm3_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(4 as libc::c_int as isize) = xmm4_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(5 as libc::c_int as isize) = xmm5_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(6 as libc::c_int as isize) = xmm6_0;
                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void as *mut __uint128_t)
                            .offset(7 as libc::c_int as isize) = xmm7_0;
                    }
                    0 | _ => {}
                }
                *((((*(*resume_co).share_stack).align_retptr as uintptr_t)
                    .wrapping_sub((*resume_co).save_stack.valid_sz) as *mut libc::c_void
                    as uintptr_t)
                    .wrapping_add((*resume_co).save_stack.valid_sz)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                    as *mut uint64_t) = *(((*resume_co).save_stack.ptr as uintptr_t)
                    .wrapping_add((*resume_co).save_stack.valid_sz)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong) as *mut uint64_t);
            } else {
                memcpy(
                    ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                        .wrapping_sub((*resume_co).save_stack.valid_sz)
                        as *mut libc::c_void,
                    (*resume_co).save_stack.ptr,
                    (*resume_co).save_stack.valid_sz,
                );
            }
            (*resume_co)
                .save_stack
                .ct_restore = ((*resume_co).save_stack.ct_restore).wrapping_add(1);
            (*resume_co).save_stack.ct_restore;
        }
        if (*resume_co).save_stack.valid_sz > (*resume_co).save_stack.max_cpsz {
            (*resume_co).save_stack.max_cpsz = (*resume_co).save_stack.valid_sz;
        }
        (*(*resume_co).share_stack)
            .align_validsz = ((*resume_co).save_stack.valid_sz)
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
        (*(*resume_co).share_stack).owner = resume_co;
    }
    aco_gtls_co = resume_co;
    acosw((*resume_co).main_co, resume_co);
    aco_gtls_co = (*resume_co).main_co;
}
pub unsafe extern "C" fn aco_destroy(mut co: *mut aco_t) {
    if !co.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if ({ ((*co).main_co == 0 as *mut libc::c_void as *mut aco_t) as libc::c_int }) != 0
    {
        free(co as *mut libc::c_void);
    } else {
        if (*(*co).share_stack).owner == co {
            (*(*co).share_stack).owner = 0 as *mut aco_t;
            (*(*co).share_stack).align_validsz = 0 as libc::c_int as size_t;
        }
        free((*co).save_stack.ptr);
        (*co).save_stack.ptr = 0 as *mut libc::c_void;
        free(co as *mut libc::c_void);
    };
}
